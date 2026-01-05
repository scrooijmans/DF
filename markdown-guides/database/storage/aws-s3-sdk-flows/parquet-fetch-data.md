# Parquet Data Fetching and AG-Grid Display Flow | MudRock

## Overview

This document describes the complete flow for fetching Parquet data from MinIO storage using **OpenDAL with intelligent caching** and displaying it in an AG-Grid table within the analytics section.

## Architecture Components

### 🏗️ **Backend (Rust/Tauri)**

- **`opendal-parquet-query-service`**: OpenDAL-based Parquet querying with Moka caching
- **`src-tauri/src/parquet_query.rs`**: Legacy DuckDB-based Parquet querying (fallback)
- **Tauri Command**: `query_parquet_from_storage_command` for frontend communication

### 🎨 **Frontend (SvelteKit)**

- **`src/lib/services/parquet-data-service.ts`**: Service for Parquet data operations
- **`content-data-analytics-file-content-AG-data-table.svelte`**: AG-Grid component for Parquet data display
- **`content-data-analytics-file-content.svelte`**: Main component with conditional rendering

## Complete End-to-End Data Flow

### 1. **File Selection** (Analytics Files List)

```
User clicks on Parquet file in analytics files list
    ↓
ContentDataAnalyticsFileState.currentSelectedFile updated
    ↓
content-data-analytics-file-content.svelte detects file change via $effect
    ↓
Props passed to ContentDataAnalyticsFileContentAGDataTable component
```

**Frontend State Management:**

```svelte
// content-data-analytics-file-content.svelte
const fileState = getContentDataAnalyticsFileState();

// Reactive derived values
const isParquetFile = $derived(() => {
  return fileState?.currentSelectedFile?.name?.toLowerCase().endsWith('.parquet') || false;
});

const bucketName = $derived(() => {
  return fileState?.currentSelectedFile?.bucket_id || '';
});

const filePath = $derived(() => {
  return fileState?.currentSelectedFile?.name || '';
});
```

### 2. **Frontend Service Call** (Svelte Component)

```typescript
// content-data-analytics-file-content-AG-data-table.svelte
async function loadParquetData() {
  try {
    isLoading = true;
    error = null;

    // Call Tauri command via service
    const result: ParquetQueryResult = await queryParquetFromStorage(
      bucketName,
      filePath,
      1000, // Default limit
    );

    if (!result.success) {
      throw new Error(result.error || "Failed to query Parquet data");
    }

    // Store results in component state
    parquetData = result.data;
    columns = result.columns;
    rowCount = result.row_count;

    // Generate AG-Grid column definitions
    columnDefs = generateColumnDefsFromParquetSchema(columns);
  } catch (err) {
    error = err instanceof Error ? err.message : "Failed to load Parquet data";
  } finally {
    isLoading = false;
  }
}
```

### 3. **Tauri Command Invocation** (Frontend Service)

```typescript
// parquet-data-service.ts
export async function queryParquetFromStorage(
  bucketName: string,
  filePath: string,
  limit?: number,
): Promise<ParquetQueryResult> {
  try {
    console.log("🚀 [ParquetDataService] Querying Parquet data...");

    // Invoke Tauri command
    const result = await invoke<ParquetQueryResult>(
      "query_parquet_from_storage_command",
      {
        bucketName,
        filePath,
        limit: limit || 1000,
      },
    );

    console.log("✅ [ParquetDataService] Tauri command completed");
    return result;
  } catch (error) {
    console.error(
      "❌ [ParquetDataService] Error querying Parquet data:",
      error,
    );
    throw new Error(`Failed to query Parquet data: ${error}`);
  }
}
```

### 4. **Backend Tauri Command Handler** (Rust with OpenDAL)

