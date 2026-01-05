//! Filesystem storage backend
//!
//! Stores blobs on the local filesystem using content-addressed storage.

use super::{StorageBackend, StorageMetadata};
use crate::error::{Result, StorageError};
use crate::{compute_hash, hash_to_path};
use async_trait::async_trait;
use bytes::Bytes;
use opendal::{services, Operator};
use std::path::PathBuf;
use std::time::Duration;
use tracing::{debug, info};

/// Filesystem storage backend using OpenDAL
pub struct FilesystemBackend {
	operator: Operator,
	root: PathBuf,
	prefix: String,
}

impl FilesystemBackend {
	/// Create a new filesystem backend
	///
	/// # Arguments
	/// * `root` - Root directory for blob storage
	/// * `prefix` - Subdirectory prefix (typically "blobs")
	pub fn new(root: impl Into<PathBuf>, prefix: impl Into<String>) -> Result<Self> {
		let root = root.into();
		let prefix = prefix.into();

		// Create root directory if it doesn't exist
		std::fs::create_dir_all(&root)?;

		let builder = services::Fs::default().root(root.to_string_lossy().as_ref());

		let operator = Operator::new(builder)?.finish();

		info!(root = %root.display(), prefix = %prefix, "Initialized filesystem storage");

		Ok(Self {
			operator,
			root,
			prefix,
		})
	}

	/// Create with default "blobs" prefix
	pub fn with_root(root: impl Into<PathBuf>) -> Result<Self> {
		Self::new(root, "blobs")
	}
}

#[async_trait]
impl StorageBackend for FilesystemBackend {
	async fn store(&self, data: &[u8]) -> Result<String> {
		let hash = compute_hash(data);
		let path = hash_to_path(&self.prefix, &hash);

		// Check if already exists (deduplication)
		if self.operator.exists(&path).await? {
			debug!(hash = %hash, "Blob already exists, skipping");
			return Ok(hash);
		}

		// Write blob
		self.operator.write(&path, data.to_vec()).await?;

		info!(hash = %hash, size = data.len(), "Stored blob to filesystem");
		Ok(hash)
	}

	async fn get(&self, hash: &str) -> Result<Bytes> {
		let path = hash_to_path(&self.prefix, hash);

		if !self.operator.exists(&path).await? {
			return Err(StorageError::NotFound(hash.to_string()));
		}

		let data = self.operator.read(&path).await?;
		let bytes = data.to_bytes();

		// Verify integrity
		let actual_hash = compute_hash(&bytes);
		if actual_hash != hash {
			return Err(StorageError::HashMismatch {
				expected: hash.to_string(),
				actual: actual_hash,
			});
		}

		Ok(bytes)
	}

	async fn exists(&self, hash: &str) -> Result<bool> {
		let path = hash_to_path(&self.prefix, hash);
		Ok(self.operator.exists(&path).await?)
	}

	async fn delete(&self, hash: &str) -> Result<()> {
		let path = hash_to_path(&self.prefix, hash);
		self.operator.delete(&path).await?;
		info!(hash = %hash, "Deleted blob from filesystem");
		Ok(())
	}

	async fn get_metadata(&self, hash: &str) -> Result<StorageMetadata> {
		let path = hash_to_path(&self.prefix, hash);

		if !self.operator.exists(&path).await? {
			return Err(StorageError::NotFound(hash.to_string()));
		}

		let stat = self.operator.stat(&path).await?;

		Ok(StorageMetadata {
			size: stat.content_length(),
			content_type: stat.content_type().map(|s| s.to_string()),
			last_modified: stat.last_modified().map(|t| t.timestamp()),
			etag: stat.etag().map(|s| s.to_string()),
		})
	}

	async fn get_presigned_url(&self, hash: &str, _expires_in: Duration) -> Result<String> {
		// Filesystem doesn't support presigned URLs in the traditional sense
		// Return a file:// URL or indicate this should be served via an API endpoint
		let path = self.root.join(hash_to_path(&self.prefix, hash));
		Ok(format!("file://{}", path.display()))
	}

	fn get_path(&self, hash: &str) -> String {
		let relative = hash_to_path(&self.prefix, hash);
		self.root.join(&relative).to_string_lossy().to_string()
	}

	fn backend_type(&self) -> &'static str {
		"filesystem"
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tempfile::tempdir;

	#[tokio::test]
	async fn test_store_and_get() {
		let dir = tempdir().unwrap();
		let backend = FilesystemBackend::with_root(dir.path()).unwrap();

		let data = b"Hello, World!";
		let hash = backend.store(data).await.unwrap();

		assert!(backend.exists(&hash).await.unwrap());

		let retrieved = backend.get(&hash).await.unwrap();
		assert_eq!(retrieved.as_ref(), data);
	}

	#[tokio::test]
	async fn test_deduplication() {
		let dir = tempdir().unwrap();
		let backend = FilesystemBackend::with_root(dir.path()).unwrap();

		let data = b"Duplicate data";
		let hash1 = backend.store(data).await.unwrap();
		let hash2 = backend.store(data).await.unwrap();

		assert_eq!(hash1, hash2);
	}

	#[tokio::test]
	async fn test_not_found() {
		let dir = tempdir().unwrap();
		let backend = FilesystemBackend::with_root(dir.path()).unwrap();

		let result = backend.get("nonexistent_hash_value").await;
		assert!(matches!(result, Err(StorageError::NotFound(_))));
	}

	#[tokio::test]
	async fn test_delete() {
		let dir = tempdir().unwrap();
		let backend = FilesystemBackend::with_root(dir.path()).unwrap();

		let hash = backend.store(b"To be deleted").await.unwrap();
		assert!(backend.exists(&hash).await.unwrap());

		backend.delete(&hash).await.unwrap();
		assert!(!backend.exists(&hash).await.unwrap());
	}

	#[tokio::test]
	async fn test_metadata() {
		let dir = tempdir().unwrap();
		let backend = FilesystemBackend::with_root(dir.path()).unwrap();

		let data = b"Test data for metadata";
		let hash = backend.store(data).await.unwrap();

		let metadata = backend.get_metadata(&hash).await.unwrap();
		assert_eq!(metadata.size, data.len() as u64);
	}
}
