# Hybrid Storage Architecture Implementation

## Overview

This document describes the implementation of a hybrid storage architecture that combines object storage (Parquet/Iceberg) with PostgreSQL for optimal performance and data management in the MudRock geoscience platform.

## Architecture Components

### 1. Well Catalog Manager (`well-catalog-manager` crate)

A new Rust crate that manages metadata catalogs for wells, curves, well tops, and surveys using Parquet files stored in object storage.

#### Key Features:
- **Fast Lookups**: In-memory catalogs for quick access to metadata
- **Persistent Storage**: Parquet files in object storage for durability
- **Project Isolation**: Data separated by project ID
- **CRUD Operations**: Full create, read, update, delete functionality
- **Advanced Queries**: Geographic, temporal, and statistical queries

#### Modules:
- `catalog_schemas.rs`: Data structures for all catalog types
- `well_catalog_manager.rs`: Core manager logic and persistence
- `well_operations.rs`: Well-specific operations
- `curve_operations.rs`: Curve-specific operations
- `catalog_queries.rs`: Advanced query capabilities

### 2. Catalog Schemas

#### Wells Catalog Entry
```rust
pub struct WellsCatalogEntry {
    pub well_id: Uuid,
    pub well_name: String,
    pub field: Option<String>,
    pub operator: Option<String>,
    pub location: Option<WellLocation>,
    pub spud_date: Option<DateTime<Utc>>,
    pub completion_date: Option<DateTime<Utc>>,
    pub total_depth: Option<f64>,
    pub status: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub project_id: Uuid,
}
```

#### Curves Catalog Entry
```rust
pub struct CurvesCatalogEntry {
    pub curve_id: Uuid,
    pub well_id: Uuid,
    pub curve_name: String,
    pub curve_type: String,
    pub unit: Option<String>,
    pub description: Option<String>,
    pub min_depth: Option<f64>,
    pub max_depth: Option<f64>,
    pub data_points: Option<u64>,
    pub file_path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub project_id: Uuid,
}
```

#### Well Tops Catalog Entry
```rust
pub struct WellTopsCatalogEntry {
    pub top_id: Uuid,
    pub well_id: Uuid,
    pub formation_name: String,
    pub depth: f64,
    pub confidence: Option<String>,
    pub source: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub project_id: Uuid,
}
```

#### Survey Catalog Entry
```rust
pub struct SurveyCatalogEntry {
    pub survey_id: Uuid,
    pub well_id: Uuid,
    pub survey_name: String,
    pub survey_type: String,
    pub start_depth: f64,
    pub end_depth: f64,
    pub data_points: Option<u64>,
    pub file_path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub project_id: Uuid,
}
```

### 3. Data Ingestion System Enhancement

#### File Type Selection
The data ingestion system now supports multiple data types:

- **Well Logs**: LAS files containing well log data
- **Well Data**: CSV files containing well metadata

#### Data Type Configuration
```typescript
export const DATA_INGESTION_TYPES: DataIngestionTypeConfig[] = [
  {
    type: "logs",
    label: "Well Logs",
    description: "Upload LAS files containing well log data",
    acceptedFileTypes: [".las"],
    icon: "📊",
    color: "blue",
    schema: "las_schema"
  },
  {
    type: "wells",
    label: "Well Data",
    description: "Upload CSV files containing well metadata",
    acceptedFileTypes: [".csv"],
    icon: "🏗️",
    color: "green",
    schema: "wells_csv_schema"
  }
];
```

#### Wells CSV Schema
```typescript
export const WELLS_CSV_SCHEMA = {
  required_columns: ["well_name"],
  optional_columns: [
    "field", "operator", "latitude", "longitude", "elevation",
    "spud_date", "completion_date", "total_depth", "status", "coordinate_system"
  ],
  validation_rules: {
    well_name: { required: true, type: "string", minLength: 1, maxLength: 100 },
    latitude: { required: false, type: "number", min: -90, max: 90 },
    longitude: { required: false, type: "number", min: -180, max: 180 },
    // ... more validation rules
  }
};
```

## Implementation Details

### 1. Frontend Components

#### Data Ingestion Main Component
- **File**: `content-data-ingestion.svelte`
- **Features**:
  - Data type selection dropdown
  - Conditional rendering based on selected type
  - LAS file processing for logs
  - CSV file processing for wells
  - Error handling and validation

#### Wells CSV Upload Confirmation
- **File**: `content-data-wells-upload-confirm.svelte`
- **Features**:
  - Data validation and error display
  - AG-Grid integration for data editing
  - Export capabilities (CSV, Excel)
  - Real-time validation feedback

