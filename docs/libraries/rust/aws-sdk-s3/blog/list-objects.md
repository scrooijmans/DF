# Project-based Object Listing in MudRock

## Overview

MudRock implements a project-based file management system where users interact with projects on the frontend, while the backend automatically maps projects to their corresponding MinIO buckets. This follows the database architecture: **Projects → MinIO Buckets**.

## Architecture

- **Frontend**: Users select projects to view their files
- **Backend**: Project IDs are mapped to MinIO bucket names (`project-{project-id}`)
- **Database**: Project metadata and permissions are stored in PostgreSQL
- **Storage**: Files are stored in MinIO with the project's bucket structure

## Project-based Object Listing

### Frontend Implementation

```typescript
// Get projects available for analytics
import {
  getProjectsForAnalytics,
  getProjectFiles,
} from "$lib/services/project-analytics-service";

// Get all projects where user has access
const projects = await getProjectsForAnalytics();

// Get files from a specific project
const files = await getProjectFiles(projectId);
```

### Backend Mapping

The system automatically maps project IDs to MinIO bucket names:

```rust
// Project ID: "123e4567-e89b-12d3-a456-426614174000"
// Bucket Name: "project-123e4567-e89b-12d3-a456-426614174000"
```

### Permission-based Access

- **Owner**: Can view files in any project they own
- **Editor**: Can view files in projects they have edit access to
- **Viewer**: Can view files in projects they have view access to

### Storage Structure

```
project-{project-id}/
├── shared/                    # Project-wide shared data
├── workspaces/
│   ├── {workspace-id}/
│   │   ├── data/             # Workspace-specific datasets
│   │   ├── notebooks/        # Notebook artifacts and outputs
│   │   ├── temp/            # Temporary processing data
│   │   └── exports/         # Workspace exports
│   └── ...
└── exports/                  # Project-level exports
```

### Complete File Listing Flow

1. **Project Selection**: User selects a project from available projects (where they have access)
2. **Project-to-Bucket Mapping**: Project ID is mapped to MinIO bucket name (`project-{project-id}`)
3. **File Listing**: Files are retrieved from the project's MinIO bucket
4. **Permission Filtering**: Only files the user has access to are displayed
5. **UI Updates**: File list is updated with project context and metadata

## Original S3/MinIO Object Listing Example

```
//! Example: List all objects in an S3 bucket that start with a given prefix.
//!
//! Usage (environment credentials or config profile required):
//! RUST_LOG=info cargo run --example list_s3_prefix -- <bucket> <prefix>
//!
//! You can also set AWS_REGION or AWS_DEFAULT_REGION; otherwise, the default
//! region resolution chain applies (env, profile, IMDS, etc.).

use anyhow::{Context, Result};
use aws_sdk_s3::types::Object;
use aws_sdk_s3::Client;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    // Basic argument parsing
    let mut args = std::env::args().skip(1);
    let bucket = args
        .next()
        .context("Missing <bucket> argument. Usage: cargo run --example list_s3_prefix -- <bucket> <prefix>")?;
    let prefix = args
        .next()
        .context("Missing <prefix> argument. Usage: cargo run --example list_s3_prefix -- <bucket> <prefix>")?;

    // Load AWS shared config (resolves region, credentials, etc.)
    let config = aws_config::from_env().behavior_version(aws_config::BehaviorVersion::latest()).load().await;

    // Configure S3 client with custom endpoint if provided
    let mut s3_config_builder = aws_sdk_s3::config::Builder::from(&config);
    if let Ok(endpoint) = std::env::var("S3_ENDPOINT") {
        s3_config_builder = s3_config_builder.endpoint_url(endpoint);
    }
    if let Ok(force_path_style) = std::env::var("S3_FORCE_PATH_STYLE") {
        if force_path_style == "1" {
            s3_config_builder = s3_config_builder.force_path_style(true);
        }
    }
    let s3_config = s3_config_builder.build();
    let client = Client::from_conf(s3_config);

    list_prefix(&client, &bucket, &prefix).await
}

async fn list_prefix(client: &Client, bucket: &str, prefix: &str) -> Result<()> {
    let mut continuation_token: Option<String> = None;
    let mut total_objects: u64 = 0;
    let mut total_bytes: u128 = 0;

    let mut stdout = io::stdout();

    loop {
        let mut req = client
            .list_objects_v2()
            .bucket(bucket)
            .prefix(prefix)
            .max_keys(1000);
        if let Some(token) = &continuation_token {
            req = req.continuation_token(token);
        }

        let resp = req
            .send()
            .await
            .with_context(|| format!("Listing objects (continuation_token={:?})", continuation_token))?;

        for obj in resp.contents() {
            output_object(&mut stdout, obj)?;
            total_objects += 1;
            if let Some(sz) = obj.size() { total_bytes += sz as u128; }
        }

        match resp.is_truncated() {
            Some(true) => {
                continuation_token = resp.next_continuation_token().map(|s| s.to_string());
            }
            _ => break,
        }
    }

    writeln!(stdout, "# SUMMARY objects={} total_bytes={}", total_objects, total_bytes)?;
    Ok(())
}

fn output_object(writer: &mut impl Write, obj: &Object) -> Result<()> {
    let key = obj.key().unwrap_or("<no-key>");
    let size = obj.size().unwrap_or_default();
    let last_modified = obj.last_modified().map(|dt| dt.to_string()).unwrap_or_else(|| "".into());
    writeln!(writer, "{}\t{}\t{}", key, size, last_modified)?;
    Ok(())
}
```
