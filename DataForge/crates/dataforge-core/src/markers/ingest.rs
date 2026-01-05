//! Marker Ingestion Service
//!
//! Handles the complete marker CSV file ingestion workflow:
//! 1. Parse CSV file
//! 2. Detect columns and validate required fields
//! 3. Group markers by well name (for multi-well files)
//! 4. Match wells to existing wells or create new ones
//! 5. Create marker_set record(s)
//! 6. Create individual marker records

use crate::blob::BlobStore;
use crate::error::{Error, Result};
use crate::markers::parser::{parse_marker_csv, ParsedMarkerFile, ParsedMarkerRow};
use crate::models::MarkerColumnType;
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::Path;
use tracing::info;
use uuid::Uuid;

/// Options for marker CSV ingestion
#[derive(Debug, Clone)]
pub struct MarkerIngestOptions {
    /// Set name (optional)
    pub set_name: Option<String>,
    /// Interpretation type (formation, sequence, etc.)
    pub interpretation_type: Option<String>,
    /// Interpreter name
    pub interpreter: Option<String>,
    /// Depth unit (ft or m)
    pub depth_unit: String,
    /// Depth reference (kb, rt, msl)
    pub depth_reference: Option<String>,
    /// Reference source
    pub reference_source: Option<String>,
    /// Whether to store raw CSV file
    pub store_raw_file: bool,
    /// Well matching mode
    pub well_match_mode: WellMatchMode,
    /// Whether to auto-create wells for unmatched entries
    pub auto_create_wells: bool,
    /// Default well ID to use for single-well files without well column
    pub default_well_id: Option<Uuid>,
    /// Whether to allow markers without mapped wells (stores well_name as text, well_id as NULL)
    pub allow_unmapped_wells: bool,
}

/// Well matching mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WellMatchMode {
    /// Exact name match
    Exact,
    /// Case-insensitive match
    CaseInsensitive,
}

impl Default for MarkerIngestOptions {
    fn default() -> Self {
        Self {
            set_name: None,
            interpretation_type: None,
            interpreter: None,
            depth_unit: "ft".to_string(),
            depth_reference: None,
            reference_source: None,
            store_raw_file: true,
            well_match_mode: WellMatchMode::CaseInsensitive,
            auto_create_wells: false,
            default_well_id: None,
            allow_unmapped_wells: false,
        }
    }
}

/// Column mapping override
#[derive(Debug, Clone)]
pub struct MarkerColumnMapping {
    pub index: usize,
    pub column_type: MarkerColumnType,
    pub unit: Option<String>,
}

/// Group of markers for a single well
#[derive(Debug, Clone)]
pub struct WellMarkerGroup {
    /// Well identifier from file
    pub well_name: String,
    /// Matched well ID (if found)
    pub matched_well_id: Option<Uuid>,
    /// Whether a new well was created
    pub well_created: bool,
    /// Markers for this well
    pub markers: Vec<ParsedMarkerRow>,
}

/// Result of ingesting a marker CSV file
#[derive(Debug)]
pub struct MarkerIngestResult {
    /// Created marker set IDs (one per well)
    pub marker_set_ids: Vec<Uuid>,
    /// Wells that were matched or created
    pub well_results: Vec<WellMarkerGroup>,
    /// Source file hash
    pub source_file_hash: String,
    /// Raw file blob hash (if stored)
    pub raw_file_blob_hash: Option<String>,
    /// Total markers imported
    pub marker_count: usize,
    /// Total wells affected
    pub well_count: usize,
    /// Wells that were newly created
    pub wells_created: usize,
    /// Any warnings
    pub warnings: Vec<String>,
}

