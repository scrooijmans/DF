# ColaNode Version Checking & Conflict Detection During Change Application (Context7 Summary)

This document dives deep into **ColaNode's architecture and design for version checking and conflict detection** when applying server-returned changes to local SQLite. It covers version comparison strategies, conflict detection algorithms, and how conflicts are handled during INSERT/UPDATE/DELETE operations.

It builds on:

- `colanode-applying-server-changes-to-local-sqlite.md`
- `colanode-crdt-and-conflict-resolution-architecture.md`

and provides detailed call stacks and implementation patterns for version-based conflict detection.

---

## 1. High-Level Architecture: Version-Based Conflict Detection

### 1.1 The Problem

When applying server changes to local SQLite, conflicts can occur when:

- **Local version > Remote version**: Local change is newer (conflict).
- **Local version = Remote version**: Already synced (skip).
- **Local version < Remote version**: Remote is newer (apply).
- **Same version, different timestamps**: Potential conflict (edge case).

ColaNode must:

- **Check versions** before applying changes.
- **Detect conflicts** early (before modifying data).
- **Handle conflicts** gracefully (queue, auto-resolve, or manual).
- **Maintain consistency** (never lose data).

### 1.2 The Solution: Version Comparison Before Apply

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Server Returns Change                             │
│  {entity_id: "...", action: "update", version: 5, data: {...}}      │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ Query local entity
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Check Local Version                                                 │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  SELECT version, updated_at FROM wells                       │  │
│  │  WHERE id = ? AND workspace_id = ?                           │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Compare versions
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Version Comparison & Conflict Detection                            │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  IF local.version > remote.version:                          │  │
│  │    → CONFLICT: Local is newer                                │  │
│  │  ELSE IF local.version === remote.version:                   │  │
│  │    → SKIP: Already synced                                    │  │
│  │  ELSE:                                                       │  │
│  │    → APPLY: Remote is newer                                  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ├─────────────────────┬─────────────────────┐
                          ▼                     ▼                     ▼
        ┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐
        │  Conflict Detected:  │  │  Already Synced:    │  │  Apply Change:      │
        │  - Queue conflict    │  │  - Skip operation   │  │  - UPDATE/INSERT/   │
        │  - Log for audit     │  │  - No action needed │  │    DELETE           │
        │  - Manual resolution │  │  - Continue to next │  │  - Update version   │
        │    or auto-resolve   │  │    change           │  │  - Continue to next │
        └──────────────────────┘  └──────────────────────┘  └──────────────────────┘
```

---

## 2. Version Schema & Tracking

### 2.1 Entity Version Fields

ColaNode tracks versions at the entity level:

```sql
-- Example: wells table with version tracking
CREATE TABLE wells (
    id TEXT PRIMARY KEY,
    workspace_id TEXT NOT NULL,
    name TEXT NOT NULL,
    -- ... other fields ...

    -- Version tracking
    version INTEGER NOT NULL DEFAULT 1,
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),

    -- Soft delete support
    deleted_at TEXT
);

-- Index for efficient version queries
CREATE INDEX idx_wells_version ON wells(version);
CREATE INDEX idx_wells_updated_at ON wells(updated_at);
```

### 2.2 Version Assignment

**Server assigns versions**:

- When an entity is created: `version = 1`.
- When an entity is updated: `version = version + 1`.
- Versions are **monotonically increasing** (never decrease).
- Versions are **server-assigned** (clients use server versions).

**Client tracks local versions**:

- Local SQLite stores the **server-assigned version**.
- When applying server changes, use the **server version** from the change.
- Never increment versions locally (server is authoritative).

---

## 3. Conflict Detection Algorithm

### 3.1 Complete Conflict Detection Logic

```typescript
enum ConflictType {
	None = 'none', // No conflict, apply change
	LocalNewer = 'local_newer', // Local version > remote version
	SameVersionDifferentTime = 'same_version_different_time', // Edge case
	AlreadySynced = 'already_synced' // Same version, already applied
}

interface ConflictDetectionResult {
	conflictType: ConflictType;
	shouldApply: boolean;
	shouldSkip: boolean;
	shouldQueueConflict: boolean;
}

