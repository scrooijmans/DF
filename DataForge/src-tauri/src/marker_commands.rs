//! Marker Commands
//!
//! Tauri commands for wellbore marker (well tops) data ingestion from CSV files.
//! Follows OSDU WellboreMarkerSet patterns with MarkerName and MarkerMeasuredDepth
//! as core required fields.

use dataforge_core::blob::BlobStore;
use dataforge_core::markers::{
    ingest_marker_file, parse_marker_csv, MarkerIngestOptions, WellMapping, WellMatchMode,
};
use dataforge_core::models::MarkerColumnType;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;
use tracing::{error, info};
use uuid::Uuid;

use crate::state::AppState;

/// Response type for marker CSV parsing
#[derive(Debug, Serialize, Deserialize)]
pub struct ParseMarkerResponse {
    /// Unique identifier for this parse result
    pub file_id: String,
    /// Original file name
    pub file_name: String,
    /// Detected columns with their types
    pub columns: Vec<MarkerColumnInfo>,
    /// Number of data rows
    pub row_count: usize,
    /// Detected delimiter
    pub delimiter: String,
    /// Whether file has a header row
    pub has_header: bool,
    /// Whether this is a multi-well file
    pub is_multi_well: bool,
    /// Well names found in the file
    pub well_names: Vec<String>,
    /// Parsing warnings
    pub warnings: Vec<String>,
    /// Whether required columns (MarkerName, MD) were found
    pub has_required_columns: bool,
    /// Missing required column names
    pub missing_required: Vec<String>,
}

/// Information about a column in the marker CSV
#[derive(Debug, Serialize, Deserialize)]
pub struct MarkerColumnInfo {
    /// Column index in CSV
    pub index: usize,
    /// Original header name
    pub header: String,
    /// Detected column type
    pub detected_type: String,
    /// Detection confidence (0.0 - 1.0)
    pub confidence: f64,
    /// Detected unit (if any)
    pub unit: Option<String>,
    /// Sample values for preview
    pub sample_values: Vec<String>,
    /// Statistics for numeric columns
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub mean_value: Option<f64>,
}

/// Request for processing marker CSV files
#[derive(Debug, Deserialize)]
pub struct IngestMarkerRequest {
    /// File paths to process
    pub file_paths: Vec<String>,
    /// Well mappings: well_name from file -> well_id
    pub well_mappings: Vec<WellMappingEntry>,
    /// Column configurations (type overrides)
    pub column_configs: Vec<MarkerColumnConfig>,
    /// Marker set metadata
    pub set_name: Option<String>,
    pub interpretation_type: Option<String>,
    pub interpreter: Option<String>,
    pub depth_unit: Option<String>,
    pub depth_reference: Option<String>,
    /// Whether to auto-create wells for unmatched entries
    pub auto_create_wells: bool,
    /// Whether to allow markers without mapped wells (stores well_name as text, well_id as NULL)
    pub allow_unmapped_wells: bool,
}

/// Well mapping entry
#[derive(Debug, Deserialize)]
pub struct WellMappingEntry {
    pub well_name: String,
    pub well_id: Option<String>,
    pub create_new: bool,
}

/// Configuration for a marker column
#[derive(Debug, Deserialize)]
pub struct MarkerColumnConfig {
    /// Column index
    pub index: usize,
    /// Override column type
    pub column_type: Option<String>,
    /// Override unit
    pub unit: Option<String>,
}

