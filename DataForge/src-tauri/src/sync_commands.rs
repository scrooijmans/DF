//! Sync-related Tauri commands
//!
//! Provides commands for the frontend to interact with sync functionality:
//! - Get sync state for a workspace
//! - Trigger manual sync
//! - Configure sync server
//! - Manage conflicts
//!
//! ## Change Application (ColaNode-inspired)
//!
//! When pulling changes from server, we follow these patterns:
//! 1. Download required blobs first (content-addressed, deduplicated)
//! 2. Apply changes transactionally with version-based conflict detection
//! 3. Queue conflicts for user resolution rather than auto-merging
//! 4. Update sync_state.last_sync_version only after successful apply

use crate::state::AppState;
use chrono::{DateTime, Utc};
use dataforge_core::blob::BlobStore;
use dataforge_core::models::{ConflictResolution, ConflictStrategy, SyncConflict, SyncStatus, SyncSummary};
use dataforge_core::sync::{
    ConflictRepository, SqliteConflictRepo, SqliteSyncQueueRepo, SqliteSyncStateRepo,
    SyncQueueRepository, SyncState, SyncStateRepository,
};
use dataforge_sync::protocol::{EntityType, PullResponse, SyncAction, SyncChange};
use dataforge_sync::SyncClient;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;
use tauri::State;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

// ============================================================
// RESPONSE TYPES
// ============================================================

/// Sync state response for frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncStateResponse {
    pub workspace_id: String,
    pub server_url: String,
    pub status: String,
    pub last_sync_at: Option<String>,
    pub pending_changes: i64,
    pub conflicts: i64,
    pub error: Option<String>,
}

impl From<SyncSummary> for SyncStateResponse {
    fn from(summary: SyncSummary) -> Self {
        Self {
            workspace_id: String::new(), // Will be set by caller
            server_url: String::new(),   // Will be set by caller
            status: summary.status.to_string(),
            last_sync_at: summary.last_sync_at.map(|dt| dt.to_rfc3339()),
            pending_changes: summary.pending_changes,
            conflicts: summary.conflicts,
            error: summary.error,
        }
    }
}

/// Sync conflict response for frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncConflictResponse {
    pub id: i64,
    pub workspace_id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub local_version: i64,
    pub local_data: String,
    pub remote_version: i64,
    pub remote_data: String,
    pub created_at: String,
}

impl From<SyncConflict> for SyncConflictResponse {
    fn from(conflict: SyncConflict) -> Self {
        Self {
            id: conflict.id,
            workspace_id: conflict.workspace_id.to_string(),
            entity_type: conflict.entity_type,
            entity_id: conflict.entity_id.to_string(),
            local_version: conflict.local_version,
            local_data: conflict.local_data,
            remote_version: conflict.remote_version,
            remote_data: conflict.remote_data,
            created_at: conflict.created_at.to_rfc3339(),
        }
    }
}

/// Sync result response
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResultResponse {
    pub success: bool,
    pub pending_changes: i64,
    pub conflicts: i64,
    pub error: Option<String>,
}

// ============================================================
// SYNC STATE COMMANDS
// ============================================================

/// Get sync state for a workspace
#[tauri::command]
pub fn get_sync_state(
    workspace_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<SyncStateResponse, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let workspace_uuid =
        Uuid::parse_str(&workspace_id).map_err(|_| "Invalid workspace ID".to_string())?;

    let state_repo = SqliteSyncStateRepo::new(db);
    let queue_repo = SqliteSyncQueueRepo::new(db);
    let conflict_repo = SqliteConflictRepo::new(db);

    // Get sync state
    let sync_state = state_repo
        .get_state(&workspace_uuid)
        .map_err(|e| e.to_string())?;

    // Get counts
    let pending_count = queue_repo
        .count_pending(&workspace_uuid)
        .map_err(|e| e.to_string())?;

    let conflict_count = conflict_repo
        .count_pending(&workspace_uuid)
        .map_err(|e| e.to_string())?;

    match sync_state {
        Some(ss) => Ok(SyncStateResponse {
            workspace_id: ss.workspace_id.to_string(),
            server_url: ss.server_url,
            status: ss.sync_status.to_string(),
            last_sync_at: ss.last_sync_at.map(|dt| dt.to_rfc3339()),
            pending_changes: pending_count,
            conflicts: conflict_count,
            error: ss.last_error,
        }),
        None => {
            // No sync state yet - return default
            Ok(SyncStateResponse {
                workspace_id,
                server_url: String::new(),
                status: "idle".to_string(),
                last_sync_at: None,
                pending_changes: pending_count,
                conflicts: conflict_count,
                error: None,
            })
        }
    }
}

/// Initialize sync for a workspace with a server URL
#[tauri::command]
pub fn init_sync(
    workspace_id: String,
    server_url: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<SyncStateResponse, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let workspace_uuid =
        Uuid::parse_str(&workspace_id).map_err(|_| "Invalid workspace ID".to_string())?;

    let state_repo = SqliteSyncStateRepo::new(db);

    // Create new sync state
    let sync_state = SyncState::new(workspace_uuid, server_url.clone());
    state_repo.save_state(&sync_state).map_err(|e| e.to_string())?;

    info!("Initialized sync for workspace {} with server {}", workspace_id, server_url);

    Ok(SyncStateResponse {
        workspace_id,
        server_url,
        status: "idle".to_string(),
        last_sync_at: None,
        pending_changes: 0,
        conflicts: 0,
        error: None,
    })
}

