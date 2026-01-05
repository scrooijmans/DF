# Chart Architecture & Persistence Plan

This document outlines the architecture for chart components, persistence, and multi-node assignment in MudRock, integrating with the DAG execution system and Parquet data sources.

## 🎯 Overview

MudRock needs to:

1. **Render large datasets** from Parquet files (via Rust backend) in high-performance SciCharts components
2. **Persist chart configurations** so users can recreate charts after relaunching the application
3. **Assign charts to multiple nodes** (similar to how nodes can be assigned to multiple pipelines)
4. **Support real-time updates** when underlying data changes

## 📊 Chart Types & Use Cases

### 1. Well Log Charts (1D Depth-Based)

**Data Source**: Parquet files from `project-{id}/wells/{well-id}/logs_{log-type}.parquet`

**Chart Type**: Multi-curve depth plot (X = depth, Y = curve values)

**Example**: Gamma Ray, Density, Neutron Porosity vs Depth

**Performance Requirements**:

- Handle 1M+ data points per curve
- Smooth panning/zooming
- Real-time updates when data changes

### 2. Cross-Well Comparison Charts

**Data Source**: Multiple wells, aggregated via DAG execution

**Chart Type**: Overlay multiple wells on same axes

**Example**: Compare GR curves across 10 wells

### 3. Statistical Charts

**Data Source**: DAG execution results (aggregated data)

**Chart Type**: Histograms, scatter plots, box plots

**Example**: Porosity distribution histogram

### 4. Time-Series Charts

**Data Source**: Production data, time-indexed

**Chart Type**: Line charts, area charts

**Example**: Production rate over time

## 🏗️ Chart Schema Design

### Database Schema

```sql
-- Table: charts (chart definitions and configurations)
CREATE TABLE IF NOT EXISTS public.charts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    chart_type TEXT NOT NULL CHECK (chart_type IN (
        'well_log',           -- 1D depth-based well logs
        'cross_well',          -- Multi-well comparison
        'statistical',         -- Histograms, scatter plots
        'time_series',         -- Time-indexed data
        'custom'               -- User-defined custom charts
    )),

    -- Chart configuration (SciCharts settings)
    chart_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    -- Example structure:
    -- {
    --   "xAxis": { "type": "numeric", "title": "Depth (m)" },
    --   "yAxes": [
    --     { "id": "left", "type": "numeric", "title": "GR (API)" },
    --     { "id": "right", "type": "numeric", "title": "RHOB (g/cc)" }
    --   ],
    --   "series": [
    --     { "dataSource": "node-123", "curve": "GR", "yAxisId": "left", "color": "#FF0000" },
    --     { "dataSource": "node-123", "curve": "RHOB", "yAxisId": "right", "color": "#0000FF" }
    --   ],
    --   "zoom": { "xMin": 1000, "xMax": 2000 },
    --   "pan": { "x": 0, "y": 0 }
    -- }

    -- Data source configuration
    data_source_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    -- Example structure:
    -- {
    --   "type": "node",           -- "node" | "well" | "dag_execution" | "parquet_file"
    --   "nodeIds": ["node-123"],  -- For node-based charts
    --   "wellIds": ["well-456"], -- For well-based charts
    --   "executionId": "exec-789", -- For DAG execution results
    --   "parquetPath": "project-123/wells/well-456/logs_composite.parquet" -- For direct Parquet access
    -- }

    -- Rendering settings
    rendering_config JSONB DEFAULT '{}'::jsonb,
    -- Example structure:
    -- {
    --   "maxDataPoints": 1000000,  -- Limit for performance
    --   "downsampling": "lttb",     -- "lttb" | "minmax" | "none"
    --   "updateMode": "realtime",   -- "realtime" | "manual" | "on_execution"
    --   "refreshInterval": 1000     -- ms (for realtime mode)
    -- }

    -- Metadata
    created_by UUID NOT NULL REFERENCES auth.users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    tags TEXT[] DEFAULT '{}',
    is_active BOOLEAN DEFAULT true,

    -- Constraints
    CONSTRAINT charts_project_name_unique UNIQUE (project_id, name)
);

CREATE INDEX idx_charts_project_id ON public.charts(project_id);
CREATE INDEX idx_charts_chart_type ON public.charts(chart_type);
CREATE INDEX idx_charts_created_by ON public.charts(created_by);
CREATE INDEX idx_charts_tags ON public.charts USING GIN(tags);
CREATE INDEX idx_charts_data_source_config ON public.charts USING GIN(data_source_config);

-- Table: chart_node_assignments (many-to-many: charts ↔ nodes)
CREATE TABLE IF NOT EXISTS public.chart_node_assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chart_id UUID NOT NULL REFERENCES charts(id) ON DELETE CASCADE,
    node_id UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,

    -- Assignment-specific configuration (overrides chart defaults)
    assignment_config JSONB DEFAULT '{}'::jsonb,
    -- Example structure:
    -- {
    --   "curveMapping": {           -- Map node output curves to chart series
    --     "GR": { "seriesId": "series-1", "yAxisId": "left" },
    --     "RHOB": { "seriesId": "series-2", "yAxisId": "right" }
    --   },
    --   "visible": true,             -- Show/hide this node's data in chart
    --   "color": "#FF0000",          -- Override default color
    --   "lineStyle": "solid"         -- "solid" | "dashed" | "dotted"
    -- }

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Constraints
    CONSTRAINT chart_node_assignments_unique UNIQUE (chart_id, node_id)
);

CREATE INDEX idx_chart_node_assignments_chart_id ON public.chart_node_assignments(chart_id);
CREATE INDEX idx_chart_node_assignments_node_id ON public.chart_node_assignments(node_id);

-- Table: chart_executions (tracks chart data refreshes)
CREATE TABLE IF NOT EXISTS public.chart_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chart_id UUID NOT NULL REFERENCES charts(id) ON DELETE CASCADE,
    execution_id UUID REFERENCES node_executions(id), -- Link to node execution if applicable

    -- Execution metadata
    status TEXT NOT NULL CHECK (status IN ('pending', 'running', 'completed', 'failed')),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    execution_time_ms BIGINT,

    -- Data snapshot (for reproducibility)
    data_snapshot_id TEXT, -- Reference to cached data snapshot

    -- Error handling
    error_message TEXT,

    -- Metrics
    data_points_loaded BIGINT,
    memory_usage_bytes BIGINT,

    CONSTRAINT chart_executions_status_check CHECK (status IN ('pending', 'running', 'completed', 'failed'))
);

CREATE INDEX idx_chart_executions_chart_id ON public.chart_executions(chart_id);
CREATE INDEX idx_chart_executions_status ON public.chart_executions(status);
CREATE INDEX idx_chart_executions_started_at ON public.chart_executions(started_at DESC);
```

