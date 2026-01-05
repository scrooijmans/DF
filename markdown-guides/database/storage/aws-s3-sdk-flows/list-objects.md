# MinIO File Listing with AWS SDK S3

This guide explains how we list files from MinIO (S3-compatible storage) using the AWS SDK S3 for Rust, bypassing Supabase Storage's metadata system entirely.

## Architecture Overview

### Third-Party SDKs Used

1. **AWS SDK S3 for Rust** (`aws-sdk-s3 = "1.107.0"`)
   - Official AWS SDK for S3 operations
   - Provides full S3 API compatibility
   - Handles authentication, pagination, and error handling

2. **AWS Config** (`aws-config = "1.8.7"`)
   - Configuration management for AWS services
   - Handles credential providers and region settings
   - Provides shared configuration across AWS services

### MinIO Communication

**Direct S3 Protocol Communication**: We configure the AWS SDK S3 to communicate directly with MinIO using the S3 protocol:

```rust
// MinIO endpoint configuration
let endpoint = "http://91.99.166.223:9000";
let region = "us-east-1";

// AWS SDK S3 client configured for MinIO
let shared_config = aws_config::from_env()
    .credentials_provider(Credentials::new(
        access_key,
        secret_key,
        None,
        None,
        "static",
    ))
    .endpoint_url(endpoint)  // Custom MinIO endpoint
    .region(region_provider)
    .load()
    .await;

// S3 client with path-style addressing for MinIO
let s3_config = s3::config::Builder::from(&shared_config)
    .force_path_style(true)  // Required for MinIO
    .build();
```

### Supabase Storage Bypass

**Complete Bypass**: We are completely bypassing Supabase Storage for file operations:

- ✅ **Direct MinIO Access**: Files are listed directly from MinIO using S3 API
- ❌ **No Supabase Storage API**: We don't use Supabase's `/object/list/{bucket}` endpoint
- ❌ **No Metadata Database**: We don't query Supabase's `storage.objects` table
- ❌ **No RLS Policies**: We bypass Supabase's Row Level Security entirely

## Step-by-Step Implementation Process

### Step 1: Frontend Request

**Component**: `content-data-analytics-files.svelte`
**Third-Party APIs**: None (Pure SvelteKit)

```typescript
// User clicks on a bucket or refreshes file list
const loadParquetFiles = async (bucketName: string) => {
  console.log(
    `🔄 [Component] Loading Parquet files from MinIO bucket: ${bucketName}...`,
  );
  isLoading = true;
  error = null;

  try {
    console.log("🔧 [Component] Calling Tauri command: get_minio_files");
    const fetchedFiles = await invoke<ParquetFile[]>("get_minio_files", {
      bucketName,
    });
    console.log(
      "✅ [Component] Successfully fetched MinIO files:",
      fetchedFiles,
    );

    parquetFiles = fetchedFiles || [];
    console.log(
      `✅ [Component] Loaded ${parquetFiles.length} Parquet files from MinIO bucket ${bucketName}`,
    );
    isLoading = false;
  } catch (err) {
    console.error(
      `❌ [Component] Failed to load Parquet files from MinIO bucket ${bucketName}:`,
      err,
    );
    error = err instanceof Error ? err.message : "Failed to load files";
    isLoading = false;
  }
};
```

### Step 2: Tauri Command Bridge

**Component**: `content-data-analytics-files.svelte`
**Third-Party APIs**: Tauri IPC (Rust ↔ TypeScript)

```typescript
import { invoke } from "@tauri-apps/api/core";
import type {
  StorageBucket,
  ParquetFile,
} from "$lib/tauri-commands/analytics-types";

// Direct Tauri command calls
const fetchedFiles = await invoke<ParquetFile[]>("get_minio_files", {
  bucketName,
});
const fetchedBuckets = await invoke<StorageBucket[]>("get_minio_buckets");
```

### Step 3: Rust Backend Processing

**Component**: `src-tauri/src/main.rs`
**Third-Party APIs**: Tauri Commands

