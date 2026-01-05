# AWS SDK Rust S3 Client with MinIO Architecture Analysis

## Executive Summary

This document analyzes how the `aws-sdk-rust` S3 client is used in real Rust services with MinIO as the backing storage. The analysis covers client initialization, streaming operations, multipart uploads, call stacks, metadata handling, failure modes, and best practices for storing large Parquet files.

---

## 1. S3 Client Initialization and Configuration

### Basic Client Initialization

```rust
use aws_sdk_s3 as s3;
use aws_sdk_s3::{Client, config::Region, config::Credentials};
use aws_config::meta::region::RegionProviderChain;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment
    let config = aws_config::load_from_env().await;
    let client = s3::Client::new(&config);
    
    // Use client...
    Ok(())
}
```

### MinIO-Specific Configuration

For MinIO or other S3-compatible services, custom endpoint configuration is required:

```rust
use aws_sdk_s3 as s3;
use aws_sdk_s3::{Client, config::Region, config::Credentials};
use aws_config::meta::region::RegionProviderChain;

pub struct MinioClient {
    s3_client: Client,
}

impl MinioClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // 1. Load credentials from environment
        let access_key = std::env::var("AWS_ACCESS_KEY_ID")?;
        let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY")?;
        
        // 2. Configure MinIO endpoint
        let endpoint = "http://localhost:9000";  // MinIO default
        let region = "us-east-1";  // MinIO doesn't care, but SDK requires it
        
        // 3. Create region provider
        let region_provider = RegionProviderChain::first_try(Region::new(region))
            .or_default_provider()
            .or_else(Region::new(region));
        
        // 4. Build AWS config with custom endpoint
        let shared_config = aws_config::from_env()
            .credentials_provider(Credentials::new(
                access_key,
                secret_key,
                None,  // session token
                None,  // expires_at
                "static",  // provider name
            ))
            .endpoint_url(endpoint)  // CRITICAL: Custom endpoint for MinIO
            .region(region_provider)
            .load()
            .await;
        
        // 5. Create S3 config with MinIO-specific settings
        let s3_config = s3::config::Builder::from(&shared_config)
            .force_path_style(true)  // REQUIRED for MinIO (path-style URLs)
            .build();
        
        // 6. Create client from config
        let s3_client = Client::from_conf(s3_config);
        
        Ok(MinioClient { s3_client })
    }
}
```

### Configuration Options

**Key Configuration Parameters:**

1. **`endpoint_url`**: Custom endpoint for S3-compatible services (MinIO, DigitalOcean Spaces, etc.)
2. **`force_path_style`**: Use path-style URLs (`http://endpoint/bucket/key`) instead of virtual-hosted style (`http://bucket.endpoint/key`)
3. **`region`**: AWS region (MinIO ignores this, but SDK requires it)
4. **`credentials_provider`**: Static credentials or credential chain
5. **`retry_config`**: Retry behavior for transient failures
6. **`timeout_config`**: Request timeout settings

### Client Reuse Pattern

**Critical Best Practice**: Reuse clients across requests to leverage connection pooling:

```rust
// ❌ BAD: Creating new client for each request
async fn upload_file_bad(bucket: &str, key: &str, data: &[u8]) -> Result<()> {
    let client = Client::new(&aws_config::load_from_env().await);  // Expensive!
    client.put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(data))
        .send()
        .await?;
    Ok(())
}

// ✅ GOOD: Reuse client
struct StorageService {
    s3_client: Client,  // Created once, reused
}

impl StorageService {
    async fn upload_file(&self, bucket: &str, key: &str, data: &[u8]) -> Result<()> {
        self.s3_client.put_object()
            .bucket(bucket)
            .key(key)
            .body(ByteStream::from(data))
            .send()
            .await?;
        Ok(())
    }
}
```

**Why Reuse?**
- Each client maintains HTTP connection pools
- Identity caches reduce authentication overhead
- Configuration loading is expensive
- Connection establishment has latency

---

## 2. Streaming Uploads and Downloads

### Streaming Upload from File

```rust
use aws_sdk_s3::{Client, primitives::ByteStream};
use std::path::Path;

async fn upload_file_streaming(
    client: &Client,
    bucket: &str,
    key: &str,
    file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // ByteStream::from_path reads file in chunks, doesn't load entire file into memory
    let body = ByteStream::from_path(file_path).await?;
    
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .content_type("application/octet-stream")
        .send()
        .await?;
    
    Ok(())
}
```

### Streaming Upload from Memory

```rust
use aws_sdk_s3::{Client, primitives::ByteStream};

async fn upload_bytes_streaming(
    client: &Client,
    bucket: &str,
    key: &str,
    data: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    // ByteStream::from() creates a stream from bytes
    let body = ByteStream::from(data);
    
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await?;
    
    Ok(())
}
```

### Streaming Download to File

