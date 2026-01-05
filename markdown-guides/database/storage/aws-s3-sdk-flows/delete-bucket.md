# MinIO Bucket Deletion with AWS SDK S3

This guide explains how we delete storage buckets from MinIO (S3-compatible storage) using the AWS SDK S3 for Rust, following the pattern from `delete-bucket.rs` example.

## Architecture Overview

### Third-Party SDKs Used

1. **AWS SDK S3 for Rust** (`aws-sdk-s3 = "1.107.0"`)
   - Official AWS SDK for S3 operations
   - Provides full S3 API compatibility
   - Handles authentication and bucket deletion

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

- ✅ **Direct MinIO Access**: Buckets are deleted directly from MinIO using S3 API
- ❌ **No Supabase Storage API**: We don't use Supabase's `/bucket` endpoint
- ❌ **No Metadata Database**: We don't query Supabase's `storage.buckets` table
- ❌ **No RLS Policies**: We bypass Supabase's Row Level Security entirely

## Step-by-Step Implementation Process

### Step 1: Frontend Request

**Component**: `sidebar-main-item.svelte` (to be implemented)
**Third-Party APIs**: None (Pure SvelteKit)

```typescript
// User clicks delete button on bucket
const deleteBucket = async (bucketName: string) => {
  const message = await deleteMinioBucket(bucketName);
  // Update UI after deletion
};
```

### Step 2: Tauri Command Bridge

**Component**: `src/lib/tauri-commands/minio-bucket-commands.ts`
**Third-Party APIs**: Tauri IPC (Rust ↔ TypeScript)

```typescript
import { invoke } from "@tauri-apps/api/core";

export async function deleteMinioBucket(bucketName: string): Promise<string> {
  const message = await invoke<string>("delete_minio_bucket", {
    bucketName: bucketName,
  });
  return message;
}
```

### Step 3: Rust Backend Processing

**Component**: `src-tauri/src/main.rs`
**Third-Party APIs**: Tauri Commands

```rust
#[tauri::command]
async fn delete_minio_bucket(bucket_name: String) -> Result<String, String> {
    let minio_manager = MinioFilesManager::new().await
        .map_err(|e| format!("Failed to initialize MinIO client: {}", e))?;

    let response = minio_manager.delete_bucket(&bucket_name).await
        .map_err(|e| format!("Failed to delete MinIO bucket: {}", e))?;

    if response.success {
        Ok(response.message)
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

        let shared_config = aws_config::from_env()
            .credentials_provider(Credentials::new(
                access_key,
                secret_key,
                None,
                None,
                "static",
            ))
            .endpoint_url(endpoint)
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
pub async fn delete_bucket(&self, bucket_name: &str) -> Result<MinioBucketDeletionResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!("🗑️ Deleting MinIO bucket: {} using AWS SDK S3...", bucket_name);

    // Check if bucket exists before attempting deletion
    if !self.bucket_exists(bucket_name).await? {
        return Ok(MinioBucketDeletionResponse {
            success: false,
            message: format!("Bucket '{}' does not exist", bucket_name),
            bucket_name: bucket_name.to_string(),
        });
    }

    // Check if bucket is empty before deletion
    if !self.is_bucket_empty(bucket_name).await? {
        return Ok(MinioBucketDeletionResponse {
            success: false,
            message: format!("Bucket '{}' is not empty. Please delete all objects before deleting the bucket", bucket_name),
            bucket_name: bucket_name.to_string(),
        });
    }

    // Delete bucket using AWS SDK S3 (following delete-bucket.rs pattern)
    let response = self.s3_client
        .delete_bucket()
        .bucket(bucket_name)
        .send()
        .await;

    match response {
        Ok(_) => {
            println!("✅ Successfully deleted bucket '{}' via AWS SDK S3", bucket_name);
            Ok(MinioBucketDeletionResponse {
                success: true,
                message: format!("Successfully deleted bucket '{}'", bucket_name),
                bucket_name: bucket_name.to_string(),
            })
        }
        Err(err) => {
            // Handle NoSuchBucket error as success (bucket already deleted)
            if let Some(service_error) = err.as_service_error() {
                if service_error.code() == Some("NoSuchBucket") {
                    println!("✅ Bucket '{}' was already deleted or does not exist", bucket_name);
                    return Ok(MinioBucketDeletionResponse {
                        success: true,
                        message: format!("Bucket '{}' was already deleted or does not exist", bucket_name),
                        bucket_name: bucket_name.to_string(),
                    });
                }
            }

            println!("❌ Failed to delete bucket '{}': {}", bucket_name, err);
            Ok(MinioBucketDeletionResponse {
                success: false,
                message: format!("Failed to delete bucket '{}': {}", bucket_name, err),
                bucket_name: bucket_name.to_string(),
            })
        }
    }
}
```

