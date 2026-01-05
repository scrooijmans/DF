//! Storage configuration
//!
//! Following ColaNode patterns, storage configuration is loaded from
//! environment variables with sensible defaults.

use crate::error::{Result, StorageError};
use std::env;
use std::path::PathBuf;

/// Storage backend configuration
///
/// Supports multiple backends with environment-based configuration:
///
/// ```bash
/// # Filesystem (default)
/// STORAGE_TYPE=filesystem
/// STORAGE_FS_ROOT=/path/to/blobs
///
/// # S3/MinIO
/// STORAGE_TYPE=s3
/// STORAGE_S3_BUCKET=dataforge-blobs
/// STORAGE_S3_REGION=us-east-1
/// STORAGE_S3_ENDPOINT=http://minio:9000
/// STORAGE_S3_ACCESS_KEY=...
/// STORAGE_S3_SECRET_KEY=...
/// STORAGE_S3_FORCE_PATH_STYLE=true  # For MinIO
/// ```
#[derive(Debug, Clone)]
pub enum StorageConfig {
	/// Local filesystem storage (default)
	Filesystem {
		/// Root directory for blob storage
		root: PathBuf,
	},

	/// S3-compatible storage (AWS S3, MinIO, DigitalOcean Spaces, etc.)
	S3 {
		/// S3 bucket name
		bucket: String,
		/// AWS region (e.g., "us-east-1")
		region: String,
		/// Optional custom endpoint for S3-compatible services
		endpoint: Option<String>,
		/// Access key ID
		access_key_id: String,
		/// Secret access key
		secret_access_key: String,
		/// Use path-style URLs (required for MinIO)
		force_path_style: bool,
	},

	/// In-memory storage (for testing only)
	Memory,
}

impl StorageConfig {
	/// Create configuration from environment variables
	///
	/// Reads `STORAGE_TYPE` to determine which backend to use,
	/// then loads backend-specific configuration.
	pub fn from_env() -> Result<Self> {
		let storage_type = env::var("STORAGE_TYPE").unwrap_or_else(|_| "filesystem".to_string());

		match storage_type.to_lowercase().as_str() {
			"filesystem" | "file" | "fs" => Self::filesystem_from_env(),
			"s3" => Self::s3_from_env(),
			"memory" | "mem" => Ok(StorageConfig::Memory),
			other => Err(StorageError::InvalidConfig {
				key: "STORAGE_TYPE".to_string(),
				message: format!(
					"Unknown storage type '{}'. Valid options: filesystem, s3, memory",
					other
				),
			}),
		}
	}

	/// Create filesystem configuration from environment
	fn filesystem_from_env() -> Result<Self> {
		let root = env::var("STORAGE_FS_ROOT").map(PathBuf::from).ok();

		// If no root specified, we'll need it to be provided later
		// This allows the Tauri app to set it based on app_data_dir
		let root = root.unwrap_or_else(|| PathBuf::from("./blobs"));

		Ok(StorageConfig::Filesystem { root })
	}

	/// Create S3 configuration from environment
	fn s3_from_env() -> Result<Self> {
		let bucket = env::var("STORAGE_S3_BUCKET").map_err(|_| {
			StorageError::MissingConfig("STORAGE_S3_BUCKET is required for S3 storage".to_string())
		})?;

		let region = env::var("STORAGE_S3_REGION").unwrap_or_else(|_| "us-east-1".to_string());

		let endpoint = env::var("STORAGE_S3_ENDPOINT").ok();

		let access_key_id = env::var("STORAGE_S3_ACCESS_KEY").map_err(|_| {
			StorageError::MissingConfig(
				"STORAGE_S3_ACCESS_KEY is required for S3 storage".to_string(),
			)
		})?;

		let secret_access_key = env::var("STORAGE_S3_SECRET_KEY").map_err(|_| {
			StorageError::MissingConfig(
				"STORAGE_S3_SECRET_KEY is required for S3 storage".to_string(),
			)
		})?;

		let force_path_style = env::var("STORAGE_S3_FORCE_PATH_STYLE")
			.map(|v| v.to_lowercase() == "true")
			.unwrap_or(false);

		Ok(StorageConfig::S3 {
			bucket,
			region,
			endpoint,
			access_key_id,
			secret_access_key,
			force_path_style,
		})
	}

	/// Create filesystem configuration with a specific root directory
	pub fn filesystem(root: impl Into<PathBuf>) -> Self {
		StorageConfig::Filesystem { root: root.into() }
	}

