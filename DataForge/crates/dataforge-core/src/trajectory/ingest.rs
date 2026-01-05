//! Trajectory Ingestion Service
//!
//! Handles the complete trajectory CSV file ingestion workflow:
//! 1. Parse CSV file
//! 2. Detect column types (MD, INC, AZI, etc.)
//! 3. Validate required columns present
//! 4. Calculate derived values (TVD, NS, EW, DLS)
//! 5. Store raw file in blob storage
//! 6. Create survey_run record
//! 7. For each column:
//!    - Create Parquet file
//!    - Store in blob storage
//!    - Create trajectory_column record
//! 8. Update well's depth range if needed

use crate::blob::BlobStore;
use crate::error::Result;
use crate::models::{CalculationMethod, SurveyType, TrajectoryColumnType};
use crate::trajectory::calculation::{
    calculate_trajectory, InputStation, TieInPoint, TrajectoryCalculationResult,
};
use crate::trajectory::parser::{
    get_required_column_indices, parse_trajectory_csv, ParsedTrajectory, ParsedTrajectoryColumn,
};
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::io::Cursor;
use std::path::Path;
use tracing::info;
use uuid::Uuid;

/// Options for trajectory CSV ingestion
#[derive(Debug, Clone)]
pub struct TrajectoryIngestOptions {
    /// Survey type (definitive, MWD, gyro, etc.)
    pub survey_type: Option<SurveyType>,
    /// Survey company
    pub survey_company: Option<String>,
    /// Magnetic declination (degrees)
    pub magnetic_declination: Option<f64>,
    /// Grid convergence (degrees)
    pub grid_convergence: Option<f64>,
    /// Source coordinate reference system (EPSG code or WKT)
    pub source_crs: Option<String>,
    /// Calculation method (default: minimum curvature)
    pub calculation_method: CalculationMethod,
    /// Tie-in point (for calculations)
    pub tie_in: Option<TieInPoint>,
    /// Whether to store raw CSV file
    pub store_raw_file: bool,
    /// Length unit for DLS normalization (100 for ft, 30 for m)
    pub dls_unit_length: f64,
    /// MD column unit (ft or m)
    pub md_unit: String,
    /// Angle column unit (deg or rad)
    pub angle_unit: String,
}

impl Default for TrajectoryIngestOptions {
    fn default() -> Self {
        Self {
            survey_type: None,
            survey_company: None,
            magnetic_declination: None,
            grid_convergence: None,
            source_crs: None,
            calculation_method: CalculationMethod::MinimumCurvature,
            tie_in: None,
            store_raw_file: true,
            dls_unit_length: 100.0,
            md_unit: "ft".to_string(),
            angle_unit: "deg".to_string(),
        }
    }
}

/// Result of ingesting a single trajectory column
#[derive(Debug)]
pub struct IngestedTrajectoryColumn {
    pub column_id: Uuid,
    pub column_type: TrajectoryColumnType,
    pub column_name: String,
    pub unit: Option<String>,
    pub parquet_hash: String,
    pub sample_count: usize,
    pub null_count: usize,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub mean_value: Option<f64>,
    pub is_input: bool,
    pub is_calculated: bool,
}

/// Result of ingesting a trajectory CSV file
#[derive(Debug)]
pub struct TrajectoryIngestResult {
    pub survey_run_id: Uuid,
    pub well_id: Uuid,
    pub source_file_hash: String,
    pub raw_file_blob_hash: Option<String>,
    pub columns: Vec<IngestedTrajectoryColumn>,
    pub station_count: usize,
    pub top_md: f64,
    pub bottom_md: f64,
    pub max_inclination: f64,
    pub max_dls: f64,
    /// Whether a new well was created
    pub well_created: bool,
}

/// Column mapping override - allows user to specify column types
#[derive(Debug, Clone)]
pub struct ColumnMapping {
    /// Column index in the CSV
    pub index: usize,
    /// Override column type
    pub column_type: TrajectoryColumnType,
    /// Override unit
    pub unit: Option<String>,
}

