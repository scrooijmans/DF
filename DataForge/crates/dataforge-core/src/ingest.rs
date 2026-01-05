//! LAS File Ingest Service
//!
//! Handles the complete LAS file ingestion workflow:
//! 1. Parse LAS file
//! 2. Store raw file in blob storage
//! 3. Create log_run record
//! 4. For each curve:
//!    - Create native Parquet (original sampling)
//!    - Resample to well's depth grid
//!    - Create gridded Parquet (analysis-ready)
//!    - Lookup canonical property from mnemonic dictionary
//! 5. Update well's depth range
//!
//! This follows the architecture from:
//! - OSDU: Work Product → Work Product Component → Dataset hierarchy
//! - ColaNode: Content-addressed blob storage
//! - PWLS: Curve property dictionary with mnemonic aliasing

use crate::blob::BlobStore;
use crate::error::Result;
use crate::las::{parse_las_file, ParsedLasFile};
use crate::parquet::{
	create_gridded_parquet, create_native_parquet, NativeCurveData, ParquetOptions,
};
use crate::wellgrid::{resample_curve, DepthUnit, ResampleMethod, WellDepthGrid};
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::path::Path;
use tracing::info;
use uuid::Uuid;

/// Options for LAS file ingestion
#[derive(Debug, Clone)]
pub struct IngestOptions {
	/// Resampling method to use
	pub resample_method: ResampleMethod,
	/// Parquet compression options
	pub parquet_options: ParquetOptions,
	/// Whether to store raw LAS file
	pub store_raw_file: bool,
}

impl Default for IngestOptions {
	fn default() -> Self {
		Self {
			resample_method: ResampleMethod::Linear,
			parquet_options: ParquetOptions::default(),
			store_raw_file: true,
		}
	}
}

/// Result of ingesting a single curve
#[derive(Debug)]
pub struct IngestedCurve {
	pub curve_id: Uuid,
	pub mnemonic: String,
	pub property_id: Option<String>,
	pub native_parquet_hash: String,
	pub gridded_parquet_hash: String,
	pub native_top_depth: f64,
	pub native_bottom_depth: f64,
	pub gridded_top_depth: f64,
	pub gridded_bottom_depth: f64,
	pub row_count: usize,
	pub null_count: usize,
	pub min_value: Option<f64>,
	pub max_value: Option<f64>,
	pub mean_value: Option<f64>,
}

/// Result of ingesting a LAS file
#[derive(Debug)]
pub struct IngestResult {
	pub log_run_id: Uuid,
	pub well_id: Uuid,
	pub source_file_hash: String,
	pub raw_file_blob_hash: Option<String>,
	pub curves: Vec<IngestedCurve>,
	/// Whether a new well was created
	pub well_created: bool,
}

