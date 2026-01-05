# ColaNode Conflict Detection During Pull: Architecture & Design (Context7 Summary)

This document dives deep into **ColaNode's architecture and design for conflict detection during pull operations** (server to client). It focuses on how conflicts are detected when pulling changes from the server, the complete call stacks, system design patterns, and how conflict detection integrates into the pull workflow.

It builds on:

- `colanode-applying-server-changes-to-local-sqlite.md`
- `colanode-version-checking-and-conflict-detection.md`
- `colanode-auto-resolution-strategies.md`

and provides detailed call stacks and implementation patterns specifically for conflict detection during pull operations.

---

## 1. High-Level Architecture: Pull Operation with Conflict Detection

### 1.1 The Pull Operation Flow

ColaNode's pull operation retrieves changes from the server and applies them to local SQLite, with conflict detection happening **before any data modification**:

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Pull Operation with Conflict Detection            │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 1: Initiate Pull Request                              │  │
│  │  - GET /api/sync/pull?workspace_id=X&from_version=Y         │  │
│  │  - Server returns list of changes since from_version         │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 2: Begin Transaction                                  │  │
│  │  - Open database connection                                  │  │
│  │  - BEGIN TRANSACTION                                         │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 3: Process Each Change (WITH CONFLICT DETECTION)      │  │
│  │  FOR each change in response.changes:                        │  │
│  │    a. Query local entity (get current version)               │  │
│  │    b. DETECT CONFLICT (compare versions)                     │  │
│  │    c. Handle conflict OR apply change                        │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 4: Update Sync State                                  │  │
│  │  - Update last_sync_version                                  │  │
│  │  - Update last_pull_at timestamp                             │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 5: Commit Transaction                                 │  │
│  │  - COMMIT (or ROLLBACK on error)                             │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 1.2 Conflict Detection Integration Point

Conflict detection happens **immediately after querying the local entity** and **before any data modification**:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Conflict Detection Integration in Pull Flow                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Pull Change → Query Local Entity → CONFLICT DETECTION → Apply/Skip │
│                                                                      │
│  The conflict detection is a GUARD that prevents invalid applies    │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. Pull Request & Response Format

### 2.1 Pull Request

```typescript
interface PullRequest {
	workspace_id: string;
	from_version: number; // Last known server version
	entity_types?: string[]; // Optional: filter by entity types
	limit?: number; // Optional: limit number of changes
}

// Example request
const request: PullRequest = {
	workspace_id: 'workspace-123',
	from_version: 42 // Client's last known server version
};
```

### 2.2 Pull Response

```typescript
interface PullResponse {
	changes: SyncChange[];
	server_version: number; // Current server version
	has_more: boolean; // Whether more changes are available
	missing_blobs?: string[]; // Blob hashes that need to be downloaded
}

interface SyncChange {
	id: string; // UUID of the change
	entity_type: string; // "well", "curve", "workspace", etc.
	entity_id: string; // UUID of the entity
	action: 'create' | 'update' | 'delete';
	version: number; // Server-assigned version
	timestamp: string; // ISO 8601 timestamp
	user_id: string; // UUID of user who made the change
	data?: string; // JSON payload (for create/update)
	blob_hashes?: string[]; // Associated blob hashes
}
```

---

## 3. Complete Pull Call Stack with Conflict Detection

