# Parquet File Schema for MudRock

## Overview

This document describes the schema and structure of Parquet files stored in MudRock's MinIO storage system. Parquet files are created from LAS files during data ingestion and represent well log data in a columnar format optimized for analytical queries.

## Schema Design: Wide Format (Columnar)

**MudRock uses a wide-format (columnar) schema** similar to LAS files, where:

- **Each row** represents a single depth point (measurement depth)
- **Each column** represents a curve (e.g., GR, RHOB, DT, DEPT)
- **Data is stored columnar** (similar to LAS ASCII format)

### Example Structure

```
Row 1: DEPT=48.0000, GR=45.2, RHOB=2.65, DT=120.5, ...
Row 2: DEPT=48.1500, GR=46.1, RHOB=2.66, DT=121.0, ...
Row 3: DEPT=48.3000, GR=47.3, RHOB=2.67, DT=121.2, ...
```

This matches the LAS file format shown in the screenshot, where each row represents a depth point and columns represent different curves.

## Implementation Details

### File Creation (`las_data_processor.rs`)

**Location**: `src-tauri/src/las_processing/las_data_processor.rs`

**Process**:

1. **Create Arrow Arrays**: One array per curve column

   ```rust
   // Add well name as first column
   let well_name_array = StringArray::from(vec![well_name; data_points]);

   // Add each curve as a column
   for column in &las_file.curves.columns {
       let curve_array = Float64Array::from(values);
       arrays.push(Arc::new(curve_array));
       field_names.push(column.mnemonic.clone());
   }
   ```

2. **Create Schema**: One field per curve

   ```rust
   fields.push(arrow::datatypes::Field::new("well_name", DataType::Utf8, false));
   for column in &las_file.curves.columns {
       fields.push(arrow::datatypes::Field::new(
           &column.mnemonic,
           DataType::Float64,
           true, // nullable for missing values
       ));
   }
   ```

3. **Create RecordBatch**: Wide format with all columns

   ```rust
   let record_batch = RecordBatch::try_new(
       Arc::new(schema),
       arrays, // One array per column
   )?;
   ```

4. **Write to Parquet**: Columnar storage format
   ```rust
   let mut writer = ArrowWriter::try_new(&mut buffer, record_batch.schema(), None)?;
   writer.write(&record_batch)?;
   ```

### Storage Path

**Format**: `project-{project_id}/wells/{well_database_id}/logs_{log_type}.parquet`

**Example**: `project-c11134a9-b9bc-49f4-b432-84aec9ea0388/wells/47/logs_composite.parquet`

### Database Mapping

**`curves` Table**:

- `parquet_file_path`: Path to the Parquet file (e.g., `wells/47/logs_composite.parquet`)
- `parquet_column_name`: Column name in the Parquet file (e.g., `DT`, `GR`, `RHOB`)
- Each curve entry points to a specific column in a Parquet file

## Data Access Patterns

### Columnar Access (Primary Use Case)

**Extract all values for a specific curve** (e.g., Gamma Ray):

```typescript
// Query parquet file and extract GR column
const result = await queryParquetFromStorage(bucketName, filePath);
const grValues = result.data.map((row) => row.GR);
```

**Benefits**:

- ✅ **Efficient**: Parquet's columnar format allows reading only the needed columns
- ✅ **Fast**: Columnar storage is optimized for analytical queries
- ✅ **Similar to LAS**: Matches the structure of LAS files (columns = curves)

### Row-Based Access (Secondary Use Case)

**Extract all curves for a specific depth**:

```typescript
// Query parquet file and filter by depth
const result = await queryParquetFromStorage(bucketName, filePath);
const depthRow = result.data.find((row) => row.DEPT === 48.0);
// depthRow contains: { DEPT: 48.0000, GR: 45.2, RHOB: 2.65, DT: 120.5, ... }
```

**Benefits**:

- ✅ **Complete Depth Point**: All curve values for a single depth in one row
- ✅ **Easy Filtering**: Can filter by depth to get all curves at that depth

## Comparison with LAS Files

### LAS File Format (ASCII)

```
~ASCII LOG DATA
48.0000  45.2  2.65  120.5  ...
48.1500  46.1  2.66  121.0  ...
48.3000  47.3  2.67  121.2  ...
```

**Structure**:

- Each row = one depth point
- Each column = one curve
- Column order matches curve definition order

### Parquet File Format (Binary)

**Structure** (conceptually the same):

- Each row = one depth point
- Each column = one curve
- Column names = curve mnemonics (DEPT, GR, RHOB, DT, etc.)

**Differences**:

- ✅ **Binary format**: More efficient storage and faster reads
- ✅ **Columnar storage**: Optimized for analytical queries
- ✅ **Compression**: Built-in compression (Zstd/Gzip)
- ✅ **Type safety**: Strongly typed columns (Float64, Utf8, etc.)

## Why Wide Format (Not Long Format)?

### Wide Format (Current Implementation)

