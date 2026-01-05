# DataForge Sync Architecture

## Overview

This document describes the sync architecture for DataForge, following ColaNode's patterns adapted for a **Git-like (pull-based) sync model** rather than real-time CRDT collaboration. This architecture is designed for enterprise geoscience environments where offline-first operation and data sovereignty are critical.

## Key Architectural Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| **Sync Model** | Git-like (pull-based) | Enterprise users don't need real-time Figma-like collaboration |
| **CRDT** | None for MVP | Version vectors suffice for pull-based sync |
| **Conflict Resolution** | Last-write-wins with manual override | Simple, predictable for enterprise users |
| **Large Data** | Content-addressed Parquet | Immutable blobs referenced by SHA-256 hash |
| **Protocol** | REST API | No WebSocket needed for pull-based sync |
| **Offline Queue** | SQLite sync_queue table | Reliable, transactional, survives crashes |

## Architecture Layers (Following ColaNode Pattern)

```
┌─────────────────────────────────────────────────────────────────────┐
│                    DataForge Sync Architecture                       │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  PRESENTATION LAYER (SvelteKit)                              │  │
│  │  - Sync status indicator                                      │  │
│  │  - Conflict resolution UI                                     │  │
│  │  - Offline mode banner                                        │  │
│  │  Dependencies: Application Layer only                         │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │ Uses                                      │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  APPLICATION LAYER (Tauri Commands)                          │  │
│  │  - SyncService (orchestrates sync operations)                 │  │
│  │  - QueueService (manages offline queue)                       │  │
│  │  - ConflictService (handles conflicts)                        │  │
│  │  Dependencies: Domain Layer                                   │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │ Uses                                      │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  DOMAIN LAYER (dataforge-core)                               │  │
│  │  - SyncChange (change tracking model)                         │  │
│  │  - SyncState (version vector tracking)                        │  │
│  │  - ConflictResolution (business rules)                        │  │
│  │  Dependencies: Interfaces only (traits)                       │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │ Depends on (interfaces)                   │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  INFRASTRUCTURE LAYER (dataforge-sync, dataforge-storage)    │  │
│  │  - SyncClient (HTTP client for server communication)          │  │
│  │  - BlobStorage (content-addressed storage abstraction)        │  │
│  │  - SQLite repositories                                        │  │
│  │  Dependencies: External systems                               │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  EXTERNAL SYSTEMS                                             │  │
│  │  - Local SQLite database                                      │  │
│  │  - Local blob storage (Parquet files)                         │  │
│  │  - Remote sync server (REST API)                              │  │
│  │  - Remote blob storage (S3/MinIO)                             │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

## Sync Flow Diagrams

### 1. Local Change → Queue → Sync

```
┌─────────────────────────────────────────────────────────────────────┐
│  User Action: Creates/Updates Well                                   │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. Local SQLite Write (Immediate)                                   │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  BEGIN TRANSACTION;                                           │  │
│  │  INSERT INTO wells (...) VALUES (...);                        │  │
│  │  UPDATE wells SET version = version + 1 WHERE id = ?;         │  │
│  │  INSERT INTO sync_queue (entity_type, entity_id, action, ...) │  │
│  │  COMMIT;                                                       │  │
│  └──────────────────────────────────────────────────────────────┘  │
│  - Change persisted locally                                          │
│  - Version incremented                                               │
│  - Queued for sync                                                   │
│  - UI updates instantly                                              │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ Background Process (Non-blocking)
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. Sync Queue Worker                                                │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  IF online:                                                   │  │
│  │    - Batch pending changes from sync_queue                    │  │
│  │    - POST /api/sync/push with changes + pending blob hashes   │  │
│  │    - Upload missing blobs via presigned URLs                  │  │
│  │    - Mark changes as synced                                   │  │
│  │    - Update last_sync_version                                 │  │
│  │  ELSE:                                                        │  │
│  │    - Keep in queue, retry when online                         │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 2. Pull Remote Changes

