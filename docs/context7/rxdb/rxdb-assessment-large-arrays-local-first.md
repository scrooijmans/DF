# RxDB Assessment: Local-First Solution for Large Array Data

This document assesses whether RxDB is an appropriate choice for maintaining changes to computed large array data that can later be synced with common storage for users.

## Executive Summary

**Verdict: ⚠️ Partially Suitable with Limitations**

RxDB is **excellent for local-first applications** but has **significant limitations for very large array data** (10,000s+ points). It's best suited when:
- ✅ Array data can be stored as **attachments** (binary blobs)
- ✅ Data can be **chunked/partitioned** per user or scope
- ✅ Total dataset per user is **under 2GB**
- ✅ You need **real-time reactive queries** on metadata

**Not recommended** if:
- ❌ You need to store **millions of points** directly in documents
- ❌ Arrays must be **queryable/indexed** as structured data
- ❌ You need **SQL-like queries** on array elements
- ❌ Total dataset exceeds **2GB per user**

## Use Case Analysis

### Your Requirements

Based on the database design document, you need to:
1. Store **large arrays** (10,000s of points) - X/Y values, time series
2. Maintain **changes** to computed data locally
3. **Sync** with common storage (PostgreSQL + Object Storage)
4. **Low-latency retrieval** for real-time visualization
5. **Selective queries** (specific ranges, columns)

## RxDB Capabilities

### ✅ Strengths

#### 1. Local-First Architecture

RxDB is designed for local-first applications:

```typescript
import { createRxDatabase } from 'rxdb';
import { getRxStorageDexie } from 'rxdb/plugins/storage-dexie';

const db = await createRxDatabase({
  name: 'mydb',
  storage: getRxStorageDexie(), // Uses IndexedDB
  eventBus: new Subject(),
});

// All operations are local-first
const collection = await db.addCollections({
  chartSeries: {
    schema: chartSeriesSchema,
  },
});

// Write locally - instant, no network latency
await collection.insert({
  id: 'series-1',
  chartId: 'chart-1',
  seriesName: 'Temperature',
  // ... metadata
});
```

**Benefits**:
- ✅ **Zero-latency writes**: All writes happen locally first
- ✅ **Offline support**: Works without network connection
- ✅ **Reactive queries**: Automatic UI updates when data changes
- ✅ **Background sync**: Network operations don't block UI

#### 2. Attachment Support for Large Data

RxDB supports **attachments** for storing large binary data separately from documents:

```typescript
import { addRxPlugin } from 'rxdb';
import { RxDBAttachmentsPlugin } from 'rxdb/plugins/attachments';

addRxPlugin(RxDBAttachmentsPlugin);

// Store large array as attachment
const series = await collection.findOne('series-1').exec();
const arrayData = new Float64Array([1.0, 2.0, 3.0, ...]); // 10,000+ points

await series.putAttachment({
  id: 'data',
  data: new Blob([arrayData.buffer], { type: 'application/octet-stream' }),
  type: 'application/octet-stream',
});

// Attachments are NOT loaded into memory until explicitly requested
const attachment = await series.getAttachment('data');
const data = await attachment.getData(); // Only loads when needed
```

**Benefits**:
- ✅ **Memory efficient**: Attachments not loaded until requested
- ✅ **Compression support**: Can compress attachment data
- ✅ **Separate from document**: Large data doesn't bloat document queries
- ✅ **Sync support**: Attachments sync with replication

#### 3. Replication/Sync Engine

RxDB has a powerful replication engine for syncing with servers:

```typescript
import { replicateRxCollection } from 'rxdb/plugins/replication';

const replicationState = replicateRxCollection({
  collection: db.chartSeries,
  replicationIdentifier: 'chart-series-sync',
  pull: {
    async handler(lastCheckpoint, batchSize) {
      // Fetch updates from server
      const response = await fetch(
        `/api/chart-series/pull?checkpoint=${lastCheckpoint}&limit=${batchSize}`
      );
      const { documents, checkpoint } = await response.json();
      return { documents, checkpoint };
    },
  },
  push: {
    async handler(documents) {
      // Send local changes to server
      await fetch('/api/chart-series/push', {
        method: 'POST',
        body: JSON.stringify(documents),
      });
      return []; // Return conflicts if any
    },
  },
  live: true, // Continuous sync
  retryTime: 5000, // Retry on errors
});
```

