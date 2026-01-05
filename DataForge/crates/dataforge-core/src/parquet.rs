//! Parquet file generation and querying
//!
//! Creates Parquet files from LAS data and provides DuckDB-based querying.
//!
//! # Storage Strategy
//!
//! The module supports two types of curve storage:
//!
//! 1. **Native Storage**: Original data at original sampling, stored immutably
//!    - Schema: `[DEPTH: f64, VALUE: f64]`
//!    - Preserves original sampling interval and null values
//!
//! 2. **Gridded Storage**: Resampled to well's canonical depth grid
//!    - Schema: `[DEPTH_INDEX: i64, VALUE: f64]` (depth can be reconstructed from index)
//!    - All curves for a well share the same depth index
//!
//! 3. **Unified Views**: Wide format combining multiple curves (chunked)
//!    - Schema: `[DEPTH: f64, curve1: f64, curve2: f64, ...]`
//!    - Built lazily for analysis queries

use arrow::array::{Array, ArrayRef, Float64Array, Int64Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;
use std::sync::Arc;
use tracing::info;

use crate::error::Result;
use crate::las::ParsedLasFile;
use crate::wellgrid::ResampledCurve;

/// Options for Parquet file generation
#[derive(Debug, Clone)]
pub struct ParquetOptions {
	/// Compression algorithm
	pub compression: Compression,

	/// Row group size (number of rows per group)
	pub row_group_size: usize,

	/// Include metadata in footer
	pub include_metadata: bool,
}

impl Default for ParquetOptions {
	fn default() -> Self {
		Self {
			compression: Compression::ZSTD(Default::default()),
			row_group_size: 10_000,
			include_metadata: true,
		}
	}
}

/// Generate a Parquet file from parsed LAS data
///
/// Returns the Parquet file as bytes.
pub fn las_to_parquet(las: &ParsedLasFile, options: &ParquetOptions) -> Result<Vec<u8>> {
	// Build schema from curves
	let fields: Vec<Field> = las
		.curves
		.iter()
		.map(|curve| Field::new(&curve.mnemonic, DataType::Float64, true))
		.collect();

	let schema = Arc::new(Schema::new(fields));

	// Build arrays from curve data
	let arrays: Vec<ArrayRef> = las
		.curves
		.iter()
		.map(|curve| {
			let array = Float64Array::from(
				curve
					.values
					.iter()
					.map(|&v| if v.is_nan() { None } else { Some(v) })
					.collect::<Vec<_>>(),
			);
			Arc::new(array) as ArrayRef
		})
		.collect();

	// Create record batch
	let batch = RecordBatch::try_new(schema.clone(), arrays)?;

	// Write to Parquet
	let mut buf = Vec::new();
	let props = WriterProperties::builder()
		.set_compression(options.compression)
		.set_max_row_group_size(options.row_group_size)
		.build();

	let mut writer = ArrowWriter::try_new(&mut buf, schema, Some(props))?;
	writer.write(&batch)?;
	writer.close()?;

	info!(
		filename = las.filename,
		curves = las.curves.len(),
		rows = las.curves.first().map(|c| c.row_count).unwrap_or(0),
		size = buf.len(),
		"Generated Parquet file"
	);

	Ok(buf)
}

// ============================================================================
// NATIVE CURVE STORAGE
// ============================================================================

/// Native curve data (original sampling, stored immutably)
///
/// Schema: `[DEPTH: f64, VALUE: f64]`
#[derive(Debug, Clone)]
pub struct NativeCurveData {
	/// Depth values (original unit)
	pub depths: Vec<f64>,
	/// Curve values (NaN for null)
	pub values: Vec<f64>,
	/// Mnemonic for column naming
	pub mnemonic: String,
}

/// Create a Parquet file for native (original) curve storage
///
/// Stores the curve at its original sampling interval.
/// Schema: `[DEPTH: f64, {mnemonic}: f64]`
pub fn create_native_parquet(
	data: &NativeCurveData,
	options: &ParquetOptions,
) -> Result<Vec<u8>> {
	if data.depths.len() != data.values.len() {
		return Err(crate::error::Error::InvalidData(format!(
			"Depth/value length mismatch: {} vs {}",
			data.depths.len(),
			data.values.len()
		)));
	}

	// Schema: DEPTH, VALUE
	let fields = vec![
		Field::new("DEPTH", DataType::Float64, false),
		Field::new(&data.mnemonic, DataType::Float64, true),
	];
	let schema = Arc::new(Schema::new(fields));

	// Build arrays
	let depth_array: ArrayRef = Arc::new(Float64Array::from(data.depths.clone()));
	let value_array: ArrayRef = Arc::new(Float64Array::from(
		data.values
			.iter()
			.map(|&v| if v.is_nan() { None } else { Some(v) })
			.collect::<Vec<_>>(),
	));

	let batch = RecordBatch::try_new(schema.clone(), vec![depth_array, value_array])?;

	// Write Parquet
	let mut buf = Vec::new();
	let props = WriterProperties::builder()
		.set_compression(options.compression)
		.set_max_row_group_size(options.row_group_size)
		.build();

	let mut writer = ArrowWriter::try_new(&mut buf, schema, Some(props))?;
	writer.write(&batch)?;
	writer.close()?;

	info!(
		mnemonic = data.mnemonic,
		rows = data.depths.len(),
		size = buf.len(),
		"Created native curve Parquet"
	);

	Ok(buf)
}

// ============================================================================
// GRIDDED CURVE STORAGE
// ============================================================================

/// Create a Parquet file for gridded (resampled) curve storage
///
/// Stores the curve aligned to the well's depth grid.
/// Schema: `[DEPTH_INDEX: i64, {mnemonic}: f64]`
///
/// The depth can be reconstructed as: `depth = origin + index * step`
pub fn create_gridded_parquet(
	resampled: &ResampledCurve,
	mnemonic: &str,
	options: &ParquetOptions,
) -> Result<Vec<u8>> {
	// Schema: DEPTH_INDEX, VALUE
	let fields = vec![
		Field::new("DEPTH_INDEX", DataType::Int64, false),
		Field::new(mnemonic, DataType::Float64, true),
	];
	let schema = Arc::new(Schema::new(fields));

	// Build arrays
	let index_array: ArrayRef = Arc::new(Int64Array::from(resampled.indices.clone()));
	let value_array: ArrayRef = Arc::new(Float64Array::from(
		resampled
			.values
			.iter()
			.map(|&v| v) // Already Option<f64>
			.collect::<Vec<_>>(),
	));

	let batch = RecordBatch::try_new(schema.clone(), vec![index_array, value_array])?;

	// Write Parquet
	let mut buf = Vec::new();
	let props = WriterProperties::builder()
		.set_compression(options.compression)
		.set_max_row_group_size(options.row_group_size)
		.build();

	let mut writer = ArrowWriter::try_new(&mut buf, schema, Some(props))?;
	writer.write(&batch)?;
	writer.close()?;

	info!(
		mnemonic = mnemonic,
		rows = resampled.indices.len(),
		valid = resampled.sample_count,
		nulls = resampled.null_count,
		size = buf.len(),
		"Created gridded curve Parquet"
	);

	Ok(buf)
}

// ============================================================================
// UNIFIED VIEW STORAGE
// ============================================================================

/// Data for a unified view chunk
#[derive(Debug)]
pub struct UnifiedViewData {
	/// Depth values (snapped to grid)
	pub depths: Vec<f64>,
	/// Curve columns: (mnemonic, values)
	pub curves: Vec<(String, Vec<Option<f64>>)>,
}

/// Create a Parquet file for a unified view chunk
///
/// Wide format combining multiple curves aligned to the same depth grid.
/// Schema: `[DEPTH: f64, curve1: f64, curve2: f64, ...]`
pub fn create_unified_view_parquet(
	data: &UnifiedViewData,
	options: &ParquetOptions,
) -> Result<Vec<u8>> {
	// Validate all curves have same length
	for (mnemonic, values) in &data.curves {
		if values.len() != data.depths.len() {
			return Err(crate::error::Error::InvalidData(format!(
				"Curve {} has {} values but expected {}",
				mnemonic,
				values.len(),
				data.depths.len()
			)));
		}
	}

	// Build schema: DEPTH + all curves
	let mut fields = vec![Field::new("DEPTH", DataType::Float64, false)];
	for (mnemonic, _) in &data.curves {
		fields.push(Field::new(mnemonic, DataType::Float64, true));
	}
	let schema = Arc::new(Schema::new(fields));

	// Build arrays
	let mut arrays: Vec<ArrayRef> = Vec::with_capacity(1 + data.curves.len());

	// Depth array
	arrays.push(Arc::new(Float64Array::from(data.depths.clone())));

	// Curve arrays
	for (_, values) in &data.curves {
		arrays.push(Arc::new(Float64Array::from(values.clone())));
	}

	let batch = RecordBatch::try_new(schema.clone(), arrays)?;

	// Write Parquet
	let mut buf = Vec::new();
	let props = WriterProperties::builder()
		.set_compression(options.compression)
		.set_max_row_group_size(options.row_group_size)
		.build();

	let mut writer = ArrowWriter::try_new(&mut buf, schema, Some(props))?;
	writer.write(&batch)?;
	writer.close()?;

	info!(
		curves = data.curves.len(),
		rows = data.depths.len(),
		size = buf.len(),
		"Created unified view Parquet"
	);

	Ok(buf)
}

// ============================================================================
// PARQUET EDITING
// ============================================================================

/// A single cell edit operation
#[derive(Debug, Clone)]
pub struct CellEdit {
	/// Row index (0-based)
	pub row_index: usize,
	/// Column name to edit
	pub column: String,
	/// New value (None for NULL)
	pub new_value: Option<f64>,
}

/// Result of applying edits to a Parquet file
#[derive(Debug)]
pub struct ParquetEditResult {
	/// New Parquet file bytes
	pub data: Vec<u8>,
	/// Number of edits successfully applied
	pub edits_applied: usize,
	/// Number of rows in the result
	pub row_count: usize,
}

/// Read a Parquet file and return its contents as column vectors
///
/// Returns (column_names, column_data) where column_data is a Vec of Option<f64> values
pub fn read_parquet_columns(data: &[u8]) -> Result<(Vec<String>, Vec<Vec<Option<f64>>>)> {
	use bytes::Bytes;
	use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

	let bytes = Bytes::copy_from_slice(data);
	let builder = ParquetRecordBatchReaderBuilder::try_new(bytes)?;
	let schema = builder.schema().clone();
	let reader = builder.build()?;

	let column_names: Vec<String> = schema.fields().iter().map(|f| f.name().clone()).collect();
	let num_columns = column_names.len();

	// Initialize column vectors
	let mut columns: Vec<Vec<Option<f64>>> = vec![Vec::new(); num_columns];

	// Read all batches
	for batch_result in reader {
		let batch = batch_result?;
		let num_rows = batch.num_rows();

		for (col_idx, column) in batch.columns().iter().enumerate() {
			if let Some(float_array) = column.as_any().downcast_ref::<Float64Array>() {
				for row_idx in 0..num_rows {
					if float_array.is_null(row_idx) {
						columns[col_idx].push(None);
					} else {
						columns[col_idx].push(Some(float_array.value(row_idx)));
					}
				}
			} else if let Some(int_array) = column.as_any().downcast_ref::<Int64Array>() {
				// Handle DEPTH_INDEX columns (Int64)
				for row_idx in 0..num_rows {
					if int_array.is_null(row_idx) {
						columns[col_idx].push(None);
					} else {
						columns[col_idx].push(Some(int_array.value(row_idx) as f64));
					}
				}
			} else {
				return Err(crate::error::Error::InvalidData(format!(
					"Unsupported column type for column {}",
					column_names[col_idx]
				)));
			}
		}
	}

	Ok((column_names, columns))
}

/// Apply edits to a Parquet file and return new Parquet bytes
///
/// This reads the original Parquet, applies the specified cell edits,
/// and writes a new Parquet file with the changes applied.
///
/// The original file is never modified - this creates a completely new file.
pub fn update_parquet_values(
	original_data: &[u8],
	edits: &[CellEdit],
	options: &ParquetOptions,
) -> Result<ParquetEditResult> {
	// Read the original data
	let (column_names, mut columns) = read_parquet_columns(original_data)?;

	if columns.is_empty() {
		return Err(crate::error::Error::InvalidData(
			"Parquet file has no columns".to_string(),
		));
	}

	let num_rows = columns[0].len();
	let mut edits_applied = 0;

	// Apply each edit
	for edit in edits {
		// Find the column index
		let col_idx = column_names
			.iter()
			.position(|name| name == &edit.column)
			.ok_or_else(|| {
				crate::error::Error::InvalidData(format!(
					"Column '{}' not found in Parquet file",
					edit.column
				))
			})?;

		// Validate row index
		if edit.row_index >= num_rows {
			return Err(crate::error::Error::InvalidData(format!(
				"Row index {} out of bounds (max {})",
				edit.row_index,
				num_rows - 1
			)));
		}

		// Apply the edit
		columns[col_idx][edit.row_index] = edit.new_value;
		edits_applied += 1;
	}

	// Rebuild the Parquet file
	// Determine if first column is Int64 (DEPTH_INDEX) or Float64 (DEPTH)
	// by checking the original schema
	let bytes = bytes::Bytes::copy_from_slice(original_data);
	let builder = parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder::try_new(bytes)?;
	let original_schema = builder.schema();

	let mut fields = Vec::new();
	let mut arrays: Vec<ArrayRef> = Vec::new();

	for (idx, field) in original_schema.fields().iter().enumerate() {
		fields.push(Field::new(field.name(), field.data_type().clone(), field.is_nullable()));

		match field.data_type() {
			DataType::Int64 => {
				// Reconstruct as Int64 array
				let values: Vec<Option<i64>> = columns[idx]
					.iter()
					.map(|v| v.map(|f| f as i64))
					.collect();
				arrays.push(Arc::new(Int64Array::from(values)));
			}
			DataType::Float64 => {
				// Reconstruct as Float64 array
				arrays.push(Arc::new(Float64Array::from(columns[idx].clone())));
			}
			_ => {
				return Err(crate::error::Error::InvalidData(format!(
					"Unsupported data type for column {}: {:?}",
					field.name(),
					field.data_type()
				)));
			}
		}
	}

	let schema = Arc::new(Schema::new(fields));
	let batch = RecordBatch::try_new(schema.clone(), arrays)?;

	// Write the new Parquet file
	let mut buf = Vec::new();
	let props = WriterProperties::builder()
		.set_compression(options.compression)
		.set_max_row_group_size(options.row_group_size)
		.build();

	let mut writer = ArrowWriter::try_new(&mut buf, schema, Some(props))?;
	writer.write(&batch)?;
	writer.close()?;

	info!(
		original_size = original_data.len(),
		new_size = buf.len(),
		edits_applied = edits_applied,
		rows = num_rows,
		"Applied edits to Parquet file"
	);

	Ok(ParquetEditResult {
		data: buf,
		edits_applied,
		row_count: num_rows,
	})
}

/// Add new rows to a Parquet file
///
/// Inserts rows at the specified indices. Each row is a map of column name to value.
/// Rows are inserted in sorted order by depth to maintain the depth ordering.
pub fn add_parquet_rows(
	original_data: &[u8],
	new_rows: &[std::collections::HashMap<String, Option<f64>>],
	depth_column: &str,
	options: &ParquetOptions,
) -> Result<ParquetEditResult> {
	let (column_names, mut columns) = read_parquet_columns(original_data)?;

	if columns.is_empty() {
		return Err(crate::error::Error::InvalidData(
			"Parquet file has no columns".to_string(),
		));
	}

	// Find depth column index
	let depth_idx = column_names
		.iter()
		.position(|name| name == depth_column)
		.ok_or_else(|| {
			crate::error::Error::InvalidData(format!(
				"Depth column '{}' not found in Parquet file",
				depth_column
			))
		})?;

	// Add each new row
	for row_data in new_rows {
		// Get the depth value for this row
		let depth = row_data
			.get(depth_column)
			.and_then(|v| *v)
			.ok_or_else(|| {
				crate::error::Error::InvalidData(
					"New row must have a depth value".to_string(),
				)
			})?;

		// Find insertion point to maintain sorted order
		let insert_idx = columns[depth_idx]
			.iter()
			.position(|d| d.map(|v| v > depth).unwrap_or(false))
			.unwrap_or(columns[depth_idx].len());

		// Insert values for each column
		for (col_idx, col_name) in column_names.iter().enumerate() {
			let value = row_data.get(col_name).copied().flatten();
			columns[col_idx].insert(insert_idx, value);
		}
	}

	let num_rows = columns[0].len();

	// Rebuild the Parquet file (same logic as update_parquet_values)
	let bytes = bytes::Bytes::copy_from_slice(original_data);
	let builder = parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder::try_new(bytes)?;
	let original_schema = builder.schema();

	let mut fields = Vec::new();
	let mut arrays: Vec<ArrayRef> = Vec::new();

	for (idx, field) in original_schema.fields().iter().enumerate() {
		fields.push(Field::new(field.name(), field.data_type().clone(), field.is_nullable()));

		match field.data_type() {
			DataType::Int64 => {
				let values: Vec<Option<i64>> = columns[idx]
					.iter()
					.map(|v| v.map(|f| f as i64))
					.collect();
				arrays.push(Arc::new(Int64Array::from(values)));
			}
			DataType::Float64 => {
				arrays.push(Arc::new(Float64Array::from(columns[idx].clone())));
			}
			_ => {
				return Err(crate::error::Error::InvalidData(format!(
					"Unsupported data type for column {}: {:?}",
					field.name(),
					field.data_type()
				)));
			}
		}
	}

	let schema = Arc::new(Schema::new(fields));
	let batch = RecordBatch::try_new(schema.clone(), arrays)?;

	let mut buf = Vec::new();
	let props = WriterProperties::builder()
		.set_compression(options.compression)
		.set_max_row_group_size(options.row_group_size)
		.build();

	let mut writer = ArrowWriter::try_new(&mut buf, schema, Some(props))?;
	writer.write(&batch)?;
	writer.close()?;

	info!(
		rows_added = new_rows.len(),
		total_rows = num_rows,
		size = buf.len(),
		"Added rows to Parquet file"
	);

	Ok(ParquetEditResult {
		data: buf,
		edits_applied: new_rows.len(),
		row_count: num_rows,
	})
}

/// Delete rows from a Parquet file by row indices
///
/// Removes the rows at the specified indices.
pub fn delete_parquet_rows(
	original_data: &[u8],
	row_indices: &[usize],
	options: &ParquetOptions,
) -> Result<ParquetEditResult> {
	let (column_names, columns) = read_parquet_columns(original_data)?;

	if columns.is_empty() {
		return Err(crate::error::Error::InvalidData(
			"Parquet file has no columns".to_string(),
		));
	}

	let original_num_rows = columns[0].len();

	// Sort indices in descending order to remove from end first
	let mut sorted_indices: Vec<usize> = row_indices.to_vec();
	sorted_indices.sort_by(|a, b| b.cmp(a));
	sorted_indices.dedup();

	// Validate indices
	for &idx in &sorted_indices {
		if idx >= original_num_rows {
			return Err(crate::error::Error::InvalidData(format!(
				"Row index {} out of bounds (max {})",
				idx,
				original_num_rows - 1
			)));
		}
	}

	// Create a set of indices to remove for efficient lookup
	let indices_to_remove: std::collections::HashSet<usize> = row_indices.iter().copied().collect();

	// Filter out the rows to delete
	let mut new_columns: Vec<Vec<Option<f64>>> = vec![Vec::new(); columns.len()];
	for row_idx in 0..original_num_rows {
		if !indices_to_remove.contains(&row_idx) {
			for (col_idx, col) in columns.iter().enumerate() {
				new_columns[col_idx].push(col[row_idx]);
			}
		}
	}

	let num_rows = new_columns[0].len();
	let rows_deleted = original_num_rows - num_rows;

	// Rebuild the Parquet file
	let bytes = bytes::Bytes::copy_from_slice(original_data);
	let builder = parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder::try_new(bytes)?;
	let original_schema = builder.schema();

	let mut fields = Vec::new();
	let mut arrays: Vec<ArrayRef> = Vec::new();

	for (idx, field) in original_schema.fields().iter().enumerate() {
		fields.push(Field::new(field.name(), field.data_type().clone(), field.is_nullable()));

		match field.data_type() {
			DataType::Int64 => {
				let values: Vec<Option<i64>> = new_columns[idx]
					.iter()
					.map(|v| v.map(|f| f as i64))
					.collect();
				arrays.push(Arc::new(Int64Array::from(values)));
			}
			DataType::Float64 => {
				arrays.push(Arc::new(Float64Array::from(new_columns[idx].clone())));
			}
			_ => {
				return Err(crate::error::Error::InvalidData(format!(
					"Unsupported data type for column {}: {:?}",
					field.name(),
					field.data_type()
				)));
			}
		}
	}

	let schema = Arc::new(Schema::new(fields));
	let batch = RecordBatch::try_new(schema.clone(), arrays)?;

	let mut buf = Vec::new();
	let props = WriterProperties::builder()
		.set_compression(options.compression)
		.set_max_row_group_size(options.row_group_size)
		.build();

	let mut writer = ArrowWriter::try_new(&mut buf, schema, Some(props))?;
	writer.write(&batch)?;
	writer.close()?;

	info!(
		rows_deleted = rows_deleted,
		total_rows = num_rows,
		size = buf.len(),
		"Deleted rows from Parquet file"
	);

	Ok(ParquetEditResult {
		data: buf,
		edits_applied: rows_deleted,
		row_count: num_rows,
	})
}

// ============================================================================
// PARQUET STATISTICS
// ============================================================================

/// Statistics about a Parquet file
#[derive(Debug, Clone)]
pub struct ParquetStats {
	pub num_rows: usize,
	pub num_columns: usize,
	pub column_names: Vec<String>,
	pub file_size: usize,
}

/// Get statistics from a Parquet file
pub fn get_parquet_stats(data: &[u8]) -> Result<ParquetStats> {
	use bytes::Bytes;
	use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

	let bytes = Bytes::copy_from_slice(data);
	let builder = ParquetRecordBatchReaderBuilder::try_new(bytes)?;

	let metadata = builder.metadata();
	let schema = builder.schema();

	let num_rows = metadata.file_metadata().num_rows() as usize;
	let num_columns = schema.fields().len();
	let column_names = schema.fields().iter().map(|f| f.name().clone()).collect();

	Ok(ParquetStats {
		num_rows,
		num_columns,
		column_names,
		file_size: data.len(),
	})
}

/// Query Parquet data using DuckDB
pub struct ParquetQuery {
	conn: duckdb::Connection,
}

impl ParquetQuery {
	/// Create a new query engine
	pub fn new() -> Result<Self> {
		let conn = duckdb::Connection::open_in_memory()
			.map_err(|e| crate::error::Error::InvalidData(e.to_string()))?;
		Ok(Self { conn })
	}

	/// Query a Parquet file and return results as vectors
	///
	/// # Arguments
	/// * `parquet_path` - Path to the Parquet file
	/// * `columns` - Columns to select (e.g., ["DEPT", "GR"])
	/// * `depth_min` - Optional minimum depth filter
	/// * `depth_max` - Optional maximum depth filter
	pub fn query_curves(
		&self,
		parquet_path: &str,
		columns: &[&str],
		depth_column: &str,
		depth_min: Option<f64>,
		depth_max: Option<f64>,
	) -> Result<Vec<Vec<f64>>> {
		let columns_str = columns.join(", ");

		let mut where_clauses = Vec::new();
		if let Some(min) = depth_min {
			where_clauses.push(format!("{} >= {}", depth_column, min));
		}
		if let Some(max) = depth_max {
			where_clauses.push(format!("{} <= {}", depth_column, max));
		}

		let where_clause = if where_clauses.is_empty() {
			String::new()
		} else {
			format!(" WHERE {}", where_clauses.join(" AND "))
		};

		let query = format!(
			"SELECT {} FROM read_parquet('{}'){} ORDER BY {}",
			columns_str, parquet_path, where_clause, depth_column
		);

		let mut stmt = self
			.conn
			.prepare(&query)
			.map_err(|e| crate::error::Error::InvalidData(e.to_string()))?;

		let mut results: Vec<Vec<f64>> = columns.iter().map(|_| Vec::new()).collect();

		let rows = stmt
			.query_map([], |row| {
				let mut values = Vec::new();
				for i in 0..columns.len() {
					let val: Option<f64> = row.get(i)?;
					values.push(val.unwrap_or(f64::NAN));
				}
				Ok(values)
			})
			.map_err(|e| crate::error::Error::InvalidData(e.to_string()))?;

		for row in rows {
			let values = row.map_err(|e| crate::error::Error::InvalidData(e.to_string()))?;
			for (i, val) in values.into_iter().enumerate() {
				results[i].push(val);
			}
		}

		Ok(results)
	}

	/// Get column statistics from a Parquet file
	pub fn get_column_stats(
		&self,
		parquet_path: &str,
		column: &str,
	) -> Result<(f64, f64, i64)> {
		let query = format!(
			"SELECT MIN({0}), MAX({0}), COUNT({0}) FROM read_parquet('{1}')",
			column, parquet_path
		);

		let mut stmt = self
			.conn
			.prepare(&query)
			.map_err(|e| crate::error::Error::InvalidData(e.to_string()))?;

		let result = stmt
			.query_row([], |row| {
				let min: f64 = row.get(0)?;
				let max: f64 = row.get(1)?;
				let count: i64 = row.get(2)?;
				Ok((min, max, count))
			})
			.map_err(|e| crate::error::Error::InvalidData(e.to_string()))?;

		Ok(result)
	}
}

impl Default for ParquetQuery {
	fn default() -> Self {
		Self::new().expect("Failed to create DuckDB connection")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::las::{ParsedCurve, ParsedLasFile};
	use crate::models::MainCurveType;

	fn create_test_las() -> ParsedLasFile {
		ParsedLasFile {
			filename: "test.las".to_string(),
			well_name: Some("Test Well".to_string()),
			uwi: None,
			field: None,
			company: None,
			location: None,
			start_depth: 0.0,
			stop_depth: 100.0,
			step: 0.5,
			null_value: -999.25,
			actual_depths: (0..200).map(|i| i as f64 * 0.5).collect(),
			depth_mnemonic: "DEPT".to_string(),
			depth_unit: "FT".to_string(),
			has_irregular_spacing: false,
			version: "2.0".to_string(),
			curves: vec![
				ParsedCurve {
					mnemonic: "DEPT".to_string(),
					unit: "FT".to_string(),
					description: "Depth".to_string(),
					detected_type: MainCurveType::DEPTH,
					values: (0..200).map(|i| i as f64 * 0.5).collect(),
					row_count: 200,
					min_value: Some(0.0),
					max_value: Some(99.5),
				},
				ParsedCurve {
					mnemonic: "GR".to_string(),
					unit: "GAPI".to_string(),
					description: "Gamma Ray".to_string(),
					detected_type: MainCurveType::GR,
					values: (0..200).map(|i| 50.0 + (i as f64 * 0.1).sin() * 20.0).collect(),
					row_count: 200,
					min_value: Some(30.0),
					max_value: Some(70.0),
				},
			],
		}
	}

	#[test]
	fn test_las_to_parquet() {
		let las = create_test_las();
		let options = ParquetOptions::default();

		let parquet_data = las_to_parquet(&las, &options).unwrap();
		assert!(!parquet_data.is_empty());

		let stats = get_parquet_stats(&parquet_data).unwrap();
		assert_eq!(stats.num_rows, 200);
		assert_eq!(stats.num_columns, 2);
		assert_eq!(stats.column_names, vec!["DEPT", "GR"]);
	}

	#[test]
	fn test_native_parquet() {
		let data = NativeCurveData {
			depths: vec![0.0, 1.0, 2.0, 3.0, 4.0],
			values: vec![10.0, 20.0, f64::NAN, 40.0, 50.0],
			mnemonic: "GR".to_string(),
		};
		let options = ParquetOptions::default();

		let parquet_data = create_native_parquet(&data, &options).unwrap();
		assert!(!parquet_data.is_empty());

		let stats = get_parquet_stats(&parquet_data).unwrap();
		assert_eq!(stats.num_rows, 5);
		assert_eq!(stats.num_columns, 2);
		assert_eq!(stats.column_names, vec!["DEPTH", "GR"]);
	}

	#[test]
	fn test_gridded_parquet() {
		use crate::wellgrid::{resample_curve, DepthUnit, ResampleMethod, WellDepthGrid};

		let source_depths = vec![0.0, 1.0, 2.0, 3.0, 4.0];
		let source_values = vec![10.0, 20.0, 30.0, 40.0, 50.0];
		let grid = WellDepthGrid::feet(0.5);

		let resampled = resample_curve(
			&source_depths,
			&source_values,
			DepthUnit::Feet,
			&grid,
			ResampleMethod::Linear,
		)
		.unwrap();

		let options = ParquetOptions::default();
		let parquet_data = create_gridded_parquet(&resampled, "GR", &options).unwrap();
		assert!(!parquet_data.is_empty());

		let stats = get_parquet_stats(&parquet_data).unwrap();
		assert_eq!(stats.num_rows, 9); // 0.0, 0.5, 1.0, ..., 4.0 = 9 points
		assert_eq!(stats.num_columns, 2);
		assert_eq!(stats.column_names, vec!["DEPTH_INDEX", "GR"]);
	}

	#[test]
	fn test_unified_view_parquet() {
		let data = UnifiedViewData {
			depths: vec![0.0, 0.5, 1.0, 1.5, 2.0],
			curves: vec![
				("GR".to_string(), vec![Some(10.0), Some(15.0), Some(20.0), Some(25.0), Some(30.0)]),
				("RHOB".to_string(), vec![Some(2.5), Some(2.6), None, Some(2.7), Some(2.8)]),
			],
		};
		let options = ParquetOptions::default();

		let parquet_data = create_unified_view_parquet(&data, &options).unwrap();
		assert!(!parquet_data.is_empty());

		let stats = get_parquet_stats(&parquet_data).unwrap();
		assert_eq!(stats.num_rows, 5);
		assert_eq!(stats.num_columns, 3);
		assert_eq!(stats.column_names, vec!["DEPTH", "GR", "RHOB"]);
	}

	#[test]
	fn test_update_parquet_values() {
		// Create a native parquet file
		let data = NativeCurveData {
			depths: vec![0.0, 1.0, 2.0, 3.0, 4.0],
			values: vec![10.0, 20.0, 30.0, 40.0, 50.0],
			mnemonic: "GR".to_string(),
		};
		let options = ParquetOptions::default();
		let original = create_native_parquet(&data, &options).unwrap();

		// Apply edits
		let edits = vec![
			CellEdit {
				row_index: 1,
				column: "GR".to_string(),
				new_value: Some(25.0),
			},
			CellEdit {
				row_index: 3,
				column: "GR".to_string(),
				new_value: None, // Set to NULL
			},
		];

		let result = update_parquet_values(&original, &edits, &options).unwrap();
		assert_eq!(result.edits_applied, 2);
		assert_eq!(result.row_count, 5);

		// Verify the edits were applied
		let (column_names, columns) = read_parquet_columns(&result.data).unwrap();
		assert_eq!(column_names, vec!["DEPTH", "GR"]);

		// Check edited values
		assert_eq!(columns[1][0], Some(10.0)); // unchanged
		assert_eq!(columns[1][1], Some(25.0)); // edited from 20.0 to 25.0
		assert_eq!(columns[1][2], Some(30.0)); // unchanged
		assert_eq!(columns[1][3], None); // edited from 40.0 to NULL
		assert_eq!(columns[1][4], Some(50.0)); // unchanged
	}

	#[test]
	fn test_add_parquet_rows() {
		use std::collections::HashMap;

		// Create a native parquet file
		let data = NativeCurveData {
			depths: vec![0.0, 2.0, 4.0],
			values: vec![10.0, 30.0, 50.0],
			mnemonic: "GR".to_string(),
		};
		let options = ParquetOptions::default();
		let original = create_native_parquet(&data, &options).unwrap();

		// Add a row at depth 1.0 (should be inserted between 0.0 and 2.0)
		let mut new_row = HashMap::new();
		new_row.insert("DEPTH".to_string(), Some(1.0));
		new_row.insert("GR".to_string(), Some(20.0));

		let result = add_parquet_rows(&original, &[new_row], "DEPTH", &options).unwrap();
		assert_eq!(result.edits_applied, 1);
		assert_eq!(result.row_count, 4);

		// Verify the row was inserted in the right place
		let (_, columns) = read_parquet_columns(&result.data).unwrap();
		assert_eq!(columns[0], vec![Some(0.0), Some(1.0), Some(2.0), Some(4.0)]);
		assert_eq!(columns[1], vec![Some(10.0), Some(20.0), Some(30.0), Some(50.0)]);
	}

	#[test]
	fn test_delete_parquet_rows() {
		// Create a native parquet file
		let data = NativeCurveData {
			depths: vec![0.0, 1.0, 2.0, 3.0, 4.0],
			values: vec![10.0, 20.0, 30.0, 40.0, 50.0],
			mnemonic: "GR".to_string(),
		};
		let options = ParquetOptions::default();
		let original = create_native_parquet(&data, &options).unwrap();

		// Delete rows at indices 1 and 3
		let result = delete_parquet_rows(&original, &[1, 3], &options).unwrap();
		assert_eq!(result.edits_applied, 2);
		assert_eq!(result.row_count, 3);

		// Verify the rows were deleted
		let (_, columns) = read_parquet_columns(&result.data).unwrap();
		assert_eq!(columns[0], vec![Some(0.0), Some(2.0), Some(4.0)]);
		assert_eq!(columns[1], vec![Some(10.0), Some(30.0), Some(50.0)]);
	}

	#[test]
	fn test_read_parquet_columns() {
		let data = NativeCurveData {
			depths: vec![0.0, 1.0, 2.0],
			values: vec![10.0, f64::NAN, 30.0],
			mnemonic: "GR".to_string(),
		};
		let options = ParquetOptions::default();
		let parquet_data = create_native_parquet(&data, &options).unwrap();

		let (column_names, columns) = read_parquet_columns(&parquet_data).unwrap();
		assert_eq!(column_names, vec!["DEPTH", "GR"]);
		assert_eq!(columns[0], vec![Some(0.0), Some(1.0), Some(2.0)]);
		assert_eq!(columns[1], vec![Some(10.0), None, Some(30.0)]); // NaN becomes None
	}
}
