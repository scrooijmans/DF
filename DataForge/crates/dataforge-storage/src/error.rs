//! Storage error types

use thiserror::Error;

/// Storage operation result type
pub type Result<T> = std::result::Result<T, StorageError>;

/// Errors that can occur during storage operations
#[derive(Error, Debug)]
pub enum StorageError {
	/// OpenDAL backend error
	#[error("Storage backend error: {0}")]
	Backend(#[from] opendal::Error),

	/// Blob not found
	#[error("Blob not found: {0}")]
	NotFound(String),

	/// Hash mismatch during integrity verification
	#[error("Hash mismatch: expected {expected}, got {actual}")]
	HashMismatch { expected: String, actual: String },

	/// IO error
	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),

	/// Configuration error
	#[error("Configuration error: {0}")]
	Config(String),

	/// Missing required configuration
	#[error("Missing required configuration: {0}")]
	MissingConfig(String),

	/// Invalid configuration value
	#[error("Invalid configuration value for {key}: {message}")]
	InvalidConfig { key: String, message: String },

	/// Presigned URL not supported by this backend
	#[error("Presigned URLs not supported by {0} backend")]
	PresignedUrlNotSupported(String),

	/// Storage not initialized
	#[error("Storage service not initialized")]
	NotInitialized,
}
