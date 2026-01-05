# SciChart Modifiers & State Management Architecture

## Executive Summary

**Goal**: Create a modular, type-safe architecture for SciChart modifiers and chart state management that integrates seamlessly with our PostgreSQL database, following the Builder API pattern and maintaining consistency with our UDF registry architecture.

**Key Principles**:
- ✅ **Single Source of Truth**: Chart type defaults in `chart_types` table
- ✅ **Type Safety**: TypeScript types matching Builder API JSON structure
- ✅ **State Persistence**: Track visible ranges, series configs, curve references in database
- ✅ **Modular Design**: Shared modifier utilities reusable across chart components
- ✅ **Builder API Integration**: Use SciChart Builder API JSON for chart definitions
- ✅ **Incremental State Sync**: Save chart state (visible ranges) as user interacts

---

## Architecture Overview

### Current State Analysis

**Existing Schema** (`013-charts-schema.sql`):
- `charts.chart_config` JSONB - Stores chart configuration
- `charts.data_source_config` JSONB - Stores data source references
- `charts.rendering_config` JSONB - Stores rendering settings

**Existing Chart Types** (`014-chart-types-registry.sql`):
- `chart_types.default_config` JSONB - Default chart configuration
- `chart_types.default_data_source_config` JSONB - Default data source config

**Current Implementation** (`SciXYChart.svelte`):
- Hardcoded modifiers in component
- No state persistence for visible ranges
- No curve ID references in series
- Manual axis configuration

### Proposed Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              CHARTS MODIFIERS & STATE ARCHITECTURE          │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Database Layer (PostgreSQL)                            │
│     ┌──────────────────────────────────────────────────┐    │
│     │ chart_types.default_builder_config (JSONB)     │    │
│     │   → Builder API JSON template                   │    │
│     │   → Includes default modifiers                 │    │
│     │                                                 │    │
│     │ charts.chart_config (JSONB)                     │    │
│     │   → Builder API JSON (current state)           │    │
│     │   → Includes modifiers, axes, series           │    │
│     │   → Visible ranges synced from chart           │    │
│     │                                                 │    │
│     │ chart_series (NEW TABLE)                        │    │
│     │   → Many-to-many: charts ↔ curves             │    │
│     │   → Series configuration per curve             │    │
│     └──────────────────────────────────────────────────┘    │
│           │                                                  │
│           │ (type-safe queries)                              │
│           ▼                                                  │
│  2. Type Layer (TypeScript)                                │
│     ┌──────────────────────────────────────────────────┐    │
│     │ Builder API Types                                │    │
│     │   → ISciChart2DDefinition                       │    │
│     │   → Modifier definitions                        │    │
│     │   → Series definitions                          │    │
│     │                                                 │    │
│     │ Chart State Types                               │    │
│     │   → ChartConfig                                 │    │
│     │   → SeriesConfig                                │    │
│     │   → ModifierConfig                              │    │
│     └──────────────────────────────────────────────────┘    │
│           │                                                  │
│           │ (type-safe operations)                           │
│           ▼                                                  │
│  3. Service Layer (TypeScript)                             │
│     ┌──────────────────────────────────────────────────┐    │
│     │ Chart Builder Service                           │    │
│     │   → buildChartFromConfig()                     │    │
│     │   → mergeDefaultsWithState()                   │    │
│     │                                                 │    │
│     │ Modifier Builder Service                       │    │
│     │   → createDefaultModifiers()                   │    │
│     │   → buildModifierFromConfig()                   │    │
│     │                                                 │    │
│     │ Chart State Sync Service                       │    │
│     │   → syncVisibleRanges()                        │    │
│     │   → syncSeriesConfig()                         │    │
│     └──────────────────────────────────────────────────┘    │
│           │                                                  │
│           │ (reactive state)                                 │
│           ▼                                                  │
│  4. Component Layer (Svelte)                               │
│     ┌──────────────────────────────────────────────────┐    │
│     │ SciXYChart.svelte                                │    │
│     │   → Uses Builder API                            │    │
│     │   → Listens to state changes                    │    │
│     │   → Syncs visible ranges                        │    │
│     │                                                 │    │
│     │ SciWellLogChart.svelte (future)                │    │
│     │   → Reuses modifier utilities                   │    │
│     │   → Same state sync pattern                     │    │
│     └──────────────────────────────────────────────────┘    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Database Schema Enhancements

### 1. Enhance `chart_types` Table

**Add `default_builder_config` JSONB field** to store Builder API JSON templates:

```sql
-- Migration: Add default_builder_config to chart_types
ALTER TABLE public.chart_types
ADD COLUMN IF NOT EXISTS default_builder_config JSONB DEFAULT '{}'::jsonb;

COMMENT ON COLUMN public.chart_types.default_builder_config IS 
'Default Builder API JSON configuration template for this chart type. 
Used with chartBuilder.build2DChart() to initialize new charts.
Includes default modifiers, axes, series structure, and styling.';
```

**Example `default_builder_config` for XY chart**:

```json
{
  "surface": {
    "theme": {
      "type": "Custom"
    }
  },
  "xAxes": {
    "type": "NumericAxis",
    "options": {
      "axisTitle": "X Axis",
      "growBy": { "min": 0.1, "max": 0.1 },
      "visibleRange": { "min": 0, "max": 10 }
    }
  },
  "yAxes": {
    "type": "NumericAxis",
    "options": {
      "axisTitle": "Y Axis",
      "growBy": { "min": 0.1, "max": 0.1 },
      "visibleRange": { "min": 0, "max": 10 }
    }
  },
  "series": [],
  "modifiers": [
    {
      "type": "MouseWheelZoom"
    },
    {
      "type": "ZoomPan",
      "options": {
        "executeCondition": {
          "button": "MouseRightButton"
        }
      }
    },
    {
      "type": "RubberBandXyZoom"
    },
    {
      "type": "ZoomExtents",
      "options": {
        "isAnimated": false
      }
    }
  ]
}
```

### 2. Enhance `charts.chart_config` Structure

**Current structure** (from `013-charts-schema.sql`):
```json
{
  "xAxis": { "type": "numeric", "title": "Depth (m)" },
  "yAxes": [{ "id": "left", "type": "numeric", "title": "GR (API)" }],
  "series": [
    { "dataSource": "node-123", "curve": "GR", "yAxisId": "left", "color": "#FF0000" }
  ],
  "zoom": { "xMin": 1000, "xMax": 2000 },
  "pan": { "x": 0, "y": 0 }
}
```

**Enhanced structure** (Builder API compatible):
```json
{
  "surface": {
    "theme": { "type": "Custom" }
  },
  "xAxes": {
    "type": "NumericAxis",
    "options": {
      "axisTitle": "Depth (m)",
      "visibleRange": { "min": 1000, "max": 2000 },
      "growBy": { "min": 0.1, "max": 0.1 }
    }
  },
  "yAxes": [
    {
      "type": "NumericAxis",
      "options": {
        "id": "left",
        "axisTitle": "GR (API)",
        "visibleRange": { "min": 0, "max": 150 },
        "growBy": { "min": 0.1, "max": 0.1 }
      }
    }
  ],
  "series": [
    {
      "type": "LineSeries",
      "options": {
        "stroke": "#f59e0b",
        "strokeThickness": 2,
        "yAxisId": "left"
      },
      "xyData": {
        "curveId": "uuid-of-curve-from-curves-table",
        "xColumn": "depth",
        "yColumn": "gr"
      }
    }
  ],
  "modifiers": [
    {
      "type": "MouseWheelZoom",
      "enabled": true
    },
    {
      "type": "ZoomPan",
      "enabled": true,
      "options": {
        "executeCondition": {
          "button": "MouseRightButton"
        }
      }
    },
    {
      "type": "RubberBandXyZoom",
      "enabled": true
    },
    {
      "type": "ZoomExtents",
      "enabled": true,
      "options": {
        "isAnimated": false
      }
    }
  ]
}
```

### 3. Create `chart_series` Table (Many-to-Many: Charts ↔ Curves)

**Purpose**: Track which curves are displayed in which series of a chart, enabling:
- Query "which charts display curve X?"
- Track series-specific configuration (color, yAxisId, etc.)
- Maintain curve references separate from Builder API JSON

