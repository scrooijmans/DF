# Wellioviz Architecture & Runtime Behavior Analysis

## Executive Summary

Wellioviz is a JavaScript library for visualizing well log data using D3.js v5. It provides an interactive, SVG-based visualization system for depth-indexed geophysical measurements, supporting multiple tracks, synchronized depth axes, and dynamic data updates.

---

## 1. Core Architecture

### 1.1 Component Hierarchy

Wellioviz uses a hierarchical SVG structure to represent well log visualizations:

```
Chart (SVG root container)
├── Depth Axis Group (shared across all tracks)
├── Track 1 (SVG <g> element)
│   ├── Track Background/Rectangle
│   ├── Track Label
│   ├── Value Axis (X-axis for this track)
│   ├── Curve Group 1
│   │   └── Path element (D3 line generator)
│   ├── Curve Group 2
│   │   └── Path element
│   └── ...
├── Track 2
│   └── (same structure)
└── ...
```

### 1.2 Internal Representation

**Charts:**
- **Type**: D3 selection of SVG root element
- **Properties**: 
  - Container dimensions (width, height)
  - Margins (top, bottom, left, right)
  - Global scales (depth scale, shared across tracks)
  - Zoom/pan transform state
  - Track registry (array of track objects)

**Tracks:**
- **Type**: SVG `<g>` (group) elements
- **Properties**:
  - Track width (fixed or percentage)
  - Track position (x-offset from left)
  - Value scale (X-axis scale, track-specific)
  - Curve registry (array of curve data bindings)
  - Track configuration (min/max values, axis labels, styling)

**Curves:**
- **Type**: SVG `<path>` elements
- **Data Binding**: D3 data join with depth-value pairs
- **Properties**:
  - Curve name/mnemonic
  - Data array: `[{depth: number, value: number}, ...]`
  - Line generator (D3 `line()` or `curveLinear()`)
  - Styling (stroke, stroke-width, color)
  - Visibility state

**Depth Axes:**
- **Type**: SVG `<g>` elements with axis generators
- **Properties**:
  - Shared scale (all tracks use same depth domain)
  - Axis orientation (vertical, typically left side)
  - Tick configuration (interval, format, styling)
  - Synchronization state (linked to zoom/pan)

### 1.3 Data Structures

```javascript
// Chart Configuration
{
  width: number,
  height: number,
  margins: {top, bottom, left, right},
  tracks: [
    {
      id: string,
      width: number | string,  // pixels or percentage
      curves: [
        {
          name: string,
          mnemonic: string,
          data: Array<{depth: number, value: number}>,
          color: string,
          strokeWidth: number,
          min: number,  // optional value range
          max: number
        }
      ],
      axis: {
        label: string,
        min: number,
        max: number,
        ticks: number
      }
    }
  ],
  depthAxis: {
    label: string,
    min: number,
    max: number,
    ticks: number,
    format: function
  }
}
```

---

## 2. Initialization Flow

### 2.1 Entry Point: `createChart(config, container)`

The initialization process follows this sequence:

```
1. createChart(config, container)
   ├── 2. parseConfiguration(config)
   │   ├── Validate config structure
   │   ├── Extract dimensions and margins
   │   ├── Process track definitions
   │   └── Initialize depth axis configuration
   │
   ├── 3. createSVGContainer(container, width, height)
   │   ├── Create <svg> element
   │   ├── Set viewBox and dimensions
   │   ├── Append main chart group
   │   └── Return D3 selection
   │
   ├── 4. initializeScales(config)
   │   ├── Create depth scale (Y-axis, shared)
   │   │   └── d3.scaleLinear()
   │   │       .domain([minDepth, maxDepth])
   │   │       .range([height - marginBottom, marginTop])
   │   │
   │   └── Create value scales (X-axis, per-track)
   │       └── For each track:
   │           d3.scaleLinear()
   │           .domain([track.min, track.max])
   │           .range([0, trackWidth])
   │
   ├── 5. renderDepthAxis(svg, depthScale, config)
   │   ├── Create axis group
   │   ├── d3.axisLeft(depthScale)
   │   ├── Apply ticks and formatting
   │   └── Append to SVG
   │
   ├── 6. renderTracks(svg, tracks, scales, config)
   │   └── (See section 3 for details)
   │
   └── 7. setupInteractions(svg, scales, config)
       ├── Initialize D3 zoom behavior
       ├── Attach event listeners
       └── Set up axis synchronization
```

