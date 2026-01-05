# Trajectory CSV Upload

## Overview

The trajectory CSV upload functionality allows users to upload well trajectory data in CSV format. This data represents the 3D path of wells through the subsurface, including X/Y coordinates, True Vertical Depth (TVD), and Measured Depth (MD).

## Architecture Overview

```
Frontend (SvelteKit)
    ↓ Trajectory CSV File Drop
ContentDataIngestionDropzone
    ↓ parseTrajectoryCsvForConfirmation()
Tauri Command (Rust Backend)
    ↓ CSV Parsing & Validation
WellTrajectoryCsvImporter (dedicated trajectory importer)
    ↓ Arrow RecordBatch Processing
RecordBatch → AG-Grid Conversion
    ↓ User Confirmation UI
ContentDataIngestionWellSelection (well selection component)
    ↓ Selected Well Information
    - If "New Well": well_id = "new", well_name from filename
    - If existing well: well_id = database ID, well_name from selection
ContentDataCsvUploadConfirm (reused component)
    ↓ AG-Grid Display & Editing
ContentDataCsvUploadConfirmAGDataTable (reused component)
    ↓ User Confirms Data + Well Selection Info
write_trajectory_catalog_command()
    ↓ Well ID Resolution & Database Integration
    - If "New Well": WellCreator::create_well() → INSERT into Postgres wells table
    - Returns database ID (e.g., 25) from Postgres
    - If existing well: Use selected database ID directly
    ↓ Well ID Available (database ID for all scenarios)
    ↓ OpenDAL Storage Processing
OpenDALStorageAdapter (unified data access layer)
    ↓ Type-Safe Path Construction
ProjectDataLayoutManager (type-safe paths)
    ↓ MinIO Storage via OpenDAL
Project Bucket (project-{id}/wells/{database_id}/trajectory/trajectory.parquet)
    - Example: project-{id}/wells/25/trajectory/trajectory.parquet (F02-1-track)
    - Example: project-{id}/wells/21/trajectory/trajectory.parquet (My-Well-1)
    ↓ Success Notification
```

## Schema Definition

### Required Fields

- **X**: X coordinate (easting) in meters (number)
- **Y**: Y coordinate (northing) in meters (number)
- **TVD**: True Vertical Depth in meters (number)
- **MD**: Measured Depth in meters (number)

**Note**: The trajectory data does not include a `well_name` field in the AG-Grid display. The well name is extracted from the filename during processing but not displayed as a column.

### Validation Rules

```typescript
{
  X: {
    required: true,
    type: "number",
    min: -10000000,
    max: 10000000,
  },
  Y: {
    required: true,
    type: "number",
    min: -10000000,
    max: 10000000,
  },
  TVD: {
    required: true,
    type: "number",
    min: -10000,
    max: 50000,
  },
  MD: {
    required: true,
    type: "number",
    min: 0,
    max: 50000,
  },
}
```

**Note**: The validation ranges for X and Y coordinates have been expanded to -10,000,000 to 10,000,000 meters to accommodate larger UTM coordinate values.

## Storage Level

**Trajectories are stored at the WELL level**, not the project level. This means:

- **Storage Path**: `wells/{well_id}/trajectory/trajectory.parquet`
- **Scope**: Each well's trajectory data is stored in its own Parquet file
- **Structure**: The Parquet file contains trajectory points for a specific well
- **Well Identification**: The well is identified by the database ID in the storage path

This approach allows for:

- Well-specific data organization and management
- Efficient querying of individual well trajectory data
- Consistent with LAS file storage (well-specific)
- Simplified data management at the well level
- Better data isolation and security

## Frontend Implementation

### File Type Detection

The system automatically detects trajectory files based on filename patterns:

```typescript
export function detectDataIngestionType(file: File): DataIngestionType | null {
  const fileExtension = "." + file.name.split(".").pop()?.toLowerCase();

  if (fileExtension === ".las") {
    return "logs";
  } else if (fileExtension === ".csv") {
    const fileName = file.name.toLowerCase();
    if (fileName.includes("track") || fileName.includes("trajectory")) {
      return "trajectories";
    } else if (fileName.includes("marker") || fileName.includes("tops")) {
      return "well-markers";
    } else {
      return "wells";
    }
  }

  return null;
}
```

