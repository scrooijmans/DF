//! Surface Commands
//!
//! Tauri commands for 3D surface data ingestion from CSV files.
//! Surfaces are workspace-level entities (horizons, faults, unconformities)
//! defined by X, Y, Z point data.

use dataforge_core::blob::BlobStore;
use dataforge_core::models::{SurfaceColumnType, SurfaceType, ZPositiveDirection};
use dataforge_core::surfaces::{
    ingest_surface_file, parse_surface_csv, SurfaceColumnMapping, SurfaceIngestConfig,
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

/// Spatial extent bounding box
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpatialExtentInfo {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub min_z: f64,
    pub max_z: f64,
}

/// Response type for surface CSV parsing
#[derive(Debug, Serialize, Deserialize)]
pub struct ParseSurfaceResponse {
    /// Unique identifier for this parse result
    pub file_id: String,
    /// Original file name
    pub file_name: String,
    /// Detected columns with their types
    pub columns: Vec<SurfaceColumnInfo>,
    /// Number of data rows
    pub row_count: usize,
    /// Detected delimiter
    pub delimiter: String,
    /// Whether this is a multi-surface file
    pub is_multi_surface: bool,
    /// Surface names found in the file (if multi-surface)
    pub surface_names: Vec<String>,
    /// Spatial extent (bounding box)
    pub extent: SpatialExtentInfo,
    /// Whether required columns (X, Y, Z) were found
    pub has_required_columns: bool,
    /// Missing required column names
    pub missing_required: Vec<String>,
    /// Parsing warnings
    pub warnings: Vec<String>,
}

/// Information about a column in the surface CSV
#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceColumnInfo {
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
}

/// Request for ingesting surface CSV files
#[derive(Debug, Deserialize)]
pub struct IngestSurfaceRequest {
    /// File paths to process
    pub file_paths: Vec<String>,
    /// Column configurations (type overrides)
    pub column_configs: Vec<SurfaceColumnConfig>,
    /// Surface metadata
    pub surface_name: Option<String>,
    pub surface_type: Option<String>,
    pub crs: Option<String>,
    pub xy_unit: String,
    pub z_unit: String,
    pub z_positive_direction: String,
}

/// Configuration for a surface column
#[derive(Debug, Deserialize)]
pub struct SurfaceColumnConfig {
    /// Column index
    pub index: usize,
    /// Override column type
    pub column_type: Option<String>,
}

/// Response from surface ingestion
#[derive(Debug, Serialize)]
pub struct IngestSurfaceResponse {
    pub success: bool,
    pub surface_ids: Vec<String>,
    pub surface_count: usize,
    pub total_points: usize,
    pub error: Option<String>,
    pub warnings: Vec<String>,
}

/// Summary of a surface
#[derive(Debug, Serialize)]
pub struct SurfaceSummary {
    pub id: String,
    pub name: String,
    pub surface_type: Option<String>,
    pub point_count: i64,
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub min_z: f64,
    pub max_z: f64,
    pub crs: Option<String>,
    pub xy_unit: String,
    pub z_unit: String,
    pub imported_at: String,
}

