# Apache ECharts: Lasso and Rectangular Selection (Brush Component)

## Overview

Yes, Apache ECharts supports both **rectangular** and **lasso (polygon)** selection to select points in charts through the **Brush Component**. The brush component provides interactive tools for selecting data items within specified areas on the chart, enabling users to highlight and analyze specific data points.

## Brush Component Overview

The Brush component is a powerful feature that allows users to:
- Select data points using rectangular areas
- Select data points using polygon (lasso) areas
- Select data along axes using line selections
- Link selections across multiple charts
- Programmatically control selections

## Brush Types

ECharts supports the following brush types:

1. **`'rect'`** - Rectangular selection
2. **`'polygon'`** - Polygon/lasso selection (free-form)
3. **`'lineX'`** - Vertical line selection (selects along X-axis)
4. **`'lineY'`** - Horizontal line selection (selects along Y-axis)

## Basic Configuration

### Enable Brush Component

To enable brush selection, add the `brush` component to your chart options:

```javascript
option = {
  brush: {
    toolbox: ['rect', 'polygon', 'keep', 'clear'],
    // ... other brush options
  },
  series: [
    {
      type: 'scatter',
      data: [
        [10, 20],
        [15, 25],
        [20, 30],
        // ... more data points
      ],
    },
  ],
};
```

### Toolbox Integration

The brush tools can be added to the toolbox for easy access:

```javascript
option = {
  toolbox: {
    feature: {
      brush: {
        type: ['rect', 'polygon', 'keep', 'clear'],
        title: {
          rect: 'Rectangle selection',
          polygon: 'Lasso selection',
          keep: 'Keep selection',
          clear: 'Clear selection',
        },
      },
    },
  },
  brush: {
    // Brush configuration
  },
  series: [
    {
      type: 'scatter',
      data: [
        [10, 20],
        [15, 25],
        [20, 30],
      ],
    },
  ],
};
```

## Rectangular Selection

### Basic Rectangular Selection

```javascript
option = {
  brush: {
    toolbox: ['rect', 'clear'],
    brushMode: 'single', // or 'multiple'
  },
  xAxis: {
    type: 'value',
  },
  yAxis: {
    type: 'value',
  },
  series: [
    {
      type: 'scatter',
      data: [
        [10, 20],
        [15, 25],
        [20, 30],
        [25, 35],
        [30, 40],
      ],
    },
  ],
};
```

### Rectangular Selection with Custom Styling

```javascript
option = {
  brush: {
    toolbox: ['rect', 'clear'],
    brushMode: 'multiple', // Allow multiple rectangles
    brushStyle: {
      borderWidth: 2,
      color: 'rgba(120, 140, 180, 0.3)',
      borderColor: 'rgba(120, 140, 180, 0.8)',
    },
    outOfBrush: {
      opacity: 0.1, // Dim unselected points
      color: '#aaa',
    },
  },
  series: [
    {
      type: 'scatter',
      data: [
        [10, 20],
        [15, 25],
        [20, 30],
      ],
    },
  ],
};
```

## Lasso (Polygon) Selection

### Basic Polygon Selection

```javascript
option = {
  brush: {
    toolbox: ['polygon', 'clear'],
    brushType: 'polygon', // Enable polygon selection
    brushMode: 'single', // or 'multiple'
  },
  xAxis: {
    type: 'value',
  },
  yAxis: {
    type: 'value',
  },
  series: [
    {
      type: 'scatter',
      data: [
        [10, 20],
        [15, 25],
        [20, 30],
        [25, 35],
        [30, 40],
      ],
    },
  ],
};
```

### Polygon Selection with Multiple Tools

```javascript
option = {
  brush: {
    toolbox: ['polygon', 'rect', 'keep', 'clear'],
    // Users can switch between polygon and rectangle
    brushMode: 'multiple', // Allow multiple selections
    brushStyle: {
      borderWidth: 2,
      color: 'rgba(255, 0, 0, 0.3)',
      borderColor: 'rgba(255, 0, 0, 0.8)',
    },
  },
  series: [
    {
      type: 'scatter',
      data: [
        [10, 20],
        [15, 25],
        [20, 30],
      ],
    },
  ],
};
```

## Complete Example: Scatter Plot with Both Selection Types

```javascript
option = {
  tooltip: {
    trigger: 'item',
  },
  toolbox: {
    feature: {
      brush: {
        type: ['rect', 'polygon', 'clear'],
        title: {
          rect: 'Rectangle Selection',
          polygon: 'Lasso Selection',
          clear: 'Clear Selection',
        },
      },
    },
  },
  brush: {
    toolbox: ['rect', 'polygon', 'keep', 'clear'],
    brushMode: 'multiple', // Allow multiple selections
    brushStyle: {
      borderWidth: 2,
      color: 'rgba(120, 140, 180, 0.3)',
      borderColor: 'rgba(120, 140, 180, 0.8)',
    },
    outOfBrush: {
      opacity: 0.1,
      color: '#aaa',
    },
  },
  xAxis: {
    type: 'value',
    name: 'X Axis',
  },
  yAxis: {
    type: 'value',
    name: 'Y Axis',
  },
  series: [
    {
      type: 'scatter',
      name: 'Data Points',
      data: [
        [10, 20],
        [15, 25],
        [20, 30],
        [25, 35],
        [30, 40],
        [35, 45],
        [40, 50],
      ],
      symbolSize: 10,
    },
  ],
};
```