```rust
use aws_sdk_s3::Client;
use tokio::io::AsyncWriteExt;

async fn download_file_streaming(
    client: &Client,
    bucket: &str,
    key: &str,
    output_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get object response
    let resp = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;
    
    // Convert response body to async reader
    let mut data = resp.body.into_async_read();
    
    // Create output file
    let mut file = tokio::fs::File::create(output_path).await?;
    
    // Stream data to file (doesn't load entire file into memory)
    tokio::io::copy(&mut data, &mut file).await?;
    file.flush().await?;
    
    Ok(())
}
```

### Streaming Download to Memory (with Size Limit)

```rust
use aws_sdk_s3::Client;
use bytes::Bytes;

async fn download_bytes_streaming(
    client: &Client,
    bucket: &str,
    key: &str,
    max_size: usize,
) -> Result<Bytes, Box<dyn std::error::Error>> {
    let resp = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;
    
    // Collect stream into bytes with size limit
    let mut data = resp.body.into_async_read();
    let mut buffer = Vec::with_capacity(max_size.min(1024 * 1024));  // 1MB initial capacity
    
    use tokio::io::AsyncReadExt;
    let mut chunk = vec![0u8; 8192];  // 8KB chunks
    
    loop {
        let n = data.read(&mut chunk).await?;
        if n == 0 {
            break;
        }
        
        buffer.extend_from_slice(&chunk[..n]);
        
        // Enforce size limit
        if buffer.len() > max_size {
            return Err("File too large".into());
        }
    }
    
    Ok(Bytes::from(buffer))
}
```

### ByteStream Configuration

```rust
use aws_sdk_s3::primitives::ByteStream;

// Configure buffer size for progress tracking
let body = ByteStream::read_from()
    .path(file_path)
    .buffer_size(8192)  // 8KB buffer for progress tracking
    .build()
    .await?;
```

**Benefits of Streaming:**
- **Memory Efficiency**: Doesn't load entire file into memory
- **Progress Tracking**: Can monitor upload/download progress
- **Large File Support**: Can handle files larger than available RAM
- **Backpressure**: Respects network and disk I/O limits

---

## 3. Multipart Uploads

### When to Use Multipart Uploads

**S3 Requirements:**
- Files > 5MB: Recommended to use multipart
- Files > 100MB: Should use multipart
- Files > 5GB: Must use multipart

**Benefits:**
- Parallel upload of parts
- Resume capability (if tracking part uploads)
- Better error recovery (only failed parts need retry)
- Progress tracking per part

### Multipart Upload Implementation

```rust
use aws_sdk_s3::{Client, primitives::ByteStream, types::CompletedPart};
use std::collections::HashMap;
use tokio::io::AsyncReadExt;

pub struct MultipartUpload {
    client: Client,
    bucket: String,
    key: String,
    upload_id: String,
    part_number: i32,
    completed_parts: Vec<CompletedPart>,
}

impl MultipartUpload {
    pub async fn new(
        client: Client,
        bucket: String,
        key: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // 1. Initiate multipart upload
        let resp = client
            .create_multipart_upload()
            .bucket(&bucket)
            .key(&key)
            .content_type("application/octet-stream")
            .send()
            .await?;
        
        let upload_id = resp.upload_id().ok_or("No upload ID")?.to_string();
        
        Ok(Self {
            client,
            bucket,
            key,
            upload_id,
            part_number: 1,
            completed_parts: Vec::new(),
        })
    }
    
    pub async fn upload_part(
        &mut self,
        data: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 2. Upload a single part
        let body = ByteStream::from(data.clone());
        
        let resp = self.client
            .upload_part()
            .bucket(&self.bucket)
            .key(&self.key)
            .upload_id(&self.upload_id)
            .part_number(self.part_number)
            .body(body)
            .send()
            .await?;
        
        // 3. Store ETag for completion
        let etag = resp.e_tag().ok_or("No ETag")?;
        let completed_part = CompletedPart::builder()
            .e_tag(etag)
            .part_number(self.part_number)
            .build();
        
        self.completed_parts.push(completed_part);
        self.part_number += 1;
        
        Ok(())
    }
    
    pub async fn complete(self) -> Result<(), Box<dyn std::error::Error>> {
        // 4. Complete multipart upload
        self.client
            .complete_multipart_upload()
            .bucket(&self.bucket)
            .key(&self.key)
            .upload_id(&self.upload_id)
            .multipart_upload(
                aws_sdk_s3::types::CompletedMultipartUpload::builder()
                    .set_parts(Some(self.completed_parts))
                    .build(),
            )
            .send()
            .await?;
        
        Ok(())
    }
    
    pub async fn abort(self) -> Result<(), Box<dyn std::error::Error>> {
        // Abort multipart upload (cleanup)
        self.client
            .abort_multipart_upload()
            .bucket(&self.bucket)
            .key(&self.key)
            .upload_id(&self.upload_id)
            .send()
            .await?;
        
        Ok(())
    }
}
```

