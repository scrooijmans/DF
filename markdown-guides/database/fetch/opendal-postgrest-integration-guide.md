# OpenDAL + PostgREST Integration Guide

## 🎯 Quick Integration Strategy

This guide shows how to integrate your existing PostgREST database operations with the OpenDAL storage layer architecture, creating a unified data access system.

## 🏗️ Current State Analysis

### **What You Have (Working)**

- ✅ **PostgREST Integration**: `postgres_query/data_fetching.rs` with Supabase
- ✅ **Database Types**: Well, Project, Team, User entities
- ✅ **Tauri Commands**: `fetch_table_data`, `fetch_wells`, `fetch_projects`
- ✅ **Frontend Integration**: Svelte components with reactive state
- ✅ **OpenDAL Storage**: `opendal-storage-adapter` with MinIO/S3
- ✅ **Caching Layer**: Moka caching for Parquet operations

### **What's Missing (Integration)**

- ❌ **Unified Data Access**: Database metadata + storage information
- ❌ **Storage-Aware Queries**: Wells with file counts and sizes
- ❌ **Project Storage Stats**: Storage usage per project
- ❌ **Cache Integration**: Combined database + storage caching

## 🚀 Step-by-Step Integration

### **Step 1: Create Unified Data Access Service**

Create a new service that combines PostgREST and OpenDAL:

```rust
// src-tauri/src/unified_data_service.rs
use crate::postgres_query::{DataFetcher, types::*};
use opendal_storage_adapter::EnhancedOpenDALStorageAdapter;
use project_data_layout::ProjectDataLayoutManager;
use std::sync::Arc;

pub struct UnifiedDataService {
    database_fetcher: DataFetcher,
    storage_adapter: Arc<EnhancedOpenDALStorageAdapter>,
    layout_manager: ProjectDataLayoutManager,
}

impl UnifiedDataService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let database_fetcher = DataFetcher::new();
        let storage_adapter = Arc::new(
            EnhancedOpenDALStorageAdapter::new_with_caching(
                "http://91.99.166.223:9000",
                "mudrock-storage",
                "mudrock-storage",
                "mudrock-storage-secret-2024",
            )?
        );
        let layout_manager = ProjectDataLayoutManager::with_default_config("default");

        Ok(Self {
            database_fetcher,
            storage_adapter,
            layout_manager,
        })
    }

    /// Get wells with storage information
    pub async fn get_wells_with_storage_info(
        &self,
        project_id: Option<&str>,
    ) -> Result<Vec<WellWithStorageInfo>, Box<dyn std::error::Error + Send + Sync>> {
        // 1. Fetch wells from database
        let wells = self.database_fetcher.fetch_wells().await?;

        // 2. Filter by project if specified
        let filtered_wells: Vec<Well> = if let Some(project_id) = project_id {
            wells.into_iter()
                .filter(|w| w.project_id.as_ref() == Some(&project_id.to_string()))
                .collect()
        } else {
            wells
        };

        // 3. Enrich with storage information
        let mut wells_with_storage = Vec::new();
        for well in filtered_wells {
            let storage_info = self.get_well_storage_info(&well.id.to_string(), project_id).await?;
            wells_with_storage.push(WellWithStorageInfo {
                well,
                storage_info,
            });
        }

        Ok(wells_with_storage)
    }

    /// Get well storage information using OpenDAL
    async fn get_well_storage_info(
        &self,
        well_id: &str,
        project_id: Option<&str>,
    ) -> Result<WellStorageInfo, Box<dyn std::error::Error + Send + Sync>> {
        let project_id = project_id.unwrap_or("default");
        let bucket_name = format!("project-{}", project_id);

        // Get well-specific paths
        let well_logs_path = self.layout_manager.well_logs_path(well_id);
        let well_markers_path = self.layout_manager.well_markers_path(well_id);
        let well_trajectory_path = self.layout_manager.well_trajectory_path(well_id);

        // List files in each directory
        let log_files = self.storage_adapter.list_files(&bucket_name, &well_logs_path).await?;
        let marker_files = self.storage_adapter.list_files(&bucket_name, &well_markers_path).await?;
        let trajectory_files = self.storage_adapter.list_files(&bucket_name, &well_trajectory_path).await?;

        // Calculate total size
        let total_size = log_files.iter().map(|f| f.size).sum::<u64>()
            + marker_files.iter().map(|f| f.size).sum::<u64>()
            + trajectory_files.iter().map(|f| f.size).sum::<u64>();

        Ok(WellStorageInfo {
            well_id: well_id.to_string(),
            project_id: project_id.to_string(),
            file_count: (log_files.len() + marker_files.len() + trajectory_files.len()) as u32,
            total_size,
            log_files: log_files.into_iter().map(|f| LogFileInfo {
                name: f.name,
                size: f.size,
                last_modified: f.last_modified,
            }).collect(),
            marker_files: marker_files.into_iter().map(|f| MarkerFileInfo {
                name: f.name,
                size: f.size,
                last_modified: f.last_modified,
            }).collect(),
            trajectory_files: trajectory_files.into_iter().map(|f| TrajectoryFileInfo {
                name: f.name,
                size: f.size,
                last_modified: f.last_modified,
            }).collect(),
        })
    }
}

// Enhanced data types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellWithStorageInfo {
    pub well: Well,
    pub storage_info: WellStorageInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WellStorageInfo {
    pub well_id: String,
    pub project_id: String,
    pub file_count: u32,
    pub total_size: u64,
    pub log_files: Vec<LogFileInfo>,
    pub marker_files: Vec<MarkerFileInfo>,
    pub trajectory_files: Vec<TrajectoryFileInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFileInfo {
    pub name: String,
    pub size: u64,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkerFileInfo {
    pub name: String,
    pub size: u64,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryFileInfo {
    pub name: String,
    pub size: u64,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}
```

