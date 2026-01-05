# DuckDB Parquet S3 Storage Fetch Flow | MudRock

## Overview

This document describes the complete data flow from frontend to backend for querying Parquet files stored in S3-compatible storage (MinIO) using DuckDB, integrated with Supabase Storage for metadata management. The system now uses standardized well-centric paths through the `storage-resolver` crate for consistent file organization.

## Architecture Components & Their Roles

### 🏪 **MinIO (S3-Compatible Storage)**

**Role**: The actual data warehouse where files are physically stored
**Analogy**: Think of MinIO as a **massive digital warehouse** with infinite shelves. It's like Amazon's fulfillment center - it stores the actual "products" (Parquet files) and can retrieve them quickly when requested. MinIO speaks the same language as Amazon S3, making it compatible with any S3-based tools.

**Why Essential**:

- Provides scalable, durable storage for large datasets
- S3-compatible API allows integration with existing tools
- Self-hosted solution gives you full control over your data

### 📊 **DuckDB (Analytics Engine)**

**Role**: The powerful analytics engine that can directly read and query Parquet files
**Analogy**: Think of DuckDB as a **super-intelligent data analyst** who can instantly read any file format and answer complex questions about the data. It's like having a data scientist who can open any spreadsheet, database, or file and immediately start running queries without needing to import or transform the data first.

**Why Essential**:

- Can directly read Parquet files from S3 without downloading them
- Extremely fast for analytical queries
- Supports SQL, making it familiar to developers
- Embedded database - no separate server needed

### 🗂️ **Storage Resolver (Path Management)**

**Role**: Manages standardized file paths and provides consistent storage operations
**Analogy**: Think of Storage Resolver as a **sophisticated filing system** that knows exactly where every file should be stored and how to retrieve it. It's like having a master librarian who not only catalogs books but also ensures they're organized in a logical, consistent structure that makes them easy to find and access.

**Why Essential**:

- Provides standardized well-centric path construction
- Manages consistent file organization across all storage operations
- Offers unified API for both upload and fetch operations
- Ensures scalability and maintainability of storage structure

### 🗂️ **Supabase Storage (Metadata Manager)**

**Role**: Manages file metadata, permissions, and provides a unified API
**Analogy**: Think of Supabase Storage as a **librarian with a sophisticated catalog system**. While the actual books (Parquet files) are stored in the warehouse (MinIO), the librarian keeps track of what books exist, who can access them, when they were added, and provides a user-friendly interface to browse and request them.

**Why Essential**:

- Provides authentication and authorization
- Manages file metadata (size, creation date, permissions)
- Offers a unified API for file operations
- Integrates with Supabase's ecosystem (auth, database, etc.)

### 📁 **Parquet (Data Format)**

**Role**: The efficient columnar storage format for analytical data
**Analogy**: Think of Parquet as a **highly organized filing cabinet** where each drawer represents a column of data. Instead of storing data row by row (like a traditional spreadsheet), it stores all values for each column together. This makes it incredibly fast to answer questions like "what's the average of column X" because all the values are stored together.

**Why Essential**:

- Columnar storage is optimized for analytics
- Compressed format saves storage space
- Fast query performance
- Industry standard for data lakes and analytics

## Complete Data Flow

### 1. **Frontend Request** (Svelte Component)

```typescript
// User clicks on a Parquet file or requests well data
async function handleFileClick() {
  // Set selected file in global state
  fileState?.setSelectedFile(file);

  // Query using standardized well path structure
  const queryResult = await queryParquetFromWellPath(
    projectId,
    wellId,
    logType,
    100, // limit
  );
}

// Or list all wells in a project
async function listWells() {
  const wells = await listProjectWells(projectId);
  console.log(`Found ${wells.length} wells in project`);
}

// Or get well data summary
async function getWellSummary() {
  const summary = await getWellDataSummary(projectId, wellId);
  console.log(`Well ${wellId} has ${summary.logs.length} log files`);
}
```

### 2. **Tauri Command** (Rust Backend)