### Parallel Multipart Upload

```rust
use tokio::task::JoinSet;

async fn upload_large_file_parallel(
    client: Client,
    bucket: &str,
    key: &str,
    file_path: &Path,
    part_size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initiate multipart upload
    let resp = client
        .create_multipart_upload()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;
    
    let upload_id = resp.upload_id().ok_or("No upload ID")?.to_string();
    
    // 2. Read file and split into parts
    let file = tokio::fs::File::open(file_path).await?;
    let mut reader = tokio::io::BufReader::new(file);
    let mut part_number = 1;
    let mut completed_parts = Vec::new();
    
    // 3. Upload parts in parallel
    let mut upload_tasks = JoinSet::new();
    
    loop {
        let mut part_data = vec![0u8; part_size];
        let n = reader.read(&mut part_data).await?;
        
        if n == 0 {
            break;
        }
        
        part_data.truncate(n);
        let part_num = part_number;
        let client_clone = client.clone();
        let bucket = bucket.to_string();
        let key = key.to_string();
        let upload_id = upload_id.clone();
        
        // Spawn parallel upload task
        upload_tasks.spawn(async move {
            let body = ByteStream::from(part_data);
            let resp = client_clone
                .upload_part()
                .bucket(&bucket)
                .key(&key)
                .upload_id(&upload_id)
                .part_number(part_num)
                .body(body)
                .send()
                .await?;
            
            let etag = resp.e_tag().ok_or("No ETag")?;
            Ok::<(i32, String), Box<dyn std::error::Error + Send + Sync>>((
                part_num,
                etag.to_string(),
            ))
        });
        
        part_number += 1;
    }
    
    // 4. Collect completed parts
    while let Some(result) = upload_tasks.join_next().await {
        let (part_num, etag) = result??;
        let completed_part = CompletedPart::builder()
            .e_tag(&etag)
            .part_number(part_num)
            .build();
        completed_parts.push(completed_part);
    }
    
    // Sort by part number (required by S3)
    completed_parts.sort_by_key(|p| p.part_number());
    
    // 5. Complete multipart upload
    client
        .complete_multipart_upload()
        .bucket(bucket)
        .key(key)
        .upload_id(&upload_id)
        .multipart_upload(
            aws_sdk_s3::types::CompletedMultipartUpload::builder()
                .set_parts(Some(completed_parts))
                .build(),
        )
        .send()
        .await?;
    
    Ok(())
}
```

### Multipart Upload Internals

**S3 Multipart Upload Process:**

1. **`create_multipart_upload()`**: 
   - Returns `upload_id`
   - No data transferred yet
   - Creates "pending" upload state in S3

2. **`upload_part()`** (called multiple times):
   - Each part: 5MB minimum (except last part)
   - Each part: 5GB maximum
   - Returns `ETag` for each part
   - Parts can be uploaded in parallel
   - Parts can be uploaded in any order

3. **`complete_multipart_upload()`**:
   - Combines all parts into final object
   - Requires all part numbers and ETags
   - Parts must be sorted by part number
   - Atomic operation (all or nothing)

4. **`abort_multipart_upload()`** (cleanup):
   - Removes all uploaded parts
   - Frees storage space
   - Should be called on failure

**Part Size Recommendations:**
- **Small files (< 100MB)**: 5-10MB parts
- **Medium files (100MB - 1GB)**: 10-50MB parts
- **Large files (> 1GB)**: 50-100MB parts
- **Very large files (> 10GB)**: 100MB parts

---

## 4. Call Stack for File Upload from HTTP Request to Object Storage

### Complete Call Stack