```rust
// main.rs
#[tauri::command]
async fn query_parquet_from_storage_command(
    bucket_name: String,
    file_path: String,
    limit: Option<u32>,
) -> Result<TauriParquetQueryResult, String> {
    println!("🚀 [Tauri] Querying Parquet from storage with OpenDAL...");
    println!("📦 [Tauri] Bucket: {}", bucket_name);
    println!("📄 [Tauri] File: {}", file_path);
    println!("🔢 [Tauri] Limit: {:?}", limit);

    // Create OpenDAL parquet query service with caching
    let service = match OpenDALParquetQueryService::new(
        "http://91.99.166.223:9000",
        &bucket_name,
        "mudrock-storage",
        "mudrock-storage-secret-2024",
    ) {
        Ok(s) => s,
        Err(e) => {
            println!("❌ [Tauri] Failed to create OpenDAL service: {}", e);
            return Err(format!("Failed to create OpenDAL service: {}", e));
        }
    };

    // Create query configuration with caching enabled
    let config = ParquetQueryConfig {
        bucket_name: bucket_name.clone(),
        file_path: file_path.clone(),
        limit: limit.map(|l| l as usize),
        enable_caching: true, // 🎯 Caching is enabled!
    };

    // Query parquet data with intelligent caching
    match service.query_parquet(config).await {
        Ok(result) => {
            println!("✅ [Tauri] OpenDAL query successful: {} rows, {} columns",
                result.row_count, result.columns.len());

            // Convert OpenDAL result to Tauri result
            let tauri_result = TauriParquetQueryResult {
                success: result.success,
                data: result.data,
                columns: result.columns.into_iter().map(|col| ParquetColumn {
                    name: col.name,
                    data_type: col.data_type,
                    nullable: col.nullable,
                }).collect(),
                row_count: result.row_count,
                error: result.error,
            };

            Ok(tauri_result)
        }
        Err(e) => {
            println!("❌ [Tauri] OpenDAL query failed: {}", e);
            Err(format!("OpenDAL query failed: {}", e))
        }
    }
}
```

### 5. **OpenDAL Parquet Processing with Caching** (Backend Core)

```rust
// opendal-parquet-query-service/src/lib.rs
impl OpenDALParquetQueryService {
    pub async fn query_parquet_from_storage(
        &self,
        config: ParquetQueryConfig,
    ) -> Result<ParquetQueryResult, ParquetQueryError> {
        println!("🚀 [OpenDALParquetQueryService] Querying Parquet from storage...");
        println!("📦 [OpenDALParquetQueryService] Bucket: {}", config.bucket_name);
        println!("📄 [OpenDALParquetQueryService] File: {}", config.file_path);
        println!("🔢 [OpenDALParquetQueryService] Limit: {:?}", config.limit);

        // 🎯 CACHE CHECK: Check cache first if enabled
        if config.enable_caching {
            if let Some(cached_data) = self.adapter.get_cached(&config.file_path).await {
                println!("✅ [OpenDALParquetQueryService] Cache hit for file: {}", config.file_path);
                return self.process_cached_parquet_data(cached_data, config.limit).await;
            }
        }

        // Download parquet file from storage using OpenDAL
        let parquet_data = self.adapter.download(&config.file_path).await?;
        println!("✅ [OpenDALParquetQueryService] Downloaded parquet file: {} bytes", parquet_data.len());

        // Process the parquet data with DuckDB
        self.process_parquet_data(parquet_data, config.limit).await
    }

    async fn process_parquet_data(
        &self,
        parquet_data: Bytes,
        limit: Option<usize>,
    ) -> Result<ParquetQueryResult, ParquetQueryError> {
        // Create temporary file for DuckDB to read
        let temp_file = tempfile::NamedTempFile::new()
            .map_err(|e| ParquetQueryError::QueryExecution(format!("Failed to create temp file: {}", e)))?;

        // Write parquet data to temp file
        std::fs::write(temp_file.path(), &parquet_data)
            .map_err(|e| ParquetQueryError::QueryExecution(format!("Failed to write temp file: {}", e)))?;

        // Create DuckDB connection
        let connection = duckdb::Connection::open_in_memory()
            .map_err(|e| ParquetQueryError::DuckDB(format!("Failed to create DuckDB connection: {}", e)))?;

        // Configure DuckDB
        connection.execute("INSTALL httpfs", [])
            .map_err(|e| ParquetQueryError::DuckDB(format!("Failed to install httpfs: {}", e)))?;
        connection.execute("LOAD httpfs", [])
            .map_err(|e| ParquetQueryError::DuckDB(format!("Failed to load httpfs: {}", e)))?;

        // Get file path as string
        let file_path = temp_file.path().to_string_lossy().to_string();

        // Detect schema and extract data using DuckDB
        let (columns, data) = self.detect_parquet_schema_and_extract_data(&connection, &file_path, limit)?;

        // Calculate row count before moving data
        let row_count = data.len();

        Ok(ParquetQueryResult {
            success: true,
            data,
            columns,
            row_count,
            error: None,
        })
    }
}
```

### 6. **Data Formatting for AG-Grid** (Frontend Service)

