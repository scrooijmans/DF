# SciChart Svelte State Reactivity Flow

## Overview

This document describes the complete end-to-end reactivity flow for SciChart components in Svelte, focusing on how user interactions in UI components trigger reactive updates to SciChart instances and database synchronization. The architecture uses a **state class pattern** where chart state is managed centrally, enabling immediate reactivity without component re-renders.

## Architecture Pattern

The reactivity system follows a **state class pattern** where:

1. **State Class** (e.g., `XYPlotState`) manages the SciChart instance and reactive properties
2. **Chart Component** (e.g., `sci-xy-chart.svelte`) provides the DOM container and delegates to the state class
3. **Settings Components** (e.g., `chart-editor-axes-settings.svelte`) update state, which directly updates SciChart
4. **Data Selection Components** (e.g., `chart-data-curve-options.svelte`) update database, triggering realtime updates

## Complete Reactivity Flow

### Flow 1: Axis Label Reactivity

```
User types in chart-editor-axes-settings.svelte input
    ↓
handleXAxisTitleChange("New Label")
    ↓
chartState.setXAxisLabel("New Label")
    ↓
XYPlotState.setXAxisLabel()
    ├─ Updates reactive state: this.xAxisLabel = "New Label"
    └─ Directly updates SciChart: this.xAxis.axisTitle = "New Label"
    ↓
Chart updates immediately (no re-render needed)
    ↓
Debounced save to database (500ms)
    ↓
PostgresChartsState.saveChartStateToDatabase()
    ↓
Supabase UPDATE charts SET chart_config = {...}
    ↓
Realtime update triggers PostgresChartsState.updateChart()
    ↓
State reloads (but user input is protected by isUserEditing flag)
```

**Key Components:**

1. **`chart-editor-axes-settings.svelte`**:
   ```svelte
   function handleXAxisTitleChange(value: string) {
     isUserEditing = true; // Prevent external updates
     xAxisTitle = value;
     
     // CRITICAL: Directly update SciChart via state class
     if (chartState) {
       chartState.setXAxisLabel(value); // Immediate chart update
     }
     
     void saveAxisSettings(); // Debounced DB save
   }
   ```

2. **`xy-plot-state.svelte.ts`**:
   ```typescript
   setXAxisLabel(label: string): void {
     this.xAxisLabel = label; // Update reactive state
     
     // Directly update SciChart axis if initialized (reactive update)
     if (this.xAxis && this.isInitialized) {
       this.xAxis.axisTitle = label; // Immediate UI update
     }
   }
   ```

3. **Database Sync**:
   ```typescript
   // Debounced save (500ms)
   async function saveAxisSettings() {
     await chartsState.saveChartStateToDatabase();
   }
   
   // PostgresChartsState.saveChartStateToDatabase()
   // Updates chart_config.xAxis.axisTitle in database
   ```

**Benefits:**
- ✅ **Immediate UI feedback**: Chart updates instantly as user types
- ✅ **No re-renders**: Direct SciChart API updates, no component re-render
- ✅ **Database sync**: Changes persist to database automatically
- ✅ **User input protection**: `isUserEditing` flag prevents overwriting active input

---

### Flow 2: Chart Series Data Reactivity

```
User selects X/Y axis curves in chart-data-curve-options.svelte
    ↓
handleCurveSelection(curveId, 'x' | 'y')
    ↓
Updates chart.chart_config.series[0].xCurveId or yCurveId
    ↓
Supabase UPDATE charts SET chart_config = {...}
    ↓
PostgreSQL WAL updated
    ↓
Supabase Realtime detects UPDATE
    ↓
RealtimeChartsService.handleChartUpdate()
    ↓
PostgresChartsState.updateChart(updatedChart)
    ├─ Detects series change (oldSeries !== newSeries)
    └─ Calls loadChartState(updatedChart)
    ↓
XYPlotState.updateChart(chart)
    ├─ Updates internal chart reference
    ├─ loadFromChartConfig(chart.chart_config)
    └─ loadSeriesData(chart) [if initialized]
    ↓
fetchAlignedCurveData(xCurveId, yCurveId, projectId)
    ├─ Fetches X curve data from parquet file
    ├─ Fetches Y curve data from parquet file
    └─ Aligns data by index (nth point of X vs nth point of Y)
    ↓
XYPlotState.loadSeriesData()
    ├─ Clears existing renderableSeries
    ├─ Creates new XyDataSeries with aligned data
    ├─ Creates FastLineRenderableSeries
    ├─ Adds series to sciChartSurface
    └─ Calls zoomExtents() to fit data
    ↓
Chart displays new data reactively
```