```
1. HTTP Request arrives
   │
   │   POST /api/blobs/upload
   │   Content-Type: multipart/form-data
   │   Body: <file data>
   │
   ▼
2. Axum Router matches route
   │
   │   Router::new()
   │   .route("/api/blobs/upload", post(upload_handler))
   │
   ▼
3. Axum Handler extracts request
   │
   │   async fn upload_handler(
   │       State(state): State<AppState>,
   │       mut multipart: Multipart,
   │   ) -> Result<Json<UploadResponse>, AppError>
   │
   │   3.1. Extract file from multipart form
   │   3.2. Validate file size/type
   │   3.3. Compute content hash (optional)
   │
   ▼
4. Storage Service Layer
   │
   │   state.storage_service.upload_file(
   │       bucket: &str,
   │       key: &str,
   │       data: Vec<u8>,
   │   ) -> Result<String>
   │
   │   4.1. Check if file already exists (deduplication)
   │   4.2. Determine upload strategy (simple vs multipart)
   │
   ▼
5. AWS SDK S3 Client
   │
   │   client.put_object()
   │   .bucket(bucket)
   │   .key(key)
   │   .body(ByteStream::from(data))
   │   .send()
   │   .await
   │
   │   5.1. ByteStream creation
   │       ByteStream::from(data)
   │       → Creates stream from bytes
   │
   │   5.2. Request building
   │       put_object()
   │       → Builds HTTP request
   │       → Signs request (AWS Signature V4)
   │       → Adds headers (Content-Type, Content-Length, etc.)
   │
   │   5.3. HTTP client execution
   │       send().await
   │       → Gets connection from pool
   │       → Sends HTTP PUT request
   │       → Streams body to HTTP connection
   │
   ▼
6. HTTP Transport Layer
   │
   │   Hyper HTTP client
   │   → Establishes TCP connection (if not pooled)
   │   → Sends HTTP request
   │   → Streams request body
   │   → Receives HTTP response
   │
   ▼
7. MinIO Server (S3-Compatible API)
   │
   │   7.1. Receives HTTP PUT request
   │   7.2. Validates AWS Signature V4
   │   7.3. Checks bucket permissions
   │   7.4. Streams data to storage backend
   │   7.5. Writes object metadata
   │   7.6. Returns HTTP 200 OK with ETag
   │
   ▼
8. Response flows back through stack
   │
   │   8.1. HTTP response received
   │   8.2. AWS SDK parses response
   │   8.3. Returns Result<PutObjectOutput>
   │   8.4. Storage service returns success
   │   8.5. Handler returns JSON response
   │
   ▼
9. HTTP Response sent to client
   │
   │   HTTP 200 OK
   │   Content-Type: application/json
   │   Body: { "success": true, "key": "...", "etag": "..." }
```

### Detailed Code Flow

```rust
// Step 1-3: HTTP Handler
use axum::{
    extract::{State, Multipart},
    Json,
};
use bytes::Bytes;

async fn upload_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, AppError> {
    // Extract file from multipart form
    while let Some(field) = multipart.next_field().await? {
        if field.name() == Some("file") {
            let filename = field.file_name()
                .ok_or(AppError::InvalidRequest)?
                .to_string();
            let data = field.bytes().await?;
            
            // Step 4: Storage service
            let key = state.storage_service
                .upload_file("my-bucket", &filename, &data)
                .await?;
            
            return Ok(Json(UploadResponse {
                success: true,
                key,
            }));
        }
    }
    
    Err(AppError::InvalidRequest)
}

// Step 4: Storage Service
struct StorageService {
    s3_client: Client,
}

impl StorageService {
    async fn upload_file(
        &self,
        bucket: &str,
        key: &str,
        data: &[u8],
    ) -> Result<String, StorageError> {
        // Check if already exists (deduplication)
        if self.exists(bucket, key).await? {
            return Ok(key.to_string());
        }
        
        // Step 5: AWS SDK S3 Client
        let body = ByteStream::from(data.to_vec());
        
        let resp = self.s3_client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(body)
            .content_type("application/octet-stream")
            .send()  // Step 6-7: HTTP transport + MinIO
            .await
            .map_err(|e| StorageError::UploadFailed(e.to_string()))?;
        
        Ok(key.to_string())
    }
}
```

### Streaming Upload Call Stack

For large files, the call stack includes streaming:

```rust
// Streaming upload from HTTP request body
async fn upload_streaming_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: StreamBody,
) -> Result<Json<UploadResponse>, AppError> {
    // Extract content length
    let content_length: usize = headers
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    
    // Determine upload strategy
    if content_length > 100 * 1024 * 1024 {  // > 100MB
        // Use multipart upload
        state.storage_service
            .upload_multipart("my-bucket", "key", body, content_length)
            .await?;
    } else {
        // Use simple upload
        let data = body.collect().await?.to_bytes();
        state.storage_service
            .upload_file("my-bucket", "key", &data)
            .await?;
    }
    
    Ok(Json(UploadResponse { success: true }))
}
```

---

## 5. Metadata vs Bulk Data Handling

### Metadata Storage

**S3 Object Metadata:**
- Stored as HTTP headers
- Limited to 2KB total
- Key-value pairs (string keys, string values)
- Automatically indexed by S3

```rust
use aws_sdk_s3::Client;

async fn upload_with_metadata(
    client: &Client,
    bucket: &str,
    key: &str,
    data: Vec<u8>,
    metadata: HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut request = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(data));
    
    // Add metadata (custom headers)
    for (k, v) in metadata {
        request = request.metadata(k, v);
    }
    
    // Standard metadata
    request = request
        .content_type("application/parquet")
        .content_encoding("gzip")
        .cache_control("max-age=3600");
    
    request.send().await?;
    Ok(())
}
```

### Retrieving Metadata

