//! Checkshot Ingestion
//!
//! Handles the complete ingestion workflow for checkshot CSV files:
//! 1. Parse CSV with column detection
//! 2. Validate data
//! 3. Store raw file in blob storage
//! 4. Create Parquet files for each column
//! 5. Create database records

use crate::blob::BlobStore;
use crate::checkshots::parser::{parse_checkshot_csv, ParsedCheckColumn, ParsedCheckshot};
use crate::error::{Error, Result};
use crate::models::CheckshotColumnType;

use arrow::array::Float64Array;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use rusqlite::{params, Connection, OptionalExtension};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

/// Column mapping override from user
#[derive(Debug, Clone)]
pub struct CheckshotColumnMapping {
    /// Column index in the CSV
    pub index: usize,
    /// User-assigned column type (overrides auto-detection)
    pub column_type: CheckshotColumnType,
}

/// Options for checkshot ingestion
#[derive(Debug, Clone)]
pub struct CheckshotIngestOptions {
    /// Survey type (checkshot, vsp, sonic_log, other)
    pub survey_type: Option<String>,
    /// Survey company name
    pub survey_company: Option<String>,
    /// Survey date
    pub survey_date: Option<String>,
    /// Depth unit (ft or m)
    pub depth_unit: String,
    /// Time unit (s or ms)
    pub time_unit: String,
    /// Datum reference (kb, msl, ground_level)
    pub datum: Option<String>,
    /// Datum elevation
    pub datum_elevation: Option<f64>,
    /// Whether to store the raw CSV file
    pub store_raw_file: bool,
    /// Whether to calculate interval velocity
    pub calculate_velocity: bool,
    /// Column type overrides
    pub column_mappings: Vec<CheckshotColumnMapping>,
}

impl Default for CheckshotIngestOptions {
    fn default() -> Self {
        Self {
            survey_type: Some("checkshot".to_string()),
            survey_company: None,
            survey_date: None,
            depth_unit: "ft".to_string(),
            time_unit: "s".to_string(),
            datum: None,
            datum_elevation: None,
            store_raw_file: true,
            calculate_velocity: false,
            column_mappings: Vec::new(),
        }
    }
}

/// Result of checkshot ingestion
#[derive(Debug, Clone)]
pub struct CheckshotIngestResult {
    /// ID of the created checkshot run
    pub checkshot_run_id: Uuid,
    /// ID of the well
    pub well_id: Uuid,
    /// Whether a new well was created
    pub well_created: bool,
    /// Number of stations ingested
    pub station_count: usize,
    /// Number of columns ingested
    pub column_count: usize,
    /// Minimum MD value
    pub min_md: f64,
    /// Maximum MD value
    pub max_md: f64,
    /// Minimum TVD value
    pub min_tvd: f64,
    /// Maximum TVD value
    pub max_tvd: f64,
    /// Minimum TWT value
    pub min_twt: f64,
    /// Maximum TWT value
    pub max_twt: f64,
}

