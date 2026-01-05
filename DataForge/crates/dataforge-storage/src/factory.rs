//! Storage factory for creating backends from configuration
//!
//! Following ColaNode's StorageFactory pattern, this module provides
//! a centralized way to create storage backends from configuration.

use crate::backend::filesystem::FilesystemBackend;
use crate::backend::memory::MemoryBackend;
use crate::backend::s3::S3Backend;
use crate::backend::StorageBackend;
use crate::config::StorageConfig;
use crate::error::Result;
use std::sync::Arc;
use tracing::info;

/// Factory for creating storage backends from configuration
///
/// Following ColaNode patterns, this factory creates the appropriate
/// backend implementation based on the configuration.
///
/// # Example
///
/// ```ignore
/// use dataforge_storage::{StorageConfig, StorageFactory};
///
/// // From environment variables
/// let config = StorageConfig::from_env()?;
/// let backend = StorageFactory::create(config)?;
///
/// // Or programmatically
/// let config = StorageConfig::filesystem("/path/to/blobs");
/// let backend = StorageFactory::create(config)?;
/// ```
pub struct StorageFactory;

impl StorageFactory {
	/// Create a storage backend from configuration
	///
	/// Validates the configuration before creating the backend.
	///
	/// # Arguments
	/// * `config` - Storage configuration
	///
	/// # Returns
	/// A boxed storage backend ready for use
	pub fn create(config: StorageConfig) -> Result<Arc<dyn StorageBackend>> {
		// Validate configuration first
		config.validate()?;

		info!(storage_type = %config.storage_type(), "Creating storage backend");

		match config {
			StorageConfig::Filesystem { root } => {
				let backend = FilesystemBackend::with_root(root)?;
				Ok(Arc::new(backend))
			}

			StorageConfig::S3 {
				bucket,
				region,
				endpoint,
				access_key_id,
				secret_access_key,
				force_path_style,
			} => {
				let backend = if let Some(endpoint) = endpoint {
					S3Backend::with_endpoint(
						bucket,
						endpoint,
						region,
						access_key_id,
						secret_access_key,
						force_path_style,
						"blobs",
					)?
				} else {
					S3Backend::new(bucket, region, access_key_id, secret_access_key, "blobs")?
				};
				Ok(Arc::new(backend))
			}

			StorageConfig::Memory => {
				let backend = MemoryBackend::new();
				Ok(Arc::new(backend))
			}
		}
	}

	/// Create a storage backend from environment variables
	///
	/// Convenience method that reads configuration from env and creates backend.
	pub fn from_env() -> Result<Arc<dyn StorageBackend>> {
		let config = StorageConfig::from_env()?;
		Self::create(config)
	}

	/// Create an in-memory backend (for testing)
	pub fn memory() -> Arc<dyn StorageBackend> {
		Arc::new(MemoryBackend::new())
	}

	/// Create a filesystem backend with the given root
	pub fn filesystem(root: impl Into<std::path::PathBuf>) -> Result<Arc<dyn StorageBackend>> {
		let backend = FilesystemBackend::with_root(root)?;
		Ok(Arc::new(backend))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_create_memory_backend() {
		let config = StorageConfig::Memory;
		let backend = StorageFactory::create(config).unwrap();
		assert_eq!(backend.backend_type(), "memory");
	}

	#[test]
	fn test_create_filesystem_backend() {
		let temp_dir = std::env::temp_dir().join("dataforge-storage-test");
		let config = StorageConfig::filesystem(&temp_dir);
		let backend = StorageFactory::create(config).unwrap();
		assert_eq!(backend.backend_type(), "filesystem");

		// Cleanup
		let _ = std::fs::remove_dir_all(&temp_dir);
	}

	#[test]
	fn test_memory_shorthand() {
		let backend = StorageFactory::memory();
		assert_eq!(backend.backend_type(), "memory");
	}

	#[test]
	fn test_filesystem_shorthand() {
		let temp_dir = std::env::temp_dir().join("dataforge-storage-test-2");
		let backend = StorageFactory::filesystem(&temp_dir).unwrap();
		assert_eq!(backend.backend_type(), "filesystem");

		// Cleanup
		let _ = std::fs::remove_dir_all(&temp_dir);
	}
}