```sql
-- Migration: Create chart_series table
CREATE TABLE IF NOT EXISTS public.chart_series (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chart_id UUID NOT NULL REFERENCES public.charts(id) ON DELETE CASCADE,
    curve_id UUID NOT NULL REFERENCES public.curves(id) ON DELETE CASCADE,
    
    -- Series configuration (overrides chart_config.series[].options)
    series_config JSONB DEFAULT '{}'::jsonb,
    -- Example structure:
    -- {
    --   "seriesIndex": 0,           -- Index in chart_config.series array
    --   "seriesId": "series-1",    -- Unique ID for this series
    --   "stroke": "#f59e0b",        -- Override color
    --   "strokeThickness": 2,       -- Override thickness
    --   "yAxisId": "left",          -- Which Y axis to use
    --   "visible": true,             -- Show/hide this series
    --   "dataColumnMapping": {      -- Map curve columns to series data
    --     "x": "depth",             -- X column name in curve parquet
    --     "y": "gr"                 -- Y column name in curve parquet
    --   }
    -- }
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Constraints
    CONSTRAINT chart_series_unique UNIQUE (chart_id, curve_id)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_chart_series_chart_id ON public.chart_series(chart_id);
CREATE INDEX IF NOT EXISTS idx_chart_series_curve_id ON public.chart_series(curve_id);
CREATE INDEX IF NOT EXISTS idx_chart_series_chart_curve ON public.chart_series(chart_id, curve_id);

-- RLS Policies
ALTER TABLE public.chart_series ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can view chart_series for charts they can view"
    ON public.chart_series FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM public.charts
            WHERE charts.id = chart_series.chart_id
            AND EXISTS (
                SELECT 1 FROM public.projects
                WHERE projects.id = charts.project_id
                AND (
                    projects.owner_id = auth.uid()
                    OR EXISTS (
                        SELECT 1 FROM public.project_permissions
                        WHERE project_permissions.project_id = projects.id
                        AND project_permissions.user_id = auth.uid()
                    )
                )
            )
        )
    );

CREATE POLICY "Users can manage chart_series for charts they can edit"
    ON public.chart_series FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM public.charts
            WHERE charts.id = chart_series.chart_id
            AND EXISTS (
                SELECT 1 FROM public.projects
                WHERE projects.id = charts.project_id
                AND (
                    projects.owner_id = auth.uid()
                    OR EXISTS (
                        SELECT 1 FROM public.project_permissions
                        WHERE project_permissions.project_id = projects.id
                        AND project_permissions.user_id = auth.uid()
                        AND project_permissions.role IN ('owner', 'editor')
                    )
                )
            )
        )
    );

COMMENT ON TABLE public.chart_series IS 
'Many-to-many relationship between charts and curves. Tracks which curves are displayed in which series of a chart.
Series-specific configuration (color, yAxisId, visibility) stored in series_config JSONB.';

COMMENT ON COLUMN public.chart_series.series_config IS 
'Series-specific configuration JSONB: seriesIndex, seriesId, stroke, strokeThickness, yAxisId, visible, dataColumnMapping';
```

---

## TypeScript Type Definitions

### 1. Builder API Types

**File**: `src/lib/components/pages/home/charts/types/scichart-builder.ts`

```typescript
/**
 * SciChart Builder API Type Definitions
 * 
 * These types match the Builder API JSON structure used by chartBuilder.build2DChart()
 * Provides type safety for chart configurations stored in database.
 */

/**
 * Builder API Chart Definition (matches ISciChart2DDefinition)
 */
export interface BuilderChartDefinition {
  surface?: {
    theme?: {
      type: string;
      [key: string]: any;
    };
    [key: string]: any;
  };
  xAxes?: BuilderAxisDefinition | BuilderAxisDefinition[];
  yAxes?: BuilderAxisDefinition | BuilderAxisDefinition[];
  series?: BuilderSeriesDefinition[];
  modifiers?: BuilderModifierDefinition[];
  annotations?: BuilderAnnotationDefinition[];
}

/**
 * Builder API Axis Definition
 */
export interface BuilderAxisDefinition {
  type: string; // "NumericAxis" | "CategoryAxis" | etc.
  options?: {
    id?: string;
    axisTitle?: string;
    visibleRange?: { min: number; max: number };
    growBy?: { min: number; max: number };
    axisAlignment?: string;
    labelPrecision?: number;
    [key: string]: any;
  };
}

/**
 * Builder API Series Definition
 */
export interface BuilderSeriesDefinition {
  type: string; // "LineSeries" | "ScatterSeries" | etc.
  options?: {
    id?: string;
    seriesId?: string;
    stroke?: string;
    strokeThickness?: number;
    yAxisId?: string;
    xAxisId?: string;
    visible?: boolean;
    [key: string]: any;
  };
  xyData?: {
    curveId?: string; // Reference to curves.id
    xColumn?: string; // Column name in curve parquet (e.g., "depth")
    yColumn?: string; // Column name in curve parquet (e.g., "gr")
    xValues?: number[]; // Direct data (for empty charts)
    yValues?: number[]; // Direct data (for empty charts)
  };
}

/**
 * Builder API Modifier Definition
 */
export interface BuilderModifierDefinition {
  type: string; // "MouseWheelZoom" | "ZoomPan" | "RubberBandXyZoom" | etc.
  enabled?: boolean; // Whether modifier is active (runtime only, not persisted)
  options?: {
    executeCondition?: {
      button?: string; // "MouseLeftButton" | "MouseRightButton" | "MouseMiddleButton"
      key?: string; // "Shift" | "Ctrl" | "Alt" | "None"
    };
    secondaryExecuteCondition?: {
      button?: string;
      key?: string;
    };
    stroke?: string;
    strokeThickness?: number;
    isAnimated?: boolean;
    [key: string]: any;
  };
}

/**
 * Builder API Annotation Definition
 */
export interface BuilderAnnotationDefinition {
  type: string;
  options?: Record<string, any>;
}

/**
 * Chart Config (stored in charts.chart_config)
 * This is the Builder API definition with additional metadata
 */
export interface ChartConfig extends BuilderChartDefinition {
  // Additional metadata not part of Builder API
  _metadata?: {
    version?: string; // Schema version for migration
    lastSyncedAt?: string; // ISO timestamp of last state sync
  };
}

/**
 * Series Config (stored in chart_series.series_config)
 */
export interface SeriesConfig {
  seriesIndex: number; // Index in chart_config.series array
  seriesId: string; // Unique ID for this series
  stroke?: string;
  strokeThickness?: number;
  yAxisId?: string;
  xAxisId?: string;
  visible?: boolean;
  dataColumnMapping: {
    x: string; // Column name for X values (e.g., "depth")
    y: string; // Column name for Y values (e.g., "gr")
  };
}
```

