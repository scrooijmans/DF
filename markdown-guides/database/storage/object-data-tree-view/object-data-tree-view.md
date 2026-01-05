# Object Data Tree View - End-to-End Flow

This document describes the complete end-to-end flow of how the object data tree is called on the frontend, fetched from the backend, and presented in the UI component.

## Architecture Overview

The object data tree view consists of four main components:
- **`projects-objects-data-tree-view.svelte`** - Main container component
- **`folder-item.svelte`** - Individual folder/bucket component
- **`file-item.svelte`** - Individual file component with selection state
- **`selected-files.svelte`** - Summary component for selected files

## Data Tree Generation

The data tree is generated through a hierarchical structure that combines MinIO buckets and files:

### 1. Tree Structure
```
Root Level
├── Bucket 1 (project-xxx)
│   ├── file1.parquet
│   ├── file2.parquet
│   └── file3.parquet
├── Bucket 2 (project-yyy)
│   ├── file4.parquet
│   └── file5.parquet
└── Bucket 3 (project-zzz)
    └── (empty - 0 files)
```

### 2. Data Flow for Tree Generation

1. **Initial Load**: `GlobalProjectObjects.loadTreeData()` fetches all buckets
2. **Preload Counts**: `preloadFileCounts()` gets file counts for each bucket without loading files
3. **Lazy Loading**: Files are only loaded when a bucket is expanded via `loadFilesForBucket()`
4. **Tree Building**: Buckets and files are converted to `TreeNode` objects with hierarchical structure

### 3. Tree Node Structure
Each item in the tree is represented as a `TreeNode`:
- **Buckets**: `type: "bucket"`, contain `children` array of files
- **Files**: `type: "file"`, have `parent` reference to bucket ID
- **State**: Each node tracks `expanded`, `selected`, and `data` properties

## Data Flow

### 1. Frontend Initialization

**Location**: `src/routes/home/+layout.svelte`
```svelte
import { setGlobalProjectObjects } from '$lib/components/pages/home/content-main/projects-objects/project-objects.svelte.ts';

// Initialize the global state for project objects
setGlobalProjectObjects();
```

### 2. Global State Management

**Location**: `src/lib/components/pages/home/content-main/projects-objects/project-objects.svelte.ts`

The `GlobalProjectObjects` class manages the entire tree state using Svelte 5 runes:

```typescript
export class GlobalProjectObjects {
  // Tree data with reactive state
  treeData = $state<TreeNode[]>([]);
  
  // Selected files
  selectedFiles = $state<ParquetFile[]>([]);
  
  // Loading states
  isLoadingBuckets = $state(false);
  isLoadingFiles = $state<Set<string>>(new Set());
  
  // Error state
  error = $state<string | null>(null);
  
  // Search query
  searchQuery = $state("");

  constructor() {
    // Initialize tree data on construction
    this.loadTreeData();
  }
}
```

### 3. Backend Data Fetching

**Location**: `src-tauri/src/main.rs`

Two main Tauri commands handle data fetching:

#### A. Fetch Buckets
```rust
#[tauri::command]
async fn get_minio_buckets() -> Result<Vec<StorageBucket>, String> {
    let minio_manager = MinioFilesManager::new()
        .await
        .map_err(|e| format!("Failed to initialize MinIO client: {}", e))?;

    let response = minio_manager
        .list_buckets()
        .await
        .map_err(|e| format!("Failed to list MinIO buckets: {}", e))?;

    // Convert MinioBucket to StorageBucket
    let storage_buckets: Vec<StorageBucket> = response
        .buckets
        .into_iter()
        .map(|bucket| StorageBucket {
            id: bucket.id,
            name: bucket.name,
            owner: "".to_string(),
            public: bucket.public,
            file_size_limit: None,
            allowed_mime_types: None,
            created_at: bucket.created_at,
            updated_at: bucket.created_at,
        })
        .collect();

    Ok(storage_buckets)
}
```