```
┌─────────────────────────────────────────────────────────────────────┐
│  User Action: Click "Sync" or Auto-sync Timer                        │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. Request Remote Changes                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  GET /api/sync/pull?workspace_id=X&from_version=Y             │  │
│  │  Response: {                                                   │  │
│  │    changes: [...],                                             │  │
│  │    server_version: Z,                                          │  │
│  │    missing_blobs: [hash1, hash2, ...]                         │  │
│  │  }                                                             │  │
│  └──────────────────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. Download Missing Blobs                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  FOR each missing_blob:                                       │  │
│  │    - GET presigned URL from /api/blobs/url?hash=X             │  │
│  │    - Download blob from URL                                    │  │
│  │    - Verify SHA-256 hash                                       │  │
│  │    - Store in local blob storage                               │  │
│  │    - Register in blob_registry                                 │  │
│  └──────────────────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. Apply Changes to Local SQLite                                    │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  BEGIN TRANSACTION;                                           │  │
│  │  FOR each change:                                             │  │
│  │    - Check for conflicts (local version > remote version)     │  │
│  │    - IF conflict: Queue for resolution                        │  │
│  │    - ELSE: Apply change (INSERT/UPDATE/DELETE)                │  │
│  │  UPDATE sync_state SET last_sync_version = Z;                 │  │
│  │  COMMIT;                                                       │  │
│  └──────────────────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  4. Notify UI of Updates                                             │
│  - Emit 'sync:complete' event                                        │
│  - UI refreshes affected views                                       │
│  - Show conflict resolution UI if needed                             │
└─────────────────────────────────────────────────────────────────────┘
```

### 3. Conflict Resolution Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│  Conflict Detected During Pull                                       │
│  - Local change: version 5, updated_at: 10:00                       │
│  - Remote change: version 4, updated_at: 09:55                      │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. Conflict Resolution Strategy                                     │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  STRATEGY 1: Last-Write-Wins (Default)                        │  │
│  │  - Compare updated_at timestamps                               │  │
│  │  - Keep newer change, discard older                            │  │
│  │  - Log discarded change for audit                              │  │
│  │                                                                │  │
│  │  STRATEGY 2: Local-Wins                                        │  │
│  │  - Always keep local changes                                   │  │
│  │  - Push local version to server                                │  │
│  │                                                                │  │
│  │  STRATEGY 3: Remote-Wins                                       │  │
│  │  - Always accept remote changes                                │  │
│  │  - Discard local changes                                       │  │
│  │                                                                │  │
│  │  STRATEGY 4: Manual Resolution                                 │  │
│  │  - Queue conflict for user review                              │  │
│  │  - Show both versions in UI                                    │  │
│  │  - User chooses which to keep                                  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

## Server URL Configuration (ColaNode Pattern)

Following ColaNode's configuration pattern, the sync server URL is determined in priority order:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Server URL Resolution (Priority Order)                              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  1. Workspace Configuration (Highest Priority)                       │
│     - Each workspace can have its own sync server                    │
│     - Stored in workspaces.sync_server_url                          │
│     - Example: 'https://houston-sync.company.com'                   │
│                                                                      │
│  2. User Settings                                                    │
│     - User can configure default server                              │
│     - Stored in user_settings table                                  │
│     - Example: 'https://my-dataforge.company.com'                   │
│                                                                      │
│  3. Environment Variable                                             │
│     - For development/testing                                        │
│     - DATAFORGE_SYNC_SERVER_URL                                     │
│     - Example: 'http://localhost:3000'                              │
│                                                                      │
│  4. Default Server (Lowest Priority)                                 │
│     - Hardcoded fallback                                             │
│     - Example: 'https://sync.dataforge.io'                          │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## Database Schema Additions

### Workspace Sync Configuration

```sql
-- Add sync_server_url to workspaces table
ALTER TABLE workspaces ADD COLUMN sync_server_url TEXT;
ALTER TABLE workspaces ADD COLUMN sync_enabled INTEGER NOT NULL DEFAULT 1;
ALTER TABLE workspaces ADD COLUMN sync_interval_seconds INTEGER NOT NULL DEFAULT 300;
```