### Data Mapping

The confirmation component maps CSV data to trajectory format, preserving the original uppercase field names:

```typescript
// For trajectory data, preserve the original field names (X, Y, TVD, MD)
if (ingestionState?.selectedDataType === "trajectories") {
  return {
    X: row.X || 0,
    Y: row.Y || 0,
    TVD: row.TVD || 0,
    MD: row.MD || 0,
    // Keep other fields for compatibility
    well_name: row.well_name || row.Well || "",
    // ... other fields
  };
}
```

### Column Definitions

The AG-Grid displays trajectory data with 4 columns (no Well Name column):

```typescript
// Trajectory schema: X, Y, TVD, MD (no well_name column)
[
  {
    field: "X",
    headerName: "X (m)",
    editable: true,
    filter: "agNumberColumnFilter",
    sortable: true,
    resizable: true,
    flex: 1,
    minWidth: 120,
    type: "numericColumn",
  },
  {
    field: "Y",
    headerName: "Y (m)",
    editable: true,
    filter: "agNumberColumnFilter",
    sortable: true,
    resizable: true,
    flex: 1,
    minWidth: 120,
    type: "numericColumn",
  },
  {
    field: "TVD",
    headerName: "TVD (m)",
    editable: true,
    filter: "agNumberColumnFilter",
    sortable: true,
    resizable: true,
    flex: 1,
    minWidth: 120,
    type: "numericColumn",
  },
  {
    field: "MD",
    headerName: "MD (m)",
    editable: true,
    filter: "agNumberColumnFilter",
    sortable: true,
    resizable: true,
    flex: 1,
    minWidth: 120,
    type: "numericColumn",
  },
];
```

**Note**: The trajectory AG-Grid does not include a "Well Name" column. The well name is extracted from the filename during processing but not displayed in the table.

## Backend Implementation

### Tauri Commands

#### Parse Command

```rust
#[tauri::command]
async fn parse_trajectory_csv_for_confirmation(
    csv_content: String,
    file_name: String,
) -> Result<TrajectoryCsvImportResult, String> {
    println!("🔍 [Tauri] Parsing Trajectory CSV file: {} using new modular crates", file_name);

    let importer = WellTrajectoryCsvImporter::new();
    match importer.import_trajectory_csv_for_confirmation(&csv_content, &file_name) {
        Ok(result) => {
            println!("✅ Successfully parsed Trajectory CSV file: {} with {} rows", file_name, result.total_rows);
            Ok(result)
        }
        Err(e) => {
            println!("❌ Failed to parse Trajectory CSV file: {} - {}", file_name, e);
            Err(format!("Failed to parse Trajectory CSV file: {}", e))
        }
    }
}
```

#### Upload Command

