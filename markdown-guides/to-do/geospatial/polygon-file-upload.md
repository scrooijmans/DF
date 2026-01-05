# Polygon File Upload Implementation in MudRock

This document describes the complete end-to-end implementation of polygon file upload functionality in MudRock, including GeoJSON parsing, user confirmation, PostgreSQL storage with PostGIS, and integration with map charts.

## 🎯 Overview

**Goal**: Enable users to upload polygon files (GeoJSON, Shapefile, KML) and automatically:

1. Parse polygon geometries
2. Store in `polygons` table (PostGIS geometry)
3. Make polygons available for selection in map charts via `data_source_config`
4. Convert PostGIS geometry to SciChart-compatible format when rendering
5. Support realtime updates when polygons are added to charts

**Current State**:

- ✅ `polygons` table exists with PostGIS geometry (POLYGON, 4326)
- ✅ Map charts can render polygons from `chart_config.mapData` (JSON arrays) - legacy support
- ✅ Map charts can render polygons from `data_source_config` (new approach)
- ✅ PostGIS → SciChart conversion implemented (`postgis-to-scichart-converter.ts`)
- ✅ Polygon selection UI in map chart data options (`chart-data-source-polygon-selector.svelte`)
- ✅ Well selection UI in map chart data options (`chart-data-source-well-selector.svelte`)
- ✅ Chart data source service for updating `data_source_config` (`chart-data-source-service.ts`)
- ✅ MapChartState loads polygons/wells from `data_source_config` and renders them
- ✅ Realtime reactivity when data sources are added/removed
- ✅ GeoJSON file upload implemented (backend + frontend)
- ❌ Shapefile support (future enhancement)
- ❌ KML support (future enhancement)

## 🏗️ Architecture Overview

```
Frontend (SvelteKit)
    ↓ GeoJSON File Drop
ContentDataIngestionDropzone
    ↓ parsePolygonFileForConfirmation()
Tauri Command (Rust Backend)
    ↓ GeoJSON Parsing & Validation
PolygonUploadManager
    ↓ User Confirmation UI
ContentDataIngestionPolygonProcessing
    ├─ Properties Tab (AG-Grid table)
    │   ├─ Name, Description, Type
    │   ├─ Color, Opacity, Stroke Width
    │   └─ Metadata Keys (read-only)
    └─ Data Tab (read-only geometry preview)
    ↓ User Confirms
processAndUploadPolygonData()
    ↓ Backend Processing Pipeline
PolygonDataProcessor
    ├─ Step 1: GeoJSON → WKT Conversion
    ├─ Step 2: Type Conversions (f64 → Decimal)
    ├─ Step 3: PostGIS Geometry Insertion
    └─ Step 4: Polygon Entry Creation (Postgres polygons table)
    ↓ Success Notification
Polygons available in map chart data options
    ↓ User selects polygons via "+" button
ChartDataSourceSelectDialog
    ↓ User confirms selection
chart-data-source-service.ts::addDataSourcesToChart()
    ↓ Updates charts.data_source_config
Supabase Realtime UPDATE event
    ↓ PostgresChartsState.updateChart()
    ↓ MapChartState.loadDataSources()
    ↓ Converts PostGIS → SciChart format
    ↓ MapChartState.reloadMap()
    ↓ SciChart renders polygons
```

## 📋 Data Flow & Type Conversions

### Phase 1: File Drop & Parsing

#### Frontend Types (TypeScript)

**File**: `src/lib/services/polygon-upload-service.ts`

```typescript
// GeoJSON Feature from file
interface GeoJSONFeature {
  type: "Feature";
  geometry: {
    type: "Polygon" | "MultiPolygon";
    coordinates: number[][][] | number[][][][]; // Nested arrays of [longitude, latitude]
  };
  properties?: Record<string, any>; // Arbitrary metadata
}

// GeoJSON File Structure
interface GeoJSONFile {
  type: "FeatureCollection";
  features: GeoJSONFeature[];
}

// Processing Result (returned from backend)
interface PolygonProcessingResult {
  polygons: Array<{
    name: string;
    geometry: {
      type: "Polygon" | "MultiPolygon";
      coordinates: number[][][] | number[][][][];
    };
    properties?: Record<string, any>;
  }>;
  total_polygons: number;
  valid_polygons: number;
  invalid_polygons: number;
  requires_user_confirmation: boolean;
  original_geojson: GeoJSONFile;
}
```

**Process**:

1. User drops `.geojson` or `.json` file into `ContentDataIngestionDropzone`
2. File validation checks extension and size (50MB limit)
3. File content read as text: `await file.text()` → `string`
4. Tauri command invoked: `parse_polygon_file_for_confirmation(fileContent, fileName)`

#### Backend Types (Rust)

**File**: `src-tauri/src/polygon_processing/polygon_upload_manager.rs`