```rust
// Updated commands using standardized well paths
#[tauri::command]
async fn query_parquet_from_well_path(
    project_id: String,
    well_id: String,
    log_type: String,
    limit: Option<usize>
) -> Result<ParquetQueryResult, String> {
    // Use WellStorageManager to construct standardized path
    crate::parquet_query::query_parquet_from_well_path(
        &project_id, &well_id, &log_type, limit
    ).await
}

#[tauri::command]
async fn list_project_wells(project_id: String) -> Result<Vec<String>, String> {
    crate::parquet_query::list_project_wells(&project_id).await
}

#[tauri::command]
async fn get_well_data_summary(
    project_id: String,
    well_id: String
) -> Result<WellDataSummary, String> {
    crate::parquet_query::get_well_data_summary(&project_id, &well_id).await
}
```

### 3. **Storage Resolver Integration** (Rust)

```rust
// Create WellStorageManager for standardized path construction
use storage_resolver::{
    well_path::helpers,
    well_storage_manager::WellStorageManager,
    resolvers::minio::MinIOResolver,
    types::{ResolverConfig, Credentials},
};

// Create well storage manager
let well_manager = create_well_storage_manager(project_id)?;

// Generate standardized well path
let well_path = helpers::log_path(
    project_id.to_string(),
    well_id.to_string(),
    log_type.to_string(),
);

// Use the standardized path for DuckDB querying
let s3_uri = format!("s3://{}/{}", well_path.bucket_name(), well_path.s3_key());
```

### 4. **DuckDB S3 Integration** (Rust)

```rust
// Configure DuckDB for S3 access
conn.execute("INSTALL httpfs", [])?;
conn.execute("LOAD httpfs", [])?;

// Set up S3 credentials and endpoint
conn.execute(&format!(
    "CREATE SECRET supabase_storage (
        TYPE S3,
        KEY_ID 'mudrock-storage',
        SECRET 'mudrock-storage-secret-2024',
        ENDPOINT '91.99.166.223:9000',
        REGION 'us-east-1',
        URL_STYLE 'path',
        USE_SSL false
    )"
), [])?;

// Query the Parquet file using standardized path
let query = format!("SELECT * FROM read_parquet('{}') LIMIT {}", s3_uri, limit);
let result = conn.execute(&query, [])?;
```

### 5. **MinIO Storage** (S3-Compatible)

```
s3://project-12345/wells/well_001/logs_gr.parquet
├── File: logs_gr.parquet (15.2KB)
├── Format: Parquet (columnar)
├── Data: 3000 rows of gamma ray measurements
├── Structure: well_name + depth + gr_curve columns
└── Access: Via S3 API with standardized paths

s3://project-12345/wells/well_001/logs_sp.parquet
├── File: logs_sp.parquet (12.8KB)
├── Format: Parquet (columnar)
├── Data: 3000 rows of spontaneous potential measurements
└── Structure: well_name + depth + sp_curve columns
```

## Component Relationships

### **Storage Resolver ↔ MinIO**

- **Relationship**: Path management and storage operations
- **Flow**: Storage Resolver constructs standardized paths and manages MinIO operations
- **Benefit**: Consistent file organization and unified storage interface

### **MinIO ↔ DuckDB**

- **Relationship**: Direct data access
- **Flow**: DuckDB connects to MinIO using S3 API and reads Parquet files directly
- **Benefit**: No need to download files - DuckDB streams data as needed

### **Supabase Storage ↔ MinIO**

- **Relationship**: Metadata management
- **Flow**: Supabase Storage tracks file metadata in PostgreSQL, actual files stored in MinIO
- **Benefit**: Unified API with proper authentication and permissions

### **DuckDB ↔ Parquet**

- **Relationship**: Native format support
- **Flow**: DuckDB has built-in Parquet readers that can efficiently process columnar data
- **Benefit**: Optimized for analytical queries on large datasets

## Why This Architecture?

### **Traditional Approach Problems**:

1. **Download-then-query**: Download entire file → Query locally (slow, memory intensive)
2. **Database import**: Import Parquet to PostgreSQL → Query (slow, storage duplication)
3. **Manual processing**: Extract data → Transform → Load into database (complex, error-prone)

### **Our Solution Benefits**:

1. **Standardized Paths**: Storage Resolver ensures consistent file organization
2. **Direct S3 Querying**: DuckDB reads Parquet files directly from S3 (fast, memory efficient)
3. **Metadata Management**: Supabase Storage handles permissions and file discovery
4. **Unified API**: Single interface for both file management and data querying
5. **Well-centric Organization**: Files organized by project/well/log-type for easy discovery
6. **Self-hosted**: Full control over data and infrastructure