/// Response from marker ingestion
#[derive(Debug, Serialize)]
pub struct IngestMarkerResponse {
    pub success: bool,
    pub marker_set_ids: Vec<String>,
    pub marker_count: usize,
    pub well_count: usize,
    pub wells_created: usize,
    pub well_results: Vec<WellMarkerResult>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WellMarkerResult {
    pub well_name: String,
    pub well_id: Option<String>,
    pub created: bool,
    pub marker_count: usize,
}

/// Parse a marker CSV file for ingestion
///
/// Returns column detection results and statistics for the wizard UI.
#[tauri::command]
pub async fn parse_markers_for_ingest(
    file_path: String,
    _state: State<'_, Mutex<AppState>>,
) -> Result<ParseMarkerResponse, String> {
    info!("Parsing marker CSV: {}", file_path);

    let path = PathBuf::from(&file_path);
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown.csv")
        .to_string();

    // Read file
    let raw_bytes = std::fs::read(&path).map_err(|e| format!("Failed to read file: {}", e))?;

    // Parse CSV
    let parsed = parse_marker_csv(Cursor::new(&raw_bytes))
        .map_err(|e| format!("Failed to parse CSV: {}", e))?;

    // Check for required columns
    let has_marker_name = parsed.columns.iter().any(|c| {
        c.detected.column_type == MarkerColumnType::MarkerName && c.detected.confidence >= 0.5
    });
    let has_md = parsed.columns.iter().any(|c| {
        c.detected.column_type == MarkerColumnType::MeasuredDepth && c.detected.confidence >= 0.5
    });

    let mut missing_required = Vec::new();
    if !has_marker_name {
        missing_required.push("Marker Name".to_string());
    }
    if !has_md {
        missing_required.push("Measured Depth".to_string());
    }

    let has_required_columns = missing_required.is_empty();

    // Build column info
    let columns: Vec<MarkerColumnInfo> = parsed
        .columns
        .iter()
        .map(|col| MarkerColumnInfo {
            index: col.index,
            header: col.header.clone(),
            // Use Display trait which outputs snake_case matching serde serialization
            detected_type: col.detected.column_type.to_string(),
            confidence: col.detected.confidence,
            unit: col.detected.unit.clone(),
            sample_values: col.sample_values.clone(),
            min_value: col.stats.as_ref().and_then(|s| s.min),
            max_value: col.stats.as_ref().and_then(|s| s.max),
            mean_value: col.stats.as_ref().and_then(|s| s.mean),
        })
        .collect();

    let delimiter = match parsed.delimiter {
        ',' => "comma",
        '\t' => "tab",
        ';' => "semicolon",
        '|' => "pipe",
        ' ' => "space",
        _ => "unknown",
    }
    .to_string();

    let file_id = Uuid::new_v4().to_string();

    Ok(ParseMarkerResponse {
        file_id,
        file_name,
        columns,
        row_count: parsed.rows.len(),
        delimiter,
        has_header: parsed.has_header,
        is_multi_well: parsed.is_multi_well,
        well_names: parsed.well_names,
        warnings: parsed.warnings,
        has_required_columns,
        missing_required,
    })
}

/// Ingest marker CSV files
#[tauri::command]
pub async fn ingest_marker_files(
    request: IngestMarkerRequest,
    state: State<'_, Mutex<AppState>>,
) -> Result<IngestMarkerResponse, String> {
    info!("Ingesting {} marker files", request.file_paths.len());

    // Get app state
    let state = state
        .lock()
        .map_err(|e| format!("Failed to lock state: {}", e))?;

    let data_dir = state
        .data_dir
        .as_ref()
        .ok_or("Data directory not initialized")?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;

    // Initialize blob store
    let blob_dir = data_dir.join("blobs");
    let blob_store =
        BlobStore::new(&blob_dir).map_err(|e| format!("Failed to create blob store: {}", e))?;

    // Get current workspace
    let workspace_id = state
        .session
        .current_workspace_id
        .ok_or("No workspace selected")?;

    // Get workspace member ID for created_by field
    let member_id: Uuid = {
        let token = state.session.token.as_ref().ok_or("Not authenticated")?;

        // Get account_id from session token
        let account_id: String = db
            .query_row(
                "SELECT s.account_id FROM sessions s WHERE s.token_hash = ?",
                params![dataforge_core::auth::hash_token(token)],
                |row| row.get(0),
            )
            .map_err(|e| format!("Failed to get account from session: {}", e))?;

        // Get workspace member ID
        let member_id_str: String = db
            .query_row(
                "SELECT id FROM workspace_members WHERE workspace_id = ? AND account_id = ?",
                params![workspace_id.to_string(), account_id],
                |row| row.get(0),
            )
            .map_err(|e| format!("Failed to get workspace member: {}", e))?;

        Uuid::parse_str(&member_id_str).map_err(|e| format!("Invalid member ID: {}", e))?
    };

    // Build well_mappings HashMap from request
    let well_mappings: HashMap<String, WellMapping> = request
        .well_mappings
        .into_iter()
        .map(|entry| {
            let well_id = entry.well_id.and_then(|id| Uuid::parse_str(&id).ok());
            (
                entry.well_name.clone(),
                WellMapping {
                    well_name: entry.well_name,
                    well_id,
                    create_new: entry.create_new,
                },
            )
        })
        .collect();

    info!("Well mappings received: {:?}", well_mappings.keys().collect::<Vec<_>>());

    // Build ingest options
    let options = MarkerIngestOptions {
        set_name: request.set_name,
        interpretation_type: request.interpretation_type,
        interpreter: request.interpreter,
        depth_unit: request.depth_unit.unwrap_or_else(|| "ft".to_string()),
        depth_reference: request.depth_reference,
        reference_source: None,
        store_raw_file: true,
        well_match_mode: WellMatchMode::CaseInsensitive,
        auto_create_wells: request.auto_create_wells,
        default_well_id: None,
        allow_unmapped_wells: request.allow_unmapped_wells,
        well_mappings,
    };

    // Process first file
    if request.file_paths.is_empty() {
        return Err("No files provided".to_string());
    }

    let file_path = PathBuf::from(&request.file_paths[0]);

    // Ingest the file
    match ingest_marker_file(
        db,
        &blob_store,
        &file_path,
        workspace_id,
        member_id,
        None,
        &options,
    ) {
        Ok(result) => {
            info!(
                "Successfully ingested markers: {} markers, {} wells",
                result.marker_count, result.well_count
            );

            let well_results: Vec<WellMarkerResult> = result
                .well_results
                .iter()
                .map(|w| WellMarkerResult {
                    well_name: w.well_name.clone(),
                    well_id: w.matched_well_id.map(|id| id.to_string()),
                    created: w.well_created,
                    marker_count: w.markers.len(),
                })
                .collect();

            Ok(IngestMarkerResponse {
                success: true,
                marker_set_ids: result.marker_set_ids.iter().map(|id| id.to_string()).collect(),
                marker_count: result.marker_count,
                well_count: result.well_count,
                wells_created: result.wells_created,
                well_results,
                error: None,
            })
        }
        Err(e) => {
            error!("Failed to ingest markers: {}", e);
            Ok(IngestMarkerResponse {
                success: false,
                marker_set_ids: Vec::new(),
                marker_count: 0,
                well_count: 0,
                wells_created: 0,
                well_results: Vec::new(),
                error: Some(format!("{}", e)),
            })
        }
    }
}

/// Get marker sets for a well
#[tauri::command]
pub async fn get_well_marker_sets(
    well_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<MarkerSetSummary>, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let mut stmt = db
        .prepare(
            r#"
            SELECT
                ms.id,
                ms.source_filename,
                ms.name,
                ms.interpretation_type,
                ms.marker_count,
                ms.min_depth,
                ms.max_depth,
                ms.depth_unit,
                ms.imported_at
            FROM marker_sets ms
            WHERE ms.well_id = ? AND ms.deleted_at IS NULL
            ORDER BY ms.imported_at DESC
            "#,
        )
        .map_err(|e| format!("Query error: {}", e))?;

    let marker_sets = stmt
        .query_map(params![well_id], |row| {
            Ok(MarkerSetSummary {
                id: row.get(0)?,
                source_filename: row.get(1)?,
                name: row.get(2)?,
                interpretation_type: row.get(3)?,
                marker_count: row.get(4)?,
                min_depth: row.get(5)?,
                max_depth: row.get(6)?,
                depth_unit: row.get(7)?,
                imported_at: row.get(8)?,
            })
        })
        .map_err(|e| format!("Query error: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row error: {}", e))?;

    Ok(marker_sets)
}

/// Summary of a marker set
#[derive(Debug, Serialize)]
pub struct MarkerSetSummary {
    pub id: String,
    pub source_filename: String,
    pub name: Option<String>,
    pub interpretation_type: Option<String>,
    pub marker_count: Option<i64>,
    pub min_depth: Option<f64>,
    pub max_depth: Option<f64>,
    pub depth_unit: String,
    pub imported_at: String,
}

/// Get markers for a marker set
#[tauri::command]
pub async fn get_markers(
    marker_set_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<MarkerData>, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let mut stmt = db
        .prepare(
            r#"
            SELECT
                m.id,
                m.name,
                m.measured_depth,
                m.true_vertical_depth,
                m.marker_type,
                m.quality,
                m.comments
            FROM markers m
            WHERE m.marker_set_id = ? AND m.deleted_at IS NULL
            ORDER BY m.measured_depth ASC
            "#,
        )
        .map_err(|e| format!("Query error: {}", e))?;

    let markers = stmt
        .query_map(params![marker_set_id], |row| {
            Ok(MarkerData {
                id: row.get(0)?,
                name: row.get(1)?,
                measured_depth: row.get(2)?,
                tvd: row.get(3)?,
                marker_type: row.get(4)?,
                quality: row.get(5)?,
                comments: row.get(6)?,
            })
        })
        .map_err(|e| format!("Query error: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row error: {}", e))?;

    Ok(markers)
}

/// Marker data for display
#[derive(Debug, Serialize)]
pub struct MarkerData {
    pub id: String,
    pub name: String,
    pub measured_depth: f64,
    pub tvd: Option<f64>,
    pub marker_type: Option<String>,
    pub quality: Option<String>,
    pub comments: Option<String>,
}

/// Get all markers for a well (across all marker sets)
#[tauri::command]
pub async fn get_well_markers(
    well_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<MarkerData>, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let mut stmt = db
        .prepare(
            r#"
            SELECT
                m.id,
                m.name,
                m.measured_depth,
                m.true_vertical_depth,
                m.marker_type,
                m.quality,
                m.comments
            FROM markers m
            WHERE m.well_id = ? AND m.deleted_at IS NULL
            ORDER BY m.measured_depth ASC
            "#,
        )
        .map_err(|e| format!("Query error: {}", e))?;

    let markers = stmt
        .query_map(params![well_id], |row| {
            Ok(MarkerData {
                id: row.get(0)?,
                name: row.get(1)?,
                measured_depth: row.get(2)?,
                tvd: row.get(3)?,
                marker_type: row.get(4)?,
                quality: row.get(5)?,
                comments: row.get(6)?,
            })
        })
        .map_err(|e| format!("Query error: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row error: {}", e))?;

    Ok(markers)
}