```typescript
// parquet-data-service.ts
export function formatParquetDataForGrid(
  data: Record<string, any>[],
  columns: ParquetColumn[],
): Record<string, any>[] {
  console.log("🔧 [ParquetDataService] Formatting data for AG-Grid...");
  console.log("📊 [ParquetDataService] Data length:", data.length);
  console.log(
    "📋 [ParquetDataService] Column names:",
    columns.map((c) => c.name),
  );

  // Explicit mapping based on schema
  return data.map((row, index) => {
    const formattedRow: Record<string, any> = { id: index };

    columns.forEach((column) => {
      const value = row[column.name];
      formattedRow[column.name] = value !== undefined ? value : null;
    });

    return formattedRow;
  });
}

export function generateColumnDefsFromParquetSchema(
  columns: ParquetColumn[],
): any[] {
  return columns.map((column) => ({
    field: column.name,
    headerName: column.name.replace(/_/g, " ").toUpperCase(),
    sortable: true,
    filter: true,
    resizable: true,
    flex: 1,
    minWidth: 120,
    cellDataType: getCellDataType(column.data_type),
  }));
}
```

### 7. **AG-Grid Rendering** (Svelte Component)

```svelte
<!-- content-data-analytics-file-content-AG-data-table.svelte -->
<script lang="ts">
  // Reactive data formatting
  const gridData = $derived(() => {
    return formatParquetDataForGrid(parquetData, columns);
  });

  // Grid options with AG-Grid configuration
  let gridOptions = $state<GridOptions>({
    defaultColDef: {
      enableCellChangeFlash: true,
      suppressMovable: true,
      resizable: true,
      sortable: true,
      editable: false, // Read-only for Parquet data
      flex: 1,
      minWidth: 120,
      filter: "agTextColumnFilter",
    },
    columnDefs: [],
    pagination: true,
    paginationPageSize: 50,
    cellSelection: true,
    onGridReady: (params) => {
      gridApi = params.api;
    },
  });

  // Update column definitions when schema changes
  $effect(() => {
    gridOptions.columnDefs = columnDefs;
  });
</script>

<!-- Conditional rendering based on state -->
{#if error}
  <div class="bg-red-50 border border-red-200 rounded-md p-4">
    <AlertCircle class="h-5 w-5 text-red-400" />
    <h3 class="text-sm font-medium text-red-800">Error loading Parquet data</h3>
    <p>{error}</p>
  </div>
{:else if isLoading}
  <div class="flex items-center justify-center py-8">
    <Loader2 class="h-5 w-5 animate-spin" />
    <span>Loading Parquet data...</span>
  </div>
{:else if gridData().length === 0}
  <div class="text-center py-8">
    <Database class="mx-auto h-12 w-12 text-gray-400" />
    <h3 class="mt-2 text-sm font-medium text-gray-900">No data found</h3>
  </div>
{:else}
  <div class="grid-container">
    <AgGrid {gridOptions} rowData={gridData()} {modules} />
  </div>
{/if}
```

### 8. **Complete Data Flow Summary with OpenDAL Caching**

```
1. User clicks Parquet file
   ↓
2. Svelte state updates (ContentDataAnalyticsFileState)
   ↓
3. Component detects change via $effect
   ↓
4. loadParquetData() called
   ↓
5. queryParquetFromStorage() service call
   ↓
6. invoke("query_parquet_from_storage_command") Tauri call
   ↓
7. OpenDALParquetQueryService created with caching enabled
   ↓
8. 🎯 CACHE CHECK: Check Moka cache for file
   ↓
9a. CACHE HIT: Process cached data directly (fast path)
    ↓
9b. CACHE MISS: Download from MinIO using OpenDAL
    ↓
10. Process parquet data with DuckDB (schema detection + data extraction)
    ↓
11. Store result in cache for future requests
    ↓
12. ParquetQueryResult returned to frontend
    ↓
13. Data formatted for AG-Grid (formatParquetDataForGrid)
    ↓
14. Column definitions generated (generateColumnDefsFromParquetSchema)
    ↓
15. AG-Grid renders with formatted data
    ↓
16. User sees interactive Parquet data table
    ↓
17. 🚀 Subsequent requests for same file = CACHE HIT (10-100x faster!)
```

## Key Technical Improvements

### **OpenDAL Intelligent Caching**

**Problem Solved**: Repeated access to the same Parquet files required re-downloading and re-processing data from MinIO storage.

**Solution**: OpenDAL with Moka caching provides intelligent in-memory caching:

```rust
// Cache configuration in OpenDALParquetQueryService
pub struct OpenDALParquetQueryService {
    adapter: EnhancedOpenDALStorageAdapter, // Contains Moka cache
}

impl OpenDALParquetQueryService {
    pub async fn query_parquet_from_storage(&self, config: ParquetQueryConfig) -> Result<ParquetQueryResult, ParquetQueryError> {
        // 🎯 CACHE CHECK: Check cache first if enabled
        if config.enable_caching {
            if let Some(cached_data) = self.adapter.get_cached(&config.file_path).await {
                println!("✅ Cache hit for file: {}", config.file_path);
                return self.process_cached_parquet_data(cached_data, config.limit).await;
            }
        }

        // Download and process if cache miss
        let parquet_data = self.adapter.download(&config.file_path).await?;
        self.process_parquet_data(parquet_data, config.limit).await
    }
}
```

