# MinIO File Upload Guide

This guide explains how to upload files to MinIO (S3-compatible storage) using multiple approaches, including the recommended AWS SDK S3 for Rust implementation.

## ⚠️ Important Note

**Direct MinIO uploads bypass Supabase Storage's metadata database.** Files uploaded this way will:

- ✅ Exist in MinIO storage
- ❌ NOT appear in Supabase Storage API calls
- ❌ NOT be visible in the Data Analytics UI (unless using our AWS SDK S3 implementation)
- ❌ NOT have metadata tracking (created_at, updated_at, etc.)

## Recommended Approach: AWS SDK S3 for Rust

**Current Implementation**: We now use AWS SDK S3 for both file listing and uploading, providing a consistent, robust solution.

## Prerequisites

### Python Requirements

```bash
pip install boto3 duckdb
```

### Environment Variables

```bash
# MinIO Configuration
MINIO_ENDPOINT=http://91.99.166.223:9000
MINIO_ACCESS_KEY=mudrock-storage
MINIO_SECRET_KEY=mudrock-storage-secret-2024
MINIO_REGION=us-east-1
```

## Method 1: AWS SDK S3 for Rust (Recommended for Backend)

### Advantages

- **Consistent with File Listing**: Uses same AWS SDK S3 as our file listing implementation
- **Full S3 API Support**: Access to all S3 features including multipart uploads
- **Robust Error Handling**: Official AWS SDK error handling
- **Progress Tracking**: Built-in support for upload progress monitoring
- **Presigned URLs**: Support for presigned upload URLs
- **Type Safety**: Full Rust type safety and compile-time checks

### Rust Implementation

```rust
use aws_sdk_s3 as s3;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{config::Region, Client, config::Credentials};

pub struct MinioUploadManager {
    s3_client: Client,
}

impl MinioUploadManager {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let access_key = env::var("AWS_ACCESS_KEY_ID")?;
        let secret_key = env::var("AWS_SECRET_ACCESS_KEY")?;
        let endpoint = "http://91.99.166.223:9000";
        let region = "us-east-1";

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

        let s3_config = s3::config::Builder::from(&shared_config)
            .force_path_style(true)
            .build();

        let s3_client = Client::from_conf(s3_config);
        Ok(MinioUploadManager { s3_client })
    }

    pub async fn upload_file(
        &self,
        bucket_name: &str,
        file_path: &str,
        content: &[u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let body = aws_sdk_s3::primitives::ByteStream::from(content);

        self.s3_client
            .put_object()
            .bucket(bucket_name)
            .key(file_path)
            .body(body)
            .content_type("application/octet-stream")
            .send()
            .await?;

        println!("✅ Successfully uploaded file to MinIO: {}/{}", bucket_name, file_path);
        Ok(())
    }

    // Upload with progress tracking (based on put-object-progress.rs)
    pub async fn upload_file_with_progress(
        &self,
        bucket_name: &str,
        file_path: &str,
        source_path: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let body = aws_sdk_s3::primitives::ByteStream::read_from()
            .path(source_path)
            .buffer_size(8192) // 8KB buffer for progress tracking
            .build()
            .await?;

        self.s3_client
            .put_object()
            .bucket(bucket_name)
            .key(file_path)
            .body(body)
            .content_type("application/octet-stream")
            .send()
            .await?;

        println!("✅ Successfully uploaded file with progress tracking: {}/{}", bucket_name, file_path);
        Ok(())
    }

    // Generate presigned upload URL (based on put-object-presigned.rs)
    pub async fn create_presigned_upload_url(
        &self,
        bucket_name: &str,
        file_path: &str,
        expires_in_seconds: u64,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let expires_in = std::time::Duration::from_secs(expires_in_seconds);
        let presigning_config = aws_sdk_s3::presigning::PresigningConfig::expires_in(expires_in)?;

        let presigned_request = self.s3_client
            .put_object()
            .bucket(bucket_name)
            .key(file_path)
            .presigned(presigning_config)
            .await?;

        Ok(presigned_request.uri().into())
    }
}
```

