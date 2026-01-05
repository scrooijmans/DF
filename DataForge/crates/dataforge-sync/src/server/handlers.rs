//! HTTP handlers for sync endpoints
//!
//! Following patterns from:
//! - Shuttle/Axum: Router composition, State injection
//! - Harbor: Presigned URL generation
//! - Pijul: Push/pull sync protocol

use axum::{
	extract::{Path, State},
	http::StatusCode,
	middleware,
	routing::{get, post},
	Json, Router,
};
use chrono::Utc;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, warn};

use crate::auth::{auth_middleware, ExtractUser};
use crate::protocol::*;

use super::state::AppState;

/// Create the main router with all sync endpoints
pub fn create_router(state: AppState) -> Router {
	let cors = CorsLayer::new()
		.allow_origin(Any)
		.allow_methods(Any)
		.allow_headers(Any);

	// Public routes (no auth required)
	let public_routes = Router::new()
		.route("/health", get(health_check))
		.route("/api/health", get(health_check));

	// Protected routes (auth required)
	let protected_routes = Router::new()
		.route("/api/sync/push", post(push_handler))
		.route("/api/sync/pull", post(pull_handler))
		.route("/api/blobs/urls", post(blob_urls_handler))
		.route("/api/blobs/upload-urls", post(blob_upload_urls_handler))
		.route("/api/blobs/:hash/uploaded", post(blob_uploaded_handler))
		.layer(middleware::from_fn_with_state(
			state.auth_config.clone(),
			auth_middleware,
		));

	Router::new()
		.merge(public_routes)
		.merge(protected_routes)
		.layer(cors)
		.layer(TraceLayer::new_for_http())
		.with_state(state)
}

/// Health check endpoint
async fn health_check() -> &'static str {
	"OK"
}

/// Push changes to server
///
/// Following Pijul's push pattern:
/// 1. Accept batch of changes
/// 2. Check for conflicts (newer versions on server)
/// 3. Store accepted changes with incremented server version
/// 4. Return presigned upload URLs for pending blobs
async fn push_handler(
	State(state): State<AppState>,
	ExtractUser(user): ExtractUser,
	Json(request): Json<PushRequest>,
) -> Result<Json<PushResponse>, StatusCode> {
	let workspace_id = user.workspace_id.as_deref().unwrap_or("default");

	info!(
		"Push request from {}: from_version={}, changes={}, blobs={}",
		user.email,
		request.from_version,
		request.changes.len(),
		request.pending_blobs.len()
	);

	// Push changes to database
	let (accepted, conflicts, server_version) = state
		.db
		.push_changes(workspace_id, &request.changes, request.from_version)
		.map_err(|e| {
			warn!("Push failed: {}", e);
			StatusCode::INTERNAL_SERVER_ERROR
		})?;

	// Generate presigned upload URLs for pending blobs
	let mut blob_upload_urls = Vec::new();
	for hash in &request.pending_blobs {
		match state.storage.presigned_upload_url(hash, Some(Duration::from_secs(3600))).await {
			Ok(presigned) => {
				blob_upload_urls.push(BlobUploadUrl {
					hash: hash.clone(),
					url: presigned.url,
					expires_at: chrono::DateTime::parse_from_rfc3339(&presigned.expires_at)
						.map(|dt| dt.with_timezone(&Utc))
						.unwrap_or_else(|_| Utc::now() + chrono::Duration::hours(1)),
				});
			}
			Err(e) => {
				warn!("Failed to generate upload URL for blob {}: {}", hash, e);
				// Continue with other blobs
			}
		}
	}

	info!(
		"Push response: version={}, accepted={}, conflicts={}, upload_urls={}",
		server_version,
		accepted.len(),
		conflicts.len(),
		blob_upload_urls.len()
	);

	Ok(Json(PushResponse {
		server_version,
		accepted,
		conflicts,
		blob_upload_urls,
	}))
}

/// Pull changes from server
///
/// Following Pijul's pull pattern:
/// 1. Query changes since client's version
/// 2. Return changes with pagination support
/// 3. Include list of required blobs for the changes
async fn pull_handler(
	State(state): State<AppState>,
	ExtractUser(user): ExtractUser,
	Json(request): Json<PullRequest>,
) -> Result<Json<PullResponse>, StatusCode> {
	let workspace_id = user.workspace_id.as_deref().unwrap_or("default");

	info!(
		"Pull request from {}: from_version={}, limit={:?}",
		user.email, request.from_version, request.limit
	);

	// Pull changes from database
	let (changes, server_version, has_more) = state
		.db
		.pull_changes(workspace_id, request.from_version, request.limit)
		.map_err(|e| {
			warn!("Pull failed: {}", e);
			StatusCode::INTERNAL_SERVER_ERROR
		})?;

	// Extract blob hashes from changes
	let required_blobs: Vec<String> = changes
		.iter()
		.filter_map(|c| extract_blob_hashes(c.data.as_deref()))
		.flatten()
		.collect();

	info!(
		"Pull response: version={}, changes={}, blobs={}, has_more={}",
		server_version,
		changes.len(),
		required_blobs.len(),
		has_more
	);

	Ok(Json(PullResponse {
		server_version,
		changes,
		required_blobs,
		has_more,
	}))
}

