# MudRock Storage Layer Architecture

This document describes the complete storage layer architecture for MudRock, including the roles and responsibilities of each component.

## 🏗️ Storage Layer Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    MUDROCK STORAGE LAYER                    │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐  │
│  │ storage-resolver│  │ storage-manager │  │   s3-config │  │
│  │ (generic paths) │  │ (well operations)│  │ (S3 config) │  │
│  └─────────────────┘  └─────────────────┘  └─────────────┘  │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────┐ │
│  │           project-data-layout                          │ │
│  │        (centralized manager)                           │ │
│  └─────────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────┐ │
│  │           object-store-adapter                         │ │
│  │        (unified storage interface)                     │ │
│  └─────────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐                  │
│  │ iceberg-tables  │  │geoscience-iceberg│                 │
│  │ (low-level ops) │  │ (domain-specific)│                 │
│  └─────────────────┘  └─────────────────┘                  │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────┐ │
│  │           parquet-log-query-engine                     │ │
│  │        (with Iceberg integration)                      │ │
│  └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## 📦 Component Roles & Responsibilities

### Layer 1: Core Storage Infrastructure

#### `object-store-adapter` - Unified Storage Interface

**Purpose**: Universal storage abstraction layer using the `object_store` crate

**Key Responsibilities**:

- **Unified API**: Single interface for all storage operations across different backends
- **DataFusion Integration**: Native support for DataFusion queries with `s3://` paths
- **Multiple Backend Support**: S3/MinIO, Azure, GCS, in-memory storage
- **Streaming I/O**: Efficient handling of large files with multipart uploads
- **Error Handling**: Comprehensive error types and retry logic
- **Configuration Management**: Integration with existing `s3-config` crate

**Use Cases**:

```rust
// Create adapter for MinIO
let adapter = ObjectStoreAdapter::new_minio(
    "http://91.99.166.223:9000",
    "mudrock-storage",
    "mudrock-storage-secret-2024",
    "project-123"
).await?;

// Basic operations
adapter.put("wells/test/logs.parquet", data).await?;
let data = adapter.get("wells/test/logs.parquet").await?;
let files = adapter.list(Some("wells/")).await?;

// DataFusion integration
let ctx = SessionContext::new();
adapter.register_with_datafusion(&ctx).await?;
```

**Key Benefits**:

- **Universal Remote Control**: One interface for all storage backends
- **Native DataFusion Support**: Direct `s3://` path queries without manual setup
- **Performance**: Streaming I/O and multipart uploads for large files
- **Ecosystem Compatibility**: Works seamlessly with Arrow, Parquet, and Iceberg
- **Future-Proof**: Easy backend switching and new backend support

#### `project-data-layout` - Centralized Project Data Layout Manager

**Purpose**: Single source of truth for project data organization, path construction, and schema definitions

**Key Responsibilities**:

- **Centralized Path Management**: Generate standardized paths for all data types (`project-id/wells/well-id/logs/log_type.parquet`)
- **Schema Definitions**: Define canonical Iceberg schemas for well logs, markers, and wells data
- **Project Structure**: Manage project folder hierarchy and bucket organization
- **Configuration Management**: Centralize project-specific layout configuration

**Use Cases**:

```rust
// Create project layout manager
let layout_manager = ProjectDataLayoutManager::with_default_config("project-123");

// Generate standardized paths
let log_path = layout_manager.well_log_path("well-456", "gr");
// Result: LogPath with s3_key "wells/well-456/logs_gr.parquet"

// Get Iceberg schemas
let logs_schema = layout_manager.well_logs_schema()?;
let table_location = layout_manager.iceberg_table_location("logs");
```

**Key Benefits**:

- **Eliminates Duplication**: Single place for path construction logic
- **Ensures Consistency**: All components use the same path patterns
- **Centralized Configuration**: Easy to modify project structure
- **Schema Management**: Unified schema definitions for Iceberg tables

#### `storage-resolver` - Generic Storage Abstraction

**Purpose**: Provides generic storage abstraction layer and path parsing

**Key Responsibilities**:

- **Generic Path Parsing**: Parse and validate storage paths using centralized layout manager
- **Storage Backend Abstraction**: Abstract storage backends (S3, MinIO, local filesystem)
- **Generic Storage Traits**: Provide common storage operations interface
- **URI Resolution**: Handle URI resolution and path validation

**Note**: Path construction is now handled by `project-data-layout` crate for consistency.

**Use Cases**:

```rust
// Path construction
let well_path = WellPath::new("project-123", "well-456", "logs");
let uri = well_path.to_s3_uri("logs_gr.parquet");
// Result: "s3://bucket/project-123/wells/well-456/logs/logs_gr.parquet"

// Generic storage operations
let resolver = MinIOResolver::new(endpoint, bucket, access_key, secret_key);
let data = resolver.get_object(&uri).await?;
```

#### `storage-manager` - Well-Specific Storage Operations

**Purpose**: High-level well data management and operations

**Key Responsibilities**:

- Manage well-specific storage operations
- Provide well data summaries and metadata
- Handle well file listing and organization
- Integrate with storage resolvers for actual I/O

**Use Cases**:

```rust
// Well data management
let storage_manager = WellStorageManager::new(resolver, "project-123");
let summary = storage_manager.get_well_data_summary("well-456").await?;
let files = storage_manager.list_well_files("well-456", "logs").await?;

// Well-specific operations
storage_manager.upload_well_logs("well-456", "gr", log_data).await?;
```

#### `s3-config` - Centralized S3/MinIO Configuration

**Purpose**: Centralized configuration management for S3-compatible storage

**Key Responsibilities**:

- Manage S3/MinIO credentials and endpoints
- Provide configuration for both AWS SDK and DataFusion
- Handle environment variable loading
- Support multiple storage backends

**Use Cases**:

```rust
// Configuration management
let s3_config = S3Config::from_env()?;
let datafusion_config = DataFusionS3Config::new(s3_config.clone());
let aws_config = AwsS3Config::new(s3_config);
```

### Layer 2: Advanced Table Management

#### `iceberg-tables` - Low-Level Iceberg Operations

**Purpose**: Low-level Apache Iceberg table management and operations

**Key Responsibilities**:

- Manage Iceberg catalogs (Memory, S3)
- Handle table creation, schema evolution
- Provide data reading/writing primitives
- Manage Iceberg metadata and snapshots

**Use Cases**:

```rust
// Low-level Iceberg operations
let catalog_manager = CatalogManager::new(CatalogType::S3 { config }).await?;
let table_manager = IcebergTableManager::new(catalog_manager, table_config);

// Create table
let table_ident = table_manager.create_table(
    &namespace,
    "well_logs",
    schema,
    partition_spec
).await?;

// Write data
table_manager.write_data(&table_ident, record_batches).await?;
```

#### `geoscience-iceberg` - Domain-Specific Iceberg Operations

**Purpose**: Geoscience-specific operations using Iceberg tables

**Key Responsibilities**:

- Define geoscience-specific schemas (well logs, markers, wells)
- Provide domain-specific query operations
- Handle spatial operations and polygon queries
- Integrate with existing storage managers

**Use Cases**:

```rust
// Geoscience-specific operations
let geo_manager = GeoscienceIcebergManager::new("project-123", s3_config).await?;

// Initialize project tables
geo_manager.initialize_project_tables().await?;

// Write well log data
geo_manager.write_well_logs("well-456", "gr", log_data).await?;

// Spatial queries
let wells_in_polygon = geo_manager.find_wells_in_polygon(&polygon).await?;
```

### Layer 3: Query Engine Integration

#### `parquet-log-query-engine` - Unified Query Engine

**Purpose**: Unified query engine supporting both Parquet and Iceberg data sources

**Key Responsibilities**:

- Execute queries across multiple data sources
- Integrate with both storage-manager and geoscience-iceberg
- Provide unified query interface
- Handle complex multi-file queries

**Use Cases**:

```rust
// Unified querying
let query_engine = QueryEngine::new(field_map)?;

// Enable Iceberg integration
query_engine.enable_iceberg_integration("project-123").await?;

// Query across both Parquet and Iceberg
let result = query_engine.execute_query("well_logs", criteria).await?;
```

