# MudRock State Synchronization Architecture

## 🎯 **Executive Summary**

This document outlines an advanced, modern, and durable solution for state synchronization between the frontend UI (AG Grid tables, chart selections) and the PostgreSQL database. The architecture addresses:

1. **Real-time synchronization** between UI components and database
2. **Offline-first capabilities** for field work without network connectivity
3. **Conflict resolution** for concurrent edits
4. **Performance optimization** through intelligent caching and batching
5. **Durability** through optimistic updates with rollback mechanisms

## 🔍 **Current Problem Analysis**

### **Issues Identified**

1. **AG Grid Selection Sync Conflicts**
   - AG Grid's automatic click selection conflicts with database-driven selection
   - Deselected items reappear when dialog is reopened
   - Selection state becomes out of sync with database state

2. **Direct Database Dependency**
   - Every user action triggers immediate database write
   - UI waits for database confirmation before proceeding
   - No offline capability - operations fail without network

3. **Reactive Loop Issues**
   - Auto-save effects create infinite loops when syncing from database
   - Race conditions between user actions and realtime updates
   - Effect tracking complexity leads to `effect_update_depth_exceeded` errors

4. **Performance Concerns**
   - Excessive database calls for every selection change
   - No batching or debouncing strategy
   - No local caching layer for frequently accessed data

## 🏗️ **Proposed Architecture**

### **Hybrid Frontend + Rust Backend Architecture**

Given MudRock's Tauri architecture (Rust backend + Svelte frontend), we use a **hybrid approach** that leverages the strengths of both:

- **Frontend (Dexie.js + IndexedDB)**: Immediate UI updates, optimistic updates, offline event queue
- **Rust Backend (Tauri Commands)**: Reliable database sync, batch processing, conflict resolution

### **Three-Layer Architecture**

```
┌─────────────────────────────────────────────────────────────┐
│                    UI Layer (Svelte)                        │
│  • AG Grid tables, chart selectors, form inputs             │
│  • Optimistic updates (immediate UI feedback)              │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│         State Management Layer (Frontend)                   │
│  • Event queue (Dexie.js + IndexedDB)                      │
│  • Local cache (Dexie.js + IndexedDB)                      │
│  • Optimistic update manager                                │
│  • Tauri IPC communication                                  │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼ (Tauri Commands)
┌─────────────────────────────────────────────────────────────┐
│      Sync Service Layer (Rust Backend)                     │
│  • Batch event processor                                    │
│  • Conflict resolution                                      │
│  • Database transaction manager                             │
│  • UnifiedDatabaseService integration                       │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Database Layer (PostgreSQL)                    │
│  • Supabase client                                          │
│  • Realtime subscriptions                                   │
│  • Transaction support                                      │
└─────────────────────────────────────────────────────────────┘
```

### **Why Hybrid?**

**Frontend (Dexie.js) Benefits:**

- ✅ **Instant UI feedback**: Optimistic updates happen immediately
- ✅ **Offline-first**: Events queued locally, synced when online
- ✅ **Reactive**: Svelte 5 runes provide automatic reactivity
- ✅ **Lightweight**: Dexie.js is only ~15KB

**Rust Backend Benefits:**

- ✅ **Reliable**: Leverages existing `UnifiedDatabaseService` infrastructure
- ✅ **Performance**: Batch processing, efficient database operations
- ✅ **Type-safe**: Rust's type system ensures correctness
- ✅ **Existing infrastructure**: Already has caching (`moka::future::Cache`), database access

**Communication:**

- **Frontend → Backend**: Tauri `invoke()` commands for batch processing
- **Backend → Frontend**: Tauri `emit()` events for sync status updates
- **Database → Frontend**: Supabase Realtime subscriptions for live updates

### **Key Components**

#### **1. Event Queue (Dexie.js + IndexedDB)**

**Purpose**: Decouple UI operations from database writes, enabling offline work and batching.

**Frontend Implementation** (Dexie.js):