### Sync State Table (Enhanced)

```sql
CREATE TABLE IF NOT EXISTS sync_state (
    workspace_id TEXT PRIMARY KEY REFERENCES workspaces(id),
    server_url TEXT NOT NULL,
    last_sync_version INTEGER NOT NULL DEFAULT 0,
    last_sync_at TEXT,
    last_push_at TEXT,
    last_pull_at TEXT,
    sync_status TEXT NOT NULL DEFAULT 'idle',  -- 'idle', 'syncing', 'error', 'offline'
    last_error TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

### Sync Queue Table (Enhanced)

```sql
CREATE TABLE IF NOT EXISTS sync_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id),
    entity_type TEXT NOT NULL,        -- 'well', 'curve', 'curve_metadata'
    entity_id TEXT NOT NULL,
    action TEXT NOT NULL,             -- 'create', 'update', 'delete'
    version INTEGER NOT NULL,         -- Entity version at time of change
    payload TEXT,                     -- JSON snapshot of entity
    blob_hashes TEXT,                 -- JSON array of associated blob hashes
    priority INTEGER NOT NULL DEFAULT 0,  -- Higher = sync first
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    attempts INTEGER NOT NULL DEFAULT 0,
    last_attempt_at TEXT,
    last_error TEXT,
    synced_at TEXT,

    UNIQUE(workspace_id, entity_type, entity_id)
);

CREATE INDEX IF NOT EXISTS idx_sync_queue_workspace ON sync_queue(workspace_id);
CREATE INDEX IF NOT EXISTS idx_sync_queue_status ON sync_queue(synced_at);
CREATE INDEX IF NOT EXISTS idx_sync_queue_priority ON sync_queue(priority DESC, created_at ASC);
```

### Conflict Log Table

```sql
CREATE TABLE IF NOT EXISTS sync_conflicts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id),
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    local_version INTEGER NOT NULL,
    local_data TEXT NOT NULL,         -- JSON snapshot
    remote_version INTEGER NOT NULL,
    remote_data TEXT NOT NULL,        -- JSON snapshot
    resolution TEXT,                  -- 'local', 'remote', 'merged', 'pending'
    resolved_by TEXT,                 -- Account ID who resolved
    resolved_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_sync_conflicts_pending ON sync_conflicts(resolution) WHERE resolution = 'pending';
```

## Rust Trait Definitions (Interface-Based Design)

Following ColaNode's architecture assessment, we define clear interfaces:

### Domain Layer Traits

```rust
// crates/dataforge-core/src/sync/traits.rs

/// Repository interface for sync state
pub trait SyncStateRepository: Send + Sync {
    fn get_state(&self, workspace_id: &Uuid) -> Result<Option<SyncState>>;
    fn save_state(&self, state: &SyncState) -> Result<()>;
    fn update_last_sync(&self, workspace_id: &Uuid, version: i64) -> Result<()>;
}

/// Repository interface for sync queue
pub trait SyncQueueRepository: Send + Sync {
    fn enqueue(&self, entry: SyncQueueEntry) -> Result<()>;
    fn get_pending(&self, workspace_id: &Uuid, limit: usize) -> Result<Vec<SyncQueueEntry>>;
    fn mark_synced(&self, ids: &[i64]) -> Result<()>;
    fn mark_failed(&self, id: i64, error: &str) -> Result<()>;
    fn clear_synced(&self, workspace_id: &Uuid) -> Result<()>;
}

/// Repository interface for conflict management
pub trait ConflictRepository: Send + Sync {
    fn save_conflict(&self, conflict: SyncConflict) -> Result<i64>;
    fn get_pending(&self, workspace_id: &Uuid) -> Result<Vec<SyncConflict>>;
    fn resolve(&self, id: i64, resolution: ConflictResolution) -> Result<()>;
}