### 3.1 High-Level Pull Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│  Pull Operation: Complete Call Stack                                │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 1: Client Initiates Pull                              │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  const response = await syncClient.pull({             │  │  │
│  │  │    workspace_id: "workspace-123",                     │  │  │
│  │  │    from_version: 42                                   │  │  │
│  │  │  });                                                  │  │  │
│  │  │                                                        │  │  │
│  │  │  HTTP GET /api/sync/pull?workspace_id=...&from_version=42│  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Server processes request
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 2: Server Returns Changes                             │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  Response: {                                           │  │  │
│  │  │    changes: [                                          │  │  │
│  │  │      {                                                 │  │  │
│  │  │        entity_id: "well-123",                          │  │  │
│  │  │        action: "update",                               │  │  │
│  │  │        version: 5,  ← Server-assigned version          │  │  │
│  │  │        timestamp: "2025-01-15T10:30:00Z",              │  │  │
│  │  │        data: "{...}"                                   │  │  │
│  │  │      },                                                │  │  │
│  │  │      ...                                               │  │  │
│  │  │    ],                                                  │  │  │
│  │  │    server_version: 50                                  │  │  │
│  │  │  }                                                     │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Client receives response
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 3: Begin Transaction                                  │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  const db = await openDatabase(dbPath);               │  │  │
│  │  │  await db.exec("BEGIN TRANSACTION");                  │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Process each change
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 4: FOR EACH CHANGE - Conflict Detection              │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  FOR each change in response.changes:                 │  │  │
│  │  │    a. Query local entity                              │  │  │
│  │  │    b. DETECT CONFLICT (version comparison)            │  │  │
│  │  │    c. Handle conflict OR apply change                 │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 5: Update Sync State                                  │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  UPDATE sync_state                                    │  │  │
│  │  │  SET last_sync_version = response.server_version,     │  │  │
│  │  │      last_pull_at = NOW()                             │  │  │
│  │  │  WHERE workspace_id = ?                               │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 6: Commit Transaction                                 │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  IF all changes processed successfully:                │  │  │
│  │  │    await db.exec("COMMIT");                           │  │  │
│  │  │  ELSE:                                                │  │  │
│  │  │    await db.exec("ROLLBACK");                         │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.2 Conflict Detection Call Stack (Detailed)

```
┌─────────────────────────────────────────────────────────────────────┐
│  Conflict Detection During Pull: Detailed Call Stack                │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Change from Server: {action: "update", version: 5, ...}    │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 1: Query Local Entity (BEFORE ANY MODIFICATION)        │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  SELECT id, version, updated_at, deleted_at, *        │  │  │
│  │  │  FROM wells                                           │  │  │
│  │  │  WHERE id = ? AND workspace_id = ?                    │  │  │
│  │  │                                                        │  │  │
│  │  │  Result:                                              │  │  │
│  │  │  - localEntity = {                                    │  │  │
│  │  │      id: "well-123",                                  │  │  │
│  │  │      version: 7,  ← Local version                     │  │  │
│  │  │      updated_at: "2025-01-15T10:35:00Z",             │  │  │
│  │  │      name: "Well-001",                                │  │  │
│  │  │      ...                                              │  │  │
│  │  │    }                                                  │  │  │
│  │  │  OR                                                   │  │  │
│  │  │  - localEntity = null (entity doesn't exist)          │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Extract version information
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 2: Extract Version Information                        │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  const localVersion = localEntity?.version ?? null;   │  │  │
│  │  │  const localUpdatedAt = localEntity?.updated_at       │  │  │
│  │  │    ? new Date(localEntity.updated_at) : null;         │  │  │
│  │  │  const localDeletedAt = localEntity?.deleted_at       │  │  │
│  │  │    ? new Date(localEntity.deleted_at) : null;         │  │  │
│  │  │                                                        │  │  │
│  │  │  const remoteVersion = change.version;  // 5          │  │  │
│  │  │  const remoteUpdatedAt = new Date(change.timestamp);  │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Call conflict detection function
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 3: Conflict Detection Function                        │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  const conflictCheck = detectConflict(                │  │  │
│  │  │    localVersion,      // 7                             │  │  │
│  │  │    remoteVersion,     // 5                             │  │  │
│  │  │    localUpdatedAt,    // Date object                   │  │  │
│  │  │    remoteUpdatedAt,   // Date object                   │  │  │
│  │  │    localDeletedAt     // null or Date                  │  │  │
│  │  │  );                                                   │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Conflict detection logic executes
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 4: Version Comparison Logic                           │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  IF localVersion === null:                            │  │  │
│  │  │    → No conflict (new entity)                         │  │  │
│  │  │    → Return: {shouldApply: true, shouldQueueConflict: false}│  │  │
│  │  │                                                        │  │  │
│  │  │  IF localVersion > remoteVersion:                     │  │  │
│  │  │    → CONFLICT: Local is newer                         │  │  │
│  │  │    → Return: {shouldApply: false, shouldQueueConflict: true}│  │  │
│  │  │                                                        │  │  │
│  │  │  IF localVersion < remoteVersion:                     │  │  │
│  │  │    → No conflict (remote is newer)                    │  │  │
│  │  │    → Return: {shouldApply: true, shouldQueueConflict: false}│  │  │
│  │  │                                                        │  │  │
│  │  │  IF localVersion === remoteVersion:                   │  │  │
│  │  │    → Already synced (skip)                            │  │  │
│  │  │    → Return: {shouldSkip: true, shouldQueueConflict: false}│  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Decision based on conflict check
│                         ├─────────────────────┬─────────────────────┐
│                         ▼                     ▼                     ▼
│        ┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐
│        │  Conflict Detected:  │  │  Already Synced:    │  │  No Conflict:       │
│        │  - Queue conflict    │  │  - Skip operation   │  │  - Apply change     │
│        │  - Save to DB        │  │  - No DB change     │  │  - Update version   │
│        │  - Continue to next  │  │  - Continue to next │  │  - Continue to next │
│        └──────────────────────┘  └──────────────────────┘  └──────────────────────┘
└─────────────────────────────────────────────────────────────────────┘
```

