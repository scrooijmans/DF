# ColaNode Auto-Resolution Strategies: Architecture & Design (Context7 Summary)

This document dives deep into **ColaNode's architecture and design for auto-resolution strategies** when conflicts are detected during sync operations. It covers when auto-resolution is applied, the different resolution strategies available, and how conflicts are automatically resolved without user intervention.

It builds on:

- `colanode-version-checking-and-conflict-detection.md`
- `colanode-crdt-and-conflict-resolution-architecture.md`
- `colanode-applying-server-changes-to-local-sqlite.md`

and provides detailed call stacks and implementation patterns for automatic conflict resolution.

---

## 1. High-Level Architecture: When Auto-Resolution Applies

### 1.1 Two Types of Conflicts

ColaNode handles conflicts differently based on the data type:

```
┌─────────────────────────────────────────────────────────────────────┐
│                    ColaNode Conflict Resolution                      │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Type 1: CRDT-Managed Data (Pages, Collaborative Records)    │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Conflicts resolved AUTOMATICALLY by Yjs             │  │  │
│  │  │  - No explicit conflict detection needed               │  │  │
│  │  │  - CRDT semantics guarantee convergence                │  │  │
│  │  │  - No user intervention required                       │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Type 2: Non-CRDT Relational Data (Messages, Metadata)      │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  - Conflicts detected via version checking             │  │  │
│  │  │  - Auto-resolution strategies applied                  │  │  │
│  │  │  - Configurable per entity type                        │  │  │
│  │  │  - Fallback to manual resolution if needed             │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 1.2 Auto-Resolution Decision Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│  Conflict Detected (local.version > remote.version)                 │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Check Entity Type                                                  │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  IF entity is CRDT-managed (page, collaborative record):     │  │
│  │    → No conflict (Yjs handles it automatically)              │  │
│  │  ELSE IF entity is relational (message, metadata):           │  │
│  │    → Proceed to auto-resolution                              │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Get Resolution Strategy for Entity Type                            │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - Check workspace-level resolution policy                   │  │
│  │  - Check entity-type-level resolution policy                 │  │
│  │  - Use default strategy if not configured                    │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ├─────────────────────┬─────────────────────┐
                          ▼                     ▼                     ▼
        ┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐
        │  Strategy:           │  │  Strategy:           │  │  Strategy:           │
        │  Last-Write-Wins     │  │  Local-Wins          │  │  Remote-Wins         │
        │  → Auto-resolve      │  │  → Auto-resolve      │  │  → Auto-resolve      │
        └──────────────────────┘  └──────────────────────┘  └──────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Apply Auto-Resolution                                              │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - Execute resolution logic                                  │  │
│  │  - Update local database                                     │  │
│  │  - Mark conflict as resolved                                 │  │
│  │  - Log resolution for audit                                  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. Resolution Strategy Configuration

### 2.1 Strategy Configuration Schema

ColaNode allows configuring resolution strategies at multiple levels:

```sql
-- Workspace-level resolution policy
CREATE TABLE workspace_resolution_policy (
    id SERIAL PRIMARY KEY,
    workspace_id TEXT NOT NULL REFERENCES workspaces(id),
    entity_type TEXT, -- NULL = default for all entity types
    strategy TEXT NOT NULL, -- 'last_write_wins', 'local_wins', 'remote_wins', 'manual'
    priority INTEGER NOT NULL DEFAULT 0, -- Higher = more specific
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(workspace_id, entity_type)
);

-- Example policies:
-- workspace_id='ws-1', entity_type=NULL, strategy='last_write_wins' (default)
-- workspace_id='ws-1', entity_type='message', strategy='remote_wins' (override)
-- workspace_id='ws-1', entity_type='file_metadata', strategy='local_wins' (override)
```

### 2.2 Strategy Resolution Logic

```typescript
type ResolutionStrategy = 'last_write_wins' | 'local_wins' | 'remote_wins' | 'manual';

interface ResolutionPolicy {
	workspace_id: string;
	entity_type: string | null; // null = default
	strategy: ResolutionStrategy;
	priority: number;
}

/**
 * Get resolution strategy for a conflict
 */