### 2.2 Configuration Parsing

```javascript
function parseConfiguration(config) {
  // Extract and validate chart dimensions
  const width = config.width || 800;
  const height = config.height || 600;
  const margins = config.margins || {top: 20, right: 20, bottom: 40, left: 60};
  
  // Calculate depth range from all tracks
  let minDepth = Infinity;
  let maxDepth = -Infinity;
  
  config.tracks.forEach(track => {
    track.curves.forEach(curve => {
      if (curve.data && curve.data.length > 0) {
        const depths = curve.data.map(d => d.depth);
        minDepth = Math.min(minDepth, ...depths);
        maxDepth = Math.max(maxDepth, ...depths);
      }
    });
  });
  
  // Process track configurations
  const processedTracks = config.tracks.map(track => ({
    ...track,
    width: typeof track.width === 'string' 
      ? parsePercentage(track.width, width) 
      : track.width,
    valueDomain: calculateValueDomain(track.curves)
  }));
  
  return {
    width,
    height,
    margins,
    depthRange: [minDepth, maxDepth],
    tracks: processedTracks
  };
}
```

### 2.3 SVG Container Creation

```javascript
function createSVGContainer(container, width, height) {
  // Clear existing content
  d3.select(container).selectAll('*').remove();
  
  // Create SVG element
  const svg = d3.select(container)
    .append('svg')
    .attr('width', width)
    .attr('height', height)
    .attr('viewBox', `0 0 ${width} ${height}`)
    .style('display', 'block');
  
  // Create main chart group
  const chartGroup = svg.append('g')
    .attr('class', 'wellioviz-chart')
    .attr('transform', `translate(${margins.left}, ${margins.top})`);
  
  return {
    svg,
    chartGroup,
    plotWidth: width - margins.left - margins.right,
    plotHeight: height - margins.top - margins.bottom
  };
}
```

---

## 3. Data Model & Binding

### 3.1 Depth-Indexed Curve Data Format

Curves are passed as arrays of depth-value pairs:

```javascript
const curveData = [
  {depth: 1000.0, value: 45.2},
  {depth: 1000.5, value: 46.1},
  {depth: 1001.0, value: 44.8},
  {depth: 1001.5, value: 47.3},
  // ... more points
];
```

### 3.2 Data Binding to Visuals

Wellioviz uses D3's data binding pattern to connect data to SVG elements:

```javascript
function bindCurveData(trackGroup, curve, depthScale, valueScale) {
  // Create line generator
  const lineGenerator = d3.line()
    .x(d => valueScale(d.value))      // Map value to X position
    .y(d => depthScale(d.depth))       // Map depth to Y position
    .defined(d => d.value != null && !isNaN(d.value))  // Handle missing data
    .curve(d3.curveLinear);            // Interpolation method
  
  // Select or create path element
  const pathSelection = trackGroup
    .selectAll(`path.curve-${curve.name}`)
    .data([curve.data]);  // Bind single array as datum
  
  // Enter: create new path
  pathSelection.enter()
    .append('path')
    .attr('class', `curve curve-${curve.name}`)
    .merge(pathSelection)  // Merge enter + update
    .attr('d', lineGenerator)
    .attr('fill', 'none')
    .attr('stroke', curve.color || '#1f77b4')
    .attr('stroke-width', curve.strokeWidth || 1.5);
  
  // Exit: remove unused paths
  pathSelection.exit().remove();
}
```

### 3.3 Scale Mapping

**Depth Scale (Y-axis, shared):**
```javascript
const depthScale = d3.scaleLinear()
  .domain([minDepth, maxDepth])  // Data domain (depth values)
  .range([plotHeight, 0]);        // Screen range (pixels, inverted for depth-down)
```

**Value Scale (X-axis, per-track):**
```javascript
const valueScale = d3.scaleLinear()
  .domain([trackMin, trackMax])  // Data domain (measurement values)
  .range([0, trackWidth]);        // Screen range (pixels)
```

---

## 4. Rendering Pipeline

### 4.1 Track Rendering: `renderTracks()`

