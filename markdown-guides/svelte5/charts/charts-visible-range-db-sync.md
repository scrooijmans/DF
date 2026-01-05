# Chart Visible Range Database Sync

## Overview

This document describes the end-to-end process of how chart visible ranges (zoom/pan state) are synchronized between the SciChart visualization and the PostgreSQL database. Visible ranges are persisted so that when users switch between charts or reload the application, their zoom/pan state is preserved.

**Note**: Chart state classes have been renamed from `*-plot-state.svelte.ts` to `*-chart-state.svelte.ts` (e.g., `XYPlotState` → `XYChartState`). All chart state classes are exported from `src/lib/state/postgres/chart-states/index.ts` for centralized imports.

## Architecture

The visible range sync system uses a **flexible, extensible registry-based architecture** that supports chart-type-specific state persistence. The system consists of four main components:

1. **Chart Components** (e.g., `sci-xy-chart.svelte`): Extract chart-type-specific state from visualization surfaces
2. **State Sync Registry** (`chart-state-sync-registry.ts`): Maps chart types to their sync handlers, provides unified API
3. **Chart-Type Handlers** (`handlers/` directory): Implement chart-type-specific extraction, saving, loading, and application logic
4. **Sync Service** (`chart-state-sync-service.ts`): Generic database operations with debouncing and immediate save modes
5. **Database** (`charts.chart_config` JSONB column): Stores chart-type-specific state in Builder API format

### Registry-Based Design

The registry pattern enables **extensibility without modification** of existing code:

- **Type Safety**: Each chart type defines its own TypeScript interface for state parameters
- **Separation of Concerns**: Chart-type-specific logic is isolated in handler classes
- **Unified API**: All chart types use the same registry methods (`extractChartState`, `saveChartState`, `loadChartState`, `applyChartState`)
- **Easy Extension**: Adding a new chart type requires only:
  1. Defining sync parameters interface
  2. Implementing handler class
  3. Registering handler at app startup

This architecture ensures that different chart types (XY charts, map charts, well logs, etc.) can persist different state parameters (visible ranges, center/zoom, depth ranges) while using the same underlying infrastructure.

## Database Schema

Visible ranges are stored in the `charts.chart_config` JSONB column using the SciChart Builder API format:

```json
{
  "xAxes": {
    "options": {
      "visibleRange": {
        "min": -4.08,
        "max": 1.28
      }
    }
  },
  "yAxes": [
    {
      "options": {
        "id": "left",
        "visibleRange": {
          "min": -6.74,
          "max": 5.34
        }
      }
    }
  ]
}
```

**Key Design Decision**: The `yAxes` array is **completely replaced** on each save (clean slate approach) to prevent accumulation of old axis entries. Only the current axes on the chart surface are preserved.

## Saving Visible Ranges

### Scenario 1: User Interaction (Debounced)

When a user zooms or pans a chart, visible ranges are saved with a 1-second debounce to prevent excessive database writes.

**Flow:**

1. User interacts with chart (zoom/pan via modifiers)
2. SciChart fires `visibleRangeChanged` events on axes
3. `syncVisibleRanges()` is called with debouncing (1 second delay)
4. If user continues interacting, previous timeout is cleared and a new one is set
5. After 1 second of inactivity, the visible range is saved to the database

**Code Location:** `src/lib/components/pages/home/charts/chart-editor/sci-xy-chart/sci-xy-chart.svelte`

```typescript
// Setup visible range listeners for debounced sync (uses registry)
import {
  extractChartState,
  saveChartState,
} from "../../services/chart-state-sync-registry";

const xAxis = sciChartSurface.xAxes.get(0);
if (xAxis && xAxis.visibleRangeChanged) {
  xAxis.visibleRangeChanged.subscribe(async () => {
    const state = await extractChartState(chart, sciChartSurface);
    if (state) {
      void saveChartState(state, false); // Debounced save
    }
  });
}

sciChartSurface.yAxes.asArray().forEach((yAxis: any) => {
  if (yAxis && yAxis.visibleRangeChanged) {
    yAxis.visibleRangeChanged.subscribe(async () => {
      const state = await extractChartState(chart, sciChartSurface);
      if (state) {
        void saveChartState(state, false); // Debounced save
      }
    });
  }
});
```