## Data Flow Diagram

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │   Tauri Backend  │    │ Storage Resolver│    │   DuckDB        │
│   (Svelte)      │    │   (Rust)         │    │   (Path Mgmt)   │    │   (Analytics)   │
└─────────────────┘    └──────────────────┘    └─────────────────┘    └─────────────────┘
         │                        │                        │                        │
         │ 1. Query well data     │                        │                        │
         ├───────────────────────►│                        │                        │
         │                        │ 2. Generate path       │                        │
         │                        ├───────────────────────►│                        │
         │                        │                        │ 3. Construct S3 URI    │
         │                        │                        ├───────────────────────►│
         │                        │                        │                        │
         │                        │ 4. Query Parquet       │                        │
         │                        ├───────────────────────►│                        │
         │                        │                        │ 5. S3 API call         │
         │                        │                        │    s3://project/wells/ │
         │                        │                        │                        │
         │                        │ 6. Return results      │                        │
         │◄───────────────────────┤◄───────────────────────┤◄───────────────────────┤
         │                        │                        │                        │
         │ 7. Display content     │                        │                        │
         │                        │                        │                        │
```

## Key Advantages

1. **Standardized Organization**: Well-centric file structure makes data discovery intuitive
2. **Performance**: DuckDB's columnar processing + Parquet's optimized format = lightning-fast queries
3. **Scalability**: MinIO can store petabytes of data, DuckDB can query it efficiently
4. **Cost-effective**: Self-hosted solution with no per-query costs
5. **Flexibility**: Can query any Parquet file without pre-processing
6. **Integration**: Seamlessly works with Supabase's authentication and API ecosystem
7. **Maintainability**: Centralized path management through Storage Resolver

## Use Cases

- **Data Analytics**: Query large datasets stored in Parquet format
- **Data Exploration**: Browse and analyze files without downloading them
- **Real-time Insights**: Get instant answers from your data lake
- **Data Science**: Perform complex analytical queries on structured data
- **Business Intelligence**: Generate reports and dashboards from raw data files
- **Well Data Management**: Organize and query geoscience data by well and log type

## 🔧 Updated Implementation Details

### **Standardized Path Management**

The Parquet fetch process now uses the `storage-resolver` crate for consistent file organization:

```rust
// In parquet_query.rs
use storage_resolver::{
    well_path::helpers,
    well_storage_manager::WellStorageManager,
    resolvers::minio::MinIOResolver,
};

// Create well storage manager
let well_manager = create_well_storage_manager(project_id)?;

// Generate standardized well path
let well_path = helpers::log_path(
    project_id.to_string(),
    well_id.to_string(),
    log_type.to_string(),
);

// Use the standardized path for DuckDB querying
query_parquet_from_s3(&well_path.bucket_name(), &well_path.s3_key(), limit).await
```

### **New Tauri Commands**

The system now provides three new commands for well-centric data access:

1. **`query_parquet_from_well_path`**: Query specific log data by well and log type
2. **`list_project_wells`**: List all wells in a project
3. **`get_well_data_summary`**: Get comprehensive summary of well data

### **Path Structure**

Files are organized using a consistent well-centric structure:

```
project-{project_id}/
├── wells/
│   ├── {well_id}/
│   │   ├── logs_gr.parquet          # Gamma ray logs
│   │   ├── logs_sp.parquet          # Spontaneous potential logs
│   │   ├── logs_res.parquet         # Resistivity logs
│   │   └── logs_density.parquet     # Density logs
│   └── {another_well_id}/
│       └── logs_composite.parquet   # Multiple curve types
```

### **Benefits of Standardized Paths**

- **Consistency**: All files follow the same organizational pattern
- **Discoverability**: Easy to locate files by well and log type
- **Scalability**: Supports unlimited wells and log types per project
- **Maintainability**: Centralized path logic in `storage-resolver` crate
- **Integration**: Seamless integration with upload and fetch operations

This architecture provides a powerful, scalable solution for modern data analytics while maintaining the simplicity and security of a self-hosted environment.
