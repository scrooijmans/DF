# Registering New Chart Types: End-to-End Guide

## Overview

This document describes the complete end-to-end process for registering new chart types in the MudRock application. It follows the same pattern as `add-chart-to-project.md` and uses the `line` and `scatter` chart types as examples.

## Architecture Overview

Registering a new chart type involves multiple layers:

```
┌─────────────────────────────────────────────────────────────┐
│                    Database Layer                            │
│  1. Update chart_types constraint                            │
│  2. Update charts table constraint                           │
│  3. Insert chart type into chart_types table                 │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                    State Management Layer                    │
│  4. Create chart-type-specific state class                  │
│  5. Register state class in PostgresChartsState              │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                    UI Component Layer                        │
│  6. Create SciChart component (if needed)                   │
│  7. Register component in chart-sci-chart.svelte            │
│  8. Update chart-editor-data-options.svelte                  │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                    State Sync Layer                          │
│  9. Update chart-state-sync handlers (if needed)            │
│  10. Update chart-data-source-registry (if needed)           │
└─────────────────────────────────────────────────────────────┘
```

## Step-by-Step Registration Process

### Step 1: Create SQL Migration

**Location**: `db/migrations/018-add-line-scatter-chart-types.sql` (or create new migration)

**Critical Order**: Update constraints BEFORE inserting new chart types!

```sql
-- =====================================================
-- 1. UPDATE CHART_TYPES CONSTRAINT FIRST
-- =====================================================
-- CRITICAL: Update constraint BEFORE inserting new chart types
-- Otherwise, INSERT will fail because constraint doesn't allow new types

ALTER TABLE public.chart_types DROP CONSTRAINT IF EXISTS chart_types_valid_chart_type;

ALTER TABLE public.chart_types ADD CONSTRAINT chart_types_valid_chart_type CHECK (
  chart_type_id IN (
    'xy',              -- Basic XY scatter/line plot
    'line',            -- Line chart (continuous lines) ← NEW
    'scatter',         -- Scatter chart (point markers) ← NEW
    'well_log',        -- 1D depth-based well logs
    'cross_well',      -- Multi-well comparison
    'statistical',     -- Histograms, scatter plots
    'time_series',     -- Time-indexed data
    'custom'           -- User-defined custom charts
  )
);

-- =====================================================
-- 2. UPDATE CHARTS TABLE CONSTRAINT
-- =====================================================

ALTER TABLE public.charts DROP CONSTRAINT IF EXISTS charts_chart_type_check;

ALTER TABLE public.charts ADD CONSTRAINT charts_chart_type_check CHECK (
  chart_type IN (
    'xy', 'line', 'scatter', 'well_log', 'cross_well', 'statistical', 'time_series', 'custom'
  )
);

-- =====================================================
-- 3. INSERT NEW CHART TYPE
-- =====================================================

INSERT INTO public.chart_types (
    chart_type_id,
    display_name,
    description,
    category,
    icon_name,
    default_config,
    default_data_source_config,
    is_active,
    sort_order
) VALUES (
    'line',  -- Internal ID (must match chart_type in charts table)
    'Line Chart',  -- Display name shown in UI
    'Continuous line chart for visualizing trends and relationships between two variables',
    'basic',  -- Category for grouping
    'LineChart',  -- Icon name (must match available icon in UI)
    jsonb_build_object(
        'xAxis', jsonb_build_object(
            'type', 'numeric',
            'title', 'X Axis',
            'axisTitle', 'X Axis'
        ),
        'yAxes', jsonb_build_array(
            jsonb_build_object(
                'id', 'left',
                'type', 'numeric',
                'title', 'Y Axis',
                'axisTitle', 'Y Axis'
            )
        ),
        'series', '[]'::jsonb,
        'legend', jsonb_build_object('visible', true)
    ),
    jsonb_build_object(
        'type', 'node',
        'nodeIds', '[]'::jsonb
    ),
    true,  -- is_active
    2  -- sort_order (adjust existing chart types if needed)
) ON CONFLICT (chart_type_id) DO UPDATE
SET
    display_name = EXCLUDED.display_name,
    description = EXCLUDED.description,
    category = EXCLUDED.category,
    icon_name = EXCLUDED.icon_name,
    default_config = EXCLUDED.default_config,
    default_data_source_config = EXCLUDED.default_data_source_config,
    sort_order = EXCLUDED.sort_order,
    updated_at = NOW();
```

