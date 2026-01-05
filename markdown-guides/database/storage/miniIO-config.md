# MinIO Configuration Guide for MudRock

This guide explains how to configure and connect to MinIO (S3-compatible storage) in the MudRock geoscience data platform.

## Overview

MudRock uses MinIO as its primary object storage backend for storing Parquet files, LAS data, and other geoscience datasets. The platform is designed to work with both local MinIO instances and remote MinIO servers.

## Current Configuration

Based on the current MudRock setup, the platform is configured to use:

- **Remote MinIO Server**: `http://91.99.166.223:9000`
- **Access Key**: `mudrock-storage`
- **Secret Key**: `mudrock-storage-secret-2024`
- **Region**: `us-east-1`

## Environment Variables

The MudRock platform automatically loads MinIO configuration from environment variables. Set these in your environment or `.env` file:

```bash
# Primary MinIO Configuration
MINIO_ENDPOINT=http://91.99.166.223:9000
MINIO_ACCESS_KEY=mudrock-storage
MINIO_SECRET_KEY=mudrock-storage-secret-2024
MINIO_REGION=us-east-1

# Alternative AWS Configuration (Fallback)
AWS_ENDPOINT_URL=http://91.99.166.223:9000
AWS_ACCESS_KEY_ID=mudrock-storage
AWS_SECRET_ACCESS_KEY=mudrock-storage-secret-2024
AWS_DEFAULT_REGION=us-east-1
```

## Storage Path Structure

MudRock uses a standardized well-centric storage organization:

```
project-{project_id}/
├── wells/
│   ├── {well_id}/
│   │   ├── logs_composite.parquet
│   │   ├── logs_gr.parquet
│   │   ├── logs_res.parquet
│   │   ├── header.parquet
│   │   ├── tops.parquet
│   │   └── trajectory.parquet
│   └── {another_well_id}/
│       └── logs_composite.parquet
└── shared/
    └── project_metadata.parquet
```

### Example Paths

- **Project**: `3bd95caa-6360-4d91-8d40-5ef6c659c93a`
- **Well**: `6038-187`
- **Log Type**: `composite`
- **Full S3 URI**: `s3://project-3bd95caa-6360-4d91-8d40-5ef6c659c93a/3bd95caa-6360-4d91-8d40-5ef6c659c93a/wells/6038-187/logs_composite.parquet`

## Integration Components

### 1. S3 Configuration Crate (`s3-config`)

The centralized S3 configuration is managed by the `s3-config` crate:

```rust
use s3_config::{S3Config, DataFusionS3Config, AwsS3Config};

// Load configuration from environment
let s3_config = DataFusionS3Config::from_env()?;

// Configure DataFusion session
let ctx = s3_config.configure_session()?;

// Register Parquet table from S3
s3_config.register_parquet_table(&ctx, "well_logs", s3_uri).await?;
```

### 2. Storage Resolver (`storage-resolver`)

The storage resolver provides standardized path management:

```rust
use storage_resolver::well_path::helpers;
use storage_resolver::well_storage_manager::WellStorageManager;

// Create standardized well path
let log_path = helpers::log_path(
    "project-123".to_string(),
    "well-001".to_string(),
    "composite".to_string(),
);

// Use with storage operations
let s3_uri = format!("s3://{}/{}", log_path.bucket_name(), log_path.s3_key());
```

### 3. Query Engine (`parquet-log-query-engine`)

The query engine integrates with MinIO for data processing:

```rust
use parquet_log_query_engine::execute_depth_slice_direct;

// Execute query with automatic MinIO configuration
let batches = execute_depth_slice_direct(
    "project-123",           // project_id
    "well-001",              // well_id
    "composite",             // log_type
    0.0,                     // min_depth
    13.0,                    // max_depth
    None,                    // curve_filters
    Some(100),               // limit
).await?;
```

## Connection Testing

### Test MinIO Connectivity

```bash
# Test basic connectivity
curl http://91.99.166.223:9000/minio/health/live

# Test with MudRock examples
cargo run --example simple_minio_test --package parquet-log-query-engine
cargo run --example s3_integration_example --package parquet-log-query-engine
```

### Verify Bucket Access

```bash
# List buckets (if MinIO client is installed)
mc ls minio/

# List objects in a specific bucket
mc ls minio/project-3bd95caa-6360-4d91-8d40-5ef6c659c93a/
```

## Troubleshooting

### Common Issues

1. **Connection Refused**
   - Verify MinIO server is running
   - Check network connectivity to `91.99.166.223:9000`
   - Ensure firewall allows port 9000

2. **Authentication Failed**
   - Verify access key and secret key are correct
   - Check that credentials match MinIO server configuration

3. **Bucket Not Found**
   - Ensure project bucket exists in MinIO
   - Check bucket naming convention: `project-{project_id}`

4. **Object Store Registration Failed**
   - Verify DataFusion S3 configuration
   - Check that object store is properly registered for the bucket

### Debug Commands

```bash
# Check environment variables
echo $MINIO_ENDPOINT
echo $MINIO_ACCESS_KEY

# Test with curl
curl -I http://91.99.166.223:9000/

# Run MudRock tests
cargo test --package parquet-log-query-engine test_depth_slice_direct -- --nocapture
```

## Configuration Files

### Tauri Application

The Tauri application uses MinIO configuration in:

- `src-tauri/src/project_management/project_manager.rs` - MinIO client creation
- `src-tauri/src/parquet_query.rs` - Query operations
- `src-tauri/src/las_processing/las_data_processor.rs` - LAS upload operations

### Query Engine

The query engine configuration is in:

- `crates/query-engine/parquet-log-query-engine/src/storage/s3_config.rs` - S3 configuration
- `crates/storage/s3-config/` - Centralized S3 configuration crate

## Security Considerations

1. **Credentials**: Store MinIO credentials securely, never commit them to version control
2. **Network**: Use HTTPS in production environments
3. **Access Control**: Implement proper bucket policies and IAM roles
4. **Encryption**: Enable server-side encryption for sensitive data

## Production Deployment

For production deployment:

1. **Use HTTPS**: Configure MinIO with SSL certificates
2. **Environment Variables**: Set production credentials securely
3. **Monitoring**: Implement logging and monitoring for MinIO operations
4. **Backup**: Set up regular backups of critical data
5. **Scaling**: Consider MinIO clustering for high availability

## Example Usage

### Complete Workflow

```rust
use s3_config::DataFusionS3Config;
use storage_resolver::well_path::helpers;
use parquet_log_query_engine::execute_depth_slice_direct;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Configure S3/MinIO connection
    let s3_config = DataFusionS3Config::from_env()?;

    // 2. Create standardized path
    let log_path = helpers::log_path(
        "3bd95caa-6360-4d91-8d40-5ef6c659c93a".to_string(),
        "6038-187".to_string(),
        "composite".to_string(),
    );

    // 3. Execute query
    let batches = execute_depth_slice_direct(
        &log_path.bucket_name(),
        &log_path.s3_key(),
        0.0,
        13.0,
        None,
        Some(100),
    ).await?;

    // 4. Process results
    println!("Retrieved {} batches", batches.len());

    Ok(())
}
```

This configuration ensures that MudRock can seamlessly connect to MinIO, manage well-centric data organization, and execute queries against Parquet files stored in the object storage system.