/// Sync client interface (for testing/mocking)
pub trait SyncClient: Send + Sync {
    async fn push(&self, request: PushRequest) -> Result<PushResponse>;
    async fn pull(&self, request: PullRequest) -> Result<PullResponse>;
    async fn get_blob_urls(&self, hashes: &[String]) -> Result<Vec<BlobUrl>>;
    async fn health_check(&self) -> Result<bool>;
}

/// Blob storage interface
pub trait BlobStorage: Send + Sync {
    async fn store(&self, data: &[u8]) -> Result<String>;  // Returns hash
    async fn get(&self, hash: &str) -> Result<Vec<u8>>;
    async fn exists(&self, hash: &str) -> Result<bool>;
    async fn delete(&self, hash: &str) -> Result<()>;
}
```

### Application Layer Services

```rust
// crates/dataforge-core/src/sync/service.rs

/// Main sync orchestration service
pub struct SyncService<S, Q, C, B>
where
    S: SyncStateRepository,
    Q: SyncQueueRepository,
    C: SyncClient,
    B: BlobStorage,
{
    state_repo: S,
    queue_repo: Q,
    client: C,
    blob_storage: B,
    conflict_strategy: ConflictStrategy,
}

impl<S, Q, C, B> SyncService<S, Q, C, B>
where
    S: SyncStateRepository,
    Q: SyncQueueRepository,
    C: SyncClient,
    B: BlobStorage,
{
    /// Push local changes to server
    pub async fn push(&self, workspace_id: &Uuid) -> Result<SyncResult> {
        // 1. Get pending changes from queue
        // 2. Batch into push request
        // 3. Upload missing blobs
        // 4. Send push request
        // 5. Mark as synced or handle errors
        todo!()
    }

    /// Pull remote changes from server
    pub async fn pull(&self, workspace_id: &Uuid) -> Result<SyncResult> {
        // 1. Get current sync state
        // 2. Request changes since last_sync_version
        // 3. Download missing blobs
        // 4. Apply changes with conflict detection
        // 5. Update sync state
        todo!()
    }

    /// Full sync (push then pull)
    pub async fn sync(&self, workspace_id: &Uuid) -> Result<SyncResult> {
        let push_result = self.push(workspace_id).await?;
        let pull_result = self.pull(workspace_id).await?;
        Ok(SyncResult::merge(push_result, pull_result))
    }
}
```

## Frontend Sync Store (Svelte 5 Runes)

```typescript
// src/lib/stores/sync.svelte.ts

import { invoke } from '@tauri-apps/api/core';

// Sync state types
type SyncStatus = 'idle' | 'syncing' | 'error' | 'offline';

interface SyncState {
    status: SyncStatus;
    lastSyncAt: string | null;
    pendingChanges: number;
    conflicts: number;
    error: string | null;
}

// Reactive state
let syncStatus = $state<SyncStatus>('idle');
let lastSyncAt = $state<string | null>(null);
let pendingChanges = $state(0);
let conflicts = $state(0);
let error = $state<string | null>(null);
let isOnline = $state(navigator.onLine);

// Derived state
const canSync = $derived(isOnline && syncStatus !== 'syncing');
const needsSync = $derived(pendingChanges > 0);
const hasConflicts = $derived(conflicts > 0);

// Initialize sync state
async function initialize(workspaceId: string): Promise<void> {
    try {
        const state = await invoke<SyncState>('get_sync_state', { workspaceId });
        syncStatus = state.status;
        lastSyncAt = state.lastSyncAt;
        pendingChanges = state.pendingChanges;
        conflicts = state.conflicts;
    } catch (e) {
        console.error('Failed to initialize sync state:', e);
        error = e instanceof Error ? e.message : String(e);
    }
}

// Trigger manual sync
async function sync(workspaceId: string): Promise<boolean> {
    if (!canSync) return false;

    try {
        syncStatus = 'syncing';
        error = null;

        const result = await invoke<SyncResult>('sync_workspace', { workspaceId });

        syncStatus = 'idle';
        lastSyncAt = new Date().toISOString();
        pendingChanges = result.remainingPending;
        conflicts = result.conflicts;

        return result.success;
    } catch (e) {
        console.error('Sync failed:', e);
        syncStatus = 'error';
        error = e instanceof Error ? e.message : String(e);
        return false;
    }
}