```rust
#[tauri::command]
async fn get_minio_files(bucket_name: String) -> Result<Vec<crate::analytics_query::types::ParquetFile>, String> {
    let minio_manager = MinioFilesManager::new().await
        .map_err(|e| format!("Failed to initialize MinIO client: {}", e))?;

    let response = minio_manager.list_files(&bucket_name).await
        .map_err(|e| format!("Failed to list MinIO files: {}", e))?;

    if response.success {
        // Convert MinioFile to ParquetFile
        let parquet_files: Vec<crate::analytics_query::types::ParquetFile> = response.files
            .into_iter()
            .map(|file| {
                let name = file.name.clone();
                let last_modified = file.last_modified.clone();

                crate::analytics_query::types::ParquetFile {
                    id: file.id,
                    name: file.name,
                    bucket_id: bucket_name.clone(),
                    owner: "".to_string(),
                    path_tokens: if name.contains('/') {
                        name.split('/').map(|s| s.to_string()).collect()
                    } else {
                        vec![name]
                    },
                    metadata: crate::analytics_query::types::FileMetadata {
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
                }
            })
            .collect();

        Ok(parquet_files)
    } else {
        Err(response.message)
    }
}
```

### Step 4: AWS SDK S3 Client Initialization

**Component**: `src-tauri/src/analytics_query/minio_files.rs`
**Third-Party APIs**:

- `aws-config = "1.8.7"` - Configuration management
- `aws-sdk-s3 = "1.107.0"` - S3 operations

```rust
impl MinioFilesManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Step 4a: Load credentials from environment
        let access_key = env::var("AWS_ACCESS_KEY_ID")?;
        let secret_key = env::var("AWS_SECRET_ACCESS_KEY")?;

        // Step 4b: Configure AWS Config for MinIO
        let shared_config = aws_config::from_env()
            .credentials_provider(Credentials::new(access_key, secret_key, None, None, "static"))
            .endpoint_url("http://91.99.166.223:9000")  // MinIO endpoint
            .region(region_provider)
            .load()
            .await;

        // Step 4c: Create S3 client with MinIO-specific settings
        let s3_config = s3::config::Builder::from(&shared_config)
            .force_path_style(true)  // Required for MinIO
            .build();

        let s3_client = Client::from_conf(s3_config);
        Ok(MinioFilesManager { s3_client })
    }
}
```

### Step 5: MinIO S3 API Communication

**Component**: `src-tauri/src/analytics_query/minio_files.rs`
**Third-Party APIs**:

- **MinIO S3-Compatible API** - Direct HTTP communication
- **AWS SDK S3** - Handles S3 protocol, authentication, and response parsing

```rust
pub async fn list_files(&self, bucket_name: &str) -> Result<MinioFilesResponse, Box<dyn std::error::Error + Send + Sync>> {
    // Step 5a: Use AWS SDK S3 to call MinIO's S3-compatible API
    let mut continuation_token: Option<String> = None;
    let mut all_files = Vec::new();

    loop {
        // Step 5b: Build S3 list_objects_v2 request
        let mut request = self.s3_client
            .list_objects_v2()
            .bucket(bucket_name)
            .prefix("")
            .max_keys(1000);

        if let Some(token) = &continuation_token {
            request = request.continuation_token(token);
        }

        // Step 5c: Send HTTP request to MinIO
        let response = request.send().await?;

        // Step 5d: Parse S3 response and extract file metadata
        if let Some(contents) = response.contents() {
            for obj in contents {
                let file = MinioFile {
                    name: obj.key().unwrap_or_default().to_string(),
                    size: obj.size().unwrap_or(0) as u64,
                    last_modified: obj.last_modified().unwrap_or_default().to_string(),
                    content_type: "application/octet-stream".to_string(),
                    etag: obj.e_tag().unwrap_or_default().to_string(),
                    id: obj.key().unwrap_or_default().to_string(),
                };
                all_files.push(file);
            }
        }

        // Step 5e: Handle pagination
        if response.is_truncated().unwrap_or(false) {
            continuation_token = response.next_continuation_token().map(|s| s.to_string());
        } else {
            break;
        }
    }

    Ok(MinioFilesResponse { files: all_files })
}
```

### Step 6: Response Processing

**Component**: `src-tauri/src/analytics_query/minio_files.rs`
**Third-Party APIs**: None (Pure Rust data processing)

```rust
// Convert AWS SDK S3 response to our internal MinioFile format
// Filter for .parquet files only
// Sort by name or date
// Return structured response
```

### Step 7: Frontend State Update

