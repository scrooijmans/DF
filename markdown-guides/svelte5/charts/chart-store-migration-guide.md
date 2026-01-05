# ChartStore Migration Guide

## Overview

This guide documents the migration from the existing `PostgresChartsState` + chart-state-classes architecture to the new CRDT-based `ChartStore` architecture.

## Architecture Comparison

### Old Architecture (PostgresChartsState)
```
PostgresChartsState (postgres-charts-state.svelte.ts)
├── charts: Chart[]                    # Chart metadata
├── lastSelectedChartId: string        # Selection state
├── xyPlotState: XYChartState          # SciChart instances
├── wellCorrelationState               # Type-specific state
├── mapState                           # Type-specific state
└── Chart-state classes (chart-states/)
    ├── XYChartState                   # Manages SciChart surface
    ├── LineChartState extends XY      # Line-specific
    ├── ScatterChartState extends XY   # Scatter-specific  
    ├── MapChartState                  # Map-specific
    └── WellCorrelationChartState      # Well correlation
```

**Pros**: Tightly integrated with SciChart lifecycle
**Cons**: No CRDT support, realtime sync via Supabase channels

### New Architecture (ChartStore)
```
ChartStore (chart-store.svelte.ts)
├── _charts: Map<string, ChartSpec>    # ChartSpec discriminated union
├── _selectedChartId: string           # Selection state
├── Type-safe update methods           # Per chart type
└── Tauri IPC for CRDT sync            # Backend handles Automerge
```

**Pros**: CRDT-backed, offline-first, type-safe ChartSpec
**Cons**: Doesn't manage SciChart lifecycle (by design)

## Migration Strategy: Hybrid Bridge

Rather than replacing the old system entirely, we use a **bridge pattern** where:
1. `ChartStore` manages the **specification** (what the chart should be)
2. `chart-states/*` manage the **rendering** (SciChart instances)
3. `ChartStoreProvider` bridges the two systems

```
User Action → ChartStore.update*() → Tauri IPC → Backend CRDT
                     ↓
              ChartStoreProvider (bridge)
                     ↓
         PostgresChartsState.loadChartState()
                     ↓
            chart-states/* → SciChart
```

## Step-by-Step Migration

### Phase 1: Type Unification ✅ (Complete)
- Created `ChartSpec` discriminated union types
- Created `ChartModifiers`, `TrackLegend` types
- All in `src/lib/charts/types/`

### Phase 2: ChartStore Integration ✅ (Complete)
- Created `ChartStore` class with CRDT sync via Tauri
- Created `ChartStoreProvider` bridge component
- Removed direct Automerge import (WASM runs in Rust backend)

### Phase 3: Settings Panel Migration 🔄 (In Progress)
New settings panels that use ChartStore:
- `ChartSettingsPanel.svelte` - Main panel with chart type switching
- `ScatterChartSettings.svelte` - Scatter/line settings
- `WellCorrelationSettings.svelte` - Well correlation settings
- `MapChartSettings.svelte` - Map settings
- `ModifierSettings.svelte` - Modifier controls
- `DepthRulerSettings.svelte` - Depth ruler (well correlation)

### Phase 4: SciChart Integration (Next)
Connect ChartStore updates to existing SciChart rendering:

```typescript
// In ChartStoreProvider or new bridge service
$effect(() => {
  const spec = chartStore.selectedChart;
  if (!spec) return;
  
  const chart = postgresChartsState.getSelectedChart();
  if (!chart) return;
  
  // Sync spec changes to chart-state classes
  if (isScatterChart(spec)) {
    const state = chartsState.xyPlotState;
    if (state) {
      state.setXAxisLabel(spec.config.x.axis.label);
      state.setYAxisLabel(spec.config.y.axis.label);
      // ... more properties
    }
  }
});
```

### Phase 5: Gradual Component Migration
Migrate components one-by-one from using `getPostgresChartsState()` to `getChartStoreContext()`:

```typescript
// Before (old)
import { getPostgresChartsState } from '$lib/state/postgres/postgres-charts-state.svelte';
const chartsState = getPostgresChartsState();
const chart = chartsState.getSelectedChart();

// After (new)
import { getChartStoreContext } from '$lib/charts/stores/chart-store.svelte';
const chartStore = getChartStoreContext();
const chart = chartStore.selectedChart;
```

### Phase 6: Backend Integration
Rust Tauri commands for CRDT operations:
- `get_chart_spec` - Load single chart spec
- `get_project_charts` - Load all charts for project
- `create_chart_spec` - Create new chart
- `update_chart_spec` - Update chart (CRDT merge)
- `delete_chart_spec` - Delete chart

## File Structure

```
src/lib/charts/
├── index.ts                    # Public API exports
├── stores/
│   └── chart-store.svelte.ts   # ChartStore class
├── types/
│   ├── chart-spec.ts           # ChartSpec discriminated union
│   ├── chart-modifiers.ts      # Modifier types & registry
│   ├── track-legend.ts         # Legend types (well correlation)
│   └── index.ts                # Type exports
├── components/
│   ├── ChartStoreProvider.svelte  # Bridge component
│   └── settings/
│       ├── ChartSettingsPanel.svelte
│       ├── ScatterChartSettings.svelte
│       ├── WellCorrelationSettings.svelte
│       ├── MapChartSettings.svelte
│       ├── ModifierSettings.svelte
│       ├── DepthRulerSettings.svelte
│       ├── common/
│       │   ├── AxisSettings.svelte
│       │   ├── ColorPicker.svelte
│       │   ├── RangeInput.svelte
│       │   ├── SelectInput.svelte
│       │   └── MultiSelect.svelte
│       └── index.ts
└── services/                   # Future: services for SciChart bridge
    └── chart-spec-sync-service.ts
```

## Usage Examples

### Using ChartStore in Components

```svelte
<script lang="ts">
  import { getChartStoreContext } from '$lib/charts';
  
  const chartStore = getChartStoreContext();
  
  // Read state (reactive)
  const selectedChart = $derived(chartStore.selectedChart);
  const isLoading = $derived(chartStore.isLoading);
  
  // Update state
  function handleAxisLabelChange(label: string) {
    if (!selectedChart) return;
    chartStore.updateXAxisConfig(selectedChart.id, { label });
  }
</script>
```

### Type-Safe Chart Type Checking

```typescript
import { isScatterChart, isWellCorrelationChart, isMapChart } from '$lib/charts';

if (isScatterChart(chart)) {
  // TypeScript knows chart.config has scatter properties
  console.log(chart.config.x.field);
} else if (isWellCorrelationChart(chart)) {
  // TypeScript knows chart.config has well correlation properties
  console.log(chart.config.depth.unit);
}
```

## Keeping Both Systems

During migration, both systems coexist:
- **Use PostgresChartsState** for: SciChart lifecycle, realtime sync, existing components
- **Use ChartStore** for: New settings panels, CRDT sync, new features

The `ChartStoreProvider` handles synchronization between them.

## When Migration is Complete

After all components are migrated:
1. `PostgresChartsState` will only manage chart loading from DB
2. `chart-states/*` will be simplified to only manage SciChart surfaces
3. `ChartStore` will be the primary state source
4. CRDT sync will happen via Tauri IPC to Rust backend