## Brush Modes

### Single Selection Mode

In single selection mode, only one selection box is available at a time. Clicking on a blank area removes the selection.

```javascript
brush: {
  brushMode: 'single',
  toolbox: ['rect', 'polygon', 'clear'],
}
```

### Multiple Selection Mode

In multiple selection mode, multiple selection boxes can be created. Selections cannot be removed by clicking blank areas; use the clear button instead.

```javascript
brush: {
  brushMode: 'multiple',
  toolbox: ['rect', 'polygon', 'keep', 'clear'],
}
```

## Handling Brush Events

### Listening to Selection Events

You can listen to the `'brushSelected'` event to handle selections:

```javascript
const dataBySeries = [
  [12, 23, 54, 6], // Data of series 0
  [34, 34433, 2223, 21122, 1232, 34], // Data of series 1
];

chart.setOption({
  brush: {
    toolbox: ['rect', 'polygon', 'clear'],
  },
  series: [
    {
      data: dataBySeries[0],
    },
    {
      data: dataBySeries[1],
    },
  ],
});

chart.on('brushSelected', function (params) {
  const brushComponent = params.batch[0];
  let sum = 0; // Sum of all selected values

  for (let sIdx = 0; sIdx < brushComponent.selected.length; sIdx++) {
    const dataIndices = brushComponent.selected[sIdx].dataIndex;

    for (let i = 0; i < dataIndices.length; i++) {
      const dataIndex = dataIndices[i];
      sum += dataBySeries[sIdx][dataIndex];
    }
  }

  console.log('Sum of selected values:', sum);
  console.log('Selected data indices:', brushComponent.selected);
});
```

### Event Parameters

The `brushSelected` event provides:
- `batch`: Array of brush components
- `selected`: Array of selected series with their data indices
- `areas`: Array of selection areas

## Programmatic Control

### Creating Selections Programmatically

You can programmatically create selections using `dispatchAction`:

```javascript
// Rectangular selection
myChart.dispatchAction({
  type: 'brush',
  areas: [
    {
      xAxisIndex: 0,
      yAxisIndex: 0,
      brushType: 'rect',
      coordRange: [
        [10, 20], // [minX, maxX]
        [15, 25], // [minY, maxY]
      ],
    },
  ],
});

// Polygon/lasso selection
myChart.dispatchAction({
  type: 'brush',
  areas: [
    {
      xAxisIndex: 0,
      yAxisIndex: 0,
      brushType: 'polygon',
      coordRange: [
        [10, 20], // Point 1
        [15, 25], // Point 2
        [20, 30], // Point 3
        [25, 35], // Point 4
        // ... more points
      ],
    },
  ],
});

// Clear all selections
myChart.dispatchAction({
  type: 'brush',
  areas: [], // Empty array clears all selections
});
```

### Activating Brush Mode Programmatically

You can activate brush mode using `takeGlobalCursor`:

```javascript
// Activate rectangle brush mode
myChart.dispatchAction({
  type: 'takeGlobalCursor',
  key: 'brush',
  brushOption: {
    brushType: 'rect',
    brushMode: 'single', // or 'multiple'
  },
});

// Activate polygon brush mode
myChart.dispatchAction({
  type: 'takeGlobalCursor',
  key: 'brush',
  brushOption: {
    brushType: 'polygon',
    brushMode: 'multiple',
  },
});

// Deactivate brush mode
myChart.dispatchAction({
  type: 'takeGlobalCursor',
  key: 'brush',
  brushOption: {
    brushType: false, // Disable brush
  },
});
```

## Linking Selections Across Charts

### Brush Linking

You can link brush selections across multiple series or charts:

```javascript
option = {
  brush: {
    toolbox: ['rect', 'polygon', 'clear'],
    brushLink: 'all', // Link all series
    // or
    brushLink: [0, 1], // Link specific series indices
  },
  series: [
    {
      type: 'bar',
      data: [232, 4434, 545, 654], // Series 0
    },
    {
      type: 'scatter',
      data: [
        [4, 5],
        [3, 5],
        [66, 33],
        [99, 66],
      ], // Series 1 - must have same number of data points
    },
  ],
};
```

**Note**: For `brushLink` to work, the linked series must have the same number of data points, as selections are linked by data index.

## Coordinate System Configuration

### Limiting Brush to Specific Axes

You can limit brush selection to specific coordinate systems:

```javascript
option = {
  grid: [
    {}, // grid 0
    {}, // grid 1
  ],
  xAxis: [
    { gridIndex: 1 }, // xAxis 0 for grid 1
    { gridIndex: 0 }, // xAxis 1 for grid 0
  ],
  yAxis: [
    { gridIndex: 1 }, // yAxis 0 for grid 1
    { gridIndex: 0 }, // yAxis 1 for grid 0
  ],
  brush: {
    xAxisIndex: [0, 1], // Enable brush only on these x-axes
    yAxisIndex: [0, 1], // Enable brush only on these y-axes
    toolbox: ['rect', 'polygon', 'clear'],
  },
  series: [
    {
      type: 'scatter',
      data: [
        [10, 20],
        [15, 25],
      ],
    },
  ],
};
```

### Geo Coordinate System

For geo/map charts:

```javascript
option = {
  geo: {
    map: 'china',
  },
  brush: {
    geoIndex: 'all', // Enable brush on all geo charts
    // or
    geoIndex: 0, // Enable brush on specific geo chart
    toolbox: ['rect', 'polygon', 'clear'],
  },
  series: [
    {
      type: 'scatter',
      coordinateSystem: 'geo',
      data: [
        [119.72, 34.85, 100],
        [121.48, 31.22, 200],
      ],
    },
  ],
};
```

## Styling Options

### Brush Style

Customize the appearance of selection areas:

```javascript
brush: {
  brushStyle: {
    borderWidth: 2,
    color: 'rgba(120, 140, 180, 0.3)', // Fill color
    borderColor: 'rgba(120, 140, 180, 0.8)', // Border color
  },
}
```

### Out-of-Brush Styling

Style elements that are not selected:

```javascript
brush: {
  outOfBrush: {
    opacity: 0.1, // Dim unselected points
    color: '#aaa', // Change color of unselected points
  },
}
```

### In-Brush Styling

Style elements that are selected (via series configuration):

```javascript
series: [
  {
    type: 'scatter',
    data: [
      [10, 20],
      [15, 25],
    ],
    emphasis: {
      itemStyle: {
        borderColor: '#000',
        borderWidth: 2,
      },
    },
  },
];
```

## Advanced Configuration

### Throttling for Performance

For large datasets, you can throttle brush events:

```javascript
brush: {
  throttleType: 'debounce', // or 'fixRate'
  throttleDelay: 100, // Delay in milliseconds
  toolbox: ['rect', 'polygon', 'clear'],
}
```

### Z-Index Control

Control the rendering order:

```javascript
brush: {
  z: 10000, // Default z-index
  toolbox: ['rect', 'polygon', 'clear'],
}
```

## Complete Working Example

```javascript
const option = {
  tooltip: {
    trigger: 'item',
  },
  toolbox: {
    feature: {
      brush: {
        type: ['rect', 'polygon', 'clear'],
        title: {
          rect: 'Rectangle Selection',
          polygon: 'Lasso Selection',
          clear: 'Clear Selection',
        },
      },
    },
  },
  brush: {
    toolbox: ['rect', 'polygon', 'keep', 'clear'],
    brushMode: 'multiple',
    brushStyle: {
      borderWidth: 2,
      color: 'rgba(120, 140, 180, 0.3)',
      borderColor: 'rgba(120, 140, 180, 0.8)',
    },
    outOfBrush: {
      opacity: 0.1,
      color: '#aaa',
    },
  },
  xAxis: {
    type: 'value',
    name: 'X Value',
  },
  yAxis: {
    type: 'value',
    name: 'Y Value',
  },
  series: [
    {
      type: 'scatter',
      name: 'Data Points',
      data: [
        [10, 20],
        [15, 25],
        [20, 30],
        [25, 35],
        [30, 40],
        [35, 45],
        [40, 50],
        [45, 55],
        [50, 60],
      ],
      symbolSize: 12,
      itemStyle: {
        color: '#5470c6',
      },
    },
  ],
};

const chart = echarts.init(document.getElementById('chart'));
chart.setOption(option);

// Handle selection events
chart.on('brushSelected', function (params) {
  console.log('Selection event:', params);
  const selected = params.batch[0].selected;
  
  selected.forEach(function (item) {
    console.log('Series index:', item.seriesIndex);
    console.log('Data indices:', item.dataIndex);
  });
});
```

## Summary

Apache ECharts fully supports both **rectangular** and **lasso (polygon)** selection through the Brush component:

- **Rectangular Selection**: Use `brushType: 'rect'` or `toolbox: ['rect']`
- **Lasso/Polygon Selection**: Use `brushType: 'polygon'` or `toolbox: ['polygon']`
- **Multiple Selection Modes**: Single or multiple selections
- **Event Handling**: Listen to `brushSelected` events
- **Programmatic Control**: Use `dispatchAction` to create/clear selections
- **Cross-Chart Linking**: Link selections across multiple series
- **Custom Styling**: Customize selection appearance and unselected elements

The brush component is highly configurable and provides a powerful way to enable interactive data selection in ECharts visualizations.

