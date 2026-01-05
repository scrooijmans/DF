//! Trajectory Commands
//!
//! Tauri commands for trajectory/survey data ingestion from CSV files.
//! Follows OSDU patterns for trajectory stations with input (MD, INC, AZI) and
//! calculated (TVD, NS, EW, DLS) columns.

use dataforge_core::blob::BlobStore;
use dataforge_core::models::{CalculationMethod, SurveyType, TrajectoryColumnType};
use dataforge_core::trajectory::{
    ingest_trajectory_file, parse_trajectory_csv, read_parquet_column_values, ColumnMapping,
    TrajectoryIngestOptions,
};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;
use tracing::{error, info};
use uuid::Uuid;

use crate::state::AppState;

/// Response type for trajectory CSV parsing
#[derive(Debug, Serialize, Deserialize)]
pub struct ParseTrajectoryResponse {
    /// Unique identifier for this parse result
    pub file_id: String,
    /// Original file name
    pub file_name: String,
    /// Detected columns with their types
    pub columns: Vec<TrajectoryColumnInfo>,
    /// Number of data rows
    pub row_count: usize,
    /// Detected delimiter
    pub delimiter: String,
    /// Parsing warnings
    pub warnings: Vec<String>,
    /// Whether required columns (MD, INC, AZI) were found
    pub has_required_columns: bool,
    /// Missing required column names
    pub missing_required: Vec<String>,
}

/// Information about a column in the trajectory CSV
#[derive(Debug, Serialize, Deserialize)]
pub struct TrajectoryColumnInfo {
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
    /// Whether this is an input column (MD, INC, AZI)
    pub is_input: bool,
    /// Whether this is a calculated column (TVD, NS, EW, DLS)
    pub is_calculated: bool,
    /// Statistics
    pub row_count: usize,
    pub null_count: usize,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub mean_value: Option<f64>,
}

/// Request for processing trajectory CSV files
#[derive(Debug, Deserialize)]
pub struct IngestTrajectoryRequest {
    /// File paths to process
    pub file_paths: Vec<String>,
    /// Target well ID (if adding to existing well)
    pub target_well_id: Option<String>,
    /// Create new well with this name
    pub new_well_name: Option<String>,
    /// Column configurations (type overrides)
    pub column_configs: Vec<TrajectoryColumnConfig>,
    /// Survey metadata
    pub survey_type: Option<String>,
    pub survey_company: Option<String>,
    pub magnetic_declination: Option<f64>,
    pub grid_convergence: Option<f64>,
    /// Depth unit (ft or m)
    pub md_unit: Option<String>,
    /// Angle unit (deg or rad)
    pub angle_unit: Option<String>,
}

/// Configuration for a trajectory column
#[derive(Debug, Deserialize)]
pub struct TrajectoryColumnConfig {
    /// Column index
    pub index: usize,
    /// Override column type
    pub column_type: Option<String>,
    /// Override unit
    pub unit: Option<String>,
}

/// Response from trajectory ingestion
#[derive(Debug, Serialize)]
pub struct IngestTrajectoryResponse {
    pub success: bool,
    pub survey_run_id: Option<String>,
    pub well_id: Option<String>,
    pub well_created: bool,
    pub station_count: usize,
    pub column_count: usize,
    pub top_md: f64,
    pub bottom_md: f64,
    pub max_inclination: f64,
    pub max_dls: f64,
    pub error: Option<String>,
}

