# SciChart Map Data Rendering: End-to-End Guide

This document describes the complete end-to-end process of how map data is rendered in the SciChart map component, including data types, conversions, and rendering pipeline.

## 🎯 Overview

Map charts in MudRock display geographic data using SciChart's `FastTriangleRenderableSeries` for filled regions, `FastLineRenderableSeries` for borders, and `FastBubbleRenderableSeries` for markers. The rendering pipeline converts geographic polygon data from the database into SciChart-compatible formats.

## 📊 Data Flow Architecture

```
Database (PostgreSQL)
    ↓ chart_config.mapData (JSONB)
MapChartState.loadFromChartConfig()
    ↓ MapConfig interface
MapChartState.reloadMap()
    ↓ Triangulation & Data Conversion
SciChart Rendering
    ├─ FastTriangleRenderableSeries (filled regions)
    ├─ FastLineRenderableSeries (outlines)
    └─ FastBubbleRenderableSeries (markers)
```

## 📋 Data Types & Structures

### 1. Database Storage Format

**Location**: `charts.chart_config.mapData` (JSONB column)

**Structure**:

```typescript
interface GeographicRegion {
  name: string; // Region name (e.g., "New South Wales")
  outline: number[][]; // Array of [longitude, latitude] pairs for polygon border
  areaData: number[][]; // Array of [longitude, latitude] pairs for filled region
  metadata?: {
    // Optional metadata for choropleth coloring
    population?: number;
    area_km2?: number;
    population_density?: number;
    [key: string]: any;
  };
}
```

**Example**:

```json
{
  "mapData": [
    {
      "name": "New South Wales",
      "outline": [
        [141.0, -37.5],
        [141.0, -28.0],
        [153.0, -28.0],
        [153.0, -37.5],
        [141.0, -37.5]
      ],
      "areaData": [
        [141.0, -37.5],
        [141.0, -28.0],
        [153.0, -28.0],
        [153.0, -37.5],
        [141.0, -37.5]
      ],
      "metadata": {
        "population": 8511151,
        "area_km2": 800150,
        "population_density": 10.49
      }
    }
  ],
  "markers": [
    {
      "name": "Sydney",
      "longitude": 151.2093,
      "latitude": -33.8688,
      "metadata": { "population": 5312163 }
    }
  ],
  "colorKey": "population",
  "colorScheme": {
    "startColor": "#5dc0c0",
    "endColor": "#1e3489"
  },
  "preserveAspectRatio": true
}
```

### 2. MapConfig Interface (TypeScript)

**Location**: `src/lib/state/postgres/chart-states/map-chart-state.svelte.ts`

```typescript
interface MapConfig {
  mapData?: Array<{
    name: string;
    outline: number[][]; // [longitude, latitude] pairs
    areaData: number[][]; // [longitude, latitude] pairs
    metadata?: Record<string, any>;
  }>;
  markers?: Array<{
    name: string;
    longitude: number;
    latitude: number;
    metadata?: Record<string, any>;
  }>;
  colorKey?: string; // Key in metadata for choropleth coloring
  colorScheme?: {
    startColor?: string; // Hex color (default: "#5dc0c0")
    endColor?: string; // Hex color (default: "#1e3489")
  };
  visibleRange?: {
    minX: number; // Minimum longitude
    maxX: number; // Maximum longitude
    minY: number; // Minimum latitude
    maxY: number; // Maximum latitude
  };
  preserveAspectRatio?: boolean; // Default: true
}
```

### 3. Coordinate System

**Coordinate Format**: `[longitude, latitude]` (WGS84, EPSG:4326)

- **Longitude (X-axis)**: -180 to +180 degrees (e.g., 141.0 for Australia)
- **Latitude (Y-axis)**: -90 to +90 degrees (e.g., -37.5 for Australia)
- **Order**: Always `[longitude, latitude]` (X, Y) for consistency with SciChart axes

**Important**: SciChart uses standard Cartesian coordinates, so:

- **X-axis** = Longitude (horizontal)
- **Y-axis** = Latitude (vertical)

## 🔄 Data Conversion Pipeline

### Step 1: Load from Database

**Component**: `MapChartState.loadFromChartConfig()`

