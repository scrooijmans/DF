//! Ingest Commands
//!
//! Tauri commands for data ingestion, including LAS file parsing and processing.
//! Based on patterns from CKAN DataPusher, Dataverse ingest, and Open Data Cube.

use dataforge_core::blob::BlobStore;
use dataforge_core::las::parse_las_file as core_parse_las;
use dataforge_core::models::MainCurveType;
use dataforge_core::parquet::ParquetOptions;
use dataforge_core::units::detect_unit_id;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::State;
use tracing::{error, info, warn};

use crate::state::AppState;

/// Response type for LAS file parsing
#[derive(Debug, Serialize, Deserialize)]
pub struct ParseLasResponse {
    /// Unique identifier for this parse result
    pub file_id: String,
    /// Original file name
    pub file_name: String,
    /// LAS file version
    pub version: String,
    /// Extracted well metadata
    pub well_metadata: WellMetadataResponse,
    /// Detected curves with their metadata
    pub curves: Vec<CurveInfo>,
    /// Number of data rows
    pub row_count: usize,
}

/// Well metadata extracted from LAS file
#[derive(Debug, Serialize, Deserialize)]
pub struct WellMetadataResponse {
    pub well_name: String,
    pub uwi: Option<String>,
    pub company: Option<String>,
    pub field: Option<String>,
    pub location: Option<String>,
    pub start_depth: f64,
    pub stop_depth: f64,
    pub step: f64,
    pub depth_unit: String,
    pub null_value: f64,
}

/// Information about a curve in the LAS file
#[derive(Debug, Serialize, Deserialize)]
pub struct CurveInfo {
    /// Original mnemonic from LAS file
    pub mnemonic: String,
    /// Unit from LAS file
    pub unit: String,
    /// Description from LAS file
    pub description: String,
    /// Detected main curve type
    pub detected_type: String,
    /// Detection confidence (0.0 - 1.0)
    pub confidence: f64,
    /// Whether to include this curve in import
    pub include: bool,
    /// Statistics
    pub row_count: usize,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
}

/// Request for processing LAS files
#[derive(Debug, Deserialize)]
pub struct IngestLasRequest {
    /// File paths to process
    pub file_paths: Vec<String>,
    /// Target well ID (if adding to existing well)
    pub target_well_id: Option<String>,
    /// Create new well with this name
    pub new_well_name: Option<String>,
    /// Curve configurations (which to include, mapped names, etc.)
    pub curve_configs: Vec<CurveConfig>,
}

/// Configuration for a single curve
#[derive(Debug, Deserialize)]
pub struct CurveConfig {
    /// Original mnemonic
    pub mnemonic: String,
    /// Mapped name (standardized name)
    pub mapped_name: Option<String>,
    /// Mapped curve type (e.g., GR, RHOB, NPHI)
    pub mapped_type: Option<String>,
    /// Target unit for conversion
    pub target_unit: Option<String>,
    /// Whether to include this curve
    pub include: bool,
}

/// Response from ingestion
#[derive(Debug, Serialize)]
pub struct IngestResponse {
    /// Whether ingestion succeeded
    pub success: bool,
    /// Number of files processed
    pub files_processed: usize,
    /// Number of curves imported
    pub curves_imported: usize,
    /// Number of data points imported
    pub data_points: usize,
    /// Created well ID (if new well was created)
    pub well_id: Option<String>,
    /// Path to generated Parquet file(s)
    pub parquet_paths: Vec<String>,
    /// Any errors encountered
    pub errors: Vec<String>,
}

fn curve_type_to_string(ct: &MainCurveType) -> String {
    match ct {
        MainCurveType::DEPTH => "DEPTH".to_string(),
        MainCurveType::GR => "GR".to_string(),
        MainCurveType::RHOB => "RHOB".to_string(),
        MainCurveType::NPHI => "NPHI".to_string(),
        MainCurveType::RT => "RT".to_string(),
        MainCurveType::CALI => "CALI".to_string(),
        MainCurveType::DT => "DT".to_string(),
        MainCurveType::SP => "SP".to_string(),
        MainCurveType::PE => "PE".to_string(),
        MainCurveType::OTHER => "OTHER".to_string(),
    }
}

fn get_confidence(ct: &MainCurveType) -> f64 {
    match ct {
        MainCurveType::OTHER => 0.0,
        _ => 0.9,
    }
}

/// Parse a LAS file and return metadata for confirmation
/// This is an enhanced version that returns detailed curve info for the ingest wizard
#[tauri::command]
pub async fn parse_las_for_ingest(file_path: String) -> Result<ParseLasResponse, String> {
    info!("Parsing LAS file for ingest: {}", file_path);

    let path = PathBuf::from(&file_path);

    // Parse using dataforge-core
    let parsed = core_parse_las(&path)
        .map_err(|e| format!("Failed to parse LAS file: {}", e))?;

    // Build response
    let response = ParseLasResponse {
        file_id: uuid::Uuid::new_v4().to_string(),
        file_name: parsed.filename.clone(),
        version: parsed.version.clone(),
        well_metadata: WellMetadataResponse {
            well_name: parsed.well_name.clone().unwrap_or_else(|| parsed.filename.clone()),
            uwi: parsed.uwi.clone(),
            company: parsed.company.clone(),
            field: parsed.field.clone(),
            location: parsed.location.clone(),
            start_depth: parsed.start_depth,
            stop_depth: parsed.stop_depth,
            step: parsed.step,
            depth_unit: "M".to_string(), // TODO: detect from LAS
            null_value: parsed.null_value,
        },
        curves: parsed
            .curves
            .iter()
            .map(|c| CurveInfo {
                mnemonic: c.mnemonic.clone(),
                unit: c.unit.clone(),
                description: c.description.clone(),
                detected_type: curve_type_to_string(&c.detected_type),
                confidence: get_confidence(&c.detected_type),
                include: true,
                row_count: c.row_count,
                min_value: c.min_value,
                max_value: c.max_value,
            })
            .collect(),
        row_count: parsed.curves.first().map_or(0, |c| c.row_count),
    };

    info!(
        "Parsed LAS file: {} curves, {} rows",
        response.curves.len(),
        response.row_count
    );
    Ok(response)
}

/// Parse multiple LAS files for ingest
#[tauri::command]
pub async fn parse_las_files_for_ingest(file_paths: Vec<String>) -> Result<Vec<ParseLasResponse>, String> {
    info!("Parsing {} LAS files for ingest", file_paths.len());

    let mut results = Vec::new();
    let mut errors = Vec::new();

    for path in file_paths {
        match parse_las_for_ingest(path.clone()).await {
            Ok(response) => results.push(response),
            Err(e) => {
                error!("Failed to parse {}: {}", path, e);
                errors.push(format!("{}: {}", path, e));
            }
        }
    }

    if results.is_empty() && !errors.is_empty() {
        return Err(format!("Failed to parse any files: {}", errors.join("; ")));
    }

    Ok(results)
}

