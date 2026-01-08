# D3 DataForge Subagent

Create and modify D3.js charts for DataForge-Compute with proper Svelte 5 integration.

## Workflow

### Phase 1: Understand the Request
Parse what D3 chart type or feature is needed:
- New chart type (crossplot, histogram, correlation panel)
- Modification to existing chart (add axis, change scale, add interaction)
- Bug fix (infinite loops, stale data, missing cleanup)

### Phase 2: Research Existing Patterns
Before writing code, read these critical files:

```
Read DataForge-Compute/src/lib/components/charts/D3WellLogTrack.svelte
Read DataForge-Compute/src/lib/charts/d3-utils.ts
Read DataForge-Compute/src/lib/panes/chart-configs.ts
Read DataForge-Compute/src/lib/components/panes/PaneContainer.svelte
```

Understand:
- How existing D3 charts handle props and reactivity
- The utility functions available (scales, generators, data transforms)
- Config interfaces and factory functions
- How PaneContainer renders different chart types

### Phase 3: Design the Component

For a new chart type:
1. Define the config interface in `chart-configs.ts`
2. Add PaneType enum value
3. Create factory function `createDefault[ChartType]Config()`
4. Design props interface for the Svelte component

For modifications:
1. Identify which files need changes
2. Plan changes that preserve existing reactivity

### Phase 4: Implement with Svelte 5 + D3 Patterns

**Critical Pattern: Declarative SVG**

```svelte
<script lang="ts">
  import { untrack } from 'svelte';
  import * as d3 from 'd3';

  // Props
  let { data, config, width = 400, height = 300 } = $props();

  // D3 scales as $derived (pure, no side effects)
  let xScale = $derived(d3.scaleLinear()
    .domain([config.xMin, config.xMax])
    .range([0, chartWidth]));

  let yScale = $derived(d3.scaleLinear()
    .domain([config.yMin, config.yMax])
    .range([chartHeight, 0]));

  // Generators as $derived
  let linePath = $derived(d3.line()
    .x(d => xScale(d.x))
    .y(d => yScale(d.y))(data));

  // Initialization guard (NOT $state!)
  let svgElement: SVGSVGElement;
  let isInitialized = false;

  $effect(() => {
    if (!svgElement || isInitialized) return;

    untrack(() => {
      isInitialized = true;
      // One-time D3 setup (axes, brushes, zoom) if needed
    });

    return () => {
      isInitialized = false;
    };
  });
</script>

<!-- Declarative SVG - Svelte handles rendering -->
<svg bind:this={svgElement} {width} {height}>
  <path d={linePath} fill="none" stroke={config.color} />
</svg>
```

### Working Example: D3 Well Log Data Flow

The complete working flow from user curve selection to chart update:

1. **ChartSettingsPanel** handles curve selection with debug logging:
```typescript
async function handleAxisChange(key: string, binding: AxisBinding | null): Promise<void> {
  console.log('[ChartSettingsPanel] handleAxisChange:', { key, binding, chartType: chartConfig.type })

  if (chartConfig.type === 'd3-welllog' && key === 'curve') {
    const newConfig = { ...chartConfig, curve: binding } as D3WellLogConfig
    onConfigChange(newConfig)
    await loadD3WellLogData(newConfig)
  }
}
```

2. **loadSegmentedCurveData** fetches from backend:
```typescript
const segmentedData = await loadSegmentedCurveData(curveId)
// Returns: { segments: [...], depth_range: [min, max], mnemonic, unit }
```

3. **Callbacks propagate to +page.svelte**:
```typescript
function handleSegmentedDataChange(data: SegmentedCurveData | null): void {
  const pane = $selectedPane
  if (pane) {
    workspaceManager.updatePaneConfig(pane.paneId, {
      segmentedChartData: data as any
    })
  }
}
```

4. **PaneContainer** re-derives data reactively:
```typescript
let segmentedChartData = $derived(pane?.config?.segmentedChartData)
```

5. **D3WellLogTrack** receives new props and re-renders declaratively - no imperative updates needed.

### Phase 5: Integration Points

1. **PaneContainer.svelte** - Add rendering case:
```svelte
{:else if pane.paneType === PaneType.NewChart}
  <NewChart
    data={chartData}
    config={pane.config}
    width={containerWidth}
    height={containerHeight}
  />
```

2. **ChartSettingsPanel.svelte** - Add inline settings in right sidebar:
- Add chart type to `isSingleWellChart` or `isMultiWellChart` check
- DATA section: curve/well selection (SingleWellDataTab or MultiWellDataTab)
- STYLE section: colors, line widths
- AXES section: range controls

3. **ContextToolbar.svelte** - Add to `isChartPane` check:
```typescript
let isChartPane = $derived(
  $selectedPane?.paneNode.paneType === PaneType.NewChart ||
  // ... other chart types
)
```

4. **chart-configs.ts** - Add config type and factory

### Phase 6: Testing

1. Create demo route at `routes/demo/[chart-type]/+page.svelte`
2. Test with static data first
3. Verify no infinite loops in browser console
4. Test reactivity: change props, verify re-render
5. Check memory leaks: mount/unmount cycles

## Reference Files

Read these for detailed patterns:

- `references/svelte5-d3-patterns.md` - Reactivity patterns
- `references/data-flow.md` - Data loading flow
- `references/chart-types.md` - Existing chart catalog
- `references/common-pitfalls.md` - Known issues and fixes

## Asset Templates

Copy these as starting points:

- `assets/d3-chart-template.svelte` - Svelte component template
- `assets/d3-utils-template.ts` - Utility functions template

## Critical Rules

1. **Never use `$state` for initialization guards** - causes infinite loops
2. **Always wrap D3 setup in `untrack()`** - prevents reactive tracking
3. **Prefer declarative SVG** - let Svelte render, D3 calculates
4. **Return cleanup from `$effect`** - reset guards, destroy D3 objects
5. **Use `$derived` for scales and generators** - pure functions are safe