```typescript
loadFromChartConfig(chartConfig: Record<string, any>): void {
  const mapData = chartConfig.mapData || [];
  const markers = chartConfig.markers || [];
  const colorKey = chartConfig.colorKey || "population";
  const colorScheme = chartConfig.colorScheme || {
    startColor: "#5dc0c0",
    endColor: "#1e3489",
  };
  const visibleRange = chartConfig.visibleRange || null;
  const preserveAspectRatio = chartConfig.preserveAspectRatio !== false;

  this.config = {
    mapData,
    markers,
    colorKey,
    colorScheme,
    visibleRange,
    preserveAspectRatio,
  };
}
```

**Process**:

1. Extract `mapData` array from `chart_config` JSONB
2. Extract `markers` array (optional)
3. Extract `colorKey` for choropleth coloring (default: "population")
4. Extract `colorScheme` for color interpolation (default: cyan to blue)
5. Extract `visibleRange` for initial zoom level (optional)
6. Extract `preserveAspectRatio` flag (default: true)

### Step 2: Triangulation (Polygon → Triangles)

**Component**: `src/lib/utils/maps/map-helpers.ts::triangulatePolygon()`

**Purpose**: Convert polygon vertices into triangles for `FastTriangleRenderableSeries` with `ETriangleSeriesDrawMode.List`.

**Algorithm**: Fan Triangulation

```typescript
export function triangulatePolygon(polygonPoints: number[][]): number[] {
  // Remove closing point if present (same as first point)
  let points = polygonPoints;
  if (points.length >= 2) {
    const first = points[0];
    const last = points[points.length - 1];
    if (first[0] === last[0] && first[1] === last[1] && points.length > 3) {
      points = points.slice(0, -1);
    }
  }

  // Fan triangulation: create triangles from first point to each pair of consecutive points
  // For rectangle [0,1,2,3]: creates triangles [0,1,2] and [0,2,3]
  const triangles: number[] = [];
  for (let i = 1; i < points.length - 1; i++) {
    // Each triangle: [first, point[i], point[i+1]]
    triangles.push(
      points[0][0],
      points[0][1], // First point (x, y)
      points[i][0],
      points[i][1], // Current point (x, y)
      points[i + 1][0],
      points[i + 1][1], // Next point (x, y)
    );
  }

  return triangles;
}
```

**Input**: `[[x1, y1], [x2, y2], [x3, y3], [x4, y4]]` (4 polygon vertices)

**Output**: `[x1, y1, x2, y2, x3, y3, x1, y1, x3, y3, x4, y4]` (2 triangles, 12 values)

**Format**: Flattened array where each group of 6 consecutive values = 1 triangle (3 points × 2 coordinates)

**Example**:

- Rectangle with 4 corners → 2 triangles
- Pentagon with 5 corners → 3 triangles
- Complex polygon with 100+ points → 98 triangles

### Step 3: Color Interpolation (Metadata → Color)

**Component**: `src/lib/utils/maps/map-helpers.ts::interpolateColor()`

**Purpose**: Convert metadata values (e.g., population) into hex colors for choropleth visualization.

**Algorithm**: Linear RGB Interpolation

```typescript
export function interpolateColor(
  min: number,
  max: number,
  value: number,
  colorA: string = "#5dc0c0", // Start color (cyan)
  colorB: string = "#1e3489", // End color (blue)
): string {
  // Clamp value between min and max
  value = Math.max(min, Math.min(max, value));
  // Normalize to [0,1]
  const t = (value - min) / (max - min);

  // Parse hex colors to RGB
  const rgbA = hexToRgb(colorA); // [r, g, b]
  const rgbB = hexToRgb(colorB); // [r, g, b]

  // Interpolate each RGB component
  const r = lerp(rgbA[0], rgbB[0], t);
  const g = lerp(rgbA[1], rgbB[1], t);
  const b = lerp(rgbA[2], rgbB[2], t);

  // Convert back to hex
  return `#${[r, g, b].map((x) => x.toString(16).padStart(2, "0")).join("")}`;
}
```

**Process**:

1. Calculate min/max values from all regions' metadata
2. Normalize current value to [0, 1] range
3. Interpolate RGB components between start and end colors
4. Convert interpolated RGB to hex color string

**Example**:

- Population range: 255,559 (min) to 8,511,151 (max)
- Value: 5,608,666 (Queensland)
- Normalized: `(5608666 - 255559) / (8511151 - 255559) = 0.648`
- Interpolated color: `#3a8fb8` (between cyan and blue)