```rust
async fn get_metadata(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let resp = client
        .head_object()  // HEAD request (metadata only, no body)
        .bucket(bucket)
        .key(key)
        .send()
        .await?;
    
    let mut metadata = HashMap::new();
    
    // Standard metadata
    if let Some(size) = resp.content_length() {
        metadata.insert("size".to_string(), size.to_string());
    }
    if let Some(content_type) = resp.content_type() {
        metadata.insert("content-type".to_string(), content_type.to_string());
    }
    if let Some(etag) = resp.e_tag() {
        metadata.insert("etag".to_string(), etag.to_string());
    }
    if let Some(last_modified) = resp.last_modified() {
        metadata.insert("last-modified".to_string(), last_modified.to_string());
    }
    
    // Custom metadata
    if let Some(custom) = resp.metadata() {
        for (k, v) in custom {
            metadata.insert(k.to_string(), v.to_string());
        }
    }
    
    Ok(metadata)
}
```

### Separate Metadata Database Pattern

For complex metadata (beyond 2KB limit or requiring queries):

```rust
use sqlx::PgPool;

struct BlobMetadata {
    hash: String,
    size: i64,
    content_type: String,
    created_at: chrono::DateTime<chrono::Utc>,
    tags: Vec<String>,
    // ... other fields
}

struct StorageService {
    s3_client: Client,
    db: PgPool,  // Metadata database
}

impl StorageService {
    async fn upload_with_metadata(
        &self,
        bucket: &str,
        key: &str,
        data: &[u8],
        metadata: BlobMetadata,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 1. Upload bulk data to S3
        self.s3_client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(ByteStream::from(data.to_vec()))
            .send()
            .await?;
        
        // 2. Store metadata in database
        sqlx::query!(
            r#"
            INSERT INTO blob_metadata (hash, size, content_type, created_at, tags)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            metadata.hash,
            metadata.size,
            metadata.content_type,
            metadata.created_at,
            &metadata.tags,
        )
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
    
    async fn list_blobs(
        &self,
        filters: BlobFilters,
    ) -> Result<Vec<BlobMetadata>, Box<dyn std::error::Error>> {
        // Query metadata from database (fast, indexed)
        let blobs = sqlx::query_as!(
            BlobMetadata,
            r#"
            SELECT hash, size, content_type, created_at, tags
            FROM blob_metadata
            WHERE ($1::text[] IS NULL OR tags && $1)
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            filters.tags,
            filters.limit,
        )
        .fetch_all(&self.db)
        .await?;
        
        Ok(blobs)
    }
    
    async fn get_blob_data(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<Bytes, Box<dyn std::error::Error>> {
        // Fetch bulk data from S3 (only when needed)
        let resp = self.s3_client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;
        
        let data = resp.body.collect().await?.to_bytes();
        Ok(data)
    }
}
```

### Metadata vs Bulk Data Tradeoffs

| Aspect | S3 Metadata | Separate Database |
|--------|-------------|-------------------|
| **Size Limit** | 2KB total | Unlimited |
| **Query Capability** | Limited (list objects) | Full SQL queries |
| **Performance** | Fast (HTTP headers) | Fast (indexed DB) |
| **Consistency** | Always consistent | Requires sync |
| **Cost** | Included in S3 | Additional DB cost |
| **Complexity** | Simple | More complex |

**Best Practice**: Use S3 metadata for simple key-value pairs, use separate database for complex queries and large metadata.

---

## 6. Failure Modes and Mitigation

### Common Failure Modes

#### 1. Network Failures

**Symptoms:**
- Connection timeouts
- Connection reset
- DNS resolution failures

**Mitigation:**
```rust
use aws_config::retry::RetryConfig;

let config = aws_config::from_env()
    .retry_config(
        RetryConfig::standard()
            .with_max_attempts(3)
            .with_initial_backoff(Duration::from_secs(1))
    )
    .timeout_config(
        TimeoutConfig::builder()
            .connect_timeout(Duration::from_secs(5))
            .read_timeout(Duration::from_secs(30))
            .build()
    )
    .load()
    .await;
```

#### 2. Authentication Failures

**Symptoms:**
- 403 Forbidden
- Invalid credentials
- Expired credentials

**Mitigation:**
```rust
use aws_config::meta::credentials::CredentialsProviderChain;

// Use credential chain with fallbacks
let credentials = CredentialsProviderChain::default()
    .with_env()
    .with_profile("default")
    .with_container_credentials()
    .with_web_identity_token();

let config = aws_config::from_env()
    .credentials_provider(credentials)
    .load()
    .await;
```

#### 3. Rate Limiting / Throttling

**Symptoms:**
- 503 Service Unavailable
- 429 Too Many Requests
- Slow responses

