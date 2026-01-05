# LAS Processing Implementation

## Overview

This document describes the current LAS file processing implementation in MudRock, including curve detection, mapping to `curve_metadata`, unit conversion, and Parquet file generation.

## Current Implementation

### Architecture Overview

```
LAS File Upload
    ↓
parse_las_file_for_confirmation (Tauri Command)
    ↓
LasUploadManager (Rust Backend)
    ↓
Curve Detection & Parsing
    ↓
Frontend: Curve Mapping UI (curve-mapping.svelte)
    ↓
User Maps Curves to curve_metadata + Selects Units
    ↓
process_and_upload_las_data (Tauri Command)
    ↓
LasDataProcessor (Rust Backend)
    ↓
1. Well Creation/Selection (Postgres wells table)
2. Unit Conversion (unit-conversions crate)
3. Parquet Generation (Arrow/Parquet)
4. MinIO Storage (OpenDAL)
5. Curve Entry Creation (Postgres curves table)
```

## Curve Mapping Flow

### 1. LAS File Parsing & Curve Detection

**Component**: `src-tauri/src/las_processing/las_upload_manager.rs`

- Parses LAS 1.2, 2.0, and 3.0 files using `las-parser` crate
- Extracts well metadata (name, field, company, location, UWI)
- Detects curve types from curve mnemonics
- Provides confidence scores for detected curve types
- Returns `LasProcessingResult` with:
  - `well_metadata`: Well information from LAS headers
  - `curve_data`: Array of parsed curves with values
  - `curve_detections`: Detected curve types with confidence scores

### 2. Frontend Curve Mapping UI

**Component**: `src/lib/components/pages/home/content-main/content-data-ingestion/content-data-ingestion-logs-processing/curve-mapping.svelte`

The curve mapping component allows users to:

1. **Map Each Parsed Curve to `curve_metadata`**:
   - Displays each curve mnemonic from the LAS file
   - Provides dropdown to select matching `curve_metadata` entry
   - Options loaded from `PostgresCurveMetadataState` (fetched from Postgres `curve_metadata` table)
   - Each `curve_metadata` entry includes:
     - `id`: UUID identifier
     - `curve_mnemonic`: Standard curve name
     - `main_curve_type`: Main category (GR, RT, RHOB, NPHI, DT, CALI, SP, PE, OTHER)
     - `subcurve_name`: Subcurve category (e.g., "RT_DEEP", "RT_SHALLOW")
     - `display_name`: Human-readable name
     - `units`: Standard unit for this curve type (stored in database)
     - `description`: Curve description

2. **Select Source Unit for Each Curve**:
   - After mapping to `curve_metadata`, user selects the unit from the LAS file
   - Available units determined by `getAvailableUnitsForCurveType()` function
   - Units are hard-coded in TypeScript (`src/lib/types/curve-units.ts`) and Rust (`crates/utils/unit-conversions/src/lib.rs`)
   - **Rationale for Hard-Coded Units**:
     - Type safety: Enums provide compile-time type checking
     - Consistency: Ensures frontend and backend use identical unit definitions
     - Performance: No database queries needed for unit lookups
     - Version control: Unit definitions are versioned with code
   - Standard unit (target for conversion) comes from `curve_metadata.units` column in database

3. **Automatic Initialization**:
   - Attempts to auto-map curves based on detected types
   - Tries to match LAS file units to available units
   - Falls back to standard unit if no match found

### 3. Backend Processing & Unit Conversion

**Component**: `src-tauri/src/las_processing/las_data_processor.rs`

#### Step 1: Well Creation/Selection

- If "New Well" selected: Creates entry in Postgres `wells` table via `WellCreator::create_well()`
- Retrieves database ID for well (used for storage path)
- If existing well: Uses provided database ID directly

#### Step 2: Unit Conversion

**Function**: `convert_las_to_parquet()`

For each curve column in the LAS file:

1. **Fetch Curve Metadata**:
   - Queries Postgres `curve_metadata` table using `curve_metadata_id` from mapping
   - Retrieves `main_curve_type` and `units` (standard unit)

2. **Compare Units**:
   - Source unit: User-selected unit from LAS file (`curve_unit_mappings`)
   - Target unit: Standard unit from `curve_metadata.units`
   - Normalizes sonic units (handles both 'µ' and 'μ' characters)

