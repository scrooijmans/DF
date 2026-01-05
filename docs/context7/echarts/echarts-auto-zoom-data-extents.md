# Apache ECharts: Automatic Zoom to Data Extents

## Overview

Apache ECharts provides several methods to automatically zoom and scale charts to fit the min and max data ranges of your series. This ensures that all data points are visible and properly displayed within the chart's viewport, without manual configuration of axis boundaries.

## Method 1: Using `dataMin` and `dataMax` (Recommended)

The most straightforward way to automatically fit axes to data ranges is by using the special string values `'dataMin'` and `'dataMax'` for the `min` and `max` properties of axes.

### Basic Syntax

```javascript
option = {
  xAxis: {
    type: 'value', // or 'time', 'log'
    min: 'dataMin', // Automatically set to minimum data value
    max: 'dataMax', // Automatically set to maximum data value
  },
  yAxis: {
    type: 'value',
    min: 'dataMin',
    max: 'dataMax',
  },
  series: [
    {
      type: 'line',
      data: [10, 20, 30, 40, 50],
    },
  ],
};
```

### Example: Line Chart with Auto-Scaling

```javascript
option = {
  xAxis: {
    type: 'category',
    data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'],
  },
  yAxis: {
    type: 'value',
    min: 'dataMin', // Automatically fits to data minimum
    max: 'dataMax', // Automatically fits to data maximum
  },
  series: [
    {
      type: 'line',
      data: [23, 45, 67, 34, 89], // Y-axis will scale from 23 to 89
    },
  ],
};
```

### Example: Candlestick Chart with Time Axis

```javascript
option = {
  xAxis: {
    type: 'time',
    scale: true,
    boundaryGap: false,
    min: 'dataMin', // Automatically fits to earliest time
    max: 'dataMax', // Automatically fits to latest time
  },
  yAxis: {
    type: 'value',
    scale: true,
    min: 'dataMin',
    max: 'dataMax',
  },
  series: [
    {
      type: 'candlestick',
      data: [
        [new Date('2024-01-01'), 100, 120, 95, 110],
        [new Date('2024-01-02'), 110, 130, 105, 125],
        // ... more data
      ],
    },
  ],
};
```

### Example: 3D Charts

```javascript
option = {
  xAxis3D: {
    type: 'value',
    min: 'dataMin',
    max: 'dataMax',
  },
  yAxis3D: {
    type: 'value',
    min: 'dataMin',
    max: 'dataMax',
  },
  zAxis3D: {
    type: 'value',
    min: 'dataMin',
    max: 'dataMax',
  },
  series: [
    {
      type: 'scatter3D',
      data: [
        [10, 20, 30],
        [15, 25, 35],
        // ... more 3D points
      ],
    },
  ],
};
```

## Method 2: Using the `scale` Property

For value axes, the `scale` property allows the axis to scale independently of the zero origin, which is particularly useful for scatter plots and charts where the data doesn't naturally include zero.

### Basic Usage

```javascript
option = {
  xAxis: {
    type: 'value',
    scale: true, // Allows axis to scale independently of zero
  },
  yAxis: {
    type: 'value',
    scale: true, // Automatically fits to data range
  },
  series: [
    {
      type: 'scatter',
      data: [
        [45, 67],
        [52, 73],
        [38, 61],
      ],
    },
  ],
};
```

**Note**: The `scale` property is only effective when `min` and `max` are not explicitly set. If you set `min: 'dataMin'` and `max: 'dataMax'`, the `scale` property is automatically implied.

### When to Use `scale`

- **Scatter plots** with dual numerical axes
- **Data that doesn't include zero** in its natural range
- **Precise data visualization** where zero is not meaningful

## Method 3: Using Functions for Dynamic Calculation

You can use functions to dynamically calculate min/max values based on the data, allowing for custom padding or adjustments.

### Function Syntax

```javascript
option = {
  yAxis: {
    type: 'value',
    min: function (value) {
      // value.min contains the minimum data value
      // Add padding below the minimum
      return value.min - 10;
    },
    max: function (value) {
      // value.max contains the maximum data value
      // Add padding above the maximum
      return value.max + 10;
    },
  },
  series: [
    {
      type: 'line',
      data: [20, 30, 40, 50, 60],
    },
  ],
};
```

