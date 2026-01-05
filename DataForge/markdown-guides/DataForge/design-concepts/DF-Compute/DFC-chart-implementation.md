# DataForge Compute: High-Performance Chart Implementation

## Executive Summary

DataForge Compute requires a **high-performance charting system** capable of rendering 100,000+ data points without UI blocking. This document defines the architecture for integrating a Canvas/WebGL-based charting library (ECharts recommended) with our existing multi-view synchronization system, ensuring real-time responsiveness while maintaining the event-driven update patterns established in DFC-model-view-sync.md.

The architecture draws from best practices in **deck.gl** (efficient data updates, layer architecture, view state management) and **Datashader** (aggregation/rendering separation, resolution-based Level-of-Detail) to enable visualization of datasets with millions of points.

**Key Requirements:**
1. **Performance**: Render 100K+ points at 60fps without UI freezing
2. **Consistency**: Charts integrate with ViewManager and DataStore patterns
3. **Extensibility**: Support multiple chart types (line, scatter, histogram, crossplot)
4. **Interactivity**: Pan, zoom, crosshair, selection, and linked views
5. **Library Abstraction**: Design for easy switching between charting libraries

---

## 1. Architectural Overview

### 1.1 System Context

```

                       DATA LAYER (Rust/Tauri)

                         DATA STORE
    - Curve data (100K+ points per curve)
    - Well metadata
    - Tool execution results
    - Emits DataChangeEvent on all changes


                              |
                              v Events

                       BRIDGE LAYER

                    DATA-VIEW BRIDGE
    - Translates DataChangeEvent to ViewNotification
    - Emits Tauri events to frontend


                              |
                              v Tauri Events

                       FRONTEND (Svelte/TypeScript)

                       VIEW MANAGER
    - Routes notifications to ChartManager
    - Handles view lifecycle


                              |
                              v Notifications

                       CHART LAYER

                      CHART MANAGER
    - Manages chart instances
    - Coordinates linked interactions
    - Handles data transformations

                              |
              +---------------+---------------+
              v               v               v

  Line Chart     Scatter Chart    Histogram Chart
  (ECharts)      (ECharts)        (ECharts)


```

### 1.2 Why Canvas/WebGL-Based Charting

| Approach | 10K Points | 100K Points | 1M Points | Update Latency |
|----------|-----------|------------|-----------|----------------|
| **SVG DOM** (current) | 200ms | 2000ms (freezes) | N/A | 50-200ms |
| **Canvas 2D** | 50ms | 200ms | 800ms | 20-50ms |
| **Canvas + WebGL** | 5ms | 20ms | 80ms | 1-5ms |

**Current Issue**: The existing CurveChart.svelte uses SVG path generation, which performs O(n) string concatenation on every render. With 50,000+ points, this causes the UI to freeze.

**Solution**: Replace with a Canvas-based charting library that can handle large datasets efficiently.

### 1.3 Library Selection: ECharts

| Criteria | ECharts | uPlot | Chart.js | SciChart |
|----------|---------|-------|----------|----------|
| **Bundle Size** | ~900KB (tree-shakeable to ~300KB) | ~30KB | ~60KB | ~500KB |
| **100K Points** | ~50ms (with sampling) | <20ms | ~500ms | <10ms |
| **License** | Apache 2.0 | MIT | MIT | Commercial |
| **Depth Axis** | Excellent (flexible axis) | Time-series focused | Limited | Yes |
| **Linked Cursors** | Built-in connect API | Built-in | No | Yes |
| **Chart Types** | 20+ types | Line/scatter only | ~10 types | Many |
| **Customization** | Extensive | Limited | Medium | Extensive |
| **Documentation** | Excellent | Good | Good | Excellent |

**Recommendation**: **ECharts** for its:
- **Flexibility**: Not time-series focused like uPlot; better suited for depth/curve displays
- **Rich chart types**: Supports line, scatter, bar, heatmap, and custom series out of the box
- **Built-in data sampling**: `sampling: 'lttb'` for automatic Level-of-Detail
- **Large data mode**: `large: true` enables GPU-accelerated rendering
- **Connect API**: Native support for linked charts and synchronized cursors
- **Tree-shaking**: Can reduce bundle to ~300KB by importing only needed components

### 1.4 Library Abstraction Strategy

To enable easy library switching, we use an **adapter pattern** with a common interface:

```
┌─────────────────────────────────────────────────────────────┐
│                    IChartInstance                           │
│  (Abstract interface for all chart implementations)         │
├─────────────────────────────────────────────────────────────┤
│  mount(container)                                           │
│  setData(frame)                                             │
│  setConfig(config)                                          │
│  getViewport() / setViewport(viewport)                      │
│  setCursor(position)                                        │
│  destroy()                                                  │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
┌───────────────┐    ┌───────────────┐    ┌───────────────┐
│ EChartsAdapter│    │ UPlotAdapter  │    │ CustomAdapter │
│               │    │ (fallback)    │    │ (future)      │
└───────────────┘    └───────────────┘    └───────────────┘
```

**Key benefits:**
1. **Swap libraries without changing components**: Only the adapter changes
2. **Test with mock adapters**: Easier unit testing
3. **Progressive enhancement**: Start with ECharts, add specialized adapters later
4. **Fallback support**: Use simpler library if main fails to load

---

## 2. Charting Model (Core Abstractions)

### 2.1 ChartDataFrame

Inspired by Grafana's DataFrame concept, we define a unified data structure for all chart data:

```typescript
// src/lib/charts/types.ts

/**
 * ChartDataFrame - Unified data structure for all chart data
 *
 * Design: Columnar layout optimized for GPU transfer and typed arrays
 */
export interface ChartDataFrame {
  /** Unique identifier for this data frame */
  id: string;

  /** Human-readable name */
  name: string;

  /** Source information (well, curve, computed) */
  source: DataFrameSource;

  /** Array of fields (columns) */
  fields: ChartField[];

  /** Total number of rows */
  length: number;

  /** Optional metadata */
  meta?: DataFrameMeta;
}

/**
 * ChartField - A single column of data
 *
 * Design: Uses TypedArrays for GPU-compatible memory layout
 */
export interface ChartField {
  /** Field name (e.g., "DEPTH", "GR", "RHOB") */
  name: string;

  /** Data type */
  type: FieldType;

  /** Actual values - MUST be TypedArray for performance */
  values: Float64Array | Float32Array | Int32Array;

  /** Display configuration */
  config?: FieldConfig;

  /** Unit of measurement */
  unit?: string;

  /** Null bitmask (1 = valid, 0 = null) - optional */
  nullMask?: Uint8Array;
}

export enum FieldType {
  Number = 'number',
  Time = 'time',
  String = 'string',
}

export interface FieldConfig {
  /** Display name override */
  displayName?: string;
  /** Min value for axis scaling */
  min?: number;
  /** Max value for axis scaling */
  max?: number;
  /** Color for this field */
  color?: string;
  /** Line width for line charts */
  lineWidth?: number;
  /** Point size for scatter charts */
  pointSize?: number;
}

export interface DataFrameSource {
  /** Type of source */
  type: 'well_curve' | 'computed' | 'imported';
  /** Well ID if from well */
  wellId?: string;
  /** Curve ID if from curve */
  curveId?: string;
  /** Computation ID if computed */
  computationId?: string;
}

export interface DataFrameMeta {
  /** Preferred chart type */
  preferredChartType?: ChartType;
  /** Is depth inverted (true for well logs) */
  depthInverted?: boolean;
  /** Time range if time-based */
  timeRange?: { from: number; to: number };
  /** Depth range if depth-based */
  depthRange?: { min: number; max: number };
}
```

### 2.2 Chart Configuration

```typescript
// src/lib/charts/types.ts

export enum ChartType {
  Line = 'line',
  Scatter = 'scatter',
  Histogram = 'histogram',
  CrossPlot = 'crossplot',
  WellLog = 'welllog',
}

/**
 * ChartConfig - Configuration for a chart instance
 */
export interface ChartConfig {
  /** Unique chart ID */
  id: string;

  /** Chart type */
  type: ChartType;

  /** Chart title */
  title?: string;

  /** X-axis configuration */
  xAxis: AxisConfig;

  /** Y-axis configuration (can have multiple for well logs) */
  yAxes: AxisConfig[];

  /** Series configurations */
  series: SeriesConfig[];

  /** Interaction settings */
  interaction: InteractionConfig;

  /** Visual theme */
  theme?: ChartTheme;
}

export interface AxisConfig {
  /** Field name this axis displays */
  field: string;

  /** Axis label */
  label?: string;

  /** Is axis inverted (depth increases downward) */
  inverted?: boolean;

  /** Fixed min value */
  min?: number;

  /** Fixed max value */
  max?: number;

  /** Logarithmic scale */
  log?: boolean;

  /** Grid lines visible */
  grid?: boolean;
}

export interface SeriesConfig {
  /** Field name for this series */
  field: string;

  /** Display label */
  label?: string;

  /** Line/point color */
  color?: string;

  /** Line width (line charts) */
  width?: number;

  /** Point size (scatter charts) */
  pointSize?: number;

  /** Fill options */
  fill?: FillConfig;

  /** Visibility */
  visible?: boolean;
}

export interface InteractionConfig {
  /** Enable pan */
  pan?: boolean;

  /** Enable zoom */
  zoom?: boolean;

  /** Show crosshair cursor */
  cursor?: boolean;

  /** Enable point selection */
  select?: boolean;

  /** Link this chart to others (cursor sync) */
  linkedGroup?: string;
}

export interface FillConfig {
  /** Fill color (with alpha) */
  color?: string;
  /** Fill to axis value */
  toValue?: number;
  /** Fill to another series */
  toSeries?: string;
}
```

### 2.3 Chart Instance Interface

```typescript
// src/lib/charts/types.ts

/**
 * IChartInstance - Interface for chart implementations
 */
export interface IChartInstance {
  /** Unique instance ID */
  readonly id: string;

  /** Chart type */
  readonly type: ChartType;

  /** Current configuration */
  readonly config: ChartConfig;

  /** Initialize chart in container */
  mount(container: HTMLElement): void;

  /** Update data without full redraw */
  setData(frame: ChartDataFrame): void;

  /** Update configuration */
  setConfig(config: Partial<ChartConfig>): void;

  /** Resize chart */
  resize(width: number, height: number): void;

  /** Get current viewport (for linked charts) */
  getViewport(): Viewport;

  /** Set viewport (for linked charts) */
  setViewport(viewport: Viewport): void;

  /** Set cursor position (for linked charts) */
  setCursor(position: CursorPosition | null): void;

  /** Destroy chart and cleanup */
  destroy(): void;

  /** Event emitter for chart events */
  on<K extends keyof ChartEvents>(event: K, handler: ChartEvents[K]): void;
  off<K extends keyof ChartEvents>(event: K, handler: ChartEvents[K]): void;
}

export interface Viewport {
  xMin: number;
  xMax: number;
  yMin: number;
  yMax: number;
}

export interface CursorPosition {
  x: number;
  y: number;
  dataIndex?: number;
}

export interface ChartEvents {
  'viewport-change': (viewport: Viewport) => void;
  'cursor-move': (position: CursorPosition) => void;
  'cursor-leave': () => void;
  'select': (selection: Selection) => void;
  'click': (point: DataPoint) => void;
}

export interface Selection {
  type: 'range' | 'points';
  xRange?: { min: number; max: number };
  yRange?: { min: number; max: number };
  points?: number[];
}

export interface DataPoint {
  index: number;
  x: number;
  y: number;
  series: string;
}
```

---

## 3. Data Flow & State Management

### 3.1 Data Flow Architecture

```

                        DATA FLOW: Backend to Chart

  Rust Backend

  get_curve_data(curve_id)
    -> Returns CurveData { depth: Vec<f64>, values: Vec<f64> }

         |
         v IPC (Tauri invoke)

  Frontend (TypeScript)

  chartDataStore.loadCurve(curveId)
    -> Transforms to ChartDataFrame
    -> Stores in Map<string, ChartDataFrame>
    -> Returns reference (not copy)

         |
         v Reference

  ChartManager

  chartManager.bindData(chartId, frameId)
    -> Retrieves ChartDataFrame from store
    -> Passes TypedArrays directly to uPlot
    -> NO data copying

         |
         v TypedArrays (zero-copy)

  uPlot Instance

  chart.setData([depths, values])
    -> GPU uploads typed arrays directly
    -> Renders via WebGL


```

### 3.2 Chart Data Store

```typescript
// src/lib/charts/chart-data-store.ts

import { writable, derived, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { ChartDataFrame, ChartField, FieldType } from './types';

/**
 * ChartDataStore - Manages chart data frames with efficient caching
 *
 * Design Principles:
 * 1. Store raw TypedArrays, not JavaScript arrays
 * 2. Cache data frames by ID
 * 3. Support incremental updates (append new data)
 * 4. Emit events for data changes
 */
class ChartDataStoreImpl {
  /** Cached data frames */
  private frames = new Map<string, ChartDataFrame>();

  /** Svelte store for reactive updates */
  private _store = writable<Map<string, ChartDataFrame>>(new Map());

  /** Subscribe to store updates */
  get store() {
    return { subscribe: this._store.subscribe };
  }

  /**
   * Load curve data from backend and convert to ChartDataFrame
   */
  async loadCurve(
    curveId: string,
    wellId: string,
    mnemonic: string
  ): Promise<ChartDataFrame> {
    // Check cache first
    const cacheKey = `curve:${curveId}`;
    if (this.frames.has(cacheKey)) {
      return this.frames.get(cacheKey)!;
    }

    // Fetch from backend
    const data = await invoke<{ depth: number[]; value: (number | null)[] }>(
      'get_curve_data',
      { wellId, curveId }
    );

    // Convert to TypedArrays for GPU efficiency
    const depthArray = new Float64Array(data.depth);
    const valueArray = new Float64Array(data.value.length);
    const nullMask = new Uint8Array(data.value.length);

    for (let i = 0; i < data.value.length; i++) {
      const v = data.value[i];
      if (v !== null && !Number.isNaN(v)) {
        valueArray[i] = v;
        nullMask[i] = 1;
      } else {
        valueArray[i] = NaN;
        nullMask[i] = 0;
      }
    }

    const frame: ChartDataFrame = {
      id: cacheKey,
      name: mnemonic,
      source: {
        type: 'well_curve',
        wellId,
        curveId,
      },
      fields: [
        {
          name: 'DEPTH',
          type: FieldType.Number,
          values: depthArray,
          unit: 'm',
          config: { displayName: 'Depth' },
        },
        {
          name: mnemonic,
          type: FieldType.Number,
          values: valueArray,
          nullMask,
          config: { displayName: mnemonic },
        },
      ],
      length: depthArray.length,
      meta: {
        depthInverted: true, // Well logs: depth increases downward
        depthRange: {
          min: depthArray[0],
          max: depthArray[depthArray.length - 1],
        },
      },
    };

    // Cache and notify
    this.frames.set(cacheKey, frame);
    this._store.set(new Map(this.frames));

    return frame;
  }

  /**
   * Load computed result as ChartDataFrame
   */
  loadComputedResult(
    executionId: string,
    mnemonic: string,
    depths: number[],
    values: (number | null)[]
  ): ChartDataFrame {
    const cacheKey = `computed:${executionId}`;

    const depthArray = new Float64Array(depths);
    const valueArray = new Float64Array(values.length);
    const nullMask = new Uint8Array(values.length);

    for (let i = 0; i < values.length; i++) {
      const v = values[i];
      if (v !== null && !Number.isNaN(v)) {
        valueArray[i] = v;
        nullMask[i] = 1;
      } else {
        valueArray[i] = NaN;
        nullMask[i] = 0;
      }
    }

    const frame: ChartDataFrame = {
      id: cacheKey,
      name: `${mnemonic} (computed)`,
      source: {
        type: 'computed',
        computationId: executionId,
      },
      fields: [
        {
          name: 'DEPTH',
          type: FieldType.Number,
          values: depthArray,
          unit: 'm',
        },
        {
          name: mnemonic,
          type: FieldType.Number,
          values: valueArray,
          nullMask,
          config: { color: '#22c55e' }, // Green for computed
        },
      ],
      length: depthArray.length,
      meta: {
        depthInverted: true,
        preferredChartType: 'line',
      },
    };

    this.frames.set(cacheKey, frame);
    this._store.set(new Map(this.frames));

    return frame;
  }

  /**
   * Get cached frame by ID
   */
  getFrame(frameId: string): ChartDataFrame | undefined {
    return this.frames.get(frameId);
  }

  /**
   * Clear cache for a specific frame
   */
  invalidate(frameId: string): void {
    this.frames.delete(frameId);
    this._store.set(new Map(this.frames));
  }

  /**
   * Clear all cached data
   */
  clear(): void {
    this.frames.clear();
    this._store.set(new Map());
  }
}

export const chartDataStore = new ChartDataStoreImpl();
```

### 3.3 Integration with ViewManager