```rust
// Backend representation (matches frontend)
pub struct PolygonProcessingResult {
    pub polygons: Vec<PolygonData>,
    pub total_polygons: usize,
    pub valid_polygons: usize,
    pub invalid_polygons: usize,
    pub requires_user_confirmation: bool,
    pub original_geojson: Value, // serde_json::Value
}

pub struct PolygonData {
    pub name: String,
    pub geometry: PolygonGeometry,
    pub properties: Option<Value>, // serde_json::Value
}

pub struct PolygonGeometry {
    #[serde(rename = "type")]
    pub geometry_type: String, // "Polygon" or "MultiPolygon"
    pub coordinates: Value, // serde_json::Value (nested arrays)
}
```

**Process**:

1. Parse JSON string → `serde_json::Value`
2. Validate GeoJSON structure (must be `FeatureCollection`)
3. Extract features array
4. For each feature:
   - Validate geometry type (`Polygon` or `MultiPolygon` only)
   - Extract name from `properties.name` or generate default
   - Store geometry coordinates as `serde_json::Value` (preserves structure)
5. Return `PolygonProcessingResult` (serialized to JSON for frontend)

**Type Conversion**: `String` (file content) → `serde_json::Value` → `PolygonData` → JSON → TypeScript types

### Phase 2: User Confirmation & Editing

#### Frontend Editable Data Structure

**File**: `src/lib/components/pages/home/content-main/content-data-ingestion/content-data-ingestion-polygon-processing/content-data-ingestion-polygon-processing.svelte`

```typescript
// Editable polygon data (user can modify before upload)
let polygonData = $state<
  Array<{
    name: string; // Editable in AG-Grid
    description?: string; // Editable in AG-Grid
    polygon_type?: string; // Editable dropdown: 'lease_boundary' | 'formation' | 'administrative' | 'custom'
    color?: string; // Editable hex color: '#3b82f6'
    fill_opacity?: number; // Editable: 0.0 - 1.0
    stroke_width?: number; // Editable: 0.5 - 10.0
    metadata?: Record<string, any>; // Read-only (from GeoJSON properties)
    geometry: {
      type: "Polygon" | "MultiPolygon";
      coordinates: number[][][] | number[][][][];
    };
  }>
>([]);
```

**Initialization**:

- Parsed polygons from `PolygonProcessingResult` are mapped to editable structure
- Defaults applied:
  - `name`: From GeoJSON `properties.name` or `"Polygon {index + 1}"`
  - `description`: `""` (empty)
  - `polygon_type`: `"custom"`
  - `color`: `"#3b82f6"` (blue)
  - `fill_opacity`: `0.3`
  - `stroke_width`: `2.0`
  - `metadata`: From GeoJSON `properties` (preserved as-is)

#### AG-Grid Display & Editing

**File**: `src/lib/components/pages/home/content-main/content-data-ingestion/content-data-polygon-upload-confirm-AG-data-table/content-data-polygon-upload-confirm-AG-data-table.svelte`

**Column Definitions**:

- **Name** (`name`): Text editor, editable
- **Description** (`description`): Text editor, editable
- **Type** (`polygon_type`): Dropdown editor (`lease_boundary`, `formation`, `administrative`, `custom`), editable
- **Color** (`color`): Text editor (hex color), editable, custom renderer shows color swatch
- **Opacity** (`fill_opacity`): Number editor (0-1, step 0.01), editable
- **Stroke** (`stroke_width`): Number editor (0.5-10, step 0.5), editable
- **Geometry Type** (`geometry_type`): Read-only (`"Polygon"` or `"MultiPolygon"`)
- **Coords** (`coordinate_count`): Read-only (computed: `coordinates.flat(Infinity).length / 2`)
- **Metadata Keys** (`metadata_keys`): Read-only (comma-separated keys from `metadata` object)

**Type Conversions in AG-Grid**:

- **Display**: Raw values shown directly (strings, numbers)
- **Editing**: AG-Grid cell editors handle type conversion automatically
- **Updates**: `onCellValueChanged` event updates `polygonData` array reactively

### Phase 3: Name Validation & Unique Name Generation

**File**: `src/lib/utils/polygons/polygon-name-validation.ts`

**Process**:

Before uploading polygons, the system validates and generates unique names to prevent database constraint violations:

1. **Check Existing Names**: Queries `polygons` table for existing names in the same project
2. **Generate Unique Name**: If duplicate exists, auto-increments suffix (`_1`, `_2`, `_3`, etc.)
3. **Project-Scoped**: Names are unique per project, not globally

**Functions**:

```typescript
// Get existing polygon names in project
export async function getExistingPolygonNamesInProject(
  projectId: string,
  baseName: string,
): Promise<string[]>;

// Generate unique name with auto-incrementing suffix
export async function generateUniquePolygonName(
  projectId: string,
  baseName: string,
): Promise<string>;
```

**Name Generation Examples**:

- First polygon: `Polygon 3` → `Polygon 3`
- Second polygon (duplicate): `Polygon 3` → `Polygon 3_1`
- Third polygon (duplicate): `Polygon 3` → `Polygon 3_2`
- Different project: `Polygon 3` → `Polygon 3` (allowed, different project)

**Integration**:

**File**: `src/lib/services/polygon-upload-service.ts` → `processAndUploadPolygonData()`

