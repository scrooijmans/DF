# SciChart.js Auto-Range and Zoom Extents

This document explains how to automatically set the visible range of a chart to extend to the full extent of the series rendered by it in SciChart.js.

## Overview

SciChart.js provides multiple ways to automatically fit the visible range to all data:

1. **Auto-Range** - Automatically adjusts axis range based on data changes
2. **Zoom Extents** - Programmatically zoom to fit all data
3. **Zoom Extents Modifier** - User-triggered zoom to fit via double-click

## Auto-Range (Automatic Ranging)

### EAutoRange Enum

The `EAutoRange` enum controls when axes automatically adjust their visible range:

```typescript
enum EAutoRange {
  Always = "Always", // Continuously adjusts as data changes
  Once = "Once", // Adjusts once on initial data load (default)
  Never = "Never", // Never auto-adjusts, manual control only
}
```

### Configure Auto-Range

#### Using Standard API

```typescript
import { NumericAxis, EAutoRange, NumberRange } from "scichart";

// Auto-range once (default) - adjusts on initial data load
sciChartSurface.yAxes.add(
  new NumericAxis(wasmContext, {
    autoRange: EAutoRange.Once,
  }),
);

// Auto-range always - continuously adjusts as data changes
sciChartSurface.yAxes.add(
  new NumericAxis(wasmContext, {
    autoRange: EAutoRange.Always,
  }),
);

// Never auto-range - manual control only
sciChartSurface.yAxes.add(
  new NumericAxis(wasmContext, {
    autoRange: EAutoRange.Never,
  }),
);
```

#### Using Builder API

```typescript
import { chartBuilder, EAxisType, EAutoRange } from "scichart";

const { sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
  yAxes: {
    type: EAxisType.NumericAxis,
    options: {
      autoRange: EAutoRange.Always, // Continuously auto-range
    },
  },
});
```

### Auto-Range Behavior

#### EAutoRange.Once (Default)

- Adjusts visible range **once** when data is first loaded
- Ignores subsequent data changes
- Useful when you want initial fit but then allow user zoom/pan

```typescript
sciChartSurface.yAxes.add(
  new NumericAxis(wasmContext, {
    autoRange: EAutoRange.Once, // Fits on load, then stops
  }),
);
```

#### EAutoRange.Always

- **Continuously** adjusts visible range as data changes
- Perfect for real-time charts that need to always show all data
- Automatically expands/contracts as new data arrives

```typescript
sciChartSurface.yAxes.add(
  new NumericAxis(wasmContext, {
    autoRange: EAutoRange.Always, // Always fits all data
  }),
);

// As you append data, the range automatically adjusts
dataSeries.appendRange(xValues, yValues); // Range auto-adjusts
```

#### EAutoRange.Never

- Never automatically adjusts
- Requires manual setting of `visibleRange`
- Useful for fixed-scale charts or when you want full control

```typescript
sciChartSurface.yAxes.add(
  new NumericAxis(wasmContext, {
    autoRange: EAutoRange.Never,
    visibleRange: new NumberRange(0, 100), // Fixed range
  }),
);
```

## Adding Padding with growBy

The `growBy` property adds padding around the data range:

```typescript
import { NumberRange } from "scichart";

sciChartSurface.yAxes.add(
  new NumericAxis(wasmContext, {
    autoRange: EAutoRange.Always,
    growBy: new NumberRange(0.1, 0.1), // 10% padding above and below
  }),
);
```

### growBy Examples

```typescript
// 10% padding on both sides
growBy: new NumberRange(0.1, 0.1);

// 20% padding above, 10% below
growBy: new NumberRange(0.2, 0.1);

// 5% padding on both sides
growBy: new NumberRange(0.05, 0.05);
```

## Zoom Extents (Programmatic)

### zoomExtents() - Fit All Axes

Zooms both X and Y axes to fit all data:

```typescript
// Zoom to fit all data (both axes)
sciChartSurface.zoomExtents();

// With animation (2000ms duration)
sciChartSurface.zoomExtents(2000);
```

### zoomExtentsX() - Fit X-Axis Only

Zooms only the X-axis to fit all data:

```typescript
// Zoom X-axis to fit
sciChartSurface.zoomExtentsX();

// With animation and custom easing
sciChartSurface.zoomExtentsX(500, easing.outExpo);

// With axis selector (only specific axes)
sciChartSurface.zoomExtentsX(500, undefined, (axis) => axis.id === "xAxis1");
```

### zoomExtentsY() - Fit Y-Axis Only

Zooms only the Y-axis to fit all data:

```typescript
// Zoom Y-axis to fit
sciChartSurface.zoomExtentsY();

// With animation
sciChartSurface.zoomExtentsY(500);

// With axis selector
sciChartSurface.zoomExtentsY(500, undefined, (axis) => axis.id === "yAxis1");
```

### Complete Example

```typescript
import {
  SciChartSurface,
  NumericAxis,
  FastLineRenderableSeries,
  XyDataSeries,
  SciChartJsNavyTheme,
  NumberRange,
} from "scichart";

const { wasmContext, sciChartSurface } = await SciChartSurface.create(
  divElementId,
  {
    theme: new SciChartJsNavyTheme(),
  },
);

// Add axes with initial ranges
sciChartSurface.xAxes.add(
  new NumericAxis(wasmContext, { visibleRange: new NumberRange(-2, 20) }),
);
sciChartSurface.yAxes.add(
  new NumericAxis(wasmContext, { visibleRange: new NumberRange(-2, 2) }),
);

// Add series
const dataSeries = new XyDataSeries(wasmContext, {
  xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
  yValues: [0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100],
});

sciChartSurface.renderableSeries.add(
  new FastLineRenderableSeries(wasmContext, {
    dataSeries,
    stroke: "#FF6600",
  }),
);

// Button click handler to zoom to fit
document.getElementById("zoom-button").addEventListener("click", () => {
  // Zoom both axes to fit all data
  sciChartSurface.zoomExtents(500); // 500ms animation

  // Or zoom individual axes
  // sciChartSurface.zoomExtentsX(500);
  // sciChartSurface.zoomExtentsY(500);
});
```

## Zoom Extents Modifier (User Interaction)

Add a modifier that allows users to double-click to zoom to fit:

```typescript
import { ZoomExtentsModifier } from "scichart";

// Add zoom extents modifier
sciChartSurface.chartModifiers.add(new ZoomExtentsModifier());

// Users can now double-click to zoom to fit all data
```

### Zoom Extents Modifier Options

```typescript
new ZoomExtentsModifier({
  executeOn: EExecuteOn.DoubleClick, // When to execute
  animationDuration: 500, // Animation duration
  isAnimated: true, // Enable animation
});
```

## Custom Zoom Extents Range

Set a custom range for zoom extents instead of using data extents:

```typescript
// Set custom zoom extents range
axis.zoomExtentsRange = new NumberRange(0, 100);

// Now zoomExtents() will zoom to 0-100 instead of data extents
sciChartSurface.zoomExtents();
```

### zoomExtentsToInitialRange

Make zoom extents return to the initial visible range:

```typescript
sciChartSurface.yAxes.add(
  new NumericAxis(wasmContext, {
    visibleRange: new NumberRange(0, 100), // Initial range
    zoomExtentsToInitialRange: true, // Zoom extents returns here
  }),
);
```

## Visible Range Limits

Limit the visible range to prevent autoranging outside bounds:

```typescript
sciChartSurface.yAxes.add(
  new NumericAxis(wasmContext, {
    autoRange: EAutoRange.Always,
    visibleRangeLimit: new NumberRange(0, 1000), // Cannot go outside 0-1000
  }),
);
```

## Real-Time Updates with Auto-Range

For real-time charts, use `EAutoRange.Always` to continuously fit data:

```typescript
// Configure axis for real-time auto-ranging
sciChartSurface.yAxes.add(
  new NumericAxis(wasmContext, {
    autoRange: EAutoRange.Always,
    growBy: new NumberRange(0.1, 0.1), // 10% padding
  }),
);

// As data is appended, range automatically adjusts
setInterval(() => {
  const x = Date.now();
  const y = Math.random() * 100;
  dataSeries.append(x, y); // Range automatically expands to include new point
}, 100);
```

## Combining Auto-Range and Manual Zoom

You can combine auto-range with manual zoom extents:

```typescript
// Initial auto-range on load
sciChartSurface.yAxes.add(
  new NumericAxis(wasmContext, {
    autoRange: EAutoRange.Once, // Fit once on load
    growBy: new NumberRange(0.1, 0.1),
  }),
);

// Add zoom extents modifier for user control
sciChartSurface.chartModifiers.add(
  new ZoomExtentsModifier(), // User can double-click to fit
);

// Programmatic zoom extents
const resetZoom = () => {
  sciChartSurface.zoomExtents(500);
};
```

## Best Practices

### 1. Use EAutoRange.Once for Initial Fit