### **Step 2: Add Storage Operations to OpenDAL Adapter**

Extend the existing OpenDAL adapter to support file listing:

```rust
// Add to opendal-storage-adapter/src/lib.rs
impl EnhancedOpenDALStorageAdapter {
    /// List files in a specific path
    pub async fn list_files(
        &self,
        bucket_name: &str,
        prefix: &str,
    ) -> Result<Vec<FileInfo>, StorageError> {
        let full_path = format!("{}/{}", bucket_name, prefix);

        let mut entries = Vec::new();
        let mut lister = self.operator.lister(&full_path).await?;

        while let Some(entry) = lister.next().await? {
            let metadata = entry.metadata();
            entries.push(FileInfo {
                name: entry.name().to_string(),
                size: metadata.content_length(),
                last_modified: metadata.last_modified(),
            });
        }

        Ok(entries)
    }
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}
```

### **Step 3: Add New Tauri Commands**

Add new commands that combine database and storage operations:

```rust
// Add to src-tauri/src/main.rs
use crate::unified_data_service::{UnifiedDataService, WellWithStorageInfo};

#[tauri::command]
async fn get_wells_with_storage_info(
    project_id: Option<String>,
) -> Result<Vec<WellWithStorageInfo>, String> {
    println!("🚀 Getting wells with storage information...");

    let unified_service = match UnifiedDataService::new() {
        Ok(service) => service,
        Err(e) => {
            println!("❌ Failed to create unified service: {}", e);
            return Err(format!("Failed to create unified service: {}", e));
        }
    };

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

// Register the new command
.invoke_handler(tauri::generate_handler![
    // ... existing commands
    get_wells_with_storage_info,
    // ... more commands
])
```

### **Step 4: Update Frontend Service**

Create a new frontend service for the unified operations:

```typescript
// src/lib/services/unified-data-service.ts
import { invoke } from "@tauri-apps/api/core";

export interface WellWithStorageInfo {
  well: {
    id: number;
    name: string;
    team_id?: string;
    x?: number;
    y?: number;
    project_id?: string;
    created_at: string;
    updated_at: string;
  };
  storage_info: {
    well_id: string;
    project_id: string;
    file_count: number;
    total_size: number;
    log_files: Array<{
      name: string;
      size: number;
      last_modified?: string;
    }>;
    marker_files: Array<{
      name: string;
      size: number;
      last_modified?: string;
    }>;
    trajectory_files: Array<{
      name: string;
      size: number;
      last_modified?: string;
    }>;
  };
}

export async function getWellsWithStorageInfo(
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

// Utility function to format file sizes
export function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 Bytes";

  const k = 1024;
  const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
}
```

### **Step 5: Update Svelte Component**

Update your existing well list component to show storage information:

```svelte
<!-- src/lib/components/pages/home/content-main/content-data/wells-list-enhanced.svelte -->
<script lang="ts">
  import { getWellsWithStorageInfo, formatBytes } from '$lib/services/unified-data-service';
  import { Loader2, Database, FileText, MapPin, Calendar } from 'lucide-svelte';

  let wells: WellWithStorageInfo[] = $state([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);
  let selectedProject = $state<string | null>(null);

  $effect(async () => {
    try {
      isLoading = true;
      error = null;

      wells = await getWellsWithStorageInfo(selectedProject || undefined);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to load wells';
    } finally {
      isLoading = false;
    }
  });
</script>

<div class="space-y-6">
  <!-- Project Filter -->
  <div class="flex items-center space-x-4">
    <label for="project-filter" class="text-sm font-medium text-gray-700">
      Filter by Project:
    </label>
    <select
      id="project-filter"
      bind:value={selectedProject}
      class="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
    >
      <option value="">All Projects</option>
      <!-- Add your project options here -->
    </select>
  </div>

  <!-- Loading State -->
  {#if isLoading}
    <div class="flex items-center justify-center py-8">
      <Loader2 class="h-5 w-5 animate-spin mr-2" />
      <span>Loading wells with storage information...</span>
    </div>
  {:else if error}
    <div class="bg-red-50 border border-red-200 rounded-md p-4">
      <div class="flex">
        <Database class="h-5 w-5 text-red-400 mr-2" />
        <div>
          <h3 class="text-sm font-medium text-red-800">Error loading wells</h3>
          <p class="text-sm text-red-700 mt-1">{error}</p>
        </div>
      </div>
    </div>
  {:else}
    <!-- Wells List -->
    <div class="grid gap-4">
      {#each wells as well (well.well.id)}
        <div class="border border-gray-200 rounded-lg p-6 hover:shadow-md transition-shadow">
          <div class="flex justify-between items-start mb-4">
            <div>
              <h3 class="text-lg font-semibold text-gray-900">{well.well.name}</h3>
              <p class="text-sm text-gray-600">
                Project: {well.storage_info.project_id || 'No Project'}
              </p>
              {#if well.well.x && well.well.y}
                <p class="text-sm text-gray-600">
                  <MapPin class="h-4 w-4 inline mr-1" />
                  Location: {well.well.x.toFixed(2)}, {well.well.y.toFixed(2)}
                </p>
              {/if}
            </div>
            <div class="text-right text-sm text-gray-500">
              <p class="flex items-center">
                <Calendar class="h-4 w-4 mr-1" />
                {new Date(well.well.created_at).toLocaleDateString()}
              </p>
            </div>
          </div>

          <!-- Storage Information -->
          <div class="bg-gray-50 rounded-md p-4">
            <h4 class="text-sm font-medium text-gray-900 mb-3 flex items-center">
              <FileText class="h-4 w-4 mr-2" />
              Storage Information
            </h4>

            <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
              <div>
                <span class="font-medium text-gray-700">Total Files:</span>
                <span class="ml-1 text-gray-900">{well.storage_info.file_count}</span>
              </div>
              <div>
                <span class="font-medium text-gray-700">Total Size:</span>
                <span class="ml-1 text-gray-900">{formatBytes(well.storage_info.total_size)}</span>
              </div>
              <div>
                <span class="font-medium text-gray-700">Log Files:</span>
                <span class="ml-1 text-gray-900">{well.storage_info.log_files.length}</span>
              </div>
              <div>
                <span class="font-medium text-gray-700">Marker Files:</span>
                <span class="ml-1 text-gray-900">{well.storage_info.marker_files.length}</span>
              </div>
            </div>

            <!-- File Details -->
            {#if well.storage_info.log_files.length > 0}
              <div class="mt-3">
                <h5 class="text-xs font-medium text-gray-700 mb-2">Log Files:</h5>
                <div class="space-y-1">
                  {#each well.storage_info.log_files.slice(0, 3) as file}
                    <div class="text-xs text-gray-600 flex justify-between">
                      <span>{file.name}</span>
                      <span>{formatBytes(file.size)}</span>
                    </div>
                  {/each}
                  {#if well.storage_info.log_files.length > 3}
                    <div class="text-xs text-gray-500">
                      +{well.storage_info.log_files.length - 3} more files
                    </div>
                  {/if}
                </div>
              </div>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
```

## 🎯 Benefits of This Integration

### **1. Unified Data View**

- Wells now show both database metadata AND storage information
- File counts, sizes, and types are visible
- Project-based filtering works with storage data

### **2. Performance Benefits**

- OpenDAL caching for storage operations
- Single API call for combined data
- Efficient file listing with OpenDAL

### **3. Enhanced User Experience**

- Users can see storage usage per well
- File organization is visible
- Better project management capabilities

### **4. Maintains Existing Functionality**

- All existing PostgREST operations still work
- Backward compatibility maintained
- Gradual migration possible

## 🚀 Next Steps

1. **Implement the unified service** (Step 1)
2. **Add storage operations to OpenDAL** (Step 2)
3. **Add new Tauri commands** (Step 3)
4. **Update frontend service** (Step 4)
5. **Enhance Svelte components** (Step 5)
6. **Test the integration**
7. **Add more storage-aware operations** (project stats, etc.)

This integration provides a solid foundation for combining your existing PostgREST database operations with the OpenDAL storage layer, creating a more powerful and user-friendly data management system.