**Benefits**:
- ✅ **Automatic conflict resolution**: Handles concurrent edits
- ✅ **Incremental sync**: Only syncs changes (deltas)
- ✅ **Offline queue**: Queues changes when offline
- ✅ **Resumable**: Can resume from last checkpoint

#### 4. Partial/Chunk-Based Replication

RxDB supports **dynamic replication** for large datasets:

```typescript
// Only sync data for visible charts
function syncVisibleCharts(chartIds: string[]) {
  chartIds.forEach(chartId => {
    const replicationState = replicateRxCollection({
      collection: db.chartSeries,
      replicationIdentifier: `chart-${chartId}`,
      pull: {
        async handler(lastCheckpoint) {
          // Only pull data for this specific chart
          return await fetchChartSeries(chartId, lastCheckpoint);
        },
      },
      push: {
        async handler(docs) {
          // Push changes for this chart
          return await pushChartSeries(chartId, docs);
        },
      },
    });
  });
}
```

**Benefits**:
- ✅ **Selective sync**: Only sync what's needed
- ✅ **Bandwidth efficient**: Reduces network usage
- ✅ **Scalable**: Can handle large datasets by chunking

#### 5. Compression

RxDB supports **key compression** and **attachment compression**:

```typescript
const schema = {
  version: 0,
  primaryKey: 'id',
  type: 'object',
  properties: {
    id: { type: 'string' },
    chartId: { type: 'string' },
    seriesName: { type: 'string' },
    // ... other fields
  },
  keyCompression: true, // Compress field names
  attachments: {
    encrypted: false,
    compression: 'gzip', // Compress attachment data
  },
};
```

**Benefits**:
- ✅ **Reduced storage**: Smaller database size
- ✅ **Faster sync**: Less data to transfer
- ✅ **Better performance**: Faster queries

### ⚠️ Limitations

#### 1. Data Size Limits

**Critical Limitation**: RxDB documentation states:

> "Local-first works best when the data set per user is reasonably sized (up to 2 Gigabytes)."

**Implications for Large Arrays**:
- ❌ **10,000 points** × 16 bytes (Float64 × 2 for X/Y) = **160KB per series** ✅ OK
- ❌ **100,000 points** × 16 bytes = **1.6MB per series** ⚠️ Getting large
- ❌ **1,000,000 points** × 16 bytes = **16MB per series** ❌ Too large for documents
- ❌ **Multiple series** can quickly exceed 2GB limit

**Solution**: Use **attachments** for arrays > 1MB, but this limits queryability.

#### 2. No Native Array Indexing

RxDB is a **document database** (NoSQL), not optimized for querying array elements:

```typescript
// ❌ CANNOT do this efficiently:
// "Find all series where array contains value > 100"

// ✅ CAN only query document-level fields:
const results = await collection
  .find({
    selector: {
      chartId: 'chart-1', // ✅ Works
      // arrayValue: { $gt: 100 } // ❌ Not supported
    },
  })
  .exec();
```

**Implications**:
- ❌ Cannot index or query **within** arrays
- ❌ Cannot filter by array values efficiently
- ❌ Must load entire array to search/filter

#### 3. Attachment Limitations

While attachments solve size issues, they have trade-offs:

```typescript
// Attachments are binary blobs - not queryable
const attachment = await series.getAttachment('data');
const data = await attachment.getData(); // Must load entire blob

// ❌ Cannot query: "Get points where x > 100"
// ❌ Cannot index array elements
// ❌ Must deserialize entire array to access
```

**Implications**:
- ❌ **No selective reads**: Must load entire attachment
- ❌ **No indexing**: Cannot create indexes on array elements
- ❌ **Manual parsing**: Must parse binary data yourself

#### 4. Sync Overhead for Large Attachments

Large attachments can slow down sync:

```typescript
// When syncing, entire attachment must be transferred
replicationState.events$.subscribe(event => {
  if (event.type === 'pushed') {
    // If attachment is 16MB, entire 16MB is sent
    // No delta sync for attachments (only metadata)
  }
});
```

**Implications**:
- ⚠️ **Full transfer**: Changed attachments send full data (not deltas)
- ⚠️ **Bandwidth**: Large attachments consume significant bandwidth
- ⚠️ **Slow sync**: Initial sync can be slow with many large attachments

## Comparison with Alternatives

### RxDB vs. Current Architecture (Parquet + PostgreSQL)