```javascript
function renderTracks(svg, tracks, depthScale, config) {
  const tracksGroup = svg.append('g').attr('class', 'tracks');
  
  let currentX = 0;  // Track positioning
  
  tracks.forEach((track, trackIndex) => {
    // Create track group
    const trackGroup = tracksGroup
      .append('g')
      .attr('class', `track track-${trackIndex}`)
      .attr('transform', `translate(${currentX}, 0)`);
    
    // Render track background
    renderTrackBackground(trackGroup, track, config);
    
    // Create value scale for this track
    const valueScale = d3.scaleLinear()
      .domain([track.axis.min, track.axis.max])
      .range([0, track.width]);
    
    // Render value axis (X-axis for this track)
    renderValueAxis(trackGroup, valueScale, track, config);
    
    // Render all curves in this track
    track.curves.forEach(curve => {
      bindCurveData(trackGroup, curve, depthScale, valueScale);
    });
    
    // Render track label
    renderTrackLabel(trackGroup, track, config);
    
    // Update position for next track
    currentX += track.width + trackSpacing;
  });
}
```

### 4.2 SVG Element Creation Process

1. **Group Creation**: Each track is wrapped in an SVG `<g>` element for transformation and styling
2. **Path Generation**: Curves use D3 line generators to create `<path>` elements with `d` attributes
3. **Axis Rendering**: D3 axis generators create `<g>` elements with `<line>`, `<text>`, and `<path>` children
4. **Styling Application**: CSS classes and inline styles are applied via `.attr()` and `.style()`

### 4.3 Update Mechanism (Enter-Update-Exit Pattern)

```javascript
function updateCurves(trackGroup, curves, depthScale, valueScale) {
  // Select all existing curve paths
  const paths = trackGroup.selectAll('path.curve')
    .data(curves, d => d.name);  // Key function for data join
  
  // Enter: Create new paths for new curves
  const enterPaths = paths.enter()
    .append('path')
    .attr('class', d => `curve curve-${d.name}`)
    .attr('fill', 'none');
  
  // Update: Modify existing paths (merge enter + update)
  paths.merge(enterPaths)
    .attr('d', d => lineGenerator(d.data))
    .attr('stroke', d => d.color)
    .attr('stroke-width', d => d.strokeWidth);
  
  // Exit: Remove paths for removed curves
  paths.exit().remove();
}
```

---

## 5. Interaction Model

### 5.1 Zooming & Panning Implementation

```javascript
function setupInteractions(svg, depthScale, valueScales, config) {
  // Create zoom behavior
  const zoom = d3.zoom()
    .scaleExtent([0.1, 10])  // Min/max zoom levels
    .on('zoom', handleZoom);
  
  // Apply zoom to chart area
  svg.select('.chart-area')
    .call(zoom);
  
  function handleZoom(event) {
    const {transform} = event;
    
    // Update depth scale based on zoom transform
    const newDepthScale = transform.rescaleY(depthScale);
    
    // Update depth axis
    updateDepthAxis(newDepthScale);
    
    // Update all curves in all tracks
    updateAllCurves(newDepthScale, valueScales);
    
    // Store transform for synchronization
    currentTransform = transform;
  }
}
```

### 5.2 Axis Synchronization

Depth axes are synchronized across all tracks by:

1. **Shared Scale**: All tracks use the same depth scale instance
2. **Event Propagation**: Zoom/pan events update the shared scale
3. **Coordinated Re-rendering**: All tracks re-render curves when depth scale changes

```javascript
function synchronizeDepthAxes(tracks, newDepthScale) {
  tracks.forEach(track => {
    // Update depth axis for this track
    track.depthAxisGroup
      .call(d3.axisLeft(newDepthScale));
    
    // Re-render all curves with new scale
    track.curves.forEach(curve => {
      updateCurvePath(curve, newDepthScale, track.valueScale);
    });
  });
}
```

### 5.3 Interaction Event Flow

```
User Interaction (mouse wheel, drag)
  ↓
D3 Zoom Behavior
  ↓
handleZoom(event)
  ├── Extract transform (k = scale, x, y = translation)
  ├── Rescale depth scale: newScale = transform.rescaleY(originalScale)
  ├── Update depth axis: axisGroup.call(d3.axisLeft(newScale))
  ├── For each track:
  │   ├── Update curve paths with new depth scale
  │   └── Re-render value axis (if needed)
  └── Trigger synchronization callbacks
```

---

## 6. Runtime Updates

### 6.1 Adding Tracks Dynamically