```rust
#[tauri::command]
async fn write_trajectory_catalog_command(
    project_id: String,
    well_id: String,
    rows: Vec<TrajectoryCsvRow>,
    file_name: Option<String>,
) -> Result<(), String> {
    println!("🔄 [Tauri] write_trajectory_catalog_command called with project_id: {}, well_id: {}, rows: {}, file_name: {:?}", project_id, well_id, rows.len(), file_name);

    // Step 1: If well_id is "new", create a new well in the database first
    let final_well_id = if well_id == "new" {
        println!("📋 [TrajectoryCommand] 'New Well' selected - creating new well in database");

        // Extract well name from filename (without extension) or CSV data
        let well_name = if let Some(filename) = file_name {
            // Remove file extension from filename
            filename.split('.').next().unwrap_or(&filename).to_string()
        } else {
            // Fallback to CSV data or default
            rows.first()
                .and_then(|r| {
                    if !r.well_name.is_empty() && !r.well_name.contains('.') {
                        Some(r.well_name.clone())
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| "Unknown Well".to_string())
        };

        println!("📋 [TrajectoryCommand] Creating well with name: {}", well_name);

        let well_creator = WellCreator::new();
        let create_request = CreateWellRequest {
            name: well_name,
            project_id: project_id.clone(),
        };

        match well_creator.create_well(create_request).await {
            Ok(created_well) => {
                println!("✅ [TrajectoryCommand] Created new well with ID: {} and name: {}", created_well.id, created_well.name);
                created_well.id.to_string()
            }
            Err(e) => {
                println!("⚠️ [TrajectoryCommand] Failed to create well in database: {}. Using 'new' for path instead.", e);
                "new".to_string()
            }
        }
    } else {
        println!("📋 [TrajectoryCommand] Using existing well ID: {}", well_id);
        well_id
    };

    println!("📋 [TrajectoryCommand] Final well ID for storage: {}", final_well_id);

    // Create project-specific bucket first
    let project_bucket = format!("project-{}", project_id);
    println!("🔄 [Tauri] Creating project bucket: {}", project_bucket);

    // Create the bucket if it doesn't exist
    let csv_upload_manager = CsvUploadManager::new();
    if let Err(e) = csv_upload_manager.create_bucket(&project_bucket).await {
        println!("⚠️ [Tauri] Bucket creation failed (may already exist): {}", e);
    } else {
        println!("✅ [Tauri] Project bucket created successfully: {}", project_bucket);
    }

    // Create OpenDAL storage adapter with the correct project bucket
    let opendal_adapter = match opendal_storage_adapter::StorageAdapterFactory::create_minio(
        "http://91.99.166.223:9000",
        &project_bucket,
        "mudrock-storage",
        "mudrock-storage-secret-2024",
    ) {
        Ok(a) => {
            println!("✅ [Tauri] OpenDAL storage adapter created successfully for bucket: {}", project_bucket);
            a
        },
        Err(e) => {
            println!("❌ [Tauri] Failed to create OpenDAL storage adapter: {}", e);
            return Err(format!("Failed to create OpenDAL storage adapter: {}", e));
        },
    };

    // Use OpenDAL adapter for storage operations
    let adapter = opendal_adapter;

    println!("🔄 [Tauri] Calling write_trajectory_for_well with {} rows for well {}", rows.len(), final_well_id);
    match write_trajectory_for_well(&adapter, project_id.clone(), final_well_id, rows).await {
        Ok(_) => {
            println!("✅ [Tauri] write_trajectory_for_well completed successfully");
            Ok(())
        },
        Err(e) => {
            println!("❌ [Tauri] write_trajectory_for_well failed: {}", e);
            Err(e)
        }
    }
}
```

### Trajectory Writer

```rust
pub async fn write_trajectory_for_well(
    adapter: &OpenDALStorageAdapter,
    project_id: String,
    well_id: String,
    rows: Vec<TrajectoryCsvRow>,
) -> Result<(), String> {
    println!("🔄 [TrajectoryWriter] Starting write_trajectory_for_well with {} rows for well {} in project {}", rows.len(), well_id, project_id);

    // Build Arrow columns
    let well_name: ArrayRef = Arc::new(StringArray::from(
        rows.iter().map(|r| r.well_name.as_str()).collect::<Vec<_>>()
    ));
    let x: ArrayRef = Arc::new(Float64Array::from(
        rows.iter().map(|r| r.x).collect::<Vec<_>>()
    ));
    let y: ArrayRef = Arc::new(Float64Array::from(
        rows.iter().map(|r| r.y).collect::<Vec<_>>()
    ));
    let tvd: ArrayRef = Arc::new(Float64Array::from(
        rows.iter().map(|r| r.tvd).collect::<Vec<_>>()
    ));
    let md: ArrayRef = Arc::new(Float64Array::from(
        rows.iter().map(|r| r.md).collect::<Vec<_>>()
    ));

    // Define Schema
    let schema = Arc::new(Schema::new(vec![
        Field::new("well_name", DataType::Utf8, false),
        Field::new("x", DataType::Float64, false),
        Field::new("y", DataType::Float64, false),
        Field::new("tvd", DataType::Float64, false),
        Field::new("md", DataType::Float64, false),
    ]));

    let batch = RecordBatch::try_new(schema.clone(), vec![well_name, x, y, tvd, md])?;
    println!("🔄 [TrajectoryWriter] Created RecordBatch with {} rows", batch.num_rows());

    // Convert to Parquet
    let mut buffer = Vec::new();
    {
        let mut writer = ArrowWriter::try_new(&mut buffer, batch.schema(), None)?;
        writer.write(&batch)?;
        writer.finish()?;
    }
    println!("🔄 [TrajectoryWriter] Parquet size: {} bytes", buffer.len());

    // Upload to well-specific path
    let path = format!("wells/{}/trajectory/trajectory.parquet", well_id);
    println!("🔄 [TrajectoryWriter] Uploading to path: {}", path);

    adapter.write(&path, buffer).await?;
    println!("✅ [TrajectoryWriter] Successfully uploaded trajectory to {}", path);

    Ok(())
}
```