/// Ingest a trajectory CSV file into a well
///
/// # Arguments
/// * `conn` - SQLite database connection
/// * `blob_store` - Blob storage for Parquet files
/// * `csv_path` - Path to the CSV file
/// * `well_id` - Existing well ID, or None to create a new well
/// * `well_name` - Name for new well (if well_id is None)
/// * `workspace_id` - Workspace to add the well/trajectory to
/// * `created_by` - Member ID creating this data
/// * `column_mappings` - Optional column type overrides
/// * `options` - Ingest options
pub fn ingest_trajectory_file(
    conn: &Connection,
    blob_store: &BlobStore,
    csv_path: &Path,
    well_id: Option<Uuid>,
    well_name: Option<&str>,
    workspace_id: Uuid,
    created_by: Uuid,
    column_mappings: Option<&[ColumnMapping]>,
    options: &TrajectoryIngestOptions,
) -> Result<TrajectoryIngestResult> {
    // 1. Read and hash raw CSV file
    let raw_bytes = std::fs::read(csv_path)?;
    let source_file_hash = hex::encode(Sha256::digest(&raw_bytes));

    // 2. Parse CSV file
    let parsed = parse_trajectory_csv(Cursor::new(&raw_bytes))
        .map_err(|e| crate::error::Error::InvalidData(e))?;

    // 3. Apply column mappings if provided
    let columns = apply_column_mappings(&parsed.columns, column_mappings);

    // 4. Get required column indices (MD, INC, AZI)
    let required_indices = get_required_column_indices(&columns)
        .map_err(|e| crate::error::Error::InvalidData(e))?;

    // 5. Extract input data
    let input_stations = extract_input_stations(&columns, &required_indices, &options.angle_unit)?;

    if input_stations.is_empty() {
        return Err(crate::error::Error::InvalidData(
            "No valid trajectory stations found".to_string(),
        ));
    }

    // 6. Calculate derived values (TVD, NS, EW, DLS)
    let calc_result =
        calculate_trajectory(&input_stations, options.tie_in.clone(), options.dls_unit_length);

    // 7. Get or create well
    let (final_well_id, well_created) =
        get_or_create_well_for_trajectory(conn, well_id, workspace_id, created_by, well_name)?;

    // 8. Store raw CSV file if requested
    let raw_file_blob_hash = if options.store_raw_file {
        let hash = blob_store.store(&raw_bytes)?;
        register_blob(conn, &hash, raw_bytes.len())?;
        Some(hash)
    } else {
        None
    };

    // 9. Create survey_run record
    let survey_run_id = create_survey_run(
        conn,
        final_well_id,
        workspace_id,
        csv_path,
        &source_file_hash,
        raw_file_blob_hash.as_deref(),
        &calc_result,
        created_by,
        options,
    )?;

    // 10. Create trajectory columns (both input and calculated)
    let mut ingested_columns = Vec::new();

    // Store input columns (MD, INC, AZI)
    for col in &columns {
        if col.detected.column_type.is_input() && col.detected.confidence >= 0.5 {
            let ingested = ingest_trajectory_column(
                conn,
                blob_store,
                survey_run_id,
                final_well_id,
                col,
                created_by,
            )?;
            ingested_columns.push(ingested);
        }
    }

    // Store calculated columns (TVD, NS, EW, DLS)
    let calculated_columns = create_calculated_columns(&calc_result, &options.md_unit);
    for (col_type, values, unit) in calculated_columns {
        let ingested = ingest_calculated_column(
            conn,
            blob_store,
            survey_run_id,
            final_well_id,
            col_type,
            &values,
            &unit,
            created_by,
        )?;
        ingested_columns.push(ingested);
    }

    // 11. Update well's depth range if needed
    if !calc_result.stations.is_empty() {
        update_well_depth_range_for_trajectory(
            conn,
            final_well_id,
            calc_result.stations.first().unwrap().md,
            calc_result.stations.last().unwrap().md,
        )?;
    }

    info!(
        survey_run_id = %survey_run_id,
        well_id = %final_well_id,
        stations = calc_result.stations.len(),
        columns = ingested_columns.len(),
        "Ingested trajectory CSV file"
    );

    Ok(TrajectoryIngestResult {
        survey_run_id,
        well_id: final_well_id,
        source_file_hash,
        raw_file_blob_hash,
        columns: ingested_columns,
        station_count: calc_result.stations.len(),
        top_md: calc_result.stations.first().map(|s| s.md).unwrap_or(0.0),
        bottom_md: calc_result.stations.last().map(|s| s.md).unwrap_or(0.0),
        max_inclination: calc_result.max_inclination,
        max_dls: calc_result.max_dls,
        well_created,
    })
}