/// Parse a trajectory CSV file for ingestion
///
/// Returns column detection results and statistics for the wizard UI.
#[tauri::command]
pub async fn parse_trajectory_for_ingest(
    file_path: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<ParseTrajectoryResponse, String> {
    info!("Parsing trajectory CSV: {}", file_path);

    let path = PathBuf::from(&file_path);
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown.csv")
        .to_string();

    // Read file
    let raw_bytes = std::fs::read(&path).map_err(|e| format!("Failed to read file: {}", e))?;

    // Parse CSV
    let parsed = parse_trajectory_csv(Cursor::new(&raw_bytes))
        .map_err(|e| format!("Failed to parse CSV: {}", e))?;

    // Check for required columns
    let mut has_md = false;
    let mut has_inc = false;
    let mut has_azi = false;

    for col in &parsed.columns {
        if col.detected.confidence >= 0.5 {
            match col.detected.column_type {
                TrajectoryColumnType::MeasuredDepth => has_md = true,
                TrajectoryColumnType::Inclination => has_inc = true,
                TrajectoryColumnType::Azimuth => has_azi = true,
                _ => {}
            }
        }
    }

    let mut missing_required = Vec::new();
    if !has_md {
        missing_required.push("Measured Depth (MD)".to_string());
    }
    if !has_inc {
        missing_required.push("Inclination (INC)".to_string());
    }
    if !has_azi {
        missing_required.push("Azimuth (AZI)".to_string());
    }

    let has_required_columns = missing_required.is_empty();

    // Build column info
    let columns: Vec<TrajectoryColumnInfo> = parsed
        .columns
        .iter()
        .map(|col| TrajectoryColumnInfo {
            index: col.index,
            header: col.header.clone(),
            // Use Display trait which outputs snake_case matching serde serialization
            detected_type: col.detected.column_type.to_string(),
            confidence: col.detected.confidence,
            unit: col.detected.unit.clone(),
            is_input: col.detected.column_type.is_input(),
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

    let file_id = Uuid::new_v4().to_string();

    Ok(ParseTrajectoryResponse {
        file_id,
        file_name,
        columns,
        row_count: parsed.rows.len(),
        delimiter,
        warnings: parsed.warnings,
        has_required_columns,
        missing_required,
    })
}

/// Ingest trajectory CSV files into a well
#[tauri::command]
pub async fn ingest_trajectory_files(
    request: IngestTrajectoryRequest,
    state: State<'_, Mutex<AppState>>,
) -> Result<IngestTrajectoryResponse, String> {
    info!(
        "Ingesting {} trajectory files",
        request.file_paths.len()
    );

    // Get app state
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;

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

    // Parse well ID if provided
    let well_id = if let Some(ref id) = request.target_well_id {
        Some(Uuid::parse_str(id).map_err(|e| format!("Invalid well ID: {}", e))?)
    } else {
        None
    };

    // Build column mappings from configs
    let column_mappings: Vec<ColumnMapping> = request
        .column_configs
        .iter()
        .filter_map(|cfg| {
            cfg.column_type.as_ref().map(|ct| {
                let column_type = match ct.as_str() {
                    "md" | "measured_depth" => TrajectoryColumnType::MeasuredDepth,
                    "inc" | "inclination" => TrajectoryColumnType::Inclination,
                    "azi" | "azimuth" => TrajectoryColumnType::Azimuth,
                    "tvd" | "true_vertical_depth" => TrajectoryColumnType::TrueVerticalDepth,
                    "ns" | "north_south" => TrajectoryColumnType::NorthSouth,
                    "ew" | "east_west" => TrajectoryColumnType::EastWest,
                    "dls" | "dogleg_severity" => TrajectoryColumnType::DoglegSeverity,
                    "dx" | "delta_x" => TrajectoryColumnType::DeltaX,
                    "dy" | "delta_y" => TrajectoryColumnType::DeltaY,
                    "dz" | "delta_z" => TrajectoryColumnType::DeltaZ,
                    _ => TrajectoryColumnType::Unknown,
                };
                ColumnMapping {
                    index: cfg.index,
                    column_type,
                    unit: cfg.unit.clone(),
                }
            })
        })
        .collect();

    // Build ingest options
    let survey_type = request.survey_type.as_ref().and_then(|s| match s.as_str() {
        "definitive" => Some(SurveyType::Definitive),
        "mwd" => Some(SurveyType::Mwd),
        "gyro" => Some(SurveyType::Gyro),
        "preliminary" => Some(SurveyType::Preliminary),
        "composite" => Some(SurveyType::Composite),
        _ => None,
    });

    let md_unit = request.md_unit.clone().unwrap_or_else(|| "ft".to_string());
    let dls_unit_length = if md_unit == "m" { 30.0 } else { 100.0 };

    let options = TrajectoryIngestOptions {
        survey_type,
        survey_company: request.survey_company.clone(),
        magnetic_declination: request.magnetic_declination,
        grid_convergence: request.grid_convergence,
        source_crs: None,
        calculation_method: CalculationMethod::MinimumCurvature,
        tie_in: None,
        store_raw_file: true,
        dls_unit_length,
        md_unit,
        angle_unit: request.angle_unit.unwrap_or_else(|| "deg".to_string()),
    };

    // Process first file (for now, single file support)
    if request.file_paths.is_empty() {
        return Err("No files provided".to_string());
    }

    let file_path = PathBuf::from(&request.file_paths[0]);
    let mappings = if column_mappings.is_empty() {
        None
    } else {
        Some(column_mappings.as_slice())
    };

    // Ingest the file
    match ingest_trajectory_file(
        &db,
        &blob_store,
        &file_path,
        well_id,
        request.new_well_name.as_deref(),
        workspace_id,
        member_id,
        mappings,
        &options,
    ) {
        Ok(result) => {
            info!(
                "Successfully ingested trajectory: {} stations, {} columns",
                result.station_count,
                result.columns.len()
            );

            Ok(IngestTrajectoryResponse {
                success: true,
                survey_run_id: Some(result.survey_run_id.to_string()),
                well_id: Some(result.well_id.to_string()),
                well_created: result.well_created,
                station_count: result.station_count,
                column_count: result.columns.len(),
                top_md: result.top_md,
                bottom_md: result.bottom_md,
                max_inclination: result.max_inclination,
                max_dls: result.max_dls,
                error: None,
            })
        }
        Err(e) => {
            error!("Failed to ingest trajectory: {}", e);
            Ok(IngestTrajectoryResponse {
                success: false,
                survey_run_id: None,
                well_id: None,
                well_created: false,
                station_count: 0,
                column_count: 0,
                top_md: 0.0,
                bottom_md: 0.0,
                max_inclination: 0.0,
                max_dls: 0.0,
                error: Some(format!("{}", e)),
            })
        }
    }
}

/// Get trajectories for a well
#[tauri::command]
pub async fn get_well_trajectories(
    well_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<TrajectorySummary>, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let mut stmt = db
        .prepare(
            r#"
            SELECT
                sr.id,
                sr.source_filename,
                sr.survey_type,
                sr.calculation_method,
                sr.top_md,
                sr.bottom_md,
                sr.station_count,
                sr.md_unit,
                sr.imported_at
            FROM survey_runs sr
            WHERE sr.well_id = ? AND sr.deleted_at IS NULL
            ORDER BY sr.imported_at DESC
            "#,
        )
        .map_err(|e| format!("Query error: {}", e))?;

    let trajectories = stmt
        .query_map(params![well_id], |row| {
            Ok(TrajectorySummary {
                id: row.get(0)?,
                source_filename: row.get(1)?,
                survey_type: row.get(2)?,
                calculation_method: row.get(3)?,
                top_md: row.get(4)?,
                bottom_md: row.get(5)?,
                station_count: row.get(6)?,
                md_unit: row.get(7)?,
                imported_at: row.get(8)?,
            })
        })
        .map_err(|e| format!("Query error: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row error: {}", e))?;

    Ok(trajectories)
}

/// Summary of a trajectory/survey run
#[derive(Debug, Serialize)]
pub struct TrajectorySummary {
    pub id: String,
    pub source_filename: String,
    pub survey_type: Option<String>,
    pub calculation_method: Option<String>,
    pub top_md: Option<f64>,
    pub bottom_md: Option<f64>,
    pub station_count: Option<i64>,
    pub md_unit: String,
    pub imported_at: String,
}

/// Get trajectory station data for visualization
#[tauri::command]
pub async fn get_trajectory_stations(
    survey_run_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<TrajectoryStationData, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let data_dir = state
        .data_dir
        .as_ref()
        .ok_or("Data directory not initialized")?;

    let blob_dir = data_dir.join("blobs");
    let blob_store =
        BlobStore::new(&blob_dir).map_err(|e| format!("Failed to create blob store: {}", e))?;

    // Get column parquet hashes
    let mut stmt = db
        .prepare(
            r#"
            SELECT column_type, parquet_hash
            FROM trajectory_columns
            WHERE survey_run_id = ?
            "#,
        )
        .map_err(|e| format!("Query error: {}", e))?;

    let column_hashes: Vec<(String, String)> = stmt
        .query_map(params![survey_run_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| format!("Query error: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row error: {}", e))?;

    // Read each column from parquet
    let mut md_values = Vec::new();
    let mut inc_values = Vec::new();
    let mut azi_values = Vec::new();
    let mut tvd_values = Vec::new();
    let mut ns_values = Vec::new();
    let mut ew_values = Vec::new();
    let mut dls_values = Vec::new();

    for (col_type, hash) in column_hashes {
        let data = blob_store
            .get(&hash)
            .map_err(|e| format!("Blob read error for {}: {}", hash, e))?;

        let values = read_parquet_column_values(&data)
            .map_err(|e| format!("Failed to read parquet: {}", e))?;

        match col_type.as_str() {
            "md" => md_values = values,
            "inclination" => inc_values = values,
            "azimuth" => azi_values = values,
            "tvd" => tvd_values = values,
            "ns" => ns_values = values,
            "ew" => ew_values = values,
            "dls" => dls_values = values,
            _ => {}
        }
    }

    Ok(TrajectoryStationData {
        md: md_values,
        inclination: inc_values,
        azimuth: azi_values,
        tvd: tvd_values,
        ns: ns_values,
        ew: ew_values,
        dls: dls_values,
    })
}

/// Trajectory station data for visualization
#[derive(Debug, Serialize)]
pub struct TrajectoryStationData {
    pub md: Vec<f64>,
    pub inclination: Vec<f64>,
    pub azimuth: Vec<f64>,
    pub tvd: Vec<f64>,
    pub ns: Vec<f64>,
    pub ew: Vec<f64>,
    pub dls: Vec<f64>,
}
