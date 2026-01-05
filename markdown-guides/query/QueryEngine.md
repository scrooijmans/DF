# QueryEngine

The `QueryEngine` is a **DataFusion-based query execution engine** specifically designed for petrophysical log data analysis. It provides a high-level interface for executing complex queries on Parquet files stored in S3/MinIO using Apache Arrow's columnar data format.

## Purpose

The `QueryEngine` serves as the **primary query execution layer** that:

1. **Executes DataFusion queries** - Runs SQL-like queries on Arrow data using DataFusion
2. **Manages S3/MinIO integration** - Handles object store registration and authentication
3. **Provides well-centric operations** - Offers domain-specific query methods for geoscience data
4. **Abstracts query complexity** - Simplifies complex multi-file, multi-filter operations

## Architecture

### Core Components

```rust
pub struct QueryEngine {
    pub ctx: SessionContext,                    // DataFusion execution context
    pub field_map: FieldMap,                   // Maps logical to physical column names
    pub storage_manager: WellStorageManager<MinIOResolver>, // File resolution
    pub s3_config: DataFusionS3Config,         // S3/MinIO configuration
}
```

### DataFusion Integration

**Yes, the QueryEngine is DataFusion-specific** for executing DataFrames on Arrow data. Here's how it works:

1. **DataFusion SessionContext** - The core execution engine for SQL queries
2. **Arrow DataFrames** - Columnar data representation for efficient processing
3. **S3 Object Store Registration** - Enables direct querying of Parquet files from S3/MinIO
4. **SQL Query Execution** - Converts high-level criteria to DataFusion SQL operations

## Key Operations

### 1. Table Registration

```rust
// Register Parquet files as DataFusion tables
async fn register_parquet_table(&self, table_name: &str, file_paths: &[String]) -> Result<()>

// Register wells as tables using WellStorageManager
async fn register_wells_table(&self, table_name: &str, project_id: &str, well_ids: &[String], log_type: &str) -> Result<()>
```

### 2. Query Execution

```rust
// Execute queries with criteria (depth ranges, curve filters, etc.)
async fn execute_query(&self, table_name: &str, criteria: QueryCriteria) -> Result<QueryResult>

// Execute aggregation queries (min/max, averages, etc.)
async fn execute_aggregation_query(&self, table_name: &str, criteria: QueryCriteria, group_by: Vec<String>, aggregations: Vec<AggregationExpr>) -> Result<QueryResult>
```

### 3. Context Access

```rust
// Get the underlying DataFusion context for direct operations
fn context(&self) -> &SessionContext

// Access field mapping for column name resolution
fn field_map(&self) -> &FieldMap

// Access storage manager for file operations
fn storage_manager(&self) -> &WellStorageManager<MinIOResolver>
```

## DataFusion Query Flow

The QueryEngine follows this execution pattern:

```
1. Input: QueryCriteria (wells, depth ranges, curve filters)
   ↓
2. File Resolution: WellStorageManager → S3 URIs
   ↓
3. Table Registration: DataFusionS3Config → Register Parquet tables
   ↓
4. Query Building: QueryBuilder → DataFusion SQL
   ↓
5. Execution: SessionContext → Arrow RecordBatches
   ↓
6. Output: QueryResult (data + execution stats)
```

## Query Types Supported

### 1. Depth Filtering

```rust
let criteria = QueryCriteria::new()
    .with_wells(vec!["well-001".to_string()])
    .with_depth_range(DepthRange::new(0.0, 100.0));
```

### 2. Curve Filtering

```rust
let criteria = QueryCriteria::new()
    .with_curve_filters(vec![
        CurveFilter::new("GR", 50.0, 150.0),
        CurveFilter::new("RHOB", 2.0, 3.0)
    ]);
```

### 3. Multi-Well Queries

```rust
let criteria = QueryCriteria::new()
    .with_wells(vec!["well-001".to_string(), "well-002".to_string()])
    .with_log_types(vec!["composite".to_string(), "gr".to_string()]);
```

## Field Mapping

The `FieldMap` handles the translation between logical field names and physical column names:

```rust
// Example: LAS-style to wide-format mapping
let field_map = FieldMap::new(
    "well_name",    // well → well_name
    "curve_name",   // curve → curve_name
    "DEPT",         // depth → DEPT
    "log_value"     // value → log_value
);
```

## S3/MinIO Integration

The QueryEngine handles S3/MinIO operations through:

1. **DataFusionS3Config** - Configures S3 object store registration
2. **Bucket-specific registration** - Registers object stores for each project bucket
3. **Authentication** - Handles MinIO credentials and endpoint configuration
4. **Path construction** - Builds proper S3 URIs for Parquet files

## Usage Example

```rust
use parquet_log_query_engine::QueryEngine;
use parquet_log_query_engine::catalog::field_map::FieldMap;
use parquet_log_query_engine::query::criteria::{QueryCriteria, DepthRange};

// Create field mapping
let field_map = FieldMap::new("well_name", "curve_name", "DEPT", "log_value");

// Create query engine
let engine = QueryEngine::new(field_map)?;

// Register wells as a table
engine.register_wells_table(
    "well_logs",
    "project-abc123",
    &["well-001".to_string(), "well-002".to_string()],
    "composite"
).await?;

// Create query criteria
let criteria = QueryCriteria::new()
    .with_wells(vec!["well-001".to_string()])
    .with_depth_range(DepthRange::new(0.0, 100.0));

// Execute query
let result = engine.execute_query("well_logs", criteria).await?;
println!("Processed {} records", result.stats.records_processed);
```

## Domain-Specific Query Executors

The QueryEngine integrates with specialized query executors:

### 1. MultiFileQueryExecutor

- Handles queries across multiple Parquet files
- Manages different schemas and column mappings
- Applies filters to each file independently

### 2. UnifiedSchemaQueryExecutor

- Combines data from files with different schemas
- Creates unified schemas with NULL values for missing columns
- Enables cross-well analysis

## Performance Features

1. **Columnar Processing** - Leverages Arrow's columnar format for efficient operations
2. **Predicate Pushdown** - Filters applied at the storage level
3. **Parallel Execution** - DataFusion's parallel query execution
4. **Memory Management** - Configurable memory limits and optimization
5. **S3 Direct Access** - No local file copying, direct S3 querying

## Integration with Storage Layer

The QueryEngine works seamlessly with:

- **WellStorageManager** - For file resolution and path construction (from `storage-manager` crate)
- **S3Config** - For MinIO/S3 authentication and configuration
- **StorageResolver** - For abstracting storage backends

## Benefits

1. **SQL-like Interface** - Familiar query syntax for complex operations
2. **High Performance** - Arrow columnar format and DataFusion optimization
3. **S3 Native** - Direct querying without data movement
4. **Well-Centric** - Domain-specific operations for geoscience workflows
5. **Scalable** - Handles large datasets and multi-file operations efficiently

The `QueryEngine` is the core execution engine that makes the MudRock platform's query capabilities possible, providing a powerful and efficient way to analyze petrophysical log data at scale.