/// Update sync server URL for a workspace
#[tauri::command]
pub fn set_sync_server(
    workspace_id: String,
    server_url: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let workspace_uuid =
        Uuid::parse_str(&workspace_id).map_err(|_| "Invalid workspace ID".to_string())?;

    let state_repo = SqliteSyncStateRepo::new(db);

    // Get existing state or create new
    let sync_state = match state_repo.get_state(&workspace_uuid).map_err(|e| e.to_string())? {
        Some(mut ss) => {
            ss.server_url = server_url.clone();
            ss
        }
        None => SyncState::new(workspace_uuid, server_url.clone()),
    };

    state_repo.save_state(&sync_state).map_err(|e| e.to_string())?;

    info!("Updated sync server for workspace {} to {}", workspace_id, server_url);

    Ok(())
}

/// Set the sync server authentication token
///
/// This token is used to authenticate with the remote sync server.
/// For development servers with skip_auth=true, this is not required.
#[tauri::command]
pub fn set_sync_auth_token(
    token: Option<String>,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| format!("Lock error: {}", e))?;

    state.session.sync_auth_token = token.clone();

    // Save session to persist the token
    state.save_session().map_err(|e| e.to_string())?;

    if token.is_some() {
        info!("Sync auth token set");
    } else {
        info!("Sync auth token cleared");
    }

    Ok(())
}

/// Get whether a sync auth token is configured
#[tauri::command]
pub fn has_sync_auth_token(
    state: State<'_, Mutex<AppState>>,
) -> Result<bool, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    Ok(state.session.sync_auth_token.is_some())
}

// ============================================================
// SYNC OPERATIONS
// ============================================================

/// Test connection to sync server
#[tauri::command]
pub async fn test_sync_connection(
    server_url: String,
) -> Result<TestConnectionResponse, String> {
    info!("Testing connection to sync server: {}", server_url);

    let client = SyncClient::new(&server_url);

    match client.health_check().await {
        Ok(healthy) => {
            if healthy {
                info!("Sync server connection successful");
                Ok(TestConnectionResponse {
                    success: true,
                    message: "Connection successful".to_string(),
                    server_version: None,
                })
            } else {
                warn!("Sync server returned unhealthy status");
                Ok(TestConnectionResponse {
                    success: false,
                    message: "Server returned unhealthy status".to_string(),
                    server_version: None,
                })
            }
        }
        Err(e) => {
            error!("Failed to connect to sync server: {}", e);
            Ok(TestConnectionResponse {
                success: false,
                message: format!("Connection failed: {}", e),
                server_version: None,
            })
        }
    }
}

/// Response for test connection
#[derive(Debug, Serialize, Deserialize)]
pub struct TestConnectionResponse {
    pub success: bool,
    pub message: String,
    pub server_version: Option<String>,
}