### Tauri Command Integration

```rust
#[tauri::command]
async fn upload_file_to_minio(
    bucket_name: String,
    file_path: String,
    content: Vec<u8>
) -> Result<(), String> {
    let upload_manager = MinioUploadManager::new().await
        .map_err(|e| format!("Failed to initialize upload manager: {}", e))?;

    upload_manager.upload_file(&bucket_name, &file_path, &content).await
        .map_err(|e| format!("Failed to upload file: {}", e))?;

    Ok(())
}
```

## Method 2: Using DuckDB S3 Integration (Alternative)

### Advantages

- Automatic CSV to Parquet conversion
- Native S3 protocol support
- Efficient for large datasets
- Supports complex queries

### Python Implementation

```python
#!/usr/bin/env python3
"""
Upload CSV file to MinIO using DuckDB S3 integration
"""

import duckdb
import os
from datetime import datetime

# Configuration
MINIO_ENDPOINT = "91.99.166.223:9000"  # No http:// prefix
MINIO_ACCESS_KEY = "mudrock-storage"
MINIO_SECRET_KEY = "mudrock-storage-secret-2024"
MINIO_REGION = "us-east-1"
BUCKET_NAME = "pets"

def setup_duckdb_minio():
    """Setup DuckDB for MinIO S3 integration"""
    conn = duckdb.connect()

    # Configure S3 secret for MinIO
    conn.sql(f"""
    DROP SECRET IF EXISTS minio_storage;
    CREATE SECRET minio_storage (
        TYPE S3,
        KEY_ID '{MINIO_ACCESS_KEY}',
        SECRET '{MINIO_SECRET_KEY}',
        ENDPOINT '{MINIO_ENDPOINT}',
        REGION '{MINIO_REGION}',
        URL_STYLE 'path',
        USE_SSL false
    )
    """)

    return conn

def upload_csv_to_minio(csv_file_path: str, bucket_name: str):
    """Upload CSV file to MinIO using DuckDB"""
    conn = setup_duckdb_minio()

    # Generate unique filename
    timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
    filename = f"upload_{timestamp}.parquet"
    s3_path = f"s3://{bucket_name}/{filename}"

    # Upload CSV as Parquet to MinIO
    conn.sql(f"""
    COPY (SELECT * FROM read_csv_auto('{csv_file_path}'))
    TO '{s3_path}'
    """)

    print(f"✅ Successfully uploaded {csv_file_path} to {s3_path}")
    conn.close()
    return s3_path

# Usage
if __name__ == "__main__":
    csv_file = "/path/to/your/file.csv"
    upload_csv_to_minio(csv_file, "pets")
```

## Method 3: Using Boto3 (Python - Alternative)

### Advantages

- Direct control over upload process
- Supports multipart uploads for large files
- More granular error handling

### Python Implementation

```python
#!/usr/bin/env python3
"""
Upload file to MinIO using Boto3
"""

import boto3
import os
from botocore.exceptions import ClientError

# Configuration
MINIO_ENDPOINT = "http://91.99.166.223:9000"
MINIO_ACCESS_KEY = "mudrock-storage"
MINIO_SECRET_KEY = "mudrock-storage-secret-2024"
MINIO_REGION = "us-east-1"

def create_minio_client():
    """Create S3 client for MinIO"""
    return boto3.client(
        's3',
        endpoint_url=MINIO_ENDPOINT,
        aws_access_key_id=MINIO_ACCESS_KEY,
        aws_secret_access_key=MINIO_SECRET_KEY,
        region_name=MINIO_REGION,
        use_ssl=False
    )

def upload_file_to_minio(file_path: str, bucket_name: str, s3_key: str):
    """Upload file to MinIO"""
    s3_client = create_minio_client()

    try:
        s3_client.upload_file(
            file_path,
            bucket_name,
            s3_key,
            ExtraArgs={
                'ContentType': 'application/octet-stream'
            }
        )
        print(f"✅ Successfully uploaded {file_path} to s3://{bucket_name}/{s3_key}")
        return True
    except ClientError as e:
        print(f"❌ Upload failed: {e}")
        return False

# Usage
if __name__ == "__main__":
    file_path = "/path/to/your/file.csv"
    bucket_name = "pets"
    s3_key = "uploaded_file.csv"

    upload_file_to_minio(file_path, bucket_name, s3_key)
```