async function getResolutionStrategy(
	db: Database,
	workspaceId: string,
	entityType: string
): Promise<ResolutionStrategy> {
	// Query policies, ordered by priority (most specific first)
	const policies = await db.all<ResolutionPolicy>(
		`SELECT entity_type, strategy, priority
     FROM workspace_resolution_policy
     WHERE workspace_id = ?
     ORDER BY priority DESC, entity_type DESC NULLS LAST`,
		[workspaceId]
	);

	// Find most specific policy
	for (const policy of policies) {
		// Exact match for entity type
		if (policy.entity_type === entityType) {
			return policy.strategy;
		}
		// Default policy (entity_type is NULL)
		if (policy.entity_type === null) {
			return policy.strategy;
		}
	}

	// Fallback to default
	return 'last_write_wins';
}
```

---

## 3. Auto-Resolution Strategies

### 3.1 Strategy 1: Last-Write-Wins (Default)

**Principle**: The change with the most recent `updated_at` timestamp wins.

**When to Use**:

- Default strategy for most entity types
- Works well when timestamps are reliable
- Simple and predictable

**Implementation**:

```typescript
async function autoResolveLastWriteWins(
	db: Database,
	conflict: Conflict
): Promise<ResolutionResult> {
	const localData = JSON.parse(conflict.local_data);
	const remoteData = JSON.parse(conflict.remote_data || '{}');

	// Compare timestamps
	const localTime = new Date(localData.updated_at || 0).getTime();
	const remoteTime = new Date(remoteData.updated_at || 0).getTime();

	if (localTime > remoteTime) {
		// Local is newer - keep local, push to server on next sync
		return {
			resolution: 'local',
			action: 'keep_local',
			reason: `Local change is newer (${localTime} > ${remoteTime})`,
			data: localData
		};
	} else if (remoteTime > localTime) {
		// Remote is newer - apply remote change
		return {
			resolution: 'remote',
			action: 'apply_remote',
			reason: `Remote change is newer (${remoteTime} > ${localTime})`,
			data: remoteData
		};
	} else {
		// Same timestamp - fallback to version comparison
		if (conflict.local_version > conflict.remote_version) {
			return {
				resolution: 'local',
				action: 'keep_local',
				reason: 'Same timestamp, local version is higher',
				data: localData
			};
		} else {
			return {
				resolution: 'remote',
				action: 'apply_remote',
				reason: 'Same timestamp, remote version is higher',
				data: remoteData
			};
		}
	}
}
```

**Call Stack**:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Conflict Detected: local.version=7, remote.version=5              │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Get Resolution Strategy                                            │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  strategy = getResolutionStrategy(db, workspaceId, 'well')  │  │
│  │  → Returns: 'last_write_wins'                                │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Compare Timestamps                                                 │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  local.updated_at = '2025-01-15T10:35:00Z'                  │  │
│  │  remote.updated_at = '2025-01-15T10:30:00Z'                 │  │
│  │                                                               │  │
│  │  Comparison:                                                 │  │
│  │  IF local > remote:                                          │  │
│  │    → Resolution: 'local' (keep local, push to server)        │  │
│  │  ELSE:                                                       │  │
│  │    → Resolution: 'remote' (apply remote change)              │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Resolution: 'local'
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Apply Resolution                                                   │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - Keep local data (no change to local DB)                   │  │
│  │  - Mark conflict as resolved: 'local'                        │  │
│  │  - Queue local change for push to server                     │  │
│  │  - Log resolution for audit                                  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.2 Strategy 2: Local-Wins

**Principle**: Always keep local changes, discard remote changes.

**When to Use**:

- User is working offline and local changes should be prioritized
- Local changes are considered more authoritative
- For entity types where local edits are more valuable

**Implementation**:

```typescript
async function autoResolveLocalWins(db: Database, conflict: Conflict): Promise<ResolutionResult> {
	const localData = JSON.parse(conflict.local_data);

	// Always keep local version
	return {
		resolution: 'local',
		action: 'keep_local',
		reason: 'Local-wins strategy: always keep local changes',
		data: localData
	};
}
```

**Call Stack**:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Conflict Detected: local.version=7, remote.version=5              │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Get Resolution Strategy                                            │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  strategy = getResolutionStrategy(db, workspaceId, 'message')│  │
│  │  → Returns: 'local_wins'                                     │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Apply Local-Wins Resolution                                        │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - Keep local data (no change to local DB)                   │  │
│  │  - Mark conflict as resolved: 'local'                        │  │
│  │  - Queue local change for push to server                     │  │
│  │  - Discard remote change                                     │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.3 Strategy 3: Remote-Wins

**Principle**: Always accept remote changes, discard local changes.

**When to Use**:

- Server is authoritative (e.g., system configuration)
- Local changes are considered less important
- For entity types where server state should always win

**Implementation**:

```typescript
async function autoResolveRemoteWins(db: Database, conflict: Conflict): Promise<ResolutionResult> {
	const remoteData = JSON.parse(conflict.remote_data || '{}');

	// Always apply remote version
	return {
		resolution: 'remote',
		action: 'apply_remote',
		reason: 'Remote-wins strategy: always accept remote changes',
		data: remoteData
	};
}
```

**Call Stack**:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Conflict Detected: local.version=7, remote.version=5              │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Get Resolution Strategy                                            │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  strategy = getResolutionStrategy(db, workspaceId, 'config') │  │
│  │  → Returns: 'remote_wins'                                    │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Apply Remote-Wins Resolution                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - Apply remote data to local DB                             │  │
│  │  - Update local entity with remote data                      │  │
│  │  - Update version to remote version                          │  │
│  │  - Mark conflict as resolved: 'remote'                       │  │
│  │  - Discard local change                                      │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.4 Strategy 4: Manual Resolution

**Principle**: Queue conflict for user review, don't auto-resolve.

**When to Use**:

- Conflicts require user judgment
- Both local and remote changes are valuable
- For critical entity types where data loss is unacceptable

**Implementation**:

```typescript
async function queueForManualResolution(
	db: Database,
	conflict: Conflict
): Promise<ResolutionResult> {
	// Don't auto-resolve - queue for user review
	await db.run(
		`UPDATE sync_conflicts
     SET resolution = 'pending',
         queued_for_review = true,
         queued_at = ?
     WHERE id = ?`,
		[new Date().toISOString(), conflict.id]
	);

	return {
		resolution: 'pending',
		action: 'queue_for_review',
		reason: 'Manual resolution required',
		data: null
	};
}
```

---

## 4. Complete Auto-Resolution Flow

### 4.1 Conflict Resolution Service

```typescript
class ConflictResolutionService {
	constructor(
		private db: Database,
		private workspaceId: string
	) {}