**Key Components:**

1. **`chart-data-curve-options.svelte`**:
   ```svelte
   async function handleCurveSelection(curveId: string, axis: 'x' | 'y') {
     // Update local state immediately
     if (axis === 'x') {
       xCurveId = curveId;
     } else {
       yCurveId = curveId;
     }
     
     // Update database
     const updatedChartConfig = {
       ...chartConfig,
       series: [{
         ...currentSeries,
         [axis === 'x' ? 'xCurveId' : 'yCurveId']: curveId,
       }],
     };
     
     await supabase
       .from('charts')
       .update({ chart_config: updatedChartConfig })
       .eq('id', chartId);
   }
   ```

2. **`postgres-charts-state.svelte.ts`**:
   ```typescript
   updateChart(updatedChart: Chart): void {
     const oldChart = this.charts[index];
     const oldSeries = JSON.stringify(oldChart.chart_config?.series || []);
     const newSeries = JSON.stringify(updatedChart.chart_config?.series || []);
     const seriesChanged = oldSeries !== newSeries;
     
     // Update chart in array
     this.charts[index] = updatedChart;
     
     // Reload state if series changed
     if (seriesChanged && this.lastSelectedChartId === updatedChart.id) {
       this.loadChartState(updatedChart);
     }
   }
   ```

3. **`xy-plot-state.svelte.ts`**:
   ```typescript
   async loadSeriesData(chart: Chart): Promise<void> {
     const seriesConfig = chart.chart_config?.series || [];
     
     // Clear existing series
     this.sciChartSurface.renderableSeries.asArray().forEach((rs) => {
       rs.dataSeries?.delete();
       this.sciChartSurface.renderableSeries.remove(rs);
     });
     
     // Load data for each series
     for (const series of seriesConfig) {
       if (series.xCurveId && series.yCurveId) {
         // Fetch aligned curve data
         const alignedData = await fetchAlignedCurveData(
           series.xCurveId,
           series.yCurveId,
           chart.project_id,
         );
         
         // Create data series
         const dataSeries = new XyDataSeries(this.wasmContext, {
           dataSeriesName: series.name,
         });
         dataSeries.appendRange(alignedData.xValues, alignedData.yValues);
         
         // Create renderable series
         const lineSeries = new FastLineRenderableSeries(this.wasmContext, {
           dataSeries: dataSeries,
           stroke: series.stroke || themeColors.lineSeriesColor,
         });
         
         this.sciChartSurface.renderableSeries.add(lineSeries);
       }
     }
     
     // Fit chart to data
     if (hasData) {
       this.sciChartSurface.zoomExtents();
     }
   }
   ```

4. **`curve-data-service.ts`**:
   ```typescript
   export async function fetchAlignedCurveData(
     xCurveId: string,
     yCurveId: string,
     projectId: string,
   ): Promise<AlignedCurveData | null> {
     // Fetch both curves in parallel
     const [xCurveData, yCurveData] = await Promise.all([
       fetchCurveData(xCurveId, projectId),
       fetchCurveData(yCurveId, projectId),
     ]);
     
     // Align by index (nth point of X vs nth point of Y)
     return alignCurveData(xCurveData, yCurveData);
   }
   
   export function alignCurveData(
     xCurveData: CurveData,
     yCurveData: CurveData,
   ): AlignedCurveData | null {
     // Create maps: index -> value
     const xMap = new Map<number, number>();
     const yMap = new Map<number, number>();
     
     for (const point of xCurveData.data) {
       xMap.set(point.index, point.value);
     }
     for (const point of yCurveData.data) {
       yMap.set(point.index, point.value);
     }
     
     // Find overlapping indices
     const overlappingIndices: number[] = [];
     for (const index of xMap.keys()) {
       if (yMap.has(index)) {
         overlappingIndices.push(index);
       }
     }
     
     // Extract aligned values
     const xValues: number[] = [];
     const yValues: number[] = [];
     for (const index of overlappingIndices.sort((a, b) => a - b)) {
       xValues.push(xMap.get(index)!);
       yValues.push(yMap.get(index)!);
     }
     
     return { xValues, yValues };
   }
   ```

