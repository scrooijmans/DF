# ColaNode Retry Logic & Network Failure Handling (Context7 Summary)

This document dives deep into **ColaNode's architecture and design for handling retry logic and transient network failures** that may impact sync operations. It covers automatic reconnection, exponential backoff, offline queue management, and error classification patterns.

It builds on:

- `colanode-sync-server-configuration.md`
- `colanode-crdt-and-conflict-resolution-architecture.md`
- `colanode-applying-server-changes-to-local-sqlite.md`

and provides detailed call stacks and implementation patterns for resilient sync operations.

---

## 1. High-Level Architecture: Local-First Resilience

### 1.1 The Problem

Network failures can occur at any time:

- **Transient failures**: Temporary network issues, server restarts, timeouts.
- **Permanent failures**: Server down, authentication errors, invalid requests.
- **Partial failures**: Some operations succeed, others fail.

ColaNode must:

- **Continue working offline** (local-first).
- **Queue changes** for later sync.
- **Retry failed operations** with exponential backoff.
- **Distinguish transient from permanent errors**.
- **Handle reconnection** gracefully.

### 1.2 The Solution: Multi-Layer Resilience

```
┌─────────────────────────────────────────────────────────────────────┐
│                    ColaNode Resilience Layers                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Layer 1: Local-First Operations                             │  │
│  │  - All changes saved to SQLite immediately                   │  │
│  │  - UI never blocks on network                                │  │
│  │  - Works completely offline                                   │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Layer 2: Offline Queue                                      │  │
│  │  - Changes queued in sync_queue table                        │  │
│  │  - Tracks attempts, errors, timestamps                       │  │
│  │  - Survives app restarts                                     │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Layer 3: Automatic Reconnection (y-websocket)               │  │
│  │  - WebSocket provider handles reconnection                   │  │
│  │  - Exponential backoff for reconnection attempts             │  │
│  │  - Status events notify of connection state                  │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Layer 4: Retry Logic (Sync Queue Worker)                    │  │
│  │  - Exponential backoff for failed sync operations            │  │
│  │  - Max retry attempts with jitter                            │  │
│  │  - Error classification (transient vs permanent)             │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. y-websocket Automatic Reconnection

### 2.1 Built-in Reconnection

ColaNode uses **y-websocket** (`WebsocketProvider`), which provides automatic reconnection:

```typescript
import { WebsocketProvider } from 'y-websocket';

