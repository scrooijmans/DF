# ColaNode Architecture & Tech Stack

This document explains ColaNode's tech stack, local-first architecture, CRDT implementation, and collaboration mechanisms based on the [ColaNode GitHub repository](https://github.com/colanode/colanode).

## Overview

ColaNode is an open-source, local-first collaboration workspace (Slack and Notion alternative) that prioritizes data privacy and control. It uses a **local-first architecture** where all changes are saved to a local SQLite database first, then synced to a self-hosted server in the background.

## Tech Stack

### Frontend (Client)

- **TypeScript** (98.6% of codebase)
- **Electron** - Desktop application framework
- **Web App** - Browser-based client
- **SQLite** - Local database for offline-first storage
- **Yjs** - CRDT library for conflict-free collaborative editing

### Backend (Server)

- **Node.js/TypeScript** - Server runtime
- **PostgreSQL** with **pgvector** extension - Primary database
- **Redis** (or Valkey) - Caching and real-time features
- **Storage Backends**:
  - Local filesystem (default)
  - S3-compatible storage
  - Google Cloud Storage
  - Azure Blob Storage
- **WebSocket Server** - Real-time collaboration sync

### Infrastructure

- **Docker Compose** - Local development and deployment
- **Kubernetes** - Production deployments (with Helm charts)
- **Self-hosted** - Full control over data and infrastructure

## Local Database: SQLite

ColaNode uses **SQLite** as the local database for the client application. This enables:

- **Offline-first operation**: All data is stored locally
- **Immediate persistence**: Changes are saved instantly without network latency
- **Fast reads**: All queries happen against local SQLite database
- **Background sync**: Synchronization happens asynchronously

### Local-First Workflow

```
┌─────────────────────────────────────────────────────────────┐
│                    User Action                               │
│  Edit page, send message, update database record            │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│              Local SQLite Database                           │
│  - Write change immediately                                  │
│  - Update local state                                        │
│  - UI updates instantly                                      │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       │ Background Sync Process
                       ▼
┌─────────────────────────────────────────────────────────────┐
│              Server (PostgreSQL)                             │
│  - Sync changes when online                                  │
│  - Handle conflicts                                          │
│  - Distribute to other clients                               │
└─────────────────────────────────────────────────────────────┘
```

**Key Benefits**:
- ✅ **Zero-latency UI**: Changes appear instantly
- ✅ **Offline support**: Works without network connection
- ✅ **Data ownership**: All data stored locally
- ✅ **Background sync**: Network operations don't block UI

## CRDT: Yjs

ColaNode uses **Yjs** (Yjs CRDT framework) for conflict-free collaborative editing. Yjs provides:

- **Automatic conflict resolution**: Multiple users can edit simultaneously
- **Real-time collaboration**: Changes merge automatically
- **Efficient sync**: Only deltas (changes) are transmitted
- **Rich data types**: Maps, Arrays, Text, XML fragments

### Yjs Data Types Used

```typescript
// Yjs Document Structure
const ydoc = new Y.Doc();

// For pages and rich text content
const ytext = ydoc.getText('content');

// For database records and structured data
const ymap = ydoc.getMap('record');

// For lists (e.g., database rows)
const yarray = ydoc.getArray('items');
```

### What Uses CRDTs in ColaNode

According to the README:
- ✅ **Pages** - Rich text documents (Notion-like)
- ✅ **Database Records** - Structured data entries
- ❌ **Messages** - Use simpler database tables (no concurrent editing)
- ❌ **File Operations** - Use simpler database tables

## Server Communication & Sync Architecture

### WebSocket-Based Sync

ColaNode uses **y-websocket** (Yjs WebSocket provider) for real-time synchronization:

```typescript
import * as Y from 'yjs';
import { WebsocketProvider } from 'y-websocket';
import { IndexeddbPersistence } from 'y-indexeddb';

// Create Yjs document
const ydoc = new Y.Doc();

// Local persistence (IndexedDB in browser, SQLite in Electron)
const indexeddbProvider = new IndexeddbPersistence('document-id', ydoc);

// WebSocket sync to server
const wsProvider = new WebsocketProvider(
  'wss://server.colanode.com',
  'workspace-id/document-id',
  ydoc
);

// Listen for sync status
wsProvider.on('status', (event) => {
  console.log('Connection status:', event.status); // 'connected' | 'disconnected'
});

wsProvider.on('sync', (isSynced) => {
  if (isSynced) {
    console.log('Document synced with server');
  }
});
```