// Listen for online/offline events
if (typeof window !== 'undefined') {
    window.addEventListener('online', () => {
        isOnline = true;
        // Auto-sync when coming online
        // syncStore.sync(currentWorkspaceId);
    });

    window.addEventListener('offline', () => {
        isOnline = false;
        syncStatus = 'offline';
    });
}

// Export store
export const syncStore = {
    get status() { return syncStatus; },
    get lastSyncAt() { return lastSyncAt; },
    get pendingChanges() { return pendingChanges; },
    get conflicts() { return conflicts; },
    get error() { return error; },
    get isOnline() { return isOnline; },
    get canSync() { return canSync; },
    get needsSync() { return needsSync; },
    get hasConflicts() { return hasConflicts; },

    initialize,
    sync,
};
```

## Tauri Commands for Sync

```rust
// src-tauri/src/commands/sync.rs

/// Get current sync state for a workspace
#[tauri::command]
pub fn get_sync_state(
    workspace_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<SyncStateResponse, String> {
    // Implementation
}

/// Trigger sync for a workspace
#[tauri::command]
pub async fn sync_workspace(
    workspace_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<SyncResult, String> {
    // Implementation
}

/// Get pending conflicts for resolution
#[tauri::command]
pub fn get_conflicts(
    workspace_id: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<SyncConflict>, String> {
    // Implementation
}

/// Resolve a conflict
#[tauri::command]
pub fn resolve_conflict(
    conflict_id: i64,
    resolution: String,  // 'local' | 'remote'
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    // Implementation
}

/// Configure sync server for workspace
#[tauri::command]
pub fn set_sync_server(
    workspace_id: String,
    server_url: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    // Implementation
}
```

## Testing Strategy (Change Safety)

Following the quality checklist, tests should encode behavior, not implementation:

### Unit Tests (Domain Layer)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Mock repositories for testing
    struct MockSyncStateRepo { /* ... */ }
    struct MockSyncQueueRepo { /* ... */ }
    struct MockSyncClient { /* ... */ }
    struct MockBlobStorage { /* ... */ }

    #[test]
    fn test_local_change_queued_for_sync() {
        // Test that local changes are queued
        let queue_repo = MockSyncQueueRepo::new();
        let service = create_test_service(queue_repo);

        service.record_change(well_change);

        assert_eq!(queue_repo.pending_count(), 1);
    }

    #[test]
    fn test_conflict_detected_when_versions_differ() {
        // Test conflict detection behavior
        let service = create_test_service_with_local_version(5);
        let remote_change = create_change_with_version(4);

        let result = service.apply_remote_change(remote_change);

        assert!(result.is_conflict());
    }

    #[test]
    fn test_blobs_uploaded_before_metadata() {
        // Test that blobs are uploaded first
        let client = MockSyncClient::new();
        let service = create_test_service(client);

        service.push(workspace_id).await;

        assert!(client.blob_uploads_before_push());
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_full_sync_cycle() {
    // Test complete push-pull cycle with real SQLite
    let db = create_test_db();
    let service = create_service_with_db(db);

    // Create local change
    service.create_well(well_data).await?;

    // Push to server
    let push_result = service.push(workspace_id).await?;
    assert!(push_result.success);

    // Simulate remote change
    mock_server.add_remote_change(remote_well);

    // Pull changes
    let pull_result = service.pull(workspace_id).await?;
    assert_eq!(pull_result.applied_changes, 1);
}
```

## File Structure

```
crates/
├── dataforge-core/
│   └── src/
│       ├── sync/
│       │   ├── mod.rs           # Module exports
│       │   ├── traits.rs        # Interface definitions
│       │   ├── models.rs        # SyncChange, SyncState, etc.
│       │   ├── service.rs       # SyncService implementation
│       │   ├── queue.rs         # Queue management
│       │   └── conflict.rs      # Conflict resolution
│       └── lib.rs
├── dataforge-sync/
│   └── src/
│       ├── client.rs            # HTTP sync client
│       ├── protocol.rs          # Wire protocol types
│       └── lib.rs
└── dataforge-storage/
    └── src/
        ├── blob.rs              # BlobStorage trait impl
        └── lib.rs

src-tauri/
└── src/
    ├── commands/
    │   ├── mod.rs
    │   ├── auth.rs
    │   └── sync.rs              # Sync-related commands
    └── lib.rs

src/
└── lib/
    └── stores/
        ├── auth.svelte.ts
        └── sync.svelte.ts       # Sync state store
```

## Implementation Phases

### Phase 1: Core Infrastructure ✅ COMPLETED
- [x] Add sync-related columns to database schema (`sync_state`, `sync_queue`, `sync_conflicts` tables)
- [x] Implement SyncStateRepository trait (`SqliteSyncStateRepo`)
- [x] Implement SyncQueueRepository trait (`SqliteSyncQueueRepo`)
- [x] Implement ConflictRepository trait (`SqliteConflictRepo`)
- [x] Create Tauri sync commands (`sync_commands.rs`)
- [x] Create frontend sync store (`sync.svelte.ts`)

### Phase 1.5: Workspace Members & Settings UI ✅ COMPLETED
- [x] Add workspace member management functions to `auth.rs`:
  - `get_member_role()` - Get member's role in workspace
  - `get_workspace_members()` - List all members with details
  - `add_workspace_member()` - Add by email
  - `update_member_role()` - Change role (with permission checks)
  - `remove_workspace_member()` - Remove member
  - `leave_workspace()` - Self-removal
- [x] Add `WorkspaceMemberInfo` and `WorkspaceInvite` types to `models.rs`
- [x] Create Tauri commands for member management (`commands.rs`)
- [x] Create members store (`members.svelte.ts`)
- [x] Create settings UI components:
  - `SettingsSidebar.svelte` - Navigation sidebar
  - `InviteUsers.svelte` - Add members by email
  - `UsersList.svelte` - Member list with role management
- [x] Create settings routes: `/settings/users`, `/settings/general`, `/settings/sync`
- [x] Create local test sync server (`crates/dataforge-sync/src/bin/test_server.rs`)

### Phase 2: Push/Pull Flow ✅ COMPLETED
- [x] Create test server with push endpoint (`POST /api/sync/push`)
- [x] Create test server with pull endpoint (`POST /api/sync/pull`)
- [x] Implement `sync_workspace` command with actual HTTP sync:
  - Push pending changes from sync_queue to server
  - Pull changes from server
  - Update sync state with server version
  - Handle errors and conflicts gracefully
- [x] Implement `test_sync_connection` command for health checks
- [x] Add "Test Connection" button to sync settings UI
- [x] Update frontend sync store with `testConnection()` action
- [x] Restructure async code to avoid holding DB connection across await points
- [ ] Implement blob upload via presigned URLs (future)
- [ ] Add retry logic with exponential backoff (future)

### Phase 3: Change Application ✅ COMPLETED
- [x] Create conflict log table (implemented in Phase 1)
- [x] Apply pulled changes to local database
  - Implemented `apply_pulled_changes()` in `sync_commands.rs`
  - Transactional change application (BEGIN/COMMIT/ROLLBACK)
  - Entity-specific handlers: `apply_well_change()`, `apply_curve_change()`, `apply_curve_metadata_change()`
- [x] Implement blob download
  - Implemented `download_required_blobs()` following Harbor's content-addressed pattern
  - Check for existing blobs (deduplication)
  - Get presigned URLs from server
  - Download with SHA-256 integrity verification
- [x] Add change application with version checking (ColaNode pattern)
  - Query local entity version before applying
  - Conflict detection: `local.version > remote.version` → queue for resolution
  - Skip if already at same version
  - Apply if local version is older
- [x] Handle blob registry updates
  - Register downloaded blobs with size and sync timestamp
  - Mark as `synced_to_server = 1`

### Phase 4: Conflict Resolution ✅ COMPLETED
- [x] Create conflict log table (implemented in Phase 1)
- [x] Build conflict resolution UI in `/settings/sync` page
- [x] Create `ConflictResolutionDialog.svelte` - Modal with side-by-side comparison
  - Parses JSON data to show field-by-field differences
  - Highlights changed fields with visual indicators
  - Shows version numbers and timestamps
  - "Keep Local" / "Keep Remote" buttons with loading states
- [x] Create `SyncStatusIndicator.svelte` - Global sync status in sidebar
  - Visual states: idle (green), syncing (blue spinner), pending (orange), conflict (yellow), error (red), offline (gray)
  - Shows pending changes count and last sync timestamp
  - Click to navigate to sync settings
- [x] Update `AppSidebar.svelte` to include sync status indicator
- [x] Update `(app)/+layout.svelte` to initialize sync on workspace change
- [x] Implement conflict detection during pull
  - Query local entity version/timestamp before applying changes
  - Compare versions: `local.version > remote.version` = potential conflict
  - Apply conflict strategy to determine resolution
- [x] Add auto-resolution strategies
  - `ConflictStrategy` enum in `models.rs`: Manual, LastWriteWins, LocalWins, RemoteWins
  - `conflict_strategy` field added to `SyncState` (persisted in sync_state table)
  - Auto-resolution in `apply_well_change()` and `apply_curve_change()`
  - LastWriteWins compares `updated_at` timestamps
  - LocalWins/RemoteWins apply unconditionally

### Phase 5: Server Implementation
- [x] Create Axum REST API test server (`dataforge-test-server` binary)
- [x] Implement server-side sync endpoints (`/api/sync/push`, `/api/sync/pull`)
- [x] Add blob endpoints (`/api/blobs/urls`, `/api/blobs/:hash`)
- [x] Add presigned URL generation for production blobs (S3/MinIO)
  - Created `storage` module in `dataforge-sync` crate with `BlobStorage` struct
  - S3/MinIO support via OpenDAL (AWS SDK compatible)
  - `presigned_download_url()` for direct GET downloads (Harbor pattern)
  - `presigned_upload_url()` for direct PUT uploads
  - `presigned_download_urls_batch()` for efficient batch operations
  - Content-addressed blob paths: `prefix/sha256/ab/cd/hash`
  - Configurable expiration (default: 1 hour, max: 7 days)
  - Filesystem backend for development/testing
- [x] Add authentication middleware
  - Created `auth` module in `dataforge-sync` crate
  - `AuthConfig` for JWT-like token configuration
  - `AuthToken` with encode/decode/verify methods (HMAC-SHA256)
  - `AuthUser` struct for authenticated user info
  - `auth_middleware` Axum middleware function
  - `ExtractUser` Axum extractor for handler functions
  - `generate_token()` helper for token creation
  - Skip auth option for development (`skip_auth=true`)
- [x] Deploy with Docker
  - `docker/Dockerfile.sync-server` - Multi-stage build for production
  - `docker/docker-compose.yml` - Development setup with MinIO
  - `docker/docker-compose.prod.yml` - Production configuration
  - `docker/.env.example` - Environment variable template
  - Health check endpoints at `/health` and `/api/health`
  - Configurable port via `SYNC_SERVER_PORT` environment variable

## Summary

This sync architecture provides:

1. **Offline-First**: All changes saved locally first, synced when online
2. **Interface-Based**: Clear boundaries between layers for testability
3. **Change Safety**: Localized changes through trait abstractions
4. **Conflict Handling**: Multiple resolution strategies
5. **Enterprise-Ready**: Self-hosted, air-gapped deployment support
6. **ColaNode Patterns**: Server URL configuration, queue-based sync

The architecture follows the quality checklist requirements:
- ✅ Clear separation between domain logic and infrastructure
- ✅ Dependency direction is explicit (traits/interfaces)
- ✅ State ownership is clear (repositories own persistence)
- ✅ External systems isolated behind interfaces
- ✅ System designed for localized change
- ✅ Core domain logic testable without infrastructure