/// Ingest a marker CSV file
pub fn ingest_marker_file(
    conn: &Connection,
    blob_store: &BlobStore,
    csv_path: &Path,
    workspace_id: Uuid,
    created_by: Uuid,
    _column_mappings: Option<&[MarkerColumnMapping]>,
    options: &MarkerIngestOptions,
) -> Result<MarkerIngestResult> {
    // 1. Read and hash raw CSV file
    let raw_bytes = std::fs::read(csv_path)?;
    let source_file_hash = hex::encode(Sha256::digest(&raw_bytes));

    // 2. Parse CSV file
    let parsed = parse_marker_csv(Cursor::new(&raw_bytes))
        .map_err(|e| Error::InvalidData(e))?;

    // 3. Group markers by well
    let well_groups = group_markers_by_well(&parsed, options);

    // 4. Match or create wells
    let mut matched_groups = Vec::new();
    let mut wells_created = 0;

    for mut group in well_groups {
        let (well_id, created) = match_or_create_well(
            conn,
            &group.well_name,
            workspace_id,
            created_by,
            options,
        )?;

        group.matched_well_id = well_id; // Can be None for unmapped wells
        group.well_created = created;
        if created {
            wells_created += 1;
        }
        matched_groups.push(group);
    }

    // 5. Store raw file if requested
    let raw_file_blob_hash = if options.store_raw_file {
        let hash = blob_store.store(&raw_bytes)?;
        register_blob(conn, &hash, raw_bytes.len())?;
        Some(hash)
    } else {
        None
    };

    // 6. Create marker sets and markers for each well
    let filename = csv_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown.csv");

    let mut marker_set_ids = Vec::new();
    let mut total_markers = 0;

    for group in &matched_groups {
        // Create marker_set record (well_id can be None for unmapped wells)
        let marker_set_id = create_marker_set(
            conn,
            group.matched_well_id,
            workspace_id,
            filename,
            &source_file_hash,
            raw_file_blob_hash.as_deref(),
            &group.markers,
            created_by,
            options,
        )?;

        marker_set_ids.push(marker_set_id);

        // Create individual marker records
        for marker_row in &group.markers {
            create_marker(
                conn,
                marker_set_id,
                group.matched_well_id,
                &group.well_name,
                marker_row,
                created_by,
            )?;
            total_markers += 1;
        }
    }

    let well_count = matched_groups.len();

    info!(
        marker_sets = marker_set_ids.len(),
        markers = total_markers,
        wells = well_count,
        wells_created = wells_created,
        "Ingested marker CSV file"
    );

    Ok(MarkerIngestResult {
        marker_set_ids,
        well_results: matched_groups,
        source_file_hash,
        raw_file_blob_hash,
        marker_count: total_markers,
        well_count,
        wells_created,
        warnings: parsed.warnings,
    })
}

/// Group markers by well name
fn group_markers_by_well(
    parsed: &ParsedMarkerFile,
    _options: &MarkerIngestOptions,
) -> Vec<WellMarkerGroup> {
    let mut groups: HashMap<String, Vec<ParsedMarkerRow>> = HashMap::new();

    for row in &parsed.rows {
        let well_name = row
            .well_name
            .clone()
            .unwrap_or_else(|| "Default Well".to_string());

        groups.entry(well_name).or_default().push(row.clone());
    }

    groups
        .into_iter()
        .map(|(well_name, markers)| WellMarkerGroup {
            well_name,
            matched_well_id: None,
            well_created: false,
            markers,
        })
        .collect()
}

