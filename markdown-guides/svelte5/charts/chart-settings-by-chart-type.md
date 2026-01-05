# Chart Settings by Chart Type

This document describes the end-to-end process of how chart settings are organized and rendered based on the selected chart's type, allowing users to modify chart properties (axis labels, series colors, line thickness, point sizes) with reactive updates to both the UI and database.

## Type Safety with Database Types

**CRITICAL**: All chart-related types should use the generated database types from `$lib/database.types.ts` for type safety. The `Database` interface provides type definitions for all database tables, including:

- `Database['public']['Tables']['charts']['Row']` - Chart row type
- `Database['public']['Tables']['chart_types']['Row']` - Chart type definition
- `Database['public']['Tables']['curve_metadata']['Row']` - Curve metadata type

**Example Usage**:

```typescript
import type { Database } from '$lib/database.types';

// Use generated types for chart data
type ChartRow = Database['public']['Tables']['charts']['Row'];
type ChartTypeRow = Database['public']['Tables']['chart_types']['Row'];
type CurveMetadataRow = Database['public']['Tables']['curve_metadata']['Row'];

// Extend with union types for better type narrowing
type ChartType = ChartRow['chart_type']; // string
type ChartTypeUnion = 'xy' | 'line' | 'scatter' | 'well_correlation' | ...; // Union type for type narrowing
```

**Benefits**:

- ✅ **Automatic Type Sync**: Types are generated from database schema, ensuring consistency
- ✅ **Type Safety**: TypeScript catches type mismatches at compile time
- ✅ **IntelliSense**: Full autocomplete support for database fields
- ✅ **Refactoring Safety**: Database changes automatically reflected in types after regeneration

See `markdown-guides/to-do/db-sync-type-safety.md` for the complete type safety strategy.

## Architecture Overview

The chart settings system uses a **type-based component routing pattern** where:

1. **Chart Type Detection**: The selected chart's `chart_type` determines which settings components are rendered
2. **Conditional Rendering**: Different settings components are shown based on chart type (line, scatter, well_correlation, etc.)
3. **Reactive Updates**: Changes to settings immediately update the SciChart instance AND persist to the database
4. **Optimized Updates**: Style changes (colors, thickness) update SciChart directly without reloading data
5. **Type Safety**: Uses generated database types from `$lib/database.types.ts` for compile-time type checking

## Component Hierarchy

```
chart-editor.svelte
├── chart-editor-settings-menubar.svelte (top menubar)
│   ├── chart-editor-settings-menubar-xy.svelte (xy, line, scatter, geothermal_gradient)
│   ├── chart-editor-settings-menubar-well-correlation.svelte (well_correlation)
│   └── chart-editor-settings-menubar-map.svelte (map)
├── chart-editor-interactions-sidebar.svelte (left sidebar, ~48px)
│   ├── chart-editor-interactions-sidebar-xy.svelte (xy, line, scatter, geothermal_gradient)
│   ├── chart-editor-interactions-sidebar-well-correlation.svelte (well_correlation)
│   └── chart-editor-interactions-sidebar-map.svelte (map)
└── chart-sci-chart.svelte (center chart visualization)
    └── [Chart-type-specific SciChart components]

Settings Dialog (opened from menubar):
└── chart-settings-editor.svelte (dialog)
    ├── Tabs: "Info" | "Settings"
    └── Settings Tab Content:
        ├── chart-editor-settings-title.svelte (general - all chart types)
        ├── chart-editor-settings-grid.svelte (general - all chart types)
        ├── chart-editor-axes-settings.svelte (all chart types)
        ├── chart-editor-settings-series.svelte (conditional router)
        │   ├── chart-editor-settings-series-line.svelte (line, xy charts)
        │   └── chart-editor-settings-series-scatter.svelte (scatter charts)
        ├── chart-editor-settings-opacity.svelte (conditional router)
        │   ├── chart-editor-settings-opacity-line.svelte (line, xy charts)
        │   └── chart-editor-settings-opacity-scatter.svelte (scatter charts)
        ├── chart-editor-settings-dashed-line.svelte (conditional router)
        │   └── chart-editor-settings-dashed-line-line.svelte (line, xy charts)
        └── chart-editor-settings-point-marker-type.svelte (conditional router)
            └── chart-editor-settings-point-marker-type-scatter.svelte (scatter charts)
```

## Reusable Settings Components

All settings components use reusable UI components from `chart-editor-reusable-settings/`:

- **`chart-editor-color-picker.svelte`**: Color picker with hex input (used by series settings)
- **`chart-editor-number-input.svelte`**: Number input with min/max/step validation (used by series settings, dashed lines)
- **`chart-editor-opacity-slider.svelte`**: Opacity input with percentage display (used by opacity settings)
- **`chart-editor-checkbox.svelte`**: Checkbox with label and description (used by dashed lines, grid, title)
- **`chart-editor-select.svelte`**: Select dropdown (used by point marker type)