### Step 4: SciChart Series Creation

**Component**: `MapChartState.reloadMap()`

#### 4.1 Triangle Series (Filled Regions)

```typescript
// Triangulate polygon
const triangulated = triangulatePolygon(region.areaData);

// Extract x and y values from triangulated data
const xValues: number[] = [];
const yValues: number[] = [];

for (let i = 0; i < triangulated.length; i += 6) {
  // Each triangle is 6 consecutive values (3 points * 2 coordinates)
  xValues.push(triangulated[i], triangulated[i + 2], triangulated[i + 4]);
  yValues.push(triangulated[i + 1], triangulated[i + 3], triangulated[i + 5]);
}

// Create data series
const dataSeries = new XyDataSeries(this.wasmContext, {
  xValues: xValues,
  yValues: yValues,
});

// Interpolate color from metadata
const colorValue = region.metadata?.[this.config.colorKey || "population"] || 0;
const fillColor = interpolateColor(min, max, colorValue, startColor, endColor);

// Create triangle series with List mode
const triangleSeries = new FastTriangleRenderableSeries(this.wasmContext, {
  dataSeries: dataSeries,
  drawMode: ETriangleSeriesDrawMode.List, // Pre-triangulated data
  fill: fillColor,
  opacity: 0.9,
});
```

**Key Points**:

- `ETriangleSeriesDrawMode.List`: Expects pre-triangulated data (each group of 3 consecutive points = 1 triangle)
- `XyDataSeries`: Stores x/y coordinates as separate arrays
- `fill`: Hex color string from interpolation
- `opacity`: 0.9 (90% opaque, 10% transparent)

#### 4.2 Line Series (Outlines)

```typescript
// Extract x and y values from outline
const xVals = region.outline.map((d: number[]) => d[0]); // Longitude
const yVals = region.outline.map((d: number[]) => d[1]); // Latitude

// Create line series
const lineSeries = new FastLineRenderableSeries(this.wasmContext, {
  dataSeries: new XyDataSeries(this.wasmContext, {
    xValues: xVals,
    yValues: yVals,
  }),
  stroke: themeColors.tickTextBrush, // Theme color for borders
  strokeThickness: 2,
  opacity: 1,
});
```

**Key Points**:

- `outline`: Array of `[longitude, latitude]` pairs representing polygon border
- `stroke`: Theme color (typically dark gray/black)
- `strokeThickness`: 2 pixels
- `opacity`: 1.0 (fully opaque)

#### 4.3 Bubble Series (Markers)

```typescript
// Extract marker coordinates
const cLongitude = this.config.markers.map((d) => d.longitude);
const cLatitude = this.config.markers.map((d) => d.latitude);
const cSize = this.config.markers.map(() => 5); // Fixed size
const cMetadata = this.config.markers.map(
  (d) => d.metadata || { name: d.name },
) as unknown as IPointMetadata[];

// Create bubble series
this.markerSeries = new FastBubbleRenderableSeries(this.wasmContext, {
  pointMarker: new EllipsePointMarker(this.wasmContext, {
    width: 64,
    height: 64,
    fill: themeColors.foregroundColor,
    strokeThickness: 0,
  }),
  dataSeries: new XyzDataSeries(this.wasmContext, {
    xValues: cLongitude,
    yValues: cLatitude,
    zValues: cSize, // Bubble size
    metadata: cMetadata, // For labels
  }),
  dataLabels: {
    verticalTextPosition: EVerticalTextPosition.Above,
    horizontalTextPosition: EHorizontalTextPosition.Right,
    style: {
      fontFamily: "Arial",
      fontSize: 14,
      padding: new Thickness(0, 0, 3, 3),
    },
    color: "#EEE",
    metaDataSelector: (md) => {
      const metadata = md as unknown as { name: string };
      return metadata.name?.toString() || "";
    },
  },
});
```