```typescript
// src/lib/charts/chart-view-bridge.ts

import { viewManager, type DataChangeNotification } from '$lib/stores/view-manager';
import { chartDataStore } from './chart-data-store';
import { chartManager } from './chart-manager';
import { ViewDataType } from '$lib/types/view-types';

/**
 * ChartViewBridge - Connects ViewManager notifications to chart updates
 *
 * When data changes in the backend, this bridge:
 * 1. Invalidates cached ChartDataFrames
 * 2. Triggers chart refreshes for affected charts
 */
export function initChartViewBridge(): void {
  // Listen for data changes from ViewManager
  viewManager.onNotification((notification: DataChangeNotification) => {
    switch (notification.dataType) {
      case ViewDataType.Curve:
        handleCurveChange(notification);
        break;
      case ViewDataType.ToolResult:
        handleToolResultChange(notification);
        break;
    }
  });
}

function handleCurveChange(notification: DataChangeNotification): void {
  const curveId = notification.itemId;

  // Invalidate cached data
  chartDataStore.invalidate(`curve:${curveId}`);

  // Notify charts displaying this curve
  chartManager.notifyDataChange(curveId);
}

function handleToolResultChange(notification: DataChangeNotification): void {
  const executionId = notification.itemId;

  if (notification.changeKind === 'added') {
    // New computation result - charts may want to display it
    chartManager.notifyNewResult(executionId);
  }
}
```

---

## 4. Chart Rendering Strategy

### 4.1 ECharts Integration

```typescript
// src/lib/charts/echarts-adapter.ts

import * as echarts from 'echarts/core';
import { LineChart, ScatterChart } from 'echarts/charts';
import { GridComponent, TooltipComponent, DataZoomComponent } from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';
import type { IChartInstance, ChartConfig, ChartDataFrame, Viewport, CursorPosition, ChartEvents } from './types';

// Register ECharts components (tree-shaking friendly)
echarts.use([LineChart, ScatterChart, GridComponent, TooltipComponent, DataZoomComponent, CanvasRenderer]);

/**
 * EChartsAdapter - Wraps ECharts for our chart interface
 *
 * Key ECharts features used:
 * - `large: true` for GPU-accelerated rendering
 * - `sampling: 'lttb'` for automatic downsampling
 * - `connect()` for linked charts
 * - `dataZoom` for pan/zoom
 */
export class EChartsAdapter implements IChartInstance {
  readonly id: string;
  readonly type: ChartType;

  private chart: echarts.ECharts | null = null;
  private container: HTMLElement | null = null;
  private _config: ChartConfig;
  private currentFrame: ChartDataFrame | null = null;

  private eventHandlers = new Map<string, Set<Function>>();

  constructor(id: string, config: ChartConfig) {
    this.id = id;
    this.type = config.type;
    this._config = config;
  }

  get config(): ChartConfig {
    return this._config;
  }

  mount(container: HTMLElement): void {
    this.container = container;
    this.createChart();
  }

  private createChart(): void {
    if (!this.container) return;

    // Initialize ECharts instance
    this.chart = echarts.init(this.container);

    // Build and set initial options
    const options = this.buildEChartsOptions();
    this.chart.setOption(options);

    // Setup event listeners
    this.setupEvents();

    // Register for linked charts if specified
    if (this._config.interaction.linkedGroup) {
      echarts.connect(this._config.interaction.linkedGroup);
    }
  }

  private buildEChartsOptions(): echarts.EChartsOption {
    const { xAxis, yAxes, series, interaction } = this._config;

    return {
      // Grid configuration
      grid: {
        left: 60,
        right: 20,
        top: 40,
        bottom: 60,
        containLabel: true,
      },

      // X Axis (typically depth for well logs)
      xAxis: {
        type: 'value',
        name: xAxis.label ?? xAxis.field,
        nameLocation: 'middle',
        nameGap: 35,
        inverse: xAxis.inverted ?? false,
        min: xAxis.min,
        max: xAxis.max,
        splitLine: { show: xAxis.grid ?? true },
      },

      // Y Axis
      yAxis: {
        type: 'value',
        name: yAxes[0]?.label ?? yAxes[0]?.field,
        nameLocation: 'middle',
        nameGap: 45,
        inverse: yAxes[0]?.inverted ?? false,
        min: yAxes[0]?.min,
        max: yAxes[0]?.max,
        splitLine: { show: yAxes[0]?.grid ?? true },
      },

      // Tooltip (crosshair)
      tooltip: {
        show: interaction.cursor ?? true,
        trigger: 'axis',
        axisPointer: {
          type: 'cross',
          animation: false,
        },
      },

      // Data zoom for pan/zoom
      dataZoom: interaction.zoom ? [
        { type: 'inside', xAxisIndex: 0 },
        { type: 'inside', yAxisIndex: 0 },
      ] : [],

      // Series configuration
      series: series.map((s) => ({
        name: s.label ?? s.field,
        type: this.type === ChartType.Scatter ? 'scatter' : 'line',
        data: [], // Will be set by setData()
        lineStyle: {
          color: s.color ?? '#3b82f6',
          width: s.width ?? 1,
        },
        itemStyle: {
          color: s.color ?? '#3b82f6',
        },
        symbol: this.type === ChartType.Scatter ? 'circle' : 'none',
        symbolSize: s.pointSize ?? 4,
        areaStyle: s.fill?.color ? { color: s.fill.color } : undefined,

        // KEY PERFORMANCE OPTIONS (ECharts large data mode)
        large: true,              // Enable GPU acceleration
        largeThreshold: 2000,     // Switch to large mode above this
        sampling: 'lttb',         // Automatic LTTB downsampling
        progressive: 5000,        // Progressive rendering chunks
        progressiveThreshold: 10000,
      })),

      // Animation settings for performance
      animation: false,           // Disable for better performance
      progressiveChunkMode: 'mod', // Better chunk distribution
    };
  }

  private setupEvents(): void {
    if (!this.chart) return;

    // Cursor/tooltip events
    this.chart.on('showTip', (params: any) => {
      if (params.dataIndex !== undefined) {
        const data = this.chart?.getOption().series?.[0]?.data?.[params.dataIndex] as number[];
        if (data) {
          this.emit('cursor-move', {
            x: data[0],
            y: data[1],
            dataIndex: params.dataIndex,
          });
        }
      }
    });

    this.chart.on('hideTip', () => {
      this.emit('cursor-leave');
    });

    // Zoom/pan events
    this.chart.on('dataZoom', () => {
      this.emit('viewport-change', this.getViewport());
    });
  }

  setData(frame: ChartDataFrame): void {
    this.currentFrame = frame;

    if (!this.chart) return;

    // Extract data for ECharts format: [[x1, y1], [x2, y2], ...]
    const xField = frame.fields.find(f =>
      f.name === 'DEPTH' || f.name === this._config.xAxis.field
    );

    if (!xField) {
      console.warn('ChartDataFrame missing X field');
      return;
    }

    // Build series data
    const seriesData = this._config.series.map((s) => {
      const yField = frame.fields.find(f => f.name === s.field);
      if (!yField) return { data: [] };

      // Convert to ECharts format efficiently
      // For large datasets, ECharts handles TypedArray well
      const data: [number, number][] = new Array(frame.length);
      for (let i = 0; i < frame.length; i++) {
        data[i] = [xField.values[i], yField.values[i]];
      }

      return { data };
    });

    // Update chart with new data
    this.chart.setOption({
      series: seriesData,
    });
  }

  setConfig(config: Partial<ChartConfig>): void {
    this._config = { ...this._config, ...config };

    // Update options without recreating
    if (this.chart) {
      const options = this.buildEChartsOptions();
      this.chart.setOption(options, { notMerge: true });

      if (this.currentFrame) {
        this.setData(this.currentFrame);
      }
    }
  }

  resize(width: number, height: number): void {
    this.chart?.resize({ width, height });
  }

  getViewport(): Viewport {
    if (!this.chart) {
      return { xMin: 0, xMax: 1, yMin: 0, yMax: 1 };
    }

    const option = this.chart.getOption();
    const xAxisOpt = option.xAxis?.[0] as any;
    const yAxisOpt = option.yAxis?.[0] as any;

    return {
      xMin: xAxisOpt?.min ?? 0,
      xMax: xAxisOpt?.max ?? 1,
      yMin: yAxisOpt?.min ?? 0,
      yMax: yAxisOpt?.max ?? 1,
    };
  }

  setViewport(viewport: Viewport): void {
    this.chart?.setOption({
      xAxis: { min: viewport.xMin, max: viewport.xMax },
      yAxis: { min: viewport.yMin, max: viewport.yMax },
    });
  }

  setCursor(position: CursorPosition | null): void {
    if (!this.chart) return;

    if (position === null) {
      this.chart.dispatchAction({ type: 'hideTip' });
    } else {
      // Convert data coordinates to pixel coordinates
      const pixelCoord = this.chart.convertToPixel('grid', [position.x, position.y]);
      this.chart.dispatchAction({
        type: 'showTip',
        x: pixelCoord[0],
        y: pixelCoord[1],
      });
    }
  }

  destroy(): void {
    if (this.chart) {
      // Disconnect from linked group
      if (this._config.interaction.linkedGroup) {
        echarts.disconnect(this._config.interaction.linkedGroup);
      }
      this.chart.dispose();
    }
    this.chart = null;
    this.container = null;
    this.eventHandlers.clear();
  }

  on<K extends keyof ChartEvents>(event: K, handler: ChartEvents[K]): void {
    if (!this.eventHandlers.has(event)) {
      this.eventHandlers.set(event, new Set());
    }
    this.eventHandlers.get(event)!.add(handler);
  }

  off<K extends keyof ChartEvents>(event: K, handler: ChartEvents[K]): void {
    this.eventHandlers.get(event)?.delete(handler);
  }

  private emit<K extends keyof ChartEvents>(event: K, ...args: Parameters<ChartEvents[K]>): void {
    this.eventHandlers.get(event)?.forEach(handler => {
      (handler as Function)(...args);
    });
  }
}
```

### 4.2 Rendering Pipeline

```

                      RENDERING PIPELINE

  ChartDataFrame

  fields: [
    { name: 'DEPTH', values: Float64Array[50000] },
    { name: 'GR', values: Float64Array[50000] }
  ]

         |
         v Transform to [[x,y], ...] format

  ECharts.setOption()

  series: [{
    data: [[x1,y1], [x2,y2], ...],  // 50K points
    large: true,                     // GPU acceleration
    sampling: 'lttb',                // Auto-downsample
  }]

         |
         v ECharts Internal

  1. LTTB downsampling (if > threshold)
  2. Progressive chunk rendering
  3. Canvas/WebGL rendering

         |
         v

  Canvas Render

  - Path drawing via Canvas 2D / WebGL
  - Tooltip overlay
  - Grid/axis labels
  - DataZoom controls


```

### 4.3 Performance Optimizations

```typescript
// src/lib/charts/performance.ts

/**
 * Performance optimizations for large datasets
 */

/**
 * Decimation strategy for zoom levels
 * When zoomed out, we don't need to render every point
 */
export function decimateData(
  data: Float64Array,
  targetPoints: number
): Float64Array {
  if (data.length <= targetPoints) {
    return data;
  }

  const step = Math.ceil(data.length / targetPoints);
  const result = new Float64Array(Math.ceil(data.length / step));

  for (let i = 0, j = 0; i < data.length; i += step, j++) {
    result[j] = data[i];
  }

  return result;
}

/**
 * Level-of-detail (LOD) decimation using min/max preservation
 * Preserves peaks and valleys for accurate visual representation
 */
export function decimateMinMax(
  xData: Float64Array,
  yData: Float64Array,
  targetPoints: number
): { x: Float64Array; y: Float64Array } {
  if (xData.length <= targetPoints) {
    return { x: xData, y: yData };
  }

  const bucketSize = Math.ceil(xData.length / (targetPoints / 2));
  const resultX: number[] = [];
  const resultY: number[] = [];

  for (let i = 0; i < xData.length; i += bucketSize) {
    const end = Math.min(i + bucketSize, xData.length);
    let minIdx = i;
    let maxIdx = i;
    let minVal = yData[i];
    let maxVal = yData[i];

    for (let j = i; j < end; j++) {
      if (yData[j] < minVal) {
        minVal = yData[j];
        minIdx = j;
      }
      if (yData[j] > maxVal) {
        maxVal = yData[j];
        maxIdx = j;
      }
    }

    // Add min and max points in order
    if (minIdx < maxIdx) {
      resultX.push(xData[minIdx], xData[maxIdx]);
      resultY.push(yData[minIdx], yData[maxIdx]);
    } else {
      resultX.push(xData[maxIdx], xData[minIdx]);
      resultY.push(yData[maxIdx], yData[minIdx]);
    }
  }

  return {
    x: new Float64Array(resultX),
    y: new Float64Array(resultY),
  };
}

/**
 * Viewport-based culling - only process visible data
 */
export function cullToViewport(
  xData: Float64Array,
  yData: Float64Array,
  xMin: number,
  xMax: number
): { x: Float64Array; y: Float64Array; startIdx: number } {
  // Binary search for start index
  let start = 0;
  let end = xData.length - 1;

  while (start < end) {
    const mid = (start + end) >>> 1;
    if (xData[mid] < xMin) {
      start = mid + 1;
    } else {
      end = mid;
    }
  }

  // Include one point before viewport for line continuity
  const startIdx = Math.max(0, start - 1);

  // Binary search for end index
  start = 0;
  end = xData.length - 1;

  while (start < end) {
    const mid = (start + end + 1) >>> 1;
    if (xData[mid] > xMax) {
      end = mid - 1;
    } else {
      start = mid;
    }
  }

  // Include one point after viewport
  const endIdx = Math.min(xData.length - 1, end + 1);

  return {
    x: xData.subarray(startIdx, endIdx + 1),
    y: yData.subarray(startIdx, endIdx + 1),
    startIdx,
  };
}
```

---

## 5. Data Optimization Patterns (deck.gl & Datashader Inspired)

This section documents best practices derived from **deck.gl** (efficient data updates, layer architecture) and **Datashader** (aggregation/rendering separation, resolution-based LOD). These patterns enable visualization of datasets with millions of points.

### 5.1 Aggregation/Rendering Separation (Datashader Pattern)

Datashader's core insight: **separate data processing from visualization**. This creates a two-stage pipeline where aggregation produces a fixed-size output regardless of input size.

```
┌─────────────────────────────────────────────────────────────────┐
│                     TWO-STAGE PIPELINE                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  STAGE 1: AGGREGATION                                           │
│  ┌─────────────┐                                                │
│  │ Raw Data    │  1M points                                     │
│  │ (TypedArray)│                                                │
│  └──────┬──────┘                                                │
│         ▼                                                       │
│  ┌─────────────┐                                                │
│  │ Aggregate   │  Canvas resolution determines output size     │
│  │ (xarray)    │  e.g., 800×600 = 480,000 bins                  │
│  └──────┬──────┘                                                │
│         │ Fixed-size output regardless of input                 │
│         ▼                                                       │
│  STAGE 2: RENDERING                                             │
│  ┌─────────────┐                                                │
│  │ tf.shade()  │  Color mapping, normalization                  │
│  │ Transfer Fn │  Multiple renderings from same aggregate       │
│  └──────┬──────┘                                                │
│         ▼                                                       │
│  ┌─────────────┐                                                │
│  │ Final Image │  Always canvas resolution                      │
│  └─────────────┘                                                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Implementation for DataForge:**

```typescript
// src/lib/charts/aggregation/aggregator.ts

/**
 * ChartAggregator - Separates aggregation from rendering
 *
 * Inspired by Datashader's two-stage pipeline:
 * 1. Aggregate raw data into fixed-size bins based on canvas resolution
 * 2. Render aggregate with transfer function (color mapping)
 */
export class ChartAggregator {
  /**
   * Aggregate data to canvas resolution
   *
   * @param frame - Raw data (potentially millions of points)
   * @param canvasWidth - Target resolution width
   * @param canvasHeight - Target resolution height
   * @param xRange - Visible x range (for viewport filtering)
   * @param yRange - Visible y range
   * @returns Fixed-size aggregate regardless of input size
   */
  aggregate(
    frame: ChartDataFrame,
    canvasWidth: number,
    canvasHeight: number,
    xRange: [number, number],
    yRange: [number, number]
  ): AggregateResult {
    const xField = frame.fields.find(f => f.name === 'DEPTH')!;
    const yField = frame.fields[1];

    // Create bin grid based on canvas resolution
    // This is the key insight: output size = canvas pixels, not data points
    const bins = new Float64Array(canvasWidth * canvasHeight);
    const counts = new Uint32Array(canvasWidth * canvasHeight);

    const [xMin, xMax] = xRange;
    const [yMin, yMax] = yRange;
    const xScale = canvasWidth / (xMax - xMin);
    const yScale = canvasHeight / (yMax - yMin);

    // Single-pass aggregation (Datashader pattern)
    for (let i = 0; i < frame.length; i++) {
      const x = xField.values[i];
      const y = yField.values[i];

      // Skip points outside viewport (range-based filtering)
      if (x < xMin || x > xMax || y < yMin || y > yMax) continue;
      if (Number.isNaN(y)) continue;

      // Map to pixel bin
      const px = Math.floor((x - xMin) * xScale);
      const py = Math.floor((y - yMin) * yScale);

      // Clamp to valid range
      const binX = Math.max(0, Math.min(canvasWidth - 1, px));
      const binY = Math.max(0, Math.min(canvasHeight - 1, py));
      const binIdx = binY * canvasWidth + binX;

      // Incremental aggregation (sum for mean calculation)
      bins[binIdx] += y;
      counts[binIdx]++;
    }

    // Compute final aggregate (mean per bin)
    const aggregate = new Float64Array(canvasWidth * canvasHeight);
    for (let i = 0; i < bins.length; i++) {
      aggregate[i] = counts[i] > 0 ? bins[i] / counts[i] : NaN;
    }

    return {
      data: aggregate,
      width: canvasWidth,
      height: canvasHeight,
      xRange,
      yRange,
      maxCount: Math.max(...counts),
    };
  }
}

