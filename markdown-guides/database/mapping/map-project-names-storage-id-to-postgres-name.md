# Project Name Mapping: Storage ID to PostgreSQL Name

## Overview

This document describes the end-to-end flow of how storage bucket names (format: `project-{uuid}`) are mapped to user-friendly project names from the Supabase PostgreSQL database and displayed in the hierarchical data tree view.

## Architecture

The solution uses a **unified mapping system** implemented in Rust crates that provides a modular, extensible approach to storage-to-database mapping. The system follows the design principles outlined in the mapper plan with intelligent caching, batch operations, and seamless integration with the existing storage layer.

## End-to-End Flow

### 1. Data Tree Initialization

**File**: `src/lib/components/pages/home/content-main/projects-objects/project-objects.svelte.ts`

```typescript
// Load all tree data using unified mapping system
async loadTreeData() {
    // 1. Use unified mapping system to get buckets with mapped names
    const mappedBuckets = await invoke<StorageBucketWithMappedName[]>(
        "get_minio_buckets_with_unified_mapping"
    );

    // 2. Convert to tree nodes with mapped names
    this.treeData = mappedBuckets.map((mappedBucket) => {
        return {
            id: mappedBucket.bucket_name, // Original bucket name as ID
            name: mappedBucket.project_name, // User-friendly name for display
            type: "bucket" as const,
            children: [],
            data: {
                name: mappedBucket.bucket_name,
                file_count: mappedBucket.file_count
            },
            expanded: false,
            selected: false,
            level: 0,
            path: mappedBucket.bucket_name, // Keep original for API calls
        };
    });
}
```

### 2. Unified Mapping Service (Backend)

**File**: `crates/core/unified-mapping-service/src/lib.rs`

The unified mapping service orchestrates the mapping process using the modular architecture:

```rust
pub struct UnifiedMappingService {
    cache: Arc<MappingCache>,
    postgres_adapter: Arc<DataSourceAdapterEnum>,
    minio_adapter: Arc<DataSourceAdapterEnum>,
    project_mapper: ProjectMapper,
    config: UnifiedMappingConfig,
}

impl UnifiedMappingService {
    /// Map storage buckets to project names using the unified system
    pub async fn map_storage_buckets_to_projects(
        &self,
        bucket_names: Vec<String>,
    ) -> Result<Vec<StorageBucketWithMappedName>, MappingError> {
        // 1. Extract project IDs from bucket names
        let project_ids: Vec<String> = bucket_names
            .iter()
            .filter_map(|bucket_name| {
                if bucket_name.starts_with("project-") {
                    if let Some(project_id) = bucket_name.strip_prefix("project-") {
                        // Only include valid UUIDs
                        if project_id.len() == 36 && project_id.contains('-') {
                            Some(project_id.to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // 2. Fetch project information from database
        let project_infos = self.fetch_projects_from_database(&project_ids).await?;

        // 3. Create mapping from project ID to project info
        let project_map: HashMap<String, ProjectInfo> = project_infos
            .into_iter()
            .map(|project| (project.id.clone(), project))
            .collect();

        // 4. Map buckets to projects - only include projects that exist in database
        let mut results = Vec::new();
        for bucket_name in bucket_names {
            if bucket_name.starts_with("project-") {
                if let Some(project_id) = bucket_name.strip_prefix("project-") {
                    if project_id.len() == 36 && project_id.contains('-') {
                        if let Some(project_info) = project_map.get(project_id) {
                            results.push(StorageBucketWithMappedName {
                                bucket_name: bucket_name.clone(),
                                project_id: project_info.id.clone(),
                                project_name: project_info.name.clone(),
                                project_description: project_info.description.clone(),
                                file_count: None,
                            });
                        }
                    }
                }
            }
        }

        Ok(results)
    }
}
```

### 3. Tauri Command Integration

**File**: `src-tauri/src/main.rs`

The Tauri command integrates the unified mapping service with the frontend:

```rust
#[tauri::command]
async fn get_minio_buckets_with_unified_mapping() -> Result<Vec<StorageBucketWithMappedName>, String> {
    // 1. Get all MinIO buckets
    let buckets = get_minio_buckets().await?;
    let bucket_names: Vec<String> = buckets.iter().map(|b| b.name.clone()).collect();

    // 2. Initialize unified mapping service
    let mapping_service = create_unified_mapping_service().await?;

    // 3. Map bucket names to project names
    let mapped_buckets = mapping_service
        .map_storage_buckets_to_projects(bucket_names)
        .await
        .map_err(|e| format!("Failed to map bucket names: {}", e))?;

    Ok(mapped_buckets)
}
```

### 4. Core Mapping Infrastructure

The system is built on the modular architecture defined in the mapper plan:

#### **`mapping-core`** - Unified Mapping Interface

- Defines core traits: `MappingService`, `Mapper<T, U>`
- Provides unified API for all mapping operations
- Handles error types and result structures

#### **`mapping-cache`** - Intelligent Caching System

- Multi-tier caching with TTL and invalidation
- Memory cache (L1) and persistent cache (L2) support
- Background refresh strategies

#### **`mapping-adapters`** - Database & Storage Adapters