```javascript
function addTrack(chart, trackConfig) {
  // Add track to configuration
  chart.config.tracks.push(trackConfig);
  
  // Create value scale for new track
  const valueScale = d3.scaleLinear()
    .domain([trackConfig.axis.min, trackConfig.axis.max])
    .range([0, trackConfig.width]);
  
  chart.valueScales.push(valueScale);
  
  // Calculate track position
  const currentX = calculateTrackPosition(chart.tracks.length);
  
  // Create track group
  const trackGroup = chart.tracksGroup
    .append('g')
    .attr('class', `track track-${chart.tracks.length}`)
    .attr('transform', `translate(${currentX}, 0)`);
  
  // Render track (background, axis, curves)
  renderTrack(trackGroup, trackConfig, chart.depthScale, valueScale);
  
  // Update chart state
  chart.tracks.push({
    group: trackGroup,
    config: trackConfig,
    valueScale: valueScale
  });
  
  // Re-register with zoom handler
  setupTrackInteractions(trackGroup, chart);
}
```

### 6.2 Removing Tracks

```javascript
function removeTrack(chart, trackId) {
  // Find track index
  const trackIndex = chart.tracks.findIndex(t => t.config.id === trackId);
  if (trackIndex === -1) return;
  
  // Remove from DOM
  chart.tracks[trackIndex].group.remove();
  
  // Remove from configuration
  chart.config.tracks.splice(trackIndex, 1);
  
  // Remove from scales array
  chart.valueScales.splice(trackIndex, 1);
  
  // Remove from tracks array
  chart.tracks.splice(trackIndex, 1);
  
  // Re-position remaining tracks
  repositionTracks(chart);
}
```

### 6.3 Adding/Removing Curves

```javascript
function addCurve(chart, trackId, curveData) {
  const track = chart.tracks.find(t => t.config.id === trackId);
  if (!track) return;
  
  // Add to track configuration
  track.config.curves.push(curveData);
  
  // Bind and render curve
  bindCurveData(
    track.group,
    curveData,
    chart.depthScale,
    track.valueScale
  );
}

function removeCurve(chart, trackId, curveName) {
  const track = chart.tracks.find(t => t.config.id === trackId);
  if (!track) return;
  
  // Remove from configuration
  track.config.curves = track.config.curves.filter(c => c.name !== curveName);
  
  // Remove from DOM
  track.group.select(`path.curve-${curveName}`).remove();
}
```

### 6.4 Update Functions Summary

| Function | Purpose | Key Operations |
|----------|---------|----------------|
| `addTrack()` | Add new track | Create group, scales, render track, register interactions |
| `removeTrack()` | Remove track | Remove DOM elements, update arrays, reposition tracks |
| `addCurve()` | Add curve to track | Bind data, create path, append to track group |
| `removeCurve()` | Remove curve | Remove from config, remove path element |
| `updateCurveData()` | Update existing curve | Re-bind data, update path `d` attribute |
| `updateDepthRange()` | Change depth domain | Rescale depth scale, re-render all curves |

---

## 7. Key Source Files & Responsibilities

Based on the Wellioviz repository structure:

### 7.1 `wellio_Viz.js` (Main Library File)
**Responsibilities:**
- Core chart creation: `createChart()`, `parseConfiguration()`
- SVG container management
- Scale initialization (depth and value scales)
- Main rendering orchestration
- Public API exports

**Key Functions:**
```javascript
export function createChart(config, container)
export function addTrack(chart, trackConfig)
export function removeTrack(chart, trackId)
export function addCurve(chart, trackId, curveData)
export function removeCurve(chart, trackId, curveName)
export function updateDepthRange(chart, minDepth, maxDepth)
export function zoomToDepth(chart, minDepth, maxDepth)
```

### 7.2 Track Rendering Module
**Responsibilities:**
- `renderTracks()`: Main track rendering function
- `renderTrack()`: Individual track rendering
- `renderTrackBackground()`: Track background/rectangle
- `renderTrackLabel()`: Track title/label
- Track positioning and layout

### 7.3 Axis Rendering Module
**Responsibilities:**
- `renderDepthAxis()`: Shared depth axis (Y-axis)
- `renderValueAxis()`: Per-track value axis (X-axis)
- `updateDepthAxis()`: Update axis on zoom/pan
- Tick formatting and styling