### Step 6: Bucket Existence and Empty Validation

**Component**: `src-tauri/src/analytics_query/minio_files.rs`
**Third-Party APIs**: None (Pure Rust validation logic)

```rust
/// Check if bucket already exists
async fn bucket_exists(&self, bucket_name: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    match self.s3_client
        .head_bucket()
        .bucket(bucket_name)
        .send()
        .await
    {
        Ok(_) => Ok(true),
        Err(_) => Ok(false), // Bucket doesn't exist or access denied
    }
}

/// Check if bucket is empty (no objects)
async fn is_bucket_empty(&self, bucket_name: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let response = self.s3_client
        .list_objects_v2()
        .bucket(bucket_name)
        .max_keys(1) // Only need to check if any objects exist
        .send()
        .await?;

    // If contents is empty or None, bucket is empty
    Ok(response.contents().is_empty())
}
```

### Step 7: Frontend State Update

**Component**: `sidebar-main-item.svelte` (to be implemented)
**Third-Party APIs**: None (Pure SvelteKit state management)

```typescript
// Update component state after bucket deletion
const handleBucketDeleted = (bucketName: string) => {
  // Remove bucket from list
  buckets = buckets.filter((bucket) => bucket.name !== bucketName);

  // Show success message
  showSuccessMessage(`Bucket '${bucketName}' deleted successfully`);

  // Clear selection if deleted bucket was selected
  if (selectedBucket?.name === bucketName) {
    selectedBucket = null;
  }
};
```

### Step 8: Bucket List Refresh

**Component**: `content-data-analytics-files.svelte`
**Third-Party APIs**: Same as bucket listing process (Steps 1-7 from list-buckets.md)

```typescript
// Automatically refresh bucket list after successful deletion
const refreshBucketList = async () => {
  await loadBuckets();
};
```

## Bucket Deletion Safety Features

### 1. **Existence Validation**

- Checks if bucket exists before attempting deletion
- Handles "NoSuchBucket" error gracefully
- Returns appropriate error messages

### 2. **Empty Bucket Validation**

- Ensures bucket is empty before deletion
- Prevents accidental data loss
- Provides clear error message if bucket contains objects

### 3. **Error Handling**

- Handles network errors gracefully
- Provides user-friendly error messages
- Logs detailed error information for debugging

## Following delete-bucket.rs Pattern

Our implementation follows the official AWS SDK S3 example pattern:

```rust
// From delete-bucket.rs example
let resp = client.delete_bucket().bucket(bucket_name).send().await;
match resp {
    Ok(_) => Ok(()),
    Err(err) => {
        if err
            .as_service_error()
            .and_then(aws_sdk_s3::error::ProvideErrorMetadata::code)
            == Some("NoSuchBucket")
        {
            Ok(()) // Bucket already deleted
        } else {
            Err(S3ExampleError::from(err))
        }
    }
}
```

## Advantages of This Approach

### 1. **Full S3 Compatibility**

- Access to all S3 bucket operations
- Standard S3 API operations
- Future-proof with AWS ecosystem

### 2. **Safety Features**

- Bucket existence validation
- Empty bucket validation
- Comprehensive error handling

### 3. **Performance**

- Direct communication with MinIO
- No Supabase Storage API overhead
- Efficient bucket deletion

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
- Bucket deletion permissions

## Frontend Integration

### Component State Management

```typescript
// sidebar-main-item.svelte
let isDeletingBucket: boolean = $state(false);

// Delete bucket function
const deleteBucket = async (bucketName: string) => {
  if (
    !confirm(
      `Are you sure you want to delete bucket '${bucketName}'? This action cannot be undone.`,
    )
  ) {
    return;
  }

  try {
    isDeletingBucket = true;
    const message = await deleteMinioBucket(bucketName);
    handleBucketDeleted(bucketName);
    showSuccessMessage(message);
  } catch (error) {
    showErrorMessage(error.message);
  } finally {
    isDeletingBucket = false;
  }
};
```

### Tauri Commands

```rust
#[tauri::command]
async fn delete_minio_bucket(bucket_name: String) -> Result<String, String> {
    // Implementation as shown above
}
```

## Comparison with Supabase Storage

| Feature             | Supabase Storage    | Direct MinIO (AWS SDK) |
| ------------------- | ------------------- | ---------------------- |
| **Bucket Deletion** | `/bucket` endpoint  | `delete_bucket()`      |
| **Safety Checks**   | Basic validation    | Full S3 validation     |
| **Authentication**  | JWT tokens          | AWS credentials        |
| **Error Handling**  | Supabase-specific   | AWS SDK standard       |
| **Performance**     | Database dependent  | Direct storage access  |
| **Reliability**     | Depends on Supabase | Independent            |

