# Svelte 5 + D3.js Integration Patterns

## Pattern 1: Declarative SVG (Recommended)

Use D3 for calculations, let Svelte render the SVG. This is the safest pattern.

```svelte
<script lang="ts">
  import * as d3 from 'd3';

  interface Props {
    data: Array<{ x: number; y: number }>;
    width?: number;
    height?: number;
  }

  let { data, width = 400, height = 300 }: Props = $props();

  // Layout constants
  const MARGIN = { top: 20, right: 20, bottom: 30, left: 40 };
  let chartWidth = $derived(width - MARGIN.left - MARGIN.right);
  let chartHeight = $derived(height - MARGIN.top - MARGIN.bottom);

  // D3 scales as $derived - pure functions, safe to recompute
  let xScale = $derived(d3.scaleLinear()
    .domain(d3.extent(data, d => d.x) as [number, number])
    .range([0, chartWidth])
    .nice());

  let yScale = $derived(d3.scaleLinear()
    .domain(d3.extent(data, d => d.y) as [number, number])
    .range([chartHeight, 0])
    .nice());

  // Line generator as $derived
  let linePath = $derived(d3.line<{ x: number; y: number }>()
    .x(d => xScale(d.x))
    .y(d => yScale(d.y))(data));

  // Axis tick values as $derived
  let xTicks = $derived(xScale.ticks(5));
  let yTicks = $derived(yScale.ticks(5));
</script>

<svg {width} {height}>
  <g transform="translate({MARGIN.left}, {MARGIN.top})">
    <!-- X-axis -->
    {#each xTicks as tick}
      <g transform="translate({xScale(tick)}, {chartHeight})">
        <line y2="6" stroke="currentColor" />
        <text y="9" dy="0.71em" text-anchor="middle">{tick}</text>
      </g>
    {/each}

    <!-- Y-axis -->
    {#each yTicks as tick}
      <g transform="translate(0, {yScale(tick)})">
        <line x2="-6" stroke="currentColor" />
        <text x="-9" dy="0.32em" text-anchor="end">{tick}</text>
      </g>
    {/each}

    <!-- Data line -->
    <path d={linePath} fill="none" stroke="steelblue" stroke-width="1.5" />

    <!-- Data points -->
    {#each data as point}
      <circle cx={xScale(point.x)} cy={yScale(point.y)} r="3" fill="steelblue" />
    {/each}
  </g>
</svg>
```

**Why this works:**
- `$derived` for scales/generators only recalculates when inputs change
- Svelte's reactive rendering handles DOM updates efficiently
- No D3 selections or DOM manipulation = no conflicts

## Pattern 2: Imperative D3 (For Complex Interactions)

When you need D3's transitions, brushes, or zoom, use this pattern carefully.

```svelte
<script lang="ts">
  import { untrack } from 'svelte';
  import * as d3 from 'd3';

  interface Props {
    data: Array<{ x: number; y: number }>;
    width?: number;
    height?: number;
  }

  let { data, width = 400, height = 300 }: Props = $props();

  let svgElement: SVGSVGElement;

  // CRITICAL: Use regular variable, NOT $state
  let isInitialized = false;

  $effect(() => {
    if (!svgElement || isInitialized) return;

    // CRITICAL: Wrap in untrack to prevent reactive tracking
    untrack(() => {
      isInitialized = true;

      const svg = d3.select(svgElement);

      // D3 imperative setup (brushes, zoom, etc.)
      const brush = d3.brush()
        .extent([[0, 0], [width, height]])
        .on('brush', brushed);

      svg.append('g')
        .attr('class', 'brush')
        .call(brush);
    });

    // CRITICAL: Return cleanup function
    return () => {
      isInitialized = false;
      // Clean up D3 elements
      d3.select(svgElement).select('.brush').remove();
    };
  });

  function brushed(event: d3.D3BrushEvent<unknown>) {
    // Handle brush events
  }
</script>

<svg bind:this={svgElement} {width} {height}>
  <!-- Base structure, D3 will add interactive elements -->
</svg>
```

**Critical Rules:**

1. **Use `let isInitialized = false;` NOT `let isInitialized = $state(false);`**
   - `$state` makes the variable reactive
   - Assigning `isInitialized = true` triggers the effect to re-run
   - Creates infinite loop

2. **Always wrap D3 setup in `untrack()`**
   - Prevents Svelte from tracking D3's DOM reads
   - D3 reads/writes DOM which creates reactive dependencies
   - Without untrack, any D3 DOM access triggers effect re-run

3. **Always return a cleanup function**
   - Reset `isInitialized = false` so component can re-mount
   - Remove D3-created elements
   - Destroy D3 objects (brushes, zoom behaviors)

## Pattern 3: Hybrid (Best of Both)

Use declarative for rendering, imperative for interactions.

```svelte
<script lang="ts">
  import { untrack } from 'svelte';
  import * as d3 from 'd3';

  let { data, width = 400, height = 300 } = $props();

  // Declarative: scales and paths
  let xScale = $derived(d3.scaleLinear().domain([0, 100]).range([0, width]));
  let yScale = $derived(d3.scaleLinear().domain([0, 100]).range([height, 0]));
  let linePath = $derived(d3.line().x(d => xScale(d.x)).y(d => yScale(d.y))(data));

  // Imperative: zoom behavior
  let svgElement: SVGSVGElement;
  let isInitialized = false;
  let transform = $state(d3.zoomIdentity);

  $effect(() => {
    if (!svgElement || isInitialized) return;

    untrack(() => {
      isInitialized = true;

      const zoom = d3.zoom()
        .scaleExtent([0.5, 5])
        .on('zoom', (event) => {
          transform = event.transform;
        });

      d3.select(svgElement).call(zoom);
    });

    return () => {
      isInitialized = false;
    };
  });
</script>

<svg bind:this={svgElement} {width} {height}>
  <g transform={transform.toString()}>
    <!-- Declarative SVG rendered with Svelte -->
    <path d={linePath} fill="none" stroke="steelblue" />
  </g>
</svg>
```

## Common Mistakes

### ❌ Mistake 1: Using $state for guards

```svelte
// WRONG - causes infinite loop
let isInitialized = $state(false);

$effect(() => {
  if (isInitialized) return;
  isInitialized = true; // This triggers effect re-run!
});
```

### ❌ Mistake 2: D3 selections without untrack

```svelte
// WRONG - D3 reads DOM, creates reactive dependency
$effect(() => {
  const svg = d3.select(svgElement);
  svg.selectAll('circle').data(data).join('circle');
});
```

### ❌ Mistake 3: Missing cleanup

```svelte
// WRONG - memory leak, stale state
$effect(() => {
  d3.select(svgElement).call(d3.brush());
  // No return cleanup!
});
```

### ✅ Correct Pattern

```svelte
let isInitialized = false; // Regular variable

$effect(() => {
  if (!svgElement || isInitialized) return;

  untrack(() => {
    isInitialized = true;
    d3.select(svgElement).call(d3.brush());
  });

  return () => {
    isInitialized = false;
    d3.select(svgElement).select('.brush').remove();
  };
});
```