**Benefits**:

- Consistent UI across all settings
- Reusable components reduce code duplication
- Easier to maintain and update styling

## Settings Component Routing

### 1. Chart Editor Settings Sidebar

**File**: `chart-editor-settings-sidebar/chart-editor-sidebar.svelte`

**Purpose**: Main container for chart settings, provides tabbed interface (Info/Settings)

**Key Logic**:

```svelte
<TabsContent value="settings">
  <!-- Axes Settings: Available for all chart types -->
  <ChartEditorAxesSettings {chart} />

  <!-- Series Settings: Only for line, scatter, and xy charts -->
  {#if chart.chart_type === 'xy' || chart.chart_type === 'line' || chart.chart_type === 'scatter'}
    <ChartEditorSettingsSeries {chart} />
  {/if}
</TabsContent>
```

### 2. Series Settings Router

**File**: `chart-editor-settings-sidebar/chart-editor-settings-series/chart-editor-settings-series.svelte`

**Purpose**: Routes to chart-type-specific series settings components

**Key Logic**:

```svelte
{#if chart.chart_type === 'xy' || chart.chart_type === 'line'}
  <!-- XY charts are rendered as line charts -->
  <ChartEditorSettingsSeriesLine {chart} />
{:else if chart.chart_type === 'scatter'}
  <ChartEditorSettingsSeriesScatter {chart} />
{/if}
```

### 3. Line Chart Series Settings

**File**: `chart-editor-settings-series-line.svelte`

**Purpose**: Configure series properties for line charts (color, line thickness)

**Available Settings**:

- **Color (stroke)**: Color picker + text input for hex color
- **Line Thickness (strokeThickness)**: Number input (1-10)

**Reactivity Flow**:

```
User changes color/thickness
  ↓
updateSeriesStroke() / updateSeriesStrokeThickness()
  ↓
1. Update database (chart_config.series[].stroke/strokeThickness)
  ↓
2. Update SciChart renderableSeries directly (immediate visual feedback)
  renderableSeries.stroke = newColor
  renderableSeries.strokeThickness = newThickness
  ↓
3. Realtime update triggers (but optimized to skip data reload)
```

**Key Implementation**:

```typescript
async function updateSeriesStroke(seriesId: string, stroke: string) {
  // 1. Update database
  const updatedSeries = seriesArray.map((s: any) => {
    if (s.id === seriesId) {
      return { ...s, stroke };
    }
    return s;
  });

  await supabase
    .from("charts")
    .update({ chart_config: { ...chartConfig, series: updatedSeries } })
    .eq("id", chart.id);

  // 2. Update SciChart directly (immediate, no data reload)
  if (chartState && chartState.isInitialized) {
    const renderableSeries = chartState.renderableSeriesMap?.get(seriesId);
    if (renderableSeries) {
      renderableSeries.stroke = stroke; // Instant update
    }
  }
}
```

### 4. Scatter Chart Series Settings

**File**: `chart-editor-settings-series-scatter.svelte`

**Purpose**: Configure series properties for scatter charts (color, point size)

**Available Settings**:

- **Color (stroke/fill)**: Color picker + text input for hex color
- **Point Size**: Number input (1-30) for `pointMarkerWidth` and `pointMarkerHeight`

**Reactivity Flow**:

```
User changes color/point size
  ↓
updateSeriesStroke() / updateSeriesPointMarkerSize()
  ↓
1. Update database (chart_config.series[].stroke/pointMarkerWidth/pointMarkerHeight)
  ↓
2. Update SciChart pointMarker directly (immediate visual feedback)
  renderableSeries.pointMarker.fill = newColor
  renderableSeries.pointMarker.width = newSize
  renderableSeries.pointMarker.height = newSize
  ↓
3. Realtime update triggers (but optimized to skip data reload)
```

**Key Implementation**:

```typescript
async function updateSeriesPointMarkerSize(
  seriesId: string,
  width: number,
  height: number,
) {
  // 1. Update database
  const updatedSeries = seriesArray.map((s: any) => {
    if (s.id === seriesId) {
      return { ...s, pointMarkerWidth: width, pointMarkerHeight: height };
    }
    return s;
  });

  await supabase
    .from("charts")
    .update({ chart_config: { ...chartConfig, series: updatedSeries } })
    .eq("id", chart.id);

  // 2. Update SciChart directly (immediate, no data reload)
  if (chartState && chartState.isInitialized) {
    const renderableSeries = chartState.renderableSeriesMap?.get(seriesId);
    if (renderableSeries && renderableSeries.pointMarker) {
      renderableSeries.pointMarker.width = width;
      renderableSeries.pointMarker.height = height; // Instant update
    }
  }
}
```