**Key Points**:

- ✅ **Constraint Update Order**: Always update `chart_types_valid_chart_type` constraint BEFORE inserting new chart types
- ✅ **Sort Order**: Check existing `sort_order` values and adjust if needed (e.g., update `well_log` from `2` to `4` to make room)
- ✅ **Default Config**: Provide sensible defaults for `default_config` (axis settings, series array, etc.)
- ✅ **Data Source Config**: Define default `default_data_source_config` (e.g., `{"type": "node", "nodeIds": []}`)

**Execute Migration**:

```bash
# Run migration via Supabase CLI or SQL editor
psql -h localhost -U postgres -d mudrock -f db/migrations/018-add-line-scatter-chart-types.sql
```

---

### Step 2: Create Chart-Type-Specific State Class

**Location**: `src/lib/state/postgres/chart-states/line-plot-state.svelte.ts`

**Pattern**: Extend `XYPlotState` for XY-based charts (line, scatter) or create new base class for other types.

**Example: Line Chart** (minimal - inherits from XYPlotState):

```typescript
/**
 * Line Plot State
 *
 * Minimal implementation of line plot chart extending XYPlotState.
 * This is essentially the same as XYPlotState (which already uses line series).
 * Created for consistency and future extensibility.
 */

import { XYPlotState } from "./xy-plot-state.svelte";

export class LinePlotState extends XYPlotState {
  constructor(chartId: string) {
    super(chartId);
    console.log("[LinePlotState] Created line plot state for chart:", chartId);
  }

  // Future: Add line-specific configuration (e.g., interpolation, line styles)
  // For now, inherits all behavior from XYPlotState
}
```

**Example: Scatter Chart** (overrides `loadSeriesData` to use scatter series):

```typescript
/**
 * Scatter Plot State
 *
 * Scatter plot chart extending XYPlotState.
 * Uses XyScatterRenderableSeries with point markers instead of FastLineRenderableSeries.
 */

import type { Chart } from "../postgres-charts-state.svelte";
import { XYPlotState } from "./xy-plot-state.svelte";
import { fetchAlignedCurveData } from "$lib/services/curve-data-service";
import { SCICHART_SERIES } from "$lib/styles/scichart-constants";
import { getThemeColors } from "$lib/utils/scichart/scichart-theme";

export class ScatterPlotState extends XYPlotState {
  constructor(chartId: string) {
    super(chartId);
    console.log(
      "[ScatterPlotState] Created scatter plot state for chart:",
      chartId,
    );
  }

  /**
   * Override loadSeriesData to use XyScatterRenderableSeries instead of FastLineRenderableSeries
   */
  async loadSeriesData(chart: Chart): Promise<void> {
    // ... implementation using XyScatterRenderableSeries and EllipsePointMarker
    // See src/lib/state/postgres/chart-states/scatter-plot-state.svelte.ts for full implementation
  }
}
```

**Key Points**:

- ✅ **Inheritance**: For XY-based charts, extend `XYPlotState` to reuse axis management, visible range handling, etc.
- ✅ **Override Methods**: Override `loadSeriesData()` if you need different renderable series types (e.g., scatter vs line)
- ✅ **Future Extensibility**: Add chart-type-specific properties/methods as needed

---

### Step 3: Register State Class in PostgresChartsState

**Location**: `src/lib/state/postgres/postgres-charts-state.svelte.ts`

**Changes Required**:

1. **Import new state class**:

```typescript
import { LinePlotState } from "./chart-states/line-plot-state.svelte";
import { ScatterPlotState } from "./chart-states/scatter-plot-state.svelte";
```

2. **Update Chart interface** (if needed):

```typescript
export interface Chart {
  // ... existing fields
  chart_type:
    | "xy"
    | "line" // ← NEW
    | "scatter" // ← NEW
    | "well_log"
    | "cross_well"
    | "statistical"
    | "time_series"
    | "custom";
  // ... rest of fields
}
```

3. **Update `loadChartState()` method**:

```typescript
private loadChartState(chart: Chart): void {
  // Handle XY-based chart types (xy, line, scatter)
  if (
    chart.chart_type === "xy" ||
    chart.chart_type === "line" ||
    chart.chart_type === "scatter"
  ) {
    // If plotState already exists and belongs to this chart, update it in place
    if (this.xyPlotState && this.xyPlotState.chartId === chart.id) {
      this.xyPlotState.updateChart(chart);
      return;
    }

    // Create new state instance based on chart type
    let plotState: XYPlotState | LinePlotState | ScatterPlotState;

    if (chart.chart_type === "line") {
      plotState = new LinePlotState(chart.id);
    } else if (chart.chart_type === "scatter") {
      plotState = new ScatterPlotState(chart.id);
    } else {
      plotState = new XYPlotState(chart.id);
    }

    // Load chart config first (this sets axis labels, visible ranges, etc.)
    plotState.loadFromChartConfig(chart.chart_config);
    plotState.updateChart(chart);
    this.xyPlotState = plotState;
  }
  // ... handle other chart types (well_log, map, etc.)
}
```

4. **Update `setSelectedChartId()` to handle new chart types**:

```typescript
async setSelectedChartId(chartId: string | null): Promise<void> {
  // Save visible range and cleanup previous chart state before switching
  if (this.lastSelectedChartId) {
    let activeChartState: XYPlotState | LinePlotState | ScatterPlotState | null = null;
    const prevChart = this.charts.find((c) => c.id === this.lastSelectedChartId);

    if (prevChart) {
      if (
        (prevChart.chart_type === "xy" ||
          prevChart.chart_type === "line" ||
          prevChart.chart_type === "scatter") &&
        this.xyPlotState?.chartId === prevChart.id
      ) {
        activeChartState = this.xyPlotState;
      }
    }

    if (activeChartState) {
      await activeChartState.saveVisibleRange();
      activeChartState.cleanup();
    }
  }

  // Clear previous chart states
  this.xyPlotState = null;

  // Load new chart state if chartId is provided
  if (chartId) {
    const chart = this.charts.find((c) => c.id === chartId);
    if (chart) {
      this.loadChartState(chart);
    }
  }

  this.lastSelectedChartId = chartId;
}
```

5. **Update `updateChart()` to handle new chart types**:

```typescript
updateChart(updatedChart: Chart): void {
  // ... existing logic

  if (
    this.lastSelectedChartId === updatedChart.id &&
    !this.isSavingState &&
    (updatedChart.chart_type === "xy" ||
      updatedChart.chart_type === "line" ||
      updatedChart.chart_type === "scatter") &&
    this.xyPlotState?.chartId === updatedChart.id &&
    oldConfig !== newConfig
  ) {
    // Reload chart state
    this.loadChartState(updatedChart);
  }
}
```

6. **Update `saveChartStateToDatabase()` to handle new chart types**:

```typescript
async saveChartStateToDatabase(): Promise<void> {
  const chart = this.getSelectedChart();
  if (!chart) return;

  let stateConfig: Record<string, any> | null = null;

  if (
    (chart.chart_type === "xy" ||
      chart.chart_type === "line" ||
      chart.chart_type === "scatter") &&
    this.xyPlotState
  ) {
    stateConfig = this.xyPlotState.toChartConfig();
  }
  // ... handle other chart types

  // Save to database
  // ...
}
```

**Key Points**:

- ✅ **Union Types**: Use union types (`XYPlotState | LinePlotState | ScatterPlotState`) for type safety
- ✅ **Conditional Logic**: Check `chart.chart_type` to determine which state class to instantiate
- ✅ **State Lifecycle**: Ensure proper cleanup and state updates when switching charts

---

### Step 4: Create SciChart Component (if needed)

**Location**: `src/lib/components/pages/home/charts/chart-editor/sci-xy-line-chart/sci-xy-line-chart.svelte`

**Pattern**: For charts that extend `XYPlotState`, the component can be identical to `sci-xy-chart.svelte` (it just uses the state class).

**Example: Line Chart Component**:

```svelte
<script lang="ts">
  import { onDestroy } from 'svelte';
  import type { Chart } from '$lib/state/postgres/postgres-charts-state.svelte';
  import { getPostgresChartsState } from '$lib/state/postgres/postgres-charts-state.svelte';

  interface Props {
    chart: Chart;
  }

  let { chart }: Props = $props();

  const chartsState = getPostgresChartsState();

  let chartContainer: HTMLDivElement | null = $state(null);
  let previousChartId: string | null = $state(null);

  // Get reactive chart state (LinePlotState)
  const linePlotState = $derived(chartsState.xyPlotState);
  const currentChartId = $derived(chartsState.lastSelectedChartId);

  // Verify that linePlotState belongs to the current chart
  const isCurrentChartState = $derived(
    linePlotState &&
      currentChartId === chart.id &&
      linePlotState.chartId === chart.id &&
      chart.chart_type === 'line'
  );

  // Handle chart changes: save previous chart's visible range, then cleanup
  $effect(() => {
    if (previousChartId && previousChartId !== chart.id && linePlotState && linePlotState.chartId === previousChartId) {
      void linePlotState.saveVisibleRange();
      linePlotState.cleanup();
    }
    previousChartId = chart.id;
  });

  // Initialize chart when container is ready and state is available
  $effect(() => {
    if (
      chartContainer &&
      isCurrentChartState &&
      linePlotState &&
      !linePlotState.isInitialized &&
      chart.id
    ) {
      requestAnimationFrame(() => {
        if (chartContainer && linePlotState && !linePlotState.isInitialized) {
          const rect = chartContainer.getBoundingClientRect();
          if (rect.width > 0 && rect.height > 0) {
            void linePlotState.initializeChart(chartContainer, chart);
          } else {
            setTimeout(() => {
              if (chartContainer && linePlotState && !linePlotState.isInitialized) {
                void linePlotState.initializeChart(chartContainer, chart);
              }
            }, 100);
          }
        }
      });
    }
  });

  onDestroy(() => {
    if (linePlotState && linePlotState.chartId === chart.id) {
      void linePlotState.saveVisibleRange();
      linePlotState.cleanup();
    }
  });
</script>

<!-- Container with explicit dimensions -->
<div class="h-full w-full relative" style="width: 100%; height: 100%;">
  <div
    bind:this={chartContainer}
    data-chart-id={chart.id}
    style="width: 100%; height: 100%; position: relative; min-width: 0; min-height: 0;"
  ></div>

  {#if !chartContainer}
    <div class="absolute inset-0 flex items-center justify-center bg-white" style="width: 100%; height: 100%;">
      <div class="text-center">
        <p class="text-gray-500">Loading chart...</p>
        <p class="text-xs text-gray-400 mt-2">{chart.name}</p>
      </div>
    </div>
  {/if}
</div>
```

**Key Points**:

- ✅ **Reuse Pattern**: For charts extending `XYPlotState`, the component is nearly identical to `sci-xy-chart.svelte`
- ✅ **Chart Type Check**: Verify `chart.chart_type === 'line'` to ensure correct state is used
- ✅ **Lifecycle Management**: Handle initialization, cleanup, and visible range saving

---

### Step 5: Register Component in chart-sci-chart.svelte

**Location**: `src/lib/components/pages/home/charts/chart-editor/chart-sci-chart/chart-sci-chart.svelte`

**Changes**:

```svelte
<script lang="ts">
  import type { Chart } from '$lib/state/postgres/postgres-charts-state.svelte';
  import SciXYChart from '../sci-xy-chart/sci-xy-chart.svelte';
  import SciXYLineChart from '../sci-xy-line-chart/sci-xy-line-chart.svelte';  // ← NEW
  import SciXYScatterChart from '../sci-xy-scatter-chart/sci-xy-scatter-chart.svelte';  // ← NEW
  import { STYLE_CONSTANTS } from '$lib/styles/constants';

  interface Props {
    chart: Chart;
  }

  let { chart }: Props = $props();
</script>

<div class="h-full w-full">
  {#if chart.chart_type === 'xy'}
    <SciXYChart {chart} />
  {:else if chart.chart_type === 'line'}  <!-- ← NEW -->
    <SciXYLineChart {chart} />
  {:else if chart.chart_type === 'scatter'}  <!-- ← NEW -->
    <SciXYScatterChart {chart} />
  {:else if chart.chart_type === 'well_log'}
    <div class="h-full w-full flex items-center justify-center">
      <p class="{STYLE_CONSTANTS.FONT_SIZE.DEFAULT} {STYLE_CONSTANTS.COLORS.TEXT.PRIMARY}">
        Well Log chart type not yet implemented
      </p>
    </div>
  {:else}
    <div class="h-full w-full flex items-center justify-center">
      <p class="{STYLE_CONSTANTS.FONT_SIZE.DEFAULT} {STYLE_CONSTANTS.COLORS.TEXT.PRIMARY}">
        Chart type "{chart.chart_type}" not yet implemented
      </p>
    </div>
  {/if}
</div>
```

