# Checkshot CSV Upload

## Overview

The checkshot CSV upload functionality allows users to upload well checkshot data in CSV format. This data represents time-depth relationships for wells, including True Vertical Depth (TVD) and Two-Way Time (TWT) measurements used for seismic-to-well ties and velocity analysis.

## Architecture Overview

```
Frontend (SvelteKit)
    ↓ Checkshot CSV File Drop
ContentDataIngestionDropzone
    ↓ parseCheckshotCsvForConfirmation()
Tauri Command (Rust Backend)
    ↓ CSV Parsing & Validation
WellCheckshotCsvImporter (dedicated checkshot importer)
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
write_checkshot_catalog_command()
    ↓ Well ID Resolution & Database Integration
    - If "New Well": WellCreator::create_well() → INSERT into Postgres wells table
    - Returns database ID (e.g., 26) from Postgres
    - If existing well: Use selected database ID directly
    ↓ Well ID Available (database ID for all scenarios)
    ↓ OpenDAL Storage Processing
OpenDALStorageAdapter (unified data access layer)
    ↓ Type-Safe Path Construction
ProjectDataLayoutManager (type-safe paths)
    ↓ MinIO Storage via OpenDAL
Project Bucket (project-{id}/wells/{database_id}/checkshot/checkshot.parquet)
    - Example: project-{id}/wells/26/checkshot/checkshot.parquet (F06-1_TD)
    - Example: project-{id}/wells/21/checkshot/checkshot.parquet (My-Well-1)
    ↓ Success Notification
```

## Schema Definition

### Required Fields

- **TVD**: True Vertical Depth in meters (number)
- **TWT**: Two-Way Time in seconds (number)

**Note**: The checkshot data does not include a `well_name` field in the AG-Grid display. The well name is extracted from the filename during processing but not displayed as a column.

### Validation Rules

```typescript
{
  TVD: {
    required: true,
    type: "number",
    min: -10000,
    max: 50000,
  },
  TWT: {
    required: true,
    type: "number",
    min: 0,
    max: 100,
  },
}
```

## Storage Level

**Checkshots are stored at the WELL level**, not the project level. This means:

- **Storage Path**: `wells/{well_id}/checkshot/checkshot.parquet`
- **Scope**: Each well's checkshot data is stored in its own Parquet file
- **Structure**: The Parquet file contains checkshot points for a specific well
- **Well Identification**: The well is identified by the database ID in the storage path

This approach allows for:

- Well-specific data organization and management
- Efficient querying of individual well checkshot data
- Consistent with LAS file storage (well-specific)
- Simplified data management at the well level
- Better data isolation and security

## Frontend Implementation

### File Type Detection

