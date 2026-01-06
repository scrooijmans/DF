//! Sync client for communicating with DataForge server

use reqwest::Client;
use std::time::Duration;
use tracing::{debug, info, warn};

use crate::protocol::*;

/// Retry configuration following PowerSync patterns
pub struct RetryConfig {
	/// Maximum number of retry attempts
	pub max_attempts: usize,
	/// Initial delay between retries
	pub initial_delay: Duration,
	/// Maximum delay between retries
	pub max_delay: Duration,
	/// Multiplier for exponential backoff
	pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
	fn default() -> Self {
		Self {
			max_attempts: 5,
			initial_delay: Duration::from_secs(1),
			max_delay: Duration::from_secs(60),
			backoff_multiplier: 2.0,
		}
	}
}

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

	#[error("Max retries exceeded after {attempts} attempts: {last_error}")]
	MaxRetriesExceeded { attempts: usize, last_error: String },
}

impl SyncError {
	/// Check if this error is transient and should be retried
	///
	/// Following PowerSync patterns:
	/// - Network errors are transient
	/// - 5xx server errors are transient
	/// - 429 (rate limit) is transient
	/// - Auth errors and conflicts are fatal (don't retry)
	pub fn is_transient(&self) -> bool {
		match self {
			// Network issues are always transient
			SyncError::Network(_) => true,

			// Server errors - 5xx and 429 are transient
			SyncError::Server { status, .. } => {
				*status >= 500 || *status == 429
			}

			// These are fatal - don't retry
			SyncError::Unauthorized => false,
			SyncError::Conflict(_) => false,
			SyncError::Serialization(_) => false,
			SyncError::MaxRetriesExceeded { .. } => false,
		}
	}
}

/// Client for syncing with DataForge server
pub struct SyncClient {
	client: Client,
	base_url: String,
	auth_token: Option<String>,
	retry_config: RetryConfig,
}

impl SyncClient {
	/// Create a new sync client
	pub fn new(base_url: &str) -> Self {
		Self {
			client: Client::new(),
			base_url: base_url.trim_end_matches('/').to_string(),
			auth_token: None,
			retry_config: RetryConfig::default(),
		}
	}

	/// Set authentication token
	pub fn with_auth(mut self, token: &str) -> Self {
		self.auth_token = Some(token.to_string());
		self
	}

	/// Set custom retry configuration
	pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
		self.retry_config = config;
		self
	}

	/// Calculate delay for a given attempt (exponential backoff)
	fn calculate_delay(&self, attempt: usize) -> Duration {
		let delay_secs = self.retry_config.initial_delay.as_secs_f64()
			* self.retry_config.backoff_multiplier.powi(attempt as i32);
		let capped_delay = Duration::from_secs_f64(delay_secs).min(self.retry_config.max_delay);
		capped_delay
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

	// ============================================================
	// RETRY-ENABLED METHODS (PowerSync pattern)
	// ============================================================

	/// Push with automatic retry for transient errors
	///
	/// Uses exponential backoff. Fatal errors (auth, conflicts) fail immediately.
	pub async fn push_with_retry(&self, request: PushRequest) -> Result<PushResponse, SyncError> {
		let mut last_error = String::new();

		for attempt in 0..self.retry_config.max_attempts {
			match self.push(request.clone()).await {
				Ok(response) => return Ok(response),
				Err(e) => {
					if !e.is_transient() {
						// Fatal error - don't retry
						return Err(e);
					}

					last_error = e.to_string();
					let delay = self.calculate_delay(attempt);

					warn!(
						attempt = attempt + 1,
						max_attempts = self.retry_config.max_attempts,
						delay_secs = delay.as_secs_f64(),
						error = %e,
						"Push failed with transient error, retrying"
					);

					tokio::time::sleep(delay).await;
				}
			}
		}

		Err(SyncError::MaxRetriesExceeded {
			attempts: self.retry_config.max_attempts,
			last_error,
		})
	}

	/// Pull with automatic retry for transient errors
	///
	/// Uses exponential backoff. Fatal errors (auth) fail immediately.
	pub async fn pull_with_retry(&self, request: PullRequest) -> Result<PullResponse, SyncError> {
		let mut last_error = String::new();

		for attempt in 0..self.retry_config.max_attempts {
			match self.pull(request.clone()).await {
				Ok(response) => return Ok(response),
				Err(e) => {
					if !e.is_transient() {
						// Fatal error - don't retry
						return Err(e);
					}

					last_error = e.to_string();
					let delay = self.calculate_delay(attempt);

					warn!(
						attempt = attempt + 1,
						max_attempts = self.retry_config.max_attempts,
						delay_secs = delay.as_secs_f64(),
						error = %e,
						"Pull failed with transient error, retrying"
					);

					tokio::time::sleep(delay).await;
				}
			}
		}

		Err(SyncError::MaxRetriesExceeded {
			attempts: self.retry_config.max_attempts,
			last_error,
		})
	}

	/// Download blob with automatic retry
	pub async fn download_blob_with_retry(&self, url: &str) -> Result<Vec<u8>, SyncError> {
		let mut last_error = String::new();

		for attempt in 0..self.retry_config.max_attempts {
			match self.download_blob(url).await {
				Ok(data) => return Ok(data),
				Err(e) => {
					if !e.is_transient() {
						return Err(e);
					}

					last_error = e.to_string();
					let delay = self.calculate_delay(attempt);

					warn!(
						attempt = attempt + 1,
						url = url,
						delay_secs = delay.as_secs_f64(),
						error = %e,
						"Blob download failed, retrying"
					);

					tokio::time::sleep(delay).await;
				}
			}
		}

		Err(SyncError::MaxRetriesExceeded {
			attempts: self.retry_config.max_attempts,
			last_error,
		})
	}

	/// Upload blob with automatic retry
	pub async fn upload_blob_with_retry(&self, url: &str, data: Vec<u8>) -> Result<(), SyncError> {
		let mut last_error = String::new();

		for attempt in 0..self.retry_config.max_attempts {
			match self.upload_blob(url, data.clone()).await {
				Ok(()) => return Ok(()),
				Err(e) => {
					if !e.is_transient() {
						return Err(e);
					}

					last_error = e.to_string();
					let delay = self.calculate_delay(attempt);

					warn!(
						attempt = attempt + 1,
						url = url,
						delay_secs = delay.as_secs_f64(),
						error = %e,
						"Blob upload failed, retrying"
					);

					tokio::time::sleep(delay).await;
				}
			}
		}

		Err(SyncError::MaxRetriesExceeded {
			attempts: self.retry_config.max_attempts,
			last_error,
		})
	}
}