#### B. Fetch Files for a Bucket
```rust
#[tauri::command]
async fn get_minio_files(bucket_name: String) -> Result<Vec<ParquetFile>, String> {
    let minio_manager = MinioFilesManager::new()
        .await
        .map_err(|e| format!("Failed to initialize MinIO client: {}", e))?;

    let response = minio_manager
        .list_files(&bucket_name)
        .await
        .map_err(|e| format!("Failed to list MinIO files: {}", e))?;

    // Convert MinioFile to ParquetFile
    let parquet_files: Vec<ParquetFile> = response
        .files
        .into_iter()
        .map(|file| ParquetFile {
            id: file.id,
            name: file.name,
            bucket_id: bucket_name.clone(),
            owner: "".to_string(),
            path_tokens: if name.contains('/') {
                name.split('/').map(|s| s.to_string()).collect()
            } else {
                vec![name]
            },
            metadata: FileMetadata {
                e_tag: file.etag,
                size: file.size as i64,
                mimetype: file.content_type,
                cache_control: "".to_string(),
                last_modified: file.last_modified,
                content_length: file.size as i64,
            },
            created_at: last_modified.clone(),
            updated_at: last_modified.clone(),
            last_accessed_at: last_modified,
        })
        .collect();

    Ok(parquet_files)
}
```

### 4. MinIO Integration

**Location**: `src-tauri/src/analytics_query/minio_files.rs`

The `MinioFilesManager` handles direct communication with MinIO using AWS SDK S3:

```rust
pub struct MinioFilesManager {
    s3_client: Client,
}

impl MinioFilesManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Configure AWS S3 client for MinIO
        let access_key = env::var("AWS_ACCESS_KEY_ID").unwrap_or_else(|_| "mudrock-storage".to_string());
        let secret_key = env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_else(|_| "mudrock-storage-secret-2024".to_string());
        let endpoint = "http://91.99.166.223:9000";
        
        // Create S3 client with custom endpoint for MinIO
        let s3_config = s3::config::Builder::from(&shared_config)
            .force_path_style(true)
            .build();
            
        Ok(MinioFilesManager { s3_client: Client::from_conf(s3_config) })
    }
}
```

### 5. Frontend Data Loading

**Location**: `src/lib/components/pages/home/content-main/projects-objects/project-objects.svelte.ts`

The global state class handles data loading:

```typescript
async loadTreeData() {
  this.isLoadingBuckets = true;
  this.error = null;
  
  try {
    // Load buckets via Tauri command
    const buckets = await invoke<StorageBucket[]>("get_minio_buckets");
    
    // Convert buckets to tree nodes
    this.treeData = buckets.map((bucket) => ({
      id: bucket.id,
      name: bucket.name,
      type: "bucket" as const,
      children: [],
      data: bucket,
      expanded: false,
      selected: false,
    }));
    
    // Preload file counts for all buckets
    await this.preloadFileCounts();
  } catch (err) {
    this.error = err instanceof Error ? err.message : "Failed to load tree data";
  } finally {
    this.isLoadingBuckets = false;
  }
}

async loadFilesForBucket(bucketId: string) {
  const bucketNode = this.findNodeById(bucketId);
  if (!bucketNode || bucketNode.type !== "bucket") return;

  this.isLoadingFiles.add(bucketId);
  
  try {
    // Load files via Tauri command
    const files = await invoke<ParquetFile[]>("get_minio_files", {
      bucketName: bucketNode.name,
    });
    
    // Convert files to tree nodes
    const fileNodes: TreeNode[] = files.map((file) => ({
      id: file.id,
      name: file.name,
      type: "file" as const,
      parent: bucketId,
      children: [],
      data: file,
      expanded: false,
      selected: false,
    }));
    
    // Update the bucket node with files
    bucketNode.children = fileNodes;
  } catch (err) {
    this.error = err instanceof Error ? err.message : "Failed to load files";
  } finally {
    this.isLoadingFiles.delete(bucketId);
  }
}
```

### 6. UI Component Rendering

**Location**: `src/lib/components/pages/home/content-main/projects-objects-data-tree-view/projects-objects-data-tree-view.svelte`

The main component renders the tree structure:

```svelte
<div class="space-y-0 text-left">
  {#each globalState.getFilteredTreeData() as bucket (bucket.id)}
    <!-- Folder Item -->
    <FolderItem {bucket} />

    <!-- Files List (indented) -->
    {#if bucket.expanded && bucket.children.length > 0}
      <div class="ml-12 border-l border-gray-200 pl-2">
        {#each bucket.children as file (file.id)}
          <FileItem {file} />
        {/each}
      </div>
    {/if}
  {/each}
</div>
```

### 7. Individual Component Behavior

#### Folder Item (`folder-item.svelte`)
- Displays chevron (right/down) and folder icon (closed/open)
- Shows bucket name and file count on single line
- Handles expansion/collapse via `globalState.toggleNodeExpansion()`
- Triggers file loading when expanded for the first time

#### File Item (`file-item.svelte`)
- Displays file icon and name with metadata on single line
- **Bold text when selected** using Svelte 5 class binding:
  ```svelte
  <div class="text-sm text-gray-900 truncate mr-2" class:font-bold={file.selected}>
    {file.name}
  </div>
  <div class="text-xs text-gray-500 flex-shrink-0" class:font-bold={file.selected}>
    {formatFileSize(file.data.metadata.size)} • {formatDate(file.data.metadata.lastModified)}
  </div>
  ```
- Handles selection via `globalState.toggleFileSelection()`
- Shows selection indicator (blue dot) when selected

## Data Types

### StorageBucket
```typescript
export interface StorageBucket {
  id: string;
  name: string;
  owner: string;
  public: boolean;
  file_size_limit?: number;
  allowed_mime_types?: string[];
  created_at: string;
  updated_at: string;
  file_count?: number; // Added for preloading
}
```

### ParquetFile
```typescript
export interface ParquetFile {
  id: string;
  name: string;
  bucket_id: string;
  owner: string;
  path_tokens: string[];
  metadata: {
    eTag: string;
    size: number;
    mimetype: string;
    cacheControl: string;
    lastModified: string;
    contentLength: number;
  };
  created_at: string;
  updated_at: string;
  last_accessed_at: string;
}
```

### TreeNode
```typescript
export interface TreeNode {
  id: string;
  name: string;
  type: "bucket" | "file";
  parent?: string;
  children: TreeNode[];
  data?: StorageBucket | ParquetFile;
  expanded: boolean;
  selected: boolean;
}
```

## UI Component Placement

### Tree View Location
The data tree view is rendered in the **left sidebar** of the notebook editor, providing easy access to data sources while working on notebook cells.

### Selected Files Display
- **Primary Location**: The `selected-files.svelte` component is displayed at the bottom of the tree view in the left sidebar
- **Removed Duplication**: Previously, there was a duplicate "Available Data Sources" section in each SQL notebook cell, which has been removed to avoid redundancy
- **Single Source of Truth**: All file selection state is managed globally through `GlobalProjectObjects`, ensuring consistency across the application

### Component Hierarchy
```
Notebook Editor
├── Left Sidebar
│   ├── projects-objects-data-tree-view.svelte
│   │   ├── folder-item.svelte (for each bucket)
│   │   ├── file-item.svelte (for each file)
│   │   └── selected-files.svelte (summary)
│   └── (other sidebar components)
└── Main Content
    └── jupyter-notebook-editor.svelte
        └── notebook-cell.svelte (SQL/Markdown/Python cells)
```

## Key Features

1. **Lazy Loading**: Files are only loaded when a bucket is expanded
2. **Preloading**: File counts are preloaded for all buckets on initial load
3. **Search**: Real-time filtering of buckets and files
4. **Multi-selection**: Multiple files can be selected with visual indicators
5. **Reactive State**: Uses Svelte 5 runes for reactive state management
6. **Bold Selection**: Selected files display in bold text
7. **Step-ladder Indentation**: Files are visually indented with a border line
8. **Single-line Layout**: Both folders and files display on single lines with right-aligned metadata
9. **No Duplication**: Single selected files display prevents UI clutter and confusion

## Error Handling

- Network errors are caught and displayed in the UI
- Loading states are managed per bucket
- Graceful fallbacks for missing data
- Console logging for debugging

## Performance Optimizations

- Lazy loading of file data
- Preloading of file counts
- Efficient tree traversal with `findNodeById()`
- Minimal re-renders using Svelte 5 runes
- Truncation of long file names