**Key Points**:

- ✅ **Conditional Rendering**: Use `{#if}` blocks to render the correct component based on `chart.chart_type`
- ✅ **Import Components**: Import all chart-type-specific components at the top

---

### Step 6: Update chart-editor-data-options.svelte

**Location**: `src/lib/components/pages/home/charts/chart-editor/chart-editor-data-options/chart-editor-data-options.svelte`

**Changes**:

```svelte
<!-- Data Source Selection Based on Chart Type -->
{#if chart.chart_type === 'xy' || chart.chart_type === 'line' || chart.chart_type === 'scatter'}
  <!-- XY-based Charts: Show curve selection -->
  <div class="{STYLE_CONSTANTS.SPACING.SPACE_Y.SMALL}">
    <h4 class="{STYLE_CONSTANTS.FONT_SIZE.DEFAULT} font-medium {STYLE_CONSTANTS.COLORS.TEXT.PRIMARY}">
      Select Curves
    </h4>
    <p class="{STYLE_CONSTANTS.FONT_SIZE.DEFAULT} {STYLE_CONSTANTS.COLORS.TEXT.SECONDARY} {STYLE_CONSTANTS.SPACING.MARGIN.MIN}">
      Select curves from your project to display in this {chart.chart_type === 'xy' ? 'XY' : chart.chart_type === 'line' ? 'Line' : 'Scatter'} chart.
    </p>
    {#if currentProjectId}
      <ChartDataCurveOptions
        chart={chart}
        projectId={currentProjectId}
      />
    {:else}
      <p class="{STYLE_CONSTANTS.FONT_SIZE.DEFAULT} {STYLE_CONSTANTS.COLORS.TEXT.MUTED}">
        No project selected
      </p>
    {/if}
  </div>
{/if}
```

**Key Points**:

- ✅ **Conditional Logic**: Add new chart types to the condition (`chart.chart_type === 'line' || chart.chart_type === 'scatter'`)
- ✅ **Dynamic Text**: Update display text to reflect chart type name

---

### Step 7: Update Chart State Sync Handlers (if needed)

**Location**: `src/lib/components/pages/home/charts/services/handlers/xy-chart-state-sync-handler.ts`

**Changes** (if new chart type uses same state sync pattern):

```typescript
export class XYChartStateSyncHandler
  implements ChartStateSyncHandler<XYChartStateSyncParams>
{
  extractState(chart: any, surface: any): XYChartStateSyncParams | null {
    // Handle XY-based chart types
    if (
      chart.chart_type === "xy" ||
      chart.chart_type === "line" || // ← NEW
      chart.chart_type === "scatter" // ← NEW
    ) {
      // Extract visible ranges from SciChart surface
      // ...
    }
    return null;
  }

  // ... other methods also check for line and scatter
}
```

**Key Points**:

- ✅ **Handler Reuse**: If new chart type uses the same state sync pattern (e.g., visible ranges), update existing handlers
- ✅ **New Handlers**: If new chart type needs different state sync (e.g., map center/zoom), create a new handler

---

### Step 8: Update Chart Data Source Registry (if needed)

**Location**: `src/lib/components/pages/home/charts/services/chart-data-source-registry.ts`

**Changes** (if new chart type needs different data sources):

```typescript
export function getAllowedDataSources(chartType: string): DataSourceType[] {
  switch (chartType) {
    case "xy":
    case "line": // ← NEW
    case "scatter": // ← NEW
      return [
        { type: "curve", display_name: "Curve" },
        { type: "well", display_name: "Well" },
      ];
    case "well_log":
      return [{ type: "curve", display_name: "Curve" }];
    // ... other chart types
    default:
      return [];
  }
}
```

**Key Points**:

- ✅ **Data Source Types**: Define which data sources are allowed for the new chart type
- ✅ **Reuse Logic**: If new chart type uses same data sources as existing type, add it to the same case

---

## Complete Registration Checklist

### Database Layer

- [ ] Create SQL migration file
- [ ] Update `chart_types_valid_chart_type` constraint BEFORE inserts
- [ ] Update `charts_chart_type_check` constraint
- [ ] Insert new chart type into `chart_types` table
- [ ] Adjust `sort_order` for existing chart types if needed
- [ ] Execute migration