	/**
	 * Auto-resolve a conflict based on configured strategy
	 */
	async autoResolveConflict(conflict: Conflict): Promise<ResolutionResult> {
		// Step 1: Get resolution strategy
		const strategy = await this.getResolutionStrategy(conflict.entity_type);

		// Step 2: Check if strategy is manual
		if (strategy === 'manual') {
			return await this.queueForManualResolution(conflict);
		}

		// Step 3: Apply auto-resolution
		let result: ResolutionResult;

		switch (strategy) {
			case 'last_write_wins':
				result = await this.autoResolveLastWriteWins(conflict);
				break;
			case 'local_wins':
				result = await this.autoResolveLocalWins(conflict);
				break;
			case 'remote_wins':
				result = await this.autoResolveRemoteWins(conflict);
				break;
			default:
				// Fallback to last-write-wins
				result = await this.autoResolveLastWriteWins(conflict);
		}

		// Step 4: Apply resolution to database
		await this.applyResolution(conflict, result);

		// Step 5: Log resolution for audit
		await this.logResolution(conflict, result);

		return result;
	}

	/**
	 * Apply resolution result to database
	 */
	private async applyResolution(conflict: Conflict, result: ResolutionResult): Promise<void> {
		await this.db.exec('BEGIN TRANSACTION');

		try {
			if (result.resolution === 'local') {
				// Keep local - no change to local DB
				// Queue local change for push to server
				await this.queueLocalChangeForPush(conflict);
			} else if (result.resolution === 'remote') {
				// Apply remote change to local DB
				await this.applyRemoteChange(conflict, result.data);
			} else if (result.resolution === 'pending') {
				// Queue for manual resolution - already handled
				return;
			}

			// Mark conflict as resolved
			await this.db.run(
				`UPDATE sync_conflicts
         SET resolution = ?,
             resolved_at = ?,
             resolution_reason = ?
         WHERE id = ?`,
				[result.resolution, new Date().toISOString(), result.reason, conflict.id]
			);

			await this.db.exec('COMMIT');
		} catch (error) {
			await this.db.exec('ROLLBACK');
			throw error;
		}
	}

	/**
	 * Apply remote change to local database
	 */
	private async applyRemoteChange(conflict: Conflict, remoteData: any): Promise<void> {
		const tableName = this.getTableName(conflict.entity_type);

		await this.db.run(
			`UPDATE ${tableName}
       SET version = ?,
           updated_at = ?,
           -- ... other fields from remoteData
       WHERE id = ? AND workspace_id = ?`,
			[conflict.remote_version, remoteData.updated_at, conflict.entity_id, this.workspaceId]
		);
	}