## Method 4: Using cURL (Command Line)

### Advantages

- No Python dependencies
- Quick testing
- Scriptable

### Implementation

```bash
#!/bin/bash
# Upload file to MinIO using cURL

MINIO_ENDPOINT="http://91.99.166.223:9000"
BUCKET_NAME="pets"
FILE_PATH="/path/to/your/file.csv"
S3_KEY="uploaded_file.csv"

# Note: This requires proper AWS4-HMAC-SHA256 signature
# For testing, use boto3 or DuckDB instead
curl -X PUT \
  -H "Content-Type: application/octet-stream" \
  --data-binary "@$FILE_PATH" \
  "$MINIO_ENDPOINT/$BUCKET_NAME/$S3_KEY"
```

## Verification

### Check Files in MinIO

```python
import boto3

def list_minio_files(bucket_name: str):
    """List all files in MinIO bucket"""
    s3_client = boto3.client(
        's3',
        endpoint_url="http://91.99.166.223:9000",
        aws_access_key_id="mudrock-storage",
        aws_secret_access_key="mudrock-storage-secret-2024",
        region_name="us-east-1",
        use_ssl=False
    )

    response = s3_client.list_objects_v2(Bucket=bucket_name)

    if 'Contents' in response:
        print(f"Files in {bucket_name}:")
        for obj in response['Contents']:
            print(f"  📄 {obj['Key']} ({obj['Size']} bytes)")
    else:
        print(f"No files found in {bucket_name}")

# Usage
list_minio_files("pets")
```

## Troubleshooting

### Common Issues

1. **"Missing signature" error**
   - Use DuckDB or boto3 instead of raw cURL
   - Ensure proper AWS credentials

2. **"Connection refused" error**
   - Check MinIO endpoint URL
   - Verify MinIO service is running

3. **"Access denied" error**
   - Verify MinIO credentials
   - Check bucket permissions

### Debugging

```python
# Test MinIO connection
def test_minio_connection():
    try:
        s3_client = create_minio_client()
        response = s3_client.list_buckets()
        print("✅ MinIO connection successful")
        print(f"📦 Available buckets: {[b['Name'] for b in response['Buckets']]}")
        return True
    except Exception as e:
        print(f"❌ MinIO connection failed: {e}")
        return False
```

## Best Practices

1. **Use AWS SDK S3 for Rust** - Consistent with file listing, full S3 API support
2. **Use DuckDB for CSV/Parquet conversion** - Automatic format conversion
3. **Use boto3 for Python scripts** - More control and error handling
4. **Always verify uploads** - Check file exists in MinIO after upload
5. **Handle errors gracefully** - Implement proper error handling
6. **Use unique filenames** - Include timestamps to avoid conflicts
7. **Implement progress tracking** - For large file uploads
8. **Use presigned URLs** - For secure, time-limited uploads

## Step-by-Step Upload Process

### Step 1: Frontend File Drop

**Component**: `content-data-ingestion-dropzone.svelte`
**Third-Party APIs**:

- **Dropzone.js** - File drag & drop handling
- **File API** - Browser file reading

```typescript
// User drops CSV file
const handleFileDrop = (files: File[]) => {
  const file = files[0];
  const reader = new FileReader();
  reader.onload = (e) => {
    const csvContent = e.target?.result as string;
    uploadCsvFile(file.name, csvContent);
  };
  reader.readAsText(file);
};
```

### Step 2: Tauri Command Bridge