**Key Change**: The component now uses the registry (`extractChartState`, `saveChartState`) instead of direct sync service calls. The registry delegates to the appropriate handler (`XYChartStateSyncHandler`) which knows how to extract and save XY chart-specific state.

### Scenario 2: Chart Switch (Immediate)

When a user switches to a different chart, the current chart's visible range must be saved immediately (before the chart is destroyed) to ensure no data loss.

**Flow:**

1. User clicks on a different chart in the sidebar
2. `$effect` detects `chart.id` change
3. `saveCurrentVisibleRange()` is called for the previous chart ID
4. Visible ranges are extracted from the SciChart surface
5. `saveVisibleRangesImmediate()` saves to database without debounce
6. Previous chart is destroyed and new chart is initialized

**Code Location:** `src/lib/components/pages/home/charts/chart-editor/sci-xy-chart/sci-xy-chart.svelte`

```typescript
import {
  extractChartState,
  saveChartState,
  cleanupChartStateSync,
} from "../../services/chart-state-sync-registry";

$effect(() => {
  // Save visible range of previous chart before switching
  if (previousChartId && previousChartId !== chart.id && sciChartSurface) {
    void saveCurrentVisibleRange(previousChartId);
    // Cleanup previous chart
    sciChartSurface.delete();
    cleanupChartStateSync(previousChartId);
    // ... reset state
  }
  previousChartId = chart.id;
});

async function saveCurrentVisibleRange(chartId: string) {
  if (!sciChartSurface) return;

  // Extract state using registry (handles chart-type-specific extraction)
  const state = await extractChartState(chart, sciChartSurface);
  if (!state) return;

  // Save state using registry (immediate save when switching charts)
  await saveChartState(state, true);
}
```

**Key Change**: The component uses the registry to extract and save state. The registry automatically routes to the correct handler (`XYChartStateSyncHandler`) based on the chart's `chart_type`.

## Registry and Handler Implementation

### Registry (`chart-state-sync-registry.ts`)

The registry provides a unified API for all chart types:

- **`extractChartState(chart, chartSurface)`**: Delegates to appropriate handler to extract state
- **`saveChartState(params, immediate)`**: Delegates to appropriate handler to save state (with debouncing support)
- **`loadChartState(chart)`**: Delegates to appropriate handler to load state from database
- **`applyChartState(chartSurface, state)`**: Delegates to appropriate handler to apply state to chart surface

The registry maintains a map of chart types to handlers and routes calls accordingly.

### Chart-Type Handlers (`handlers/xy-chart-state-sync-handler.ts`)

Each chart type implements the `ChartStateSyncHandler` interface:

**`extractState()`**: Extracts chart-type-specific state from the chart surface

- For XY charts: Extracts visible ranges from X and Y axes
- For map charts: Would extract center, zoom, bounds
- For well logs: Would extract depth range, curve ranges

**`saveState()`**: Saves state to database in chart-type-specific format

- For XY charts: Saves to `chart_config.xAxes.options.visibleRange`, `chart_config.yAxes[]`
- Uses `saveChartStateToDatabase()` for generic database operations

**`loadState()`**: Loads state from database

- Parses `chart_config` JSONB to extract chart-type-specific state
- Returns typed state object or null if not found

**`applyState()`**: Applies loaded state to chart surface

- For XY charts: Sets `visibleRange` on axes
- For map charts: Would set center, zoom, bounds

### Generic Sync Service (`chart-state-sync-service.ts`)

Provides low-level database operations:

- **`saveChartStateToDatabase(chartId, stateData, immediate)`**: Generic function to merge state data into `chart_config` JSONB
- Handles debouncing internally (if `immediate = false`)
- Fetches current config, merges state data, saves back to database

