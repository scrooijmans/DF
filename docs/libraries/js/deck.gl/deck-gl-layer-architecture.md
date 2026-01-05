# deck.gl Layer Architecture: Deep Dive

## Overview

deck.gl is a WebGL-powered framework for high-performance visualization of large-scale data. Its architecture is built around a **reactive layer system** that efficiently manages GPU resources, data transformations, and rendering operations. The core innovation lies in how layers abstract away the complexity of WebGL programming while maintaining maximum performance through intelligent change detection, GPU buffer management, and optimized rendering pipelines.

## Layer Architecture Fundamentals

### The Layer Base Class

All deck.gl layers inherit from a base `Layer` class that provides:

- **Lifecycle management**: Standardized methods for initialization, updates, and rendering
- **State management**: Internal state object that persists across renders
- **Property diffing**: Automatic detection of prop changes using shallow comparison
- **GPU resource management**: Integration with AttributeManager for buffer handling
- **Rendering abstraction**: Default rendering via luma.gl `Model` instances

### Reactive Architecture

deck.gl follows a **reactive programming model** where:

1. **Layers are descriptors**: Each render cycle creates new layer instances, but these are lightweight descriptor objects
2. **Efficient matching**: deck.gl matches new layers against existing ones by ID, reusing layers when possible
3. **Change detection**: Only layers with actual prop changes trigger expensive GPU buffer updates
4. **Optimized diffing**: Layer matching uses O(N) map-based lookups instead of O(N²) linear searches

This design allows applications to create new layer instances on every render without performance penalties, as deck.gl intelligently reuses unchanged layers and their GPU resources.

## Data Management

### Data Flow Architecture

Layers receive data through the `data` prop, which can be:

- **Iterable arrays**: Standard JavaScript arrays or array-like objects
- **Typed arrays**: Direct binary data (Float32Array, Uint8Array, etc.)
- **GPU buffers**: Pre-allocated luma.gl Buffer instances
- **URLs**: Automatic loading of remote data (JSON, CSV, etc.)
- **Async iterables**: Incremental data loading (v7.2+)

### Accessor Pattern

Instead of requiring pre-formatted data, deck.gl uses **accessor functions** that extract attributes from raw data:

```javascript
new ScatterplotLayer({
  data: myData, // Raw data objects
  getPosition: (d) => [d.lon, d.lat, d.alt], // Accessor extracts position
  getColor: (d) => [d.r, d.g, d.b], // Accessor extracts color
  getRadius: 5, // Constant value
});
```

This pattern provides:

- **Flexibility**: Works with any data structure
- **Performance**: Accessors are only called during buffer updates, not every frame
- **Separation of concerns**: Data format is independent of layer implementation

### Change Detection and Update Triggers

deck.gl uses **shallow comparison** to detect data changes:

- **Shallow comparison**: Only checks if the `data` reference changed, not deep equality
- **Performance optimization**: Avoids expensive deep comparisons on large datasets
- **Update triggers**: Fine-grained control over which attributes update when data changes

When `data` changes, all GPU buffers are recalculated. To optimize, use `updateTriggers`:

```javascript
new ScatterplotLayer({
  data: myData,
  getPosition: (d) => [d.lon, d.lat],
  getColor: (d) => calculateColor(d),
  updateTriggers: {
    getColor: ['colorScheme'], // Only recalculate colors when colorScheme changes
  },
});
```

This allows selective buffer updates when only specific attributes change, avoiding full buffer regeneration.

### Incremental Data Loading

For large datasets, deck.gl supports incremental loading:

- **Separate layers per chunk**: Create a new layer for each data chunk with unique IDs
- **Selective updates**: Only new chunks trigger buffer generation; existing layers remain unchanged
- **Async iterables**: Automatic handling of streaming data

```javascript
// Efficient incremental loading
const layers = dataChunks.map(
  (chunk, index) =>
    new ScatterplotLayer({
      id: `points-${index}`, // Unique ID for matching
      data: chunk,
      getPosition: (d) => d.position,
    })
);
```

## GPU Buffer Management

### The AttributeManager

The `AttributeManager` class is the core component responsible for GPU buffer lifecycle management. It provides:

- **Automatic buffer creation**: Allocates GPU buffers based on attribute definitions
- **Change detection**: Tracks which attributes need updates
- **Efficient updates**: Only updates changed attributes
- **Memory management**: Handles buffer allocation and deallocation

### Attribute Definition

Attributes are defined with:

- **Size**: Number of components per vertex (e.g., 2 for 2D positions, 3 for RGB colors)
- **Type**: Data type (uint8, float32, etc.)
- **Accessor**: Function or property name to extract data
- **Update function**: Custom calculation logic (optional)

```javascript
attributeManager.add({
  positions: {
    size: 3, // x, y, z coordinates
    accessor: 'getPosition', // Accessor function name
    update: calculatePositions, // Custom update function
  },
  colors: {
    size: 4, // RGBA
    type: 'unorm8', // Normalized 8-bit unsigned
    accessor: 'getColor',
  },
});
```