### Chart Configuration Schema (TypeScript)

```typescript
// src/lib/types/charts.ts

export type ChartType =
  | "well_log"
  | "cross_well"
  | "statistical"
  | "time_series"
  | "custom";

export interface ChartConfig {
  xAxis: {
    type: "numeric" | "category" | "date";
    title: string;
    min?: number;
    max?: number;
    visibleRange?: { min: number; max: number };
  };

  yAxes: Array<{
    id: string;
    type: "numeric" | "logarithmic";
    title: string;
    position: "left" | "right";
    min?: number;
    max?: number;
    visibleRange?: { min: number; max: number };
  }>;

  series: Array<{
    id: string;
    dataSource: {
      type: "node" | "well" | "dag_execution" | "parquet_file";
      nodeId?: string;
      wellId?: string;
      executionId?: string;
      parquetPath?: string;
    };
    curve?: string; // Curve name (for well logs)
    xColumn?: string; // Column name for X axis
    yColumn?: string; // Column name for Y axis
    yAxisId: string;
    color: string;
    lineStyle?: "solid" | "dashed" | "dotted";
    lineWidth?: number;
    visible?: boolean;
  }>;

  zoom?: {
    xMin?: number;
    xMax?: number;
    yMin?: number;
    yMax?: number;
  };

  pan?: {
    x: number;
    y: number;
  };

  legend?: {
    visible: boolean;
    position: "top" | "bottom" | "left" | "right";
  };

  grid?: {
    visible: boolean;
    style: "solid" | "dashed";
  };
}

export interface DataSourceConfig {
  type: "node" | "well" | "dag_execution" | "parquet_file";
  nodeIds?: string[];
  wellIds?: string[];
  executionId?: string;
  parquetPath?: string;
}

export interface RenderingConfig {
  maxDataPoints: number; // Limit for performance (default: 1M)
  downsampling: "lttb" | "minmax" | "none"; // Largest-Triangle-Three-Buckets, MinMax, or none
  updateMode: "realtime" | "manual" | "on_execution";
  refreshInterval?: number; // ms (for realtime mode)
}

export interface ChartNodeAssignmentConfig {
  curveMapping?: Record<
    string,
    {
      seriesId: string;
      yAxisId: string;
    }
  >;
  visible?: boolean;
  color?: string;
  lineStyle?: "solid" | "dashed" | "dotted";
}

export interface Chart {
  id: string;
  projectId: string;
  name: string;
  description?: string;
  chartType: ChartType;
  chartConfig: ChartConfig;
  dataSourceConfig: DataSourceConfig;
  renderingConfig: RenderingConfig;
  createdBy: string;
  createdAt: string;
  updatedAt: string;
  tags: string[];
  isActive: boolean;
}
```

## 🔄 Chart Data Flow Architecture

### 1. Chart Creation Flow

```
User creates chart
    ↓
Frontend: ChartBuilder component
    ↓
User selects:
  - Chart type (well_log, cross_well, etc.)
  - Data source (nodes, wells, DAG execution)
  - Series configuration (curves, colors, axes)
    ↓
Frontend: Save chart to PostgreSQL
    ↓
PostgreSQL: INSERT INTO charts (chart_config, data_source_config, ...)
    ↓
Frontend: Load chart data
    ↓
Backend: Fetch data based on data_source_config
  - If nodeIds: Fetch from node_executions or realtime node data
  - If wellIds: Fetch from Parquet files via OpenDAL
  - If executionId: Fetch from DAG execution results
    ↓
Frontend: Render in SciCharts component
```

