# Apache ECharts: Creating Well Log Charts with Depth on Y-Axis

## Overview

Yes, Apache ECharts can create well log-style charts where the **Y-axis represents depth (index)** and the **X-axis represents measurement values**. This is the standard format for geophysical well logs, where depth increases downward along the vertical axis and measurements are plotted horizontally.

## Basic Configuration

### Standard Well Log Setup

For a well log chart, you need:

- **Y-axis**: Depth (value axis, typically inverted so depth increases downward)
- **X-axis**: Measurement values (value axis)
- **Data format**: `[measurementValue, depthValue]` - measurement first, then depth

### Basic Example

```javascript
option = {
  title: {
    text: 'Natural γ-ray',
    left: 'center',
  },
  tooltip: {
    trigger: 'axis',
    axisPointer: {
      type: 'cross',
    },
  },
  grid: {
    left: '10%',
    right: '10%',
    bottom: '10%',
    top: '15%',
  },
  xAxis: {
    type: 'value',
    name: 'API',
    nameLocation: 'middle',
    nameGap: 30,
    min: 0,
    max: 200,
    splitLine: {
      show: true,
      lineStyle: {
        type: 'dashed',
      },
    },
  },
  yAxis: {
    type: 'value',
    name: 'Depth, m',
    nameLocation: 'middle',
    nameGap: 50,
    nameRotate: 90,
    inverse: true, // Depth increases downward (standard for well logs)
    min: 0,
    max: 130,
    splitLine: {
      show: true,
      lineStyle: {
        type: 'dashed',
      },
    },
  },
  series: [
    {
      type: 'line',
      name: 'Gamma Ray',
      data: [
        [50, 0], // [API value, depth in meters]
        [55, 5],
        [60, 10],
        [100, 15],
        [120, 20],
        [40, 25],
        [80, 30],
        // ... more data points
      ],
      lineStyle: {
        color: '#1f77b4', // Dark blue
        width: 1.5,
      },
      symbol: 'none', // No symbols on line
      smooth: false, // Straight lines between points
    },
  ],
};
```

## Complete Well Log Example

### Natural Gamma-Ray Log

```javascript
// Sample gamma-ray data: [API, Depth in meters]
const gammaRayData = [
  [50, 0],
  [55, 2],
  [60, 5],
  [100, 10],
  [120, 15],
  [40, 20],
  [80, 25],
  [70, 30],
  [60, 35],
  [50, 40],
  [45, 45],
  [120, 50],
  [130, 55],
  [140, 60],
  [60, 65],
  [40, 70],
  [45, 75],
  [50, 80],
  [180, 85], // Peak value
  [90, 90],
  [100, 95],
  [80, 100],
  [60, 105],
  [120, 110],
  [100, 115],
  [80, 120],
  [60, 125],
  [50, 130],
];

const option = {
  title: {
    text: 'Natural γ-ray',
    left: 'center',
    textStyle: {
      fontSize: 16,
      fontWeight: 'bold',
    },
  },
  tooltip: {
    trigger: 'axis',
    axisPointer: {
      type: 'cross',
    },
    formatter: function (params) {
      const point = params[0];
      return `Depth: ${point.value[1]} m<br/>API: ${point.value[0]}`;
    },
  },
  grid: {
    left: '12%',
    right: '8%',
    bottom: '10%',
    top: '15%',
    containLabel: false,
  },
  xAxis: {
    type: 'value',
    name: 'API',
    nameLocation: 'middle',
    nameGap: 30,
    min: 0,
    max: 200,
    interval: 100, // Major ticks every 100 API
    splitLine: {
      show: true,
      lineStyle: {
        type: 'dashed',
        color: '#ccc',
      },
    },
    axisLine: {
      show: true,
    },
  },
  yAxis: {
    type: 'value',
    name: 'Depth, m',
    nameLocation: 'middle',
    nameGap: 50,
    nameRotate: 90,
    inverse: true, // Critical: Depth increases downward
    min: 0,
    max: 130,
    interval: 10, // Major ticks every 10 meters
    splitLine: {
      show: true,
      lineStyle: {
        type: 'dashed',
        color: '#ccc',
      },
    },
    axisLine: {
      show: true,
    },
  },
  series: [
    {
      type: 'line',
      name: 'Gamma Ray',
      data: gammaRayData,
      lineStyle: {
        color: '#1f77b4', // Dark blue
        width: 1.5,
      },
      symbol: 'none', // No point markers
      smooth: false, // Straight line segments
      areaStyle: {
        show: false, // No fill under curve
      },
    },
  ],
};

const chart = echarts.init(
  document.getElementById('chart')
);
chart.setOption(option);
```