**Key Points**:

- `XyzDataSeries`: Stores x, y, z (size) coordinates
- `EllipsePointMarker`: Circular markers
- `dataLabels`: Text labels above markers (e.g., city names)
- `metaDataSelector`: Extracts label text from metadata

## 🎨 Rendering Pipeline

### Initialization Flow

```
1. User selects map chart
   ↓
2. PostgresChartsState.loadChartState()
   ↓
3. MapChartState.initializeChart(container, chart)
   ↓
4. MapChartState.loadFromChartConfig(chart.chart_config)
   ↓
5. Initialize SciChart surface and axes
   ↓
6. MapChartState.reloadMap(skipInitializedCheck=true)
   ↓
7. Render triangle, line, and bubble series
```

### Rendering Steps

1. **Clear existing series**: Remove all previous renderable series
2. **Calculate color range**: Find min/max values from metadata
3. **Triangulate polygons**: Convert `areaData` to triangles
4. **Create triangle series**: One series per region (filled)
5. **Create line series**: One series per region (outlines)
6. **Create bubble series**: One series for all markers
7. **Add to surface**: Add all series to `sciChartSurface.renderableSeries`
8. **Zoom to extents**: Auto-fit map to data (if no visible range set)

### Aspect Ratio Preservation

**Component**: `src/lib/utils/maps/map-helpers.ts::preserveAspectRatio()`

**Purpose**: Maintain correct geographic proportions when container is resized.

**Algorithm**: Adjust visible range to match container aspect ratio

```typescript
export function preserveAspectRatio(
  width: number, // Container width
  height: number, // Container height
  minVisibleX: number, // Current min longitude
  maxVisibleX: number, // Current max longitude
  minVisibleY: number, // Current min latitude
  maxVisibleY: number, // Current max latitude
): {
  minVisibleX: number;
  maxVisibleX: number;
  minVisibleY: number;
  maxVisibleY: number;
} {
  const containerAspectRatio = width / height;
  const visibleAspectRatio =
    (maxVisibleX - minVisibleX) / (maxVisibleY - minVisibleY);

  if (containerAspectRatio > visibleAspectRatio) {
    // Container is wider - expand visible X range
    const newVisibleWidth = (maxVisibleY - minVisibleY) * containerAspectRatio;
    const widthDiff = newVisibleWidth - (maxVisibleX - minVisibleX);
    return {
      minVisibleX: minVisibleX - widthDiff / 2,
      maxVisibleX: maxVisibleX + widthDiff / 2,
      minVisibleY,
      maxVisibleY,
    };
  } else {
    // Container is taller - expand visible Y range
    const newVisibleHeight = (maxVisibleX - minVisibleX) / containerAspectRatio;
    const heightDiff = newVisibleHeight - (maxVisibleY - minVisibleY);
    return {
      minVisibleX,
      maxVisibleX,
      minVisibleY: minVisibleY - heightDiff / 2,
      maxVisibleY: maxVisibleY + heightDiff / 2,
    };
  }
}
```

**Implementation**: Subscribed to `sciChartSurface.preRender` event to adjust axes before each render.

## 📐 Coordinate System Details

### Geographic Coordinates (WGS84, EPSG:4326)

- **Format**: `[longitude, latitude]`
- **Longitude (X)**: -180° to +180° (West to East)
- **Latitude (Y)**: -90° to +90° (South to North)
- **Example**: `[151.2093, -33.8688]` = Sydney, Australia

### SciChart Axes Configuration

```typescript
// X-axis: Longitude
this.xAxis = new NumericAxis(this.wasmContext, {
  axisTitle: "Longitude",
  growBy: new NumberRange(0.1, 0.1), // 10% padding
  isVisible: true,
  drawLabels: true,
  drawMajorGridLines: true,
});

// Y-axis: Latitude
this.yAxis = new NumericAxis(this.wasmContext, {
  axisTitle: "Latitude",
  growBy: new NumberRange(0.1, 0.1), // 10% padding
  isVisible: true,
  drawLabels: true,
  drawMajorGridLines: true,
});
```

### Visible Range (Zoom Level)

**Storage**: `chart_config.visibleRange` or `chart_config.xAxes.options.visibleRange` / `chart_config.yAxes[].options.visibleRange`

