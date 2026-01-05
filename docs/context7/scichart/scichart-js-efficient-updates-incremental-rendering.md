# SciChart.js Efficient Updates and Incremental Rendering

This document explains how SciChart.js allows for updating series data, axes data, and other chart properties without fully rerendering the chart, enabling high-performance real-time updates.

## Overview

SciChart.js uses an efficient update mechanism that:

- **Only redraws what changed** - Not the entire chart
- **Batches updates** - Multiple changes can be grouped together
- **Uses WebGL/WebAssembly** - Hardware-accelerated rendering
- **Incremental updates** - Data changes trigger minimal redraws
- **Event-driven invalidation** - Smart detection of what needs updating

## How Data Series Updates Work

### Automatic Invalidation on Data Changes

When you modify a `DataSeries`, SciChart.js automatically detects the change and schedules a redraw. The chart doesn't fully rerender - only the affected series and related elements are updated.

```typescript
import { XyDataSeries, FastLineRenderableSeries } from "scichart";

// Create data series and series
const dataSeries = new XyDataSeries(wasmContext);
const lineSeries = new FastLineRenderableSeries(wasmContext, {
  dataSeries,
  stroke: "#FF6600",
});
sciChartSurface.renderableSeries.add(lineSeries);

// Update data - automatically triggers incremental redraw
dataSeries.append(1, 10); // Only the line series redraws, not the entire chart
dataSeries.append(2, 20); // Another incremental update
dataSeries.update(0, 15); // Updates existing point - minimal redraw
```

### Data Series Update Methods

All data series update methods automatically trigger incremental redraws:

```typescript
// Append single point
dataSeries.append(x, y); // Triggers incremental redraw

// Append multiple points (more efficient)
dataSeries.appendRange(xValues, yValues); // Single redraw for all points

// Insert at specific index
dataSeries.insert(index, x, y); // Incremental redraw

// Update existing point
dataSeries.update(index, newY); // Minimal redraw - only affected point

// Update multiple points
dataSeries.updateN(startIndex, xValues, yValues); // Single redraw

// Remove point
dataSeries.removeAt(index); // Incremental redraw

// Remove range
dataSeries.removeRange(startIndex, count); // Single redraw for range
```

### Batch Updates for Performance

For multiple updates, use batch operations instead of individual calls:

```typescript
// ❌ SLOW: Multiple individual updates = multiple redraws
for (let i = 0; i < 1000; i++) {
  dataSeries.append(i, Math.sin(i)); // 1000 redraws!
}

// ✅ FAST: Single batch update = one redraw
const xValues = [];
const yValues = [];
for (let i = 0; i < 1000; i++) {
  xValues.push(i);
  yValues.push(Math.sin(i));
}
dataSeries.appendRange(xValues, yValues); // Single redraw
```

## Suspending and Resuming Updates

### Batching Multiple Changes

When making multiple changes (data updates, axis changes, property modifications), suspend updates to batch them into a single redraw:

```typescript
// Suspend updates
sciChartSurface.suspendUpdates();

try {
  // Make multiple changes - no redraws occur yet
  dataSeries.appendRange(xValues1, yValues1);
  dataSeries.appendRange(xValues2, yValues2);
  axis.visibleRange = new NumberRange(0, 100);
  lineSeries.stroke = "#00FF00";
  // ... more changes ...
} finally {
  // Resume updates - triggers single redraw for all changes
  sciChartSurface.resumeUpdates({
    invalidateOnResume: true, // Force redraw on resume
  });
}
```

### Resume Options

```typescript
// Resume with options
sciChartSurface.resumeUpdates({
  force: true, // Force immediate redraw
  invalidateOnResume: true, // Invalidate and redraw on resume
});

// Simple resume (redraws if changes were made)
sciChartSurface.resumeUpdates();
```

### Nested Suspension

`suspendUpdates()` uses a counter, so nested calls are safe:

```typescript
sciChartSurface.suspendUpdates(); // Counter = 1
// ... some changes ...
sciChartSurface.suspendUpdates(); // Counter = 2
// ... more changes ...
sciChartSurface.resumeUpdates(); // Counter = 1 (still suspended)
sciChartSurface.resumeUpdates(); // Counter = 0 (now resumes)
```

### Create Suspended Chart

Create a chart with rendering initially suspended to configure everything before the first render:

```typescript
// Create chart with rendering suspended
const { sciChartSurface } = await SciChartSurface.create(rootElement, {
  createSuspended: true, // No initial render
});

// Configure chart completely
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));
sciChartSurface.renderableSeries.add(lineSeries);
// ... all configuration ...

// Trigger first render after configuration
await sciChartSurface.nextStateRender({
  resumeBefore: true,
  invalidateOnResume: true,
  suspendAfter: false,
});
```

