# LAS File Upload Implementation in MudRock

This document describes the complete implementation of LAS file upload functionality in MudRock, including curve mapping, unit conversion, and database integration.

## 🏗️ Architecture Overview

```
Frontend (SvelteKit)
    ↓ LAS File Drop
ContentDataIngestionDropzone
    ↓ parseLasFileForConfirmation()
Tauri Command (Rust Backend)
    ↓ LAS Parsing & Detection
LasUploadManager
    ↓ User Confirmation UI
ContentDataIngestionLogsProcessing
    ├─ Well Selection (existing or new)
    ├─ Curve Mapping (curve-mapping.svelte)
    │   ├─ Map to curve_metadata table
    │   └─ Select source units
    └─ Data Review (AG-Grid tables)
    ↓ User Confirms
process_and_upload_las_data()
    ↓ Backend Processing Pipeline
LasDataProcessor
    ├─ Step 1: Well Creation/Selection (Postgres wells table)
    ├─ Step 2: Unit Conversion (unit-conversions crate)
    ├─ Step 3: Parquet Generation (Arrow/Parquet)
    ├─ Step 4: MinIO Storage (OpenDAL)
    └─ Step 5: Curve Entry Creation (Postgres curves table)
    ↓ Success Notification
```

## 📋 Implementation Details

### Phase 1: File Drop & Parsing

1. **User drops LAS file** into `ContentDataIngestionDropzone`
2. **File validation** checks file extension and size
3. **LAS parsing** calls `parse_las_file_for_confirmation` Tauri command
4. **Backend processing** (`LasUploadManager`) parses LAS file:
   - Extracts well metadata (name, field, company, location, UWI)
   - Parses curve data with values
   - Detects curve types with confidence scores
5. **Result returned** to frontend as `LasProcessingResult`

### Phase 2: User Confirmation & Curve Mapping

#### Well Selection

**Component**: `content-data-ingestion-logs-processing.svelte`

- User selects existing well or "New Well"
- Options loaded from `PostgresWellsState` global state
- Selected well info stored: `{ well_id: string, well_name: string | null }`

#### Curve Mapping

**Component**: `curve-mapping.svelte`

For each parsed curve from the LAS file:

1. **Map to `curve_metadata`**:
   - Dropdown shows all entries from `PostgresCurveMetadataState`
   - Each entry includes: `id`, `main_curve_type`, `subcurve_name`, `display_name`, `units` (standard unit)
   - User selects matching `curve_metadata` entry
   - Mapping stored: `curve_mnemonic → curve_metadata_id`

2. **Select Source Unit**:
   - After mapping, user selects unit from LAS file
   - Available units determined by `getAvailableUnitsForCurveType()`:
     - **Hard-coded in TypeScript** (`src/lib/types/curve-units.ts`)
     - **Hard-coded in Rust** (`crates/utils/unit-conversions/src/lib.rs`)
     - Matches `main_curve_type` and `subcurve_name` to return valid units
   - Unit selection stored: `curve_mnemonic → selected_unit`

3. **Validation**:
   - All curves must be mapped to `curve_metadata`
   - All curves must have unit selected
   - Warning shown if mappings incomplete

#### Data Review

- Well header table (editable metadata)
- Curve data table (editable values)
- User can review and edit before confirming

### Phase 3: Backend Processing Pipeline

**Component**: `src-tauri/src/las_processing/las_data_processor.rs`

#### Step 1: Well Creation/Selection

```rust
// If "New Well" selected
if well_id == "new" {
    let well_response = WellCreator::create_well(
        well_name,
        project_id
    ).await?;
    well_id = well_response.id.to_string();
}

// Use database ID for storage path
let well_id_int: i32 = well_id.parse()?;
```

- Creates entry in Postgres `wells` table if new well
- Retrieves database ID (always used for storage path)
- Storage path: `project-{project_id}/wells/{database_id}/logs_{log_type}.parquet`

#### Step 2: Unit Conversion

**Function**: `convert_las_to_parquet()`

