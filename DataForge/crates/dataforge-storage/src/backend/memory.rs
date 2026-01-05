//! In-memory storage backend
//!
//! Stores blobs in memory. Useful for testing and development.

use super::{StorageBackend, StorageMetadata};
use crate::error::{Result, StorageError};
use crate::{compute_hash, hash_to_path};
use async_trait::async_trait;
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Duration;
use tracing::debug;

/// In-memory storage backend for testing
pub struct MemoryBackend {
	storage: RwLock<HashMap<String, Vec<u8>>>,
}

impl MemoryBackend {
	/// Create a new in-memory backend
	pub fn new() -> Self {
		Self {
			storage: RwLock::new(HashMap::new()),
		}
	}
}

impl Default for MemoryBackend {
	fn default() -> Self {
		Self::new()
	}
}

#[async_trait]
impl StorageBackend for MemoryBackend {
	async fn store(&self, data: &[u8]) -> Result<String> {
		let hash = compute_hash(data);

		let mut storage = self.storage.write().map_err(|_| {
			StorageError::Config("Failed to acquire write lock on memory storage".to_string())
		})?;

		// Deduplication - don't store if already exists
		if storage.contains_key(&hash) {
			debug!(hash = %hash, "Blob already exists in memory, skipping");
			return Ok(hash);
		}

		storage.insert(hash.clone(), data.to_vec());
		debug!(hash = %hash, size = data.len(), "Stored blob in memory");
		Ok(hash)
	}

	async fn get(&self, hash: &str) -> Result<Bytes> {
		let storage = self.storage.read().map_err(|_| {
			StorageError::Config("Failed to acquire read lock on memory storage".to_string())
		})?;

		storage
			.get(hash)
			.map(|data| Bytes::from(data.clone()))
			.ok_or_else(|| StorageError::NotFound(hash.to_string()))
	}

	async fn exists(&self, hash: &str) -> Result<bool> {
		let storage = self.storage.read().map_err(|_| {
			StorageError::Config("Failed to acquire read lock on memory storage".to_string())
		})?;
		Ok(storage.contains_key(hash))
	}

	async fn delete(&self, hash: &str) -> Result<()> {
		let mut storage = self.storage.write().map_err(|_| {
			StorageError::Config("Failed to acquire write lock on memory storage".to_string())
		})?;
		storage.remove(hash);
		debug!(hash = %hash, "Deleted blob from memory");
		Ok(())
	}

	async fn get_metadata(&self, hash: &str) -> Result<StorageMetadata> {
		let storage = self.storage.read().map_err(|_| {
			StorageError::Config("Failed to acquire read lock on memory storage".to_string())
		})?;

		let data = storage
			.get(hash)
			.ok_or_else(|| StorageError::NotFound(hash.to_string()))?;

		Ok(StorageMetadata {
			size: data.len() as u64,
			content_type: Some("application/octet-stream".to_string()),
			last_modified: None,
			etag: Some(hash.to_string()),
		})
	}

	async fn get_presigned_url(&self, _hash: &str, _expires_in: Duration) -> Result<String> {
		Err(StorageError::PresignedUrlNotSupported("memory".to_string()))
	}

	fn get_path(&self, hash: &str) -> String {
		hash_to_path("memory", hash)
	}

	fn backend_type(&self) -> &'static str {
		"memory"
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_store_and_get() {
		let backend = MemoryBackend::new();

		let data = b"Hello, World!";
		let hash = backend.store(data).await.unwrap();

		assert!(backend.exists(&hash).await.unwrap());

		let retrieved = backend.get(&hash).await.unwrap();
		assert_eq!(retrieved.as_ref(), data);
	}

	#[tokio::test]
	async fn test_deduplication() {
		let backend = MemoryBackend::new();

		let data = b"Duplicate data";
		let hash1 = backend.store(data).await.unwrap();
		let hash2 = backend.store(data).await.unwrap();

		assert_eq!(hash1, hash2);
	}

	#[tokio::test]
	async fn test_not_found() {
		let backend = MemoryBackend::new();
		let result = backend.get("nonexistent_hash").await;
		assert!(matches!(result, Err(StorageError::NotFound(_))));
	}

	#[tokio::test]
	async fn test_delete() {
		let backend = MemoryBackend::new();

		let hash = backend.store(b"To be deleted").await.unwrap();
		assert!(backend.exists(&hash).await.unwrap());

		backend.delete(&hash).await.unwrap();
		assert!(!backend.exists(&hash).await.unwrap());
	}

	#[tokio::test]
	async fn test_metadata() {
		let backend = MemoryBackend::new();

		let data = b"Test data for metadata";
		let hash = backend.store(data).await.unwrap();

		let metadata = backend.get_metadata(&hash).await.unwrap();
		assert_eq!(metadata.size, data.len() as u64);
	}

	#[tokio::test]
	async fn test_presigned_url_not_supported() {
		let backend = MemoryBackend::new();
		let hash = backend.store(b"test").await.unwrap();

		let result = backend
			.get_presigned_url(&hash, Duration::from_secs(3600))
			.await;
		assert!(matches!(
			result,
			Err(StorageError::PresignedUrlNotSupported(_))
		));
	}
}
