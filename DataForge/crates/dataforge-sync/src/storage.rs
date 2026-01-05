//! Production blob storage with S3/MinIO presigned URL support
//!
//! Following Harbor's pattern for direct client-to-storage downloads:
//! - Server generates presigned URLs for authorized clients
//! - Client downloads directly from S3/MinIO (bypasses server)
//! - Reduces server bandwidth and improves download performance
//!
//! ## Architecture
//!
//! ```text
//!                 1. Request presigned URL
//! ┌────────┐ ─────────────────────────────────▶ ┌─────────────────┐
//! │ Client │                                    │ DataForge Sync  │
//! │        │ ◀───────────────────────────────── │ Server          │
//! └────────┘       2. Return presigned URL      └─────────────────┘
//!      │
//!      │   3. Direct download with presigned URL
//!      │   (bypasses server)
//!      ▼
//! ┌─────────────┐
//! │ S3 / MinIO  │
//! │ Bucket      │
//! └─────────────┘
//! ```

use opendal::services::S3;
use opendal::Operator;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use tracing::info;

/// Storage configuration for S3/MinIO
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageConfig {
	/// Storage backend type: "s3", "minio", or "filesystem"
	pub backend: StorageBackend,
	/// S3 region (e.g., "us-east-1", "auto" for MinIO)
	pub region: String,
	/// S3 endpoint URL (e.g., "https://s3.amazonaws.com" or "http://minio:9000")
	pub endpoint: String,
	/// S3 bucket name
	pub bucket: String,
	/// AWS access key ID
	pub access_key_id: String,
	/// AWS secret access key
	pub secret_access_key: String,
	/// Blob path prefix within bucket (e.g., "blobs/")
	#[serde(default = "default_prefix")]
	pub prefix: String,
	/// Default presigned URL expiration in seconds
	#[serde(default = "default_presign_expiry")]
	pub presign_expiry_seconds: u64,
}

fn default_prefix() -> String {
	"blobs/".to_string()
}

fn default_presign_expiry() -> u64 {
	3600 // 1 hour
}

/// Storage backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum StorageBackend {
	/// AWS S3
	#[default]
	S3,
	/// MinIO (S3-compatible)
	Minio,
	/// Local filesystem (for development/testing)
	Filesystem,
}

/// Storage errors
#[derive(Debug, Error)]
pub enum StorageError {
	#[error("Storage configuration error: {0}")]
	Config(String),
	#[error("OpenDAL error: {0}")]
	Opendal(#[from] opendal::Error),
	#[error("Blob not found: {0}")]
	NotFound(String),
	#[error("Presigned URL generation failed: {0}")]
	PresignFailed(String),
}

/// Production blob storage with S3/MinIO support
///
/// Following Harbor's presigned URL pattern for direct client-to-storage access.
pub struct BlobStorage {
	operator: Operator,
	config: StorageConfig,
}

impl BlobStorage {
	/// Create a new blob storage instance
	pub async fn new(config: StorageConfig) -> Result<Self, StorageError> {
		let operator = match config.backend {
			StorageBackend::S3 | StorageBackend::Minio => {
				Self::create_s3_operator(&config)?
			}
			StorageBackend::Filesystem => {
				Self::create_fs_operator(&config)?
			}
		};

		info!(
			"Initialized {} blob storage: bucket={}, prefix={}",
			match config.backend {
				StorageBackend::S3 => "S3",
				StorageBackend::Minio => "MinIO",
				StorageBackend::Filesystem => "Filesystem",
			},
			config.bucket,
			config.prefix
		);

		Ok(Self { operator, config })
	}

	/// Create S3/MinIO operator using OpenDAL
	fn create_s3_operator(config: &StorageConfig) -> Result<Operator, StorageError> {
		// OpenDAL builder methods consume self, so we chain them
		let mut builder = S3::default()
			.bucket(&config.bucket)
			.region(&config.region)
			.access_key_id(&config.access_key_id)
			.secret_access_key(&config.secret_access_key);

		// Endpoint configuration (for MinIO or custom S3)
		if !config.endpoint.is_empty() {
			builder = builder.endpoint(&config.endpoint);
		}

		// Enable virtual host style for MinIO compatibility
		if config.backend == StorageBackend::Minio {
			builder = builder.enable_virtual_host_style();
		}

		let operator = Operator::new(builder)
			.map_err(|e| StorageError::Config(e.to_string()))?
			.finish();

		Ok(operator)
	}

	/// Create filesystem operator (for development/testing)
	fn create_fs_operator(config: &StorageConfig) -> Result<Operator, StorageError> {
		use opendal::services::Fs;

		// OpenDAL builder methods consume self
		let builder = Fs::default()
			.root(&config.bucket); // Use bucket as root path

		let operator = Operator::new(builder)
			.map_err(|e| StorageError::Config(e.to_string()))?
			.finish();

		Ok(operator)
	}

