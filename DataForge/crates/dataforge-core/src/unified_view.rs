//! Unified View Builder
//!
//! Generates wide-format Parquet files that combine multiple curves
//! aligned to the same depth grid. Views are chunked by depth range
//! for efficient memory usage.
//!
//! # Architecture
//!
//! - Each well can have multiple unified view chunks
//! - Chunks are 5000 depth points by default (~2500 ft at 0.5 ft step)
//! - Views are built lazily and invalidated when curves change
//! - All curves are resampled to the well's canonical grid

use crate::blob::BlobStore;
use crate::error::Result;
use crate::parquet::{create_unified_view_parquet, ParquetOptions, UnifiedViewData};
use crate::wellgrid::WellDepthGrid;
use rusqlite::{params, Connection};
use tracing::info;
use uuid::Uuid;

/// Default chunk size in depth points
pub const DEFAULT_CHUNK_SIZE: usize = 5000;

/// Options for unified view generation
#[derive(Debug, Clone)]
pub struct UnifiedViewOptions {
	/// Number of depth points per chunk
	pub chunk_size: usize,
	/// Parquet compression options
	pub parquet_options: ParquetOptions,
}

impl Default for UnifiedViewOptions {
	fn default() -> Self {
		Self {
			chunk_size: DEFAULT_CHUNK_SIZE,
			parquet_options: ParquetOptions::default(),
		}
	}
}

/// Result of building unified views for a well
#[derive(Debug)]
pub struct UnifiedViewResult {
	pub well_id: Uuid,
	pub chunks_created: usize,
	pub chunks_updated: usize,
	pub chunks_skipped: usize,
	pub total_rows: usize,
}

/// Build or update unified views for a well
///
/// This function:
/// 1. Gets all gridded curves for the well
/// 2. Determines the overall depth range
/// 3. Divides into chunks
/// 4. For each chunk, creates a wide-format Parquet with all curves
pub fn build_unified_views(
	conn: &Connection,
	blob_store: &BlobStore,
	well_id: Uuid,
	options: &UnifiedViewOptions,
) -> Result<UnifiedViewResult> {
	// 1. Get well's depth grid
	let grid = get_well_depth_grid(conn, well_id)?;

	// 2. Get overall depth range from curves
	let (min_depth, max_depth) = get_well_depth_range(conn, well_id)?;

	if min_depth.is_none() || max_depth.is_none() {
		info!(well_id = %well_id, "No curves with depth data for unified view");
		return Ok(UnifiedViewResult {
			well_id,
			chunks_created: 0,
			chunks_updated: 0,
			chunks_skipped: 0,
			total_rows: 0,
		});
	}

	let min_depth = min_depth.unwrap();
	let max_depth = max_depth.unwrap();

	// 3. Get all gridded curve data
	let curves = get_gridded_curves(conn, well_id)?;
	if curves.is_empty() {
		info!(well_id = %well_id, "No gridded curves for unified view");
		return Ok(UnifiedViewResult {
			well_id,
			chunks_created: 0,
			chunks_updated: 0,
			chunks_skipped: 0,
			total_rows: 0,
		});
	}

	// 4. Get current source version (max curve version)
	let source_version = get_max_curve_version(conn, well_id)?;

	// 5. Calculate chunks
	let start_idx = grid.depth_to_index(min_depth);
	let end_idx = grid.depth_to_index(max_depth);
	let total_points = (end_idx - start_idx + 1) as usize;
	let num_chunks = (total_points + options.chunk_size - 1) / options.chunk_size;

	let workspace_id = get_well_workspace(conn, well_id)?;

	let mut chunks_created = 0;
	let mut chunks_updated = 0;
	let mut chunks_skipped = 0;
	let mut total_rows = 0;

	// 6. Build each chunk
	for chunk_idx in 0..num_chunks {
		let chunk_start_idx = start_idx + (chunk_idx * options.chunk_size) as i64;
		let chunk_end_idx = (chunk_start_idx + options.chunk_size as i64 - 1).min(end_idx);

		let chunk_start_depth = grid.index_to_depth(chunk_start_idx);
		let chunk_end_depth = grid.index_to_depth(chunk_end_idx);

		// Check if chunk already exists and is up to date
		let existing = get_existing_chunk(conn, well_id, chunk_idx as i32)?;
		if let Some((existing_id, existing_version)) = existing {
			if existing_version >= source_version {
				chunks_skipped += 1;
				continue;
			}
			// Delete old chunk (we'll recreate it)
			delete_chunk(conn, existing_id)?;
			chunks_updated += 1;
		} else {
			chunks_created += 1;
		}

		// Build chunk data
		let chunk_data = build_chunk_data(
			conn,
			&grid,
			&curves,
			chunk_start_idx,
			chunk_end_idx,
		)?;

		let row_count = chunk_data.depths.len();
		total_rows += row_count;

		// Create Parquet file
		let parquet_data = create_unified_view_parquet(&chunk_data, &options.parquet_options)?;
		let parquet_hash = blob_store.store(&parquet_data)?;

		// Register blob
		register_blob(conn, &parquet_hash, parquet_data.len())?;

		// Create unified_views record
		let view_id = Uuid::new_v4();
		let curve_ids: Vec<String> = curves.iter().map(|c| c.id.to_string()).collect();

		conn.execute(
			r#"
			INSERT INTO unified_views (
				id, well_id, workspace_id, chunk_index, start_depth, end_depth,
				curve_ids, row_count, parquet_hash, status, source_version
			)
			VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
			"#,
			params![
				view_id.to_string(),
				well_id.to_string(),
				workspace_id.to_string(),
				chunk_idx as i32,
				chunk_start_depth,
				chunk_end_depth,
				serde_json::to_string(&curve_ids)?,
				row_count as i64,
				parquet_hash,
				"ready",
				source_version,
			],
		)?;

		info!(
			view_id = %view_id,
			well_id = %well_id,
			chunk = chunk_idx,
			rows = row_count,
			"Created unified view chunk"
		);
	}

	Ok(UnifiedViewResult {
		well_id,
		chunks_created,
		chunks_updated,
		chunks_skipped,
		total_rows,
	})
}