interface AggregateResult {
  data: Float64Array;      // Aggregated values (canvas resolution)
  width: number;           // Canvas width
  height: number;          // Canvas height
  xRange: [number, number];
  yRange: [number, number];
  maxCount: number;        // For density visualization
}
```

**Key benefits (from Datashader):**

1. **Fixed memory footprint**: 1M points → same 480K aggregate values as 1B points
2. **Single-pass processing**: Each point processed once, O(n)
3. **Incremental updates**: Can add points without reprocessing all
4. **Reusable aggregates**: Same aggregate, different color mappings

### 5.2 Resolution-Based Level-of-Detail (Datashader Pattern)

Datashader uses **canvas dimensions to automatically determine aggregation level**. Higher resolution = finer binning, lower resolution = coarser binning.

```typescript
// src/lib/charts/aggregation/lod-manager.ts

/**
 * LODManager - Automatic Level-of-Detail based on viewport
 *
 * Inspired by Datashader's canvas-based LOD:
 * - Zoom out: Coarser bins, fewer points needed
 * - Zoom in: Finer bins, more detail visible
 */
export class LODManager {
  private cache = new Map<string, CachedLOD>();

  /**
   * Get appropriate LOD data for current viewport
   */
  getLODData(
    frame: ChartDataFrame,
    viewport: Viewport,
    canvasWidth: number
  ): LODData {
    // Calculate points per pixel
    const visibleRange = viewport.xMax - viewport.xMin;
    const totalRange = this.getTotalRange(frame);
    const visibleFraction = visibleRange / totalRange;
    const pointsInView = Math.ceil(frame.length * visibleFraction);
    const pointsPerPixel = pointsInView / canvasWidth;

    // Determine LOD level
    // If more than 2 points per pixel, we need downsampling
    if (pointsPerPixel <= 2) {
      // Full resolution - no downsampling needed
      return this.getViewportData(frame, viewport);
    }

    // Calculate target points (2 points per pixel for min/max preservation)
    const targetPoints = canvasWidth * 2;

    // Check cache
    const cacheKey = this.getCacheKey(frame.id, viewport, targetPoints);
    if (this.cache.has(cacheKey)) {
      return this.cache.get(cacheKey)!.data;
    }

    // Downsample with min/max preservation (LTTB alternative)
    const lodData = this.downsampleMinMax(frame, viewport, targetPoints);

    // Cache result
    this.cache.set(cacheKey, { data: lodData, timestamp: Date.now() });
    this.pruneCache();

    return lodData;
  }

  /**
   * Downsample preserving min/max values per bucket
   * This ensures visual fidelity even at low resolution
   */
  private downsampleMinMax(
    frame: ChartDataFrame,
    viewport: Viewport,
    targetPoints: number
  ): LODData {
    const xField = frame.fields[0];
    const yField = frame.fields[1];

    // Filter to viewport first (Datashader range filtering)
    const { startIdx, endIdx } = this.getViewportIndices(
      xField.values,
      viewport.xMin,
      viewport.xMax
    );

    const viewLength = endIdx - startIdx;
    const bucketSize = Math.ceil(viewLength / (targetPoints / 2));

    const resultX: number[] = [];
    const resultY: number[] = [];

    for (let i = startIdx; i < endIdx; i += bucketSize) {
      const bucketEnd = Math.min(i + bucketSize, endIdx);
      let minIdx = i, maxIdx = i;
      let minVal = yField.values[i], maxVal = yField.values[i];

      for (let j = i; j < bucketEnd; j++) {
        const v = yField.values[j];
        if (v < minVal) { minVal = v; minIdx = j; }
        if (v > maxVal) { maxVal = v; maxIdx = j; }
      }

      // Add points in order
      if (minIdx <= maxIdx) {
        resultX.push(xField.values[minIdx], xField.values[maxIdx]);
        resultY.push(yField.values[minIdx], yField.values[maxIdx]);
      } else {
        resultX.push(xField.values[maxIdx], xField.values[minIdx]);
        resultY.push(yField.values[maxIdx], yField.values[minIdx]);
      }
    }

    return {
      x: new Float64Array(resultX),
      y: new Float64Array(resultY),
      lodLevel: bucketSize,
      originalLength: frame.length,
      viewportLength: viewLength,
    };
  }

  // ... helper methods
}

interface LODData {
  x: Float64Array;
  y: Float64Array;
  lodLevel: number;        // Points per bucket
  originalLength: number;  // Original data length
  viewportLength: number;  // Points in viewport before downsampling
}
```

### 5.3 Efficient Data Updates (deck.gl Pattern)

deck.gl uses **shallow comparison and update triggers** to minimize unnecessary recalculations. We adapt these patterns for chart updates.

```typescript
// src/lib/charts/optimization/change-detector.ts

/**
 * ChangeDetector - O(1) change detection using shallow comparison
 *
 * Inspired by deck.gl's layer update system:
 * - Shallow comparison for props (O(1) vs O(n) deep compare)
 * - UpdateTriggers for selective recalculation
 * - Partial updates via _dataDiff
 */
export class ChangeDetector {
  private previousProps: ChartProps | null = null;
  private previousData: ChartDataFrame | null = null;

  /**
   * Detect what changed using shallow comparison
   * Returns specific change flags to enable selective updates
   */
  detectChanges(
    newProps: ChartProps,
    newData: ChartDataFrame
  ): ChangeFlags {
    const flags: ChangeFlags = {
      dataChanged: false,
      viewportChanged: false,
      styleChanged: false,
      needsRedraw: false,
      needsAggregation: false,
    };

    if (!this.previousProps || !this.previousData) {
      // First render - everything changed
      flags.dataChanged = true;
      flags.viewportChanged = true;
      flags.styleChanged = true;
      flags.needsRedraw = true;
      flags.needsAggregation = true;
    } else {
      // SHALLOW COMPARISON (deck.gl pattern)
      // O(1) - just compare references, not contents
      flags.dataChanged = newData !== this.previousData;

      // Viewport comparison
      flags.viewportChanged =
        newProps.viewport.xMin !== this.previousProps.viewport.xMin ||
        newProps.viewport.xMax !== this.previousProps.viewport.xMax ||
        newProps.viewport.yMin !== this.previousProps.viewport.yMin ||
        newProps.viewport.yMax !== this.previousProps.viewport.yMax;

      // Style comparison (color, width, etc.)
      flags.styleChanged =
        newProps.color !== this.previousProps.color ||
        newProps.lineWidth !== this.previousProps.lineWidth;

      // Determine what needs to happen
      // KEY INSIGHT (from deck.gl): Viewport changes are CHEAP
      // Only need to update transform matrix, not regenerate buffers
      flags.needsRedraw = flags.viewportChanged || flags.styleChanged;
      flags.needsAggregation = flags.dataChanged || flags.viewportChanged;
    }

    this.previousProps = newProps;
    this.previousData = newData;

    return flags;
  }

  /**
   * Compute data diff for partial updates
   *
   * Inspired by deck.gl's _dataDiff:
   * Instead of re-uploading all 1M points, only update changed rows
   */
  computeDataDiff(
    oldData: ChartDataFrame,
    newData: ChartDataFrame
  ): DataDiff | null {
    // If lengths differ, full update needed
    if (oldData.length !== newData.length) {
      return null; // Full update
    }

    // Find changed ranges (for append-only patterns like streaming)
    const oldX = oldData.fields[0].values;
    const newX = newData.fields[0].values;

    // Check if only tail changed (common streaming pattern)
    let firstDiff = -1;
    for (let i = oldData.length - 1; i >= 0; i--) {
      if (oldX[i] !== newX[i]) {
        firstDiff = i;
      } else if (firstDiff !== -1) {
        break;
      }
    }

    if (firstDiff === -1) {
      return { ranges: [] }; // No changes
    }

    // Return partial update range
    return {
      ranges: [{
        startRow: firstDiff,
        endRow: newData.length,
      }],
    };
  }
}

interface ChangeFlags {
  dataChanged: boolean;
  viewportChanged: boolean;
  styleChanged: boolean;
  needsRedraw: boolean;
  needsAggregation: boolean;
}

interface DataDiff {
  ranges: Array<{ startRow: number; endRow: number }>;
}
```

### 5.4 Viewport-Optimized Rendering (deck.gl Pattern)

deck.gl's key insight: **viewport changes are extremely cheap** because they only update GPU transformation matrices, not data buffers.

```typescript
// src/lib/charts/optimization/viewport-optimizer.ts

/**
 * ViewportOptimizer - Efficient handling of pan/zoom
 *
 * Inspired by deck.gl's view state management:
 * - Viewport changes don't regenerate buffers
 * - Only transformation matrices updated
 * - Data stays on GPU
 */
export class ViewportOptimizer {
  private gpuBuffer: GPUBufferState | null = null;

  /**
   * Handle viewport change efficiently
   *
   * From deck.gl: "Viewport changes are extremely cheap because
   * they don't require any buffer updates - only the projection
   * matrix changes"
   */
  handleViewportChange(
    newViewport: Viewport,
    chart: IChartInstance
  ): void {
    // CHEAP: Just update projection matrix
    // NO buffer regeneration, NO data re-upload
    const projectionMatrix = this.computeProjectionMatrix(newViewport);

    // Send only the 4x4 matrix to GPU
    this.updateProjectionUniform(projectionMatrix);

    // Trigger redraw with existing buffers
    chart.requestRedraw();
  }

  /**
   * Check if viewport change requires data update
   *
   * For large datasets, we may need to:
   * 1. Load more data if zoomed into uncached region
   * 2. Downsample if zoomed out significantly
   * 3. Otherwise, just update projection
   */
  requiresDataUpdate(
    oldViewport: Viewport,
    newViewport: Viewport,
    dataRange: { xMin: number; xMax: number }
  ): boolean {
    // Check if zoomed beyond cached data
    const zoomRatio =
      (oldViewport.xMax - oldViewport.xMin) /
      (newViewport.xMax - newViewport.xMin);

    // If zoomed in significantly (>4x), may need higher LOD
    if (zoomRatio > 4) return true;

    // If zoomed out significantly (>4x), may need lower LOD
    if (zoomRatio < 0.25) return true;

    // If panned outside cached region
    if (newViewport.xMin < dataRange.xMin ||
        newViewport.xMax > dataRange.xMax) {
      return true;
    }

    // Otherwise, just projection update (CHEAP)
    return false;
  }

  private computeProjectionMatrix(viewport: Viewport): Float32Array {
    // Standard orthographic projection for 2D charts
    const { xMin, xMax, yMin, yMax } = viewport;

    return new Float32Array([
      2 / (xMax - xMin), 0, 0, 0,
      0, 2 / (yMax - yMin), 0, 0,
      0, 0, 1, 0,
      -(xMax + xMin) / (xMax - xMin),
      -(yMax + yMin) / (yMax - yMin),
      0, 1,
    ]);
  }
}
```

### 5.5 Range-Based Filtering for Dynamic Updates (Datashader Pattern)

Datashader efficiently handles zoom/pan by **filtering data to viewport range before processing**.

```typescript
// src/lib/charts/optimization/range-filter.ts

/**
 * RangeFilter - Efficient data filtering for zoom/pan
 *
 * Inspired by Datashader's x_range/y_range parameters:
 * - Only data within viewport is processed
 * - Spatial indexing for O(log n) range queries
 * - Early exit when panning within cached region
 */
export class RangeFilter {
  private sortedIndex: Float64Array | null = null;

  /**
   * Filter data to viewport range
   *
   * Datashader pattern: "Points outside x_range/y_range are skipped
   * during aggregation. This enables efficient zoom/pan by processing
   * only the visible subset."
   */
  filterToRange(
    xData: Float64Array,
    yData: Float64Array,
    xRange: [number, number],
    yRange: [number, number]
  ): FilteredData {
    const [xMin, xMax] = xRange;
    const [yMin, yMax] = yRange;

    // Binary search for start/end indices (O(log n))
    const startIdx = this.binarySearchLower(xData, xMin);
    const endIdx = this.binarySearchUpper(xData, xMax);

    // Return subarray (zero-copy view into original)
    return {
      x: xData.subarray(startIdx, endIdx + 1),
      y: yData.subarray(startIdx, endIdx + 1),
      startIdx,
      endIdx,
      originalLength: xData.length,
    };
  }

  /**
   * Check if pan is within already-loaded data range
   * If yes, we can skip data fetching entirely
   */
  isPanWithinCache(
    newViewport: Viewport,
    cachedRange: { xMin: number; xMax: number }
  ): boolean {
    return (
      newViewport.xMin >= cachedRange.xMin &&
      newViewport.xMax <= cachedRange.xMax
    );
  }

  /**
   * Calculate buffer zone for prefetching
   * Prefetch extra data around viewport to enable smooth panning
   */
  getPrefetchRange(
    viewport: Viewport,
    bufferFactor: number = 0.5
  ): { xMin: number; xMax: number } {
    const visibleRange = viewport.xMax - viewport.xMin;
    const buffer = visibleRange * bufferFactor;

    return {
      xMin: viewport.xMin - buffer,
      xMax: viewport.xMax + buffer,
    };
  }

  // Binary search helpers
  private binarySearchLower(arr: Float64Array, value: number): number {
    let lo = 0, hi = arr.length;
    while (lo < hi) {
      const mid = (lo + hi) >>> 1;
      if (arr[mid] < value) lo = mid + 1;
      else hi = mid;
    }
    return Math.max(0, lo - 1); // Include one before for continuity
  }

  private binarySearchUpper(arr: Float64Array, value: number): number {
    let lo = 0, hi = arr.length;
    while (lo < hi) {
      const mid = (lo + hi) >>> 1;
      if (arr[mid] <= value) lo = mid + 1;
      else hi = mid;
    }
    return Math.min(arr.length - 1, lo); // Include one after for continuity
  }
}

interface FilteredData {
  x: Float64Array;
  y: Float64Array;
  startIdx: number;
  endIdx: number;
  originalLength: number;
}
```

### 5.6 Summary: Optimization Strategy

```
┌────────────────────────────────────────────────────────────────────┐
│                    DATA OPTIMIZATION FLOW                          │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│  User Interaction       Optimization Applied         Cost          │
│  ─────────────────      ─────────────────────        ─────         │
│                                                                    │
│  1. Load Data           Full aggregation             O(n)          │
│                         (once per dataset)                         │
│                                                                    │
│  2. Pan (small)         Projection matrix only       O(1)          │
│                         (deck.gl pattern)                          │
│                                                                    │
│  3. Pan (beyond cache)  Range filter + partial       O(log n)      │
│                         aggregation                                │
│                                                                    │
│  4. Zoom out            LOD downsampling             O(n/bucket)   │
│                         (Datashader pattern)                       │
│                                                                    │
│  5. Zoom in             Higher LOD from cache        O(1) - O(n)   │
│                         or re-aggregate                            │
│                                                                    │
│  6. Style change        Redraw only                  O(1)          │
│                         (no data processing)                       │
│                                                                    │
│  7. Data append         Partial buffer update        O(Δn)         │
│                         (deck.gl _dataDiff)                        │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

**Key Takeaways:**

1. **Separate aggregation from rendering** (Datashader): Aggregation produces fixed-size output
2. **Resolution determines LOD** (Datashader): Canvas pixels = aggregation bins
3. **Shallow comparison for change detection** (deck.gl): O(1) prop comparison
4. **Viewport changes are cheap** (deck.gl): Only projection matrix changes
5. **Range-based filtering** (Datashader): Skip data outside viewport
6. **Partial updates** (deck.gl `_dataDiff`): Only update changed data

---

## 6. Interaction Model

### 6.1 Linked Charts (Cursor Sync)