/// Ingest a LAS file into a well
///
/// # Arguments
/// * `conn` - SQLite database connection
/// * `blob_store` - Blob storage for Parquet files
/// * `las_path` - Path to the LAS file
/// * `well_id` - Existing well ID, or None to create a new well
/// * `workspace_id` - Workspace to add the well/curves to
/// * `created_by` - Member ID creating this data
/// * `options` - Ingest options
pub fn ingest_las_file(
	conn: &Connection,
	blob_store: &BlobStore,
	las_path: &Path,
	well_id: Option<Uuid>,
	workspace_id: Uuid,
	created_by: Uuid,
	options: &IngestOptions,
) -> Result<IngestResult> {
	// 1. Read and hash raw LAS file
	let raw_bytes = std::fs::read(las_path)?;
	let source_file_hash = hex::encode(Sha256::digest(&raw_bytes));

	// 2. Parse LAS file
	let parsed = parse_las_file(las_path)?;

	// 3. Get or create well
	let (well_id, well_created) = get_or_create_well(
		conn,
		well_id,
		workspace_id,
		created_by,
		&parsed,
	)?;

	// 4. Get well's depth grid
	let grid = get_well_depth_grid(conn, well_id)?;

	// 5. Store raw LAS file if requested
	let raw_file_blob_hash = if options.store_raw_file {
		let hash = blob_store.store(&raw_bytes)?;
		register_blob(conn, &hash, raw_bytes.len())?;
		Some(hash)
	} else {
		None
	};

	// 6. Create log_run record
	let log_run_id = create_log_run(
		conn,
		well_id,
		workspace_id,
		las_path,
		&source_file_hash,
		raw_file_blob_hash.as_deref(),
		&parsed,
		created_by,
	)?;

	// 7. Find depth curve index
	let depth_curve_idx = parsed
		.curves
		.iter()
		.position(|c| c.mnemonic.to_uppercase() == "DEPT" || c.mnemonic.to_uppercase() == "DEPTH")
		.ok_or_else(|| crate::error::Error::InvalidData("No depth curve found".to_string()))?;

	let depth_values = &parsed.curves[depth_curve_idx].values;

	// Detect source depth unit from the curve
	let source_unit = detect_depth_unit(&parsed.curves[depth_curve_idx].unit);

	// 8. Process each curve (except depth)
	let mut ingested_curves = Vec::new();
	let mut min_depth_overall = f64::INFINITY;
	let mut max_depth_overall = f64::NEG_INFINITY;

	for (idx, curve) in parsed.curves.iter().enumerate() {
		if idx == depth_curve_idx {
			continue; // Skip the depth curve itself
		}

		let ingested = ingest_curve(
			conn,
			blob_store,
			log_run_id,
			well_id,
			&curve.mnemonic,
			&curve.unit,
			depth_values,
			&curve.values,
			source_unit,
			&grid,
			&options,
		)?;

		// Track overall depth range
		min_depth_overall = min_depth_overall.min(ingested.gridded_top_depth);
		max_depth_overall = max_depth_overall.max(ingested.gridded_bottom_depth);

		ingested_curves.push(ingested);
	}

	// 9. Update well's depth range
	if !ingested_curves.is_empty() {
		update_well_depth_range(conn, well_id, min_depth_overall, max_depth_overall)?;
	}

	info!(
		log_run_id = %log_run_id,
		well_id = %well_id,
		curves = ingested_curves.len(),
		"Ingested LAS file"
	);

	Ok(IngestResult {
		log_run_id,
		well_id,
		source_file_hash,
		raw_file_blob_hash,
		curves: ingested_curves,
		well_created,
	})
}

/// Get or create a well for the LAS file
fn get_or_create_well(
	conn: &Connection,
	well_id: Option<Uuid>,
	workspace_id: Uuid,
	created_by: Uuid,
	parsed: &ParsedLasFile,
) -> Result<(Uuid, bool)> {
	if let Some(id) = well_id {
		// Verify well exists
		let exists: bool = conn.query_row(
			"SELECT 1 FROM wells WHERE id = ? AND deleted_at IS NULL",
			params![id.to_string()],
			|_| Ok(true),
		).unwrap_or(false);

		if !exists {
			return Err(crate::error::Error::InvalidData(
				format!("Well {} not found", id)
			));
		}
		Ok((id, false))
	} else {
		// Create new well with default 0.5 ft grid
		let new_id = Uuid::new_v4();
		let well_name = parsed.well_name.clone()
			.unwrap_or_else(|| parsed.filename.clone());

		conn.execute(
			r#"
			INSERT INTO wells (
				id, workspace_id, name, uwi, field, company, location,
				depth_unit, depth_step, depth_origin,
				created_by
			)
			VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
			"#,
			params![
				new_id.to_string(),
				workspace_id.to_string(),
				well_name,
				parsed.uwi,
				parsed.field,
				parsed.company,
				parsed.location,
				"ft",
				0.5,
				0.0,
				created_by.to_string(),
			],
		)?;

		info!(well_id = %new_id, name = well_name, "Created new well");
		Ok((new_id, true))
	}
}

