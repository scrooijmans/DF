# MinIO Bucket Creation with AWS SDK S3

This guide explains how we create storage buckets in MinIO (S3-compatible storage) using the AWS SDK S3 for Rust, following the pattern from `create-bucket.rs` example.

## Architecture Overview

### Third-Party SDKs Used

1. **AWS SDK S3 for Rust** (`aws-sdk-s3 = "1.107.0"`)
   - Official AWS SDK for S3 operations
   - Provides full S3 API compatibility
   - Handles authentication and bucket creation

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

- ✅ **Direct MinIO Access**: Buckets are created directly in MinIO using S3 API
- ❌ **No Supabase Storage API**: We don't use Supabase's `/bucket` endpoint
- ❌ **No Metadata Database**: We don't query Supabase's `storage.buckets` table
- ❌ **No RLS Policies**: We bypass Supabase's Row Level Security entirely

## Step-by-Step Implementation Process

### Step 1: Frontend Request

**Component**: `sidebar-main-new-item.svelte` (to be implemented)
**Third-Party APIs**: None (Pure SvelteKit)

```typescript
// User submits bucket creation form
const createBucket = async (bucketName: string) => {
  const bucket = await createMinioBucket(bucketName);
  // Update UI with new bucket
};
```

### Step 2: Tauri Command Bridge

**Component**: `src/lib/tauri-commands/minio-bucket-commands.ts`
**Third-Party APIs**: Tauri IPC (Rust ↔ TypeScript)

```typescript
import { invoke } from "@tauri-apps/api/core";

export async function createMinioBucket(
  bucketName: string,
): Promise<StorageBucket> {
  const bucket = await invoke<StorageBucket>("create_minio_bucket", {
    bucketName: bucketName,
  });
  return bucket;
}
```

### Step 3: Rust Backend Processing

**Component**: `src-tauri/src/main.rs`
**Third-Party APIs**: Tauri Commands

```rust
#[tauri::command]
async fn create_minio_bucket(bucket_name: String) -> Result<StorageBucket, String> {
    let minio_manager = MinioFilesManager::new().await
        .map_err(|e| format!("Failed to initialize MinIO client: {}", e))?;

    let response = minio_manager.create_bucket(&bucket_name).await
        .map_err(|e| format!("Failed to create MinIO bucket: {}", e))?;

    if response.success {
        if let Some(bucket) = response.bucket {
            // Convert MinioBucket to StorageBucket
            let created_at = bucket.created_at.clone();
            let storage_bucket = StorageBucket {
                id: bucket.id,
                name: bucket.name,
                owner: "".to_string(),
                public: bucket.public,
                file_size_limit: None,
                allowed_mime_types: None,
                created_at: bucket.created_at,
                updated_at: created_at,
            };

            Ok(storage_bucket)
        } else {
            Err("Bucket creation succeeded but no bucket data returned".to_string())
        }
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
pub async fn create_bucket(&self, bucket_name: &str) -> Result<MinioBucketCreationResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!("🆕 Creating MinIO bucket: {} using AWS SDK S3...", bucket_name);

    // Validate bucket name (S3 naming rules)
    if !self.is_valid_bucket_name(bucket_name) {
        return Ok(MinioBucketCreationResponse {
            success: false,
            message: "Invalid bucket name. Bucket names must be 3-63 characters long, contain only lowercase letters, numbers, dots, and hyphens, and not start or end with a dot or hyphen.".to_string(),
            bucket: None,
        });
    }

    // Check if bucket already exists
    if self.bucket_exists(bucket_name).await? {
        return Ok(MinioBucketCreationResponse {
            success: false,
            message: format!("Bucket '{}' already exists", bucket_name),
            bucket: None,
        });
    }

    // Create bucket using AWS SDK S3
    let response = self.s3_client
        .create_bucket()
        .bucket(bucket_name)
        .send()
        .await?;

    // Get bucket location for verification
    let location_response = self.s3_client
        .get_bucket_location()
        .bucket(bucket_name)
        .send()
        .await?;

    let location = location_response.location_constraint()
        .map(|c| c.as_str().to_string())
        .unwrap_or_else(|| "us-east-1".to_string());

    println!("✅ Successfully created bucket '{}' in region '{}' via AWS SDK S3", bucket_name, location);

    // Create MinioBucket struct for response
    let created_bucket = MinioBucket {
        id: bucket_name.to_string(),
        name: bucket_name.to_string(),
        public: false, // Default to private
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(MinioBucketCreationResponse {
        success: true,
        message: format!("Successfully created bucket '{}' in region '{}'", bucket_name, location),
        bucket: Some(created_bucket),
    })
}
```

### Step 6: Bucket Name Validation

**Component**: `src-tauri/src/analytics_query/minio_files.rs`
**Third-Party APIs**: None (Pure Rust validation logic)