### 2. Modifier Configuration Types

**File**: `src/lib/components/pages/home/charts/types/scichart-modifiers.ts`

```typescript
/**
 * SciChart Modifier Configuration Types
 * 
 * Type-safe definitions for modifier configurations.
 */

/**
 * Default modifier configuration for a chart type
 */
export interface DefaultModifierConfig {
  type: string; // Modifier type (e.g., "MouseWheelZoom", "ZoomPan")
  enabled: boolean; // Default enabled state
  options?: Record<string, any>;
}

/**
 * Modifier registry entry
 */
export interface ModifierRegistryEntry {
  type: string;
  displayName: string;
  description: string;
  defaultEnabled: boolean;
  defaultOptions: Record<string, any>;
  category: 'zoom' | 'pan' | 'interaction' | 'display';
}

/**
 * Modifier registry
 * Centralized registry of available modifiers with their default configurations
 */
export const MODIFIER_REGISTRY: Record<string, ModifierRegistryEntry> = {
  MouseWheelZoom: {
    type: 'MouseWheelZoom',
    displayName: 'Mouse Wheel Zoom',
    description: 'Zoom chart using mouse wheel or touchpad',
    defaultEnabled: true,
    defaultOptions: {},
    category: 'zoom',
  },
  ZoomPan: {
    type: 'ZoomPan',
    displayName: 'Zoom & Pan',
    description: 'Pan chart by dragging with right mouse button',
    defaultEnabled: true,
    defaultOptions: {
      executeCondition: {
        button: 'MouseRightButton',
        key: 'None',
      },
    },
    category: 'pan',
  },
  RubberBandXyZoom: {
    type: 'RubberBandXyZoom',
    displayName: 'Rubber Band Zoom',
    description: 'Zoom by drawing a rectangle on the chart',
    defaultEnabled: true,
    defaultOptions: {},
    category: 'zoom',
  },
  ZoomExtents: {
    type: 'ZoomExtents',
    displayName: 'Zoom Extents',
    description: 'Double-click to reset zoom to data extents',
    defaultEnabled: true,
    defaultOptions: {
      isAnimated: false,
    },
    category: 'zoom',
  },
  Rollover: {
    type: 'Rollover',
    displayName: 'Rollover Tooltip',
    description: 'Show tooltip when hovering over chart',
    defaultEnabled: false,
    defaultOptions: {},
    category: 'interaction',
  },
  Cursor: {
    type: 'Cursor',
    displayName: 'Cursor Crosshair',
    description: 'Show crosshair cursor with axis labels',
    defaultEnabled: false,
    defaultOptions: {},
    category: 'interaction',
  },
  Legend: {
    type: 'Legend',
    displayName: 'Legend',
    description: 'Display legend showing series names and colors',
    defaultEnabled: false,
    defaultOptions: {},
    category: 'display',
  },
} as const;

/**
 * Get default modifiers for a chart type
 */
export function getDefaultModifiersForChartType(
  chartType: string,
): DefaultModifierConfig[] {
  // XY and well_log charts use same defaults
  if (chartType === 'xy' || chartType === 'well_log') {
    return [
      MODIFIER_REGISTRY.MouseWheelZoom,
      MODIFIER_REGISTRY.ZoomPan,
      MODIFIER_REGISTRY.RubberBandXyZoom,
      MODIFIER_REGISTRY.ZoomExtents,
    ].map((entry) => ({
      type: entry.type,
      enabled: entry.defaultEnabled,
      options: entry.defaultOptions,
    }));
  }

  // Default: return all modifiers
  return Object.values(MODIFIER_REGISTRY).map((entry) => ({
    type: entry.type,
    enabled: entry.defaultEnabled,
    options: entry.defaultOptions,
  }));
}

/**
 * Get modifier registry entry by type
 */
export function getModifierEntry(
  type: string,
): ModifierRegistryEntry | undefined {
  return Object.values(MODIFIER_REGISTRY).find((entry) => entry.type === type);
}
```

