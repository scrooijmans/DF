//! Sync client for communicating with DataForge server

use reqwest::Client;
use tracing::{debug, info, warn};

use crate::protocol::*;

/// Error type for sync operations
#[derive(Debug, thiserror::Error)]
pub enum SyncError {
	#[error("Network error: {0}")]
	Network(#[from] reqwest::Error),

	#[error("Server error: {status} - {message}")]
	Server { status: u16, message: String },

	#[error("Authentication required")]
	Unauthorized,

	#[error("Conflict detected: {0} conflicts")]
	Conflict(usize),

	#[error("Serialization error: {0}")]
	Serialization(#[from] serde_json::Error),
}

/// Client for syncing with DataForge server
pub struct SyncClient {
	client: Client,
	base_url: String,
	auth_token: Option<String>,
}

impl SyncClient {
	/// Create a new sync client
	pub fn new(base_url: &str) -> Self {
		Self {
			client: Client::new(),
			base_url: base_url.trim_end_matches('/').to_string(),
			auth_token: None,
		}
	}

	/// Set authentication token
	pub fn with_auth(mut self, token: &str) -> Self {
		self.auth_token = Some(token.to_string());
		self
	}

	/// Push local changes to server
	pub async fn push(&self, request: PushRequest) -> Result<PushResponse, SyncError> {
		info!(
			from_version = request.from_version,
			changes = request.changes.len(),
			blobs = request.pending_blobs.len(),
			"Pushing changes to server"
		);

		let url = format!("{}/api/sync/push", self.base_url);

		let mut req = self.client.post(&url).json(&request);

		if let Some(token) = &self.auth_token {
			req = req.bearer_auth(token);
		}

		let response = req.send().await?;

		if response.status() == reqwest::StatusCode::UNAUTHORIZED {
			return Err(SyncError::Unauthorized);
		}

		if !response.status().is_success() {
			let status = response.status().as_u16();
			let message = response.text().await.unwrap_or_default();
			return Err(SyncError::Server { status, message });
		}

		let push_response: PushResponse = response.json().await?;

		if !push_response.conflicts.is_empty() {
			warn!(conflicts = push_response.conflicts.len(), "Push had conflicts");
			return Err(SyncError::Conflict(push_response.conflicts.len()));
		}

		info!(
			server_version = push_response.server_version,
			accepted = push_response.accepted.len(),
			"Push completed"
		);

		Ok(push_response)
	}

	/// Pull changes from server
	pub async fn pull(&self, request: PullRequest) -> Result<PullResponse, SyncError> {
		info!(
			from_version = request.from_version,
			"Pulling changes from server"
		);

		let url = format!("{}/api/sync/pull", self.base_url);

		let mut req = self.client.post(&url).json(&request);

		if let Some(token) = &self.auth_token {
			req = req.bearer_auth(token);
		}

		let response = req.send().await?;

		if response.status() == reqwest::StatusCode::UNAUTHORIZED {
			return Err(SyncError::Unauthorized);
		}

		if !response.status().is_success() {
			let status = response.status().as_u16();
			let message = response.text().await.unwrap_or_default();
			return Err(SyncError::Server { status, message });
		}

		let pull_response: PullResponse = response.json().await?;

		info!(
			server_version = pull_response.server_version,
			changes = pull_response.changes.len(),
			blobs = pull_response.required_blobs.len(),
			"Pull completed"
		);

		Ok(pull_response)
	}

	/// Get presigned URLs for blob downloads
	pub async fn get_blob_urls(
		&self,
		hashes: Vec<String>,
	) -> Result<BlobUrlsResponse, SyncError> {
		debug!(count = hashes.len(), "Requesting blob download URLs");

		let url = format!("{}/api/blobs/urls", self.base_url);
		let request = BlobUrlsRequest { hashes };

		let mut req = self.client.post(&url).json(&request);

		if let Some(token) = &self.auth_token {
			req = req.bearer_auth(token);
		}

		let response = req.send().await?;

		if response.status() == reqwest::StatusCode::UNAUTHORIZED {
			return Err(SyncError::Unauthorized);
		}

		if !response.status().is_success() {
			let status = response.status().as_u16();
			let message = response.text().await.unwrap_or_default();
			return Err(SyncError::Server { status, message });
		}

		Ok(response.json().await?)
	}

	/// Download a blob by hash
	pub async fn download_blob(&self, url: &str) -> Result<Vec<u8>, SyncError> {
		debug!(url = url, "Downloading blob");

		let response = self.client.get(url).send().await?;

		if !response.status().is_success() {
			let status = response.status().as_u16();
			let message = response.text().await.unwrap_or_default();
			return Err(SyncError::Server { status, message });
		}

		Ok(response.bytes().await?.to_vec())
	}

	/// Upload a blob
	pub async fn upload_blob(&self, url: &str, data: Vec<u8>) -> Result<(), SyncError> {
		debug!(url = url, size = data.len(), "Uploading blob");

		let response = self
			.client
			.put(url)
			.body(data)
			.header("Content-Type", "application/octet-stream")
			.send()
			.await?;

		if !response.status().is_success() {
			let status = response.status().as_u16();
			let message = response.text().await.unwrap_or_default();
			return Err(SyncError::Server { status, message });
		}

		Ok(())
	}

	/// Check server connectivity
	pub async fn health_check(&self) -> Result<bool, SyncError> {
		let url = format!("{}/api/health", self.base_url);

		let response = self.client.get(&url).send().await?;

		Ok(response.status().is_success())
	}
}