```rust
/// Validate bucket name according to S3 naming rules
fn is_valid_bucket_name(&self, name: &str) -> bool {
    // S3 bucket naming rules:
    // - 3-63 characters long
    // - Only lowercase letters, numbers, dots, and hyphens
    // - Cannot start or end with a dot or hyphen
    // - Cannot contain two adjacent dots
    // - Cannot be formatted as an IP address

    if name.len() < 3 || name.len() > 63 {
        return false;
    }

    if name.starts_with('.') || name.ends_with('.') ||
       name.starts_with('-') || name.ends_with('-') {
        return false;
    }

    if name.contains("..") {
        return false;
    }

    // Check for valid characters only
    for ch in name.chars() {
        if !ch.is_ascii_lowercase() && !ch.is_ascii_digit() && ch != '.' && ch != '-' {
            return false;
        }
    }

    // Check if it looks like an IP address
    if name.chars().filter(|&c| c == '.').count() == 3 {
        let parts: Vec<&str> = name.split('.').collect();
        if parts.len() == 4 && parts.iter().all(|part| part.parse::<u8>().is_ok()) {
            return false;
        }
    }

    true
}
```

### Step 7: Frontend State Update

**Component**: `sidebar-main-new-item.svelte` (to be implemented)
**Third-Party APIs**: None (Pure SvelteKit state management)

```typescript
// Update component state with new bucket
const handleBucketCreated = (bucket: StorageBucket) => {
  // Add bucket to list
  buckets = [...buckets, bucket];

  // Show success message
  showSuccessMessage(`Bucket '${bucket.name}' created successfully`);

  // Close creation dialog
  isCreatingBucket = false;
};
```

### Step 8: Bucket List Refresh

**Component**: `content-data-analytics-files.svelte`
**Third-Party APIs**: Same as bucket listing process (Steps 1-7 from list-buckets.md)

```typescript
// Automatically refresh bucket list after successful creation
const refreshBucketList = async () => {
  await loadBuckets();
};
```

## Bucket Metadata Created

- **Bucket Name**: S3 bucket name
- **Bucket ID**: Same as bucket name for compatibility
- **Creation Date**: Current timestamp when bucket was created
- **Public Access**: Defaults to false (private)
- **Owner**: Empty string (not applicable for MinIO)
- **Region**: us-east-1 (MinIO default)

## Following create-bucket.rs Pattern

Our implementation follows the official AWS SDK S3 example pattern:

```rust
// From create-bucket.rs example
let shared_config = aws_config::from_env().region(region_provider).load().await;
let client = Client::new(&shared_config);

create_bucket(&client, &bucket, &region).await?;
println!("Created bucket.");
```

## Advantages of This Approach

### 1. **Full S3 Compatibility**

- Access to all S3 bucket operations
- Standard S3 API operations
- Future-proof with AWS ecosystem

### 2. **Robust Validation**

- S3-compliant bucket name validation
- Duplicate bucket detection
- Comprehensive error messages

### 3. **Performance**

- Direct communication with MinIO
- No Supabase Storage API overhead
- Efficient bucket creation

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
- Bucket creation permissions

## Frontend Integration

### Component State Management

```typescript
// sidebar-main-new-item.svelte
let bucketName: string = $state("");
let isCreatingBucket: boolean = $state(false);
let validationMessage: string = $state("");

// Create bucket function
const createBucket = async () => {
  if (!isValidBucketName(bucketName)) {
    validationMessage = getBucketNameValidationMessage(bucketName);
    return;
  }

  try {
    isCreatingBucket = true;
    const bucket = await createMinioBucket(bucketName);
    handleBucketCreated(bucket);
  } catch (error) {
    showErrorMessage(error.message);
  } finally {
    isCreatingBucket = false;
  }
};
```

### Tauri Commands

```rust
#[tauri::command]
async fn create_minio_bucket(bucket_name: String) -> Result<StorageBucket, String> {
    // Implementation as shown above
}
```

## Comparison with Supabase Storage

| Feature             | Supabase Storage    | Direct MinIO (AWS SDK) |
| ------------------- | ------------------- | ---------------------- |
| **Bucket Creation** | `/bucket` endpoint  | `create_bucket()`      |
| **Validation**      | Basic validation    | Full S3 validation     |
| **Authentication**  | JWT tokens          | AWS credentials        |
| **Error Handling**  | Supabase-specific   | AWS SDK standard       |
| **Performance**     | Database dependent  | Direct storage access  |
| **Reliability**     | Depends on Supabase | Independent            |

## Troubleshooting

### Common Issues

1. **"Bucket already exists"**
   - Check if bucket name is already in use
   - Use a different bucket name

2. **"Invalid bucket name"**
   - Ensure name follows S3 naming rules
   - Use only lowercase letters, numbers, dots, and hyphens

3. **"Access denied"**
   - Verify AWS credentials
   - Check MinIO bucket creation permissions

