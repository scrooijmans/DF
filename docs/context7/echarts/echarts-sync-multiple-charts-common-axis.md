# Apache ECharts: Syncing Multiple Charts to a Common Axis

## Overview

Apache ECharts provides several methods to synchronize multiple chart instances so they share a common axis and maintain a unified visible range. When you zoom or pan on one chart, all connected charts update their viewports to match. This is particularly useful for:

- **Well log displays**: Multiple tracks showing different measurements aligned by depth
- **Time series dashboards**: Multiple charts synchronized to a common time axis
- **Multi-panel visualizations**: Related charts that should stay aligned

## Method 1: Using `echarts.connect()` for Basic Synchronization

The simplest way to synchronize charts is using ECharts' built-in `connect()` method, which automatically syncs tooltips, cursors, and basic interactions.

### Basic Setup

```javascript
// Initialize multiple chart instances
var chart1 = echarts.init(document.getElementById('chart1'));
var chart2 = echarts.init(document.getElementById('chart2'));
var chart3 = echarts.init(document.getElementById('chart3'));

// Configure each chart with dataZoom
var commonOption = {
  xAxis: {
    type: 'value',
    // ... other x-axis config
  },
  yAxis: {
    type: 'value',
    // ... other y-axis config
  },
  dataZoom: [
    {
      type: 'inside',
      xAxisIndex: 0,  // Enable x-axis zooming
      filterMode: 'none',
      throttle: 50
    }
  ],
  series: [
    // ... series configuration
  ]
};

// Apply the same base configuration to all charts
chart1.setOption(commonOption);
chart2.setOption(commonOption);
chart3.setOption(commonOption);

// Connect all charts - this syncs tooltips and cursors
echarts.connect([chart1, chart2, chart3]);
```

### Using Group IDs for Better Organization

Instead of passing chart instances directly, you can use group IDs for more flexible management:

```javascript
// Initialize charts with group IDs
var chart1 = echarts.init(document.getElementById('chart1'), null, {
  group: 'well-log-group'
});
var chart2 = echarts.init(document.getElementById('chart2'), null, {
  group: 'well-log-group'
});
var chart3 = echarts.init(document.getElementById('chart3'), null, {
  group: 'well-log-group'
});

// Connect by group ID
echarts.connect('well-log-group');
```

**Note**: `echarts.connect()` primarily synchronizes **tooltips and cursors**. For full viewport synchronization (zoom/pan), you need Method 2.

## Method 2: Manual Viewport Synchronization with `dataZoom` Events

For complete control over viewport synchronization, listen to `datazoom` events and manually update other charts. This allows you to:

- Sync only specific axes (x or y)
- Control which charts are synchronized
- Add custom logic for synchronization

### Complete Implementation

```javascript
// Chart instances
var chart1 = echarts.init(document.getElementById('chart1'));
var chart2 = echarts.init(document.getElementById('chart2'));
var chart3 = echarts.init(document.getElementById('chart3'));

// Flag to prevent recursive updates
var isUpdating = false;

// Common chart configuration
function createChartOption(chartData) {
  return {
    xAxis: {
      type: 'value',
      name: 'Depth (m)',
      // ... other config
    },
    yAxis: {
      type: 'value',
      name: 'Value',
      // ... other config
    },
    dataZoom: [
      {
        type: 'inside',
        xAxisIndex: 0,  // Sync x-axis (depth)
        filterMode: 'none',
        throttle: 50
      },
      {
        type: 'inside',
        yAxisIndex: 0,  // Don't sync y-axis (each chart has different values)
        filterMode: 'none',
        throttle: 50
      }
    ],
    series: [
      {
        type: 'line',
        data: chartData
      }
    ]
  };
}

// Set initial options
chart1.setOption(createChartOption(data1));
chart2.setOption(createChartOption(data2));
chart3.setOption(createChartOption(data3));

// Function to sync viewport from source chart to target charts
function syncViewport(sourceChart, targetCharts, axisToSync) {
  if (isUpdating) return;
  
  isUpdating = true;
  try {
    const option = sourceChart.getOption();
    const dataZoom = option.dataZoom || [];
    
    // Find the dataZoom configuration for the axis we want to sync
    const axisZoom = dataZoom.find(dz => {
      if (axisToSync === 'x') {
        return dz.xAxisIndex === 0;
      } else if (axisToSync === 'y') {
        return dz.yAxisIndex === 0;
      }
      return false;
    });
    
    if (!axisZoom) return;
    
    // Update all target charts with the same zoom state
    targetCharts.forEach(chart => {
      if (chart === sourceChart) return; // Skip source chart
      
      const syncOption = {
        dataZoom: [{
          [axisToSync === 'x' ? 'xAxisIndex' : 'yAxisIndex']: 0,
          start: axisZoom.start,
          end: axisZoom.end,
          startValue: axisZoom.startValue,
          endValue: axisZoom.endValue
        }]
      };
      
      chart.setOption(syncOption, { notMerge: false });
    });
  } finally {
    isUpdating = false;
  }
}

// All charts except the one being interacted with
const allCharts = [chart1, chart2, chart3];

// Setup event listeners for each chart
allCharts.forEach(chart => {
  chart.on('datazoom', () => {
    syncViewport(chart, allCharts, 'x'); // Sync x-axis only
  });
});

// Also connect for cursor sync
echarts.connect(allCharts);
```

