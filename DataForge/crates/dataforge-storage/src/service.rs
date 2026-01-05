//! Storage service singleton for Tauri
//!
//! Following ColaNode's pattern, this module provides a singleton service
//! that holds the storage backend and can be shared across the application.

use crate::backend::{StorageBackend, StorageMetadata};
use crate::config::StorageConfig;
use crate::error::{Result, StorageError};
use crate::factory::StorageFactory;
use bytes::Bytes;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::OnceCell;
use tracing::info;

/// Global storage service instance
static STORAGE_SERVICE: OnceCell<StorageService> = OnceCell::const_new();

/// Storage service singleton for Tauri commands
///
/// This service holds the storage backend and provides a unified interface
/// for all storage operations. It follows ColaNode's singleton pattern
/// where the service is initialized once at app startup.
///
/// # Example
///
/// ```ignore
/// use dataforge_storage::{StorageService, StorageConfig};
///
/// // Initialize once at startup (typically in Tauri setup)
/// let config = StorageConfig::from_env()?;
/// StorageService::initialize(config).await?;
///
/// // Use anywhere in the application
/// let service = StorageService::instance()?;
/// let hash = service.store(b"Hello, World!").await?;
/// ```
#[derive(Clone)]
pub struct StorageService {
	backend: Arc<dyn StorageBackend>,
}

impl StorageService {
	/// Create a new storage service with the given backend
	pub fn new(backend: Arc<dyn StorageBackend>) -> Self {
		Self { backend }
	}

	/// Create a storage service from configuration
	pub fn from_config(config: StorageConfig) -> Result<Self> {
		let backend = StorageFactory::create(config)?;
		Ok(Self::new(backend))
	}

	/// Initialize the global storage service singleton
	///
	/// This should be called once during app startup.
	/// Returns an error if already initialized.
	pub async fn initialize(config: StorageConfig) -> Result<()> {
		let service = Self::from_config(config)?;

		STORAGE_SERVICE
			.set(service)
			.map_err(|_| StorageError::Config("Storage service already initialized".to_string()))?;

		info!("Storage service initialized");
		Ok(())
	}

	/// Get the global storage service instance
	///
	/// Returns an error if not initialized.
	pub fn instance() -> Result<&'static StorageService> {
		STORAGE_SERVICE.get().ok_or(StorageError::NotInitialized)
	}

	/// Check if the storage service is initialized
	pub fn is_initialized() -> bool {
		STORAGE_SERVICE.get().is_some()
	}

	/// Get the underlying backend (for advanced usage)
	pub fn backend(&self) -> &Arc<dyn StorageBackend> {
		&self.backend
	}

	/// Get the backend type name
	pub fn backend_type(&self) -> &'static str {
		self.backend.backend_type()
	}

	// ============ Delegated StorageBackend methods ============

	/// Store data and return its SHA-256 hash
	pub async fn store(&self, data: &[u8]) -> Result<String> {
		self.backend.store(data).await
	}

	/// Retrieve data by hash
	pub async fn get(&self, hash: &str) -> Result<Bytes> {
		self.backend.get(hash).await
	}

	/// Check if a blob exists
	pub async fn exists(&self, hash: &str) -> Result<bool> {
		self.backend.exists(hash).await
	}

	/// Delete a blob by hash
	pub async fn delete(&self, hash: &str) -> Result<()> {
		self.backend.delete(hash).await
	}

	/// Get metadata about a blob
	pub async fn get_metadata(&self, hash: &str) -> Result<StorageMetadata> {
		self.backend.get_metadata(hash).await
	}

	/// Generate a presigned URL for direct access
	pub async fn get_presigned_url(&self, hash: &str, expires_in: Duration) -> Result<String> {
		self.backend.get_presigned_url(hash, expires_in).await
	}

	/// Get the storage path for a blob
	pub fn get_path(&self, hash: &str) -> String {
		self.backend.get_path(hash)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::backend::memory::MemoryBackend;

	#[tokio::test]
	async fn test_service_operations() {
		// Create service with in-memory backend (not using global singleton)
		let backend = Arc::new(MemoryBackend::new());
		let service = StorageService::new(backend);

		// Test store
		let data = b"Hello, World!";
		let hash = service.store(data).await.unwrap();
		assert_eq!(hash.len(), 64);

		// Test exists
		assert!(service.exists(&hash).await.unwrap());

		// Test get
		let retrieved = service.get(&hash).await.unwrap();
		assert_eq!(retrieved.as_ref(), data);

		// Test metadata
		let metadata = service.get_metadata(&hash).await.unwrap();
		assert_eq!(metadata.size, data.len() as u64);

		// Test delete
		service.delete(&hash).await.unwrap();
		assert!(!service.exists(&hash).await.unwrap());
	}

	#[test]
	fn test_service_from_config() {
		let config = StorageConfig::Memory;
		let service = StorageService::from_config(config).unwrap();
		assert_eq!(service.backend_type(), "memory");
	}
}
