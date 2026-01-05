//! Storage backend trait and implementations
//!
//! Defines the `StorageBackend` trait that all storage implementations must satisfy,
//! following ColaNode's StorageInterface pattern.

pub mod filesystem;
pub mod memory;
pub mod s3;

use crate::error::Result;
use async_trait::async_trait;
use bytes::Bytes;
use std::time::Duration;

/// Metadata about a stored blob
#[derive(Debug, Clone)]
pub struct StorageMetadata {
	/// Size in bytes
	pub size: u64,
	/// Content type (e.g., "application/octet-stream")
	pub content_type: Option<String>,
	/// Last modified timestamp (Unix timestamp)
	pub last_modified: Option<i64>,
	/// ETag or hash
	pub etag: Option<String>,
}

impl Default for StorageMetadata {
	fn default() -> Self {
		Self {
			size: 0,
			content_type: Some("application/octet-stream".to_string()),
			last_modified: None,
			etag: None,
		}
	}
}

/// Storage backend trait following ColaNode's StorageInterface pattern
///
/// All storage backends (filesystem, S3, memory) implement this trait,
/// allowing the application to swap backends without code changes.
#[async_trait]
pub trait StorageBackend: Send + Sync {
	/// Store data and return its SHA-256 hash
	///
	/// If a blob with the same hash already exists, this should be a no-op
	/// (deduplication) and return the existing hash.
	///
	/// # Arguments
	/// * `data` - The data to store
	///
	/// # Returns
	/// The SHA-256 hash of the stored data (hex-encoded)
	async fn store(&self, data: &[u8]) -> Result<String>;

	/// Retrieve data by hash
	///
	/// # Arguments
	/// * `hash` - The SHA-256 hash of the blob to retrieve
	///
	/// # Returns
	/// The blob data, or error if not found
	async fn get(&self, hash: &str) -> Result<Bytes>;

	/// Check if a blob exists
	///
	/// # Arguments
	/// * `hash` - The SHA-256 hash to check
	///
	/// # Returns
	/// `true` if the blob exists
	async fn exists(&self, hash: &str) -> Result<bool>;

	/// Delete a blob by hash
	///
	/// # Arguments
	/// * `hash` - The SHA-256 hash of the blob to delete
	async fn delete(&self, hash: &str) -> Result<()>;

	/// Get metadata about a blob
	///
	/// # Arguments
	/// * `hash` - The SHA-256 hash of the blob
	///
	/// # Returns
	/// Metadata about the blob (size, content type, etc.)
	async fn get_metadata(&self, hash: &str) -> Result<StorageMetadata>;

	/// Generate a presigned URL for direct access
	///
	/// This is primarily useful for S3-compatible backends where clients
	/// can download blobs directly without going through the server.
	///
	/// # Arguments
	/// * `hash` - The SHA-256 hash of the blob
	/// * `expires_in` - How long the URL should be valid
	///
	/// # Returns
	/// A presigned URL, or error if not supported
	async fn get_presigned_url(&self, hash: &str, expires_in: Duration) -> Result<String>;

	/// Get the storage path for a blob (for debugging/logging)
	fn get_path(&self, hash: &str) -> String;

	/// Get the backend type name (for logging)
	fn backend_type(&self) -> &'static str;
}
