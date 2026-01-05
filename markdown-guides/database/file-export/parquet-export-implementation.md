# Parquet Export Implementation - Modular Flow

## Overview

This document describes the implementation of a modular Parquet export flow that runs alongside the existing data visualization functionality. When a user clicks on a Parquet file, they can now both visualize the data in the AG-Grid table AND export it to various formats.

## Architecture

### 1. **Modular Design**

The export functionality is implemented as a separate, modular flow that:

- Runs in parallel with the existing visualization
- Doesn't interfere with the current data table display
- Can be easily extended or modified independently
- Provides a clean separation of concerns

### 2. **Component Structure**

```
src/lib/components/pages/home/content-main/content-data-analytics/
├── content-data-analytics-file-item/
│   └── content-data-analytics-file-item.svelte (enhanced with export button)
├── content-data-analytics-export-dialog/
│   └── content-data-analytics-export-dialog.svelte (new export dialog)
└── content-data-analytics-file-content/
    └── content-data-analytics-file-content-AG-data-table.svelte (existing visualization)
```

## Implementation Details

### 1. **Backend (Rust/Tauri)**

#### **Export Service (`src-tauri/src/parquet_query.rs`)**

```rust
// Export result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParquetExportResult {
    pub success: bool,
    pub file_path: Option<String>,
    pub file_name: Option<String>,
    pub file_size: Option<u64>,
    pub row_count: Option<usize>,
    pub error: Option<String>,
}

// Export options structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParquetExportOptions {
    pub format: String,
    pub compression: Option<String>,
    pub include_metadata: bool,
    pub filename: Option<String>,
}

// Main export function using DuckDB
pub async fn export_parquet_data(
    bucket_name: String,
    file_path: String,
    options: ParquetExportOptions,
) -> Result<ParquetExportResult, String>
```

#### **Supported Export Formats**

- **CSV**: Comma-separated values with headers
- **Parquet**: Columnar format with compression options
- **JSON**: JavaScript Object Notation (array format)
- **Excel**: Microsoft Excel format (via CSV conversion)

#### **Compression Options (Parquet only)**

- **None**: No compression (fastest)
- **GZIP**: Good compression ratio
- **Snappy**: Fast compression (recommended)
- **LZ4**: Very fast compression
- **Zstandard**: High compression ratio

### 2. **Frontend (Svelte/TypeScript)**

#### **Export Service (`src/lib/services/parquet-export-service.ts`)**

```typescript
export interface ParquetExportOptions {
  format: "csv" | "parquet" | "json" | "xlsx";
  compression?: "none" | "gzip" | "snappy" | "lz4" | "zstd";
  includeMetadata?: boolean;
  filename?: string;
}

export interface ParquetExportProgress {
  stage: "preparing" | "querying" | "exporting" | "completed" | "error";
  progress: number; // 0-100
  message: string;
}

// Main export function with progress tracking
export async function exportParquetData(
  bucketName: string,
  filePath: string,
  options: ParquetExportOptions,
  onProgress?: (progress: ParquetExportProgress) => void,
): Promise<ParquetExportResult>;
```

#### **Export Dialog Component**

The export dialog provides:

- **Format Selection**: Dropdown with format descriptions
- **Compression Options**: Only shown for Parquet format
- **Filename Input**: Auto-generated with timestamp
- **Metadata Option**: Include schema information
- **Progress Tracking**: Real-time progress updates
- **Result Display**: Success/error feedback with file details

### 3. **User Interface Integration**

#### **File Item Enhancement**

The file item component now includes:

- **Export Button**: Download icon next to each file
- **Click Separation**: Export button doesn't trigger file selection
- **Visual Feedback**: Hover states and accessibility

#### **Export Dialog Features**

- **File Information**: Shows source file details
- **Format Selection**: User-friendly format picker
- **Compression Options**: Context-aware compression settings
- **Progress Tracking**: Real-time export progress
- **Result Feedback**: Success/error messages with file details

## Usage Flow

### 1. **File Selection and Visualization**

1. User clicks on a Parquet file
2. File is selected and data is loaded
3. AG-Grid table displays the data
4. Export button becomes available

### 2. **Export Process**

1. User clicks the export button (download icon)
2. Export dialog opens with file information
3. User selects format and options
4. Export process begins with progress tracking
5. File is exported to local `exports/` directory
6. Success/error feedback is displayed

### 3. **Parallel Operation**

- **Visualization**: Continues to work independently
- **Export**: Runs in background without affecting display
- **State Management**: Separate state for each operation

## Technical Features

### 1. **DuckDB Integration**

- **S3 Direct Access**: Queries Parquet files directly from S3
- **Format Conversion**: Uses DuckDB's COPY command for export
- **Compression Support**: Native compression options
- **Performance**: Efficient columnar data processing

### 2. **Error Handling**

- **Graceful Degradation**: Export failures don't affect visualization
- **Detailed Error Messages**: Clear error reporting
- **Retry Capability**: Users can retry failed exports

### 3. **Progress Tracking**

- **Real-time Updates**: Progress callback system
- **Stage Information**: Clear progress stages
- **User Feedback**: Visual progress indicators

### 4. **File Management**

- **Auto-generated Names**: Timestamp-based naming
- **Format Extensions**: Automatic file extension handling
- **Local Storage**: Exports saved to `exports/` directory

## Configuration

### 1. **Export Directory**

```rust
// Backend configuration
let output_dir = "exports";
```

### 2. **Default Options**

```typescript
// Frontend defaults
const defaultOptions: ParquetExportOptions = {
  format: "csv",
  compression: "none",
  includeMetadata: false,
  filename: "",
};
```

### 3. **Supported Formats**

```typescript
const exportFormats = [
  { value: "csv", label: "CSV", description: "Comma-separated values" },
  {
    value: "parquet",
    label: "Parquet",
    description: "Columnar format (recommended)",
  },
  { value: "json", label: "JSON", description: "JavaScript Object Notation" },
  { value: "xlsx", label: "Excel", description: "Microsoft Excel format" },
];
```

## Benefits

### 1. **User Experience**

- **Dual Functionality**: View and export in one interface
- **No Interruption**: Export doesn't affect visualization
- **Progress Feedback**: Clear progress indication
- **Format Flexibility**: Multiple export options

### 2. **Developer Experience**

- **Modular Design**: Easy to extend or modify
- **Clean Separation**: Export logic separate from visualization
- **Reusable Components**: Export dialog can be used elsewhere
- **Type Safety**: Full TypeScript support

### 3. **Performance**

- **Efficient Processing**: DuckDB's optimized export
- **Memory Management**: Streaming data processing
- **Compression Options**: Reduced file sizes
- **Background Processing**: Non-blocking operations

## Future Enhancements

### 1. **Additional Formats**

- **Excel Native**: True Excel format support
- **PDF**: Report generation
- **XML**: Structured data export

### 2. **Advanced Options**

- **Data Filtering**: Export specific rows/columns
- **Custom Queries**: SQL-based export
- **Batch Export**: Multiple files at once

### 3. **Integration Features**

- **Cloud Storage**: Direct upload to cloud services
- **Email Integration**: Send exports via email
- **API Integration**: Export to external systems

## Conclusion

The modular Parquet export implementation provides a seamless way for users to both visualize and export their data. The design ensures that the export functionality enhances the existing workflow without disrupting the core visualization features, while providing a solid foundation for future enhancements.

The implementation leverages DuckDB's powerful data processing capabilities and provides a user-friendly interface for data export, making MudRock a more comprehensive platform for petrophysical data management.