```
DEPT    | GR   | RHOB | DT
--------|------|------|-----
48.0000 | 45.2 | 2.65 | 120.5
48.1500 | 46.1 | 2.66 | 121.0
48.3000 | 47.3 | 2.67 | 121.2
```

**Advantages**:

- ✅ **Matches LAS format**: Direct mapping from LAS to Parquet
- ✅ **Easy column extraction**: Read entire curve column efficiently
- ✅ **Familiar structure**: Same as LAS files users are familiar with
- ✅ **Efficient for charts**: Easy to extract X (depth) and Y (curve) columns

### Long Format (Alternative, Not Used)

```
depth   | curve_name | value
--------|------------|-------
48.0000 | GR         | 45.2
48.0000 | RHOB       | 2.65
48.0000 | DT         | 120.5
48.1500 | GR         | 46.1
48.1500 | RHOB       | 2.66
48.1500 | DT         | 121.0
```

**Disadvantages** (for our use case):

- ❌ **More complex queries**: Need to filter by curve_name
- ❌ **Less efficient**: More rows, more storage
- ❌ **Different from LAS**: Doesn't match LAS file structure

## Query Examples

### Extract Single Curve Column

```typescript
// Get all Gamma Ray values for a well
const result = await queryParquetFromStorage(
  "project-xxx",
  "wells/47/logs_composite.parquet",
  1000,
);

// Extract GR column
const grValues = result.data.map((row) => row.GR);
const depths = result.data.map((row) => row.DEPT);

// Use for charting
chartData = depths.map((depth, i) => ({
  x: depth,
  y: grValues[i],
}));
```

### Extract Multiple Curves for Depth Range

```typescript
// Get all curves for depths between 48.0 and 50.0
const result = await queryParquetFromStorage(
  "project-xxx",
  "wells/47/logs_composite.parquet",
  1000,
);

const filteredData = result.data.filter(
  (row) => row.DEPT >= 48.0 && row.DEPT <= 50.0,
);

// filteredData contains all curves (GR, RHOB, DT, etc.) for that depth range
```

### Extract All Curves for Single Depth

```typescript
// Get all curve values at depth 48.0000
const result = await queryParquetFromStorage(
  "project-xxx",
  "wells/47/logs_composite.parquet",
  1000,
);

const depthPoint = result.data.find((row) => row.DEPT === 48.0);
// depthPoint = { DEPT: 48.0000, GR: 45.2, RHOB: 2.65, DT: 120.5, ... }
```

## Storage Efficiency

### Columnar Storage Benefits

1. **Compression**: Similar values in columns compress well (e.g., depth values are sequential)
2. **Selective Reading**: Can read only needed columns without reading entire file
3. **Vectorization**: Columnar format enables SIMD operations for faster processing
4. **Analytical Queries**: Optimized for aggregations, filtering, and analytical operations

### File Size Comparison

**Wide Format** (current):

- One row per depth point
- One column per curve
- Example: 1000 depth points × 7 curves = 1000 rows × 7 columns

**Long Format** (alternative):

- One row per depth-curve combination
- Example: 1000 depth points × 7 curves = 7000 rows × 3 columns

**Result**: Wide format is more storage-efficient for our use case.

## Database Schema Integration

### `curves` Table

Each curve entry in the database points to:

- **`parquet_file_path`**: The Parquet file containing the curve data
- **`parquet_column_name`**: The column name in that Parquet file

**Example**:

```sql
-- Well 47 has a Gamma Ray curve
INSERT INTO curves (
  well_id,
  curve_id,
  curve_name,
  curve_metadata_id,
  parquet_file_path,
  parquet_column_name
) VALUES (
  47,
  'GR',
  'GR',
  'uuid-of-gr-metadata',
  'wells/47/logs_composite.parquet',
  'GR'  -- Column name in parquet file
);
```

### Multiple Curves in Same File

**One Parquet file** can contain **multiple curves**:

```
wells/47/logs_composite.parquet:
  - Column: DEPT (depth)
  - Column: GR (Gamma Ray)
  - Column: RHOB (Density)
  - Column: DT (Sonic)
  - Column: AI (Acoustic Impedance)
  - ... (more curves)
```

**Multiple `curves` entries** point to the same file but different columns:

```sql
-- GR curve
parquet_file_path: 'wells/47/logs_composite.parquet'
parquet_column_name: 'GR'

-- RHOB curve
parquet_file_path: 'wells/47/logs_composite.parquet'
parquet_column_name: 'RHOB'

-- DT curve
parquet_file_path: 'wells/47/logs_composite.parquet'
parquet_column_name: 'DT'
```

## Parquet File Viewer Flow

### Component Architecture

The Parquet File Viewer follows a clear data flow pattern:

```
parquet-file-viewer.svelte (orchestrator)
    ├── parquet-curves.svelte (left sidebar - 30% width)
    │   └── ChartDataCurveOptionsAGTable (displays curves with parquet files)
    │       └── onRowClick → handleCurveRowClick()
    │           └── onCurveSelect(curve) callback
    │
    └── ag-parquet-data-viewer.svelte (right side - 70% width)
        └── $effect(() => { loadParquetData() })
            └── queryParquetFromStorage(bucketName, filePath, limit)
                └── Tauri command: query_parquet_from_storage_command
                    └── Returns: { data: Array<Record<string, any>>, columns: ParquetColumn[] }
```

