//! DataForge Storage
//!
//! Storage abstraction following ColaNode patterns for pluggable backends.
//!
//! This module provides a unified interface for storing and retrieving
//! content-addressed blobs (Parquet files) regardless of the underlying storage.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │  StorageService (Singleton)                                  │
//! │  - Initialized once at app startup                           │
//! │  - Holds StorageBackend instance                             │
//! └────────────────────┬────────────────────────────────────────┘
//!                      │
//! ┌────────────────────▼────────────────────────────────────────┐
//! │  StorageBackend Trait                                        │
//! │  - store(), get(), exists(), delete()                        │
//! │  - get_presigned_url(), get_metadata()                       │
//! └────────────────────┬────────────────────────────────────────┘
//!                      │
//!        ┌─────────────┼─────────────┐
//!        │             │             │
//! ┌──────▼──────┐ ┌───▼────┐ ┌─────▼─────┐
//! │ Filesystem  │ │   S3   │ │  Memory   │
//! └─────────────┘ └────────┘ └───────────┘
//! ```
//!
//! # Configuration
//!
//! Storage backend is configured via environment variables:
//!
//! ```bash
//! # Filesystem (default)
//! STORAGE_TYPE=filesystem
//! STORAGE_FS_ROOT=/path/to/blobs
//!
//! # S3/MinIO
//! STORAGE_TYPE=s3
//! STORAGE_S3_BUCKET=dataforge-blobs
//! STORAGE_S3_REGION=us-east-1
//! STORAGE_S3_ENDPOINT=http://minio:9000  # Optional
//! STORAGE_S3_ACCESS_KEY=...
//! STORAGE_S3_SECRET_KEY=...
//! ```
//!
//! # Example
//!
//! ```ignore
//! use dataforge_storage::{StorageService, StorageConfig};
//!
//! // Create storage service from config
//! let config = StorageConfig::from_env()?;
//! let service = StorageService::new(config)?;
//!
//! // Store a blob
//! let hash = service.store(b"Hello, World!").await?;
//!
//! // Retrieve a blob
//! let data = service.get(&hash).await?;
//! ```

mod backend;
mod config;
mod error;
mod factory;
mod service;

pub use backend::{StorageBackend, StorageMetadata};
pub use config::StorageConfig;
pub use error::{Result, StorageError};
pub use factory::StorageFactory;
pub use service::StorageService;

// Re-export backend implementations for direct use if needed
pub use backend::filesystem::FilesystemBackend;
pub use backend::memory::MemoryBackend;
pub use backend::s3::S3Backend;

/// Compute SHA-256 hash of data (utility function)
pub fn compute_hash(data: &[u8]) -> String {
	use sha2::{Digest, Sha256};
	let mut hasher = Sha256::new();
	hasher.update(data);
	hex::encode(hasher.finalize())
}

/// Convert hash to storage path with two-level directory nesting
///
/// e.g., "a3f2b8c9..." -> "blobs/a3/f2/a3f2b8c9....parquet"
pub fn hash_to_path(prefix: &str, hash: &str) -> String {
	let prefix1 = &hash[0..2];
	let prefix2 = &hash[2..4];
	format!("{}/{}/{}/{}.parquet", prefix, prefix1, prefix2, hash)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_compute_hash() {
		let hash = compute_hash(b"Hello, World!");
		assert_eq!(hash.len(), 64); // SHA-256 = 32 bytes = 64 hex chars
	}

	#[test]
	fn test_hash_to_path() {
		let path = hash_to_path(
			"blobs",
			"a3f2b8c9d1e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0",
		);
		assert_eq!(
			path,
			"blobs/a3/f2/a3f2b8c9d1e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0.parquet"
		);
	}
}
