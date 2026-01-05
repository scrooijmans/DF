# WellStorageManager

The `WellStorageManager` is a high-level abstraction layer that provides unified well data storage and retrieval operations using the storage resolver system with standardized well path structures. It is part of the `storage-manager` crate, which focuses specifically on well-centric storage operations.

## Purpose

The `WellStorageManager` serves as a **well-centric storage interface** that:

1. **Standardizes file organization** - Enforces a consistent `project-id/wells/well-id/logs/*.parquet` schema
2. **Abstracts storage backends** - Works with any `StorageResolver` implementation (MinIO, S3, Local, etc.)
3. **Provides well-specific operations** - Offers methods tailored to geoscience data workflows
4. **Manages project context** - Associates all operations with a specific project ID

## Key Components

### Core Structure

```rust
pub struct WellStorageManager<R: StorageResolver> {
    resolver: R,           // The underlying storage resolver (MinIO, S3, etc.)
    project_id: String,    // The project this manager operates on
}
```

### Well Data Types Supported

- **Log Data** - Petrophysical measurements (e.g., `logs_composite.parquet`, `logs_gr.parquet`)
- **Well Headers** - Basic well information and metadata
- **Well Tops** - Geological formation boundaries
- **Well Trajectory** - 3D well path coordinates
- **Analysis Results** - Processed analysis outputs

## Primary Operations

### 1. Log Data Management

```rust
// Store log data for a well
async fn store_log_data(&self, well_id: &str, log_type: &str, data: &[u8]) -> Result<WellPath>

// Retrieve log data for a well
async fn get_log_data(&self, well_id: &str, log_type: &str) -> Result<Vec<Url>>

// List all logs for a well
async fn list_well_logs(&self, well_id: &str) -> Result<Vec<WellPath>>
```

### 2. Well Discovery

```rust
// List all wells in the project
async fn list_wells(&self) -> Result<Vec<String>>

// Get comprehensive well data summary
async fn get_well_data(&self, well_id: &str) -> Result<WellDataSummary>
```

### 3. Other Data Types

```rust
// Well headers, tops, trajectory, and analysis results
async fn store_well_header(&self, well_id: &str, data: &[u8]) -> Result<WellPath>
async fn store_well_tops(&self, well_id: &str, data: &[u8]) -> Result<WellPath>
async fn store_well_trajectory(&self, well_id: &str, data: &[u8]) -> Result<WellPath>
async fn store_analysis_results(&self, well_id: &str, analysis_type: &str, data: &[u8]) -> Result<WellPath>
```

## Well Data Summary

The `WellDataSummary` struct provides a comprehensive overview of available data for a well:

```rust
pub struct WellDataSummary {
    pub well_id: String,        // Well identifier
    pub project_id: String,     // Project identifier
    pub logs: Vec<String>,      // Available log files
    pub has_header: bool,       // Whether well header exists
    pub has_tops: bool,         // Whether well tops exist
    pub has_trajectory: bool,   // Whether trajectory exists
}
```

## File Path Standardization

The `WellStorageManager` enforces a consistent file organization pattern:

```
project-{project_id}/
├── wells/
│   ├── {well_id}/
│   │   ├── logs_composite.parquet
│   │   ├── logs_gr.parquet
│   │   ├── logs_neutron.parquet
│   │   ├── header.json
│   │   ├── tops.json
│   │   ├── trajectory.json
│   │   └── analysis_{type}.json
│   └── {another_well_id}/
│       └── ...
```

## Usage Example

```rust
use storage_resolver::well_storage_manager::WellStorageManager;
use storage_resolver::resolvers::MinIOResolver;

// Create a MinIO resolver
let resolver = MinIOResolver::new(
    "http://91.99.166.223:9000",
    "project-abc123",
    "access-key",
    "secret-key"
)?;

// Create well storage manager
let manager = WellStorageManager::new(resolver, "abc123".to_string());

// List all wells in the project
let wells = manager.list_wells().await?;
println!("Found {} wells", wells.len());

// Get data summary for a specific well
let summary = manager.get_well_data("well-001").await?;
println!("Well {} has {} log files", summary.well_id, summary.logs.len());

// List all logs for a well
let logs = manager.list_well_logs("well-001").await?;
for log in logs {
    println!("Log: {}", log.relative_path());
}
```

## Integration with Query Engine

The `WellStorageManager` is tightly integrated with the `QueryEngine` to provide:

1. **File Resolution** - Converts logical well selections to concrete S3 URIs
2. **Path Construction** - Builds standardized paths for DataFusion table registration
3. **Project Context** - Maintains project-specific storage operations

## Benefits

1. **Consistency** - Enforces standardized file organization across all storage operations
2. **Abstraction** - Hides storage backend complexity from higher-level components
3. **Well-Centric** - Provides operations that match geoscience workflows
4. **Flexibility** - Works with any storage backend through the `StorageResolver` trait
5. **Project Isolation** - Keeps data organized by project for multi-tenant scenarios

The `WellStorageManager` is the foundation for all well data operations in the MudRock platform, ensuring consistent, organized, and efficient storage management for geoscience data.
