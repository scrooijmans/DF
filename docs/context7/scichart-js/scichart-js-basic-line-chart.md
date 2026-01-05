# SciChart.js Basic Line Chart Guide

This document explains how to generate a basic LineChart using the SciChart.js library. SciChart.js is a high-performance JavaScript charting library that provides API documentation for creating interactive 2D and 3D charts.

## Overview

A basic line chart in SciChart.js consists of:
1. A **SciChartSurface** - the main chart container
2. **Axes** - X and Y axes for data visualization
3. **Data Series** - the data to be plotted
4. **Renderable Series** - the visual representation (line) of the data

## Step-by-Step Guide

### Step 1: Import Required Components

First, import the necessary components from SciChart.js:

```javascript
const { 
    SciChartSurface, 
    NumericAxis, 
    FastLineRenderableSeries, 
    XyDataSeries, 
    SciChartJsNavyTheme 
} = SciChart;
```

Or, if using npm/ES modules:

```javascript
import { 
    SciChartSurface, 
    NumericAxis, 
    FastLineRenderableSeries, 
    XyDataSeries, 
    SciChartJsNavyTheme 
} from "scichart";
```

### Step 2: Create the Chart Surface

Create a `SciChartSurface` in a container element. The `create` method is asynchronous and returns both the `wasmContext` and `sciChartSurface`:

```javascript
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
    theme: new SciChartJsNavyTheme()
});
```

**Parameters:**
- `divElementId`: The ID of the HTML div element where the chart will be rendered
- `theme`: Optional theme configuration (e.g., `SciChartJsNavyTheme` for dark theme)

**Returns:**
- `wasmContext`: The WebAssembly context used by SciChart for high-performance rendering
- `sciChartSurface`: The main chart surface object

### Step 3: Add Axes

Add X and Y axes to the chart surface:

```javascript
sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
sciChartSurface.yAxes.add(new NumericAxis(wasmContext));
```

The `NumericAxis` creates numeric axes that automatically scale based on the data range. You can customize axes with additional options:

```javascript
sciChartSurface.xAxes.add(new NumericAxis(wasmContext, {
    axisTitle: "X Axis",
    visibleRange: new NumberRange(0, 100)
}));

sciChartSurface.yAxes.add(new NumericAxis(wasmContext, {
    axisTitle: "Y Axis",
    growBy: new NumberRange(0.1, 0.1) // Add 10% padding
}));
```

### Step 4: Prepare Your Data

Create arrays for X and Y values. The arrays must have the same length:

```javascript
const xValues = [];
const yValues = [];
for (let i = 0; i < 100; i++) {
    xValues.push(i);
    yValues.push(0.2 * Math.sin(i * 0.1) - Math.cos(i * 0.01));
}
```

Or use static data:

```javascript
const xValues = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
const yValues = [2.5, 3.5, 3.7, 4.0, 5.0, 5.5, 5.0, 4.0, 3.0, 2.0];
```

### Step 5: Create the Data Series

Create an `XyDataSeries` to hold your data:

```javascript
const dataSeries = new XyDataSeries(wasmContext, {
    xValues,
    yValues
});
```

Alternatively, you can create an empty series and append data:

```javascript
const dataSeries = new XyDataSeries(wasmContext);

// Append single point
dataSeries.append(1, 10);

// Append multiple points (more efficient)
dataSeries.appendRange(xValues, yValues);
```

### Step 6: Create the Line Series

Create a `FastLineRenderableSeries` with your data series:

```javascript
const lineSeries = new FastLineRenderableSeries(wasmContext, {
    dataSeries: dataSeries,
    stroke: "#FF6600",        // Line color (hex, rgb, or named color)
    strokeThickness: 5,       // Line thickness in pixels
});
```

**Common Options:**
- `stroke`: Line color (default: "#FF6600")
- `strokeThickness`: Line thickness in pixels (default: 1)
- `isDigitalLine`: Set to `true` for step/digital line chart
- `pointMarker`: Add point markers (circles, squares, etc.)

### Step 7: Add the Series to the Chart

Add the line series to the chart surface:

```javascript
sciChartSurface.renderableSeries.add(lineSeries);
```

## Complete Example

Here's a complete working example:

```javascript
const { 
    SciChartSurface, 
    NumericAxis, 
    FastLineRenderableSeries, 
    XyDataSeries, 
    SciChartJsNavyTheme 
} = SciChart;

async function createBasicLineChart(divElementId) {
    // Step 1: Create chart surface
    const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId, {
        theme: new SciChartJsNavyTheme()
    });

    // Step 2: Add axes
    sciChartSurface.xAxes.add(new NumericAxis(wasmContext));
    sciChartSurface.yAxes.add(new NumericAxis(wasmContext));

    // Step 3: Prepare data
    const xValues = [];
    const yValues = [];
    for (let i = 0; i < 100; i++) {
        xValues.push(i);
        yValues.push(0.2 * Math.sin(i * 0.1) - Math.cos(i * 0.01));
    }

    // Step 4: Create line series
    const lineSeries = new FastLineRenderableSeries(wasmContext, {
        dataSeries: new XyDataSeries(wasmContext, {
            xValues,
            yValues
        }),
        stroke: "#FF6600",
        strokeThickness: 5,
    });

    // Step 5: Add series to chart
    sciChartSurface.renderableSeries.add(lineSeries);
}

// Call the function with your div ID
createBasicLineChart("scichart-root");
```