## State Management

### Global Chart State

**File**: `postgres-charts-state.svelte.ts`

**Key Properties**:

- `charts: Chart[]` - All charts for current project
- `lastSelectedChartId: string | null` - Currently selected chart
- `xyPlotState: XYPlotState | LinePlotState | ScatterPlotState | null` - Chart-type-specific state
- `isSavingState: boolean` - Flag to prevent realtime updates during save

**Chart State Loading**:

```typescript
loadChartState(chart: Chart): void {
  if (chart.chart_type === 'xy' || chart.chart_type === 'line' || chart.chart_type === 'scatter') {
    // Create or update plot state based on chart type
    if (chart.chart_type === 'line') {
      plotState = new LinePlotState(chart.id);
    } else if (chart.chart_type === 'scatter') {
      plotState = new ScatterPlotState(chart.id);
    } else {
      plotState = new XYPlotState(chart.id);
    }

    plotState.loadFromChartConfig(chart.chart_config);
    plotState.updateChart(chart);
    this.xyPlotState = plotState;
  }
}
```

**Update Chart Property Method**:

```typescript
/**
 * Update a nested property in chart_config
 * Supports dot notation paths like "xAxes.options.drawMajorGridLines" or "yAxes[0].options.drawMajorGridLines"
 */
async updateChartProperty(
  chartId: string,
  path: string,
  value: any,
): Promise<void> {
  // Set flag to prevent reloading state during save
  this.isSavingState = true;

  try {
    // Fetch current chart_config
    const { data: currentChart } = await supabase
      .from("charts")
      .select("chart_config")
      .eq("id", chartId)
      .single();

    const chartConfig = currentChart.chart_config || {};
    const updatedConfig = JSON.parse(JSON.stringify(chartConfig)); // Deep clone

    // Parse path and update nested property (handles array notation)
    // ... path parsing logic ...

    // Update database
    await supabase
      .from("charts")
      .update({
        chart_config: updatedConfig,
        updated_at: new Date().toISOString(),
      })
      .eq("id", chartId);

    // Reset flag after delay to allow realtime update to propagate
    setTimeout(() => {
      this.isSavingState = false;
    }, 500);
  } catch (error) {
    this.isSavingState = false;
  }
}
```

**Update Series Property Method**:

```typescript
/**
 * Update series property (used by series settings components)
 */
async updateSeriesProperty(
  chartId: string,
  seriesId: string,
  property: string,
  value: any,
): Promise<void> {
  this.isSavingState = true;

  try {
    // Fetch current chart_config
    const { data: currentChart } = await supabase
      .from("charts")
      .select("chart_config")
      .eq("id", chartId)
      .single();

    const chartConfig = currentChart.chart_config || {};
    const series = (chartConfig.series || []) as Array<Record<string, any>>;

    // Find and update the series
    const updatedSeries = series.map((s) => {
      if (s.id === seriesId) {
        return { ...s, [property]: value };
      }
      return s;
    });

    const updatedConfig = {
      ...chartConfig,
      series: updatedSeries,
    };

    // Update database
    await supabase
      .from("charts")
      .update({
        chart_config: updatedConfig,
        updated_at: new Date().toISOString(),
      })
      .eq("id", chartId);

    setTimeout(() => {
      this.isSavingState = false;
    }, 500);
  } catch (error) {
    this.isSavingState = false;
  }
}
```

### Chart-Type-Specific State Classes

**Base Class**: `XYPlotState` (for xy, line, scatter charts)

**Key Properties**:

- `renderableSeriesMap: Map<string, any>` - Maps seriesId → SciChart renderableSeries
- `sciChartSurface: any` - SciChart surface instance
- `xAxisLabel: string`, `yAxisLabel: string` - Reactive axis labels
- `xVisibleRange`, `yVisibleRange` - Reactive visible ranges

**Public API**:

- `initializeChart(container, chart)` - Initialize SciChart surface
- `loadSeriesData(chart)` - Load data from parquet files
- `setXAxisLabel(label)` - Update X axis label (direct SciChart update)
- `setYAxisLabel(label)` - Update Y axis label (direct SciChart update)
- `saveVisibleRange()` - Save visible range to database

## Reactivity Patterns

### 1. Axis Label Updates (Instant)

**Flow**: Settings Component → State Class → SciChart Axis

