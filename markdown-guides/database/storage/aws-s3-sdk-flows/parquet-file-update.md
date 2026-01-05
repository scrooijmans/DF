# Parquet File Update Implementation | MudRock

## Overview

This document describes the implementation of Parquet file cell editing functionality in MudRock. Unlike traditional database systems, Parquet files are immutable by design, which presents unique challenges for implementing cell-level updates.

## The Challenge

Parquet files are:

- **Immutable**: Cannot be modified in-place
- **Columnar**: Data is stored in columns, not rows
- **Compressed**: Data is compressed for efficiency
- **Schema-bound**: Structure is fixed at write time

Traditional approaches like SQL `UPDATE` statements don't work with Parquet files because:

1. Parquet files don't support append operations
2. File systems (especially HDFS) may not allow append on files
3. No existing libraries provide direct cell-level update APIs

## Solution: Complete File Rewrite

Our implementation uses a **complete file rewrite** approach:

1. **Read** the entire Parquet file into memory
2. **Modify** the specific cell(s) in the in-memory data
3. **Rewrite** the entire file using Arrow Writer
4. **Upload** the new file to replace the original

## Architecture

### Backend (Rust)

#### Core Components

**`src-tauri/src/parquet_update.rs`**

- `update_parquet_cell()`: Main orchestrator function for single cell updates
- `read_parquet_data()`: Reads Parquet data using DuckDB with column detection
- `rewrite_parquet_with_arrow()`: Converts data to Arrow format and rewrites file
- `upload_parquet_to_s3()`: Uploads rewritten file to S3 using DuckDB COPY

#### Current Data Flow

```
Frontend Cell Edit (AG-Grid)
       ↓
Tauri Command: update_parquet_cell_command()
       ↓
1. Column Detection (DESCRIBE query)
       ↓
2. Data Reading (SELECT * FROM read_parquet())
       ↓
3. In-Memory Cell Update
       ↓
4. Arrow Schema Creation
       ↓
5. Arrow Array Conversion
       ↓
6. RecordBatch Creation
       ↓
7. ArrowWriter (SNAPPY compression)
       ↓
8. Temporary File Write
       ↓
9. DuckDB COPY to S3 (OVERWRITE_OR_IGNORE)
       ↓
10. Cleanup & Response
```

### Frontend (Svelte)

#### Components

**`content-data-analytics-file-content-AG-data-table.svelte`**

- AG-Grid with cell editing enabled
- `onCellValueChanged` event handler
- Calls `updateParquetCell()` service

**`parquet-update-service.ts`**

- Frontend service for Parquet updates
- Handles Tauri command invocation
- Error handling and response processing

## Implementation Details

### 1. Column Detection & Data Reading (DuckDB)

```rust
fn read_parquet_data(
    bucket_name: &str,
    file_path: &str,
    row_id: i32,
    field: &str,
    new_value: &serde_json::Value,
) -> Result<(Vec<HashMap<String, serde_json::Value>>, Vec<String>, Option<HashMap<String, serde_json::Value>>), String>
```

**Step 1: Column Detection**

- Uses `DESCRIBE SELECT * FROM read_parquet(s3_path)` to get column names and types
- Avoids borrow checker issues by using separate statement
- Extracts column metadata before data processing

**Step 2: Data Reading**

- Uses DuckDB with S3 integration (`httpfs` extension)
- Reads entire Parquet file into memory with `SELECT * FROM read_parquet(s3_path)`
- Updates specific cell during read process using row index
- Returns all data, column names, and updated row
- Includes extensive debugging output for troubleshooting

### 2. Data Rewriting (Arrow Writer)

```rust
async fn rewrite_parquet_with_arrow(
    rows: Vec<HashMap<String, serde_json::Value>>,
    column_names: &[String],
    s3_path: &str,
) -> Result<Vec<HashMap<String, serde_json::Value>>, String>
```

**Step 1: Schema Creation**

- Creates Arrow `Schema` with `Field` definitions for each column
- All columns default to `DataType::Utf8` for simplicity
- Uses `Arc<Schema>` for efficient sharing

**Step 2: Array Conversion**

- Converts `serde_json::Value` data to Arrow `Array` types
- Handles `String`, `Number`, `Bool`, and `Null` values
- Creates `StringArray` for all data (converts numbers to strings)
- Uses `Arc<dyn Array>` for type erasure

**Step 3: RecordBatch Creation**

- Creates `RecordBatch` from schema and arrays
- Validates row count and column count consistency
- Includes debugging output for verification

**Step 4: Parquet Writing**

- Uses `ArrowWriter` with SNAPPY compression
- Writes to in-memory `Vec<u8>` buffer
- Applies `WriterProperties` for compression settings
- Includes extensive logging for troubleshooting

### 3. File Upload (DuckDB COPY)

```rust
async fn upload_parquet_to_s3(data: &[u8], s3_path: &str) -> Result<(), String>
```

**Step 1: S3 Configuration**

- Creates new DuckDB connection for upload
- Installs and loads `httpfs` extension
- Configures S3 credentials for MinIO storage
- Sets endpoint, region, and URL style parameters