### Server Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    ColaNode Server                           │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │  WebSocket Server (y-websocket-server)             │    │
│  │  - Handles Yjs document sync                        │    │
│  │  - Manages room/workspace connections               │    │
│  │  - Broadcasts updates to connected clients          │    │
│  └────────────────────┬───────────────────────────────┘    │
│                       │                                      │
│  ┌────────────────────▼───────────────────────────────┐    │
│  │  Redis                                              │    │
│  │  - Caching                                          │    │
│  │  - Pub/Sub for real-time updates                    │    │
│  └────────────────────┬───────────────────────────────┘    │
│                       │                                      │
│  ┌────────────────────▼───────────────────────────────┐    │
│  │  PostgreSQL (with pgvector)                        │    │
│  │  - Persistent storage                               │    │
│  │  - User accounts, permissions                       │    │
│  │  - Workspace metadata                               │    │
│  │  - Yjs document snapshots                           │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │  Storage Backend (S3/Filesystem/GCS/Azure)         │    │
│  │  - User-uploaded files                              │    │
│  │  - Attachments                                      │    │
│  └────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## Call Stack: Local-First Data Maintenance

### 1. User Edits a Page

```
User Types in Editor
    ↓
┌─────────────────────────────────────────────────────────────┐
│  Frontend: Yjs Document Change                              │
│  ydoc.getText('content').insert(index, text)                │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  Local SQLite: Immediate Write                              │
│  INSERT INTO local_changes (doc_id, change_data, timestamp) │
│  - Change persisted locally                                 │
│  - UI updates instantly                                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  IndexedDB Persistence (Browser)                            │
│  IndexeddbPersistence saves Yjs document state              │
│  - Enables offline editing                                  │
│  - Fast document loading                                    │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       │ Background Process (Non-blocking)
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  WebSocket Provider: Sync to Server                         │
│  wsProvider sends Yjs update to server                      │
│  - Only sends delta (change), not full document             │
│  - Handles reconnection automatically                       │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  Server: Process Update                                     │
│  1. Receive Yjs update via WebSocket                        │
│  2. Apply to server's Yjs document                          │
│  3. Store snapshot in PostgreSQL                            │
│  4. Broadcast to other connected clients                    │
└─────────────────────────────────────────────────────────────┘
```

### 2. Reading Data (Local-First)

```
User Opens Page
    ↓
┌─────────────────────────────────────────────────────────────┐
│  Frontend: Query Local SQLite                               │
│  SELECT * FROM pages WHERE id = ?                           │
│  - Instant response (no network latency)                    │
│  - Works offline                                            │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       │ Background: Check for Updates
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  WebSocket: Request Latest State                            │
│  - If online, sync latest changes from server               │
│  - Merge with local state using Yjs CRDT                    │
│  - Update local SQLite with merged state                    │
└─────────────────────────────────────────────────────────────┘
```

## Call Stack: Collaboration Between Users

### 1. Two Users Editing Same Page

```
User A Types "Hello"
    ↓
┌─────────────────────────────────────────────────────────────┐
│  User A: Local Change                                       │
│  1. Yjs: ydoc.getText('content').insert(0, 'Hello')        │
│  2. SQLite: Save change locally                             │
│  3. UI: Display "Hello" immediately                         │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       │ WebSocket: Send Update
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  Server: Receive & Process                                  │
│  1. Apply Yjs update to server document                     │
│  2. Store in PostgreSQL                                     │
│  3. Broadcast to all connected clients (including User B)   │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       │ WebSocket: Broadcast Update
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  User B: Receive Update                                     │
│  1. Yjs: Automatically merge update into local document     │
│  2. SQLite: Update local database                           │
│  3. UI: Display "Hello" in real-time                        │
│  - No conflicts! CRDT handles merge automatically           │
└─────────────────────────────────────────────────────────────┘

User B Types " World" (at position 5)
    ↓
┌─────────────────────────────────────────────────────────────┐
│  User B: Local Change                                       │
│  1. Yjs: ydoc.getText('content').insert(5, ' World')       │
│  2. SQLite: Save change locally                             │
│  3. UI: Display "Hello World" immediately                   │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       │ WebSocket: Send Update
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  Server: Receive & Broadcast                                │
│  - Broadcasts to User A                                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  User A: Receive Update                                     │
│  - Yjs merges " World" into existing "Hello"                │
│  - Result: "Hello World" (automatic conflict resolution)    │
│  - Both users see same final state                          │
└─────────────────────────────────────────────────────────────┘
```