**Key Feature:** The sync service is chart-type-agnostic. Chart-type-specific logic is handled by handlers.

### Clean Slate Approach

**Problem:** Previous implementation accumulated Y axis entries over time, creating arrays with dozens of old axis entries.

**Solution:** Replace the entire `yAxes` array on each save:

```typescript
// Replace entire yAxes array with only current axes (clean slate)
config.yAxes = yAxesRanges.map((range) => ({
  options: {
    id: range.id,
    visibleRange: { min: range.min, max: range.max },
  },
}));
```

This ensures:

- Only current axes are preserved
- No accumulation of old entries
- Clean, predictable data structure

## Loading Visible Ranges

When a chart is initialized, visible ranges are loaded from the database and applied to the SciChart axes.

**Flow:**

1. Chart component receives `chart` prop with `chart_config`
2. Extract `xAxes.options.visibleRange` from config
3. Find first Y axis in `yAxes` array with a `visibleRange`
4. Create `NumberRange` objects for X and Y axes
5. Default to `(-10, 10)` if no saved range exists
6. Pass ranges to `NumericAxis` constructor

**Code Location:** `src/lib/components/pages/home/charts/chart-editor/sci-xy-chart/sci-xy-chart.svelte`

```typescript
import {
  loadChartState,
  applyChartState,
} from "../../services/chart-state-sync-registry";

// Load chart state using registry (handles chart-type-specific loading)
const loadedState = loadChartState(chart);

// Determine visible ranges: use loaded state or default to -10 to +10
let xVisibleRange: any;
let yVisibleRange: any;

if (loadedState && loadedState.chartType === "xy") {
  const xyState = loadedState as XYChartStateSyncParams;
  xVisibleRange = new NumberRange(
    xyState.xAxisRange?.min ?? -10,
    xyState.xAxisRange?.max ?? 10,
  );
  const firstYRange = xyState.yAxesRanges?.[0];
  yVisibleRange = new NumberRange(
    firstYRange?.min ?? -10,
    firstYRange?.max ?? 10,
  );
} else {
  // Fallback to defaults if no state loaded
  xVisibleRange = new NumberRange(-10, 10);
  yVisibleRange = new NumberRange(-10, 10);
}

// Create axes with visible ranges
const xAxis = new NumericAxis(wasmContext, {
  visibleRange: xVisibleRange,
  // ... other config
});

// Apply loaded state to chart surface (if available)
if (loadedState && sciChartSurface) {
  applyChartState(sciChartSurface, loadedState);
}
```

**Key Change**: The component uses `loadChartState()` from the registry, which delegates to `XYChartStateSyncHandler.loadState()`. The handler knows how to parse XY chart-specific state from `chart_config`.

## Component Integration

### Chart Selection Flow

1. User clicks chart in sidebar → `chart-sidebar-item.svelte`
2. `chartsState.setSelectedChartId(chartId)` updates global state
3. `chart-editor.svelte` reactively gets selected chart via `chartsState.getSelectedChart()`
4. `sci-xy-chart.svelte` receives new `chart` prop
5. `$effect` detects `chart.id` change
6. Previous chart's visible range is saved immediately
7. Previous chart is destroyed
8. New chart is initialized with visible range from database

### State Management

- **Global State:** `PostgresChartsState` manages `lastSelectedChartId`
- **Component State:** `sci-xy-chart.svelte` tracks `previousChartId` to detect switches
- **Sync State:** `chart-state-sync-service.ts` maintains timeout map for debouncing

## Data Flow Diagram

```
User Interaction (Zoom/Pan)
    ↓
SciChart visibleRangeChanged Event
    ↓
syncVisibleRanges() [Debounced 1s]
    ↓
[Wait 1s for inactivity]
    ↓
Fetch chart_config from DB
    ↓
Update xAxes.options.visibleRange
Replace yAxes array with current axes
    ↓
Save to charts.chart_config JSONB
```

