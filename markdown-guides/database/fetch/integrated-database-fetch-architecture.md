# Integrated Database Fetch Architecture - PostgREST + OpenDAL + Storage Layer

## 🎯 Overview

This document outlines how to integrate the existing PostgREST Supabase database fetch operations with our modular OpenDAL and storage layer architecture, creating a unified data access system that leverages both database metadata and object storage.

## 🏗️ Current Architecture Analysis

### **Existing PostgREST Implementation**

**Current Components:**

- ✅ **`postgres_query/data_fetching.rs`**: PostgREST client for database operations
- ✅ **`postgres_query/types.rs`**: Database entity types (Well, Project, Team, User)
- ✅ **Tauri Commands**: `fetch_table_data`, `fetch_wells`, `fetch_projects`
- ✅ **Frontend Integration**: Svelte components with reactive state management

**Current Capabilities:**

- Database table discovery and data fetching
- Well, project, team, and user data retrieval
- PostgREST API integration with Supabase
- Direct PostgreSQL fallback support

### **Current Storage Layer Architecture**

**OpenDAL Integration:**

- ✅ **`opendal-storage-adapter`**: Unified storage operations with MinIO/S3
- ✅ **`opendal-parquet-query-service`**: Intelligent caching for Parquet data
- ✅ **`project-data-layout`**: Centralized path management
- ✅ **`storage-manager`**: Well-specific storage operations

## 🚀 Integrated Architecture Design

### **1. Unified Data Access Layer**

Create a unified data access layer that combines database metadata with object storage operations:

```rust
// New unified data access service
pub struct UnifiedDataAccessService {
    // Database operations
    database_client: DatabaseClient,

    // Storage operations
    storage_adapter: EnhancedOpenDALStorageAdapter,

    // Caching layer
    cache_manager: CacheManager,

    // Path management
    layout_manager: ProjectDataLayoutManager,
}

impl UnifiedDataAccessService {
    /// Get well data with associated storage metadata
    pub async fn get_well_with_storage_info(
        &self,
        well_id: &str,
        project_id: &str,
    ) -> Result<WellWithStorageInfo, DataAccessError> {
        // 1. Fetch well metadata from database
        let well_metadata = self.database_client.get_well(well_id).await?;

        // 2. Get storage information using OpenDAL
        let storage_info = self.get_well_storage_info(project_id, well_id).await?;

        // 3. Combine metadata and storage info
        Ok(WellWithStorageInfo {
            well: well_metadata,
            storage_info,
            last_updated: storage_info.last_modified,
        })
    }

    /// Get project data with storage statistics
    pub async fn get_project_with_storage_stats(
        &self,
        project_id: &str,
    ) -> Result<ProjectWithStorageStats, DataAccessError> {
        // 1. Fetch project metadata
        let project = self.database_client.get_project(project_id).await?;

        // 2. Get storage statistics using OpenDAL
        let storage_stats = self.get_project_storage_stats(project_id).await?;

        // 3. Get well count and file counts
        let well_count = self.database_client.count_wells_in_project(project_id).await?;

        Ok(ProjectWithStorageStats {
            project,
            storage_stats,
            well_count,
            total_files: storage_stats.file_count,
            total_size: storage_stats.total_size,
        })
    }
}
```

### **2. Enhanced Database Client with Storage Integration**

Extend the existing `DatabaseClient` to include storage-aware operations:

```rust
// Enhanced database client with storage integration
pub struct EnhancedDatabaseClient {
    postgrest_client: Postgrest,
    storage_adapter: Arc<EnhancedOpenDALStorageAdapter>,
    cache_manager: Arc<CacheManager>,
}

impl EnhancedDatabaseClient {
    /// Get wells with storage file information
    pub async fn get_wells_with_storage_info(
        &self,
        project_id: Option<&str>,
    ) -> Result<Vec<WellWithStorageInfo>, DatabaseError> {
        // 1. Fetch wells from database
        let wells = self.fetch_wells_from_db(project_id).await?;

        // 2. Enrich with storage information
        let mut wells_with_storage = Vec::new();
        for well in wells {
            let storage_info = self.get_well_storage_info(&well.id).await?;
            wells_with_storage.push(WellWithStorageInfo {
                well,
                storage_info,
                last_updated: storage_info.last_modified,
            });
        }

        Ok(wells_with_storage)
    }

    /// Get project storage statistics
    pub async fn get_project_storage_stats(
        &self,
        project_id: &str,
    ) -> Result<ProjectStorageStats, DatabaseError> {
        // 1. Get project metadata
        let project = self.get_project(project_id).await?;

        // 2. Get storage statistics using OpenDAL
        let bucket_name = format!("project-{}", project_id);
        let storage_stats = self.storage_adapter.get_bucket_stats(&bucket_name).await?;

        // 3. Get well count from database
        let well_count = self.count_wells_in_project(project_id).await?;

        Ok(ProjectStorageStats {
            project_id: project_id.to_string(),
            project_name: project.name,
            total_files: storage_stats.file_count,
            total_size: storage_stats.total_size,
            well_count,
            last_updated: storage_stats.last_modified,
            storage_breakdown: storage_stats.breakdown_by_type,
        })
    }
}
```

