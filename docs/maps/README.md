# Map Charts Documentation

Documentation for map/choropleth chart functionality in MudRock.

## Overview

Map charts display geographic data with color-coded regions using SciChart's `FastTriangleRenderableSeries` for filled regions, `FastLineRenderableSeries` for borders, and `FastBubbleRenderableSeries` for markers.

## Key Documents

- **[Loading Geographic Data](./loading-geographic-data.md)** - Complete guide on loading external geographic data files into map charts
- **[Chart Rendering Guide](../markdown-guides/svelte5/charts/chart-rendering.md)** - General chart rendering guidelines
- **[Chart Settings by Type](../markdown-guides/svelte5/charts/chart-settings-by-chart-type.md)** - Chart-type-specific settings

## Quick Start

### 1. Load Geographic Data

```bash
# Load from local JSON file
bun run scripts/database/update-map-chart-data.ts \
  --file ./australiaConverted.json \
  --chart-id 97c446d2-6ca8-41ce-b9b7-5e4294a7193a
```

### 2. Data Format

Geographic data must be an array of regions with:
- `name`: Region name
- `outline`: Array of `[longitude, latitude]` pairs for borders
- `areaData`: Pre-triangulated polygon data (from poly2tri)
- `metadata`: Optional metadata for choropleth coloring

### 3. Rendering

The map chart automatically:
- Renders filled regions using `FastTriangleRenderableSeries` with `ETriangleSeriesDrawMode.List`
- Draws borders using `FastLineRenderableSeries`
- Displays markers using `FastBubbleRenderableSeries`
- Applies color interpolation based on metadata values
- Preserves aspect ratio during resizing

## Data Sources

### Pre-Triangulated Data (Recommended)

Use pre-triangulated data like `australiaConverted.json` from the SciChart examples:
- ✅ Optimal performance
- ✅ Works directly with `List` draw mode
- ✅ No client-side processing needed

### GeoJSON Data

Convert GeoJSON to the required format:
1. Extract polygon coordinates
2. Triangulate using poly2tri library
3. Format as required structure

See [Loading Geographic Data Guide](./loading-geographic-data.md) for details.

## Troubleshooting

### Issue: Seeing rectangles instead of geographic shapes

**Solution**: Update database with proper polygon data using the update script:
```bash
bun run scripts/database/update-map-chart-data.ts --file ./australiaConverted.json
```

### Issue: Invalid coordinate format

**Solution**: Ensure coordinates are `[longitude, latitude]` pairs (numbers) in WGS84 format.

## Related Files

- **State Class**: `src/lib/state/postgres/chart-states/map-chart-state.svelte.ts`
- **Component**: `src/lib/components/pages/home/charts/chart-editor/sci-charts/sci-map-multi-layer.svelte`
- **Helpers**: `src/lib/utils/maps/map-helpers.ts`
- **Update Script**: `scripts/database/update-map-chart-data.ts`