	/// Get the full storage path for a blob hash
	fn blob_path(&self, hash: &str) -> String {
		// Content-addressed path: prefix/sha256/ab/cd/hash
		// This follows Harbor's pattern for efficient storage organization
		let prefix = if hash.starts_with("sha256:") {
			hash.strip_prefix("sha256:").unwrap_or(hash)
		} else {
			hash
		};

		if prefix.len() >= 4 {
			format!(
				"{}sha256/{}/{}/{}",
				self.config.prefix,
				&prefix[0..2],
				&prefix[2..4],
				prefix
			)
		} else {
			format!("{}sha256/{}", self.config.prefix, prefix)
		}
	}

	/// Generate a presigned URL for blob download (GET)
	///
	/// Following Harbor's presigned URL pattern:
	/// - Client requests presigned URL from server
	/// - Server validates access and generates presigned URL
	/// - Client downloads directly from S3/MinIO using presigned URL
	///
	/// # Arguments
	/// - `hash`: Blob hash (SHA-256 digest)
	/// - `expires_in`: Optional expiration duration (defaults to config value)
	///
	/// # Returns
	/// Presigned URL for direct GET request to S3/MinIO
	pub async fn presigned_download_url(
		&self,
		hash: &str,
		expires_in: Option<Duration>,
	) -> Result<PresignedUrl, StorageError> {
		let path = self.blob_path(hash);
		let expiry = expires_in.unwrap_or(Duration::from_secs(self.config.presign_expiry_seconds));

		// Check if blob exists first (better error messages)
		if !self.blob_exists(hash).await? {
			return Err(StorageError::NotFound(hash.to_string()));
		}

		// Generate presigned URL using OpenDAL
		let presigned = self
			.operator
			.presign_read(&path, expiry)
			.await
			.map_err(|e| StorageError::PresignFailed(e.to_string()))?;

		let expires_at = chrono::Utc::now() + chrono::Duration::from_std(expiry).unwrap_or_default();

		Ok(PresignedUrl {
			url: presigned.uri().to_string(),
			method: "GET".to_string(),
			expires_at: expires_at.to_rfc3339(),
		})
	}

	/// Generate a presigned URL for blob upload (PUT)
	///
	/// # Arguments
	/// - `hash`: Expected blob hash (SHA-256 digest)
	/// - `expires_in`: Optional expiration duration
	///
	/// # Returns
	/// Presigned URL for direct PUT request to S3/MinIO
	pub async fn presigned_upload_url(
		&self,
		hash: &str,
		expires_in: Option<Duration>,
	) -> Result<PresignedUrl, StorageError> {
		let path = self.blob_path(hash);
		let expiry = expires_in.unwrap_or(Duration::from_secs(self.config.presign_expiry_seconds));

		// Generate presigned URL for upload
		let presigned = self
			.operator
			.presign_write(&path, expiry)
			.await
			.map_err(|e| StorageError::PresignFailed(e.to_string()))?;

		let expires_at = chrono::Utc::now() + chrono::Duration::from_std(expiry).unwrap_or_default();

		Ok(PresignedUrl {
			url: presigned.uri().to_string(),
			method: "PUT".to_string(),
			expires_at: expires_at.to_rfc3339(),
		})
	}

	/// Generate presigned URLs for multiple blobs (batch operation)
	///
	/// # Arguments
	/// - `hashes`: List of blob hashes
	/// - `expires_in`: Optional expiration duration
	///
	/// # Returns
	/// Map of hash -> presigned URL
	pub async fn presigned_download_urls_batch(
		&self,
		hashes: &[String],
		expires_in: Option<Duration>,
	) -> Result<Vec<(String, PresignedUrl)>, StorageError> {
		let mut results = Vec::with_capacity(hashes.len());

		for hash in hashes {
			match self.presigned_download_url(hash, expires_in).await {
				Ok(url) => results.push((hash.clone(), url)),
				Err(StorageError::NotFound(_)) => {
					// Skip missing blobs (client may need to upload first)
					continue;
				}
				Err(e) => return Err(e),
			}
		}

		Ok(results)
	}

	/// Check if a blob exists in storage
	pub async fn blob_exists(&self, hash: &str) -> Result<bool, StorageError> {
		let path = self.blob_path(hash);
		match self.operator.exists(&path).await {
			Ok(exists) => Ok(exists),
			Err(e) => Err(StorageError::Opendal(e)),
		}
	}

