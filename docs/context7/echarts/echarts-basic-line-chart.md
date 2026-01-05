# How to Create a Basic Line Chart Using ECharts

## Overview

Apache ECharts is a powerful, interactive charting and visualization library. This guide demonstrates how to create a basic line chart using ECharts, covering initialization, configuration, and common customizations.

## 1. Basic Setup

### HTML Structure

First, create a container element in your HTML where the chart will be rendered:

```html
<!DOCTYPE html>
<html>
  <head>
    <script src="https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js"></script>
  </head>
  <body>
    <div
      id="main"
      style="width: 600px; height: 400px;"
    ></div>
    <script>
      // Chart code will go here
    </script>
  </body>
</html>
```

**Key Points:**

- Include the ECharts library (via CDN or npm)
- Create a container `<div>` with a fixed width and height
- The container needs an `id` or can be selected by other means

## 2. Initialize the Chart

Initialize an ECharts instance by selecting the DOM element:

```javascript
// Select the DOM element
var chartDom = document.getElementById('main');

// Initialize the chart
var myChart = echarts.init(chartDom);
```

**Alternative initialization:**

```javascript
// Using querySelector
var chartDom = document.querySelector('#main');
var myChart = echarts.init(chartDom);

// With theme (optional)
var myChart = echarts.init(chartDom, 'vintage');
```

## 3. Basic Line Chart Configuration

The simplest line chart requires:

- An `xAxis` with category data
- A `yAxis` (can be empty for auto-configuration)
- A `series` with type `'line'` and data values

### Minimal Example

```javascript
const option = {
  xAxis: {
    type: 'category',
    data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
  },
  yAxis: {},
  series: [
    {
      data: [820, 932, 901, 934, 1290, 1330, 1320],
      type: 'line',
    },
  ],
};

myChart.setOption(option);
```

### Complete HTML Example

```html
<div style="width:100%;height:500px;"></div>
<script src="https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js"></script>
<script>
  var chartDom = document.querySelector('div');
  var myChart = echarts.init(chartDom);
  var option;

  option = {
    xAxis: {
      type: 'category',
      data: [
        'Mon',
        'Tue',
        'Wed',
        'Thu',
        'Fri',
        'Sat',
        'Sun',
      ],
    },
    yAxis: {
      type: 'value',
    },
    series: [
      {
        data: [150, 230, 224, 218, 135, 147, 260],
        type: 'line',
      },
    ],
  };

  option && myChart.setOption(option);
</script>
```

## 4. Configuration Options Explained

### X-Axis Configuration

```javascript
xAxis: {
    type: 'category',  // 'category' for discrete data, 'value' for continuous
    data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
    // Optional customizations:
    axisLine: {
        lineStyle: {
            color: '#aaa'
        }
    },
    axisLabel: {
        color: '#000'
    }
}
```

### Y-Axis Configuration

```javascript
yAxis: {
    type: 'value',  // Default for numerical data
    // Optional customizations:
    splitLine: {
        show: false  // Hide grid lines
    },
    axisLine: {
        lineStyle: {
            width: 4,
            color: '#f0cf85'
        }
    }
}
```

### Series Configuration

```javascript
series: [
  {
    type: 'line',
    data: [820, 932, 901, 934, 1290, 1330, 1320],
    // Optional customizations:
    name: 'Sales', // For legend
    smooth: true, // Smooth curve
    symbolSize: 8, // Size of data point markers
    showSymbol: true, // Show/hide data point markers
    lineStyle: {
      width: 2,
      color: '#5470c6',
    },
    itemStyle: {
      color: '#5470c6',
    },
  },
];
```

## 5. Enhanced Line Chart Example

Here's a more complete example with additional features:

```javascript
var option = {
  // Title
  title: {
    text: 'Weekly Sales Data',
    left: 'center',
  },

  // Tooltip
  tooltip: {
    trigger: 'axis',
    axisPointer: {
      type: 'cross',
    },
  },

  // Legend
  legend: {
    data: ['Sales'],
    top: 30,
  },

  // Grid (spacing)
  grid: {
    left: '3%',
    right: '4%',
    bottom: '3%',
    containLabel: true,
  },

  // X-Axis
  xAxis: {
    type: 'category',
    boundaryGap: false,
    data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
  },

  // Y-Axis
  yAxis: {
    type: 'value',
  },

  // Series
  series: [
    {
      name: 'Sales',
      type: 'line',
      smooth: true,
      data: [820, 932, 901, 934, 1290, 1330, 1320],
      areaStyle: {
        color: {
          type: 'linear',
          x: 0,
          y: 0,
          x2: 0,
          y2: 1,
          colorStops: [
            { offset: 0, color: 'rgba(84, 112, 198, 0.5)' },
            { offset: 1, color: 'rgba(84, 112, 198, 0)' },
          ],
        },
      },
      lineStyle: {
        width: 3,
      },
      symbolSize: 8,
    },
  ],
};

myChart.setOption(option);
```

## 6. Multiple Line Series

You can display multiple lines on the same chart:

```javascript
var option = {
  xAxis: {
    type: 'category',
    data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
  },
  yAxis: {
    type: 'value',
  },
  legend: {
    data: ['Product A', 'Product B'],
  },
  series: [
    {
      name: 'Product A',
      type: 'line',
      data: [120, 132, 101, 134, 90, 230, 210],
    },
    {
      name: 'Product B',
      type: 'line',
      data: [220, 182, 191, 234, 290, 330, 310],
    },
  ],
};

myChart.setOption(option);
```

## 7. Using Dataset for Data Binding

ECharts supports dataset-based data binding, which separates data from visual encoding:

```javascript
var option = {
  dataset: {
    source: [
      ['Jan', 34, 20, 54],
      ['Feb', 28, 14, 64],
      ['Mar', 45, 32, 43],
      ['Apr', 69, 46, 74],
      ['May', 39, 24, 94],
      ['Jun', 43, 23, 64],
    ],
  },
  xAxis: {
    type: 'category',
  },
  yAxis: {},
  series: [
    {
      type: 'line',
      encode: {
        x: 0, // First column as x-axis
        y: 1, // Second column as y-axis
      },
    },
  ],
};

myChart.setOption(option);
```

## 8. Common Customizations

### Smooth Line

```javascript
series: [
  {
    type: 'line',
    smooth: true, // Enable smooth curve
    data: [820, 932, 901, 934, 1290, 1330, 1320],
  },
];
```

### Step Line

```javascript
series: [
  {
    type: 'line',
    step: 'middle', // Options: true, 'start', 'middle', 'end'
    data: [820, 932, 901, 934, 1290, 1330, 1320],
  },
];
```

### Area Chart

```javascript
series: [
  {
    type: 'line',
    areaStyle: {}, // Fill area under line
    data: [820, 932, 901, 934, 1290, 1330, 1320],
  },
];
```

### Custom Line Style

```javascript
series: [
  {
    type: 'line',
    lineStyle: {
      width: 3,
      color: '#5470c6',
      type: 'solid', // 'solid', 'dashed', 'dotted'
    },
    data: [820, 932, 901, 934, 1290, 1330, 1320],
  },
];
```

### Hide Data Points

```javascript
series: [
  {
    type: 'line',
    showSymbol: false, // Hide data point markers
    data: [820, 932, 901, 934, 1290, 1330, 1320],
  },
];
```

### Custom Data Point Markers

```javascript
series: [
  {
    type: 'line',
    symbol: 'circle', // 'circle', 'rect', 'roundRect', 'triangle', 'diamond', 'pin', 'arrow', 'none'
    symbolSize: 10,
    data: [820, 932, 901, 934, 1290, 1330, 1320],
  },
];
```

## 9. Responsive Charts

Make the chart responsive to window resizing:

```javascript
// Handle window resize
window.onresize = function () {
  myChart.resize();
};

// Or with event listener
window.addEventListener('resize', function () {
  myChart.resize();
});
```