/// Apply user column mappings to parsed columns
fn apply_column_mappings(
    columns: &[ParsedTrajectoryColumn],
    mappings: Option<&[ColumnMapping]>,
) -> Vec<ParsedTrajectoryColumn> {
    let mut result = columns.to_vec();

    if let Some(maps) = mappings {
        for mapping in maps {
            if mapping.index < result.len() {
                result[mapping.index].detected.column_type = mapping.column_type.clone();
                result[mapping.index].detected.confidence = 1.0; // User override = max confidence
                if let Some(ref unit) = mapping.unit {
                    result[mapping.index].detected.unit = Some(unit.clone());
                }
            }
        }
    }

    result
}

/// Extract input stations from parsed columns
fn extract_input_stations(
    columns: &[ParsedTrajectoryColumn],
    indices: &crate::trajectory::parser::RequiredColumnIndices,
    angle_unit: &str,
) -> Result<Vec<InputStation>> {
    let md_col = &columns[indices.md_index];
    let inc_col = &columns[indices.inc_index];
    let azi_col = &columns[indices.azi_index];

    let mut stations = Vec::new();
    let num_rows = md_col.values.len();

    for i in 0..num_rows {
        if let (Some(md), Some(inc), Some(azi)) =
            (md_col.values[i], inc_col.values[i], azi_col.values[i])
        {
            // Convert angles to degrees if needed
            let inc_deg = if angle_unit == "rad" {
                inc.to_degrees()
            } else {
                inc
            };
            let azi_deg = if angle_unit == "rad" {
                azi.to_degrees()
            } else {
                azi
            };

            stations.push(InputStation {
                md,
                inclination: inc_deg,
                azimuth: azi_deg,
            });
        }
    }

    Ok(stations)
}

/// Create calculated column data from trajectory results
fn create_calculated_columns(
    result: &TrajectoryCalculationResult,
    md_unit: &str,
) -> Vec<(TrajectoryColumnType, Vec<Option<f64>>, String)> {
    let mut columns = Vec::new();

    // TVD
    let tvd_values: Vec<Option<f64>> = result.stations.iter().map(|s| Some(s.tvd)).collect();
    columns.push((
        TrajectoryColumnType::TrueVerticalDepth,
        tvd_values,
        md_unit.to_string(),
    ));

    // NS (North-South)
    let ns_values: Vec<Option<f64>> = result.stations.iter().map(|s| Some(s.ns)).collect();
    columns.push((
        TrajectoryColumnType::NorthSouth,
        ns_values,
        md_unit.to_string(),
    ));

    // EW (East-West)
    let ew_values: Vec<Option<f64>> = result.stations.iter().map(|s| Some(s.ew)).collect();
    columns.push((
        TrajectoryColumnType::EastWest,
        ew_values,
        md_unit.to_string(),
    ));

    // DLS (Dogleg Severity)
    let dls_unit = if md_unit == "m" {
        "deg/30m".to_string()
    } else {
        "deg/100ft".to_string()
    };
    let dls_values: Vec<Option<f64>> = result.stations.iter().map(|s| Some(s.dls)).collect();
    columns.push((TrajectoryColumnType::DoglegSeverity, dls_values, dls_unit));

    columns
}

/// Get or create a well for the trajectory
fn get_or_create_well_for_trajectory(
    conn: &Connection,
    well_id: Option<Uuid>,
    workspace_id: Uuid,
    created_by: Uuid,
    well_name: Option<&str>,
) -> Result<(Uuid, bool)> {
    if let Some(id) = well_id {
        // Verify well exists
        let exists: bool = conn
            .query_row(
                "SELECT 1 FROM wells WHERE id = ? AND deleted_at IS NULL",
                params![id.to_string()],
                |_| Ok(true),
            )
            .unwrap_or(false);

        if !exists {
            return Err(crate::error::Error::InvalidData(format!(
                "Well {} not found",
                id
            )));
        }
        Ok((id, false))
    } else {
        // Create new well
        let new_id = Uuid::new_v4();
        let name = well_name.unwrap_or("New Well");

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
                new_id.to_string(),
                workspace_id.to_string(),
                name,
                "ft",
                0.5,
                0.0,
                created_by.to_string(),
            ],
        )?;

        info!(well_id = %new_id, name = name, "Created new well for trajectory");
        Ok((new_id, true))
    }
}

