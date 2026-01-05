//! Trajectory/Survey Data Module
//!
//! Handles parsing, processing, and storing trajectory (directional survey) data.
//! Following OSDU patterns for trajectory stations with input (MD, INC, AZI) and
//! calculated (TVD, NS, EW, DLS) columns.
//!
//! ## Submodules
//!
//! - `detection` - Column type detection from CSV headers
//! - `parser` - CSV parsing with delimiter detection
//! - `calculation` - Minimum curvature and other trajectory calculations
//! - `ingest` - Main ingestion workflow

pub mod calculation;
pub mod detection;
pub mod ingest;
pub mod parser;

// Re-export commonly used types
pub use calculation::{
    calculate_dls, minimum_curvature, CalculatedStation, TieInPoint, TrajectoryCalculationResult,
};
pub use detection::{detect_column_type, DetectedColumn};
pub use ingest::{
    ingest_trajectory_file, ColumnMapping, IngestedTrajectoryColumn, TrajectoryIngestOptions,
    TrajectoryIngestResult,
};
pub use parser::{parse_trajectory_csv, ParsedTrajectory, ParsedTrajectoryColumn};

use crate::error::Result;
use arrow::array::{Array, Float64Array};
use bytes::Bytes;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

/// Read values from a single-column parquet file
pub fn read_parquet_column_values(data: &[u8]) -> Result<Vec<f64>> {
    let bytes = Bytes::copy_from_slice(data);
    let builder = ParquetRecordBatchReaderBuilder::try_new(bytes)
        .map_err(|e| crate::error::Error::InvalidData(format!("Parquet error: {}", e)))?;

    let reader = builder
        .build()
        .map_err(|e| crate::error::Error::InvalidData(format!("Parquet reader error: {}", e)))?;

    let mut values = Vec::new();
    for batch_result in reader {
        let batch = batch_result
            .map_err(|e| crate::error::Error::InvalidData(format!("Batch error: {}", e)))?;
        if batch.num_columns() > 0 {
            let col = batch.column(0);
            if let Some(array) = col.as_any().downcast_ref::<Float64Array>() {
                for i in 0..array.len() {
                    if array.is_valid(i) {
                        values.push(array.value(i));
                    } else {
                        values.push(f64::NAN);
                    }
                }
            }
        }
    }

    Ok(values)
}