/// Match well name to existing well or create new one
/// Returns (Option<well_id>, was_created) - well_id is None when allow_unmapped_wells is true and no match found
fn match_or_create_well(
    conn: &Connection,
    well_name: &str,
    workspace_id: Uuid,
    created_by: Uuid,
    options: &MarkerIngestOptions,
) -> Result<(Option<Uuid>, bool)> {
    // Try to match existing well
    let query = match options.well_match_mode {
        WellMatchMode::Exact => {
            "SELECT id FROM wells WHERE workspace_id = ? AND name = ? AND deleted_at IS NULL"
        }
        WellMatchMode::CaseInsensitive => {
            "SELECT id FROM wells WHERE workspace_id = ? AND LOWER(name) = LOWER(?) AND deleted_at IS NULL"
        }
    };

    // Try name match
    if let Ok(well_id_str) = conn.query_row(
        query,
        params![workspace_id.to_string(), well_name],
        |row| row.get::<_, String>(0),
    ) {
        let well_id = Uuid::parse_str(&well_id_str)
            .map_err(|e| Error::InvalidData(format!("Invalid well ID: {}", e)))?;
        return Ok((Some(well_id), false));
    }

    // Try matching by UWI
    if let Ok(well_id_str) = conn.query_row(
        "SELECT id FROM wells WHERE workspace_id = ? AND uwi = ? AND deleted_at IS NULL",
        params![workspace_id.to_string(), well_name],
        |row| row.get::<_, String>(0),
    ) {
        let well_id = Uuid::parse_str(&well_id_str)
            .map_err(|e| Error::InvalidData(format!("Invalid well ID: {}", e)))?;
        return Ok((Some(well_id), false));
    }

    // If no match and auto-create is enabled, create new well
    if options.auto_create_wells {
        let new_well_id = Uuid::new_v4();

        conn.execute(
            r#"
            INSERT INTO wells (
                id, workspace_id, name,
                depth_unit, depth_step, depth_origin,
                created_by
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            params![
                new_well_id.to_string(),
                workspace_id.to_string(),
                well_name,
                &options.depth_unit,
                0.5,
                0.0,
                created_by.to_string(),
            ],
        )?;

        info!(well_id = %new_well_id, name = well_name, "Created new well for markers");
        return Ok((Some(new_well_id), true));
    }

    // Use default well if provided
    if let Some(default_id) = options.default_well_id {
        return Ok((Some(default_id), false));
    }

    // If unmapped wells are allowed, return None for well_id (will store well_name as text)
    if options.allow_unmapped_wells {
        info!(well_name = well_name, "Allowing unmapped well - will store well_name as text");
        return Ok((None, false));
    }

    Err(Error::InvalidData(format!(
        "No matching well found for '{}' and auto-create is disabled",
        well_name
    )))
}

fn create_marker_set(
    conn: &Connection,
    well_id: Option<Uuid>,
    workspace_id: Uuid,
    filename: &str,
    source_file_hash: &str,
    raw_file_blob_hash: Option<&str>,
    markers: &[ParsedMarkerRow],
    created_by: Uuid,
    options: &MarkerIngestOptions,
) -> Result<Uuid> {
    let marker_set_id = Uuid::new_v4();

    let min_depth = markers
        .iter()
        .filter_map(|m| m.measured_depth)
        .fold(f64::INFINITY, f64::min);
    let max_depth = markers
        .iter()
        .filter_map(|m| m.measured_depth)
        .fold(f64::NEG_INFINITY, f64::max);

    conn.execute(
        r#"
        INSERT INTO marker_sets (
            id, well_id, workspace_id, source_filename, source_file_hash, raw_file_blob_hash,
            name, interpretation_type, interpreter, depth_unit, depth_reference, reference_source,
            min_depth, max_depth, marker_count, imported_by
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
        "#,
        params![
            marker_set_id.to_string(),
            well_id.map(|id| id.to_string()), // NULL for unmapped wells
            workspace_id.to_string(),
            filename,
            source_file_hash,
            raw_file_blob_hash,
            options.set_name,
            options.interpretation_type,
            options.interpreter,
            &options.depth_unit,
            options.depth_reference,
            options.reference_source,
            if min_depth.is_finite() { Some(min_depth) } else { None },
            if max_depth.is_finite() { Some(max_depth) } else { None },
            markers.len() as i64,
            created_by.to_string(),
        ],
    )?;

    Ok(marker_set_id)
}

fn create_marker(
    conn: &Connection,
    marker_set_id: Uuid,
    well_id: Option<Uuid>,
    well_name: &str,
    row: &ParsedMarkerRow,
    created_by: Uuid,
) -> Result<Uuid> {
    let marker_id = Uuid::new_v4();

    let marker_name = row
        .marker_name
        .clone()
        .unwrap_or_else(|| format!("Marker at {}", row.measured_depth.unwrap_or(0.0)));

    let original_values = serde_json::to_string(&row.raw_values).ok();

    conn.execute(
        r#"
        INSERT INTO markers (
            id, marker_set_id, well_id, well_name, name, marker_type,
            measured_depth, true_vertical_depth, true_vertical_depth_ss,
            thickness, quality, picked_by, comments,
            original_row_index, original_values, created_by
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
        "#,
        params![
            marker_id.to_string(),
            marker_set_id.to_string(),
            well_id.map(|id| id.to_string()), // NULL for unmapped wells
            well_name, // Store original well name from CSV
            marker_name,
            row.marker_type,
            row.measured_depth,
            row.tvd,
            row.tvdss,
            row.thickness,
            row.quality,
            row.interpreter,
            row.comments,
            row.row_index as i64,
            original_values,
            created_by.to_string(),
        ],
    )?;

    Ok(marker_id)
}

fn register_blob(conn: &Connection, hash: &str, size: usize) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO blob_registry (hash, size_bytes) VALUES (?1, ?2)",
        params![hash, size as i64],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::open_memory_db;
    use tempfile::tempdir;

    fn setup_test_db() -> (Connection, BlobStore) {
        let conn = open_memory_db().expect("Failed to create test database");
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let blob_store = BlobStore::new(temp_dir.path()).expect("Failed to create blob store");
        (conn, blob_store)
    }

    fn create_test_workspace_and_member(conn: &Connection) -> (Uuid, Uuid, Uuid) {
        let account_id = Uuid::new_v4();
        let workspace_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();

        conn.execute(
            "INSERT INTO accounts (id, email, name) VALUES (?, ?, ?)",
            params![account_id.to_string(), "test@test.com", "Test User"],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO workspaces (id, name, owner_account_id) VALUES (?, ?, ?)",
            params![workspace_id.to_string(), "Test Workspace", account_id.to_string()],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO workspace_members (id, workspace_id, account_id, role) VALUES (?, ?, ?, ?)",
            params![member_id.to_string(), workspace_id.to_string(), account_id.to_string(), "owner"],
        )
        .unwrap();

        (workspace_id, member_id, account_id)
    }

    fn create_test_well(conn: &Connection, workspace_id: Uuid, member_id: Uuid, name: &str) -> Uuid {
        let well_id = Uuid::new_v4();
        conn.execute(
            "INSERT INTO wells (id, workspace_id, name, depth_unit, depth_step, depth_origin, created_by) VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![well_id.to_string(), workspace_id.to_string(), name, "ft", 0.5, 0.0, member_id.to_string()],
        )
        .unwrap();
        well_id
    }

    #[test]
    fn test_ingest_simple_marker_file() {
        let (conn, blob_store) = setup_test_db();
        let (workspace_id, member_id, _) = create_test_workspace_and_member(&conn);
        let well_id = create_test_well(&conn, workspace_id, member_id, "Test Well");

        // Create a temp marker file
        let temp_dir = tempdir().unwrap();
        let csv_path = temp_dir.path().join("markers.txt");
        std::fs::write(&csv_path, "30\tSeasurface\n553.6\tMFS11\n612.9\tFS11\n").unwrap();

        let options = MarkerIngestOptions {
            default_well_id: Some(well_id),
            ..Default::default()
        };

        let result = ingest_marker_file(
            &conn,
            &blob_store,
            &csv_path,
            workspace_id,
            member_id,
            None,
            &options,
        )
        .unwrap();

        assert_eq!(result.marker_count, 3);
        assert_eq!(result.well_count, 1);
        assert_eq!(result.wells_created, 0);
        assert_eq!(result.marker_set_ids.len(), 1);
    }

    #[test]
    fn test_ingest_with_auto_create_well() {
        let (conn, blob_store) = setup_test_db();
        let (workspace_id, member_id, _) = create_test_workspace_and_member(&conn);

        // Create a temp marker file with well names
        let temp_dir = tempdir().unwrap();
        let csv_path = temp_dir.path().join("markers.csv");
        std::fs::write(&csv_path, "Well,Depth,Marker\nNewWell,100,TopA\nNewWell,200,TopB\n").unwrap();

        let options = MarkerIngestOptions {
            auto_create_wells: true,
            ..Default::default()
        };

        let result = ingest_marker_file(
            &conn,
            &blob_store,
            &csv_path,
            workspace_id,
            member_id,
            None,
            &options,
        )
        .unwrap();

        assert_eq!(result.marker_count, 2);
        assert_eq!(result.wells_created, 1);

        // Verify well was created
        let well_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM wells WHERE workspace_id = ?",
                params![workspace_id.to_string()],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(well_count, 1);
    }

    #[test]
    fn test_ingest_with_unmapped_wells() {
        let (conn, blob_store) = setup_test_db();
        let (workspace_id, member_id, _) = create_test_workspace_and_member(&conn);

        // Create a temp marker file with well names but no matching wells
        let temp_dir = tempdir().unwrap();
        let csv_path = temp_dir.path().join("markers.csv");
        std::fs::write(&csv_path, "Well Name,Depth,Marker\nF06-1,589.14,FS11\nF06-1,670.54,MFS10\nF02-1,30,Seasurface\n").unwrap();

        let options = MarkerIngestOptions {
            allow_unmapped_wells: true,
            ..Default::default()
        };

        let result = ingest_marker_file(
            &conn,
            &blob_store,
            &csv_path,
            workspace_id,
            member_id,
            None,
            &options,
        )
        .unwrap();

        assert_eq!(result.marker_count, 3);
        assert_eq!(result.well_count, 2); // Two different well names
        assert_eq!(result.wells_created, 0); // No wells created

        // Verify markers were created with NULL well_id but with well_name stored
        let marker_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM markers WHERE well_id IS NULL AND well_name IS NOT NULL",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(marker_count, 3);

        // Verify well_name was stored correctly
        let well_names: Vec<String> = conn
            .prepare("SELECT DISTINCT well_name FROM markers ORDER BY well_name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<std::result::Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(well_names, vec!["F02-1", "F06-1"]);
    }
}
