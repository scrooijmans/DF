# ColaNode Sync Server Configuration & Sync Triggers

This document explains how ColaNode determines which server to sync the local database instance to when users come online, how the server URL is configured, and when sync is triggered.

## Overview

ColaNode uses a **local-first architecture** where:

1. All changes are saved to local SQLite immediately
2. Sync to server happens in the background via WebSocket
3. The WebSocket provider handles reconnection automatically
4. Offline changes are queued and synced when connection is restored

## Server URL Configuration

### 1. Server URL Definition

The server URL is configured when initializing the `WebsocketProvider` from `y-websocket`:

```typescript
import { WebsocketProvider } from 'y-websocket';

// Server URL is passed as first parameter
const wsProvider = new WebsocketProvider(
	'wss://server.colanode.com', // ← Server URL (WebSocket endpoint)
	'workspace-id/document-id', // ← Room/workspace identifier
	ydoc // ← Yjs document
);
```

### 2. Configuration Sources

Based on typical local-first application patterns, the server URL would be configured through:

#### **A. Environment Variables (Development/Production)**

```typescript
// .env or environment configuration
const SYNC_SERVER_URL = process.env.SYNC_SERVER_URL || 'wss://server.colanode.com';
const SYNC_SERVER_URL = import.meta.env.VITE_SYNC_SERVER_URL || 'wss://server.colanode.com';
```

#### **B. User Settings (Self-Hosted)**

For self-hosted ColaNode instances, users configure their server URL:

```typescript
// User settings stored in local SQLite
interface UserSettings {
	syncServerUrl: string; // e.g., 'wss://my-colanode.example.com'
	workspaceId: string;
	// ... other settings
}

// Load from local database
const settings = await loadUserSettings();
const serverUrl = settings.syncServerUrl || getDefaultServerUrl();
```

#### **C. Workspace Configuration**

The server URL might be associated with a workspace:

```typescript
// Workspace metadata stored in SQLite
interface Workspace {
	id: string;
	name: string;
	syncServerUrl: string; // Each workspace can have its own server
	// ... other fields
}

// When opening a workspace
const workspace = await getWorkspace(workspaceId);
const wsProvider = new WebsocketProvider(
	workspace.syncServerUrl,
	`${workspace.id}/${documentId}`,
	ydoc
);
```

#### **D. Default Server (Cloud Hosting)**

For cloud-hosted ColaNode, a default server URL is used:

```typescript
const DEFAULT_SYNC_SERVER = 'wss://sync.colanode.com';

function getSyncServerUrl(): string {
	// 1. Check user settings (self-hosted)
	const userSettings = getUserSettings();
	if (userSettings?.syncServerUrl) {
		return userSettings.syncServerUrl;
	}

	// 2. Check workspace configuration
	const workspace = getCurrentWorkspace();
	if (workspace?.syncServerUrl) {
		return workspace.syncServerUrl;
	}

	// 3. Use default (cloud hosting)
	return DEFAULT_SYNC_SERVER;
}
```

## When Sync is Triggered

### 1. Application Initialization

Sync is initialized when the application starts and a workspace/document is opened:

```typescript
class SyncService {
	private wsProvider: WebsocketProvider | null = null;

	async initialize(workspaceId: string, documentId: string) {
		// 1. Get server URL from configuration
		const serverUrl = getSyncServerUrl();

		// 2. Create Yjs document
		const ydoc = new Y.Doc();

		// 3. Set up local persistence (IndexedDB/SQLite)
		const persistence = new IndexeddbPersistence(documentId, ydoc);
		await persistence.whenSynced; // Wait for local data to load

		// 4. Initialize WebSocket provider (starts sync automatically)
		this.wsProvider = new WebsocketProvider(serverUrl, `${workspaceId}/${documentId}`, ydoc, {
			connect: true, // ← Automatically connects when created
			awareness: this.awareness
		});

		// 5. Listen for connection status
		this.wsProvider.on('status', (event) => {
			console.log('Sync status:', event.status); // 'connected' | 'disconnected'
			this.handleSyncStatus(event.status);
		});

		// 6. Listen for sync completion
		this.wsProvider.on('sync', (isSynced) => {
			if (isSynced) {
				console.log('Document synced with server');
				this.onSyncComplete();
			}
		});
	}
}
```

### 2. Online Event Listener

