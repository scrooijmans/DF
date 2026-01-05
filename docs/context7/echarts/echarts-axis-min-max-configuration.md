# Apache ECharts: Setting Min and Max for X and Y Axes

## Overview

Apache ECharts provides flexible ways to set the minimum and maximum values for both X and Y axes. You can use numeric values, special string values, or functions to dynamically calculate axis ranges based on your data.

## Basic Syntax

The `min` and `max` properties can be set on any axis configuration:

```javascript
option = {
  xAxis: {
    type: 'value', // or 'category', 'time', 'log'
    min: 0, // Minimum value
    max: 100, // Maximum value
  },
  yAxis: {
    type: 'value',
    min: 0,
    max: 200,
  },
  series: [
    {
      type: 'line',
      data: [10, 20, 30, 40, 50],
    },
  ],
};
```

## Method 1: Numeric Values

The simplest way to set axis ranges is using numeric values.

### Example: Fixed Range

```javascript
option = {
  xAxis: {
    type: 'value',
    min: 0,
    max: 100,
  },
  yAxis: {
    type: 'value',
    min: 0,
    max: 500,
  },
  series: [
    {
      type: 'line',
      data: [10, 20, 30, 40, 50],
    },
  ],
};
```

### Example: Negative Values

```javascript
option = {
  xAxis: {
    type: 'value',
    min: -50,
    max: 50,
  },
  yAxis: {
    type: 'value',
    min: -100,
    max: 100,
  },
  series: [
    {
      type: 'scatter',
      data: [
        [-20, -30],
        [10, 40],
        [30, 70],
      ],
    },
  ],
};
```

## Method 2: Special String Values

ECharts provides special string values that automatically calculate min/max based on your data.

### `'dataMin'` and `'dataMax'`

These values automatically set the axis range to the minimum and maximum values in your data.

```javascript
option = {
  xAxis: {
    type: 'value',
    min: 'dataMin', // Automatically uses minimum data value
    max: 'dataMax', // Automatically uses maximum data value
  },
  yAxis: {
    type: 'value',
    min: 'dataMin',
    max: 'dataMax',
  },
  series: [
    {
      type: 'line',
      data: [23, 45, 67, 34, 89], // Y-axis will scale from 23 to 89
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
    min: 'dataMin', // Fits to minimum data value
    max: 'dataMax', // Fits to maximum data value
  },
  series: [
    {
      type: 'line',
      data: [120, 132, 101, 134, 90], // Y-axis scales from 90 to 134
    },
  ],
};
```

## Method 3: Functions for Dynamic Calculation

You can use functions to dynamically calculate min/max values based on data statistics. The function receives an object with `min` and `max` properties containing the data statistics.

### Basic Function Syntax

```javascript
option = {
  yAxis: {
    type: 'value',
    min: function (value) {
      // value.min contains the minimum data value
      // value.max contains the maximum data value
      return value.min - 10; // Add padding below minimum
    },
    max: function (value) {
      return value.max + 10; // Add padding above maximum
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

### Example: Percentage Padding

```javascript
option = {
  yAxis: {
    type: 'value',
    min: function (value) {
      const range = value.max - value.min;
      return value.min - range * 0.1; // 10% padding below
    },
    max: function (value) {
      const range = value.max - value.min;
      return value.max + range * 0.1; // 10% padding above
    },
  },
  series: [
    {
      type: 'bar',
      data: [100, 200, 150, 300, 250],
    },
  ],
};
```

### Returning `null` for Default Behavior

Since ECharts 4.8.0, you can return `null` from min/max functions to use the default automatic scaling:

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
  series: [
    {
      type: 'line',
      data: [10, 20, 30, 40, 50],
    },
  ],
};
```

## Method 4: Category Axes

For category axes, `min` and `max` can be set using category names or indices.

### Using Category Names

```javascript
option = {
  xAxis: {
    type: 'category',
    data: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug'],
    min: 'Feb', // Start from 'Feb'
    max: 'Jul', // End at 'Jul'
  },
  yAxis: {
    type: 'value',
  },
  series: [
    {
      type: 'bar',
      data: [10, 20, 30, 40, 50, 60, 70, 80],
    },
  ],
};
```