**Component**: `src/lib/tauri-commands/csv-upload-fetching.ts`
**Third-Party APIs**: Tauri IPC (Rust ↔ TypeScript)

```typescript
export async function uploadCsvAsParquetToMinio(
  bucketName: string,
  csvContent: string,
  fileName: string,
): Promise<UploadResult> {
  return await invoke<UploadResult>("upload_csv_as_parquet_to_minio", {
    bucketName,
    csvContent,
    fileName,
  });
}
```

### Step 3: Rust Backend Processing

**Component**: `src-tauri/src/main.rs`
**Third-Party APIs**: Tauri Commands

```rust
#[tauri::command]
async fn upload_csv_as_parquet_to_minio(
    bucket_name: String,
    csv_content: String,
    file_name: String
) -> Result<UploadResult, String> {
    let upload_manager = MinioUploadManager::new().await?;
    upload_manager.upload_csv_as_parquet(&bucket_name, &csv_content, &file_name).await
}
```

### Step 4: AWS SDK S3 Client Initialization

**Component**: `src-tauri/src/analytics_query/minio_upload.rs`
**Third-Party APIs**:

- `aws-config = "1.8.7"` - Configuration management
- `aws-sdk-s3 = "1.107.0"` - S3 operations

```rust
impl MinioUploadManager {
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
        Ok(MinioUploadManager { s3_client })
    }
}
```

### Step 5: CSV to Parquet Conversion

**Component**: `src-tauri/src/analytics_query/minio_upload.rs`
**Third-Party APIs**:

- **DuckDB** (`duckdb = "1.3.2"`) - CSV parsing and Parquet conversion
- **Chrono** (`chrono = "0.4"`) - Timestamp generation

```rust
pub async fn upload_csv_as_parquet(
    &self,
    bucket_name: &str,
    csv_content: &str,
    file_name: &str,
) -> Result<UploadResult, Box<dyn std::error::Error + Send + Sync>> {
    // Step 5a: Create temporary file with CSV content
    let temp_file_path = format!("/tmp/temp_csv_{}.csv", chrono::Utc::now().timestamp_millis());
    std::fs::write(&temp_file_path, csv_content)?;

    // Step 5b: Initialize DuckDB connection
    let conn = Connection::open_in_memory()?;

    // Step 5c: Setup S3 secret for MinIO (split into separate statements)
    conn.execute("DROP SECRET IF EXISTS minio_upload", [])?;
    conn.execute(&format!(
        "CREATE SECRET minio_upload (
            TYPE S3,
            KEY_ID '{}',
            SECRET '{}',
            ENDPOINT '{}',
            REGION '{}',
            URL_STYLE 'path',
            USE_SSL false
        )",
        access_key, secret_key, endpoint, region
    ), [])?;

    // Step 5d: Read CSV and convert to Parquet in memory
    let parquet_data = conn.query_arrow(
        &format!("SELECT * FROM read_csv_auto('{}')", temp_file_path)
    )?;

    // Step 5e: Convert Arrow data to Parquet bytes
    let mut buffer = Cursor::new(Vec::new());
    parquet_data.write_parquet(&mut buffer)?;
    let parquet_bytes = buffer.into_inner();

    // Step 5f: Clean up temporary file
    let _ = std::fs::remove_file(&temp_file_path);

    // Step 5g: Upload Parquet bytes to MinIO
    self.upload_file(bucket_name, file_name, &parquet_bytes).await
}
```

### Step 6: MinIO S3 API Upload

**Component**: `src-tauri/src/analytics_query/minio_upload.rs`
**Third-Party APIs**:

- **MinIO S3-Compatible API** - Direct HTTP communication
- **AWS SDK S3** - Handles S3 protocol, authentication, and upload

