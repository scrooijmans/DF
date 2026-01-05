# Offline-First Solution for Large Parquet Curve Data

This document provides a comprehensive solution for enabling offline access to large Parquet curve data files stored in object storage (MinIO/S3), based on MudRock's current storage architecture.

## Problem Statement

**Current State**:
- Large curve data stored in Parquet files in object storage (MinIO/S3)
- Users request data at runtime via `parquet-log-query-engine`
- **Problem**: Users cannot access data when offline
- **Challenge**: Parquet files can be large (MBs to GBs), cannot sync everything

**Requirements**:
1. ✅ Offline access to recently viewed/active curve data
2. ✅ Selective sync based on user activity (visible charts)
3. ✅ Background sync when online
4. ✅ Efficient storage (don't duplicate entire dataset)
5. ✅ Low-latency reads (local cache faster than network)

## Recommended Solution: Hybrid SQLite + Local Parquet Cache

Based on analysis of RxDB, ColaNode, and your existing architecture, the **best approach** is:

### **SQLite (Rust/Tauri) + Local Parquet File Cache**

**Why This Approach**:
- ✅ **Already in your stack**: You have SQLite in Tauri (`local_database.rs`)
- ✅ **No size limits**: Unlike RxDB (2GB limit), SQLite can handle large datasets
- ✅ **Native performance**: Rust SQLite is fast, no WASM overhead
- ✅ **File-based cache**: Store Parquet files on disk (efficient, queryable)
- ✅ **Selective sync**: Only cache what users need
- ✅ **Queryable**: Can use DuckDB/DataFusion to query cached Parquet files

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│  Frontend (SvelteKit)                                       │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  TanStack Virtual (visible range tracking)          │   │
│  │  - Determines which charts/series are visible       │   │
│  │  - Triggers cache requests for visible data         │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Query Service (parquet-log-query-engine)           │   │
│  │  - Checks local cache first                         │   │
│  │  - Falls back to object storage if not cached       │   │
│  └─────────────────────────────────────────────────────┘   │
└──────────────────────┬──────────────────────────────────────┘
                       │ Tauri IPC
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  Backend (Rust/Tauri)                                       │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Local SQLite Database                              │   │
│  │  - Cache metadata (what's cached, when, size)       │   │
│  │  - Sync state (pending uploads, last sync)          │   │
│  │  - User preferences                                 │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Local Parquet Cache (File System)                  │   │
│  │  ./data/cache/parquet/                              │   │
│  │  ├── project-123/                                   │   │
│  │  │   ├── wells/well-456/logs_gr.parquet            │   │
│  │  │   └── wells/well-456/logs_rhob.parquet          │   │
│  │  └── ...                                            │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Cache Manager                                      │   │
│  │  - Manages cache lifecycle                          │   │
│  │  - Handles selective sync                           │   │
│  │  - Background sync when online                      │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Sync Service                                       │   │
│  │  - Downloads Parquet files from object storage      │   │
│  │  - Uploads local changes back to object storage     │   │
│  │  - Handles conflicts                                │   │
│  └─────────────────────────────────────────────────────┘   │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       │ Background Sync (when online)
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  Object Storage (MinIO/S3)                                  │
│  - Authoritative source of Parquet files                    │
│  - Stores all curve data                                    │
└─────────────────────────────────────────────────────────────┘
```

## Implementation

### 1. SQLite Schema for Cache Management

```rust
// src-tauri/src/cache/local_parquet_cache.rs

use rusqlite::{Connection, Result};

pub struct LocalParquetCache {
    conn: Connection,
    cache_dir: PathBuf,
}

impl LocalParquetCache {
    pub fn new(cache_dir: PathBuf) -> Result<Self> {
        let db_path = cache_dir.join("cache_metadata.db");
        let conn = Connection::open(&db_path)?;
        
        // Create cache metadata tables
        conn.execute_batch(
            r#"
            -- Tracks which Parquet files are cached locally
            CREATE TABLE IF NOT EXISTS cached_parquet_files (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                storage_path TEXT NOT NULL,  -- Original S3 path
                local_path TEXT NOT NULL,    -- Local file path
                file_size INTEGER NOT NULL,  -- Size in bytes
                point_count INTEGER,         -- Number of data points
                last_accessed TEXT NOT NULL DEFAULT (datetime('now')),
                last_synced TEXT NOT NULL DEFAULT (datetime('now')),
                access_count INTEGER NOT NULL DEFAULT 0,
                cache_priority INTEGER NOT NULL DEFAULT 0,  -- Higher = more important
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            -- Tracks pending uploads (local changes to sync)
            CREATE TABLE IF NOT EXISTS pending_uploads (
                id TEXT PRIMARY KEY,
                local_path TEXT NOT NULL,
                storage_path TEXT NOT NULL,
                operation TEXT NOT NULL,  -- 'create', 'update', 'delete'
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                retry_count INTEGER NOT NULL DEFAULT 0,
                last_error TEXT
            );

            -- Tracks sync state
            CREATE TABLE IF NOT EXISTS sync_state (
                id INTEGER PRIMARY KEY,
                last_sync_timestamp TEXT,
                sync_in_progress INTEGER NOT NULL DEFAULT 0,
                last_error TEXT,
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            -- Indexes for performance
            CREATE INDEX IF NOT EXISTS idx_cached_files_project ON cached_parquet_files(project_id);
            CREATE INDEX IF NOT EXISTS idx_cached_files_last_accessed ON cached_parquet_files(last_accessed);
            CREATE INDEX IF NOT EXISTS idx_cached_files_priority ON cached_parquet_files(cache_priority DESC);
            CREATE INDEX IF NOT EXISTS idx_pending_uploads_created ON pending_uploads(created_at);
            "#
        )?;
        
        Ok(Self {
            conn,
            cache_dir,
        })
    }
}
```

### 2. Cache Manager Implementation

```rust
// src-tauri/src/cache/cache_manager.rs

use std::path::PathBuf;
use tokio::fs;
use crate::storage::object_store_adapter::ObjectStoreAdapter;

pub struct CacheManager {
    cache: LocalParquetCache,
    object_store: ObjectStoreAdapter,
    max_cache_size: u64,  // Maximum cache size in bytes (e.g., 10GB)
}

impl CacheManager {
    /// Check if a Parquet file is cached locally
    pub async fn is_cached(&self, storage_path: &str) -> Result<bool> {
        let cached = self.cache.get_cached_file(storage_path).await?;
        Ok(cached.is_some())
    }

    /// Get cached Parquet file path, or download if not cached
    pub async fn get_or_cache(
        &self,
        project_id: &str,
        storage_path: &str,
    ) -> Result<PathBuf> {
        // Check if already cached
        if let Some(cached) = self.cache.get_cached_file(storage_path).await? {
            // Update access metadata
            self.cache.update_access_metadata(&cached.id).await?;
            return Ok(PathBuf::from(&cached.local_path));
        }

        // Not cached - download from object storage
        self.download_and_cache(project_id, storage_path).await
    }

    /// Download Parquet file from object storage and cache locally
    async fn download_and_cache(
        &self,
        project_id: &str,
        storage_path: &str,
    ) -> Result<PathBuf> {
        // Ensure cache directory exists
        let cache_dir = self.cache.cache_dir.join(project_id);
        fs::create_dir_all(&cache_dir).await?;

        // Generate local path
        let local_path = cache_dir.join(
            storage_path
                .split('/')
                .last()
                .unwrap_or("data.parquet")
        );

        // Download from object storage
        let data = self.object_store.get(storage_path).await?;
        fs::write(&local_path, data).await?;

        // Get file metadata
        let metadata = fs::metadata(&local_path).await?;
        let file_size = metadata.len();

        // Register in cache database
        let cache_id = format!("{}-{}", project_id, storage_path);
        self.cache.register_cached_file(CachedFile {
            id: cache_id,
            project_id: project_id.to_string(),
            storage_path: storage_path.to_string(),
            local_path: local_path.to_string_lossy().to_string(),
            file_size,
            last_accessed: chrono::Utc::now(),
            last_synced: chrono::Utc::now(),
            access_count: 1,
            cache_priority: 0,
        }).await?;

        // Check cache size and evict if necessary
        self.evict_if_needed().await?;

        Ok(local_path)
    }

    /// Evict least recently used files if cache exceeds max size
    async fn evict_if_needed(&self) -> Result<()> {
        let total_size = self.cache.get_total_cache_size().await?;
        
        if total_size > self.max_cache_size {
            // Get files sorted by priority and last access
            let files_to_evict = self.cache.get_files_to_evict(
                total_size - self.max_cache_size
            ).await?;

            for file in files_to_evict {
                // Delete local file
                fs::remove_file(&file.local_path).await?;
                
                // Remove from cache database
                self.cache.remove_cached_file(&file.id).await?;
            }
        }

        Ok(())
    }

    /// Prefetch Parquet files for visible charts (called from frontend)
    pub async fn prefetch_for_visible_charts(
        &self,
        project_id: &str,
        chart_series: Vec<ChartSeriesInfo>,
    ) -> Result<()> {
        for series in chart_series {
            let storage_path = format!(
                "projects/{}/wells/{}/logs_{}.parquet",
                project_id,
                series.well_id,
                series.log_type
            );

            // Check if already cached
            if !self.is_cached(&storage_path).await? {
                // Download in background (don't block)
                let cache_manager = self.clone();
                let storage_path_clone = storage_path.clone();
                let project_id_clone = project_id.to_string();
                
                tokio::spawn(async move {
                    if let Err(e) = cache_manager
                        .download_and_cache(&project_id_clone, &storage_path_clone)
                        .await
                    {
                        eprintln!("Failed to prefetch {}: {}", storage_path_clone, e);
                    }
                });
            }
        }

        Ok(())
    }
}
```

### 3. Tauri Commands for Frontend Integration

```rust
// src-tauri/src/commands/cache.rs

use tauri::command;

#[command]
pub async fn get_cached_parquet_file(
    project_id: String,
    storage_path: String,
    cache_manager: tauri::State<'_, CacheManager>,
) -> Result<String, String> {
    let local_path = cache_manager
        .get_or_cache(&project_id, &storage_path)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(local_path.to_string_lossy().to_string())
}

#[command]
pub async fn prefetch_chart_data(
    project_id: String,
    chart_series: Vec<ChartSeriesInfo>,
    cache_manager: tauri::State<'_, CacheManager>,
) -> Result<(), String> {
    cache_manager
        .prefetch_for_visible_charts(&project_id, chart_series)
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn get_cache_stats(
    cache_manager: tauri::State<'_, CacheManager>,
) -> Result<CacheStats, String> {
    cache_manager
        .get_cache_stats()
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn clear_cache(
    project_id: Option<String>,
    cache_manager: tauri::State<'_, CacheManager>,
) -> Result<(), String> {
    cache_manager
        .clear_cache(project_id.as_deref())
        .await
        .map_err(|e| e.to_string())
}
```

### 4. Frontend Integration with TanStack Virtual

```typescript
// src/lib/services/parquet-cache-service.ts

import { invoke } from '@tauri-apps/api/core';

export interface ChartSeriesInfo {
  wellId: string;
  logType: string;
  storagePath: string;
}

export class ParquetCacheService {
  /**
   * Prefetch Parquet files for visible chart series
   * Called when TanStack Virtual detects visible range changes
   */
  async prefetchVisibleSeries(
    projectId: string,
    visibleSeries: ChartSeriesInfo[]
  ): Promise<void> {
    try {
      await invoke('prefetch_chart_data', {
        projectId,
        chartSeries: visibleSeries,
      });
    } catch (error) {
      console.error('Failed to prefetch chart data:', error);
    }
  }

  /**
   * Get cached Parquet file path
   * Falls back to object storage if not cached
   */
  async getCachedFile(
    projectId: string,
    storagePath: string
  ): Promise<string> {
    try {
      // Try to get from cache first
      const localPath = await invoke<string>('get_cached_parquet_file', {
        projectId,
        storagePath,
      });
      return localPath;
    } catch (error) {
      console.error('Failed to get cached file:', error);
      // Fallback to object storage path
      return storagePath;
    }
  }

  /**
   * Get cache statistics
   */
  async getCacheStats(): Promise<{
    totalSize: number;
    fileCount: number;
    hitRate: number;
  }> {
    return await invoke('get_cache_stats');
  }
}
```

### 5. Integration with Query Engine

```rust
// src-tauri/src/query/parquet_query_engine.rs

use crate::cache::CacheManager;

impl ParquetQueryEngine {
    /// Query Parquet file (checks cache first)
    pub async fn query_parquet_file(
        &self,
        project_id: &str,
        storage_path: &str,
        query: &QueryCriteria,
    ) -> Result<RecordBatch> {
        // Get cached file path (downloads if not cached)
        let file_path = self.cache_manager
            .get_or_cache(project_id, storage_path)
            .await?;

        // Query local Parquet file using DataFusion
        let ctx = SessionContext::new();
        let df = ctx
            .read_parquet(&file_path.to_string_lossy(), ParquetReadOptions::default())
            .await?;

        // Apply query filters
        let filtered_df = self.apply_query_criteria(df, query).await?;
        
        // Execute query
        let results = filtered_df.collect().await?;
        
        Ok(results)
    }
}
```

### 6. Background Sync Service

```rust
// src-tauri/src/sync/background_sync.rs

use tokio::time::{interval, Duration};

pub struct BackgroundSyncService {
    cache_manager: CacheManager,
    sync_interval: Duration,
}

impl BackgroundSyncService {
    pub fn new(cache_manager: CacheManager) -> Self {
        Self {
            cache_manager,
            sync_interval: Duration::from_secs(300), // Sync every 5 minutes
        }
    }

    pub async fn start(&self) {
        let mut interval = interval(self.sync_interval);
        
        loop {
            interval.tick().await;
            
            // Check if online
            if self.is_online().await {
                // Sync pending uploads
                if let Err(e) = self.sync_pending_uploads().await {
                    eprintln!("Sync error: {}", e);
                }

                // Update cache metadata
                if let Err(e) = self.update_cache_metadata().await {
                    eprintln!("Cache metadata update error: {}", e);
                }
            }
        }
    }

    async fn sync_pending_uploads(&self) -> Result<()> {
        let pending = self.cache_manager.get_pending_uploads().await?;
        
        for upload in pending {
            match upload.operation.as_str() {
                "create" | "update" => {
                    // Upload local file to object storage
                    let data = fs::read(&upload.local_path).await?;
                    self.cache_manager
                        .object_store
                        .put(&upload.storage_path, data)
                        .await?;
                    
                    // Mark as synced
                    self.cache_manager.mark_upload_synced(&upload.id).await?;
                }
                "delete" => {
                    // Delete from object storage
                    self.cache_manager
                        .object_store
                        .delete(&upload.storage_path)
                        .await?;
                    
                    // Mark as synced
                    self.cache_manager.mark_upload_synced(&upload.id).await?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
```

## Library Recommendations

### ✅ Recommended: SQLite (via `rusqlite`)

**Why**:
- ✅ **Already in your stack**: You have `local_database.rs` using SQLite
- ✅ **Native performance**: Rust SQLite is fast, no WASM overhead
- ✅ **No size limits**: Unlike RxDB (2GB), SQLite can handle large datasets
- ✅ **File-based**: Perfect for tracking Parquet file cache metadata
- ✅ **ACID transactions**: Reliable cache state management

**Implementation**:
```rust
// Already have this!
use rusqlite::Connection;
```

### ✅ Recommended: Local File System for Parquet Cache

**Why**:
- ✅ **Efficient**: Store Parquet files directly on disk (no database overhead)
- ✅ **Queryable**: Can use DuckDB/DataFusion to query cached files
- ✅ **Large files**: No size limits (unlike IndexedDB/RxDB)
- ✅ **Fast access**: Direct file I/O is fast

**Implementation**:
```rust
use tokio::fs;
use std::path::PathBuf;

// Cache directory structure
// ./data/cache/parquet/project-123/wells/well-456/logs_gr.parquet
```

### ❌ Not Recommended: RxDB

**Why Not**:
- ❌ **2GB limit**: Your Parquet files can exceed this
- ❌ **Browser-only**: You're using Tauri (Rust backend)
- ❌ **IndexedDB overhead**: Slower than native file system
- ❌ **Attachment limitations**: Can't query attachments efficiently

### ❌ Not Recommended: IndexedDB/Dexie.js

**Why Not**:
- ❌ **Browser-only**: You're using Tauri (Rust backend)
- ❌ **Size limits**: Browser storage quotas (varies, typically 50MB-1GB)
- ❌ **Not queryable**: Can't use DuckDB/DataFusion on IndexedDB blobs
- ❌ **Slower**: Browser storage is slower than native file system

### ✅ Optional: TanStack Query for Frontend State

**Why**:
- ✅ **Cache management**: Built-in caching and invalidation
- ✅ **Offline support**: Can work with cached data
- ✅ **Reactive**: Automatic UI updates

**Implementation**:
```typescript
import { useQuery } from '@tanstack/svelte-query';

const { data } = useQuery({
  queryKey: ['parquet-file', projectId, storagePath],
  queryFn: async () => {
    // Get from cache (via Tauri command)
    return await invoke('get_cached_parquet_file', {
      projectId,
      storagePath,
    });
  },
  staleTime: Infinity, // Cache forever (until evicted)
});
```

## Data Flow: Offline Access

### Scenario 1: User Views Chart (Online)

```
1. User opens chart with series data
   ↓
2. TanStack Virtual detects visible series
   ↓
3. Frontend calls prefetch_chart_data()
   ↓
4. Tauri: Check if Parquet file is cached
   ↓
5. If not cached: Download from object storage
   ↓
6. Save to local cache directory
   ↓
7. Register in SQLite cache metadata
   ↓
8. Query local Parquet file using DataFusion
   ↓
9. Return data to frontend
```

### Scenario 2: User Views Chart (Offline)

```
1. User opens chart with series data
   ↓
2. Frontend calls get_cached_parquet_file()
   ↓
3. Tauri: Check SQLite cache metadata
   ↓
4. If cached: Return local file path
   ↓
5. Query local Parquet file using DataFusion
   ↓
6. Return data to frontend
   ↓
7. If not cached: Return error (no offline data)
```

### Scenario 3: Background Sync (Online)

```
1. Background sync service runs every 5 minutes
   ↓
2. Check for pending uploads in SQLite
   ↓
3. Upload local changes to object storage
   ↓
4. Update cache metadata (last_synced timestamp)
   ↓
5. Check for updated files in object storage
   ↓
6. Download updates if cache is stale
```

## Cache Eviction Strategy

### LRU (Least Recently Used) with Priority

```rust
impl CacheManager {
    /// Evict files based on:
    /// 1. Priority (lower priority = evict first)
    /// 2. Last access time (older = evict first)
    /// 3. Access count (fewer accesses = evict first)
    async fn get_files_to_evict(&self, bytes_to_free: u64) -> Result<Vec<CachedFile>> {
        let mut files = self.cache.get_all_cached_files().await?;
        
        // Sort by eviction priority
        files.sort_by(|a, b| {
            // First by priority (lower = evict first)
            a.cache_priority.cmp(&b.cache_priority)
                // Then by last access (older = evict first)
                .then(a.last_accessed.cmp(&b.last_accessed))
                // Then by access count (fewer = evict first)
                .then(a.access_count.cmp(&b.access_count))
        });

        // Select files to evict
        let mut to_evict = Vec::new();
        let mut total_size = 0;
        
        for file in files {
            if total_size >= bytes_to_free {
                break;
            }
            to_evict.push(file.clone());
            total_size += file.file_size;
        }

        Ok(to_evict)
    }
}
```

### Priority Assignment

```rust
impl CacheManager {
    /// Assign cache priority based on:
    /// - Visible charts: Priority 10
    /// - Recently viewed: Priority 5
    /// - User favorites: Priority 8
    /// - Default: Priority 0
    fn assign_priority(&self, series: &ChartSeriesInfo) -> i32 {
        if series.is_visible {
            10  // Highest priority for visible data
        } else if series.is_favorite {
            8   // High priority for favorites
        } else if series.last_viewed > chrono::Utc::now() - chrono::Duration::hours(24) {
            5   // Medium priority for recently viewed
        } else {
            0   // Low priority (default)
        }
    }
}
```

## Performance Considerations

### Cache Size Management

```rust
// Recommended cache sizes
const MAX_CACHE_SIZE: u64 = 10 * 1024 * 1024 * 1024; // 10GB
const WARN_CACHE_SIZE: u64 = 8 * 1024 * 1024 * 1024;  // 8GB (warn user)

// Per-project cache limits
const MAX_CACHE_PER_PROJECT: u64 = 2 * 1024 * 1024 * 1024; // 2GB per project
```

### Query Performance

```rust
// Query cached Parquet file (local) vs object storage (remote)
// Local: ~10-50ms
// Remote: ~100-500ms (depending on network)

// Use local cache when available
if cache_manager.is_cached(&storage_path).await? {
    // Query local file (fast)
    query_local_parquet(&local_path).await
} else {
    // Query remote file (slower, but works)
    query_remote_parquet(&storage_path).await
}
```

## Summary

### ✅ Recommended Stack

1. **SQLite** (`rusqlite`) - Cache metadata and sync state
2. **Local File System** - Store Parquet files on disk
3. **TanStack Virtual** - Determine visible data to prefetch
4. **TanStack Query** (optional) - Frontend cache management
5. **DataFusion/DuckDB** - Query cached Parquet files

### ❌ Not Recommended

1. **RxDB** - 2GB limit, browser-only, not suitable for large Parquet files
2. **IndexedDB** - Size limits, browser-only, not queryable
3. **Automerge-Repo** - Not designed for large binary files

### Key Benefits

- ✅ **Offline access**: Users can access cached data offline
- ✅ **Selective sync**: Only cache what users need
- ✅ **Efficient storage**: No size limits, direct file access
- ✅ **Queryable**: Can use DuckDB/DataFusion on cached files
- ✅ **Native performance**: Rust SQLite + file system is fast
- ✅ **Already in stack**: Uses existing Tauri + SQLite infrastructure

This solution provides the best balance of offline capability, performance, and storage efficiency for large Parquet curve data.