/// Parse a surface CSV file for ingestion
///
/// Returns column detection results and spatial extent for the wizard UI.
#[tauri::command]
pub async fn parse_surface_for_ingest(
    file_path: String,
    _state: State<'_, Mutex<AppState>>,
) -> Result<ParseSurfaceResponse, String> {
    info!("Parsing surface CSV: {}", file_path);

    let path = PathBuf::from(&file_path);
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown.csv")
        .to_string();

    // Read file
    let raw_bytes = std::fs::read(&path).map_err(|e| format!("Failed to read file: {}", e))?;

    // Generate file ID
    let file_id = Uuid::new_v4().to_string();

    // Parse CSV
    let parsed = parse_surface_csv(Cursor::new(&raw_bytes), file_id.clone())
        .map_err(|e| format!("Failed to parse CSV: {}", e))?;

    // Build column info
    let columns: Vec<SurfaceColumnInfo> = parsed
        .columns
        .iter()
        .map(|col| {
            let detected_type = match col.column_type {
                SurfaceColumnType::X => "x",
                SurfaceColumnType::Y => "y",
                SurfaceColumnType::Z => "z",
                SurfaceColumnType::SurfaceName => "surface_name",
                SurfaceColumnType::Quality => "quality",
                SurfaceColumnType::Attribute => "attribute",
                SurfaceColumnType::Unknown => "unknown",
            }
            .to_string();

            SurfaceColumnInfo {
                index: col.index,
                header: col.original_name.clone(),
                detected_type,
                confidence: col.confidence,
                unit: col.unit.clone(),
                sample_values: Vec::new(), // TODO: Could add sample values
            }
        })
        .collect();

    // Calculate overall extent from all groups
    let extent = if !parsed.groups.is_empty() {
        let first = &parsed.groups[0].extent;
        let mut ext = SpatialExtentInfo {
            min_x: first.min_x,
            max_x: first.max_x,
            min_y: first.min_y,
            max_y: first.max_y,
            min_z: first.min_z,
            max_z: first.max_z,
        };

        for group in &parsed.groups[1..] {
            ext.min_x = ext.min_x.min(group.extent.min_x);
            ext.max_x = ext.max_x.max(group.extent.max_x);
            ext.min_y = ext.min_y.min(group.extent.min_y);
            ext.max_y = ext.max_y.max(group.extent.max_y);
            ext.min_z = ext.min_z.min(group.extent.min_z);
            ext.max_z = ext.max_z.max(group.extent.max_z);
        }
        ext
    } else {
        SpatialExtentInfo {
            min_x: 0.0,
            max_x: 0.0,
            min_y: 0.0,
            max_y: 0.0,
            min_z: 0.0,
            max_z: 0.0,
        }
    };

    let delimiter = match parsed.delimiter {
        ',' => "comma",
        '\t' => "tab",
        ';' => "semicolon",
        '|' => "pipe",
        ' ' => "space",
        _ => "unknown",
    }
    .to_string();

    // Get surface names from groups
    let surface_names: Vec<String> = parsed
        .groups
        .iter()
        .filter_map(|g| g.name.clone())
        .collect();

    let is_multi_surface = surface_names.len() > 1
        || (surface_names.len() == 1 && parsed.groups.len() > 1);

    Ok(ParseSurfaceResponse {
        file_id,
        file_name,
        columns,
        row_count: parsed.row_count,
        delimiter,
        is_multi_surface,
        surface_names,
        extent,
        has_required_columns: parsed.has_required_columns,
        missing_required: parsed.missing_required,
        warnings: Vec::new(),
    })
}

/// Ingest surface CSV files
#[tauri::command]
pub async fn ingest_surface_files(
    request: IngestSurfaceRequest,
    state: State<'_, Mutex<AppState>>,
) -> Result<IngestSurfaceResponse, String> {
    info!("Ingesting {} surface files", request.file_paths.len());

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

    // Get workspace member ID for imported_by field
    let member_id: Option<Uuid> = {
        if let Some(token) = state.session.token.as_ref() {
            // Get account_id from session token
            let account_id: Result<String, _> = db.query_row(
                "SELECT s.account_id FROM sessions s WHERE s.token_hash = ?",
                params![dataforge_core::auth::hash_token(token)],
                |row| row.get(0),
            );

            if let Ok(account_id) = account_id {
                // Get workspace member ID
                let member_id_str: Result<String, _> = db.query_row(
                    "SELECT id FROM workspace_members WHERE workspace_id = ? AND account_id = ?",
                    params![workspace_id.to_string(), account_id],
                    |row| row.get(0),
                );

                member_id_str.ok().and_then(|s| Uuid::parse_str(&s).ok())
            } else {
                None
            }
        } else {
            None
        }
    };

    // Parse surface type
    let surface_type = request.surface_type.as_ref().and_then(|s| match s.as_str() {
        "horizon" => Some(SurfaceType::Horizon),
        "fault" => Some(SurfaceType::Fault),
        "unconformity" => Some(SurfaceType::Unconformity),
        "contact" => Some(SurfaceType::Contact),
        "other" => Some(SurfaceType::Other),
        _ => None,
    });

    // Parse z positive direction
    let z_positive_direction = match request.z_positive_direction.as_str() {
        "up" => ZPositiveDirection::Up,
        _ => ZPositiveDirection::Down,
    };

    // Build column mappings from config
    let column_mappings: Option<Vec<SurfaceColumnMapping>> = if !request.column_configs.is_empty() {
        Some(
            request
                .column_configs
                .iter()
                .filter_map(|cfg| {
                    cfg.column_type.as_ref().map(|t| {
                        let assigned_type = match t.as_str() {
                            "x" => SurfaceColumnType::X,
                            "y" => SurfaceColumnType::Y,
                            "z" => SurfaceColumnType::Z,
                            "surface_name" => SurfaceColumnType::SurfaceName,
                            "quality" => SurfaceColumnType::Quality,
                            "attribute" => SurfaceColumnType::Attribute,
                            _ => SurfaceColumnType::Unknown,
                        };

                        SurfaceColumnMapping {
                            index: cfg.index,
                            original_name: String::new(), // Will be filled from parsed data
                            assigned_type,
                        }
                    })
                })
                .collect(),
        )
    } else {
        None
    };

    // Build ingest config
    let config = SurfaceIngestConfig {
        workspace_id,
        surface_name: request.surface_name,
        surface_type,
        crs: request.crs,
        xy_unit: request.xy_unit,
        z_unit: request.z_unit,
        z_positive_direction,
        column_mappings,
        imported_by: member_id,
    };

    // Process files
    if request.file_paths.is_empty() {
        return Err("No files provided".to_string());
    }

    let mut all_surface_ids = Vec::new();
    let mut total_surfaces = 0;
    let mut total_points = 0;
    let mut all_warnings = Vec::new();

    for file_path_str in &request.file_paths {
        let file_path = PathBuf::from(file_path_str);

        match ingest_surface_file(db, &blob_store, &file_path, config.clone()) {
            Ok(result) => {
                info!(
                    "Successfully ingested surface file: {} surfaces, {} points",
                    result.surface_count, result.total_points
                );

                for surface in &result.surfaces {
                    all_surface_ids.push(surface.id.to_string());
                }
                total_surfaces += result.surface_count;
                total_points += result.total_points;
                all_warnings.extend(result.warnings);
            }
            Err(e) => {
                error!("Failed to ingest surface file {}: {}", file_path_str, e);
                return Ok(IngestSurfaceResponse {
                    success: false,
                    surface_ids: Vec::new(),
                    surface_count: 0,
                    total_points: 0,
                    error: Some(format!("Failed to ingest {}: {}", file_path_str, e)),
                    warnings: Vec::new(),
                });
            }
        }
    }

    Ok(IngestSurfaceResponse {
        success: true,
        surface_ids: all_surface_ids,
        surface_count: total_surfaces,
        total_points,
        error: None,
        warnings: all_warnings,
    })
}

