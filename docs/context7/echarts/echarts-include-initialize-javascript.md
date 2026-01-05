# How to Include and Initialize ECharts in a JavaScript Project

## Overview

This guide covers different methods for including and initializing Apache ECharts in JavaScript projects, from simple HTML scripts to modern module-based setups with tree-shaking support.

## 1. Installation Methods

### Method 1: NPM Installation (Recommended for Modern Projects)

Install ECharts as a dependency in your project:

```bash
npm install echarts --save
```

This installs ECharts in your `node_modules` directory and adds it to your `package.json` dependencies.

### Method 2: CDN (Quick Start)

Include ECharts directly from a CDN in your HTML:

```html
<script src="https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js"></script>
```

Or use the latest version:

```html
<script src="https://cdn.jsdelivr.net/npm/echarts/dist/echarts.min.js"></script>
```

### Method 3: Download and Include Locally

Download the ECharts library and include it locally:

```html
<script src="./path/to/echarts.min.js"></script>
```

## 2. Import Methods

### ES6 Modules - Full Import

Import the entire ECharts library (ECharts 5+):

```javascript
import * as echarts from 'echarts';
```

**Note:** In ECharts 5, default exports are removed. Always use `import * as echarts`.

**Complete Example:**

```typescript
import * as echarts from 'echarts';

// Initialize the echarts instance
var myChart = echarts.init(document.getElementById('main'));

// Draw the chart
myChart.setOption({
  title: {
    text: 'ECharts Getting Started Example',
  },
  tooltip: {},
  xAxis: {
    data: [
      'shirt',
      'cardigan',
      'chiffon',
      'pants',
      'heels',
      'socks',
    ],
  },
  yAxis: {},
  series: [
    {
      name: 'sales',
      type: 'bar',
      data: [5, 20, 36, 10, 10, 20],
    },
  ],
});
```

### ES6 Modules - Tree-Shaking (Recommended for Production)

For smaller bundle sizes, import only what you need:

```typescript
// Import the echarts core module
import * as echarts from 'echarts/core';

// Import bar charts (all charts have Chart suffix)
import { BarChart } from 'echarts/charts';

// Import components (all components have Component suffix)
import {
  TitleComponent,
  TooltipComponent,
  GridComponent,
} from 'echarts/components';

// Import the Canvas renderer
import { CanvasRenderer } from 'echarts/renderers';

// Register the required components
echarts.use([
  TitleComponent,
  TooltipComponent,
  GridComponent,
  BarChart,
  CanvasRenderer,
]);

// Now you can use echarts.init() and create charts
var myChart = echarts.init(document.getElementById('main'));
myChart.setOption({
  // ... your chart options
});
```

**Tree-Shaking Example for Line Chart:**

```typescript
import * as echarts from 'echarts/core';
import { LineChart } from 'echarts/charts';
import {
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent,
} from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';

// Register components
echarts.use([
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent,
  LineChart,
  CanvasRenderer,
]);

// Initialize and use
var myChart = echarts.init(document.getElementById('main'));
```

### CommonJS (Node.js / Older Projects)

For CommonJS modules:

```javascript
const echarts = require('echarts/lib/echarts');
require('echarts/lib/chart/bar');
require('echarts/lib/component/grid');
```

Or using the tree-shaking API:

```javascript
const echarts = require('echarts/core');
const { BarChart } = require('echarts/charts');
const { GridComponent } = require('echarts/components');
const { CanvasRenderer } = require('echarts/renderers');

echarts.use([BarChart, GridComponent, CanvasRenderer]);
```

## 3. Initialization

### Basic Initialization

Initialize an ECharts instance on a DOM element:

```javascript
// Using getElementById
var myChart = echarts.init(document.getElementById('main'));

// Using querySelector
var myChart = echarts.init(document.querySelector('#main'));

// Using a variable
var chartDom = document.getElementById('main');
var myChart = echarts.init(chartDom);
```

### Initialization with Theme

ECharts supports built-in themes:

```javascript
// Light theme
var myChart = echarts.init(
  document.getElementById('main'),
  'light'
);

// Dark theme
var myChart = echarts.init(
  document.getElementById('main'),
  'dark'
);

// Custom theme (requires theme file)
var myChart = echarts.init(
  document.getElementById('main'),
  'vintage'
);
```

### Initialization with Options

You can also pass initialization options:

```javascript
var myChart = echarts.init(
  document.getElementById('main'),
  null,
  {
    renderer: 'canvas', // or 'svg'
    width: 600,
    height: 400,
  }
);
```