```typescript
// src/lib/charts/chart-manager.ts

import { chartRegistry } from './chart-registry';
import type { IChartInstance, ChartConfig, CursorPosition, Viewport } from './types';

/**
 * ChartManager - Coordinates multiple chart instances
 *
 * Responsibilities:
 * 1. Create and destroy chart instances
 * 2. Manage linked chart groups
 * 3. Synchronize cursors across linked charts
 * 4. Synchronize viewports for panning/zooming
 */
class ChartManagerImpl {
  /** All chart instances */
  private charts = new Map<string, IChartInstance>();

  /** Linked chart groups */
  private linkedGroups = new Map<string, Set<string>>();

  /**
   * Create a new chart instance
   */
  createChart(config: ChartConfig): IChartInstance {
    // Use registry to create appropriate adapter (ECharts, uPlot, etc.)
    const chart = chartRegistry.create(config);
    this.charts.set(config.id, chart);

    // Register in linked group if specified
    if (config.interaction.linkedGroup) {
      this.addToLinkedGroup(config.id, config.interaction.linkedGroup);
    }

    // Setup event listeners for synchronization
    this.setupChartEvents(chart);

    return chart;
  }

  /**
   * Destroy a chart instance
   */
  destroyChart(chartId: string): void {
    const chart = this.charts.get(chartId);
    if (!chart) return;

    // Remove from linked group
    if (chart.config.interaction.linkedGroup) {
      this.removeFromLinkedGroup(chartId, chart.config.interaction.linkedGroup);
    }

    chart.destroy();
    this.charts.delete(chartId);
  }

  /**
   * Get chart by ID
   */
  getChart(chartId: string): IChartInstance | undefined {
    return this.charts.get(chartId);
  }

  /**
   * Add chart to linked group
   */
  private addToLinkedGroup(chartId: string, groupKey: string): void {
    if (!this.linkedGroups.has(groupKey)) {
      this.linkedGroups.set(groupKey, new Set());
    }
    this.linkedGroups.get(groupKey)!.add(chartId);
  }

  /**
   * Remove chart from linked group
   */
  private removeFromLinkedGroup(chartId: string, groupKey: string): void {
    this.linkedGroups.get(groupKey)?.delete(chartId);
    if (this.linkedGroups.get(groupKey)?.size === 0) {
      this.linkedGroups.delete(groupKey);
    }
  }

  /**
   * Setup event listeners for chart synchronization
   */
  private setupChartEvents(chart: IChartInstance): void {
    // Cursor synchronization
    chart.on('cursor-move', (position: CursorPosition) => {
      this.syncCursor(chart.id, position);
    });

    chart.on('cursor-leave', () => {
      this.syncCursor(chart.id, null);
    });

    // Viewport synchronization (optional - for linked zoom/pan)
    chart.on('viewport-change', (viewport: Viewport) => {
      // Only sync if explicitly enabled
      if (chart.config.interaction.linkedGroup) {
        this.syncViewport(chart.id, viewport);
      }
    });
  }

  /**
   * Synchronize cursor across linked charts
   */
  private syncCursor(sourceChartId: string, position: CursorPosition | null): void {
    const sourceChart = this.charts.get(sourceChartId);
    if (!sourceChart?.config.interaction.linkedGroup) return;

    const groupKey = sourceChart.config.interaction.linkedGroup;
    const linkedChartIds = this.linkedGroups.get(groupKey);
    if (!linkedChartIds) return;

    // Update cursor on all other charts in the group
    for (const chartId of linkedChartIds) {
      if (chartId === sourceChartId) continue;

      const chart = this.charts.get(chartId);
      chart?.setCursor(position);
    }
  }

  /**
   * Synchronize viewport across linked charts
   */
  private syncViewport(sourceChartId: string, viewport: Viewport): void {
    const sourceChart = this.charts.get(sourceChartId);
    if (!sourceChart?.config.interaction.linkedGroup) return;

    const groupKey = sourceChart.config.interaction.linkedGroup;
    const linkedChartIds = this.linkedGroups.get(groupKey);
    if (!linkedChartIds) return;

    // Update viewport on all other charts in the group
    for (const chartId of linkedChartIds) {
      if (chartId === sourceChartId) continue;

      const chart = this.charts.get(chartId);
      if (chart) {
        // Only sync X axis (depth) for well log correlation
        chart.setViewport({
          xMin: viewport.xMin,
          xMax: viewport.xMax,
          yMin: chart.getViewport().yMin,
          yMax: chart.getViewport().yMax,
        });
      }
    }
  }

  /**
   * Notify charts of data changes
   */
  notifyDataChange(curveId: string): void {
    // Find charts displaying this curve and refresh them
    for (const chart of this.charts.values()) {
      const displaysCurve = chart.config.series.some(
        s => s.field === curveId
      );
      if (displaysCurve) {
        // Re-fetch and update data
        // (Implementation depends on how data binding works)
      }
    }
  }

  /**
   * Notify charts of new computation result
   */
  notifyNewResult(executionId: string): void {
    // Charts can subscribe to new results if they want to auto-display
    // (Implementation depends on UI requirements)
  }
}

export const chartManager = new ChartManagerImpl();
```

### 6.2 Selection and Crosshair

```typescript
// src/lib/charts/interactions.ts

import type { Selection, CursorPosition, ChartDataFrame } from './types';

/**
 * Cursor tracker for linked chart crosshairs
 */
export class CursorTracker {
  private position: CursorPosition | null = null;
  private listeners = new Set<(pos: CursorPosition | null) => void>();

  setPosition(pos: CursorPosition | null): void {
    this.position = pos;
    this.notify();
  }

  getPosition(): CursorPosition | null {
    return this.position;
  }

  subscribe(listener: (pos: CursorPosition | null) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  private notify(): void {
    for (const listener of this.listeners) {
      listener(this.position);
    }
  }
}

/**
 * Selection manager for point/range selection
 */
export class SelectionManager {
  private selection: Selection | null = null;
  private listeners = new Set<(sel: Selection | null) => void>();

  setSelection(sel: Selection | null): void {
    this.selection = sel;
    this.notify();
  }

  getSelection(): Selection | null {
    return this.selection;
  }

  /**
   * Get data points within the current selection
   */
  getSelectedData(frame: ChartDataFrame): { indices: number[]; values: number[] } {
    if (!this.selection) {
      return { indices: [], values: [] };
    }

    const xField = frame.fields[0];
    const yField = frame.fields[1];

    if (!xField || !yField) {
      return { indices: [], values: [] };
    }

    const indices: number[] = [];
    const values: number[] = [];

    if (this.selection.type === 'range') {
      const { xRange, yRange } = this.selection;

      for (let i = 0; i < frame.length; i++) {
        const x = xField.values[i];
        const y = yField.values[i];

        const inXRange = !xRange || (x >= xRange.min && x <= xRange.max);
        const inYRange = !yRange || (y >= yRange.min && y <= yRange.max);

        if (inXRange && inYRange) {
          indices.push(i);
          values.push(y);
        }
      }
    } else if (this.selection.type === 'points' && this.selection.points) {
      for (const idx of this.selection.points) {
        indices.push(idx);
        values.push(yField.values[idx]);
      }
    }

    return { indices, values };
  }

  subscribe(listener: (sel: Selection | null) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  private notify(): void {
    for (const listener of this.listeners) {
      listener(this.selection);
    }
  }
}

export const cursorTracker = new CursorTracker();
export const selectionManager = new SelectionManager();
```

---

## 7. Extensibility Model

### 7.1 Chart Type Registry

```typescript
// src/lib/charts/chart-registry.ts

import type { ChartType, ChartConfig, IChartInstance } from './types';
import { EChartsAdapter } from './echarts-adapter';

/**
 * ChartFactory - Creates chart instances based on type
 */
type ChartFactory = (config: ChartConfig) => IChartInstance;

/**
 * ChartRegistry - Extensible registry for chart types
 *
 * Allows adding new chart types without modifying core code.
 * The adapter pattern enables switching chart libraries by
 * registering different adapters.
 */
class ChartRegistryImpl {
  private factories = new Map<ChartType, ChartFactory>();

  constructor() {
    // Register built-in chart types using ECharts adapter
    this.register(ChartType.Line, (config) => new EChartsAdapter(config.id, config));
    this.register(ChartType.Scatter, (config) => new EChartsAdapter(config.id, config));
    this.register(ChartType.WellLog, (config) => new EChartsAdapter(config.id, config));
    // Histogram uses custom implementation (see section 7.2)
  }

  /**
   * Register a chart type factory
   */
  register(type: ChartType, factory: ChartFactory): void {
    this.factories.set(type, factory);
  }

  /**
   * Create a chart instance
   */
  create(config: ChartConfig): IChartInstance {
    const factory = this.factories.get(config.type);
    if (!factory) {
      throw new Error(`Unknown chart type: ${config.type}`);
    }
    return factory(config);
  }

  /**
   * Check if a chart type is supported
   */
  isSupported(type: ChartType): boolean {
    return this.factories.has(type);
  }

  /**
   * Get all supported chart types
   */
  getSupportedTypes(): ChartType[] {
    return Array.from(this.factories.keys());
  }
}

export const chartRegistry = new ChartRegistryImpl();
```

### 7.2 Custom Chart Implementation Example

```typescript
// src/lib/charts/custom/histogram-chart.ts

import type { IChartInstance, ChartConfig, ChartDataFrame, Viewport, CursorPosition, ChartEvents } from '../types';

/**
 * HistogramChart - Custom chart implementation for histogram visualization
 *
 * Example of extending the chart system with a new chart type
 */
export class HistogramChart implements IChartInstance {
  readonly id: string;
  readonly type = ChartType.Histogram;

  private _config: ChartConfig;
  private container: HTMLElement | null = null;
  private canvas: HTMLCanvasElement | null = null;
  private ctx: CanvasRenderingContext2D | null = null;
  private bins: { start: number; end: number; count: number }[] = [];

  constructor(id: string, config: ChartConfig) {
    this.id = id;
    this._config = config;
  }

  get config(): ChartConfig {
    return this._config;
  }

  mount(container: HTMLElement): void {
    this.container = container;

    this.canvas = document.createElement('canvas');
    this.canvas.style.width = '100%';
    this.canvas.style.height = '100%';
    container.appendChild(this.canvas);

    this.ctx = this.canvas.getContext('2d');
    this.resize(container.clientWidth, container.clientHeight);
  }

  setData(frame: ChartDataFrame): void {
    // Compute histogram bins
    const valueField = frame.fields.find(f =>
      f.name !== 'DEPTH' && f.name !== 'TIME'
    );

    if (!valueField) return;

    const values = valueField.values;
    const numBins = 50;

    // Find min/max
    let min = Infinity;
    let max = -Infinity;
    for (let i = 0; i < values.length; i++) {
      const v = values[i];
      if (!Number.isNaN(v)) {
        min = Math.min(min, v);
        max = Math.max(max, v);
      }
    }

    if (!Number.isFinite(min) || !Number.isFinite(max)) return;

    // Create bins
    const binWidth = (max - min) / numBins;
    this.bins = [];

    for (let i = 0; i < numBins; i++) {
      this.bins.push({
        start: min + i * binWidth,
        end: min + (i + 1) * binWidth,
        count: 0,
      });
    }

    // Count values in bins
    for (let i = 0; i < values.length; i++) {
      const v = values[i];
      if (Number.isNaN(v)) continue;

      const binIdx = Math.min(
        Math.floor((v - min) / binWidth),
        numBins - 1
      );
      this.bins[binIdx].count++;
    }

    this.render();
  }

  private render(): void {
    if (!this.ctx || !this.canvas) return;

    const { width, height } = this.canvas;
    this.ctx.clearRect(0, 0, width, height);

    if (this.bins.length === 0) return;

    // Find max count for scaling
    const maxCount = Math.max(...this.bins.map(b => b.count));

    // Draw bars
    const barWidth = width / this.bins.length;
    const color = this._config.series[0]?.color ?? '#3b82f6';

    this.ctx.fillStyle = color;

    for (let i = 0; i < this.bins.length; i++) {
      const bin = this.bins[i];
      const barHeight = (bin.count / maxCount) * (height - 40);

      this.ctx.fillRect(
        i * barWidth + 1,
        height - 20 - barHeight,
        barWidth - 2,
        barHeight
      );
    }

    // Draw axes labels (simplified)
    this.ctx.fillStyle = '#666';
    this.ctx.font = '12px sans-serif';
    this.ctx.fillText(
      this.bins[0].start.toFixed(2),
      5,
      height - 5
    );
    this.ctx.fillText(
      this.bins[this.bins.length - 1].end.toFixed(2),
      width - 50,
      height - 5
    );
  }

  setConfig(config: Partial<ChartConfig>): void {
    this._config = { ...this._config, ...config };
    this.render();
  }

  resize(width: number, height: number): void {
    if (!this.canvas) return;
    this.canvas.width = width;
    this.canvas.height = height;
    this.render();
  }

  getViewport(): Viewport {
    return { xMin: 0, xMax: 1, yMin: 0, yMax: 1 };
  }

  setViewport(_viewport: Viewport): void {
    // Histograms don't have zoomable viewport
  }

  setCursor(_position: CursorPosition | null): void {
    // Histograms don't show linked cursor
  }

  destroy(): void {
    this.canvas?.remove();
    this.canvas = null;
    this.ctx = null;
    this.container = null;
  }

  on<K extends keyof ChartEvents>(_event: K, _handler: ChartEvents[K]): void {
    // Event handling implementation
  }

  off<K extends keyof ChartEvents>(_event: K, _handler: ChartEvents[K]): void {
    // Event handling implementation
  }
}

// Register with chart registry
import { chartRegistry } from '../chart-registry';
chartRegistry.register(ChartType.Histogram, (config) => new HistogramChart(config.id, config));
```

---

## 8. Error Handling & Robustness

### 8.1 Data Validation

```typescript
// src/lib/charts/validation.ts

import type { ChartDataFrame, ChartConfig } from './types';

/**
 * Validate ChartDataFrame before rendering
 */
export function validateDataFrame(frame: ChartDataFrame): ValidationResult {
  const errors: string[] = [];
  const warnings: string[] = [];

  // Check for empty data
  if (frame.length === 0) {
    warnings.push('Data frame is empty');
  }

  // Check for required fields
  if (frame.fields.length < 2) {
    errors.push('Data frame must have at least 2 fields (X and Y)');
  }

  // Check field lengths match
  const expectedLength = frame.length;
  for (const field of frame.fields) {
    if (field.values.length !== expectedLength) {
      errors.push(`Field '${field.name}' has ${field.values.length} values, expected ${expectedLength}`);
    }
  }

  // Check for NaN/Infinity in X axis
  const xField = frame.fields[0];
  if (xField) {
    let nanCount = 0;
    let infCount = 0;
    for (let i = 0; i < xField.values.length; i++) {
      const v = xField.values[i];
      if (Number.isNaN(v)) nanCount++;
      if (!Number.isFinite(v)) infCount++;
    }
    if (nanCount > 0) {
      warnings.push(`X axis contains ${nanCount} NaN values`);
    }
    if (infCount > 0) {
      errors.push(`X axis contains ${infCount} non-finite values`);
    }
  }

  // Check X axis is sorted
  if (xField && xField.values.length > 1) {
    let isSorted = true;
    for (let i = 1; i < xField.values.length; i++) {
      if (xField.values[i] < xField.values[i - 1]) {
        isSorted = false;
        break;
      }
    }
    if (!isSorted) {
      warnings.push('X axis is not sorted - may cause rendering issues');
    }
  }

  return {
    valid: errors.length === 0,
    errors,
    warnings,
  };
}

export interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

/**
 * Validate chart configuration
 */
export function validateConfig(config: ChartConfig): ValidationResult {
  const errors: string[] = [];
  const warnings: string[] = [];

  // Check required fields
  if (!config.id) {
    errors.push('Chart ID is required');
  }

  if (!config.xAxis?.field) {
    errors.push('X axis field is required');
  }

  if (!config.series || config.series.length === 0) {
    errors.push('At least one series is required');
  }

  // Check for duplicate series
  const seriesFields = new Set<string>();
  for (const series of config.series ?? []) {
    if (seriesFields.has(series.field)) {
      warnings.push(`Duplicate series for field '${series.field}'`);
    }
    seriesFields.add(series.field);
  }

  return {
    valid: errors.length === 0,
    errors,
    warnings,
  };
}
```

### 8.2 Error Boundaries

```svelte
<!-- src/lib/charts/components/ChartErrorBoundary.svelte -->

<script lang="ts">
  import { onMount, onDestroy } from 'svelte';

  interface Props {
    chartId: string;
    children: import('svelte').Snippet;
  }

  let { chartId, children }: Props = $props();

  let hasError = $state(false);
  let errorMessage = $state('');

  function handleError(event: ErrorEvent) {
    // Check if error is from our chart
    if (event.message.includes(chartId) || event.message.includes('uPlot')) {
      hasError = true;
      errorMessage = event.message;
      event.preventDefault();
    }
  }

  onMount(() => {
    window.addEventListener('error', handleError);
  });

  onDestroy(() => {
    window.removeEventListener('error', handleError);
  });

  function retry() {
    hasError = false;
    errorMessage = '';
  }
</script>

{#if hasError}
  <div class="chart-error">
    <div class="error-icon">!</div>
    <h3>Chart Error</h3>
    <p>{errorMessage}</p>
    <button onclick={retry}>Retry</button>
  </div>
{:else}
  {@render children()}
{/if}

<style>
  .chart-error {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 8px;
    padding: 1rem;
    text-align: center;
  }

  .error-icon {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    background: #ef4444;
    color: white;
    font-size: 24px;
    font-weight: bold;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 1rem;
  }

  h3 {
    color: #991b1b;
    margin: 0 0 0.5rem 0;
  }

  p {
    color: #7f1d1d;
    font-size: 0.875rem;
    margin: 0 0 1rem 0;
  }

  button {
    background: #ef4444;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
  }

  button:hover {
    background: #dc2626;
  }
</style>
```