/// Get the depth grid for a well
fn get_well_depth_grid(conn: &Connection, well_id: Uuid) -> Result<WellDepthGrid> {
	let (unit_str, step, origin): (String, f64, f64) = conn.query_row(
		"SELECT depth_unit, depth_step, depth_origin FROM wells WHERE id = ?",
		params![well_id.to_string()],
		|row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
	)?;

	let unit = match unit_str.to_lowercase().as_str() {
		"m" | "meters" | "metre" => DepthUnit::Meters,
		_ => DepthUnit::Feet,
	};

	Ok(WellDepthGrid::new(unit, step, origin))
}

/// Create a log_run record for the LAS import
fn create_log_run(
	conn: &Connection,
	well_id: Uuid,
	workspace_id: Uuid,
	las_path: &Path,
	source_file_hash: &str,
	raw_file_blob_hash: Option<&str>,
	parsed: &ParsedLasFile,
	imported_by: Uuid,
) -> Result<Uuid> {
	let log_run_id = Uuid::new_v4();
	let filename = las_path.file_name()
		.and_then(|n| n.to_str())
		.unwrap_or("unknown.las");

	conn.execute(
		r#"
		INSERT INTO log_runs (
			id, well_id, workspace_id, source_filename, source_file_hash, raw_file_blob_hash,
			original_top_depth, original_bottom_depth, original_step, original_null_value,
			las_version, imported_by
		)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
		"#,
		params![
			log_run_id.to_string(),
			well_id.to_string(),
			workspace_id.to_string(),
			filename,
			source_file_hash,
			raw_file_blob_hash,
			parsed.start_depth,
			parsed.stop_depth,
			parsed.step,
			parsed.null_value,
			parsed.version,
			imported_by.to_string(),
		],
	)?;

	Ok(log_run_id)
}

/// Ingest a single curve
fn ingest_curve(
	conn: &Connection,
	blob_store: &BlobStore,
	log_run_id: Uuid,
	well_id: Uuid,
	mnemonic: &str,
	unit: &str,
	depth_values: &[f64],
	curve_values: &[f64],
	source_unit: DepthUnit,
	target_grid: &WellDepthGrid,
	options: &IngestOptions,
) -> Result<IngestedCurve> {
	let curve_id = Uuid::new_v4();

	// 1. Create native Parquet (original data)
	let native_data = NativeCurveData {
		depths: depth_values.to_vec(),
		values: curve_values.to_vec(),
		mnemonic: mnemonic.to_string(),
	};
	let native_parquet = create_native_parquet(&native_data, &options.parquet_options)?;
	let native_hash = blob_store.store(&native_parquet)?;
	register_blob(conn, &native_hash, native_parquet.len())?;

	// Calculate native depth range
	let native_top = depth_values.iter().copied().fold(f64::INFINITY, f64::min);
	let native_bottom = depth_values.iter().copied().fold(f64::NEG_INFINITY, f64::max);

	// 2. Resample to grid
	let resampled = resample_curve(
		depth_values,
		curve_values,
		source_unit,
		target_grid,
		options.resample_method,
	)?;

	// 3. Create gridded Parquet
	let gridded_parquet = create_gridded_parquet(&resampled, mnemonic, &options.parquet_options)?;
	let gridded_hash = blob_store.store(&gridded_parquet)?;
	register_blob(conn, &gridded_hash, gridded_parquet.len())?;

	// 4. Lookup canonical property from mnemonic dictionary
	let property_id: Option<String> = conn.query_row(
		"SELECT property_id FROM curve_mnemonics WHERE mnemonic = ? COLLATE NOCASE",
		params![mnemonic],
		|row| row.get(0),
	).ok();

	// 5. Determine if unit conversion was applied
	let was_unit_converted = source_unit != target_grid.unit;

	// 6. Insert curve record
	conn.execute(
		r#"
		INSERT INTO curves (
			id, log_run_id, well_id, mnemonic, property_id, original_unit,
			native_top_depth, native_bottom_depth, native_step, native_parquet_hash,
			gridded_top_depth, gridded_bottom_depth, resample_method, was_unit_converted, gridded_parquet_hash,
			row_count, null_count, min_value, max_value, mean_value
		)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)
		"#,
		params![
			curve_id.to_string(),
			log_run_id.to_string(),
			well_id.to_string(),
			mnemonic,
			property_id,
			unit,
			native_top,
			native_bottom,
			None::<f64>, // native_step (could calculate if regular)
			native_hash,
			resampled.policy.snapped_start,
			resampled.policy.snapped_end,
			options.resample_method.to_string(),
			was_unit_converted,
			gridded_hash,
			resampled.sample_count as i64,
			resampled.null_count as i64,
			resampled.min_value,
			resampled.max_value,
			resampled.mean_value,
		],
	)?;

	info!(
		curve_id = %curve_id,
		mnemonic = mnemonic,
		property_id = ?property_id,
		samples = resampled.sample_count,
		nulls = resampled.null_count,
		"Ingested curve"
	);

	Ok(IngestedCurve {
		curve_id,
		mnemonic: mnemonic.to_string(),
		property_id,
		native_parquet_hash: native_hash,
		gridded_parquet_hash: gridded_hash,
		native_top_depth: native_top,
		native_bottom_depth: native_bottom,
		gridded_top_depth: resampled.policy.snapped_start,
		gridded_bottom_depth: resampled.policy.snapped_end,
		row_count: resampled.sample_count,
		null_count: resampled.null_count,
		min_value: resampled.min_value,
		max_value: resampled.max_value,
		mean_value: resampled.mean_value,
	})
}

