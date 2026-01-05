# Storage Technology Recommendation: IndexedDB + Dexie.js

## 🎯 **Executive Summary**

**Recommendation: Use IndexedDB with Dexie.js wrapper for event queue and local cache.**

**Why not raw IndexedDB?** The API is messy (callback-based, manual cursors). Dexie.js provides a clean, promise-based API with SQL-like queries.

**Why not other technologies?**

- ❌ **LocalStorage**: Blocks main thread, 5MB limit, no indexing
- ❌ **OPFS**: Too complex, meant for binary data, slower for JSON
- ❌ **WASM SQLite**: 500ms+ initialization, 939KB bundle, overkill

## 📊 **Performance Comparison**

Based on real-world benchmarks (Chrome 128):

| Operation            | IndexedDB | LocalStorage      | OPFS (Worker) | WASM SQLite |
| -------------------- | --------- | ----------------- | ------------- | ----------- |
| **Init Time**        | 46ms      | Instant           | 27ms          | 500ms+      |
| **Write (single)**   | 0.17ms    | 0.017ms ⚠️ blocks | 1.5ms         | 0.17-3ms    |
| **Read (single)**    | 0.1ms     | 0.005ms ⚠️ blocks | 1.3ms         | 0.45-3ms    |
| **Bulk Write (200)** | 13ms      | 5.8ms ⚠️ blocks   | 104ms         | 19-37ms     |
| **Bulk Read (100)**  | 5ms       | 0.4ms ⚠️ blocks   | 26ms          | 3.6-6ms     |

**Key Insight**: LocalStorage is faster but **blocks the main thread**, making it unsuitable for bulk operations. IndexedDB provides the best balance of speed and non-blocking behavior.

## ✅ **Why IndexedDB + Dexie.js is Perfect for Our Use Case**

### **1. Event Queue Requirements**

**What we need:**

- Store JSON objects (event entries)
- Query by status (`pending`, `processing`, `completed`, `failed`)
- Query by timestamp (oldest first for processing)
- Query by entity type (batch by type)
- Large storage (potentially thousands of events)

**IndexedDB + Dexie.js provides:**

```typescript
// Clean query API (vs. raw IndexedDB cursors)
const pendingEvents = await db.events
  .where("status")
  .equals("pending")
  .and((e) => e.timestamp < Date.now() - 60000) // Older than 1 minute
  .sortBy("timestamp");

// Bulk operations (faster than individual writes)
await db.events.bulkPut(events);

// Compound indexes for complex queries
db.version(1).stores({
  events: "id, status, timestamp, [status+timestamp]",
});
```

### **2. Local Cache Requirements**

**What we need:**

- Store JSON objects (chart configs, well metadata)
- Query by key pattern (invalidate by prefix)
- TTL support (expire old entries)
- Fast reads (cache hits)

**IndexedDB + Dexie.js provides:**

```typescript
// Fast key-based lookups
const cached = await db.cache.get(key);

// Pattern-based invalidation
await db.cache.where("key").startsWith("chart:").delete();

// TTL cleanup
await db.cache.where("expiresAt").below(Date.now()).delete();
```

### **3. Multi-Tab Support**

**Challenge**: Multiple browser tabs need to share state.

**Solution**: BroadcastChannel API (works with IndexedDB):

```typescript
// In each tab
const channel = new BroadcastChannel("event-queue-sync");

// When event is added
await db.events.add(event);
channel.postMessage({ type: "event-added", id: event.id });

// Listen for changes from other tabs
channel.onmessage = (e) => {
  if (e.data.type === "event-added") {
    // Refresh UI or sync state
  }
};
```

### **4. Web Worker Support**

**Why Web Workers?**

- Move heavy sync operations off main thread
- Keep UI responsive during bulk operations
- Can use faster OPFS API inside worker (future optimization)

**IndexedDB works in Web Workers:**

```typescript
// Main thread
const worker = new Worker("/workers/sync-worker.js");
worker.postMessage({ type: "processQueue" });

// Worker thread (sync-worker.js)
import Dexie from "dexie";
const db = new EventQueueDB(); // Same Dexie instance

self.onmessage = async (e) => {
  const events = await db.events.where("status").equals("pending").toArray();
  // Process events...
};
```

## 🔍 **Detailed Technology Analysis**

### **IndexedDB + Dexie.js**

**Pros:**