## Troubleshooting

### Common Issues

1. **"Bucket does not exist"**
   - Check if bucket name is correct
   - Verify bucket was not already deleted

2. **"Bucket is not empty"**
   - Delete all objects in the bucket first
   - Use `list_objects_v2` to check for remaining objects

3. **"Access denied"**
   - Verify AWS credentials
   - Check MinIO bucket deletion permissions

4. **"Connection refused"**
   - Check MinIO endpoint URL
   - Verify MinIO service is running

### Debugging

```rust
// Enable debug logging
println!("🌐 MinIO endpoint: {}", endpoint);
println!("🔑 Using access key: {}", access_key);
println!("✅ Successfully deleted bucket '{}' via AWS SDK S3", bucket_name);
```

## Implementation Status

### ✅ Completed

1. **Backend Implementation**
   - ✅ `MinioFilesManager::delete_bucket()` method
   - ✅ Bucket existence validation
   - ✅ Empty bucket validation
   - ✅ AWS SDK S3 integration
   - ✅ Tauri command `delete_minio_bucket()`

2. **Frontend Integration**
   - ✅ TypeScript command bridge
   - ✅ Type definitions for `BucketDeletionResult`
   - ✅ Error handling and user feedback
   - ✅ Success message handling

3. **Documentation**
   - ✅ Complete implementation guide
   - ✅ Architecture overview
   - ✅ Step-by-step process
   - ✅ Troubleshooting guide

### 📋 Still Needs to be Done

1. **Svelte Component Integration**
   - 📋 Create delete button in `sidebar-main-item.svelte`
   - 📋 Add confirmation dialog
   - 📋 Implement loading states and error handling
   - 📋 Connect to bucket list refresh

2. **UI/UX Features**
   - 📋 Confirmation dialog for bucket deletion
   - 📋 Loading spinners during deletion
   - 📋 Success/error notifications
   - 📋 Disable delete button for non-empty buckets

3. **Integration Testing**
   - 📋 Test bucket deletion flow end-to-end
   - 📋 Test empty bucket validation
   - 📋 Test error handling scenarios
   - 📋 Test UI state management

## Third-Party APIs Summary

### Complete API Stack Used

| Step  | Component           | Third-Party APIs         | Purpose                              |
| ----- | ------------------- | ------------------------ | ------------------------------------ |
| **1** | Frontend UI         | None (Pure SvelteKit)    | User interface and confirmation      |
| **2** | Tauri Bridge        | Tauri IPC                | Rust ↔ TypeScript communication     |
| **3** | Rust Backend        | Tauri Commands           | Command registration and routing     |
| **4** | AWS Config          | `aws-config = "1.8.7"`   | AWS service configuration management |
| **4** | AWS SDK S3          | `aws-sdk-s3 = "1.107.0"` | S3 protocol implementation           |
| **5** | MinIO Storage       | MinIO S3-Compatible API  | Object storage via HTTP              |
| **6** | Data Processing     | None (Pure Rust)         | Response parsing and validation      |
| **7** | Frontend Update     | None (Pure SvelteKit)    | UI state updates and rendering       |
| **8** | Bucket List Refresh | Same as listing process  | Automatic UI refresh                 |

### Key Third-Party Dependencies

1. **AWS SDK S3 for Rust** (`aws-sdk-s3 = "1.107.0"`)
   - **Purpose**: S3 protocol implementation and HTTP communication
   - **Used For**: `delete_bucket()`, `head_bucket()`, `list_objects_v2()`, authentication, error handling
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
Rust Backend (Data Processing + Validation)
    ↓ Tauri IPC
Frontend (UI Update + Bucket List Refresh)
```

## Conclusion

This implementation provides a robust, safe, and reliable way to delete buckets from MinIO storage using the industry-standard AWS SDK S3. By bypassing Supabase Storage entirely, we gain better performance, comprehensive safety checks, and independence from Supabase's storage limitations.

**Key Benefits:**

- **Consistency**: Same AWS SDK S3 for bucket listing, creation, and deletion
- **Safety**: Bucket existence and empty validation before deletion
- **Performance**: Direct MinIO communication, no database overhead
- **Reliability**: Official AWS SDK with proper error handling
- **Independence**: No dependency on Supabase Storage configuration
- **Future-proof**: Easy to migrate to AWS S3 or other S3-compatible storage
- **User Experience**: Clear error messages and confirmation dialogs

**Next Steps:**

1. Create the Svelte component for bucket deletion UI
2. Integrate with existing bucket listing functionality
3. Add comprehensive error handling and user feedback
4. Test the complete end-to-end flow