/// Mark all unified views for a well as stale (needing rebuild)
pub fn invalidate_unified_views(conn: &Connection, well_id: Uuid) -> Result<usize> {
	let affected = conn.execute(
		"UPDATE unified_views SET status = 'stale', updated_at = datetime('now') WHERE well_id = ?",
		params![well_id.to_string()],
	)?;
	Ok(affected)
}

/// Curve info for building views
#[derive(Debug)]
struct CurveInfo {
	id: Uuid,
	mnemonic: String,
	gridded_parquet_hash: String,
}

/// Get the well's depth grid
fn get_well_depth_grid(conn: &Connection, well_id: Uuid) -> Result<WellDepthGrid> {
	let (unit_str, step, origin): (String, f64, f64) = conn.query_row(
		"SELECT depth_unit, depth_step, depth_origin FROM wells WHERE id = ?",
		params![well_id.to_string()],
		|row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
	)?;

	let unit = match unit_str.to_lowercase().as_str() {
		"m" | "meters" | "metre" => crate::wellgrid::DepthUnit::Meters,
		_ => crate::wellgrid::DepthUnit::Feet,
	};

	Ok(WellDepthGrid::new(unit, step, origin))
}

/// Get the overall depth range for a well from its curves
fn get_well_depth_range(conn: &Connection, well_id: Uuid) -> Result<(Option<f64>, Option<f64>)> {
	let result = conn.query_row(
		r#"
		SELECT MIN(gridded_top_depth), MAX(gridded_bottom_depth)
		FROM curves
		WHERE well_id = ? AND deleted_at IS NULL AND gridded_parquet_hash IS NOT NULL
		"#,
		params![well_id.to_string()],
		|row| Ok((row.get::<_, Option<f64>>(0)?, row.get::<_, Option<f64>>(1)?)),
	)?;
	Ok(result)
}

/// Get all gridded curves for a well
fn get_gridded_curves(conn: &Connection, well_id: Uuid) -> Result<Vec<CurveInfo>> {
	let mut stmt = conn.prepare(
		r#"
		SELECT id, mnemonic, gridded_parquet_hash
		FROM curves
		WHERE well_id = ? AND deleted_at IS NULL AND gridded_parquet_hash IS NOT NULL
		ORDER BY mnemonic
		"#,
	)?;

	let curves = stmt.query_map(params![well_id.to_string()], |row| {
		let id_str: String = row.get(0)?;
		Ok(CurveInfo {
			id: Uuid::parse_str(&id_str).unwrap_or_default(),
			mnemonic: row.get(1)?,
			gridded_parquet_hash: row.get(2)?,
		})
	})?
	.filter_map(|r| r.ok())
	.collect();

	Ok(curves)
}

/// Get maximum curve version for the well
fn get_max_curve_version(conn: &Connection, well_id: Uuid) -> Result<i64> {
	let version: i64 = conn.query_row(
		"SELECT COALESCE(MAX(version), 0) FROM curves WHERE well_id = ? AND deleted_at IS NULL",
		params![well_id.to_string()],
		|row| row.get(0),
	)?;
	Ok(version)
}