```typescript
// Validate and generate unique names for each polygon
const polygonsWithUniqueNames = await Promise.all(
  polygons.map(async (polygon) => {
    const uniqueName = await generateUniquePolygonName(projectId, polygon.name);

    if (uniqueName !== polygon.name) {
      console.log(
        `Name conflict resolved: "${polygon.name}" → "${uniqueName}"`,
      );
    }

    return {
      ...polygon,
      name: uniqueName,
    };
  }),
);
```

**Database Constraint**:

```sql
CONSTRAINT polygons_project_name_unique UNIQUE (project_id, name)
```

This ensures that:

- ✅ Same name can exist in different projects
- ✅ Same project cannot have duplicate names
- ✅ Auto-incrementing prevents constraint violations

**Similar Pattern**: This follows the same validation pattern as chart name validation (see `markdown-guides/svelte5/charts/add-chart-to-project.md`).

### Phase 4: Backend Processing & Database Storage

#### Upload Data Structure

**Frontend → Backend** (via Tauri command):

```typescript
// Frontend sends this structure
{
  polygons: Array<{
    name: string;
    description?: string;
    polygon_type?: string;
    color?: string;
    fill_opacity?: number; // TypeScript: number (f64)
    stroke_width?: number; // TypeScript: number (f64)
    metadata?: Record<string, any>;
    geometry: {
      type: "Polygon" | "MultiPolygon";
      coordinates: number[][][] | number[][][][];
    };
  }>;
  projectId: string;
  fileName: string;
  createdBy: string;
}
```

**Backend Receives** (Rust):

**File**: `src-tauri/src/polygon_processing/polygon_data_processor.rs`

```rust
pub struct PolygonUploadData {
    pub name: String,
    pub description: Option<String>,
    pub polygon_type: Option<String>,
    pub color: Option<String>,
    pub fill_opacity: Option<f64>,      // Rust: Option<f64>
    pub stroke_width: Option<f64>,      // Rust: Option<f64>
    pub metadata: Option<Value>,        // serde_json::Value
    pub geometry: PolygonGeometryData,
}
```

**Type Conversion**: TypeScript `number` → Rust `f64` (automatic via serde)

#### GeoJSON → WKT Conversion

**File**: `src-tauri/src/polygon_processing/polygon_data_processor.rs`

**Process**:

1. Extract coordinates from `PolygonGeometryData.coordinates` (nested `serde_json::Value`)
2. Convert to WKT format:
   - **Polygon**: `POLYGON((lon1 lat1, lon2 lat2, ...))`
   - **MultiPolygon**: `MULTIPOLYGON(((lon1 lat1, ...)), ((lon2 lat2, ...)))`
3. Coordinate order: GeoJSON uses `[longitude, latitude]`, WKT uses `longitude latitude` (space-separated)

**Example**:

```rust
// GeoJSON coordinates
[[[100.0, 0.0], [101.0, 0.0], [101.0, 1.0], [100.0, 1.0], [100.0, 0.0]]]

// WKT output
"POLYGON((100.0 0.0, 101.0 0.0, 101.0 1.0, 100.0 1.0, 100.0 0.0))"
```

**Type Conversion**: `serde_json::Value` (nested arrays) → `String` (WKT format)

#### NUMERIC Type Conversion (f64 → Decimal)

**File**: `src-tauri/src/polygon_processing/polygon_data_processor.rs`

**Problem**: PostgreSQL `NUMERIC` type requires `rust_decimal::Decimal`, not `f64`.

**Solution**:

```rust
use rust_decimal::Decimal;
use std::str::FromStr;

// Convert Option<f64> to Option<Decimal>
let fill_opacity_val = polygon.fill_opacity.unwrap_or(0.3);
let stroke_width_val = polygon.stroke_width.unwrap_or(2.0);

// Convert f64 → String → Decimal (reliable conversion)
let fill_opacity: Option<Decimal> = Decimal::from_str(&fill_opacity_val.to_string())
    .ok()
    .or(Some(Decimal::new(3, 1))); // Fallback to 0.3

let stroke_width: Option<Decimal> = Decimal::from_str(&stroke_width_val.to_string())
    .ok()
    .or(Some(Decimal::new(20, 1))); // Fallback to 2.0
```

**Type Conversion**: `Option<f64>` → `String` → `Option<Decimal>`

**Dependencies**:

- `rust_decimal = { version = "1.36", features = ["tokio-pg"] }` in `Cargo.toml`
- `tokio-pg` feature enables `tokio_postgres` integration

#### Metadata JSONB Conversion

**Process**:

```rust
// Convert Option<Value> to JSONB string
let metadata_json = polygon.metadata
    .map(|m| serde_json::to_string(&m).unwrap_or_else(|_| "{}".to_string()))
    .unwrap_or_else(|| "{}".to_string());
```

**Type Conversion**: `Option<serde_json::Value>` → `String` (JSON) → PostgreSQL `JSONB` (via `$10::jsonb` cast)

#### Database Insertion

**File**: `src-tauri/src/polygon_processing/polygon_data_processor.rs`

**SQL Query**:

```sql
INSERT INTO polygons (
    id, project_id, name, description, geometry,
    polygon_type, color, fill_opacity, stroke_width,
    metadata, created_by, source_file
)
VALUES (
    $1, $2, $3, $4, ST_GeomFromText($5, 4326),
    $6, $7, $8, $9, $10::jsonb, $11, $12
)
RETURNING id
```