```
User Clicks Different Chart
    ↓
chart.id changes in $effect
    ↓
saveCurrentVisibleRange(previousChartId)
    ↓
Extract ranges from SciChart surface
    ↓
saveVisibleRangesImmediate() [No debounce]
    ↓
Fetch chart_config from DB
    ↓
Update xAxes.options.visibleRange
Replace yAxes array with current axes
    ↓
Save to charts.chart_config JSONB
    ↓
Destroy previous chart
Initialize new chart with saved ranges
```

## Key Design Decisions

### 1. Clean Slate for Y Axes

**Decision:** Replace entire `yAxes` array on each save instead of updating individual entries.

**Rationale:** Prevents accumulation of old axis entries that can occur when axes are dynamically added/removed or when axis IDs change.

### 2. Debounced vs Immediate Saves

**Decision:** Use debounced saves during interaction, immediate saves on chart switch.

**Rationale:**

- Debouncing reduces database load during rapid zoom/pan
- Immediate saves ensure no data loss when switching charts

### 3. Default Visible Range

**Decision:** Default to `(-10, 10)` if no saved range exists.

**Rationale:** Provides a predictable starting point for empty charts or charts without saved state.

### 4. Builder API Format

**Decision:** Store visible ranges in SciChart Builder API format (`xAxes.options.visibleRange`, `yAxes[].options.visibleRange`).

**Rationale:** Consistent with SciChart's JSON-based configuration system and allows for future Builder API integration.

## Error Handling

- **Failed Fetch:** Logs error and returns early (doesn't crash component)
- **Failed Save:** Logs error and returns `Result` with error (allows caller to handle)
- **Missing Axes:** Defaults to `(-10, 10)` range
- **Null Surface:** Skips save if chart surface doesn't exist

## Debugging

The implementation includes extensive console logging:

- `[SciXYChart] Chart ID changed:` - Tracks chart switches
- `[SciXYChart] Saving visible range for chart:` - Logs ranges being saved
- `[SciXYChart] Loading chart config:` - Logs config structure on load
- `[SciXYChart] Loaded visible ranges:` - Logs extracted ranges
- `[ChartStateSync] Current chart config before update:` - Logs config before modification
- `[ChartStateSync] Updated chart config:` - Logs config after modification
- `[ChartStateSync] Saved visible ranges immediately for chart:` - Confirms successful save

## Troubleshooting

### Visible Ranges Not Saving

1. Check console for `[ChartStateSync]` logs
2. Verify chart has a valid `id`
3. Ensure `sciChartSurface` exists before saving
4. Check database permissions (RLS policies)

### Visible Ranges Not Loading

1. Check console for `[SciXYChart] Loading chart config:` logs
2. Verify `chart_config` structure matches expected format
3. Check if `xAxes.options.visibleRange` exists
4. Verify Y axis has `options.visibleRange` (not just at root level)

### Accumulation of Y Axis Entries

1. Ensure `saveVisibleRangesImmediate()` uses clean slate approach
2. Verify `yAxes` array is replaced, not updated
3. Check for multiple save calls without cleanup

### Default Range Always Applied

1. Check if `chart_config` is empty or malformed
2. Verify visible range extraction logic
3. Check console logs for config structure

## Related Files

- **Component:** `src/lib/components/pages/home/charts/chart-editor/sci-xy-chart/sci-xy-chart.svelte`
- **Registry:** `src/lib/components/pages/home/charts/services/chart-state-sync-registry.ts`
- **XY Chart Handler:** `src/lib/components/pages/home/charts/services/handlers/xy-chart-state-sync-handler.ts`
- **Type Definitions:** `src/lib/components/pages/home/charts/types/chart-state-sync.ts`
- **Sync Service:** `src/lib/components/pages/home/charts/services/chart-state-sync-service.ts`
- **Initialization:** `src/lib/components/pages/home/charts/services/chart-state-sync-init.ts`
- **State Management:** `src/lib/state/postgres/postgres-charts-state.svelte.ts`
- **Database Schema:** `db/migrations/013-charts-schema.sql`
- **Builder API Support:** `db/migrations/016-enhance-charts-builder-api-support.sql`