| Feature | RxDB | Parquet + PostgreSQL |
|---------|------|----------------------|
| **Local Storage** | ✅ IndexedDB (browser) | ❌ Requires object storage |
| **Array Size** | ⚠️ 2GB limit per user | ✅ Unlimited |
| **Query Performance** | ❌ No array indexing | ✅ Column pruning, predicate pushdown |
| **Selective Reads** | ❌ Must load entire attachment | ✅ Read only needed columns |
| **Compression** | ✅ Gzip (moderate) | ✅ Snappy/ZSTD (excellent) |
| **Sync** | ✅ Built-in replication | ⚠️ Custom implementation needed |
| **Offline Support** | ✅ Full offline-first | ⚠️ Requires custom caching |
| **Real-time Updates** | ✅ Reactive queries | ⚠️ Requires WebSocket/Realtime |

### RxDB vs. SQLite (ColaNode Pattern)

| Feature | RxDB | SQLite |
|---------|------|--------|
| **Local Storage** | ✅ IndexedDB (browser) | ⚠️ Requires native/Electron |
| **Array Storage** | ⚠️ Attachments (binary) | ✅ BLOB or JSON |
| **Query Language** | ❌ NoSQL (limited) | ✅ SQL (powerful) |
| **Array Queries** | ❌ Not supported | ✅ JSON functions, full-text search |
| **Sync** | ✅ Built-in replication | ⚠️ Custom implementation |
| **Reactive** | ✅ Observable queries | ❌ Manual change detection |
| **Bundle Size** | ⚠️ ~200KB | ✅ Native (0KB) or WASM (939KB) |

### RxDB vs. Automerge-Repo (Current MudRock Pattern)

| Feature | RxDB | Automerge-Repo |
|---------|------|----------------|
| **CRDT** | ❌ No (conflict resolution via server) | ✅ Yes (automatic merge) |
| **Array Size** | ⚠️ 2GB limit | ⚠️ Similar limits |
| **Query** | ✅ NoSQL queries | ❌ No queries (load entire doc) |
| **Sync** | ✅ Built-in replication | ✅ Built-in sync |
| **Conflict Resolution** | ⚠️ Server-based | ✅ Automatic (CRDT) |
| **Reactive** | ✅ Observable queries | ✅ Reactive stores |

## Recommended Architecture for Large Arrays

### Hybrid Approach: RxDB + Object Storage

Based on the assessment, here's a recommended architecture:

```
┌─────────────────────────────────────────────────────────────┐
│  Frontend (RxDB)                                            │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │  RxDB Collections                                  │    │
│  │  - chartSeries (metadata only)                     │    │
│  │  - chartConfig                                     │    │
│  │  - userPreferences                                 │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │  RxDB Attachments (for computed arrays)            │    │
│  │  - series-1-data.bin (Float64Array)               │    │
│  │  - series-2-data.bin                               │    │
│  │  - Compressed with gzip                            │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │  RxDB Replication                                  │    │
│  │  - Sync metadata to PostgreSQL                     │    │
│  │  - Sync attachments to Object Storage              │    │
│  └────────────────────────────────────────────────────┘    │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       │ Sync
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  Backend (PostgreSQL + Object Storage)                      │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │  PostgreSQL                                        │    │
│  │  - chart_series (metadata)                         │    │
│  │  - storage_path (reference to object storage)      │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │  Object Storage (S3/MinIO)                        │    │
│  │  - series-1-data.parquet (compressed)             │    │
│  │  - series-2-data.parquet                          │    │
│  └────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

### Implementation Pattern

```typescript
// 1. Store metadata in RxDB
const series = await db.chartSeries.insert({
  id: 'series-1',
  chartId: 'chart-1',
  seriesName: 'Temperature',
  pointCount: 10000,
  xMin: 0,
  xMax: 100,
  yMin: -10,
  yMax: 50,
  storagePath: 's3://bucket/series-1-data.parquet', // Reference
  computedAt: new Date().toISOString(),
});

// 2. Store large array as attachment (local cache)
const arrayData = new Float64Array([...]); // 10,000 points
await series.putAttachment({
  id: 'data',
  data: new Blob([arrayData.buffer], { type: 'application/octet-stream' }),
  type: 'application/octet-stream',
});