**Benefits:**
- ✅ **Automatic updates**: Chart updates when curve selections change
- ✅ **Data alignment**: Curves aligned by index (no DEPT column required)
- ✅ **Realtime sync**: Database changes trigger automatic chart updates
- ✅ **Performance**: Only fetches data when series configuration changes

---

## State Class Pattern

### Why State Classes?

**Problem**: SciChart instances need to be managed across component lifecycle, with reactive updates to axes, series, and visible ranges.

**Solution**: Centralize SciChart management in a state class (`XYPlotState`) that:

1. **Manages SciChart lifecycle**: Initialization, updates, cleanup
2. **Provides reactive properties**: `$state` runes for reactive values
3. **Direct API updates**: Methods directly update SciChart axes/series
4. **Database sync**: Loads from and saves to `chart_config` JSONB

### State Class Structure

```typescript
export class XYPlotState {
  // Reactive state properties
  xAxisLabel = $state<string>("X Axis");
  yAxisLabel = $state<string>("Y Axis");
  xVisibleRange = $state<{ min: number; max: number } | null>(null);
  yVisibleRange = $state<{ min: number; max: number } | null>(null);
  isInitialized = $state(false);
  
  // SciChart instance (managed here)
  sciChartSurface: any = null;
  wasmContext: any = null;
  xAxis: any = null;
  yAxis: any = null;
  
  // Chart reference
  private chart: Chart | null = null;
  
  // CRITICAL: Directly update SciChart when state changes
  setXAxisLabel(label: string): void {
    this.xAxisLabel = label; // Update reactive state
    if (this.xAxis && this.isInitialized) {
      this.xAxis.axisTitle = label; // Direct SciChart update
    }
  }
  
  // Initialize chart
  async initializeChart(container: HTMLDivElement, chart: Chart): Promise<void> {
    // ... initialization logic ...
    this.isInitialized = true;
  }
  
  // Load series data reactively
  async loadSeriesData(chart: Chart): Promise<void> {
    // ... fetch data and update series ...
  }
  
  // Cleanup
  cleanup(): void {
    if (this.sciChartSurface) {
      this.sciChartSurface.delete();
    }
  }
}
```

---

## Component Integration

### Chart Component (`sci-xy-chart.svelte`)

**Role**: Thin wrapper providing DOM container

```svelte
<script lang="ts">
  const chartsState = getPostgresChartsState();
  const xyPlotState = $derived(chartsState.xyPlotState);
  const currentChartId = $derived(chartsState.lastSelectedChartId);
  
  // Verify state belongs to current chart
  const isCurrentChartState = $derived(
    xyPlotState && currentChartId === chart.id && xyPlotState.chartId === chart.id
  );
  
  // Initialize chart when container ready
  $effect(() => {
    if (chartContainer && isCurrentChartState && xyPlotState && !xyPlotState.isInitialized) {
      void xyPlotState.initializeChart(chartContainer, chart);
    }
  });
  
  // Save visible range before switching charts
  $effect(() => {
    if (previousChartId && previousChartId !== chart.id && xyPlotState) {
      void xyPlotState.saveVisibleRange();
      xyPlotState.cleanup();
    }
    previousChartId = chart.id;
  });
</script>

<div bind:this={chartContainer}></div>
```

**Key Points:**
- ✅ Delegates all SciChart logic to state class
- ✅ Handles chart switching (save previous, cleanup, initialize new)
- ✅ Waits for container dimensions before initializing

---

### Settings Component (`chart-editor-axes-settings.svelte`)

**Role**: UI for editing axis labels with reactive updates