## Axis Updates

### Visible Range Updates

Updating axis visible ranges triggers incremental redraws - only the viewport changes, not the entire chart:

```typescript
// Update visible range - incremental redraw
axis.visibleRange = new NumberRange(0, 100);

// Animate visible range change
axis.animateVisibleRange(
  new NumberRange(0, 100),
  500, // duration in ms
  easingFunction,
  () => console.log("Animation complete"),
);
```

### Listen to Range Changes

Axis range changes fire events without full rerenders:

```typescript
axis.visibleRangeChanged.subscribe((args) => {
  console.log(
    `Range changed: ${args.visibleRange.min} to ${args.visibleRange.max}`,
  );
  // This doesn't trigger a redraw - it's just a notification
  // The axis itself already redrew incrementally
});
```

### Axis Property Updates

Updating axis properties triggers minimal redraws:

```typescript
// These updates are incremental
axis.axisTitle = "New Title"; // Only axis title redraws
axis.isVisible = false; // Only axis visibility changes
axis.labelPrecision = 2; // Only labels redraw
axis.drawMajorGridLines = false; // Only grid lines update
```

## Series Property Updates

### Visual Property Updates

Updating series visual properties triggers incremental redraws:

```typescript
// These updates are incremental - only the series redraws
lineSeries.stroke = "#00FF00"; // Only line color changes
lineSeries.strokeThickness = 3; // Only line thickness changes
lineSeries.isVisible = false; // Only visibility changes
lineSeries.opacity = 0.5; // Only opacity changes
```

### Data Series Replacement

Replacing a data series triggers a redraw, but the chart structure remains:

```typescript
// Create new data series
const newDataSeries = new XyDataSeries(wasmContext, {
  xValues: [1, 2, 3, 4, 5],
  yValues: [10, 20, 30, 40, 50],
});

// Replace data series - incremental update
lineSeries.dataSeries = newDataSeries; // Only the series redraws
```

## Manual Invalidation

### Triggering Redraws

You can manually trigger redraws when needed:

```typescript
// Invalidate and schedule redraw
sciChartSurface.invalidateElement();

// Force immediate redraw
sciChartSurface.invalidateElement({ force: true });
```

### When to Use Manual Invalidation

Manual invalidation is rarely needed since SciChart.js automatically invalidates on changes. Use it when:

- Making external changes that SciChart doesn't detect
- Forcing a redraw after custom modifications
- Debugging rendering issues

```typescript
// Example: Custom property change
myCustomSeries.customProperty = newValue;
// If SciChart doesn't detect this, manually invalidate
sciChartSurface.invalidateElement();
```

## Event-Driven Updates

### Data Changed Events

Data series fire events when data changes, allowing you to react without full rerenders:

```typescript
// Subscribe to data changes
dataSeries.dataChanged.subscribe((args) => {
  console.log(`Data changed: ${args.changeType} at index ${args.index}`);
  // React to change without triggering additional redraws
  updateCustomUI(args);
});
```

### Series Data Changed

Renderable series receive notifications when their data series changes:

```typescript
// This is called internally when data changes
// You typically don't need to call this manually
renderableSeries.dataSeriesDataChanged(args);
```

### Notify Data Changed

For custom data modifications, explicitly notify SciChart:

```typescript
// After custom data modifications
dataSeries.notifyDataChanged();

// Or with specific change details
dataSeries.notifyDataChanged({
  changeType: EDataChangeType.DataChanged,
  index: 0,
  count: 100,
});
```

## Real-Time Updates Pattern

### Efficient Real-Time Data Streaming

For real-time data, batch updates and use suspend/resume:

```typescript
let updateBuffer: { x: number; y: number }[] = [];

// Collect updates in buffer
const collectUpdate = (x: number, y: number) => {
  updateBuffer.push({ x, y });
};

// Periodically flush buffer
setInterval(() => {
  if (updateBuffer.length === 0) return;

  // Suspend updates
  sciChartSurface.suspendUpdates();

  try {
    // Extract arrays
    const xValues = updateBuffer.map((u) => u.x);
    const yValues = updateBuffer.map((u) => u.y);

    // Batch append
    dataSeries.appendRange(xValues, yValues);

    // Clear buffer
    updateBuffer = [];
  } finally {
    // Resume - single redraw for all buffered updates
    sciChartSurface.resumeUpdates({ invalidateOnResume: true });
  }
}, 100); // Flush every 100ms
```

### Reusing Float64Array Buffers

For maximum performance with frequent updates, reuse typed arrays:

```typescript
// Allocate buffers once
const BUFFER_SIZE = 1000;
const xBuffer = new Float64Array(BUFFER_SIZE);
const yBuffer = new Float64Array(BUFFER_SIZE);

// Reuse buffers for updates
const updateData = () => {
  // Fill buffers
  for (let i = 0; i < BUFFER_SIZE; i++) {
    xBuffer[i] = i;
    yBuffer[i] = Math.sin(i * 0.1);
  }

  // Append using reused buffers - no allocation overhead
  dataSeries.appendRange(xBuffer, yBuffer);
};
```

## Performance Optimizations

### Pre-allocate Data Series Capacity

Pre-allocate capacity to avoid memory reallocations:

```typescript
// Pre-allocate capacity for 100,000 points
const dataSeries = new XyDataSeries(wasmContext, {
  capacity: 100000, // Avoids reallocations during appends
});
```

### Suspend When Not Visible

Suspend updates for charts not currently visible:

```typescript
// Using IntersectionObserver
const observer = new IntersectionObserver((entries) => {
  entries.forEach((entry) => {
    if (entry.isIntersecting) {
      // Chart is visible - resume updates
      if (sciChartSurface.isSuspended) {
        sciChartSurface.resumeUpdates();
      }
    } else {
      // Chart is not visible - suspend updates
      if (!sciChartSurface.isSuspended) {
        sciChartSurface.suspendUpdates();
      }
    }
  });
});

observer.observe(sciChartSurface.domChartRoot);
```

### Clear vs Delete

Use `clear()` to reuse data series without memory deallocation:

```typescript
// Clear data but keep capacity - faster for reuse
dataSeries.clear(); // Size = 0, but capacity remains

// Later, reuse the same series
dataSeries.appendRange(newXValues, newYValues); // No reallocation needed

// Only delete when completely done
dataSeries.delete(); // Frees all memory
```

## What Gets Redrawn

### Incremental Redraws

SciChart.js only redraws what changed:

- **Data Series Update**: Only the affected series redraws
- **Axis Range Change**: Only viewport and affected series redraw
- **Axis Property Change**: Only the axis and labels redraw
- **Series Property Change**: Only the series redraws
- **Annotation Change**: Only the annotation redraws

### Full Redraws (Rare)

Full redraws only occur when:

- Chart is first created
- Theme changes
- Surface size changes significantly
- Major structural changes (adding/removing axes)

## Best Practices

### 1. Batch Updates

```typescript
// ✅ Good: Batch multiple updates
sciChartSurface.suspendUpdates();
dataSeries.appendRange(x1, y1);
dataSeries.appendRange(x2, y2);
axis.visibleRange = new NumberRange(0, 100);
sciChartSurface.resumeUpdates({ invalidateOnResume: true });

// ❌ Bad: Individual updates
dataSeries.appendRange(x1, y1); // Redraw 1
dataSeries.appendRange(x2, y2); // Redraw 2
axis.visibleRange = new NumberRange(0, 100); // Redraw 3
```

### 2. Use appendRange for Multiple Points

```typescript
// ✅ Good: Single batch append
dataSeries.appendRange(xValues, yValues);

// ❌ Bad: Multiple individual appends
xValues.forEach((x, i) => {
  dataSeries.append(x, yValues[i]); // Multiple redraws
});
```

### 3. Reuse Buffers for Real-Time Data

```typescript
// ✅ Good: Reuse typed arrays
const xBuffer = new Float64Array(1000);
const yBuffer = new Float64Array(1000);
// ... fill and reuse ...

// ❌ Bad: Allocate new arrays each time
const xValues = new Array(1000);
const yValues = new Array(1000);
```

### 4. Suspend When Making Multiple Changes

```typescript
// ✅ Good: Suspend for multiple changes
sciChartSurface.suspendUpdates();
// ... multiple changes ...
sciChartSurface.resumeUpdates();

// ❌ Bad: Each change triggers redraw
// ... multiple changes, each causing redraw ...
```

### 5. Pre-allocate Capacity

```typescript
// ✅ Good: Pre-allocate capacity
const dataSeries = new XyDataSeries(wasmContext, { capacity: 100000 });

// ❌ Bad: Let it grow dynamically
const dataSeries = new XyDataSeries(wasmContext); // Reallocations occur
```

## Summary

SciChart.js provides efficient incremental updates through:

1. **Automatic Invalidation**: Data changes automatically trigger minimal redraws
2. **Batch Updates**: `suspendUpdates()` / `resumeUpdates()` groups multiple changes
3. **Incremental Rendering**: Only changed elements redraw, not the entire chart
4. **Event-Driven**: Changes fire events without additional redraws
5. **Performance Optimizations**: Buffer reuse, capacity pre-allocation, visibility-based suspension

This architecture enables high-performance real-time charts that can handle thousands of updates per second while maintaining smooth 60 FPS rendering.