- `PostgresAdapter` for PostgreSQL queries via PostgREST
- `MinIOAdapter` for storage metadata operations
- Unified `DataSourceAdapter` trait

#### **`project-mapper`** - Project-Specific Mapping

- Maps `StorageBucketId` to `ProjectInfo`
- Handles project-specific validation and caching
- Integrates with PostgreSQL via PostgREST

## Data Flow Diagram

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Data Tree     │    │ Unified Mapping  │    │   Database      │
│   Component     │    │    Service       │    │   (PostgreSQL)  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                        │                        │
         │ 1. loadTreeData()      │                        │
         ├────────────────────────►                        │
         │                        │                        │
         │ 2. get_minio_buckets   │                        │
         │    _with_unified_      │                        │
         │    _mapping()          │                        │
         ├────────────────────────►                        │
         │                        │                        │
         │                        │ 3. Extract project IDs │
         │                        │    from bucket names   │
         │                        │                        │
         │                        │ 4. fetch_projects_     │
         │                        │    from_database()     │
         │                        ├────────────────────────►
         │                        │                        │
         │                        │ 5. PostgREST query     │
         │                        │    with UUID filter    │
         │                        ├────────────────────────►
         │                        │                        │
         │                        │ 6. Return project info │
         │                        ◄────────────────────────┤
         │                        │                        │
         │                        │ 7. Map buckets to      │
         │                        │    projects & filter   │
         │                        │                        │
         │ 8. Return mapped       │                        │
         │    buckets with names  │                        │
         ◄────────────────────────┤                        │
         │                        │                        │
         │ 9. Render with         │                        │
         │    real project names  │                        │
         └────────────────────────┘                        │
```

## Key Components

### 1. Storage Bucket Format

- **Format**: `project-{uuid}`
- **Example**: `project-3bd95caa-6360-4d91-8d40-5ef6c659c93a`
- **Purpose**: Unique identifier for MinIO storage buckets

### 2. Project Information Structure

```typescript
interface ProjectInfo {
  id: string; // UUID from projects table
  name: string; // User-friendly project name
  description?: string; // Optional project description
  created_at: string; // Creation timestamp
  updated_at: string; // Last update timestamp
}
```

### 3. Mapped Bucket Information

```typescript
interface StorageBucketWithMappedName {
  bucket_name: string; // Original bucket name
  project_id: string; // Extracted project ID
  project_name: string; // User-friendly name
  project_description?: string; // Project description
  file_count?: number; // Number of files in bucket
}
```

## Caching Strategy

- **Multi-tier Caching**: Memory cache (L1) and persistent cache (L2)
- **Cache Key**: Project ID (UUID) with mapper type prefix
- **Cache Lifetime**: Configurable TTL with intelligent refresh strategies
- **Cache Benefits**: Reduces database queries, supports background refresh
- **Cache Invalidation**: Automatic invalidation on data changes

## Error Handling

1. **Invalid Bucket Names**: Non-project buckets (e.g., `adjust`, `pets`) are filtered out
2. **Missing Projects**: Only projects that exist in the database are included
3. **Invalid UUIDs**: Non-UUID project IDs are filtered out before database queries
4. **Database Errors**: Graceful error handling with detailed logging
5. **Network Issues**: Cached data used when available, with fallback strategies

## Performance Optimizations

1. **Batch Processing**: All bucket names mapped in single operation
2. **Intelligent Caching**: Multi-tier caching with TTL and background refresh
3. **Database Filtering**: Only valid UUIDs are queried against the database
4. **Efficient Queries**: Single PostgREST query for all projects with UUID filtering
5. **Lazy Loading**: Files loaded only when bucket expanded
6. **Connection Pooling**: Reused database connections for better performance

## Security Considerations

1. **RLS Policies**: Projects filtered by `owner_id` in database query
2. **API Key**: PostgREST authentication via Supabase API key
3. **Input Validation**: Bucket name format validation before processing
4. **Error Sanitization**: Sensitive error details not exposed to frontend

## Future Improvements

1. **Additional Mappers**: Implement `curve-mapper` and `well-mapper` as planned
2. **Advanced Caching**: Add persistent cache support and cache warming
3. **Real-time Updates**: WebSocket integration for live mapping updates
4. **Performance Monitoring**: Add metrics and analytics for mapping operations
5. **Configuration UI**: Frontend interface for mapping system configuration

## Troubleshooting

### Common Issues

1. **"No project found"**: Check if project exists in database and RLS policies
2. **"Command get_projects not found"**: Ensure Tauri command is registered
3. **"PostgREST query failed"**: Verify API key and database connection
4. **"Failed to parse project ID"**: Check bucket name format

### Debug Logs

Enable detailed logging by checking console output for:

- `🔍 [UnifiedMapping] Bucket names to map:`
- `🔍 [UnifiedMapping] Project buckets found:`
- `🔍 [UnifiedMapping] Extracted valid UUID project IDs:`
- `🔍 [UnifiedMapping] Fetching projects for IDs:`
- `🔍 [UnifiedMapping] PostgREST response:`
- `✅ [UnifiedMapping] Including project from database:`
- `🔍 [UnifiedMapping] Skipping project not found in database:`
- `✅ [GlobalProjectObjects] Loaded buckets with unified mapping:`