## 🔄 Key Differences: storage-manager vs geoscience-iceberg

### `storage-manager` - Traditional File-Based Operations

**Focus**: Direct file operations and well data management

**Characteristics**:

- **File-centric**: Works with individual Parquet files
- **Simple operations**: Upload, download, list files
- **Well metadata**: Manages well summaries and file organization
- **Storage abstraction**: Uses storage resolvers for actual I/O
- **Immediate consistency**: Changes are immediately visible

**Example Use Case**:

```rust
// Upload a new LAS file converted to Parquet
let storage_manager = WellStorageManager::new(resolver, "project-123");
storage_manager.upload_well_logs("well-456", "gr", gr_data).await?;

// List all files for a well
let files = storage_manager.list_well_files("well-456", "logs").await?;
// Returns: ["logs_gr.parquet", "logs_rhob.parquet", "logs_nphi.parquet"]
```

### `geoscience-iceberg` - Advanced Table Management

**Focus**: Iceberg table operations and advanced analytics

**Characteristics**:

- **Table-centric**: Works with Iceberg tables containing multiple files
- **Advanced operations**: Schema evolution, time travel, ACID transactions
- **Spatial queries**: Polygon-based well discovery
- **Data versioning**: Historical data access and rollback
- **Optimized analytics**: Columnar storage with advanced indexing

**Example Use Case**:

```rust
// Create a managed Iceberg table for all well logs
let geo_manager = GeoscienceIcebergManager::new("project-123", s3_config).await?;
geo_manager.initialize_project_tables().await?;

// Write data with automatic schema evolution
geo_manager.write_well_logs("well-456", "gr", gr_data).await?;
geo_manager.write_well_logs("well-456", "rhob", rhob_data).await?;

// Advanced spatial query
let wells_in_area = geo_manager.find_wells_in_polygon(&survey_polygon).await?;
// Returns: ["well-456", "well-789", "well-101"]

// Time travel to previous version
let historical_data = geo_manager.time_travel(&table_ident, timestamp, filters).await?;
```

## 🎯 When to Use Which Component

### Use `storage-manager` when:

- ✅ Working with individual Parquet files
- ✅ Simple upload/download operations
- ✅ Well metadata management
- ✅ File organization and listing
- ✅ Immediate data availability needed
- ✅ Simple well data summaries

### Use `geoscience-iceberg` when:

- ✅ Advanced analytics required
- ✅ Schema evolution needed
- ✅ Spatial queries (polygon-based)
- ✅ Data versioning and time travel
- ✅ ACID transactions required
- ✅ Large-scale data management
- ✅ Integration with external analytics tools

## 🔗 Integration Patterns

### Pattern 1: Hybrid Approach

```rust
// Use both for different purposes
let storage_manager = WellStorageManager::new(resolver, project_id);
let geo_manager = GeoscienceIcebergManager::new(project_id, s3_config).await?;

// Simple file operations
storage_manager.upload_well_logs(well_id, curve, data).await?;

// Advanced analytics
geo_manager.write_well_logs(well_id, curve, data).await?;
let spatial_results = geo_manager.find_wells_in_polygon(&polygon).await?;
```

### Pattern 2: Migration Path

```rust
// Start with storage-manager, migrate to geoscience-iceberg
let storage_manager = WellStorageManager::new(resolver, project_id);

// Initial data loading
for well_data in well_data_batch {
    storage_manager.upload_well_logs(well_data.well_id, well_data.curve, well_data.data).await?;
}

// Migrate to Iceberg for advanced operations
let geo_manager = GeoscienceIcebergManager::new(project_id, s3_config).await?;
geo_manager.migrate_from_storage_manager(&storage_manager).await?;
```

### Pattern 3: Query Engine Integration

```rust
// Unified querying across both
let query_engine = QueryEngine::new(field_map)?;
query_engine.enable_iceberg_integration(project_id).await?;

// Queries automatically use the best available data source
let result = query_engine.execute_query("well_logs", criteria).await?;
```

## 📊 Data Flow Architecture

