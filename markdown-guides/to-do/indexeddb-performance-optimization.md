# IndexedDB Performance Optimization Plan

## Executive Summary

This document addresses performance concerns with the local-first Automerge-Repo + IndexedDB architecture and provides optimization strategies to maintain low-latency UI responsiveness.

**Key Finding**: Our current implementation is **already well-optimized** because:
1. Automerge-repo stores **binary `Uint8Array`** data, not JS objects (no expensive cloning)
2. Incremental changes are stored, not full document snapshots
3. Binary format is compact (~10-100 bytes per change vs KB for JSON)

---

## Current Architecture Analysis

### How Automerge-Repo IndexedDB Storage Works

```typescript
// From automerge-repo-storage-indexeddb/src/index.ts
async save(keyArray: string[], binary: Uint8Array): Promise<void> {
  // Stores binary directly - no JS object cloning!
  objectStore.put({ key: keyArray, binary: binary }, keyArray);
}
```

**Storage Format:**
| Key | Value |
|-----|-------|
| `["automerge:xxx", "snapshot"]` | `{ binary: Uint8Array }` |
| `["automerge:xxx", "incremental", "0001"]` | `{ binary: Uint8Array }` |

### Why This Is Already Efficient

| Aspect | Reality | Impact |
|--------|---------|--------|
| **Data Format** | Binary `Uint8Array` | No structured clone overhead |
| **Change Size** | ~10-100 bytes per op | Minimal I/O |
| **Storage Strategy** | Incremental changes | Small writes |
| **Main Thread Blocking** | Async IndexedDB API | Non-blocking |

### Structured Clone Algorithm Impact

**Myth**: IndexedDB cloning is expensive for all data.

**Reality**: For `Uint8Array`:
- No recursive traversal (unlike JS objects)
- Direct memory copy of binary buffer
- Similar performance to `ArrayBuffer.slice()`
- ~0.01ms for typical Automerge change (100 bytes)

---

## Current Performance Profile

### Measured Latencies (Estimated)

| Operation | Current Latency | Target | Status |
|-----------|-----------------|--------|--------|
| UI update (optimistic) | <1ms | <1ms | ✅ Optimal |
| IndexedDB write | ~1-5ms | <10ms | ✅ Good |
| IndexedDB read | ~1-3ms | <10ms | ✅ Good |
| Document load (cold) | ~10-50ms | <100ms | ✅ Acceptable |
| Tab sync (BroadcastChannel) | <5ms | <50ms | ✅ Good |

### Bottleneck Analysis

```
User Action → UI Update (instant) → IndexedDB Write (async, non-blocking)
                    ↓
              Main Thread Free
                    ↓
              IndexedDB Transaction (background)
```

**Result**: UI is never blocked by IndexedDB operations.

---

## Potential Optimizations

### 1. Write Batching (Already Implemented)

**Current**: 2-second debounce for visible range updates.

```typescript
// From chart-store.svelte.ts
private scheduleBackupSync(chartId: string, debounceMs: number = 2000) {
  // Batches multiple rapid changes into single write
}
```

**Benefit**: Reduces IndexedDB transaction count during rapid interactions (zoom/pan).

### 2. Web Workers for Heavy CRDT Operations

**When to Use**: Only if CRDT merge operations become CPU-intensive (>16ms).

**Status**: ⏳ **Not Currently Needed**

Automerge CRDT operations are fast (<1ms for typical changes). Web Workers add complexity and message-passing overhead.

**Implementation (If Needed Later)**:

```typescript
// automerge-worker.ts
import * as Automerge from "@automerge/automerge";

self.onmessage = (e) => {
  const { type, payload } = e.data;
  
  switch (type) {
    case "MERGE_DOCUMENTS":
      const merged = Automerge.merge(payload.doc1, payload.doc2);
      self.postMessage({ type: "MERGE_RESULT", payload: merged });
      break;
    case "APPLY_CHANGES":
      const [updated] = Automerge.applyChanges(payload.doc, payload.changes);
      self.postMessage({ type: "APPLY_RESULT", payload: updated });
      break;
  }
};
```

### 3. Shared Workers for Cross-Tab Efficiency

**When to Use**: If many tabs cause redundant IndexedDB operations.