/// Get presigned download URLs for blobs
///
/// Following Harbor's presigned URL pattern:
/// - Client requests URLs for specific blob hashes
/// - Server validates access and generates presigned URLs
/// - Client downloads directly from S3/MinIO
async fn blob_urls_handler(
	State(state): State<AppState>,
	ExtractUser(user): ExtractUser,
	Json(request): Json<BlobUrlsRequest>,
) -> Result<Json<BlobUrlsResponse>, StatusCode> {
	info!(
		"Blob download URLs request from {}: {} hashes",
		user.email,
		request.hashes.len()
	);

	let mut urls = Vec::new();
	for hash in &request.hashes {
		match state.storage.presigned_download_url(hash, Some(Duration::from_secs(3600))).await {
			Ok(presigned) => {
				urls.push(BlobDownloadUrl {
					hash: hash.clone(),
					url: presigned.url,
					expires_at: chrono::DateTime::parse_from_rfc3339(&presigned.expires_at)
						.map(|dt| dt.with_timezone(&Utc))
						.unwrap_or_else(|_| Utc::now() + chrono::Duration::hours(1)),
				});
			}
			Err(e) => {
				warn!("Failed to generate download URL for blob {}: {}", hash, e);
				// Skip missing blobs
			}
		}
	}

	Ok(Json(BlobUrlsResponse { urls }))
}

/// Get presigned upload URLs for blobs
///
/// Separate endpoint for requesting upload URLs without pushing changes
async fn blob_upload_urls_handler(
	State(state): State<AppState>,
	ExtractUser(user): ExtractUser,
	Json(request): Json<BlobUrlsRequest>,
) -> Result<Json<BlobUploadUrlsResponse>, StatusCode> {
	info!(
		"Blob upload URLs request from {}: {} hashes",
		user.email,
		request.hashes.len()
	);

	let mut urls = Vec::new();
	for hash in &request.hashes {
		match state.storage.presigned_upload_url(hash, Some(Duration::from_secs(3600))).await {
			Ok(presigned) => {
				urls.push(BlobUploadUrl {
					hash: hash.clone(),
					url: presigned.url,
					expires_at: chrono::DateTime::parse_from_rfc3339(&presigned.expires_at)
						.map(|dt| dt.with_timezone(&Utc))
						.unwrap_or_else(|_| Utc::now() + chrono::Duration::hours(1)),
				});
			}
			Err(e) => {
				warn!("Failed to generate upload URL for blob {}: {}", hash, e);
			}
		}
	}

	Ok(Json(BlobUploadUrlsResponse { urls }))
}

/// Confirm a blob has been uploaded
///
/// Called after client uploads blob to presigned URL
async fn blob_uploaded_handler(
	State(state): State<AppState>,
	ExtractUser(user): ExtractUser,
	Path(hash): Path<String>,
) -> Result<StatusCode, StatusCode> {
	let workspace_id = user.workspace_id.as_deref().unwrap_or("default");

	info!("Blob upload confirmed from {}: {}", user.email, hash);

	// Verify blob exists in storage
	let exists = state
		.storage
		.blob_exists(&hash)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	if !exists {
		warn!("Blob {} not found in storage", hash);
		return Err(StatusCode::NOT_FOUND);
	}

	// Get blob metadata for size
	let size = state
		.storage
		.blob_metadata(&hash)
		.await
		.map(|m| m.size as i64)
		.ok();

	// Register in database
	state
		.db
		.register_blob(&hash, workspace_id, size, &user.account_id)
		.map_err(|e| {
			warn!("Failed to register blob: {}", e);
			StatusCode::INTERNAL_SERVER_ERROR
		})?;

	Ok(StatusCode::OK)
}

/// Response for blob upload URLs request
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BlobUploadUrlsResponse {
	pub urls: Vec<BlobUploadUrl>,
}

/// Extract blob hashes from change data
fn extract_blob_hashes(data: Option<&str>) -> Option<Vec<String>> {
	let data = data?;

	// Parse JSON and look for blob hash fields
	let value: serde_json::Value = serde_json::from_str(data).ok()?;

	let mut hashes = Vec::new();

	// Check common blob hash field names
	for field in ["blob_hash", "native_parquet_hash", "gridded_parquet_hash", "parquet_hash"] {
		if let Some(hash) = value.get(field).and_then(|v| v.as_str()) {
			if !hash.is_empty() {
				hashes.push(hash.to_string());
			}
		}
	}

	if hashes.is_empty() {
		None
	} else {
		Some(hashes)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_extract_blob_hashes() {
		let data = r#"{"name": "test", "blob_hash": "abc123"}"#;
		let hashes = extract_blob_hashes(Some(data)).unwrap();
		assert_eq!(hashes, vec!["abc123"]);

		let data2 = r#"{"name": "test", "native_parquet_hash": "def456", "gridded_parquet_hash": "ghi789"}"#;
		let hashes2 = extract_blob_hashes(Some(data2)).unwrap();
		assert!(hashes2.contains(&"def456".to_string()));
		assert!(hashes2.contains(&"ghi789".to_string()));

		let data3 = r#"{"name": "no blob"}"#;
		assert!(extract_blob_hashes(Some(data3)).is_none());

		assert!(extract_blob_hashes(None).is_none());
	}
}