### **3. Storage-Aware Data Types**

Extend existing database types to include storage information:

```rust
// Enhanced data types with storage information
#[derive(Debug, Serialize, Deserialize)]
pub struct WellWithStorageInfo {
    pub well: Well,
    pub storage_info: WellStorageInfo,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WellStorageInfo {
    pub well_id: String,
    pub project_id: String,
    pub file_count: u32,
    pub total_size: u64,
    pub log_files: Vec<LogFileInfo>,
    pub marker_files: Vec<MarkerFileInfo>,
    pub trajectory_files: Vec<TrajectoryFileInfo>,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectWithStorageStats {
    pub project: Project,
    pub storage_stats: ProjectStorageStats,
    pub well_count: u32,
    pub total_files: u32,
    pub total_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectStorageStats {
    pub project_id: String,
    pub project_name: String,
    pub total_files: u32,
    pub total_size: u64,
    pub well_count: u32,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub storage_breakdown: StorageBreakdown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageBreakdown {
    pub wells: u32,
    pub logs: u32,
    pub markers: u32,
    pub trajectories: u32,
    pub analysis: u32,
}
```

### **4. Caching Strategy for Database + Storage Operations**

Implement intelligent caching that combines database metadata with storage information:

```rust
// Enhanced cache manager for database + storage operations
pub struct DatabaseStorageCacheManager {
    // Database metadata cache
    metadata_cache: Arc<MokaCache<String, DatabaseMetadata>>,

    // Storage information cache
    storage_cache: Arc<MokaCache<String, StorageInfo>>,

    // Combined data cache
    combined_cache: Arc<MokaCache<String, CombinedData>>,
}

impl DatabaseStorageCacheManager {
    /// Get cached well data with storage information
    pub async fn get_well_with_storage(
        &self,
        well_id: &str,
        project_id: &str,
    ) -> Option<WellWithStorageInfo> {
        let cache_key = format!("well:{project_id}:{well_id}");
        self.combined_cache.get(&cache_key).await
    }

    /// Cache well data with storage information
    pub async fn cache_well_with_storage(
        &self,
        well_id: &str,
        project_id: &str,
        data: WellWithStorageInfo,
    ) {
        let cache_key = format!("well:{project_id}:{well_id}");
        self.combined_cache.insert(cache_key, data).await;
    }

    /// Invalidate cache when storage changes
    pub async fn invalidate_storage_cache(&self, project_id: &str, well_id: Option<&str>) {
        if let Some(well_id) = well_id {
            let cache_key = format!("well:{project_id}:{well_id}");
            self.combined_cache.invalidate(&cache_key).await;
        } else {
            // Invalidate all project caches
            self.combined_cache.invalidate_pattern(&format!("well:{project_id}:*")).await;
        }
    }
}
```

### **5. Enhanced Tauri Commands**

Extend existing Tauri commands to include storage-aware operations:

```rust
// Enhanced Tauri commands with storage integration
#[tauri::command]
async fn get_wells_with_storage_info(
    project_id: Option<String>,
) -> Result<Vec<WellWithStorageInfo>, String> {
    println!("🚀 Getting wells with storage information...");

    let unified_service = UnifiedDataAccessService::new().await?;

    match unified_service.get_wells_with_storage_info(project_id.as_deref()).await {
        Ok(wells) => {
            println!("✅ Successfully fetched {} wells with storage info", wells.len());
            Ok(wells)
        }
        Err(e) => {
            println!("❌ Failed to fetch wells with storage info: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn get_project_storage_stats(
    project_id: String,
) -> Result<ProjectWithStorageStats, String> {
    println!("🚀 Getting project storage statistics...");

    let unified_service = UnifiedDataAccessService::new().await?;

    match unified_service.get_project_with_storage_stats(&project_id).await {
        Ok(stats) => {
            println!("✅ Successfully fetched project storage stats");
            Ok(stats)
        }
        Err(e) => {
            println!("❌ Failed to fetch project storage stats: {}", e);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
async fn get_well_storage_info(
    project_id: String,
    well_id: String,
) -> Result<WellStorageInfo, String> {
    println!("🚀 Getting well storage information...");

    let unified_service = UnifiedDataAccessService::new().await?;

    match unified_service.get_well_storage_info(&well_id, &project_id).await {
        Ok(info) => {
            println!("✅ Successfully fetched well storage info");
            Ok(info)
        }
        Err(e) => {
            println!("❌ Failed to fetch well storage info: {}", e);
            Err(e.to_string())
        }
    }
}
```

