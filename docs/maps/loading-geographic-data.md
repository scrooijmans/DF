# Loading Geographic Data for Map Charts

This guide explains how to load external geographic data files (e.g., GeoJSON, pre-triangulated polygon data) into map charts in MudRock.

## Overview

Map charts in MudRock use geographic polygon data stored in the `chart_config.mapData` field. The data structure follows the format used by SciChart's MapExample, which uses pre-triangulated polygon data for optimal rendering performance.

## Data Format

### Required Structure

Geographic data must be an array of region objects, each containing:

```typescript
interface GeographicRegion {
  name: string;                    // Region name (e.g., "New South Wales")
  outline: number[][];             // Array of [longitude, latitude] pairs for borders
  areaData: number[][];            // Pre-triangulated polygon data (from poly2tri)
  metadata?: {                     // Optional metadata for choropleth coloring
    population?: number;
    area_km2?: number;
    population_density?: number;
    [key: string]: any;
  };
}
```

### Example Data Structure

```json
[
  {
    "name": "New South Wales",
    "outline": [
      [159.10542, -31.563994],
      [159.09775, -31.564275],
      [159.099634, -31.573372],
      ...
    ],
    "areaData": [
      [159.03796, -31.512156],
      [159.056995, -31.519772],
      [159.064657, -31.509666],
      ...
    ],
    "metadata": {
      "population": 8511151,
      "area_km2": 800150,
      "population_density": 10.49
    }
  },
  ...
]
```

## Data Sources

### 1. Pre-Triangulated Data (Recommended)

**Best for**: Production use, optimal performance