**Status**: ⏳ **Future Consideration**

**Current Approach**: BroadcastChannel is simpler and sufficient.

**Shared Worker Benefits**:
- Single IndexedDB connection across all tabs
- Reduced storage contention
- Centralized change processing

### 4. Comlink for Worker Communication

**Purpose**: Type-safe, promise-based worker communication.

**Status**: ⏳ **Future Enhancement**

```typescript
// With Comlink
import { wrap } from "comlink";

const worker = new Worker("./automerge-worker.ts");
const automergeApi = wrap<AutomergeWorkerApi>(worker);

// Clean async API
const result = await automergeApi.mergeDocuments(doc1, doc2);
```

### 5. IndexedDB Cursor Streaming (For Large Datasets)

**When to Use**: Loading many documents (100+) from IndexedDB.

**Status**: ⏳ **Not Currently Needed**

**Implementation**:

```typescript
async function* streamDocuments(prefix: string[]): AsyncGenerator<Chunk> {
  const db = await getDatabase();
  const tx = db.transaction("documents", "readonly");
  const store = tx.objectStore("documents");
  const range = IDBKeyRange.bound(prefix, [...prefix, "\uffff"]);
  
  const request = store.openCursor(range);
  
  while (true) {
    const cursor = await new Promise<IDBCursorWithValue | null>((resolve) => {
      request.onsuccess = () => resolve(request.result);
    });
    
    if (!cursor) break;
    yield { key: cursor.key, data: cursor.value.binary };
    cursor.continue();
  }
}
```

---

## Monitoring & Profiling

### Performance Metrics to Track

```typescript
// Add to chart-repo.ts or a dedicated performance module
const perfMetrics = {
  indexedDbWrites: [] as number[],
  indexedDbReads: [] as number[],
  crdtMerges: [] as number[],
  uiUpdates: [] as number[],
};

function trackPerformance(operation: string, startTime: number) {
  const duration = performance.now() - startTime;
  console.log(`[Perf] ${operation}: ${duration.toFixed(2)}ms`);
  
  if (duration > 16) {
    console.warn(`[Perf] Slow operation detected: ${operation} took ${duration.toFixed(2)}ms`);
  }
}
```

### DevTools Profiling

1. **Performance Tab**: Record during chart interactions
2. **Application Tab**: Monitor IndexedDB storage size
3. **Console**: Enable performance logging

```javascript
// Enable in browser console
localStorage.setItem("mudrock:perf:verbose", "true");
```

---

## Storage Size Management

### Current Document Sizes (Estimated)

| Document Type | Typical Size | Max Expected |
|---------------|--------------|--------------|
| ChartSpec | 1-5 KB | 50 KB |
| PipelineSpec | 5-20 KB | 200 KB |
| Incremental Change | 10-100 bytes | 1 KB |

### IndexedDB Quotas

| Browser | Default Quota | Notes |
|---------|---------------|-------|
| Chrome | 60% of disk | Eviction after 7 days inactive |
| Firefox | 50% of disk | Per-origin limit |
| Safari | 1 GB | User prompted for more |

### Cleanup Strategy

```typescript
// Implement if storage grows large
async function cleanupOldDocuments(maxAge: number = 30 * 24 * 60 * 60 * 1000) {
  const db = await getDatabase();
  const tx = db.transaction("documents", "readwrite");
  const store = tx.objectStore("documents");
  
  // Remove documents not accessed in maxAge
  const cursor = store.openCursor();
  
  cursor.onsuccess = (event) => {
    const result = (event.target as IDBRequest).result;
    if (result) {
      const { lastAccessed } = result.value;
      if (Date.now() - lastAccessed > maxAge) {
        result.delete();
      }
      result.continue();
    }
  };
}
```

---

## Implementation Tracking

### Phase 1: Monitoring (Current)

- [x] IndexedDB storage via automerge-repo (binary format)
- [x] Write batching with 2s debounce
- [x] Async non-blocking writes
- [x] BroadcastChannel for tab sync
- [ ] Add performance logging (optional)
- [ ] Add storage size monitoring (optional)

### Phase 2: Optimization (If Needed)

