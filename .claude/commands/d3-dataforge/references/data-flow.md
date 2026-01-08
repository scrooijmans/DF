# Data Flow in DataForge-Compute Charts

## Overview

```
User Clicks Pane → selectionContext.selectPane() → ContextToolbar shows ChartSettingsPanel
                                                  → User changes settings
                                                  → Callbacks → workspaceManager.updatePaneConfig()
                                                  → PaneContainer re-renders D3 Component
```

## Architecture

### Inline Settings Panel (Primary)

Settings are displayed inline in the right sidebar when a chart pane is selected:

```
WorkspaceContainer → LayoutRenderer → PaneContainer (user clicks)
                                           ↓
                                    selectionContext.selectPane()
                                           ↓
                     +page.svelte ← ContextToolbar ← ChartSettingsPanel
                                           ↓
                     workspaceManager.updatePaneConfig()
                                           ↓
                     PaneContainer receives updated config → D3 Component
```

### Key Components

| Component | Location | Purpose |
|-----------|----------|---------|
| `ChartSettingsPanel` | `src/lib/components/panes/` | Inline settings in right sidebar |
| `ContextToolbar` | `src/lib/components/panes/` | Context-sensitive right panel |
| `SingleWellDataTab` | `src/lib/components/charts/settings/data-tabs/` | Well/curve selection UI |
| `D3WellLogStyleSection` | `src/lib/components/charts/settings/style/` | D3 chart styling options |

## Step-by-Step Flow

### 1. User Selects Chart Pane

**Location:** PaneContainer handles click focus

**Trigger:**
```svelte
<!-- PaneContainer.svelte -->
<div
  class="pane-container"
  onfocus={handleFocus}
  role="button"
  tabindex="0"
>
```

**Result:** `selectionContext.selectPane(pane.id, pane, chartConfig)`

### 2. ContextToolbar Shows Inline Settings

**Location:** `ContextToolbar.svelte` reacts to selection

```svelte
{:else if $selectionType === 'pane' && $selectedPane && isChartPane}
  <ChartSettingsPanel
    pane={$selectedPane.paneNode}
    config={$selectedPane.chartConfig ?? null}
    {wells}
    {curves}
    {well}
    onWellChange={(wellId) => onWellChange?.(wellId)}
    onConfigChange={(config) => onConfigChange?.(config)}
    onSegmentedDataChange={(data) => onSegmentedDataChange?.(data)}
  />
{/if}
```

### 3. User Changes Settings in Panel

**Location:** `ChartSettingsPanel.svelte` → `SingleWellDataTab.svelte`

**Key Functions:**
```typescript
// Handle curve selection
async function handleAxisChange(key: string, binding: AxisBinding | null): Promise<void> {
  if (chartConfig.type === 'd3-welllog' && key === 'curve') {
    const newConfig = { ...chartConfig, curve: binding } as D3WellLogConfig;
    onConfigChange(newConfig);
    await loadD3WellLogData(newConfig);
  }
}

// Load segmented data from backend
async function loadD3WellLogData(config: D3WellLogConfig): Promise<void> {
  const curveId = config.curve?.curveId;
  if (!curveId) {
    onSegmentedDataChange?.(null);
    return;
  }

  const segmentedData = await loadSegmentedCurveData(curveId);
  onSegmentedDataChange?.(segmentedData);
}
```

### 4. Callbacks Flow Through Workbench Page

**Location:** `+page.svelte` callback handlers

```typescript
// Handle chart config changes from inline settings panel
function handleConfigChange(config: ChartConfiguration): void {
  const pane = $selectedPane;
  if (pane) {
    selectionContext.updatePaneConfig(config);
    workspaceManager.updatePaneConfig(pane.paneId, {
      chartConfig: config as any
    });
  }
}

// Handle segmented data changes from inline settings panel
function handleSegmentedDataChange(data: SegmentedCurveData | null): void {
  const pane = $selectedPane;
  if (pane) {
    workspaceManager.updatePaneConfig(pane.paneId, {
      segmentedChartData: data as any
    });
  }
}
```

### 5. PaneContainer Renders D3 Component

**Location:** `PaneContainer.svelte` template

Charts require real data - there is no demo/fallback data. If no data is loaded yet, an empty state is shown:

```svelte
{:else if pane.paneType === PaneType.D3WellLog}
  {@const d3Config = chartConfig as D3WellLogConfig}

  {#if segmentedChartData && segmentedChartData.segments?.length > 0}
    <D3WellLogTrack
      segments={segmentedChartData.segments}
      depthRange={{ min: segmentedChartData.depth_range[0], max: segmentedChartData.depth_range[1] }}
      config={{
        title: d3Config?.curve?.mnemonic ?? segmentedChartData.mnemonic ?? 'Curve',
        unit: d3Config?.curve?.unit ?? segmentedChartData.unit ?? '',
        xMin: d3Config?.xMin ?? 0,
        xMax: d3Config?.xMax ?? 150,
        curveColor: d3Config?.curveColor ?? '#22c55e',
        fillColor: d3Config?.fillColor ?? '#ffff99',
        fillDirection: d3Config?.fillDirection ?? 'left',
        lineWidth: d3Config?.lineWidth ?? 1.5
      }}
      width={containerWidth}
      height={containerHeight}
    />
  {:else}
    <!-- Empty state when no data is loaded -->
    <div class="empty-chart-state">
      <p>Select a curve to display</p>
    </div>
  {/if}
{/if}
```

**Key Pattern:** Always check for data before rendering. The config may exist before data is loaded.

## Data Formats

### SegmentedCurveData (from backend)
```typescript
interface SegmentedCurveData {
  segments: Array<{ depths: number[]; values: number[] }>;
  depth_range: [number, number];
  mnemonic: string;
  unit: string;
}
```

### WellLogDataPoint (internal D3 format)
```typescript
interface WellLogDataPoint {
  depth: number;
  value: number;
}
```

### TrackConfig (D3 component config)
```typescript
interface TrackConfig {
  title: string;
  unit: string;
  xMin: number;
  xMax: number;
  curveColor: string;
  fillColor: string | null;
  fillDirection: 'left' | 'right' | 'none';
  logScale?: boolean;
  lineWidth?: number;
}
```

## Adding a New Chart Type

1. **Define config in `chart-configs.ts`**
2. **Add PaneType enum value in `layout-model.ts`**
3. **Update `isChartPane` check in `ContextToolbar.svelte`**
4. **Add data tab support in `ChartSettingsPanel.svelte`**
5. **Add rendering in `PaneContainer.svelte`**
6. **Create D3 component**

## Legacy: ChartSettingsDialog

The modal `ChartSettingsDialog` still exists in `WorkspaceContainer.svelte` but is no longer the primary way to edit settings. It remains for advanced settings or complex chart types (correlation, multi-well charts) that haven't been fully migrated to inline editing yet.