## Method 3: Syncing Specific Axes (X or Y Only)

For well logs or depth-based visualizations, you typically want to sync only the depth axis (usually Y-axis) while allowing each chart's value axis (X-axis) to remain independent.

### Y-Axis Synchronization (Depth Axis)

```javascript
var chart1 = echarts.init(document.getElementById('chart1'));
var chart2 = echarts.init(document.getElementById('chart2'));

// Configuration with Y-axis as depth (inverted)
var option1 = {
  xAxis: {
    type: 'value',
    name: 'Resistivity',
    // Each chart has different value ranges
  },
  yAxis: {
    type: 'value',
    name: 'Depth (m)',
    inverse: true,  // Depth increases downward
    min: 0,
    max: 1000
  },
  dataZoom: [
    {
      type: 'inside',
      yAxisIndex: 0,  // Only enable Y-axis zoom (depth)
      filterMode: 'none'
    }
  ],
  series: [/* ... */]
};

var option2 = {
  xAxis: {
    type: 'value',
    name: 'Density',
    // Different range than chart1
  },
  yAxis: {
    type: 'value',
    name: 'Depth (m)',
    inverse: true,
    min: 0,  // Same depth range
    max: 1000
  },
  dataZoom: [
    {
      type: 'inside',
      yAxisIndex: 0,  // Only Y-axis zoom
      filterMode: 'none'
    }
  ],
  series: [/* ... */]
};

chart1.setOption(option1);
chart2.setOption(option2);

// Sync Y-axis (depth) only
var isUpdating = false;
[chart1, chart2].forEach(chart => {
  chart.on('datazoom', () => {
    if (isUpdating) return;
    isUpdating = true;
    
    const sourceOption = chart.getOption();
    const sourceDataZoom = sourceOption.dataZoom || [];
    const yZoom = sourceDataZoom.find(dz => dz.yAxisIndex === 0);
    
    if (yZoom) {
      [chart1, chart2].forEach(targetChart => {
        if (targetChart === chart) return;
        targetChart.setOption({
          dataZoom: [{
            yAxisIndex: 0,
            start: yZoom.start,
            end: yZoom.end,
            startValue: yZoom.startValue,
            endValue: yZoom.endValue
          }]
        });
      });
    }
    
    isUpdating = false;
  });
});

// Connect for cursor sync
echarts.connect([chart1, chart2]);
```

### X-Axis Synchronization (Time or Index Axis)

For time series or index-based charts, sync the X-axis:

```javascript
var chart1 = echarts.init(document.getElementById('chart1'));
var chart2 = echarts.init(document.getElementById('chart2'));

var commonOption = {
  xAxis: {
    type: 'time',  // or 'category' or 'value'
    // Common time/index range
  },
  yAxis: {
    type: 'value',
    // Each chart can have different Y ranges
  },
  dataZoom: [
    {
      type: 'inside',
      xAxisIndex: 0,  // Only X-axis zoom
      filterMode: 'none'
    }
  ],
  series: [/* ... */]
};

chart1.setOption({ ...commonOption, series: [series1] });
chart2.setOption({ ...commonOption, series: [series2] });

// Sync X-axis
var isUpdating = false;
[chart1, chart2].forEach(chart => {
  chart.on('datazoom', () => {
    if (isUpdating) return;
    isUpdating = true;
    
    const sourceOption = chart.getOption();
    const xZoom = sourceOption.dataZoom?.find(dz => dz.xAxisIndex === 0);
    
    if (xZoom) {
      [chart1, chart2].forEach(targetChart => {
        if (targetChart === chart) return;
        targetChart.setOption({
          dataZoom: [{
            xAxisIndex: 0,
            start: xZoom.start,
            end: xZoom.end,
            startValue: xZoom.startValue,
            endValue: xZoom.endValue
          }]
        });
      });
    }
    
    isUpdating = false;
  });
});

echarts.connect([chart1, chart2]);
```

## Method 4: Advanced Pattern with Chart Manager (Production-Ready)

For production applications, use a chart manager pattern to handle multiple chart groups, prevent recursive updates, and provide a clean API.

### Chart Manager Implementation

```javascript
class ChartSyncManager {
  constructor() {
    this.charts = new Map();
    this.groups = new Map();
    this.isUpdating = false;
  }
  
  /**
   * Register a chart in a sync group
   * @param {string} chartId - Unique chart identifier
   * @param {ECharts} instance - ECharts instance
   * @param {string} groupId - Group ID for synchronization
   * @param {Object} options - Sync options
   */
  registerChart(chartId, instance, groupId, options = {}) {
    const opts = {
      syncCursor: true,
      syncViewport: true,
      syncAxis: 'y',  // 'x', 'y', or 'both'
      ...options
    };
    
    // Store chart
    this.charts.set(chartId, {
      id: chartId,
      instance,
      groupId,
      options: opts
    });
    
    // Add to group
    if (!this.groups.has(groupId)) {
      this.groups.set(groupId, {
        id: groupId,
        chartIds: new Set(),
        options: opts
      });
    }
    this.groups.get(groupId).chartIds.add(chartId);
    
    // Connect for cursor sync
    if (opts.syncCursor) {
      const groupCharts = this.getGroupCharts(groupId);
      echarts.connect(groupCharts);
    }
    
    // Setup viewport sync
    if (opts.syncViewport) {
      this.setupViewportSync(chartId, instance, groupId, opts);
    }
  }
  
  /**
   * Get all chart instances in a group
   */
  getGroupCharts(groupId) {
    const group = this.groups.get(groupId);
    if (!group) return [];
    
    return Array.from(group.chartIds)
      .map(id => this.charts.get(id)?.instance)
      .filter(Boolean);
  }
  
  /**
   * Setup viewport synchronization for a chart
   */
  setupViewportSync(chartId, instance, groupId, options) {
    instance.on('datazoom', () => {
      if (this.isUpdating) return;
      this.syncViewport(chartId, instance, groupId, options);
    });
  }
  
  /**
   * Synchronize viewport across charts in a group
   */
  syncViewport(sourceChartId, sourceInstance, groupId, options) {
    const option = sourceInstance.getOption();
    const dataZoom = option.dataZoom || [];
    
    // Find zoom state for the axis we're syncing
    const xZoom = dataZoom.find((dz, i) => {
      return dz.xAxisIndex === 0 || (i === 0 && !dz.yAxisIndex);
    });
    const yZoom = dataZoom.find((dz, i) => {
      return dz.yAxisIndex === 0 || (i === 1 && !dz.xAxisIndex);
    });
    
    this.isUpdating = true;
    try {
      const group = this.groups.get(groupId);
      if (!group) return;
      
      // Update all other charts in the group
      for (const chartId of group.chartIds) {
        if (chartId === sourceChartId) continue;
        
        const chart = this.charts.get(chartId);
        if (!chart) continue;
        
        const syncOption = { dataZoom: [] };
        const updates = [];
        
        // Sync X-axis if configured
        if ((options.syncAxis === 'x' || options.syncAxis === 'both') && xZoom) {
          updates.push({
            xAxisIndex: 0,
            start: xZoom.start,
            end: xZoom.end,
            startValue: xZoom.startValue,
            endValue: xZoom.endValue
          });
        }
        
        // Sync Y-axis if configured
        if ((options.syncAxis === 'y' || options.syncAxis === 'both') && yZoom) {
          updates.push({
            yAxisIndex: 0,
            start: yZoom.start,
            end: yZoom.end,
            startValue: yZoom.startValue,
            endValue: yZoom.endValue
          });
        }
        
        if (updates.length > 0) {
          syncOption.dataZoom = updates;
          chart.instance.setOption(syncOption);
        }
      }
    } finally {
      this.isUpdating = false;
    }
  }
  
  /**
   * Unregister a chart
   */
  unregisterChart(chartId) {
    const registration = this.charts.get(chartId);
    if (!registration) return;
    
    const { groupId } = registration;
    const group = this.groups.get(groupId);
    
    if (group) {
      group.chartIds.delete(chartId);
      if (group.chartIds.size === 0) {
        this.groups.delete(groupId);
        echarts.disconnect(groupId);
      }
    }
    
    this.charts.delete(chartId);
  }
}

// Usage
const syncManager = new ChartSyncManager();

var chart1 = echarts.init(document.getElementById('chart1'));
var chart2 = echarts.init(document.getElementById('chart2'));
var chart3 = echarts.init(document.getElementById('chart3'));

// Register charts in the same group
syncManager.registerChart('chart1', chart1, 'well-log-group', {
  syncAxis: 'y',  // Sync Y-axis (depth) only
  syncCursor: true,
  syncViewport: true
});

syncManager.registerChart('chart2', chart2, 'well-log-group', {
  syncAxis: 'y',
  syncCursor: true,
  syncViewport: true
});

syncManager.registerChart('chart3', chart3, 'well-log-group', {
  syncAxis: 'y',
  syncCursor: true,
  syncViewport: true
});
```