**HTML:**
```html
<div id="scichart-root" style="width: 800px; height: 600px;"></div>
```

## Alternative: Builder API

SciChart.js also provides a Builder API that simplifies chart creation using a declarative JSON configuration:

```javascript
const { chartBuilder, ESeriesType, EThemeProviderType } = SciChart;

const { wasmContext, sciChartSurface } = await chartBuilder.build2DChart(divElementId, {
    surface: { 
        theme: { type: EThemeProviderType.Dark } 
    },
    series: [
        {
            type: ESeriesType.LineSeries,
            xyData: {
                xValues: [0, 1, 2, 3, 4, 5, 6, 7, 8],
                yValues: [2.5, 3.5, 3.7, 4.0, 5.0, 5.5, 5.0, 4.0, 3.0]
            },
            options: {
                stroke: "#0066FF",
                strokeThickness: 5
            }
        }
    ]
});
```

## Customization Options

### Line Styling

```javascript
const lineSeries = new FastLineRenderableSeries(wasmContext, {
    dataSeries: dataSeries,
    stroke: "#FF6600",           // Line color
    strokeThickness: 3,           // Line width
    strokeDashArray: [5, 5],      // Dashed line pattern
    opacity: 0.8,                 // Line opacity (0-1)
});
```

### Digital/Step Line

Create a step line chart:

```javascript
const lineSeries = new FastLineRenderableSeries(wasmContext, {
    dataSeries: dataSeries,
    stroke: "#FF6600",
    strokeThickness: 5,
    isDigitalLine: true  // Enable step/digital line
});
```

### Point Markers

Add markers at data points:

```javascript
const { EllipsePointMarker } = SciChart;

const lineSeries = new FastLineRenderableSeries(wasmContext, {
    dataSeries: dataSeries,
    stroke: "#FF6600",
    strokeThickness: 5,
    pointMarker: new EllipsePointMarker(wasmContext, {
        width: 7,
        height: 7,
        fill: "white",
        stroke: "#FF6600",
        strokeThickness: 2
    })
});
```

### Multiple Line Series

You can add multiple line series to the same chart:

```javascript
// First line series
const lineSeries1 = new FastLineRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [0, 1, 2, 3, 4],
        yValues: [0, 10, 20, 30, 40]
    }),
    stroke: "#FF6600",
    strokeThickness: 3
});

// Second line series
const lineSeries2 = new FastLineRenderableSeries(wasmContext, {
    dataSeries: new XyDataSeries(wasmContext, {
        xValues: [0, 1, 2, 3, 4],
        yValues: [0, 5, 15, 25, 35]
    }),
    stroke: "#0066FF",
    strokeThickness: 3
});

// Add both to chart
sciChartSurface.renderableSeries.add(lineSeries1, lineSeries2);
```

## Key Concepts

### WebAssembly Context (wasmContext)

SciChart.js uses WebAssembly for high-performance rendering. The `wasmContext` is required for creating most SciChart objects and must be obtained from the `SciChartSurface.create()` method.

### Data Series vs Renderable Series

- **Data Series** (`XyDataSeries`): Stores the actual data points (X, Y values)
- **Renderable Series** (`FastLineRenderableSeries`): Defines how the data is visually rendered (line style, color, etc.)

### Asynchronous Initialization

The `SciChartSurface.create()` method is asynchronous because it needs to load WebAssembly modules. Always use `await` or handle it with promises:

```javascript
// Using async/await
const { wasmContext, sciChartSurface } = await SciChartSurface.create(divElementId);

// Using promises
SciChartSurface.create(divElementId).then(({ wasmContext, sciChartSurface }) => {
    // Chart setup code
});
```

## Performance Tips

1. **Use `appendRange` instead of multiple `append` calls** for better performance when adding data
2. **Set initial capacity** for large datasets:
   ```javascript
   const dataSeries = new XyDataSeries(wasmContext, { capacity: 100000 });
   ```
3. **Use Float64Array** for better performance with large datasets:
   ```javascript
   const xArray = new Float64Array([0, 1, 2, 3, 4]);
   const yArray = new Float64Array([10, 20, 30, 40, 50]);
   dataSeries.appendRange(xArray, yArray);
   ```

## Common Issues

### Chart Not Appearing

- Ensure the div element exists and has dimensions (width and height)
- Check that `SciChartSurface.create()` completed successfully (use `await`)
- Verify that the div ID is correct

### Data Not Displaying

- Ensure X and Y value arrays have the same length
- Check that axes are added before adding renderable series
- Verify data values are within a reasonable numeric range

### Performance Issues

- Use `appendRange` instead of multiple `append` calls
- Set appropriate capacity for data series
- Consider using data filters or resampling for very large datasets

## Summary

Creating a basic line chart in SciChart.js involves:

1. **Import** the required components
2. **Create** a `SciChartSurface` (asynchronously)
3. **Add** X and Y axes
4. **Prepare** your data (X and Y arrays)
5. **Create** an `XyDataSeries` with your data
6. **Create** a `FastLineRenderableSeries` with styling options
7. **Add** the series to the chart surface

The library provides both a direct API (more control) and a Builder API (simpler configuration) for creating charts. Choose the approach that best fits your needs.

