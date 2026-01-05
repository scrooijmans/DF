//! Checkshot Commands
//!
//! Tauri commands for checkshot (time-depth) data ingestion from CSV files.
//! Follows OSDU patterns for checkshot stations with MD, TVD, TWT columns.

use dataforge_core::blob::BlobStore;
use dataforge_core::models::CheckshotColumnType;
use dataforge_core::checkshots::{
    ingest_checkshot_file, parse_checkshot_csv, CheckshotColumnMapping,
    CheckshotIngestOptions,
};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;
use tracing::info;
use uuid::Uuid;

use crate::state::AppState;

/// Response type for checkshot CSV parsing
#[derive(Debug, Serialize, Deserialize)]
pub struct ParseCheckResponse {
    /// Unique identifier for this parse result
    pub file_id: String,
    /// Original file name
    pub file_name: String,
    /// Detected columns with their types
    pub columns: Vec<CheckColumnInfo>,
    /// Number of data rows
    pub row_count: usize,
    /// Detected delimiter
    pub delimiter: String,
    /// Parsing warnings
    pub warnings: Vec<String>,
    /// Whether required columns (MD, TVD, TWT) were found
    pub has_required_columns: bool,
    /// Missing required column names
    pub missing_required: Vec<String>,
}

/// Information about a column in the checkshot CSV
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckColumnInfo {
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
    /// Whether this is a required input column (MD, TVD, TWT)
    pub is_required: bool,
    /// Whether this is a calculated column (velocity)
    pub is_calculated: bool,
    /// Statistics
    pub row_count: usize,
    pub null_count: usize,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub mean_value: Option<f64>,
}

/// Request for processing checkshot CSV files
#[derive(Debug, Deserialize)]
pub struct IngestCheckRequest {
    /// File paths to process
    pub file_paths: Vec<String>,
    /// Target well ID (if adding to existing well)
    pub well_id: Option<String>,
    /// Create new well with this name
    pub well_name: Option<String>,
    /// Whether to create a new well
    pub create_well: bool,
    /// Column configurations (type overrides)
    pub column_configs: Vec<CheckColumnConfig>,
    /// Survey type
    pub survey_type: Option<String>,
    /// Survey company
    pub survey_company: Option<String>,
    /// Depth unit (ft or m)
    pub depth_unit: String,
    /// Time unit (s or ms)
    pub time_unit: String,
    /// Datum reference
    pub datum: Option<String>,
    /// Whether to calculate interval velocity
    pub calculate_velocity: bool,
}

/// Configuration for a checkshot column
#[derive(Debug, Deserialize)]
pub struct CheckColumnConfig {
    /// Column index
    pub index: usize,
    /// Override column type
    pub column_type: String,
}

/// Response from checkshot ingestion
#[derive(Debug, Serialize)]
pub struct IngestCheckResponse {
    pub success: bool,
    pub checkshot_run_id: Option<String>,
    pub well_id: Option<String>,
    pub well_created: bool,
    pub station_count: usize,
    pub column_count: usize,
    pub min_md: f64,
    pub max_md: f64,
    pub min_tvd: f64,
    pub max_tvd: f64,
    pub min_twt: f64,
    pub max_twt: f64,
    pub error: Option<String>,
}