/// Ingest a checkshot CSV file
pub fn ingest_checkshot_file(
    conn: &Connection,
    blob_store: &BlobStore,
    file_path: &Path,
    workspace_id: Uuid,
    well_id: Option<Uuid>,
    well_name: Option<&str>,
    member_id: Uuid,
    options: CheckshotIngestOptions,
) -> Result<CheckshotIngestResult> {
    // 1. Read and hash the file
    let file_content = fs::read(file_path)
        .map_err(|e| Error::Io(e))?;
    let file_hash = compute_hash(&file_content);
    let filename = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown.csv")
        .to_string();

    // 2. Parse the CSV
    let parsed = parse_checkshot_csv(std::io::Cursor::new(&file_content))
        .map_err(|e| Error::InvalidData(e))?;

    // 3. Apply column mappings
    let columns = apply_column_mappings(&parsed.columns, &options.column_mappings);

    // 4. Get required column indices
    let (md_idx, tvd_idx, twt_idx) = get_required_column_indices(&columns)?;

    // 5. Extract data and compute statistics
    let md_values = &columns[md_idx].values;
    let tvd_values = &columns[tvd_idx].values;
    let twt_values = &columns[twt_idx].values;

    let station_count = md_values.len();
    let (min_md, max_md) = compute_min_max(md_values);
    let (min_tvd, max_tvd) = compute_min_max(tvd_values);
    let (min_twt, max_twt) = compute_min_max(twt_values);

    // 6. Get or create well
    let (well_id, well_created) = get_or_create_well(
        conn,
        workspace_id,
        well_id,
        well_name,
        member_id,
    )?;

    // 7. Store raw file in blob store (optional)
    let raw_file_blob_hash = if options.store_raw_file {
        let hash = blob_store.store(&file_content)?;
        register_blob(conn, &hash, file_content.len())?;
        Some(hash)
    } else {
        None
    };

    // 8. Create checkshot_run record
    let checkshot_run_id = Uuid::new_v4();
    conn.execute(
        r#"INSERT INTO checkshot_runs (
            id, well_id, workspace_id, source_filename, source_file_hash, raw_file_blob_hash,
            name, survey_type, survey_company, survey_date,
            min_md, max_md, min_tvd, max_tvd, min_twt, max_twt, station_count,
            depth_unit, time_unit, datum, datum_elevation,
            imported_by
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        params![
            checkshot_run_id.to_string(),
            well_id.to_string(),
            workspace_id.to_string(),
            filename,
            file_hash,
            raw_file_blob_hash,
            well_name.or(Some(&filename)), // Use filename as name if not provided
            options.survey_type,
            options.survey_company,
            options.survey_date,
            min_md,
            max_md,
            min_tvd,
            max_tvd,
            min_twt,
            max_twt,
            station_count as i64,
            options.depth_unit,
            options.time_unit,
            options.datum,
            options.datum_elevation,
            member_id.to_string(),
        ],
    )?;

    // 9. Create Parquet files and checkshot_column records for each column
    let mut column_count = 0;
    for col in &columns {
        if col.detected.column_type == CheckshotColumnType::Unknown {
            continue;
        }

        let parquet_data = create_column_parquet(&col.header, &col.values)?;
        let parquet_hash = blob_store.store(&parquet_data)?;
        register_blob(conn, &parquet_hash, parquet_data.len())?;

        let column_id = Uuid::new_v4();
        let is_input = col.detected.column_type.is_required() as i32;
        let is_calculated = col.detected.column_type.is_calculated() as i32;

        conn.execute(
            r#"INSERT INTO checkshot_columns (
                id, checkshot_run_id, well_id, column_type, column_name, unit,
                is_input, is_calculated,
                min_value, max_value, mean_value, null_count, sample_count,
                parquet_hash, created_by
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            params![
                column_id.to_string(),
                checkshot_run_id.to_string(),
                well_id.to_string(),
                col.detected.column_type.to_string(),
                col.header,
                col.detected.unit,
                is_input,
                is_calculated,
                col.stats.min,
                col.stats.max,
                col.stats.mean,
                col.stats.null_count as i64,
                col.stats.count as i64,
                parquet_hash,
                member_id.to_string(),
            ],
        )?;

        column_count += 1;
    }

    // 10. Calculate and store interval velocity (optional)
    if options.calculate_velocity {
        if let Some(velocity_data) = calculate_interval_velocity(md_values, tvd_values, twt_values, &options.time_unit) {
            let parquet_data = create_column_parquet("VINT", &velocity_data)?;
            let parquet_hash = blob_store.store(&parquet_data)?;
            register_blob(conn, &parquet_hash, parquet_data.len())?;

            let (min_vel, max_vel) = compute_min_max(&velocity_data);
            let mean_vel = velocity_data.iter()
                .filter_map(|v| *v)
                .sum::<f64>() / velocity_data.iter().filter(|v| v.is_some()).count().max(1) as f64;
            let null_count = velocity_data.iter().filter(|v| v.is_none()).count();

            let column_id = Uuid::new_v4();
            conn.execute(
                r#"INSERT INTO checkshot_columns (
                    id, checkshot_run_id, well_id, column_type, column_name, unit,
                    is_input, is_calculated,
                    min_value, max_value, mean_value, null_count, sample_count,
                    parquet_hash, created_by
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
                params![
                    column_id.to_string(),
                    checkshot_run_id.to_string(),
                    well_id.to_string(),
                    "velocity",
                    "VINT",
                    format!("{}/{}", options.depth_unit, options.time_unit),
                    0, // is_input
                    1, // is_calculated
                    min_vel,
                    max_vel,
                    mean_vel,
                    null_count as i64,
                    velocity_data.len() as i64,
                    parquet_hash,
                    member_id.to_string(),
                ],
            )?;

            column_count += 1;
        }
    }

    // 11. Update well depth range
    update_well_depth_range(conn, well_id, min_md, max_md)?;

    Ok(CheckshotIngestResult {
        checkshot_run_id,
        well_id,
        well_created,
        station_count,
        column_count,
        min_md,
        max_md,
        min_tvd,
        max_tvd,
        min_twt,
        max_twt,
    })
}