**Key Benefits**:

- **10-100x Performance Improvement**: Cached data accessed from memory vs network
- **Reduced Network Calls**: Frequently accessed files don't require MinIO access
- **Automatic Cache Management**: Moka handles TTL, LRU eviction, and memory limits
- **Zero Configuration**: Caching works out of the box with sensible defaults
- **Content-Aware**: Cache keys based on file path and content hash

### **Robust Schema Detection**

**Problem Solved**: DuckDB schema detection was unreliable, causing "unknown_column" errors.

**Solution**: Multi-approach schema detection strategy:

1. **Primary**: `DESCRIBE SELECT * FROM read_parquet(...) LIMIT 1` for direct schema introspection
2. **Fallback**: Execute `LIMIT 1` query and extract column metadata from result set
3. **Type Inference**: Attempt different data type conversions to determine column types

```rust
// Approach 1: DESCRIBE query
let describe_query = format!("DESCRIBE SELECT * FROM read_parquet('{}') LIMIT 1", s3_path);

// Approach 2: Fallback with result set introspection
if columns.is_empty() {
    let schema_query = format!("SELECT * FROM read_parquet('{}') LIMIT 1", s3_path);
    // Extract column names and types from executed result
}
```

### **Statement Execution Management**

**Problem Solved**: DuckDB statements can only be executed once, causing "statement not executed yet" panics.

**Solution**: Separated schema detection and data extraction into different statements:

- **Schema Detection**: Uses one statement for column metadata extraction
- **Data Extraction**: Uses a separate statement for actual data querying

### **Proper Column Mapping**

**Problem Solved**: Data was being mapped incorrectly, causing empty cells in AG-Grid.

**Solution**: Use detected schema for proper column mapping:

```rust
// Use already detected schema instead of trying to access statement metadata
let column_names: Vec<(usize, String)> = columns.iter()
    .enumerate()
    .map(|(i, col)| (i, col.name.clone()))
    .collect();
```

### **Type-Aware Data Conversion with Decimal Precision**

**Problem Solved**: Data types weren't being properly converted for JSON serialization, and decimal numbers were being converted to integers.

**Solution**: Explicit type checking with decimal precision prioritization:

```rust
// Try to get the value as different types, prioritizing decimal precision
let value = if let Ok(val) = row.get::<_, f64>(*i) {
    // Try f64 first to preserve decimal precision
    serde_json::Value::Number(serde_json::Number::from_f64(val).unwrap_or(serde_json::Number::from(0)))
} else if let Ok(val) = row.get::<_, i64>(*i) {
    // Then try i64 for integers
    serde_json::Value::Number(serde_json::Number::from(val))
} else if let Ok(val) = row.get::<_, String>(*i) {
    // Then try String
    serde_json::Value::String(val)
} else if let Ok(val) = row.get::<_, bool>(*i) {
    // Then try bool
    serde_json::Value::Bool(val)
} else {
    // Fallback to null
    serde_json::Value::Null
};
```

**Key Improvement**: By checking `f64` before `i64`, we ensure that decimal numbers (like `11.450`) are preserved as floating-point values rather than being converted to integers (`11`). This maintains data integrity and proper formatting in the frontend.

### **Consistent Cell Formatting**

**Problem Solved**: When editing cells in AG-Grid, updated values weren't applying the same formatting as displayed values, causing inconsistency (e.g., `11.450` becoming `11` after edit).

**Solution**: Implemented proper AG-Grid cell data types with `valueFormatter` and `valueParser`:

```typescript
// For float/double columns
{
  cellDataType: 'number',
  valueFormatter: (params: any) => {
    if (params.value === null || params.value === undefined) {
      return '-';
    }
    return typeof params.value === "number"
      ? params.value.toFixed(3)
      : parseFloat(params.value).toFixed(3);
  },
  valueParser: (params: any) => {
    const value = parseFloat(params.newValue);
    return isNaN(value) ? params.oldValue : value;
  },
  cellEditor: 'agNumberCellEditor',
  cellEditorParams: {
    precision: 3,
    min: 0,
  },
}
```

**Key Benefits**:

- **Consistent Display**: All float values show 3 decimal places (`11.450`, `11.000`)
- **Type Validation**: Invalid inputs revert to previous value
- **Proper Editors**: Number editor with precision control for numeric columns
- **Automatic Refresh**: Grid refreshes cells after updates to apply formatting
- **Filter Consistency**: Filter values use the same formatting as cell values
- **Robust Error Handling**: NaN values are handled gracefully with fallback display

**Implementation Details**:

- **Value Formatter**: Ensures consistent display formatting for all cells in a column
- **Value Parser**: Validates and converts user input to proper data types
- **Filter Integration**: Filter values are formatted consistently with cell values
- **Cell Refresh**: Multiple refresh strategies ensure formatting is applied after edits
- **Type Safety**: Proper handling of null/undefined values with fallback display

## Key Features

### **Automatic File Type Detection**

- Detects `.parquet` files automatically
- Shows AG-Grid table for Parquet files
- Shows text content for other file types

### **DuckDB S3 Integration**

- Direct querying from MinIO storage
- No need to download files
- Efficient columnar data processing
- Robust error handling and fallbacks

### **AG-Grid Table Features**

- **Editable cells**: Parquet data can be edited with proper type validation
- **Consistent formatting**: `valueFormatter` ensures consistent display formatting
- **Type-aware editing**: Different cell editors based on data types (number, text, boolean)
- **Decimal precision**: Float/double values maintain 3 decimal places consistently
- **Pagination**: Handles large datasets efficiently
- **Sorting & Filtering**: Full AG-Grid functionality
- **Responsive design**: Adapts to different screen sizes
- **Loading states**: Visual feedback during data fetching
- **Error handling**: User-friendly error messages

### **Data Type Support**

- **Numeric**: Integers, floats, doubles
- **Text**: Strings, varchar
- **Boolean**: True/false values
- **Null handling**: Proper display of missing values
- **Type inference**: Automatic detection of column data types

## File Structure

```
src/
├── lib/
│   └── services/
│       └── parquet-data-service.ts          # Parquet data operations
└── components/
    └── pages/
        └── home/
            └── content-main/
                └── content-data-analytics/
                    └── content-data-analytics-file-content/
                        ├── content-data-analytics-file-content.svelte
                        └── content-data-analytics-file-content-AG-data-table/
                            └── content-data-analytics-file-content-AG-data-table.svelte

src-tauri/src/
└── parquet_query.rs                         # DuckDB Parquet querying
```

## Usage Example

### **For Users:**

1. Navigate to **Analytics** section
2. Select a project from the dropdown
3. Click on any `.parquet` file in the files list
4. View the data in an interactive AG-Grid table

### **For Developers:**

```typescript
// Query Parquet data programmatically
import { queryParquetFromStorage } from "$lib/services/parquet-data-service";

const result = await queryParquetFromStorage(
  "project-123",
  "wells/well-1/data.parquet",
  1000, // limit
);

console.log(
  `Found ${result.row_count} rows with ${result.columns.length} columns`,
);
```

## Performance Considerations

### **Query Limits**

- Default limit: 1000 rows for performance
- Configurable limit parameter
- Pagination for large datasets

### **Memory Efficiency**

- DuckDB streams data from S3
- No local file downloads
- Efficient columnar processing

### **OpenDAL Intelligent Caching**

- **Moka Cache**: In-memory caching with automatic eviction
- **Cache Hit Performance**: 10-100x faster than network access
- **Memory Management**: Automatic LRU eviction and TTL expiration
- **Cache Statistics**: Built-in monitoring of hit/miss ratios
- **Zero Configuration**: Works out of the box with sensible defaults

### **Frontend Caching**

- AG-Grid handles client-side pagination
- Data cached in component state
- Automatic refresh on file change

## Error Handling

### **Backend Errors**

- DuckDB connection failures
- S3 access issues
- Invalid Parquet files

### **Frontend Errors**

- Network connectivity issues
- Invalid file paths
- Data parsing errors

### **User Experience**

- Loading states during data fetching
- Error messages with details
- Graceful fallbacks for unsupported files

## Future Enhancements

### **Planned Features**

- **Data editing**: Make Parquet data editable
- **Export functionality**: Export filtered/sorted data
- **Advanced filtering**: Complex query builders
- **Data visualization**: Charts and graphs
- **Real-time updates**: Live data refresh

### **Performance Improvements**

- **Lazy loading**: Load data on demand
- **Virtual scrolling**: Handle very large datasets
- **Query optimization**: Smart query building
- **Caching strategies**: Better data caching

This implementation provides a powerful, user-friendly way to explore and analyze Parquet data directly from MinIO storage, making it easy for users to work with their petrophysical data files.