### 2. Chart Assignment to Nodes

```
User assigns chart to node
    ↓
Frontend: ChartNodeAssignmentDialog
    ↓
User selects:
  - Chart (from charts table)
  - Node (from nodes table)
  - Assignment-specific config (curve mapping, colors, etc.)
    ↓
Frontend: INSERT INTO chart_node_assignments
    ↓
Chart automatically updates when:
  - Node execution completes
  - Node data changes (realtime)
  - User manually refreshes
```

### 3. Multi-Node Chart Updates

```
Node execution completes
    ↓
Backend: Update node_executions table
    ↓
Realtime: Broadcast node_executions update
    ↓
Frontend: ChartState detects update
    ↓
For each chart assigned to this node:
  - Check chart.renderingConfig.updateMode
  - If 'realtime' or 'on_execution': Refresh chart data
  - Fetch new data from node execution results
  - Update SciCharts dataSeries
    ↓
Chart re-renders with new data
```

## 🎨 Chart Component Architecture

### Component Structure

```
src/lib/components/charts/
├── chart-builder.svelte              # Chart creation/editing UI
├── chart-viewer.svelte                # Chart rendering component
├── chart-list.svelte                  # List of charts for project
├── chart-node-assignment-dialog.svelte # Assign chart to node(s)
├── chart-config/
│   ├── well-log-chart-config.svelte  # Well log chart configuration
│   ├── cross-well-chart-config.svelte # Cross-well chart configuration
│   └── statistical-chart-config.svelte # Statistical chart configuration
└── sci-charts/
    ├── sci-chart-surface.svelte       # SciCharts wrapper component
    ├── sci-chart-series.svelte       # Series configuration component
    └── sci-chart-axes.svelte          # Axes configuration component
```

### State Management

```typescript
// src/lib/state/charts/chart-state.svelte.ts

import { getContext, setContext } from "svelte";
import { getPostgresProjectsState } from "$lib/state/postgres/postgres-projects-state.svelte";

const CHARTS_STATE_KEY = Symbol("charts-state");

export class ChartState {
  charts = $state<Chart[]>([]);
  selectedChartId = $state<string | null>(null);
  private projectsState: ReturnType<typeof getPostgresProjectsState> | null =
    null;
  private lastLoadedProjectId: string | null = null;

  constructor() {
    this.projectsState = getPostgresProjectsState();
    this.initializeChartsLoading();
    this.initializeRealtimeSubscriptions();
  }

  private initializeChartsLoading() {
    $effect(() => {
      const projectId = this.projectsState?.currentSelectedProjectId ?? null;

      if (projectId === this.lastLoadedProjectId) {
        return;
      }

      if (projectId) {
        this.charts = [];
        this.lastLoadedProjectId = projectId;
        void this.loadCharts();
      } else {
        this.charts = [];
        this.lastLoadedProjectId = null;
      }
    });
  }

  async loadCharts() {
    const projectId = this.projectsState?.currentSelectedProjectId ?? null;
    if (!projectId) return;

    // Fetch charts from PostgreSQL
    const { data, error } = await supabase
      .from("charts")
      .select("*")
      .eq("project_id", projectId)
      .eq("is_active", true)
      .order("updated_at", { ascending: false });

    if (error) {
      console.error("Failed to load charts:", error);
      return;
    }

    this.charts = data || [];
  }

  async createChart(chart: Omit<Chart, "id" | "createdAt" | "updatedAt">) {
    // Save to PostgreSQL
    const { data, error } = await supabase
      .from("charts")
      .insert({
        ...chart,
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
      })
      .select()
      .single();

    if (error) {
      throw new Error(`Failed to create chart: ${error.message}`);
    }

    // Reload charts
    await this.loadCharts();
    return data;
  }

  async assignChartToNode(
    chartId: string,
    nodeId: string,
    config?: ChartNodeAssignmentConfig,
  ) {
    const { error } = await supabase.from("chart_node_assignments").insert({
      chart_id: chartId,
      node_id: nodeId,
      assignment_config: config || {},
    });

    if (error) {
      throw new Error(`Failed to assign chart to node: ${error.message}`);
    }
  }

  async getChartsForNode(nodeId: string): Promise<Chart[]> {
    const { data, error } = await supabase
      .from("chart_node_assignments")
      .select(
        `
        chart_id,
        charts (*)
      `,
      )
      .eq("node_id", nodeId);

    if (error) {
      console.error("Failed to get charts for node:", error);
      return [];
    }

    return data?.map((row: any) => row.charts) || [];
  }

  private initializeRealtimeSubscriptions() {
    $effect(() => {
      const projectId = this.projectsState?.currentSelectedProjectId ?? null;
      if (!projectId) return;

      // Subscribe to charts table changes
      const chartsChannel = supabase
        .channel("charts-changes")
        .on(
          "postgres_changes",
          {
            event: "*",
            schema: "public",
            table: "charts",
            filter: `project_id=eq.${projectId}`,
          },
          (payload) => {
            void this.loadCharts();
          },
        )
        .subscribe();

      // Subscribe to chart_node_assignments changes
      const assignmentsChannel = supabase
        .channel("chart-assignments-changes")
        .on(
          "postgres_changes",
          {
            event: "*",
            schema: "public",
            table: "chart_node_assignments",
          },
          (payload) => {
            // Reload charts if assignments change
            void this.loadCharts();
          },
        )
        .subscribe();

      return () => {
        supabase.removeChannel(chartsChannel);
        supabase.removeChannel(assignmentsChannel);
      };
    });
  }
}

export function setChartState() {
  if (hasContext(CHARTS_STATE_KEY)) {
    return getChartState();
  }
  const state = new ChartState();
  setContext(CHARTS_STATE_KEY, state);
  return state;
}

export function getChartState() {
  return getContext<ChartState>(CHARTS_STATE_KEY);
}
```

