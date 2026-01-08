---
name: d3-charts
description: Creating custom D3.js charts and visualizations for DataForge applications. Use this skill when building geoscience visualizations like well log tracks, crossplots, correlation panels, or any custom SVG-based charts requiring fine-grained control.
---

# D3.js Charts for DataForge

## Overview

This skill provides guidance for creating custom D3.js visualizations in the DataForge platform. Use D3.js when you need precise SVG control, custom fills/patterns, or publication-quality geoscience graphics.

## When to Use D3.js vs ECharts

**Use D3.js for:**
- Well log tracks with custom fills (sand/shale indicators)
- Lithology columns with patterns
- Formation tops and markers
- Custom annotations and labels
- Crossover shading between curves
- Publication-quality exports

**Use ECharts for:**
- Standard crossplots and scatter charts
- High-performance rendering of large datasets (>10K points)
- Built-in zoom/pan interactions
- Linked cursor synchronization (already implemented)

## DataForge D3 Components

### D3WellLogTrack

Location: `DataForge-Compute/src/lib/components/charts/D3WellLogTrack.svelte`

A well log track component using D3.js for SVG rendering:

```svelte
<D3WellLogTrack
  data={grData}
  depthRange={{ min: 2000, max: 2200 }}
  config={{
    title: 'Gamma Ray',
    unit: 'gAPI',
    xMin: 0,
    xMax: 150,
    curveColor: '#22c55e',
    fillColor: '#ffff99',
    fillDirection: 'left',
    lineWidth: 1.5
  }}
  width={200}
  height={600}
  showLithologyLabels={true}
  grCutoff={75}
/>
```

### D3 Utilities

Location: `DataForge-Compute/src/lib/charts/d3-utils.ts`

Provides:
- `createDepthScale()` - Inverted Y scale for depth
- `createValueScale()` - X scale for curve values (linear/log)
- `createCurveLineGenerator()` - D3 line generator
- `createCurveAreaGenerator()` - D3 area generator for fills
- `segmentsToDataPoints()` - Convert backend data format
- `detectLithologyZones()` - Auto-detect sand/shale from GR

## Creating New D3 Charts

### Step 1: Import D3

```typescript
import * as d3 from 'd3'
```

### Step 2: Use Svelte 5 Patterns

```svelte
<script lang="ts">
  import { untrack } from 'svelte'
  import * as d3 from 'd3'

  let svgElement: SVGSVGElement
  let isInitialized = $state(false)

  // Use $derived for reactive scales
  let xScale = $derived(d3.scaleLinear()
    .domain([xMin, xMax])
    .range([0, width]))

  // Use $effect for D3 DOM manipulation
  $effect(() => {
    if (!svgElement || isInitialized) return
    isInitialized = true

    untrack(() => {
      // D3 initialization here
      d3.select(svgElement)
        .append('g')
        .attr('class', 'chart-area')
    })

    return () => {
      isInitialized = false
    }
  })
</script>

<svg bind:this={svgElement}>
  <!-- SVG content -->
</svg>
```

### Step 3: Declarative SVG Rendering

For simpler charts, use Svelte's built-in SVG support with D3 for calculations only:

```svelte
<svg width={width} height={height}>
  <!-- Use D3-calculated path data -->
  <path d={linePath} fill="none" stroke="green" />

  <!-- Use Svelte iteration for grid lines -->
  {#each gridLines as line}
    <line x1={line.x1} y1={line.y1} x2={line.x2} y2={line.y2} />
  {/each}
</svg>
```

## Data Formats

### SegmentedCurveData (from Rust backend)

```typescript
interface SegmentedCurveData {
  curve_id: string
  mnemonic: string
  unit: string | null
  segments: Array<{
    depth_start: number
    depth_end: number
    depths: number[]
    values: number[]
  }>
  depth_range: [number, number]
  total_points: number
}
```

### WellLogDataPoint (for D3)

```typescript
interface WellLogDataPoint {
  depth: number
  value: number
}
```

Use `segmentsToDataPoints()` to convert between formats.

## References

For detailed D3.js patterns, see:
- `.claude/commands/claude-d3js-skill-main/SKILL.md` - Full D3 skill documentation
- `.claude/commands/claude-d3js-skill-main/references/d3-patterns.md` - Chart patterns
- `.claude/commands/claude-d3js-skill-main/references/scale-reference.md` - Scale guide
- `.claude/commands/claude-d3js-skill-main/references/colour-schemes.md` - Color palettes

## Demo Page

View the working demo at:
```
http://localhost:1420/demo/d3-welllog
```

Run with: `cd DataForge-Compute && bun tauri dev`