## Storage Layer Architecture

### OpenDAL Storage Processing

The trajectory upload uses OpenDAL for unified storage operations:

```rust
// In wells_processing/trajectory_catalog_writer.rs
pub async fn write_trajectory_for_well(
    adapter: &OpenDALStorageAdapter,
    project_id: String,
    well_id: String,
    rows: Vec<TrajectoryCsvRow>,
) -> Result<(), String> {
    // Convert CSV rows to Arrow RecordBatch
    let batch = create_trajectory_record_batch(rows)?;

    // Convert to Parquet format
    let parquet_data = convert_to_parquet(&batch)?;

    // Upload to well-specific path using OpenDAL
    let path = format!("wells/{}/trajectory/trajectory.parquet", well_id);
    adapter.write(&path, parquet_data).await?;

    Ok(())
}
```

### Type-Safe Path Management

```rust
impl ProjectDataLayoutManager {
    pub fn well_trajectory_path(&self, well_id: &str) -> String {
        format!("wells/{}/trajectory/trajectory.parquet", well_id)
    }
}
```

## Workflow Summary

1. **Parse**: Parse CSV file using `WellTrajectoryCsvImporter` with tab-delimited parsing
2. **Convert**: Convert to Arrow RecordBatch for efficient processing
3. **Display**: Convert RecordBatch to AG-Grid format for user review (4 columns: X, Y, TVD, MD)
4. **Well Selection**: User selects existing well or "New Well" via `ContentDataIngestionWellSelection`
5. **Confirm**: User reviews and edits data via AG-Grid interface
6. **Validate**: Use trajectory-specific validation rules (expanded coordinate ranges)
7. **Well Creation**: If "New Well" selected, create well in Postgres database with filename-derived name
8. **Upload**: Use `write_trajectory_for_well` with OpenDAL and well-specific path management
9. **Complete**: Success notification and file available in project bucket at well-specific path

## Key Features

- **Well-Specific Storage**: Each well's trajectory data stored in its own Parquet file
- **Database Integration**: Automatic well creation in Postgres when "New Well" selected
- **OpenDAL Integration**: Uses unified OpenDAL storage adapter for consistent data access
- **Dedicated Importer**: Uses `WellTrajectoryCsvImporter` for tab-delimited CSV parsing
- **Reused Components**: Leverages existing wells upload components for consistency
- **Arrow Integration**: Uses Arrow RecordBatch for efficient data processing
- **Parquet Storage**: Stores data in efficient Parquet format
- **MinIO Integration**: Uploads to project-specific MinIO buckets
- **No Well Name Column**: AG-Grid displays only X, Y, TVD, MD columns (matching CSV structure)
- **Well Selection UI**: Integrated well selection component for choosing existing or new wells

## Performance Considerations

- **Arrow Processing**: Uses Arrow RecordBatch for memory-efficient data processing
- **Parquet Compression**: Leverages Parquet's built-in compression for storage efficiency
- **Batch Processing**: Processes multiple trajectory points in single batch
- **Type Safety**: Compile-time path validation prevents runtime errors
- **Unified Architecture**: Consistent processing pipeline across all data types

## Configuration

### File Detection

Trajectory files are automatically detected based on filename patterns:

- Files containing "track" or "trajectory" in the name
- CSV file extension

