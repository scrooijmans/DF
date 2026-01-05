# Apache ECharts Cursor Types Configuration

## Overview

Apache ECharts allows you to configure the mouse cursor style for different chart elements to provide visual feedback and enhance user interaction. Cursor types are specified using standard CSS cursor values, which change the appearance of the mouse pointer when hovering over interactive chart elements.

## Series-Level Cursor Configuration

### Basic Syntax

You can configure the cursor style for any series type using the `cursor` property:

```javascript
option = {
  series: [
    {
      type: 'line',
      cursor: 'pointer', // Changes cursor on hover
    },
  ],
};
```

### Supported Series Types

ECharts supports cursor configuration for the following series types:

- `series-line.cursor` - Line charts
- `series-bar.cursor` - Bar charts
- `series-pie.cursor` - Pie charts
- `series-scatter.cursor` - Scatter plots
- `series-effectScatter.cursor` - Effect scatter plots
- `series-graph.cursor` - Graph/network diagrams
- `series-pictorialBar.cursor` - Pictorial bar charts

## Available Cursor Types

ECharts supports all standard CSS cursor values. Here are the most commonly used types:

### 1. `'pointer'`

**Use Case**: Indicates clickable elements or interactive items

```javascript
{
    type: 'line',
    cursor: 'pointer'  // Hand with pointing finger
}
```

**Best For**:

- Clickable data points
- Interactive legends
- Series that trigger actions on click

### 2. `'crosshair'`

**Use Case**: Precise selection or measurement

```javascript
{
    type: 'bar',
    cursor: 'crosshair'  // Crosshair cursor
}
```

**Best For**:

- Bar charts requiring precise selection
- Charts with axis pointer interactions
- Measurement or analysis tools

### 3. `'default'`

**Use Case**: Standard cursor, no special interaction

```javascript
{
    type: 'pie',
    cursor: 'default'  // Standard arrow cursor
}
```

**Best For**:

- Non-interactive elements
- Read-only visualizations
- When you want to disable custom cursor behavior

### 4. `'move'`

**Use Case**: Draggable or movable elements

```javascript
{
    type: 'pictorialBar',
    cursor: 'move'  // Four-directional arrow
}
```

**Best For**:

- Draggable chart elements
- Pan operations
- Elements that can be repositioned

### 5. `'grab'` / `'grabbing'`

**Use Case**: Elements that can be grabbed and dragged

```javascript
{
    type: 'graph',
    cursor: 'grab'  // Open hand (grab)
    // Use 'grabbing' when actively dragging
}
```

**Best For**:

- Force-directed graphs
- Network diagrams with draggable nodes
- Interactive graph layouts

### 6. `'help'`

**Use Case**: Elements with additional information or tooltips

```javascript
{
    type: 'scatter',
    cursor: 'help'  // Question mark cursor
}
```

**Best For**:

- Data points with detailed tooltips
- Educational visualizations
- Elements that provide contextual help

### 7. `'text'`

**Use Case**: Text selection or editing

```javascript
{
    type: 'line',
    cursor: 'text'  // Text selection cursor (I-beam)
}
```

**Best For**:

- Editable labels
- Text-based interactions
- Input fields within charts

### 8. `'wait'`

**Use Case**: Loading or processing states

```javascript
{
    type: 'line',
    cursor: 'wait'  // Hourglass/spinner cursor
}
```

**Best For**:

- Charts loading data
- Processing states
- Async operations

### 9. `'not-allowed'`

**Use Case**: Disabled or unavailable interactions

```javascript
{
    type: 'bar',
    cursor: 'not-allowed'  // Prohibited symbol
}
```

**Best For**:

- Disabled series
- Restricted interactions
- Read-only modes

### 10. `'zoom-in'` / `'zoom-out'`

**Use Case**: Zoom operations

```javascript
{
    type: 'scatter',
    cursor: 'zoom-in'  // Magnifying glass with plus
}
```

**Best For**:

- Zoomable charts
- Data exploration tools
- Interactive maps

## Component-Level Cursor Configuration

### DataZoom Component

The DataZoom component has specific cursor handling for its move handles:

```javascript
{
  dataZoom: [
    {
      type: 'slider',
      cursor: 'default', // Cursor for move handler
    },
  ];
}
```