**Parameter Types**:

- `$1`: `UUID` (generated `Uuid::new_v4()`)
- `$2`: `UUID` (project_id from string)
- `$3`: `String` (name)
- `$4`: `Option<String>` (description)
- `$5`: `String` (WKT geometry) → `ST_GeomFromText($5, 4326)` converts to PostGIS `GEOMETRY(POLYGON, 4326)`
- `$6`: `Option<String>` (polygon_type)
- `$7`: `Option<String>` (color)
- `$8`: `Option<Decimal>` (fill_opacity) → PostgreSQL `NUMERIC`
- `$9`: `Option<Decimal>` (stroke_width) → PostgreSQL `NUMERIC`
- `$10`: `String` (JSON string) → `$10::jsonb` casts to PostgreSQL `JSONB`
- `$11`: `UUID` (created_by from string)
- `$12`: `String` (file_name)

**Type Conversions**:

- `String` → `UUID`: `Uuid::parse_str(project_id)?`
- `String` (WKT) → PostGIS `GEOMETRY`: `ST_GeomFromText($5, 4326)` (SRID 4326 = WGS84)
- `Option<Decimal>` → PostgreSQL `NUMERIC`: Direct (via `tokio-pg` feature)
- `String` (JSON) → PostgreSQL `JSONB`: `$10::jsonb` cast

### Phase 4: Database Schema

**Table**: `public.polygons`

**Columns**:

- `id`: `UUID` (PRIMARY KEY)
- `project_id`: `UUID` (FOREIGN KEY to `projects`)
- `name`: `TEXT` (NOT NULL)
- `description`: `TEXT` (NULLABLE)
- `geometry`: `GEOMETRY(POLYGON, 4326)` (PostGIS, NOT NULL)
- `polygon_type`: `TEXT` (NULLABLE)
- `color`: `TEXT` (NULLABLE, hex color)
- `fill_opacity`: `NUMERIC` (NULLABLE, 0.0-1.0)
- `stroke_width`: `NUMERIC` (NULLABLE, 0.5-10.0)
- `metadata`: `JSONB` (NULLABLE, arbitrary properties)
- `created_by`: `UUID` (FOREIGN KEY to `users`)
- `source_file`: `TEXT` (NULLABLE, original filename)
- `created_at`: `TIMESTAMP`
- `updated_at`: `TIMESTAMP`
- `tags`: `TEXT[]` (NULLABLE)
- `is_active`: `BOOLEAN` (NULLABLE, default `true`)

**PostGIS Extension**:

- Requires `CREATE EXTENSION IF NOT EXISTS postgis;`
- `geometry` column stores spatial data in WGS84 (SRID 4326)
- Supports spatial queries (intersection, containment, distance, etc.)

**PostGIS Geometry Format**:

PostGIS stores geometry as **WKB (Well-Known Binary)** internally, but Supabase/PostgREST automatically converts it to **GeoJSON** when querying:

```json
{
  "type": "Polygon",
  "coordinates": [
    [
      [100.0, 0.0],
      [101.0, 0.0],
      [101.0, 1.0],
      [100.0, 1.0],
      [100.0, 0.0]
    ]
  ]
}
```

This is why the `geometry` field in `scripts/db-exports/polygons.json` appears as a hex string (WKB) - it's the raw PostGIS format. When fetched via Supabase client, it's automatically converted to GeoJSON.

## 🔄 Complete Type Conversion Chain

```
GeoJSON File (JSON string)
    ↓
serde_json::Value (Rust)
    ↓
PolygonData (Rust struct)
    ↓
JSON serialization
    ↓
PolygonProcessingResult (TypeScript)
    ↓
User edits in AG-Grid
    ↓
polygonData (TypeScript, edited)
    ↓
Tauri command (JSON serialization)
    ↓
PolygonUploadData (Rust struct)
    ↓
GeoJSON coordinates → WKT string
    ↓
f64 → Decimal (for NUMERIC)
    ↓
serde_json::Value → JSON string → JSONB
    ↓
PostgreSQL INSERT
    ↓
PostGIS GEOMETRY(POLYGON, 4326)
    ↓
Supabase Query (automatic conversion)
    ↓
GeoJSON (TypeScript)
    ↓
convertPostGisToSciChart()
    ↓
SciChart format (outline, areaData arrays)
```

## 📊 Map Chart Data Source Integration

### Reusable Multi-Row Selection Component

**File**: `src/lib/components/AG-data-tables/ag-data-table-multi-row-select.svelte`

**Purpose**: Reusable AG-Grid component for multi-row selection used by polygon, well, and curve selectors.

**Features**:

- **Multi-row selection**: Click rows to select/deselect (toggle behavior)
- **Data type identification**: `dataType` prop uses type-safe `DataSourceType` from `chart-data-sources.ts` (`'curve' | 'well' | 'surface' | 'well_marker' | 'polygon' | 'seismic' | 'node_execution'`) for compile-time type checking
- **Existing row highlighting**: Highlights rows that are already in the chart (green background)
- **Selection state sync**: Syncs selection state between grid and parent component
- **Click behavior**:
  - Click selected row → Deselects it
  - Click unselected row → Selects it (doesn't affect other rows)
  - No modifier keys needed for toggle behavior

**Usage**:

```svelte
<AgDataTableMultiRowSelect
  rowData={polygonData}
  {columnDefs}
  bind:selectedRowIds={selectedPolygonIds}
  getRowId={(params) => params.data.id}
  existingRowIds={existingPolygonIds}
  dataType="polygon"
/>
```

**Props**:

- `rowData`: Array of data rows to display
- `columnDefs`: AG-Grid column definitions
- `selectedRowIds`: Bindable array of selected row IDs
- `getRowId`: Function to extract row ID from data
- `existingRowIds`: Array of IDs already in chart (highlighted in green)
- `dataType`: Type-safe data source type (`DataSourceType` from `chart-data-sources.ts`) - identifies data type throughout call stack for type safety

**Selection Behavior**:

- **Toggle on click**: Clicking a row toggles its selection state
- **Independent selection**: Selecting one row doesn't deselect others
- **Visual feedback**: Selected rows have green background, existing rows have different styling

### Adding Polygons/Wells to Map Charts

**Flow**:

1. User clicks "+" button in chart menubar
2. `ChartDataSourceSelectDialog` opens
3. User selects polygons/wells from AG-Grid tables
4. User clicks "Add to Chart"
5. `chart-data-source-service.ts::addDataSourcesToChart()` updates `charts.data_source_config`
6. Supabase Realtime detects UPDATE event
7. `PostgresChartsState.updateChart()` is called
8. `MapChartState.loadDataSources()` fetches polygons/wells from database
9. `convertPostGisToSciChart()` converts PostGIS geometry to SciChart format
10. `MapChartState.reloadMap()` renders polygons/wells on map

### Data Source Config Format

**Location**: `charts.data_source_config` (JSONB)

```json
[
  {
    "id": "polygon-b92e2daf-1f93-4907-8335-69ab519a542d",
    "source": {
      "type": "polygon",
      "polygonId": "b92e2daf-1f93-4907-8335-69ab519a542d"
    },
    "displaySettings": {
      "visible": true,
      "fillColor": "#3b82f6",
      "fillOpacity": 0.3,
      "strokeColor": "#1e40af",
      "strokeWidth": 2,
      "strokeStyle": "solid"
    }
  },
  {
    "id": "well-123",
    "source": {
      "type": "well",
      "wellId": "123"
    },
    "displaySettings": {
      "visible": true,
      "markerSize": 5,
      "markerColor": "#3b82f6"
    }
  }
]
```

### PostGIS → SciChart Conversion

**File**: `src/lib/utils/maps/postgis-to-scichart-converter.ts`

**Function**: `convertPostGisToSciChart(geometry: any)`

**Process**:

1. Receives GeoJSON geometry from Supabase (PostGIS automatically converted)
2. Extracts coordinates from GeoJSON `Polygon` or `MultiPolygon`
3. Converts to `[longitude, latitude]` pairs
4. Ensures polygon is closed (first point = last point)
5. Returns `{ outline, areaData }` arrays for SciChart

**Example**:

```typescript
// PostGIS geometry (as GeoJSON from Supabase)
{
  "type": "Polygon",
  "coordinates": [[[100.0, 0.0], [101.0, 0.0], [101.0, 1.0], [100.0, 1.0], [100.0, 0.0]]]
}

// Converts to:
{
  outline: [[100.0, 0.0], [101.0, 0.0], [101.0, 1.0], [100.0, 1.0], [100.0, 0.0]],
  areaData: [[100.0, 0.0], [101.0, 0.0], [101.0, 1.0], [100.0, 1.0], [100.0, 0.0]]
}
```

### MapChartState Integration

**File**: `src/lib/state/postgres/chart-states/map-chart-state.svelte.ts`

**Methods**:

- `loadFromChartConfig()`: Loads map data from `chart_config` (legacy) and `data_source_config` (new)
- `loadDataSources()`: Fetches polygons/wells from database based on `data_source_config`
- `reloadMap()`: Renders all map data (from config + data sources)

**Flow**:

```typescript
async loadFromChartConfig(chartConfig: Record<string, any>): Promise<void> {
  // 1. Load legacy mapData from chart_config
  const mapDataFromConfig = chartConfig.mapData || [];

  // 2. Load polygons/wells from data_source_config
  const mapDataFromSources = await this.loadDataSources();

  // 3. Merge both sources
  const mapData = [...mapDataFromConfig, ...mapDataFromSources.polygons];
  const markers = [...(chartConfig.markers || []), ...mapDataFromSources.wells];

  // 4. Update config
  this.config = { mapData, markers, ... };
}
```

## 🔄 Realtime Reactivity

### Chart Data Source Updates

**Pattern**: Similar to node creation in pipelines (see `markdown-guides/dags/node-creation/realtime-node-creation-add-to-active-pipeline.md`)

**Flow**:

```
User adds polygons/wells via "+" button
    ↓
chart-data-source-service.ts::addDataSourcesToChart()
    ↓
Supabase UPDATE charts.data_source_config
    ↓
PostgreSQL WAL updated
    ↓
Supabase Realtime detects UPDATE
    ↓
WebSocket message sent
    ↓
PostgresChartsState.updateChart()
    ↓
Detects data_source_config changed
    ↓
MapChartState.loadDataSources() (async)
    ↓
Fetches polygons/wells from database
    ↓
Converts PostGIS → SciChart format
    ↓
MapChartState.reloadMap()
    ↓
SciChart renders updated map
```

**Implementation**:

- `PostgresChartsState.updateChart()` checks if `data_source_config` changed
- If changed and chart is map type, calls `MapChartState.updateChart()`
- `MapChartState.updateChart()` calls `loadFromChartConfig()` which loads data sources
- `reloadMap()` is called automatically to render changes

## 🗄️ Database Integration

### Query Polygons from Database

**Location**: `src/lib/state/postgres/chart-states/map-chart-state.svelte.ts::loadDataSources()`

**Process**:

```typescript
// Fetch polygons by IDs from data_source_config
const { data: polygonRows } = await supabase
  .from("polygons")
  .select("*")
  .in("id", polygonIds)
  .eq("is_active", true);

// Convert PostGIS geometry (GeoJSON) to SciChart format
for (const polygonRow of polygonRows) {
  const { outline, areaData } = convertPostGisToSciChart(polygonRow.geometry);
  polygons.push({
    name: polygonRow.name,
    outline,
    areaData,
    metadata: polygonRow.metadata || {},
  });
}
```

**Note**: PostGIS geometry is returned as GeoJSON by Supabase/PostgREST automatically.

### Query Wells from Database

**Process**:

```typescript
// Fetch wells by IDs from data_source_config
const { data: wellRows } = await supabase
  .from("wells")
  .select("*")
  .in("id", wellIds)
  .eq("is_active", true);

// Convert to marker format
for (const wellRow of wellRows) {
  if (wellRow.x !== null && wellRow.y !== null) {
    wells.push({
      name: wellRow.name,
      longitude: wellRow.x,
      latitude: wellRow.y,
      metadata: { well_id: wellRow.id, project_id: wellRow.project_id },
    });
  }
}
```

## 🧹 Removing Hardcoded Map Data

**Script**: `scripts/database/remove-hardcoded-map-data.ts`

**Purpose**: Remove hardcoded Australia mapData from `chart_config.mapData` to encourage use of `data_source_config` instead.

**Usage**:

```bash
bun run scripts/database/remove-hardcoded-map-data.ts
```

**Process**:

1. Finds all map charts with `chart_config.mapData` array
2. Removes `mapData` from `chart_config` (preserves other config)
3. Updates `updated_at` timestamp
4. Charts will then rely on `data_source_config` for polygon/well data

**Note**: This script is safe to run multiple times - it only removes `mapData` if it exists.

## 📝 Key Implementation Files

### Frontend

- `src/lib/services/polygon-upload-service.ts` - Service functions for parsing and uploading
- `src/lib/services/chart-data-source-service.ts` - Service for updating chart `data_source_config`
- `src/lib/components/pages/home/content-main/content-data-ingestion/content-data-ingestion-polygon-processing/content-data-ingestion-polygon-processing.svelte` - Main processing component
- `src/lib/components/pages/home/content-main/content-data-ingestion/content-data-polygon-upload-confirm-AG-data-table/content-data-polygon-upload-confirm-AG-data-table.svelte` - AG-Grid table for editing
- `src/lib/components/pages/home/charts/chart-editor/chart-editor-data-options/chart-data-source-polygon-selector.svelte` - Polygon selection for map charts
- `src/lib/components/pages/home/charts/chart-editor/chart-editor-data-options/chart-data-source-well-selector.svelte` - Well selection for map charts
- `src/lib/components/pages/home/charts/chart-editor/chart-editor-data-options/chart-data-source-select-dialog.svelte` - Dialog for adding data sources
- `src/lib/utils/maps/postgis-to-scichart-converter.ts` - PostGIS → SciChart conversion
- `src/lib/state/postgres/chart-states/map-chart-state.svelte.ts` - Map chart state management

### Backend

- `src-tauri/src/polygon_processing/polygon_upload_manager.rs` - GeoJSON parsing and validation
- `src-tauri/src/polygon_processing/polygon_data_processor.rs` - Database insertion and type conversions
- `src-tauri/src/polygon_processing/mod.rs` - Module declarations

### Database

- `db/migrations/018-polygons-schema.sql` - Initial `polygons` table schema
- `db/migrations/026-enhance-polygons-table-for-map-charts.sql` - Added `metadata` and `source_file` columns

### Scripts

- `scripts/database/remove-hardcoded-map-data.ts` - Remove hardcoded mapData from charts

## 🎯 Key Design Decisions

1. **GeoJSON as Source Format**: Standard format for geographic data, widely supported
2. **WKT for PostGIS**: PostGIS requires WKT format for geometry insertion
3. **Decimal for NUMERIC**: `rust_decimal::Decimal` required for PostgreSQL `NUMERIC` type (not `f64`)
4. **JSONB for Metadata**: Flexible storage for arbitrary GeoJSON properties
5. **User Confirmation Step**: Allows editing before database insertion (name, description, styling)
6. **AG-Grid for Editing**: Provides table-based editing interface similar to LAS file upload
7. **Name Validation**: Auto-incrementing suffix (`_1`, `_2`, etc.) prevents duplicate name violations, project-scoped uniqueness (same pattern as chart name validation)
8. **data_source_config over chart_config.mapData**: New approach uses `data_source_config` to reference polygons/wells by ID, enabling realtime updates and better data management
9. **PostGIS Geometry Format**: PostGIS stores geometry as WKB internally, but Supabase/PostgREST automatically converts to GeoJSON when querying - this is why `scripts/db-exports/polygons.json` shows hex strings (WKB), but the actual queries return GeoJSON
10. **Realtime Reactivity**: Chart updates automatically when `data_source_config` changes via Supabase Realtime subscriptions
11. **Reusable Multi-Row Selection**: `ag-data-table-multi-row-select.svelte` component supports multiple data types (well, curve, polygon, surface, well_marker, seismic, node_execution) with type-safe `dataType` prop using `DataSourceType` from `chart-data-sources.ts` for compile-time type checking

## 🐛 Common Issues & Solutions

### Issue: "cannot convert between Rust type `f64` and Postgres type `numeric`"

**Solution**: Use `rust_decimal::Decimal` instead of `f64`:

```rust
let fill_opacity: Option<Decimal> = Decimal::from_str(&fill_opacity_val.to_string())
    .ok()
    .or(Some(Decimal::new(3, 1)));
```

### Issue: Parameter order mismatch in SQL

**Solution**: Ensure parameter array order matches SQL `$1, $2, ...` order exactly. Debug logging shows parameter indices.

### Issue: Invalid geometry in PostGIS

**Solution**: Ensure WKT format is correct:

- Polygon must be closed (first point = last point)
- Coordinates must be valid (longitude: -180 to 180, latitude: -90 to 90)
- Use SRID 4326 (WGS84) for geographic coordinates

### Issue: PostGIS geometry appears as hex string in exports

**Explanation**: `scripts/db-exports/polygons.json` shows PostGIS geometry as WKB (Well-Known Binary) hex string. This is normal - PostGIS stores geometry as binary internally. When fetched via Supabase client, it's automatically converted to GeoJSON format.

**Solution**: Use Supabase client for queries (not direct SQL exports) to get GeoJSON format automatically.

### Issue: Polygons not rendering on map after adding

**Debugging Steps**:

1. Check `charts.data_source_config` in database - verify polygon IDs are present
2. Check browser console for errors in `MapChartState.loadDataSources()`
3. Verify polygon geometry is valid GeoJSON format
4. Check `convertPostGisToSciChart()` output - ensure `outline` and `areaData` arrays are populated
5. Verify `MapChartState.reloadMap()` is being called after data sources are loaded

## 🎯 MVP vs Future Enhancements

### MVP (Completed)

**Scope**:

- ✅ GeoJSON file upload
- ✅ Store in `polygons` table (PostGIS)
- ✅ Convert PostGIS → SciChart format
- ✅ Select polygons in map chart data options
- ✅ Select wells in map chart data options
- ✅ Render polygons/wells in map charts
- ✅ Realtime updates when data sources are added/removed
- ✅ Remove hardcoded map data

**Rationale**:

- **Postgres is simpler** than GeoParquet/GeoArrow for MVP
- PostGIS provides spatial queries (find polygons within bounds, etc.)
- JSONB metadata column is flexible for choropleth coloring
- `data_source_config` enables better data management and realtime updates

### Future Enhancements (Post-MVP)

1. **Shapefile Support**:
   - Parse `.shp`, `.dbf`, `.shx` files
   - Handle multi-part polygons
   - Extract attributes from `.dbf` files

2. **KML Support**:
   - Parse `.kml` files
   - Handle nested folders
   - Extract styles and metadata

3. **GeoParquet/GeoArrow** (if needed):
   - Store large polygon datasets in Parquet format
   - Use GeoArrow for efficient spatial operations
   - Migrate from Postgres when dataset size exceeds limits

4. **Spatial Queries**:
   - Find polygons within bounds
   - Find polygons containing a point
   - Find polygons intersecting with other polygons

5. **Polygon Editing**:
   - Edit polygon vertices in UI
   - Add/remove holes
   - Merge/split polygons

6. **Polygon Validation**:
   - Check for self-intersections
   - Validate coordinate systems
   - Fix invalid geometries

7. **Display Settings**:
   - Allow editing polygon colors/opacity/stroke from chart UI
   - Persist display settings per polygon instance in chart
   - Support choropleth coloring based on metadata

## 📊 Comparison: Postgres vs GeoParquet/GeoArrow

### Postgres (Current Implementation)

**Pros**:

- ✅ Simple to implement (existing `polygons` table)
- ✅ PostGIS provides spatial queries out of the box
- ✅ JSONB metadata column is flexible
- ✅ Real-time updates via Supabase subscriptions
- ✅ Easy to query and filter
- ✅ No additional storage layer needed
- ✅ Automatic GeoJSON conversion via Supabase/PostgREST

**Cons**:

- ❌ Limited scalability for very large datasets (1000+ polygons)
- ❌ Slower for complex spatial operations on large datasets
- ❌ Requires PostGIS extension (already installed)

**Best For**: MVP, small to medium datasets (< 1000 polygons per project)

### GeoParquet/GeoArrow (Future)

**Pros**:

- ✅ Excellent for large datasets (10,000+ polygons)
- ✅ Efficient columnar storage
- ✅ Fast spatial operations with GeoArrow
- ✅ Integrates with existing Parquet storage layer

**Cons**:

- ❌ More complex to implement
- ❌ Requires additional conversion layer (PostGIS → GeoParquet)
- ❌ No real-time updates (file-based)
- ❌ Overkill for MVP

**Best For**: Large-scale datasets, advanced spatial analytics

## 🚀 Implementation Checklist

### Phase 1: Database Schema ✅

- [x] `polygons` table exists
- [x] Add `metadata` JSONB column
- [x] Add `source_file` TEXT column
- [x] Create migration script

### Phase 2: Backend Parser ✅

- [x] Create `polygon_file_parser.rs`
- [x] Implement GeoJSON parsing
- [ ] Implement Shapefile parsing (future)
- [ ] Implement KML parsing (future)
- [x] Create `polygon_data_processor.rs`
- [x] Implement PostGIS geometry conversion
- [x] Create Tauri commands
- [x] Add error handling

### Phase 3: Frontend UI ✅

- [x] Create `content-data-ingestion-polygon-dropzone.svelte`
- [x] Create `content-data-ingestion-polygon-processing.svelte`
- [x] Implement project selection
- [x] Implement polygon type selection
- [x] Implement metadata entry
- [x] Implement data review table
- [x] Add success/error notifications
- [x] Implement name validation and unique name generation

### Phase 4: PostGIS → SciChart Conversion ✅

- [x] Create `postgis-to-scichart-converter.ts`
- [x] Implement GeoJSON → SciChart conversion
- [ ] Add unit tests (future)

### Phase 5: Map Chart Integration ✅

- [x] Create `chart-data-source-polygon-selector.svelte`
- [x] Create `chart-data-source-well-selector.svelte`
- [x] Create `chart-data-source-select-dialog.svelte`
- [x] Create `chart-data-source-service.ts`
- [x] Update `MapChartState` to load polygons/wells from `data_source_config`
- [x] Test polygon/well rendering in map charts
- [x] Implement realtime reactivity
- [x] Remove hardcoded map data

## 📚 Related Files

- **Database Schema**: `db/migrations/018-polygons-schema.sql`
- **Map Chart State**: `src/lib/state/postgres/chart-states/map-chart-state.svelte.ts`
- **Map Helpers**: `src/lib/utils/maps/map-helpers.ts`
- **PostGIS Converter**: `src/lib/utils/maps/postgis-to-scichart-converter.ts`
- **Chart Data Source Service**: `src/lib/services/chart-data-source-service.ts`
- **Chart Data Source Registry**: `src/lib/data/chart-data-source-registry.ts`
- **LAS Upload Reference**: `markdown-guides/database/storage/io/file-upload/las-file-upload/las-file-upload.md`
- **Realtime Node Creation**: `markdown-guides/dags/node-creation/realtime-node-creation-add-to-active-pipeline.md`
- **SciChart Map Rendering**: `markdown-guides/charts/scichart/scichart-maps-render-data.md`
- **Remove Hardcoded Data Script**: `scripts/database/remove-hardcoded-map-data.ts`

## 🎓 Key Takeaways

1. **PostGIS Geometry Format**: PostGIS stores geometry as WKB (hex string) internally, but Supabase/PostgREST automatically converts to GeoJSON when querying. This is why `scripts/db-exports/polygons.json` shows hex strings - use Supabase client for queries to get GeoJSON.
2. **Name Validation**: Polygon names are validated and auto-incremented (`_1`, `_2`, etc.) to prevent duplicate name violations. Names are unique per project, not globally. Same pattern as chart name validation.
3. **data_source_config Pattern**: Use `data_source_config` to reference polygons/wells by ID, enabling realtime updates and better data management than hardcoding data in `chart_config`.
4. **Realtime Reactivity**: Chart updates automatically when `data_source_config` changes via Supabase Realtime subscriptions, similar to node creation in pipelines.
5. **Type Conversions**: Careful handling of f64 → Decimal for PostgreSQL NUMERIC, and PostGIS WKB → GeoJSON → SciChart format conversions.
6. **Modular Architecture**: Separate concerns: upload (polygon-upload-service), selection (chart-data-source-selectors), conversion (postgis-to-scichart-converter), rendering (MapChartState).
7. **Reusable Multi-Row Selection**: `ag-data-table-multi-row-select.svelte` component supports multiple data types (well, curve, polygon, surface, well_marker, seismic, node_execution) with type-safe `dataType` prop using `DataSourceType` from `chart-data-sources.ts` for compile-time type checking throughout the call stack.
