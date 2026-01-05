//! DataForge Test Sync Server
//!
//! A simple test server for local development and testing.
//! Run with: `cargo run --bin dataforge-test-server`
//!
//! Default: http://localhost:3000
//!
//! Endpoints:
//! - GET  /api/health - Health check
//! - POST /api/sync/push - Push changes (mock)
//! - POST /api/sync/pull - Pull changes (mock)
//! - POST /api/blobs/urls - Get blob URLs (mock)

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use dataforge_sync::protocol::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock},
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use uuid::Uuid;

/// Server state (in-memory for testing)
#[derive(Debug, Default)]
struct ServerState {
    /// Current server version
    version: i64,
    /// Stored changes (version -> changes)
    changes: Vec<SyncChange>,
    /// Stored blobs (hash -> data)
    blobs: HashMap<String, Vec<u8>>,
}

type SharedState = Arc<RwLock<ServerState>>;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .init();

    let state: SharedState = Arc::new(RwLock::new(ServerState::default()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/health", get(health_check))
        .route("/api/sync/push", post(push_handler))
        .route("/api/sync/pull", post(pull_handler))
        .route("/api/blobs/urls", post(blob_urls_handler))
        .route("/api/blobs/:hash", get(blob_download_handler))
        .route("/api/blobs/:hash", axum::routing::put(blob_upload_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let port = env::var("SYNC_SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    info!("Starting DataForge sync server on http://{}", addr);
    info!("Press Ctrl+C to stop");
    info!("");
    info!("Endpoints:");
    info!("  GET  /api/health       - Health check");
    info!("  POST /api/sync/push    - Push changes");
    info!("  POST /api/sync/pull    - Pull changes");
    info!("  POST /api/blobs/urls   - Get blob URLs");
    info!("  GET  /api/blobs/:hash  - Download blob");
    info!("  PUT  /api/blobs/:hash  - Upload blob");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

/// Push changes to server
async fn push_handler(
    State(state): State<SharedState>,
    Json(request): Json<PushRequest>,
) -> Result<Json<PushResponse>, StatusCode> {
    info!(
        "Push request: from_version={}, changes={}, blobs={}",
        request.from_version,
        request.changes.len(),
        request.pending_blobs.len()
    );

    let mut state = state.write().unwrap();

    // Check for conflicts (simple version check)
    let mut conflicts = Vec::new();
    let mut accepted = Vec::new();

    for change in &request.changes {
        // Check if there's a newer version of this entity
        let has_conflict = state.changes.iter().any(|c| {
            c.entity_type == change.entity_type
                && c.entity_id == change.entity_id
                && c.version > change.version
        });

        if has_conflict {
            // Find the server's version
            let server_change = state
                .changes
                .iter()
                .filter(|c| c.entity_type == change.entity_type && c.entity_id == change.entity_id)
                .max_by_key(|c| c.version);

            if let Some(sc) = server_change {
                conflicts.push(SyncConflict {
                    change_id: change.id,
                    entity_type: change.entity_type,
                    entity_id: change.entity_id,
                    server_version: sc.version,
                    client_version: change.version,
                    server_data: sc.data.clone(),
                });
            }
        } else {
            accepted.push(change.id);
            state.changes.push(change.clone());
        }
    }

    // Increment server version
    state.version += 1;

    // Generate upload URLs for pending blobs
    let blob_upload_urls: Vec<BlobUploadUrl> = request
        .pending_blobs
        .iter()
        .map(|hash| BlobUploadUrl {
            hash: hash.clone(),
            url: format!("http://localhost:3000/api/blobs/{}", hash),
            expires_at: Utc::now() + chrono::Duration::hours(1),
        })
        .collect();

    info!(
        "Push response: version={}, accepted={}, conflicts={}",
        state.version,
        accepted.len(),
        conflicts.len()
    );

    Ok(Json(PushResponse {
        server_version: state.version,
        accepted,
        conflicts,
        blob_upload_urls,
    }))
}

/// Pull changes from server
async fn pull_handler(
    State(state): State<SharedState>,
    Json(request): Json<PullRequest>,
) -> Result<Json<PullResponse>, StatusCode> {
    info!(
        "Pull request: from_version={}, limit={:?}",
        request.from_version, request.limit
    );

    let state = state.read().unwrap();

    // Get changes since client's version
    let changes: Vec<SyncChange> = state
        .changes
        .iter()
        .filter(|c| c.version > request.from_version)
        .take(request.limit.unwrap_or(100) as usize)
        .cloned()
        .collect();

    // Collect required blobs
    let required_blobs: Vec<String> = changes
        .iter()
        .filter_map(|c| {
            // Parse data to find blob references (simplified)
            c.data.as_ref().and_then(|d| {
                if d.contains("blob_hash") {
                    // Extract blob hash from JSON (simplified)
                    serde_json::from_str::<serde_json::Value>(d)
                        .ok()
                        .and_then(|v| v.get("blob_hash").and_then(|h| h.as_str().map(String::from)))
                } else {
                    None
                }
            })
        })
        .collect();

    let has_more = state.changes.len() > changes.len();

    info!(
        "Pull response: version={}, changes={}, blobs={}",
        state.version,
        changes.len(),
        required_blobs.len()
    );

    Ok(Json(PullResponse {
        server_version: state.version,
        changes,
        required_blobs,
        has_more,
    }))
}

/// Get blob download URLs
async fn blob_urls_handler(
    Json(request): Json<BlobUrlsRequest>,
) -> Result<Json<BlobUrlsResponse>, StatusCode> {
    info!("Blob URLs request: {} hashes", request.hashes.len());

    let urls: Vec<BlobDownloadUrl> = request
        .hashes
        .iter()
        .map(|hash| BlobDownloadUrl {
            hash: hash.clone(),
            url: format!("http://localhost:3000/api/blobs/{}", hash),
            expires_at: Utc::now() + chrono::Duration::hours(1),
        })
        .collect();

    Ok(Json(BlobUrlsResponse { urls }))
}

/// Download a blob
async fn blob_download_handler(
    State(state): State<SharedState>,
    axum::extract::Path(hash): axum::extract::Path<String>,
) -> Result<Vec<u8>, StatusCode> {
    let state = state.read().unwrap();

    state
        .blobs
        .get(&hash)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)
}

/// Upload a blob
async fn blob_upload_handler(
    State(state): State<SharedState>,
    axum::extract::Path(hash): axum::extract::Path<String>,
    body: axum::body::Bytes,
) -> Result<StatusCode, StatusCode> {
    info!("Blob upload: hash={}, size={}", hash, body.len());

    let mut state = state.write().unwrap();
    state.blobs.insert(hash, body.to_vec());

    Ok(StatusCode::OK)
}