// 3. Sync metadata to PostgreSQL
const replicationState = replicateRxCollection({
  collection: db.chartSeries,
  replicationIdentifier: 'chart-series-sync',
  pull: {
    async handler(lastCheckpoint) {
      // Pull metadata from PostgreSQL
      const response = await fetch(`/api/chart-series/pull?checkpoint=${lastCheckpoint}`);
      return await response.json();
    },
  },
  push: {
    async handler(documents) {
      // Push metadata to PostgreSQL
      await fetch('/api/chart-series/push', {
        method: 'POST',
        body: JSON.stringify(documents),
      });
      
      // Also sync attachments to object storage
      for (const doc of documents) {
        const series = await db.chartSeries.findOne(doc.id).exec();
        const attachment = await series.getAttachment('data');
        if (attachment) {
          const data = await attachment.getData();
          // Upload to S3/MinIO
          await uploadToObjectStorage(doc.storagePath, data);
        }
      }
      
      return [];
    },
  },
});
```

## Recommendations

### ✅ Use RxDB If:

1. **Metadata Management**: You need reactive queries on chart/series metadata
2. **Moderate Array Sizes**: Arrays are < 1MB each, total < 2GB per user
3. **Local-First Priority**: Offline support and instant UI updates are critical
4. **Simple Queries**: You only query document-level fields, not array elements
5. **Real-time Collaboration**: You need reactive updates across tabs/devices

### ❌ Don't Use RxDB If:

1. **Very Large Arrays**: Arrays are > 10MB each or total > 2GB per user
2. **Array Queries**: You need to query/filter within arrays
3. **Selective Reads**: You need to read only specific ranges of arrays
4. **SQL Queries**: You need complex SQL-like queries on array data
5. **Columnar Operations**: You need column pruning, predicate pushdown

### 🔄 Hybrid Approach (Recommended):

1. **RxDB for Metadata**: Use RxDB for chart/series metadata, user preferences, config
2. **Attachments for Caching**: Use RxDB attachments as **local cache** for recently accessed arrays
3. **Object Storage for Source**: Store authoritative data in Parquet files (S3/MinIO)
4. **Selective Sync**: Only sync attachments for visible/active charts
5. **Background Sync**: Sync attachments to object storage in background

## Performance Considerations

### Storage Efficiency

```typescript
// ❌ BAD: Store large array in document
const series = {
  id: 'series-1',
  data: [1.0, 2.0, 3.0, ...], // 10,000 points = 160KB in JSON
  // Document becomes huge, slow to query
};

// ✅ GOOD: Store as attachment
const series = {
  id: 'series-1',
  pointCount: 10000,
  // Metadata only, fast queries
};
await series.putAttachment({
  id: 'data',
  data: new Blob([float64Array.buffer]), // Binary, compressed
});
```

### Query Performance

```typescript
// ✅ FAST: Query metadata
const results = await db.chartSeries
  .find({
    selector: { chartId: 'chart-1' },
  })
  .exec(); // Only loads metadata, not arrays

// ⚠️ SLOW: Access attachment
const series = await db.chartSeries.findOne('series-1').exec();
const attachment = await series.getAttachment('data');
const data = await attachment.getData(); // Loads entire array
```

### Sync Performance

```typescript
// ✅ EFFICIENT: Sync only metadata
replicationState.events$.subscribe(event => {
  if (event.type === 'pushed') {
    // Only metadata is synced (small, fast)
  }
});

// ⚠️ SLOW: Sync attachments
// Attachments sync separately, can be slow for large files
// Consider: Only sync attachments for active charts
```

## Conclusion

**RxDB is a good choice for local-first applications**, but has **limitations for very large array data**. For your use case:

1. **Use RxDB for**: Metadata, configuration, user preferences, reactive queries
2. **Use Attachments for**: Local caching of computed arrays (< 1MB each)
3. **Use Object Storage for**: Authoritative storage of large arrays (Parquet)
4. **Use Selective Sync**: Only sync attachments for visible/active charts

**Best Architecture**: **RxDB (metadata) + Attachments (cache) + Object Storage (source)**

This gives you:
- ✅ Local-first experience (RxDB)
- ✅ Offline support (RxDB)
- ✅ Reactive queries (RxDB)
- ✅ Scalability (Object Storage)
- ✅ Query performance (Parquet + DuckDB)
- ✅ Selective reads (Parquet column pruning)

## References

- [RxDB Documentation](https://rxdb.info/)
- [RxDB Local-First Limitations](https://rxdb.info/articles/local-first-future)
- [RxDB Attachments](https://rxdb.info/rx-attachment)
- [RxDB Replication](https://rxdb.info/replication)
- [Database Design for Large Arrays](./database-design-large-arrays-low-latency.md)