### Validation Rules

- X/Y coordinates: -10,000,000 to 10,000,000 meters (expanded for UTM coordinates)
- TVD: -10,000 to 50,000 meters
- MD: 0 to 50,000 meters
- No well name validation (extracted from filename)

## Usage Examples

### Sample CSV Format

```csv
X,Y,TVD,MD
606554,6080126,-30,0
606554,6080126,1665,1695
```

### Frontend Usage

```typescript
// File is automatically detected as trajectory type
const file = new File([csvContent], "F02-1-track.csv", { type: "text/csv" });

// System automatically routes to trajectory processing
const result = await parseTrajectoryCsvForConfirmation(file);
```

### Backend Usage

```rust
// Tauri command automatically handles trajectory data with well selection
let rows = vec![
    TrajectoryCsvRow {
        well_name: "F02-1-track".to_string(),
        x: 606554.0,
        y: 6080126.0,
        tvd: -30.0,
        md: 0.0,
    },
    // ... more rows
];

// Command now accepts well_id and file_name parameters
write_trajectory_catalog_command(
    project_id,
    "new", // or existing well ID like "21"
    rows,
    Some("F02-1-track.csv".to_string())
).await?;
```

## Current Status

✅ **Fully Implemented & Tested**

- CSV file upload and processing
- Arrow RecordBatch conversion
- AG-Grid display and editing
- Trajectory-specific validation
- Unified storage upload
- Type-safe path management
- Parquet file generation
- MinIO storage integration

### Testing Results

- **Data Flow**: CSV → Arrow → AG-Grid conversion works reliably
- **Validation**: Trajectory-specific validation rules applied correctly
- **Well Selection**: Well selection component properly integrated
- **Well Creation**: "New Well" option creates wells in Postgres database with filename-derived names
- **Upload**: Data successfully uploaded to well-specific paths in project bucket
- **Storage**: Parquet files generated and stored in MinIO at `wells/{well_id}/trajectory/trajectory.parquet`
- **UI**: AG-Grid displays trajectory data with appropriate column definitions
- **OpenDAL**: Unified storage operations working correctly

### Key Files

- **Backend**: `src-tauri/src/wells_processing/trajectory_catalog_writer.rs`
- **Frontend**: `src/lib/components/pages/home/content-main/content-data-ingestion/csv/wells/`
- **Storage**: `crates/storage/opendal-storage-adapter/` (OpenDAL integration)
- **Well Creation**: `src-tauri/src/postgres_query/create_well.rs`
- **Types**: `src/lib/components/pages/home/content-main/content-data-ingestion/content-data-ingestion-file-types.ts`

## Debugging

### Common Issues

1. **File Detection**: Ensure filename contains "track" or "trajectory"
2. **Column Headers**: Use exact column names: X, Y, TVD, MD
3. **Data Types**: Ensure numeric columns contain valid numbers
4. **Validation**: Check that all required fields are present and valid

### Console Logs

The system provides detailed logging for debugging:

```
🔄 [Tauri] write_trajectory_catalog_command called with project_id: {id}, well_id: {well_id}, rows: {count}, file_name: {filename}
📋 [TrajectoryCommand] 'New Well' selected - creating new well in database
📋 [TrajectoryCommand] Creating well with name: {well_name}
✅ [TrajectoryCommand] Created new well with ID: {id} and name: {name}
📋 [TrajectoryCommand] Final well ID for storage: {id}
✅ [Tauri] OpenDAL storage adapter created successfully for bucket: project-{id}
🔄 [Tauri] Calling write_trajectory_for_well with {count} rows for well {id}
🔄 [TrajectoryWriter] Uploading to path: wells/{id}/trajectory/trajectory.parquet
✅ [TrajectoryWriter] Successfully uploaded trajectory to wells/{id}/trajectory/trajectory.parquet
✅ [Tauri] write_trajectory_for_well completed successfully
```

### Error Handling

- **Validation Errors**: Displayed in UI with specific field requirements
- **Upload Errors**: Logged to console with detailed error messages
- **Storage Errors**: Handled gracefully with user-friendly error messages