### **6. Frontend Integration**

Update frontend components to display combined database and storage information:

```typescript
// Enhanced frontend service for combined data
export class UnifiedDataService {
  private unifiedService: UnifiedDataAccessService;

  async getWellsWithStorageInfo(
    projectId?: string,
  ): Promise<WellWithStorageInfo[]> {
    try {
      console.log("🔍 Fetching wells with storage information...");

      const wells = await invoke<WellWithStorageInfo[]>(
        "get_wells_with_storage_info",
        {
          projectId: projectId || null,
        },
      );

      console.log("✅ Wells with storage info fetched:", wells);
      return wells;
    } catch (error) {
      console.error("❌ Failed to fetch wells with storage info:", error);
      throw new Error(`Failed to fetch wells with storage info: ${error}`);
    }
  }

  async getProjectStorageStats(
    projectId: string,
  ): Promise<ProjectWithStorageStats> {
    try {
      console.log("🔍 Fetching project storage statistics...");

      const stats = await invoke<ProjectWithStorageStats>(
        "get_project_storage_stats",
        {
          projectId,
        },
      );

      console.log("✅ Project storage stats fetched:", stats);
      return stats;
    } catch (error) {
      console.error("❌ Failed to fetch project storage stats:", error);
      throw new Error(`Failed to fetch project storage stats: ${error}`);
    }
  }
}
```

### **7. Enhanced Svelte Components**

Update Svelte components to display storage information alongside database metadata:

```svelte
<!-- Enhanced well list component with storage information -->
<script lang="ts">
  import { getWellsWithStorageInfo } from '$lib/services/unified-data-service';

  let wells: WellWithStorageInfo[] = $state([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);

  $effect(async () => {
    try {
      isLoading = true;
      error = null;

      wells = await getWellsWithStorageInfo();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to load wells';
    } finally {
      isLoading = false;
    }
  });
</script>

{#if isLoading}
  <div class="flex items-center justify-center py-8">
    <Loader2 class="h-5 w-5 animate-spin" />
    <span>Loading wells with storage information...</span>
  </div>
{:else if error}
  <div class="bg-red-50 border border-red-200 rounded-md p-4">
    <AlertCircle class="h-5 w-5 text-red-400" />
    <h3 class="text-sm font-medium text-red-800">Error loading wells</h3>
    <p>{error}</p>
  </div>
{:else}
  <div class="space-y-4">
    {#each wells as well (well.well.id)}
      <div class="border rounded-lg p-4">
        <div class="flex justify-between items-start">
          <div>
            <h3 class="text-lg font-semibold">{well.well.name}</h3>
            <p class="text-sm text-gray-600">Project: {well.well.project_id}</p>
            <p class="text-sm text-gray-600">
              Files: {well.storage_info.file_count} |
              Size: {formatBytes(well.storage_info.total_size)}
            </p>
          </div>
          <div class="text-right text-sm text-gray-500">
            <p>Last Updated: {formatDate(well.last_updated)}</p>
            <p>Log Files: {well.storage_info.log_files.length}</p>
          </div>
        </div>

        <!-- Storage breakdown -->
        <div class="mt-3 grid grid-cols-2 gap-4 text-sm">
          <div>
            <span class="font-medium">Log Files:</span> {well.storage_info.log_files.length}
          </div>
          <div>
            <span class="font-medium">Marker Files:</span> {well.storage_info.marker_files.length}
          </div>
          <div>
            <span class="font-medium">Trajectory Files:</span> {well.storage_info.trajectory_files.length}
          </div>
          <div>
            <span class="font-medium">Total Size:</span> {formatBytes(well.storage_info.total_size)}
          </div>
        </div>
      </div>
    {/each}
  </div>
{/if}
```

