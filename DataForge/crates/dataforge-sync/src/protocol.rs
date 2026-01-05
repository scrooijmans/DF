//! Sync protocol types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request to push local changes to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushRequest {
	/// Client's last known server version
	pub from_version: i64,

	/// Changes to push
	pub changes: Vec<SyncChange>,

	/// Blob hashes that need to be uploaded
	pub pending_blobs: Vec<String>,
}

/// Response from push request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushResponse {
	/// New server version after applying changes
	pub server_version: i64,

	/// Changes that were accepted
	pub accepted: Vec<Uuid>,

	/// Changes that conflicted (client should pull and retry)
	pub conflicts: Vec<SyncConflict>,

	/// Presigned URLs for blob uploads (if any)
	pub blob_upload_urls: Vec<BlobUploadUrl>,
}

/// Request to pull changes from server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
	/// Client's current version
	pub from_version: i64,

	/// Maximum number of changes to return
	pub limit: Option<i64>,
}

/// Response from pull request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullResponse {
	/// Current server version
	pub server_version: i64,

	/// Changes since client's version
	pub changes: Vec<SyncChange>,

	/// Blob hashes that client needs to download
	pub required_blobs: Vec<String>,

	/// Whether there are more changes (pagination)
	pub has_more: bool,
}

/// A single change to be synced
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncChange {
	/// Unique ID of this change
	pub id: Uuid,

	/// Entity type being changed
	pub entity_type: EntityType,

	/// ID of the entity
	pub entity_id: Uuid,

	/// Type of change
	pub action: SyncAction,

	/// Version number of this change
	pub version: i64,

	/// Timestamp of the change
	pub timestamp: DateTime<Utc>,

	/// User who made the change
	pub user_id: Uuid,

	/// Serialized entity data (for create/update)
	pub data: Option<String>,
}

/// Types of entities that can be synced
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
	Project,
	Well,
	Curve,
	CurveMetadata,
}

/// Types of sync actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncAction {
	Create,
	Update,
	Delete,
}

/// A conflict detected during push
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
	/// ID of the conflicting change
	pub change_id: Uuid,

	/// Entity that conflicted
	pub entity_type: EntityType,
	pub entity_id: Uuid,

	/// Server's current version of the entity
	pub server_version: i64,

	/// Client's version that conflicted
	pub client_version: i64,

	/// Server's current data
	pub server_data: Option<String>,
}

/// Presigned URL for blob upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobUploadUrl {
	/// Hash of the blob
	pub hash: String,

	/// Presigned URL for upload
	pub url: String,

	/// Expiration time
	pub expires_at: DateTime<Utc>,
}

/// Presigned URL for blob download
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobDownloadUrl {
	/// Hash of the blob
	pub hash: String,

	/// Presigned URL for download
	pub url: String,

	/// Expiration time
	pub expires_at: DateTime<Utc>,
}

/// Request blob download URLs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobUrlsRequest {
	pub hashes: Vec<String>,
}

/// Response with blob download URLs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobUrlsResponse {
	pub urls: Vec<BlobDownloadUrl>,
}