## 🚀 Performance Optimization Strategies

### 1. Data Streaming for Large Datasets

```typescript
// src/lib/services/chart-data-service.ts

import { invoke } from "@tauri-apps/api/core";

export async function* streamChartData(
  chart: Chart,
  chunkSize: number = 10000,
): AsyncGenerator<{ x: Float64Array; y: Float64Array; seriesId: string }> {
  const { dataSourceConfig, chartConfig } = chart;

  if (dataSourceConfig.type === "node" && dataSourceConfig.nodeIds) {
    // Stream from node execution results
    for (const nodeId of dataSourceConfig.nodeIds) {
      for await (const chunk of streamNodeData(nodeId, chunkSize)) {
        yield chunk;
      }
    }
  } else if (dataSourceConfig.type === "well" && dataSourceConfig.wellIds) {
    // Stream from Parquet files
    for (const wellId of dataSourceConfig.wellIds) {
      for await (const chunk of streamParquetData(wellId, chunkSize)) {
        yield chunk;
      }
    }
  }
}

async function* streamParquetData(
  wellId: string,
  chunkSize: number,
): AsyncGenerator<{ x: Float64Array; y: Float64Array }> {
  let offset = 0;
  let hasMore = true;

  while (hasMore) {
    const result = await invoke<{
      x: number[];
      y: number[];
      hasMore: boolean;
    }>("fetch_parquet_chunk", {
      wellId,
      offset,
      limit: chunkSize,
    });

    yield {
      x: new Float64Array(result.x),
      y: new Float64Array(result.y),
    };

    offset += chunkSize;
    hasMore = result.hasMore;
  }
}
```

### 2. Downsampling for Performance

```typescript
// src/lib/utils/charts/downsampling.ts

/**
 * Largest-Triangle-Three-Buckets (LTTB) downsampling
 * Reduces data points while preserving visual shape
 */
export function downsampleLTTB(
  x: Float64Array,
  y: Float64Array,
  targetPoints: number,
): { x: Float64Array; y: Float64Array } {
  if (x.length <= targetPoints) {
    return { x, y };
  }

  const bucketSize = (x.length - 2) / (targetPoints - 2);
  const sampledX = new Float64Array(targetPoints);
  const sampledY = new Float64Array(targetPoints);

  // Always include first point
  sampledX[0] = x[0];
  sampledY[0] = y[0];

  let a = 0;
  let nextA = 0;

  for (let i = 0; i < targetPoints - 2; i++) {
    const rangeStart = Math.floor((i + 1) * bucketSize) + 1;
    const rangeEnd = Math.min(Math.floor((i + 2) * bucketSize) + 1, x.length);

    let avgX = 0;
    let avgY = 0;
    let avgRangeStart = Math.floor((i + 0) * bucketSize) + 1;
    let avgRangeEnd = Math.min(Math.floor((i + 1) * bucketSize) + 1, x.length);

    for (let j = avgRangeStart; j < avgRangeEnd; j++) {
      avgX += x[j];
      avgY += y[j];
    }
    avgX /= avgRangeEnd - avgRangeStart;
    avgY /= avgRangeEnd - avgRangeStart;

    let maxArea = -1;
    for (let j = rangeStart; j < rangeEnd; j++) {
      const area = Math.abs(
        (x[a] - avgX) * (y[j] - y[a]) - (x[a] - x[j]) * (avgY - y[a]),
      );
      if (area > maxArea) {
        maxArea = area;
        nextA = j;
      }
    }

    sampledX[i + 1] = x[nextA];
    sampledY[i + 1] = y[nextA];
    a = nextA;
  }

  // Always include last point
  sampledX[targetPoints - 1] = x[x.length - 1];
  sampledY[targetPoints - 1] = y[y.length - 1];

  return { x: sampledX, y: sampledY };
}
```

### 3. Incremental Chart Updates