- ✅ Native JSON support (no stringify overhead)
- ✅ Non-blocking async operations
- ✅ Built-in indexing (fast queries)
- ✅ Large storage (GBs, 80% of disk)
- ✅ WebWorker support
- ✅ Clean API with Dexie.js
- ✅ Small bundle size (~15KB)

**Cons:**

- ⚠️ Requires BroadcastChannel for multi-tab sync
- ⚠️ Raw IndexedDB API is complex (but Dexie.js fixes this)

**Best for:**

- Event queues
- Local caches
- Structured JSON data
- Offline-first applications

### **LocalStorage**

**Pros:**

- ✅ Very fast reads/writes (0.005-0.017ms)
- ✅ Simple API
- ✅ Built-in multi-tab sync (storage event)

**Cons:**

- ❌ **Blocks main thread** (bad for bulk operations)
- ❌ **5MB limit** (too small for event queue)
- ❌ **No indexing** (can't query efficiently)
- ❌ **No WebWorker support**
- ❌ String-only (requires JSON.stringify)

**Verdict**: ❌ Not suitable for our use case (blocking + size limit)

### **OPFS (Origin Private File System)**

**Pros:**

- ✅ Very fast in WebWorker (1ms writes with sync handle)
- ✅ Large storage (GBs)
- ✅ WebWorker support

**Cons:**

- ❌ **Too complex** (meant for binary data, not JSON)
- ❌ **Slower for JSON** (1.5ms vs 0.17ms writes)
- ❌ **Manual file management** (no database features)
- ❌ **No indexing** (would need to build manually)

**Verdict**: ❌ Overkill for our use case (better for images/files)

### **WASM SQLite**

**Pros:**

- ✅ SQL queries (familiar API)
- ✅ Fast bulk operations
- ✅ Large storage
- ✅ WebWorker support

**Cons:**

- ❌ **500ms+ initialization** (bad for app startup)
- ❌ **939KB bundle size** (significant download)
- ❌ **Overkill** (we don't need SQL queries)
- ❌ **Complex setup** (VFS adapters, WASM loading)

**Verdict**: ❌ Too heavy for our use case (IndexedDB + Dexie provides similar query capabilities)

## 🏗️ **Recommended Architecture**

### **Phase 1: IndexedDB + Dexie.js (Immediate)**

```typescript
// src/lib/storage/event-queue-db.ts
import Dexie, { Table } from "dexie";

interface QueuedEvent {
  id: string;
  type: "add" | "remove" | "update";
  entity: "chart_data_source" | "well" | "polygon" | "curve";
  entityId: string;
  payload: Record<string, any>;
  timestamp: number;
  status: "pending" | "processing" | "completed" | "failed";
  retryCount: number;
}

class EventQueueDB extends Dexie {
  events!: Table<QueuedEvent>;

  constructor() {
    super("EventQueueDB");
    this.version(1).stores({
      // Compound index: [status+timestamp] for efficient querying
      events:
        "id, status, timestamp, entity, [status+timestamp], [entity+status]",
    });
  }
}

export const eventQueueDB = new EventQueueDB();
```

### **Phase 2: Web Worker for Background Sync (Future)**

```typescript
// src/lib/workers/sync-worker.ts
import Dexie from "dexie";
import { eventQueueDB } from "../storage/event-queue-db";

self.onmessage = async (e) => {
  if (e.data.type === "processQueue") {
    // Get pending events (oldest first)
    const events = await eventQueueDB.events
      .where("status")
      .equals("pending")
      .and((e) => e.retryCount < 5)
      .sortBy("timestamp");

    // Process in batches
    for (const event of events) {
      try {
        await processEvent(event);
        await eventQueueDB.events.update(event.id, {
          status: "completed",
        });
      } catch (error) {
        await eventQueueDB.events.update(event.id, {
          status: "failed",
          retryCount: event.retryCount + 1,
        });
      }
    }

    self.postMessage({ type: "complete", processed: events.length });
  }
};
```

### **Phase 3: OPFS Optimization (Future, if needed)**

If we need even faster writes for binary data (e.g., parquet file caching), we can use OPFS inside the Web Worker:

```typescript
// Inside Web Worker only
const root = await navigator.storage.getDirectory();
const fileHandle = await root.getFileHandle("cache.parquet", { create: true });
const accessHandle = await fileHandle.createSyncAccessHandle();

// Fast binary writes (1ms vs 1.5ms for JSON)
accessHandle.write(new Uint8Array(data), { at: 0 });
```

**Note**: Only use OPFS for binary data (parquet files, images). Keep JSON data in IndexedDB.

## 📦 **Implementation Plan**

### **Step 1: Install Dexie.js**

```bash
bun add dexie
```

### **Step 2: Create Database Schema**

```typescript
// src/lib/storage/event-queue-db.ts
import Dexie, { Table } from "dexie";

export interface QueuedEvent {
  id: string;
  type: "add" | "remove" | "update";
  entity: string;
  entityId: string;
  payload: Record<string, any>;
  timestamp: number;
  status: "pending" | "processing" | "completed" | "failed";
  retryCount: number;
}

export interface CacheEntry<T = any> {
  key: string;
  data: T;
  timestamp: number;
  expiresAt: number;
  version: number;
}

class MudRockStorageDB extends Dexie {
  events!: Table<QueuedEvent>;
  cache!: Table<CacheEntry>;

  constructor() {
    super("MudRockStorageDB");
    this.version(1).stores({
      events:
        "id, status, timestamp, entity, [status+timestamp], [entity+status]",
      cache: "key, expiresAt, [key+expiresAt]",
    });
  }
}

export const storageDB = new MudRockStorageDB();
```

### **Step 3: Implement Event Queue**

```typescript
// src/lib/storage/event-queue.ts
import { storageDB, type QueuedEvent } from "./event-queue-db";

export class EventQueue {
  async enqueue(
    event: Omit<QueuedEvent, "id" | "timestamp" | "status" | "retryCount">,
  ): Promise<void> {
    const queuedEvent: QueuedEvent = {
      ...event,
      id: crypto.randomUUID(),
      timestamp: Date.now(),
      status: "pending",
      retryCount: 0,
    };

    await storageDB.events.add(queuedEvent);
    this.syncQueue(); // Trigger background sync
  }

  async getPendingEvents(): Promise<QueuedEvent[]> {
    return await storageDB.events
      .where("status")
      .equals("pending")
      .sortBy("timestamp");
  }

  async markCompleted(eventId: string): Promise<void> {
    await storageDB.events.update(eventId, { status: "completed" });
  }

  async markFailed(eventId: string, retryCount: number): Promise<void> {
    await storageDB.events.update(eventId, {
      status: "failed",
      retryCount,
    });
  }

  private syncQueue(): void {
    // Trigger background sync (can use Web Worker)
    // Implementation in Phase 2
  }
}
```

### **Step 4: Implement Local Cache**

```typescript
// src/lib/storage/local-cache.ts
import { storageDB, type CacheEntry } from "./event-queue-db";

export class LocalCache {
  async get<T>(
    key: string,
    fetcher: () => Promise<T>,
    ttl: number = 3600000,
  ): Promise<T> {
    const cached = await storageDB.cache.get(key);

    if (cached && cached.expiresAt > Date.now()) {
      return cached.data as T;
    }

    // Cache miss - fetch fresh data
    const fresh = await fetcher();

    // Update cache in background
    await storageDB.cache.put({
      key,
      data: fresh,
      timestamp: Date.now(),
      expiresAt: Date.now() + ttl,
      version: 1,
    });

    return fresh;
  }

  async invalidate(pattern: string): Promise<void> {
    await storageDB.cache.where("key").startsWith(pattern).delete();
  }

  async clearExpired(): Promise<void> {
    await storageDB.cache.where("expiresAt").below(Date.now()).delete();
  }
}
```

## 🎯 **Final Recommendation**

**✅ Use IndexedDB + Dexie.js for:**

- Event queue (offline-first operations)
- Local cache (frequently accessed data)
- Optimistic update storage

**✅ Use Web Workers for:**

- Background event queue processing
- Bulk database sync operations
- Heavy conflict resolution

**❌ Don't use:**

- LocalStorage (blocking, size limit)
- OPFS (too complex, meant for binary)
- WASM SQLite (too heavy, slow init)

**Future optimization:**

- Consider OPFS inside Web Worker for binary data (parquet file caching)
- Keep JSON data in IndexedDB (faster for structured data)

## 📚 **Resources**

- [Dexie.js Documentation](https://dexie.org/)
- [IndexedDB API](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API)
- [Storage Performance Comparison](https://rxdb.info/articles/storage-performance.html)
- [Web Workers API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API)
- [BroadcastChannel API](https://developer.mozilla.org/en-US/docs/Web/API/BroadcastChannel)