/// Register a blob in the blob_registry table
fn register_blob(conn: &Connection, hash: &str, size: usize) -> Result<()> {
	conn.execute(
		"INSERT OR IGNORE INTO blob_registry (hash, size_bytes) VALUES (?1, ?2)",
		params![hash, size as i64],
	)?;
	Ok(())
}

/// Update well's overall depth range
fn update_well_depth_range(
	conn: &Connection,
	well_id: Uuid,
	min_depth: f64,
	max_depth: f64,
) -> Result<()> {
	conn.execute(
		r#"
		UPDATE wells SET
			min_depth = CASE
				WHEN min_depth IS NULL THEN ?2
				WHEN ?2 < min_depth THEN ?2
				ELSE min_depth
			END,
			max_depth = CASE
				WHEN max_depth IS NULL THEN ?3
				WHEN ?3 > max_depth THEN ?3
				ELSE max_depth
			END,
			updated_at = datetime('now'),
			version = version + 1
		WHERE id = ?1
		"#,
		params![well_id.to_string(), min_depth, max_depth],
	)?;
	Ok(())
}

/// Detect depth unit from string
fn detect_depth_unit(unit_str: &str) -> DepthUnit {
	match unit_str.to_lowercase().as_str() {
		"m" | "meters" | "metre" | "metres" => DepthUnit::Meters,
		"ft" | "feet" | "f" => DepthUnit::Feet,
		_ => DepthUnit::Feet, // Default to feet
	}
}

// ============================================================================
// CURVE EDITING
// ============================================================================

use crate::parquet::{
	update_parquet_values, add_parquet_rows, delete_parquet_rows,
	read_parquet_columns, CellEdit,
};
use std::collections::HashMap;

/// Result of a curve data update operation
#[derive(Debug)]
pub struct CurveUpdateResult {
	/// The new parquet hash after edits
	pub new_native_hash: String,
	/// Number of edits applied
	pub edits_applied: usize,
	/// New row count
	pub row_count: usize,
	/// New version number
	pub new_version: i64,
}

/// Get curve info by ID
pub fn get_curve_info(
	conn: &Connection,
	curve_id: Uuid,
) -> Result<(String, String, i64)> {
	let (native_hash, mnemonic, curve_version): (Option<String>, String, i64) = conn.query_row(
		"SELECT native_parquet_hash, mnemonic, version FROM curves WHERE id = ?1",
		params![curve_id.to_string()],
		|row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
	)?;

	let native_hash = native_hash.ok_or_else(|| {
		crate::error::Error::InvalidData("Curve has no native parquet hash".to_string())
	})?;

	// Get max version from curve_versions to avoid UNIQUE constraint violations
	// This handles cases where curve.version and curve_versions might be out of sync
	let max_version: i64 = conn
		.query_row(
			"SELECT COALESCE(MAX(version), 0) FROM curve_versions WHERE curve_id = ?1",
			params![curve_id.to_string()],
			|row| row.get(0),
		)
		.unwrap_or(0);

	// Use the higher of the two versions to be safe
	let version = curve_version.max(max_version);

	Ok((native_hash, mnemonic, version))
}