#### Wells CSV AG-Grid Data Table
- **File**: `content-data-wells-upload-confirm-AG-data-table.svelte`
- **Features**:
  - Editable cells with validation
  - Required field indicators
  - Invalid value highlighting
  - Export functionality
  - Real-time data refresh

### 2. Backend Integration

#### Well Catalog Manager Usage
```rust
// Initialize catalog manager
let catalog_manager = WellCatalogManager::new(object_store, project_id);
catalog_manager.initialize().await?;

// Add a new well
let well_entry = WellsCatalogEntry {
    well_id: Uuid::new_v4(),
    well_name: "Well-001".to_string(),
    field: Some("North Field".to_string()),
    operator: Some("OilCorp".to_string()),
    // ... other fields
};
catalog_manager.add_well(well_entry).await?;

// Query wells by field
let wells = catalog_manager.find_wells_by_field("North Field").await;

// Get unique curve names
let curve_names = catalog_manager.list_unique_curve_names().await;
```

#### Data Flow
1. **Upload**: Files uploaded via dropzone
2. **Validation**: Data validated against schema
3. **Confirmation**: User reviews and edits data
4. **Processing**: Data processed and stored
5. **Catalog Update**: Catalogs updated with new metadata

### 3. Storage Strategy

#### Object Storage (Parquet Files)
- **Location**: `catalogs/{project_id}/`
- **Files**:
  - `wells_catalog.parquet`
  - `curves_catalog.parquet`
  - `well_tops_catalog.parquet`
  - `survey_catalog.parquet`

#### PostgreSQL
- **Purpose**: Authentication and project management
- **Tables**: User accounts, project permissions, project metadata
- **Integration**: Project IDs used for data isolation

## Benefits

### 1. Performance
- **Fast Lookups**: In-memory catalogs for quick access
- **Efficient Storage**: Parquet format for compressed, columnar storage
- **Lazy Loading**: Files loaded only when needed

### 2. Scalability
- **Project Isolation**: Data separated by project ID
- **Horizontal Scaling**: Object storage can scale independently
- **Caching**: In-memory catalogs reduce I/O operations

### 3. Flexibility
- **Schema Evolution**: Parquet supports schema changes
- **Query Capabilities**: Advanced filtering and aggregation
- **Export Options**: Multiple export formats (CSV, Excel, Parquet)

### 4. Data Integrity
- **Validation**: Comprehensive data validation rules
- **Consistency**: ACID properties for critical operations
- **Audit Trail**: Timestamps and change tracking

## Usage Examples

### 1. Adding a New Well
```rust
let well = WellsCatalogEntry {
    well_id: Uuid::new_v4(),
    well_name: "Well-001".to_string(),
    field: Some("North Field".to_string()),
    operator: Some("OilCorp".to_string()),
    location: Some(WellLocation {
        latitude: Some(40.7128),
        longitude: Some(-74.0060),
        elevation: Some(100.0),
        coordinate_system: Some("WGS84".to_string()),
    }),
    spud_date: Some(Utc::now()),
    total_depth: Some(3000.0),
    status: Some("active".to_string()),
    created_at: Utc::now(),
    updated_at: Utc::now(),
    project_id: project_id,
};

catalog_manager.add_well(well).await?;
```

### 2. Querying Wells by Geographic Location
```rust
let wells = catalog_manager.find_wells_within_radius(
    40.7128,  // center latitude
    -74.0060, // center longitude
    10.0      // radius in km
).await;
```

### 3. Getting Curve Statistics
```rust
let stats = catalog_manager.get_curve_statistics().await;
println!("Total curves: {}", stats.total_curves);
println!("Unique curve names: {}", stats.unique_names);
println!("Total data points: {}", stats.total_data_points);
```

## Future Enhancements

### 1. Advanced Analytics
- **Machine Learning**: Integration with ML models for data analysis
- **Statistical Functions**: Built-in statistical calculations
- **Visualization**: Chart and graph generation

### 2. Data Versioning
- **Snapshot Management**: Point-in-time data recovery
- **Change Tracking**: Detailed audit logs
- **Rollback Capabilities**: Revert to previous states

### 3. Integration
- **API Endpoints**: RESTful API for external access
- **Webhook Support**: Real-time notifications
- **Third-party Tools**: Integration with industry-standard tools

## Conclusion

The hybrid storage architecture provides a robust, scalable, and performant solution for managing geoscience data in the MudRock platform. By combining the strengths of object storage and PostgreSQL, it offers fast access to metadata while maintaining data integrity and supporting complex queries.

The implementation is modular and extensible, allowing for future enhancements and integrations as the platform evolves.