```typescript
// src/lib/components/charts/chart-viewer.svelte.ts

import { onDestroy } from "svelte";
import { SciChartSurface } from "scichart";

let chartSurface: SciChartSurface;
let dataSeriesMap = new Map<string, XyDataSeries>();

export async function updateChartData(chart: Chart) {
  if (!chartSurface) return;

  // Suspend updates for batch operation
  chartSurface.suspendUpdates();

  try {
    for (const series of chart.chartConfig.series) {
      const dataSeries = dataSeriesMap.get(series.id);
      if (!dataSeries) continue;

      // Stream data in chunks
      for await (const chunk of streamChartDataForSeries(chart, series)) {
        // Append incrementally
        dataSeries.appendRange(chunk.x, chunk.y);
      }
    }
  } finally {
    chartSurface.resumeUpdates(); // Single redraw
  }
}

onDestroy(() => {
  if (chartSurface) {
    chartSurface.delete(); // Free WebAssembly memory
  }
});
```

## 🔄 Chart Persistence & Restoration

### Chart-Type-Specific State Persistence Architecture

Different chart types require different state parameters to be persisted:

- **XY Charts**: Visible ranges for X and Y axes (`xAxisRange`, `yAxesRanges`)
- **Well Log Charts**: Depth range and curve axis ranges (`depthRange`, `curveRanges`)
- **Map Charts**: Center coordinates, zoom level, bounds (`center`, `zoom`, `bounds`)
- **Statistical Charts**: Filter ranges, bin sizes (`filterRanges`, `binSize`)
- **Time Series Charts**: Time range, aggregation level (`timeRange`, `aggregation`)

**Design Decision**: Use a **registry-based approach** with TypeScript type safety, similar to the `udf-registry-type-safety-architecture.md` pattern.

#### Architecture Components

1. **Type Definitions** (`chart-state-sync.ts`):
   - TypeScript interfaces for each chart type's sync parameters
   - Union type for all chart state sync parameters
   - Handler interface for chart-type-specific logic

2. **Registry Service** (`chart-state-sync-registry.ts`):
   - Maps chart types to their sync handlers
   - Provides unified API: `extractChartState()`, `saveChartState()`, `loadChartState()`, `applyChartState()`
   - Handles debouncing and immediate saves

3. **Chart-Type Handlers** (`handlers/` directory):
   - `xy-chart-state-sync-handler.ts`: Handles XY chart visible ranges
   - `well-log-chart-state-sync-handler.ts`: Handles well log depth/curve ranges (future)
   - `map-chart-state-sync-handler.ts`: Handles map center/zoom/bounds (future)
   - Each handler implements `ChartStateSyncHandler` interface

4. **Initialization** (`chart-state-sync-init.ts`):
   - Registers all chart type handlers at app startup
   - Called in `+layout.svelte` once

#### Implementation Pattern

```typescript
// 1. Define chart-type-specific sync parameters
export interface XYChartStateSyncParams extends ChartStateSyncParams {
  chartType: "xy";
  xAxisRange: { min: number; max: number };
  yAxesRanges: Array<{ id: string; min: number; max: number }>;
}

export interface MapChartStateSyncParams extends ChartStateSyncParams {
  chartType: "map";
  center: { lat: number; lng: number };
  zoom: number;
  bounds?: { north: number; south: number; east: number; west: number };
}

// 2. Implement handler for each chart type
export class XYChartStateSyncHandler
  implements ChartStateSyncHandler<XYChartStateSyncParams>
{
  extractState(chart: any, chartSurface: any): XYChartStateSyncParams | null {
    // Extract visible ranges from SciChart surface
  }

  async saveState(
    params: XYChartStateSyncParams,
  ): Promise<Result<void, Error>> {
    // Save to chart_config.xAxes.options.visibleRange, chart_config.yAxes[]
  }

  loadState(chart: any): XYChartStateSyncParams | null {
    // Load from chart_config
  }

  applyState(chartSurface: any, state: XYChartStateSyncParams): void {
    // Apply visible ranges to SciChart axes
  }
}

// 3. Register handler at app startup
registerChartStateSyncHandler("xy", new XYChartStateSyncHandler());

// 4. Use registry in chart components
const state = await extractChartState(chart, sciChartSurface);
await saveChartState(state, immediate);
const loadedState = loadChartState(chart);
applyChartState(sciChartSurface, loadedState);
```

#### Database Storage Strategy

**Decision**: Store chart-type-specific state in `chart_config` JSONB column using Builder API format.

- **XY Charts**: `chart_config.xAxes.options.visibleRange`, `chart_config.yAxes[].options.visibleRange`
- **Map Charts**: `chart_config.mapState.center`, `chart_config.mapState.zoom`, `chart_config.mapState.bounds`
- **Well Log Charts**: `chart_config.depthRange`, `chart_config.curveRanges[]`

**Rationale**:

- Flexible JSONB structure allows different chart types to store different parameters
- No schema migrations needed when adding new chart types
- Consistent with SciChart Builder API format
- Type-safe extraction via TypeScript interfaces

#### Adding New Chart Types

To add a new chart type (e.g., map charts):

1. **Define sync parameters** in `chart-state-sync.ts`:

   ```typescript
   export interface MapChartStateSyncParams extends ChartStateSyncParams {
     chartType: "map";
     center: { lat: number; lng: number };
     zoom: number;
   }
   ```