**Component**: `content-data-analytics-files.svelte`
**Third-Party APIs**: None (Pure SvelteKit state management)

```typescript
// Update component state with file list
parquetFiles = fetchedFiles || [];
console.log(
  `✅ [Component] Loaded ${parquetFiles.length} Parquet files from MinIO bucket ${bucketName}`,
);

// Trigger UI re-render
// Display files in two-pane layout with bucket selection
```

### File Metadata Retrieved

- **File Name**: Object key (full path including subfolders)
- **File Size**: Actual file size in bytes
- **Last Modified**: Timestamp of last modification
- **Content Type**: MIME type (application/octet-stream for Parquet)
- **ETag**: Unique identifier for the object
- **ID**: Same as file name for compatibility

### Pagination Support

The implementation uses `list_objects_v2` with pagination to handle large numbers of files:

```rust
let mut continuation_token: Option<String> = None;

loop {
    let mut request = self.s3_client
        .list_objects_v2()
        .bucket(bucket_name)
        .prefix("")
        .max_keys(1000);

    if let Some(token) = &continuation_token {
        request = request.continuation_token(token);
    }

    let response = request.send().await?;

    // Process files...

    if response.is_truncated().unwrap_or(false) {
        continuation_token = response.next_continuation_token().map(|s| s.to_string());
    } else {
        break;
    }
}
```

## Advantages of This Approach

### 1. **Full S3 Compatibility**

- Access to all S3 features and metadata
- Standard S3 API operations
- Future-proof with AWS ecosystem

### 2. **Rich Metadata**

- File sizes, timestamps, ETags
- No need for separate metadata queries
- Consistent with S3 standards

### 3. **Performance**

- Direct communication with MinIO
- No Supabase Storage API overhead
- Efficient pagination for large datasets

### 4. **Reliability**

- Official AWS SDK with proper error handling
- No dependency on Supabase Storage configuration
- Works even if Supabase Storage is down

## Configuration Requirements

### Environment Variables

```bash
# MinIO Configuration
AWS_ACCESS_KEY_ID=mudrock-storage
AWS_SECRET_ACCESS_KEY=mudrock-storage-secret-2024
MINIO_ENDPOINT=http://91.99.166.223:9000
MINIO_REGION=us-east-1
```

### MinIO Server Requirements

- S3-compatible API enabled
- Path-style addressing support
- Proper CORS configuration for web access

## Frontend Integration

### Global State Management

```typescript
// content-data-analytics-files.svelte
let buckets: StorageBucket[] = $state([]);
let selectedBucket: StorageBucket | null = $state(null);
let parquetFiles: ParquetFile[] = $state([]);
let isLoading = $state(false);
let error = $state<string | null>(null);
```

### Tauri Commands

```rust
#[tauri::command]
async fn get_minio_buckets() -> Result<Vec<StorageBucket>, String> {
    let minio_manager = MinioFilesManager::new().await?;
    let response = minio_manager.list_buckets().await?;

    // Convert MinioBucket to StorageBucket
    let storage_buckets: Vec<StorageBucket> = response.buckets
        .into_iter()
        .map(|bucket| {
            let created_at = bucket.created_at.clone();
            StorageBucket {
                id: bucket.id,
                name: bucket.name,
                owner: "".to_string(),
                public: bucket.public,
                file_size_limit: None,
                allowed_mime_types: None,
                created_at: bucket.created_at,
                updated_at: created_at,
            }
        })
        .collect();

    Ok(storage_buckets)
}

#[tauri::command]
async fn get_minio_files(bucket_name: String) -> Result<Vec<ParquetFile>, String> {
    let minio_manager = MinioFilesManager::new().await?;
    let response = minio_manager.list_files(&bucket_name).await?;

    // Convert MinioFile to ParquetFile
    let parquet_files: Vec<ParquetFile> = response.files
        .into_iter()
        .map(|file| {
            let name = file.name.clone();
            let last_modified = file.last_modified.clone();

            ParquetFile {
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
            }
        })
        .collect();

    Ok(parquet_files)
}
```

## Comparison with Supabase Storage

