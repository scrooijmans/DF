# Wells CSV to AG-Grid End-to-End Flow Guide

This guide documents the complete flow of function calls and crates used to load a CSV file and display it in an AG-Grid table, specifically for wells data using the new unified storage architecture. **✅ COMPLETED** - The flow is now fully functional and stable.

## 🏗️ Architecture Overview

The flow uses a modular architecture with the unified storage manager and frontend services:

```
CSV File → Arrow RecordBatch → AG-Grid JSON → Svelte UI
    ↓              ↓              ↓           ↓
csv-to-arrow  arrow-to-ag-grid  wells-csv-importer  frontend
    ↓              ↓              ↓           ↓
opendal-storage-adapter (unified data access layer)
    ↓              ↓              ↓           ↓
mudrock-storage-manager (unified validation & upload)
```

## 📦 Crates Used

### 1. **csv-to-arrow** (`crates/search/document_loading/csv-to-arrow/`)

- **Purpose**: Pure CSV to Arrow RecordBatch conversion
- **Key Function**: `parse_csv_to_record_batch()`
- **Dependencies**: `arrow`, `arrow-csv`, `serde`, `chrono`

### 2. **arrow-to-ag-grid** (`crates/search/document_loading/arrow-to-ag-grid/`)

- **Purpose**: Arrow RecordBatch to AG-Grid compatible JSON conversion
- **Key Function**: `record_batch_to_ag_grid_data()`
- **Dependencies**: `arrow`, `serde_json`, `chrono`, `base64`

### 3. **wells-csv-importer** (`crates/search/document_loading/wells-csv-importer/`)

- **Purpose**: Orchestrates the complete CSV to AG-Grid flow with wells-specific validation
- **Key Function**: `import_wells_csv_for_confirmation()`
- **Dependencies**: `csv-to-arrow`, `arrow-to-ag-grid`, `wells-parser`

### 4. **opendal-storage-adapter** (`crates/storage/opendal-storage-adapter/`)

- **Purpose**: Unified data access layer for storage operations
- **Key Functions**: `OpenDALStorageAdapter::upload()`, `StorageAdapterFactory::create_minio()`
- **Dependencies**: `opendal`, `project-data-layout`, `futures-util`, `bytes`

### 5. **mudrock-storage-manager** (`crates/storage/mudrock-storage-manager/`)

- **Purpose**: Unified storage management with type-safe validation and upload
- **Key Functions**: `UnifiedUploadService::upload_csv_data()`, `CsvDataValidator`, `CsvDataUploader`
- **Dependencies**: `project-data-layout`, `opendal-storage-adapter`, `object-store-adapter` (fallback), `arrow`, `parquet`

## 🔄 End-to-End Function Call Flow

### **Phase 1: Frontend File Selection & Upload**

```typescript
// 1. User drops CSV file in dropzone
content-data-ingestion-dropzone.svelte
├── processFiles()
├── parseCsvFileForConfirmation()
└── wells-csv-importer-service.ts
    └── parseWellsCsvForConfirmation()
        └── invoke('parse_wells_csv_for_confirmation_new')
```

**Files Involved:**

- `src/lib/components/pages/home/content-main/content-data-ingestion/content-data-ingestion-dropzone.svelte`
- `src/lib/services/wells-csv-importer-service.ts`

### **Phase 2: Tauri Backend Processing**

```rust
// 2. Tauri command receives file content
src-tauri/src/main.rs
└── parse_wells_csv_for_confirmation_new()
    └── WellsCsvImporter::new()
        └── import_wells_csv_for_confirmation()
```

**Files Involved:**

- `src-tauri/src/main.rs`
- `crates/search/document_loading/wells-csv-importer/src/lib.rs`

### **Phase 3: CSV to Arrow Conversion**

```rust
// 3. CSV content → Arrow RecordBatch
wells-csv-importer/src/lib.rs
└── import_wells_csv_for_confirmation()
    ├── parse_csv_with_metadata()  // csv-to-arrow crate
    │   ├── infer_schema_from_files()
    │   ├── ReaderBuilder::new()
    │   └── reader.next() → RecordBatch
    └── csv_metadata.get_record_batch()
```

**Files Involved:**