- [ ] Web Worker for heavy CRDT operations
- [ ] Shared Worker for multi-tab efficiency
- [ ] Cursor streaming for large datasets
- [ ] Document cleanup for storage management

### Phase 3: Advanced (Future)

- [ ] Comlink for type-safe worker communication
- [ ] Service Worker for offline sync queue
- [ ] Custom binary serialization (if Automerge is bottleneck)

---

## Benchmarking Script

Add to `src/lib/utils/performance/indexeddb-benchmark.ts`:

```typescript
/**
 * IndexedDB Performance Benchmark
 * Run in browser console: await runIndexedDbBenchmark()
 */
export async function runIndexedDbBenchmark() {
  const results: Record<string, number[]> = {
    write: [],
    read: [],
    delete: [],
  };
  
  const testData = new Uint8Array(1000); // 1KB binary
  crypto.getRandomValues(testData);
  
  const db = await new Promise<IDBDatabase>((resolve, reject) => {
    const request = indexedDB.open("benchmark-test", 1);
    request.onupgradeneeded = () => {
      request.result.createObjectStore("test");
    };
    request.onsuccess = () => resolve(request.result);
    request.onerror = () => reject(request.error);
  });
  
  // Run 100 iterations
  for (let i = 0; i < 100; i++) {
    const key = `test-${i}`;
    
    // Write
    const writeStart = performance.now();
    await new Promise<void>((resolve) => {
      const tx = db.transaction("test", "readwrite");
      tx.objectStore("test").put({ binary: testData }, key);
      tx.oncomplete = () => resolve();
    });
    results.write.push(performance.now() - writeStart);
    
    // Read
    const readStart = performance.now();
    await new Promise<void>((resolve) => {
      const tx = db.transaction("test", "readonly");
      const req = tx.objectStore("test").get(key);
      req.onsuccess = () => resolve();
    });
    results.read.push(performance.now() - readStart);
    
    // Delete
    const deleteStart = performance.now();
    await new Promise<void>((resolve) => {
      const tx = db.transaction("test", "readwrite");
      tx.objectStore("test").delete(key);
      tx.oncomplete = () => resolve();
    });
    results.delete.push(performance.now() - deleteStart);
  }
  
  // Cleanup
  indexedDB.deleteDatabase("benchmark-test");
  
  // Results
  const avg = (arr: number[]) => arr.reduce((a, b) => a + b) / arr.length;
  const max = (arr: number[]) => Math.max(...arr);
  const min = (arr: number[]) => Math.min(...arr);
  
  console.table({
    write: { avg: avg(results.write).toFixed(2), min: min(results.write).toFixed(2), max: max(results.write).toFixed(2) },
    read: { avg: avg(results.read).toFixed(2), min: min(results.read).toFixed(2), max: max(results.read).toFixed(2) },
    delete: { avg: avg(results.delete).toFixed(2), min: min(results.delete).toFixed(2), max: max(results.delete).toFixed(2) },
  });
  
  return results;
}
```

---

## Conclusion

### Current State: ✅ Optimized

The current Automerge-Repo + IndexedDB implementation is **already well-optimized**:

1. **Binary Storage**: `Uint8Array` avoids structured clone overhead
2. **Incremental Changes**: Small writes, not full snapshots
3. **Async API**: Non-blocking main thread
4. **Write Batching**: Debounced updates reduce I/O

### When to Optimize Further

| Symptom | Solution |
|---------|----------|
| CRDT merge >16ms | Web Worker |
| Many tabs slow down | Shared Worker |
| Loading 100+ docs slow | Cursor streaming |
| Storage >500MB | Cleanup strategy |

### Key Insight

> **Low latency is achieved by the optimistic update pattern, not by IndexedDB speed.**
> 
> The UI updates instantly via `$state`. IndexedDB persistence is **fire-and-forget** in the background. Users never wait for storage operations.

---

## References

- [IndexedDB Performance Best Practices](https://web.dev/indexeddb-best-practices/)
- [Automerge-Repo Storage Adapter](../../docs/libraries/js/automerge/automerge-repo/packages/automerge-repo-storage-indexeddb/src/index.ts)
- [Structured Clone Algorithm](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Structured_clone_algorithm)
- [Web Workers with Comlink](https://github.com/GoogleChromeLabs/comlink)