For each curve column in LAS file:

```rust
// 1. Get curve metadata mapping
if let Some(curve_metadata_id) = curve_mappings.get(&column.mnemonic) {
    // 2. Get source unit from user selection
    if let Some(source_unit) = curve_unit_mappings.get(&column.mnemonic) {
        // 3. Fetch curve metadata from database
        let (curve_type, standard_unit) = 
            get_curve_metadata_info(curve_metadata_id).await?;
        
        // 4. Compare units
        if source_unit != standard_unit {
            // 5. Convert all values
            values = values.iter()
                .map(|v| unit_conversions::convert_curve_value(
                    v,
                    &curve_type,
                    source_unit,
                    &standard_unit
                ))
                .collect();
        }
    }
}
```

**Unit Conversion Details**:

1. **Fetch Curve Metadata**:
   - Queries Postgres `curve_metadata` table via `get_curve_metadata_info()`
   - Retrieves `main_curve_type` and `units` (standard unit)
   - Uses PostgREST client for database access

2. **Unit Normalization**:
   - Handles sonic unit character variations ('µ' vs 'μ')
   - Normalizes both source and standard units before comparison

3. **Conversion Execution**:
   - Uses `unit_conversions::convert_curve_value()` from `unit-conversions` crate
   - Converts all data points in the curve column
   - Handles conversion errors gracefully (logs warning, keeps original value)

4. **Conversion Logic** (`crates/utils/unit-conversions/src/lib.rs`):
   - Defines enums for each curve type
   - Implements conversion formulas:
     - **Depth**: ft ↔ m (multiply by 0.3048)
     - **Density**: g/cm³ ↔ kg/m³ (multiply by 1000)
     - **Porosity**: v/v ↔ fraction ↔ % (multiply by 100 for %)
     - **Resistivity**: ohm-m ↔ ohm-cm (multiply by 100)
     - **Sonic**: µs/ft ↔ µs/m (multiply by 3.28084)
     - **Caliper**: inches ↔ cm ↔ mm (multiply by 2.54 or 25.4)
   - Special handling for "OTHER" type to detect depth curves

#### Step 3: Parquet Generation

```rust
// Create Arrow RecordBatch
let mut arrays: Vec<Arc<dyn Array>> = Vec::new();
let mut field_names: Vec<String> = Vec::new();

// Add well_name column
arrays.push(Arc::new(StringArray::from(vec![well_name; data_points])));
field_names.push("well_name".to_string());

// Add curve columns (with converted values)
for column in &las_file.curves.columns {
    let values_array = Float64Array::from(values); // Converted values
    arrays.push(Arc::new(values_array));
    field_names.push(column.mnemonic.clone());
}

// Create RecordBatch and write to Parquet
let record_batch = RecordBatch::try_new(schema, arrays)?;
let mut writer = ArrowWriter::try_new(buffer, schema, None)?;
writer.write(&record_batch)?;
writer.close()?;
```

- Creates wide-format Parquet file
- Includes `well_name` + all curve columns
- Uses Arrow arrays for efficient data handling
- Applies compression

#### Step 4: MinIO Storage

```rust
// Construct storage path
let well_path = self.layout_manager.well_log_path(
    &well_id,
    &log_type
);

// Upload via OpenDAL
self.opendal_adapter.upload(
    &well_path.s3_key(),
    parquet_data.into()
).await?;
```

- Uses `OpenDALStorageAdapter` for unified storage access
- Path construction via `ProjectDataLayoutManager`
- Storage path: `project-{project_id}/wells/{well_database_id}/logs_{log_type}.parquet`
- Uploads Parquet file to MinIO

#### Step 5: Curve Entry Creation

**Function**: `create_curve_entries()`