/// Get well's workspace ID
fn get_well_workspace(conn: &Connection, well_id: Uuid) -> Result<Uuid> {
	let ws_str: String = conn.query_row(
		"SELECT workspace_id FROM wells WHERE id = ?",
		params![well_id.to_string()],
		|row| row.get(0),
	)?;
	Ok(Uuid::parse_str(&ws_str).unwrap_or_default())
}

/// Get existing chunk if any
fn get_existing_chunk(
	conn: &Connection,
	well_id: Uuid,
	chunk_index: i32,
) -> Result<Option<(Uuid, i64)>> {
	let result = conn.query_row(
		"SELECT id, source_version FROM unified_views WHERE well_id = ? AND chunk_index = ?",
		params![well_id.to_string(), chunk_index],
		|row| {
			let id_str: String = row.get(0)?;
			let version: i64 = row.get(1)?;
			Ok((Uuid::parse_str(&id_str).unwrap_or_default(), version))
		},
	);

	match result {
		Ok(r) => Ok(Some(r)),
		Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
		Err(e) => Err(e.into()),
	}
}

/// Delete a chunk
fn delete_chunk(conn: &Connection, chunk_id: Uuid) -> Result<()> {
	conn.execute(
		"DELETE FROM unified_views WHERE id = ?",
		params![chunk_id.to_string()],
	)?;
	Ok(())
}

/// Build the unified view data for a chunk
fn build_chunk_data(
	conn: &Connection,
	grid: &WellDepthGrid,
	curves: &[CurveInfo],
	start_idx: i64,
	end_idx: i64,
) -> Result<UnifiedViewData> {
	// Generate depth values for this chunk
	let depths: Vec<f64> = (start_idx..=end_idx)
		.map(|idx| grid.index_to_depth(idx))
		.collect();

	let num_rows = depths.len();

	// For each curve, load the gridded data and extract values for this range
	let mut curve_columns: Vec<(String, Vec<Option<f64>>)> = Vec::new();

	for curve in curves {
		let values = load_curve_values_for_range(conn, &curve.gridded_parquet_hash, start_idx, end_idx, num_rows)?;
		curve_columns.push((curve.mnemonic.clone(), values));
	}

	Ok(UnifiedViewData {
		depths,
		curves: curve_columns,
	})
}

/// Load curve values for a given index range
///
/// This reads the gridded Parquet file and extracts values for the specified range.
/// Values outside the curve's range are filled with None.
fn load_curve_values_for_range(
	conn: &Connection,
	parquet_hash: &str,
	start_idx: i64,
	end_idx: i64,
	num_rows: usize,
) -> Result<Vec<Option<f64>>> {
	// For now, we'll query the curve's stored indices and values
	// In a full implementation, we'd read from the Parquet file
	// Here we create a placeholder that returns None for all values
	// (The actual implementation would use DuckDB or parquet reader)

	// TODO: Implement actual Parquet reading
	// For now, return empty values (this is a placeholder)
	let _ = (conn, parquet_hash, start_idx, end_idx);
	Ok(vec![None; num_rows])
}

/// Register a blob in the blob_registry table
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
	fn test_build_empty_well() {
		let (_temp_dir, conn, blob_store) = setup_test_env();

		// Create a workspace, account, and well
		let workspace_id = Uuid::new_v4();
		let account_id = Uuid::new_v4();
		let member_id = Uuid::new_v4();
		let well_id = Uuid::new_v4();

		conn.execute(
			"INSERT INTO accounts (id, email, password_hash, name) VALUES (?1, ?2, ?3, ?4)",
			params![account_id.to_string(), "test@example.com", "hash", "Test"],
		).unwrap();

		conn.execute(
			"INSERT INTO workspaces (id, name, owner_account_id) VALUES (?1, ?2, ?3)",
			params![workspace_id.to_string(), "Test", account_id.to_string()],
		).unwrap();

		conn.execute(
			"INSERT INTO workspace_members (id, workspace_id, account_id, role) VALUES (?1, ?2, ?3, ?4)",
			params![member_id.to_string(), workspace_id.to_string(), account_id.to_string(), "owner"],
		).unwrap();

		conn.execute(
			r#"
			INSERT INTO wells (id, workspace_id, name, depth_unit, depth_step, depth_origin, created_by)
			VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
			"#,
			params![well_id.to_string(), workspace_id.to_string(), "Test Well", "ft", 0.5, 0.0, member_id.to_string()],
		).unwrap();

		let options = UnifiedViewOptions::default();
		let result = build_unified_views(&conn, &blob_store, well_id, &options).unwrap();

		assert_eq!(result.chunks_created, 0);
		assert_eq!(result.total_rows, 0);
	}
}