/// Ingest LAS files into the database
/// Uses the new dual-storage schema with native and gridded Parquet files
#[tauri::command]
pub async fn ingest_las_files(
    request: IngestLasRequest,
    state: State<'_, Mutex<AppState>>,
) -> Result<IngestResponse, String> {
    info!("Ingesting {} LAS files", request.file_paths.len());

    let mut errors = Vec::new();
    let mut total_curves = 0;
    let mut total_data_points = 0;
    let mut parquet_paths = Vec::new();
    let mut well_id = None;

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

    // Get current workspace and user
    let workspace_id = state
        .session
        .current_workspace_id
        .ok_or("No workspace selected")?
        .to_string();

    // Get workspace member ID for created_by field
    let member_id: String = {
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
        db.query_row(
            "SELECT id FROM workspace_members WHERE workspace_id = ? AND account_id = ?",
            params![workspace_id, account_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Failed to get workspace member: {}", e))?
    };

    // Process each LAS file
    for file_path in &request.file_paths {
        info!("Processing LAS file: {}", file_path);

        // 1. Parse LAS file
        let path = PathBuf::from(file_path);
        let parsed = match core_parse_las(&path) {
            Ok(p) => p,
            Err(e) => {
                let msg = format!("Failed to parse {}: {}", file_path, e);
                error!("{}", msg);
                errors.push(msg);
                continue;
            }
        };

        // 2. Create or get well
        let current_well_id = if let Some(ref target_id) = request.target_well_id {
            // Use existing well
            target_id.clone()
        } else {
            // Create new well with depth grid configuration
            let new_well_id = uuid::Uuid::new_v4().to_string();
            let well_name = request
                .new_well_name
                .clone()
                .or_else(|| parsed.well_name.clone())
                .unwrap_or_else(|| parsed.filename.clone());

            // Detect depth unit from LAS file
            let depth_unit = detect_depth_unit(&parsed);

            db.execute(
                r#"
                INSERT INTO wells (id, workspace_id, name, uwi, field, company, location,
                                   depth_unit, depth_step, depth_origin, min_depth, max_depth, created_by)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
                "#,
                params![
                    new_well_id,
                    workspace_id,
                    well_name,
                    parsed.uwi,
                    parsed.field,
                    parsed.company,
                    parsed.location,
                    depth_unit,
                    0.5_f64, // Default 0.5 ft step
                    0.0_f64, // Default origin
                    parsed.start_depth,
                    parsed.stop_depth,
                    member_id,
                ],
            )
            .map_err(|e| format!("Failed to create well: {}", e))?;

            info!("Created new well: {} ({})", well_name, new_well_id);

            // Create default wellbore (OSDU pattern: Well + Wellbore separation)
            let wellbore_id = uuid::Uuid::new_v4().to_string();
            db.execute(
                r#"
                INSERT INTO wellbores (id, workspace_id, well_id, name, wellbore_number, status)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
                params![
                    wellbore_id,
                    workspace_id,
                    new_well_id,
                    "Main Bore",
                    1_i32,
                    "active",
                ],
            )
            .map_err(|e| format!("Failed to create default wellbore: {}", e))?;

            info!("Created default wellbore: {}", wellbore_id);

            new_well_id
        };

        well_id = Some(current_well_id.clone());

        // 3. Create log_run record for this LAS file
        let log_run_id = uuid::Uuid::new_v4().to_string();
        let source_filename = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown.las".to_string());

        db.execute(
            r#"
            INSERT INTO log_runs (id, well_id, workspace_id, source_filename,
                                  original_top_depth, original_bottom_depth, original_step,
                                  original_depth_unit, las_version, imported_by)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
            params![
                log_run_id,
                current_well_id,
                workspace_id,
                source_filename,
                parsed.start_depth,
                parsed.stop_depth,
                parsed.step,
                detect_depth_unit(&parsed),
                &parsed.version,
                member_id,
            ],
        )
        .map_err(|e| format!("Failed to create log_run: {}", e))?;

        info!("Created log_run: {}", log_run_id);

        // 4. Process each curve
        for curve in &parsed.curves {
            // Check if curve should be included based on config
            let include = request
                .curve_configs
                .iter()
                .find(|c| c.mnemonic == curve.mnemonic)
                .map(|c| c.include)
                .unwrap_or(true);

            if !include {
                continue;
            }

            let curve_id = uuid::Uuid::new_v4().to_string();

            // Get curve config for this mnemonic
            let curve_config = request
                .curve_configs
                .iter()
                .find(|c| c.mnemonic == curve.mnemonic);

            // Use mapped_type if specified, otherwise fall back to detected_type
            let curve_type = curve_config
                .and_then(|c| c.mapped_type.clone())
                .unwrap_or_else(|| curve_type_to_string(&curve.detected_type));

            // Look up property_id from curve_mnemonics table (PWLS-style mapping)
            let property_id: Option<String> = db
                .query_row(
                    "SELECT property_id FROM curve_mnemonics WHERE mnemonic = ? ORDER BY priority DESC LIMIT 1",
                    params![curve.mnemonic],
                    |row| row.get(0),
                )
                .ok()
                .flatten();

            // Use actual depth values from the LAS file's DEPT curve
            // This preserves gaps and irregular spacing per LAS standard
            let depths: Vec<f64> = if !parsed.actual_depths.is_empty() {
                // Verify length matches curve data
                if parsed.actual_depths.len() != curve.values.len() {
                    warn!(
                        "Depth array length ({}) doesn't match curve {} length ({})",
                        parsed.actual_depths.len(),
                        curve.mnemonic,
                        curve.values.len()
                    );
                }
                parsed.actual_depths.clone()
            } else {
                // Fallback: reconstruct from header only if no actual depths available
                // This should rarely happen with valid LAS files
                warn!(
                    "No actual depth values found in LAS file, reconstructing from header for curve {}",
                    curve.mnemonic
                );
                if parsed.step.abs() > 1e-9 {
                    let mut d = Vec::new();
                    let mut current = parsed.start_depth;
                    let step = if parsed.start_depth < parsed.stop_depth { parsed.step.abs() } else { -parsed.step.abs() };
                    while (step > 0.0 && current <= parsed.stop_depth) || (step < 0.0 && current >= parsed.stop_depth) {
                        d.push(current);
                        current += step;
                    }
                    d.truncate(curve.values.len());
                    while d.len() < curve.values.len() {
                        d.push(d.last().copied().unwrap_or(0.0) + step);
                    }
                    d
                } else {
                    (0..curve.values.len())
                        .map(|i| parsed.start_depth + (parsed.stop_depth - parsed.start_depth) * i as f64 / curve.values.len().max(1) as f64)
                        .collect()
                }
            };

            // Create native Parquet file for this curve
            let parquet_options = ParquetOptions::default();
            let native_parquet = dataforge_core::parquet::create_native_parquet(
                &dataforge_core::parquet::NativeCurveData {
                    mnemonic: curve.mnemonic.clone(),
                    depths: depths.clone(),
                    values: curve.values.clone(),
                },
                &parquet_options,
            ).map_err(|e| format!("Failed to create native parquet for {}: {}", curve.mnemonic, e))?;

            // Store native Parquet in blob store
            let native_hash = blob_store.store(&native_parquet)
                .map_err(|e| format!("Failed to store native parquet: {}", e))?;

            // Register blob
            db.execute(
                "INSERT OR IGNORE INTO blob_registry (hash, size_bytes) VALUES (?1, ?2)",
                params![native_hash, native_parquet.len() as i64],
            ).map_err(|e| format!("Failed to register blob: {}", e))?;

            parquet_paths.push(blob_store.get_path(&native_hash).to_string_lossy().to_string());

            // Calculate statistics
            let valid_values: Vec<f64> = curve.values.iter()
                .filter(|v| !v.is_nan() && v.is_finite())
                .copied()
                .collect();

            let (min_val, max_val, mean_val) = if valid_values.is_empty() {
                (None, None, None)
            } else {
                let min = valid_values.iter().cloned().fold(f64::INFINITY, f64::min);
                let max = valid_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let mean = valid_values.iter().sum::<f64>() / valid_values.len() as f64;
                (Some(min), Some(max), Some(mean))
            };

            let null_count = curve.values.iter().filter(|v| v.is_nan()).count() as i64;

            // Detect unit_id from the curve's unit string using the Unit Service
            let unit_id: Option<String> = detect_unit_id(db, &curve.unit)
                .ok()
                .flatten();

            // Get null value from LAS file (parsed.null_value)
            let null_value = parsed.null_value;

            // Get service company from log_run if available
            // For now, we extract from parsed metadata if available
            let service_company: Option<&str> = None; // Could be extracted from LAS ~W section

            // Determine quality flag based on data characteristics
            // "raw" = standard raw data
            // "raw_irregular" = raw data with irregular depth spacing (gaps present)
            let quality_flag = if parsed.has_irregular_spacing {
                "raw_irregular"
            } else {
                "raw"
            };

            // Calculate actual depth range from the depth array
            let actual_top_depth = depths.iter()
                .filter(|d| !d.is_nan())
                .copied()
                .reduce(f64::min)
                .unwrap_or(parsed.start_depth);
            let actual_bottom_depth = depths.iter()
                .filter(|d| !d.is_nan())
                .copied()
                .reduce(f64::max)
                .unwrap_or(parsed.stop_depth);

            // Insert curve with new schema including quality fields
            db.execute(
                r#"
                INSERT INTO curves (id, log_run_id, well_id, mnemonic, property_id, unit, unit_id, description,
                                   native_top_depth, native_bottom_depth, native_step, native_sample_count,
                                   min_value, max_value, mean_value, null_count,
                                   null_value, quality_flag, service_company,
                                   native_parquet_hash, created_by)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)
                "#,
                params![
                    curve_id,
                    log_run_id,
                    current_well_id,
                    curve.mnemonic,
                    property_id,
                    curve.unit,
                    unit_id,
                    curve.description,
                    actual_top_depth,
                    actual_bottom_depth,
                    parsed.step,
                    curve.row_count as i64,
                    min_val,
                    max_val,
                    mean_val,
                    null_count,
                    null_value,
                    quality_flag,
                    service_company,
                    native_hash,
                    member_id,
                ],
            )
            .map_err(|e| format!("Failed to create curve {}: {}", curve.mnemonic, e))?;

            total_curves += 1;
            total_data_points += curve.row_count;

            info!(
                "Created curve: {} ({} rows, hash: {})",
                curve.mnemonic, curve.row_count, &native_hash[..8]
            );
        }
    }

    let success = errors.is_empty();
    let files_processed = request.file_paths.len() - errors.len();

    info!(
        "Ingestion complete: {} files, {} curves, {} data points",
        files_processed, total_curves, total_data_points
    );

    Ok(IngestResponse {
        success,
        files_processed,
        curves_imported: total_curves,
        data_points: total_data_points,
        well_id,
        parquet_paths,
        errors,
    })
}

/// Detect depth unit from parsed LAS data
fn detect_depth_unit(parsed: &dataforge_core::las::ParsedLasFile) -> &'static str {
    // Check if there's a DEPT or DEPTH curve and look at its unit
    for curve in &parsed.curves {
        let mnemonic_upper = curve.mnemonic.to_uppercase();
        if mnemonic_upper == "DEPT" || mnemonic_upper == "DEPTH" || mnemonic_upper == "MD" {
            let unit_upper = curve.unit.to_uppercase();
            if unit_upper.contains('M') && !unit_upper.contains("FT") {
                return "m";
            }
            return "ft";
        }
    }
    // Default to feet
    "ft"
}

/// Get supported file types for ingestion
#[tauri::command]
pub fn get_supported_ingest_types() -> Vec<FileTypeInfo> {
    vec![
        FileTypeInfo {
            id: "las".to_string(),
            name: "LAS Files".to_string(),
            extensions: vec![".las".to_string(), ".LAS".to_string()],
            description: "Well log data in LAS 2.0/3.0 format".to_string(),
            available: true,
        },
        FileTypeInfo {
            id: "csv".to_string(),
            name: "CSV Files".to_string(),
            extensions: vec![".csv".to_string(), ".CSV".to_string()],
            description: "Comma-separated values".to_string(),
            available: false,
        },
        FileTypeInfo {
            id: "excel".to_string(),
            name: "Excel Files".to_string(),
            extensions: vec![
                ".xlsx".to_string(),
                ".xls".to_string(),
                ".XLSX".to_string(),
                ".XLS".to_string(),
            ],
            description: "Microsoft Excel spreadsheets".to_string(),
            available: false,
        },
        FileTypeInfo {
            id: "parquet".to_string(),
            name: "Parquet Files".to_string(),
            extensions: vec![".parquet".to_string(), ".PARQUET".to_string()],
            description: "Apache Parquet columnar format".to_string(),
            available: false,
        },
    ]
}

#[derive(Debug, Serialize)]
pub struct FileTypeInfo {
    pub id: String,
    pub name: String,
    pub extensions: Vec<String>,
    pub description: String,
    pub available: bool,
}