4. **"Connection refused"**
   - Check MinIO endpoint URL
   - Verify MinIO service is running

### Debugging

```rust
// Enable debug logging
println!("🌐 MinIO endpoint: {}", endpoint);
println!("🔑 Using access key: {}", access_key);
println!("✅ Successfully created bucket '{}' in region '{}'", bucket_name, location);
```

## Implementation Status

### ✅ Completed

1. **Backend Implementation**
   - ✅ `MinioFilesManager::create_bucket()` method
   - ✅ S3-compliant bucket name validation
   - ✅ Duplicate bucket detection
   - ✅ AWS SDK S3 integration
   - ✅ Tauri command `create_minio_bucket()`

2. **Frontend Integration**
   - ✅ TypeScript command bridge
   - ✅ Type definitions for `StorageBucket`
   - ✅ Frontend validation functions
   - ✅ Error handling and user feedback

3. **Documentation**
   - ✅ Complete implementation guide
   - ✅ Architecture overview
   - ✅ Step-by-step process
   - ✅ Troubleshooting guide

### 📋 Still Needs to be Done

1. **Svelte Component Integration**
   - 📋 Create `sidebar-main-new-item.svelte` component
   - 📋 Add bucket creation form UI
   - 📋 Implement form validation display
   - 📋 Add loading states and error handling
   - 📋 Connect to bucket list refresh

2. **UI/UX Features**
   - 📋 Bucket creation dialog/modal
   - 📋 Real-time validation feedback
   - 📋 Success/error notifications
   - 📋 Loading spinners during creation

3. **Integration Testing**
   - 📋 Test bucket creation flow end-to-end
   - 📋 Test validation error handling
   - 📋 Test duplicate bucket detection
   - 📋 Test UI state management

## Third-Party APIs Summary

### Complete API Stack Used

| Step  | Component           | Third-Party APIs         | Purpose                              |
| ----- | ------------------- | ------------------------ | ------------------------------------ |
| **1** | Frontend UI         | None (Pure SvelteKit)    | User interface and form handling     |
| **2** | Tauri Bridge        | Tauri IPC                | Rust ↔ TypeScript communication     |
| **3** | Rust Backend        | Tauri Commands           | Command registration and routing     |
| **4** | AWS Config          | `aws-config = "1.8.7"`   | AWS service configuration management |
| **4** | AWS SDK S3          | `aws-sdk-s3 = "1.107.0"` | S3 protocol implementation           |
| **5** | MinIO Storage       | MinIO S3-Compatible API  | Object storage via HTTP              |
| **6** | Data Processing     | None (Pure Rust)         | Response parsing and type conversion |
| **7** | Frontend Update     | None (Pure SvelteKit)    | UI state updates and rendering       |
| **8** | Bucket List Refresh | Same as listing process  | Automatic UI refresh                 |

### Key Third-Party Dependencies

1. **AWS SDK S3 for Rust** (`aws-sdk-s3 = "1.107.0"`)
   - **Purpose**: S3 protocol implementation and HTTP communication
   - **Used For**: `create_bucket()`, `head_bucket()`, `get_bucket_location()`, authentication, error handling
   - **Why**: Industry standard, full S3 compatibility, robust error handling

2. **AWS Config** (`aws-config = "1.8.7"`)
   - **Purpose**: AWS service configuration and credential management
   - **Used For**: Credential providers, region settings, endpoint configuration
   - **Why**: Centralized AWS configuration, credential chain support

3. **Chrono** (`chrono = "0.4"`)
   - **Purpose**: Date and time handling
   - **Used For**: Timestamp generation for bucket creation time
   - **Why**: Reliable date/time library, timezone support

4. **MinIO S3-Compatible API**
   - **Purpose**: Object storage backend
   - **Used For**: Bucket storage, metadata storage, HTTP API endpoints
   - **Why**: S3-compatible, self-hosted, cost-effective

5. **Tauri IPC**
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
Frontend (UI Update + Bucket List Refresh)
```

## Conclusion

This implementation provides a robust, performant, and reliable way to create buckets in MinIO storage using the industry-standard AWS SDK S3. By bypassing Supabase Storage entirely, we gain better performance, comprehensive validation, and independence from Supabase's storage limitations.

**Key Benefits:**

- **Consistency**: Same AWS SDK S3 for both bucket creation and listing
- **Performance**: Direct MinIO communication, no database overhead
- **Reliability**: Official AWS SDK with proper error handling
- **Independence**: No dependency on Supabase Storage configuration
- **Future-proof**: Easy to migrate to AWS S3 or other S3-compatible storage
- **Validation**: Full S3-compliant bucket name validation
- **User Experience**: Comprehensive error messages and feedback

**Next Steps:**

1. Create the Svelte component for bucket creation UI
2. Integrate with existing bucket listing functionality
3. Add comprehensive error handling and user feedback
4. Test the complete end-to-end flow