The official SciChart MapExample uses `australiaConverted.json`, which contains pre-triangulated polygon data generated using [poly2tri](https://github.com/r3mi/poly2tri.js), a 2D constrained Delaunay triangulation library.

**Advantages**:
- ✅ Optimal rendering performance
- ✅ Works directly with `ETriangleSeriesDrawMode.List`
- ✅ No client-side triangulation needed

**How to obtain**:
- Use the official SciChart examples repository
- Generate from GeoJSON using poly2tri library
- Use existing pre-triangulated datasets

### 2. GeoJSON Data

**Best for**: Raw geographic data, custom regions

GeoJSON is a standard format for geographic data. You'll need to:
1. Convert GeoJSON polygons to the required format
2. Triangulate polygons using poly2tri or similar library
3. Extract metadata for choropleth coloring

**Conversion Process**:
```typescript
// Pseudo-code for GeoJSON conversion
import { triangulate } from 'poly2tri';

function convertGeoJSONToMapData(geoJson: GeoJSON.FeatureCollection) {
  return geoJson.features.map(feature => {
    const coordinates = feature.geometry.coordinates[0]; // Polygon coordinates
    const triangulated = triangulate(coordinates); // Use poly2tri
    
    return {
      name: feature.properties.name,
      outline: coordinates,
      areaData: triangulated, // Pre-triangulated data
      metadata: feature.properties
    };
  });
}
```

### 3. Simplified Polygon Data

**Best for**: Testing, simple shapes

For testing purposes, you can use simplified rectangular coordinates (4-5 points per region). However, this will render as simple rectangles, not accurate geographic shapes.

**Example**:
```json
{
  "name": "New South Wales",
  "outline": [
    [141, -37.5],
    [141, -28],
    [153, -28],
    [153, -37.5],
    [141, -37.5]
  ],
  "areaData": [
    [141, -37.5],
    [141, -28],
    [153, -28],
    [153, -37.5],
    [141, -37.5]
  ]
}
```

## Loading Data into Database

### Option 1: Using the Update Script (Recommended)

Use the provided script to load geographic data from a file or URL:

```bash
# Load from local JSON file
bun run scripts/database/update-map-chart-data.ts \
  --file ./australiaConverted.json \
  --chart-id 97c446d2-6ca8-41ce-b9b7-5e4294a7193a

# Load from URL
bun run scripts/database/update-map-chart-data.ts \
  --url https://example.com/geographic-data.json \
  --chart-id <your-chart-id>

# Replace entire config (don't merge with existing)
bun run scripts/database/update-map-chart-data.ts \
  --file ./data.json \
  --replace

# Customize color scheme
bun run scripts/database/update-map-chart-data.ts \
  --file ./data.json \
  --color-key population_density \
  --color-start "#ff0000" \
  --color-end "#0000ff"
```

**Script Options**:
- `--file <path>`: Load from local JSON file
- `--url <url>`: Load from URL
- `--chart-id <id>`: Chart ID to update (default: map_chart ID)
- `--replace`: Replace entire chart_config (default: merge)
- `--color-key <key>`: Set color key for choropleth
- `--color-start <color>`: Set start color for gradient
- `--color-end <color>`: Set end color for gradient

### Option 2: Direct SQL Update

For advanced users, you can update the database directly using SQL:

```sql
-- Update chart_config with geographic data
UPDATE public.charts
SET chart_config = jsonb_set(
  chart_config,
  '{mapData}',
  $1::jsonb  -- Your geographic data array
)
WHERE id = '97c446d2-6ca8-41ce-b9b7-5e4294a7193a';
```

### Option 3: Programmatic Update (TypeScript)

```typescript
import { Client } from "pg";

const client = new Client({
  host: process.env.DB_HOST,
  port: parseInt(process.env.DB_PORT || "5432"),
  user: process.env.DB_USER,
  password: process.env.POSTGRES_PASSWORD,
  database: process.env.DB_NAME,
});

await client.connect();

// Load geographic data
const geographicData = await fetch("https://example.com/data.json")
  .then(r => r.json());

// Update chart config
await client.query(
  `UPDATE public.charts 
   SET chart_config = jsonb_set(
     chart_config,
     '{mapData}',
     $1::jsonb
   )
   WHERE id = $2`,
  [JSON.stringify(geographicData), chartId]
);

await client.end();
```

## Data Validation

The update script validates geographic data before updating the database:

- ✅ Each region must have a `name` (string)
- ✅ Each region must have an `outline` array with at least 3 coordinate pairs
- ✅ Each region must have an `areaData` array with at least 3 coordinate pairs
- ✅ All coordinates must be `[longitude, latitude]` pairs (numbers)

Invalid regions are skipped with warnings, but the update continues with valid regions.

## Coordinate System

**Important**: Coordinates must be in **WGS84** (EPSG:4326) format:
- **Longitude** (X-axis): -180 to 180 degrees
- **Latitude** (Y-axis): -90 to 90 degrees

For Australia:
- Longitude range: ~110 to ~155
- Latitude range: ~-45 to ~-10

## Performance Considerations

### Pre-Triangulated Data

- **Recommended**: Use pre-triangulated data (like `australiaConverted.json`)
- **Performance**: Optimal - no client-side processing needed
- **File Size**: Larger (hundreds of points per region)

### Client-Side Triangulation

- **Current Implementation**: Our code triangulates simple polygons on the client
- **Performance**: Good for simple shapes, slower for complex polygons
- **File Size**: Smaller (4-5 points per region for rectangles)

### Best Practices

1. **Pre-process data**: Triangulate polygons server-side or during data preparation
2. **Cache triangulated data**: Store pre-triangulated data in the database
3. **Optimize polygon complexity**: Use appropriate detail level for your use case
4. **Lazy load**: Load geographic data only when map chart is viewed

## Example: Loading Australia Map Data

### Step 1: Obtain Data

Get `australiaConverted.json` from the SciChart examples repository or generate from GeoJSON.

### Step 2: Update Database

```bash
# Using the update script
bun run scripts/database/update-map-chart-data.ts \
  --file ./australiaConverted.json \
  --chart-id 97c446d2-6ca8-41ce-b9b7-5e4294a7193a
```

### Step 3: Verify

Check the chart in the UI - it should now display accurate geographic shapes instead of rectangles.

## Troubleshooting

### Issue: Still seeing rectangles/squares

**Cause**: Database still has simplified rectangular coordinates

**Solution**: 
1. Verify the update script ran successfully
2. Check that the geographic data file has complex polygon coordinates (hundreds of points per region)
3. Ensure the chart ID is correct

### Issue: Invalid coordinate format

**Cause**: Coordinates not in `[longitude, latitude]` format

**Solution**: 
- Ensure coordinates are arrays of 2 numbers: `[lon, lat]`
- Check coordinate order (longitude first, then latitude)
- Verify coordinates are within valid ranges

### Issue: Regions not rendering

**Cause**: Invalid polygon data (less than 3 points, invalid coordinates)

**Solution**:
- Check console logs for validation warnings
- Ensure each region has at least 3 coordinate points
- Verify all coordinates are valid numbers

## References

- [SciChart MapExample](https://github.com/ABTSoftware/SciChart.JS.Examples/tree/master/Examples/src/components/Examples/Charts2D/v4Charts/MapExample)
- [poly2tri Library](https://github.com/r3mi/poly2tri.js)
- [GeoJSON Specification](https://geojson.org/)
- [Update Script](../scripts/database/update-map-chart-data.ts)