/// Well summary for selection in ingest wizard
#[derive(Debug, Serialize)]
pub struct WellSummary {
    pub id: String,
    pub name: String,
    pub uwi: Option<String>,
    pub field: Option<String>,
    pub curve_count: i64,
}

/// Get wells for the current workspace (for well selection in ingest)
#[tauri::command]
pub fn get_workspace_wells(
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<WellSummary>, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;

    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let workspace_id = state
        .session
        .current_workspace_id
        .ok_or("No workspace selected")?
        .to_string();

    let mut stmt = db
        .prepare(
            r#"
            SELECT w.id, w.name, w.uwi, w.field,
                   (SELECT COUNT(*) FROM curves c WHERE c.well_id = w.id AND c.deleted_at IS NULL) as curve_count
            FROM wells w
            WHERE w.workspace_id = ? AND w.deleted_at IS NULL
            ORDER BY w.name ASC
            "#,
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let wells = stmt
        .query_map(params![workspace_id], |row| {
            Ok(WellSummary {
                id: row.get(0)?,
                name: row.get(1)?,
                uwi: row.get(2)?,
                field: row.get(3)?,
                curve_count: row.get(4)?,
            })
        })
        .map_err(|e| format!("Failed to query wells: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(wells)
}

// ============================================================
// INSPECTOR COMMANDS (DBeaver-style SQLite browser)
// ============================================================

/// Tables that should not be modified through the inspector
/// These are system configuration, authentication, sync, and reference data tables
const READONLY_TABLES: &[&str] = &[
    // Reference data (system configuration)
    "measurement_types",
    "units",
    "curve_properties",
    "curve_mnemonics",
    "log_types",
    "acquisition_types",
    // Auth/system
    "accounts",
    "sessions",
    "workspaces",
    "workspace_members",
    "workspace_invites",
    // Sync state
    "sync_state",
    "sync_queue",
    "sync_conflicts",
    // Storage registry
    "blob_registry",
];

/// Check if a table is read-only
fn is_table_readonly(table_name: &str) -> bool {
    READONLY_TABLES.contains(&table_name)
}

/// Table info for the inspector
#[derive(Debug, Serialize)]
pub struct TableInfo {
    pub name: String,
    pub row_count: i64,
    pub is_editable: bool,
}

/// Foreign key info for a column
#[derive(Debug, Clone, Serialize)]
pub struct ForeignKeyInfo {
    pub target_table: String,
    pub target_column: String,
}

/// Column info from the database
#[derive(Debug, Serialize)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub foreign_key: Option<ForeignKeyInfo>,
}

/// Query result row as JSON values
#[derive(Debug, Serialize)]
pub struct QueryResult {
    pub columns: Vec<ColumnInfo>,
    pub rows: Vec<Vec<serde_json::Value>>,
    pub total_rows: i64,
}

/// Get list of tables in the database
#[tauri::command]
pub fn inspector_get_tables(state: State<'_, Mutex<AppState>>) -> Result<Vec<TableInfo>, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let mut tables = Vec::new();

    // Get all tables
    let mut stmt = db
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name")
        .map_err(|e| format!("Failed to query tables: {}", e))?;

    let table_names: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| format!("Failed to get table names: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    // Get row count for each table
    for name in table_names {
        let count: i64 = db
            .query_row(&format!("SELECT COUNT(*) FROM \"{}\"", name), [], |row| {
                row.get(0)
            })
            .unwrap_or(0);

        let is_editable = !is_table_readonly(&name);

        tables.push(TableInfo {
            name,
            row_count: count,
            is_editable,
        });
    }

    Ok(tables)
}

/// Get columns for a table (includes foreign key information)
#[tauri::command]
pub fn inspector_get_columns(
    table_name: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<ColumnInfo>, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    // First, get foreign key info for this table
    let mut fk_map: std::collections::HashMap<String, ForeignKeyInfo> = std::collections::HashMap::new();
    {
        let mut fk_stmt = db
            .prepare(&format!("PRAGMA foreign_key_list(\"{}\")", table_name))
            .map_err(|e| format!("Failed to query foreign keys: {}", e))?;

        // PRAGMA foreign_key_list returns: id, seq, table, from, to, on_update, on_delete, match
        let fk_rows: Vec<(String, String, String)> = fk_stmt
            .query_map([], |row| {
                let target_table: String = row.get(2)?;
                let from_col: String = row.get(3)?;
                let to_col: String = row.get(4)?;
                Ok((from_col, target_table, to_col))
            })
            .map_err(|e| format!("Failed to get foreign keys: {}", e))?
            .filter_map(|r| r.ok())
            .collect();

        for (from_col, target_table, to_col) in fk_rows {
            fk_map.insert(from_col, ForeignKeyInfo {
                target_table,
                target_column: to_col,
            });
        }
    }

    // Now get column info
    let mut stmt = db
        .prepare(&format!("PRAGMA table_info(\"{}\")", table_name))
        .map_err(|e| format!("Failed to query columns: {}", e))?;

    let columns: Vec<ColumnInfo> = stmt
        .query_map([], |row| {
            let name: String = row.get(1)?;
            let data_type: String = row.get(2)?;
            let not_null: i32 = row.get(3)?;
            let pk: i32 = row.get(5)?;
            Ok((name, data_type, not_null == 0, pk > 0))
        })
        .map_err(|e| format!("Failed to get columns: {}", e))?
        .filter_map(|r| r.ok())
        .map(|(name, data_type, nullable, primary_key)| {
            let foreign_key = fk_map.get(&name).cloned();
            ColumnInfo {
                name,
                data_type,
                nullable,
                primary_key,
                foreign_key,
            }
        })
        .collect();

    Ok(columns)
}

/// Query table data with pagination
#[tauri::command]
pub fn inspector_query_table(
    table_name: String,
    offset: i64,
    limit: i64,
    state: State<'_, Mutex<AppState>>,
) -> Result<QueryResult, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    // Get foreign key info for this table
    let mut fk_map: std::collections::HashMap<String, ForeignKeyInfo> = std::collections::HashMap::new();
    {
        let mut fk_stmt = db
            .prepare(&format!("PRAGMA foreign_key_list(\"{}\")", table_name))
            .map_err(|e| format!("Failed to query foreign keys: {}", e))?;

        let fk_rows: Vec<(String, String, String)> = fk_stmt
            .query_map([], |row| {
                let target_table: String = row.get(2)?;
                let from_col: String = row.get(3)?;
                let to_col: String = row.get(4)?;
                Ok((from_col, target_table, to_col))
            })
            .map_err(|e| format!("Failed to get foreign keys: {}", e))?
            .filter_map(|r| r.ok())
            .collect();

        for (from_col, target_table, to_col) in fk_rows {
            fk_map.insert(from_col, ForeignKeyInfo {
                target_table,
                target_column: to_col,
            });
        }
    }

    // Get columns with foreign key info
    let columns: Vec<ColumnInfo> = {
        let mut stmt = db
            .prepare(&format!("PRAGMA table_info(\"{}\")", table_name))
            .map_err(|e| format!("Failed to query columns: {}", e))?;

        let results: Vec<ColumnInfo> = stmt
            .query_map([], |row| {
                let name: String = row.get(1)?;
                let data_type: String = row.get(2)?;
                let not_null: i32 = row.get(3)?;
                let pk: i32 = row.get(5)?;
                Ok((name, data_type, not_null == 0, pk > 0))
            })
            .map_err(|e| format!("Failed to get columns: {}", e))?
            .filter_map(|r| r.ok())
            .map(|(name, data_type, nullable, primary_key)| {
                let foreign_key = fk_map.get(&name).cloned();
                ColumnInfo {
                    name,
                    data_type,
                    nullable,
                    primary_key,
                    foreign_key,
                }
            })
            .collect();
        results
    };

    // Get total count
    let total_rows: i64 = db
        .query_row(&format!("SELECT COUNT(*) FROM \"{}\"", table_name), [], |row| {
            row.get(0)
        })
        .map_err(|e| format!("Failed to count rows: {}", e))?;

    // Query data with pagination
    let query = format!(
        "SELECT * FROM \"{}\" LIMIT {} OFFSET {}",
        table_name, limit, offset
    );

    let mut stmt = db
        .prepare(&query)
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let column_count = columns.len();
    let mut rows = Vec::new();

    let row_iter = stmt
        .query_map([], |row| {
            let mut values = Vec::with_capacity(column_count);
            for i in 0..column_count {
                let value = row.get_ref(i)?;
                let json_value = match value {
                    rusqlite::types::ValueRef::Null => serde_json::Value::Null,
                    rusqlite::types::ValueRef::Integer(i) => serde_json::Value::Number(i.into()),
                    rusqlite::types::ValueRef::Real(f) => {
                        serde_json::Number::from_f64(f)
                            .map(serde_json::Value::Number)
                            .unwrap_or(serde_json::Value::Null)
                    }
                    rusqlite::types::ValueRef::Text(t) => {
                        serde_json::Value::String(String::from_utf8_lossy(t).to_string())
                    }
                    rusqlite::types::ValueRef::Blob(b) => {
                        serde_json::Value::String(format!("<blob {} bytes>", b.len()))
                    }
                };
                values.push(json_value);
            }
            Ok(values)
        })
        .map_err(|e| format!("Failed to query data: {}", e))?;

    for row_result in row_iter {
        if let Ok(row) = row_result {
            rows.push(row);
        }
    }

    Ok(QueryResult {
        columns,
        rows,
        total_rows,
    })
}

/// Update a single cell value in a table
/// Requires primary key(s) to identify the row
#[tauri::command]
pub fn inspector_update_cell(
    table_name: String,
    column_name: String,
    new_value: serde_json::Value,
    primary_keys: Vec<(String, serde_json::Value)>,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    // Check if table is read-only
    if is_table_readonly(&table_name) {
        return Err(format!(
            "Table '{}' is read-only and cannot be modified",
            table_name
        ));
    }

    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    if primary_keys.is_empty() {
        return Err("No primary key provided - cannot safely update row".to_string());
    }

    // Build the WHERE clause from primary keys
    let where_clauses: Vec<String> = primary_keys
        .iter()
        .map(|(col, _)| format!("\"{}\" = ?", col))
        .collect();
    let where_clause = where_clauses.join(" AND ");

    // Build the UPDATE query
    let query = format!(
        "UPDATE \"{}\" SET \"{}\" = ? WHERE {}",
        table_name, column_name, where_clause
    );

    // Convert JSON values to rusqlite params
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    // Add the new value as first param
    match &new_value {
        serde_json::Value::Null => params.push(Box::new(rusqlite::types::Null)),
        serde_json::Value::Bool(b) => params.push(Box::new(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                params.push(Box::new(i));
            } else if let Some(f) = n.as_f64() {
                params.push(Box::new(f));
            } else {
                params.push(Box::new(n.to_string()));
            }
        }
        serde_json::Value::String(s) => params.push(Box::new(s.clone())),
        _ => params.push(Box::new(new_value.to_string())),
    }

    // Add primary key values as params
    for (_, pk_value) in &primary_keys {
        match pk_value {
            serde_json::Value::Null => params.push(Box::new(rusqlite::types::Null)),
            serde_json::Value::Bool(b) => params.push(Box::new(*b)),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    params.push(Box::new(i));
                } else if let Some(f) = n.as_f64() {
                    params.push(Box::new(f));
                } else {
                    params.push(Box::new(n.to_string()));
                }
            }
            serde_json::Value::String(s) => params.push(Box::new(s.clone())),
            _ => params.push(Box::new(pk_value.to_string())),
        }
    }

    // Execute the update
    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let rows_affected = db
        .execute(&query, params_refs.as_slice())
        .map_err(|e| format!("Failed to update cell: {}", e))?;

    if rows_affected == 0 {
        return Err("No rows matched the primary key - update failed".to_string());
    }

    if rows_affected > 1 {
        tracing::warn!(
            "Update affected {} rows (expected 1) for table={}, column={}",
            rows_affected,
            table_name,
            column_name
        );
    }

    Ok(())
}