/// Parse a checkshot CSV file for ingestion
///
/// Returns column detection results and statistics for the wizard UI.
#[tauri::command]
pub async fn parse_checkshot_for_ingest(
    file_path: String,
    _state: State<'_, Mutex<AppState>>,
) -> Result<ParseCheckResponse, String> {
    info!("Parsing checkshot CSV: {}", file_path);

    let path = PathBuf::from(&file_path);
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown.csv")
        .to_string();

    // Read file
    let raw_bytes = std::fs::read(&path).map_err(|e| format!("Failed to read file: {}", e))?;

    // Parse CSV
    let parsed = parse_checkshot_csv(Cursor::new(&raw_bytes))
        .map_err(|e| format!("Failed to parse CSV: {}", e))?;

    // Check for required columns
    let mut has_md = false;
    let mut has_tvd = false;
    let mut has_twt = false;

    for col in &parsed.columns {
        if col.detected.confidence >= 0.5 {
            match col.detected.column_type {
                CheckshotColumnType::MeasuredDepth => has_md = true,
                CheckshotColumnType::TrueVerticalDepth => has_tvd = true,
                CheckshotColumnType::TwoWayTime => has_twt = true,
                _ => {}
            }
        }
    }

    let mut missing_required = Vec::new();
    if !has_md {
        missing_required.push("Measured Depth (MD)".to_string());
    }
    if !has_tvd {
        missing_required.push("True Vertical Depth (TVD)".to_string());
    }
    if !has_twt {
        missing_required.push("Two-Way Time (TWT)".to_string());
    }

    let has_required_columns = missing_required.is_empty();

    // Build column info
    let columns: Vec<CheckColumnInfo> = parsed
        .columns
        .iter()
        .map(|col| CheckColumnInfo {
            index: col.index,
            header: col.header.clone(),
            // Use Display trait which outputs snake_case matching serde serialization
            detected_type: col.detected.column_type.to_string(),
            confidence: col.detected.confidence,
            unit: col.detected.unit.clone(),
            is_required: col.detected.column_type.is_required(),
            is_calculated: col.detected.column_type.is_calculated(),
            row_count: col.stats.count,
            null_count: col.stats.null_count,
            min_value: col.stats.min,
            max_value: col.stats.max,
            mean_value: col.stats.mean,
        })
        .collect();

    let delimiter = match parsed.delimiter {
        ',' => "comma",
        '\t' => "tab",
        ';' => "semicolon",
        '|' => "pipe",
        _ => "unknown",
    }
    .to_string();

    Ok(ParseCheckResponse {
        file_id: Uuid::new_v4().to_string(),
        file_name,
        columns,
        row_count: parsed.rows.len(),
        delimiter,
        warnings: parsed.warnings,
        has_required_columns,
        missing_required,
    })
}