	/**
	 * Queue local change for push to server
	 */
	private async queueLocalChangeForPush(conflict: Conflict): Promise<void> {
		// Add to sync queue for push
		await this.db.run(
			`INSERT INTO sync_queue (
        workspace_id, entity_type, entity_id, action, version, payload
      ) VALUES (?, ?, ?, ?, ?, ?)
      ON CONFLICT(workspace_id, entity_type, entity_id) DO UPDATE SET
        action = excluded.action,
        version = excluded.version,
        payload = excluded.payload`,
			[
				this.workspaceId,
				conflict.entity_type,
				conflict.entity_id,
				'update',
				conflict.local_version,
				conflict.local_data
			]
		);
	}

	/**
	 * Log resolution for audit
	 */
	private async logResolution(conflict: Conflict, result: ResolutionResult): Promise<void> {
		await this.db.run(
			`INSERT INTO conflict_resolution_log (
        conflict_id, workspace_id, entity_type, entity_id,
        strategy, resolution, reason, resolved_at
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)`,
			[
				conflict.id,
				this.workspaceId,
				conflict.entity_type,
				conflict.entity_id,
				await this.getResolutionStrategy(conflict.entity_type),
				result.resolution,
				result.reason,
				new Date().toISOString()
			]
		);
	}
}
```

### 4.2 Batch Auto-Resolution

```typescript
/**
 * Auto-resolve all pending conflicts in a workspace
 */
async function autoResolveAllConflicts(
	db: Database,
	workspaceId: string
): Promise<AutoResolutionResult> {
	const resolutionService = new ConflictResolutionService(db, workspaceId);

	// Get all pending conflicts
	const conflicts = await db.all<Conflict>(
		`SELECT * FROM sync_conflicts
     WHERE workspace_id = ? AND resolution = 'pending'
     ORDER BY created_at ASC`,
		[workspaceId]
	);

	const results: ResolutionResult[] = [];
	const errors: Error[] = [];

	// Process each conflict
	for (const conflict of conflicts) {
		try {
			const result = await resolutionService.autoResolveConflict(conflict);
			results.push(result);
		} catch (error) {
			errors.push(error as Error);
			// Log error but continue processing other conflicts
			console.error(`Failed to resolve conflict ${conflict.id}:`, error);
		}
	}

	return {
		total: conflicts.length,
		resolved: results.length,
		failed: errors.length,
		results: results,
		errors: errors
	};
}
```

---

## 5. CRDT Auto-Resolution (Yjs)

### 5.1 Automatic Conflict Resolution for CRDT Data

For CRDT-managed data (pages, collaborative records), conflicts are resolved **automatically by Yjs** without explicit conflict detection:

```typescript
// CRDT-managed page edit
const ydoc = new Y.Doc();
const ytext = ydoc.getText('content');

// Device A types "Hello"
ytext.insert(0, 'Hello');

// Device B types "World" (concurrently)
ytext.insert(0, 'World');

// Yjs automatically merges: "WorldHello" or "HelloWorld"
// No conflict detection or resolution code needed
```

**Key Points**:

- **No explicit conflict detection**: Yjs handles all conflicts automatically
- **CRDT semantics**: Operations are commutative and idempotent
- **Automatic convergence**: All clients eventually see the same state
- **No user intervention**: Conflicts are resolved transparently

### 5.2 CRDT vs Non-CRDT Resolution

```
┌─────────────────────────────────────────────────────────────────────┐
│  Conflict Resolution: CRDT vs Non-CRDT                              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  CRDT-Managed (Pages, Collaborative Records)                 │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  Conflict: Two users edit same page concurrently       │  │  │
│  │  │  Detection: None (Yjs handles it)                      │  │  │
│  │  │  Resolution: Automatic (CRDT merge)                    │  │  │
│  │  │  User Action: None required                            │  │  │
│  │  │  Result: Merged content (e.g., "HelloWorld")           │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Non-CRDT (Messages, Metadata)                              │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │  Conflict: local.version=7 > remote.version=5         │  │  │
│  │  │  Detection: Version comparison                         │  │  │
│  │  │  Resolution: Auto-resolution strategy                  │  │  │
│  │  │  User Action: May be required (if strategy='manual')   │  │  │
│  │  │  Result: One version kept (local or remote)            │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 6. Resolution Audit & Logging

