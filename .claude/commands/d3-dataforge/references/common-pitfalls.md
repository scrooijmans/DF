# Common Pitfalls in D3 + Svelte 5

## Pitfall 1: Infinite Effect Loops

### Symptom
Browser console shows: `effect_update_depth_exceeded`
Component re-renders infinitely, browser becomes unresponsive.

### Cause
Using `$state` for initialization guards:

```svelte
// WRONG
let isInitialized = $state(false);

$effect(() => {
  if (isInitialized) return;
  isInitialized = true; // Triggers effect re-run!
});
```

### Fix
Use regular variable:

```svelte
// CORRECT
let isInitialized = false;

$effect(() => {
  if (isInitialized) return;
  isInitialized = true; // No reactivity, no re-run
});
```

## Pitfall 2: D3 Selections Causing Reactivity

### Symptom
Effect runs repeatedly even with guard.

### Cause
D3 reads DOM properties which Svelte tracks:

```svelte
$effect(() => {
  if (!svgElement) return;
  // D3 reads svgElement properties - creates dependency!
  const svg = d3.select(svgElement);
  svg.selectAll('circle').data(data);
});
```

### Fix
Wrap D3 code in `untrack()`:

```svelte
import { untrack } from 'svelte';

$effect(() => {
  if (!svgElement || isInitialized) return;

  untrack(() => {
    isInitialized = true;
    const svg = d3.select(svgElement);
    svg.selectAll('circle').data(data);
  });
});
```

## Pitfall 3: Memory Leaks

### Symptom
Memory usage grows on repeated mount/unmount.
Event handlers fire on unmounted components.

### Cause
Missing cleanup in `$effect`:

```svelte
$effect(() => {
  const brush = d3.brush().on('brush', handleBrush);
  d3.select(svgElement).call(brush);
  // No cleanup!
});
```

### Fix
Return cleanup function:

```svelte
$effect(() => {
  const brush = d3.brush().on('brush', handleBrush);
  d3.select(svgElement).call(brush);

  return () => {
    isInitialized = false;
    d3.select(svgElement).select('.brush').remove();
    brush.on('brush', null); // Remove listener
  };
});
```

## Pitfall 4: Stale Data in Callbacks

### Symptom
Event handlers (brush, zoom) use outdated data values.

### Cause
Closures capture values at creation time:

```svelte
$effect(() => {
  const brush = d3.brush().on('brush', () => {
    // `data` is stale - captured when effect ran
    console.log(data);
  });
});
```

### Fix
Access current values via getter or use `untrack`:

```svelte
// Option 1: Store reference that updates
let currentData = $state(data);
$effect(() => { currentData = data; });

// In callback:
brush.on('brush', () => {
  console.log(currentData); // Current value
});

// Option 2: Re-create on data change
$effect(() => {
  const currentData = data; // Capture current
  brush.on('brush', () => {
    console.log(currentData);
  });
});
```

## Pitfall 5: Lost Reactivity After Extraction

### Symptom
Extracted component doesn't update when parent data changes.

### Cause
Passing non-reactive values to child:

```svelte
<!-- Parent -->
<Child config={pane.config} />

<!-- Child expects reactive updates but gets static value -->
```

### Fix
Use `$derived` or ensure props are reactive:

```svelte
<!-- Parent: derive values -->
{@const derivedConfig = pane.config}
<Child config={derivedConfig} />

<!-- Or in child, derive from props -->
<script>
  let { config } = $props();
  let derivedValue = $derived(config.someField);
</script>
```

## Pitfall 6: TypeScript Errors with D3 Scales

### Symptom
TypeScript complains about scale return types.

### Cause
D3 scale return types are complex unions.

### Fix
Use explicit type assertions or intermediate variables:

```typescript
// Explicit return type
let xScale = $derived(d3.scaleLinear()
  .domain([0, 100])
  .range([0, width])) as d3.ScaleLinear<number, number>;

// Or use utility function with types
function createXScale(domain: [number, number], range: [number, number]) {
  return d3.scaleLinear().domain(domain).range(range);
}
```

## Debugging Checklist

1. **Infinite loop?**
   - Check all `$state` guards → convert to regular `let`
   - Wrap D3 code in `untrack()`

2. **Not updating?**
   - Verify props are reactive
   - Check `$derived` dependencies
   - Ensure cleanup resets `isInitialized`

3. **Memory growing?**
   - Add cleanup to `$effect`
   - Remove event listeners
   - Destroy D3 objects

4. **Stale data?**
   - Check closure captures
   - Use current state references