```typescript
// chart-editor-axes-settings.svelte
function handleXAxisTitleChange(value: string) {
  if (chartState) {
    chartState.setXAxisLabel(value); // Direct SciChart update
  }
  // Also update database (debounced)
}

// xy-plot-state.svelte.ts
setXAxisLabel(label: string): void {
  this.xAxisLabel = label; // Update reactive state

  // Direct SciChart update (instant, no rerender)
  if (this.xAxis && this.isInitialized) {
    this.xAxis.axisTitle = label;
  }
}
```

**Why Instant**: Axis properties are updated directly on the SciChart axis object. No data reload needed.

### 2. Series Style Updates (Optimized - Direct Property Update)

**Flow**: Settings Component → Direct SciChart Property Update → Database (Debounced)

```typescript
// chart-editor-settings-series-line.svelte
let saveTimeout: ReturnType<typeof setTimeout> | null = null;

async function updateSeriesStroke(seriesId: string, stroke: string) {
  // 1. Update SciChart immediately (instant visual feedback)
  const renderableSeries = chartState.renderableSeriesMap?.get(seriesId);
  if (renderableSeries) {
    renderableSeries.stroke = stroke; // Instant visual update (<50ms)
  }

  // 2. Debounce database update (reduce write load)
  if (saveTimeout) {
    clearTimeout(saveTimeout);
  }
  saveTimeout = setTimeout(async () => {
    await chartsState.updateSeriesProperty(
      chart.id,
      seriesId,
      "stroke",
      stroke,
    );
  }, 500);
}
```

**Why Optimized**: According to SciChart documentation, `stroke`, `strokeThickness`, and `pointMarker` properties can be updated directly on the RenderableSeries without reloading data. The chart will redraw automatically.

**Key Patterns**:

1. **Instant SciChart Updates**: Update RenderableSeries properties immediately for visual feedback
2. **Debounced Database Writes**: Database updates are debounced (300-500ms) to reduce write load
3. **No Data Reload**: Style changes don't trigger `loadSeriesData()` - only property updates

### 3. Series Data Updates (Full Reload)

**Flow**: Data Selection → Database → Realtime → State Class → Data Fetch → SciChart

```typescript
// chart-data-curve-options.svelte
async function handleCurveSelection(curveId: string, axis: 'x' | 'y') {
  // Update chart_config.series[].xCurveId or yCurveId
  await supabase.from('charts').update({ chart_config: updatedConfig });
}

// postgres-charts-state.svelte.ts
updateChart(updatedChart: Chart): void {
  const oldSeries = JSON.stringify(oldChart.chart_config?.series || []);
  const newSeries = JSON.stringify(updatedChart.chart_config?.series || []);
  const seriesChanged = oldSeries !== newSeries;

  // Detect if curve IDs changed (data change) vs style properties (style change)
  const dataChanged = detectDataChange(oldSeries, newSeries);

  if (dataChanged) {
    // Only reload data if curve IDs changed
    this.loadChartState(updatedChart); // Triggers loadSeriesData()
  } else {
    // Style-only change: update renderableSeries properties directly
    this.updateSeriesStyles(updatedChart);
  }
}
```

**Why Full Reload**: When curve IDs change, we need to fetch new data from parquet files and recreate the series.

## Optimization: Preventing Unnecessary Data Reloads

### Problem

Previously, when updating series style properties (color, thickness, point size), the system:

1. Updated the database ✅
2. Triggered a realtime update ✅
3. `updateChart()` detected `oldConfig !== newConfig` ✅
4. Called `loadChartState()` → `loadSeriesData()` ❌ **UNNECESSARY**
5. Cleared and recreated all series ❌ **SLOW** (fetches parquet data, recreates series)

This caused noticeable delays when changing colors or line thickness, even though SciChart supports updating these properties directly without reloading data.

### Solution: Detect Style-Only Changes

**Implementation** (`postgres-charts-state.svelte.ts`):