/// Create a survey_run record
fn create_survey_run(
    conn: &Connection,
    well_id: Uuid,
    workspace_id: Uuid,
    csv_path: &Path,
    source_file_hash: &str,
    raw_file_blob_hash: Option<&str>,
    calc_result: &TrajectoryCalculationResult,
    imported_by: Uuid,
    options: &TrajectoryIngestOptions,
) -> Result<Uuid> {
    let survey_run_id = Uuid::new_v4();
    let filename = csv_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown.csv");

    let survey_type_str = options
        .survey_type
        .as_ref()
        .map(|t| t.to_string());
    let calc_method_str = options.calculation_method.to_string();

    let station_count = calc_result.stations.len() as i64;
    let top_md = calc_result.stations.first().map(|s| s.md);
    let bottom_md = calc_result.stations.last().map(|s| s.md);

    // Tie-in values
    let tie_in_md = options.tie_in.as_ref().map(|t| t.md);
    let tie_in_tvd = options.tie_in.as_ref().map(|t| t.tvd);
    let tie_in_ns = options.tie_in.as_ref().map(|t| t.ns);
    let tie_in_ew = options.tie_in.as_ref().map(|t| t.ew);

    conn.execute(
        r#"
        INSERT INTO survey_runs (
            id, well_id, workspace_id, source_filename, source_file_hash, raw_file_blob_hash,
            survey_type, survey_company, magnetic_declination, grid_convergence, source_crs,
            calculation_method, tie_in_md, tie_in_tvd, tie_in_ns, tie_in_ew,
            top_md, bottom_md, station_count, md_unit, imported_by
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)
        "#,
        params![
            survey_run_id.to_string(),
            well_id.to_string(),
            workspace_id.to_string(),
            filename,
            source_file_hash,
            raw_file_blob_hash,
            survey_type_str,
            options.survey_company,
            options.magnetic_declination,
            options.grid_convergence,
            options.source_crs,
            calc_method_str,
            tie_in_md,
            tie_in_tvd,
            tie_in_ns,
            tie_in_ew,
            top_md,
            bottom_md,
            station_count,
            &options.md_unit,
            imported_by.to_string(),
        ],
    )?;

    Ok(survey_run_id)
}

/// Ingest a trajectory column from parsed CSV data
fn ingest_trajectory_column(
    conn: &Connection,
    blob_store: &BlobStore,
    survey_run_id: Uuid,
    well_id: Uuid,
    column: &ParsedTrajectoryColumn,
    created_by: Uuid,
) -> Result<IngestedTrajectoryColumn> {
    let column_id = Uuid::new_v4();

    // Create Parquet file for the column
    let parquet_data = create_column_parquet(&column.detected.original_name, &column.values)?;
    let parquet_hash = blob_store.store(&parquet_data)?;
    register_blob(conn, &parquet_hash, parquet_data.len())?;

    let column_type = &column.detected.column_type;
    let is_input = column_type.is_input();
    let is_calculated = column_type.is_calculated();

    conn.execute(
        r#"
        INSERT INTO trajectory_columns (
            id, survey_run_id, well_id, column_type, column_name, unit,
            is_input, is_calculated,
            min_value, max_value, mean_value, null_count, sample_count,
            parquet_hash, created_by
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
        "#,
        params![
            column_id.to_string(),
            survey_run_id.to_string(),
            well_id.to_string(),
            column_type.canonical_name(),
            column.header,
            column.detected.unit,
            is_input as i32,
            is_calculated as i32,
            column.stats.min,
            column.stats.max,
            column.stats.mean,
            column.stats.null_count as i64,
            column.stats.count as i64,
            parquet_hash,
            created_by.to_string(),
        ],
    )?;

    Ok(IngestedTrajectoryColumn {
        column_id,
        column_type: column_type.clone(),
        column_name: column.header.clone(),
        unit: column.detected.unit.clone(),
        parquet_hash,
        sample_count: column.stats.count,
        null_count: column.stats.null_count,
        min_value: column.stats.min,
        max_value: column.stats.max,
        mean_value: column.stats.mean,
        is_input,
        is_calculated,
    })
}

/// Ingest a calculated column
fn ingest_calculated_column(
    conn: &Connection,
    blob_store: &BlobStore,
    survey_run_id: Uuid,
    well_id: Uuid,
    column_type: TrajectoryColumnType,
    values: &[Option<f64>],
    unit: &str,
    created_by: Uuid,
) -> Result<IngestedTrajectoryColumn> {
    let column_id = Uuid::new_v4();

    // Calculate stats
    let valid_values: Vec<f64> = values.iter().filter_map(|v| *v).collect();
    let count = values.len();
    let null_count = count - valid_values.len();
    let (min, max, mean) = if valid_values.is_empty() {
        (None, None, None)
    } else {
        let min = valid_values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = valid_values
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let sum: f64 = valid_values.iter().sum();
        let mean = sum / valid_values.len() as f64;
        (Some(min), Some(max), Some(mean))
    };

    // Create Parquet file
    let parquet_data = create_column_parquet(column_type.canonical_name(), values)?;
    let parquet_hash = blob_store.store(&parquet_data)?;
    register_blob(conn, &parquet_hash, parquet_data.len())?;

    conn.execute(
        r#"
        INSERT INTO trajectory_columns (
            id, survey_run_id, well_id, column_type, column_name, unit,
            is_input, is_calculated,
            min_value, max_value, mean_value, null_count, sample_count,
            parquet_hash, created_by
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
        "#,
        params![
            column_id.to_string(),
            survey_run_id.to_string(),
            well_id.to_string(),
            column_type.canonical_name(),
            column_type.canonical_name(),
            unit,
            0, // is_input = false
            1, // is_calculated = true
            min,
            max,
            mean,
            null_count as i64,
            count as i64,
            parquet_hash,
            created_by.to_string(),
        ],
    )?;

    Ok(IngestedTrajectoryColumn {
        column_id,
        column_type,
        column_name: column_type.canonical_name().to_string(),
        unit: Some(unit.to_string()),
        parquet_hash,
        sample_count: count,
        null_count,
        min_value: min,
        max_value: max,
        mean_value: mean,
        is_input: false,
        is_calculated: true,
    })
}

/// Create a simple Parquet file for a single column
fn create_column_parquet(column_name: &str, values: &[Option<f64>]) -> Result<Vec<u8>> {
    use arrow::array::Float64Array;
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use parquet::arrow::ArrowWriter;
    use std::sync::Arc;

    let schema = Schema::new(vec![Field::new(column_name, DataType::Float64, true)]);
    let schema_ref = Arc::new(schema);

    let array = Float64Array::from(values.to_vec());
    let batch = RecordBatch::try_new(schema_ref.clone(), vec![Arc::new(array)])
        .map_err(|e| crate::error::Error::InvalidData(e.to_string()))?;

    let mut buffer = Vec::new();
    {
        let mut writer = ArrowWriter::try_new(&mut buffer, schema_ref, None)
            .map_err(|e| crate::error::Error::InvalidData(e.to_string()))?;
        writer
            .write(&batch)
            .map_err(|e| crate::error::Error::InvalidData(e.to_string()))?;
        writer
            .close()
            .map_err(|e| crate::error::Error::InvalidData(e.to_string()))?;
    }

    Ok(buffer)
}

/// Register a blob in the blob_registry table
fn register_blob(conn: &Connection, hash: &str, size: usize) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO blob_registry (hash, size_bytes) VALUES (?1, ?2)",
        params![hash, size as i64],
    )?;
    Ok(())
}