```typescript
// src/lib/storage/event-queue.ts
import Dexie, { Table } from "dexie";
import { invoke } from "@tauri-apps/api/core";

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
      events: "id, status, timestamp, [status+timestamp]", // Compound index
    });
  }
}

export class EventQueue {
  private db = new EventQueueDB();

  // Add event to queue (immediate return, no database wait)
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

    // Store in IndexedDB (persistent, survives app restart)
    await this.db.events.add(queuedEvent);

    // Trigger background sync via Rust backend (non-blocking)
    this.syncQueue();
  }

  // Process queue (batched, retry on failure)
  private async syncQueue(): Promise<void> {
    const pendingEvents = await this.db.events
      .where("status")
      .equals("pending")
      .sortBy("timestamp");

    if (pendingEvents.length === 0) return;

    // Batch events by entity type for efficiency
    const batches = this.batchEvents(pendingEvents);

    for (const batch of batches) {
      try {
        // Send batch to Rust backend via Tauri command
        await invoke("process_event_batch", { events: batch });

        // Mark batch as completed
        await this.db.events
          .where("id")
          .anyOf(batch.map((e) => e.id))
          .modify({ status: "completed" });
      } catch (error) {
        // Mark batch as failed, increment retry count
        await this.db.events
          .where("id")
          .anyOf(batch.map((e) => e.id))
          .modify((event) => {
            event.status = "failed";
            event.retryCount += 1;
          });

        // Exponential backoff retry
        setTimeout(
          () => this.syncQueue(),
          this.getRetryDelay(batch[0].retryCount),
        );
      }
    }
  }

  private batchEvents(events: QueuedEvent[]): QueuedEvent[][] {
    // Group events by entity type for efficient batch processing
    const batches = new Map<string, QueuedEvent[]>();
    for (const event of events) {
      const key = `${event.entity}:${event.type}`;
      if (!batches.has(key)) {
        batches.set(key, []);
      }
      batches.get(key)!.push(event);
    }
    return Array.from(batches.values());
  }

  private getRetryDelay(retryCount: number): number {
    // Exponential backoff: 1s, 2s, 4s, 8s, max 30s
    return Math.min(1000 * Math.pow(2, retryCount), 30000);
  }
}
```

**Rust Backend Implementation** (Tauri Command):

```rust
// src-tauri/src/tauri_commands/event_queue_commands.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueuedEvent {
    pub id: String,
    pub r#type: String, // "add" | "remove" | "update"
    pub entity: String, // "chart_data_source" | "well" | "polygon" | "curve"
    pub entity_id: String,
    pub payload: serde_json::Value,
    pub timestamp: i64,
    pub status: String,
    pub retry_count: i32,
}

#[tauri::command]
pub async fn process_event_batch(
    events: Vec<QueuedEvent>,
) -> Result<(), String> {
    // Get unified database service
    let service = UnifiedDatabaseService::get_instance()
        .map_err(|e| format!("Failed to get service: {}", e))?;

    // Process batch in a transaction
    for event in events {
        match event.entity.as_str() {
            "chart_data_source" => {
                // Update chart's data_source_config
                service.update_chart_data_source(
                    &event.entity_id,
                    &event.r#type,
                    &event.payload,
                ).await?;
            }
            "well" | "polygon" | "curve" => {
                // Handle other entity types
                // ...
            }
            _ => return Err(format!("Unknown entity type: {}", event.entity)),
        }
    }

    Ok(())
}
```

**Benefits**:

- ✅ **Offline-first**: Events queued locally, synced when online
- ✅ **Batching**: Multiple events combined into single database transaction
- ✅ **Retry logic**: Automatic retry with exponential backoff
- ✅ **Durability**: Events persist across app restarts

#### **2. Local Cache (Dexie.js + IndexedDB)**

**Purpose**: Cache frequently accessed data locally, reducing database queries and enabling offline reads.

**Frontend Implementation** (Dexie.js):

```typescript
// src/lib/storage/local-cache.ts
import Dexie, { Table } from "dexie";

interface CacheEntry<T> {
  key: string;
  data: T;
  timestamp: number;
  ttl: number; // Time-to-live in milliseconds
  version: number; // For cache invalidation
}

class CacheDB extends Dexie {
  cache!: Table<CacheEntry<any>>;

  constructor() {
    super("CacheDB");
    this.version(1).stores({
      cache: "key, timestamp, [timestamp+ttl]", // Index for expiration queries
    });
  }
}

export class LocalCache {
  private db = new CacheDB();

  // Get from cache, fallback to database if cache miss
  async get<T>(key: string, fetcher: () => Promise<T>): Promise<T> {
    const cached = await this.db.cache.get(key);

    if (cached && !this.isExpired(cached)) {
      // Return cached data immediately
      return cached.data as T;
    }

    // Cache miss or expired - fetch from database
    const fresh = await fetcher();

    // Update cache in background (non-blocking)
    await this.set(key, fresh);

    return fresh;
  }

  async set<T>(key: string, data: T, ttl: number = 300000): Promise<void> {
    // Default TTL: 5 minutes
    await this.db.cache.put({
      key,
      data,
      timestamp: Date.now(),
      ttl,
      version: 1,
    });
  }

  private isExpired(entry: CacheEntry<any>): boolean {
    return Date.now() - entry.timestamp > entry.ttl;
  }

  // Invalidate cache on database updates (via realtime)
  async invalidate(pattern: string): Promise<void> {
    // Delete entries matching pattern (e.g., "chart:*" or "well:*")
    const regex = new RegExp(pattern.replace("*", ".*"));
    await this.db.cache
      .where("key")
      .filter((key) => regex.test(key))
      .delete();
  }
}
```