```svelte
<script lang="ts">
  const chartsState = getPostgresChartsState();
  const xyPlotState = $derived(chartsState.xyPlotState);
  const chartState = $derived(
    xyPlotState && currentChartId === chart.id ? xyPlotState : null
  );
  
  let xAxisTitle = $state(getAxisTitleFromConfig('x'));
  let isUserEditing = $state(false);
  let previousChartId = $state(chart.id);
  
  // Reset inputs when chart changes
  $effect(() => {
    if (previousChartId !== chart.id) {
      isUserEditing = false;
      xAxisTitle = getAxisTitleFromConfig('x');
      previousChartId = chart.id;
    }
  });
  
  // Sync inputs with state (skip if user editing)
  $effect(() => {
    if (!isUserEditing && chartState) {
      xAxisTitle = chartState.xAxisLabel;
    }
  });
  
  function handleXAxisTitleChange(value: string) {
    isUserEditing = true;
    xAxisTitle = value;
    
    // CRITICAL: Update state immediately (chart updates reactively)
    if (chartState) {
      chartState.setXAxisLabel(value);
    }
    
    // Debounced DB save
    setTimeout(() => {
      isUserEditing = false;
      void chartsState.saveChartStateToDatabase();
    }, 1000);
  }
</script>

<Input bind:value={xAxisTitle} oninput={(e) => handleXAxisTitleChange(e.currentTarget.value)} />
```

**Key Points:**
- ✅ Uses local `$state` for inputs (prevents external overwrites)
- ✅ `isUserEditing` flag protects active input
- ✅ Directly calls state class methods for immediate updates
- ✅ Debounced database save (prevents excessive writes)

---

### Data Selection Component (`chart-data-curve-options.svelte`)

**Role**: UI for selecting X/Y axis curves

```svelte
<script lang="ts">
  async function handleCurveSelection(curveId: string, axis: 'x' | 'y') {
    // Update local state immediately
    if (axis === 'x') {
      xCurveId = curveId;
    } else {
      yCurveId = curveId;
    }
    
    // Update database
    const updatedChartConfig = {
      ...chartConfig,
      series: [{
        ...currentSeries,
        [axis === 'x' ? 'xCurveId' : 'yCurveId']: curveId,
      }],
    };
    
    await supabase
      .from('charts')
      .update({ chart_config: updatedChartConfig })
      .eq('id', chartId);
    
    // Realtime update will trigger chart reload automatically
  }
</script>

<select
  value={xCurveId || ''}
  onchange={(e) => handleCurveSelection(e.currentTarget.value, 'x')}
>
  {#each curveOptions as curve}
    <option value={curve.id}>{curve.well_name} - {curve.curve_name}</option>
  {/each}
</select>
```

**Key Points:**
- ✅ Updates database directly (triggers realtime update)
- ✅ Realtime service handles chart reload automatically
- ✅ No direct SciChart manipulation (delegated to state class)

---

## Database Synchronization

### Saving State to Database

**Flow**: Settings changes → State class → Database

```typescript
// PostgresChartsState.saveChartStateToDatabase()
async saveChartStateToDatabase(): Promise<void> {
  const chart = this.getSelectedChart();
  if (!chart) return;
  
  // Get state config from state class
  let stateConfig: Record<string, any> | null = null;
  if (chart.chart_type === 'xy' && this.xyPlotState) {
    stateConfig = this.xyPlotState.toChartConfig();
  }
  
  // Merge with existing config
  const updatedConfig = {
    ...currentConfig,
    ...stateConfig,
    xAxis: { ...currentConfig.xAxis, ...stateConfig.xAxis },
    yAxes: [/* updated yAxes */],
  };
  
  // Update database
  await supabase
    .from('charts')
    .update({ chart_config: updatedConfig })
    .eq('id', chart.id);
}
```

**Storage Format** (`chart_config` JSONB):

```json
{
  "xAxis": {
    "axisTitle": "X Axis",
    "visibleRange": { "min": -10, "max": 10 }
  },
  "yAxes": [
    {
      "options": {
        "id": "left",
        "axisTitle": "Y Axis",
        "visibleRange": { "min": -10, "max": 10 }
      }
    }
  ],
  "series": [
    {
      "id": "series-1",
      "xCurveId": "curve-uuid-1",
      "yCurveId": "curve-uuid-2",
      "wellId": "well-uuid",
      "name": "GR vs RHOB"
    }
  ]
}
```