**Note**: In recent ECharts versions, the DataZoom move handler cursor was changed to `'default'` for a more consistent user experience (Issue #20304).

### VisualMap Component

The VisualMap component automatically manages cursor styles based on interaction state:

```javascript
{
    visualMap: {
        selectedMode: true,  // When enabled, cursor is 'pointer'
        // When disabled, cursor reverts to default
    }
}
```

**Fix**: ECharts addresses an issue where the cursor remained `'pointer'` even when `selectedMode` was disabled (Issue #20551).

## Practical Examples

### Example 1: Multi-Series Chart with Different Cursors

```javascript
option = {
  series: [
    {
      type: 'line',
      name: 'Temperature',
      cursor: 'pointer', // Clickable line
      data: [20, 22, 25, 23, 21],
    },
    {
      type: 'bar',
      name: 'Precipitation',
      cursor: 'crosshair', // Precise bar selection
      data: [10, 15, 12, 18, 14],
    },
    {
      type: 'scatter',
      name: 'Events',
      cursor: 'help', // Additional info available
      data: [
        [1, 20],
        [2, 22],
        [3, 25],
      ],
    },
  ],
};
```

### Example 2: Interactive Graph with Grab Cursor

```javascript
option = {
  series: [
    {
      type: 'graph',
      layout: 'force',
      cursor: 'grab', // Indicates draggable nodes
      data: [
        { name: 'Node 1', value: 10 },
        { name: 'Node 2', value: 20 },
      ],
      links: [{ source: 'Node 1', target: 'Node 2' }],
    },
  ],
};
```

### Example 3: Pie Chart with Pointer Cursor

```javascript
option = {
  series: [
    {
      type: 'pie',
      cursor: 'pointer', // Clickable segments
      data: [
        { value: 335, name: 'Direct' },
        { value: 310, name: 'Email' },
        { value: 234, name: 'Ad' },
      ],
    },
  ],
};
```

## Advanced Configuration

### Conditional Cursor Based on State

While ECharts doesn't directly support conditional cursors in the option, you can achieve this through event handlers:

```javascript
myChart.on('mouseover', function (params) {
  // Change cursor based on data value
  if (params.value > 100) {
    myChart.getDom().style.cursor = 'pointer';
  } else {
    myChart.getDom().style.cursor = 'default';
  }
});

myChart.on('mouseout', function () {
  myChart.getDom().style.cursor = 'default';
});
```

### Custom Cursor with CSS

You can also use custom cursor images via CSS:

```css
.echarts-container {
  cursor: url('custom-cursor.png'), auto;
}
```

Or for specific series:

```javascript
// Apply via CSS class
option = {
  series: [
    {
      type: 'line',
      // Use CSS to set cursor
    },
  ],
};
```

```css
/* In your stylesheet */
.custom-chart .echarts-series {
  cursor: url('custom-pointer.png'), pointer;
}
```

## Best Practices

1. **Consistency**: Use consistent cursor types across similar interactions

   - `'pointer'` for clickable elements
   - `'grab'`/`'grabbing'` for draggable elements
   - `'crosshair'` for precise selection

2. **User Feedback**: Match cursor type to the available interaction

   - If elements are clickable, use `'pointer'`
   - If elements are draggable, use `'grab'` or `'move'`
   - If no interaction, use `'default'`

3. **Accessibility**: Consider cursor changes as part of accessibility

   - Clear visual feedback helps users understand interactivity
   - Consistent patterns improve usability

4. **Performance**: Cursor changes are lightweight
   - No performance impact from cursor configuration
   - Can be used liberally for better UX

## Browser Compatibility

All standard CSS cursor values are supported across modern browsers:

- Chrome/Edge: Full support
- Firefox: Full support
- Safari: Full support
- IE11: Most cursors supported (some newer values may not work)

## Version History

- **ECharts 4.0+**: Added support for `cursor` property on series
- **ECharts 5.0+**: Enhanced cursor handling for various components
- **Recent versions**: Fixed DataZoom move handler cursor to `'default'`
- **Recent versions**: Fixed VisualMap cursor when `selectedMode` is disabled

## Summary

ECharts provides flexible cursor configuration through the `cursor` property on series and components. The available cursor types include:

- **Interactive**: `'pointer'`, `'grab'`, `'grabbing'`
- **Precision**: `'crosshair'`
- **Navigation**: `'move'`, `'zoom-in'`, `'zoom-out'`
- **Information**: `'help'`
- **Standard**: `'default'`, `'text'`
- **States**: `'wait'`, `'not-allowed'`

Choose cursor types that match the interaction capabilities of your chart elements to provide clear visual feedback and enhance user experience.