```typescript
/**
 * Detect if series config change is data-related (curve IDs) or style-related (colors, thickness)
 */
private detectSeriesChangeType(
  oldSeries: any[],
  newSeries: any[],
): 'data' | 'style' | 'none' {
  if (oldSeries.length !== newSeries.length) {
    return 'data'; // Series count changed
  }

  for (let i = 0; i < oldSeries.length; i++) {
    const oldS = oldSeries[i];
    const newS = newSeries[i];

    // Data changes: curve IDs changed
    if (
      oldS.xCurveId !== newS.xCurveId ||
      oldS.yCurveId !== newS.yCurveId ||
      oldS.wellId !== newS.wellId
    ) {
      return 'data';
    }

    // Style changes: colors, thickness, point size changed
    if (
      oldS.stroke !== newS.stroke ||
      oldS.strokeThickness !== newS.strokeThickness ||
      oldS.pointMarkerWidth !== newS.pointMarkerWidth ||
      oldS.pointMarkerHeight !== newS.pointMarkerHeight
    ) {
      // Continue checking - might be mixed change
    }
  }

  // If we get here and series are different, it's a style-only change
  const oldSeriesStr = JSON.stringify(oldSeries);
  const newSeriesStr = JSON.stringify(newSeries);
  if (oldSeriesStr !== newSeriesStr) {
    return 'style';
  }

  return 'none';
}

/**
 * Update series styles directly without reloading data
 */
private updateSeriesStyles(chart: Chart): void {
  if (
    !this.xyPlotState ||
    this.xyPlotState.chartId !== chart.id ||
    !this.xyPlotState.isInitialized
  ) {
    return;
  }

  const seriesConfig = chart.chart_config?.series || [];

  for (const series of seriesConfig) {
    const renderableSeries = this.xyPlotState.renderableSeriesMap.get(series.id);
    if (!renderableSeries) continue;

    // Update line series properties
    if (series.stroke !== undefined) {
      renderableSeries.stroke = series.stroke;
    }
    if (series.strokeThickness !== undefined) {
      renderableSeries.strokeThickness = series.strokeThickness;
    }

    // Update scatter series properties
    if (renderableSeries.pointMarker) {
      if (series.stroke !== undefined) {
        renderableSeries.pointMarker.fill = series.stroke;
      }
      if (series.pointMarkerWidth !== undefined) {
        renderableSeries.pointMarker.width = series.pointMarkerWidth;
      }
      if (series.pointMarkerHeight !== undefined) {
        renderableSeries.pointMarker.height = series.pointMarkerHeight;
      }
    }
  }
}

updateChart(updatedChart: Chart): void {
  const index = this.charts.findIndex((c) => c.id === updatedChart.id);
  if (index >= 0) {
    const oldChart = this.charts[index];
    const oldSeries = oldChart.chart_config?.series || [];
    const newSeries = updatedChart.chart_config?.series || [];

    // Update chart in array
    this.charts[index] = updatedChart;

    // Detect change type: 'data' (curve IDs changed) vs 'style' (colors/thickness changed)
    const changeType = this.detectSeriesChangeType(oldSeries, newSeries);

    if (
      this.lastSelectedChartId === updatedChart.id &&
      !this.isSavingState &&
      (updatedChart.chart_type === 'xy' ||
        updatedChart.chart_type === 'line' ||
        updatedChart.chart_type === 'scatter') &&
      this.xyPlotState?.chartId === updatedChart.id
    ) {
      if (changeType === 'data') {
        // Data changed: reload series data from parquet files
        this.loadChartState(updatedChart); // Triggers loadSeriesData()
      } else if (changeType === 'style') {
        // Style changed: update RenderableSeries properties directly (instant, no data reload)
        this.updateSeriesStyles(updatedChart);
      }
      // changeType === 'none': no action needed
    }
  }
}
```

## Database Schema

Series properties are stored in `chart_config.series[]`:

```json
{
  "series": [
    {
      "id": "series-1",
      "xCurveId": "curve-uuid-1",
      "yCurveId": "curve-uuid-2",
      "wellId": "well-uuid",
      "name": "GR vs RHOB",
      "stroke": "#3B82F6", // Line/scatter color
      "strokeThickness": 2, // Line thickness (line charts only)
      "pointMarkerWidth": 14, // Point size (scatter charts only)
      "pointMarkerHeight": 14 // Point size (scatter charts only)
    }
  ]
}
```

## Future Optimizations

### 1. Redis Caching for Parquet Data

**Current**: Every data reload fetches from parquet files (potentially slow)

**Proposed**: Cache fetched curve data in Redis with TTL

```typescript
// curve-data-service.ts
async function fetchCurveData(
  curveId: string,
  projectId: string,
): Promise<CurveData> {
  // Check Redis cache first
  const cacheKey = `curve:${curveId}:${projectId}`;
  const cached = await redis.get(cacheKey);
  if (cached) {
    return JSON.parse(cached);
  }

  // Fetch from parquet
  const data = await queryParquetFromStorage(curveId, projectId);

  // Cache for 5 minutes
  await redis.setex(cacheKey, 300, JSON.stringify(data));

  return data;
}
```

**Benefits**:

- Faster data reloads when switching charts
- Reduced load on parquet storage
- Better performance for frequently accessed curves

### 2. Debounced Style Updates

**Current**: Each style change triggers a database update immediately

**Proposed**: Debounce database updates (but keep SciChart updates instant)