### SVG Renderer Initialization

To use SVG rendering (useful for lower memory usage):

```typescript
// Import SVG renderer module
import 'zrender/lib/svg/svg';

// Initialize with SVG renderer
var chart = echarts.init(containerDom, null, {
  renderer: 'svg',
});
```

Or with tree-shaking:

```typescript
import * as echarts from 'echarts/core';
import { SVGRenderer } from 'echarts/renderers';

echarts.use([SVGRenderer]);

var chart = echarts.init(containerDom, null, {
  renderer: 'svg',
});
```

## 4. Complete Setup Examples

### Example 1: HTML with CDN (Simple)

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>ECharts Example</title>
    <script src="https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js"></script>
  </head>
  <body>
    <div
      id="main"
      style="width: 600px; height: 400px;"
    ></div>
    <script type="text/javascript">
      // Initialize echarts instance
      var myChart = echarts.init(
        document.getElementById('main')
      );

      // Specify chart configuration item and data
      var option = {
        title: {
          text: 'ECharts entry example',
        },
        tooltip: {},
        legend: {
          data: ['Sales'],
        },
        xAxis: {
          data: [
            'shirt',
            'cardigan',
            'chiffon',
            'pants',
            'heels',
            'socks',
          ],
        },
        yAxis: {},
        series: [
          {
            name: 'Sales',
            type: 'bar',
            data: [5, 20, 36, 10, 10, 20],
          },
        ],
      };

      // Use configuration item and data to show chart
      myChart.setOption(option);
    </script>
  </body>
</html>
```

### Example 2: ES6 Module with Full Import

```javascript
// main.js
import * as echarts from 'echarts';

function initChart() {
  var chartDom = document.getElementById('main');
  var myChart = echarts.init(chartDom);

  var option = {
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
        data: [820, 932, 901, 934, 1290, 1330, 1320],
        type: 'line',
      },
    ],
  };

  myChart.setOption(option);

  // Handle window resize
  window.addEventListener('resize', function () {
    myChart.resize();
  });
}

// Initialize when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initChart);
} else {
  initChart();
}
```

### Example 3: ES6 Module with Tree-Shaking

```typescript
// chart.ts
import * as echarts from 'echarts/core';
import { LineChart } from 'echarts/charts';
import {
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent,
} from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';

// Register required components
echarts.use([
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent,
  LineChart,
  CanvasRenderer,
]);

export function createLineChart(
  containerId: string,
  data: number[]
) {
  const chartDom = document.getElementById(containerId);
  if (!chartDom) {
    throw new Error(`Container ${containerId} not found`);
  }

  const myChart = echarts.init(chartDom);

  const option = {
    title: {
      text: 'Line Chart',
    },
    tooltip: {
      trigger: 'axis',
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
        name: 'Data',
        type: 'line',
        smooth: true,
        data: data,
      },
    ],
  };

  myChart.setOption(option);

  // Return chart instance for further manipulation
  return myChart;
}

// Usage
const chart = createLineChart(
  'main',
  [820, 932, 901, 934, 1290, 1330, 1320]
);
```

### Example 4: React Component

```jsx
import React, { useEffect, useRef } from 'react';
import * as echarts from 'echarts/core';
import { LineChart } from 'echarts/charts';
import {
  TitleComponent,
  TooltipComponent,
  GridComponent,
} from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';

echarts.use([
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LineChart,
  CanvasRenderer,
]);

function LineChartComponent({ data }) {
  const chartRef = useRef(null);
  const chartInstance = useRef(null);

  useEffect(() => {
    if (chartRef.current) {
      // Initialize chart
      chartInstance.current = echarts.init(
        chartRef.current
      );

      const option = {
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
            data: data,
            type: 'line',
          },
        ],
      };

      chartInstance.current.setOption(option);

      // Handle resize
      const handleResize = () => {
        chartInstance.current?.resize();
      };
      window.addEventListener('resize', handleResize);

      return () => {
        window.removeEventListener('resize', handleResize);
        chartInstance.current?.dispose();
      };
    }
  }, [data]);

  return (
    <div
      ref={chartRef}
      style={{ width: '100%', height: '400px' }}
    />
  );
}

export default LineChartComponent;
```

### Example 5: Vue.js Component

```vue
<template>
  <div
    ref="chartContainer"
    style="width: 100%; height: 400px"
  ></div>
</template>

<script>
import * as echarts from 'echarts/core';
import { LineChart } from 'echarts/charts';
import {
  TitleComponent,
  TooltipComponent,
  GridComponent,
} from 'echarts/components';
import { CanvasRenderer } from 'echarts/renderers';