### Using Indices

```javascript
option = {
  xAxis: {
    type: 'category',
    data: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug'],
    min: 1, // Start from index 1 ('Feb')
    max: 6, // End at index 6 ('Jul')
  },
  yAxis: {
    type: 'value',
  },
  series: [
    {
      type: 'bar',
      data: [10, 20, 30, 40, 50, 60, 70, 80],
    },
  ],
};
```

### Using Negative Indices

```javascript
option = {
  xAxis: {
    type: 'category',
    data: ['Jan', 'Feb', 'Mar', 'Apr', 'May'],
    min: -2, // Can use negative indices
    max: 4,
  },
  series: [
    {
      type: 'line',
      data: [10, 20, 30, 40, 50],
    },
  ],
};
```

## Method 5: Time Axes

For time axes, you can use Date objects or date strings.

### Using Date Objects

```javascript
option = {
  xAxis: {
    type: 'time',
    min: new Date('2024-01-01'),
    max: new Date('2024-12-31'),
  },
  yAxis: {
    type: 'value',
  },
  series: [
    {
      type: 'line',
      data: [
        [new Date('2024-01-01'), 100],
        [new Date('2024-06-15'), 200],
        [new Date('2024-12-31'), 150],
      ],
    },
  ],
};
```

### Using Date Strings

```javascript
option = {
  xAxis: {
    type: 'time',
    min: '2024-01-01',
    max: '2024-12-31',
  },
  yAxis: {
    type: 'value',
  },
  series: [
    {
      type: 'line',
      data: [
        ['2024-01-01', 100],
        ['2024-06-15', 200],
        ['2024-12-31', 150],
      ],
    },
  ],
};
```

### Using `'dataMin'` and `'dataMax'` with Time

```javascript
option = {
  xAxis: {
    type: 'time',
    min: 'dataMin', // Automatically uses earliest date
    max: 'dataMax', // Automatically uses latest date
  },
  yAxis: {
    type: 'value',
  },
  series: [
    {
      type: 'line',
      data: [
        [new Date('2024-01-01'), 100],
        [new Date('2024-06-15'), 200],
        [new Date('2024-12-31'), 150],
      ],
    },
  ],
};
```

## Complete Examples

### Example 1: Line Chart with Custom Ranges

```javascript
option = {
  tooltip: {
    trigger: 'axis',
  },
  xAxis: {
    type: 'category',
    data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
  },
  yAxis: {
    type: 'value',
    min: 0, // Fixed minimum at zero
    max: 200, // Fixed maximum at 200
  },
  series: [
    {
      type: 'line',
      data: [120, 132, 101, 134, 90, 230, 210],
    },
  ],
};
```

### Example 2: Scatter Plot with Auto-Scaling

```javascript
option = {
  xAxis: {
    type: 'value',
    min: 'dataMin', // Auto-scale X-axis
    max: 'dataMax',
    scale: true, // Don't force zero
  },
  yAxis: {
    type: 'value',
    min: 'dataMin', // Auto-scale Y-axis
    max: 'dataMax',
    scale: true, // Don't force zero
  },
  series: [
    {
      type: 'scatter',
      data: [
        [45, 67],
        [52, 73],
        [38, 61],
        [55, 80],
        [42, 65],
      ],
    },
  ],
};
```

### Example 3: Bar Chart with Function-Based Padding

```javascript
option = {
  xAxis: {
    type: 'category',
    data: ['Product A', 'Product B', 'Product C', 'Product D'],
  },
  yAxis: {
    type: 'value',
    min: function (value) {
      // Ensure minimum is at least 0, or add padding if negative
      return Math.min(0, value.min - 10);
    },
    max: function (value) {
      // Add 10% padding above maximum
      const range = value.max - value.min;
      return value.max + range * 0.1;
    },
  },
  series: [
    {
      type: 'bar',
      data: [120, 200, 150, 300],
    },
  ],
};
```

### Example 4: Multiple Y-Axes with Different Ranges