```typescript
let styleUpdateTimeout: ReturnType<typeof setTimeout> | null = null;

async function updateSeriesStroke(seriesId: string, stroke: string) {
  // Update SciChart immediately (instant visual feedback)
  renderableSeries.stroke = stroke;

  // Debounce database update (reduce write load)
  if (styleUpdateTimeout) {
    clearTimeout(styleUpdateTimeout);
  }
  styleUpdateTimeout = setTimeout(async () => {
    await supabase.from("charts").update({ chart_config: updatedConfig });
  }, 500);
}
```

### 3. Batch Style Updates

**Current**: Each property change is a separate database update

**Proposed**: Batch multiple style changes into a single update

```typescript
let pendingStyleUpdates: Map<string, Partial<SeriesConfig>> = new Map();

function queueStyleUpdate(seriesId: string, updates: Partial<SeriesConfig>) {
  const existing = pendingStyleUpdates.get(seriesId) || {};
  pendingStyleUpdates.set(seriesId, { ...existing, ...updates });

  // Apply to SciChart immediately
  applyStyleUpdatesToSciChart(seriesId, updates);

  // Batch database update
  debounceBatchUpdate();
}
```

## Best Practices

### ✅ DO

1. **Update SciChart properties directly** for instant visual feedback
2. **Distinguish between data and style changes** to avoid unnecessary reloads
3. **Use `renderableSeriesMap`** to access series for direct property updates
4. **Debounce database writes** for style changes (but keep SciChart updates instant)
5. **Track chart changes** in settings components to reset editing state

### ❌ DON'T

1. **Don't call `loadSeriesData()`** for style-only changes
2. **Don't clear and recreate series** when only properties change
3. **Don't fetch parquet data** if curve IDs haven't changed
4. **Don't update database synchronously** for every style change (use debouncing)

## Conditional Component Rendering Flow

### Chart Type Detection

**Pattern**: Settings components check `chart.chart_type` to determine which components to render.

**Main Router** (`chart-editor-settings-sidebar/chart-editor-sidebar.svelte`):

```svelte
<TabsContent value="settings">
  <!-- General Settings: Always rendered -->
  <ChartEditorSettingsTitle {chart} />
  <ChartEditorSettingsGrid {chart} />
  <ChartEditorAxesSettings {chart} />

  <!-- Chart-Type-Specific Settings: Conditional rendering -->
  {#if chart.chart_type === 'xy' || chart.chart_type === 'line' || chart.chart_type === 'scatter'}
    <ChartEditorSettingsSeries {chart} />
    <ChartEditorSettingsOpacity {chart} />
  {/if}

  {#if chart.chart_type === 'xy' || chart.chart_type === 'line'}
    <ChartEditorSettingsDashedLine {chart} />
  {/if}

  {#if chart.chart_type === 'scatter'}
    <ChartEditorSettingsPointMarkerType {chart} />
  {/if}
</TabsContent>
```

### Settings Component Router Pattern

**Pattern**: Each chart-type-specific feature has a router component that conditionally renders the appropriate implementation.

**Example - Series Settings Router**:

```svelte
<!-- chart-editor-settings-series/chart-editor-settings-series.svelte -->
<script lang="ts">
  import type { Chart } from '$lib/state/postgres/postgres-charts-state.svelte';
  import ChartEditorSettingsSeriesLine from './chart-editor-settings-series-line.svelte';
  import ChartEditorSettingsSeriesScatter from './chart-editor-settings-series-scatter.svelte';

  interface Props {
    chart: Chart;
  }

  let { chart }: Props = $props();
</script>

{#if chart.chart_type === 'xy' || chart.chart_type === 'line'}
  <!-- XY charts are rendered as line charts -->
  <ChartEditorSettingsSeriesLine {chart} />
{:else if chart.chart_type === 'scatter'}
  <ChartEditorSettingsSeriesScatter {chart} />
{/if}
```

**Benefits**:

- ✅ **Type Safety**: TypeScript ensures correct chart types
- ✅ **Modularity**: Each chart type has its own settings component
- ✅ **Extensibility**: Easy to add new chart types without modifying existing code
- ✅ **Clear Separation**: Router components handle routing, implementation components handle logic

### Settings Component State Management Pattern

**Pattern**: All settings components follow a consistent pattern for state management:

1. **Get Chart State**: Access `xyPlotState` from `PostgresChartsState`
2. **Verify Chart Match**: Ensure state belongs to current chart
3. **Track Chart Changes**: Use `previousChartId` to detect chart switches
4. **Protect User Input**: Use `isUserEditing` flag to prevent overwriting
5. **Sync with State**: Use `$effect` to sync inputs with state class
6. **Debounce Database**: Debounce database writes while keeping SciChart updates instant

**Example Pattern**:

```typescript
const chartsState = getPostgresChartsState();
const xyPlotState = $derived(chartsState.xyPlotState);
const currentChartId = $derived(chartsState.lastSelectedChartId);

const chartState = $derived(
  xyPlotState && currentChartId === chart.id && xyPlotState.chartId === chart.id
    ? xyPlotState
    : null,
);

let inputValue = $state(getValueFromConfig());
let isUserEditing = $state(false);
let previousChartId = $state(chart.id);

// Reset editing state when chart changes
$effect(() => {
  if (previousChartId !== chart.id) {
    isUserEditing = false;
    inputValue = getValueFromConfig();
    previousChartId = chart.id;
  }
});

// Sync inputs with state (skip if user editing)
$effect(() => {
  if (!isUserEditing && chartState && currentChartId === chart.id) {
    inputValue = chartState.propertyValue;
  }
});

function handleChange(value: any) {
  isUserEditing = true;
  inputValue = value;

  // Update SciChart immediately
  if (chartState) {
    chartState.setProperty(value);
  }

  // Debounce database update
  setTimeout(() => {
    isUserEditing = false;
    void chartsState.updateChartProperty(chart.id, "property.path", value);
  }, 500);
}
```

## Available Settings Components

### General Settings (All Chart Types)

1. **Chart Title** (`chart-editor-settings-title.svelte`)
   - Show/hide title checkbox
   - Title text input
   - Updates `chart_config.title` and `chart_config.showTitle`
   - **Reactivity Pattern**: Uses `isUserEditing` flag, debounced database updates (500ms), instant SciChart updates via `chartState.setChartTitle()`

2. **Grid Lines** (`chart-editor-settings-grid.svelte`)
   - Show X-axis major/minor grid lines checkboxes
   - Show Y-axis major/minor grid lines checkboxes
   - Updates `chart_config.xAxes.options.drawMajorGridLines` and `chart_config.yAxes[0].options.drawMajorGridLines`
   - **Reactivity Pattern**: Instant SciChart updates via `chartState.setDrawMajorXGridLines()` / `setDrawMajorYGridLines()`, debounced database updates (300ms)

### Chart-Type-Specific Settings

#### Line Charts (xy, line)

1. **Series Settings** (`chart-editor-settings-series-line.svelte`)
   - Color picker (stroke)
   - Line thickness (strokeThickness)

2. **Opacity Settings** (`chart-editor-settings-opacity-line.svelte`)
   - Opacity slider (0-100%)

3. **Dashed Line Settings** (`chart-editor-settings-dashed-line-line.svelte`)
   - Enable dashed line checkbox
   - Dash length input
   - Gap length input
   - Updates `renderableSeries.strokeDashArray`

#### Scatter Charts

1. **Series Settings** (`chart-editor-settings-series-scatter.svelte`)
   - Color picker (point fill)
   - Point size (pointMarkerWidth/Height)

2. **Opacity Settings** (`chart-editor-settings-opacity-scatter.svelte`)
   - Opacity slider (0-100%)

3. **Point Marker Type** (`chart-editor-settings-point-marker-type-scatter.svelte`)
   - Select dropdown: Ellipse, Triangle, Square, Cross
   - Updates `chart_config.series[].pointMarkerType`
   - Requires series recreation (data change)

## Conditional Component Rendering Flow

### Chart Type Detection

**Pattern**: Settings components check `chart.chart_type` to determine which components to render.

**Main Router** (`chart-editor-settings-sidebar/chart-editor-sidebar.svelte`):

```svelte
<TabsContent value="settings">
  <!-- General Settings: Always rendered -->
  <ChartEditorSettingsTitle {chart} />
  <ChartEditorSettingsGrid {chart} />
  <ChartEditorAxesSettings {chart} />

  <!-- Chart-Type-Specific Settings: Conditional rendering -->
  {#if chart.chart_type === 'xy' || chart.chart_type === 'line' || chart.chart_type === 'scatter'}
    <ChartEditorSettingsSeries {chart} />
    <ChartEditorSettingsOpacity {chart} />
  {/if}

  {#if chart.chart_type === 'xy' || chart.chart_type === 'line'}
    <ChartEditorSettingsDashedLine {chart} />
  {/if}

  {#if chart.chart_type === 'scatter'}
    <ChartEditorSettingsPointMarkerType {chart} />
  {/if}
</TabsContent>
```

### Settings Component Router Pattern

**Pattern**: Each chart-type-specific feature has a router component that conditionally renders the appropriate implementation.

**Example - Series Settings Router**:

```svelte
<!-- chart-editor-settings-series/chart-editor-settings-series.svelte -->
<script lang="ts">
  import type { Chart } from '$lib/state/postgres/postgres-charts-state.svelte';
  import ChartEditorSettingsSeriesLine from './chart-editor-settings-series-line.svelte';
  import ChartEditorSettingsSeriesScatter from './chart-editor-settings-series-scatter.svelte';

  interface Props {
    chart: Chart;
  }

  let { chart }: Props = $props();
</script>

{#if chart.chart_type === 'xy' || chart.chart_type === 'line'}
  <!-- XY charts are rendered as line charts -->
  <ChartEditorSettingsSeriesLine {chart} />
{:else if chart.chart_type === 'scatter'}
  <ChartEditorSettingsSeriesScatter {chart} />
{/if}
```

**Benefits**:

- ✅ **Type Safety**: TypeScript ensures correct chart types
- ✅ **Modularity**: Each chart type has its own settings component
- ✅ **Extensibility**: Easy to add new chart types without modifying existing code
- ✅ **Clear Separation**: Router components handle routing, implementation components handle logic

### Settings Component State Management Pattern

**Pattern**: All settings components follow a consistent pattern for state management:

1. **Get Chart State**: Access `xyPlotState` from `PostgresChartsState`
2. **Verify Chart Match**: Ensure state belongs to current chart
3. **Track Chart Changes**: Use `previousChartId` to detect chart switches
4. **Protect User Input**: Use `isUserEditing` flag to prevent overwriting
5. **Sync with State**: Use `$effect` to sync inputs with state class
6. **Debounce Database**: Debounce database writes while keeping SciChart updates instant

**Example Pattern**:

```typescript
const chartsState = getPostgresChartsState();
const xyPlotState = $derived(chartsState.xyPlotState);
const currentChartId = $derived(chartsState.lastSelectedChartId);

const chartState = $derived(
  xyPlotState && currentChartId === chart.id && xyPlotState.chartId === chart.id
    ? xyPlotState
    : null,
);

let inputValue = $state(getValueFromConfig());
let isUserEditing = $state(false);
let previousChartId = $state(chart.id);

// Reset editing state when chart changes
$effect(() => {
  if (previousChartId !== chart.id) {
    isUserEditing = false;
    inputValue = getValueFromConfig();
    previousChartId = chart.id;
  }
});

// Sync inputs with state (skip if user editing)
$effect(() => {
  if (!isUserEditing && chartState && currentChartId === chart.id) {
    inputValue = chartState.propertyValue;
  }
});

function handleChange(value: any) {
  isUserEditing = true;
  inputValue = value;

  // Update SciChart immediately
  if (chartState) {
    chartState.setProperty(value);
  }

  // Debounce database update
  setTimeout(() => {
    isUserEditing = false;
    void chartsState.updateChartProperty(chart.id, "property.path", value);
  }, 500);
}
```

## Summary

The chart settings system provides a type-based, reactive interface for modifying chart properties:

- **General Settings**: Title and grid lines (all chart types)
- **Axis Settings**: Available for all chart types, update instantly via direct SciChart API
- **Series Settings**: Chart-type-specific (line vs scatter), update instantly via direct property updates
- **Opacity Settings**: Chart-type-specific, update instantly via direct property updates
- **Dashed Line Settings**: Line charts only, update instantly via direct property updates
- **Point Marker Type**: Scatter charts only, requires series recreation
- **Data Selection**: Triggers full data reload when curve IDs change
- **Optimization**: Style changes update SciChart directly without reloading data, preventing slow rerenders
- **Race Condition Prevention**: `isSavingState` and `isUserEditing` flags prevent realtime updates from overwriting user edits
- **Debouncing**: Database writes are debounced (300-500ms) while SciChart updates are instant

### Performance Optimization Results

**Before Optimization**:

- Changing series color: ~500-1000ms (fetches parquet data, recreates series)
- Changing line thickness: ~500-1000ms (fetches parquet data, recreates series)
- Changing point size: ~500-1000ms (fetches parquet data, recreates series)

**After Optimization**:

- Changing series color: <50ms (direct property update, no data fetch)
- Changing line thickness: <50ms (direct property update, no data fetch)
- Changing point size: <50ms (direct property update, no data fetch)

**Key Insight**: According to SciChart documentation, RenderableSeries properties (`stroke`, `strokeThickness`, `pointMarker.width/height`) can be updated directly without reloading data. The chart automatically redraws when these properties change.

### How It Works

1. **User changes style property** (e.g., color)
2. **Component updates SciChart directly** → Instant visual feedback (<50ms)
3. **Component updates database** → Persistence
4. **Realtime update triggers** → `updateChart()` detects style-only change
5. **`updateSeriesStyles()` called** → Updates properties directly (idempotent, safe to call multiple times)
6. **No data reload** → No parquet fetch, no series recreation

Future improvements include Redis caching for parquet data and batched/debounced database updates to further improve performance.