/// Delete a row from a table using primary key(s)
#[tauri::command]
pub fn inspector_delete_row(
    table_name: String,
    primary_keys: Vec<(String, serde_json::Value)>,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    // Check if table is read-only
    if is_table_readonly(&table_name) {
        return Err(format!(
            "Table '{}' is read-only and cannot be modified",
            table_name
        ));
    }

    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    if primary_keys.is_empty() {
        return Err("No primary key provided - cannot safely delete row".to_string());
    }

    // Build the WHERE clause from primary keys
    let where_clauses: Vec<String> = primary_keys
        .iter()
        .map(|(col, _)| format!("\"{}\" = ?", col))
        .collect();
    let where_clause = where_clauses.join(" AND ");

    // Build the DELETE query
    let query = format!(
        "DELETE FROM \"{}\" WHERE {}",
        table_name, where_clause
    );

    // Convert JSON values to rusqlite params
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    for (_, pk_value) in &primary_keys {
        match pk_value {
            serde_json::Value::Null => params.push(Box::new(rusqlite::types::Null)),
            serde_json::Value::Bool(b) => params.push(Box::new(*b)),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    params.push(Box::new(i));
                } else if let Some(f) = n.as_f64() {
                    params.push(Box::new(f));
                } else {
                    params.push(Box::new(n.to_string()));
                }
            }
            serde_json::Value::String(s) => params.push(Box::new(s.clone())),
            _ => params.push(Box::new(pk_value.to_string())),
        }
    }

    // Execute the delete
    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let rows_affected = db
        .execute(&query, params_refs.as_slice())
        .map_err(|e| format!("Failed to delete row: {}", e))?;

    if rows_affected == 0 {
        return Err("No rows matched the primary key - delete failed".to_string());
    }

    if rows_affected > 1 {
        tracing::warn!(
            "Delete affected {} rows (expected 1) for table={}",
            rows_affected,
            table_name
        );
    }

    Ok(())
}

/// Insert a new row into a table
#[tauri::command]
pub fn inspector_insert_row(
    table_name: String,
    row_data: std::collections::HashMap<String, serde_json::Value>,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    // Check if table is read-only
    if is_table_readonly(&table_name) {
        return Err(format!(
            "Table '{}' is read-only and cannot be modified",
            table_name
        ));
    }

    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    if row_data.is_empty() {
        return Err("No data provided for insert".to_string());
    }

    // Build column list and placeholders
    let columns: Vec<&String> = row_data.keys().collect();
    let column_names = columns
        .iter()
        .map(|c| format!("\"{}\"", c))
        .collect::<Vec<_>>()
        .join(", ");
    let placeholders = columns.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

    // Build the INSERT query
    let query = format!(
        "INSERT INTO \"{}\" ({}) VALUES ({})",
        table_name, column_names, placeholders
    );

    // Convert JSON values to rusqlite params (in same order as columns)
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    for col in &columns {
        let value = &row_data[*col];
        match value {
            serde_json::Value::Null => params.push(Box::new(rusqlite::types::Null)),
            serde_json::Value::Bool(b) => params.push(Box::new(*b)),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    params.push(Box::new(i));
                } else if let Some(f) = n.as_f64() {
                    params.push(Box::new(f));
                } else {
                    params.push(Box::new(n.to_string()));
                }
            }
            serde_json::Value::String(s) => params.push(Box::new(s.clone())),
            _ => params.push(Box::new(value.to_string())),
        }
    }

    // Execute the insert
    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    db.execute(&query, params_refs.as_slice())
        .map_err(|e| format!("Failed to insert row: {}", e))?;

    Ok(())
}

/// Lookup value for foreign key dropdown (id + display value)
#[derive(Debug, Serialize)]
pub struct ForeignKeyLookupValue {
    pub id: serde_json::Value,
    pub display: String,
}