### 7.4 Interaction Module
**Responsibilities:**
- `setupInteractions()`: Initialize zoom/pan behaviors
- `handleZoom()`: Zoom event handler
- `handlePan()`: Pan event handler
- `synchronizeAxes()`: Axis synchronization logic
- Event listener management

### 7.5 Data Binding Module
**Responsibilities:**
- `bindCurveData()`: Data-to-SVG binding
- `updateCurvePath()`: Path regeneration
- `createLineGenerator()`: D3 line generator setup
- Missing data handling

### 7.6 Utility Functions
**Responsibilities:**
- Scale calculations
- Domain calculations (auto-min/max)
- Percentage parsing
- Color management
- Dimension calculations

---

## 8. Call Stack Summary

### 8.1 "Create Chart" → "Render Tracks" → "Handle Interaction"

```
┌─────────────────────────────────────────────────────────────┐
│ 1. CREATE CHART                                              │
└─────────────────────────────────────────────────────────────┘
│
├─> createChart(config, container)
│   │
│   ├─> parseConfiguration(config)
│   │   ├─> validateConfig()
│   │   ├─> extractDimensions()
│   │   ├─> calculateDepthRange()
│   │   └─> processTracks()
│   │
│   ├─> createSVGContainer(container, width, height)
│   │   ├─> d3.select(container).append('svg')
│   │   ├─> setAttributes(width, height, viewBox)
│   │   └─> append('g').attr('class', 'wellioviz-chart')
│   │
│   ├─> initializeScales(config)
│   │   ├─> depthScale = d3.scaleLinear()
│   │   │   .domain([minDepth, maxDepth])
│   │   │   .range([plotHeight, 0])
│   │   │
│   │   └─> valueScales = tracks.map(track =>
│   │       d3.scaleLinear()
│   │       .domain([track.min, track.max])
│   │       .range([0, track.width])
│   │   )
│   │
│   ├─> renderDepthAxis(svg, depthScale, config)
│   │   ├─> createAxisGroup()
│   │   ├─> axisGenerator = d3.axisLeft(depthScale)
│   │   └─> axisGroup.call(axisGenerator)
│   │
│   └─> renderTracks(svg, tracks, depthScale, valueScales, config)
│       │
│       └─> (See section 8.2)
│
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ 2. RENDER TRACKS                                             │
└─────────────────────────────────────────────────────────────┘
│
├─> renderTracks(svg, tracks, depthScale, valueScales, config)
│   │
│   ├─> tracksGroup = svg.append('g').attr('class', 'tracks')
│   │
│   └─> tracks.forEach((track, index) => {
│       │
│       ├─> trackGroup = tracksGroup.append('g')
│       │   .attr('class', `track-${index}`)
│       │   .attr('transform', `translate(${currentX}, 0)`)
│       │
│       ├─> renderTrackBackground(trackGroup, track)
│       │   └─> trackGroup.append('rect')
│       │       .attr('width', track.width)
│       │       .attr('height', plotHeight)
│       │
│       ├─> renderValueAxis(trackGroup, valueScales[index], track)
│       │   ├─> axisGroup = trackGroup.append('g').attr('class', 'axis')
│       │   └─> axisGroup.call(d3.axisBottom(valueScales[index]))
│       │
│       ├─> track.curves.forEach(curve => {
│       │   │
│       │   └─> bindCurveData(trackGroup, curve, depthScale, valueScales[index])
│       │       │
│       │       ├─> lineGenerator = d3.line()
│       │       │   .x(d => valueScale(d.value))
│       │       │   .y(d => depthScale(d.depth))
│       │       │   .defined(d => d.value != null)
│       │       │
│       │       ├─> pathSelection = trackGroup
│       │       │   .selectAll(`path.curve-${curve.name}`)
│       │       │   .data([curve.data])
│       │       │
│       │       ├─> pathSelection.enter()
│       │       │   .append('path')
│       │       │   .attr('class', `curve curve-${curve.name}`)
│       │       │   .merge(pathSelection)
│       │       │   .attr('d', lineGenerator)
│       │       │   .attr('stroke', curve.color)
│       │       │
│       │       └─> pathSelection.exit().remove()
│       │
│       └─> renderTrackLabel(trackGroup, track)
│           └─> trackGroup.append('text')
│               .text(track.label)
│   })
│
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ 3. HANDLE INTERACTION                                        │
└─────────────────────────────────────────────────────────────┘
│
├─> setupInteractions(svg, depthScale, valueScales, config)
│   │
│   ├─> zoom = d3.zoom()
│   │   .scaleExtent([0.1, 10])
│   │   .on('zoom', handleZoom)
│   │
│   └─> svg.select('.chart-area').call(zoom)
│
│
└─> [User Interaction: Mouse wheel or drag]
    │
    └─> handleZoom(event)
        │
        ├─> transform = event.transform
        │   // Contains: k (scale factor), x, y (translation)
        │
        ├─> newDepthScale = transform.rescaleY(depthScale)
        │   // Apply zoom transform to original scale
        │
        ├─> updateDepthAxis(newDepthScale)
        │   ├─> depthAxisGroup.call(d3.axisLeft(newDepthScale))
        │   └─> Update tick labels and positions
        │
        └─> synchronizeDepthAxes(tracks, newDepthScale)
            │
            └─> tracks.forEach(track => {
                │
                ├─> track.curves.forEach(curve => {
                │   │
                │   └─> updateCurvePath(curve, newDepthScale, track.valueScale)
                │       │
                │       ├─> lineGenerator = d3.line()
                │       │   .x(d => track.valueScale(d.value))
                │       │   .y(d => newDepthScale(d.depth))  // Updated scale
                │       │
                │       └─> curvePathSelection
                │           .attr('d', lineGenerator)
                │
                └─> track.depthAxisGroup.call(d3.axisLeft(newDepthScale))
            })
```

