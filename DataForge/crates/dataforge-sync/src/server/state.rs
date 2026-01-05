//! Application state for the sync server
//!
//! Following Shuttle/Axum patterns for state management:
//! - Arc-wrapped for thread safety
//! - Contains all services needed by handlers
//! - Injected via Axum's State extractor

use std::sync::Arc;

use crate::auth::AuthConfig;
use crate::storage::BlobStorage;
use super::db::ServerDb;

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
	/// Database for sync change persistence
	pub db: Arc<ServerDb>,
	/// Blob storage for presigned URLs
	pub storage: Arc<BlobStorage>,
	/// Authentication configuration
	pub auth_config: Arc<AuthConfig>,
}

impl AppState {
	/// Create new application state
	pub fn new(db: ServerDb, storage: BlobStorage, auth_config: AuthConfig) -> Self {
		Self {
			db: Arc::new(db),
			storage: Arc::new(storage),
			auth_config: Arc::new(auth_config),
		}
	}
}