const wsProvider = new WebsocketProvider(
	'wss://server.colanode.com',
	'workspace-id/document-id',
	ydoc,
	{
		connect: true // Auto-connect and auto-reconnect
		// Reconnection is handled automatically by y-websocket
	}
);
```

**Key Features**:

- **Automatic reconnection**: Reconnects when connection is lost.
- **Exponential backoff**: Gradually increases delay between reconnection attempts.
- **Status events**: Notifies when connected/disconnected.
- **Buffered updates**: Queues Yjs updates while disconnected, sends when reconnected.

### 2.2 Reconnection Call Stack

```
┌─────────────────────────────────────────────────────────────────────┐
│  Network Failure Detected                                            │
│  - WebSocket connection lost                                         │
│  - Network timeout                                                   │
│  - Server unavailable                                                │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  y-websocket: Detect Disconnection                                  │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - WebSocket 'close' or 'error' event                        │  │
│  │  - Set status to 'disconnected'                              │  │
│  │  - Emit 'status' event: {status: 'disconnected'}             │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Buffer Yjs updates (don't lose them)
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  y-websocket: Buffer Updates                                        │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  - Yjs updates are queued in memory                          │  │
│  │  - Updates will be sent when reconnected                     │  │
│  │  - No data loss during disconnection                         │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          │ Exponential backoff reconnection
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  y-websocket: Reconnection Attempt (Exponential Backoff)            │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Attempt 1: Wait 1 second, try reconnect                    │  │
│  │  Attempt 2: Wait 2 seconds, try reconnect                   │  │
│  │  Attempt 3: Wait 4 seconds, try reconnect                   │  │
│  │  Attempt 4: Wait 8 seconds, try reconnect                   │  │
│  │  ... (continues until connected or max attempts)             │  │
│  │                                                               │  │
│  │  Max delay: ~30 seconds (configurable)                       │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ├─────────────────────┬─────────────────────┐
                          ▼                     ▼                     ▼
        ┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐
        │  Success:            │  │  Failure:            │  │  Network Online:     │
        │  - Connected         │  │  - Continue retrying │  │  - Trigger immediate │
        │  - Send buffered     │  │  - Exponential       │  │    reconnection      │
        │    updates           │  │    backoff           │  │  - Process offline   │
        │  - Emit 'status':    │  │  - Max attempts      │  │    queue             │
        │    'connected'       │  │    reached?          │  │                      │
        └──────────────────────┘  └──────────────────────┘  └──────────────────────┘
```

### 2.3 Reconnection Configuration

```typescript
const wsProvider = new WebsocketProvider(serverUrl, room, ydoc, {
	connect: true
	// Reconnection parameters (y-websocket defaults)
	// - Initial delay: ~1 second
	// - Max delay: ~30 seconds
	// - Exponential backoff: 2x per attempt
	// - Jitter: Random variation to prevent thundering herd
});

// Listen for reconnection events
wsProvider.on('status', (event) => {
	if (event.status === 'connected') {
		console.log('Reconnected to server');
		// Process any queued changes
		processOfflineQueue();
	} else if (event.status === 'disconnected') {
		console.log('Disconnected from server');
		// UI can show offline indicator
	}
});
```

---

## 3. Offline Queue Pattern

### 3.1 Queue Structure

ColaNode maintains an **offline queue** in SQLite for non-CRDT operations (messages, file metadata, etc.):

```sql
CREATE TABLE sync_queue (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    workspace_id TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    action TEXT NOT NULL,          -- 'create', 'update', 'delete'
    version INTEGER NOT NULL,
    payload TEXT,                  -- JSON snapshot
    blob_hashes TEXT,              -- JSON array of blob hashes
    priority INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    attempts INTEGER NOT NULL DEFAULT 0,
    last_attempt_at TEXT,
    last_error TEXT,
    synced_at TEXT,                -- NULL = pending, timestamp = synced

    UNIQUE(workspace_id, entity_type, entity_id)
);
```

### 3.2 Queueing Changes

```typescript
class SyncQueue {
	async enqueueChange(change: SyncChange): Promise<void> {
		await this.db.run(
			`INSERT INTO sync_queue (
        workspace_id, entity_type, entity_id, action, version,
        payload, blob_hashes, priority, created_at, attempts
      ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
      ON CONFLICT(workspace_id, entity_type, entity_id) DO UPDATE SET
        action = excluded.action,
        version = excluded.version,
        payload = excluded.payload,
        synced_at = NULL,  -- Reset sync status
        last_error = NULL,
        attempts = 0`,
			[
				change.workspace_id,
				change.entity_type,
				change.entity_id,
				change.action,
				change.version,
				JSON.stringify(change.payload),
				JSON.stringify(change.blob_hashes || []),
				change.priority || 0,
				new Date().toISOString(),
				0 // Initial attempts
			]
		);
	}
}
```

---

## 4. Retry Logic with Exponential Backoff

### 4.1 Retry Strategy

ColaNode uses **exponential backoff with jitter** for retrying failed sync operations:

```typescript
class SyncWorker {
	private readonly MAX_ATTEMPTS = 10;
	private readonly INITIAL_DELAY_MS = 1000; // 1 second
	private readonly MAX_DELAY_MS = 30000; // 30 seconds
	private readonly BACKOFF_MULTIPLIER = 2;

	/**
	 * Calculate retry delay with exponential backoff and jitter
	 */
	private calculateRetryDelay(attempts: number): number {
		// Exponential backoff: delay = INITIAL_DELAY * (BACKOFF_MULTIPLIER ^ attempts)
		const exponentialDelay = this.INITIAL_DELAY_MS * Math.pow(this.BACKOFF_MULTIPLIER, attempts);

		// Cap at max delay
		const cappedDelay = Math.min(exponentialDelay, this.MAX_DELAY_MS);

		// Add jitter (±25% random variation) to prevent thundering herd
		const jitter = cappedDelay * 0.25 * (Math.random() * 2 - 1); // -0.25 to +0.25

		return Math.max(100, cappedDelay + jitter); // Minimum 100ms
	}

	/**
	 * Retry delays for each attempt:
	 * Attempt 1: ~1s (1000ms ± 250ms)
	 * Attempt 2: ~2s (2000ms ± 500ms)
	 * Attempt 3: ~4s (4000ms ± 1000ms)
	 * Attempt 4: ~8s (8000ms ± 2000ms)
	 * Attempt 5: ~16s (16000ms ± 4000ms)
	 * Attempt 6+: ~30s (30000ms ± 7500ms) [capped]
	 */
}
```

### 4.2 Retry Call Stack

```
┌─────────────────────────────────────────────────────────────────────┐
│  Sync Queue Worker: Process Pending Changes                         │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  SELECT * FROM sync_queue                                    │  │
│  │  WHERE workspace_id = ? AND synced_at IS NULL                │  │
│  │  ORDER BY priority DESC, created_at ASC                      │  │
│  │  LIMIT 100                                                    │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  FOR each pending change:                                           │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  Try to sync change to server                                │  │
│  │  - POST /api/sync/push                                       │  │
│  │  - Or: Apply to Yjs document (for CRDT)                      │  │
│  └──────────────────────┬───────────────────────────────────────┘  │
└─────────────────────────┼──────────────────────────────────────────┘
                          │
                          ├─────────────────────┬─────────────────────┐
                          ▼                     ▼                     ▼
        ┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐
        │  Success:            │  │  Transient Error:    │  │  Permanent Error:    │
        │  - Mark as synced    │  │  - Network timeout   │  │  - 400 Bad Request   │
        │  - UPDATE sync_queue │  │  - 503 Service       │  │  - 401 Unauthorized  │
        │    SET synced_at = ? │  │    Unavailable       │  │  - 404 Not Found     │
        │  - Continue to next  │  │  - Connection error  │  │  - 422 Validation    │
        │    change            │  │                      │  │                      │
        │                      │  │  - Increment attempts│  │  - Mark as failed    │
        │                      │  │  - Calculate delay   │  │  - Stop retrying     │
        │                      │  │  - Schedule retry    │  │  - Queue for manual  │
        │                      │  │                      │  │    resolution        │
        └──────────────────────┘  └──────────────────────┘  └──────────────────────┘
                          │
                          │ Retry after delay
                          ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Schedule Retry (Exponential Backoff)                               │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  const delay = calculateRetryDelay(entry.attempts);          │  │
│  │  setTimeout(() => {                                          │  │
│  │    processSyncQueue(); // Retry this change                  │  │
│  │  }, delay);                                                  │  │
│  │                                                               │  │
│  │  UPDATE sync_queue                                           │  │
│  │  SET attempts = attempts + 1,                                │  │
│  │      last_attempt_at = NOW(),                                │  │
│  │      last_error = ?                                          │  │
│  │  WHERE id = ?                                                │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 5. Error Classification: Transient vs Permanent

### 5.1 Error Classification Logic

ColaNode classifies errors to determine retry strategy:

```typescript
enum ErrorType {
	Transient = 'transient', // Retry with exponential backoff
	Permanent = 'permanent', // Don't retry, queue for manual resolution
	Network = 'network' // Retry when network is available
}

function classifyError(error: Error): ErrorType {
	// Network errors (transient)
	if (
		error.message.includes('ECONNREFUSED') ||
		error.message.includes('ETIMEDOUT') ||
		error.message.includes('ENOTFOUND') ||
		error.message.includes('network') ||
		error.message.includes('timeout')
	) {
		return ErrorType.Network;
	}

	// HTTP 5xx errors (transient - server issues)
	if (error.status >= 500 && error.status < 600) {
		return ErrorType.Transient;
	}

	// HTTP 429 Too Many Requests (transient - rate limiting)
	if (error.status === 429) {
		return ErrorType.Transient;
	}

	// HTTP 4xx errors (permanent - client errors)
	if (error.status >= 400 && error.status < 500) {
		// Except 408 Request Timeout (transient)
		if (error.status === 408) {
			return ErrorType.Transient;
		}
		return ErrorType.Permanent;
	}

	// Connection errors (transient)
	if (error.code === 'ECONNRESET' || error.code === 'EPIPE') {
		return ErrorType.Transient;
	}

	// Default: treat as transient (safer to retry)
	return ErrorType.Transient;
}
```

### 5.2 Retry Decision Logic

```typescript
class SyncWorker {
	async processChange(entry: SyncQueueEntry): Promise<void> {
		// Check if max attempts reached
		if (entry.attempts >= this.MAX_ATTEMPTS) {
			// Max attempts reached - mark as permanently failed
			await this.markPermanentlyFailed(entry.id, 'Max retry attempts reached');
			return;
		}

		try {
			// Attempt to sync
			await this.syncToServer(entry);

			// Success - mark as synced
			await this.markSynced(entry.id);
		} catch (error) {
			const errorType = classifyError(error);

			if (errorType === ErrorType.Permanent) {
				// Permanent error - don't retry
				await this.markPermanentlyFailed(entry.id, error.message);
				return;
			}

			// Transient or network error - retry with exponential backoff
			const delay = this.calculateRetryDelay(entry.attempts);

			await this.markFailed(entry.id, error.message);

			// Schedule retry
			setTimeout(() => {
				this.processChange(entry);
			}, delay);
		}
	}
}
```

---

## 6. Network Status Monitoring

### 6.1 Online/Offline Event Handling

ColaNode monitors network status and adjusts sync behavior:

```typescript
class SyncService {
	private isOnline: boolean = navigator.onLine;

	constructor() {
		// Listen for online/offline events
		window.addEventListener('online', () => {
			console.log('Network online - resuming sync');
			this.isOnline = true;
			this.resumeSync();
		});

		window.addEventListener('offline', () => {
			console.log('Network offline - pausing sync');
			this.isOnline = false;
			this.pauseSync();
		});

		// Also listen for WebSocket connection status
		this.wsProvider?.on('status', (event) => {
			if (event.status === 'connected') {
				this.isOnline = true;
				this.resumeSync();
			} else if (event.status === 'disconnected') {
				// WebSocket disconnected, but network might still be up
				// (could be server issue, not network issue)
				this.handleDisconnection();
			}
		});
	}

	private async resumeSync(): Promise<void> {
		// Process offline queue
		await this.processOfflineQueue();

		// Resume periodic sync
		this.startPeriodicSync();
	}

	private pauseSync(): void {
		// Stop periodic sync
		this.stopPeriodicSync();

		// Changes will be queued locally
	}
}
```

### 6.2 Network-Aware Queue Processing

```typescript
class SyncWorker {
	async processQueue(): Promise<void> {
		// Only process if online
		if (!this.isOnline) {
			console.log('Offline - skipping queue processing');
			return;
		}

		// Check WebSocket connection status
		if (this.wsProvider && this.wsProvider.ws?.readyState !== WebSocket.OPEN) {
			console.log('WebSocket not connected - skipping queue processing');
			return;
		}

		// Get pending changes
		const pending = await this.getPendingChanges();

		for (const entry of pending) {
			// Check if enough time has passed since last attempt
			if (entry.last_attempt_at) {
				const lastAttempt = new Date(entry.last_attempt_at);
				const delay = this.calculateRetryDelay(entry.attempts);
				const nextAttemptTime = new Date(lastAttempt.getTime() + delay);

				if (new Date() < nextAttemptTime) {
					// Not time to retry yet - skip
					continue;
				}
			}

			// Process change
			await this.processChange(entry);
		}
	}
}
```

---

## 7. Complete Retry Flow: Example Scenario

### 7.1 Scenario: Network Failure During Sync

```
┌─────────────────────────────────────────────────────────────────────┐
│  User Creates Well (Offline)                                        │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  1. INSERT INTO wells (...)                                  │  │
│  │  2. INSERT INTO sync_queue (entity_type='well', ...)         │  │
│  │  3. UI updates immediately (local-first)                     │  │
│  └──────────────────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ User comes online
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Sync Worker: Attempt 1                                             │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  POST /api/sync/push                                         │  │
│  │  Error: ECONNREFUSED (server not reachable)                  │  │
│  │  Classify: Network error (transient)                         │  │
│  │  Delay: 1s (1000ms ± 250ms jitter)                          │  │
│  │  UPDATE sync_queue SET attempts=1, last_error=...            │  │
│  └──────────────────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ Wait ~1 second
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Sync Worker: Attempt 2                                             │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  POST /api/sync/push                                         │  │
│  │  Error: ETIMEDOUT (request timeout)                          │  │
│  │  Classify: Network error (transient)                         │  │
│  │  Delay: 2s (2000ms ± 500ms jitter)                          │  │
│  │  UPDATE sync_queue SET attempts=2, last_error=...            │  │
│  └──────────────────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ Wait ~2 seconds
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Sync Worker: Attempt 3                                             │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  POST /api/sync/push                                         │  │
│  │  Success: 200 OK                                             │  │
│  │  UPDATE sync_queue SET synced_at=NOW()                       │  │
│  │  Well successfully synced!                                   │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

### 7.2 Scenario: Permanent Error (No Retry)

```
┌─────────────────────────────────────────────────────────────────────┐
│  User Creates Well (Invalid Data)                                   │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  1. INSERT INTO wells (name=NULL, ...)  -- Invalid           │  │
│  │  2. INSERT INTO sync_queue (...)                              │  │
│  └──────────────────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────────────────┘
                       │
                       │ Sync attempt
                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Sync Worker: Attempt 1                                             │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  POST /api/sync/push                                         │  │
│  │  Error: 422 Validation Error (name is required)              │  │
│  │  Classify: Permanent error (client error)                    │  │
│  │  Action: Mark as permanently failed                          │  │
│  │  UPDATE sync_queue SET                                      │  │
│  │    last_error='422: Validation error',                       │  │
│  │    attempts=1                                                │  │
│  │  -- Don't retry, queue for manual resolution                 │  │
│  └──────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 8. Implementation: Complete Sync Worker

### 8.1 Sync Worker with Retry Logic

```typescript
class SyncWorker {
	private readonly MAX_ATTEMPTS = 10;
	private readonly INITIAL_DELAY_MS = 1000;
	private readonly MAX_DELAY_MS = 30000;
	private readonly BACKOFF_MULTIPLIER = 2;
	private isProcessing: boolean = false;
	private processingInterval: NodeJS.Timeout | null = null;

	constructor(
		private db: Database,
		private syncClient: SyncClient,
		private wsProvider: WebsocketProvider | null
	) {
		// Start periodic processing
		this.startPeriodicProcessing();
	}

	/**
	 * Start periodic queue processing
	 */
	private startPeriodicProcessing(): void {
		// Process queue every 5 seconds
		this.processingInterval = setInterval(() => {
			this.processQueue().catch((error) => {
				console.error('Error processing sync queue:', error);
			});
		}, 5000);

		// Also process immediately
		this.processQueue();
	}

	/**
	 * Process pending changes in sync queue
	 */
	async processQueue(): Promise<void> {
		// Prevent concurrent processing
		if (this.isProcessing) {
			return;
		}

		// Check if online
		if (!navigator.onLine) {
			return;
		}

		// Check WebSocket connection (for CRDT sync)
		if (this.wsProvider && this.wsProvider.ws?.readyState !== WebSocket.OPEN) {
			return;
		}

		this.isProcessing = true;

		try {
			// Get pending changes
			const pending = await this.getPendingChanges();

			for (const entry of pending) {
				// Check if enough time has passed since last attempt
				if (entry.last_attempt_at) {
					const lastAttempt = new Date(entry.last_attempt_at);
					const delay = this.calculateRetryDelay(entry.attempts);
					const nextAttemptTime = new Date(lastAttempt.getTime() + delay);

					if (new Date() < nextAttemptTime) {
						// Not time to retry yet
						continue;
					}
				}

				// Process change
				await this.processChange(entry);
			}
		} finally {
			this.isProcessing = false;
		}
	}

	/**
	 * Process a single sync queue entry
	 */
	private async processChange(entry: SyncQueueEntry): Promise<void> {
		// Check max attempts
		if (entry.attempts >= this.MAX_ATTEMPTS) {
			await this.markPermanentlyFailed(entry.id, 'Max retry attempts reached');
			return;
		}

		try {
			// Attempt to sync
			await this.syncToServer(entry);

			// Success - mark as synced
			await this.markSynced(entry.id);
		} catch (error) {
			const errorType = this.classifyError(error);

			if (errorType === ErrorType.Permanent) {
				// Permanent error - don't retry
				await this.markPermanentlyFailed(entry.id, error.message);
				return;
			}

			// Transient error - mark as failed and will retry later
			await this.markFailed(entry.id, error.message);
		}
	}

	/**
	 * Calculate retry delay with exponential backoff and jitter
	 */
	private calculateRetryDelay(attempts: number): number {
		const exponentialDelay = this.INITIAL_DELAY_MS * Math.pow(this.BACKOFF_MULTIPLIER, attempts);
		const cappedDelay = Math.min(exponentialDelay, this.MAX_DELAY_MS);
		const jitter = cappedDelay * 0.25 * (Math.random() * 2 - 1);
		return Math.max(100, cappedDelay + jitter);
	}

	/**
	 * Classify error type
	 */
	private classifyError(error: any): ErrorType {
		// Network errors
		if (
			error.message?.includes('ECONNREFUSED') ||
			error.message?.includes('ETIMEDOUT') ||
			error.message?.includes('ENOTFOUND') ||
			error.code === 'ECONNRESET' ||
			error.code === 'EPIPE'
		) {
			return ErrorType.Network;
		}

		// HTTP 5xx (server errors - transient)
		if (error.status >= 500 && error.status < 600) {
			return ErrorType.Transient;
		}

		// HTTP 429 (rate limiting - transient)
		if (error.status === 429) {
			return ErrorType.Transient;
		}

		// HTTP 408 (timeout - transient)
		if (error.status === 408) {
			return ErrorType.Transient;
		}

		// HTTP 4xx (client errors - permanent)
		if (error.status >= 400 && error.status < 500) {
			return ErrorType.Permanent;
		}

		// Default: transient (safer to retry)
		return ErrorType.Transient;
	}

	/**
	 * Mark entry as synced
	 */
	private async markSynced(id: number): Promise<void> {
		await this.db.run('UPDATE sync_queue SET synced_at = ? WHERE id = ?', [
			new Date().toISOString(),
			id
		]);
	}

	/**
	 * Mark entry as failed (will retry)
	 */
	private async markFailed(id: number, error: string): Promise<void> {
		await this.db.run(
			`UPDATE sync_queue
       SET attempts = attempts + 1,
           last_attempt_at = ?,
           last_error = ?
       WHERE id = ?`,
			[new Date().toISOString(), error, id]
		);
	}

	/**
	 * Mark entry as permanently failed (won't retry)
	 */
	private async markPermanentlyFailed(id: number, error: string): Promise<void> {
		await this.db.run(
			`UPDATE sync_queue
       SET attempts = attempts + 1,
           last_attempt_at = ?,
           last_error = ?,
           synced_at = NULL  -- Keep as pending for manual resolution
       WHERE id = ?`,
			[new Date().toISOString(), `PERMANENT: ${error}`, id]
		);
	}
}
```

---

## 9. CRDT Sync: y-websocket Retry

### 9.1 Yjs Updates Are Idempotent

For CRDT-managed data (pages, collaborative records), Yjs updates are **idempotent**:

- **No explicit retry needed**: Yjs operations are commutative and idempotent.
- **Automatic buffering**: `WebsocketProvider` buffers updates while disconnected.
- **Automatic replay**: When reconnected, buffered updates are sent automatically.

```typescript
// User edits page while offline
ydoc.getText('content').insert(0, 'Hello');

// y-websocket buffers this update
// When reconnected, update is automatically sent
// No explicit retry logic needed for Yjs updates
```

### 9.2 Yjs + SQLite Sync Pattern

For entities that combine Yjs (CRDT) and SQLite (metadata):

```typescript
class PageService {
	async updatePage(pageId: string, content: string): Promise<void> {
		// 1. Update Yjs (CRDT - automatic sync via WebSocket)
		const ydoc = getYDoc(pageId);
		ydoc.getText('content').insert(0, content);
		// y-websocket handles sync automatically (with retry)

		// 2. Update SQLite metadata (for queries/search)
		await this.db.run(`UPDATE pages SET updated_at = ?, title = ? WHERE id = ?`, [
			new Date().toISOString(),
			extractTitle(content),
			pageId
		]);
		// This is local-only, no sync needed (derived from Yjs)
	}
}
```

---

## 10. Design Patterns for DataForge

### 10.1 Multi-Layer Resilience

**ColaNode Pattern**:

- Layer 1: Local-first (always works offline).
- Layer 2: Offline queue (persistent, survives restarts).
- Layer 3: Automatic reconnection (y-websocket).
- Layer 4: Retry logic (exponential backoff).

**DataForge Application**:

- Use same multi-layer approach.
- SQLite sync_queue for non-CRDT operations.
- Exponential backoff for failed sync operations.
- Network status monitoring.

### 10.2 Error Classification

**ColaNode Pattern**:

- Classify errors as transient vs permanent.
- Retry transient errors with exponential backoff.
- Don't retry permanent errors (queue for manual resolution).

**DataForge Application**:

- Classify HTTP errors (5xx = transient, 4xx = permanent).
- Classify network errors (timeout, connection refused = transient).
- Handle rate limiting (429 = transient, retry with longer delay).

### 10.3 Exponential Backoff with Jitter

**ColaNode Pattern**:

- Exponential backoff: delay = initial \* (multiplier ^ attempts).
- Jitter: random variation to prevent thundering herd.
- Max delay cap: prevent excessive delays.

**DataForge Application**:

- Use same exponential backoff formula.
- Add jitter (±25% random variation).
- Cap at reasonable max (e.g., 30 seconds).

### 10.4 Network Status Monitoring

**ColaNode Pattern**:

- Listen for `online`/`offline` events.
- Monitor WebSocket connection status.
- Pause sync when offline, resume when online.

**DataForge Application**:

- Use `navigator.onLine` API.
- Monitor HTTP client connection status.
- Pause sync queue processing when offline.

---

## 11. Summary

### ✅ Key Patterns

1. **Multi-Layer Resilience**: Local-first + offline queue + auto-reconnection + retry logic.
2. **Exponential Backoff**: Gradually increase delay between retries (1s → 2s → 4s → 8s → ...).
3. **Jitter**: Random variation to prevent thundering herd.
4. **Error Classification**: Distinguish transient (retry) from permanent (don't retry).
5. **Network Monitoring**: Listen for online/offline events, pause/resume sync accordingly.

### 📋 Best Practices

1. **Always save locally first** (local-first).
2. **Queue changes for sync** (offline queue).
3. **Use exponential backoff** (1s, 2s, 4s, 8s, ...).
4. **Add jitter** (±25% random variation).
5. **Classify errors** (transient vs permanent).
6. **Monitor network status** (online/offline events).
7. **Cap retry attempts** (e.g., max 10 attempts).
8. **Track attempts and errors** (for debugging and manual resolution).

This architecture provides a robust foundation for handling transient network failures while maintaining data consistency and user experience.