## Complete Example: Well Log Dashboard

A complete example showing multiple well log tracks synchronized by depth:

```javascript
// Well log data for different measurements
const resistivityData = [[0, 10], [100, 15], [200, 12], [300, 18]];
const densityData = [[0, 2.5], [100, 2.6], [200, 2.4], [300, 2.7]];
const porosityData = [[0, 0.15], [100, 0.18], [200, 0.12], [300, 0.20]];

// Chart manager
const syncManager = new ChartSyncManager();

// Create three chart instances
const chart1 = echarts.init(document.getElementById('resistivity-chart'));
const chart2 = echarts.init(document.getElementById('density-chart'));
const chart3 = echarts.init(document.getElementById('porosity-chart'));

// Common depth axis configuration
const depthAxisConfig = {
  type: 'value',
  name: 'Depth (m)',
  inverse: true,  // Depth increases downward
  min: 0,
  max: 300,
  position: 'left'
};

// Chart 1: Resistivity
chart1.setOption({
  xAxis: {
    type: 'value',
    name: 'Resistivity (Ω·m)',
    position: 'top'
  },
  yAxis: depthAxisConfig,
  dataZoom: [{
    type: 'inside',
    yAxisIndex: 0,  // Only depth axis zoom
    filterMode: 'none'
  }],
  series: [{
    type: 'line',
    data: resistivityData,
    lineStyle: { color: '#5470c6' }
  }]
});

// Chart 2: Density
chart2.setOption({
  xAxis: {
    type: 'value',
    name: 'Density (g/cm³)',
    position: 'top'
  },
  yAxis: depthAxisConfig,
  dataZoom: [{
    type: 'inside',
    yAxisIndex: 0,
    filterMode: 'none'
  }],
  series: [{
    type: 'line',
    data: densityData,
    lineStyle: { color: '#91cc75' }
  }]
});

// Chart 3: Porosity
chart3.setOption({
  xAxis: {
    type: 'value',
    name: 'Porosity',
    position: 'top'
  },
  yAxis: depthAxisConfig,
  dataZoom: [{
    type: 'inside',
    yAxisIndex: 0,
    filterMode: 'none'
  }],
  series: [{
    type: 'line',
    data: porosityData,
    lineStyle: { color: '#fac858' }
  }]
});

// Register all charts for synchronization
syncManager.registerChart('resistivity', chart1, 'well-log-group', {
  syncAxis: 'y',  // Sync depth axis only
  syncCursor: true,
  syncViewport: true
});

syncManager.registerChart('density', chart2, 'well-log-group', {
  syncAxis: 'y',
  syncCursor: true,
  syncViewport: true
});

syncManager.registerChart('porosity', chart3, 'well-log-group', {
  syncAxis: 'y',
  syncCursor: true,
  syncViewport: true
});
```

