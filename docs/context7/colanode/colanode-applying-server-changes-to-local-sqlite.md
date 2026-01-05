# Applying Server Changes to Local SQLite: Architecture & Design (Context7 Summary)

This document dives deep into the architecture and design for **applying server-returned changes to local SQLite**, focusing on the INSERT/UPDATE/DELETE operations, transaction handling, conflict detection, and error recovery patterns used in local-first applications like ColaNode and DataForge.

It builds on:

- `colanode-database-request-and-view-architecture.md`
- `colanode-crdt-and-conflict-resolution-architecture.md`
- `colanode-sync-server-configuration.md`

and provides detailed call stacks and implementation patterns for the critical "server → local DB" sync path.

---

## 1. High-Level Architecture Pattern

### 1.1 The Problem

When a server returns changes (from a pull/sync operation), the client must:

1. **Parse** the server response (list of changes).
2. **Detect conflicts** (local version vs. remote version).
3. **Apply changes** to local SQLite (INSERT/UPDATE/DELETE).
4. **Update sync state** (last_sync_version, timestamps).
5. **Handle errors** gracefully (rollback, retry, queue conflicts).

All of this must happen **atomically** (transaction) and **efficiently** (batch operations).

### 1.2 The Solution: Transactional Change Application

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Server Returns Changes                            │
│  GET /api/sync/pull?workspace_id=X&from_version=Y                   │
│  Response: {                                                         │
│    changes: [                                                        │
│      {entity_type: "well", entity_id: "...", action: "create", ...},│
│      {entity_type: "well", entity_id: "...", action: "update", ...},│
│      {entity_type: "well", entity_id: "...", action: "delete", ...} │
│    ],                                                                │
│    server_version: 42                                                │
│  }                                                                   │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Change Application Service                                          │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  1. BEGIN TRANSACTION                                         │  │
│  │  2. FOR each change:                                          │  │
│  │     a. Check for conflicts                                    │  │
│  │     b. Apply change (INSERT/UPDATE/DELETE)                    │  │
│  │  3. Update sync_state (last_sync_version)                     │  │
│  │  4. COMMIT (or ROLLBACK on error)                             │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. Server Response Format

### 2.1 Pull Response Structure

```typescript
// Server response from /api/sync/pull
interface PullResponse {
	changes: SyncChange[];
	server_version: number;
	missing_blobs?: string[]; // Blob hashes that need to be downloaded
}

interface SyncChange {
	id: string; // UUID
	entity_type: string; // "well", "curve", "workspace", etc.
	entity_id: string; // UUID
	action: 'create' | 'update' | 'delete';
	version: number; // Server-assigned version
	timestamp: string; // ISO 8601 timestamp
	user_id: string; // UUID of user who made the change
	data?: string; // JSON payload (for create/update)
}
```

### 2.2 Example Server Response

```json
{
	"changes": [
		{
			"id": "550e8400-e29b-41d4-a716-446655440000",
			"entity_type": "well",
			"entity_id": "123e4567-e89b-12d3-a456-426614174000",
			"action": "create",
			"version": 5,
			"timestamp": "2025-01-15T10:30:00Z",
			"user_id": "789e0123-e45b-67c8-d901-234567890abc",
			"data": "{\"name\": \"Well-001\", \"uwi\": \"1234567890\", \"workspace_id\": \"...\"}"
		},
		{
			"id": "660e8400-e29b-41d4-a716-446655440001",
			"entity_type": "well",
			"entity_id": "123e4567-e89b-12d3-a456-426614174001",
			"action": "update",
			"version": 3,
			"timestamp": "2025-01-15T10:31:00Z",
			"user_id": "789e0123-e45b-67c8-d901-234567890abc",
			"data": "{\"name\": \"Well-002-Updated\", \"field\": \"Eagle Ford\"}"
		},
		{
			"id": "770e8400-e29b-41d4-a716-446655440002",
			"entity_type": "well",
			"entity_id": "123e4567-e89b-12d3-a456-426614174002",
			"action": "delete",
			"version": 2,
			"timestamp": "2025-01-15T10:32:00Z",
			"user_id": "789e0123-e45b-67c8-d901-234567890abc"
		}
	],
	"server_version": 42,
	"missing_blobs": ["sha256:abc123...", "sha256:def456..."]
}
```

---

## 3. Complete Call Stack: Applying Server Changes