/// Update curve data with cell edits
///
/// This applies a batch of cell edits to a curve's native Parquet file:
/// 1. Reads current Parquet from blob store
/// 2. Applies cell edits
/// 3. Stores new Parquet blob
/// 4. Creates version record
/// 5. Updates curve metadata to point to new blob
pub fn update_curve_data(
	conn: &Connection,
	blob_store: &BlobStore,
	curve_id: Uuid,
	edits: &[CellEdit],
	reason: &str,
	user_id: Option<&str>,
) -> Result<CurveUpdateResult> {
	// Get current curve info
	let (native_hash, mnemonic, current_version) = get_curve_info(conn, curve_id)?;

	// Read current Parquet from blob store
	let original_data = blob_store.get(&native_hash)?;

	// Apply edits
	let parquet_options = ParquetOptions::default();
	let result = update_parquet_values(&original_data, edits, &parquet_options)?;

	// Store new blob
	let new_hash = blob_store.store(&result.data)?;
	register_blob(conn, &new_hash, result.data.len())?;

	// Calculate statistics for the edited data
	let (column_names, columns) = read_parquet_columns(&result.data)?;
	let value_col_idx = column_names.iter().position(|n| n == &mnemonic).unwrap_or(1);
	let values = &columns[value_col_idx];

	let mut min_val: Option<f64> = None;
	let mut max_val: Option<f64> = None;
	let mut sum = 0.0;
	let mut count = 0;
	let mut null_count = 0;

	for val in values {
		match val {
			Some(v) => {
				min_val = Some(min_val.map(|m| m.min(*v)).unwrap_or(*v));
				max_val = Some(max_val.map(|m| m.max(*v)).unwrap_or(*v));
				sum += v;
				count += 1;
			}
			None => null_count += 1,
		}
	}
	let mean_val = if count > 0 { Some(sum / count as f64) } else { None };

	// Increment version
	let new_version = current_version + 1;

	// Create version record (preserves history)
	let version_id = Uuid::new_v4();
	conn.execute(
		r#"
		INSERT INTO curve_versions (
			id, curve_id, version, reason, created_by,
			native_parquet_hash, min_value, max_value, mean_value, null_count
		)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
		"#,
		params![
			version_id.to_string(),
			curve_id.to_string(),
			new_version,
			reason,
			user_id,
			new_hash,
			min_val,
			max_val,
			mean_val,
			null_count as i64,
		],
	)?;

	// Update curve record
	conn.execute(
		r#"
		UPDATE curves SET
			native_parquet_hash = ?2,
			version = ?3,
			min_value = ?4,
			max_value = ?5,
			mean_value = ?6,
			null_count = ?7,
			quality_flag = 'edited',
			updated_at = datetime('now')
		WHERE id = ?1
		"#,
		params![
			curve_id.to_string(),
			new_hash,
			new_version,
			min_val,
			max_val,
			mean_val,
			null_count as i64,
		],
	)?;

	info!(
		curve_id = %curve_id,
		edits = result.edits_applied,
		old_hash = %native_hash,
		new_hash = %new_hash,
		version = new_version,
		"Updated curve data"
	);

	Ok(CurveUpdateResult {
		new_native_hash: new_hash,
		edits_applied: result.edits_applied,
		row_count: result.row_count,
		new_version,
	})
}