**Format**:

```json
{
  "visibleRange": {
    "minX": 110, // Minimum longitude
    "maxX": 155, // Maximum longitude
    "minY": -45, // Minimum latitude
    "maxY": -10 // Maximum latitude
  }
}
```

**Default**: If not set, calculated from `mapData` bounds with 10% padding.

## 🔧 Data Loading from External Sources

### Loading from JSON File

**Script**: `scripts/database/update-map-chart-data.ts`

**Usage**:

```bash
# Load from local file
bun run scripts/database/update-map-chart-data.ts --file ./australiaConverted.json

# Load from URL
bun run scripts/database/update-map-chart-data.ts --url https://example.com/data.json

# Specify chart ID
bun run scripts/database/update-map-chart-data.ts --file ./data.json --chart-id <uuid>
```

**Data Format** (from SciChart examples):

```json
[
  {
    "name": "New South Wales",
    "outline": [[141.0, -37.5], [141.0, -28.0], ...],
    "areaData": [[141.0, -37.5], [141.0, -28.0], ...],
    "metadata": {
      "population": 8511151,
      "area_km2": 800150,
      "population_density": 10.49
    }
  }
]
```

**Process**:

1. Load JSON file (array of regions)
2. Validate data structure
3. Update `charts.chart_config.mapData` in database
4. Chart automatically reloads via realtime subscription

## 🎯 Performance Considerations

### Large Datasets

- **Triangulation**: O(n) for n polygon vertices (fan triangulation)
- **Rendering**: SciChart handles large datasets efficiently (WebAssembly)
- **Memory**: Each region creates separate series (can be optimized for 100+ regions)

### Optimization Strategies

1. **Downsampling**: Reduce polygon vertices for very complex shapes
2. **LOD (Level of Detail)**: Use simplified polygons at low zoom levels
3. **Caching**: Cache triangulated data in `chart_config` (future enhancement)
4. **Streaming**: Load regions incrementally for very large datasets (future enhancement)

## 🔄 State Persistence

### Saving Visible Range

**Component**: `MapChartState.saveVisibleRange()`

```typescript
async saveVisibleRange(): Promise<void> {
  if (!this.xAxis || !this.yAxis) return;

  this.visibleRange = {
    minX: this.xAxis.visibleRange.min,
    maxX: this.xAxis.visibleRange.max,
    minY: this.yAxis.visibleRange.min,
    maxY: this.yAxis.visibleRange.max,
  };

  // Saved to database via PostgresChartsState.saveChartStateToDatabase()
}
```

**Storage**: Saved in both formats for consistency:

- `chart_config.visibleRange` (map-specific)
- `chart_config.xAxes.options.visibleRange` / `chart_config.yAxes[].options.visibleRange` (consistent with other chart types)

### Loading Visible Range

**Priority**:

1. `chart_config.xAxes.options.visibleRange` / `chart_config.yAxes[].options.visibleRange` (if present)
2. `chart_config.visibleRange` (if present)
3. Calculate from `mapData` bounds (if mapData exists)
4. Default geographic range (e.g., Australia: 110-155°E, -45 to -10°N)

## 📚 Related Files

- **State Management**: `src/lib/state/postgres/chart-states/map-chart-state.svelte.ts`
- **Component**: `src/lib/components/pages/home/charts/chart-editor/sci-charts/sci-map-multi-layer.svelte`
- **Helpers**: `src/lib/utils/maps/map-helpers.ts`
- **Update Script**: `scripts/database/update-map-chart-data.ts`
- **Database Migration**: `db/migrations/023-add-map-chart-type.sql`

## 🎓 Key Takeaways

1. **Data Format**: Polygons stored as `[longitude, latitude]` arrays in JSONB
2. **Triangulation**: Required for `FastTriangleRenderableSeries` with `List` mode
3. **Color Interpolation**: Metadata values → hex colors via linear RGB interpolation
4. **Coordinate System**: WGS84 (EPSG:4326), `[longitude, latitude]` format
5. **Aspect Ratio**: Preserved via `preRender` subscription
6. **State Persistence**: Visible range saved in multiple formats for consistency
