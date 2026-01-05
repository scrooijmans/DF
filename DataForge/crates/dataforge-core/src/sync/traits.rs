//! Trait definitions for sync repositories
//!
//! These traits define the interface boundaries for sync operations,
//! following the interface-based architecture from ColaNode.
//! Implementations can be swapped for testing or different storage backends.

use crate::models::{ConflictResolution, SyncConflict, SyncQueueEntry, SyncState, SyncStatus};
use crate::Result;
use uuid::Uuid;

/// Repository interface for sync state management
///
/// Tracks the sync state for each workspace, including:
/// - Last synchronized version
/// - Sync status (idle, syncing, error, offline)
/// - Timestamps for last sync operations
pub trait SyncStateRepository {
    /// Get the sync state for a workspace
    fn get_state(&self, workspace_id: &Uuid) -> Result<Option<SyncState>>;

    /// Create or update sync state for a workspace
    fn save_state(&self, state: &SyncState) -> Result<()>;

    /// Update the last sync version after a successful sync
    fn update_last_sync(&self, workspace_id: &Uuid, version: i64) -> Result<()>;

    /// Update sync status (idle, syncing, error, offline)
    fn update_status(&self, workspace_id: &Uuid, status: SyncStatus) -> Result<()>;

    /// Record a sync error
    fn set_error(&self, workspace_id: &Uuid, error: &str) -> Result<()>;

    /// Clear sync error (on successful sync)
    fn clear_error(&self, workspace_id: &Uuid) -> Result<()>;

    /// Delete sync state for a workspace
    fn delete_state(&self, workspace_id: &Uuid) -> Result<()>;
}

/// Repository interface for the sync queue
///
/// Manages the queue of local changes waiting to be synced to the server.
/// Changes are queued when offline and processed when connection is restored.
pub trait SyncQueueRepository {
    /// Add a change to the sync queue
    ///
    /// If an entry for the same entity already exists, it will be updated
    /// (upsert behavior via UNIQUE constraint on workspace_id, entity_type, entity_id)
    fn enqueue(&self, entry: &SyncQueueEntry) -> Result<i64>;

    /// Get pending (unsynced) entries for a workspace
    ///
    /// Returns entries ordered by priority (desc) and created_at (asc)
    fn get_pending(&self, workspace_id: &Uuid, limit: usize) -> Result<Vec<SyncQueueEntry>>;

    /// Get all pending entries for a workspace (no limit)
    fn get_all_pending(&self, workspace_id: &Uuid) -> Result<Vec<SyncQueueEntry>>;

    /// Count pending entries for a workspace
    fn count_pending(&self, workspace_id: &Uuid) -> Result<i64>;

    /// Mark entries as synced
    fn mark_synced(&self, ids: &[i64]) -> Result<()>;

    /// Mark an entry as failed with error message
    fn mark_failed(&self, id: i64, error: &str) -> Result<()>;

    /// Increment attempt count for an entry
    fn increment_attempts(&self, id: i64) -> Result<()>;

    /// Clear all synced entries for a workspace (cleanup)
    fn clear_synced(&self, workspace_id: &Uuid) -> Result<()>;

    /// Delete a specific entry by ID
    fn delete(&self, id: i64) -> Result<()>;

    /// Delete all entries for an entity (e.g., when entity is deleted)
    fn delete_for_entity(&self, workspace_id: &Uuid, entity_type: &str, entity_id: &Uuid)
        -> Result<()>;
}

/// Repository interface for conflict management
///
/// Tracks conflicts detected during pull operations for user resolution.
pub trait ConflictRepository {
    /// Save a new conflict
    fn save_conflict(&self, conflict: &SyncConflict) -> Result<i64>;

    /// Get all pending conflicts for a workspace
    fn get_pending(&self, workspace_id: &Uuid) -> Result<Vec<SyncConflict>>;

    /// Get a specific conflict by ID
    fn get_by_id(&self, id: i64) -> Result<Option<SyncConflict>>;

    /// Count pending conflicts for a workspace
    fn count_pending(&self, workspace_id: &Uuid) -> Result<i64>;

    /// Resolve a conflict
    fn resolve(
        &self,
        id: i64,
        resolution: ConflictResolution,
        resolved_by: &Uuid,
    ) -> Result<()>;

    /// Delete a conflict (after resolution is applied)
    fn delete(&self, id: i64) -> Result<()>;

    /// Delete all resolved conflicts for a workspace (cleanup)
    fn clear_resolved(&self, workspace_id: &Uuid) -> Result<()>;
}

#[cfg(test)]
mod tests {
    // Trait tests would go here with mock implementations
}