---

## Service Layer Architecture

### 1. Chart Builder Service

**File**: `src/lib/components/pages/home/charts/services/chart-builder-service.ts`

**Purpose**: Build charts from Builder API JSON, merge defaults with instance state.

```typescript
/**
 * Chart Builder Service
 * 
 * Provides utilities for building SciChart instances from Builder API JSON
 * stored in database, merging defaults with instance-specific state.
 */

import { chartBuilder } from 'scichart';
import type { BuilderChartDefinition, ChartConfig } from '../types/scichart-builder';
import type { ChartType } from '$lib/services/chart-service';

/**
 * Build a chart from Builder API JSON definition
 */
export async function buildChartFromConfig(
  container: string | HTMLDivElement,
  config: BuilderChartDefinition,
): Promise<{ sciChartSurface: any; wasmContext: any }> {
  return await chartBuilder.build2DChart(container, config);
}

/**
 * Merge chart type defaults with instance-specific config
 * Instance config takes precedence over defaults
 */
export function mergeDefaultsWithState(
  chartType: ChartType,
  instanceConfig: ChartConfig,
): BuilderChartDefinition {
  const defaultConfig = (chartType.default_builder_config || {}) as BuilderChartDefinition;
  const instanceBuilderConfig = instanceConfig as BuilderChartDefinition;

  // Deep merge: instance config overrides defaults
  // For arrays (modifiers, series), we use instance config if present, otherwise defaults
  return {
    ...defaultConfig,
    ...instanceBuilderConfig,
    // Surface: instance takes precedence
    surface: instanceBuilderConfig.surface || defaultConfig.surface,
    // Axes: instance takes precedence
    xAxes: instanceBuilderConfig.xAxes || defaultConfig.xAxes,
    yAxes: instanceBuilderConfig.yAxes || defaultConfig.yAxes,
    // Series: instance takes precedence (user may have added curves)
    series: instanceBuilderConfig.series || defaultConfig.series || [],
    // Modifiers: merge defaults with instance (instance can override defaults)
    modifiers: mergeModifiers(
      defaultConfig.modifiers || [],
      instanceBuilderConfig.modifiers || [],
    ),
    // Annotations: instance takes precedence
    annotations: instanceBuilderConfig.annotations || defaultConfig.annotations,
  };
}

/**
 * Merge modifier arrays: instance modifiers override defaults by type
 */
function mergeModifiers(
  defaults: BuilderChartDefinition['modifiers'] = [],
  instance: BuilderChartDefinition['modifiers'] = [],
): BuilderChartDefinition['modifiers'] {
  if (!instance || instance.length === 0) {
    return defaults;
  }

  // Create a map of instance modifiers by type
  const instanceMap = new Map(
    instance.map((mod) => [mod.type, mod]),
  );

  // Start with defaults, override with instance modifiers
  const merged = defaults.map((defaultMod) => {
    const instanceMod = instanceMap.get(defaultMod.type);
    if (instanceMod) {
      // Instance modifier overrides default
      return {
        ...defaultMod,
        ...instanceMod,
        options: {
          ...defaultMod.options,
          ...instanceMod.options,
        },
      };
    }
    return defaultMod;
  });

  // Add any instance modifiers not in defaults
  instance.forEach((instanceMod) => {
    if (!defaults.find((d) => d.type === instanceMod.type)) {
      merged.push(instanceMod);
    }
  });

  return merged;
}

/**
 * Get default Builder API config for a chart type
 */
export function getDefaultBuilderConfig(chartType: ChartType): BuilderChartDefinition {
  return (chartType.default_builder_config || {}) as BuilderChartDefinition;
}
```

### 2. Chart State Sync Service