### 8.3 Graceful Degradation

```typescript
// src/lib/charts/fallback.ts

/**
 * Fallback rendering for when WebGL is unavailable
 */
export function createFallbackChart(
  container: HTMLElement,
  frame: ChartDataFrame,
  config: ChartConfig
): void {
  // Create simple HTML table as fallback
  const table = document.createElement('table');
  table.className = 'chart-fallback-table';

  // Header row
  const thead = document.createElement('thead');
  const headerRow = document.createElement('tr');

  for (const field of frame.fields) {
    const th = document.createElement('th');
    th.textContent = field.config?.displayName ?? field.name;
    headerRow.appendChild(th);
  }

  thead.appendChild(headerRow);
  table.appendChild(thead);

  // Data rows (limited to first 100 for performance)
  const tbody = document.createElement('tbody');
  const maxRows = Math.min(frame.length, 100);

  for (let i = 0; i < maxRows; i++) {
    const row = document.createElement('tr');

    for (const field of frame.fields) {
      const td = document.createElement('td');
      const value = field.values[i];
      td.textContent = Number.isNaN(value) ? '-' : value.toFixed(4);
      row.appendChild(td);
    }

    tbody.appendChild(row);
  }

  table.appendChild(tbody);

  // Add message if data was truncated
  if (frame.length > maxRows) {
    const footer = document.createElement('tfoot');
    const footerRow = document.createElement('tr');
    const footerCell = document.createElement('td');
    footerCell.colSpan = frame.fields.length;
    footerCell.textContent = `Showing ${maxRows} of ${frame.length} rows (WebGL unavailable)`;
    footerRow.appendChild(footerCell);
    footer.appendChild(footerRow);
    table.appendChild(footer);
  }

  container.innerHTML = '';
  container.appendChild(table);
}

/**
 * Check if WebGL is available
 */
export function isWebGLAvailable(): boolean {
  try {
    const canvas = document.createElement('canvas');
    const gl = canvas.getContext('webgl') || canvas.getContext('experimental-webgl');
    return gl !== null;
  } catch {
    return false;
  }
}
```

---

## 9. End-to-End Flow Example

### 9.1 Scenario: Execute UDF and Display Result

```

  SCENARIO: User executes Moving Average UDF, result displays in chart

  Timeline:

  T0: User clicks "Execute" on Moving Average UDF
      Input: GR curve (50,000 points)
      Parameters: window = 5

  T1: Frontend sends execute_udf command to Rust
      invoke('execute_udf', { providerId, udfId, parameters })

  T2: Rust UDF Executor
      -> Loads GR curve from parquet (50K points)
      -> Executes moving_average algorithm
      -> Returns ExecutionResult { success, data: [...] }

  T3: Frontend receives result
      executionResult.set({ success: true, data: [...] })

  T4: chartDataStore.loadComputedResult()
      -> Converts data.depth[] to Float64Array
      -> Converts data.value[] to Float64Array
      -> Creates ChartDataFrame
      -> Caches in frames Map

  T5: Chart component receives data

      <LineChart
        config={chartConfig}
        dataFrameId="computed:exec_123"
      />

      -> Retrieves ChartDataFrame from store
      -> Calls chart.setData(frame)

  T6: uPlot renders
      -> Receives Float64Array references (zero-copy)
      -> Uploads to GPU via WebGL
      -> Draws 50K points in ~20ms

  T7: UI is responsive
      -> User can pan/zoom immediately
      -> Cursor crosshair moves smoothly
      -> No frame drops or freezing

  TOTAL TIME: ~150ms (vs 2000ms+ with SVG)

```

### 9.2 Component Implementation

```svelte
<!-- src/lib/components/ExecutionResult.svelte -->

<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { executionResult, isSaving, saveOutputCurve } from '$lib/stores/compute';
  import { chartDataStore } from '$lib/charts/chart-data-store';
  import { chartManager } from '$lib/charts/chart-manager';
  import { ChartType } from '$lib/charts/types';
  import type { IChartInstance, ChartConfig } from '$lib/charts/types';
  import DataGrid from './DataGrid.svelte';

  let activeTab = $state<'table' | 'chart'>('chart');
  let chartContainer = $state<HTMLElement | null>(null);
  let chartInstance: IChartInstance | null = null;

  // Chart configuration
  const chartConfig: ChartConfig = {
    id: 'execution-result-chart',
    type: ChartType.Line,
    title: 'Computation Result',
    xAxis: {
      field: 'DEPTH',
      label: 'Depth (m)',
      inverted: true, // Well log convention
    },
    yAxes: [{
      field: 'value',
      label: 'Value',
    }],
    series: [{
      field: 'value',
      label: 'Result',
      color: '#22c55e',
      width: 1,
    }],
    interaction: {
      pan: true,
      zoom: true,
      cursor: true,
    },
  };

  // Watch for execution result changes
  $effect(() => {
    const result = $executionResult;

    if (result?.success && result.data && chartContainer) {
      // Load data into chart data store
      const frame = chartDataStore.loadComputedResult(
        result.executionId ?? 'latest',
        result.outputMnemonic ?? 'Result',
        result.data.map(d => d.depth),
        result.data.map(d => d.value)
      );

      // Update chart
      if (chartInstance) {
        chartInstance.setData(frame);
      }
    }
  });

  // Initialize chart when container is ready
  $effect(() => {
    if (chartContainer && activeTab === 'chart' && !chartInstance) {
      chartInstance = chartManager.createChart(chartConfig);
      chartInstance.mount(chartContainer);

      // If we have data, render it
      const result = $executionResult;
      if (result?.success && result.data) {
        const frame = chartDataStore.getFrame(`computed:${result.executionId ?? 'latest'}`);
        if (frame) {
          chartInstance.setData(frame);
        }
      }
    }
  });

  // Cleanup on destroy
  onDestroy(() => {
    if (chartInstance) {
      chartManager.destroyChart(chartInstance.id);
      chartInstance = null;
    }
  });

  // Handle resize
  function handleResize() {
    if (chartInstance && chartContainer) {
      chartInstance.resize(
        chartContainer.clientWidth,
        chartContainer.clientHeight
      );
    }
  }

  // Prepare table data
  const tableData = $derived(
    $executionResult?.data?.map((d, i) => ({
      index: i + 1,
      depth: d.depth,
      value: d.value,
    })) ?? []
  );

  const columnDefs = [
    { field: 'index', headerName: '#', width: 60 },
    { field: 'depth', headerName: 'Depth (m)', width: 120, valueFormatter: (p: any) => p.value?.toFixed(2) },
    { field: 'value', headerName: 'Value', width: 120, valueFormatter: (p: any) => p.value?.toFixed(4) ?? '-' },
  ];
</script>

<svelte:window onresize={handleResize} />

<div class="execution-result">
  {#if $executionResult?.success}
    <div class="tabs">
      <button
        class:active={activeTab === 'chart'}
        onclick={() => activeTab = 'chart'}
      >
        Chart
      </button>
      <button
        class:active={activeTab === 'table'}
        onclick={() => activeTab = 'table'}
      >
        Table ({tableData.length} rows)
      </button>

      <div class="spacer"></div>

      <button
        class="save-btn"
        onclick={() => saveOutputCurve()}
        disabled={$isSaving}
      >
        {#if $isSaving}
          Saving...
        {:else}
          Save to DataForge
        {/if}
      </button>
    </div>

    <div class="content">
      {#if activeTab === 'chart'}
        <div class="chart-container" bind:this={chartContainer}></div>
      {:else}
        <DataGrid rowData={tableData} {columnDefs} />
      {/if}
    </div>
  {:else if $executionResult?.error}
    <div class="error">
      <h3>Execution Failed</h3>
      <p>{$executionResult.error}</p>
    </div>
  {:else}
    <div class="empty">
      <p>Execute a computation to see results</p>
    </div>
  {/if}
</div>

<style>
  .execution-result {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-secondary);
    border-radius: 8px;
    overflow: hidden;
  }

  .tabs {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem;
    background: var(--bg-tertiary);
    border-bottom: 1px solid var(--border);
  }

  .tabs button {
    padding: 0.5rem 1rem;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    border-radius: 4px;
  }

  .tabs button:hover {
    background: var(--bg-hover);
  }

  .tabs button.active {
    background: var(--bg-primary);
    color: var(--text-primary);
  }

  .spacer {
    flex: 1;
  }

  .save-btn {
    background: var(--accent) !important;
    color: white !important;
  }

  .save-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .content {
    flex: 1;
    overflow: hidden;
  }

  .chart-container {
    width: 100%;
    height: 100%;
  }

  .error, .empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-secondary);
  }

  .error {
    color: var(--error);
  }
</style>
```

---

## 10. Design Tradeoffs & Rejected Alternatives

### 10.1 Tradeoffs Made

| Decision | Tradeoff | Rationale |
|----------|----------|-----------|
| **ECharts over uPlot** | Larger bundle size | Better suited for depth/curve displays; rich chart types; tree-shakeable |
| **Adapter pattern** | Additional abstraction layer | Enables library switching without component changes |
| **TypedArrays required** | API complexity | Zero-copy GPU transfer is essential for performance |
| **Aggregation/rendering separation** | More complex pipeline | Enables million-point datasets with fixed memory (Datashader pattern) |
| **Columnar data frames** | Memory layout constraint | GPU-optimal, matches Arrow format |
| **Linked cursor via manager** | Centralized coordination | Simpler than distributed coordination |
| **Resolution-based LOD** | Automatic quality tradeoff | Canvas size determines aggregation level automatically |

### 10.2 Rejected Alternatives

#### Alternative 1: SVG with Virtual DOM (Current Approach)

**Rejected because:**
- O(n) DOM operations for n data points
- Browser layout engine becomes bottleneck at 10K+ points
- Memory overhead of DOM nodes
- No GPU acceleration

**Measurements:**
- 10K points: 200ms render time
- 50K points: 1000ms+ render time, UI freezes
- 100K points: Unusable

#### Alternative 2: Canvas 2D (No WebGL)

**Rejected because:**
- CPU-bound rendering
- No GPU vertex batching
- Manual anti-aliasing needed
- Limited line quality at high zoom

**When to use:** Fallback when WebGL unavailable

#### Alternative 3: D3.js with Canvas

**Rejected because:**
- D3 designed for DOM manipulation
- Canvas mode is an afterthought
- No built-in linked cursors
- Higher bundle size for less performance

#### Alternative 4: SciChart (Commercial)

**Considered but rejected because:**
- Commercial license cost
- ~500KB bundle size
- Excellent performance but overkill for our needs
- uPlot provides 90% of features at 10% size

**When to reconsider:** If we need real-time streaming (>1M points/sec)

#### Alternative 5: WebGPU (Future)

**Not chosen because:**
- Browser support still limited (Chrome only)
- API still evolving
- uPlot WebGL is mature and stable

**When to reconsider:** 2025+ when Safari/Firefox support WebGPU

### 10.3 Implementation Priorities

1. **Phase 1: Core Rendering** (This document)
   - ECharts integration with adapter pattern
   - ChartDataFrame abstraction
   - Basic line/scatter charts
   - Performance validation with 100K points
   - Data optimization patterns (LOD, aggregation)

2. **Phase 2: Interaction**
   - Linked cursors across charts (ECharts connect API)
   - Pan/zoom synchronization
   - Selection and highlighting
   - Viewport-optimized rendering

3. **Phase 3: Chart Types**
   - Histogram
   - Crossplot
   - Well log tracks (multi-curve)
   - Heatmap (for crossplot density)

4. **Phase 4: Polish**
   - Theming (dark/light mode)
   - Export (PNG, SVG)
   - Annotations
   - Print layout

5. **Phase 5: Advanced Optimization**
   - WebWorker aggregation for >1M points
   - Streaming data support
   - Progressive loading

---

## 11. References

### Internal Documentation
- [DFC-model-view-sync.md](./DFC-model-view-sync.md) - Multi-view synchronization architecture
- [Grafana DataFrame](../../../docs/context7/grafana/grafana-architecture-unit-of-truth.md) - Data frame design inspiration