```typescript
// ✅ Good: Fit once on load, then allow user control
sciChartSurface.yAxes.add(
  new NumericAxis(wasmContext, {
    autoRange: EAutoRange.Once,
    growBy: new NumberRange(0.1, 0.1),
  }),
);
```

### 2. Use EAutoRange.Always for Real-Time Charts

```typescript
// ✅ Good: Continuously fit data for real-time updates
sciChartSurface.yAxes.add(
  new NumericAxis(wasmContext, {
    autoRange: EAutoRange.Always,
    growBy: new NumberRange(0.05, 0.05),
  }),
);
```

### 3. Add Padding with growBy

```typescript
// ✅ Good: Add padding so data doesn't touch edges
growBy: new NumberRange(0.1, 0.1); // 10% padding
```

### 4. Use zoomExtents After Data Updates

```typescript
// ✅ Good: Zoom to fit after major data changes
dataSeries.appendRange(largeXValues, largeYValues);
sciChartSurface.zoomExtents(500); // Animated zoom to fit
```

### 5. Limit Range for Safety

```typescript
// ✅ Good: Prevent extreme ranges
visibleRangeLimit: new NumberRange(-1000, 1000);
```

## Complete Example: Auto-Fitting Chart

```typescript
import {
  SciChartSurface,
  NumericAxis,
  FastLineRenderableSeries,
  XyDataSeries,
  SciChartJsNavyTheme,
  EAutoRange,
  NumberRange,
  ZoomExtentsModifier,
  ZoomPanModifier,
  MouseWheelZoomModifier,
} from "scichart";

async function createAutoFittingChart(divElementId: string) {
  const { wasmContext, sciChartSurface } = await SciChartSurface.create(
    divElementId,
    {
      theme: new SciChartJsNavyTheme(),
    },
  );

  // Configure Y-axis to auto-range with padding
  sciChartSurface.yAxes.add(
    new NumericAxis(wasmContext, {
      autoRange: EAutoRange.Always, // Continuously fit data
      growBy: new NumberRange(0.1, 0.1), // 10% padding
      axisTitle: "Y Axis",
    }),
  );

  // X-axis with auto-range once
  sciChartSurface.xAxes.add(
    new NumericAxis(wasmContext, {
      autoRange: EAutoRange.Once, // Fit once on load
      growBy: new NumberRange(0.05, 0.05), // 5% padding
      axisTitle: "X Axis",
    }),
  );

  // Create and add series
  const dataSeries = new XyDataSeries(wasmContext);
  const lineSeries = new FastLineRenderableSeries(wasmContext, {
    dataSeries,
    stroke: "#FF6600",
    strokeThickness: 2,
  });
  sciChartSurface.renderableSeries.add(lineSeries);

  // Add modifiers for user interaction
  sciChartSurface.chartModifiers.add(
    new ZoomPanModifier(),
    new MouseWheelZoomModifier(),
    new ZoomExtentsModifier(), // Double-click to fit
  );

  // Function to add data (auto-range will adjust)
  const addData = (x: number, y: number) => {
    dataSeries.append(x, y);
    // Y-axis auto-ranges automatically (EAutoRange.Always)
  };

  // Function to manually zoom to fit
  const zoomToFit = () => {
    sciChartSurface.zoomExtents(500); // Animated
  };

  return {
    sciChartSurface,
    addData,
    zoomToFit,
  };
}

// Usage
const chart = await createAutoFittingChart("chart");
chart.addData(1, 10); // Y-axis auto-adjusts
chart.addData(2, 20); // Y-axis auto-adjusts
chart.zoomToFit(); // Manually fit all data
```

## Summary

### Auto-Range Options

- **EAutoRange.Once**: Fits once on initial load (default)
- **EAutoRange.Always**: Continuously adjusts as data changes
- **EAutoRange.Never**: Manual control only

### Zoom Extents Methods

- **zoomExtents()**: Fit both X and Y axes
- **zoomExtentsX()**: Fit X-axis only
- **zoomExtentsY()**: Fit Y-axis only

### Key Properties

- **growBy**: Add padding around data range
- **zoomExtentsRange**: Custom range for zoom extents
- **visibleRangeLimit**: Limit autorange bounds

### Use Cases

- **Initial Fit**: `EAutoRange.Once` with `growBy`
- **Real-Time Charts**: `EAutoRange.Always` with `growBy`
- **User Control**: `ZoomExtentsModifier` for double-click fit
- **Programmatic Fit**: `zoomExtents()` after data updates

This provides flexible control over how charts automatically fit their data, from continuous auto-ranging to manual zoom-to-fit operations.