/// Apply user column mappings to override auto-detection
fn apply_column_mappings(
    columns: &[ParsedCheckColumn],
    mappings: &[CheckshotColumnMapping],
) -> Vec<ParsedCheckColumn> {
    columns
        .iter()
        .map(|col| {
            let mut col = col.clone();
            if let Some(mapping) = mappings.iter().find(|m| m.index == col.index) {
                col.detected.column_type = mapping.column_type;
                col.detected.confidence = 1.0; // User override has full confidence
            }
            col
        })
        .collect()
}

/// Get indices of required columns (MD, TVD, TWT)
fn get_required_column_indices(columns: &[ParsedCheckColumn]) -> Result<(usize, usize, usize)> {
    let md_idx = columns
        .iter()
        .position(|c| c.detected.column_type == CheckshotColumnType::MeasuredDepth)
        .ok_or_else(|| Error::InvalidData("Missing Measured Depth (MD) column".to_string()))?;

    let tvd_idx = columns
        .iter()
        .position(|c| c.detected.column_type == CheckshotColumnType::TrueVerticalDepth)
        .ok_or_else(|| Error::InvalidData("Missing True Vertical Depth (TVD) column".to_string()))?;

    let twt_idx = columns
        .iter()
        .position(|c| c.detected.column_type == CheckshotColumnType::TwoWayTime)
        .ok_or_else(|| Error::InvalidData("Missing Two-Way Time (TWT) column".to_string()))?;

    Ok((md_idx, tvd_idx, twt_idx))
}

/// Compute min and max from values
fn compute_min_max(values: &[Option<f64>]) -> (f64, f64) {
    let valid: Vec<f64> = values.iter().filter_map(|v| *v).collect();
    if valid.is_empty() {
        return (0.0, 0.0);
    }
    let min = valid.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = valid.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    (min, max)
}

/// Get or create a well record
fn get_or_create_well(
    conn: &Connection,
    workspace_id: Uuid,
    well_id: Option<Uuid>,
    well_name: Option<&str>,
    member_id: Uuid,
) -> Result<(Uuid, bool)> {
    // If well_id provided, verify it exists
    if let Some(id) = well_id {
        let exists: Option<String> = conn
            .query_row(
                "SELECT id FROM wells WHERE id = ? AND workspace_id = ?",
                params![id.to_string(), workspace_id.to_string()],
                |row| row.get(0),
            )
            .optional()?;

        if exists.is_some() {
            return Ok((id, false));
        }
    }

    // Create new well
    let new_well_id = Uuid::new_v4();
    let name = well_name.unwrap_or("Unnamed Well");

    conn.execute(
        r#"INSERT INTO wells (id, workspace_id, name, created_by)
           VALUES (?, ?, ?, ?)"#,
        params![
            new_well_id.to_string(),
            workspace_id.to_string(),
            name,
            member_id.to_string(),
        ],
    )?;

    Ok((new_well_id, true))
}

/// Update well depth range based on checkshot data
fn update_well_depth_range(conn: &Connection, well_id: Uuid, min_md: f64, max_md: f64) -> Result<()> {
    conn.execute(
        r#"UPDATE wells SET
           min_depth = CASE WHEN min_depth IS NULL OR ? < min_depth THEN ? ELSE min_depth END,
           max_depth = CASE WHEN max_depth IS NULL OR ? > max_depth THEN ? ELSE max_depth END,
           updated_at = datetime('now')
           WHERE id = ?"#,
        params![min_md, min_md, max_md, max_md, well_id.to_string()],
    )?;
    Ok(())
}