/// Trigger sync for a workspace - performs actual HTTP sync
#[tauri::command]
pub async fn sync_workspace(
    workspace_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<SyncResultResponse, String> {
    // Extract data from state while holding the lock briefly
    let (db_path, server_url, account_id) = {
        let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
        let db = state.db.as_ref().ok_or("Database not initialized")?;

        let workspace_uuid =
            Uuid::parse_str(&workspace_id).map_err(|_| "Invalid workspace ID".to_string())?;

        let state_repo = SqliteSyncStateRepo::new(db);

        // Check if sync is configured
        let sync_state = state_repo
            .get_state(&workspace_uuid)
            .map_err(|e| e.to_string())?
            .ok_or("Sync not configured for this workspace")?;

        if sync_state.server_url.is_empty() {
            return Err("No sync server configured".to_string());
        }

        // Get current user ID
        let account_id = state
            .session
            .token
            .as_ref()
            .and_then(|token| {
                dataforge_core::auth::validate_session(db, token)
                    .ok()
                    .map(|result| result.account.id)
            })
            .ok_or("Not authenticated")?;

        // Update status to syncing
        state_repo
            .update_status(&workspace_uuid, SyncStatus::Syncing)
            .map_err(|e| e.to_string())?;

        // Get db path for reopening in async context
        let db_path = db.path()
            .map(|p| std::path::PathBuf::from(p))
            .ok_or("No database path")?;

        (db_path, sync_state.server_url.clone(), account_id)
    };

    let workspace_uuid =
        Uuid::parse_str(&workspace_id).map_err(|_| "Invalid workspace ID".to_string())?;

    // Get sync auth token if available
    let sync_auth_token = {
        let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
        state.session.sync_auth_token.clone()
    };

    info!("Starting sync for workspace {} to {}", workspace_id, &server_url);

    // Create sync client with optional auth token
    let client = SyncClient::new(&server_url);
    let client = if let Some(ref token) = sync_auth_token {
        debug!("Using sync auth token for server authentication");
        client.with_auth(token)
    } else {
        debug!("No sync auth token configured - server must have skip_auth enabled");
        client
    };

    // Perform sync operations
    let sync_result = perform_sync(&db_path, &workspace_uuid, &account_id, &client).await;

    // Update final status
    {
        let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
        let db = state.db.as_ref().ok_or("Database not initialized")?;

        let state_repo = SqliteSyncStateRepo::new(db);
        let queue_repo = SqliteSyncQueueRepo::new(db);
        let conflict_repo = SqliteConflictRepo::new(db);

        match &sync_result {
            Ok(_) => {
                // Clear any previous error and set status to idle
                state_repo
                    .clear_error(&workspace_uuid)
                    .map_err(|e| e.to_string())?;
            }
            Err(err) => {
                // Set error (this also updates status to error)
                state_repo
                    .set_error(&workspace_uuid, err)
                    .map_err(|e| e.to_string())?;
            }
        }

        let pending_count = queue_repo
            .count_pending(&workspace_uuid)
            .map_err(|e| e.to_string())?;

        let conflict_count = conflict_repo
            .count_pending(&workspace_uuid)
            .map_err(|e| e.to_string())?;

        match sync_result {
            Ok(_) => Ok(SyncResultResponse {
                success: true,
                pending_changes: pending_count,
                conflicts: conflict_count,
                error: None,
            }),
            Err(err) => Ok(SyncResultResponse {
                success: false,
                pending_changes: pending_count,
                conflicts: conflict_count,
                error: Some(err),
            }),
        }
    }
}

/// Perform the actual sync operations (push and pull)
///
/// This function carefully structures database operations to not hold
/// Connection references across await points (Connection is not Send).
async fn perform_sync(
    db_path: &std::path::Path,
    workspace_id: &Uuid,
    account_id: &Uuid,
    client: &SyncClient,
) -> Result<(), String> {
    // Step 1: Read state and pending changes (scoped to drop connection before await)
    let (from_version, pending, entry_ids) = {
        let db = rusqlite::Connection::open(db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;
        let state_repo = SqliteSyncStateRepo::new(&db);
        let queue_repo = SqliteSyncQueueRepo::new(&db);

        let sync_state = state_repo
            .get_state(workspace_id)
            .map_err(|e| e.to_string())?
            .ok_or("Sync state not found")?;

        let from_version = sync_state.last_sync_version;

        let pending = queue_repo
            .get_pending(workspace_id, 100)
            .map_err(|e| e.to_string())?;

        let entry_ids: Vec<i64> = pending.iter().map(|e| e.id).collect();

        (from_version, pending, entry_ids)
    }; // db connection dropped here

    info!(
        "Sync: {} pending changes to push, from version {}",
        pending.len(),
        from_version
    );

    // Step 2: Push changes if any
    let mut push_server_version: Option<i64> = None;

    if !pending.is_empty() {
        // Convert pending entries to sync changes
        let changes: Vec<dataforge_sync::protocol::SyncChange> = pending
            .iter()
            .map(|entry| dataforge_sync::protocol::SyncChange {
                id: Uuid::new_v4(),
                entity_type: parse_entity_type(&entry.entity_type),
                entity_id: entry.entity_id,
                action: parse_sync_action(&entry.action.to_string()),
                version: entry.version,
                timestamp: entry.created_at,
                user_id: *account_id,
                data: entry.payload.clone(),
            })
            .collect();

        let push_request = dataforge_sync::protocol::PushRequest {
            from_version,
            changes,
            pending_blobs: vec![],
        };

        // Await happens here - no db connection held
        match client.push(push_request).await {
            Ok(response) => {
                info!(
                    "Push successful: {} accepted, {} conflicts, server version {}",
                    response.accepted.len(),
                    response.conflicts.len(),
                    response.server_version
                );

                push_server_version = Some(response.server_version);

                if !response.conflicts.is_empty() {
                    warn!("Sync push had {} conflicts - need to handle", response.conflicts.len());
                }
            }
            Err(dataforge_sync::client::SyncError::Conflict(count)) => {
                warn!("Push failed with {} conflicts - need to pull first", count);
            }
            Err(e) => {
                error!("Push failed: {}", e);
                return Err(format!("Push failed: {}", e));
            }
        }

        // Step 3: Update database after push (new connection)
        if let Some(server_version) = push_server_version {
            let db = rusqlite::Connection::open(db_path)
                .map_err(|e| format!("Failed to open database: {}", e))?;
            let state_repo = SqliteSyncStateRepo::new(&db);
            let queue_repo = SqliteSyncQueueRepo::new(&db);

            if !entry_ids.is_empty() {
                if let Err(e) = queue_repo.mark_synced(&entry_ids) {
                    warn!("Failed to mark entries as synced: {}", e);
                }
            }

            state_repo
                .update_last_sync(workspace_id, server_version)
                .map_err(|e| e.to_string())?;
        }
    }

    // Step 4: Get current version and conflict strategy for pull (new connection)
    let (current_version, conflict_strategy) = {
        let db = rusqlite::Connection::open(db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;
        let state_repo = SqliteSyncStateRepo::new(&db);

        let sync_state = state_repo
            .get_state(workspace_id)
            .map_err(|e| e.to_string())?;

        match sync_state {
            Some(s) => (s.last_sync_version, s.conflict_strategy),
            None => (0, ConflictStrategy::default()),
        }
    }; // db connection dropped here

    // Step 5: Pull from server (await happens here)
    let pull_request = dataforge_sync::protocol::PullRequest {
        from_version: current_version,
        limit: Some(100),
    };

    match client.pull(pull_request).await {
        Ok(response) => {
            info!(
                "Pull successful: {} changes, server version {}, has_more: {}",
                response.changes.len(),
                response.server_version,
                response.has_more
            );

            // Step 6: Download required blobs (ColaNode/Harbor pattern)
            if !response.required_blobs.is_empty() {
                download_required_blobs(db_path, client, &response.required_blobs).await?;
            }

            // Step 7: Apply changes to local database (ColaNode pattern with conflict strategy)
            if !response.changes.is_empty() {
                apply_pulled_changes(db_path, workspace_id, &response, conflict_strategy).await?;
            }

            // Step 8: Update sync version after successful apply
            {
                let db = rusqlite::Connection::open(db_path)
                    .map_err(|e| format!("Failed to open database: {}", e))?;
                let state_repo = SqliteSyncStateRepo::new(&db);

                state_repo
                    .update_last_sync(workspace_id, response.server_version)
                    .map_err(|e| e.to_string())?;
            }

            if response.has_more {
                info!("Server has more changes - will need another sync");
            }
        }
        Err(e) => {
            error!("Pull failed: {}", e);
            return Err(format!("Pull failed: {}", e));
        }
    }

    info!("Sync completed successfully");
    Ok(())
}

/// Parse entity type string to enum
fn parse_entity_type(s: &str) -> dataforge_sync::protocol::EntityType {
    match s.to_lowercase().as_str() {
        "project" => dataforge_sync::protocol::EntityType::Project,
        "well" => dataforge_sync::protocol::EntityType::Well,
        "curve" => dataforge_sync::protocol::EntityType::Curve,
        "curve_metadata" | "curvemetadata" => dataforge_sync::protocol::EntityType::CurveMetadata,
        _ => dataforge_sync::protocol::EntityType::Project, // Default fallback
    }
}

/// Parse operation string to sync action
fn parse_sync_action(s: &str) -> dataforge_sync::protocol::SyncAction {
    match s.to_lowercase().as_str() {
        "create" | "insert" => dataforge_sync::protocol::SyncAction::Create,
        "update" => dataforge_sync::protocol::SyncAction::Update,
        "delete" => dataforge_sync::protocol::SyncAction::Delete,
        _ => dataforge_sync::protocol::SyncAction::Update, // Default fallback
    }
}

// ============================================================
// CONFLICT MANAGEMENT
// ============================================================

/// Get pending conflicts for a workspace
#[tauri::command]
pub fn get_conflicts(
    workspace_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<SyncConflictResponse>, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let workspace_uuid =
        Uuid::parse_str(&workspace_id).map_err(|_| "Invalid workspace ID".to_string())?;

    let conflict_repo = SqliteConflictRepo::new(db);

    let conflicts = conflict_repo
        .get_pending(&workspace_uuid)
        .map_err(|e| e.to_string())?;

    Ok(conflicts.into_iter().map(|c| c.into()).collect())
}

/// Resolve a conflict
#[tauri::command]
pub fn resolve_conflict(
    conflict_id: i64,
    resolution: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let app_state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = app_state.db.as_ref().ok_or("Database not initialized")?;

    // Get current account ID from session
    let account_id = app_state
        .session
        .token
        .as_ref()
        .and_then(|token| {
            // Validate session and get account
            dataforge_core::auth::validate_session(db, token)
                .ok()
                .map(|result| result.account.id)
        })
        .ok_or("Not authenticated")?;

    let conflict_repo = SqliteConflictRepo::new(db);

    // Parse resolution
    let resolution: ConflictResolution = resolution
        .parse()
        .map_err(|e: String| format!("Invalid resolution: {}", e))?;

    conflict_repo
        .resolve(conflict_id, resolution, &account_id)
        .map_err(|e| e.to_string())?;

    info!("Resolved conflict {} with {:?}", conflict_id, resolution);

    Ok(())
}

// ============================================================
// QUEUE MANAGEMENT
// ============================================================

/// Get count of pending sync changes for a workspace
#[tauri::command]
pub fn get_pending_sync_count(
    workspace_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<i64, String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let workspace_uuid =
        Uuid::parse_str(&workspace_id).map_err(|_| "Invalid workspace ID".to_string())?;

    let queue_repo = SqliteSyncQueueRepo::new(db);

    queue_repo
        .count_pending(&workspace_uuid)
        .map_err(|e| e.to_string())
}

/// Clear synced entries from the queue (cleanup)
#[tauri::command]
pub fn clear_synced_queue(
    workspace_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let state = state.lock().map_err(|e| format!("Lock error: {}", e))?;
    let db = state.db.as_ref().ok_or("Database not initialized")?;

    let workspace_uuid =
        Uuid::parse_str(&workspace_id).map_err(|_| "Invalid workspace ID".to_string())?;

    let queue_repo = SqliteSyncQueueRepo::new(db);

    queue_repo
        .clear_synced(&workspace_uuid)
        .map_err(|e| e.to_string())?;

    info!("Cleared synced queue entries for workspace {}", workspace_id);

    Ok(())
}

// ============================================================
// CHANGE APPLICATION (ColaNode-inspired)
// ============================================================

/// Download required blobs from server before applying changes
///
/// Follows Harbor's content-addressed storage pattern:
/// 1. Check if blob already exists locally (deduplication)
/// 2. Get presigned download URLs from server
/// 3. Download and verify integrity (SHA-256)
/// 4. Register in blob_registry
async fn download_required_blobs(
    db_path: &Path,
    client: &SyncClient,
    required_hashes: &[String],
) -> Result<(), String> {
    if required_hashes.is_empty() {
        return Ok(());
    }

    info!("Downloading {} required blobs", required_hashes.len());

    // Get blob store path (sibling to database)
    let blob_dir = db_path
        .parent()
        .ok_or("Invalid database path")?
        .join("blobs");

    let blob_store =
        BlobStore::new(&blob_dir).map_err(|e| format!("Failed to create blob store: {}", e))?;

    // Filter out blobs we already have (deduplication)
    let missing_hashes: Vec<String> = required_hashes
        .iter()
        .filter(|hash| !blob_store.exists(hash))
        .cloned()
        .collect();

    if missing_hashes.is_empty() {
        info!("All required blobs already exist locally");
        return Ok(());
    }

    info!(
        "{} blobs already cached, {} need download",
        required_hashes.len() - missing_hashes.len(),
        missing_hashes.len()
    );

    // Get presigned download URLs from server
    let urls_response = client
        .get_blob_urls(missing_hashes.clone())
        .await
        .map_err(|e| format!("Failed to get blob URLs: {}", e))?;

    // Download each blob
    for blob_url in urls_response.urls {
        debug!("Downloading blob: {}", blob_url.hash);

        // Download blob data
        let data = client
            .download_blob(&blob_url.url)
            .await
            .map_err(|e| format!("Failed to download blob {}: {}", blob_url.hash, e))?;

        // Verify integrity (Harbor pattern: always verify hash)
        let actual_hash = BlobStore::compute_hash(&data);
        if actual_hash != blob_url.hash {
            return Err(format!(
                "Blob hash mismatch: expected {}, got {}",
                blob_url.hash, actual_hash
            ));
        }

        // Store blob (content-addressed, atomic write)
        blob_store
            .store(&data)
            .map_err(|e| format!("Failed to store blob: {}", e))?;

        info!(
            "Downloaded and stored blob: {} ({} bytes)",
            blob_url.hash,
            data.len()
        );
    }

    // Register downloaded blobs in blob_registry
    {
        let db = Connection::open(db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        for hash in &missing_hashes {
            if let Ok(size) = blob_store.size(hash) {
                db.execute(
                    r#"
                    INSERT OR IGNORE INTO blob_registry (hash, size_bytes, synced_to_server, synced_at)
                    VALUES (?1, ?2, 1, datetime('now'))
                    "#,
                    params![hash, size as i64],
                )
                .map_err(|e| format!("Failed to register blob: {}", e))?;
            }
        }

        info!("Registered {} blobs in registry", missing_hashes.len());
    }

    Ok(())
}

/// Apply pulled changes to local database
///
/// Follows ColaNode's change application patterns:
/// 1. Process changes in a transaction
/// 2. For each change, check local version vs remote version
/// 3. If local.version > remote.version → apply conflict strategy
/// 4. If local.version <= remote.version → apply change
/// 5. Use soft deletes (deleted_at) instead of hard deletes
///
/// ## Conflict Strategies
/// - Manual: Queue conflicts for user resolution (default, safest)
/// - LastWriteWins: Automatically keep the newer change based on timestamp
/// - LocalWins: Always keep local changes (skip remote)
/// - RemoteWins: Always accept remote changes
async fn apply_pulled_changes(
    db_path: &Path,
    workspace_id: &Uuid,
    response: &PullResponse,
    strategy: ConflictStrategy,
) -> Result<(), String> {
    info!(
        "Applying {} changes from server (strategy: {:?})",
        response.changes.len(),
        strategy
    );

    let db = Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // Begin transaction for atomic change application
    db.execute("BEGIN TRANSACTION", [])
        .map_err(|e| format!("Failed to begin transaction: {}", e))?;

    let mut applied_count = 0;
    let mut conflict_count = 0;
    let mut skipped_count = 0;
    let mut auto_resolved_count = 0;

    for change in &response.changes {
        match apply_single_change(&db, workspace_id, change, strategy) {
            Ok(ApplyResult::Applied) => {
                applied_count += 1;
                debug!(
                    "Applied {:?} {:?} for entity {}",
                    change.action, change.entity_type, change.entity_id
                );
            }
            Ok(ApplyResult::Conflict) => {
                conflict_count += 1;
                warn!(
                    "Conflict detected for {:?} entity {} (queued for resolution)",
                    change.entity_type, change.entity_id
                );
            }
            Ok(ApplyResult::AutoResolved(resolution)) => {
                auto_resolved_count += 1;
                info!(
                    "Auto-resolved {:?} entity {} using {:?}",
                    change.entity_type, change.entity_id, resolution
                );
            }
            Ok(ApplyResult::Skipped) => {
                skipped_count += 1;
                debug!(
                    "Skipped {:?} {:?} for entity {} (already up to date)",
                    change.action, change.entity_type, change.entity_id
                );
            }
            Err(e) => {
                // Rollback on error
                let _ = db.execute("ROLLBACK", []);
                return Err(format!(
                    "Failed to apply change for {:?} {}: {}",
                    change.entity_type, change.entity_id, e
                ));
            }
        }
    }

    // Commit transaction
    db.execute("COMMIT", [])
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    info!(
        "Change application complete: {} applied, {} auto-resolved, {} conflicts, {} skipped",
        applied_count, auto_resolved_count, conflict_count, skipped_count
    );

    Ok(())
}

/// Result of applying a single change
enum ApplyResult {
    Applied,
    Conflict,
    Skipped,
    /// Auto-resolved using the specified strategy
    AutoResolved(ConflictResolution),
}

/// Apply a single change with version-based conflict detection and auto-resolution
///
/// ColaNode conflict detection pattern with configurable resolution strategies:
/// - Query local entity first
/// - Compare versions: local.version > remote.version = potential conflict
/// - Apply conflict strategy to determine resolution:
///   - Manual: Queue for user resolution
///   - LastWriteWins: Compare timestamps, keep newer
///   - LocalWins: Keep local, skip remote
///   - RemoteWins: Apply remote unconditionally
fn apply_single_change(
    db: &Connection,
    workspace_id: &Uuid,
    change: &SyncChange,
    strategy: ConflictStrategy,
) -> Result<ApplyResult, String> {
    match change.entity_type {
        EntityType::Well => apply_well_change(db, workspace_id, change, strategy),
        EntityType::Curve => apply_curve_change(db, workspace_id, change, strategy),
        EntityType::CurveMetadata => apply_curve_metadata_change(db, change),
        EntityType::Project => {
            // Projects/workspaces are handled separately
            debug!("Skipping project change (handled by workspace sync)");
            Ok(ApplyResult::Skipped)
        }
    }
}

/// Apply a well change with conflict strategy
fn apply_well_change(
    db: &Connection,
    workspace_id: &Uuid,
    change: &SyncChange,
    strategy: ConflictStrategy,
) -> Result<ApplyResult, String> {
    let entity_id = change.entity_id.to_string();

    // Check local version and updated_at (ColaNode pattern: query before apply)
    let local_info: Option<(i64, Option<DateTime<Utc>>)> = db
        .query_row(
            "SELECT version, updated_at FROM wells WHERE id = ?1",
            params![&entity_id],
            |row| {
                let version: i64 = row.get(0)?;
                let updated_at: Option<String> = row.get(1)?;
                let updated_at = updated_at.and_then(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .ok()
                        .map(|dt| dt.with_timezone(&Utc))
                        .or_else(|| {
                            chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                                .ok()
                                .map(|ndt| ndt.and_utc())
                        })
                });
                Ok((version, updated_at))
            },
        )
        .ok();

    // Conflict detection: local.version > remote.version
    if let Some((local_ver, local_updated_at)) = local_info {
        if local_ver > change.version {
            // Potential conflict - apply strategy
            match strategy {
                ConflictStrategy::Manual => {
                    // Queue for user resolution
                    let local_data = get_well_json(db, &entity_id)?;
                    let remote_data = change.data.clone().unwrap_or_default();

                    queue_conflict(
                        db,
                        workspace_id,
                        "well",
                        &change.entity_id,
                        local_ver,
                        &local_data,
                        change.version,
                        &remote_data,
                    )?;

                    return Ok(ApplyResult::Conflict);
                }
                ConflictStrategy::LocalWins => {
                    // Keep local, skip remote
                    debug!("LocalWins: keeping local well {}", entity_id);
                    return Ok(ApplyResult::AutoResolved(ConflictResolution::Local));
                }
                ConflictStrategy::RemoteWins => {
                    // Apply remote unconditionally (fall through to apply)
                    debug!("RemoteWins: applying remote well {}", entity_id);
                }
                ConflictStrategy::LastWriteWins => {
                    // Compare timestamps
                    let remote_timestamp = change.timestamp;
                    let local_timestamp = local_updated_at.unwrap_or(Utc::now());

                    if local_timestamp > remote_timestamp {
                        // Local is newer, skip remote
                        debug!(
                            "LastWriteWins: local ({}) newer than remote ({}), keeping local well {}",
                            local_timestamp, remote_timestamp, entity_id
                        );
                        return Ok(ApplyResult::AutoResolved(ConflictResolution::Local));
                    } else {
                        // Remote is newer or equal, apply remote (fall through)
                        debug!(
                            "LastWriteWins: remote ({}) newer than local ({}), applying remote well {}",
                            remote_timestamp, local_timestamp, entity_id
                        );
                    }
                }
            }
        }

        // Skip if already at same version (no conflict)
        if local_ver == change.version {
            return Ok(ApplyResult::Skipped);
        }
    }

    // Apply the change
    match change.action {
        SyncAction::Create | SyncAction::Update => {
            let data = change
                .data
                .as_ref()
                .ok_or("Missing data for create/update")?;
            upsert_well(db, &entity_id, data, change.version)?;
        }
        SyncAction::Delete => {
            // Soft delete (ColaNode pattern)
            db.execute(
                r#"
                UPDATE wells
                SET deleted_at = datetime('now'), version = ?1, updated_at = datetime('now')
                WHERE id = ?2
                "#,
                params![change.version, &entity_id],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(ApplyResult::Applied)
}

/// Apply a curve change with conflict strategy
fn apply_curve_change(
    db: &Connection,
    workspace_id: &Uuid,
    change: &SyncChange,
    strategy: ConflictStrategy,
) -> Result<ApplyResult, String> {
    let entity_id = change.entity_id.to_string();

    // Check local version and updated_at
    let local_info: Option<(i64, Option<DateTime<Utc>>)> = db
        .query_row(
            "SELECT version, updated_at FROM curves WHERE id = ?1",
            params![&entity_id],
            |row| {
                let version: i64 = row.get(0)?;
                let updated_at: Option<String> = row.get(1)?;
                let updated_at = updated_at.and_then(|s| {
                    DateTime::parse_from_rfc3339(&s)
                        .ok()
                        .map(|dt| dt.with_timezone(&Utc))
                        .or_else(|| {
                            chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                                .ok()
                                .map(|ndt| ndt.and_utc())
                        })
                });
                Ok((version, updated_at))
            },
        )
        .ok();

    // Conflict detection
    if let Some((local_ver, local_updated_at)) = local_info {
        if local_ver > change.version {
            // Potential conflict - apply strategy
            match strategy {
                ConflictStrategy::Manual => {
                    // Queue for user resolution
                    let local_data = get_curve_json(db, &entity_id)?;
                    let remote_data = change.data.clone().unwrap_or_default();

                    queue_conflict(
                        db,
                        workspace_id,
                        "curve",
                        &change.entity_id,
                        local_ver,
                        &local_data,
                        change.version,
                        &remote_data,
                    )?;

                    return Ok(ApplyResult::Conflict);
                }
                ConflictStrategy::LocalWins => {
                    // Keep local, skip remote
                    debug!("LocalWins: keeping local curve {}", entity_id);
                    return Ok(ApplyResult::AutoResolved(ConflictResolution::Local));
                }
                ConflictStrategy::RemoteWins => {
                    // Apply remote unconditionally (fall through to apply)
                    debug!("RemoteWins: applying remote curve {}", entity_id);
                }
                ConflictStrategy::LastWriteWins => {
                    // Compare timestamps
                    let remote_timestamp = change.timestamp;
                    let local_timestamp = local_updated_at.unwrap_or(Utc::now());

                    if local_timestamp > remote_timestamp {
                        // Local is newer, skip remote
                        debug!(
                            "LastWriteWins: local ({}) newer than remote ({}), keeping local curve {}",
                            local_timestamp, remote_timestamp, entity_id
                        );
                        return Ok(ApplyResult::AutoResolved(ConflictResolution::Local));
                    } else {
                        // Remote is newer or equal, apply remote (fall through)
                        debug!(
                            "LastWriteWins: remote ({}) newer than local ({}), applying remote curve {}",
                            remote_timestamp, local_timestamp, entity_id
                        );
                    }
                }
            }
        }

        if local_ver == change.version {
            return Ok(ApplyResult::Skipped);
        }
    }

    // Apply the change
    match change.action {
        SyncAction::Create | SyncAction::Update => {
            let data = change
                .data
                .as_ref()
                .ok_or("Missing data for create/update")?;
            upsert_curve(db, &entity_id, data, change.version)?;
        }
        SyncAction::Delete => {
            // Soft delete
            db.execute(
                r#"
                UPDATE curves
                SET deleted_at = datetime('now'), version = ?1, updated_at = datetime('now')
                WHERE id = ?2
                "#,
                params![change.version, &entity_id],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(ApplyResult::Applied)
}

/// Apply a curve metadata change
fn apply_curve_metadata_change(
    db: &Connection,
    change: &SyncChange,
) -> Result<ApplyResult, String> {
    let entity_id = change.entity_id.to_string();

    // Curve metadata doesn't have version tracking in our schema
    // Just upsert based on action
    match change.action {
        SyncAction::Create | SyncAction::Update => {
            let data = change
                .data
                .as_ref()
                .ok_or("Missing data for create/update")?;
            upsert_curve_metadata(db, &entity_id, data)?;
        }
        SyncAction::Delete => {
            // Hard delete for metadata (it's reference data)
            db.execute(
                "DELETE FROM curve_metadata WHERE id = ?1",
                params![&entity_id],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(ApplyResult::Applied)
}

/// Queue a conflict for user resolution
fn queue_conflict(
    db: &Connection,
    workspace_id: &Uuid,
    entity_type: &str,
    entity_id: &Uuid,
    local_version: i64,
    local_data: &str,
    remote_version: i64,
    remote_data: &str,
) -> Result<(), String> {
    db.execute(
        r#"
        INSERT INTO sync_conflicts (
            workspace_id, entity_type, entity_id,
            local_version, local_data, remote_version, remote_data,
            resolution, created_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'pending', datetime('now'))
        ON CONFLICT DO UPDATE SET
            local_version = excluded.local_version,
            local_data = excluded.local_data,
            remote_version = excluded.remote_version,
            remote_data = excluded.remote_data,
            resolution = 'pending',
            resolved_by = NULL,
            resolved_at = NULL
        "#,
        params![
            workspace_id.to_string(),
            entity_type,
            entity_id.to_string(),
            local_version,
            local_data,
            remote_version,
            remote_data,
        ],
    )
    .map_err(|e| format!("Failed to queue conflict: {}", e))?;

    Ok(())
}

/// Get well as JSON for conflict storage
fn get_well_json(db: &Connection, entity_id: &str) -> Result<String, String> {
    let result: Result<String, _> = db.query_row(
        r#"
        SELECT json_object(
            'id', id,
            'workspace_id', workspace_id,
            'name', name,
            'uwi', uwi,
            'field', field,
            'company', company,
            'location', location,
            'x', x,
            'y', y,
            'created_by', created_by,
            'version', version
        ) FROM wells WHERE id = ?1
        "#,
        params![entity_id],
        |row| row.get(0),
    );

    result.map_err(|e| e.to_string())
}

/// Get curve as JSON for conflict storage
fn get_curve_json(db: &Connection, entity_id: &str) -> Result<String, String> {
    let result: Result<String, _> = db.query_row(
        r#"
        SELECT json_object(
            'id', id,
            'well_id', well_id,
            'curve_name', curve_name,
            'blob_hash', blob_hash,
            'parquet_column_name', parquet_column_name,
            'version', version
        ) FROM curves WHERE id = ?1
        "#,
        params![entity_id],
        |row| row.get(0),
    );

    result.map_err(|e| e.to_string())
}

/// Upsert a well from JSON data
fn upsert_well(db: &Connection, entity_id: &str, json_data: &str, version: i64) -> Result<(), String> {
    let well: serde_json::Value =
        serde_json::from_str(json_data).map_err(|e| format!("Invalid well JSON: {}", e))?;

    db.execute(
        r#"
        INSERT INTO wells (id, workspace_id, name, uwi, field, company, location, x, y, created_by, version, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, datetime('now'), datetime('now'))
        ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            uwi = excluded.uwi,
            field = excluded.field,
            company = excluded.company,
            location = excluded.location,
            x = excluded.x,
            y = excluded.y,
            version = excluded.version,
            updated_at = datetime('now'),
            deleted_at = NULL
        "#,
        params![
            entity_id,
            well["workspace_id"].as_str().unwrap_or(""),
            well["name"].as_str().unwrap_or(""),
            well["uwi"].as_str(),
            well["field"].as_str(),
            well["company"].as_str(),
            well["location"].as_str(),
            well["x"].as_f64(),
            well["y"].as_f64(),
            well["created_by"].as_str().unwrap_or(""),
            version,
        ],
    )
    .map_err(|e| format!("Failed to upsert well: {}", e))?;

    Ok(())
}

/// Upsert a curve from JSON data
fn upsert_curve(db: &Connection, entity_id: &str, json_data: &str, version: i64) -> Result<(), String> {
    let curve: serde_json::Value =
        serde_json::from_str(json_data).map_err(|e| format!("Invalid curve JSON: {}", e))?;

    db.execute(
        r#"
        INSERT INTO curves (
            id, well_id, curve_name, curve_metadata_id, blob_hash, parquet_column_name,
            row_count, min_depth, max_depth, min_value, max_value,
            created_by, version, created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, datetime('now'), datetime('now'))
        ON CONFLICT(id) DO UPDATE SET
            curve_name = excluded.curve_name,
            curve_metadata_id = excluded.curve_metadata_id,
            blob_hash = excluded.blob_hash,
            parquet_column_name = excluded.parquet_column_name,
            row_count = excluded.row_count,
            min_depth = excluded.min_depth,
            max_depth = excluded.max_depth,
            min_value = excluded.min_value,
            max_value = excluded.max_value,
            version = excluded.version,
            updated_at = datetime('now'),
            deleted_at = NULL
        "#,
        params![
            entity_id,
            curve["well_id"].as_str().unwrap_or(""),
            curve["curve_name"].as_str().unwrap_or(""),
            curve["curve_metadata_id"].as_str(),
            curve["blob_hash"].as_str().unwrap_or(""),
            curve["parquet_column_name"].as_str().unwrap_or(""),
            curve["row_count"].as_i64(),
            curve["min_depth"].as_f64(),
            curve["max_depth"].as_f64(),
            curve["min_value"].as_f64(),
            curve["max_value"].as_f64(),
            curve["created_by"].as_str().unwrap_or(""),
            version,
        ],
    )
    .map_err(|e| format!("Failed to upsert curve: {}", e))?;

    Ok(())
}

/// Upsert curve metadata from JSON data
fn upsert_curve_metadata(db: &Connection, entity_id: &str, json_data: &str) -> Result<(), String> {
    let meta: serde_json::Value =
        serde_json::from_str(json_data).map_err(|e| format!("Invalid curve metadata JSON: {}", e))?;

    let vendor_variations = meta["vendor_variations"]
        .as_array()
        .map(|arr| serde_json::to_string(arr).unwrap_or_default());

    db.execute(
        r#"
        INSERT INTO curve_metadata (
            id, curve_mnemonic, main_curve_type, subcurve_name,
            display_name, description, units, vendor_variations, created_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, datetime('now'))
        ON CONFLICT(id) DO UPDATE SET
            curve_mnemonic = excluded.curve_mnemonic,
            main_curve_type = excluded.main_curve_type,
            subcurve_name = excluded.subcurve_name,
            display_name = excluded.display_name,
            description = excluded.description,
            units = excluded.units,
            vendor_variations = excluded.vendor_variations
        "#,
        params![
            entity_id,
            meta["curve_mnemonic"].as_str().unwrap_or(""),
            meta["main_curve_type"].as_str().unwrap_or("OTHER"),
            meta["subcurve_name"].as_str(),
            meta["display_name"].as_str().unwrap_or(""),
            meta["description"].as_str(),
            meta["units"].as_str().unwrap_or(""),
            vendor_variations,
        ],
    )
    .map_err(|e| format!("Failed to upsert curve metadata: {}", e))?;

    Ok(())
}