## 10. Complete Working Example

Here's a complete, production-ready example:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1.0"
    />
    <title>ECharts Line Chart Example</title>
    <script src="https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js"></script>
    <style>
      #chart-container {
        width: 100%;
        height: 500px;
      }
    </style>
  </head>
  <body>
    <div id="chart-container"></div>

    <script>
      // Initialize chart
      var chartDom = document.getElementById(
        'chart-container'
      );
      var myChart = echarts.init(chartDom);

      // Configure options
      var option = {
        title: {
          text: 'Weekly Sales Performance',
          left: 'center',
        },
        tooltip: {
          trigger: 'axis',
        },
        legend: {
          data: ['Sales'],
          top: 30,
        },
        grid: {
          left: '3%',
          right: '4%',
          bottom: '3%',
          containLabel: true,
        },
        xAxis: {
          type: 'category',
          boundaryGap: false,
          data: [
            'Mon',
            'Tue',
            'Wed',
            'Thu',
            'Fri',
            'Sat',
            'Sun',
          ],
        },
        yAxis: {
          type: 'value',
        },
        series: [
          {
            name: 'Sales',
            type: 'line',
            smooth: true,
            data: [820, 932, 901, 934, 1290, 1330, 1320],
            areaStyle: {
              color: {
                type: 'linear',
                x: 0,
                y: 0,
                x2: 0,
                y2: 1,
                colorStops: [
                  {
                    offset: 0,
                    color: 'rgba(84, 112, 198, 0.3)',
                  },
                  {
                    offset: 1,
                    color: 'rgba(84, 112, 198, 0)',
                  },
                ],
              },
            },
            lineStyle: {
              width: 3,
              color: '#5470c6',
            },
            symbolSize: 8,
            itemStyle: {
              color: '#5470c6',
            },
          },
        ],
      };

      // Set option and render
      myChart.setOption(option);

      // Handle window resize
      window.addEventListener('resize', function () {
        myChart.resize();
      });
    </script>
  </body>
</html>
```

## 11. Key Concepts

### Option Object Structure

The `option` object is the core configuration for ECharts charts:

```javascript
{
    // Components
    title: {},      // Chart title
    tooltip: {},    // Tooltip configuration
    legend: {},     // Legend configuration
    grid: {},       // Grid/plot area configuration

    // Axes
    xAxis: {},      // X-axis configuration
    yAxis: {},      // Y-axis configuration

    // Data series
    series: []      // Array of series configurations
}
```

### Data Formats

ECharts supports multiple data formats:

**Simple array:**

```javascript
data: [820, 932, 901, 934, 1290, 1330, 1320];
```

**Array of arrays (for scatter/line with coordinates):**

```javascript
data: [
  [15, 0],
  [-50, 10],
  [-56.5, 20],
  [-46.5, 30],
];
```

**Array of objects:**

```javascript
data: [
  { value: 820, name: 'Mon' },
  { value: 932, name: 'Tue' },
];
```

## 12. Best Practices

1. **Always set container dimensions**: The chart container must have explicit width and height
2. **Handle window resize**: Use `myChart.resize()` for responsive charts
3. **Use dataset for complex data**: Dataset-based binding is cleaner for large datasets
4. **Set appropriate axis types**: Use `'category'` for discrete data, `'value'` for continuous
5. **Configure tooltips**: Always configure tooltips for better user experience
6. **Use legends for multiple series**: Help users distinguish between different data series

## Summary

Creating a basic line chart in ECharts involves:

1. **Setup**: Include ECharts library and create a container element
2. **Initialization**: Use `echarts.init()` to create a chart instance
3. **Configuration**: Define the `option` object with axes and series
4. **Rendering**: Call `setOption()` to render the chart

The minimal configuration requires:

- `xAxis` with category data
- `yAxis` (can be empty)
- `series` with type `'line'` and data values

ECharts provides extensive customization options for styling, interactivity, and data handling, making it suitable for both simple and complex visualization needs.