/// Create a Parquet file for a column
fn create_column_parquet(column_name: &str, values: &[Option<f64>]) -> Result<Vec<u8>> {
    let schema = Arc::new(Schema::new(vec![
        Field::new(column_name, DataType::Float64, true),
    ]));

    let array = Float64Array::from(values.to_vec());
    let batch = RecordBatch::try_new(schema.clone(), vec![Arc::new(array)])
        .map_err(|e| Error::InvalidData(format!("Failed to create record batch: {}", e)))?;

    let mut buffer = Vec::new();
    {
        let mut writer = ArrowWriter::try_new(&mut buffer, schema, None)
            .map_err(|e| Error::InvalidData(format!("Failed to create parquet writer: {}", e)))?;

        writer
            .write(&batch)
            .map_err(|e| Error::InvalidData(format!("Failed to write parquet batch: {}", e)))?;

        writer
            .close()
            .map_err(|e| Error::InvalidData(format!("Failed to close parquet writer: {}", e)))?;
    }

    Ok(buffer)
}

/// Calculate interval velocity from TVD and TWT
fn calculate_interval_velocity(
    _md_values: &[Option<f64>],
    tvd_values: &[Option<f64>],
    twt_values: &[Option<f64>],
    time_unit: &str,
) -> Option<Vec<Option<f64>>> {
    if tvd_values.len() < 2 {
        return None;
    }

    let mut velocities: Vec<Option<f64>> = Vec::with_capacity(tvd_values.len());
    velocities.push(None); // First station has no interval velocity

    for i in 1..tvd_values.len() {
        let vel = match (tvd_values[i], tvd_values[i - 1], twt_values[i], twt_values[i - 1]) {
            (Some(tvd2), Some(tvd1), Some(twt2), Some(twt1)) => {
                let delta_tvd = tvd2 - tvd1;
                let mut delta_twt = twt2 - twt1;

                // Convert time to seconds if in milliseconds
                if time_unit == "ms" {
                    delta_twt /= 1000.0;
                }

                // TWT is two-way, so one-way time is TWT/2
                // Velocity = distance / time = delta_tvd / (delta_twt / 2) = 2 * delta_tvd / delta_twt
                if delta_twt.abs() > 0.0001 {
                    Some(2.0 * delta_tvd / delta_twt)
                } else {
                    None
                }
            }
            _ => None,
        };
        velocities.push(vel);
    }

    Some(velocities)
}

/// Compute SHA256 hash of data
fn compute_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Register a blob in the blob_registry table
fn register_blob(conn: &Connection, hash: &str, size: usize) -> Result<()> {
    conn.execute(
        r#"INSERT OR IGNORE INTO blob_registry (hash, size_bytes) VALUES (?, ?)"#,
        params![hash, size as i64],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_test_db() -> (Connection, BlobStore, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let blob_dir = temp_dir.path().join("blobs");
        std::fs::create_dir_all(&blob_dir).unwrap();

        let conn = crate::db::open_memory_db().unwrap();

        let blob_store = BlobStore::new(&blob_dir).unwrap();

        (conn, blob_store, temp_dir)
    }

    #[test]
    fn test_calculate_interval_velocity() {
        let tvd = vec![Some(0.0), Some(100.0), Some(200.0), Some(300.0)];
        let twt = vec![Some(0.0), Some(0.05), Some(0.10), Some(0.15)];

        let velocities = calculate_interval_velocity(&[], &tvd, &twt, "s").unwrap();

        assert_eq!(velocities.len(), 4);
        assert!(velocities[0].is_none()); // First has no velocity

        // Velocity = 2 * delta_tvd / delta_twt
        // = 2 * 100 / 0.05 = 4000
        assert!((velocities[1].unwrap() - 4000.0).abs() < 1.0);
    }

    #[test]
    fn test_compute_min_max() {
        let values = vec![Some(10.0), Some(20.0), None, Some(5.0), Some(30.0)];
        let (min, max) = compute_min_max(&values);
        assert!((min - 5.0).abs() < 0.001);
        assert!((max - 30.0).abs() < 0.001);
    }

    #[test]
    fn test_create_column_parquet() {
        let values = vec![Some(1.0), Some(2.0), None, Some(4.0)];
        let parquet_data = create_column_parquet("test_column", &values).unwrap();
        assert!(!parquet_data.is_empty());
    }
}
