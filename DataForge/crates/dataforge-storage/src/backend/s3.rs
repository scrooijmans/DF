//! S3-compatible storage backend
//!
//! Stores blobs on S3, MinIO, DigitalOcean Spaces, or any S3-compatible service.
//! Supports presigned URLs for direct client access.

use super::{StorageBackend, StorageMetadata};
use crate::error::{Result, StorageError};
use crate::{compute_hash, hash_to_path};
use async_trait::async_trait;
use bytes::Bytes;
use opendal::{services, Operator};
use std::time::Duration;
use tracing::{debug, info};

/// S3-compatible storage backend using OpenDAL
pub struct S3Backend {
	operator: Operator,
	bucket: String,
	prefix: String,
}

impl S3Backend {
	/// Create a new S3 backend
	///
	/// # Arguments
	/// * `bucket` - S3 bucket name
	/// * `region` - AWS region
	/// * `access_key_id` - Access key
	/// * `secret_access_key` - Secret key
	/// * `prefix` - Subdirectory prefix for blobs
	pub fn new(
		bucket: impl Into<String>,
		region: impl Into<String>,
		access_key_id: impl Into<String>,
		secret_access_key: impl Into<String>,
		prefix: impl Into<String>,
	) -> Result<Self> {
		let bucket = bucket.into();
		let region = region.into();
		let prefix = prefix.into();

		let builder = services::S3::default()
			.bucket(&bucket)
			.region(&region)
			.access_key_id(&access_key_id.into())
			.secret_access_key(&secret_access_key.into());

		let operator = Operator::new(builder)?.finish();

		info!(
			bucket = %bucket,
			region = %region,
			prefix = %prefix,
			"Initialized S3 storage"
		);

		Ok(Self {
			operator,
			bucket,
			prefix,
		})
	}

	/// Create a new S3 backend with a custom endpoint (for MinIO, etc.)
	///
	/// # Arguments
	/// * `bucket` - S3 bucket name
	/// * `endpoint` - Custom endpoint URL (e.g., "http://localhost:9000")
	/// * `region` - AWS region (use "us-east-1" for MinIO)
	/// * `access_key_id` - Access key
	/// * `secret_access_key` - Secret key
	/// * `force_path_style` - Use path-style URLs (required for MinIO)
	/// * `prefix` - Subdirectory prefix for blobs
	pub fn with_endpoint(
		bucket: impl Into<String>,
		endpoint: impl Into<String>,
		region: impl Into<String>,
		access_key_id: impl Into<String>,
		secret_access_key: impl Into<String>,
		force_path_style: bool,
		prefix: impl Into<String>,
	) -> Result<Self> {
		let bucket = bucket.into();
		let endpoint = endpoint.into();
		let region = region.into();
		let prefix = prefix.into();

		let mut builder = services::S3::default()
			.bucket(&bucket)
			.endpoint(&endpoint)
			.region(&region)
			.access_key_id(&access_key_id.into())
			.secret_access_key(&secret_access_key.into());

		if force_path_style {
			builder = builder.enable_virtual_host_style();
		}

		let operator = Operator::new(builder)?.finish();

		info!(
			bucket = %bucket,
			endpoint = %endpoint,
			prefix = %prefix,
			"Initialized S3-compatible storage"
		);

		Ok(Self {
			operator,
			bucket,
			prefix,
		})
	}

	/// Create with default "blobs" prefix
	pub fn with_bucket(
		bucket: impl Into<String>,
		region: impl Into<String>,
		access_key_id: impl Into<String>,
		secret_access_key: impl Into<String>,
	) -> Result<Self> {
		Self::new(bucket, region, access_key_id, secret_access_key, "blobs")
	}
}

#[async_trait]
impl StorageBackend for S3Backend {
	async fn store(&self, data: &[u8]) -> Result<String> {
		let hash = compute_hash(data);
		let path = hash_to_path(&self.prefix, &hash);

		// Check if already exists (deduplication)
		if self.operator.exists(&path).await? {
			debug!(hash = %hash, "Blob already exists in S3, skipping");
			return Ok(hash);
		}

		// Write blob
		self.operator.write(&path, data.to_vec()).await?;

		info!(hash = %hash, size = data.len(), bucket = %self.bucket, "Stored blob to S3");
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
		info!(hash = %hash, bucket = %self.bucket, "Deleted blob from S3");
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

	async fn get_presigned_url(&self, hash: &str, expires_in: Duration) -> Result<String> {
		let path = hash_to_path(&self.prefix, hash);

		// Check if blob exists first
		if !self.operator.exists(&path).await? {
			return Err(StorageError::NotFound(hash.to_string()));
		}

		// Generate presigned URL using OpenDAL
		let presigned = self
			.operator
			.presign_read(&path, expires_in)
			.await
			.map_err(|e| StorageError::Config(format!("Failed to generate presigned URL: {}", e)))?;

		Ok(presigned.uri().to_string())
	}

	fn get_path(&self, hash: &str) -> String {
		format!("s3://{}/{}", self.bucket, hash_to_path(&self.prefix, hash))
	}

	fn backend_type(&self) -> &'static str {
		"s3"
	}
}

// Note: S3 backend tests require a running S3-compatible service.
// For unit testing, use the MemoryBackend or mock the S3 service.
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_path_generation() {
		// This doesn't require actual S3 connection
		let hash = "a3f2b8c9d1e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0";
		let path = hash_to_path("blobs", hash);
		assert!(path.starts_with("blobs/a3/f2/"));
		assert!(path.ends_with(".parquet"));
	}
}