**Mitigation:**
```rust
use tokio::time::{sleep, Duration};

async fn upload_with_backoff(
    client: &Client,
    bucket: &str,
    key: &str,
    data: Vec<u8>,
    max_retries: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut retries = 0;
    
    loop {
        match client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(ByteStream::from(data.clone()))
            .send()
            .await
        {
            Ok(_) => return Ok(()),
            Err(e) => {
                if retries >= max_retries {
                    return Err(e.into());
                }
                
                // Check if retryable error
                if is_retryable_error(&e) {
                    let backoff = exponential_backoff(retries);
                    sleep(backoff).await;
                    retries += 1;
                } else {
                    return Err(e.into());
                }
            }
        }
    }
}

fn is_retryable_error(e: &aws_sdk_s3::Error) -> bool {
    // Check error code
    if let Some(code) = e.code() {
        matches!(code, "Throttling" | "ServiceUnavailable" | "InternalError")
    } else {
        false
    }
}

fn exponential_backoff(retry: u32) -> Duration {
    Duration::from_secs(2_u64.pow(retry))
}
```

#### 4. Partial Upload Failures (Multipart)

**Symptoms:**
- Some parts succeed, others fail
- Incomplete multipart uploads
- Storage costs for orphaned parts

**Mitigation:**
```rust
async fn upload_multipart_with_cleanup(
    client: &Client,
    bucket: &str,
    key: &str,
    file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Initiate upload
    let resp = client
        .create_multipart_upload()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;
    
    let upload_id = resp.upload_id().ok_or("No upload ID")?.to_string();
    
    // Upload parts with error handling
    let mut completed_parts = Vec::new();
    let mut part_number = 1;
    
    // ... upload parts ...
    
    // On success: complete upload
    match client
        .complete_multipart_upload()
        .bucket(bucket)
        .key(key)
        .upload_id(&upload_id)
        .multipart_upload(/* ... */)
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            // On failure: abort upload (cleanup)
            let _ = client
                .abort_multipart_upload()
                .bucket(bucket)
                .key(key)
                .upload_id(&upload_id)
                .send()
                .await;
            Err(e.into())
        }
    }
}
```

#### 5. Data Corruption

**Symptoms:**
- Hash mismatch
- Invalid file format
- Partial reads

**Mitigation:**
```rust
use sha2::{Sha256, Digest};

async fn upload_with_verification(
    client: &Client,
    bucket: &str,
    key: &str,
    data: &[u8],
) -> Result<String, Box<dyn std::error::Error>> {
    // 1. Compute hash before upload
    let mut hasher = Sha256::new();
    hasher.update(data);
    let expected_hash = hex::encode(hasher.finalize());
    
    // 2. Upload with content MD5 (S3 verifies)
    let mut hasher = md5::Md5::new();
    hasher.update(data);
    let md5_hash = base64::encode(hasher.finalize());
    
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(data.to_vec()))
        .content_md5(&md5_hash)  // S3 verifies MD5
        .metadata("sha256", &expected_hash)  // Store SHA256 in metadata
        .send()
        .await?;
    
    // 3. Verify after upload (optional)
    let resp = client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;
    
    let downloaded = resp.body.collect().await?.to_bytes();
    let mut hasher = Sha256::new();
    hasher.update(&downloaded);
    let actual_hash = hex::encode(hasher.finalize());
    
    if actual_hash != expected_hash {
        return Err("Hash mismatch after upload".into());
    }
    
    Ok(expected_hash)
}
```

#### 6. Bucket/Key Not Found

**Symptoms:**
- 404 Not Found
- NoSuchBucket
- NoSuchKey

**Mitigation:**
```rust
async fn safe_upload(
    client: &Client,
    bucket: &str,
    key: &str,
    data: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Check if bucket exists
    match client.head_bucket().bucket(bucket).send().await {
        Ok(_) => {},
        Err(e) if e.code() == Some("NotFound") => {
            // Create bucket if it doesn't exist
            client
                .create_bucket()
                .bucket(bucket)
                .send()
                .await?;
        },
        Err(e) => return Err(e.into()),
    }
    
    // 2. Upload with error handling
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(data))
        .send()
        .await?;
    
    Ok(())
}
```

### Comprehensive Error Handling

```rust
use aws_sdk_s3::Error as S3Error;

async fn upload_with_comprehensive_error_handling(
    client: &Client,
    bucket: &str,
    key: &str,
    data: Vec<u8>,
) -> Result<(), StorageError> {
    let result = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(data))
        .send()
        .await;
    
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            match e {
                S3Error::NoSuchBucket(_) => {
                    Err(StorageError::BucketNotFound(bucket.to_string()))
                },
                S3Error::InvalidAccessKeyId(_) => {
                    Err(StorageError::AuthenticationFailed)
                },
                S3Error::ServiceError(service_err) => {
                    if service_err.code() == Some("Throttling") {
                        Err(StorageError::RateLimited)
                    } else {
                        Err(StorageError::ServiceError(service_err.to_string()))
                    }
                },
                _ => Err(StorageError::Unknown(e.to_string())),
            }
        }
    }
}
```

---