	/// Get blob metadata (size, etc.)
	pub async fn blob_metadata(&self, hash: &str) -> Result<BlobMetadata, StorageError> {
		let path = self.blob_path(hash);
		let stat = self
			.operator
			.stat(&path)
			.await
			.map_err(|e| {
				if e.kind() == opendal::ErrorKind::NotFound {
					StorageError::NotFound(hash.to_string())
				} else {
					StorageError::Opendal(e)
				}
			})?;

		Ok(BlobMetadata {
			hash: hash.to_string(),
			size: stat.content_length(),
			content_type: stat.content_type().map(|s| s.to_string()),
			last_modified: stat.last_modified().map(|dt| dt.to_rfc3339()),
		})
	}

	/// Read blob content directly (for proxied downloads)
	pub async fn read_blob(&self, hash: &str) -> Result<Vec<u8>, StorageError> {
		let path = self.blob_path(hash);
		let data = self
			.operator
			.read(&path)
			.await
			.map_err(|e| {
				if e.kind() == opendal::ErrorKind::NotFound {
					StorageError::NotFound(hash.to_string())
				} else {
					StorageError::Opendal(e)
				}
			})?;

		Ok(data.to_vec())
	}

	/// Write blob content directly (for server-side uploads)
	pub async fn write_blob(&self, hash: &str, data: &[u8]) -> Result<(), StorageError> {
		let path = self.blob_path(hash);
		self.operator
			.write(&path, data.to_vec())
			.await
			.map_err(StorageError::Opendal)?;

		Ok(())
	}

	/// Delete a blob from storage
	pub async fn delete_blob(&self, hash: &str) -> Result<(), StorageError> {
		let path = self.blob_path(hash);
		self.operator
			.delete(&path)
			.await
			.map_err(StorageError::Opendal)?;

		Ok(())
	}
}

/// Presigned URL response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignedUrl {
	/// The presigned URL for direct access
	pub url: String,
	/// HTTP method to use (GET or PUT)
	pub method: String,
	/// Expiration timestamp (RFC3339)
	pub expires_at: String,
}

/// Blob metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobMetadata {
	/// Blob hash (SHA-256 digest)
	pub hash: String,
	/// Size in bytes
	pub size: u64,
	/// Content type (if known)
	pub content_type: Option<String>,
	/// Last modified timestamp (RFC3339)
	pub last_modified: Option<String>,
}

/// Request for presigned URL generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignRequest {
	/// Blob hash (SHA-256 digest)
	pub hash: String,
	/// URL expiration in seconds (optional, defaults to server config)
	pub expires_in: Option<u64>,
}

/// Response for presigned URL generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignResponse {
	/// Presigned URL for direct access
	pub url: String,
	/// HTTP method to use
	pub method: String,
	/// Expiration timestamp (RFC3339)
	pub expires_at: String,
}

/// Batch presigned URL request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPresignRequest {
	/// List of blob hashes
	pub hashes: Vec<String>,
	/// URL expiration in seconds (optional)
	pub expires_in: Option<u64>,
}

/// Batch presigned URL response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPresignResponse {
	/// Map of hash -> presigned URL info
	pub urls: Vec<PresignedBlobUrl>,
}

/// Individual blob presigned URL info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignedBlobUrl {
	/// Blob hash
	pub hash: String,
	/// Presigned URL
	pub url: String,
	/// Expiration timestamp (RFC3339)
	pub expires_at: String,
}

impl Default for StorageConfig {
	fn default() -> Self {
		Self {
			backend: StorageBackend::Filesystem,
			region: "us-east-1".to_string(),
			endpoint: String::new(),
			bucket: "./blob-storage".to_string(),
			access_key_id: String::new(),
			secret_access_key: String::new(),
			prefix: default_prefix(),
			presign_expiry_seconds: default_presign_expiry(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_blob_path() {
		let config = StorageConfig {
			prefix: "blobs/".to_string(),
			..Default::default()
		};

		// Create a mock storage to test blob_path (using filesystem backend)
		let storage = BlobStorage {
			operator: Operator::new(opendal::services::Memory::default()).unwrap().finish(),
			config,
		};

		// Test with full digest
		let hash = "abc123456789";
		let path = storage.blob_path(hash);
		assert_eq!(path, "blobs/sha256/ab/c1/abc123456789");

		// Test with sha256: prefix
		let hash_with_prefix = "sha256:def456789012";
		let path = storage.blob_path(hash_with_prefix);
		assert_eq!(path, "blobs/sha256/de/f4/def456789012");
	}

	#[test]
	fn test_storage_config_defaults() {
		let config = StorageConfig::default();
		assert_eq!(config.prefix, "blobs/");
		assert_eq!(config.presign_expiry_seconds, 3600);
		assert_eq!(config.backend, StorageBackend::Filesystem);
	}
}