### Charting Libraries
- [ECharts Documentation](https://echarts.apache.org/en/index.html) - Primary charting library
- [ECharts Large Data](https://echarts.apache.org/handbook/en/how-to/bindData/binddata/) - Large dataset handling
- [uPlot Documentation](https://github.com/leeoniya/uPlot) - Alternative lightweight library

### Optimization Patterns (External Inspiration)
- [deck.gl Efficient Data Updates](https://deck.gl/docs/developer-guide/performance) - Shallow comparison, updateTriggers, _dataDiff
- [deck.gl Layer Architecture](https://deck.gl/docs/developer-guide/custom-layers) - AttributeManager, accessor patterns
- [deck.gl View State](https://deck.gl/docs/developer-guide/views) - Viewport optimization patterns
- [Datashader Pipeline](https://datashader.org/getting_started/Pipeline.html) - Aggregation/rendering separation
- [Datashader Performance](https://datashader.org/user_guide/Performance.html) - Resolution-based LOD, range filtering

### Data Formats
- [Arrow Columnar Format](https://arrow.apache.org/docs/format/Columnar.html) - Memory layout reference

---

## 12. Windowing & Pane System Plan

This section defines the architecture for a dynamic multi-pane workspace where users can add, remove, resize, and organize multiple charts. The design draws from proven patterns in VS Code, JupyterLab, GoldenLayout, Lumino, and Eclipse Theia.

### 12.1 Architectural Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         PANE SYSTEM ARCHITECTURE                         │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │                        WorkspaceManager                             │ │
│  │  - Single source of truth for layout state                         │ │
│  │  - Emits events on layout changes                                  │ │
│  │  - Handles persistence/restoration                                  │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                                    │                                     │
│                                    ▼                                     │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │                        PaneLayoutEngine                             │ │
│  │  - Tree-based layout structure (splits, tabs, panes)              │ │
│  │  - Resize calculations and constraint handling                     │ │
│  │  - Drag-and-drop coordination                                      │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                                    │                                     │
│            ┌───────────────────────┼───────────────────────┐            │
│            ▼                       ▼                       ▼            │
│  ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐      │
│  │   SplitContainer │  │   TabContainer   │  │   PaneContainer  │      │
│  │   (horizontal/   │  │   (tabbed views) │  │   (single pane)  │      │
│  │   vertical)      │  │                  │  │                  │      │
│  └──────────────────┘  └──────────────────┘  └──────────────────┘      │
│                                    │                                     │
│                                    ▼                                     │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │                          PaneInstance                               │ │
│  │  - Wraps chart/view content                                        │ │
│  │  - Lifecycle management (mount/unmount)                            │ │
│  │  - Title, toolbar, state                                           │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                                    │                                     │
│                                    ▼                                     │
│  ┌────────────────────────────────────────────────────────────────────┐ │
│  │                         Chart Instance                              │ │
│  │  (EChartsAdapter / HistogramChart / etc.)                          │ │
│  └────────────────────────────────────────────────────────────────────┘ │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 12.2 Core Design Principles

Drawing from VS Code, JupyterLab, and Lumino architectures:

| Principle | Source | Application |
|-----------|--------|-------------|
| **Model-View Separation** | VS Code, JupyterLab | Pane layout model is separate from DOM rendering |
| **Single Source of Truth** | VS Code IModelService | WorkspaceManager owns all layout state |
| **Event-Driven Updates** | Lumino MessageLoop | State changes propagate via events, not direct coupling |
| **Factory Pattern** | JupyterLab WidgetFactory | PaneFactory creates pane instances |
| **Reference Counting** | VS Code ModelService | Shared chart instances are reference-counted |
| **Tree Layout Structure** | GoldenLayout, Lumino | Hierarchical split/tab/pane tree |
| **Shallow Comparison** | deck.gl | O(1) change detection for layout updates |

### 12.3 Layout Model (Data Layer)

```typescript
// src/lib/panes/layout-model.ts

/**
 * Layout node types - forms a tree structure
 * Inspired by GoldenLayout's ContentItem hierarchy
 */
export type LayoutNode = SplitNode | TabNode | PaneNode;

/**
 * SplitNode - Divides space between children (horizontal or vertical)
 * Inspired by VS Code SplitView and Lumino SplitPanel
 */
export interface SplitNode {
  type: 'split';
  id: string;
  orientation: 'horizontal' | 'vertical';
  children: LayoutNode[];
  /** Relative sizes of children (sum to 1.0) */
  sizes: number[];
}

/**
 * TabNode - Contains multiple panes in tabs
 * Inspired by VS Code EditorGroup and GoldenLayout Stack
 */
export interface TabNode {
  type: 'tab';
  id: string;
  children: PaneNode[];
  activeIndex: number;
}

/**
 * PaneNode - Leaf node containing a single pane
 * Inspired by JupyterLab's Widget and Lumino's ContentItem
 */
export interface PaneNode {
  type: 'pane';
  id: string;
  /** Pane type determines which factory creates it */
  paneType: PaneType;
  /** Pane-specific configuration */
  config: PaneConfig;
  /** Display title */
  title: string;
}

export enum PaneType {
  LineChart = 'line-chart',
  ScatterChart = 'scatter-chart',
  Histogram = 'histogram',
  CrossPlot = 'crossplot',
  WellLog = 'welllog',
  DataGrid = 'data-grid',
  Empty = 'empty',
}

export interface PaneConfig {
  /** Data binding - which data frame to display */
  dataFrameId?: string;
  /** Chart-specific configuration */
  chartConfig?: ChartConfig;
  /** Linked group for cursor/viewport sync */
  linkedGroup?: string;
}

/**
 * Complete workspace layout state
 * Inspired by GoldenLayout's ILayoutConfig
 */
export interface WorkspaceLayout {
  /** Root of the layout tree */
  root: LayoutNode;
  /** Version for migration support */
  version: number;
  /** Active pane ID */
  activePaneId?: string;
}
```

### 12.4 Workspace Manager (State Management)

```typescript
// src/lib/panes/workspace-manager.ts

import { writable, derived, get } from 'svelte/store';
import type { WorkspaceLayout, LayoutNode, PaneNode, PaneType, PaneConfig } from './layout-model';
import { generateId } from '$lib/utils/id';

/**
 * WorkspaceEvent - All layout changes are communicated via events
 * Inspired by Theia's event-driven architecture and Lumino MessageLoop
 */
export type WorkspaceEvent =
  | { type: 'pane-added'; paneId: string; paneNode: PaneNode }
  | { type: 'pane-removed'; paneId: string }
  | { type: 'pane-moved'; paneId: string; newLocation: string }
  | { type: 'layout-changed'; layout: WorkspaceLayout }
  | { type: 'pane-activated'; paneId: string }
  | { type: 'split-resized'; splitId: string; sizes: number[] };

/**
 * WorkspaceManager - Single source of truth for workspace layout
 *
 * Design patterns used:
 * - VS Code ModelService: Single source of truth for layout state
 * - JupyterLab LayoutRestorer: Persistence and restoration
 * - Lumino DockPanel: Tree-based layout structure
 * - GoldenLayout EventEmitter: Event-driven updates
 */
class WorkspaceManagerImpl {
  /** Current layout state */
  private _layout = writable<WorkspaceLayout>(this.createDefaultLayout());

  /** Event emitter for layout changes */
  private _events = writable<WorkspaceEvent | null>(null);

  /** Pane instance registry (for reference counting) */
  private _paneInstances = new Map<string, { instance: unknown; refCount: number }>();

  /** Expose layout as readable store */
  get layout() {
    return { subscribe: this._layout.subscribe };
  }

  /** Expose events for subscribers */
  get events() {
    return { subscribe: this._events.subscribe };
  }

  /** Derived store for all pane nodes in the layout */
  readonly panes = derived(this._layout, ($layout) =>
    this.collectPanes($layout.root)
  );

  /** Derived store for active pane */
  readonly activePane = derived(this._layout, ($layout) => {
    if (!$layout.activePaneId) return null;
    return this.findPane($layout.root, $layout.activePaneId);
  });

  /**
   * Create default layout with single empty pane
   */
  private createDefaultLayout(): WorkspaceLayout {
    return {
      root: {
        type: 'pane',
        id: generateId(),
        paneType: PaneType.Empty,
        config: {},
        title: 'New Pane',
      },
      version: 1,
    };
  }

  /**
   * Add a new pane to the workspace
   *
   * @param paneType - Type of pane to create
   * @param config - Pane configuration
   * @param options - Placement options
   */
  addPane(
    paneType: PaneType,
    config: PaneConfig,
    options: AddPaneOptions = {}
  ): string {
    const paneId = generateId();
    const paneNode: PaneNode = {
      type: 'pane',
      id: paneId,
      paneType,
      config,
      title: options.title ?? this.getDefaultTitle(paneType),
    };

    this._layout.update((layout) => {
      const newLayout = this.insertPane(layout, paneNode, options);
      return newLayout;
    });

    this._events.set({ type: 'pane-added', paneId, paneNode });

    if (options.activate !== false) {
      this.activatePane(paneId);
    }

    return paneId;
  }

  /**
   * Remove a pane from the workspace
   */
  removePane(paneId: string): void {
    this._layout.update((layout) => {
      return this.removePaneFromLayout(layout, paneId);
    });

    this._events.set({ type: 'pane-removed', paneId });
  }

  /**
   * Activate a pane (bring to focus)
   */
  activatePane(paneId: string): void {
    this._layout.update((layout) => {
      // Also update tab activeIndex if pane is in a tab container
      const newLayout = this.updateTabActiveIndex(layout, paneId);
      return { ...newLayout, activePaneId: paneId };
    });

    this._events.set({ type: 'pane-activated', paneId });
  }

  /**
   * Update pane configuration
   */
  updatePaneConfig(paneId: string, config: Partial<PaneConfig>): void {
    this._layout.update((layout) => {
      return this.updatePaneInLayout(layout, paneId, (pane) => ({
        ...pane,
        config: { ...pane.config, ...config },
      }));
    });
  }

  /**
   * Resize a split container
   */
  resizeSplit(splitId: string, sizes: number[]): void {
    this._layout.update((layout) => {
      return this.updateNodeInLayout(layout, splitId, (node) => {
        if (node.type !== 'split') return node;
        return { ...node, sizes };
      });
    });

    this._events.set({ type: 'split-resized', splitId, sizes });
  }

  /**
   * Split a pane horizontally or vertically
   */
  splitPane(
    paneId: string,
    direction: 'left' | 'right' | 'top' | 'bottom',
    newPaneType: PaneType = PaneType.Empty,
    newPaneConfig: PaneConfig = {}
  ): string {
    const newPaneId = generateId();
    const newPane: PaneNode = {
      type: 'pane',
      id: newPaneId,
      paneType: newPaneType,
      config: newPaneConfig,
      title: this.getDefaultTitle(newPaneType),
    };

    this._layout.update((layout) => {
      return this.splitPaneInLayout(layout, paneId, direction, newPane);
    });

    this._events.set({ type: 'pane-added', paneId: newPaneId, paneNode: newPane });

    return newPaneId;
  }

  /**
   * Save layout for persistence
   * Inspired by JupyterLab's saveLayout and GoldenLayout's toConfig
   */
  saveLayout(): WorkspaceLayout {
    return get(this._layout);
  }

  /**
   * Restore layout from saved state
   * Inspired by JupyterLab's restoreLayout and GoldenLayout's loadLayout
   */
  restoreLayout(layout: WorkspaceLayout): void {
    // Validate and migrate if needed
    const migratedLayout = this.migrateLayout(layout);

    this._layout.set(migratedLayout);
    this._events.set({ type: 'layout-changed', layout: migratedLayout });
  }

  /**
   * Get the current layout tree for debugging/inspection
   */
  getLayoutTree(): LayoutNode {
    return get(this._layout).root;
  }

  // --- Private helper methods ---

  private insertPane(
    layout: WorkspaceLayout,
    paneNode: PaneNode,
    options: AddPaneOptions
  ): WorkspaceLayout {
    const { referenceId, position = 'right' } = options;

    // If no reference, add as tab to root or create split
    if (!referenceId) {
      return this.addPaneToRoot(layout, paneNode, position);
    }

    // Find reference node and insert relative to it
    return this.addPaneRelativeTo(layout, paneNode, referenceId, position);
  }

  private addPaneToRoot(
    layout: WorkspaceLayout,
    paneNode: PaneNode,
    position: AddPanePosition
  ): WorkspaceLayout {
    const root = layout.root;

    // If root is a single pane, convert to split or tab
    if (root.type === 'pane') {
      if (position === 'tab') {
        // Convert to tab container
        return {
          ...layout,
          root: {
            type: 'tab',
            id: generateId(),
            children: [root, paneNode],
            activeIndex: 1,
          },
        };
      } else {
        // Convert to split container
        const orientation: 'horizontal' | 'vertical' =
          position === 'left' || position === 'right' ? 'horizontal' : 'vertical';
        const children = position === 'left' || position === 'top'
          ? [paneNode, root]
          : [root, paneNode];

        return {
          ...layout,
          root: {
            type: 'split',
            id: generateId(),
            orientation,
            children,
            sizes: [0.5, 0.5],
          },
        };
      }
    }

    // If root is already a container, add to it
    if (root.type === 'tab') {
      return {
        ...layout,
        root: {
          ...root,
          children: [...root.children, paneNode],
          activeIndex: root.children.length,
        },
      };
    }

    // Root is split - add as new child
    if (root.type === 'split') {
      const newSizes = this.redistributeSizes(root.sizes, 1);
      return {
        ...layout,
        root: {
          ...root,
          children: [...root.children, paneNode],
          sizes: newSizes,
        },
      };
    }

    return layout;
  }

  private addPaneRelativeTo(
    layout: WorkspaceLayout,
    paneNode: PaneNode,
    referenceId: string,
    position: AddPanePosition
  ): WorkspaceLayout {
    // Deep clone and modify the layout tree
    const newRoot = this.cloneAndModifyTree(layout.root, (node, parent) => {
      if (node.type !== 'pane' || node.id !== referenceId) {
        return node;
      }

      // Found reference pane - create appropriate container
      if (position === 'tab') {
        // Wrap in tab container
        return {
          type: 'tab',
          id: generateId(),
          children: [node, paneNode],
          activeIndex: 1,
        } as TabNode;
      } else {
        // Wrap in split container
        const orientation: 'horizontal' | 'vertical' =
          position === 'left' || position === 'right' ? 'horizontal' : 'vertical';
        const children = position === 'left' || position === 'top'
          ? [paneNode, node]
          : [node, paneNode];

        return {
          type: 'split',
          id: generateId(),
          orientation,
          children,
          sizes: [0.5, 0.5],
        } as SplitNode;
      }
    });

    return { ...layout, root: newRoot };
  }

  private removePaneFromLayout(
    layout: WorkspaceLayout,
    paneId: string
  ): WorkspaceLayout {
    const newRoot = this.removeNodeFromTree(layout.root, paneId);

    if (!newRoot) {
      // All panes removed - create empty layout
      return this.createDefaultLayout();
    }

    // Clean up empty containers
    const cleanedRoot = this.cleanupEmptyContainers(newRoot);

    return {
      ...layout,
      root: cleanedRoot,
      activePaneId: layout.activePaneId === paneId ? undefined : layout.activePaneId,
    };
  }

  private removeNodeFromTree(
    node: LayoutNode,
    paneId: string
  ): LayoutNode | null {
    if (node.type === 'pane') {
      return node.id === paneId ? null : node;
    }

    if (node.type === 'tab') {
      const newChildren = node.children.filter(c => c.id !== paneId);
      if (newChildren.length === 0) return null;
      if (newChildren.length === 1) return newChildren[0];
      return {
        ...node,
        children: newChildren,
        activeIndex: Math.min(node.activeIndex, newChildren.length - 1),
      };
    }

    if (node.type === 'split') {
      const newChildren: LayoutNode[] = [];
      const newSizes: number[] = [];

      for (let i = 0; i < node.children.length; i++) {
        const child = this.removeNodeFromTree(node.children[i], paneId);
        if (child) {
          newChildren.push(child);
          newSizes.push(node.sizes[i]);
        }
      }

      if (newChildren.length === 0) return null;
      if (newChildren.length === 1) return newChildren[0];

      // Normalize sizes
      const total = newSizes.reduce((a, b) => a + b, 0);
      const normalizedSizes = newSizes.map(s => s / total);

      return { ...node, children: newChildren, sizes: normalizedSizes };
    }

    return node;
  }

  private cleanupEmptyContainers(node: LayoutNode): LayoutNode {
    if (node.type === 'pane') return node;

    if (node.type === 'tab') {
      const cleanedChildren = node.children.map(c => this.cleanupEmptyContainers(c) as PaneNode);
      if (cleanedChildren.length === 1) return cleanedChildren[0];
      return { ...node, children: cleanedChildren };
    }

    if (node.type === 'split') {
      const cleanedChildren = node.children.map(c => this.cleanupEmptyContainers(c));
      if (cleanedChildren.length === 1) return cleanedChildren[0];
      return { ...node, children: cleanedChildren };
    }

    return node;
  }

  private splitPaneInLayout(
    layout: WorkspaceLayout,
    paneId: string,
    direction: 'left' | 'right' | 'top' | 'bottom',
    newPane: PaneNode
  ): WorkspaceLayout {
    const orientation: 'horizontal' | 'vertical' =
      direction === 'left' || direction === 'right' ? 'horizontal' : 'vertical';

    const newRoot = this.cloneAndModifyTree(layout.root, (node) => {
      if (node.type !== 'pane' || node.id !== paneId) {
        return node;
      }

      const children = direction === 'left' || direction === 'top'
        ? [newPane, node]
        : [node, newPane];

      return {
        type: 'split',
        id: generateId(),
        orientation,
        children,
        sizes: [0.5, 0.5],
      } as SplitNode;
    });

    return { ...layout, root: newRoot };
  }

  private updateTabActiveIndex(
    layout: WorkspaceLayout,
    paneId: string
  ): WorkspaceLayout {
    const newRoot = this.cloneAndModifyTree(layout.root, (node) => {
      if (node.type !== 'tab') return node;

      const index = node.children.findIndex(c => c.id === paneId);
      if (index === -1) return node;

      return { ...node, activeIndex: index };
    });

    return { ...layout, root: newRoot };
  }

  private updatePaneInLayout(
    layout: WorkspaceLayout,
    paneId: string,
    updater: (pane: PaneNode) => PaneNode
  ): WorkspaceLayout {
    const newRoot = this.cloneAndModifyTree(layout.root, (node) => {
      if (node.type !== 'pane' || node.id !== paneId) return node;
      return updater(node);
    });

    return { ...layout, root: newRoot };
  }

  private updateNodeInLayout(
    layout: WorkspaceLayout,
    nodeId: string,
    updater: (node: LayoutNode) => LayoutNode
  ): WorkspaceLayout {
    const newRoot = this.cloneAndModifyTree(layout.root, (node) => {
      if (node.id !== nodeId) return node;
      return updater(node);
    });

    return { ...layout, root: newRoot };
  }

  private cloneAndModifyTree(
    node: LayoutNode,
    modifier: (node: LayoutNode, parent?: LayoutNode) => LayoutNode,
    parent?: LayoutNode
  ): LayoutNode {
    const modified = modifier(node, parent);

    if (modified !== node) {
      // Node was replaced, return new node
      return modified;
    }

    // Recurse into children
    if (node.type === 'split') {
      const newChildren = node.children.map(c =>
        this.cloneAndModifyTree(c, modifier, node)
      );
      if (newChildren.some((c, i) => c !== node.children[i])) {
        return { ...node, children: newChildren };
      }
    }

    if (node.type === 'tab') {
      const newChildren = node.children.map(c =>
        this.cloneAndModifyTree(c, modifier, node) as PaneNode
      );
      if (newChildren.some((c, i) => c !== node.children[i])) {
        return { ...node, children: newChildren };
      }
    }

    return node;
  }

  private collectPanes(node: LayoutNode): PaneNode[] {
    if (node.type === 'pane') return [node];

    if (node.type === 'tab' || node.type === 'split') {
      return node.children.flatMap(c => this.collectPanes(c));
    }

    return [];
  }

  private findPane(node: LayoutNode, paneId: string): PaneNode | null {
    if (node.type === 'pane') {
      return node.id === paneId ? node : null;
    }

    for (const child of node.type === 'split' ? node.children : node.children) {
      const found = this.findPane(child, paneId);
      if (found) return found;
    }

    return null;
  }

  private redistributeSizes(currentSizes: number[], additionalCount: number): number[] {
    const total = currentSizes.length + additionalCount;
    const newSize = 1 / total;
    const scale = (total - additionalCount) / total;

    const newSizes = currentSizes.map(s => s * scale);
    for (let i = 0; i < additionalCount; i++) {
      newSizes.push(newSize);
    }

    return newSizes;
  }

  private getDefaultTitle(paneType: PaneType): string {
    const titles: Record<PaneType, string> = {
      [PaneType.LineChart]: 'Line Chart',
      [PaneType.ScatterChart]: 'Scatter Chart',
      [PaneType.Histogram]: 'Histogram',
      [PaneType.CrossPlot]: 'Cross Plot',
      [PaneType.WellLog]: 'Well Log',
      [PaneType.DataGrid]: 'Data Grid',
      [PaneType.Empty]: 'New Pane',
    };
    return titles[paneType] ?? 'Pane';
  }

  private migrateLayout(layout: WorkspaceLayout): WorkspaceLayout {
    // Add migration logic for future version changes
    if (layout.version < 1) {
      // Migrate from version 0 to 1
    }
    return layout;
  }
}

export interface AddPaneOptions {
  /** Reference pane ID for relative positioning */
  referenceId?: string;
  /** Position relative to reference (or root if no reference) */
  position?: AddPanePosition;
  /** Custom title */
  title?: string;
  /** Whether to activate the new pane */
  activate?: boolean;
}

export type AddPanePosition = 'left' | 'right' | 'top' | 'bottom' | 'tab';

/** Singleton instance */
export const workspaceManager = new WorkspaceManagerImpl();
```

### 12.5 Pane Factory (Instance Creation)

```typescript
// src/lib/panes/pane-factory.ts

import type { PaneNode, PaneType } from './layout-model';
import type { IChartInstance, ChartConfig } from '$lib/charts/types';
import { chartRegistry } from '$lib/charts/chart-registry';
import { chartDataStore } from '$lib/charts/chart-data-store';
import { ChartType } from '$lib/charts/types';

/**
 * IPaneInstance - Interface for all pane instances
 * Inspired by JupyterLab's Widget and VS Code's EditorPane
 */
export interface IPaneInstance {
  /** Unique instance ID */
  readonly id: string;

  /** Pane type */
  readonly paneType: PaneType;

  /** Mount pane to container element */
  mount(container: HTMLElement): void;

  /** Update pane with new data/config */
  update(config: Partial<PaneConfig>): void;

  /** Resize pane to new dimensions */
  resize(width: number, height: number): void;

  /** Focus the pane */
  focus(): void;

  /** Blur the pane */
  blur(): void;

  /** Destroy and cleanup */
  destroy(): void;

  /** Event emitter */
  on<K extends keyof PaneEvents>(event: K, handler: PaneEvents[K]): void;
  off<K extends keyof PaneEvents>(event: K, handler: PaneEvents[K]): void;
}

export interface PaneEvents {
  'focus': () => void;
  'blur': () => void;
  'title-change': (title: string) => void;
  'config-change': (config: PaneConfig) => void;
  'close-request': () => void;
}

/**
 * PaneFactory - Creates pane instances based on type
 * Inspired by JupyterLab's WidgetFactory and VS Code's EditorFactory
 */
class PaneFactoryImpl {
  private factories = new Map<PaneType, (node: PaneNode) => IPaneInstance>();

  constructor() {
    // Register built-in pane types
    this.register(PaneType.LineChart, (node) =>
      new ChartPaneInstance(node, ChartType.Line));
    this.register(PaneType.ScatterChart, (node) =>
      new ChartPaneInstance(node, ChartType.Scatter));
    this.register(PaneType.Histogram, (node) =>
      new ChartPaneInstance(node, ChartType.Histogram));
    this.register(PaneType.WellLog, (node) =>
      new ChartPaneInstance(node, ChartType.WellLog));
    this.register(PaneType.DataGrid, (node) =>
      new DataGridPaneInstance(node));
    this.register(PaneType.Empty, (node) =>
      new EmptyPaneInstance(node));
  }

  /**
   * Register a pane type factory
   */
  register(type: PaneType, factory: (node: PaneNode) => IPaneInstance): void {
    this.factories.set(type, factory);
  }

  /**
   * Create a pane instance
   */
  create(node: PaneNode): IPaneInstance {
    const factory = this.factories.get(node.paneType);
    if (!factory) {
      console.warn(`Unknown pane type: ${node.paneType}, using empty pane`);
      return new EmptyPaneInstance(node);
    }
    return factory(node);
  }
}

/**
 * ChartPaneInstance - Wraps a chart instance in a pane
 */
class ChartPaneInstance implements IPaneInstance {
  readonly id: string;
  readonly paneType: PaneType;

  private chart: IChartInstance | null = null;
  private container: HTMLElement | null = null;
  private config: PaneConfig;
  private eventHandlers = new Map<string, Set<Function>>();

  constructor(node: PaneNode, private chartType: ChartType) {
    this.id = node.id;
    this.paneType = node.paneType;
    this.config = node.config;
  }

  mount(container: HTMLElement): void {
    this.container = container;

    // Create chart instance
    const chartConfig: ChartConfig = {
      id: `chart-${this.id}`,
      type: this.chartType,
      title: '',
      xAxis: { field: 'DEPTH', inverted: true },
      yAxes: [{ field: 'value' }],
      series: [{ field: 'value', color: '#3b82f6' }],
      interaction: {
        pan: true,
        zoom: true,
        cursor: true,
        linkedGroup: this.config.linkedGroup,
      },
      ...this.config.chartConfig,
    };

    this.chart = chartRegistry.create(chartConfig);
    this.chart.mount(container);

    // Load data if specified
    if (this.config.dataFrameId) {
      const frame = chartDataStore.getFrame(this.config.dataFrameId);
      if (frame) {
        this.chart.setData(frame);
      }
    }
  }

  update(config: Partial<PaneConfig>): void {
    this.config = { ...this.config, ...config };

    if (config.dataFrameId && this.chart) {
      const frame = chartDataStore.getFrame(config.dataFrameId);
      if (frame) {
        this.chart.setData(frame);
      }
    }

    if (config.chartConfig && this.chart) {
      this.chart.setConfig(config.chartConfig);
    }
  }

  resize(width: number, height: number): void {
    this.chart?.resize(width, height);
  }

  focus(): void {
    this.emit('focus');
  }

  blur(): void {
    this.emit('blur');
  }

  destroy(): void {
    this.chart?.destroy();
    this.chart = null;
    this.container = null;
    this.eventHandlers.clear();
  }

  on<K extends keyof PaneEvents>(event: K, handler: PaneEvents[K]): void {
    if (!this.eventHandlers.has(event)) {
      this.eventHandlers.set(event, new Set());
    }
    this.eventHandlers.get(event)!.add(handler);
  }

  off<K extends keyof PaneEvents>(event: K, handler: PaneEvents[K]): void {
    this.eventHandlers.get(event)?.delete(handler);
  }

  private emit<K extends keyof PaneEvents>(event: K, ...args: Parameters<PaneEvents[K]>): void {
    this.eventHandlers.get(event)?.forEach(handler => {
      (handler as Function)(...args);
    });
  }
}

/**
 * DataGridPaneInstance - Wraps a data grid in a pane
 */
class DataGridPaneInstance implements IPaneInstance {
  readonly id: string;
  readonly paneType = PaneType.DataGrid;

  private container: HTMLElement | null = null;
  private eventHandlers = new Map<string, Set<Function>>();

  constructor(private node: PaneNode) {
    this.id = node.id;
  }

  mount(container: HTMLElement): void {
    this.container = container;
    // DataGrid is rendered as Svelte component - handled by PaneContainer.svelte
  }

  update(_config: Partial<PaneConfig>): void {
    // Handle config updates
  }

  resize(_width: number, _height: number): void {
    // DataGrid handles its own resizing
  }

  focus(): void {
    this.emit('focus');
  }

  blur(): void {
    this.emit('blur');
  }

  destroy(): void {
    this.container = null;
    this.eventHandlers.clear();
  }

  on<K extends keyof PaneEvents>(event: K, handler: PaneEvents[K]): void {
    if (!this.eventHandlers.has(event)) {
      this.eventHandlers.set(event, new Set());
    }
    this.eventHandlers.get(event)!.add(handler);
  }

  off<K extends keyof PaneEvents>(event: K, handler: PaneEvents[K]): void {
    this.eventHandlers.get(event)?.delete(handler);
  }

  private emit<K extends keyof PaneEvents>(event: K, ...args: Parameters<PaneEvents[K]>): void {
    this.eventHandlers.get(event)?.forEach(handler => {
      (handler as Function)(...args);
    });
  }
}

/**
 * EmptyPaneInstance - Placeholder for empty panes
 */
class EmptyPaneInstance implements IPaneInstance {
  readonly id: string;
  readonly paneType = PaneType.Empty;

  private container: HTMLElement | null = null;
  private eventHandlers = new Map<string, Set<Function>>();

  constructor(private node: PaneNode) {
    this.id = node.id;
  }

  mount(container: HTMLElement): void {
    this.container = container;
    container.innerHTML = `
      <div class="empty-pane">
        <div class="empty-pane-content">
          <span class="empty-icon">+</span>
          <p>Add a chart to this pane</p>
        </div>
      </div>
    `;
  }

  update(_config: Partial<PaneConfig>): void {}
  resize(_width: number, _height: number): void {}
  focus(): void { this.emit('focus'); }
  blur(): void { this.emit('blur'); }

  destroy(): void {
    if (this.container) {
      this.container.innerHTML = '';
    }
    this.container = null;
    this.eventHandlers.clear();
  }

  on<K extends keyof PaneEvents>(event: K, handler: PaneEvents[K]): void {
    if (!this.eventHandlers.has(event)) {
      this.eventHandlers.set(event, new Set());
    }
    this.eventHandlers.get(event)!.add(handler);
  }

  off<K extends keyof PaneEvents>(event: K, handler: PaneEvents[K]): void {
    this.eventHandlers.get(event)?.delete(handler);
  }

  private emit<K extends keyof PaneEvents>(event: K, ...args: Parameters<PaneEvents[K]>): void {
    this.eventHandlers.get(event)?.forEach(handler => {
      (handler as Function)(...args);
    });
  }
}

export const paneFactory = new PaneFactoryImpl();
```

### 12.6 Svelte Components

#### WorkspaceContainer.svelte

```svelte
<!-- src/lib/panes/components/WorkspaceContainer.svelte -->

<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { workspaceManager, type WorkspaceEvent } from '../workspace-manager';
  import LayoutRenderer from './LayoutRenderer.svelte';
  import PaneToolbar from './PaneToolbar.svelte';

  /** Workspace ID for persistence */
  interface Props {
    workspaceId?: string;
  }

  let { workspaceId = 'default' }: Props = $props();

  let containerRef: HTMLElement | null = $state(null);
  let layout = $state(workspaceManager.getLayoutTree());

  // Subscribe to layout changes
  let unsubscribeLayout: (() => void) | null = null;
  let unsubscribeEvents: (() => void) | null = null;

  onMount(() => {
    // Subscribe to layout store
    unsubscribeLayout = workspaceManager.layout.subscribe((state) => {
      layout = state.root;
    });

    // Subscribe to workspace events
    unsubscribeEvents = workspaceManager.events.subscribe((event) => {
      if (event) {
        handleWorkspaceEvent(event);
      }
    });

    // Load persisted layout if available
    loadPersistedLayout();
  });

  onDestroy(() => {
    unsubscribeLayout?.();
    unsubscribeEvents?.();

    // Save layout on destroy
    saveLayout();
  });

  function handleWorkspaceEvent(event: WorkspaceEvent) {
    switch (event.type) {
      case 'pane-added':
        console.log('Pane added:', event.paneId);
        break;
      case 'pane-removed':
        console.log('Pane removed:', event.paneId);
        break;
      case 'layout-changed':
        console.log('Layout changed');
        break;
    }
  }

  async function loadPersistedLayout() {
    try {
      const stored = localStorage.getItem(`workspace-layout-${workspaceId}`);
      if (stored) {
        const savedLayout = JSON.parse(stored);
        workspaceManager.restoreLayout(savedLayout);
      }
    } catch (e) {
      console.warn('Failed to load persisted layout:', e);
    }
  }

  function saveLayout() {
    try {
      const layoutState = workspaceManager.saveLayout();
      localStorage.setItem(
        `workspace-layout-${workspaceId}`,
        JSON.stringify(layoutState)
      );
    } catch (e) {
      console.warn('Failed to save layout:', e);
    }
  }

  // Handle window resize
  function handleResize() {
    // Trigger layout recalculation
    // Individual panes will handle their own resize
  }
</script>

<svelte:window on:resize={handleResize} />

<div class="workspace-container" bind:this={containerRef}>
  <PaneToolbar />

  <div class="workspace-content">
    {#if layout}
      <LayoutRenderer node={layout} />
    {/if}
  </div>
</div>

<style>
  .workspace-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    overflow: hidden;
    background: var(--bg-primary, #1e1e1e);
  }

  .workspace-content {
    flex: 1;
    overflow: hidden;
    position: relative;
  }
</style>
```

#### LayoutRenderer.svelte

```svelte
<!-- src/lib/panes/components/LayoutRenderer.svelte -->

<script lang="ts">
  import type { LayoutNode, SplitNode, TabNode, PaneNode } from '../layout-model';
  import SplitContainer from './SplitContainer.svelte';
  import TabContainer from './TabContainer.svelte';
  import PaneContainer from './PaneContainer.svelte';

  interface Props {
    node: LayoutNode;
    depth?: number;
  }

  let { node, depth = 0 }: Props = $props();
</script>

{#if node.type === 'split'}
  <SplitContainer node={node as SplitNode} {depth}>
    {#each node.children as child, index (child.id)}
      <svelte:self node={child} depth={depth + 1} />
    {/each}
  </SplitContainer>
{:else if node.type === 'tab'}
  <TabContainer node={node as TabNode} {depth} />
{:else if node.type === 'pane'}
  <PaneContainer node={node as PaneNode} {depth} />
{/if}
```

#### SplitContainer.svelte

```svelte
<!-- src/lib/panes/components/SplitContainer.svelte -->

<script lang="ts">
  import { onMount } from 'svelte';
  import type { SplitNode } from '../layout-model';
  import { workspaceManager } from '../workspace-manager';
  import type { Snippet } from 'svelte';

  interface Props {
    node: SplitNode;
    depth: number;
    children: Snippet;
  }

  let { node, depth, children }: Props = $props();

  let containerRef: HTMLElement | null = $state(null);
  let draggingIndex = $state<number | null>(null);
  let startPos = $state(0);
  let startSizes = $state<number[]>([]);

  function handleMouseDown(e: MouseEvent, index: number) {
    e.preventDefault();
    draggingIndex = index;
    startPos = node.orientation === 'horizontal' ? e.clientX : e.clientY;
    startSizes = [...node.sizes];

    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
  }

  function handleMouseMove(e: MouseEvent) {
    if (draggingIndex === null || !containerRef) return;

    const containerRect = containerRef.getBoundingClientRect();
    const containerSize = node.orientation === 'horizontal'
      ? containerRect.width
      : containerRect.height;

    const currentPos = node.orientation === 'horizontal' ? e.clientX : e.clientY;
    const delta = (currentPos - startPos) / containerSize;

    // Calculate new sizes
    const newSizes = [...startSizes];
    const minSize = 0.1; // Minimum 10% of container

    // Adjust sizes for the two panels around the sash
    newSizes[draggingIndex] = Math.max(minSize, startSizes[draggingIndex] + delta);
    newSizes[draggingIndex + 1] = Math.max(minSize, startSizes[draggingIndex + 1] - delta);

    // Normalize if needed
    const total = newSizes.reduce((a, b) => a + b, 0);
    if (Math.abs(total - 1) > 0.001) {
      const scale = 1 / total;
      for (let i = 0; i < newSizes.length; i++) {
        newSizes[i] *= scale;
      }
    }

    workspaceManager.resizeSplit(node.id, newSizes);
  }

  function handleMouseUp() {
    draggingIndex = null;
    window.removeEventListener('mousemove', handleMouseMove);
    window.removeEventListener('mouseup', handleMouseUp);
  }
</script>

<div
  class="split-container"
  class:horizontal={node.orientation === 'horizontal'}
  class:vertical={node.orientation === 'vertical'}
  bind:this={containerRef}
>
  {#each node.children as child, index (child.id)}
    <div
      class="split-pane"
      style="flex: {node.sizes[index]};"
    >
      {@render children()}
    </div>

    {#if index < node.children.length - 1}
      <div
        class="split-sash"
        class:horizontal={node.orientation === 'horizontal'}
        class:vertical={node.orientation === 'vertical'}
        class:dragging={draggingIndex === index}
        onmousedown={(e) => handleMouseDown(e, index)}
        role="separator"
        aria-orientation={node.orientation}
      ></div>
    {/if}
  {/each}
</div>

<style>
  .split-container {
    display: flex;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .split-container.horizontal {
    flex-direction: row;
  }

  .split-container.vertical {
    flex-direction: column;
  }

  .split-pane {
    overflow: hidden;
    min-width: 0;
    min-height: 0;
  }

  .split-sash {
    flex-shrink: 0;
    background: var(--border, #333);
    transition: background 0.15s;
  }

  .split-sash.horizontal {
    width: 4px;
    cursor: col-resize;
  }

  .split-sash.vertical {
    height: 4px;
    cursor: row-resize;
  }

  .split-sash:hover,
  .split-sash.dragging {
    background: var(--accent, #007acc);
  }
</style>
```

#### TabContainer.svelte

```svelte
<!-- src/lib/panes/components/TabContainer.svelte -->

<script lang="ts">
  import type { TabNode, PaneNode } from '../layout-model';
  import { workspaceManager } from '../workspace-manager';
  import PaneContainer from './PaneContainer.svelte';

  interface Props {
    node: TabNode;
    depth: number;
  }

  let { node, depth }: Props = $props();

  function activateTab(index: number) {
    const pane = node.children[index];
    if (pane) {
      workspaceManager.activatePane(pane.id);
    }
  }

  function closeTab(e: MouseEvent, paneId: string) {
    e.stopPropagation();
    workspaceManager.removePane(paneId);
  }

  function handleDragStart(e: DragEvent, pane: PaneNode) {
    e.dataTransfer?.setData('text/plain', pane.id);
    e.dataTransfer!.effectAllowed = 'move';
  }
</script>

<div class="tab-container">
  <div class="tab-bar">
    {#each node.children as pane, index (pane.id)}
      <button
        class="tab"
        class:active={index === node.activeIndex}
        onclick={() => activateTab(index)}
        draggable="true"
        ondragstart={(e) => handleDragStart(e, pane)}
      >
        <span class="tab-title">{pane.title}</span>
        <button
          class="tab-close"
          onclick={(e) => closeTab(e, pane.id)}
          aria-label="Close tab"
        >
          ×
        </button>
      </button>
    {/each}
  </div>

  <div class="tab-content">
    {#if node.children[node.activeIndex]}
      <PaneContainer
        node={node.children[node.activeIndex]}
        {depth}
      />
    {/if}
  </div>
</div>

<style>
  .tab-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .tab-bar {
    display: flex;
    background: var(--bg-secondary, #252526);
    border-bottom: 1px solid var(--border, #333);
    overflow-x: auto;
    flex-shrink: 0;
  }

  .tab {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: transparent;
    border: none;
    color: var(--text-secondary, #999);
    cursor: pointer;
    font-size: 0.875rem;
    white-space: nowrap;
    border-right: 1px solid var(--border, #333);
  }

  .tab:hover {
    background: var(--bg-hover, #2a2a2a);
    color: var(--text-primary, #fff);
  }

  .tab.active {
    background: var(--bg-primary, #1e1e1e);
    color: var(--text-primary, #fff);
    border-bottom: 2px solid var(--accent, #007acc);
    margin-bottom: -1px;
  }

  .tab-title {
    max-width: 150px;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tab-close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    background: transparent;
    border: none;
    border-radius: 3px;
    color: inherit;
    cursor: pointer;
    opacity: 0;
    transition: opacity 0.15s, background 0.15s;
  }

  .tab:hover .tab-close {
    opacity: 0.7;
  }

  .tab-close:hover {
    opacity: 1;
    background: var(--bg-hover, rgba(255, 255, 255, 0.1));
  }

  .tab-content {
    flex: 1;
    overflow: hidden;
  }
</style>
```

#### PaneContainer.svelte

```svelte
<!-- src/lib/panes/components/PaneContainer.svelte -->

<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import type { PaneNode } from '../layout-model';
  import { PaneType } from '../layout-model';
  import { paneFactory, type IPaneInstance } from '../pane-factory';
  import { workspaceManager } from '../workspace-manager';

  interface Props {
    node: PaneNode;
    depth: number;
  }

  let { node, depth }: Props = $props();

  let containerRef: HTMLElement | null = $state(null);
  let paneInstance: IPaneInstance | null = $state(null);

  // Track node changes
  let previousNodeId = $state<string | null>(null);

  // Create/update pane instance when node changes
  $effect(() => {
    if (!containerRef) return;

    if (previousNodeId !== node.id) {
      // Node changed - destroy old instance and create new one
      paneInstance?.destroy();
      paneInstance = paneFactory.create(node);
      paneInstance.mount(containerRef);
      previousNodeId = node.id;
    } else {
      // Same node - just update config if needed
      paneInstance?.update(node.config);
    }
  });

  // Handle resize via ResizeObserver
  let resizeObserver: ResizeObserver | null = null;

  onMount(() => {
    if (containerRef) {
      resizeObserver = new ResizeObserver((entries) => {
        for (const entry of entries) {
          const { width, height } = entry.contentRect;
          paneInstance?.resize(width, height);
        }
      });
      resizeObserver.observe(containerRef);
    }
  });

  onDestroy(() => {
    resizeObserver?.disconnect();
    paneInstance?.destroy();
  });

  function handleClick() {
    workspaceManager.activatePane(node.id);
  }

  function handleContextMenu(e: MouseEvent) {
    e.preventDefault();
    // TODO: Show context menu with options like:
    // - Split Left/Right/Top/Bottom
    // - Change chart type
    // - Close pane
  }

  // Drag-and-drop for data binding
  function handleDrop(e: DragEvent) {
    e.preventDefault();
    const dataFrameId = e.dataTransfer?.getData('application/dataforge-frame');
    if (dataFrameId) {
      workspaceManager.updatePaneConfig(node.id, { dataFrameId });
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    e.dataTransfer!.dropEffect = 'copy';
  }
</script>

<div
  class="pane-container"
  class:active={true}
  bind:this={containerRef}
  onclick={handleClick}
  oncontextmenu={handleContextMenu}
  ondrop={handleDrop}
  ondragover={handleDragOver}
  role="region"
  aria-label={node.title}
>
  <!-- Content is mounted by pane instance -->
</div>

<style>
  .pane-container {
    width: 100%;
    height: 100%;
    overflow: hidden;
    background: var(--bg-primary, #1e1e1e);
    border: 1px solid transparent;
    transition: border-color 0.15s;
  }

  .pane-container:focus-within,
  .pane-container.active {
    border-color: var(--accent, #007acc);
  }

  /* Empty pane styles (injected by EmptyPaneInstance) */
  :global(.empty-pane) {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }

  :global(.empty-pane-content) {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    color: var(--text-secondary, #666);
  }

  :global(.empty-icon) {
    font-size: 3rem;
    opacity: 0.3;
  }
</style>
```

#### PaneToolbar.svelte

```svelte
<!-- src/lib/panes/components/PaneToolbar.svelte -->

<script lang="ts">
  import { workspaceManager } from '../workspace-manager';
  import { PaneType } from '../layout-model';
  import { chartDataStore } from '$lib/charts/chart-data-store';

  let showAddMenu = $state(false);

  const paneTypes = [
    { type: PaneType.LineChart, label: 'Line Chart', icon: '📈' },
    { type: PaneType.ScatterChart, label: 'Scatter Chart', icon: '⚫' },
    { type: PaneType.Histogram, label: 'Histogram', icon: '📊' },
    { type: PaneType.WellLog, label: 'Well Log', icon: '📏' },
    { type: PaneType.DataGrid, label: 'Data Grid', icon: '🗃️' },
  ];

  function addPane(type: PaneType) {
    workspaceManager.addPane(type, {}, {
      position: 'right',
      activate: true,
    });
    showAddMenu = false;
  }

  function resetLayout() {
    if (confirm('Reset workspace to default layout?')) {
      localStorage.removeItem('workspace-layout-default');
      location.reload();
    }
  }
</script>

<div class="pane-toolbar">
  <div class="toolbar-left">
    <div class="add-menu-container">
      <button
        class="toolbar-btn"
        onclick={() => showAddMenu = !showAddMenu}
        aria-expanded={showAddMenu}
      >
        <span class="icon">+</span>
        Add Pane
      </button>

      {#if showAddMenu}
        <div class="add-menu">
          {#each paneTypes as paneType}
            <button
              class="menu-item"
              onclick={() => addPane(paneType.type)}
            >
              <span class="menu-icon">{paneType.icon}</span>
              {paneType.label}
            </button>
          {/each}
        </div>
      {/if}
    </div>
  </div>

  <div class="toolbar-right">
    <button class="toolbar-btn secondary" onclick={resetLayout}>
      Reset Layout
    </button>
  </div>
</div>

{#if showAddMenu}
  <div
    class="menu-backdrop"
    onclick={() => showAddMenu = false}
    role="presentation"
  ></div>
{/if}

<style>
  .pane-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 1rem;
    background: var(--bg-secondary, #252526);
    border-bottom: 1px solid var(--border, #333);
  }

  .toolbar-left,
  .toolbar-right {
    display: flex;
    gap: 0.5rem;
  }

  .toolbar-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.375rem 0.75rem;
    background: var(--accent, #007acc);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
  }

  .toolbar-btn:hover {
    background: var(--accent-hover, #0088e0);
  }

  .toolbar-btn.secondary {
    background: var(--bg-tertiary, #333);
    color: var(--text-secondary, #999);
  }

  .toolbar-btn.secondary:hover {
    background: var(--bg-hover, #3a3a3a);
    color: var(--text-primary, #fff);
  }

  .icon {
    font-weight: bold;
  }

  .add-menu-container {
    position: relative;
  }

  .add-menu {
    position: absolute;
    top: 100%;
    left: 0;
    margin-top: 0.25rem;
    background: var(--bg-primary, #1e1e1e);
    border: 1px solid var(--border, #333);
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    z-index: 100;
    min-width: 180px;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    padding: 0.625rem 1rem;
    background: transparent;
    border: none;
    color: var(--text-primary, #fff);
    cursor: pointer;
    font-size: 0.875rem;
    text-align: left;
  }

  .menu-item:hover {
    background: var(--bg-hover, #2a2a2a);
  }

  .menu-item:first-child {
    border-radius: 6px 6px 0 0;
  }

  .menu-item:last-child {
    border-radius: 0 0 6px 6px;
  }

  .menu-icon {
    font-size: 1rem;
  }

  .menu-backdrop {
    position: fixed;
    inset: 0;
    z-index: 99;
  }
</style>
```

### 12.7 Integration with Chart System

The pane system integrates with the existing chart implementation:

```typescript
// src/lib/panes/integration.ts

import { workspaceManager } from './workspace-manager';
import { chartDataStore } from '$lib/charts/chart-data-store';
import { chartManager } from '$lib/charts/chart-manager';
import { PaneType } from './layout-model';

/**
 * Bridge between pane system and chart system
 * Ensures charts in panes are properly connected to shared services
 */
export function initPaneChartIntegration() {
  // When data frames change, notify relevant panes
  chartDataStore.store.subscribe((frames) => {
    // Panes with dataFrameId bindings will auto-update via their config
  });

  // Subscribe to workspace events for chart lifecycle management
  workspaceManager.events.subscribe((event) => {
    if (!event) return;

    switch (event.type) {
      case 'pane-added':
        // Chart instance is created by PaneFactory
        break;

      case 'pane-removed':
        // Chart instance is destroyed by PaneContainer.onDestroy
        break;

      case 'pane-activated':
        // Could trigger focus behavior in chart
        break;
    }
  });
}

/**
 * Add a chart pane with data binding
 */
export function addChartPane(
  chartType: PaneType,
  dataFrameId: string,
  options?: { linkedGroup?: string }
): string {
  return workspaceManager.addPane(chartType, {
    dataFrameId,
    linkedGroup: options?.linkedGroup,
  });
}

/**
 * Display execution result in a new or existing pane
 */
export function displayExecutionResult(
  executionId: string,
  mnemonic: string,
  depths: number[],
  values: (number | null)[]
): string {
  // Load data into chart data store
  const frame = chartDataStore.loadComputedResult(
    executionId,
    mnemonic,
    depths,
    values
  );

  // Add a chart pane to display it
  return addChartPane(PaneType.LineChart, frame.id, {
    linkedGroup: 'default',
  });
}
```

### 12.8 Persistence Strategy

```typescript
// src/lib/panes/persistence.ts

import type { WorkspaceLayout } from './layout-model';
import { invoke } from '@tauri-apps/api/core';

/**
 * WorkspacePersistence - Handles saving/loading workspace layouts
 *
 * Storage hierarchy:
 * 1. localStorage for quick session restore
 * 2. Tauri backend for cross-session persistence
 * 3. (Future) Cloud sync for multi-device
 */
export class WorkspacePersistence {
  private storageKey: string;

  constructor(workspaceId: string) {
    this.storageKey = `workspace-layout-${workspaceId}`;
  }

  /**
   * Save layout to local storage (immediate)
   */
  saveToLocal(layout: WorkspaceLayout): void {
    try {
      localStorage.setItem(this.storageKey, JSON.stringify(layout));
    } catch (e) {
      console.warn('Failed to save layout to localStorage:', e);
    }
  }

  /**
   * Load layout from local storage
   */
  loadFromLocal(): WorkspaceLayout | null {
    try {
      const stored = localStorage.getItem(this.storageKey);
      return stored ? JSON.parse(stored) : null;
    } catch (e) {
      console.warn('Failed to load layout from localStorage:', e);
      return null;
    }
  }

  /**
   * Save layout to backend (persistent)
   */
  async saveToBackend(layout: WorkspaceLayout): Promise<void> {
    try {
      await invoke('save_workspace_layout', {
        workspaceId: this.storageKey,
        layout: JSON.stringify(layout),
      });
    } catch (e) {
      console.warn('Failed to save layout to backend:', e);
    }
  }

  /**
   * Load layout from backend
   */
  async loadFromBackend(): Promise<WorkspaceLayout | null> {
    try {
      const layout = await invoke<string | null>('load_workspace_layout', {
        workspaceId: this.storageKey,
      });
      return layout ? JSON.parse(layout) : null;
    } catch (e) {
      console.warn('Failed to load layout from backend:', e);
      return null;
    }
  }

  /**
   * Clear stored layout
   */
  clear(): void {
    localStorage.removeItem(this.storageKey);
    // Also clear from backend if needed
  }
}
```

### 12.9 Event Flow Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         EVENT FLOW ARCHITECTURE                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  USER ACTION                                                             │
│  (Click "Add Pane")                                                     │
│       │                                                                  │
│       ▼                                                                  │
│  ┌─────────────────┐                                                    │
│  │  PaneToolbar    │ ──── calls ────►  workspaceManager.addPane()       │
│  └─────────────────┘                            │                        │
│                                                 │                        │
│                                                 ▼                        │
│                                   ┌─────────────────────────┐           │
│                                   │   WorkspaceManager      │           │
│                                   │   1. Update _layout     │           │
│                                   │   2. Emit pane-added    │           │
│                                   └───────────┬─────────────┘           │
│                                               │                          │
│               ┌───────────────────────────────┼────────────────┐        │
│               │                               │                │        │
│               ▼                               ▼                ▼        │
│  ┌─────────────────────┐    ┌─────────────────────┐   ┌────────────┐   │
│  │  Svelte Store       │    │  Event Subscribers  │   │ Persistence│   │
│  │  (layout update)    │    │  (logging, etc.)    │   │  (save)    │   │
│  └──────────┬──────────┘    └─────────────────────┘   └────────────┘   │
│             │                                                            │
│             ▼                                                            │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    LayoutRenderer                                │   │
│  │  (Reactive update - creates new component tree)                 │   │
│  └──────────────────────────────────────────────────────────────────┘   │
│             │                                                            │
│             ▼                                                            │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    PaneContainer                                 │   │
│  │  1. paneFactory.create(node)                                    │   │
│  │  2. paneInstance.mount(containerRef)                            │   │
│  │  3. ResizeObserver for size changes                             │   │
│  └──────────────────────────────────────────────────────────────────┘   │
│             │                                                            │
│             ▼                                                            │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    ChartPaneInstance                             │   │
│  │  1. chartRegistry.create(chartConfig)                           │   │
│  │  2. chart.mount(container)                                      │   │
│  │  3. chart.setData(frame) if dataFrameId present                 │   │
│  └──────────────────────────────────────────────────────────────────┘   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 12.10 Implementation Phases

#### Phase 1: Core Infrastructure (Week 1-2)
1. Implement `layout-model.ts` - Layout node types
2. Implement `workspace-manager.ts` - State management
3. Implement basic `LayoutRenderer.svelte`
4. Implement `SplitContainer.svelte` with resize handles
5. Test with static layout

#### Phase 2: Pane Management (Week 2-3)
1. Implement `pane-factory.ts` - Instance creation
2. Implement `PaneContainer.svelte` - Lifecycle management
3. Implement `ChartPaneInstance` - Chart integration
4. Connect to existing chart system

#### Phase 3: Tab Support (Week 3-4)
1. Implement `TabContainer.svelte`
2. Tab drag-and-drop reordering
3. Tab close/add functionality
4. Active tab state management

#### Phase 4: User Interaction (Week 4-5)
1. Implement `PaneToolbar.svelte` - Add pane menu
2. Context menu for pane options
3. Drag-and-drop data binding
4. Keyboard shortcuts

#### Phase 5: Persistence & Polish (Week 5-6)
1. Implement `WorkspacePersistence`
2. Layout save/restore
3. Error handling and edge cases
4. Performance optimization

### 12.11 Key Patterns Summary

| Pattern | Source | Purpose |
|---------|--------|---------|
| **Tree Layout Model** | GoldenLayout, Lumino | Hierarchical split/tab/pane structure |
| **Event-Driven Updates** | Lumino MessageLoop, Theia | Decoupled state propagation |
| **Factory Pattern** | JupyterLab WidgetFactory | Type-safe pane instance creation |
| **Reference Counting** | VS Code ModelService | Shared chart instance management |
| **Persistence Strategy** | JupyterLab LayoutRestorer | Save/restore workspace layouts |
| **Svelte Stores** | Svelte reactivity | Reactive layout state management |
| **ResizeObserver** | Modern web APIs | Efficient resize handling |

### 12.12 Future Enhancements

1. **Drag-and-Drop Reordering**: Allow dragging panes between containers
2. **Pane Maximization**: Double-click to maximize/restore a pane
3. **Layout Templates**: Predefined layouts (2x2 grid, side-by-side, etc.)
4. **Undo/Redo**: Layout change history
5. **Multi-Window Support**: Pop out panes to separate windows (Tauri)
6. **Synchronized Viewports**: Lock pan/zoom across linked charts
7. **Custom Pane Types**: Plugin system for third-party pane types