### 2. Awareness API (Presence Indicators)

Yjs provides an **Awareness API** for sharing user-specific information:

```typescript
import * as awarenessProtocol from 'y-protocols/awareness';

// Create awareness instance
const awareness = new awarenessProtocol.Awareness(ydoc);

// Set local user state (cursor position, name, color)
awareness.setLocalStateField('user', {
  name: 'John Doe',
  color: '#ff0000',
  cursor: { index: 10 }
});

// Listen for remote user states
awareness.on('change', (changes) => {
  // Other users' cursor positions, names, etc.
  const states = awareness.getStates();
  states.forEach((state, clientId) => {
    if (clientId !== awareness.clientID) {
      console.log('Remote user:', state.user);
      // Update UI to show other users' cursors
    }
  });
});

// Attach awareness to WebSocket provider
const wsProvider = new WebsocketProvider(serverUrl, room, ydoc, {
  awareness: awareness
});
```

**Awareness Flow**:
```
User A Moves Cursor
    ↓
awareness.setLocalStateField('cursor', { index: 15 })
    ↓
WebSocket: Broadcast Awareness Update
    ↓
Server: Forward to Other Clients
    ↓
User B: Receive Awareness Update
    ↓
UI: Show User A's Cursor at Position 15
```

## Complete Collaboration Flow Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│  User A (Client 1)                  User B (Client 2)               │
│                                                                      │
│  ┌──────────────────────┐          ┌──────────────────────┐        │
│  │  Yjs Document (Local)│          │  Yjs Document (Local)│        │
│  │  - Y.Text            │          │  - Y.Text            │        │
│  │  - Y.Map             │          │  - Y.Map             │        │
│  └──────────┬───────────┘          └──────────┬───────────┘        │
│             │                                  │                    │
│  ┌──────────▼───────────┐          ┌──────────▼───────────┐        │
│  │  SQLite (Local)      │          │  SQLite (Local)      │        │
│  │  - Immediate writes  │          │  - Immediate writes  │        │
│  └──────────┬───────────┘          └──────────┬───────────┘        │
│             │                                  │                    │
│  ┌──────────▼───────────┐          ┌──────────▼───────────┐        │
│  │  IndexedDB           │          │  IndexedDB           │        │
│  │  (Persistence)       │          │  (Persistence)       │        │
│  └──────────┬───────────┘          └──────────┬───────────┘        │
│             │                                  │                    │
│             │ WebSocket                        │ WebSocket          │
│             │ (y-websocket)                    │ (y-websocket)      │
│             └──────────┬───────────────────────┘                    │
│                        │                                            │
│                        ▼                                            │
│        ┌───────────────────────────────────────┐                   │
│        │  ColaNode Server                      │                   │
│        │                                       │                   │
│        │  ┌─────────────────────────────────┐ │                   │
│        │  │  WebSocket Server               │ │                   │
│        │  │  - y-websocket-server           │ │                   │
│        │  │  - Room management              │ │                   │
│        │  │  - Update broadcasting          │ │                   │
│        │  └──────────┬──────────────────────┘ │                   │
│        │             │                         │                   │
│        │  ┌──────────▼──────────────────────┐ │                   │
│        │  │  Redis                          │ │                   │
│        │  │  - Pub/Sub                      │ │                   │
│        │  │  - Caching                      │ │                   │
│        │  └──────────┬──────────────────────┘ │                   │
│        │             │                         │                   │
│        │  ┌──────────▼──────────────────────┐ │                   │
│        │  │  PostgreSQL                     │ │                   │
│        │  │  - Yjs document snapshots       │ │                   │
│        │  │  - User accounts                │ │                   │
│        │  │  - Workspace metadata           │ │                   │
│        │  └─────────────────────────────────┘ │                   │
│        └───────────────────────────────────────┘                   │
└─────────────────────────────────────────────────────────────────────┘
```

## Key Architecture Patterns

### 1. Local-First Pattern

**Principle**: All operations work locally first, sync happens in background.

```typescript
// User edits page
function editPage(pageId: string, content: string) {
  // 1. Update local SQLite immediately
  localDB.update('pages', { id: pageId, content });
  
  // 2. Update Yjs document (for collaboration)
  const ydoc = getYDoc(pageId);
  ydoc.getText('content').delete(0, ydoc.getText('content').length);
  ydoc.getText('content').insert(0, content);
  
  // 3. UI updates instantly (no waiting)
  updateUI(pageId, content);
  
  // 4. Background sync (non-blocking)
  syncToServer(pageId); // Async, doesn't block
}
```

### 2. CRDT Merge Pattern

**Yjs automatically handles concurrent edits**:

```typescript
// User A: Inserts "Hello" at position 0
ydoc.getText('content').insert(0, 'Hello');
// Local state: "Hello"