## Key Points and Best Practices

### 1. Prevent Recursive Updates

Always use a flag to prevent infinite update loops:

```javascript
var isUpdating = false;

chart.on('datazoom', () => {
  if (isUpdating) return;
  isUpdating = true;
  // ... sync logic
  isUpdating = false;
});
```

### 2. Use `filterMode: 'none'` for Viewport Sync

When syncing viewports, use `filterMode: 'none'` to avoid data filtering issues:

```javascript
dataZoom: [{
  type: 'inside',
  xAxisIndex: 0,
  filterMode: 'none'  // Don't filter data, just change viewport
}]
```

### 3. Sync Only Necessary Axes

For well logs, typically sync only the depth axis (Y-axis), allowing each chart's value axis (X-axis) to remain independent.

### 4. Combine `connect()` with Manual Sync

Use `echarts.connect()` for cursor/tooltip sync and manual `datazoom` event handling for viewport sync:

```javascript
// Cursor sync (automatic)
echarts.connect([chart1, chart2, chart3]);

// Viewport sync (manual)
chart1.on('datazoom', () => {
  // ... manual sync logic
});
```

### 5. Use Group IDs for Scalability

When managing many charts, use group IDs for better organization:

```javascript
const chart1 = echarts.init(dom1, null, { group: 'well-log-group' });
const chart2 = echarts.init(dom2, null, { group: 'well-log-group' });
echarts.connect('well-log-group');
```

## Common Issues and Solutions

### Issue 1: Charts Not Syncing

**Problem**: Charts don't update when zooming on another chart.

**Solution**: Ensure you're listening to `datazoom` events and updating other charts' `dataZoom` configuration:

```javascript
chart.on('datazoom', () => {
  const option = chart.getOption();
  const dataZoom = option.dataZoom;
  // Update other charts with the same dataZoom values
});
```

### Issue 2: Infinite Update Loop

**Problem**: Charts keep updating each other in a loop.

**Solution**: Use an `isUpdating` flag to prevent recursive updates:

```javascript
var isUpdating = false;
chart.on('datazoom', () => {
  if (isUpdating) return;
  isUpdating = true;
  // ... sync logic
  isUpdating = false;
});
```

### Issue 3: Only One Axis Syncing

**Problem**: Only X-axis or Y-axis is syncing, not both.

**Solution**: Check your `dataZoom` configuration and ensure you're finding and syncing the correct axis:

```javascript
// Find X-axis zoom
const xZoom = dataZoom.find(dz => dz.xAxisIndex === 0);

// Find Y-axis zoom
const yZoom = dataZoom.find(dz => dz.yAxisIndex === 0);

// Sync both if needed
if (xZoom) {
  // Sync X-axis
}
if (yZoom) {
  // Sync Y-axis
}
```

### Issue 4: Cursor Not Syncing

**Problem**: Viewport syncs but cursor/tooltip doesn't.

**Solution**: Use `echarts.connect()` for cursor synchronization:

```javascript
echarts.connect([chart1, chart2, chart3]);
```

## Summary

To sync multiple Apache ECharts charts to a common axis:

1. **Use `echarts.connect()`** for cursor/tooltip synchronization
2. **Listen to `datazoom` events** for viewport synchronization
3. **Manually update `dataZoom` configuration** on other charts when one chart zooms
4. **Use a flag** to prevent recursive updates
5. **Specify which axis to sync** (x, y, or both) based on your use case
6. **Use a chart manager pattern** for production applications with many charts

The key is combining ECharts' built-in `connect()` method for cursor sync with manual `datazoom` event handling for precise viewport synchronization.