- `crates/search/document_loading/csv-to-arrow/src/lib.rs`
- `crates/search/document_loading/wells-csv-importer/src/lib.rs`

**Key Functions:**

- `parse_csv_to_record_batch()` - Main conversion function
- `infer_schema_from_files()` - Schema inference
- `ReaderBuilder::new()` - CSV reader configuration

### **Phase 4: Arrow to AG-Grid Conversion**

```rust
// 4. Arrow RecordBatch → AG-Grid JSON
wells-csv-importer/src/lib.rs
└── record_batch_to_ag_grid_data()
    └── arrow-to-ag-grid crate
        ├── convert_arrow_value_to_json()  // Per cell conversion
        ├── DataType::Utf8 → String
        ├── DataType::Float64 → Number
        ├── DataType::Date32 → String (ISO date)
        └── DataType::Struct → Object
```

**Files Involved:**

- `crates/search/document_loading/arrow-to-ag-grid/src/lib.rs`
- `crates/search/document_loading/wells-csv-importer/src/lib.rs`

**Key Functions:**

- `record_batch_to_ag_grid_data()` - Main conversion function
- `convert_arrow_value_to_json()` - Type-specific conversion
- `convert_primitive_array_to_json()` - Primitive type handling

### **Phase 5: Wells Data Validation**

```rust
// 5. Validate wells data structure
wells-csv-importer/src/lib.rs
└── validate_wells_data()
    └── wells-parser crate
        ├── validate_wells_csv_data()
        ├── validate_wells_csv_row()
        └── apply_validation_rule()
```

**Files Involved:**

- `crates/search/document_loading/wells-parser/src/wells_validation.rs`
- `crates/search/document_loading/wells-csv-importer/src/lib.rs`

### **Phase 6: Response Creation & Return**

```rust
// 6. Create WellsCsvImportResult
wells-csv-importer/src/lib.rs
└── WellsCsvImportResult {
    data: Vec<HashMap<String, Value>>,  // AG-Grid ready data
    total_rows: usize,
    file_name: String,
    columns: Vec<String>,
    wells_metadata: WellsValidationResult
}
```

### **Phase 7: Frontend Data Processing**

```typescript
// 7. Frontend receives and processes data
wells-csv-importer-service.ts
├── parseWellsCsvForConfirmation()
├── convertWellsImportResultToLegacyFormat()
└── content-data-ingestion-dropzone.svelte
    └── dispatch('csv-processing-result')
```

### **Phase 8: AG-Grid Display**

```typescript
// 8. AG-Grid renders the data (using stable AG-data-table component)
content-data-wells-upload-confirm.svelte
├── AGDataTable component (external data mode)
│   ├── externalData = csvData
│   ├── externalColumnDefs = generateWellsColumnDefsFromSchema()
│   └── tableName = "wells"
└── content-data-wells-upload-confirm-AG-data-table.svelte
    ├── AgGrid component with full configuration
    ├── themeQuartz with CSS variables
    ├── Enterprise modules (RangeSelectionModule, MenuModule)
    ├── Complete gridOptions with pagination, editing, context menu
    └── ModuleRegistry.registerModules([ClientSideRowModelModule, RangeSelectionModule, MenuModule])
```

**Files Involved:**

- `src/lib/components/pages/home/content-main/content-data-ingestion/csv/wells/content-data-wells-upload-confirm/content-data-wells-upload-confirm.svelte`
- `src/lib/components/pages/home/content-main/content-data-ingestion/csv/wells/content-data-wells-upload-confirm/content-data-wells-upload-confirm-AG-data-table/content-data-wells-upload-confirm-AG-data-table.svelte`

## 🔍 Detailed Function Call Sequence

### **1. File Upload Initiation**

```typescript
// Frontend: User drops file
content-data-ingestion-dropzone.svelte
├── onDrop() event handler
├── processFiles()
│   ├── validateFileForType()
│   └── parseCsvFileForConfirmation()
└── wells-csv-importer-service.ts
    └── parseWellsCsvForConfirmation(file: File)
        ├── file.text() // Read file content
        └── invoke('parse_wells_csv_for_confirmation_new', { csvContent, fileName })
```

### **2. Tauri Command Execution**