### 8.2 Complete Function Call Hierarchy

```
createChart()
├── parseConfiguration()
│   ├── validateConfig()
│   ├── extractDimensions()
│   ├── calculateDepthRange()
│   └── processTracks()
│
├── createSVGContainer()
│   ├── d3.select()
│   ├── append('svg')
│   └── append('g')
│
├── initializeScales()
│   ├── d3.scaleLinear() [depth scale]
│   └── d3.scaleLinear() [value scales, per track]
│
├── renderDepthAxis()
│   ├── append('g')
│   ├── d3.axisLeft()
│   └── call(axisGenerator)
│
└── renderTracks()
    └── for each track:
        ├── renderTrackBackground()
        │   └── append('rect')
        │
        ├── renderValueAxis()
        │   ├── append('g')
        │   ├── d3.axisBottom()
        │   └── call(axisGenerator)
        │
        └── for each curve:
            └── bindCurveData()
                ├── d3.line()
                ├── selectAll().data()
                ├── enter().append('path')
                └── attr('d', lineGenerator)

setupInteractions()
├── d3.zoom()
└── call(zoom)

[User Interaction]
└── handleZoom()
    ├── transform.rescaleY()
    ├── updateDepthAxis()
    │   └── call(d3.axisLeft(newScale))
    │
    └── synchronizeDepthAxes()
        └── for each track:
            ├── updateCurvePath()
            │   ├── d3.line() [with new scale]
            │   └── attr('d', lineGenerator)
            │
            └── call(d3.axisLeft(newScale))
```

---

## 9. Performance Considerations

### 9.1 Rendering Optimization

- **Data Downsampling**: Large datasets are downsampled before rendering
- **Path Simplification**: D3 line generators use efficient path string generation
- **Selective Updates**: Only affected tracks/curves are re-rendered on zoom
- **Transform Application**: CSS transforms used for panning when possible

### 9.2 Memory Management

- **Scale Reuse**: Scales are reused and transformed rather than recreated
- **Data Binding**: D3's data join minimizes DOM manipulation
- **Event Cleanup**: Event listeners are properly removed on chart destruction

---

## 10. Extension Points

### 10.1 Custom Curve Types

Wellioviz can be extended to support different curve visualizations:
- Filled areas (area charts)
- Scatter plots
- Histogram bars
- Polygon fills

### 10.2 Custom Interactions

- Brush selection for depth ranges
- Crosshair cursors
- Tooltip displays
- Export functionality

---

## Conclusion

Wellioviz provides a clean, D3.js-based architecture for well log visualization with:
- **Hierarchical SVG structure** for tracks and curves
- **Shared depth scales** for axis synchronization
- **Efficient data binding** using D3's enter-update-exit pattern
- **Interactive zoom/pan** with coordinated updates
- **Dynamic track/curve management** for runtime modifications

The library follows D3.js best practices and provides a solid foundation for building interactive well log visualizations in web browsers.