	/// Create S3 configuration programmatically
	pub fn s3(
		bucket: impl Into<String>,
		region: impl Into<String>,
		access_key_id: impl Into<String>,
		secret_access_key: impl Into<String>,
	) -> Self {
		StorageConfig::S3 {
			bucket: bucket.into(),
			region: region.into(),
			endpoint: None,
			access_key_id: access_key_id.into(),
			secret_access_key: secret_access_key.into(),
			force_path_style: false,
		}
	}

	/// Create S3 configuration for MinIO or other S3-compatible services
	pub fn s3_compatible(
		bucket: impl Into<String>,
		endpoint: impl Into<String>,
		access_key_id: impl Into<String>,
		secret_access_key: impl Into<String>,
	) -> Self {
		StorageConfig::S3 {
			bucket: bucket.into(),
			region: "us-east-1".to_string(),
			endpoint: Some(endpoint.into()),
			access_key_id: access_key_id.into(),
			secret_access_key: secret_access_key.into(),
			force_path_style: true,
		}
	}

	/// Create in-memory configuration (for testing)
	pub fn memory() -> Self {
		StorageConfig::Memory
	}

	/// Validate the configuration
	pub fn validate(&self) -> Result<()> {
		match self {
			StorageConfig::Filesystem { root } => {
				// Ensure parent directory exists or can be created
				if let Some(parent) = root.parent() {
					if !parent.exists() && parent != std::path::Path::new("") {
						return Err(StorageError::InvalidConfig {
							key: "STORAGE_FS_ROOT".to_string(),
							message: format!(
								"Parent directory does not exist: {}",
								parent.display()
							),
						});
					}
				}
				Ok(())
			}
			StorageConfig::S3 {
				bucket,
				access_key_id,
				secret_access_key,
				..
			} => {
				if bucket.is_empty() {
					return Err(StorageError::InvalidConfig {
						key: "STORAGE_S3_BUCKET".to_string(),
						message: "Bucket name cannot be empty".to_string(),
					});
				}
				if access_key_id.is_empty() {
					return Err(StorageError::InvalidConfig {
						key: "STORAGE_S3_ACCESS_KEY".to_string(),
						message: "Access key cannot be empty".to_string(),
					});
				}
				if secret_access_key.is_empty() {
					return Err(StorageError::InvalidConfig {
						key: "STORAGE_S3_SECRET_KEY".to_string(),
						message: "Secret key cannot be empty".to_string(),
					});
				}
				Ok(())
			}
			StorageConfig::Memory => Ok(()),
		}
	}

	/// Get a human-readable description of the storage type
	pub fn storage_type(&self) -> &'static str {
		match self {
			StorageConfig::Filesystem { .. } => "filesystem",
			StorageConfig::S3 { .. } => "s3",
			StorageConfig::Memory => "memory",
		}
	}
}

impl Default for StorageConfig {
	fn default() -> Self {
		StorageConfig::Filesystem {
			root: PathBuf::from("./blobs"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_default_config() {
		let config = StorageConfig::default();
		assert_eq!(config.storage_type(), "filesystem");
	}

	#[test]
	fn test_filesystem_config() {
		let config = StorageConfig::filesystem("/tmp/blobs");
		match config {
			StorageConfig::Filesystem { root } => {
				assert_eq!(root, PathBuf::from("/tmp/blobs"));
			}
			_ => panic!("Expected Filesystem config"),
		}
	}

	#[test]
	fn test_s3_config() {
		let config = StorageConfig::s3("my-bucket", "us-west-2", "key", "secret");
		match config {
			StorageConfig::S3 {
				bucket,
				region,
				endpoint,
				force_path_style,
				..
			} => {
				assert_eq!(bucket, "my-bucket");
				assert_eq!(region, "us-west-2");
				assert!(endpoint.is_none());
				assert!(!force_path_style);
			}
			_ => panic!("Expected S3 config"),
		}
	}

	#[test]
	fn test_minio_config() {
		let config =
			StorageConfig::s3_compatible("my-bucket", "http://localhost:9000", "key", "secret");
		match config {
			StorageConfig::S3 {
				endpoint,
				force_path_style,
				..
			} => {
				assert_eq!(endpoint, Some("http://localhost:9000".to_string()));
				assert!(force_path_style);
			}
			_ => panic!("Expected S3 config"),
		}
	}

	#[test]
	fn test_memory_config() {
		let config = StorageConfig::memory();
		assert_eq!(config.storage_type(), "memory");
	}

	#[test]
	fn test_validate_empty_bucket() {
		let config = StorageConfig::S3 {
			bucket: String::new(),
			region: "us-east-1".to_string(),
			endpoint: None,
			access_key_id: "key".to_string(),
			secret_access_key: "secret".to_string(),
			force_path_style: false,
		};
		assert!(config.validate().is_err());
	}
}
