# Chart Rendering and Layout Guidelines

## Container Width and Height - CRITICAL

**MANDATORY:** `MUST` use explicit inline styles for width and height to ensure SciChart receives proper dimensions.

### Problem

SciChart components require explicit dimensions before initialization. Using only Tailwind classes (`w-full`, `h-full`) may not provide concrete pixel values that SciChart needs, leading to:

- Charts rendering with incorrect/narrow widths
- Layout shifts during initialization
- Undefined container dimensions

### Solution

Use explicit inline styles (`style="width: 100%; height: 100%;"`) in addition to Tailwind classes to ensure containers have defined dimensions.

**Required Pattern:**

```svelte
<!-- ✅ CORRECT: Explicit inline styles + Tailwind classes -->
<div class="h-full w-full relative" style="width: 100%; height: 100%;">
  <div
    bind:this={chartContainer}
    style="width: 100%; height: 100%; position: relative; min-width: 0; min-height: 0;"
  ></div>
</div>

<!-- ❌ AVOID: Only Tailwind classes (may not provide explicit dimensions) -->
<div class="h-full w-full">
  <div bind:this={chartContainer} class="h-full w-full"></div>
</div>
```

### Layout Structure for 80/20 Split

When creating a chart editor with sidebar:

```svelte
<!-- Main container -->
<div class="h-full w-full flex flex-row overflow-hidden" style="width: 100%; height: 100%;">
  <!-- Chart Visualization (80% width) -->
  <div
    class="flex-shrink-0 flex-grow-0 overflow-hidden border-r"
    style="width: 80%; height: 100%; min-width: 0; min-height: 0;"
  >
    <ChartComponent chart={selectedChart} />
  </div>

  <!-- Sidebar (20% width) -->
  <div
    class="flex-shrink-0 flex-grow-0 border-l overflow-y-auto"
    style="width: 20%; height: 100%; min-width: 0; min-height: 0;"
  >
    <SidebarComponent />
  </div>
</div>
```

### Key Principles

1. **Explicit Dimensions**: Always use `style="width: 100%; height: 100%;"` on containers
2. **Parent Constraints**: Ensure parent containers also have explicit dimensions
3. **Flex Constraints**: Use `flex-shrink-0 flex-grow-0` to prevent flex items from resizing
4. **Overflow Handling**: Use `overflow-hidden` on chart containers, `overflow-y-auto` on sidebars
5. **Min Dimensions**: Set `min-width: 0; min-height: 0;` to prevent flex overflow issues

### Initialization Timing

Use `$effect` to wait for container dimensions before initializing SciChart:

```typescript
$effect(() => {
  if (chartContainer && !isInitialized) {
    requestAnimationFrame(() => {
      if (chartContainer) {
        const rect = chartContainer.getBoundingClientRect();
        if (rect.width > 0 && rect.height > 0) {
          void initSciChart();
        }
      }
    });
  }
});
```

This ensures:

- Container has rendered and has dimensions
- SciChart receives proper width/height values
- No layout shifts during initialization

## Visible Range Persistence

Charts should persist their visible range (zoom/pan state) to the database so users can restore their view when switching between charts.

### Storage Structure

Visible ranges are stored in `chart_config` using Builder API format:

```json
{
  "xAxes": {
    "options": {
      "visibleRange": { "min": -10, "max": 10 }
    }
  },
  "yAxes": [
    {
      "options": {
        "id": "left",
        "visibleRange": { "min": -10, "max": 10 }
      }
    }
  ]
}
```

### Default Visible Range

If no visible range is stored, use default range: `-10` to `+10` for both X and Y axes.

### Implementation Pattern

1. Save visible range\*\* when switching charts (before `lastSelectedChartId` changes)

- **Load visible range** when chart is selected (from DB or use defaults)
- **Sync visible range** during user interaction (debounced to prevent excessive writes)

See `chart-state-sync-service.ts` for implementation details.
