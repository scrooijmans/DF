# MinIO Bucket Listing with AWS SDK S3

This guide explains how we list storage buckets from MinIO (S3-compatible storage) using the AWS SDK S3 for Rust, following the pattern from `list-buckets.rs` example.

## Architecture Overview

### Third-Party SDKs Used

1. **AWS SDK S3 for Rust** (`aws-sdk-s3 = "1.107.0"`)
   - Official AWS SDK for S3 operations
   - Provides full S3 API compatibility
   - Handles authentication and bucket enumeration

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
let shared_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
    .credentials_provider(Credentials::new(
        access_key,
        secret_key,
        None,
        None,
        "static",
    ))
    .endpoint_url(endpoint)  // Custom MinIO endpoint
    .region(region_provider)
    .retry_config(aws_config::retry::RetryConfig::standard())
    .load()
    .await;

// S3 client with path-style addressing for MinIO
let s3_config = s3::config::Builder::from(&shared_config)
    .force_path_style(true)  // Required for MinIO
    .build();
```

### Supabase Storage Bypass

**Complete Bypass**: We are completely bypassing Supabase Storage for bucket operations:

- ✅ **Direct MinIO Access**: Buckets are listed directly from MinIO using S3 API
- ❌ **No Supabase Storage API**: We don't use Supabase's `/bucket` endpoint
- ❌ **No Metadata Database**: We don't query Supabase's `storage.buckets` table
- ❌ **No RLS Policies**: We bypass Supabase's Row Level Security entirely

## Step-by-Step Implementation Process

### Step 1: Frontend Request

**Component**: `content-data-analytics-files.svelte`
**Third-Party APIs**: None (Pure SvelteKit)

```typescript
// User navigates to Data Analytics page
const loadBuckets = async () => {
  const buckets = await getMinioBuckets();
  // Updates component state
};
```

### Step 2: Tauri Command Bridge

**Component**: `src/lib/components/pages/home/content-main/content-data-analytics/content-data-analytics-files/content-data-analytics-files.svelte`
**Third-Party APIs**: Tauri IPC (Rust ↔ TypeScript)

```typescript
import { invoke } from "@tauri-apps/api/core";