```
User Request
    ↓
Query Engine (parquet-log-query-engine)
    ↓
┌─────────────────┬─────────────────┐
│  Storage Manager │ Geoscience Iceberg │
│  (File-based)   │  (Table-based)    │
└─────────────────┴─────────────────┘
    ↓                     ↓
Storage Resolver    Iceberg Tables
    ↓                     ↓
S3/MinIO Storage    S3/MinIO Storage
    ↓                     ↓
Parquet Files       Iceberg Tables
```

## 🚀 Enhanced Unified Architecture (Current Implementation)

### **New Unified Storage Manager**

The storage layer has been enhanced with a unified architecture that provides consistent interfaces across all data types while maintaining appropriate divergence for different use cases.

#### `mudrock-storage-manager` - Unified Storage Interface

**Purpose**: Centralized storage management with type-safe operations and unified validation

**Key Features**:

- **Unified Upload API**: Single interface for all data types (Wells, Well Markers, LAS)
- **Type-Safe Path Management**: Enum-based path generation with compile-time safety
- **Centralized Validation**: Consistent validation patterns across all upload types
- **Extensible Architecture**: Easy to add new data types and upload strategies

**Core Components**:

```rust
// Unified upload trait
pub trait DataUploader {
    type DataType;
    type ValidationError;

    async fn validate_data(&self, data: &[RecordBatch]) -> Result<(), ValidationError>;
    async fn upload_data(
        &self,
        project_id: Uuid,
        data: Vec<RecordBatch>,
        metadata: Option<serde_json::Value>,
    ) -> Result<UploadResult, UploadError>;
}

// Enhanced path management with enums
pub enum CatalogType {
    Wells,
    WellMarkers,
    Curves,
    Seismic,
}

pub enum WellDataType {
    Logs(String),      // log_type
    Markers,
    Trajectory,
    Header,
}

// Unified storage manager
pub struct MudRockStorageManager {
    adapter: ObjectStoreAdapter,
    layout_manager: ProjectDataLayoutManager,
    validators: HashMap<String, Box<dyn DataValidator<dyn Any>>>,
    uploaders: HashMap<String, Box<dyn DataUploader>>,
}
```

#### **Enhanced Project Data Layout**

**New Features**:

- **Type-Safe Path Generation**: Enum-based path construction
- **Generic Catalog Management**: Unified catalog path generation
- **Well Data Abstraction**: Consistent well-specific data organization

```rust
impl ProjectDataLayoutManager {
    // Generic catalog path generation
    pub fn get_catalog_path(&self, catalog_type: CatalogType) -> String {
        match catalog_type {
            CatalogType::Wells => self.catalog_path("wells_catalog.parquet"),
            CatalogType::WellMarkers => self.catalog_path("well_tops_catalog.parquet"),
            CatalogType::Curves => self.catalog_path("curves_catalog.parquet"),
            CatalogType::Seismic => self.catalog_path("seismic_catalog.parquet"),
        }
    }

    // Generic well data path generation
    pub fn get_well_data_path(&self, well_id: &str, data_type: WellDataType) -> String {
        match data_type {
            WellDataType::Logs(log_type) => self.well_log_path(well_id, &log_type),
            WellDataType::Markers => self.well_markers_path(well_id),
            WellDataType::Trajectory => self.well_trajectory_path(well_id),
            WellDataType::Header => self.well_header_path(well_id),
        }
    }
}
```

#### **Unified Upload Service**

**Purpose**: Single entry point for all data upload operations

**Key Features**:

- **Consistent API**: Same interface for CSV and LAS uploads
- **Automatic Validation**: Built-in validation based on data type
- **Error Handling**: Unified error types and handling
- **Performance Optimization**: Optimized upload strategies per data type

```rust
pub struct UnifiedUploadService {
    storage_manager: MudRockStorageManager,
    validation_registry: ValidationRegistry,
}

impl UnifiedUploadService {
    pub async fn upload_csv_data(
        &self,
        project_id: Uuid,
        data_type: DataIngestionType,
        csv_data: Vec<RecordBatch>,
    ) -> Result<UploadResult, UploadError> {
        // Unified CSV upload logic
    }

    pub async fn upload_las_data(
        &self,
        project_id: Uuid,
        well_id: String,
        las_data: LasFile,
    ) -> Result<UploadResult, UploadError> {
        // Unified LAS upload logic
    }
}
```