2. **Implement handler** in `handlers/map-chart-state-sync-handler.ts`:

   ```typescript
   export class MapChartStateSyncHandler
     implements ChartStateSyncHandler<MapChartStateSyncParams>
   {
     extractState(
       chart: any,
       mapInstance: any,
     ): MapChartStateSyncParams | null {
       // Extract center, zoom from map instance
     }
     // ... implement other methods
   }
   ```

3. **Register handler** in `chart-state-sync-init.ts`:

   ```typescript
   registerChartStateSyncHandler("map", new MapChartStateSyncHandler());
   ```

4. **Use in chart component**:
   ```typescript
   const state = await extractChartState(chart, mapInstance);
   await saveChartState(state, immediate);
   ```

#### Benefits

- **Type Safety**: TypeScript ensures correct parameter types for each chart type
- **Extensibility**: Easy to add new chart types without modifying existing code
- **Separation of Concerns**: Each chart type's sync logic is isolated
- **Consistency**: Unified API for all chart types
- **Maintainability**: Clear pattern for future chart types

### Saving Chart State

Chart state is saved automatically:

1. **During User Interaction** (debounced, 1 second delay):
   - User zooms/pans chart
   - Chart component extracts state via registry
   - State is saved with debounce to prevent excessive writes

2. **When Switching Charts** (immediate):
   - User clicks different chart in sidebar
   - Previous chart's state is saved immediately (before destruction)
   - New chart's state is loaded from database

### Restoring Chart on Reload

```typescript
// On component mount
async function restoreChart(chartId: string) {
  const { data } = await supabase
    .from("charts")
    .select("*")
    .eq("id", chartId)
    .single();

  if (!data) return;

  // Load chart state using registry (chart-type-specific)
  const loadedState = loadChartState(data);

  if (loadedState) {
    // Apply state to chart surface
    applyChartState(chartSurface, loadedState);
  }

  // Load and render data
  await loadChartData(data);
}
```

## 📋 Implementation Checklist

### Phase 1: Database Schema ✅

- [ ] Create `charts` table migration
- [ ] Create `chart_node_assignments` table migration
- [ ] Create `chart_executions` table migration
- [ ] Add indexes for performance
- [ ] Enable Realtime subscriptions

### Phase 2: Chart State Management ✅

- [ ] Create `ChartState` class
- [ ] Implement chart loading/creation
- [ ] Implement chart-to-node assignment
- [ ] Add realtime subscriptions

### Phase 3: Chart Components ✅

- [ ] Create `chart-builder.svelte`
- [ ] Create `chart-viewer.svelte` with SciCharts integration
- [ ] Create `chart-list.svelte`
- [ ] Create `chart-node-assignment-dialog.svelte`
- [ ] Implement chart configuration UIs

### Phase 4: Data Loading ✅

- [ ] Implement Parquet data streaming
- [ ] Implement node execution data loading
- [ ] Add downsampling utilities
- [ ] Add incremental update support

### Phase 5: Performance Optimization ✅

- [ ] Implement LTTB downsampling
- [ ] Add data point limits
- [ ] Optimize SciCharts rendering
- [ ] Add memory cleanup

### Phase 6: Persistence ✅

- [ ] Implement chart state saving
- [ ] Implement chart restoration
- [ ] Add auto-save on chart modifications
- [ ] Test persistence across app restarts

## 📊 Chart Data Source Configuration

### Overview

Different chart types require different types of data sources:

- **XY Charts**: Curve data from `curves` table (well logs, measurements)
- **Map Charts**: Geographical data (wells x/y coordinates, polygons, surfaces)
- **Well Log Charts**: Depth-based curve data from `curves` table
- **Statistical Charts**: Aggregated data from node executions
- **Time Series Charts**: Time-indexed data from node executions

### Architecture: Data Source Registry

Similar to the state sync registry, we use a **registry-based approach** for data source configuration:

1. **Type Definitions** (`chart-data-sources.ts`): TypeScript interfaces for allowed data sources per chart type
2. **Data Source Registry** (`chart-data-source-registry.ts`): Maps chart types to their allowed data sources
3. **Database Extension** (`chart_types.allowed_data_sources` JSONB): Optional database storage for data source metadata
4. **Validation Service** (`chart-data-source-validation.ts`): Validates data source selections against allowed types

### Design Decision: Hybrid Approach

**Decision**: Use **both TypeScript types and database storage**:

- **TypeScript Types** (`chart-data-sources.ts`): Primary source of truth for type safety and compile-time validation
- **Database Column** (`chart_types.allowed_data_sources` JSONB): Stores metadata, descriptions, and UI hints for data source selection

**Rationale**:

- TypeScript ensures type safety and prevents invalid data source assignments
- Database storage enables dynamic configuration and UI generation
- Hybrid approach provides both compile-time safety and runtime flexibility

### Data Source Type Definitions