3. **Convert Values** (if units differ):
   - Uses `unit_conversions::convert_curve_value()` from `unit-conversions` crate
   - Converts all data points in the curve column
   - Handles conversion errors gracefully (logs warning, keeps original value)

4. **Unit Conversion Logic**:
   - **Rust Crate**: `crates/utils/unit-conversions/src/lib.rs`
   - Defines enums for each curve type: `DepthUnit`, `DensityUnit`, `PorosityUnit`, `ResistivityUnit`, `SonicUnit`, `CaliperUnit`, `SpontaneousPotentialUnit`, `PhotoElectricUnit`, `GammaRayUnit`
   - Implements conversion functions for each unit type
   - Central dispatch via `convert_curve_value(curve_type, from_unit, to_unit)`
   - Special handling for "OTHER" `main_curve_type` to detect depth curves by mnemonic

#### Step 3: Parquet Generation

- Creates Arrow RecordBatch with converted values
- Includes `well_name` column + all curve columns
- Writes to Parquet format with compression
- Returns Parquet file as bytes

#### Step 4: MinIO Storage

- Uses `OpenDALStorageAdapter` for unified storage access
- Path construction via `ProjectDataLayoutManager`
- Storage path: `project-{project_id}/wells/{well_database_id}/logs_{log_type}.parquet`
- Uploads Parquet file to MinIO

#### Step 5: Curve Entry Creation

**Function**: `create_curve_entries()`

- For each mapped curve, creates entry in Postgres `curves` table
- Uses `find_or_create_curve` PostgreSQL RPC function
- Links curve to:
  - `well_id`: Database ID of the well
  - `curve_metadata_id`: UUID from `curve_metadata` table
  - `parquet_file_path`: Path to Parquet file in MinIO
  - `parquet_column_name`: Column name in Parquet file (curve mnemonic)
- Handles conflicts by updating existing entries

## Database Schema

### `curve_metadata` Table (Postgres Supabase)

Centralized table storing curve type definitions:

```sql
CREATE TABLE curve_metadata (
    id UUID PRIMARY KEY,
    curve_mnemonic TEXT UNIQUE NOT NULL,
    main_curve_type TEXT NOT NULL CHECK (main_curve_type IN ('GR', 'RT', 'RHOB', 'NPHI', 'CALI', 'DT', 'SP', 'PE', 'OTHER')),
    subcurve_name TEXT,
    display_name TEXT NOT NULL,
    description TEXT,
    units TEXT NOT NULL,  -- Standard unit for this curve type
    vendor_variations TEXT[],
    is_primary BOOLEAN,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

**Key Fields**:

- `units`: Standard unit stored in database (e.g., "g/cm³", "µs/ft", "ft")
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

## Unit Conversion Implementation

### TypeScript (`src/lib/types/curve-units.ts`)

- Defines unit enums matching Rust implementation
- `getAvailableUnitsForCurveType()`: Returns available units for a curve type
- `getStandardUnitForCurveType()`: Returns standard unit for a curve type
- Handles special case for "OTHER" type to detect depth curves

### Rust (`crates/utils/unit-conversions/src/lib.rs`)

- Defines unit enums with conversion logic
- `convert_curve_value()`: Central function for unit conversion
- Implements conversion formulas for each unit type
- Handles edge cases (sonic unit normalization, depth curve detection)

### Why Hard-Coded Units?

**Available Units**: Hard-coded in TypeScript/Rust enums

- Type safety: Compile-time checking
- Consistency: Frontend and backend always match
- Performance: No database queries
- Version control: Unit definitions tracked in code

**Standard Unit**: Stored in `curve_metadata.units` column

- Single source of truth per curve type
- Can be updated in database without code changes
- Used as target for unit conversion

## Current Status

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

## Key Files

### Frontend

- `src/lib/components/pages/home/content-main/content-data-ingestion/content-data-ingestion-logs-processing/curve-mapping.svelte`: Curve mapping UI
- `src/lib/types/curve-units.ts`: Unit type definitions and helpers
- `src/lib/services/las-upload-service.ts`: LAS parsing service
- `src/lib/services/las-data-processing-service.ts`: LAS processing service

### Backend

- `src-tauri/src/las_processing/las_upload_manager.rs`: LAS parsing and detection
- `src-tauri/src/las_processing/las_data_processor.rs`: Unit conversion and Parquet generation
- `crates/utils/unit-conversions/src/lib.rs`: Unit conversion logic

### Database

- `sql/schemas/curves.sql`: `curve_metadata` and `curves` table schemas