| Feature            | Supabase Storage        | Direct MinIO (AWS SDK) |
| ------------------ | ----------------------- | ---------------------- |
| **File Listing**   | `/object/list/{bucket}` | `list_objects_v2()`    |
| **Metadata**       | Database queries        | S3 API response        |
| **Authentication** | JWT tokens              | AWS credentials        |
| **Pagination**     | Custom implementation   | Built-in S3 pagination |
| **Error Handling** | Supabase-specific       | AWS SDK standard       |
| **Performance**    | Database dependent      | Direct storage access  |
| **Reliability**    | Depends on Supabase     | Independent            |

## Troubleshooting

### Common Issues

1. **"Connection refused"**
   - Check MinIO endpoint URL
   - Verify MinIO service is running

2. **"Access denied"**
   - Verify AWS credentials
   - Check MinIO bucket permissions

3. **"Invalid signature"**
   - Ensure correct AWS credentials
   - Check MinIO S3 compatibility

### Debugging

```rust
// Enable debug logging
println!("🌐 MinIO endpoint: {}", endpoint);
println!("🔑 Using access key: {}", access_key);
println!("✅ Found {} Parquet files via AWS SDK S3", files.len());
```

## Future Considerations

### Potential Improvements

1. **Caching**: Implement client-side caching for file lists
2. **Filtering**: Add server-side filtering by file type, date, etc.
3. **Search**: Implement file name search functionality
4. **Sorting**: Add sorting options (name, size, date)

### Migration Path

If we ever need to switch back to Supabase Storage:

1. Update Tauri commands to use Supabase Storage API
2. Modify frontend to handle different data structures
3. Implement metadata synchronization

## Third-Party APIs Summary

### Complete API Stack Used

| Step  | Component       | Third-Party APIs         | Purpose                              |
| ----- | --------------- | ------------------------ | ------------------------------------ |
| **1** | Frontend UI     | None (Pure SvelteKit)    | User interface and state management  |
| **2** | Tauri Bridge    | Tauri IPC                | Rust ↔ TypeScript communication     |
| **3** | Rust Backend    | Tauri Commands           | Command registration and routing     |
| **4** | AWS Config      | `aws-config = "1.8.7"`   | AWS service configuration management |
| **4** | AWS SDK S3      | `aws-sdk-s3 = "1.107.0"` | S3 protocol implementation           |
| **5** | MinIO Storage   | MinIO S3-Compatible API  | Object storage via HTTP              |
| **6** | Data Processing | None (Pure Rust)         | Response parsing and formatting      |
| **7** | Frontend Update | None (Pure SvelteKit)    | UI state updates and rendering       |

### Key Third-Party Dependencies

1. **AWS SDK S3 for Rust** (`aws-sdk-s3 = "1.107.0"`)
   - **Purpose**: S3 protocol implementation and HTTP communication
   - **Used For**: `list_objects_v2()`, authentication, pagination, error handling
   - **Why**: Industry standard, full S3 compatibility, robust error handling

2. **AWS Config** (`aws-config = "1.8.7"`)
   - **Purpose**: AWS service configuration and credential management
   - **Used For**: Credential providers, region settings, endpoint configuration
   - **Why**: Centralized AWS configuration, credential chain support

3. **MinIO S3-Compatible API**
   - **Purpose**: Object storage backend
   - **Used For**: File storage, metadata storage, HTTP API endpoints
   - **Why**: S3-compatible, self-hosted, cost-effective

4. **Tauri IPC**
   - **Purpose**: Frontend-backend communication
   - **Used For**: TypeScript ↔ Rust function calls
   - **Why**: Secure, type-safe, efficient

### Data Flow Architecture

```
Frontend (SvelteKit)
    ↓ Tauri IPC
Rust Backend (Tauri Commands)
    ↓ AWS SDK S3
MinIO Storage (S3-Compatible API)
    ↓ HTTP Response
Rust Backend (Data Processing)
    ↓ Tauri IPC
Frontend (UI Update)
```

## Conclusion

This implementation provides a robust, performant, and reliable way to list files from MinIO storage using the industry-standard AWS SDK S3. By bypassing Supabase Storage entirely, we gain better performance, richer metadata, and independence from Supabase's storage limitations.

**Key Benefits:**

- **Consistency**: Same AWS SDK S3 for both listing and uploading
- **Performance**: Direct MinIO communication, no database overhead
- **Reliability**: Official AWS SDK with proper error handling
- **Independence**: No dependency on Supabase Storage configuration
- **Future-proof**: Easy to migrate to AWS S3 or other S3-compatible storage