```javascript
option = {
  xAxis: {
    type: 'category',
    data: ['Jan', 'Feb', 'Mar', 'Apr', 'May'],
  },
  yAxis: [
    {
      type: 'value',
      name: 'Temperature (°C)',
      min: 0,
      max: 40,
      position: 'left',
    },
    {
      type: 'value',
      name: 'Humidity (%)',
      min: 0,
      max: 100,
      position: 'right',
    },
  ],
  series: [
    {
      name: 'Temperature',
      type: 'line',
      yAxisIndex: 0,
      data: [15, 18, 22, 25, 20],
    },
    {
      name: 'Humidity',
      type: 'line',
      yAxisIndex: 1,
      data: [60, 65, 70, 68, 72],
    },
  ],
};
```

### Example 5: Time Series with Date Range

```javascript
option = {
  tooltip: {
    trigger: 'axis',
  },
  xAxis: {
    type: 'time',
    min: '2024-01-01',
    max: '2024-12-31',
    boundaryGap: false,
  },
  yAxis: {
    type: 'value',
    min: 'dataMin',
    max: 'dataMax',
  },
  series: [
    {
      type: 'line',
      data: [
        ['2024-01-15', 100],
        ['2024-03-20', 150],
        ['2024-06-10', 120],
        ['2024-09-05', 180],
        ['2024-11-30', 160],
      ],
    },
  ],
};
```

## Important Notes

### 1. DataZoom Interaction

When `min` and `max` are explicitly set on an axis, the extent of that axis will **not** be modified by DataZoom operations on other axes:

```javascript
option = {
  dataZoom: [
    {
      type: 'slider',
      xAxisIndex: [0],
    },
  ],
  xAxis: {
    type: 'value',
    // DataZoom can modify this axis
  },
  yAxis: {
    type: 'value',
    min: 0,
    max: 400, // This range will NOT be modified by DataZoom
  },
  series: [
    {
      type: 'bar',
      data: [[12, 24], [90, 80], [3, 9]],
    },
  ],
};
```

### 2. Type Compatibility

- **Value axes**: Support numbers, `'dataMin'`, `'dataMax'`, and functions
- **Category axes**: Support category names, indices (including negative), or `null`
- **Time axes**: Support Date objects, date strings, `'dataMin'`, `'dataMax'`, or functions
- **Log axes**: Support numbers, `'dataMin'`, `'dataMax'`, or functions

### 3. Setting Only Min or Only Max

You can set just `min` or just `max`, and ECharts will automatically calculate the other:

```javascript
option = {
  yAxis: {
    type: 'value',
    min: 0, // Only set minimum, max will be auto-calculated
    // max is not set, so it will be auto-calculated from data
  },
  series: [
    {
      type: 'line',
      data: [10, 20, 30, 40, 50],
    },
  ],
};
```

### 4. Null Values

Setting `min` or `max` to `null` uses the default automatic calculation:

```javascript
option = {
  yAxis: {
    type: 'value',
    min: null, // Use default (auto-calculate from data)
    max: null, // Use default (auto-calculate from data)
  },
};
```

## Best Practices

### 1. Use `'dataMin'`/`'dataMax'` for Automatic Scaling

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

### 2. Use Functions for Custom Padding

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

### 3. Force Zero for Bar Charts

```javascript
// ✅ Good: Bar charts typically should include zero
yAxis: {
  type: 'value',
  min: 0, // Force zero to be included
  max: 'dataMax',
}
```

### 4. Handle Dynamic Data Updates

When data changes, explicitly set min/max will remain fixed. Use `'dataMin'`/`'dataMax'` or functions for dynamic updates:

```javascript
// Initial setup
const option = {
  yAxis: {
    type: 'value',
    min: 'dataMin', // Will auto-update when data changes
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

## Summary

Apache ECharts provides multiple ways to set min and max for X and Y axes:

1. **Numeric values**: Fixed ranges (e.g., `min: 0, max: 100`)
2. **Special strings**: `'dataMin'` and `'dataMax'` for automatic scaling
3. **Functions**: Dynamic calculation with padding or conditions
4. **Category values**: Category names or indices for category axes
5. **Date values**: Date objects or strings for time axes

Choose the method that best fits your use case:
- Use `'dataMin'`/`'dataMax'` for automatic scaling
- Use numeric values for fixed ranges
- Use functions for custom padding or conditional logic
- Use category names/indices for category axes
- Use Date objects/strings for time axes