### **Benefits of Enhanced Architecture**

1. **Better Convergence**: Unified API across all data types
2. **Type Safety**: Compile-time path validation and data type checking
3. **Maintainability**: Single source of truth for upload logic
4. **Extensibility**: Easy to add new data types and upload strategies
5. **Testing**: Better testability with dependency injection
6. **Performance**: Optimized upload strategies per data type

## 🚀 Future Extensions

### Planned Enhancements

1. **GeoParquet Support**: Enhanced spatial data handling
2. **Real-time Streaming**: Live data ingestion
3. **Machine Learning Integration**: ML pipeline support
4. **Advanced Indexing**: Spatial and temporal indexes
5. **Multi-tenant Support**: Project isolation and security

### Integration Points

- **DataFusion**: Direct integration for SQL queries
- **Arrow**: Native Arrow data format support
- **GeoArrow**: Spatial data operations
- **Petgraph**: DAG-based workflow execution
- **Tauri**: Frontend integration

## 🔧 Configuration Management

### Environment Variables

All S3/MinIO configuration is centralized in the `s3-config` crate:

```bash
# MinIO Configuration
MINIO_ENDPOINT=http://91.99.166.223:9000
MINIO_ACCESS_KEY=mudrock-storage
MINIO_SECRET_KEY=mudrock-storage-secret-2024
MINIO_BUCKET_PREFIX=project
```

### Project Layout Configuration

```rust
use project_data_layout::ProjectLayoutConfig;

let config = ProjectLayoutConfig {
    bucket_prefix: "project".to_string(),
    well_folder_name: "wells".to_string(),
    logs_folder_name: "logs".to_string(),
    markers_folder_name: "markers".to_string(),
    trajectory_folder_name: "trajectory".to_string(),
    header_folder_name: "header".to_string(),
    analysis_folder_name: "analysis".to_string(),
};
```

## 🚀 Iceberg Integration Status

### ✅ **Completed Implementation**

The Iceberg Rust integration has been successfully implemented as outlined in the `iceberg-rs.md` plan:

1. **`iceberg-tables` Crate**: Low-level Iceberg operations with catalog management, table operations, and data reading/writing primitives
2. **`geoscience-iceberg` Crate**: Domain-specific Iceberg operations with geoscience schemas and spatial operations
3. **Query Engine Integration**: Extended `parquet-log-query-engine` to support Iceberg tables
4. **Centralized Architecture**: Integrated with `project-data-layout` for consistent path and schema management

### 🔄 **Current Capabilities**

- **Table Creation**: Create Iceberg tables with geoscience-specific schemas
- **Data Writing**: Write well log data to Iceberg tables
- **Schema Management**: Centralized schema definitions for well logs, markers, and wells
- **Path Management**: Unified path construction for both file-based and table-based storage
- **Query Integration**: DataFusion integration for querying Iceberg tables

### 🎯 **Key Benefits Achieved**

1. **Schema Evolution**: Safe evolution of LAS-to-Parquet schemas
2. **ACID Transactions**: Multi-user safety for concurrent operations
3. **Time Travel**: Query historical data states (foundation implemented)
4. **Better Metadata**: Efficient queries across large datasets
5. **Spatial Operations**: Polygon-based well discovery capabilities
6. **Unified Architecture**: Consistent data organization across storage types

### 📋 **Remaining Considerations from iceberg-rs.md**

The original plan contained several advanced features that are now available as foundation:

- **Data Migration Tools**: Can be built on top of the current architecture
- **Performance Optimization**: Manifest pruning and advanced indexing
- **GeoArrow Integration**: For enhanced spatial data handling
- **Multi-tenant Support**: Project isolation and security features

The current implementation provides a solid foundation for these advanced features while maintaining the existing architecture and adding powerful new capabilities for geoscience data workflows.

## 🧪 Testing and Validation

### Compilation Status

- ✅ `project-data-layout` crate compiles successfully
- ✅ `storage-resolver` crate compiles successfully
- ✅ `storage-manager` crate compiles successfully
- ✅ `geoscience-iceberg` crate compiles successfully
- ✅ `iceberg-tables` crate compiles successfully
- ✅ Tauri application compiles successfully