const fetchedBuckets = await invoke<StorageBucket[]>("get_minio_buckets");
```

### Step 3: Rust Backend Processing

**Component**: `src-tauri/src/main.rs`
**Third-Party APIs**: Tauri Commands

```rust
#[tauri::command]
async fn get_minio_buckets() -> Result<Vec<crate::analytics_query::types::StorageBucket>, String> {
    let minio_manager = MinioFilesManager::new().await
        .map_err(|e| format!("Failed to initialize MinIO client: {}", e))?;

    let response = minio_manager.list_buckets().await
        .map_err(|e| format!("Failed to list MinIO buckets: {}", e))?;

    if response.success {
        // Convert MinioBucket to StorageBucket
        let storage_buckets: Vec<crate::analytics_query::types::StorageBucket> = response.buckets
            .into_iter()
            .map(|bucket| {
                let created_at = bucket.created_at.clone();
                crate::analytics_query::types::StorageBucket {
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
        let access_key = env::var("AWS_ACCESS_KEY_ID")
            .unwrap_or_else(|_| "mudrock-storage".to_string());
        let secret_key = env::var("AWS_SECRET_ACCESS_KEY")
            .unwrap_or_else(|_| "mudrock-storage-secret-2024".to_string());
        let endpoint = "http://91.99.166.223:9000";
        let region = "us-east-1";

        // Step 4b: Configure AWS Config for MinIO
        let region_provider = RegionProviderChain::first_try(Region::new(region))
            .or_default_provider()
            .or_else(Region::new(region));

        let shared_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .credentials_provider(Credentials::new(
                access_key,
                secret_key,
                None,
                None,
                "static",
            ))
            .endpoint_url(endpoint)
            .region(region_provider)
            .retry_config(aws_config::retry::RetryConfig::standard())
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
pub async fn list_buckets(&self) -> Result<MinioBucketsResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!("🔍 Listing MinIO buckets using AWS SDK S3...");

    // Step 5a: Use AWS SDK S3 to call MinIO's S3-compatible API
    let response = self.s3_client
        .list_buckets()
        .send()
        .await?;

    let mut buckets = Vec::new();

    // Step 5b: Parse S3 response and extract bucket metadata
    for bucket in response.buckets() {
        let bucket_name = bucket.name().unwrap_or_default();
        let creation_date = bucket.creation_date()
            .map(|dt| dt.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        buckets.push(MinioBucket {
            id: bucket_name.to_string(),
            name: bucket_name.to_string(),
            public: false, // MinIO doesn't expose this in list_buckets
            created_at: creation_date,
        });
    }

    println!("✅ Found {} buckets via AWS SDK S3", buckets.len());
    Ok(MinioBucketsResponse {
        success: true,
        message: format!("Successfully listed {} buckets", buckets.len()),
        buckets,
    })
}
```

### Step 6: Type Conversion

**Component**: `src-tauri/src/main.rs`
**Third-Party APIs**: None (Pure Rust data processing)

```rust
// Convert MinioBucket to StorageBucket for frontend compatibility
let storage_buckets: Vec<crate::analytics_query::types::StorageBucket> = response.buckets
    .into_iter()
    .map(|bucket| {
        let created_at = bucket.created_at.clone();
        crate::analytics_query::types::StorageBucket {
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
```

### Step 7: Frontend State Update

**Component**: `content-data-analytics-files.svelte`
**Third-Party APIs**: None (Pure SvelteKit state management)

```typescript
// Update component state with bucket list
buckets = fetchedBuckets;

// Auto-select the first bucket if available
if (buckets.length > 0) {
  selectedBucket = buckets[0];
  await loadParquetFiles(selectedBucket.name);
}
```

## Bucket Metadata Retrieved

- **Bucket Name**: S3 bucket name
- **Bucket ID**: Same as bucket name for compatibility
- **Creation Date**: Timestamp when bucket was created
- **Public Access**: Currently set to false (MinIO doesn't expose this in list_buckets)
- **Owner**: Empty string (not available from MinIO list_buckets)

## Following list-buckets.rs Pattern

Our implementation follows the official AWS SDK S3 example pattern:

```rust
// From list-buckets.rs example
let mut buckets = client.list_buckets().into_paginator().send();
let mut num_buckets = 0;

while let Some(Ok(output)) = buckets.next().await {
    for bucket in output.buckets() {
        num_buckets += 1;
        let bucket_name = bucket.name().unwrap_or("unknown");
        let creation_date = bucket.creation_date()
            .map(|dt| dt.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        println!("Found bucket: {} (created: {})", bucket_name, creation_date);
    }
}
```

## Advantages of This Approach

### 1. **Full S3 Compatibility**

- Access to all S3 bucket operations
- Standard S3 API operations
- Future-proof with AWS ecosystem

### 2. **Rich Metadata**

- Bucket creation dates
- Consistent with S3 standards
- No need for separate metadata queries

### 3. **Performance**

- Direct communication with MinIO
- No Supabase Storage API overhead
- Efficient bucket enumeration

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

### Component State Management

```typescript
// content-data-analytics-files.svelte
let buckets: StorageBucket[] = $state([]);
let selectedBucket: StorageBucket | null = $state(null);

// Load buckets on component mount
onMount(async () => {
  await loadBuckets();
});
```

### Tauri Commands

```rust
#[tauri::command]
async fn get_minio_buckets() -> Result<Vec<StorageBucket>, String> {
    // Implementation as shown above
}
```

## Comparison with Supabase Storage

| Feature            | Supabase Storage    | Direct MinIO (AWS SDK) |
| ------------------ | ------------------- | ---------------------- |
| **Bucket Listing** | `/bucket` endpoint  | `list_buckets()`       |
| **Metadata**       | Database queries    | S3 API response        |
| **Authentication** | JWT tokens          | AWS credentials        |
| **Error Handling** | Supabase-specific   | AWS SDK standard       |
| **Performance**    | Database dependent  | Direct storage access  |
| **Reliability**    | Depends on Supabase | Independent            |

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
println!("✅ Found {} buckets via AWS SDK S3", buckets.len());
```

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
| **6** | Data Processing | None (Pure Rust)         | Response parsing and type conversion |
| **7** | Frontend Update | None (Pure SvelteKit)    | UI state updates and rendering       |

### Key Third-Party Dependencies

1. **AWS SDK S3 for Rust** (`aws-sdk-s3 = "1.107.0"`)
   - **Purpose**: S3 protocol implementation and HTTP communication
   - **Used For**: `list_buckets()`, authentication, error handling
   - **Why**: Industry standard, full S3 compatibility, robust error handling

2. **AWS Config** (`aws-config = "1.8.7"`)
   - **Purpose**: AWS service configuration and credential management
   - **Used For**: Credential providers, region settings, endpoint configuration
   - **Why**: Centralized AWS configuration, credential chain support

3. **MinIO S3-Compatible API**
   - **Purpose**: Object storage backend
   - **Used For**: Bucket storage, metadata storage, HTTP API endpoints
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
Rust Backend (Data Processing + Type Conversion)
    ↓ Tauri IPC
Frontend (UI Update)
```

## Conclusion

This implementation provides a robust, performant, and reliable way to list buckets from MinIO storage using the industry-standard AWS SDK S3. By bypassing Supabase Storage entirely, we gain better performance, richer metadata, and independence from Supabase's storage limitations.

**Key Benefits:**

- **Consistency**: Same AWS SDK S3 for both bucket listing and file operations
- **Performance**: Direct MinIO communication, no database overhead
- **Reliability**: Official AWS SDK with proper error handling
- **Independence**: No dependency on Supabase Storage configuration
- **Future-proof**: Easy to migrate to AWS S3 or other S3-compatible storage