**Note**: The Rust backend already has caching via `moka::future::Cache` in `UnifiedDatabaseService`. The frontend cache complements this by providing:

- Offline access to cached data
- Faster UI updates (no network round-trip)
- Reduced load on Rust backend

**Benefits**:

- ✅ **Fast reads**: Instant access to cached data
- ✅ **Offline reads**: Access cached data without network
- ✅ **Reduced load**: Fewer database queries
- ✅ **Smart invalidation**: Cache cleared on realtime updates

#### **3. Optimistic Update Manager**

**Purpose**: Update UI immediately, rollback on failure.

```typescript
// src/lib/storage/optimistic-update.ts
interface OptimisticUpdate<T> {
  id: string;
  originalState: T;
  optimisticState: T;
  rollback: () => void;
  commit: () => void;
}

export class OptimisticUpdateManager {
  private updates = new Map<string, OptimisticUpdate<any>>();

  // Apply optimistic update (immediate UI feedback)
  applyOptimistic<T>(id: string, updateFn: (current: T) => T): T {
    const current = this.getCurrentState<T>(id);
    const optimistic = updateFn(current);

    // Store original for rollback
    this.updates.set(id, {
      id,
      originalState: current,
      optimisticState: optimistic,
      rollback: () => this.setState(id, current),
      commit: () => this.setState(id, optimistic),
    });

    // Update UI immediately
    this.setState(id, optimistic);

    return optimistic;
  }

  // Commit update (database confirmed)
  commit(id: string): void {
    const update = this.updates.get(id);
    if (update) {
      update.commit();
      this.updates.delete(id);
    }
  }

  // Rollback update (database failed)
  rollback(id: string): void {
    const update = this.updates.get(id);
    if (update) {
      update.rollback();
      this.updates.delete(id);
    }
  }
}
```

**Benefits**:

- ✅ **Instant feedback**: UI updates immediately
- ✅ **Rollback on failure**: Automatic revert if database write fails
- ✅ **Conflict resolution**: Handle concurrent edits gracefully

#### **4. State Synchronization Service**

**Purpose**: Centralized service managing the entire sync flow.

```typescript
// src/lib/services/state-sync-service.ts
export class StateSyncService {
  private eventQueue: EventQueue;
  private localCache: LocalCache;
  private optimisticManager: OptimisticUpdateManager;
  private realtimeSubscriptions: Map<string, RealtimeChannel>;

  constructor() {
    this.eventQueue = new EventQueue();
    this.localCache = new LocalCache();
    this.optimisticManager = new OptimisticUpdateManager();
  }

  // User action: Add data source to chart
  async addDataSourceToChart(
    chartId: string,
    source: ChartDataSourceEntry,
  ): Promise<void> {
    const updateId = crypto.randomUUID();

    // 1. Optimistic update (immediate UI feedback)
    this.optimisticManager.applyOptimistic(`chart:${chartId}`, (current) => {
      const config = current.data_source_config || [];
      return {
        ...current,
        data_source_config: [...config, source],
      };
    });

    // 2. Queue event (non-blocking, works offline)
    await this.eventQueue.enqueue({
      type: "add",
      entity: "chart_data_source",
      entityId: chartId,
      payload: source,
    });

    // 3. Background sync (batched, retried on failure)
    this.eventQueue.syncQueue();
  }

  // Database update received (via realtime)
  async handleRealtimeUpdate(
    entity: string,
    id: string,
    data: any,
  ): Promise<void> {
    // 1. Invalidate cache
    await this.localCache.invalidate(`${entity}:${id}`);

    // 2. Commit optimistic update if it matches
    const updateId = `${entity}:${id}`;
    this.optimisticManager.commit(updateId);

    // 3. Update UI reactively (Svelte will handle this)
    this.notifySubscribers(entity, id, data);
  }

  // Conflict resolution (concurrent edits)
  async resolveConflict(
    local: any,
    remote: any,
    strategy: "local-wins" | "remote-wins" | "merge" | "manual",
  ): Promise<any> {
    switch (strategy) {
      case "local-wins":
        return local;
      case "remote-wins":
        return remote;
      case "merge":
        return this.mergeStates(local, remote);
      case "manual":
        // Show conflict resolution UI
        return this.showConflictResolutionUI(local, remote);
    }
  }
}
```