/// Add rows to a curve's native Parquet file
pub fn add_curve_rows(
	conn: &Connection,
	blob_store: &BlobStore,
	curve_id: Uuid,
	new_rows: &[HashMap<String, Option<f64>>],
	depth_column: &str,
	reason: &str,
	user_id: Option<&str>,
) -> Result<CurveUpdateResult> {
	// Get current curve info
	let (native_hash, mnemonic, current_version) = get_curve_info(conn, curve_id)?;

	// Read current Parquet from blob store
	let original_data = blob_store.get(&native_hash)?;

	// Apply row additions
	let parquet_options = ParquetOptions::default();
	let result = add_parquet_rows(&original_data, new_rows, depth_column, &parquet_options)?;

	// Store new blob
	let new_hash = blob_store.store(&result.data)?;
	register_blob(conn, &new_hash, result.data.len())?;

	// Calculate statistics
	let (column_names, columns) = read_parquet_columns(&result.data)?;
	let value_col_idx = column_names.iter().position(|n| n == &mnemonic).unwrap_or(1);
	let values = &columns[value_col_idx];

	let mut min_val: Option<f64> = None;
	let mut max_val: Option<f64> = None;
	let mut sum = 0.0;
	let mut count = 0;
	let mut null_count = 0;

	for val in values {
		match val {
			Some(v) => {
				min_val = Some(min_val.map(|m| m.min(*v)).unwrap_or(*v));
				max_val = Some(max_val.map(|m| m.max(*v)).unwrap_or(*v));
				sum += v;
				count += 1;
			}
			None => null_count += 1,
		}
	}
	let mean_val = if count > 0 { Some(sum / count as f64) } else { None };

	// Increment version
	let new_version = current_version + 1;

	// Create version record
	let version_id = Uuid::new_v4();
	conn.execute(
		r#"
		INSERT INTO curve_versions (
			id, curve_id, version, reason, created_by,
			native_parquet_hash, min_value, max_value, mean_value, null_count
		)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
		"#,
		params![
			version_id.to_string(),
			curve_id.to_string(),
			new_version,
			reason,
			user_id,
			new_hash,
			min_val,
			max_val,
			mean_val,
			null_count as i64,
		],
	)?;

	// Update curve record
	conn.execute(
		r#"
		UPDATE curves SET
			native_parquet_hash = ?2,
			version = ?3,
			native_sample_count = ?4,
			min_value = ?5,
			max_value = ?6,
			mean_value = ?7,
			null_count = ?8,
			quality_flag = 'edited',
			updated_at = datetime('now')
		WHERE id = ?1
		"#,
		params![
			curve_id.to_string(),
			new_hash,
			new_version,
			result.row_count as i64,
			min_val,
			max_val,
			mean_val,
			null_count as i64,
		],
	)?;

	info!(
		curve_id = %curve_id,
		rows_added = result.edits_applied,
		total_rows = result.row_count,
		version = new_version,
		"Added rows to curve"
	);

	Ok(CurveUpdateResult {
		new_native_hash: new_hash,
		edits_applied: result.edits_applied,
		row_count: result.row_count,
		new_version,
	})
}