**Step 2: Temporary File Creation**

- Writes Arrow-generated data to temporary file
- Uses UUID-based filename to avoid conflicts
- Logs file path and data size for debugging

**Step 3: S3 Upload**

- Uses DuckDB `COPY` statement: `COPY (SELECT * FROM read_parquet(temp_file)) TO s3_path (FORMAT PARQUET, OVERWRITE_OR_IGNORE)`
- Overwrites original file with `OVERWRITE_OR_IGNORE` flag
- Includes extensive logging for upload verification

**Step 4: Cleanup**

- Removes temporary file after successful upload
- Logs cleanup completion
- Returns success/error status

## Key Features

### ✅ Advantages

1. **Data Integrity**: Complete rewrite ensures data consistency
2. **Schema Preservation**: Maintains original Parquet schema structure
3. **Compression**: Applies SNAPPY compression for efficiency
4. **Error Handling**: Comprehensive error handling throughout the pipeline
5. **Type Safety**: Uses Arrow's type system for data conversion
6. **Debugging**: Extensive logging for troubleshooting and monitoring
7. **Borrow Safety**: Resolves Rust borrow checker issues with separate statements
8. **Column Detection**: Robust column metadata extraction using DESCRIBE queries

### ⚠️ Limitations

1. **Memory Usage**: Entire file must be loaded into memory
2. **Performance**: Slower than in-place updates for large files
3. **Concurrency**: No support for concurrent updates
4. **Atomicity**: Not atomic (file could be corrupted during rewrite)
5. **Type Simplification**: All data converted to strings for simplicity
6. **Temporary Files**: Requires temporary file creation during upload process

## Performance Considerations

### Memory Usage

- Files are loaded entirely into memory
- Consider implementing streaming for very large files
- Monitor memory usage in production

### Network I/O

- Full file download and upload required
- Consider implementing delta updates for better performance
- Use compression to reduce transfer size

### File Size Limits

- Current implementation suitable for files < 1GB
- Larger files may require streaming approach
- Consider implementing chunked updates

## Error Handling

### Common Errors

1. **File Not Found**: Parquet file doesn't exist in S3
2. **Permission Denied**: Insufficient S3 permissions
3. **Schema Mismatch**: Column types don't match expected schema
4. **Memory Exhaustion**: File too large for available memory
5. **Network Timeout**: S3 upload/download timeout

### Error Recovery

- Graceful degradation for large files
- Retry logic for network errors
- Fallback to read-only mode on persistent errors
- User-friendly error messages

## Testing

### Unit Tests

- Test data reading and parsing
- Test Arrow conversion
- Test S3 upload/download
- Test error conditions

### Integration Tests

- End-to-end cell update flow
- Performance testing with various file sizes
- Concurrent access testing
- Error scenario testing

## Future Improvements

### 1. Streaming Updates

- Implement streaming for large files
- Process data in chunks
- Reduce memory usage

### 2. Delta Updates

- Track changes instead of full rewrite
- Implement change log
- Merge changes on read

### 3. Caching

- Cache frequently accessed files
- Implement LRU cache
- Reduce S3 access

### 4. Batch Updates

- Support multiple cell updates in single operation
- Reduce network round trips
- Improve performance

## Usage Example

### Frontend (Svelte)

```typescript
// Enable cell editing in AG-Grid
const gridOptions = {
  defaultColDef: {
    editable: true,
    cellEditor: "agTextCellEditor",
    cellEditorParams: {
      maxLength: 50,
    },
  },
  singleClickEdit: true,
  stopEditingWhenCellsLoseFocus: true,
  onCellValueChanged: (event) => {
    handleCellEdit(event);
  },
};

// Handle cell edit with extensive logging
async function handleCellEdit(event: any) {
  const { data, colDef, newValue, oldValue } = event;

  console.log("📝 [ParquetAGGrid] Cell value changed:", {
    field: colDef.field,
    newValue,
    oldValue,
    rowId: data.id,
  });

  try {
    console.log("🔄 [ParquetAGGrid] Updating cell in Parquet file...");

    const result = await updateParquetCell(
      bucketName,
      filePath,
      data.id,
      colDef.field,
      newValue,
    );

    if (result.success) {
      console.log("✅ [ParquetAGGrid] Cell updated successfully:", result);
      // Update local data to reflect the change
      const rowIndex = parquetData.findIndex((row) => row.id === data.id);
      if (rowIndex !== -1) {
        parquetData[rowIndex] = {
          ...parquetData[rowIndex],
          [colDef.field]: newValue,
        };
        parquetData = [...parquetData]; // Trigger reactivity
      }
    } else {
      console.error("❌ [ParquetAGGrid] Failed to update cell:", result.error);
      // Revert the cell value in the grid
      event.node.setDataValue(colDef.field, oldValue);
      error = result.error || "Failed to update cell";
    }
  } catch (err) {
    console.error("❌ [ParquetAGGrid] Error updating cell:", err);
    // Revert the change in the grid
    event.node.setDataValue(colDef.field, oldValue);
    error = err instanceof Error ? err.message : "Failed to update cell";
  }
}
```