/// Get surfaces for a workspace
#[tauri::command]
pub async fn get_workspace_surfaces(
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<SurfaceSummary>, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let workspace_id = state
        .session
        .current_workspace_id
        .ok_or("No workspace selected")?;

    let mut stmt = db
        .prepare(
            r#"
            SELECT
                id,
                name,
                surface_type,
                point_count,
                min_x,
                max_x,
                min_y,
                max_y,
                min_z,
                max_z,
                crs,
                xy_unit,
                z_unit,
                imported_at
            FROM surfaces
            WHERE workspace_id = ? AND deleted_at IS NULL
            ORDER BY name
            "#,
        )
        .map_err(|e| format!("Query error: {}", e))?;

    let surfaces = stmt
        .query_map(params![workspace_id.to_string()], |row| {
            Ok(SurfaceSummary {
                id: row.get(0)?,
                name: row.get(1)?,
                surface_type: row.get(2)?,
                point_count: row.get(3)?,
                min_x: row.get(4)?,
                max_x: row.get(5)?,
                min_y: row.get(6)?,
                max_y: row.get(7)?,
                min_z: row.get(8)?,
                max_z: row.get(9)?,
                crs: row.get(10)?,
                xy_unit: row.get(11)?,
                z_unit: row.get(12)?,
                imported_at: row.get(13)?,
            })
        })
        .map_err(|e| format!("Query error: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Row error: {}", e))?;

    Ok(surfaces)
}

/// Get a single surface by ID
#[tauri::command]
pub async fn get_surface(
    surface_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Option<SurfaceSummary>, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let result = db
        .query_row(
            r#"
            SELECT
                id,
                name,
                surface_type,
                point_count,
                min_x,
                max_x,
                min_y,
                max_y,
                min_z,
                max_z,
                crs,
                xy_unit,
                z_unit,
                imported_at
            FROM surfaces
            WHERE id = ? AND deleted_at IS NULL
            "#,
            params![surface_id],
            |row| {
                Ok(SurfaceSummary {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    surface_type: row.get(2)?,
                    point_count: row.get(3)?,
                    min_x: row.get(4)?,
                    max_x: row.get(5)?,
                    min_y: row.get(6)?,
                    max_y: row.get(7)?,
                    min_z: row.get(8)?,
                    max_z: row.get(9)?,
                    crs: row.get(10)?,
                    xy_unit: row.get(11)?,
                    z_unit: row.get(12)?,
                    imported_at: row.get(13)?,
                })
            },
        )
        .ok();

    Ok(result)
}

/// Delete a surface
#[tauri::command]
pub async fn delete_surface(
    surface_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<bool, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    // Soft delete - set deleted_at
    let rows_affected = db
        .execute(
            "UPDATE surfaces SET deleted_at = datetime('now') WHERE id = ? AND deleted_at IS NULL",
            params![surface_id],
        )
        .map_err(|e| format!("Delete error: {}", e))?;

    Ok(rows_affected > 0)
}