```rust
pub async fn upload_file(
    &self,
    bucket_name: &str,
    file_path: &str,
    content: &[u8],
) -> Result<UploadResult, Box<dyn std::error::Error + Send + Sync>> {
    // Step 6a: Convert content to AWS SDK S3 ByteStream
    let body = aws_sdk_s3::primitives::ByteStream::from(content.to_vec());

    // Step 6b: Build S3 put_object request
    self.s3_client
        .put_object()
        .bucket(bucket_name)
        .key(file_path)
        .body(body)
        .content_type("application/octet-stream")
        .send()
        .await?;

    // Step 6c: Return success result
    Ok(UploadResult {
        success: true,
        message: format!("Successfully uploaded {} to MinIO", file_path),
        file_path: file_path.to_string(),
    })
}
```

### Step 7: Frontend State Update

**Component**: `content-data-ingestion-dropzone.svelte`
**Third-Party APIs**: None (Pure SvelteKit state management)

```typescript
// Update upload progress
// Show success/error message
// Trigger file list refresh
const refreshFileList = async () => {
  await getMinioFiles(currentBucket);
};
```

### Step 8: File List Refresh

**Component**: `content-data-analytics-files-minio.svelte`
**Third-Party APIs**: Same as file listing process (Steps 1-7 from list-object.md)

```typescript
// Automatically refresh file list after successful upload
// New file appears immediately in UI
// User can click on file to view content
```

## Integration with Our Data Analytics UI

**Current Status**: Our Data Analytics UI now uses AWS SDK S3 for both listing and uploading files, providing a seamless experience:

### Advantages of This Approach

- **Consistent API**: Same AWS SDK S3 for both upload and listing
- **Immediate Visibility**: Uploaded files appear instantly in UI
- **No Sync Issues**: No metadata synchronization problems
- **Rich Metadata**: Full file metadata (size, date, etc.) available
- **Reliable**: Official AWS SDK with proper error handling

### Migration from Supabase Storage

We have completely migrated away from Supabase Storage for file operations:

- ❌ **Supabase Storage API**: No longer used for file operations
- ❌ **Metadata Database**: No dependency on `storage.objects` table
- ❌ **RLS Policies**: No Row Level Security dependencies
- ✅ **Direct MinIO**: All operations go directly to MinIO via S3 API
- ✅ **AWS SDK S3**: Consistent, reliable, feature-rich implementation

## Implementation Recommendations

### Current Status

**File Listing**: ✅ Implemented with AWS SDK S3
**File Upload**: ⚠️ Currently using DuckDB S3 integration

### Recommended Next Steps

1. **Replace DuckDB Upload with AWS SDK S3**
   - Create `MinioUploadManager` struct (as shown above)
   - Replace current `CsvUploadManager` with AWS SDK S3 implementation
   - Update Tauri commands to use new upload manager

2. **Add Advanced Upload Features**
   - Progress tracking for large files
   - Multipart uploads for files > 100MB
   - Presigned URLs for secure uploads
   - Resume capability for interrupted uploads

3. **Improve Error Handling**
   - AWS SDK specific error types
   - Retry logic for transient failures
   - Better user feedback for upload failures

### Code Changes Required

```rust
// Replace in src-tauri/src/csv_upload/csv_upload_manager.rs
impl CsvUploadManager {
    // Replace DuckDB S3 integration with AWS SDK S3
    pub async fn upload_parquet_to_minio_aws_sdk(
        &self,
        bucket_name: &str,
        file_path: &str,
        parquet_data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let upload_manager = MinioUploadManager::new().await?;
        upload_manager.upload_file(bucket_name, file_path, parquet_data).await?;
        Ok(())
    }
}
```

### Benefits of Migration

- **Consistency**: Same SDK for listing and uploading
- **Performance**: Better error handling and retry logic
- **Features**: Access to advanced S3 features
- **Maintainability**: Single codebase for all S3 operations
- **Future-proof**: Easy to add new S3 features

## Third-Party APIs Summary

### Complete API Stack Used

