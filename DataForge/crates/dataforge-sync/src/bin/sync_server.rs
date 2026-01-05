//! DataForge Production Sync Server
//!
//! A production-ready sync server with:
//! - SQLite database for change persistence
//! - JWT authentication
//! - S3/MinIO blob storage with presigned URLs
//!
//! ## Configuration
//!
//! All configuration is via environment variables:
//!
//! ### Server
//! - `SYNC_SERVER_PORT` - Server port (default: 3000)
//! - `SYNC_SERVER_DB_PATH` - SQLite database path (default: ./sync.db)
//!
//! ### Authentication
//! - `SYNC_AUTH_SECRET` - JWT signing secret (required in production)
//! - `SYNC_AUTH_SKIP` - Set to "true" to skip auth (development only!)
//! - `SYNC_AUTH_EXPIRY` - Token expiry in seconds (default: 3600)
//!
//! ### Storage
//! - `STORAGE_BACKEND` - Backend type: "s3", "minio", "filesystem" (default: filesystem)
//! - `STORAGE_BUCKET` - S3 bucket name or filesystem root path
//! - `STORAGE_REGION` - S3 region (default: us-east-1)
//! - `STORAGE_ENDPOINT` - S3 endpoint URL (for MinIO)
//! - `STORAGE_ACCESS_KEY` - S3 access key ID
//! - `STORAGE_SECRET_KEY` - S3 secret access key
//!
//! ## Running
//!
//! ```bash
//! # Development (skip auth, filesystem storage)
//! SYNC_AUTH_SKIP=true cargo run --bin dataforge-sync-server
//!
//! # Production (with MinIO)
//! SYNC_AUTH_SECRET=your-secret \
//! STORAGE_BACKEND=minio \
//! STORAGE_ENDPOINT=http://minio:9000 \
//! STORAGE_BUCKET=dataforge-blobs \
//! STORAGE_ACCESS_KEY=minioadmin \
//! STORAGE_SECRET_KEY=minioadmin \
//! cargo run --bin dataforge-sync-server --release
//! ```

use std::{env, path::PathBuf};
use tracing::{info, Level};

use dataforge_sync::{
	auth::AuthConfig,
	server::{create_router, AppState, ServerDb},
	storage::{BlobStorage, StorageBackend, StorageConfig},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// Initialize tracing
	tracing_subscriber::fmt()
		.with_max_level(Level::INFO)
		.with_target(false)
		.init();

	info!("Starting DataForge Sync Server");

	// Load configuration from environment
	let port = env::var("SYNC_SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
	let db_path = env::var("SYNC_SERVER_DB_PATH").unwrap_or_else(|_| "./sync.db".to_string());

	// Auth configuration
	let auth_config = load_auth_config();

	// Storage configuration
	let storage_config = load_storage_config()?;

	// Initialize database
	info!("Initializing database at {}", db_path);
	let db = ServerDb::open(&PathBuf::from(&db_path))?;

	// Initialize blob storage
	info!("Initializing {:?} storage backend", storage_config.backend);
	let storage = BlobStorage::new(storage_config).await?;

	// Create application state
	let state = AppState::new(db, storage, auth_config);

	// Create router
	let app = create_router(state);

	// Start server
	let addr = format!("0.0.0.0:{}", port);
	info!("Listening on http://{}", addr);
	info!("");
	info!("Endpoints:");
	info!("  GET  /api/health              - Health check");
	info!("  POST /api/sync/push           - Push changes");
	info!("  POST /api/sync/pull           - Pull changes");
	info!("  POST /api/blobs/urls          - Get blob download URLs");
	info!("  POST /api/blobs/upload-urls   - Get blob upload URLs");
	info!("  POST /api/blobs/:hash/uploaded - Confirm blob upload");
	info!("");
	info!("Press Ctrl+C to stop");

	let listener = tokio::net::TcpListener::bind(&addr).await?;
	axum::serve(listener, app).await?;

	Ok(())
}

/// Load authentication configuration from environment
fn load_auth_config() -> AuthConfig {
	let skip_auth = env::var("SYNC_AUTH_SKIP")
		.map(|v| v.to_lowercase() == "true")
		.unwrap_or(false);

	let secret_key = env::var("SYNC_AUTH_SECRET").unwrap_or_else(|_| {
		if skip_auth {
			"dev-secret-key".to_string()
		} else {
			panic!("SYNC_AUTH_SECRET is required in production. Set SYNC_AUTH_SKIP=true for development.")
		}
	});

	let expiry_seconds = env::var("SYNC_AUTH_EXPIRY")
		.ok()
		.and_then(|v| v.parse().ok())
		.unwrap_or(3600);

	let mut config = AuthConfig::new(secret_key).with_expiry(expiry_seconds);

	if skip_auth {
		info!("WARNING: Authentication disabled (SYNC_AUTH_SKIP=true)");
		config = config.skip_auth();
	}

	config
}

/// Load storage configuration from environment
fn load_storage_config() -> anyhow::Result<StorageConfig> {
	let backend_str = env::var("STORAGE_BACKEND").unwrap_or_else(|_| "filesystem".to_string());

	let backend = match backend_str.to_lowercase().as_str() {
		"s3" => StorageBackend::S3,
		"minio" => StorageBackend::Minio,
		"filesystem" | "fs" => StorageBackend::Filesystem,
		other => {
			anyhow::bail!("Unknown storage backend: {}. Use 's3', 'minio', or 'filesystem'", other);
		}
	};

	let bucket = env::var("STORAGE_BUCKET").unwrap_or_else(|_| "./blob-storage".to_string());
	let region = env::var("STORAGE_REGION").unwrap_or_else(|_| "us-east-1".to_string());
	let endpoint = env::var("STORAGE_ENDPOINT").unwrap_or_default();
	let access_key_id = env::var("STORAGE_ACCESS_KEY").unwrap_or_default();
	let secret_access_key = env::var("STORAGE_SECRET_KEY").unwrap_or_default();

	// Validate S3/MinIO config
	if matches!(backend, StorageBackend::S3 | StorageBackend::Minio) {
		if access_key_id.is_empty() || secret_access_key.is_empty() {
			anyhow::bail!("STORAGE_ACCESS_KEY and STORAGE_SECRET_KEY are required for S3/MinIO");
		}
		if backend == StorageBackend::Minio && endpoint.is_empty() {
			anyhow::bail!("STORAGE_ENDPOINT is required for MinIO");
		}
	}

	Ok(StorageConfig {
		backend,
		region,
		endpoint,
		bucket,
		access_key_id,
		secret_access_key,
		prefix: "blobs/".to_string(),
		presign_expiry_seconds: 3600,
	})
}