```typescript
// src/lib/components/pages/home/charts/types/chart-data-sources.ts

export type DataSourceType =
  | "curve" // From curves table (well logs)
  | "well" // From wells table (geographical data)
  | "polygon" // From polygons table (boundaries, formations)
  | "surface" // From surfaces catalog (Parquet files)
  | "node_execution" // From node_executions table (computed data)
  | "custom"; // Custom data source

export interface CurveDataSource {
  type: "curve";
  curveId: string; // Reference to curves.id
  wellId: number; // Reference to wells.id
  xColumn?: string; // Column name for X axis (default: depth)
  yColumn: string; // Column name for Y axis (curve column)
}

export interface WellDataSource {
  type: "well";
  wellId: number; // Reference to wells.id
  xColumn: "x" | "longitude"; // X coordinate column
  yColumn: "y" | "latitude"; // Y coordinate column
}

export interface PolygonDataSource {
  type: "polygon";
  polygonId: string; // Reference to polygons.id
}

export interface SurfaceDataSource {
  type: "surface";
  surfaceName: string; // Surface name from surfaces catalog
  projectId: string; // Project ID for surface catalog path
}

export interface NodeExecutionDataSource {
  type: "node_execution";
  executionId: string; // Reference to node_executions.id
  outputColumn: string; // Column name from execution output
}

export type ChartDataSource =
  | CurveDataSource
  | WellDataSource
  | PolygonDataSource
  | SurfaceDataSource
  | NodeExecutionDataSource;

// Chart-type-specific allowed data sources
export interface XYChartDataSources {
  allowed: ["curve", "node_execution"];
  required: ["curve"]; // At least one curve is required
}

export interface MapChartDataSources {
  allowed: ["well", "polygon", "surface"];
  required: []; // No required sources (can show empty map)
}

export interface WellLogChartDataSources {
  allowed: ["curve"];
  required: ["curve"]; // At least one curve is required
}

export interface StatisticalChartDataSources {
  allowed: ["node_execution"];
  required: ["node_execution"];
}

export interface TimeSeriesChartDataSources {
  allowed: ["node_execution"];
  required: ["node_execution"];
}
```

### Database Schema Extension

Add `allowed_data_sources` JSONB column to `chart_types` table:

```sql
-- Migration: Add allowed_data_sources to chart_types
ALTER TABLE public.chart_types
ADD COLUMN IF NOT EXISTS allowed_data_sources JSONB DEFAULT '[]'::jsonb;

-- Example: Update XY chart type
UPDATE public.chart_types
SET allowed_data_sources = jsonb_build_array(
  jsonb_build_object(
    'type', 'curve',
    'description', 'Well log curves (e.g., GR, RHOB, DT)',
    'required', true,
    'multiple', true
  ),
  jsonb_build_object(
    'type', 'node_execution',
    'description', 'Computed data from node executions',
    'required', false,
    'multiple', true
  )
)
WHERE chart_type_id = 'xy';

-- Example: Update Map chart type
UPDATE public.chart_types
SET allowed_data_sources = jsonb_build_array(
  jsonb_build_object(
    'type', 'well',
    'description', 'Well locations (x, y coordinates)',
    'required', false,
    'multiple', true
  ),
  jsonb_build_object(
    'type', 'polygon',
    'description', 'Geographical polygons (lease boundaries, formations)',
    'required', false,
    'multiple', true
  ),
  jsonb_build_object(
    'type', 'surface',
    'description', '3D surfaces from surfaces catalog',
    'required', false,
    'multiple', true
  )
)
WHERE chart_type_id = 'map';
```

### Data Source Registry

```typescript
// src/lib/components/pages/home/charts/services/chart-data-source-registry.ts

import type { ChartType, ChartDataSource } from "../types/chart-data-sources";

const allowedDataSources = new Map<ChartType, ChartDataSource["type"][]>([
  ["xy", ["curve", "node_execution"]],
  ["map", ["well", "polygon", "surface"]],
  ["well_log", ["curve"]],
  ["statistical", ["node_execution"]],
  ["time_series", ["node_execution"]],
]);

export function getAllowedDataSources(
  chartType: ChartType,
): ChartDataSource["type"][] {
  return allowedDataSources.get(chartType) || [];
}

export function isValidDataSource(
  chartType: ChartType,
  dataSource: ChartDataSource,
): boolean {
  const allowed = getAllowedDataSources(chartType);
  return allowed.includes(dataSource.type);
}

export function validateChartDataSources(
  chartType: ChartType,
  dataSources: ChartDataSource[],
): { valid: boolean; errors: string[] } {
  const allowed = getAllowedDataSources(chartType);
  const errors: string[] = [];

  for (const ds of dataSources) {
    if (!allowed.includes(ds.type)) {
      errors.push(
        `Data source type '${ds.type}' is not allowed for chart type '${chartType}'`,
      );
    }
  }

  return { valid: errors.length === 0, errors };
}
```

### Chart Configuration Storage

Data sources are stored in `charts.data_source_config` JSONB:

```json
{
  "type": "curve",
  "sources": [
    {
      "type": "curve",
      "curveId": "uuid-here",
      "wellId": 123,
      "xColumn": "depth",
      "yColumn": "gr"
    }
  ]
}
```

For map charts:

```json
{
  "type": "map",
  "sources": [
    {
      "type": "well",
      "wellId": 123,
      "xColumn": "x",
      "yColumn": "y"
    },
    {
      "type": "polygon",
      "polygonId": "uuid-here"
    },
    {
      "type": "surface",
      "surfaceName": "F3-Horizon-Truncation",
      "projectId": "uuid-here"
    }
  ]
}
```

### Implementation Plan

1. **Create Type Definitions** (`chart-data-sources.ts`):
   - Define `DataSourceType` union type
   - Define interfaces for each data source type
   - Define chart-type-specific allowed sources

2. **Create Registry** (`chart-data-source-registry.ts`):
   - Map chart types to allowed data source types
   - Provide validation functions
   - Provide helper functions for UI generation

3. **Update Database Schema**:
   - Add `allowed_data_sources` JSONB to `chart_types` table
   - Populate with metadata for each chart type
   - Create migration script

4. **Update Chart Creation**:
   - Validate data sources when creating charts
   - Store data sources in `data_source_config` JSONB
   - Provide UI for selecting data sources

5. **Update Chart Components**:
   - Load data based on `data_source_config`
   - Fetch data from appropriate sources (curves, wells, polygons, surfaces)
   - Render data in chart-specific format

### Display Settings Architecture

**Problem**: Different data source types require different display settings, and the same data source (e.g., a polygon) may have different display settings in different charts.

**Solution**: Store display settings **per data source instance** in `charts.data_source_config` JSONB array. Each entry contains:

- `id`: Unique identifier for this data source instance in the chart
- `source`: The data source definition (type, IDs, etc.)
- `displaySettings`: Chart-type and data-source-type-specific display settings

**Example Structure**:

```json
[
  {
    "id": "polygon-1",
    "source": {
      "type": "polygon",
      "polygonId": "uuid-here"
    },
    "displaySettings": {
      "visible": true,
      "fillColor": "#3B82F6",
      "fillOpacity": 0.3,
      "strokeColor": "#1E40AF",
      "strokeWidth": 2,
      "strokeStyle": "dashed"
    }
  },
  {
    "id": "surface-1",
    "source": {
      "type": "surface",
      "surfaceName": "F3-Horizon-Truncation",
      "projectId": "uuid-here"
    },
    "displaySettings": {
      "visible": true,
      "showContours": true,
      "contourInterval": 10,
      "contourColor": "#000000",
      "colormap": "viridis"
    }
  }
]
```

**Key Design Decisions**:

1. **Per-Instance Settings**: Each data source instance in a chart has its own display settings. The same polygon can appear in multiple charts with different settings.
2. **Chart-Type-Specific Defaults**: Default display settings are provided by `getDefaultDisplaySettings(chartType, dataSourceType)` based on chart type and data source type.
3. **Type Safety**: TypeScript interfaces ensure display settings match the expected structure for each data source type.
4. **Database Storage**: Display settings are stored in `data_source_config` JSONB, allowing flexible schema evolution without migrations.

### Implementation Status

1. ✅ Create `chart-data-sources.ts` with type definitions
2. ✅ Create `chart-data-source-registry.ts` with validation logic
3. ✅ Create migration `020-chart-data-source-display-settings.sql` to add `allowed_data_sources` to `chart_types`
4. ✅ Create `chart-editor-sidebar.svelte` with Info and Data tabs
5. ✅ Create `chart-editor-data-options.svelte` for data source selection UI
6. ✅ Create `chart-data-curve-options.svelte` for curve selection (XY charts)
7. ✅ Create `chart-data-curve-options-AG-table.svelte` for displaying curves
8. ✅ Separate `input-curves.svelte` component from `content-node-editor.svelte`
9. ⏳ Update `chart-service.ts` to validate data sources on creation
10. ⏳ Implement data loading logic in chart components (load curves, polygons, surfaces)
11. ⏳ Add display settings editing UI (color pickers, stroke style selectors, etc.)
12. ⏳ Implement map chart data source selection (wells, polygons, surfaces)

## 🎯 Summary

**Chart Architecture**:

- ✅ **PostgreSQL storage** for chart configurations (flexible JSONB schema)
- ✅ **Many-to-many relationship** between charts and nodes (via `chart_node_assignments`)
- ✅ **Realtime updates** when node data changes
- ✅ **Performance optimized** with streaming, downsampling, and typed arrays
- ✅ **Persistence** across app restarts via PostgreSQL
- ✅ **Flexible state sync** via registry-based architecture
- ✅ **Type-safe data sources** via registry and TypeScript types

**Key Design Decisions**:

1. **PostgreSQL over file storage**: More flexible, queryable, supports realtime
2. **JSONB for configuration**: Allows schema evolution without migrations
3. **Many-to-many assignments**: Charts can be assigned to multiple nodes (like nodes to pipelines)
4. **Streaming data loading**: Handles large datasets efficiently
5. **Downsampling**: Reduces data points while preserving visual fidelity
6. **Registry-based state sync**: Extensible architecture for chart-type-specific state persistence
7. **Hybrid data source configuration**: TypeScript types + database metadata for type safety and flexibility