**File**: `src/lib/components/pages/home/charts/services/chart-state-sync-service.ts`

**Purpose**: Sync chart state (visible ranges, series configs) to database with debouncing.

```typescript
/**
 * Chart State Sync Service
 * 
 * Handles syncing chart state (visible ranges, series configs) to database.
 * Uses debouncing to avoid excessive database writes.
 */

import { supabase } from '$lib/supabase';
import type { ChartConfig } from '../types/scichart-builder';

// Debounce delay for state sync (ms)
const SYNC_DEBOUNCE_MS = 1000;

let syncTimeouts: Map<string, NodeJS.Timeout> = new Map();

/**
 * Sync visible ranges from chart to database
 */
export async function syncVisibleRanges(
  chartId: string,
  xAxisRange: { min: number; max: number },
  yAxesRanges: Array<{ id: string; min: number; max: number }>,
): Promise<void> {
  // Clear existing timeout
  const existingTimeout = syncTimeouts.get(chartId);
  if (existingTimeout) {
    clearTimeout(existingTimeout);
  }

  // Set new timeout for debounced sync
  const timeout = setTimeout(async () => {
    try {
      // Fetch current chart config
      const { data: chart, error: fetchError } = await supabase
        .from('charts')
        .select('chart_config')
        .eq('id', chartId)
        .single();

      if (fetchError || !chart) {
        console.error('[ChartStateSync] Failed to fetch chart:', fetchError);
        return;
      }

      const config = chart.chart_config as ChartConfig;

      // Update visible ranges
      if (config.xAxes) {
        const xAxesArray = Array.isArray(config.xAxes) ? config.xAxes : [config.xAxes];
        if (xAxesArray.length > 0) {
          xAxesArray[0].options = xAxesArray[0].options || {};
          xAxesArray[0].options.visibleRange = xAxisRange;
          config.xAxes = xAxesArray.length === 1 ? xAxesArray[0] : xAxesArray;
        }
      }

      if (config.yAxes) {
        const yAxesArray = Array.isArray(config.yAxes) ? config.yAxes : [config.yAxes];
        yAxesRanges.forEach((range) => {
          const yAxis = yAxesArray.find((axis) => axis.options?.id === range.id);
          if (yAxis) {
            yAxis.options = yAxis.options || {};
            yAxis.options.visibleRange = { min: range.min, max: range.max };
          }
        });
        config.yAxes = yAxesArray.length === 1 ? yAxesArray[0] : yAxesArray;
      }

      // Update metadata
      config._metadata = config._metadata || {};
      config._metadata.lastSyncedAt = new Date().toISOString();

      // Save to database
      const { error: updateError } = await supabase
        .from('charts')
        .update({ chart_config: config })
        .eq('id', chartId);

      if (updateError) {
        console.error('[ChartStateSync] Failed to sync visible ranges:', updateError);
      } else {
        console.log('[ChartStateSync] Synced visible ranges for chart:', chartId);
      }
    } catch (error) {
      console.error('[ChartStateSync] Error syncing visible ranges:', error);
    } finally {
      syncTimeouts.delete(chartId);
    }
  }, SYNC_DEBOUNCE_MS);

  syncTimeouts.set(chartId, timeout);
}

/**
 * Cleanup sync timeouts (call on component unmount)
 */
export function cleanupSync(chartId: string): void {
  const timeout = syncTimeouts.get(chartId);
  if (timeout) {
    clearTimeout(timeout);
    syncTimeouts.delete(chartId);
  }
}
```

---

## Type Safety Architecture

### End-to-End Type Safety Flow