// User B (simultaneously): Inserts "Hi" at position 0
ydoc.getText('content').insert(0, 'Hi');
// Local state: "Hi"

// When updates sync:
// Yjs CRDT automatically merges based on logical timestamps
// Result: "HiHello" or "HelloHi" (deterministic based on CRDT algorithm)
// Both users see the same final state
```

### 3. Offline Queue Pattern

```typescript
// When offline, changes are queued locally
function syncToServer(pageId: string) {
  if (!navigator.onLine) {
    // Queue for later sync
    offlineQueue.push({ type: 'update', pageId });
    return;
  }
  
  // Send via WebSocket
  wsProvider.sendUpdate(ydoc);
}

// When coming back online
window.addEventListener('online', () => {
  // Process queued changes
  offlineQueue.forEach(change => {
    syncToServer(change.pageId);
  });
  offlineQueue = [];
});
```

## Server-Side Storage

### PostgreSQL Schema (Inferred)

```sql
-- Yjs document snapshots
CREATE TABLE yjs_documents (
  id UUID PRIMARY KEY,
  workspace_id UUID NOT NULL,
  document_type TEXT NOT NULL, -- 'page', 'database_record', etc.
  snapshot BYTEA, -- Yjs document binary snapshot
  version BIGINT,
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Operation log (for recovery)
CREATE TABLE yjs_operations (
  id BIGSERIAL PRIMARY KEY,
  document_id UUID REFERENCES yjs_documents(id),
  operation_data BYTEA, -- Yjs update binary
  client_id UUID,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Workspace metadata
CREATE TABLE workspaces (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

-- User accounts
CREATE TABLE users (
  id UUID PRIMARY KEY,
  email TEXT UNIQUE NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Redis Usage

- **Pub/Sub**: Real-time message broadcasting
- **Caching**: Frequently accessed data
- **Session management**: WebSocket connections

## Deployment Architecture

### Self-Hosted Setup

```yaml
# docker-compose.yaml (simplified)
services:
  postgres:
    image: pgvector/pgvector:pg16
    environment:
      POSTGRES_DB: colanode
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    volumes:
      - redis_data:/data

  colanode-server:
    image: colanode/server:latest
    environment:
      DATABASE_URL: postgresql://postgres:password@postgres:5432/colanode
      REDIS_URL: redis://redis:6379
      STORAGE_TYPE: filesystem # or s3, gcs, azure
    ports:
      - "3000:3000" # HTTP API
      - "8080:8080" # WebSocket server
    volumes:
      - ./storage:/app/storage
```

## Comparison with Other Local-First Apps

| Feature | ColaNode | Automerge-Repo | PowerSync |
|---------|----------|----------------|-----------|
| **CRDT Library** | Yjs | Automerge | Custom SQLite CRDT |
| **Local DB** | SQLite | IndexedDB | SQLite |
| **Server DB** | PostgreSQL | PostgreSQL | PostgreSQL |
| **Sync Protocol** | WebSocket (Yjs) | WebSocket/Custom | WebSocket (Custom) |
| **Conflict Resolution** | Yjs CRDT | Automerge CRDT | SQLite CRDT |
| **Offline Support** | ✅ Full | ✅ Full | ✅ Full |
| **Real-time Collaboration** | ✅ Yjs Awareness | ✅ Automerge | ⚠️ Eventual |

## Key Takeaways

1. **Local-First**: SQLite stores all data locally, enabling offline operation
2. **Yjs CRDT**: Handles automatic conflict resolution for concurrent edits
3. **WebSocket Sync**: Real-time collaboration via y-websocket provider
4. **Background Sync**: Network operations never block the UI
5. **Self-Hosted**: Full control over data and infrastructure
6. **Hybrid Storage**: SQLite (local) + PostgreSQL (server) + Object Storage (files)

## References

- [ColaNode GitHub Repository](https://github.com/colanode/colanode)
- [Yjs Documentation](https://docs.yjs.dev/)
- [y-websocket Provider](https://github.com/yjs/y-websocket)
- [y-indexeddb Persistence](https://github.com/yjs/y-indexeddb)