echarts.use([
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LineChart,
  CanvasRenderer,
]);

export default {
  name: 'LineChart',
  props: {
    data: {
      type: Array,
      required: true,
    },
  },
  data() {
    return {
      chartInstance: null,
    };
  },
  mounted() {
    this.initChart();
    window.addEventListener('resize', this.handleResize);
  },
  beforeUnmount() {
    window.removeEventListener('resize', this.handleResize);
    if (this.chartInstance) {
      this.chartInstance.dispose();
    }
  },
  watch: {
    data: {
      handler() {
        this.updateChart();
      },
      deep: true,
    },
  },
  methods: {
    initChart() {
      this.chartInstance = echarts.init(
        this.$refs.chartContainer
      );

      const option = {
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
            data: this.data,
            type: 'line',
          },
        ],
      };

      this.chartInstance.setOption(option);
    },
    updateChart() {
      if (this.chartInstance) {
        this.chartInstance.setOption({
          series: [
            {
              data: this.data,
            },
          ],
        });
      }
    },
    handleResize() {
      if (this.chartInstance) {
        this.chartInstance.resize();
      }
    },
  },
};
</script>
```

## 5. Key Points

### ECharts 5 Changes

**Important:** ECharts 5 removed default exports. Always use:

```javascript
// ✅ Correct
import * as echarts from 'echarts';

// ❌ Wrong (doesn't work in ECharts 5)
import echarts from 'echarts';
```

### Container Requirements

- The container element must have explicit width and height
- Can be set via CSS or inline styles
- The container should exist in the DOM before initialization

```javascript
// Container must have dimensions
<div id="main" style="width: 600px; height: 400px;"></div>

// Or via CSS
#main {
  width: 600px;
  height: 400px;
}
```

### Responsive Charts

Always handle window resize for responsive charts:

```javascript
window.addEventListener('resize', function () {
  myChart.resize();
});
```

### Cleanup

Dispose of chart instances when no longer needed:

```javascript
myChart.dispose();
```

## 6. Available Charts and Components

When using tree-shaking, import only what you need:

**Charts:**

- `BarChart`, `LineChart`, `PieChart`, `ScatterChart`
- `RadarChart`, `MapChart`, `TreeChart`, `TreemapChart`
- `GraphChart`, `GaugeChart`, `FunnelChart`, `ParallelChart`
- `SankeyChart`, `BoxplotChart`, `CandlestickChart`
- `EffectScatterChart`, `LinesChart`, `HeatmapChart`
- `PictorialBarChart`, `ThemeRiverChart`, `SunburstChart`
- `CustomChart`

**Components:**

- `TitleComponent`, `TooltipComponent`, `LegendComponent`
- `GridComponent`, `PolarComponent`, `RadarComponent`
- `GeoComponent`, `SingleAxisComponent`, `ParallelComponent`
- `CalendarComponent`, `GraphicComponent`, `BrushComponent`
- `MarkPointComponent`, `MarkLineComponent`, `MarkAreaComponent`
- `TimelineComponent`, `VisualMapComponent`, `DataZoomComponent`
- `DataZoomInsideComponent`, `DataZoomSliderComponent`
- `ToolboxComponent`, `AxisPointerComponent`

**Renderers:**

- `CanvasRenderer` (default)
- `SVGRenderer`

## 7. Best Practices

1. **Use Tree-Shaking for Production**: Import only necessary modules to reduce bundle size
2. **Handle Resize**: Always add resize listeners for responsive charts
3. **Dispose Properly**: Clean up chart instances when components unmount
4. **Check Container Existence**: Verify DOM element exists before initialization
5. **Use TypeScript**: ECharts has excellent TypeScript support
6. **Theme Management**: Use themes for consistent styling across charts
7. **Error Handling**: Wrap initialization in try-catch blocks

## Summary

Including and initializing ECharts in JavaScript projects:

1. **Install**: Use `npm install echarts` or include via CDN
2. **Import**: Use `import * as echarts from 'echarts'` (ES6) or tree-shaking approach
3. **Initialize**: Call `echarts.init(domElement)` with a DOM container
4. **Configure**: Use `setOption()` to set chart configuration
5. **Handle Resize**: Add window resize listeners for responsive behavior

Choose the method that best fits your project:

- **CDN**: Quick prototyping, simple HTML pages
- **Full Import**: Simple projects, all features needed
- **Tree-Shaking**: Production apps, bundle size optimization