### 3.1 High-Level Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│  1. Receive Server Response                                         │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  const response = await syncClient.pull(workspaceId, fromVersion);│
│  │  // response.changes = [SyncChange, ...]                     │  │
│  │  // response.server_version = 42                             │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. Open Database Connection & Begin Transaction                    │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  const db = await openDatabase(dbPath);                      │  │
│  │  await db.exec("BEGIN TRANSACTION");                         │  │
│  │  // All changes applied atomically                            │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. Process Each Change                                             │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  FOR each change in response.changes:                        │  │
│  │    a. Check for conflicts                                    │  │
│  │    b. Apply change based on action                           │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ├─────────────────────┬─────────────────────┐
                          ▼                     ▼                     ▼
        ┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐
        │  3a. INSERT (create) │  │  3b. UPDATE (update) │  │  3c. DELETE (delete) │
        │  - Parse JSON data   │  │  - Parse JSON data   │  │  - Check exists      │
        │  - INSERT INTO table │  │  - Check version     │  │  - DELETE FROM table │
        │  - Set version       │  │  - UPDATE table      │  │  - Update version    │
        └──────────────────────┘  └──────────────────────┘  └──────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  4. Update Sync State                                               │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  UPDATE sync_state                                           │  │
│  │  SET last_sync_version = response.server_version,            │  │
│  │      last_pull_at = NOW(),                                   │  │
│  │      updated_at = NOW()                                      │  │
│  │  WHERE workspace_id = ?                                      │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  5. Commit Transaction (or Rollback on Error)                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  IF all changes applied successfully:                        │  │
│  │    await db.exec("COMMIT");                                  │  │
│  │  ELSE:                                                       │  │
│  │    await db.exec("ROLLBACK");                                │  │
│  │    // Queue conflicts for manual resolution                  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 4. Conflict Detection

### 4.1 Version-Based Conflict Detection

Before applying a change, check if a local version exists and compare versions:

```sql
-- Check for conflict: local version > remote version
SELECT version, updated_at
FROM wells
WHERE id = ? AND workspace_id = ?
```

**Conflict Detection Logic**:

```typescript
function detectConflict(
	localVersion: number | null,
	remoteVersion: number,
	localUpdatedAt: Date | null,
	remoteUpdatedAt: Date
): ConflictType {
	// No local version = no conflict (new entity)
	if (localVersion === null) {
		return ConflictType.None;
	}

	// Local version > remote version = conflict (local is newer)
	if (localVersion > remoteVersion) {
		return ConflictType.LocalNewer;
	}

	// Local version < remote version = no conflict (remote is newer, apply it)
	if (localVersion < remoteVersion) {
		return ConflictType.None;
	}

	// Same version but different timestamps = potential conflict
	if (localVersion === remoteVersion && localUpdatedAt && localUpdatedAt > remoteUpdatedAt) {
		return ConflictType.SameVersionDifferentTime;
	}

	// Same version, same timestamp = already synced (skip)
	return ConflictType.AlreadySynced;
}
```

### 4.2 Conflict Resolution Strategies

**Strategy 1: Last-Write-Wins (Default)**:

- Compare `updated_at` timestamps.
- Keep the newer change, discard the older.
- Log discarded change for audit.

**Strategy 2: Local-Wins**:

- Always keep local changes.
- Push local version to server on next sync.

**Strategy 3: Remote-Wins**:

- Always accept remote changes.
- Discard local changes.

**Strategy 4: Manual Resolution**:

- Queue conflict for user review.
- Show both versions in UI.
- User chooses which to keep.

---

## 5. Applying Changes: INSERT Operations

### 5.1 INSERT Call Stack