### State Management Layer

- [ ] Create chart-type-specific state class (e.g., `LinePlotState`, `ScatterPlotState`)
- [ ] Import state class in `postgres-charts-state.svelte.ts`
- [ ] Update `Chart` interface `chart_type` union type
- [ ] Update `loadChartState()` to handle new chart type
- [ ] Update `setSelectedChartId()` to handle new chart type
- [ ] Update `updateChart()` to handle new chart type
- [ ] Update `saveChartStateToDatabase()` to handle new chart type
- [ ] Update `getCurrentChartState()` to return correct state type

### UI Component Layer

- [ ] Create SciChart component (e.g., `sci-xy-line-chart.svelte`)
- [ ] Import component in `chart-sci-chart.svelte`
- [ ] Add conditional rendering in `chart-sci-chart.svelte`
- [ ] Update `chart-editor-data-options.svelte` to handle new chart type
- [ ] Update `chart-editor-axes-settings.svelte` if needed

### State Sync Layer

- [ ] Update `xy-chart-state-sync-handler.ts` (if using same sync pattern)
- [ ] Create new handler (if different sync pattern needed)
- [ ] Update `chart-data-source-registry.ts` (if different data sources)

---

## Example: Line and Scatter Chart Registration

### Files Created/Modified

**Database**:

- `db/migrations/018-add-line-scatter-chart-types.sql`

**State Classes**:

- `src/lib/state/postgres/chart-states/line-plot-state.svelte.ts` (created)
- `src/lib/state/postgres/chart-states/scatter-plot-state.svelte.ts` (created)
- `src/lib/state/postgres/postgres-charts-state.svelte.ts` (modified)

**UI Components**:

- `src/lib/components/pages/home/charts/chart-editor/sci-xy-line-chart/sci-xy-line-chart.svelte` (created)
- `src/lib/components/pages/home/charts/chart-editor/sci-xy-scatter-chart/sci-xy-scatter-chart.svelte` (created)
- `src/lib/components/pages/home/charts/chart-editor/chart-sci-chart/chart-sci-chart.svelte` (modified)
- `src/lib/components/pages/home/charts/chart-editor/chart-editor-data-options/chart-editor-data-options.svelte` (modified)

**Services**:

- `src/lib/components/pages/home/charts/services/handlers/xy-chart-state-sync-handler.ts` (modified)
- `src/lib/components/pages/home/charts/services/chart-data-source-registry.ts` (modified)

---

## Testing Checklist

After registering a new chart type:

1. **Database**:
   - [ ] Verify chart type appears in `chart_types` table
   - [ ] Verify chart type icon appears in `charts-menubar.svelte`
   - [ ] Create a new chart instance via UI
   - [ ] Verify chart appears in `charts-sidebar.svelte`

2. **State Management**:
   - [ ] Verify chart state is created when chart is selected
   - [ ] Verify chart state is cleaned up when switching charts
   - [ ] Verify visible range is saved/loaded correctly

3. **UI Components**:
   - [ ] Verify correct SciChart component renders
   - [ ] Verify axis labels update reactively
   - [ ] Verify data selection works (if applicable)
   - [ ] Verify chart updates when data changes

4. **Realtime**:
   - [ ] Verify new chart appears in sidebar via realtime
   - [ ] Verify chart updates propagate via realtime

---

## Summary

Registering a new chart type requires changes across multiple layers:

1. **Database**: Update constraints and insert chart type definition
2. **State Management**: Create state class and register in `PostgresChartsState`
3. **UI Components**: Create SciChart component and register in `chart-sci-chart.svelte`
4. **Services**: Update state sync handlers and data source registry

The `line` and `scatter` chart types follow this exact pattern, extending `XYPlotState` for shared functionality while allowing chart-type-specific customization (e.g., scatter uses `XyScatterRenderableSeries` instead of `FastLineRenderableSeries`).

**Key Design Principles**:

- ✅ **Modularity**: Each chart type has its own state class and component
- ✅ **Reusability**: Common functionality (axis management, visible ranges) is shared via inheritance
- ✅ **Type Safety**: TypeScript union types ensure correct chart type handling
- ✅ **Consistency**: Follows the same pattern as `add-chart-to-project.md` for chart creation

This architecture ensures that adding new chart types is straightforward and maintainable, while preserving type safety and reactivity throughout the application.
