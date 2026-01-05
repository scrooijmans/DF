//! Sync module for DataForge
//!
//! Provides interface-based sync architecture following ColaNode patterns:
//! - Git-like pull-based sync model (not real-time CRDT)
//! - Offline-first with queued changes
//! - Content-addressed blob storage
//! - Conflict resolution support
//!
//! ## Architecture Layers
//!
//! - **Traits**: Interface definitions for repositories and clients
//! - **Repository**: SQLite implementations of sync repositories
//! - **Models**: Re-exported from `crate::models` for sync-specific types
//!
//! ## Usage
//!
//! ```rust,ignore
//! use dataforge_core::sync::{SyncStateRepository, SqliteSyncStateRepo};
//!
//! let repo = SqliteSyncStateRepo::new(&conn);
//! let state = repo.get_state(&workspace_id)?;
//! ```

mod repository;
mod traits;

// Re-export traits
pub use traits::{ConflictRepository, SyncQueueRepository, SyncStateRepository};

// Re-export repository implementations
pub use repository::{SqliteConflictRepo, SqliteSyncQueueRepo, SqliteSyncStateRepo};

// Re-export models for convenience
pub use crate::models::{
    ConflictResolution, ConflictStrategy, SyncAction, SyncConflict, SyncQueueEntry, SyncState,
    SyncStatus, SyncSummary,
};