### Returning `null` for Default Behavior

Since ECharts 4+, you can return `null` from min/max functions to use the default automatic scaling:

```javascript
option = {
  yAxis: {
    type: 'value',
    min: function (value) {
      // Use default if data min is less than 0
      if (value.min < 0) {
        return null; // Use default behavior
      }
      return value.min - 5; // Otherwise add padding
    },
    max: function (value) {
      return value.max + 5; // Always add padding
    },
  },
};
```

## Method 4: Combining with `boundaryGap`

The `boundaryGap` property controls spacing at axis boundaries and works well with automatic scaling.

### For Category Axes

```javascript
option = {
  xAxis: {
    type: 'category',
    data: ['A', 'B', 'C', 'D', 'E'],
    boundaryGap: true, // Default: adds gap at both ends
    // boundaryGap: false removes gaps
  },
  yAxis: {
    type: 'value',
    min: 'dataMin',
    max: 'dataMax',
  },
  series: [
    {
      type: 'bar',
      data: [10, 20, 30, 40, 50],
    },
  ],
};
```

### For Value Axes (Percentage or Numeric)

```javascript
option = {
  xAxis: {
    type: 'value',
    min: 'dataMin',
    max: 'dataMax',
    boundaryGap: ['10%', '10%'], // 10% padding on both sides
    // or
    // boundaryGap: [20, 20], // 20 units padding on both sides
  },
  series: [
    {
      type: 'line',
      data: [10, 20, 30, 40, 50],
    },
  ],
};
```

## Method 5: Using DataZoom for Interactive Zooming

While `dataMin`/`dataMax` set the initial view, `dataZoom` allows users to interactively zoom into specific data ranges while maintaining automatic scaling.

### Inside DataZoom (Mouse Wheel)

```javascript
option = {
  xAxis: {
    type: 'value',
    min: 'dataMin',
    max: 'dataMax',
  },
  yAxis: {
    type: 'value',
    min: 'dataMin',
    max: 'dataMax',
  },
  dataZoom: [
    {
      type: 'inside', // Enables mouse wheel zooming
      start: 0, // Start at beginning of data
      end: 100, // Show all data initially
    },
  ],
  series: [
    {
      type: 'line',
      data: [10, 20, 30, 40, 50, 60, 70, 80, 90, 100],
    },
  ],
};
```

### Slider DataZoom

```javascript
option = {
  xAxis: {
    type: 'time',
    min: 'dataMin',
    max: 'dataMax',
  },
  yAxis: {
    type: 'value',
    min: 'dataMin',
    max: 'dataMax',
  },
  dataZoom: [
    {
      type: 'slider',
      start: 0,
      end: 100,
      // Automatically fits to data range
    },
  ],
  series: [
    {
      type: 'line',
      data: [
        [new Date('2024-01-01'), 100],
        [new Date('2024-01-02'), 120],
        // ... more data
      ],
    },
  ],
};
```

### Programmatic Zoom to Data Range

You can programmatically zoom to fit data using the `dispatchAction` API:

```javascript
// After chart is initialized
myChart.dispatchAction({
  type: 'dataZoom',
  startValue: dataMin, // Minimum value in your data
  endValue: dataMax, // Maximum value in your data
});
```

## Method 6: Custom Series with `encode`

For custom series, you must specify data encoding to ensure axes automatically fit the data:

```javascript
option = {
  xAxis: {
    type: 'value',
    // Will automatically scale to fit encoded data
  },
  yAxis: {
    type: 'value',
    // Will automatically scale to fit encoded data
  },
  series: [
    {
      type: 'custom',
      renderItem: function (params, api) {
        // Custom rendering logic
      },
      encode: {
        // Dimension 1 maps to X-axis
        x: 1,
        // Dimension 0 maps to Y-axis
        y: 0,
      },
      data: [
        // Dimension 0  Dimension 1  Dimension 2
        [12, 44, 55],
        [53, 31, 21],
        [71, 33, 10],
      ],
    },
  ],
};
```

## Complete Example: Multi-Series Chart with Auto-Scaling