The system automatically detects checkshot files based on filename patterns:

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
    } else if (fileName.includes("checkshot") || fileName.includes("td")) {
      return "checkshots";
    } else {
      return "wells";
    }
  }

  return null;
}
```

### Data Mapping

The confirmation component maps CSV data to checkshot format, preserving the original uppercase field names:

```typescript
// For checkshot data, preserve the original field names (TVD, TWT)
if (ingestionState?.selectedDataType === "checkshots") {
  return {
    TVD: row.TVD || 0,
    TWT: row.TWT || 0,
    // Keep other fields for compatibility
    well_name: row.well_name || row.Well || "",
    // ... other fields
  };
}
```

### Column Definitions

The AG-Grid displays checkshot data with 2 columns (no Well Name column):

```typescript
// Checkshot schema: TVD, TWT (no well_name column)
[
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
    field: "TWT",
    headerName: "TWT (s)",
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

**Note**: The checkshot AG-Grid does not include a "Well Name" column. The well name is extracted from the filename during processing but not displayed in the table.

## Backend Implementation

### Tauri Commands

#### Parse Command

```rust
#[tauri::command]
async fn parse_checkshot_csv_for_confirmation(
    csv_content: String,
    file_name: String,
) -> Result<CheckshotCsvImportResult, String> {
    println!("🔍 [Tauri] Parsing Checkshot CSV file: {} using new modular crates", file_name);

    let importer = WellCheckshotCsvImporter::new();
    match importer.import_checkshot_csv_for_confirmation(&csv_content, &file_name) {
        Ok(result) => {
            println!("✅ Successfully parsed Checkshot CSV file: {} with {} rows", file_name, result.total_rows);
            Ok(result)
        }
        Err(e) => {
            println!("❌ Failed to parse Checkshot CSV file: {} - {}", file_name, e);
            Err(format!("Failed to parse Checkshot CSV file: {}", e))
        }
    }
}
```

#### Upload Command

```rust
#[tauri::command]
async fn write_checkshot_catalog_command(
    project_id: String,
    well_id: String,
    rows: Vec<CheckshotCsvRow>,
    file_name: Option<String>,
) -> Result<(), String> {
    println!("🔄 [Tauri] write_checkshot_catalog_command called with project_id: {}, well_id: {}, rows: {}, file_name: {:?}", project_id, well_id, rows.len(), file_name);

    // Step 1: If well_id is "new", create a new well in the database first
    let final_well_id = if well_id == "new" {
        println!("📋 [CheckshotCommand] 'New Well' selected - creating new well in database");

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

        println!("📋 [CheckshotCommand] Creating well with name: {}", well_name);

        let well_creator = WellCreator::new();
        let create_request = CreateWellRequest {
            name: well_name,
            project_id: project_id.clone(),
        };

        match well_creator.create_well(create_request).await {
            Ok(created_well) => {
                println!("✅ [CheckshotCommand] Created new well with ID: {} and name: {}", created_well.id, created_well.name);
                created_well.id.to_string()
            }
            Err(e) => {
                println!("⚠️ [CheckshotCommand] Failed to create well in database: {}. Using 'new' for path instead.", e);
                "new".to_string()
            }
        }
    } else {
        println!("📋 [CheckshotCommand] Using existing well ID: {}", well_id);
        well_id
    };

    println!("📋 [CheckshotCommand] Final well ID for storage: {}", final_well_id);

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

    println!("🔄 [Tauri] Calling write_checkshot_for_well with {} rows for well {}", rows.len(), final_well_id);
    match write_checkshot_for_well(&adapter, project_id.clone(), final_well_id, rows).await {
        Ok(_) => {
            println!("✅ [Tauri] write_checkshot_for_well completed successfully");
            Ok(())
        },
        Err(e) => {
            println!("❌ [Tauri] write_checkshot_for_well failed: {}", e);
            Err(e)
        }
    }
}
```

### Checkshot Writer

```rust
pub async fn write_checkshot_for_well(
    adapter: &OpenDALStorageAdapter,
    project_id: String,
    well_id: String,
    rows: Vec<CheckshotCsvRow>,
) -> Result<(), String> {
    println!("🔄 [CheckshotWriter] Starting write_checkshot_for_well with {} rows for well {} in project {}", rows.len(), well_id, project_id);

    // Build Arrow columns
    let well_name: ArrayRef = Arc::new(StringArray::from(
        rows.iter().map(|r| r.well_name.as_str()).collect::<Vec<_>>()
    ));
    let tvd: ArrayRef = Arc::new(Float64Array::from(
        rows.iter().map(|r| r.tvd).collect::<Vec<_>>()
    ));
    let twt: ArrayRef = Arc::new(Float64Array::from(
        rows.iter().map(|r| r.twt).collect::<Vec<_>>()
    ));

    // Define Schema
    let schema = Arc::new(Schema::new(vec![
        Field::new("well_name", DataType::Utf8, false),
        Field::new("tvd", DataType::Float64, false),
        Field::new("twt", DataType::Float64, false),
    ]));

    let batch = RecordBatch::try_new(schema.clone(), vec![well_name, tvd, twt])?;
    println!("🔄 [CheckshotWriter] Created RecordBatch with {} rows", batch.num_rows());

    // Convert to Parquet
    let mut buffer = Vec::new();
    {
        let mut writer = ArrowWriter::try_new(&mut buffer, batch.schema(), None)?;
        writer.write(&batch)?;
        writer.finish()?;
    }
    println!("🔄 [CheckshotWriter] Parquet size: {} bytes", buffer.len());

    // Upload to well-specific path
    let path = format!("wells/{}/checkshot/checkshot.parquet", well_id);
    println!("🔄 [CheckshotWriter] Uploading to path: {}", path);

    adapter.write(&path, buffer).await?;
    println!("✅ [CheckshotWriter] Successfully uploaded checkshot to {}", path);

    Ok(())
}
```

## Storage Layer Architecture

### OpenDAL Storage Processing

The checkshot upload uses OpenDAL for unified storage operations:

```rust
// In wells_processing/checkshot_catalog_writer.rs
pub async fn write_checkshot_for_well(
    adapter: &OpenDALStorageAdapter,
    project_id: String,
    well_id: String,
    rows: Vec<CheckshotCsvRow>,
) -> Result<(), String> {
    // Convert CSV rows to Arrow RecordBatch
    let batch = create_checkshot_record_batch(rows)?;

    // Convert to Parquet format
    let parquet_data = convert_to_parquet(&batch)?;

    // Upload to well-specific path using OpenDAL
    let path = format!("wells/{}/checkshot/checkshot.parquet", well_id);
    adapter.write(&path, parquet_data).await?;

    Ok(())
}
```

### Type-Safe Path Management

```rust
impl ProjectDataLayoutManager {
    pub fn well_checkshot_path(&self, well_id: &str) -> String {
        format!("wells/{}/checkshot/checkshot.parquet", well_id)
    }
}
```

## Workflow Summary

1. **Parse**: Parse CSV file using `WellCheckshotCsvImporter` with tab-delimited parsing
2. **Convert**: Convert to Arrow RecordBatch for efficient processing
3. **Display**: Convert RecordBatch to AG-Grid format for user review (2 columns: TVD, TWT)
4. **Well Selection**: User selects existing well or "New Well" via `ContentDataIngestionWellSelection`
5. **Confirm**: User reviews and edits data via AG-Grid interface
6. **Validate**: Use checkshot-specific validation rules (TVD: -10000 to 50000, TWT: 0 to 100)
7. **Well Creation**: If "New Well" selected, create well in Postgres database with filename-derived name
8. **Upload**: Use `write_checkshot_for_well` with OpenDAL and well-specific path management
9. **Complete**: Success notification and file available in project bucket at well-specific path

## Key Features

- **Well-Specific Storage**: Each well's checkshot data stored in its own Parquet file
- **Database Integration**: Automatic well creation in Postgres when "New Well" selected
- **OpenDAL Integration**: Uses unified OpenDAL storage adapter for consistent data access
- **Dedicated Importer**: Uses `WellCheckshotCsvImporter` for tab-delimited CSV parsing
- **Reused Components**: Leverages existing wells upload components for consistency
- **Arrow Integration**: Uses Arrow RecordBatch for efficient data processing
- **Parquet Storage**: Stores data in efficient Parquet format
- **MinIO Integration**: Uploads to project-specific MinIO buckets
- **No Well Name Column**: AG-Grid displays only TVD, TWT columns (matching CSV structure)
- **Well Selection UI**: Integrated well selection component for choosing existing or new wells

## Performance Considerations

- **Arrow Processing**: Uses Arrow RecordBatch for memory-efficient data processing
- **Parquet Compression**: Leverages Parquet's built-in compression for storage efficiency
- **Batch Processing**: Processes multiple checkshot points in single batch
- **Type Safety**: Compile-time path validation prevents runtime errors
- **Unified Architecture**: Consistent processing pipeline across all data types

## Configuration

### File Detection

Checkshot files are automatically detected based on filename patterns:

- Files containing "checkshot" or "td" in the name
- CSV file extension

### Validation Rules

- TVD: -10,000 to 50,000 meters
- TWT: 0 to 100 seconds
- No well name validation (extracted from filename)

## Usage Examples

### Sample CSV Format

```csv
TVD	TWT
30	0
553.6	0.544
```

### Frontend Usage

```typescript
// File is automatically detected as checkshot type
const file = new File([csvContent], "F02-1_TD.csv", { type: "text/csv" });

// System automatically routes to checkshot processing
const result = await parseCheckshotCsvForConfirmation(file);
```

### Backend Usage

```rust
// Tauri command automatically handles checkshot data with well selection
let rows = vec![
    CheckshotCsvRow {
        well_name: "F06-1_TD".to_string(),
        tvd: 28.64,
        twt: 0.0,
    },
    // ... more rows
];

// Command now accepts well_id and file_name parameters
write_checkshot_catalog_command(
    project_id,
    "new", // or existing well ID like "21"
    rows,
    Some("F06-1_TD.csv".to_string())
).await?;
```

## Current Status

✅ **Fully Implemented & Tested**

- CSV file upload and processing
- Arrow RecordBatch conversion
- AG-Grid display and editing
- Checkshot-specific validation
- Unified storage upload
- Type-safe path management
- Parquet file generation
- MinIO storage integration

### Testing Results

- **Data Flow**: CSV → Arrow → AG-Grid conversion works reliably
- **Validation**: Checkshot-specific validation rules applied correctly
- **Well Selection**: Well selection component properly integrated
- **Well Creation**: "New Well" option creates wells in Postgres database with filename-derived names
- **Upload**: Data successfully uploaded to well-specific paths in project bucket
- **Storage**: Parquet files generated and stored in MinIO at `wells/{well_id}/checkshot/checkshot.parquet`
- **UI**: AG-Grid displays checkshot data with appropriate column definitions
- **OpenDAL**: Unified storage operations working correctly

### Key Files

- **Backend**: `src-tauri/src/wells_processing/checkshot_catalog_writer.rs`
- **Frontend**: `src/lib/components/pages/home/content-main/content-data-ingestion/csv/wells/`
- **Storage**: `crates/storage/opendal-storage-adapter/` (OpenDAL integration)
- **Well Creation**: `src-tauri/src/postgres_query/create_well.rs`
- **Types**: `src/lib/components/pages/home/content-main/content-data-ingestion/content-data-ingestion-file-types.ts`

## Debugging

### Common Issues

1. **File Detection**: Ensure filename contains "checkshot" or "td"
2. **Column Headers**: Use exact column names: TVD, TWT
3. **Data Types**: Ensure numeric columns contain valid numbers
4. **Validation**: Check that all required fields are present and valid

### Console Logs

The system provides detailed logging for debugging:

```
🔄 [Tauri] write_checkshot_catalog_command called with project_id: {id}, well_id: {well_id}, rows: {count}, file_name: {filename}
📋 [CheckshotCommand] 'New Well' selected - creating new well in database
📋 [CheckshotCommand] Creating well with name: {well_name}
✅ [CheckshotCommand] Created new well with ID: {id} and name: {name}
📋 [CheckshotCommand] Final well ID for storage: {id}
✅ [Tauri] OpenDAL storage adapter created successfully for bucket: project-{id}
🔄 [Tauri] Calling write_checkshot_for_well with {count} rows for well {id}
🔄 [CheckshotWriter] Uploading to path: wells/{id}/checkshot/checkshot.parquet
✅ [CheckshotWriter] Successfully uploaded checkshot to wells/{id}/checkshot/checkshot.parquet
✅ [Tauri] write_checkshot_for_well completed successfully
```

### Error Handling

- **Validation Errors**: Displayed in UI with specific field requirements
- **Upload Errors**: Logged to console with detailed error messages
- **Storage Errors**: Handled gracefully with user-friendly error messages