---

### Loading State from Database

**Flow**: Database → Realtime update → State class → SciChart

```typescript
// PostgresChartsState.updateChart()
updateChart(updatedChart: Chart): void {
  const oldChart = this.charts[index];
  const oldSeries = JSON.stringify(oldChart.chart_config?.series || []);
  const newSeries = JSON.stringify(updatedChart.chart_config?.series || []);
  const seriesChanged = oldSeries !== newSeries;
  
  // Update chart in array
  this.charts[index] = updatedChart;
  
  // Reload state if config changed
  if (oldConfig !== newConfig && this.lastSelectedChartId === updatedChart.id) {
    this.loadChartState(updatedChart);
  }
}

// XYPlotState.updateChart()
updateChart(chart: Chart): void {
  this.chart = chart;
  this.loadFromChartConfig(chart.chart_config);
  
  // CRITICAL: Reload series data if chart is initialized
  if (this.isInitialized) {
    void this.loadSeriesData(chart);
  }
}
```

---

## Key Design Decisions

### 1. State Class Pattern

**Why**: Centralizes SciChart management, enables direct API updates, provides reactive interface.

**Benefits**:
- ✅ Single source of truth for chart state
- ✅ Direct SciChart API updates (no re-renders)
- ✅ Reactive properties via `$state` runes
- ✅ Clean lifecycle management

### 2. Direct SciChart Updates

**Why**: Immediate UI feedback without component re-renders.

**Example**:
```typescript
setXAxisLabel(label: string): void {
  this.xAxisLabel = label; // Reactive state
  if (this.xAxis && this.isInitialized) {
    this.xAxis.axisTitle = label; // Direct SciChart update
  }
}
```

**Benefits**:
- ✅ Instant UI updates
- ✅ No component re-renders
- ✅ Better performance

### 3. User Input Protection

**Why**: Prevent external updates (realtime) from overwriting active user input.

**Implementation**:
```typescript
let isUserEditing = $state(false);

$effect(() => {
  // Skip sync if user is actively editing
  if (isUserEditing) return;
  
  // Sync with state
  if (chartState) {
    xAxisTitle = chartState.xAxisLabel;
  }
});

function handleInputChange(value: string) {
  isUserEditing = true;
  // Update state immediately
  chartState.setXAxisLabel(value);
  
  // Reset flag after debounce
  setTimeout(() => { isUserEditing = false; }, 1000);
}
```

### 4. Index-Based Curve Alignment

**Why**: DEPT column may be missing; curves from same parquet file have same row count.

**Implementation**:
```typescript
// Align by index (nth point of X vs nth point of Y)
export function alignCurveData(
  xCurveData: CurveData,
  yCurveData: CurveData,
): AlignedCurveData | null {
  const xMap = new Map<number, number>(); // index -> value
  const yMap = new Map<number, number>();
  
  // Find overlapping indices
  const overlappingIndices = [];
  for (const index of xMap.keys()) {
    if (yMap.has(index)) {
      overlappingIndices.push(index);
    }
  }
  
  // Extract aligned values
  return {
    xValues: overlappingIndices.map(i => xMap.get(i)!),
    yValues: overlappingIndices.map(i => yMap.get(i)!),
  };
}
```

**Benefits**:
- ✅ Works without DEPT column
- ✅ Simple alignment logic
- ✅ Handles curves from same parquet file

---

## Summary

The reactivity flow ensures that:

1. **User interactions** (typing axis labels, selecting curves) trigger immediate chart updates
2. **State classes** manage SciChart instances and provide reactive interfaces
3. **Database sync** happens automatically via debounced saves and realtime updates
4. **Component separation** keeps UI components decoupled from SciChart implementation

**Key Flow Patterns**:

- **Axis Labels**: User input → State class → Direct SciChart update → Debounced DB save
- **Series Data**: User selection → Database update → Realtime → State class → Data fetch → Chart update

This architecture provides a seamless, reactive experience where chart updates happen instantly while maintaining database consistency.