## 🔄 Data Flow Architecture

### **Enhanced Data Flow**

```
User Request (Well List with Storage Info)
    ↓
Svelte Component (getWellsWithStorageInfo)
    ↓
Tauri Command (get_wells_with_storage_info)
    ↓
UnifiedDataAccessService
    ↓
┌─────────────────┬─────────────────┐
│ Database Client │ Storage Adapter │
│ (PostgREST)     │ (OpenDAL)       │
└─────────────────┴─────────────────┘
    ↓                     ↓
Supabase Database    MinIO/S3 Storage
    ↓                     ↓
Well Metadata        Storage Statistics
    ↓                     ↓
    └───────── Cache Manager ─────────┘
    ↓
Combined WellWithStorageInfo
    ↓
Frontend Display
```

### **Caching Strategy**

1. **Database Metadata Cache**: Cache well/project metadata from PostgREST
2. **Storage Information Cache**: Cache storage statistics from OpenDAL
3. **Combined Data Cache**: Cache combined database + storage information
4. **Cache Invalidation**: Invalidate when storage changes occur

## 🎯 Benefits of Integration

### **1. Unified Data View**

- Single API for both database metadata and storage information
- Consistent data types across database and storage operations
- Reduced complexity for frontend components

### **2. Performance Optimization**

- Intelligent caching of combined data
- Reduced API calls by batching database and storage operations
- OpenDAL caching for storage operations

### **3. Enhanced User Experience**

- Display storage statistics alongside database metadata
- Real-time storage information updates
- Better project and well management capabilities

### **4. Maintainability**

- Single service for data access operations
- Consistent error handling across database and storage
- Centralized caching strategy

## 🚀 Implementation Plan

### **Phase 1: Core Integration (1-2 weeks)**

1. **Create `UnifiedDataAccessService`**
   - Combine existing `DatabaseClient` with `EnhancedOpenDALStorageAdapter`
   - Implement basic combined data operations

2. **Extend Data Types**
   - Add storage information to existing database types
   - Create combined data structures

3. **Update Tauri Commands**
   - Add new commands for combined operations
   - Maintain backward compatibility

### **Phase 2: Caching Integration (1 week)**

1. **Implement `DatabaseStorageCacheManager`**
   - Cache database metadata
   - Cache storage information
   - Cache combined data

2. **Cache Invalidation Strategy**
   - Invalidate on storage changes
   - TTL-based expiration
   - Pattern-based invalidation

### **Phase 3: Frontend Integration (1 week)**

1. **Update Frontend Services**
   - Create `UnifiedDataService`
   - Update existing services

2. **Enhance Svelte Components**
   - Display storage information
   - Add storage statistics
   - Improve user experience

### **Phase 4: Advanced Features (1-2 weeks)**

1. **Real-time Updates**
   - WebSocket integration for storage changes
   - Automatic cache invalidation

2. **Advanced Analytics**
   - Storage usage trends
   - Project storage comparisons
   - Well storage analytics

## 🔧 Configuration

### **Environment Variables**

```bash
# Database Configuration (existing)
POSTGREST_URL=http://91.99.166.223:8000/rest/v1
POSTGRES_URL=postgresql://postgres:password@91.99.166.223:5432/postgres
SUPABASE_API_KEY=your_api_key

# Storage Configuration (existing)
MINIO_ENDPOINT=http://91.99.166.223:9000
MINIO_ACCESS_KEY=mudrock-storage
MINIO_SECRET_KEY=mudrock-storage-secret-2024

# Cache Configuration (new)
CACHE_TTL_SECONDS=3600
CACHE_MAX_SIZE=1000
CACHE_ENABLED=true
```

### **Service Configuration**

```rust
// Unified service configuration
pub struct UnifiedServiceConfig {
    pub database: DatabaseConfig,
    pub storage: StorageConfig,
    pub cache: CacheConfig,
}

impl Default for UnifiedServiceConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig::default(),
            storage: StorageConfig::default(),
            cache: CacheConfig {
                ttl_seconds: 3600,
                max_size: 1000,
                enabled: true,
            },
        }
    }
}
```

## 🎯 Conclusion

This integrated architecture provides a unified data access layer that combines the strengths of PostgREST database operations with OpenDAL storage management, creating a more powerful and user-friendly system for managing geoscience data in MudRock.

The integration maintains backward compatibility while adding powerful new capabilities for storage-aware data management, setting the foundation for advanced analytics and project management features.
