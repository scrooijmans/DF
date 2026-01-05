//! Error types for DataForge Core

use thiserror::Error;

use crate::wellgrid::ResampleError;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Database error: {0}")]
	Database(#[from] rusqlite::Error),

	#[error("Parquet error: {0}")]
	Parquet(#[from] parquet::errors::ParquetError),

	#[error("Arrow error: {0}")]
	Arrow(#[from] arrow::error::ArrowError),

	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),

	#[error("JSON error: {0}")]
	Json(#[from] serde_json::Error),

	#[error("Resample error: {0}")]
	Resample(#[from] ResampleError),

	#[error("LAS parsing error: {0}")]
	LasParsing(String),

	#[error("Blob not found: {0}")]
	BlobNotFound(String),

	#[error("Well not found: {0}")]
	WellNotFound(String),

	#[error("Curve not found: {0}")]
	CurveNotFound(String),

	#[error("Project not found: {0}")]
	ProjectNotFound(String),

	#[error("Invalid data: {0}")]
	InvalidData(String),

	#[error("Hash mismatch: expected {expected}, got {actual}")]
	HashMismatch { expected: String, actual: String },
}

pub type Result<T> = std::result::Result<T, Error>;