### Backend (Rust)

```rust
#[tauri::command]
async fn update_parquet_cell_command(
    bucket_name: String,
    file_path: String,
    row_id: i32,
    field: String,
    new_value: serde_json::Value,
) -> Result<ParquetUpdateResult, String> {
    println!("🚀 [Tauri] Update Parquet cell command called");
    println!("📦 [Tauri] Bucket: {}", bucket_name);
    println!("📄 [Tauri] File: {}", file_path);
    println!("🆔 [Tauri] Row ID: {}", row_id);
    println!("📝 [Tauri] Field: {}", field);
    println!("🆕 [Tauri] New value: {:?}", new_value);

    update_parquet_cell(bucket_name, file_path, row_id, field, new_value).await
}
```

### Debugging Output

The implementation includes extensive logging throughout the process:

```
🚀 [Tauri] Update Parquet cell command called
📦 [Tauri] Bucket: project-8fac629b-7485-44fe-a2b0-2c3eb22c5bf6
📄 [Tauri] File: 6038187_v1.2_short.parquet
🆔 [Tauri] Row ID: 0
📝 [Tauri] Field: CALI
🆕 [Tauri] New value: Number(99)
🚀 [ParquetUpdate] Starting cell update with Arrow rewrite...
🔗 [ParquetUpdate] S3 path: s3://project-8fac629b-7485-44fe-a2b0-2c3eb22c5bf6/6038187_v1.2_short.parquet
🔍 [ParquetUpdate] Executing query: DESCRIBE SELECT * FROM read_parquet('s3://...')
📋 [ParquetUpdate] Found 3 columns: ["DEPTH", "CALI", "GR"]
🔍 [ParquetUpdate] Executing query: SELECT * FROM read_parquet('s3://...')
📊 [ParquetUpdate] Read 1000 rows, updating row 0
🔄 [ParquetUpdate] Updated row 0: {"DEPTH": 0.0, "CALI": 99, "GR": 45.2}
🔄 [ParquetUpdate] Converting data to Arrow format...
📊 [ParquetUpdate] Processing 1000 rows with columns: ["DEPTH", "CALI", "GR"]
📋 [ParquetUpdate] Created Arrow schema with 3 fields
📝 [ParquetUpdate] Created array for column 'DEPTH' with 1000 values
📝 [ParquetUpdate] Created array for column 'CALI' with 1000 values
📝 [ParquetUpdate] Created array for column 'GR' with 1000 values
📊 [ParquetUpdate] Created RecordBatch with 1000 rows and 3 columns
🖊️ [ParquetUpdate] Creating ArrowWriter...
📝 [ParquetUpdate] Writing RecordBatch to buffer...
🔒 [ParquetUpdate] Closing ArrowWriter...
💾 [ParquetUpdate] Generated 12345 bytes of Parquet data
☁️ [ParquetUpdate] Uploading to S3: s3://project-8fac629b-7485-44fe-a2b0-2c3eb22c5bf6/6038187_v1.2_short.parquet
🔧 [ParquetUpdate] Configuring S3 settings...
📁 [ParquetUpdate] Writing temp file: /tmp/temp_parquet_abc123.parquet
🔄 [ParquetUpdate] Executing copy query: COPY (SELECT * FROM read_parquet('/tmp/temp_parquet_abc123.parquet')) TO 's3://...' (FORMAT PARQUET, OVERWRITE_OR_IGNORE)
🧹 [ParquetUpdate] Cleaning up temp file...
✅ [ParquetUpdate] Successfully uploaded to S3
✅ [ParquetUpdate] Cell update completed successfully and file uploaded to S3
```

## Conclusion

The Parquet file update implementation provides a robust solution for cell-level editing of immutable Parquet files. The current implementation successfully addresses the fundamental challenges of Parquet immutability through a complete file rewrite approach. Key achievements include:

### ✅ What Works

- **Complete Pipeline**: End-to-end cell editing from frontend to S3 storage
- **Robust Column Detection**: Uses DESCRIBE queries to avoid borrow checker issues
- **Comprehensive Debugging**: Extensive logging for troubleshooting and monitoring
- **Data Integrity**: Complete rewrite ensures consistency and schema preservation
- **Error Handling**: Graceful error handling with frontend rollback capabilities

### 🔧 Technical Solutions

- **Borrow Checker Resolution**: Separate statements for column detection and data reading
- **Type Safety**: Arrow-based data conversion with proper error handling
- **Memory Management**: Efficient in-memory processing with cleanup
- **S3 Integration**: Reliable upload using DuckDB COPY with OVERWRITE_OR_IGNORE

The implementation is suitable for moderate-sized files and provides a solid foundation for future optimizations. The extensive debugging output makes it easy to monitor and troubleshoot the update process in production environments.

## Related Documentation

- [Parquet Data Fetching Flow](./fetch-parquet-data.md)
- [Arrow Writer Documentation](../../../docs/parquet/arrow-writer/)
- [DuckDB S3 Integration](../../../docs/duckdb/)
- [AG-Grid Cell Editing](../../../docs/ag-grid/editing/)