### 6.1 Resolution Log Schema

```sql
-- Conflict resolution audit log
CREATE TABLE conflict_resolution_log (
    id SERIAL PRIMARY KEY,
    conflict_id INTEGER NOT NULL REFERENCES sync_conflicts(id),
    workspace_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    strategy TEXT NOT NULL, -- 'last_write_wins', 'local_wins', etc.
    resolution TEXT NOT NULL, -- 'local', 'remote', 'pending'
    reason TEXT, -- Why this resolution was chosen
    resolved_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    resolved_by TEXT, -- User ID or 'system' for auto-resolution
    local_version INTEGER NOT NULL,
    remote_version INTEGER NOT NULL,
    local_data_snapshot TEXT, -- JSON snapshot of local data
    remote_data_snapshot TEXT -- JSON snapshot of remote data
);

CREATE INDEX idx_resolution_log_workspace ON conflict_resolution_log(workspace_id);
CREATE INDEX idx_resolution_log_entity ON conflict_resolution_log(entity_type, entity_id);
CREATE INDEX idx_resolution_log_resolved_at ON conflict_resolution_log(resolved_at);
```

### 6.2 Resolution Analytics

```typescript
/**
 * Get resolution statistics for a workspace
 */
async function getResolutionStats(
	db: Database,
	workspaceId: string,
	startDate?: Date,
	endDate?: Date
): Promise<ResolutionStats> {
	let query = `
    SELECT
      strategy,
      resolution,
      COUNT(*) as count
    FROM conflict_resolution_log
    WHERE workspace_id = ?
  `;
	const params: any[] = [workspaceId];

	if (startDate) {
		query += ` AND resolved_at >= ?`;
		params.push(startDate.toISOString());
	}

	if (endDate) {
		query += ` AND resolved_at <= ?`;
		params.push(endDate.toISOString());
	}

	query += ` GROUP BY strategy, resolution`;

	const stats = await db.all(query, params);

	return {
		total: stats.reduce((sum, s) => sum + s.count, 0),
		by_strategy: stats.reduce(
			(acc, s) => {
				if (!acc[s.strategy]) {
					acc[s.strategy] = {};
				}
				acc[s.strategy][s.resolution] = s.count;
				return acc;
			},
			{} as Record<string, Record<string, number>>
		)
	};
}
```

---

## 7. Design Patterns for DataForge

### 7.1 Strategy Pattern for Resolution

**ColaNode Pattern**:

- Configurable resolution strategies per entity type
- Default strategy with overrides
- Strategy resolution based on priority

**DataForge Application**:

- Use same strategy pattern
- Configure strategies in workspace settings
- Support per-entity-type overrides

### 7.2 Automatic vs Manual Resolution

**ColaNode Pattern**:

- Auto-resolve when strategy is not 'manual'
- Queue for manual resolution when strategy is 'manual'
- Log all resolutions for audit

**DataForge Application**:

- Auto-resolve conflicts for non-critical entities
- Queue critical conflicts for user review
- Provide UI for manual conflict resolution

### 7.3 CRDT for Collaborative Content

**ColaNode Pattern**:

- Use Yjs for collaborative content (pages, records)
- No explicit conflict detection needed
- Automatic merge via CRDT semantics

**DataForge Application**:

- Consider Yjs for collaborative editing features
- Use version-based conflict detection for relational data
- Combine both approaches as needed

---

## 8. Summary

### ✅ Key Patterns

1. **Two-Tier Resolution**: CRDT (automatic) vs Non-CRDT (explicit strategies)
2. **Configurable Strategies**: Per-entity-type resolution policies
3. **Auto-Resolution**: Last-write-wins, local-wins, remote-wins
4. **Manual Resolution**: Queue conflicts for user review when needed
5. **Audit Logging**: Track all resolutions for compliance and debugging

### 📋 Best Practices

1. **Use CRDTs for collaborative content** (automatic conflict resolution)
2. **Configure strategies per entity type** (flexible resolution policies)
3. **Default to last-write-wins** (simple and predictable)
4. **Log all resolutions** (audit trail and analytics)
5. **Queue critical conflicts** (manual review when needed)
6. **Support strategy overrides** (workspace and entity-type level)

This architecture provides a flexible, auditable system for automatic conflict resolution while maintaining data integrity and user control when needed.