When the user comes back online, sync is automatically resumed:

```typescript
// Listen for online/offline events
window.addEventListener('online', () => {
	console.log('Network connection restored');

	// WebSocket provider automatically reconnects
	// But we can also manually trigger sync for queued changes
	syncService.processOfflineQueue();
});

window.addEventListener('offline', () => {
	console.log('Network connection lost');
	// Changes continue to be saved locally
	// WebSocket provider will queue updates
});
```

### 3. Automatic Reconnection

The `WebsocketProvider` handles reconnection automatically:

```typescript
const wsProvider = new WebsocketProvider(serverUrl, room, ydoc, {
	connect: true
	// Automatic reconnection is built-in
	// Provider will attempt to reconnect on connection loss
});

// Monitor reconnection attempts
wsProvider.on('status', (event) => {
	if (event.status === 'disconnected') {
		console.log('Disconnected, will attempt to reconnect...');
	} else if (event.status === 'connected') {
		console.log('Reconnected to server');
		// Sync any queued changes
		syncService.syncQueuedChanges();
	}
});
```

### 4. Manual Sync Trigger

Sync can also be triggered manually:

```typescript
class SyncService {
	// Force sync of all pending changes
	async syncNow() {
		if (!this.wsProvider) return;

		// Check if connected
		if (this.wsProvider.shouldConnect) {
			// Provider will sync automatically
			// But we can trigger immediate sync for queued changes
			await this.processOfflineQueue();
		}
	}

	// Process offline queue when coming online
	private async processOfflineQueue() {
		const queuedChanges = await this.getQueuedChanges();

		for (const change of queuedChanges) {
			try {
				// Apply change to Yjs document
				// WebSocket provider will automatically sync
				await this.applyQueuedChange(change);
				await this.markChangeAsSynced(change.id);
			} catch (error) {
				console.error('Failed to sync queued change:', error);
				// Keep in queue for retry
			}
		}
	}
}
```

## Complete Sync Initialization Flow

```
┌─────────────────────────────────────────────────────────────┐
│  Application Startup                                         │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  1. Load User Settings from SQLite                          │
│     - Get syncServerUrl (if self-hosted)                    │
│     - Get workspaceId                                        │
│     - Get current documentId                                 │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  2. Determine Server URL                                     │
│     - Check user settings → syncServerUrl                    │
│     - Check workspace config → workspace.syncServerUrl       │
│     - Fallback to default → DEFAULT_SYNC_SERVER             │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  3. Initialize Local Persistence                             │
│     - Create Yjs document                                    │
│     - Load from IndexedDB/SQLite                            │
│     - Wait for local data to load                           │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  4. Initialize WebSocket Provider                            │
│     - new WebsocketProvider(serverUrl, room, ydoc)          │
│     - connect: true (auto-connects)                          │
│     - Sets up automatic reconnection                         │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ├─────────────────────┬─────────────────┐
                       ▼                     ▼                 ▼
        ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
        │  If Online:      │  │  If Offline:     │  │  Connection     │
        │  - Connects      │  │  - Queues        │  │  Lost:          │
        │  - Syncs changes │  │  - Waits for     │  │  - Auto-        │
        │  - Receives      │  │    connection    │  │    reconnect    │
        │    updates       │  │  - Processes     │  │  - Resumes sync │
        └──────────────────┘  │    queue on      │  └──────────────────┘
                              │    reconnect     │
                              └──────────────────┘
```

## Offline Queue Pattern

When offline, changes are queued locally:

```typescript
class OfflineQueue {
	private queue: QueuedChange[] = [];

	async queueChange(change: DocumentChange) {
		// Save to local SQLite queue
		await this.db.run(
			`INSERT INTO sync_queue (entity_type, entity_id, action, payload, created_at)
       VALUES (?, ?, ?, ?, ?)`,
			[change.type, change.id, change.action, JSON.stringify(change.payload), new Date()]
		);

		this.queue.push(change);
	}

	async processQueue() {
		// Only process if online
		if (!navigator.onLine) {
			return;
		}

		const queued = await this.getQueuedChanges();

		for (const change of queued) {
			try {
				// Apply to Yjs document (triggers sync)
				await this.applyChange(change);

				// Mark as synced
				await this.markAsSynced(change.id);
			} catch (error) {
				console.error('Failed to sync change:', error);
				// Keep in queue for retry
			}
		}
	}
}

// Listen for online event
window.addEventListener('online', () => {
	offlineQueue.processQueue();
});
```