/// Ingest checkshot CSV files into the database
#[tauri::command]
pub async fn ingest_checkshot_files(
    request: IngestCheckRequest,
    state: State<'_, Mutex<AppState>>,
) -> Result<IngestCheckResponse, String> {
    info!(
        "Ingesting {} checkshot files",
        request.file_paths.len()
    );

    // Get workspace and member IDs
    let (workspace_id, member_id, data_dir) = {
        let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

        let workspace_id = state
            .session
            .current_workspace_id
            .ok_or("No workspace selected")?;

        let db = state.db.as_ref().ok_or("Database not initialized")?;
        let token = state.session.token.as_ref().ok_or("Not authenticated")?;

        // Get account ID from session
        let account_id: String = db
            .query_row(
                "SELECT s.account_id FROM sessions s WHERE s.token_hash = ?",
                params![dataforge_core::auth::hash_token(token)],
                |row| row.get(0),
            )
            .map_err(|e| format!("Failed to get account: {}", e))?;

        // Get member ID
        let member_id_str: String = db
            .query_row(
                "SELECT id FROM workspace_members WHERE workspace_id = ? AND account_id = ?",
                params![workspace_id.to_string(), account_id],
                |row| row.get(0),
            )
            .map_err(|e| format!("Failed to get member: {}", e))?;

        let member_id =
            Uuid::parse_str(&member_id_str).map_err(|e| format!("Invalid member ID: {}", e))?;

        let data_dir = state
            .data_dir
            .clone()
            .ok_or("Data directory not initialized")?;

        (workspace_id, member_id, data_dir)
    };

    let db_path = data_dir.join("dataforge.db");
    let blob_dir = data_dir.join("blobs");

    // Process each file
    let mut total_station_count = 0;
    let mut total_column_count = 0;
    let mut last_checkshot_run_id = None;
    let mut last_well_id = None;
    let mut well_created = false;
    let mut min_md = f64::MAX;
    let mut max_md = f64::MIN;
    let mut min_tvd = f64::MAX;
    let mut max_tvd = f64::MIN;
    let mut min_twt = f64::MAX;
    let mut max_twt = f64::MIN;

    for file_path in &request.file_paths {
        let path = PathBuf::from(file_path);

        // Open a fresh connection for each file
        let db = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        let blob_store = BlobStore::new(&blob_dir)
            .map_err(|e| format!("Failed to open blob store: {}", e))?;

        // Build column mappings
        let column_mappings: Vec<CheckshotColumnMapping> = request
            .column_configs
            .iter()
            .filter_map(|cfg| {
                let col_type: CheckshotColumnType = cfg.column_type.parse().ok()?;
                Some(CheckshotColumnMapping {
                    index: cfg.index,
                    column_type: col_type,
                })
            })
            .collect();

        // Build ingest options
        let options = CheckshotIngestOptions {
            survey_type: request.survey_type.clone(),
            survey_company: request.survey_company.clone(),
            survey_date: None,
            depth_unit: request.depth_unit.clone(),
            time_unit: request.time_unit.clone(),
            datum: request.datum.clone(),
            datum_elevation: None,
            store_raw_file: true,
            calculate_velocity: request.calculate_velocity,
            column_mappings,
        };

        // Determine well ID
        let well_id = if let Some(ref id) = request.well_id {
            Some(Uuid::parse_str(id).map_err(|e| format!("Invalid well ID: {}", e))?)
        } else {
            None
        };

        // Ingest the file
        let result = ingest_checkshot_file(
            &db,
            &blob_store,
            &path,
            workspace_id,
            well_id,
            request.well_name.as_deref(),
            member_id,
            options,
        )
        .map_err(|e| format!("Failed to ingest file: {}", e))?;

        total_station_count += result.station_count;
        total_column_count += result.column_count;
        last_checkshot_run_id = Some(result.checkshot_run_id);
        last_well_id = Some(result.well_id);
        well_created = well_created || result.well_created;

        min_md = min_md.min(result.min_md);
        max_md = max_md.max(result.max_md);
        min_tvd = min_tvd.min(result.min_tvd);
        max_tvd = max_tvd.max(result.max_tvd);
        min_twt = min_twt.min(result.min_twt);
        max_twt = max_twt.max(result.max_twt);
    }

    Ok(IngestCheckResponse {
        success: true,
        checkshot_run_id: last_checkshot_run_id.map(|id| id.to_string()),
        well_id: last_well_id.map(|id| id.to_string()),
        well_created,
        station_count: total_station_count,
        column_count: total_column_count,
        min_md,
        max_md,
        min_tvd,
        max_tvd,
        min_twt,
        max_twt,
        error: None,
    })
}