### Data Flow Steps

1. **Curve Selection** (`parquet-curves.svelte`):
   - User clicks a curve row in the AG Grid table
   - `handleCurveRowClick(curveId)` is called
   - Finds curve from `curveOptions` array
   - Calls `onCurveSelect()` callback with curve metadata:
     ```typescript
     {
       id: string,
       parquet_file_path: string,
       parquet_column_name: string,
       well_id: number,
       curve_name: string
     }
     ```

2. **Data Loading** (`ag-parquet-data-viewer.svelte`):
   - Receives `curve` prop from parent
   - `$effect` watches `curve` and `currentProjectId`
   - When curve changes, calls `loadParquetData()`
   - Constructs bucket name: `project-${currentProjectId}`
   - Calls `queryParquetFromStorage(bucketName, filePath, limit)`

3. **Parquet Query** (`parquet-data-service.ts`):
   - Invokes Tauri command: `query_parquet_from_storage_command`
   - Returns full parquet file data (all columns, all rows up to limit)
   - Data structure: `Array<Record<string, any>>` where each record is a row
   - Example row: `{ DEPT: 48.0000, GR: 45.2, RHOB: 2.65, DT: 120.5, ... }`

4. **Data Display** (`ag-parquet-data-viewer.svelte`):
   - Generates AG Grid column definitions from `result.columns`
   - Displays all rows and columns in AG Grid table
   - User can see all curve data for the selected curve's parquet file

### Key Pattern: Full File Loading

**Important**: The Parquet File Viewer loads the **entire parquet file** (all columns, all rows), not just the selected curve column. This allows users to:

- See all curves in the same file
- Compare multiple curves side-by-side
- Understand the full data structure

**Contrast with Chart Data Loading**:

- Charts only need specific curve columns (X and Y axes)
- Charts use `curve-data-service.ts` which:
  1. Fetches metadata to get `parquet_file_path` and `parquet_column_name`
  2. Queries parquet file (gets all columns, but extracts only needed ones)
  3. Extracts specific columns (DEPT + curve column)
  4. Aligns multiple curves by DEPT for plotting

## Chart Data Loading Pattern (Based on Parquet Viewer)

### Recommended Flow for Charts

Charts should follow a similar pattern to the Parquet File Viewer:

```
chart-data-curve-options.svelte (user selects curves)
    ↓ Updates chart_config.series[] in database
    ↓
realtime update → PostgresChartsState.updateChart()
    ↓
PostgresChartsState.loadChartState(chart)
    ↓
XYPlotState.updateChart(chart)
    ↓
XYPlotState.loadSeriesData(chart)
    ↓
curve-data-service.ts:
    1. fetchCurveMetadata(curveId) → Get parquet_file_path, parquet_column_name
    2. queryParquetFromStorage(bucketName, filePath) → Get full file data
    3. Extract DEPT and curve column values
    4. alignCurveData(xCurve, yCurve) → Align by DEPT
    ↓
XYPlotState: Update SciChart renderableSeries with aligned data
    ↓
sciChartSurface.zoomExtents() → Auto-fit visible range to data
```

### Critical Implementation Details

1. **Always Load Full Parquet File**: Like the Parquet File Viewer, charts should query the entire parquet file (not column-specific queries). This ensures we have all DEPT values for alignment.

2. **Extract Columns After Loading**: After getting the full file data, extract only the needed columns (DEPT + X curve + Y curve).

3. **Align by DEPT**: Use the DEPT column to align curves from the same well_id, ensuring only overlapping depth points are plotted.

4. **Auto-Fit After Loading**: Call `zoomExtents()` after adding series data to automatically fit the visible range to the data.

5. **Reactive Updates**: Use `$effect` in `XYPlotState.updateChart()` to automatically reload data when `chart.chart_config.series` changes.

## Summary

**MudRock Parquet Schema**:

- ✅ **Wide Format**: Each row = depth point, each column = curve (matches LAS format)
- ✅ **Columnar Storage**: Optimized for extracting entire curve columns
- ✅ **Efficient**: Can read specific columns without reading entire file
- ✅ **Familiar**: Same structure as LAS files users are familiar with
- ✅ **Database Integration**: Each curve entry points to a specific column in a Parquet file

**Primary Use Case**: Extract all values for a specific curve (e.g., all Gamma Ray values)
**Secondary Use Case**: Extract all curves for a specific depth point

**Parquet File Viewer Pattern**: Load entire parquet file → Display all columns/rows in AG Grid
**Chart Data Pattern**: Load entire parquet file → Extract specific columns → Align by DEPT → Plot

This design matches the LAS file format shown in the screenshot, where each row represents a depth point and columns represent different curves (DEPT, RHOB, DT, GR, AI, AI_rel, PHIE).