## 7. Best Practices for Storing Large Parquet Files

### File Size Optimization

**Optimal Parquet File Sizes:**
- **Minimum**: 128 MB
- **Optimal**: 256-512 MB
- **Maximum**: 1 GB (for single file)

**Why?**
- Too small: Increased metadata overhead, slower queries
- Too large: Slower parallel processing, memory issues

```rust
async fn write_optimal_parquet_files(
    data: Vec<Record>,
    bucket: &str,
    prefix: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    const TARGET_SIZE: usize = 256 * 1024 * 1024;  // 256 MB
    let mut current_batch = Vec::new();
    let mut current_size = 0;
    let mut file_keys = Vec::new();
    
    for record in data {
        let record_size = estimate_record_size(&record);
        
        if current_size + record_size > TARGET_SIZE && !current_batch.is_empty() {
            // Write current batch to Parquet file
            let key = format!("{}/part_{}.parquet", prefix, file_keys.len());
            write_parquet_to_s3(&client, bucket, &key, &current_batch).await?;
            file_keys.push(key);
            
            // Reset batch
            current_batch.clear();
            current_size = 0;
        }
        
        current_batch.push(record);
        current_size += record_size;
    }
    
    // Write remaining batch
    if !current_batch.is_empty() {
        let key = format!("{}/part_{}.parquet", prefix, file_keys.len());
        write_parquet_to_s3(&client, bucket, &key, &current_batch).await?;
        file_keys.push(key);
    }
    
    Ok(file_keys)
}
```

### Partitioning Strategy

**Partition by frequently queried fields:**

```rust
// Partition by date and region
// s3://bucket/data/date=2024-01-01/region=us-east/part_0.parquet

async fn write_partitioned_parquet(
    data: Vec<Record>,
    bucket: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Group by partition keys
    let mut partitions: HashMap<(String, String), Vec<Record>> = HashMap::new();
    
    for record in data {
        let partition_key = (
            record.date.format("%Y-%m-%d").to_string(),
            record.region.clone(),
        );
        partitions.entry(partition_key).or_insert_with(Vec::new).push(record);
    }
    
    // Write each partition
    for ((date, region), records) in partitions {
        let prefix = format!("data/date={}/region={}", date, region);
        write_optimal_parquet_files(records, bucket, &prefix).await?;
    }
    
    Ok(())
}
```

### Compression

**Use Snappy compression (good balance):**

```rust
use parquet::file::properties::WriterProperties;

fn create_parquet_writer_properties() -> WriterProperties {
    WriterProperties::builder()
        .set_compression(parquet::basic::Compression::SNAPPY)
        .set_write_batch_size(1024 * 1024)  // 1MB row groups
        .set_max_row_group_size(128 * 1024 * 1024)  // 128MB row groups
        .build()
}
```

### Multipart Upload for Large Parquet Files

```rust
async fn upload_large_parquet_file(
    client: &Client,
    bucket: &str,
    key: &str,
    parquet_data: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    const MULTIPART_THRESHOLD: usize = 100 * 1024 * 1024;  // 100 MB
    
    if parquet_data.len() > MULTIPART_THRESHOLD {
        // Use multipart upload
        upload_multipart(client, bucket, key, parquet_data, 10 * 1024 * 1024).await?;
    } else {
        // Use simple upload
        client
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(ByteStream::from(parquet_data))
            .content_type("application/parquet")
            .content_encoding("snappy")
            .send()
            .await?;
    }
    
    Ok(())
}
```

### Metadata for Parquet Files

```rust
async fn upload_parquet_with_metadata(
    client: &Client,
    bucket: &str,
    key: &str,
    parquet_data: Vec<u8>,
    metadata: ParquetMetadata,
) -> Result<(), Box<dyn std::error::Error>> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(parquet_data))
        .content_type("application/parquet")
        .content_encoding("snappy")
        // Store metadata in S3 metadata
        .metadata("row-count", &metadata.row_count.to_string())
        .metadata("column-count", &metadata.column_count.to_string())
        .metadata("schema", &metadata.schema_json)
        .metadata("created-at", &metadata.created_at.to_rfc3339())
        .send()
        .await?;
    
    Ok(())
}
```

### Parallel Processing

```rust
use tokio::task::JoinSet;

async fn upload_multiple_parquet_files_parallel(
    client: Client,
    bucket: &str,
    files: Vec<(String, Vec<u8>)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = JoinSet::new();
    
    // Spawn parallel upload tasks
    for (key, data) in files {
        let client_clone = client.clone();
        let bucket = bucket.to_string();
        
        tasks.spawn(async move {
            client_clone
                .put_object()
                .bucket(&bucket)
                .key(&key)
                .body(ByteStream::from(data))
                .content_type("application/parquet")
                .send()
                .await
        });
    }
    
    // Wait for all uploads
    while let Some(result) = tasks.join_next().await {
        result??;  // Propagate errors
    }
    
    Ok(())
}
```