```rust
// Backend: Tauri command
src-tauri/src/main.rs
└── parse_wells_csv_for_confirmation_new(csv_content: String, file_name: String)
    └── WellsCsvImporter::new()
        └── import_wells_csv_for_confirmation(csv_content, file_name)
```

### **3. CSV to Arrow Pipeline**

```rust
// CSV → Arrow conversion
wells-csv-importer/src/lib.rs
└── import_wells_csv_for_confirmation()
    └── parse_csv_with_metadata(csv_content, file_name, config)
        └── csv-to-arrow/src/lib.rs
            ├── parse_csv_to_record_batch()
            │   ├── infer_schema_from_files()
            │   │   ├── Format::default()
            │   │   └── infer_schema()
            │   └── ReaderBuilder::new()
            │       ├── with_format(format)
            │       ├── with_batch_size()
            │       └── build(cursor)
            └── reader.next() → RecordBatch
```

### **4. Arrow to AG-Grid Pipeline**

```rust
// Arrow → AG-Grid conversion
wells-csv-importer/src/lib.rs
└── record_batch_to_ag_grid_data(record_batch)
    └── arrow-to-ag-grid/src/lib.rs
        ├── record_batch_to_ag_grid_data()
        │   ├── for each row in batch
        │   │   ├── for each column in batch
        │   │   │   └── convert_arrow_value_to_json()
        │   │   └── row_map.insert(column_name, value)
        │   └── result.push(row_map)
        └── convert_arrow_value_to_json()
            ├── match array.data_type()
            ├── DataType::Utf8 → as_string_array()
            ├── DataType::Float64 → as_primitive_array()
            ├── DataType::Date32 → as_primitive_array() + date conversion
            └── DataType::Struct → struct_array processing
```

### **5. Wells Validation Pipeline**

```rust
// Wells-specific validation
wells-csv-importer/src/lib.rs
└── validate_wells_data(&ag_grid_data)
    └── wells-parser/src/wells_validation.rs
        ├── validate_wells_csv_data()
        │   ├── for each row in data
        │   │   └── validate_wells_csv_row()
        │   │       ├── check required fields
        │   │       ├── validate data types
        │   │       └── apply_validation_rule()
        └── WellsValidationResult
```

### **6. Response Creation**

```rust
// Create final response
wells-csv-importer/src/lib.rs
└── WellsCsvImportResult {
    data: ag_grid_data,                    // Vec<HashMap<String, Value>>
    total_rows: csv_metadata.total_rows,   // usize
    columns: csv_metadata.get_column_names(), // Vec<String>
    file_name: csv_metadata.file_name,     // String
    wells_metadata: wells_metadata,        // WellsValidationResult
}
```

### **7. Frontend Data Processing**

```typescript
// Frontend: Process response
wells-csv-importer-service.ts
├── parseWellsCsvForConfirmation()
│   ├── invoke('parse_wells_csv_for_confirmation_new')
│   └── convertWellsImportResultToLegacyFormat()
└── content-data-ingestion-dropzone.svelte
    ├── handleCsvProcessingResult()
    └── dispatch('csv-processing-result')
```

### **8. AG-Grid Rendering**

```typescript
// AG-Grid: Display data (using stable component architecture)
content-data-wells-upload-confirm.svelte
├── AGDataTable component (external data mode)
│   ├── externalData = csvData
│   ├── externalColumnDefs = generateWellsColumnDefsFromSchema()
│   └── tableName = "wells"
└── content-data-wells-upload-confirm-AG-data-table.svelte
    ├── AgGrid component with complete configuration
    │   ├── themeQuartz with CSS variables (--ag-row-height, --ag-header-height)
    │   ├── Enterprise modules (RangeSelectionModule, MenuModule)
    │   ├── Complete gridOptions (pagination, editing, context menu, cell selection)
    │   └── ModuleRegistry.registerModules([ClientSideRowModelModule, RangeSelectionModule, MenuModule])
    └── External data handling via $effect() for reliable rendering
```

## 📊 Data Flow Summary