## Server URL Resolution Priority

The server URL is determined in this order:

1. **User Settings** (highest priority - self-hosted)
   - Stored in local SQLite `user_settings` table
   - User can configure their own server URL
   - Example: `'wss://my-colanode.example.com'`

2. **Workspace Configuration**
   - Each workspace can have its own sync server
   - Stored in `workspaces` table
   - Example: `'wss://workspace-server.example.com'`

3. **Environment Variable**
   - For development/testing
   - `SYNC_SERVER_URL` or `VITE_SYNC_SERVER_URL`
   - Example: `'ws://localhost:8080'`

4. **Default Server** (lowest priority - cloud hosting)
   - Hardcoded default for cloud-hosted ColaNode
   - Example: `'wss://sync.colanode.com'`

## Implementation Example

```typescript
// src/lib/sync/sync-service.ts
import { WebsocketProvider } from 'y-websocket';
import { IndexeddbPersistence } from 'y-indexeddb';
import * as Y from 'yjs';
import { invoke } from '@tauri-apps/api/core';

class SyncService {
	private wsProvider: WebsocketProvider | null = null;
	private ydoc: Y.Doc | null = null;

	async initialize(workspaceId: string, documentId: string) {
		// 1. Get server URL from backend (stored in SQLite)
		const serverUrl = await invoke<string>('get_sync_server_url', { workspaceId });

		// 2. Create Yjs document
		this.ydoc = new Y.Doc();

		// 3. Load from local persistence
		const persistence = new IndexeddbPersistence(documentId, this.ydoc);
		await persistence.whenSynced;

		// 4. Initialize WebSocket provider
		this.wsProvider = new WebsocketProvider(serverUrl, `${workspaceId}/${documentId}`, this.ydoc, {
			connect: true, // Auto-connect
			params: {
				// Optional: Add auth token
				token: await this.getAuthToken()
			}
		});

		// 5. Set up event listeners
		this.setupEventListeners();
	}

	private setupEventListeners() {
		if (!this.wsProvider) return;

		// Connection status
		this.wsProvider.on('status', (event) => {
			console.log('Sync status:', event.status);
			if (event.status === 'connected') {
				this.onConnected();
			} else if (event.status === 'disconnected') {
				this.onDisconnected();
			}
		});

		// Sync completion
		this.wsProvider.on('sync', (isSynced) => {
			if (isSynced) {
				console.log('Document synced with server');
			}
		});

		// Online/offline events
		window.addEventListener('online', () => {
			console.log('Network online, resuming sync');
			// Provider auto-reconnects, but we can process queue
			this.processOfflineQueue();
		});

		window.addEventListener('offline', () => {
			console.log('Network offline, queuing changes');
		});
	}

	private async processOfflineQueue() {
		// Process any queued changes from offline period
		const queued = await invoke<QueuedChange[]>('get_queued_sync_changes');
		for (const change of queued) {
			// Apply to Yjs document (triggers sync)
			await this.applyQueuedChange(change);
		}
	}
}
```

## Key Takeaways

1. **Server URL Configuration**:
   - Configured via user settings, workspace config, or environment variables
   - Stored in local SQLite database
   - Default fallback for cloud hosting

2. **Sync Initialization**:
   - Happens when app starts and workspace/document is opened
   - WebSocket provider auto-connects when created (`connect: true`)
   - Local data loads first (IndexedDB/SQLite), then syncs with server

3. **Sync Triggers**:
   - **App initialization**: When workspace/document opens
   - **Online event**: When network connection is restored
   - **Automatic reconnection**: WebSocket provider handles this automatically
   - **Manual trigger**: Can force sync of queued changes

4. **Offline Handling**:
   - Changes are saved locally immediately
   - Queued in SQLite `sync_queue` table when offline
   - Processed automatically when connection is restored

5. **Reconnection**:
   - `WebsocketProvider` handles reconnection automatically
   - No manual reconnection logic needed
   - Status events notify when connected/disconnected

## References

- [Yjs WebsocketProvider Documentation](https://github.com/yjs/y-websocket)
- [ColaNode Architecture Tech Stack](./colanode-architecture-tech-stack.md)
- [ColaNode Architecture Assessment](./colanode-architecture-assessment.md)