```javascript
option = {
  tooltip: {
    trigger: 'axis',
  },
  legend: {
    data: ['Series A', 'Series B'],
  },
  xAxis: {
    type: 'category',
    data: ['Jan', 'Feb', 'Mar', 'Apr', 'May'],
    boundaryGap: false,
  },
  yAxis: {
    type: 'value',
    min: 'dataMin', // Automatically fits to all series' minimum
    max: 'dataMax', // Automatically fits to all series' maximum
    scale: true, // Allows scaling independent of zero
  },
  dataZoom: [
    {
      type: 'inside',
      start: 0,
      end: 100,
    },
    {
      type: 'slider',
      start: 0,
      end: 100,
    },
  ],
  series: [
    {
      name: 'Series A',
      type: 'line',
      data: [120, 132, 101, 134, 90],
    },
    {
      name: 'Series B',
      type: 'line',
      data: [220, 182, 191, 234, 290],
    },
  ],
};
```

## Best Practices

### 1. Use `dataMin`/`dataMax` for Automatic Scaling

```javascript
// ✅ Good: Automatic scaling
yAxis: {
  type: 'value',
  min: 'dataMin',
  max: 'dataMax',
}

// ❌ Avoid: Hard-coded values (unless necessary)
yAxis: {
  type: 'value',
  min: 0,
  max: 100,
}
```

### 2. Combine with `scale` for Non-Zero Data

```javascript
// ✅ Good: For scatter plots or data not including zero
yAxis: {
  type: 'value',
  scale: true, // Automatically fits without forcing zero
}
```

### 3. Use Functions for Custom Padding

```javascript
// ✅ Good: Add padding for better visualization
yAxis: {
  type: 'value',
  min: function (value) {
    return value.min - (value.max - value.min) * 0.1; // 10% padding
  },
  max: function (value) {
    return value.max + (value.max - value.min) * 0.1; // 10% padding
  },
}
```

### 4. Handle Dynamic Data Updates

When data changes, ECharts automatically recalculates extents if using `dataMin`/`dataMax`:

```javascript
// Initial setup
const option = {
  yAxis: {
    type: 'value',
    min: 'dataMin',
    max: 'dataMax',
  },
  series: [
    {
      type: 'line',
      data: [10, 20, 30],
    },
  ],
};

myChart.setOption(option);

// Update with new data - axis will automatically rescale
myChart.setOption({
  series: [
    {
      data: [50, 60, 70, 80], // New range, axis will auto-adjust
    },
  ],
});
```

## Common Issues and Solutions

### Issue 1: Data Points Cut Off

**Problem**: Data points at the edges are partially visible or cut off.

**Solution**: Use `boundaryGap` or add padding with functions:

```javascript
yAxis: {
  type: 'value',
  min: function (value) {
    return value.min - 5; // Add padding
  },
  max: function (value) {
    return value.max + 5; // Add padding
  },
}
```

### Issue 2: Zero Not Included When Needed

**Problem**: For bar charts, you might want zero included even if data doesn't include it.

**Solution**: Don't use `scale: true` or explicitly set `min: 0`:

```javascript
yAxis: {
  type: 'value',
  min: 0, // Force zero to be included
  // Don't use scale: true
}
```

### Issue 3: Multiple Series with Different Ranges

**Problem**: Multiple series have vastly different value ranges.

**Solution**: ECharts automatically fits to the combined range of all series when using `dataMin`/`dataMax`:

```javascript
// ECharts will automatically find the min/max across all series
yAxis: {
  type: 'value',
  min: 'dataMin', // Minimum across all series
  max: 'dataMax', // Maximum across all series
}
```

## Summary

Apache ECharts provides multiple methods to automatically zoom to data extents:

1. **`min: 'dataMin'` / `max: 'dataMax'`** - Simplest and most common method
2. **`scale: true`** - For value axes that shouldn't include zero
3. **Functions** - For custom calculations with padding or conditions
4. **`boundaryGap`** - Controls spacing at axis boundaries
5. **`dataZoom`** - For interactive zooming while maintaining auto-scaling
6. **`encode`** - Required for custom series to enable auto-scaling

The recommended approach is to use `min: 'dataMin'` and `max: 'dataMax'` for automatic scaling, combined with `scale: true` when zero shouldn't be included, and `boundaryGap` for fine-tuning the visual appearance.