```rust
// For each mapped curve
for column in &las_file.curves.columns {
    if let Some(curve_metadata_id) = curve_mappings.get(&column.mnemonic) {
        // Call PostgreSQL RPC function
        let rpc_data = json!({
            "p_well_id": well_id_int,
            "p_curve_id": column.mnemonic,
            "p_curve_name": column.mnemonic,
            "p_curve_metadata_id": curve_metadata_id,
            "p_parquet_file_path": parquet_file_path,
            "p_parquet_column_name": column.mnemonic,
        });
        
        client.rpc("find_or_create_curve", &rpc_data.to_string())
            .execute()
            .await?;
    }
}
```

- Creates entry in Postgres `curves` table for each mapped curve
- Uses `find_or_create_curve` PostgreSQL RPC function
- Links curve to:
  - `well_id`: Database ID of the well
  - `curve_metadata_id`: UUID from `curve_metadata` table
  - `parquet_file_path`: Path to Parquet file in MinIO
  - `parquet_column_name`: Column name in Parquet file
- Handles conflicts by updating existing entries

## Database Integration

### `curve_metadata` Table (Postgres Supabase)

Centralized table storing curve type definitions:

```sql
CREATE TABLE curve_metadata (
    id UUID PRIMARY KEY,
    curve_mnemonic TEXT UNIQUE NOT NULL,
    main_curve_type TEXT NOT NULL CHECK (
        main_curve_type IN ('GR', 'RT', 'RHOB', 'NPHI', 'CALI', 'DT', 'SP', 'PE', 'OTHER')
    ),
    subcurve_name TEXT,
    display_name TEXT NOT NULL,
    description TEXT,
    units TEXT NOT NULL,  -- Standard unit (target for conversion)
    vendor_variations TEXT[],
    is_primary BOOLEAN,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Key Fields**:
- `units`: Standard unit stored in database (e.g., "g/cm³", "µs/ft", "ft")
  - Used as target unit for conversion
  - Retrieved during unit conversion process
- `main_curve_type`: Main curve category
- `subcurve_name`: Subcurve identifier (e.g., "RT_DEEP", "RT_SHALLOW")

### `curves` Table (Postgres Supabase)

Links actual curve data to wells and metadata:

```sql
CREATE TABLE curves (
    id UUID PRIMARY KEY,
    well_id INTEGER REFERENCES wells(id),
    curve_id TEXT NOT NULL,
    curve_name TEXT NOT NULL,
    curve_metadata_id UUID REFERENCES curve_metadata(id),
    parquet_file_path TEXT,
    parquet_column_name TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Key Relationships**:
- `well_id` → `wells.id`: Links curve to well
- `curve_metadata_id` → `curve_metadata.id`: Links curve to metadata definition
- `parquet_file_path`: Path to Parquet file in MinIO storage
- `parquet_column_name`: Column name in Parquet file

### PostgreSQL RPC Functions

#### `find_or_create_curve`

Creates or updates curve entry:

```sql
CREATE FUNCTION find_or_create_curve(
    p_well_id INTEGER,
    p_curve_id TEXT,
    p_curve_name TEXT,
    p_curve_metadata_id UUID,
    p_parquet_file_path TEXT DEFAULT NULL,
    p_parquet_column_name TEXT DEFAULT NULL
) RETURNS UUID
```

- Finds existing curve by `well_id` and `curve_id`
- Updates if exists, creates if not
- Returns curve UUID

## Unit Conversion Architecture

### Available Units: Hard-Coded Enums

**Rationale**:
- **Type Safety**: Enums provide compile-time type checking in both TypeScript and Rust
- **Consistency**: Frontend and backend always use identical unit definitions
- **Performance**: No database queries needed for unit lookups
- **Version Control**: Unit definitions tracked in code, easy to review changes
- **Maintainability**: Single source of truth in code, easier to update

**Implementation**:
- **TypeScript**: `src/lib/types/curve-units.ts`
  - Defines unit enums: `DepthUnit`, `DensityUnit`, `PorosityUnit`, etc.
  - `getAvailableUnitsForCurveType()`: Returns available units for curve type
  - `getStandardUnitForCurveType()`: Returns standard unit (fallback if database missing)

- **Rust**: `crates/utils/unit-conversions/src/lib.rs`
  - Defines matching unit enums
  - Implements conversion formulas
  - `convert_curve_value()`: Central conversion function

### Standard Unit: Database Column

**Rationale**:
- **Single Source of Truth**: Each `curve_metadata` entry has canonical standard unit
- **Flexibility**: Can update standard units in database without code changes
- **Data Integrity**: Standard unit stored with curve metadata definition

**Implementation**:
- Stored in `curve_metadata.units` column
- Retrieved during unit conversion: `get_curve_metadata_info()`
- Used as target unit for conversion

## Key Files

### Frontend
- `src/lib/components/pages/home/content-main/content-data-ingestion/content-data-ingestion.svelte`: Main ingestion component with tabs
- `src/lib/components/pages/home/content-main/content-data-ingestion/content-data-ingestion-logs-processing/curve-mapping.svelte`: Curve mapping UI
- `src/lib/types/curve-units.ts`: Unit type definitions and helpers
- `src/lib/services/las-upload-service.ts`: LAS parsing service
- `src/lib/services/las-data-processing-service.ts`: LAS processing service

### Backend
- `src-tauri/src/las_processing/las_upload_manager.rs`: LAS parsing and detection
- `src-tauri/src/las_processing/las_data_processor.rs`: Unit conversion and Parquet generation
- `crates/utils/unit-conversions/src/lib.rs`: Unit conversion logic
- `src-tauri/src/postgres_query/create_well.rs`: Well creation

### Database
- `sql/schemas/curves.sql`: `curve_metadata` and `curves` table schemas

## 🔄 Complete Upload Flow

### Phase 1: File Drop & Parsing

1. User drops LAS file → `ContentDataIngestionDropzone`
2. File validation (extension, size)
3. `parse_las_file_for_confirmation()` Tauri command
4. `LasUploadManager` parses LAS file
5. Returns `LasProcessingResult` with curve data and detections

### Phase 2: User Confirmation

1. **Well Selection**: User selects existing well or "New Well"
2. **Curve Mapping**: 
   - Map each curve to `curve_metadata` entry
   - Select source unit from LAS file
3. **Data Review**: Review well metadata and curve data in AG-Grid tables
4. User confirms → triggers `process_and_upload_las_data()`

### Phase 3: Backend Processing

1. **Well Creation/Selection**: Create well in Postgres if new, get database ID
2. **Unit Conversion**: 
   - Fetch `curve_metadata` for each mapped curve
   - Compare source unit (from LAS) vs standard unit (from database)
   - Convert all values if units differ
3. **Parquet Generation**: Create Arrow RecordBatch with converted values, write to Parquet
4. **MinIO Storage**: Upload Parquet file via OpenDAL
5. **Curve Entry Creation**: Create entries in `curves` table linking to `curve_metadata` and Parquet file

### Phase 4: Completion

- Success notification with statistics
- Parquet file available in MinIO
- Database entries created in `wells` and `curves` tables
- Curves linked to `curve_metadata` for type information

## 🎯 Key Features

- **Curve Mapping**: User maps LAS curves to standardized `curve_metadata` entries
- **Unit Conversion**: Automatic conversion from LAS units to standard units
- **Database Integration**: Creates entries in `wells` and `curves` tables
- **Type Safety**: Hard-coded unit enums ensure consistency
- **Storage Integration**: Parquet files stored in MinIO with standardized paths
- **Metadata Preservation**: Well and curve metadata preserved in database

## 📊 Current Status

### ✅ Implemented Features

- LAS file parsing (LAS 1.2, 2.0, 3.0)
- Curve detection with confidence scoring
- User curve mapping to `curve_metadata` table
- Unit selection and conversion
- Well creation/selection integration
- Parquet file generation with converted units
- MinIO storage integration
- Database entries in `curves` table
- Real-time frontend state management

### 🔄 Current Limitations

- Single file processing (no batch processing)
- Manual curve mapping required (no auto-mapping)
- Wide-format Parquet only (no long-format option)