### Integration Tests

- ✅ LAS upload flow works with new architecture
- ✅ Path construction is consistent across all components
- ✅ Schema definitions are centralized and accessible
- ✅ Iceberg table creation and management
- ✅ Query engine integration with both Parquet and Iceberg

## 🎭 **The Storage Ecosystem Analogy**

Think of MudRock's storage architecture like a **modern data center with multiple specialized departments**:

### 🏢 **The Data Center Building (MinIO/S3)**

- **MinIO/S3** = The **physical building** where all data is stored
- Like a massive warehouse with infinite shelves, organized by project and well
- Provides the raw storage space and basic file operations (put, get, delete)
- **Role**: The foundation - where all data actually lives

### 🚛 **The Universal Delivery Truck (object-store-adapter)**

- **object-store-adapter** = The **universal delivery truck** that can go to any warehouse
- Like a smart truck that knows how to navigate to different storage buildings (S3, Azure, GCS)
- Handles all the logistics of moving data in and out of storage
- **Role**: The transporter - moves data between your app and storage

### 📋 **The Filing System (project-data-layout)**

- **project-data-layout** = The **central filing system** that knows where everything goes
- Like a master directory that says "Project 123, Well 456, GR logs go in shelf A-15"
- Ensures consistent organization across all projects
- **Role**: The organizer - tells everyone where to put and find data

### 🏗️ **The Construction Crew (storage-manager)**

- **storage-manager** = The **construction crew** that builds and manages wells
- Like workers who know how to organize files for specific wells
- Handles well-specific operations and metadata
- **Role**: The specialist - knows how to work with well data specifically

### 🏛️ **The Government Registry (Iceberg)**

- **iceberg-tables** = The **government registry** that keeps official records
- Like a city hall that maintains official records with version history
- Provides ACID transactions, schema evolution, and time travel
- **Role**: The official record keeper - maintains authoritative data with history

### 🧑‍💼 **The Specialized Department (geoscience-iceberg)**

- **geoscience-iceberg** = The **geoscience department** in the government
- Like a specialized branch that knows about wells, spatial data, and geological operations
- Handles domain-specific operations like spatial queries
- **Role**: The expert - knows how to work with geological data specifically

### 🔍 **The Research Library (parquet-log-query-engine)**

- **parquet-log-query-engine** = The **research library** where scientists query data
- Like a library where you can ask complex questions across all the data
- Provides unified querying across both file-based and table-based storage
- **Role**: The researcher - answers complex questions about the data

### 🎛️ **The Control Panel (s3-config)**

- **s3-config** = The **control panel** that manages all the settings
- Like the master control room that knows all the passwords and connection details
- Centralizes configuration for all storage operations
- **Role**: The administrator - manages all the technical settings

## 🔄 **How They Work Together**

```
1. User wants to query well data
   ↓
2. parquet-log-query-engine (Research Library) receives the request
   ↓
3. project-data-layout (Filing System) determines where the data should be
   ↓
4. object-store-adapter (Delivery Truck) goes to MinIO/S3 (Warehouse)
   ↓
5. Data comes back through the same path
   ↓
6. Results are processed and returned to user
```

**For Advanced Operations:**

```
1. User wants spatial analysis
   ↓
2. geoscience-iceberg (Geoscience Department) handles the request
   ↓
3. iceberg-tables (Government Registry) provides versioned, ACID data
   ↓
4. object-store-adapter (Delivery Truck) retrieves data from MinIO/S3 (Warehouse)
   ↓
5. Spatial analysis is performed and results returned
```

## 🎯 **Why This Architecture Works**

- **Separation of Concerns**: Each component has a specific job
- **Flexibility**: Can use file-based (storage-manager) or table-based (iceberg) approaches
- **Scalability**: Easy to add new storage backends or query engines
- **Consistency**: All components use the same filing system and delivery truck
- **Performance**: Direct data access without unnecessary layers
- **Future-Proof**: Easy to upgrade individual components without affecting others

This architecture provides a flexible, scalable foundation for geoscience data management while maintaining clear separation of concerns and enabling future enhancements.