/// Get checkshot runs for a well
#[tauri::command]
pub async fn get_well_checkshots(
    well_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<CheckshotSummary>, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let mut stmt = db
        .prepare(
            r#"SELECT id, name, survey_type, station_count, min_md, max_md,
                      min_twt, max_twt, depth_unit, time_unit, imported_at
               FROM checkshot_runs
               WHERE well_id = ? AND deleted_at IS NULL
               ORDER BY imported_at DESC"#,
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let runs = stmt
        .query_map(params![well_id], |row| {
            Ok(CheckshotSummary {
                id: row.get(0)?,
                name: row.get(1)?,
                survey_type: row.get(2)?,
                station_count: row.get(3)?,
                min_md: row.get(4)?,
                max_md: row.get(5)?,
                min_twt: row.get(6)?,
                max_twt: row.get(7)?,
                depth_unit: row.get(8)?,
                time_unit: row.get(9)?,
                imported_at: row.get(10)?,
            })
        })
        .map_err(|e| format!("Failed to execute query: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(runs)
}

/// Summary of a checkshot run
#[derive(Debug, Serialize)]
pub struct CheckshotSummary {
    pub id: String,
    pub name: Option<String>,
    pub survey_type: Option<String>,
    pub station_count: Option<i64>,
    pub min_md: Option<f64>,
    pub max_md: Option<f64>,
    pub min_twt: Option<f64>,
    pub max_twt: Option<f64>,
    pub depth_unit: String,
    pub time_unit: String,
    pub imported_at: String,
}

/// Get checkshot data for a run
#[tauri::command]
pub async fn get_checkshot_data(
    checkshot_run_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<CheckshotData, String> {
    let data_dir = {
        let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
        state
            .data_dir
            .clone()
            .ok_or("Data directory not initialized")?
    };

    let db_path = data_dir.join("dataforge.db");
    let blob_dir = data_dir.join("blobs");

    let db = rusqlite::Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    let blob_store = BlobStore::new(&blob_dir)
        .map_err(|e| format!("Failed to open blob store: {}", e))?;

    // Get checkshot run metadata
    let (name, survey_type, station_count, depth_unit, time_unit): (
        Option<String>,
        Option<String>,
        Option<i64>,
        String,
        String,
    ) = db
        .query_row(
            r#"SELECT name, survey_type, station_count, depth_unit, time_unit
               FROM checkshot_runs WHERE id = ?"#,
            params![checkshot_run_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
        )
        .map_err(|e| format!("Failed to get checkshot run: {}", e))?;

    // Get column data
    let mut stmt = db
        .prepare(
            r#"SELECT column_type, column_name, unit, parquet_hash
               FROM checkshot_columns
               WHERE checkshot_run_id = ? AND deleted_at IS NULL"#,
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let mut columns = Vec::new();
    let rows = stmt
        .query_map(params![checkshot_run_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, Option<String>>(3)?,
            ))
        })
        .map_err(|e| format!("Failed to execute query: {}", e))?;

    for row in rows {
        let (col_type, col_name, unit, parquet_hash) =
            row.map_err(|e| format!("Row error: {}", e))?;

        let values = if let Some(hash) = parquet_hash {
            let data = blob_store
                .get(&hash)
                .map_err(|e| format!("Failed to get blob: {}", e))?;
            dataforge_core::trajectory::read_parquet_column_values(&data)
                .map_err(|e| format!("Failed to read parquet: {}", e))?
        } else {
            Vec::new()
        };

        columns.push(CheckshotColumnData {
            column_type: col_type,
            column_name: col_name,
            unit,
            values,
        });
    }

    Ok(CheckshotData {
        id: checkshot_run_id,
        name,
        survey_type,
        station_count: station_count.unwrap_or(0) as usize,
        depth_unit,
        time_unit,
        columns,
    })
}

/// Full checkshot data for a run
#[derive(Debug, Serialize)]
pub struct CheckshotData {
    pub id: String,
    pub name: Option<String>,
    pub survey_type: Option<String>,
    pub station_count: usize,
    pub depth_unit: String,
    pub time_unit: String,
    pub columns: Vec<CheckshotColumnData>,
}

/// Column data for a checkshot run
#[derive(Debug, Serialize)]
pub struct CheckshotColumnData {
    pub column_type: String,
    pub column_name: String,
    pub unit: Option<String>,
    pub values: Vec<f64>,
}

/// Delete a checkshot run
#[tauri::command]
pub async fn delete_checkshot_run(
    checkshot_run_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<bool, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    // Soft delete the run and its columns
    db.execute(
        "UPDATE checkshot_runs SET deleted_at = datetime('now') WHERE id = ?",
        params![checkshot_run_id],
    )
    .map_err(|e| format!("Failed to delete checkshot run: {}", e))?;

    db.execute(
        "UPDATE checkshot_columns SET deleted_at = datetime('now') WHERE checkshot_run_id = ?",
        params![checkshot_run_id],
    )
    .map_err(|e| format!("Failed to delete checkshot columns: {}", e))?;

    info!("Deleted checkshot run: {}", checkshot_run_id);
    Ok(true)
}