```
┌─────────────────────────────────────────────────────────────────────┐
│  Change: {action: "create", entity_type: "well", data: {...}}       │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. Parse JSON Data                                                 │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  const wellData = JSON.parse(change.data);                   │  │
│  │  // {name: "Well-001", uwi: "1234567890", ...}              │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. Check if Entity Already Exists                                  │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  SELECT id, version FROM wells                               │  │
│  │  WHERE id = ? AND workspace_id = ?                           │  │
│  │                                                               │  │
│  │  IF exists:                                                  │  │
│  │    - This is an UPDATE, not INSERT (handle as update)        │  │
│  │  ELSE:                                                       │  │
│  │    - Proceed with INSERT                                     │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. Insert Entity                                                   │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  INSERT INTO wells (                                         │  │
│  │    id, workspace_id, name, uwi, field, company,              │  │
│  │    x, y, created_by, created_at, updated_at, version         │  │
│  │  ) VALUES (                                                  │  │
│  │    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?                        │  │
│  │  )                                                           │  │
│  │                                                               │  │
│  │  Parameters:                                                 │  │
│  │  - id: change.entity_id (UUID)                               │  │
│  │  - workspace_id: current workspace                           │  │
│  │  - name: wellData.name                                       │  │
│  │  - uwi: wellData.uwi                                         │  │
│  │  - version: change.version (server version)                  │  │
│  │  - created_at: change.timestamp                              │  │
│  │  - updated_at: change.timestamp                              │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 5.2 INSERT SQL Example

```sql
-- Insert new well from server change
INSERT INTO wells (
    id,
    workspace_id,
    name,
    uwi,
    field,
    company,
    x,
    y,
    created_by,
    created_at,
    updated_at,
    version,
    deleted_at
) VALUES (
    '123e4567-e89b-12d3-a456-426614174000',  -- entity_id from change
    'workspace-uuid',                         -- current workspace
    'Well-001',                               -- from change.data
    '1234567890',                             -- from change.data
    'Eagle Ford',                             -- from change.data
    'Acme Corp',                              -- from change.data
    100.5,                                    -- from change.data
    200.3,                                    -- from change.data
    '789e0123-e45b-67c8-d901-234567890abc',  -- change.user_id
    '2025-01-15T10:30:00Z',                   -- change.timestamp
    '2025-01-15T10:30:00Z',                   -- change.timestamp
    5,                                        -- change.version (server version)
    NULL                                      -- not deleted
);
```

### 5.3 INSERT with Conflict Handling

```typescript
async function applyInsert(
	db: Database,
	change: SyncChange,
	workspaceId: string
): Promise<ApplyResult> {
	// Parse JSON data
	const entityData = JSON.parse(change.data || '{}');

	// Check if entity already exists
	const existing = await db.get(
		'SELECT id, version, updated_at FROM wells WHERE id = ? AND workspace_id = ?',
		[change.entity_id, workspaceId]
	);

	if (existing) {
		// Entity exists - this is actually an UPDATE, not INSERT
		// Check for conflict
		if (existing.version > change.version) {
			// Local version is newer - conflict
			return {
				success: false,
				conflict: {
					entity_type: change.entity_type,
					entity_id: change.entity_id,
					local_version: existing.version,
					remote_version: change.version,
					local_data: JSON.stringify(existing),
					remote_data: change.data || ''
				}
			};
		}

		// Remote version is newer or equal - apply as UPDATE
		return await applyUpdate(db, change, workspaceId);
	}

	// Entity doesn't exist - proceed with INSERT
	try {
		await db.run(
			`INSERT INTO wells (
        id, workspace_id, name, uwi, field, company,
        x, y, created_by, created_at, updated_at, version
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
			[
				change.entity_id,
				workspaceId,
				entityData.name,
				entityData.uwi,
				entityData.field,
				entityData.company,
				entityData.x,
				entityData.y,
				change.user_id,
				change.timestamp,
				change.timestamp,
				change.version // Use server version
			]
		);

		return { success: true };
	} catch (error) {
		// Handle constraint violations, etc.
		return {
			success: false,
			error: error.message
		};
	}
}
```

---

## 6. Applying Changes: UPDATE Operations

### 6.1 UPDATE Call Stack

```
┌─────────────────────────────────────────────────────────────────────┐
│  Change: {action: "update", entity_type: "well", data: {...}}       │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. Check if Entity Exists                                          │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  SELECT id, version, updated_at, * FROM wells                │  │
│  │  WHERE id = ? AND workspace_id = ?                           │  │
│  │                                                               │  │
│  │  IF not exists:                                              │  │
│  │    - This is actually an INSERT (handle as insert)           │  │
│  │  ELSE:                                                       │  │
│  │    - Proceed with UPDATE                                     │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. Detect Conflict                                                 │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  IF local.version > remote.version:                          │  │
│  │    - Conflict: local is newer                                │  │
│  │    - Queue for resolution                                    │  │
│  │    - Skip UPDATE                                             │  │
│  │  ELSE IF local.version === remote.version:                   │  │
│  │    - Already synced (skip)                                   │  │
│  │  ELSE:                                                       │  │
│  │    - Remote is newer (proceed with UPDATE)                   │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. Parse JSON Data & Merge with Existing                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  const updateData = JSON.parse(change.data);                 │  │
│  │  // Merge with existing entity (partial update)              │  │
│  │  const merged = { ...existing, ...updateData };              │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  4. Update Entity                                                   │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  UPDATE wells SET                                            │  │
│  │    name = ?,                                                 │  │
│  │    uwi = ?,                                                  │  │
│  │    field = ?,                                                │  │
│  │    updated_at = ?,                                           │  │
│  │    version = ?                                               │  │
│  │  WHERE id = ? AND workspace_id = ?                           │  │
│  │                                                               │  │
│  │  Parameters:                                                 │  │
│  │  - name: merged.name                                         │  │
│  │  - version: change.version (server version)                  │  │
│  │  - updated_at: change.timestamp                              │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 6.2 UPDATE SQL Example

```sql
-- Update existing well from server change
UPDATE wells
SET
    name = 'Well-002-Updated',              -- from change.data
    field = 'Eagle Ford',                   -- from change.data
    updated_at = '2025-01-15T10:31:00Z',    -- change.timestamp
    version = 3                             -- change.version (server version)
WHERE
    id = '123e4567-e89b-12d3-a456-426614174001'
    AND workspace_id = 'workspace-uuid';
```

### 6.3 UPDATE with Conflict Handling

```typescript
async function applyUpdate(
	db: Database,
	change: SyncChange,
	workspaceId: string
): Promise<ApplyResult> {
	// Check if entity exists
	const existing = await db.get('SELECT * FROM wells WHERE id = ? AND workspace_id = ?', [
		change.entity_id,
		workspaceId
	]);

	if (!existing) {
		// Entity doesn't exist - this is actually an INSERT
		return await applyInsert(db, change, workspaceId);
	}

	// Detect conflict
	if (existing.version > change.version) {
		// Local version is newer - conflict
		return {
			success: false,
			conflict: {
				entity_type: change.entity_type,
				entity_id: change.entity_id,
				local_version: existing.version,
				remote_version: change.version,
				local_data: JSON.stringify(existing),
				remote_data: change.data || ''
			}
		};
	}

	if (existing.version === change.version) {
		// Already synced - skip
		return { success: true, skipped: true };
	}

	// Parse update data and merge with existing
	const updateData = JSON.parse(change.data || '{}');
	const merged = { ...existing, ...updateData };

	// Apply UPDATE
	try {
		await db.run(
			`UPDATE wells SET
        name = ?,
        uwi = ?,
        field = ?,
        company = ?,
        x = ?,
        y = ?,
        updated_at = ?,
        version = ?
      WHERE id = ? AND workspace_id = ?`,
			[
				merged.name,
				merged.uwi,
				merged.field,
				merged.company,
				merged.x,
				merged.y,
				change.timestamp,
				change.version, // Use server version
				change.entity_id,
				workspaceId
			]
		);

		return { success: true };
	} catch (error) {
		return {
			success: false,
			error: error.message
		};
	}
}
```

---

## 7. Applying Changes: DELETE Operations

### 7.1 DELETE Call Stack

```
┌─────────────────────────────────────────────────────────────────────┐
│  Change: {action: "delete", entity_type: "well"}                    │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. Check if Entity Exists                                          │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  SELECT id, version, deleted_at FROM wells                   │  │
│  │  WHERE id = ? AND workspace_id = ?                           │  │
│  │                                                               │  │
│  │  IF not exists:                                              │  │
│  │    - Already deleted (skip)                                  │  │
│  │  ELSE:                                                       │  │
│  │    - Proceed with DELETE                                     │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. Detect Conflict                                                 │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  IF local.version > remote.version:                          │  │
│  │    - Conflict: local is newer                                │  │
│  │    - Queue for resolution                                    │  │
│  │    - Skip DELETE                                             │  │
│  │  ELSE IF local.deleted_at IS NOT NULL:                       │  │
│  │    - Already deleted (skip)                                  │  │
│  │  ELSE:                                                       │  │
│  │    - Remote is newer (proceed with DELETE)                   │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. Soft Delete or Hard Delete                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  OPTION A: Soft Delete (recommended)                         │  │
│  │  UPDATE wells SET                                            │  │
│  │    deleted_at = ?,                                           │  │
│  │    version = ?                                               │  │
│  │  WHERE id = ?                                                │  │
│  │                                                               │  │
│  │  OPTION B: Hard Delete                                       │  │
│  │  DELETE FROM wells WHERE id = ?                              │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 7.2 DELETE SQL Example (Soft Delete)

```sql
-- Soft delete well from server change
UPDATE wells
SET
    deleted_at = '2025-01-15T10:32:00Z',    -- change.timestamp
    version = 2                              -- change.version (server version)
WHERE
    id = '123e4567-e89b-12d3-a456-426614174002'
    AND workspace_id = 'workspace-uuid'
    AND deleted_at IS NULL;                  -- Only delete if not already deleted
```

### 7.3 DELETE with Conflict Handling

```typescript
async function applyDelete(
	db: Database,
	change: SyncChange,
	workspaceId: string
): Promise<ApplyResult> {
	// Check if entity exists
	const existing = await db.get(
		'SELECT id, version, deleted_at FROM wells WHERE id = ? AND workspace_id = ?',
		[change.entity_id, workspaceId]
	);

	if (!existing) {
		// Already deleted or never existed - skip
		return { success: true, skipped: true };
	}

	// Check if already soft-deleted
	if (existing.deleted_at) {
		// Already deleted - skip
		return { success: true, skipped: true };
	}

	// Detect conflict
	if (existing.version > change.version) {
		// Local version is newer - conflict
		return {
			success: false,
			conflict: {
				entity_type: change.entity_type,
				entity_id: change.entity_id,
				local_version: existing.version,
				remote_version: change.version,
				local_data: JSON.stringify(existing),
				remote_data: null // DELETE has no data
			}
		};
	}

	// Apply soft delete
	try {
		await db.run(
			`UPDATE wells SET
        deleted_at = ?,
        version = ?
      WHERE id = ? AND workspace_id = ? AND deleted_at IS NULL`,
			[change.timestamp, change.version, change.entity_id, workspaceId]
		);

		return { success: true };
	} catch (error) {
		return {
			success: false,
			error: error.message
		};
	}
}
```

---

## 8. Transaction Handling & Error Recovery

### 8.1 Complete Transaction Flow

```typescript
async function applyServerChanges(
	db: Database,
	changes: SyncChange[],
	serverVersion: number,
	workspaceId: string
): Promise<ApplyResult> {
	// Begin transaction
	await db.exec('BEGIN TRANSACTION');

	const results: ApplyResult[] = [];
	const conflicts: Conflict[] = [];

	try {
		// Process each change
		for (const change of changes) {
			let result: ApplyResult;

			switch (change.action) {
				case 'create':
					result = await applyInsert(db, change, workspaceId);
					break;
				case 'update':
					result = await applyUpdate(db, change, workspaceId);
					break;
				case 'delete':
					result = await applyDelete(db, change, workspaceId);
					break;
				default:
					result = {
						success: false,
						error: `Unknown action: ${change.action}`
					};
			}

			results.push(result);

			// Collect conflicts
			if (!result.success && result.conflict) {
				conflicts.push(result.conflict);
			}
		}

		// If conflicts found, decide on strategy
		if (conflicts.length > 0) {
			// Strategy: Last-write-wins (auto-resolve)
			for (const conflict of conflicts) {
				// Compare timestamps
				const localData = JSON.parse(conflict.local_data);
				const remoteData = JSON.parse(conflict.remote_data || '{}');

				if (localData.updated_at > remoteData.updated_at) {
					// Local wins - skip remote change
					continue;
				} else {
					// Remote wins - apply remote change
					// Re-apply the change (already in results, but we need to force it)
					// This is simplified - in practice, you'd re-run applyUpdate/applyDelete
				}
			}
		}

		// Update sync state
		await db.run(
			`UPDATE sync_state
       SET last_sync_version = ?,
           last_pull_at = ?,
           updated_at = ?
       WHERE workspace_id = ?`,
			[serverVersion, new Date().toISOString(), new Date().toISOString(), workspaceId]
		);

		// Commit transaction
		await db.exec('COMMIT');

		return {
			success: true,
			applied: results.filter((r) => r.success && !r.skipped).length,
			skipped: results.filter((r) => r.skipped).length,
			conflicts: conflicts.length
		};
	} catch (error) {
		// Rollback on any error
		await db.exec('ROLLBACK');

		return {
			success: false,
			error: error.message,
			conflicts: conflicts
		};
	}
}
```

### 8.2 Error Recovery Patterns

**Pattern 1: Rollback on Any Error**:

- If any change fails, rollback entire transaction.
- Queue all changes for retry.
- User can manually resolve conflicts.

**Pattern 2: Partial Success**:

- Apply changes that succeed.
- Queue failed changes for retry.
- Update sync state only if all changes succeed.

**Pattern 3: Conflict Queue**:

- Apply non-conflicting changes.
- Queue conflicts for manual resolution.
- Update sync state with partial progress.

---

## 9. Design Patterns for Change Application

### 9.1 Repository Pattern

```typescript
// Abstract repository interface
interface EntityRepository {
  findById(id: string, workspaceId: string): Promise<Entity | null>;
  insert(entity: Entity): Promise<void>;
  update(id: string, data: Partial<Entity>, version: number): Promise<void>;
  delete(id: string, version: number): Promise<void>;
}

// Well repository implementation
class WellRepository implements EntityRepository {
  constructor(private db: Database) {}

  async findById(id: string, workspaceId: string): Promise<Well | null> {
    return await this.db.get(
      'SELECT * FROM wells WHERE id = ? AND workspace_id = ?',
      [id, workspaceId]
    );
  }

  async insert(well: Well): Promise<void> {
    await this.db.run(
      `INSERT INTO wells (...) VALUES (...)`,
      [well.id, well.name, ...]
    );
  }

  async update(id: string, data: Partial<Well>, version: number): Promise<void> {
    await this.db.run(
      `UPDATE wells SET name = ?, version = ? WHERE id = ?`,
      [data.name, version, id]
    );
  }

  async delete(id: string, version: number): Promise<void> {
    await this.db.run(
      `UPDATE wells SET deleted_at = ?, version = ? WHERE id = ?`,
      [new Date().toISOString(), version, id]
    );
  }
}
```

### 9.2 Change Application Service

```typescript
class ChangeApplicationService {
	constructor(
		private db: Database,
		private repositories: Map<string, EntityRepository>
	) {}

	async applyChanges(
		changes: SyncChange[],
		serverVersion: number,
		workspaceId: string
	): Promise<ApplyResult> {
		await this.db.exec('BEGIN TRANSACTION');

		try {
			const results: ApplyResult[] = [];

			for (const change of changes) {
				const repo = this.repositories.get(change.entity_type);
				if (!repo) {
					results.push({
						success: false,
						error: `Unknown entity type: ${change.entity_type}`
					});
					continue;
				}

				const result = await this.applyChange(repo, change, workspaceId);
				results.push(result);
			}

			// Update sync state
			await this.updateSyncState(workspaceId, serverVersion);

			await this.db.exec('COMMIT');

			return {
				success: true,
				applied: results.filter((r) => r.success).length,
				conflicts: results.filter((r) => r.conflict).length
			};
		} catch (error) {
			await this.db.exec('ROLLBACK');
			throw error;
		}
	}

	private async applyChange(
		repo: EntityRepository,
		change: SyncChange,
		workspaceId: string
	): Promise<ApplyResult> {
		const existing = await repo.findById(change.entity_id, workspaceId);

		// Conflict detection
		if (existing && existing.version > change.version) {
			return {
				success: false,
				conflict: {
					entity_type: change.entity_type,
					entity_id: change.entity_id,
					local_version: existing.version,
					remote_version: change.version,
					local_data: JSON.stringify(existing),
					remote_data: change.data || ''
				}
			};
		}

		// Apply change
		switch (change.action) {
			case 'create':
				if (!existing) {
					const entity = JSON.parse(change.data || '{}');
					await repo.insert({ ...entity, version: change.version });
				}
				break;
			case 'update':
				if (existing) {
					const updateData = JSON.parse(change.data || '{}');
					await repo.update(change.entity_id, updateData, change.version);
				}
				break;
			case 'delete':
				if (existing && !existing.deleted_at) {
					await repo.delete(change.entity_id, change.version);
				}
				break;
		}

		return { success: true };
	}
}
```

---

## 10. Summary

### ✅ Key Patterns

1. **Transactional Change Application**: All changes applied atomically in a single transaction.
2. **Version-Based Conflict Detection**: Compare local vs. remote versions before applying.
3. **Action-Based Routing**: Route to INSERT/UPDATE/DELETE based on `change.action`.
4. **Soft Deletes**: Use `deleted_at` timestamp instead of hard deletes.
5. **Error Recovery**: Rollback on error, queue conflicts for resolution.

### 📋 Best Practices

1. **Always use transactions** for batch change application.
2. **Check for conflicts** before applying changes.
3. **Use server-assigned versions** (don't increment locally).
4. **Support partial updates** (merge with existing data).
5. **Log conflicts** for audit and manual resolution.
6. **Handle edge cases** (entity doesn't exist, already deleted, etc.).

This architecture provides a robust foundation for applying server changes to local SQLite while maintaining data consistency and handling conflicts gracefully.

