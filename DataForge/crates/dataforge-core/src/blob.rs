//! Content-addressed blob storage
//!
//! Stores Parquet files by their SHA-256 hash for deduplication and integrity.
//!
//! ## Storage Layout
//!
//! ```text
//! blobs/
//! ├── a3/
//! │   └── f2/
//! │       └── a3f2b8c9d1e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0.parquet
//! └── 7b/
//!     └── 1c/
//!         └── 7b1c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2.parquet
//! ```

use sha2::{Digest, Sha256};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tracing::{debug, info};

use crate::error::{Error, Result};

/// Content-addressed blob store
pub struct BlobStore {
	root: PathBuf,
}

impl BlobStore {
	/// Create a new blob store at the given root directory
	pub fn new(root: impl AsRef<Path>) -> Result<Self> {
		let root = root.as_ref().to_path_buf();
		fs::create_dir_all(&root)?;
		Ok(Self { root })
	}

	/// Store data and return its SHA-256 hash
	///
	/// If a blob with the same hash already exists, this is a no-op (deduplication).
	pub fn store(&self, data: &[u8]) -> Result<String> {
		let hash = Self::compute_hash(data);
		let path = self.hash_to_path(&hash);

		// Skip if already exists (deduplication)
		if path.exists() {
			debug!(hash = %hash, "Blob already exists, skipping");
			return Ok(hash);
		}

		// Create parent directories
		if let Some(parent) = path.parent() {
			fs::create_dir_all(parent)?;
		}

		// Write to temp file first, then rename (atomic)
		let temp_path = path.with_extension("tmp");
		{
			let mut file = fs::File::create(&temp_path)?;
			file.write_all(data)?;
			file.sync_all()?;
		}

		// Atomic rename
		fs::rename(&temp_path, &path)?;

		info!(hash = %hash, size = data.len(), "Stored new blob");
		Ok(hash)
	}

	/// Retrieve data by hash
	pub fn get(&self, hash: &str) -> Result<Vec<u8>> {
		let path = self.hash_to_path(hash);

		if !path.exists() {
			return Err(Error::BlobNotFound(hash.to_string()));
		}

		let mut file = fs::File::open(&path)?;
		let mut data = Vec::new();
		file.read_to_end(&mut data)?;

		// Verify integrity
		let actual_hash = Self::compute_hash(&data);
		if actual_hash != hash {
			return Err(Error::HashMismatch {
				expected: hash.to_string(),
				actual: actual_hash,
			});
		}

		Ok(data)
	}

	/// Check if a blob exists
	pub fn exists(&self, hash: &str) -> bool {
		self.hash_to_path(hash).exists()
	}

	/// Delete a blob by hash
	pub fn delete(&self, hash: &str) -> Result<bool> {
		let path = self.hash_to_path(hash);

		if path.exists() {
			fs::remove_file(&path)?;
			info!(hash = %hash, "Deleted blob");
			Ok(true)
		} else {
			Ok(false)
		}
	}

	/// Get the file path for a blob
	pub fn get_path(&self, hash: &str) -> PathBuf {
		self.hash_to_path(hash)
	}

	/// Get the size of a blob in bytes
	pub fn size(&self, hash: &str) -> Result<u64> {
		let path = self.hash_to_path(hash);

		if !path.exists() {
			return Err(Error::BlobNotFound(hash.to_string()));
		}

		let metadata = fs::metadata(&path)?;
		Ok(metadata.len())
	}

	/// List all blob hashes in the store
	pub fn list(&self) -> Result<Vec<String>> {
		let mut hashes = Vec::new();

		// Iterate through first-level directories (e.g., "a3")
		for entry1 in fs::read_dir(&self.root)? {
			let entry1 = entry1?;
			if !entry1.file_type()?.is_dir() {
				continue;
			}

			// Iterate through second-level directories (e.g., "f2")
			for entry2 in fs::read_dir(entry1.path())? {
				let entry2 = entry2?;
				if !entry2.file_type()?.is_dir() {
					continue;
				}

				// Iterate through parquet files
				for file in fs::read_dir(entry2.path())? {
					let file = file?;
					let path = file.path();

					if path.extension().map(|e| e == "parquet").unwrap_or(false) {
						if let Some(stem) = path.file_stem() {
							hashes.push(stem.to_string_lossy().to_string());
						}
					}
				}
			}
		}

		Ok(hashes)
	}

	/// Compute SHA-256 hash of data
	pub fn compute_hash(data: &[u8]) -> String {
		let mut hasher = Sha256::new();
		hasher.update(data);
		hex::encode(hasher.finalize())
	}

	/// Convert hash to file path
	///
	/// Uses two levels of directory nesting to avoid too many files in one directory.
	/// e.g., "a3f2b8c9..." -> "blobs/a3/f2/a3f2b8c9....parquet"
	fn hash_to_path(&self, hash: &str) -> PathBuf {
		let prefix1 = &hash[0..2];
		let prefix2 = &hash[2..4];
		self.root
			.join(prefix1)
			.join(prefix2)
			.join(format!("{}.parquet", hash))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use tempfile::tempdir;

	#[test]
	fn test_store_and_get() {
		let dir = tempdir().unwrap();
		let store = BlobStore::new(dir.path()).unwrap();

		let data = b"Hello, World!";
		let hash = store.store(data).unwrap();

		assert!(store.exists(&hash));

		let retrieved = store.get(&hash).unwrap();
		assert_eq!(retrieved, data);
	}

	#[test]
	fn test_deduplication() {
		let dir = tempdir().unwrap();
		let store = BlobStore::new(dir.path()).unwrap();

		let data = b"Duplicate data";
		let hash1 = store.store(data).unwrap();
		let hash2 = store.store(data).unwrap();

		assert_eq!(hash1, hash2);
	}

	#[test]
	fn test_blob_not_found() {
		let dir = tempdir().unwrap();
		let store = BlobStore::new(dir.path()).unwrap();

		let result = store.get("nonexistent_hash");
		assert!(matches!(result, Err(Error::BlobNotFound(_))));
	}

	#[test]
	fn test_list_blobs() {
		let dir = tempdir().unwrap();
		let store = BlobStore::new(dir.path()).unwrap();

		let hash1 = store.store(b"Data 1").unwrap();
		let hash2 = store.store(b"Data 2").unwrap();

		let hashes = store.list().unwrap();
		assert!(hashes.contains(&hash1));
		assert!(hashes.contains(&hash2));
	}

	#[test]
	fn test_delete() {
		let dir = tempdir().unwrap();
		let store = BlobStore::new(dir.path()).unwrap();

		let hash = store.store(b"To be deleted").unwrap();
		assert!(store.exists(&hash));

		let deleted = store.delete(&hash).unwrap();
		assert!(deleted);
		assert!(!store.exists(&hash));
	}
}