## 🔄 **Sync Flow Example: AG Grid Selection**

### **Before (Current Implementation)**

```typescript
// User clicks row → immediate database write → wait for confirmation → update UI
onRowClicked: async (params) => {
  // 1. Update database (blocking)
  await addDataSourceToChart(chartId, source);

  // 2. Wait for realtime update
  // 3. Update selectedRowIds
  // 4. Sync AG Grid
};
```

**Problems**:

- ❌ UI waits for database
- ❌ No offline support
- ❌ Race conditions with realtime
- ❌ Excessive database calls

### **After (Proposed Implementation)**

```typescript
// User clicks row → optimistic update → queue event → background sync
onRowClicked: (params) => {
  const rowId = getRowId(params.data);

  // 1. Optimistic update (immediate UI feedback)
  const newSelection = stateSyncService.applyOptimistic(
    `chart:${chartId}:selection`,
    (current) => {
      if (current.includes(rowId)) {
        return current.filter((id) => id !== rowId); // Deselect
      } else {
        return [...current, rowId]; // Select
      }
    },
  );

  // 2. Update AG Grid immediately
  updateGridSelection(newSelection);

  // 3. Queue event (non-blocking, works offline)
  stateSyncService.addDataSourceToChart(chartId, {
    id: `curve-${rowId}`,
    source: { type: "curve", curveId: rowId },
    displaySettings: { visible: true },
  });

  // 4. Background sync handles database write
  // 5. Realtime update commits optimistic update
};
```

**Benefits**:

- ✅ Instant UI feedback
- ✅ Works offline (events queued)
- ✅ Batched database writes
- ✅ Automatic retry on failure
- ✅ Conflict resolution

## 📦 **Implementation Plan**

### **Phase 1: Event Queue (Week 1-2)**

1. **Create IndexedDB schema** for event queue
2. **Implement `EventQueue` class** with enqueue/sync methods
3. **Add batching logic** (group events by entity type)
4. **Add retry logic** (exponential backoff)
5. **Integrate with chart data source service**

### **Phase 2: Local Cache (Week 2-3)**

1. **Create IndexedDB schema** for cache
2. **Implement `LocalCache` class** with get/set/invalidate
3. **Add TTL support** (time-to-live for cache entries)
4. **Add cache invalidation** on realtime updates
5. **Integrate with chart/well/polygon state**

### **Phase 3: Optimistic Updates (Week 3-4)**

1. **Implement `OptimisticUpdateManager`** class
2. **Add rollback mechanism** for failed updates
3. **Add conflict detection** (compare local vs remote)
4. **Integrate with AG Grid components**
5. **Add visual feedback** for pending/committed/rolled-back updates

### **Phase 4: State Sync Service (Week 4-5)**

1. **Create `StateSyncService`** integrating all components
2. **Add realtime subscription management**
3. **Add conflict resolution strategies**
4. **Add monitoring/logging** for sync status
5. **Update all chart data source selectors** to use new service

### **Phase 5: Testing & Optimization (Week 5-6)**

1. **Test offline scenarios** (queue events, sync when online)
2. **Test conflict resolution** (concurrent edits)
3. **Test performance** (batching, caching effectiveness)
4. **Add error handling** and user notifications
5. **Documentation** and migration guide

## 🔧 **Third-Party Libraries**

### **Recommended Stack**