/// Delete rows from a curve's native Parquet file
pub fn delete_curve_rows(
	conn: &Connection,
	blob_store: &BlobStore,
	curve_id: Uuid,
	row_indices: &[usize],
	reason: &str,
	user_id: Option<&str>,
) -> Result<CurveUpdateResult> {
	// Get current curve info
	let (native_hash, mnemonic, current_version) = get_curve_info(conn, curve_id)?;

	// Read current Parquet from blob store
	let original_data = blob_store.get(&native_hash)?;

	// Apply row deletions
	let parquet_options = ParquetOptions::default();
	let result = delete_parquet_rows(&original_data, row_indices, &parquet_options)?;

	// Store new blob
	let new_hash = blob_store.store(&result.data)?;
	register_blob(conn, &new_hash, result.data.len())?;

	// Calculate statistics
	let (column_names, columns) = read_parquet_columns(&result.data)?;
	let value_col_idx = column_names.iter().position(|n| n == &mnemonic).unwrap_or(1);
	let values = &columns[value_col_idx];

	let mut min_val: Option<f64> = None;
	let mut max_val: Option<f64> = None;
	let mut sum = 0.0;
	let mut count = 0;
	let mut null_count = 0;

	for val in values {
		match val {
			Some(v) => {
				min_val = Some(min_val.map(|m| m.min(*v)).unwrap_or(*v));
				max_val = Some(max_val.map(|m| m.max(*v)).unwrap_or(*v));
				sum += v;
				count += 1;
			}
			None => null_count += 1,
		}
	}
	let mean_val = if count > 0 { Some(sum / count as f64) } else { None };

	// Increment version
	let new_version = current_version + 1;

	// Create version record
	let version_id = Uuid::new_v4();
	conn.execute(
		r#"
		INSERT INTO curve_versions (
			id, curve_id, version, reason, created_by,
			native_parquet_hash, min_value, max_value, mean_value, null_count
		)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
		"#,
		params![
			version_id.to_string(),
			curve_id.to_string(),
			new_version,
			reason,
			user_id,
			new_hash,
			min_val,
			max_val,
			mean_val,
			null_count as i64,
		],
	)?;

	// Update curve record
	conn.execute(
		r#"
		UPDATE curves SET
			native_parquet_hash = ?2,
			version = ?3,
			native_sample_count = ?4,
			min_value = ?5,
			max_value = ?6,
			mean_value = ?7,
			null_count = ?8,
			quality_flag = 'edited',
			updated_at = datetime('now')
		WHERE id = ?1
		"#,
		params![
			curve_id.to_string(),
			new_hash,
			new_version,
			result.row_count as i64,
			min_val,
			max_val,
			mean_val,
			null_count as i64,
		],
	)?;

	info!(
		curve_id = %curve_id,
		rows_deleted = result.edits_applied,
		remaining_rows = result.row_count,
		version = new_version,
		"Deleted rows from curve"
	);

	Ok(CurveUpdateResult {
		new_native_hash: new_hash,
		edits_applied: result.edits_applied,
		row_count: result.row_count,
		new_version,
	})
}

/// Get curve version history
pub fn get_curve_versions(
	conn: &Connection,
	curve_id: Uuid,
) -> Result<Vec<CurveVersion>> {
	let mut stmt = conn.prepare(
		r#"
		SELECT id, version, created_at, created_by, reason,
			   native_parquet_hash, min_value, max_value, mean_value, null_count
		FROM curve_versions
		WHERE curve_id = ?1
		ORDER BY version DESC
		"#,
	)?;

	let versions = stmt.query_map(params![curve_id.to_string()], |row| {
		Ok(CurveVersion {
			id: row.get(0)?,
			version: row.get(1)?,
			created_at: row.get(2)?,
			created_by: row.get(3)?,
			reason: row.get(4)?,
			native_parquet_hash: row.get(5)?,
			min_value: row.get(6)?,
			max_value: row.get(7)?,
			mean_value: row.get(8)?,
			null_count: row.get(9)?,
		})
	})?.collect::<std::result::Result<Vec<_>, _>>()?;

	Ok(versions)
}