```
┌─────────────────────────────────────────────────────────────┐
│              TYPE SAFETY ARCHITECTURE                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Database Schema (PostgreSQL)                            │
│     ┌──────────────────────────────────────────────────┐    │
│     │ chart_types.default_builder_config (JSONB)       │    │
│     │   → Validated Builder API JSON structure         │    │
│     │   → CHECK constraints ensure valid chart_type_id  │    │
│     │                                                 │    │
│     │ charts.chart_config (JSONB)                     │    │
│     │   → Builder API JSON (validated structure)     │    │
│     │   → Foreign key to chart_types                 │    │
│     │                                                 │    │
│     │ chart_series.curve_id                          │    │
│     │   → Foreign key to curves.id                   │    │
│     │   → Ensures curve exists                       │    │
│     └──────────────────────────────────────────────────┘    │
│           │                                                  │
│           │ (type-safe queries)                              │
│           ▼                                                  │
│  2. TypeScript Types                                        │
│     ┌──────────────────────────────────────────────────┐    │
│     │ BuilderChartDefinition                          │    │
│     │   → Matches ISciChart2DDefinition              │    │
│     │   → Type-safe Builder API structure             │    │
│     │                                                 │    │
│     │ ChartConfig extends BuilderChartDefinition      │    │
│     │   → Additional metadata fields                 │    │
│     │                                                 │    │
│     │ SeriesConfig                                   │    │
│     │   → Type-safe series configuration             │    │
│     └──────────────────────────────────────────────────┘    │
│           │                                                  │
│           │ (compile-time type checking)                     │
│           ▼                                                  │
│  3. Service Layer (TypeScript)                             │
│     ┌──────────────────────────────────────────────────┐    │
│     │ chart-builder-service.ts                        │    │
│     │   → buildChartFromConfig()                      │    │
│     │   → Type-safe config merging                   │    │
│     │                                                 │    │
│     │ chart-state-sync-service.ts                    │    │
│     │   → syncVisibleRanges()                        │    │
│     │   → Type-safe state sync                       │    │
│     └──────────────────────────────────────────────────┘    │
│           │                                                  │
│           │ (runtime validation)                             │
│           ▼                                                  │
│  4. Component Layer (Svelte)                               │
│     ┌──────────────────────────────────────────────────┐    │
│     │ SciXYChart.svelte                               │    │
│     │   → Type-safe chart prop                        │    │
│     │   → Type-safe config usage                      │    │
│     │   → Type-safe state sync                        │    │
│     └──────────────────────────────────────────────────┘    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Component Integration Pattern

### Updated `SciXYChart.svelte` Pattern

```typescript
// src/lib/components/pages/home/charts/chart-editor/sci-xy-chart/sci-xy-chart.svelte

<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import type { Chart } from '$lib/state/postgres/postgres-charts-state.svelte';
  import { initializeSciChart } from '$lib/utils/scichart/scichart-init';
  import { createCustomTheme } from '$lib/utils/scichart/scichart-theme';
  import { buildChartFromConfig, mergeDefaultsWithState } from '../services/chart-builder-service';
  import { syncVisibleRanges, cleanupSync } from '../services/chart-state-sync-service';
  import { getChartTypes } from '$lib/services/chart-service';

  interface Props {
    chart: Chart;
  }

  let { chart }: Props = $props();

  let chartContainer: HTMLDivElement | null = $state(null);
  let sciChartSurface: any = null;
  let wasmContext: any = null;
  let chartType: any = null;

  async function initSciChart() {
    if (!chartContainer) return;

    try {
      await initializeSciChart();

      // Fetch chart type to get defaults
      const chartTypesResult = await getChartTypes();
      if (chartTypesResult.isOk()) {
        chartType = chartTypesResult.value.find((ct) => ct.chart_type_id === chart.chart_type);
      }

      // Merge defaults with instance config
      const builderConfig = chartType
        ? mergeDefaultsWithState(chartType, chart.chart_config)
        : chart.chart_config;

      // Apply custom theme
      builderConfig.surface = builderConfig.surface || {};
      builderConfig.surface.theme = { type: 'Custom' };

      // Build chart using Builder API
      const result = await buildChartFromConfig(chartContainer, builderConfig);
      sciChartSurface = result.sciChartSurface;
      wasmContext = result.wasmContext;

      // Apply custom theme (Builder API may not support custom themes fully)
      const customTheme = createCustomTheme();
      sciChartSurface.applyTheme(customTheme);

      // Set up visible range change listeners
      setupVisibleRangeListeners();

      console.log('[SciXYChart] Chart initialized successfully');
    } catch (error) {
      console.error('[SciXYChart] Failed to initialize chart:', error);
    }
  }

  function setupVisibleRangeListeners() {
    if (!sciChartSurface) return;

    // Listen to X axis visible range changes
    const xAxis = sciChartSurface.xAxes.get(0);
    if (xAxis) {
      xAxis.visibleRangeChanged.subscribe((range: any) => {
        syncVisibleRanges(chart.id, { min: range.min, max: range.max }, []);
      });
    }

    // Listen to Y axes visible range changes
    sciChartSurface.yAxes.asArray().forEach((yAxis: any, index: number) => {
      yAxis.visibleRangeChanged.subscribe((range: any) => {
        const yAxesRanges = [{ id: yAxis.id || `y${index}`, min: range.min, max: range.max }];
        syncVisibleRanges(chart.id, { min: 0, max: 0 }, yAxesRanges);
      });
    });
  }

  onMount(() => {
    if (chartContainer) {
      void initSciChart();
    }
  });

  onDestroy(() => {
    cleanupSync(chart.id);
    if (sciChartSurface) {
      try {
        sciChartSurface.delete();
      } catch (error) {
        console.error('[SciXYChart] Error destroying chart:', error);
      }
    }
  });