/// Update well's depth range based on trajectory
fn update_well_depth_range_for_trajectory(
    conn: &Connection,
    well_id: Uuid,
    top_md: f64,
    bottom_md: f64,
) -> Result<()> {
    // Get current well depth range
    let (current_top, current_bottom): (Option<f64>, Option<f64>) = conn.query_row(
        "SELECT min_depth, max_depth FROM wells WHERE id = ?",
        params![well_id.to_string()],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;

    let new_top = match current_top {
        Some(t) => t.min(top_md),
        None => top_md,
    };
    let new_bottom = match current_bottom {
        Some(b) => b.max(bottom_md),
        None => bottom_md,
    };

    conn.execute(
        "UPDATE wells SET min_depth = ?, max_depth = ? WHERE id = ?",
        params![new_top, new_bottom, well_id.to_string()],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blob::BlobStore;
    use crate::db::open_memory_db;
    use std::io::Write;
    use tempfile::{NamedTempFile, TempDir};

    fn create_test_csv() -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "MD,INC,AZI").unwrap();
        writeln!(file, "0,0,0").unwrap();
        writeln!(file, "100,5,45").unwrap();
        writeln!(file, "200,10,45").unwrap();
        writeln!(file, "300,15,50").unwrap();
        file
    }

    #[test]
    fn test_ingest_trajectory_basic() {
        let conn = open_memory_db().unwrap();
        let blob_dir = TempDir::new().unwrap();
        let blob_store = BlobStore::new(blob_dir.path()).unwrap();

        // Create an account first
        let account_id = Uuid::new_v4();
        conn.execute(
            "INSERT INTO accounts (id, email, password_hash, name) VALUES (?, 'test@test.com', 'hash', 'Test')",
            params![account_id.to_string()],
        )
        .unwrap();

        // Create a workspace
        let workspace_id = Uuid::new_v4();
        conn.execute(
            "INSERT INTO workspaces (id, name, owner_account_id) VALUES (?, 'Test', ?)",
            params![workspace_id.to_string(), account_id.to_string()],
        )
        .unwrap();

        // Create a member
        let member_id = Uuid::new_v4();
        conn.execute(
            "INSERT INTO workspace_members (id, workspace_id, account_id, display_name, role) VALUES (?, ?, ?, 'Test User', 'owner')",
            params![member_id.to_string(), workspace_id.to_string(), account_id.to_string()],
        )
        .unwrap();

        let csv_file = create_test_csv();
        let options = TrajectoryIngestOptions::default();

        let result = ingest_trajectory_file(
            &conn,
            &blob_store,
            csv_file.path(),
            None,
            Some("Test Well"),
            workspace_id,
            member_id,
            None,
            &options,
        )
        .unwrap();

        assert!(result.well_created);
        assert_eq!(result.station_count, 4);
        assert!(result.columns.len() >= 7); // MD, INC, AZI + TVD, NS, EW, DLS

        // Verify survey run was created
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM survey_runs WHERE id = ?",
                params![result.survey_run_id.to_string()],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);

        // Verify columns were created
        let col_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM trajectory_columns WHERE survey_run_id = ?",
                params![result.survey_run_id.to_string()],
                |row| row.get(0),
            )
            .unwrap();
        assert!(col_count >= 7);
    }

    #[test]
    fn test_ingest_to_existing_well() {
        let conn = open_memory_db().unwrap();
        let blob_dir = TempDir::new().unwrap();
        let blob_store = BlobStore::new(blob_dir.path()).unwrap();

        // Create an account first
        let account_id = Uuid::new_v4();
        conn.execute(
            "INSERT INTO accounts (id, email, password_hash, name) VALUES (?, 'test2@test.com', 'hash', 'Test2')",
            params![account_id.to_string()],
        )
        .unwrap();

        // Create workspace
        let workspace_id = Uuid::new_v4();
        conn.execute(
            "INSERT INTO workspaces (id, name, owner_account_id) VALUES (?, 'Test', ?)",
            params![workspace_id.to_string(), account_id.to_string()],
        )
        .unwrap();

        // Create member
        let member_id = Uuid::new_v4();
        conn.execute(
            "INSERT INTO workspace_members (id, workspace_id, account_id, display_name, role) VALUES (?, ?, ?, 'Test User', 'owner')",
            params![member_id.to_string(), workspace_id.to_string(), account_id.to_string()],
        )
        .unwrap();

        // Create well
        let well_id = Uuid::new_v4();
        conn.execute(
            "INSERT INTO wells (id, workspace_id, name, depth_unit, depth_step, depth_origin, created_by) VALUES (?, ?, 'Existing Well', 'ft', 0.5, 0.0, ?)",
            params![well_id.to_string(), workspace_id.to_string(), member_id.to_string()],
        )
        .unwrap();

        let csv_file = create_test_csv();
        let options = TrajectoryIngestOptions::default();

        let result = ingest_trajectory_file(
            &conn,
            &blob_store,
            csv_file.path(),
            Some(well_id),
            None,
            workspace_id,
            member_id,
            None,
            &options,
        )
        .unwrap();

        assert!(!result.well_created);
        assert_eq!(result.well_id, well_id);
    }

    #[test]
    fn test_column_mapping_override() {
        let csv_data = "Depth,Angle1,Angle2\n0,0,0\n100,5,45";
        let parsed = parse_trajectory_csv(csv_data.as_bytes()).unwrap();

        // The columns won't be detected correctly, so we override
        let mappings = vec![
            ColumnMapping {
                index: 0,
                column_type: TrajectoryColumnType::MeasuredDepth,
                unit: Some("ft".to_string()),
            },
            ColumnMapping {
                index: 1,
                column_type: TrajectoryColumnType::Inclination,
                unit: Some("deg".to_string()),
            },
            ColumnMapping {
                index: 2,
                column_type: TrajectoryColumnType::Azimuth,
                unit: Some("deg".to_string()),
            },
        ];

        let columns = apply_column_mappings(&parsed.columns, Some(&mappings));

        assert_eq!(columns[0].detected.column_type, TrajectoryColumnType::MeasuredDepth);
        assert_eq!(columns[0].detected.confidence, 1.0);
        assert_eq!(columns[1].detected.column_type, TrajectoryColumnType::Inclination);
        assert_eq!(columns[2].detected.column_type, TrajectoryColumnType::Azimuth);
    }
}