## Key Configuration Options

### 1. Y-Axis Inversion

The most important setting for well logs is `inverse: true` on the Y-axis:

```javascript
yAxis: {
  type: 'value',
  inverse: true, // Depth increases downward (standard for well logs)
  min: 0,
  max: 130,
}
```

**Why invert?** In well logging, depth conventionally increases downward, which matches the visual representation when the Y-axis is inverted.

### 2. Data Format

For well logs, data should be formatted as `[measurement, depth]`:

```javascript
series: [
  {
    type: 'line',
    data: [
      [50, 0], // [API value, depth in meters]
      [100, 10],
      [120, 20],
      // ...
    ],
  },
];
```

### 3. Grid Lines

Well logs typically use dashed grid lines for readability:

```javascript
xAxis: {
  splitLine: {
    show: true,
    lineStyle: {
      type: 'dashed',
      color: '#ccc',
    },
  },
},
yAxis: {
  splitLine: {
    show: true,
    lineStyle: {
      type: 'dashed',
      color: '#ccc',
    },
  },
},
```

### 4. Axis Labels

Configure axis labels for clarity:

```javascript
xAxis: {
  name: 'API',
  nameLocation: 'middle',
  nameGap: 30,
},
yAxis: {
  name: 'Depth, m',
  nameLocation: 'middle',
  nameGap: 50,
  nameRotate: 90, // Rotate label for vertical axis
},
```

## Multiple Well Log Tracks

You can create multiple tracks (panels) side by side:

```javascript
option = {
  grid: [
    {
      left: '5%',
      right: '55%',
      top: '10%',
      bottom: '10%',
    },
    {
      left: '55%',
      right: '5%',
      top: '10%',
      bottom: '10%',
    },
  ],
  xAxis: [
    {
      gridIndex: 0,
      type: 'value',
      name: 'API',
      min: 0,
      max: 200,
    },
    {
      gridIndex: 1,
      type: 'value',
      name: 'Resistivity (Ω·m)',
      min: 0,
      max: 100,
    },
  ],
  yAxis: [
    {
      gridIndex: 0,
      type: 'value',
      name: 'Depth, m',
      inverse: true,
      min: 0,
      max: 130,
    },
    {
      gridIndex: 1,
      type: 'value',
      name: 'Depth, m',
      inverse: true,
      min: 0,
      max: 130,
    },
  ],
  series: [
    {
      xAxisIndex: 0,
      yAxisIndex: 0,
      type: 'line',
      name: 'Gamma Ray',
      data: gammaRayData,
    },
    {
      xAxisIndex: 1,
      yAxisIndex: 1,
      type: 'line',
      name: 'Resistivity',
      data: resistivityData,
    },
  ],
};
```

## Auto-Scaling Depth Axis

Use `'dataMin'` and `'dataMax'` to automatically scale to data:

```javascript
yAxis: {
  type: 'value',
  inverse: true,
  min: 'dataMin', // Automatically use minimum depth in data
  max: 'dataMax', // Automatically use maximum depth in data
}
```

## Custom Depth Intervals

For specific depth intervals:

```javascript
yAxis: {
  type: 'value',
  inverse: true,
  min: 0,
  max: 130,
  interval: 10, // Major ticks every 10 meters
  minorTick: {
    show: true,
    splitNumber: 5, // 5 minor ticks between major ticks (every 2 meters)
  },
}
```

## Styling for Well Log Appearance

### Light Gray Background

```javascript
option = {
  backgroundColor: '#f5f5f5', // Light gray background
  // ... rest of configuration
};
```

### Custom Line Style

```javascript
series: [
  {
    type: 'line',
    lineStyle: {
      color: '#1f77b4', // Dark blue
      width: 1.5,
    },
    symbol: 'none', // No symbols
    smooth: false, // Straight lines
  },
];
```

### Axis Styling

```javascript
xAxis: {
  axisLine: {
    lineStyle: {
      color: '#333',
      width: 1,
    },
  },
  axisLabel: {
    color: '#333',
    fontSize: 11,
  },
},
yAxis: {
  axisLine: {
    lineStyle: {
      color: '#333',
      width: 1,
    },
  },
  axisLabel: {
    color: '#333',
    fontSize: 11,
  },
},
```