### Buffer Update Lifecycle

1. **Invalidation**: Attributes are marked as needing updates

   ```javascript
   attributeManager.invalidate('positions');
   attributeManager.invalidateAll(); // Mark all as dirty
   ```

2. **Update**: Buffers are synchronized with data

   ```javascript
   attributeManager.update({
     data: myData,
     numInstances: myData.length,
     props: layerProps,
     buffers: preAllocatedBuffers, // Optional pre-allocated buffers
   });
   ```

3. **GPU Upload**: Typed arrays are uploaded to GPU memory as WebGL buffers

### Direct Buffer Supply

For maximum performance, applications can pre-create GPU buffers:

```javascript
import { Buffer } from 'luma.gl';

// Pre-allocate GPU buffer
const gpuBuffer = new Buffer(gl, {
  data: typedArrayData,
  usage: gl.STATIC_DRAW,
});

// Pass directly to layer
new MyLayer({
  attributes: {
    positions: {
      buffer: gpuBuffer,
      size: 3,
    },
  },
});
```

This bypasses deck.gl's internal buffer management, providing:

- **Maximum control**: Full control over memory allocation
- **Performance**: Avoids intermediate data transformations
- **Efficiency**: Reuse buffers across renders

### Partial Updates

deck.gl supports partial buffer updates for incremental data:

```javascript
attributeManager.invalidate('positions', {
  startRow: 0,
  endRow: 100, // Only update first 100 items
});
```

This enables efficient updates when only portions of data change.

## Rendering Lifecycle

### Layer Lifecycle Stages

The deck.gl rendering pipeline follows these stages:

#### 1. Initialization (`initializeState`)

Called once when a layer is first created:

```javascript
initializeState() {
  const {gl, attributeManager} = this.context;

  // Set up AttributeManager
  attributeManager.add({
    positions: {size: 3, accessor: 'getPosition'},
    colors: {size: 4, accessor: 'getColor'}
  });

  // Create GPU resources (Model, textures, etc.)
  this.setState({
    model: this._createModel(gl)
  });
}
```

This stage:

- Sets up attribute definitions
- Creates luma.gl Model instances
- Allocates initial GPU resources
- Initializes layer-specific state

#### 2. Change Detection (`shouldUpdateState`)

Determines if a layer needs updates:

```javascript
shouldUpdateState({props, oldProps, changeFlags}) {
  // Default: returns true if data or props changed
  // Can be overridden for custom logic
  return changeFlags.dataChanged || changeFlags.propsChanged;
}
```

Change flags include:

- `dataChanged`: Data prop reference changed
- `propsChanged`: Other props changed
- `viewportChanged`: Camera/viewport changed
- `extensionsChanged`: WebGL extensions changed

#### 3. State Update (`updateState`)

Called when `shouldUpdateState` returns true:

```javascript
updateState({props, oldProps, changeFlags}) {
  // Default implementation invalidates all attributes if data changed
  if (changeFlags.dataChanged) {
    this.state.attributeManager.invalidateAll();
  }

  // Update AttributeManager buffers
  this.state.attributeManager.update({
    data: props.data,
    numInstances: props.data.length,
    props: props
  });

  // Update layer-specific state
  if (changeFlags.viewportChanged) {
    this._updateViewportDependentState();
  }
}
```

This stage:

- Invalidates dirty attributes
- Updates GPU buffers via AttributeManager
- Updates layer-specific state
- Handles transitions and animations

#### 4. Rendering (`draw`)

Called every frame to render the layer:

```javascript
draw({moduleParameters, uniforms, context}) {
  const {model} = this.state;

  // Default: draws the Model with viewport uniforms
  model.draw({
    uniforms: {
      ...uniforms,  // Viewport uniforms (projection matrices, etc.)
      ...this.getModuleParameters()  // Layer-specific uniforms
    }
  });
}
```

The `draw` method:

- Receives viewport uniforms (projection matrices, camera position)
- Can add layer-specific uniforms
- Invokes WebGL draw calls via luma.gl Model
- Supports custom rendering logic

#### 5. Picking

For interactive layers, picking renders to an off-screen buffer:

```javascript
// Picking uses the same draw method with special uniforms
draw({moduleParameters, uniforms, context}) {
  if (context.picking.isActive) {
    // Render picking colors instead of visual colors
    uniforms.picking_uActive = 1;
  }

  this.state.model.draw({uniforms});
}
```

**Color Picking Technique**:

- Layers render to off-screen framebuffer
- Each object gets a unique picking color (encoded as RGB)
- Mouse coordinates map to picking color
- Color decoded back to object index

This GPU-based approach is much faster than CPU ray-casting for large datasets.

### Composite Layers

Composite layers (like `GeoJsonLayer`) generate sub-layers:

```javascript
renderLayers() {
  const {data} = this.props;

  // Generate sub-layers based on data
  return [
    new PolygonLayer({...}),
    new LineLayer({...}),
    new PointLayer({...})
  ];
}
```

Composite layers:

- Decompose complex data into primitive layers
- Can optimize by reusing sub-layers when data hasn't changed
- Support `shouldUpdate` to prevent unnecessary sub-layer regeneration

## Performance Optimizations

### Layer Matching and Reuse

deck.gl optimizes layer reuse:

- **ID-based matching**: Layers matched by `id` prop
- **Map-based lookup**: O(N) matching instead of O(N²)
- **State preservation**: Unchanged layers keep their GPU resources
- **Selective updates**: Only changed layers update buffers

### Minimizing Buffer Regeneration

The most expensive operation is GPU buffer regeneration. Optimizations:

1. **Avoid unnecessary data changes**: Supply the same data object reference
2. **Use updateTriggers**: Update only specific attributes
3. **Pre-allocate buffers**: Supply buffers directly to avoid allocation
4. **Incremental loading**: Use separate layers for data chunks
5. **Data comparators**: Custom comparison logic for complex data structures

```javascript
new ScatterplotLayer({
  data: myData,
  dataComparator: (newData, oldData) => {
    // Custom comparison to avoid unnecessary updates
    return (
      newData.length === oldData.length &&
      newData[0].timestamp === oldData[0].timestamp
    );
  },
});
```

### Visibility Optimization

- **visible prop**: Use `visible: false` instead of conditional rendering
- **Layer filtering**: `layerFilter` function to skip layers during rendering
- **Non-visible layer updates**: Future optimization to skip attribute updates for hidden layers

### Picking Performance

- **Selective picking**: Only pickable layers render to picking buffer
- **Area-based picking**: Can render small areas around cursor for efficiency
- **Parallel picking**: Multiple layers can render to same framebuffer

## High-Performance Design Principles

### 1. GPU-First Architecture

Everything is designed for GPU efficiency:

- **Batch rendering**: All primitives of same type rendered in single draw call
- **Instanced rendering**: Same geometry rendered multiple times with different attributes
- **Minimal CPU-GPU transfers**: Buffers updated only when necessary
- **Shader-based computation**: Complex calculations done in GPU shaders

### 2. Reactive with Smart Diffing

- **Shallow comparison**: Fast change detection without deep equality checks
- **Granular updates**: Only changed attributes trigger buffer updates
- **Layer reuse**: Unchanged layers preserve GPU state

### 3. Memory Efficiency

- **Buffer reuse**: GPU buffers reused across renders when possible
- **Typed arrays**: Efficient binary data representation
- **Lazy allocation**: Resources allocated only when needed
- **Chunked data**: Large datasets split into manageable chunks

### 4. Parallel Processing

- **WebGL parallelism**: GPU processes vertices/fragments in parallel
- **Layer independence**: Layers can update/render independently
- **Async data loading**: Data loading doesn't block rendering

### 5. Developer Experience

Despite the complexity, deck.gl provides:

- **Declarative API**: Simple prop-based configuration
- **Automatic optimization**: Many optimizations happen automatically
- **Flexibility**: Can override defaults for advanced use cases
- **Debugging tools**: AttributeManager logging, layer inspection

## Example: Complete Layer Lifecycle

```javascript
// 1. Layer creation (descriptor)
const layer = new ScatterplotLayer({
  id: 'points',
  data: myData,
  getPosition: (d) => [d.lon, d.lat],
  getColor: [255, 0, 0],
  getRadius: 5,
});

// 2. First render: initializeState called
//    - AttributeManager sets up attribute definitions
//    - Model created with shaders
//    - GPU buffers allocated

// 3. updateState called
//    - AttributeManager iterates data
//    - Calls accessors (getPosition, getColor)
//    - Builds typed arrays
//    - Uploads to GPU buffers

// 4. draw called every frame
//    - Model.draw() with viewport uniforms
//    - WebGL renders to canvas

// 5. Data changes: new layer instance created
const newLayer = new ScatterplotLayer({
  id: 'points', // Same ID!
  data: newData, // Different data
  // ... same props
});

// 6. deck.gl matches by ID, reuses layer
//    - shouldUpdateState returns true (data changed)
//    - updateState called
//    - Only positions buffer updated (colors unchanged)
//    - Model reused (no re-creation)

// 7. Picking: user hovers over canvas
//    - draw called with picking uniforms
//    - Renders to off-screen buffer
//    - Picking color decoded to object index
```

## Conclusion

deck.gl's layer architecture achieves high-performance interactive visualization through:

1. **Intelligent resource management**: AttributeManager handles GPU buffer lifecycle automatically
2. **Efficient change detection**: Shallow comparison and granular updates minimize work
3. **GPU-optimized rendering**: Batch rendering, instancing, and shader-based computation
4. **Reactive architecture**: Layer reuse and matching prevent unnecessary updates
5. **Flexible data handling**: Accessor pattern works with any data format
6. **Developer-friendly API**: Complex optimizations hidden behind simple declarative interface

This architecture enables deck.gl to handle millions of data points with smooth 60fps rendering, making it suitable for real-time geospatial visualization, data exploration, and interactive analytics applications.