/// Get lookup values for a foreign key column (used for dropdown editors)
/// Returns the primary key value and a display string for each row in the target table
#[tauri::command]
pub fn inspector_get_fk_lookup_values(
    target_table: String,
    target_column: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<ForeignKeyLookupValue>, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    // Get column info to find a good display column (typically 'name', 'title', or the second column)
    let mut display_column = target_column.clone();
    {
        let mut stmt = db
            .prepare(&format!("PRAGMA table_info(\"{}\")", target_table))
            .map_err(|e| format!("Failed to query columns: {}", e))?;

        let columns: Vec<(i32, String)> = stmt
            .query_map([], |row| {
                let cid: i32 = row.get(0)?;
                let name: String = row.get(1)?;
                Ok((cid, name))
            })
            .map_err(|e| format!("Failed to get columns: {}", e))?
            .filter_map(|r| r.ok())
            .collect();

        // Look for common display column names
        for (_, name) in &columns {
            let lower = name.to_lowercase();
            if lower == "name" || lower == "title" || lower == "label" || lower == "display_name" {
                display_column = name.clone();
                break;
            }
        }

        // If no display column found, use the second column if available (first is usually id)
        if display_column == target_column && columns.len() > 1 {
            display_column = columns[1].1.clone();
        }
    }

    // Query the target table for lookup values
    // Limit to 1000 values to prevent huge dropdowns
    let query = if display_column == target_column {
        format!(
            "SELECT \"{}\", \"{}\" FROM \"{}\" ORDER BY \"{}\" LIMIT 1000",
            target_column, target_column, target_table, target_column
        )
    } else {
        format!(
            "SELECT \"{}\", \"{}\" FROM \"{}\" ORDER BY \"{}\" LIMIT 1000",
            target_column, display_column, target_table, display_column
        )
    };

    let mut stmt = db
        .prepare(&query)
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let values: Vec<ForeignKeyLookupValue> = stmt
        .query_map([], |row| {
            let id_value = row.get_ref(0)?;
            let id = match id_value {
                rusqlite::types::ValueRef::Null => serde_json::Value::Null,
                rusqlite::types::ValueRef::Integer(i) => serde_json::Value::Number(i.into()),
                rusqlite::types::ValueRef::Real(f) => {
                    serde_json::Number::from_f64(f)
                        .map(serde_json::Value::Number)
                        .unwrap_or(serde_json::Value::Null)
                }
                rusqlite::types::ValueRef::Text(t) => {
                    serde_json::Value::String(String::from_utf8_lossy(t).to_string())
                }
                rusqlite::types::ValueRef::Blob(b) => {
                    serde_json::Value::String(format!("<blob {} bytes>", b.len()))
                }
            };

            let display_value = row.get_ref(1)?;
            let display = match display_value {
                rusqlite::types::ValueRef::Null => "NULL".to_string(),
                rusqlite::types::ValueRef::Integer(i) => i.to_string(),
                rusqlite::types::ValueRef::Real(f) => f.to_string(),
                rusqlite::types::ValueRef::Text(t) => String::from_utf8_lossy(t).to_string(),
                rusqlite::types::ValueRef::Blob(_) => "<blob>".to_string(),
            };

            Ok(ForeignKeyLookupValue { id, display })
        })
        .map_err(|e| format!("Failed to query lookup values: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(values)
}

// ============================================================
// ORPHANED BLOB CLEANUP
// ============================================================

/// Result of orphaned blob cleanup operation
#[derive(Debug, Serialize)]
pub struct OrphanCleanupResult {
    /// Number of orphaned blobs found
    pub orphan_count: i64,
    /// Number of blobs successfully deleted
    pub deleted_count: i64,
    /// Total bytes freed
    pub bytes_freed: i64,
    /// Any errors encountered (partial success possible)
    pub errors: Vec<String>,
}

/// Find and delete orphaned blobs from blob_registry and filesystem
/// Orphaned blobs are entries in blob_registry that are not referenced by any table
#[tauri::command]
pub fn inspector_cleanup_orphaned_blobs(
    state: State<'_, Mutex<AppState>>,
) -> Result<OrphanCleanupResult, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let data_dir = state.data_dir.as_ref().ok_or("Data directory not set")?;

    // Build the query to find orphaned blobs
    // A blob is orphaned if its hash is not referenced by any table
    let orphan_query = r#"
        SELECT hash, size_bytes FROM blob_registry
        WHERE hash NOT IN (SELECT raw_file_blob_hash FROM log_runs WHERE raw_file_blob_hash IS NOT NULL)
          AND hash NOT IN (SELECT native_parquet_hash FROM curves WHERE native_parquet_hash IS NOT NULL)
          AND hash NOT IN (SELECT gridded_parquet_hash FROM curves WHERE gridded_parquet_hash IS NOT NULL)
          AND hash NOT IN (SELECT native_parquet_hash FROM curve_versions WHERE native_parquet_hash IS NOT NULL)
          AND hash NOT IN (SELECT raw_file_blob_hash FROM survey_runs WHERE raw_file_blob_hash IS NOT NULL)
          AND hash NOT IN (SELECT parquet_hash FROM trajectory_columns WHERE parquet_hash IS NOT NULL)
          AND hash NOT IN (SELECT raw_file_blob_hash FROM checkshot_runs WHERE raw_file_blob_hash IS NOT NULL)
          AND hash NOT IN (SELECT parquet_hash FROM checkshot_columns WHERE parquet_hash IS NOT NULL)
          AND hash NOT IN (SELECT raw_file_blob_hash FROM marker_sets WHERE raw_file_blob_hash IS NOT NULL)
          AND hash NOT IN (SELECT raw_file_blob_hash FROM surfaces WHERE raw_file_blob_hash IS NOT NULL)
          AND hash NOT IN (SELECT data_blob_hash FROM surfaces WHERE data_blob_hash IS NOT NULL)
          AND hash NOT IN (SELECT parquet_hash FROM unified_views WHERE parquet_hash IS NOT NULL)
    "#;

    // Find all orphaned blobs
    let mut stmt = db
        .prepare(orphan_query)
        .map_err(|e| format!("Failed to prepare orphan query: {}", e))?;

    let orphans: Vec<(String, i64)> = stmt
        .query_map([], |row| {
            let hash: String = row.get(0)?;
            let size: i64 = row.get(1)?;
            Ok((hash, size))
        })
        .map_err(|e| format!("Failed to query orphans: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    let orphan_count = orphans.len() as i64;
    let mut deleted_count = 0i64;
    let mut bytes_freed = 0i64;
    let mut errors = Vec::new();

    if orphans.is_empty() {
        return Ok(OrphanCleanupResult {
            orphan_count: 0,
            deleted_count: 0,
            bytes_freed: 0,
            errors: vec![],
        });
    }

    // Get the blobs directory (subdirectory of data_dir)
    let blobs_dir = data_dir.join("blobs");

    // Delete each orphaned blob
    for (hash, size) in orphans {
        // Delete from filesystem first
        let blob_path = blobs_dir.join(format!("{}.parquet", hash));
        if blob_path.exists() {
            if let Err(e) = std::fs::remove_file(&blob_path) {
                errors.push(format!("Failed to delete blob file {}: {}", hash, e));
                continue;
            }
        }

        // Delete from blob_registry
        match db.execute("DELETE FROM blob_registry WHERE hash = ?", params![hash]) {
            Ok(_) => {
                deleted_count += 1;
                bytes_freed += size;
                info!("Deleted orphaned blob: {} ({} bytes)", hash, size);
            }
            Err(e) => {
                errors.push(format!("Failed to delete blob registry entry {}: {}", hash, e));
            }
        }
    }

    info!(
        "Orphan cleanup complete: {} orphans found, {} deleted, {} bytes freed",
        orphan_count, deleted_count, bytes_freed
    );

    Ok(OrphanCleanupResult {
        orphan_count,
        deleted_count,
        bytes_freed,
        errors,
    })
}

/// Preview orphaned blobs without deleting (dry run)
#[tauri::command]
pub fn inspector_preview_orphaned_blobs(
    state: State<'_, Mutex<AppState>>,
) -> Result<OrphanCleanupResult, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    // Same query as cleanup, but only returns counts
    let orphan_query = r#"
        SELECT hash, size_bytes FROM blob_registry
        WHERE hash NOT IN (SELECT raw_file_blob_hash FROM log_runs WHERE raw_file_blob_hash IS NOT NULL)
          AND hash NOT IN (SELECT native_parquet_hash FROM curves WHERE native_parquet_hash IS NOT NULL)
          AND hash NOT IN (SELECT gridded_parquet_hash FROM curves WHERE gridded_parquet_hash IS NOT NULL)
          AND hash NOT IN (SELECT native_parquet_hash FROM curve_versions WHERE native_parquet_hash IS NOT NULL)
          AND hash NOT IN (SELECT raw_file_blob_hash FROM survey_runs WHERE raw_file_blob_hash IS NOT NULL)
          AND hash NOT IN (SELECT parquet_hash FROM trajectory_columns WHERE parquet_hash IS NOT NULL)
          AND hash NOT IN (SELECT raw_file_blob_hash FROM checkshot_runs WHERE raw_file_blob_hash IS NOT NULL)
          AND hash NOT IN (SELECT parquet_hash FROM checkshot_columns WHERE parquet_hash IS NOT NULL)
          AND hash NOT IN (SELECT raw_file_blob_hash FROM marker_sets WHERE raw_file_blob_hash IS NOT NULL)
          AND hash NOT IN (SELECT raw_file_blob_hash FROM surfaces WHERE raw_file_blob_hash IS NOT NULL)
          AND hash NOT IN (SELECT data_blob_hash FROM surfaces WHERE data_blob_hash IS NOT NULL)
          AND hash NOT IN (SELECT parquet_hash FROM unified_views WHERE parquet_hash IS NOT NULL)
    "#;

    let mut stmt = db
        .prepare(orphan_query)
        .map_err(|e| format!("Failed to prepare orphan query: {}", e))?;

    let orphans: Vec<(String, i64)> = stmt
        .query_map([], |row| {
            let hash: String = row.get(0)?;
            let size: i64 = row.get(1)?;
            Ok((hash, size))
        })
        .map_err(|e| format!("Failed to query orphans: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    let orphan_count = orphans.len() as i64;
    let bytes_total: i64 = orphans.iter().map(|(_, size)| size).sum();

    Ok(OrphanCleanupResult {
        orphan_count,
        deleted_count: 0,
        bytes_freed: bytes_total, // Shows potential savings in preview
        errors: vec![],
    })
}

// ============================================================
// HOME PAGE - RECENT ACTIVITY
// ============================================================

/// Recent activity item (well or curve)
#[derive(Debug, Serialize)]
pub struct RecentActivityItem {
    pub id: String,
    pub name: String,
    pub item_type: String, // "well" or "curve"
    pub parent_name: Option<String>, // Well name for curves
    pub parent_id: Option<String>,   // Well ID for curves
    pub size_bytes: Option<i64>,
    pub row_count: Option<i64>,
    pub updated_at: String,
    pub created_at: String,
}

/// Workspace stats for home page
#[derive(Debug, Serialize)]
pub struct WorkspaceStats {
    pub well_count: i64,
    pub curve_count: i64,
    pub total_data_points: i64,
    pub total_size_bytes: i64,
}

/// Get recent activity for the current workspace
#[tauri::command]
pub fn get_recent_activity(
    limit: Option<i64>,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<RecentActivityItem>, String> {
    let limit = limit.unwrap_or(20);
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let workspace_id = state
        .session
        .current_workspace_id
        .ok_or("No workspace selected")?
        .to_string();

    // Query wells and curves, union them together, order by updated_at
    // Updated to use new schema: mnemonic instead of curve_name, native_parquet_hash instead of blob_hash
    let query = r#"
        SELECT
            'well' as item_type,
            w.id,
            w.name,
            NULL as parent_name,
            NULL as parent_id,
            (SELECT SUM(b.size_bytes) FROM curves c
             JOIN blob_registry b ON c.native_parquet_hash = b.hash
             WHERE c.well_id = w.id AND c.deleted_at IS NULL) as size_bytes,
            (SELECT SUM(c.native_sample_count) FROM curves c WHERE c.well_id = w.id AND c.deleted_at IS NULL) as row_count,
            w.updated_at,
            w.created_at
        FROM wells w
        WHERE w.workspace_id = ?1 AND w.deleted_at IS NULL

        UNION ALL

        SELECT
            'curve' as item_type,
            c.id,
            c.mnemonic as name,
            w.name as parent_name,
            w.id as parent_id,
            b.size_bytes,
            c.native_sample_count as row_count,
            c.updated_at,
            c.created_at
        FROM curves c
        JOIN wells w ON c.well_id = w.id
        LEFT JOIN blob_registry b ON c.native_parquet_hash = b.hash
        WHERE w.workspace_id = ?1 AND c.deleted_at IS NULL AND w.deleted_at IS NULL

        ORDER BY updated_at DESC
        LIMIT ?2
    "#;

    let mut stmt = db
        .prepare(query)
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let items = stmt
        .query_map(params![workspace_id, limit], |row| {
            Ok(RecentActivityItem {
                item_type: row.get(0)?,
                id: row.get(1)?,
                name: row.get(2)?,
                parent_name: row.get(3)?,
                parent_id: row.get(4)?,
                size_bytes: row.get(5)?,
                row_count: row.get(6)?,
                updated_at: row.get(7)?,
                created_at: row.get(8)?,
            })
        })
        .map_err(|e| format!("Failed to query recent activity: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(items)
}

/// Get workspace statistics for home page
#[tauri::command]
pub fn get_workspace_stats(
    state: State<'_, Mutex<AppState>>,
) -> Result<WorkspaceStats, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let workspace_id = state
        .session
        .current_workspace_id
        .ok_or("No workspace selected")?
        .to_string();

    // Get well count
    let well_count: i64 = db
        .query_row(
            "SELECT COUNT(*) FROM wells WHERE workspace_id = ? AND deleted_at IS NULL",
            params![workspace_id],
            |row| row.get(0),
        )
        .unwrap_or(0);

    // Get curve count and total data points (using new schema: native_sample_count)
    let (curve_count, total_data_points): (i64, i64) = db
        .query_row(
            r#"
            SELECT COUNT(*), COALESCE(SUM(c.native_sample_count), 0)
            FROM curves c
            JOIN wells w ON c.well_id = w.id
            WHERE w.workspace_id = ? AND c.deleted_at IS NULL AND w.deleted_at IS NULL
            "#,
            params![workspace_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap_or((0, 0));

    // Get total size (using new schema: native_parquet_hash)
    let total_size_bytes: i64 = db
        .query_row(
            r#"
            SELECT COALESCE(SUM(b.size_bytes), 0)
            FROM curves c
            JOIN wells w ON c.well_id = w.id
            JOIN blob_registry b ON c.native_parquet_hash = b.hash
            WHERE w.workspace_id = ? AND c.deleted_at IS NULL AND w.deleted_at IS NULL
            "#,
            params![workspace_id],
            |row| row.get(0),
        )
        .unwrap_or(0);

    Ok(WorkspaceStats {
        well_count,
        curve_count,
        total_data_points,
        total_size_bytes,
    })
}

// ============================================================
// CURVE VIEWER COMMANDS
// ============================================================

/// Stored curve info for the curve viewer (different from CurveInfo used during ingest)
#[derive(Debug, Serialize)]
pub struct StoredCurveInfo {
    pub id: String,
    pub mnemonic: String,
    pub unit: Option<String>,
    pub description: Option<String>,
    pub sample_count: i64,
    pub min_depth: Option<f64>,
    pub max_depth: Option<f64>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub native_parquet_hash: Option<String>,
    pub quality_flag: Option<String>,
}

/// Curve data result for the viewer
#[derive(Debug, Serialize)]
pub struct CurveDataResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<serde_json::Value>>,
    pub total_rows: i64,
}

/// Get all curves for a specific well
#[tauri::command]
pub fn get_well_curves(
    well_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<StoredCurveInfo>, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let mut stmt = db
        .prepare(
            r#"
            SELECT
                c.id,
                c.mnemonic,
                u.symbol as unit,
                c.description,
                COALESCE(c.native_sample_count, 0) as sample_count,
                c.native_top_depth,
                c.native_bottom_depth,
                c.min_value,
                c.max_value,
                c.native_parquet_hash,
                c.quality_flag
            FROM curves c
            LEFT JOIN units u ON c.unit_id = u.id
            WHERE c.well_id = ? AND c.deleted_at IS NULL
            ORDER BY c.mnemonic ASC
            "#,
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let curves = stmt
        .query_map(params![well_id], |row| {
            Ok(StoredCurveInfo {
                id: row.get(0)?,
                mnemonic: row.get(1)?,
                unit: row.get(2)?,
                description: row.get(3)?,
                sample_count: row.get(4)?,
                min_depth: row.get(5)?,
                max_depth: row.get(6)?,
                min_value: row.get(7)?,
                max_value: row.get(8)?,
                native_parquet_hash: row.get(9)?,
                quality_flag: row.get(10)?,
            })
        })
        .map_err(|e| format!("Failed to query curves: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(curves)
}

/// Query curve data from Parquet using DuckDB
///
/// Returns tabular data for display in AG Grid
#[tauri::command]
pub fn query_curve_data(
    curve_ids: Vec<String>,
    depth_min: Option<f64>,
    depth_max: Option<f64>,
    limit: Option<i64>,
    offset: Option<i64>,
    state: State<'_, Mutex<AppState>>,
) -> Result<CurveDataResult, String> {
    let limit = limit.unwrap_or(1000);
    let offset = offset.unwrap_or(0);
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let data_dir = state.data_dir.as_ref().ok_or("Data directory not set")?;

    if curve_ids.is_empty() {
        return Ok(CurveDataResult {
            columns: vec![],
            rows: vec![],
            total_rows: 0,
        });
    }

    // Get curve info and blob hashes
    let placeholders: String = curve_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        r#"
        SELECT c.id, c.mnemonic, c.native_parquet_hash
        FROM curves c
        WHERE c.id IN ({}) AND c.native_parquet_hash IS NOT NULL AND c.deleted_at IS NULL
        ORDER BY c.mnemonic ASC
        "#,
        placeholders
    );

    let mut stmt = db.prepare(&query).map_err(|e| format!("Failed to prepare query: {}", e))?;

    let curve_info: Vec<(String, String, String)> = {
        let params: Vec<&dyn rusqlite::ToSql> = curve_ids.iter().map(|s| s as &dyn rusqlite::ToSql).collect();
        stmt.query_map(params.as_slice(), |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
        })
        .map_err(|e| format!("Failed to query curve info: {}", e))?
        .filter_map(|r| r.ok())
        .collect()
    };

    if curve_info.is_empty() {
        return Ok(CurveDataResult {
            columns: vec![],
            rows: vec![],
            total_rows: 0,
        });
    }

    // Create DuckDB connection for Parquet queries
    let duckdb_conn = duckdb::Connection::open_in_memory()
        .map_err(|e| format!("Failed to create DuckDB connection: {}", e))?;

    let blobs_dir = data_dir.join("blobs");

    // Build column list and find common depth range
    let mut columns = vec!["DEPTH".to_string()];
    let mut parquet_files: Vec<(String, std::path::PathBuf)> = Vec::new();

    for (_id, mnemonic, hash) in &curve_info {
        columns.push(mnemonic.clone());

        // Get blob path (content-addressed: two levels of nesting + .parquet extension)
        // e.g., "a3f2b8c9..." -> "blobs/a3/f2/a3f2b8c9....parquet"
        let blob_path = blobs_dir
            .join(&hash[..2])
            .join(&hash[2..4])
            .join(format!("{}.parquet", hash));
        if blob_path.exists() {
            parquet_files.push((mnemonic.clone(), blob_path));
        }
    }

    if parquet_files.is_empty() {
        return Ok(CurveDataResult {
            columns,
            rows: vec![],
            total_rows: 0,
        });
    }

    // For single curve, query directly
    if parquet_files.len() == 1 {
        let (mnemonic, path) = &parquet_files[0];
        let path_str = path.to_string_lossy();

        // Build WHERE clause for depth filtering
        let mut where_clauses = Vec::new();
        if let Some(min) = depth_min {
            where_clauses.push(format!("DEPTH >= {}", min));
        }
        if let Some(max) = depth_max {
            where_clauses.push(format!("DEPTH <= {}", max));
        }
        let where_clause = if where_clauses.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", where_clauses.join(" AND "))
        };

        // Count total rows
        let count_query = format!(
            "SELECT COUNT(*) FROM read_parquet('{}'){}",
            path_str, where_clause
        );
        let total_rows: i64 = duckdb_conn
            .query_row(&count_query, [], |row| row.get(0))
            .unwrap_or(0);

        // Query data with pagination
        let data_query = format!(
            "SELECT DEPTH, \"{}\" FROM read_parquet('{}'){} ORDER BY DEPTH LIMIT {} OFFSET {}",
            mnemonic, path_str, where_clause, limit, offset
        );

        let mut stmt = duckdb_conn
            .prepare(&data_query)
            .map_err(|e| format!("Failed to prepare DuckDB query: {}", e))?;

        let mut rows = Vec::new();
        let row_iter = stmt
            .query_map([], |row| {
                let depth: f64 = row.get(0)?;
                let value: Option<f64> = row.get(1)?;
                Ok(vec![
                    serde_json::Number::from_f64(depth)
                        .map(serde_json::Value::Number)
                        .unwrap_or(serde_json::Value::Null),
                    value
                        .and_then(|v| serde_json::Number::from_f64(v).map(serde_json::Value::Number))
                        .unwrap_or(serde_json::Value::Null),
                ])
            })
            .map_err(|e| format!("Failed to query data: {}", e))?;

        for row_result in row_iter {
            if let Ok(row) = row_result {
                rows.push(row);
            }
        }

        return Ok(CurveDataResult {
            columns: vec!["DEPTH".to_string(), mnemonic.clone()],
            rows,
            total_rows,
        });
    }

    // For multiple curves, join them on DEPTH using DuckDB
    // Build a query that joins all parquet files on DEPTH
    let num_curves = parquet_files.len();

    // Create table aliases and build SELECT clause
    let mut select_parts = vec!["t0.DEPTH".to_string()];
    let mut from_parts = Vec::new();
    let mut join_parts = Vec::new();

    for (i, (mnemonic, path)) in parquet_files.iter().enumerate() {
        let path_str = path.to_string_lossy();
        let alias = format!("t{}", i);

        // Add column to SELECT (quote mnemonic to handle special chars)
        select_parts.push(format!("{}.\"{}\" AS \"{}\"", alias, mnemonic, mnemonic));

        if i == 0 {
            // First table in FROM clause
            from_parts.push(format!("read_parquet('{}') AS {}", path_str, alias));
        } else {
            // Join subsequent tables on DEPTH
            join_parts.push(format!(
                "LEFT JOIN read_parquet('{}') AS {} ON t0.DEPTH = {}.DEPTH",
                path_str, alias, alias
            ));
        }
    }

    // Build WHERE clause for depth filtering
    let mut where_clauses = Vec::new();
    if let Some(min) = depth_min {
        where_clauses.push(format!("t0.DEPTH >= {}", min));
    }
    if let Some(max) = depth_max {
        where_clauses.push(format!("t0.DEPTH <= {}", max));
    }
    let where_clause = if where_clauses.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", where_clauses.join(" AND "))
    };

    // Build full query
    let from_clause = from_parts.join("");
    let join_clause = join_parts.join(" ");

    // Count total rows (from first table as base)
    let count_query = format!(
        "SELECT COUNT(*) FROM {} {} {}",
        from_clause, join_clause, where_clause
    );
    let total_rows: i64 = duckdb_conn
        .query_row(&count_query, [], |row| row.get(0))
        .unwrap_or(0);

    // Query data with pagination
    let data_query = format!(
        "SELECT {} FROM {} {} {} ORDER BY t0.DEPTH LIMIT {} OFFSET {}",
        select_parts.join(", "),
        from_clause,
        join_clause,
        where_clause,
        limit,
        offset
    );

    let mut stmt = duckdb_conn
        .prepare(&data_query)
        .map_err(|e| format!("Failed to prepare DuckDB query: {}", e))?;

    let mut rows = Vec::new();
    let mut row_result = stmt.query([])
        .map_err(|e| format!("Failed to execute query: {}", e))?;

    while let Some(row) = row_result.next().map_err(|e| format!("Failed to fetch row: {}", e))? {
        let mut row_data = Vec::with_capacity(num_curves + 1);

        // Get DEPTH (column 0)
        let depth: f64 = row.get(0).map_err(|e| format!("Failed to get depth: {}", e))?;
        row_data.push(
            serde_json::Number::from_f64(depth)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null)
        );

        // Get each curve value (columns 1..n)
        for i in 1..=num_curves {
            let value: Option<f64> = row.get(i).ok();
            row_data.push(
                value
                    .and_then(|v| serde_json::Number::from_f64(v).map(serde_json::Value::Number))
                    .unwrap_or(serde_json::Value::Null)
            );
        }

        rows.push(row_data);
    }

    // Build column names
    let mut result_columns = vec!["DEPTH".to_string()];
    for (mnemonic, _) in &parquet_files {
        result_columns.push(mnemonic.clone());
    }

    Ok(CurveDataResult {
        columns: result_columns,
        rows,
        total_rows,
    })
}

/// A depth segment where data is present (non-null)
#[derive(Debug, Serialize)]
pub struct DepthSegment {
    /// Start depth of this segment
    pub start_depth: f64,
    /// End depth of this segment
    pub end_depth: f64,
    /// Number of samples in this segment
    pub sample_count: i64,
}

/// Coverage information for a single curve
#[derive(Debug, Serialize)]
pub struct CurveCoverage {
    /// Curve ID
    pub id: String,
    /// Curve mnemonic (e.g., GR, RHOB)
    pub mnemonic: String,
    /// Unit symbol
    pub unit: Option<String>,
    /// Overall min depth (from metadata)
    pub min_depth: Option<f64>,
    /// Overall max depth (from metadata)
    pub max_depth: Option<f64>,
    /// Total sample count
    pub total_samples: i64,
    /// Non-null sample count
    pub valid_samples: i64,
    /// Depth segments where data is present
    pub segments: Vec<DepthSegment>,
}

/// Coverage result for a well
#[derive(Debug, Serialize)]
pub struct WellCoverageResult {
    /// Well ID
    pub well_id: String,
    /// Well name
    pub well_name: String,
    /// Overall min depth across all curves
    pub min_depth: f64,
    /// Overall max depth across all curves
    pub max_depth: f64,
    /// Coverage data for each curve
    pub curves: Vec<CurveCoverage>,
}

/// Get curve coverage with gap detection for a well
///
/// This function reads each curve's Parquet file and identifies contiguous
/// segments where data is present (non-null). Gaps in the data are detected
/// when the depth interval between adjacent samples exceeds a threshold.
#[tauri::command]
pub fn get_curve_coverage(
    well_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<WellCoverageResult, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let data_dir = state.data_dir.as_ref().ok_or("Data directory not set")?;

    // Get well info
    let (well_name, well_depth_step): (String, f64) = db
        .query_row(
            "SELECT name, COALESCE(depth_step, 0.5) FROM wells WHERE id = ?",
            params![well_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| format!("Failed to get well: {}", e))?;

    // Get all curves for this well
    let mut stmt = db
        .prepare(
            r#"
            SELECT
                c.id,
                c.mnemonic,
                u.symbol as unit,
                c.native_top_depth,
                c.native_bottom_depth,
                COALESCE(c.native_sample_count, 0) as sample_count,
                c.native_parquet_hash
            FROM curves c
            LEFT JOIN units u ON c.unit_id = u.id
            WHERE c.well_id = ? AND c.deleted_at IS NULL
            ORDER BY c.mnemonic ASC
            "#,
        )
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let curve_rows: Vec<(String, String, Option<String>, Option<f64>, Option<f64>, i64, Option<String>)> = stmt
        .query_map(params![well_id], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
            ))
        })
        .map_err(|e| format!("Failed to query curves: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    // Create DuckDB connection for Parquet queries
    let duckdb_conn = duckdb::Connection::open_in_memory()
        .map_err(|e| format!("Failed to create DuckDB connection: {}", e))?;

    let blobs_dir = data_dir.join("blobs");

    // Gap detection threshold: 3x the depth step (to allow for minor irregularities)
    let gap_threshold = well_depth_step * 3.0;

    let mut curves_coverage = Vec::new();
    let mut overall_min_depth = f64::MAX;
    let mut overall_max_depth = f64::MIN;

    for (id, mnemonic, unit, min_depth, max_depth, sample_count, parquet_hash) in curve_rows {
        // Update overall depth range
        if let Some(min) = min_depth {
            overall_min_depth = overall_min_depth.min(min);
        }
        if let Some(max) = max_depth {
            overall_max_depth = overall_max_depth.max(max);
        }

        let mut segments = Vec::new();
        let mut valid_samples: i64 = 0;

        if let Some(hash) = parquet_hash {
            // Get blob path
            let blob_path = blobs_dir
                .join(&hash[..2])
                .join(&hash[2..4])
                .join(format!("{}.parquet", hash));

            if blob_path.exists() {
                // Query depths where value is not null, ordered by depth
                let query = format!(
                    r#"
                    SELECT DEPTH, "{}"
                    FROM read_parquet('{}')
                    WHERE "{}" IS NOT NULL AND NOT isnan("{}")
                    ORDER BY DEPTH ASC
                    "#,
                    mnemonic,
                    blob_path.to_string_lossy(),
                    mnemonic,
                    mnemonic
                );

                if let Ok(mut stmt) = duckdb_conn.prepare(&query) {
                    let mut current_segment_start: Option<f64> = None;
                    let mut current_segment_end: f64 = 0.0;
                    let mut current_segment_count: i64 = 0;
                    let mut prev_depth: Option<f64> = None;

                    let result = stmt.query([]);
                    if let Ok(mut rows) = result {
                        while let Ok(Some(row)) = rows.next() {
                            if let Ok(depth) = row.get::<_, f64>(0) {
                                valid_samples += 1;

                                // Check if this is a new segment (gap detected)
                                let is_new_segment = match prev_depth {
                                    Some(prev) => (depth - prev).abs() > gap_threshold,
                                    None => true,
                                };

                                if is_new_segment {
                                    // Save previous segment if exists
                                    if let Some(start) = current_segment_start {
                                        segments.push(DepthSegment {
                                            start_depth: start,
                                            end_depth: current_segment_end,
                                            sample_count: current_segment_count,
                                        });
                                    }
                                    // Start new segment
                                    current_segment_start = Some(depth);
                                    current_segment_count = 1;
                                } else {
                                    current_segment_count += 1;
                                }

                                current_segment_end = depth;
                                prev_depth = Some(depth);
                            }
                        }

                        // Don't forget the last segment
                        if let Some(start) = current_segment_start {
                            segments.push(DepthSegment {
                                start_depth: start,
                                end_depth: current_segment_end,
                                sample_count: current_segment_count,
                            });
                        }
                    }
                }
            }
        }

        curves_coverage.push(CurveCoverage {
            id,
            mnemonic,
            unit,
            min_depth,
            max_depth,
            total_samples: sample_count,
            valid_samples,
            segments,
        });
    }

    // Handle case where no curves have depth data
    if overall_min_depth == f64::MAX {
        overall_min_depth = 0.0;
    }
    if overall_max_depth == f64::MIN {
        overall_max_depth = 0.0;
    }

    Ok(WellCoverageResult {
        well_id,
        well_name,
        min_depth: overall_min_depth,
        max_depth: overall_max_depth,
        curves: curves_coverage,
    })
}

// ============================================================
// CURVE EDITING COMMANDS (Blob data modification)
// ============================================================

use dataforge_core::{
    update_curve_data, add_curve_rows, delete_curve_rows,
    get_curve_versions, revert_curve_to_version,
    CellEdit,
};

/// Request for updating curve data cells
#[derive(Debug, Deserialize)]
pub struct UpdateCurveDataRequest {
    /// Curve ID to update
    pub curve_id: String,
    /// List of cell edits to apply
    pub edits: Vec<CellEditDto>,
    /// Reason for the edit (e.g., "edit", "correction")
    pub reason: Option<String>,
}

/// Single cell edit data transfer object
#[derive(Debug, Deserialize)]
pub struct CellEditDto {
    /// Row index (0-based)
    pub row_index: usize,
    /// Column name to edit
    pub column: String,
    /// New value (null to set as NULL)
    pub new_value: Option<f64>,
}

/// Response for curve update operations
#[derive(Debug, Serialize)]
pub struct CurveUpdateResponse {
    /// New parquet hash after edits
    pub new_hash: String,
    /// Number of edits applied
    pub edits_applied: usize,
    /// Total row count after edits
    pub row_count: usize,
    /// New version number
    pub new_version: i64,
}

/// Request for adding rows to curve
#[derive(Debug, Deserialize)]
pub struct AddCurveRowsRequest {
    /// Curve ID
    pub curve_id: String,
    /// Rows to add (each row is a map of column name to value)
    pub rows: Vec<std::collections::HashMap<String, Option<f64>>>,
    /// Name of the depth column (typically "DEPTH")
    pub depth_column: String,
    /// Reason for the addition
    pub reason: Option<String>,
}

/// Request for deleting rows from curve
#[derive(Debug, Deserialize)]
pub struct DeleteCurveRowsRequest {
    /// Curve ID
    pub curve_id: String,
    /// Row indices to delete (0-based)
    pub row_indices: Vec<usize>,
    /// Reason for the deletion
    pub reason: Option<String>,
}

/// Curve version info for the frontend
#[derive(Debug, Serialize)]
pub struct CurveVersionDto {
    pub id: String,
    pub version: i64,
    pub created_at: String,
    pub created_by: Option<String>,
    pub reason: Option<String>,
    pub native_parquet_hash: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub mean_value: Option<f64>,
    pub null_count: Option<i64>,
}

/// Update curve data (cell edits)
#[tauri::command]
pub fn curve_update_cells(
    request: UpdateCurveDataRequest,
    state: State<'_, Mutex<AppState>>,
) -> Result<CurveUpdateResponse, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let data_dir = state.data_dir.as_ref().ok_or("Data directory not set")?;

    let blob_store = BlobStore::new(data_dir.join("blobs"))
        .map_err(|e| format!("Failed to open blob store: {}", e))?;

    let curve_id = uuid::Uuid::parse_str(&request.curve_id)
        .map_err(|e| format!("Invalid curve ID: {}", e))?;

    // Convert DTOs to CellEdit structs
    let edits: Vec<CellEdit> = request.edits
        .into_iter()
        .map(|e| CellEdit {
            row_index: e.row_index,
            column: e.column,
            new_value: e.new_value,
        })
        .collect();

    let reason = request.reason.as_deref().unwrap_or("edit");

    // Note: user_id tracking would require getting account_id from token validation
    // For now, pass None - could be enhanced later
    let result = update_curve_data(db, &blob_store, curve_id, &edits, reason, None)
        .map_err(|e| format!("Failed to update curve data: {}", e))?;

    info!(
        curve_id = %curve_id,
        edits = result.edits_applied,
        version = result.new_version,
        "Updated curve data cells"
    );

    Ok(CurveUpdateResponse {
        new_hash: result.new_native_hash,
        edits_applied: result.edits_applied,
        row_count: result.row_count,
        new_version: result.new_version,
    })
}

/// Add rows to curve data
#[tauri::command]
pub fn curve_add_rows(
    request: AddCurveRowsRequest,
    state: State<'_, Mutex<AppState>>,
) -> Result<CurveUpdateResponse, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let data_dir = state.data_dir.as_ref().ok_or("Data directory not set")?;

    let blob_store = BlobStore::new(data_dir.join("blobs"))
        .map_err(|e| format!("Failed to open blob store: {}", e))?;

    let curve_id = uuid::Uuid::parse_str(&request.curve_id)
        .map_err(|e| format!("Invalid curve ID: {}", e))?;

    let reason = request.reason.as_deref().unwrap_or("add_rows");

    let result = add_curve_rows(
        db,
        &blob_store,
        curve_id,
        &request.rows,
        &request.depth_column,
        reason,
        None,
    )
    .map_err(|e| format!("Failed to add rows: {}", e))?;

    info!(
        curve_id = %curve_id,
        rows_added = result.edits_applied,
        version = result.new_version,
        "Added rows to curve"
    );

    Ok(CurveUpdateResponse {
        new_hash: result.new_native_hash,
        edits_applied: result.edits_applied,
        row_count: result.row_count,
        new_version: result.new_version,
    })
}

/// Delete rows from curve data
#[tauri::command]
pub fn curve_delete_rows(
    request: DeleteCurveRowsRequest,
    state: State<'_, Mutex<AppState>>,
) -> Result<CurveUpdateResponse, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;
    let data_dir = state.data_dir.as_ref().ok_or("Data directory not set")?;

    let blob_store = BlobStore::new(data_dir.join("blobs"))
        .map_err(|e| format!("Failed to open blob store: {}", e))?;

    let curve_id = uuid::Uuid::parse_str(&request.curve_id)
        .map_err(|e| format!("Invalid curve ID: {}", e))?;

    let reason = request.reason.as_deref().unwrap_or("delete_rows");

    let result = delete_curve_rows(
        db,
        &blob_store,
        curve_id,
        &request.row_indices,
        reason,
        None,
    )
    .map_err(|e| format!("Failed to delete rows: {}", e))?;

    info!(
        curve_id = %curve_id,
        rows_deleted = result.edits_applied,
        version = result.new_version,
        "Deleted rows from curve"
    );

    Ok(CurveUpdateResponse {
        new_hash: result.new_native_hash,
        edits_applied: result.edits_applied,
        row_count: result.row_count,
        new_version: result.new_version,
    })
}

/// Get curve version history
#[tauri::command]
pub fn curve_get_versions(
    curve_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<CurveVersionDto>, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let curve_uuid = uuid::Uuid::parse_str(&curve_id)
        .map_err(|e| format!("Invalid curve ID: {}", e))?;

    let versions = get_curve_versions(db, curve_uuid)
        .map_err(|e| format!("Failed to get curve versions: {}", e))?;

    Ok(versions
        .into_iter()
        .map(|v| CurveVersionDto {
            id: v.id,
            version: v.version,
            created_at: v.created_at,
            created_by: v.created_by,
            reason: v.reason,
            native_parquet_hash: v.native_parquet_hash,
            min_value: v.min_value,
            max_value: v.max_value,
            mean_value: v.mean_value,
            null_count: v.null_count,
        })
        .collect())
}

/// Revert curve to a previous version
#[tauri::command]
pub fn curve_revert_to_version(
    curve_id: String,
    target_version: i64,
    state: State<'_, Mutex<AppState>>,
) -> Result<CurveUpdateResponse, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let curve_uuid = uuid::Uuid::parse_str(&curve_id)
        .map_err(|e| format!("Invalid curve ID: {}", e))?;

    let result = revert_curve_to_version(db, curve_uuid, target_version, None)
        .map_err(|e| format!("Failed to revert curve: {}", e))?;

    info!(
        curve_id = %curve_uuid,
        reverted_to = target_version,
        new_version = result.new_version,
        "Reverted curve to previous version"
    );

    Ok(CurveUpdateResponse {
        new_hash: result.new_native_hash,
        edits_applied: result.edits_applied,
        row_count: result.row_count,
        new_version: result.new_version,
    })
}

// ============================================================
// WELL DETAIL PAGE COMMANDS
// ============================================================

/// Full well details including metadata and statistics
#[derive(Debug, Serialize)]
pub struct WellDetails {
    pub id: String,
    pub name: String,
    pub uwi: Option<String>,
    pub field: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub depth_unit: String,
    pub depth_step: f64,
    pub depth_origin: f64,
    pub min_depth: Option<f64>,
    pub max_depth: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
    pub curve_count: i64,
    pub total_data_points: i64,
    pub total_size_bytes: i64,
}

/// Get detailed information about a specific well
#[tauri::command]
pub fn get_well_details(
    well_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<WellDetails, String> {
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    // Get well metadata
    let well: WellDetails = db
        .query_row(
            r#"
            SELECT
                w.id,
                w.name,
                w.uwi,
                w.field,
                w.company,
                w.location,
                w.x,
                w.y,
                w.depth_unit,
                w.depth_step,
                w.depth_origin,
                w.min_depth,
                w.max_depth,
                w.created_at,
                w.updated_at,
                (SELECT COUNT(*) FROM curves c WHERE c.well_id = w.id AND c.deleted_at IS NULL) as curve_count,
                (SELECT COALESCE(SUM(c.native_sample_count), 0) FROM curves c WHERE c.well_id = w.id AND c.deleted_at IS NULL) as total_data_points,
                (SELECT COALESCE(SUM(b.size_bytes), 0) FROM curves c
                 JOIN blob_registry b ON c.native_parquet_hash = b.hash
                 WHERE c.well_id = w.id AND c.deleted_at IS NULL) as total_size_bytes
            FROM wells w
            WHERE w.id = ? AND w.deleted_at IS NULL
            "#,
            params![well_id],
            |row| {
                Ok(WellDetails {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    uwi: row.get(2)?,
                    field: row.get(3)?,
                    company: row.get(4)?,
                    location: row.get(5)?,
                    x: row.get(6)?,
                    y: row.get(7)?,
                    depth_unit: row.get(8)?,
                    depth_step: row.get(9)?,
                    depth_origin: row.get(10)?,
                    min_depth: row.get(11)?,
                    max_depth: row.get(12)?,
                    created_at: row.get(13)?,
                    updated_at: row.get(14)?,
                    curve_count: row.get(15)?,
                    total_data_points: row.get(16)?,
                    total_size_bytes: row.get(17)?,
                })
            },
        )
        .map_err(|e| format!("Failed to get well details: {}", e))?;

    Ok(well)
}

/// Get recent activity for a specific well (curves modified, added, etc.)
#[tauri::command]
pub fn get_well_activity(
    well_id: String,
    limit: Option<i64>,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<RecentActivityItem>, String> {
    let limit = limit.unwrap_or(20);
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    // Get well name for parent_name field
    let well_name: String = db
        .query_row(
            "SELECT name FROM wells WHERE id = ? AND deleted_at IS NULL",
            params![well_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Well not found: {}", e))?;

    // Query curves for this well ordered by updated_at
    let query = r#"
        SELECT
            'curve' as item_type,
            c.id,
            c.mnemonic as name,
            ?1 as parent_name,
            ?2 as parent_id,
            b.size_bytes,
            c.native_sample_count as row_count,
            c.updated_at,
            c.created_at
        FROM curves c
        LEFT JOIN blob_registry b ON c.native_parquet_hash = b.hash
        WHERE c.well_id = ?2 AND c.deleted_at IS NULL
        ORDER BY c.updated_at DESC
        LIMIT ?3
    "#;

    let mut stmt = db
        .prepare(query)
        .map_err(|e| format!("Failed to prepare query: {}", e))?;

    let items = stmt
        .query_map(params![well_name, well_id, limit], |row| {
            Ok(RecentActivityItem {
                item_type: row.get(0)?,
                id: row.get(1)?,
                name: row.get(2)?,
                parent_name: row.get(3)?,
                parent_id: row.get(4)?,
                size_bytes: row.get(5)?,
                row_count: row.get(6)?,
                updated_at: row.get(7)?,
                created_at: row.get(8)?,
            })
        })
        .map_err(|e| format!("Failed to query well activity: {}", e))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(items)
}