</script>

<div class="h-full w-full flex flex-col">
  <div
    bind:this={chartContainer}
    class="h-full w-full"
    data-chart-id={chart.id}
  ></div>
</div>
```

---

## Database Migration Plan

### Migration: `016-enhance-charts-builder-api-support.sql`

See `db/migrations/016-enhance-charts-builder-api-support.sql` for the complete migration script.

**Key Changes**:
1. ✅ Adds `default_builder_config` JSONB to `chart_types` table
2. ✅ Creates `chart_series` table (many-to-many: charts ↔ curves)
3. ✅ Updates existing chart types with Builder API JSON templates
4. ✅ Enables realtime for `chart_series` table

---

## Key Design Decisions

### 1. Builder API JSON as Single Source of Truth

**Decision**: Store Builder API JSON in `chart_config` and `default_builder_config`.

**Rationale**:
- ✅ SciChart Builder API is designed for serialization
- ✅ Can use `chartBuilder.build2DChart()` directly
- ✅ Supports `toJSON()` for state persistence
- ✅ Type-safe with TypeScript definitions

### 2. Modifiers Not Persisted Per Chart

**Decision**: Modifiers are part of `default_builder_config` (chart type defaults), not per-chart state.

**Rationale**:
- ✅ User can enable/disable modifiers at runtime (not persisted)
- ✅ Default modifiers defined in chart type registry
- ✅ Reduces database writes
- ✅ Simpler state management

### 3. Visible Ranges Synced to Database

**Decision**: Track visible ranges in `chart_config.xAxes.options.visibleRange` and `chart_config.yAxes[].options.visibleRange`.

**Rationale**:
- ✅ Important for user experience (restore zoom state)
- ✅ Debounced sync prevents excessive writes
- ✅ Realtime updates ensure multi-user consistency

### 4. Chart-Series Table for Curve References

**Decision**: Create separate `chart_series` table to track curve-to-series mappings.

**Rationale**:
- ✅ Normalized database design
- ✅ Query "which charts display curve X?"
- ✅ Series-specific config separate from Builder API JSON
- ✅ Foreign key constraints ensure data integrity

### 5. Debounced State Sync

**Decision**: Use 1-second debounce for visible range syncing.

**Rationale**:
- ✅ Prevents excessive database writes during zoom/pan
- ✅ Balances responsiveness with performance
- ✅ User experience not impacted (1s delay acceptable)

---

## Realtime Updates for Visible Range

Following the pattern established for nodes (`@realtime-node-creation-add-to-active-pipeline.md`), visible range updates should trigger realtime notifications:

1. **When visible range changes**: The `syncVisibleRanges()` function updates the database
2. **Realtime subscription**: Other users' charts should receive updates via Supabase Realtime
3. **State synchronization**: The `PostgresChartsState` should listen to realtime updates and update chart configs accordingly

**Implementation Notes**:
- The `charts` table is already enabled for realtime (from `013-charts-schema.sql`)
- When `chart_config` is updated, realtime subscribers will receive the change
- Components should reactively update when `chart_config` changes in global state

---

## Implementation Status

### ✅ Completed

1. ✅ Database migration `016-enhance-charts-builder-api-support.sql` created and executed
2. ✅ Type definitions created in `src/lib/components/pages/home/charts/types/`
3. ✅ Service layer created in `src/lib/components/pages/home/charts/services/`
4. ✅ Files organized in charts directory structure

### ⏳ Pending

1. ⏳ Refactor `SciXYChart.svelte` to use Builder API and state sync service
2. ⏳ Implement realtime updates for visible range changes
3. ⏳ Create chart-series service for managing curve relationships
4. ⏳ Test end-to-end: create chart → load chart → interact → verify state sync

---

## References

- **Migration Script**: `db/migrations/016-enhance-charts-builder-api-support.sql`
- **Type Definitions**: `src/lib/components/pages/home/charts/types/scichart-builder.ts`
- **Modifier Registry**: `src/lib/components/pages/home/charts/types/scichart-modifiers.ts`
- **Builder Service**: `src/lib/components/pages/home/charts/services/chart-builder-service.ts`
- **State Sync Service**: `src/lib/components/pages/home/charts/services/chart-state-sync-service.ts`
- **Realtime Pattern**: `markdown-guides/svelte5/realtime-node-creation-add-to-active-pipeline.md`