---

## 4. Pull Service Implementation

### 4.1 Complete Pull Service with Conflict Detection

```typescript
class PullService {
	constructor(
		private db: Database,
		private syncClient: SyncClient,
		private conflictResolver: ConflictResolver
	) {}

	/**
	 * Pull changes from server with conflict detection
	 */
	async pullChanges(workspaceId: string, fromVersion: number): Promise<PullResult> {
		// Step 1: Request changes from server
		const response = await this.syncClient.pull({
			workspace_id: workspaceId,
			from_version: fromVersion
		});

		// Step 2: Begin transaction
		await this.db.exec('BEGIN TRANSACTION');

		const results: ChangeResult[] = [];
		const conflicts: Conflict[] = [];

		try {
			// Step 3: Process each change with conflict detection
			for (const change of response.changes) {
				const result = await this.processChangeWithConflictDetection(workspaceId, change);

				results.push(result);

				if (result.conflict) {
					conflicts.push(result.conflict);
				}
			}

			// Step 4: Update sync state
			await this.updateSyncState(workspaceId, response.server_version);

			// Step 5: Commit transaction
			await this.db.exec('COMMIT');

			return {
				success: true,
				applied: results.filter((r) => r.applied).length,
				skipped: results.filter((r) => r.skipped).length,
				conflicts: conflicts.length,
				server_version: response.server_version
			};
		} catch (error) {
			// Rollback on error
			await this.db.exec('ROLLBACK');
			throw error;
		}
	}

	/**
	 * Process a single change with conflict detection
	 */
	private async processChangeWithConflictDetection(
		workspaceId: string,
		change: SyncChange
	): Promise<ChangeResult> {
		// Step 1: Query local entity (CRITICAL: before any modification)
		const localEntity = await this.queryLocalEntity(
			workspaceId,
			change.entity_type,
			change.entity_id
		);

		// Step 2: Conflict detection
		const conflictCheck = this.detectConflict(localEntity, change);

		// Step 3: Handle conflict or apply change
		if (conflictCheck.shouldQueueConflict) {
			// Conflict detected - queue for resolution
			const conflict = await this.queueConflict(workspaceId, localEntity!, change);

			return {
				applied: false,
				skipped: false,
				conflict: conflict
			};
		}

		if (conflictCheck.shouldSkip) {
			// Already synced - skip
			return {
				applied: false,
				skipped: true,
				conflict: null
			};
		}

		// Step 4: No conflict - apply change
		await this.applyChange(workspaceId, change);

		return {
			applied: true,
			skipped: false,
			conflict: null
		};
	}

	/**
	 * Query local entity for version comparison
	 */
	private async queryLocalEntity(
		workspaceId: string,
		entityType: string,
		entityId: string
	): Promise<LocalEntity | null> {
		const tableName = this.getTableName(entityType);

		const entity = await this.db.get<{
			id: string;
			version: number;
			updated_at: string;
			deleted_at: string | null;
			[key: string]: any;
		}>(
			`SELECT * FROM ${tableName}
       WHERE id = ? AND workspace_id = ?`,
			[entityId, workspaceId]
		);

		if (!entity) {
			return null;
		}

		return {
			id: entity.id,
			version: entity.version,
			updated_at: entity.updated_at ? new Date(entity.updated_at) : null,
			deleted_at: entity.deleted_at ? new Date(entity.deleted_at) : null,
			data: entity
		};
	}

	/**
	 * Detect conflict between local and remote versions
	 */
	private detectConflict(
		localEntity: LocalEntity | null,
		change: SyncChange
	): ConflictDetectionResult {
		// Case 1: Entity doesn't exist locally (new entity)
		if (!localEntity) {
			return {
				conflictType: ConflictType.None,
				shouldApply: true,
				shouldSkip: false,
				shouldQueueConflict: false
			};
		}

		const localVersion = localEntity.version;
		const remoteVersion = change.version;
		const localUpdatedAt = localEntity.updated_at;
		const remoteUpdatedAt = new Date(change.timestamp);

		// Case 2: Local version > Remote version (conflict)
		if (localVersion > remoteVersion) {
			return {
				conflictType: ConflictType.LocalNewer,
				shouldApply: false,
				shouldSkip: false,
				shouldQueueConflict: true
			};
		}

		// Case 3: Local version < Remote version (no conflict, apply)
		if (localVersion < remoteVersion) {
			return {
				conflictType: ConflictType.None,
				shouldApply: true,
				shouldSkip: false,
				shouldQueueConflict: false
			};
		}

		// Case 4: Same version (already synced or edge case)
		if (localVersion === remoteVersion) {
			// Check timestamps for edge cases
			if (
				localUpdatedAt &&
				remoteUpdatedAt &&
				localUpdatedAt.getTime() !== remoteUpdatedAt.getTime()
			) {
				// Same version but different timestamps - potential edge case
				return {
					conflictType: ConflictType.SameVersionDifferentTime,
					shouldApply: false,
					shouldSkip: true,
					shouldQueueConflict: false
				};
			}

			// Same version, same timestamp (or no timestamp) - already synced
			return {
				conflictType: ConflictType.AlreadySynced,
				shouldApply: false,
				shouldSkip: true,
				shouldQueueConflict: false
			};
		}

		// Default: no conflict (shouldn't reach here)
		return {
			conflictType: ConflictType.None,
			shouldApply: true,
			shouldSkip: false,
			shouldQueueConflict: false
		};
	}

	/**
	 * Queue conflict for resolution
	 */
	private async queueConflict(
		workspaceId: string,
		localEntity: LocalEntity,
		change: SyncChange
	): Promise<Conflict> {
		const conflict: Conflict = {
			workspace_id: workspaceId,
			entity_type: change.entity_type,
			entity_id: change.entity_id,
			local_version: localEntity.version,
			remote_version: change.version,
			local_data: JSON.stringify(localEntity.data),
			remote_data: change.data || null,
			resolution: 'pending',
			created_at: new Date().toISOString()
		};

		await this.db.run(
			`INSERT INTO sync_conflicts (
        workspace_id, entity_type, entity_id,
        local_version, local_data,
        remote_version, remote_data,
        resolution, created_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`,
			[
				conflict.workspace_id,
				conflict.entity_type,
				conflict.entity_id,
				conflict.local_version,
				conflict.local_data,
				conflict.remote_version,
				conflict.remote_data,
				conflict.resolution,
				conflict.created_at
			]
		);

		return conflict;
	}

	/**
	 * Apply change to local database
	 */
	private async applyChange(workspaceId: string, change: SyncChange): Promise<void> {
		const tableName = this.getTableName(change.entity_type);

		switch (change.action) {
			case 'create':
				await this.applyInsert(workspaceId, change, tableName);
				break;
			case 'update':
				await this.applyUpdate(workspaceId, change, tableName);
				break;
			case 'delete':
				await this.applyDelete(workspaceId, change, tableName);
				break;
		}
	}

	/**
	 * Update sync state after successful pull
	 */
	private async updateSyncState(workspaceId: string, serverVersion: number): Promise<void> {
		await this.db.run(
			`UPDATE sync_state
       SET last_sync_version = ?,
           last_pull_at = ?,
           updated_at = ?
       WHERE workspace_id = ?`,
			[serverVersion, new Date().toISOString(), new Date().toISOString(), workspaceId]
		);
	}
}
```

---

## 5. Conflict Detection During Pull: Per-Operation Details

### 5.1 CREATE Operation Conflict Detection

```
┌─────────────────────────────────────────────────────────────────────┐
│  CREATE Operation: Conflict Detection Call Stack                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Server Change: {action: "create", entity_id: "well-123"}   │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 1: Query Local Entity                                 │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  SELECT * FROM wells                                   │  │  │
│  │  │  WHERE id = ? AND workspace_id = ?                    │  │  │
│  │  │                                                        │  │  │
│  │  │  Result: localEntity = null (doesn't exist)           │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 2: Conflict Detection                                 │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  IF localEntity === null:                             │  │  │
│  │  │    → No conflict (new entity)                         │  │  │
│  │  │    → shouldApply = true                               │  │  │
│  │  │                                                        │  │  │
│  │  │  ELSE IF localEntity exists:                          │  │  │
│  │  │    → Concurrent create (conflict)                     │  │  │
│  │  │    → Compare versions                                 │  │  │
│  │  │    → Queue conflict if local.version >= remote.version│  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ No conflict detected
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 3: Apply INSERT                                       │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  INSERT INTO wells (...)                               │  │  │
│  │  │  VALUES (..., change.version, ...)                     │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 5.2 UPDATE Operation Conflict Detection

```
┌─────────────────────────────────────────────────────────────────────┐
│  UPDATE Operation: Conflict Detection Call Stack                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Server Change: {action: "update", version: 5, ...}         │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 1: Query Local Entity                                 │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  SELECT version, updated_at, deleted_at, *            │  │  │
│  │  │  FROM wells                                           │  │  │
│  │  │  WHERE id = ? AND workspace_id = ?                    │  │  │
│  │  │                                                        │  │  │
│  │  │  Result:                                              │  │  │
│  │  │  - localVersion: 7                                    │  │  │
│  │  │  - localUpdatedAt: "2025-01-15T10:35:00Z"            │  │  │
│  │  │  - remoteVersion: 5                                   │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 2: Conflict Detection                                 │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  Comparison:                                           │  │  │
│  │  │  IF 7 > 5:                                             │  │  │
│  │  │    → CONFLICT: Local is newer                          │  │  │
│  │  │    → shouldQueueConflict = true                        │  │  │
│  │  │    → shouldApply = false                               │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Conflict detected
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 3: Queue Conflict                                     │  │
│  │  ┌────────────────────────────────────────────────────────┐  │
│  │  │  - Save conflict to sync_conflicts table              │  │  │
│  │  │  - Store local and remote data                        │  │  │
│  │  │  - Mark as 'pending' resolution                       │  │  │
│  │  │  - Skip applying this change                          │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 5.3 DELETE Operation Conflict Detection

```
┌─────────────────────────────────────────────────────────────────────┐
│  DELETE Operation: Conflict Detection Call Stack                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Server Change: {action: "delete", version: 5, ...}         │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 1: Query Local Entity                                 │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  SELECT version, deleted_at FROM wells                │  │  │
│  │  │  WHERE id = ? AND workspace_id = ?                    │  │  │
│  │  │                                                        │  │  │
│  │  │  Result:                                              │  │  │
│  │  │  - localVersion: 7                                    │  │  │
│  │  │  - localDeletedAt: null (not deleted locally)         │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 2: Conflict Detection                                 │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  Comparison:                                           │  │  │
│  │  │  IF 7 > 5:                                             │  │  │
│  │  │    → CONFLICT: Local version is newer                  │  │  │
│  │  │    → Local entity was updated after server delete      │  │  │
│  │  │    → Queue conflict for resolution                     │  │  │
│  │  │                                                        │  │  │
│  │  │  IF localDeletedAt IS NOT NULL:                       │  │  │
│  │  │    → Already deleted locally                           │  │  │
│  │  │    → Check version to see if we should update          │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         │ Conflict detected
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 3: Queue Conflict                                     │  │
│  │  ┌────────────────────────────────────────────────────────┐  │
│  │  │  - Save conflict to sync_conflicts table              │  │  │
│  │  │  - Store local data (entity still exists)             │  │  │
│  │  │  - Store remote action (delete)                       │  │  │
│  │  │  - Mark as 'pending' resolution                       │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 6. Batch Conflict Detection

### 6.1 Efficient Batch Processing

ColaNode processes changes in batches for efficiency:

```typescript
/**
 * Process changes in batches with conflict detection
 */
async function pullChangesBatch(
  db: Database,
  workspaceId: string,
  changes: SyncChange[],
  batchSize: number = 100
): Promise<PullResult> {
  await db.exec('BEGIN TRANSACTION');

  const results: ChangeResult[] = [];
  const conflicts: Conflict[] = [];

  try {
    // Process in batches
    for (let i = 0; i < changes.length; i += batchSize) {
      const batch = changes.slice(i, i + batchSize);

      // Pre-fetch all local entities for this batch
      const localEntities = await this.preFetchLocalEntities(
        workspaceId,
        batch
      );

      // Process batch with pre-fetched entities
      for (const change of batch) {
        const localEntity = localEntities.get(change.entity_id);

        const conflictCheck = this.detectConflict(localEntity, change);

        if (conflictCheck.shouldQueueConflict) {
          const conflict = await this.queueConflict(
            workspaceId,
            localEntity!,
            change
          );
          conflicts.push(conflict);
          results.push({ applied: false, skipped: false, conflict });
        } else if (conflictCheck.shouldSkip) {
          results.push({ applied: false, skipped: true, conflict: null });
        } else {
          await this.applyChange(workspaceId, change);
          results.push({ applied: true, skipped: false, conflict: null });
        }
      }
    }

    await db.exec('COMMIT');

    return {
      success: true,
      applied: results.filter((r) => r.applied).length,
      skipped: results.filter((r) => r.skipped).length,
      conflicts: conflicts.length,
    };
  } catch (error) {
    await db.exec('ROLLBACK');
    throw error;
  }
}

/**
 * Pre-fetch local entities for batch processing
 */
private async preFetchLocalEntities(
  workspaceId: string,
  changes: SyncChange[]
): Promise<Map<string, LocalEntity>> {
  const entityIds = changes.map((c) => c.entity_id);
  const entityTypes = [...new Set(changes.map((c) => c.entity_type))];

  const entities = new Map<string, LocalEntity>();

  // Fetch entities by type (more efficient than per-entity queries)
  for (const entityType of entityTypes) {
    const typeChanges = changes.filter((c) => c.entity_type === entityType);
    const typeIds = typeChanges.map((c) => c.entity_id);
    const tableName = this.getTableName(entityType);

    const rows = await this.db.all<{
      id: string;
      version: number;
      updated_at: string;
      deleted_at: string | null;
      [key: string]: any;
    }>(
      `SELECT * FROM ${tableName}
       WHERE id IN (${typeIds.map(() => '?').join(',')})
       AND workspace_id = ?`,
      [...typeIds, workspaceId]
    );

    for (const row of rows) {
      entities.set(row.id, {
        id: row.id,
        version: row.version,
        updated_at: row.updated_at ? new Date(row.updated_at) : null,
        deleted_at: row.deleted_at ? new Date(row.deleted_at) : null,
        data: row,
      });
    }
  }

  return entities;
}
```

---

## 7. Pull vs Push: Conflict Detection Differences

### 7.1 Conflict Detection During Pull

**Pull Operation** (Server → Client):

- Client receives changes from server
- Client compares **local version** vs **remote version**
- If `local.version > remote.version`: **Conflict** (local is newer)
- Client queues conflict or auto-resolves
- Client applies remote changes if no conflict

### 7.2 Conflict Detection During Push

**Push Operation** (Client → Server):

- Client sends changes to server
- Server compares **server version** vs **client version**
- If `server.version > client.version`: **Conflict** (server is newer)
- Server rejects change or returns conflict
- Client handles server response

### 7.3 Key Differences

```
┌─────────────────────────────────────────────────────────────────────┐
│  Pull vs Push: Conflict Detection Comparison                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  PULL (Server → Client)                                      │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Client initiates                                    │  │  │
│  │  │  - Client receives changes                             │  │  │
│  │  │  - Client detects conflicts locally                    │  │  │
│  │  │  - Client queues conflicts locally                     │  │  │
│  │  │  - Client applies changes locally                      │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  PUSH (Client → Server)                                      │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Client initiates                                    │  │  │
│  │  │  - Client sends changes                                │  │  │
│  │  │  - Server detects conflicts                            │  │  │
│  │  │  - Server returns conflicts in response                │  │  │
│  │  │  - Client handles server response                      │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 8. Pull with Auto-Resolution

### 8.1 Pull with Automatic Conflict Resolution

ColaNode can auto-resolve conflicts during pull:

```typescript
/**
 * Pull changes with automatic conflict resolution
 */
async function pullChangesWithAutoResolution(
	db: Database,
	workspaceId: string,
	fromVersion: number,
	resolutionStrategy: ResolutionStrategy = 'last_write_wins'
): Promise<PullResult> {
	// Step 1: Pull changes
	const response = await syncClient.pull({
		workspace_id: workspaceId,
		from_version: fromVersion
	});

	await db.exec('BEGIN TRANSACTION');

	const results: ChangeResult[] = [];
	const conflicts: Conflict[] = [];

	try {
		for (const change of response.changes) {
			// Step 2: Query local entity
			const localEntity = await queryLocalEntity(db, workspaceId, change);

			// Step 3: Detect conflict
			const conflictCheck = detectConflict(localEntity, change);

			if (conflictCheck.shouldQueueConflict) {
				// Step 4: Auto-resolve conflict
				const resolution = await autoResolveConflict(
					db,
					workspaceId,
					localEntity!,
					change,
					resolutionStrategy
				);

				if (resolution.resolution === 'local') {
					// Keep local - skip remote change
					results.push({ applied: false, skipped: true, conflict: null });
				} else if (resolution.resolution === 'remote') {
					// Apply remote change
					await applyChange(db, workspaceId, change);
					results.push({ applied: true, skipped: false, conflict: null });
				} else {
					// Queue for manual resolution
					const conflict = await queueConflict(db, workspaceId, localEntity!, change);
					conflicts.push(conflict);
					results.push({ applied: false, skipped: false, conflict });
				}
			} else if (conflictCheck.shouldSkip) {
				results.push({ applied: false, skipped: true, conflict: null });
			} else {
				await applyChange(db, workspaceId, change);
				results.push({ applied: true, skipped: false, conflict: null });
			}
		}

		await this.updateSyncState(db, workspaceId, response.server_version);
		await db.exec('COMMIT');

		return {
			success: true,
			applied: results.filter((r) => r.applied).length,
			skipped: results.filter((r) => r.skipped).length,
			conflicts: conflicts.length,
			server_version: response.server_version
		};
	} catch (error) {
		await db.exec('ROLLBACK');
		throw error;
	}
}
```

---

## 9. Design Patterns for DataForge

### 9.1 Pull with Conflict Detection Pattern

**ColaNode Pattern**:

- Query local entity before applying change
- Compare versions to detect conflicts
- Queue conflicts or auto-resolve
- Apply changes only if no conflict

**DataForge Application**:

- Use same pattern for pull operations
- Query local SQLite before applying server changes
- Compare versions to detect conflicts
- Queue conflicts for resolution

### 9.2 Transactional Pull Pattern

**ColaNode Pattern**:

- All changes applied in single transaction
- Conflicts detected and queued within transaction
- Rollback on error, commit on success

**DataForge Application**:

- Use SQLite transactions for atomicity
- Detect conflicts within transaction
- Queue conflicts in same transaction
- Never lose data (conflicts are queued, not discarded)

### 9.3 Batch Processing Pattern

**ColaNode Pattern**:

- Process changes in batches
- Pre-fetch local entities for batch
- Efficient conflict detection

**DataForge Application**:

- Process pull changes in batches
- Pre-fetch local entities for efficiency
- Reduce database queries

---

## 10. Summary

### ✅ Key Patterns

1. **Query Before Apply**: Always query local entity before applying change
2. **Version Comparison**: Compare `local.version` vs `remote.version` to detect conflicts
3. **Conflict Detection Guard**: Conflict detection acts as guard clause preventing invalid applies
4. **Transactional Safety**: All changes applied atomically in transaction
5. **Batch Processing**: Process changes in batches for efficiency

### 📋 Best Practices

1. **Query local entity first** (before any modification)
2. **Compare versions** to detect conflicts
3. **Queue conflicts** (don't discard data)
4. **Apply changes atomically** (use transactions)
5. **Pre-fetch entities** for batch processing
6. **Support auto-resolution** during pull
7. **Log all conflicts** for audit

This architecture provides a robust foundation for conflict detection during pull operations while maintaining data consistency and never losing user data.