function detectConflict(
	localVersion: number | null,
	remoteVersion: number,
	localUpdatedAt: Date | null,
	remoteUpdatedAt: Date,
	localDeletedAt: Date | null
): ConflictDetectionResult {
	// Case 1: Entity doesn't exist locally (new entity)
	if (localVersion === null) {
		return {
			conflictType: ConflictType.None,
			shouldApply: true,
			shouldSkip: false,
			shouldQueueConflict: false
		};
	}

	// Case 2: Local version > Remote version (local is newer)
	if (localVersion > remoteVersion) {
		return {
			conflictType: ConflictType.LocalNewer,
			shouldApply: false,
			shouldSkip: false,
			shouldQueueConflict: true // Queue for resolution
		};
	}

	// Case 3: Local version < Remote version (remote is newer)
	if (localVersion < remoteVersion) {
		return {
			conflictType: ConflictType.None,
			shouldApply: true,
			shouldSkip: false,
			shouldQueueConflict: false
		};
	}

	// Case 4: Same version (already synced or concurrent edit)
	if (localVersion === remoteVersion) {
		// Check if timestamps differ (edge case: same version, different time)
		if (
			localUpdatedAt &&
			remoteUpdatedAt &&
			localUpdatedAt.getTime() !== remoteUpdatedAt.getTime()
		) {
			// Same version but different timestamps - potential conflict
			// This can happen if:
			// - Server clock skew
			// - Concurrent edits that resulted in same version
			// - Replay of same change
			return {
				conflictType: ConflictType.SameVersionDifferentTime,
				shouldApply: false, // Don't apply (already have this version)
				shouldSkip: true, // Skip (no action needed)
				shouldQueueConflict: false // Usually safe to skip
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
```

### 3.2 Conflict Detection Call Stack

```
┌─────────────────────────────────────────────────────────────────────┐
│  Server Change: {action: "update", version: 5, ...}                 │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. Query Local Entity                                              │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  SELECT id, version, updated_at, deleted_at, *               │  │
│  │  FROM wells                                                   │  │
│  │  WHERE id = ? AND workspace_id = ?                           │  │
│  │                                                               │  │
│  │  Result:                                                      │  │
│  │  - localVersion: 7 (or NULL if not exists)                   │  │
│  │  - localUpdatedAt: '2025-01-15T10:35:00Z'                    │  │
│  │  - localDeletedAt: NULL                                      │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Compare versions
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. Version Comparison                                              │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  localVersion = 7                                            │  │
│  │  remoteVersion = 5                                           │  │
│  │                                                               │  │
│  │  Comparison:                                                 │  │
│  │  IF 7 > 5:                                                   │  │
│  │    → CONFLICT: Local is newer                                │  │
│  │    → Action: Queue conflict, don't apply                     │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Conflict detected
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. Handle Conflict                                                 │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - Save conflict to sync_conflicts table                     │  │
│  │  - Log for audit                                             │  │
│  │  - Queue for resolution (manual or auto)                     │  │
│  │  - Skip applying this change                                 │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 4. Applying Changes with Version Checking: INSERT

### 4.1 INSERT with Version Check

```typescript
async function applyInsert(
	db: Database,
	change: SyncChange,
	workspaceId: string
): Promise<ApplyResult> {
	// Parse JSON data
	const entityData = JSON.parse(change.data || '{}');

	// Step 1: Check if entity already exists
	const existing = await db.get<{
		id: string;
		version: number;
		updated_at: string;
		deleted_at: string | null;
	}>(
		`SELECT id, version, updated_at, deleted_at
     FROM wells
     WHERE id = ? AND workspace_id = ?`,
		[change.entity_id, workspaceId]
	);

	// Step 2: If exists, this is actually an UPDATE, not INSERT
	if (existing) {
		// Check if soft-deleted
		if (existing.deleted_at) {
			// Entity was deleted locally - handle as UPDATE (undelete + update)
			return await applyUpdate(db, change, workspaceId);
		}

		// Step 3: Version check for conflict detection
		const conflictCheck = detectConflict(
			existing.version,
			change.version,
			existing.updated_at ? new Date(existing.updated_at) : null,
			new Date(change.timestamp),
			existing.deleted_at ? new Date(existing.deleted_at) : null
		);

		if (conflictCheck.shouldQueueConflict) {
			// Conflict detected - queue for resolution
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

		if (conflictCheck.shouldSkip) {
			// Already synced - skip
			return { success: true, skipped: true };
		}

		// Remote is newer - apply as UPDATE
		return await applyUpdate(db, change, workspaceId);
	}

	// Step 4: Entity doesn't exist - proceed with INSERT
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
				change.version // Use server-assigned version
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

## 5. Applying Changes with Version Checking: UPDATE

### 5.1 UPDATE with Version Check

```typescript
async function applyUpdate(
	db: Database,
	change: SyncChange,
	workspaceId: string
): Promise<ApplyResult> {
	// Step 1: Check if entity exists
	const existing = await db.get<{
		id: string;
		version: number;
		updated_at: string;
		deleted_at: string | null;
		[key: string]: any;
	}>(
		`SELECT * FROM wells
     WHERE id = ? AND workspace_id = ?`,
		[change.entity_id, workspaceId]
	);

	// Step 2: If doesn't exist, this is actually an INSERT
	if (!existing) {
		return await applyInsert(db, change, workspaceId);
	}

	// Step 3: Version check for conflict detection
	const conflictCheck = detectConflict(
		existing.version,
		change.version,
		existing.updated_at ? new Date(existing.updated_at) : null,
		new Date(change.timestamp),
		existing.deleted_at ? new Date(existing.deleted_at) : null
	);

	// Step 4: Handle conflict detection result
	if (conflictCheck.shouldQueueConflict) {
		// Conflict: local version is newer
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

	if (conflictCheck.shouldSkip) {
		// Already synced - skip
		return { success: true, skipped: true };
	}

	// Step 5: Remote is newer - apply UPDATE
	// Parse update data and merge with existing
	const updateData = JSON.parse(change.data || '{}');
	const merged = { ...existing, ...updateData };

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
        version = ?  -- Use server-assigned version
      WHERE id = ? AND workspace_id = ?`,
			[
				merged.name,
				merged.uwi,
				merged.field,
				merged.company,
				merged.x,
				merged.y,
				change.timestamp,
				change.version, // Server-assigned version
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

## 6. Applying Changes with Version Checking: DELETE

### 6.1 DELETE with Version Check

```typescript
async function applyDelete(
	db: Database,
	change: SyncChange,
	workspaceId: string
): Promise<ApplyResult> {
	// Step 1: Check if entity exists
	const existing = await db.get<{
		id: string;
		version: number;
		updated_at: string;
		deleted_at: string | null;
	}>(
		`SELECT id, version, updated_at, deleted_at
     FROM wells
     WHERE id = ? AND workspace_id = ?`,
		[change.entity_id, workspaceId]
	);

	// Step 2: If doesn't exist, already deleted or never existed
	if (!existing) {
		return { success: true, skipped: true };
	}

	// Step 3: Check if already soft-deleted
	if (existing.deleted_at) {
		// Already deleted - check version to see if we should update
		const conflictCheck = detectConflict(
			existing.version,
			change.version,
			existing.updated_at ? new Date(existing.updated_at) : null,
			new Date(change.timestamp),
			existing.deleted_at ? new Date(existing.deleted_at) : null
		);

		if (conflictCheck.shouldSkip) {
			// Already synced - skip
			return { success: true, skipped: true };
		}
	}

	// Step 4: Version check for conflict detection
	const conflictCheck = detectConflict(
		existing.version,
		change.version,
		existing.updated_at ? new Date(existing.updated_at) : null,
		new Date(change.timestamp),
		existing.deleted_at ? new Date(existing.deleted_at) : null
	);

	// Step 5: Handle conflict detection result
	if (conflictCheck.shouldQueueConflict) {
		// Conflict: local version is newer
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

	if (conflictCheck.shouldSkip) {
		// Already synced - skip
		return { success: true, skipped: true };
	}

	// Step 6: Remote is newer - apply soft delete
	try {
		await db.run(
			`UPDATE wells SET
        deleted_at = ?,
        version = ?  -- Use server-assigned version
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

## 7. Version Checking Integration: Complete Flow

### 7.1 Where Version Checking Happens in the Apply Flow

Version checking is **integrated at the very beginning** of each change application operation, before any data modification occurs:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Complete Change Application Flow with Version Checking              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 1: Receive Server Change                               │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  change = {                                            │  │  │
│  │  │    entity_id: "well-123",                              │  │  │
│  │  │    action: "update",                                   │  │  │
│  │  │    version: 5,  ← Server-assigned version              │  │  │
│  │  │    timestamp: "2025-01-15T10:30:00Z",                  │  │  │
│  │  │    data: "{...}"                                       │  │  │
│  │  │  }                                                      │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 2: Query Local Entity (BEFORE ANY MODIFICATION)        │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  SELECT id, version, updated_at, deleted_at, *         │  │  │
│  │  │  FROM wells                                            │  │  │
│  │  │  WHERE id = ? AND workspace_id = ?                     │  │  │
│  │  │                                                         │  │  │
│  │  │  Result:                                               │  │  │
│  │  │  - localEntity = {                                     │  │  │
│  │  │      id: "well-123",                                   │  │  │
│  │  │      version: 7,  ← Local version                      │  │  │
│  │  │      updated_at: "2025-01-15T10:35:00Z",               │  │  │
│  │  │      name: "Well-001",                                 │  │  │
│  │  │      ...                                               │  │  │
│  │  │    }                                                   │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ▼                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 3: VERSION CHECK (CRITICAL - BEFORE APPLY)             │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  const conflictCheck = detectConflict(                 │  │  │
│  │  │    localEntity.version,  // 7                          │  │  │
│  │  │    change.version,       // 5                          │  │  │
│  │  │    localEntity.updated_at,                             │  │  │
│  │  │    change.timestamp,                                   │  │  │
│  │  │    localEntity.deleted_at                              │  │  │
│  │  │  );                                                    │  │  │
│  │  │                                                         │  │  │
│  │  │  Comparison:                                           │  │  │
│  │  │  IF 7 > 5:                                             │  │  │
│  │  │    → CONFLICT: Local is newer                          │  │  │
│  │  │    → shouldQueueConflict = true                        │  │  │
│  │  │    → shouldApply = false                               │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
│                         │                                           │
│                         ├─────────────────────┬─────────────────────┐
│                         ▼                     ▼                     ▼
│        ┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐
│        │  Conflict Detected:  │  │  Already Synced:    │  │  No Conflict:       │
│        │  - Queue conflict    │  │  - Skip operation   │  │  - Proceed to apply │
│        │  - Log for audit     │  │  - No DB change     │  │  - Apply change     │
│        │  - Skip apply        │  │  - Continue to next │  │  - Update version   │
│        │  - Return early      │  │  - Return success   │  │  - Continue to next │
│        └──────────────────────┘  └──────────────────────┘  └──────────────────────┘
│                         │
│                         │ (Only if no conflict)
│                         ▼
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Step 4: Apply Change (ONLY IF NO CONFLICT)                  │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  UPDATE wells SET                                     │  │  │
│  │  │    name = ?,                                          │  │  │
│  │  │    updated_at = ?,                                    │  │  │
│  │  │    version = 5  ← Use server-assigned version         │  │  │
│  │  │  WHERE id = ? AND workspace_id = ?                    │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 7.2 Key Integration Points

**Version checking happens at THREE critical points**:

1. **Before INSERT**: Check if entity exists (concurrent create scenario).
2. **Before UPDATE**: Check local version vs remote version.
3. **Before DELETE**: Check local version vs remote version.

**The version check is ALWAYS the first operation** after querying the local entity, and it **determines whether to proceed with the apply operation**.

### 7.3 Version Check as a Guard Clause

The version check acts as a **guard clause** that prevents invalid operations:

```typescript
async function applyChangeWithVersionCheck(
	db: Database,
	change: SyncChange,
	workspaceId: string
): Promise<ApplyResult> {
	// GUARD 1: Query local entity first
	const localEntity = await queryLocalEntity(db, change.entity_id, workspaceId);

	// GUARD 2: Version check (CRITICAL - prevents invalid applies)
	const conflictCheck = detectConflict(
		localEntity?.version ?? null,
		change.version,
		localEntity?.updated_at ? new Date(localEntity.updated_at) : null,
		new Date(change.timestamp),
		localEntity?.deleted_at ? new Date(localEntity.deleted_at) : null
	);

	// GUARD 3: Early return if conflict (don't modify data)
	if (conflictCheck.shouldQueueConflict) {
		return await queueConflict(db, change, localEntity);
	}

	// GUARD 4: Early return if already synced (idempotency)
	if (conflictCheck.shouldSkip) {
		return { success: true, skipped: true };
	}

	// ONLY REACH HERE IF: No conflict and not already synced
	// Now safe to apply the change
	return await applyChangeOperation(db, change, workspaceId);
}
```

---

## 8. Complete Change Application with Version Checking

### 8.1 Transactional Change Application

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
			// Step 1: Query local entity for version check
			const localEntity = await db.get<{
				version: number;
				updated_at: string;
				deleted_at: string | null;
			}>(
				`SELECT version, updated_at, deleted_at
         FROM ${getTableName(change.entity_type)}
         WHERE id = ? AND workspace_id = ?`,
				[change.entity_id, workspaceId]
			);

			// Step 2: Version check and conflict detection
			const conflictCheck = localEntity
				? detectConflict(
						localEntity.version,
						change.version,
						localEntity.updated_at ? new Date(localEntity.updated_at) : null,
						new Date(change.timestamp),
						localEntity.deleted_at ? new Date(localEntity.deleted_at) : null
					)
				: {
						conflictType: ConflictType.None,
						shouldApply: true,
						shouldSkip: false,
						shouldQueueConflict: false
					};

			// Step 3: Handle conflict or apply change
			if (conflictCheck.shouldQueueConflict) {
				// Conflict detected - save to conflicts table
				const conflict: Conflict = {
					entity_type: change.entity_type,
					entity_id: change.entity_id,
					local_version: localEntity!.version,
					remote_version: change.version,
					local_data: JSON.stringify(localEntity),
					remote_data: change.data || null
				};

				await saveConflict(db, conflict);
				conflicts.push(conflict);

				results.push({
					success: false,
					conflict
				});
				continue; // Skip applying this change
			}

			if (conflictCheck.shouldSkip) {
				// Already synced - skip
				results.push({ success: true, skipped: true });
				continue;
			}

			// Step 4: Apply change (no conflict)
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
		}

		// Step 5: Update sync state
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

---

## 8. Conflict Storage & Resolution

### 8.1 Save Conflict to Database

```typescript
async function saveConflict(db: Database, conflict: Conflict): Promise<void> {
	await db.run(
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
			conflict.remote_data || null,
			'pending', // Resolution status
			new Date().toISOString()
		]
	);
}
```

### 8.2 Conflict Resolution Strategies

**Strategy 1: Last-Write-Wins (Auto-Resolve)**:

```typescript
async function autoResolveConflict(
	db: Database,
	conflictId: number,
	strategy: 'local' | 'remote' | 'last_write_wins'
): Promise<void> {
	const conflict = await getConflict(db, conflictId);
	if (!conflict) return;

	const localData = JSON.parse(conflict.local_data);
	const remoteData = JSON.parse(conflict.remote_data || '{}');

	let resolution: ConflictResolution;

	if (strategy === 'last_write_wins') {
		// Compare timestamps
		const localTime = new Date(localData.updated_at || 0).getTime();
		const remoteTime = new Date(remoteData.updated_at || 0).getTime();

		resolution = localTime > remoteTime ? ConflictResolution.Local : ConflictResolution.Remote;
	} else {
		resolution = strategy === 'local' ? ConflictResolution.Local : ConflictResolution.Remote;
	}

	// Apply resolution
	if (resolution === ConflictResolution.Local) {
		// Keep local version - push to server on next sync
		// No action needed (local is already correct)
	} else {
		// Apply remote version
		await applyChange(db, {
			entity_type: conflict.entity_type,
			entity_id: conflict.entity_id,
			action: 'update',
			version: conflict.remote_version,
			data: conflict.remote_data,
			timestamp: remoteData.updated_at
		});
	}

	// Mark conflict as resolved
	await db.run(
		`UPDATE sync_conflicts
     SET resolution = ?, resolved_at = ?
     WHERE id = ?`,
		[resolution, new Date().toISOString(), conflictId]
	);
}
```

---

## 9. Edge Cases & Special Scenarios

### 9.1 Concurrent Creates (Same Entity ID)

**Scenario**: Two clients create the same entity (same UUID) simultaneously.

```typescript
// Client A creates well with ID "well-123" (version 1)
// Client B creates well with ID "well-123" (version 1)
// Both push to server
// Server accepts both, assigns versions 1 and 2
// On sync, Client A receives Client B's create (version 2)

async function applyInsertWithConcurrentCreate(
	db: Database,
	change: SyncChange,
	workspaceId: string
): Promise<ApplyResult> {
	const existing = await db.get('SELECT version FROM wells WHERE id = ? AND workspace_id = ?', [
		change.entity_id,
		workspaceId
	]);

	if (existing) {
		// Entity already exists - this is a concurrent create
		// Check version
		if (existing.version >= change.version) {
			// Local version is same or newer - conflict
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

		// Remote version is newer - apply as UPDATE
		return await applyUpdate(db, change, workspaceId);
	}

	// Proceed with INSERT
	// ...
}
```

### 9.2 Version Rollback (Should Never Happen)

**Scenario**: Server version is lower than local version (shouldn't happen in normal operation).

```typescript
// This indicates a serious issue:
// - Server data corruption
// - Clock skew
// - Manual database manipulation

if (localVersion > remoteVersion) {
	// Log warning
	console.warn(`Version rollback detected: local=${localVersion}, remote=${remoteVersion}`);

	// Options:
	// 1. Queue conflict for manual resolution (safest)
	// 2. Trust local version (if confident)
	// 3. Request full entity from server (verify)

	return {
		success: false,
		conflict: {
			entity_type: change.entity_type,
			entity_id: change.entity_id,
			local_version: localVersion,
			remote_version: remoteVersion,
			local_data: JSON.stringify(localEntity),
			remote_data: change.data || ''
		}
	};
}
```

### 9.3 Soft-Deleted Entity Updates

**Scenario**: Server sends UPDATE for an entity that was soft-deleted locally.

```typescript
async function applyUpdateToDeletedEntity(
	db: Database,
	change: SyncChange,
	workspaceId: string
): Promise<ApplyResult> {
	const existing = await db.get<{
		version: number;
		deleted_at: string | null;
	}>('SELECT version, deleted_at FROM wells WHERE id = ? AND workspace_id = ?', [
		change.entity_id,
		workspaceId
	]);

	if (existing && existing.deleted_at) {
		// Entity is soft-deleted locally
		// Check version
		if (existing.version >= change.version) {
			// Local version is same or newer - conflict
			// Option: Keep deleted, or undelete and apply update?
			// Strategy: If remote version is newer, undelete and apply
			if (existing.version < change.version) {
				// Remote is newer - undelete and apply update
				await db.run(
					`UPDATE wells SET
            deleted_at = NULL,
            version = ?,
            -- ... other fields from change.data ...
          WHERE id = ?`,
					[change.version, change.entity_id]
				);
				return { success: true };
			}
		}
	}

	// Normal update flow
	// ...
}
```

---

## 10. Design Patterns for DataForge

### 10.1 Version Comparison Pattern

**ColaNode Pattern**:

- Compare `local.version` vs `remote.version`.
- If `local > remote`: conflict (local is newer).
- If `local < remote`: apply (remote is newer).
- If `local === remote`: skip (already synced).

**DataForge Application**:

- Use same version comparison logic.
- Track versions in `wells`, `curves`, etc. tables.
- Server assigns versions (clients use server versions).

### 10.2 Conflict Detection Before Apply

**ColaNode Pattern**:

- Check version **before** modifying data.
- Query local entity first.
- Compare versions.
- Decide: apply, skip, or queue conflict.

**DataForge Application**:

- Always query local entity before applying change.
- Compare versions in application code (not SQL).
- Queue conflicts for resolution (don't lose data).

### 10.3 Transactional Conflict Handling

**ColaNode Pattern**:

- All changes applied in single transaction.
- Conflicts detected and queued within transaction.
- Rollback on error, commit on success.

**DataForge Application**:

- Use SQLite transactions for atomicity.
- Queue conflicts in same transaction.
- Never lose data (conflicts are queued, not discarded).

---

## 11. Summary

### ✅ Key Patterns

1. **Version Comparison**: Compare `local.version` vs `remote.version` before applying.
2. **Conflict Detection**: Detect conflicts early (before modifying data).
3. **Conflict Queueing**: Queue conflicts for resolution (don't lose data).
4. **Server-Assigned Versions**: Use server versions (never increment locally).
5. **Transactional Safety**: Apply all changes atomically in a transaction.

### 📋 Best Practices

1. **Always check version** before applying changes.
2. **Query local entity first** (to get current version).
3. **Compare versions** in application code (not SQL).
4. **Queue conflicts** (don't discard data).
5. **Use server-assigned versions** (server is authoritative).
6. **Handle edge cases** (concurrent creates, soft deletes, etc.).

This architecture provides a robust foundation for version-based conflict detection while maintaining data consistency and never losing user data.