1. **IndexedDB Wrapper**: **Dexie.js** (Recommended over raw IndexedDB)
   - **Why Dexie.js over raw IndexedDB?**
     - Clean, promise-based API (vs. callback-based IndexedDB)
     - TypeScript support with excellent type inference
     - Built-in querying (vs. manual cursor iteration)
     - Schema versioning and migrations
     - Bulk operations (faster than individual writes)
     - Small bundle size (~15KB gzipped)
   - **Why not `idb`?**
     - `idb` is lower-level, still requires manual cursor iteration
     - Dexie provides higher-level query API (closer to SQL)
     - Better developer experience for complex queries
   - **Installation**: `bun add dexie`
   - **Example**:

     ```typescript
     import Dexie, { Table } from "dexie";

     interface QueuedEvent {
       id: string;
       type: "add" | "remove" | "update";
       status: "pending" | "processing" | "completed" | "failed";
       timestamp: number;
     }

     class EventQueueDB extends Dexie {
       events!: Table<QueuedEvent>;

       constructor() {
         super("EventQueueDB");
         this.version(1).stores({
           events: "id, status, timestamp, [status+timestamp]", // Compound index
         });
       }
     }

     const db = new EventQueueDB();

     // Clean query API (vs. raw IndexedDB cursors)
     const pendingEvents = await db.events
       .where("status")
       .equals("pending")
       .sortBy("timestamp");
     ```

2. **Web Workers**: For background sync operations
   - **Why Web Workers?**
     - Move heavy sync operations off main thread
     - Keep UI responsive during bulk operations
     - Can use faster OPFS API inside worker (if needed later)
   - **Architecture**:
     - Main thread: UI updates, user interactions
     - Web Worker: Event queue processing, database sync, conflict resolution
     - Communication: `postMessage()` for commands, `BroadcastChannel` for multi-tab sync
   - **Implementation**: Use Dexie inside Web Worker (IndexedDB works in workers)
   - **Note**: OPFS `createSyncAccessHandle()` is faster but only works in workers - consider for future optimization

3. **Conflict Resolution**: Custom implementation
   - Simple merge strategies for most cases
   - Manual resolution UI for complex conflicts

4. **Batching**: Custom implementation
   - Entity-type-based batching
   - Configurable batch size and delay

5. **Monitoring**: Custom + Supabase Realtime
   - Track queue size, sync status
   - Real-time sync status in UI

## 🎯 **Migration Strategy**

### **Step 1: Parallel Implementation**

- Keep existing direct database writes
- Add event queue alongside
- Gradually migrate components

### **Step 2: Feature Flag**

```typescript
const USE_EVENT_QUEUE = true; // Feature flag

if (USE_EVENT_QUEUE) {
  await stateSyncService.addDataSourceToChart(chartId, source);
} else {
  await addDataSourcesToChart(chartId, [source]); // Old way
}
```

### **Step 3: Gradual Rollout**

1. **Week 1**: Chart data source selectors
2. **Week 2**: Well/polygon selectors
3. **Week 3**: All AG Grid tables
4. **Week 4**: Form inputs and other UI components

### **Step 4: Remove Old Code**

- Once all components migrated
- Remove direct database write paths
- Clean up unused code

## 📊 **Expected Benefits**

### **Performance**

- **50-70% reduction** in database calls (batching)
- **Instant UI feedback** (optimistic updates)
- **Faster reads** (local cache)

### **Reliability**

- **Offline support** (event queue persists)
- **Automatic retry** (exponential backoff)
- **Conflict resolution** (concurrent edits handled)

### **User Experience**

- **Instant feedback** (no waiting for database)
- **Works offline** (field work without network)
- **Visual sync status** (pending/committed indicators)

## 🔐 **Security Considerations**

1. **Event Queue Encryption**: Encrypt sensitive data in IndexedDB
2. **Cache Validation**: Verify cache entries haven't been tampered with
3. **Rate Limiting**: Prevent queue flooding
4. **Audit Logging**: Track all sync operations for compliance

## 🔍 **Storage Technology Analysis**

### **Why IndexedDB (with Dexie.js) is the Best Choice**

Based on performance benchmarks and feature analysis:

| Requirement           | IndexedDB + Dexie   | LocalStorage          | OPFS                | WASM SQLite         |
| --------------------- | ------------------- | --------------------- | ------------------- | ------------------- |
| **JSON Support**      | ✅ Native           | ❌ String only        | ❌ Binary only      | ✅ JSON columns     |
| **Async Operations**  | ✅ Non-blocking     | ❌ Blocks main thread | ✅ Async            | ✅ Async            |
| **Indexing**          | ✅ Built-in         | ❌ None               | ❌ Manual           | ✅ SQL indexes      |
| **Storage Size**      | ✅ GBs (80% disk)   | ❌ 5-10MB             | ✅ GBs              | ✅ GBs              |
| **WebWorker Support** | ✅ Yes              | ❌ No                 | ✅ Yes (faster)     | ✅ Yes              |
| **Multi-Tab Sync**    | ⚠️ BroadcastChannel | ✅ Storage event      | ⚠️ BroadcastChannel | ⚠️ BroadcastChannel |
| **Initialization**    | ✅ 46ms             | ✅ Instant            | ✅ 23-27ms          | ❌ 500ms+           |
| **Write Latency**     | ✅ 0.17ms           | ✅ 0.017ms            | ⚠️ 1.5ms            | ⚠️ 0.17-3ms         |
| **Read Latency**      | ✅ 0.1ms            | ✅ 0.005ms            | ⚠️ 1.3ms            | ⚠️ 0.45-3ms         |
| **Bulk Writes (200)** | ✅ 13ms             | ⚠️ 5.8ms (blocks)     | ⚠️ 104-280ms        | ✅ 19-37ms          |
| **Bulk Reads (100)**  | ✅ 5ms              | ⚠️ 0.4ms (blocks)     | ⚠️ 26-55ms          | ✅ 3.6-6ms          |
| **API Complexity**    | ⚠️ Complex (raw)    | ✅ Simple             | ❌ Very complex     | ⚠️ SQL (complex)    |
| **Bundle Size**       | ✅ ~15KB (Dexie)    | ✅ 0KB                | ✅ 0KB              | ❌ 939KB            |

### **Decision Matrix**

**✅ IndexedDB + Dexie.js is the clear winner because:**

1. **JSON Support**: Native JSON storage (no stringify overhead)
2. **Non-Blocking**: Async operations don't block UI
3. **Indexing**: Built-in indexes for fast queries (status, timestamp, entity type)
4. **Storage Size**: Can store GBs (perfect for large event queues)
5. **WebWorker Support**: Can move sync operations to background thread
6. **API Quality**: Dexie.js provides clean, SQL-like query API
7. **Performance**: Good balance of speed and features
8. **Bundle Size**: Small (~15KB) compared to WASM SQLite (939KB)

**❌ Why not LocalStorage?**

- Blocks main thread (bad for bulk operations)
- 5MB limit (too small for event queue)
- No indexing (can't query by status efficiently)
- No WebWorker support

**❌ Why not OPFS?**

- Too complex (meant for binary data, not JSON)
- Slower for JSON operations (1.5ms vs 0.17ms writes)
- Requires manual file management
- Better suited for images/files, not structured data

**❌ Why not WASM SQLite?**

- 500ms+ initialization time (bad for app startup)
- 939KB bundle size (significant download)
- Overkill for our use case (we don't need SQL queries)
- IndexedDB + Dexie provides similar query capabilities with better performance

### **Web Workers: Recommended for Background Sync**

**Use Web Workers for:**

- ✅ Background event queue processing
- ✅ Bulk database sync operations
- ✅ Conflict resolution (heavy computation)
- ✅ Cache invalidation (scanning large cache)

**Architecture:**

```
Main Thread (UI)                    Web Worker (Background)
├── User interactions               ├── Event queue processor
├── Optimistic updates             ├── Database sync operations
├── UI updates                     ├── Conflict resolution
└── postMessage() commands ───────>└── postMessage() results
```

**Benefits:**

- UI stays responsive during heavy sync operations
- Can use faster OPFS API inside worker (future optimization)
- Parallel processing (main thread + worker)

**Implementation:**

```typescript
// Main thread
const worker = new Worker("/workers/sync-worker.js");
worker.postMessage({ type: "processQueue" });

// Worker thread (sync-worker.js)
import Dexie from "dexie";
const db = new EventQueueDB(); // Same Dexie instance in worker

self.onmessage = async (e) => {
  if (e.data.type === "processQueue") {
    const events = await db.events.where("status").equals("pending").toArray();
    // Process events...
    self.postMessage({ type: "complete", count: events.length });
  }
};
```

## 📚 **References**

- [IndexedDB API](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API)
- [Dexie.js Documentation](https://dexie.org/)
- [Storage API Performance Comparison](https://rxdb.info/articles/storage-performance.html)
- [Optimistic UI Updates](https://www.apollographql.com/docs/react/performance/optimistic-ui/)
- [Offline-First Architecture](https://offlinefirst.org/)
- [PowerSync Documentation](https://docs.powersync.com/) (for inspiration)
- [WLIP Architecture](./WLIP_MudRock_Compatibility_and_Security.md)
- [Performance Plan](../performance/PERFORMANCE_PLAN.md)
- [Database Architecture](./DATABASE_ARCHITECTURE.md)