| Step  | Component         | Third-Party APIs            | Purpose                              |
| ----- | ----------------- | --------------------------- | ------------------------------------ |
| **1** | Frontend UI       | Dropzone.js, File API       | File drag & drop, file reading       |
| **2** | Tauri Bridge      | Tauri IPC                   | Rust ↔ TypeScript communication     |
| **3** | Rust Backend      | Tauri Commands              | Command registration and routing     |
| **4** | AWS Config        | `aws-config = "1.8.7"`      | AWS service configuration management |
| **4** | AWS SDK S3        | `aws-sdk-s3 = "1.107.0"`    | S3 protocol implementation           |
| **5** | CSV Processing    | DuckDB (`duckdb = "1.3.2"`) | CSV parsing and Parquet conversion   |
| **5** | Timestamps        | Chrono (`chrono = "0.4"`)   | Timestamp generation                 |
| **6** | MinIO Storage     | MinIO S3-Compatible API     | Object storage via HTTP              |
| **7** | Frontend Update   | None (Pure SvelteKit)       | UI state updates and rendering       |
| **8** | File List Refresh | Same as listing process     | Automatic UI refresh                 |

### Key Third-Party Dependencies

1. **AWS SDK S3 for Rust** (`aws-sdk-s3 = "1.107.0"`)
   - **Purpose**: S3 protocol implementation and HTTP communication
   - **Used For**: `put_object()`, authentication, upload progress, error handling
   - **Why**: Industry standard, full S3 compatibility, robust error handling

2. **AWS Config** (`aws-config = "1.8.7"`)
   - **Purpose**: AWS service configuration and credential management
   - **Used For**: Credential providers, region settings, endpoint configuration
   - **Why**: Centralized AWS configuration, credential chain support

3. **DuckDB** (`duckdb = "1.3.2"`)
   - **Purpose**: CSV parsing and Parquet conversion
   - **Used For**: `read_csv_auto()`, `write_parquet()`, S3 integration
   - **Why**: Fast analytics database, native Parquet support, S3 integration

4. **Chrono** (`chrono = "0.4"`)
   - **Purpose**: Date and time handling
   - **Used For**: Timestamp generation for unique filenames
   - **Why**: Reliable date/time library, timezone support

5. **MinIO S3-Compatible API**
   - **Purpose**: Object storage backend
   - **Used For**: File storage, metadata storage, HTTP API endpoints
   - **Why**: S3-compatible, self-hosted, cost-effective

6. **Tauri IPC**
   - **Purpose**: Frontend-backend communication
   - **Used For**: TypeScript ↔ Rust function calls
   - **Why**: Secure, type-safe, efficient

7. **Dropzone.js**
   - **Purpose**: File drag & drop handling
   - **Used For**: User file selection, drag & drop events
   - **Why**: User-friendly file upload interface

8. **File API**
   - **Purpose**: Browser file reading
   - **Used For**: Reading file contents as text
   - **Why**: Native browser API for file handling

### Data Flow Architecture

```
Frontend (SvelteKit + Dropzone.js)
    ↓ File API (readAsText)
Frontend (CSV content)
    ↓ Tauri IPC
Rust Backend (Tauri Commands)
    ↓ AWS SDK S3 + DuckDB
MinIO Storage (S3-Compatible API)
    ↓ HTTP Response
Rust Backend (Data Processing)
    ↓ Tauri IPC
Frontend (UI Update + File List Refresh)
```

### Upload vs Listing Comparison

| Feature             | File Listing                   | File Upload                   |
| ------------------- | ------------------------------ | ----------------------------- |
| **Primary API**     | AWS SDK S3 `list_objects_v2()` | AWS SDK S3 `put_object()`     |
| **Data Processing** | Response parsing               | CSV → Parquet conversion      |
| **Additional APIs** | None                           | DuckDB, Chrono, Dropzone.js   |
| **Complexity**      | Simple (read-only)             | Complex (data transformation) |
| **Performance**     | Fast (metadata only)           | Slower (file processing)      |
| **Error Handling**  | Standard S3 errors             | S3 + DuckDB + File API errors |