/// A version record for a curve
#[derive(Debug)]
pub struct CurveVersion {
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

/// Revert a curve to a previous version
pub fn revert_curve_to_version(
	conn: &Connection,
	curve_id: Uuid,
	target_version: i64,
	user_id: Option<&str>,
) -> Result<CurveUpdateResult> {
	// Get the target version record
	let (hash, min_val, max_val, mean_val, null_count): (
		Option<String>, Option<f64>, Option<f64>, Option<f64>, Option<i64>
	) = conn.query_row(
		r#"
		SELECT native_parquet_hash, min_value, max_value, mean_value, null_count
		FROM curve_versions
		WHERE curve_id = ?1 AND version = ?2
		"#,
		params![curve_id.to_string(), target_version],
		|row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
	)?;

	let native_hash = hash.ok_or_else(|| {
		crate::error::Error::InvalidData(format!(
			"Version {} has no parquet hash",
			target_version
		))
	})?;

	// Get current version
	let current_version: i64 = conn.query_row(
		"SELECT version FROM curves WHERE id = ?1",
		params![curve_id.to_string()],
		|row| row.get(0),
	)?;

	let new_version = current_version + 1;

	// Create version record for the revert
	let version_id = Uuid::new_v4();
	conn.execute(
		r#"
		INSERT INTO curve_versions (
			id, curve_id, version, reason, created_by,
			native_parquet_hash, min_value, max_value, mean_value, null_count
		)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
		"#,
		params![
			version_id.to_string(),
			curve_id.to_string(),
			new_version,
			format!("revert to version {}", target_version),
			user_id,
			native_hash,
			min_val,
			max_val,
			mean_val,
			null_count,
		],
	)?;

	// Update curve record
	conn.execute(
		r#"
		UPDATE curves SET
			native_parquet_hash = ?2,
			version = ?3,
			min_value = ?4,
			max_value = ?5,
			mean_value = ?6,
			null_count = ?7,
			updated_at = datetime('now')
		WHERE id = ?1
		"#,
		params![
			curve_id.to_string(),
			native_hash,
			new_version,
			min_val,
			max_val,
			mean_val,
			null_count,
		],
	)?;

	info!(
		curve_id = %curve_id,
		reverted_to = target_version,
		new_version = new_version,
		"Reverted curve to previous version"
	);

	Ok(CurveUpdateResult {
		new_native_hash: native_hash,
		edits_applied: 0,
		row_count: 0, // Would need to read from parquet to get this
		new_version,
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::db::init_db;
	use tempfile::TempDir;

	fn setup_test_env() -> (TempDir, Connection, BlobStore) {
		let temp_dir = TempDir::new().unwrap();
		let db_path = temp_dir.path().join("test.db");
		let blob_path = temp_dir.path().join("blobs");

		let conn = Connection::open(&db_path).unwrap();
		init_db(&conn).unwrap();

		let blob_store = BlobStore::new(&blob_path).unwrap();

		(temp_dir, conn, blob_store)
	}

	#[test]
	fn test_detect_depth_unit() {
		assert_eq!(detect_depth_unit("M"), DepthUnit::Meters);
		assert_eq!(detect_depth_unit("meters"), DepthUnit::Meters);
		assert_eq!(detect_depth_unit("FT"), DepthUnit::Feet);
		assert_eq!(detect_depth_unit("feet"), DepthUnit::Feet);
		assert_eq!(detect_depth_unit("unknown"), DepthUnit::Feet); // Default
	}

	#[test]
	fn test_get_well_depth_grid() {
		let (_temp_dir, conn, _blob_store) = setup_test_env();

		// Create a workspace and member first
		let workspace_id = Uuid::new_v4();
		let account_id = Uuid::new_v4();
		let member_id = Uuid::new_v4();

		conn.execute(
			"INSERT INTO accounts (id, email, password_hash, name) VALUES (?1, ?2, ?3, ?4)",
			params![account_id.to_string(), "test@example.com", "hash", "Test User"],
		).unwrap();

		conn.execute(
			"INSERT INTO workspaces (id, name, owner_account_id) VALUES (?1, ?2, ?3)",
			params![workspace_id.to_string(), "Test Workspace", account_id.to_string()],
		).unwrap();

		conn.execute(
			"INSERT INTO workspace_members (id, workspace_id, account_id, role) VALUES (?1, ?2, ?3, ?4)",
			params![member_id.to_string(), workspace_id.to_string(), account_id.to_string(), "owner"],
		).unwrap();

		// Create a well
		let well_id = Uuid::new_v4();
		conn.execute(
			r#"
			INSERT INTO wells (id, workspace_id, name, depth_unit, depth_step, depth_origin, created_by)
			VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
			"#,
			params![well_id.to_string(), workspace_id.to_string(), "Test Well", "ft", 0.5, 0.0, member_id.to_string()],
		).unwrap();

		let grid = get_well_depth_grid(&conn, well_id).unwrap();
		assert_eq!(grid.unit, DepthUnit::Feet);
		assert!((grid.step - 0.5).abs() < 0.001);
		assert!((grid.origin - 0.0).abs() < 0.001);
	}
}