## Data Zoom for Depth Navigation

Add data zoom for navigating through depth:

```javascript
dataZoom: [
  {
    type: 'slider',
    yAxisIndex: 0,
    start: 0,
    end: 100,
    height: 20,
    bottom: 10,
  },
  {
    type: 'inside',
    yAxisIndex: 0,
  },
],
```

## Complete Working Example

```javascript
// Generate sample well log data
function generateWellLogData() {
  const data = [];
  for (let depth = 0; depth <= 130; depth += 0.5) {
    // Simulate gamma-ray values with some variation
    let api =
      50 + Math.sin(depth / 10) * 30 + Math.random() * 20;

    // Add some peaks
    if (depth > 50 && depth < 60) {
      api += 50;
    }
    if (depth > 85 && depth < 90) {
      api += 100; // Large peak
    }

    api = Math.max(0, Math.min(200, api)); // Clamp to 0-200
    data.push([api, depth]);
  }
  return data;
}

const option = {
  title: {
    text: 'Natural γ-ray',
    left: 'center',
    textStyle: {
      fontSize: 16,
      fontWeight: 'bold',
    },
  },
  tooltip: {
    trigger: 'axis',
    axisPointer: {
      type: 'cross',
    },
    formatter: function (params) {
      const point = params[0];
      return `Depth: ${point.value[1].toFixed(
        1
      )} m<br/>API: ${point.value[0].toFixed(1)}`;
    },
  },
  grid: {
    left: '12%',
    right: '8%',
    bottom: '10%',
    top: '15%',
  },
  xAxis: {
    type: 'value',
    name: 'API',
    nameLocation: 'middle',
    nameGap: 30,
    min: 0,
    max: 200,
    interval: 100,
    splitLine: {
      show: true,
      lineStyle: {
        type: 'dashed',
        color: '#ccc',
      },
    },
  },
  yAxis: {
    type: 'value',
    name: 'Depth, m',
    nameLocation: 'middle',
    nameGap: 50,
    nameRotate: 90,
    inverse: true, // Depth increases downward
    min: 0,
    max: 130,
    interval: 10,
    splitLine: {
      show: true,
      lineStyle: {
        type: 'dashed',
        color: '#ccc',
      },
    },
  },
  dataZoom: [
    {
      type: 'slider',
      yAxisIndex: 0,
      start: 0,
      end: 100,
      height: 20,
      bottom: 10,
    },
    {
      type: 'inside',
      yAxisIndex: 0,
    },
  ],
  series: [
    {
      type: 'line',
      name: 'Gamma Ray',
      data: generateWellLogData(),
      lineStyle: {
        color: '#1f77b4',
        width: 1.5,
      },
      symbol: 'none',
      smooth: false,
    },
  ],
};

const chart = echarts.init(
  document.getElementById('chart')
);
chart.setOption(option);
```

## Important Notes

### 1. Data Format

- **Format**: `[measurementValue, depthValue]`
- **Order**: Measurement first, depth second
- **Example**: `[50, 0]` means 50 API at 0 meters depth

### 2. Y-Axis Inversion

- **Always use** `inverse: true` for well logs
- This makes depth increase downward (standard convention)
- Without inversion, depth would increase upward (non-standard)

### 3. Axis Types

- Both axes should be `type: 'value'` for continuous measurements
- Use numeric ranges for min/max
- Can use `'dataMin'`/`'dataMax'` for automatic scaling

### 4. Grid Configuration

- Adjust `left`, `right`, `top`, `bottom` to accommodate axis labels
- Use `containLabel: false` if labels extend beyond grid
- Increase `nameGap` for longer axis names

## Summary

Apache ECharts fully supports well log-style charts with depth on the Y-axis:

- **Y-axis**: Configure as value axis with `inverse: true` for depth
- **X-axis**: Configure as value axis for measurements
- **Data format**: `[measurement, depth]` pairs
- **Styling**: Use dashed grid lines, custom colors, and appropriate axis labels
- **Multiple tracks**: Use multiple grids and axis indices for side-by-side logs

The key is using `yAxis.inverse: true` to make depth increase downward, which is the standard convention for well logs in geophysics and petroleum engineering.