| Phase | Input          | Output            | Crate/Service        |
| ----- | -------------- | ----------------- | -------------------- |
| 1     | CSV File       | File Content      | Frontend Dropzone    |
| 2     | File Content   | Tauri Command     | Tauri Backend        |
| 3     | CSV String     | Arrow RecordBatch | `csv-to-arrow`       |
| 4     | RecordBatch    | AG-Grid JSON      | `arrow-to-ag-grid`   |
| 5     | AG-Grid JSON   | Validated Data    | `wells-parser`       |
| 6     | Validated Data | Import Result     | `wells-csv-importer` |
| 7     | Import Result  | Legacy Format     | Frontend Service     |
| 8     | Legacy Format  | AG-Grid Table     | Svelte Component     |

## 🎯 Key Benefits

1. **Modular Architecture**: Each crate has a single responsibility
2. **Type Safety**: Arrow provides strong typing throughout the pipeline
3. **Performance**: Arrow's columnar format is optimized for data processing
4. **Validation**: Wells-specific validation ensures data quality
5. **Reusability**: Crates can be used independently for other data types
6. **Reliable Rendering**: Uses stable AG-Grid component with complete configuration
7. **Enterprise Features**: Full context menu, cell selection, and editing capabilities

## 🔧 Implementation Improvements

### **AG-Grid Rendering Fixes**

The flow was enhanced to resolve initial rendering issues:

1. **Component Architecture**: Replaced custom minimal AG-Grid component with stable `AG-data-table.svelte`
2. **External Data Mode**: Added support for external data props to avoid database dependency
3. **Complete Configuration**: Implemented full AG-Grid configuration including:
   - `themeQuartz` with proper CSS variables
   - Enterprise modules (`RangeSelectionModule`, `MenuModule`)
   - Complete `gridOptions` with pagination, editing, and context menu
   - Proper `$effect()` handling for data updates

### **File Structure**

```
content-data-wells-upload-confirm/
├── content-data-wells-upload-confirm.svelte (orchestrator)
└── content-data-wells-upload-confirm-AG-data-table/
    ├── content-data-wells-upload-confirm-AG-data-table.svelte (stable AG-Grid)
    ├── add-row.ts (row management)
    └── column-state-manager.ts (column state)
```

### **Data Flow Reliability**

- **External Data Props**: Wells data passed as `externalData` and `externalColumnDefs`
- **Reactive Updates**: Uses Svelte 5 `$effect()` for reliable data binding
- **Error Handling**: Comprehensive error states and loading indicators
- **Validation**: Wells-specific validation with user-friendly error messages

## 🔧 Configuration

The flow uses these configuration options:

```rust
// csv-to-arrow configuration
CsvParseConfig {
    delimiter: b',',
    has_header: true,
    batch_size: 1000,
}

// wells-parser validation
WellsSchema {
    required_columns: vec!["well_name", "field", "operator", ...],
    optional_columns: vec!["company", "coordinate_system"],
    validation_rules: {...}
}
```

This modular approach ensures clean separation of concerns while maintaining high performance and type safety throughout the CSV to AG-Grid conversion pipeline.

## ✅ Completion Status

### **Fully Implemented & Tested**

- ✅ CSV file upload and processing
- ✅ Arrow RecordBatch conversion
- ✅ AG-Grid JSON data transformation
- ✅ Wells data validation
- ✅ Stable AG-Grid rendering with external data
- ✅ Complete component architecture
- ✅ Error handling and loading states
- ✅ Enterprise AG-Grid features

### **Testing Results**

- **Data Flow**: CSV → Arrow → AG-Grid conversion works reliably
- **Rendering**: AG-Grid displays data consistently on first attempt
- **Validation**: Wells-specific validation catches data issues
- **UI/UX**: Loading states, error handling, and user feedback work properly
- **Performance**: Handles large CSV files efficiently with Arrow's columnar format

### **Key Files**

- **Backend**: `crates/search/document_loading/wells-csv-importer/`
- **Frontend**: `src/lib/components/pages/home/content-main/content-data-ingestion/csv/wells/`
- **Services**: `src/lib/services/wells-csv-importer-service.ts`
- **Documentation**: `markdown-guides/ag-grid/issues/failure-to-render-table-with-data.md`

The Wells CSV to AG-Grid flow is now **production-ready** and provides a robust, scalable solution for wells data ingestion and visualization.