### Complete Parquet Upload Service

```rust
pub struct ParquetStorageService {
    s3_client: Client,
    bucket: String,
}

impl ParquetStorageService {
    pub async fn upload_parquet_file(
        &self,
        key: &str,
        data: Vec<u8>,
        metadata: ParquetMetadata,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // 1. Validate file size
        if data.len() < 128 * 1024 * 1024 {
            tracing::warn!("Parquet file smaller than recommended 128MB");
        }
        
        // 2. Choose upload strategy
        if data.len() > 100 * 1024 * 1024 {
            // Multipart upload for large files
            self.upload_multipart(key, data, metadata).await
        } else {
            // Simple upload
            self.upload_simple(key, data, metadata).await
        }
    }
    
    async fn upload_simple(
        &self,
        key: &str,
        data: Vec<u8>,
        metadata: ParquetMetadata,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let resp = self.s3_client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(data))
            .content_type("application/parquet")
            .content_encoding("snappy")
            .metadata("row-count", &metadata.row_count.to_string())
            .metadata("schema", &metadata.schema_json)
            .send()
            .await?;
        
        Ok(resp.e_tag().unwrap_or("").to_string())
    }
    
    async fn upload_multipart(
        &self,
        key: &str,
        data: Vec<u8>,
        metadata: ParquetMetadata,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // ... multipart upload implementation ...
        Ok("multipart-etag".to_string())
    }
}
```

---

## 8. Patterns for Custom Blob Sync Server

### Architecture Pattern

```
┌─────────────────────────────────────────────────────────┐
│              HTTP API (Axum)                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │   Upload     │  │   Download   │  │    List      │ │
│  │   Handler    │  │   Handler    │  │   Handler    │ │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘ │
└─────────┼──────────────────┼──────────────────┼─────────┘
          │                  │                  │
          ▼                  ▼                  ▼
┌─────────────────────────────────────────────────────────┐
│           Storage Service Layer                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │   Dedup      │  │   Metadata   │  │   Presigned  │ │
│  │   Check      │  │   Storage    │  │   URLs       │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
└─────────┼──────────────────┼──────────────────┼─────────┘
          │                  │                  │
          ▼                  ▼                  ▼
┌─────────────────────────────────────────────────────────┐
│        AWS SDK S3 Client (Reused)                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │   Put        │  │   Get        │  │   List       │ │
│  │   Object     │  │   Object     │  │   Objects    │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
└─────────┼──────────────────┼──────────────────┼─────────┘
          │                  │                  │
          ▼                  ▼                  ▼
┌─────────────────────────────────────────────────────────┐
│              MinIO (S3-Compatible)                      │
└─────────────────────────────────────────────────────────┘
```

### Key Design Patterns

1. **Client Reuse**: Single S3 client instance shared across requests
2. **Streaming**: Use ByteStream for memory-efficient transfers
3. **Multipart for Large Files**: Automatic multipart for files > 100MB
4. **Deduplication**: Content-addressed storage (hash-based keys)
5. **Metadata Separation**: Complex metadata in database, simple in S3
6. **Error Handling**: Comprehensive retry and error recovery
7. **Progress Tracking**: Stream-based progress for large uploads

---

## 9. Summary and Recommendations

### Key Takeaways

1. **Client Initialization**: Reuse clients, configure custom endpoints for MinIO
2. **Streaming**: Use ByteStream for memory efficiency
3. **Multipart Uploads**: Use for files > 100MB, implement cleanup on failure
4. **Error Handling**: Implement retries, exponential backoff, cleanup
5. **Metadata**: Use S3 metadata for simple data, database for complex queries
6. **Parquet Files**: Optimize size (256-512MB), use Snappy compression, partition by query patterns

### Production Checklist

- [ ] Reuse S3 client instances
- [ ] Configure custom endpoint for MinIO
- [ ] Use `force_path_style(true)` for MinIO
- [ ] Implement retry logic with exponential backoff
- [ ] Use multipart uploads for large files
- [ ] Implement cleanup for failed multipart uploads
- [ ] Add content verification (hash checking)
- [ ] Separate metadata storage for complex queries
- [ ] Optimize Parquet file sizes (256-512MB)
- [ ] Use Snappy compression for Parquet files
- [ ] Implement partitioning strategy
- [ ] Add comprehensive error handling
- [ ] Monitor and log all operations
- [ ] Implement rate limiting/throttling handling

---

## References

- AWS SDK for Rust: https://docs.rs/aws-sdk-s3
- AWS SDK Best Practices: https://docs.aws.amazon.com/sdk-for-rust/latest/dg/best-practices.html
- S3 Performance Optimization: https://docs.aws.amazon.com/AmazonS3/latest/userguide/optimizing-performance.html
- MinIO Documentation: https://min.io/docs

