# deck.gl View State Management: User Interactions and Rendering Efficiency

## Overview

deck.gl's view state management system provides a sophisticated architecture for handling camera controls (zoom, pan, pitch, bearing), user interactions, and efficient rendering. The system is designed to maintain smooth 60fps performance even during continuous user interactions by intelligently detecting changes, minimizing CPU work, and leveraging GPU capabilities for viewport transformations.

## View State Architecture

### View State Object

The view state is a plain JavaScript object that describes the camera's current position and orientation:

```javascript
const viewState = {
  // Geospatial position (for MapView)
  longitude: -122.4,
  latitude: 37.8,
  zoom: 12,
  pitch: 0, // Tilt angle in degrees (0-60)
  bearing: 0, // Rotation angle in degrees (0-360)

  // Alternative: Generic 3D view (for OrthographicView, FirstPersonView, etc.)
  target: [0, 0, 0],
  rotationX: 0,
  rotationY: 0,
  fov: 75,
};
```

**Key Properties:**

- **longitude/latitude**: Center point of the map (geospatial views)
- **zoom**: Zoom level (higher = more zoomed in)
- **pitch**: Camera tilt angle (0 = top-down, 60 = tilted)
- **bearing**: Map rotation (0 = north up, 90 = east up)
- **target**: 3D target point for non-geospatial views
- **rotationX/Y**: Camera rotation angles for 3D views

### View, View State, and Viewport Relationship

deck.gl uses a three-tier architecture:

1. **View**: Defines the projection type and viewport layout

   - `MapView`: Web Mercator projection for maps
   - `OrthographicView`: 2D orthographic projection
   - `FirstPersonView`: First-person 3D camera
   - `OrbitView`: Orbit controller for 3D scenes

2. **View State**: Runtime parameters (camera position, zoom, etc.)

   - Describes the current viewpoint
   - Updated by controllers or programmatically
   - Can be constrained or validated

3. **Viewport**: Computed projection matrices and coordinate transformations
   - Created from View + View State
   - Provides projection/unprojection methods
   - Supplies uniforms to shaders

```javascript
// View defines the projection type
const view = new MapView({ id: 'map' });

// View state describes current camera
const viewState = {
  longitude: -122.4,
  latitude: 37.8,
  zoom: 12,
};

// Viewport is computed from View + View State
// (created internally by deck.gl)
const viewport = view.makeViewport({
  width: 800,
  height: 600,
  viewState,
});
```

## User Interaction System

### Controller Architecture

Controllers translate user input (mouse, touch, keyboard) into view state changes. The architecture has three layers:

#### 1. DOM Event Handling (EventManager)

The lowest level handles raw browser events:

- **Event registration**: Wires up DOM events (mouse, touch, keyboard)
- **Touch normalization**: Converts touch events to unified gesture callbacks
- **Gesture recognition**: Detects pan, zoom, rotate gestures
- **Platform abstraction**: Handles browser/platform differences
- **Scroll wheel detection**: Distinguishes touchpad vs. mouse wheel

```javascript
// EventManager normalizes events
eventManager.on('panstart', handlePanStart);
eventManager.on('panmove', handlePanMove);
eventManager.on('panend', handlePanEnd);
eventManager.on('wheel', handleWheel);
```

#### 2. Viewport Event Handling (Controllers)

Controllers interpret events and update view state:

- **MapController**: Pan, zoom, rotate for map views
- **OrbitController**: Rotate around target point
- **FirstPersonController**: First-person navigation
- **Controller State**: Maintains interaction state (drag start position, etc.)

```javascript
// Controller updates view state based on events
class MapController {
  handlePan(event) {
    const { deltaX, deltaY } = event;
    // Calculate new longitude/latitude
    const newViewState = {
      ...currentViewState,
      longitude: currentViewState.longitude + deltaLng,
      latitude: currentViewState.latitude + deltaLat,
    };
    onViewStateChange({ viewState: newViewState });
  }

  handleZoom(event) {
    const { scale } = event;
    const newViewState = {
      ...currentViewState,
      zoom: currentViewState.zoom + Math.log2(scale),
    };
    onViewStateChange({ viewState: newViewState });
  }
}
```

#### 3. Model Event Handling (Picking)

Handles object selection and interaction:

- **Color picking**: GPU-based object identification
- **Event delegation**: Routes events to appropriate handlers
- **Hover/click detection**: Identifies objects under cursor

### Controller Configuration

Controllers can be enabled simply or configured in detail:

```javascript
// Simple: Enable default controller
const deck = new Deck({
  controller: true,  // Enables MapController for MapView
  initialViewState: {...}
});

// Advanced: Configure controller
const deck = new Deck({
  controller: {
    type: MapController,
    doubleClickZoom: true,
    dragRotate: true,
    dragPan: true,
    scrollZoom: true,
    touchZoom: true,
    touchRotate: true,
    keyboard: true,
    inertia: true,  // Momentum scrolling
    inertiaDeceleration: 0.014,
    inertiaMaxSpeed: 0.01
  },
  initialViewState: {...}
});

// Multiple controllers for multiple views
const deck = new Deck({
  views: [
    new MapView({id: 'map', height: '50%'}),
    new FirstPersonView({id: 'fpv', y: '50%', height: '50%'})
  ],
  controllers: [
    new MapController({viewId: 'map'}),
    new FirstPersonController({viewId: 'fpv'})
  ],
  viewState: {
    map: {...},
    fpv: {...}
  }
});
```

### View State Update Flow

When a user interacts, the flow is:

1. **User Input**: Mouse/touch/keyboard event
2. **EventManager**: Normalizes event to gesture
3. **Controller**: Calculates new view state
4. **onViewStateChange**: Callback fires with new state
5. **Application**: Updates view state (controlled mode) or accepts (uncontrolled mode)
6. **Viewport Recalculation**: New viewport created from view state
7. **Redraw**: Layers redraw with new viewport uniforms

```javascript
// Uncontrolled mode: deck.gl manages view state internally
const deck = new Deck({
  initialViewState: {...},
  controller: true
  // No onViewStateChange - deck.gl manages state
});

// Controlled mode: Application manages view state
const [viewState, setViewState] = useState(INITIAL_VIEW_STATE);

const deck = new Deck({
  viewState,  // Application controls state
  controller: true,
  onViewStateChange: ({viewState}) => {
    // Validate/constrain view state
    const constrained = applyConstraints(viewState);
    setViewState(constrained);
  }
});
```

## Viewport and Projection

### Viewport Class

The `Viewport` class is the bridge between view state and GPU rendering:

- **Projection matrices**: Computes view and projection matrices
- **Coordinate transformation**: Projects world → screen and screen → world
- **Uniform generation**: Creates uniforms for shaders

```javascript
// Viewport is created from view state
const viewport = new WebMercatorViewport({
  width: 800,
  height: 600,
  longitude: -122.4,
  latitude: 37.8,
  zoom: 12,
  pitch: 45,
  bearing: 30,
});

// Project world coordinates to screen
const [x, y] = viewport.project([-122.4, 37.8]);
// Returns: [400, 300] (center of screen)

// Unproject screen coordinates to world
const [lng, lat] = viewport.unproject([400, 300]);
// Returns: [-122.4, 37.8]
```

### Projection Matrices

Viewports compute three key matrices:

1. **View Matrix**: Camera position and orientation
2. **Projection Matrix**: Perspective/orthographic projection
3. **Model Matrix**: World space transformations

These are combined into uniforms passed to shaders:

```glsl
// Shader receives viewport uniforms
uniform mat4 project_uViewProjectionMatrix;
uniform mat4 project_uModelMatrix;
uniform vec3 project_uCameraPosition;
uniform vec3 project_uCommonOrigin;

// Vertex shader uses these to transform positions
vec4 position_worldspace = project_uModelMatrix * vec4(position, 1.0);
vec4 position_clipspace = project_uViewProjectionMatrix * position_worldspace;
```

### Viewport Comparison

deck.gl efficiently detects viewport changes:

```javascript
// Viewport comparison
const oldViewport = view.makeViewport({ ...oldViewState });
const newViewport = view.makeViewport({ ...newViewState });

if (!oldViewport.equals(newViewport)) {
  // Viewport changed - trigger redraw
  redraw();
}
```

The `equals` method compares:

- View and projection matrices
- Viewport dimensions
- Geospatial parameters (if applicable)

## Rendering and Redraw Triggers

### Change Detection System

deck.gl uses a sophisticated change detection system to minimize unnecessary work:

#### 1. Viewport Change Detection

When view state changes:

- New viewport is created from new view state
- Viewport is compared with previous viewport
- If different, `viewportChanged` flag is set
- Layers receive `changeFlags.viewportChanged` in `shouldUpdateState`

```javascript
// Layer receives viewport change flag
shouldUpdateState({changeFlags}) {
  // Most layers don't need to update state on viewport change
  // They automatically re-render with new viewport uniforms
  if (changeFlags.viewportChanged) {
    // Only update if layer depends on viewport (e.g., screen-space calculations)
    return this.needsViewportUpdate();
  }
  return changeFlags.dataChanged || changeFlags.propsChanged;
}
```

#### 2. Layer Update Optimization

**Key Optimization**: `shouldUpdateState` was changed in deck.gl 4.1 to NOT return `true` for viewport changes by default. This means:

- **Most layers**: Don't update state on viewport change (only re-render)
- **Viewport-dependent layers**: Must explicitly check `viewportChanged` flag
- **Result**: Viewport changes are very cheap - only GPU redraw, no CPU buffer updates

```javascript
// Default behavior: viewport changes don't trigger updateState
shouldUpdateState({changeFlags}) {
  // Returns true only for data/props changes
  // Viewport changes trigger redraw but not state update
  return changeFlags.dataChanged || changeFlags.propsChanged;
}

// Viewport-dependent layer (e.g., ScreenGridLayer)
shouldUpdateState({changeFlags}) {
  // Explicitly check viewport changes
  if (changeFlags.viewportChanged) {
    return true;  // Need to recalculate screen-space grid
  }
  return changeFlags.dataChanged || changeFlags.propsChanged;
}
```

### Rendering Loop

deck.gl uses a reactive rendering model:

#### 1. Animation Frame Loop

deck.gl sets up a `requestAnimationFrame` loop:

```javascript
// Simplified rendering loop
function renderLoop() {
  // Check if redraw is needed
  if (needsRedraw()) {
    // Update viewports from view states
    updateViewports();

    // Draw all layers
    drawLayers();
  }

  // Continue loop
  requestAnimationFrame(renderLoop);
}
```

#### 2. Redraw Conditions

A redraw is triggered when:

- **View state changes**: User interaction or programmatic update
- **Layer props change**: Data or properties modified
- **Layer list changes**: Layers added/removed
- **Transition active**: View state transition in progress
- **Animation active**: `_animate` prop is true
- **Manual redraw**: `deck.redraw()` called

```javascript
// Manual redraw
deck.redraw(); // Redraw if needed
deck.redraw(true); // Force immediate redraw

// Check if redraw needed
const needsRedraw = layerManager.needsRedraw();
```

#### 3. Efficient Redraw Strategy

deck.gl optimizes redraws:

- **Change detection**: Only redraws when something actually changed
- **Viewport comparison**: Compares viewports to detect real changes
- **Layer matching**: Reuses layers when IDs match
- **Selective updates**: Only changed layers update state

```javascript
// Viewport change flow
1. User pans map
2. Controller calculates new viewState
3. onViewStateChange fires
4. Application updates viewState prop
5. deck.gl creates new viewport
6. Compares with old viewport
7. If different:
   - Sets viewportChanged flag
   - Triggers redraw
   - Layers draw with new viewport uniforms
8. GPU transforms vertices (cheap)
9. No CPU buffer updates (expensive)
```

## View State Transitions

### Transition System

deck.gl supports smooth camera transitions for programmatic view changes:

```javascript
// Simple transition
deck.setProps({
  viewState: {
    longitude: -74.006,
    latitude: 40.7128,
    zoom: 15,
    transitionDuration: 1000, // 1 second
    transitionEasing: (t) => t * t, // Ease-in
  },
});

// Fly-to transition
import { FlyToInterpolator } from '@deck.gl/core';

deck.setProps({
  viewState: {
    longitude: -74.006,
    latitude: 40.7128,
    zoom: 15,
    transitionInterpolator: new FlyToInterpolator(),
    transitionDuration: 'auto', // Calculated from distance
    transitionEasing: (t) => t * (2 - t), // Ease-in-out
  },
});
```

### Transition Interpolators

Interpolators define how view state transitions:

1. **LinearInterpolator** (default): Linear interpolation of all properties
2. **FlyToInterpolator**: Arc-based camera movement for geospatial views
3. **Custom Interpolator**: Implement `TransitionInterpolator` for custom behavior

```javascript
// Custom interpolator example
class SphericalLinearInterpolator extends TransitionInterpolator {
  constructor({ speed = 100 } = {}) {
    super(['longitude', 'latitude']);
    this.speed = speed; // degrees per second
  }

  getDuration(startViewState, endViewState) {
    const deltaLat = Math.abs(
      startViewState.latitude - endViewState.latitude
    );
    let deltaLng = Math.abs(
      startViewState.longitude - endViewState.longitude
    );
    if (deltaLng > 180) deltaLng = 360 - deltaLng;
    return (
      (Math.max(deltaLng, deltaLat) / this.speed) * 1000
    );
  }

  interpolateProps(start, end, t) {
    return {
      longitude: lerp(start.longitude, end.longitude, t),
      latitude: lerp(start.latitude, end.latitude, t),
    };
  }
}
```

### Transition Control

Transitions can be controlled:

```javascript
deck.setProps({
  viewState: {
    ...targetViewState,
    transitionDuration: 2000,
    transitionInterruption: TRANSITION_EVENTS.BREAK, // Stop current transition
    onTransitionStart: () =>
      console.log('Transition started'),
    onTransitionEnd: () => console.log('Transition ended'),
    onTransitionInterrupt: () =>
      console.log('Transition interrupted'),
  },
});
```

**Transition Interruption Modes:**

- `BREAK`: Stop at current state, start new transition
- `SNAP_TO_END`: Jump to end, start new transition
- `IGNORE`: Ignore new view state until current transition completes

## Performance Optimizations

### 1. Viewport Change Optimization

**The Key Insight**: Viewport changes are extremely cheap because:

- **GPU handles transformation**: Projection matrices are computed on CPU but applied on GPU
- **No buffer updates**: Vertex buffers don't change, only uniforms
- **Shader-based**: All coordinate transformation happens in shaders
- **Parallel processing**: GPU transforms millions of vertices in parallel

```javascript
// Viewport change: Very cheap
// - CPU: Compute new projection matrices (~0.1ms)
// - GPU: Transform vertices with new matrices (parallel, ~16ms for 60fps)
// Total: ~16ms per frame (smooth 60fps)

// Data change: Expensive
// - CPU: Iterate data, call accessors, build buffers (~100-1000ms)
// - GPU: Upload buffers (~10-50ms)
// Total: ~100-1000ms (causes stutter)
```

### 2. Selective Layer Updates

Layers can opt into viewport-dependent updates:

```javascript
// Most layers: Ignore viewport changes
shouldUpdateState({changeFlags}) {
  // Only update on data/props changes
  return changeFlags.dataChanged || changeFlags.propsChanged;
}

// Viewport-dependent layers: Check viewport
shouldUpdateState({changeFlags}) {
  if (changeFlags.viewportChanged) {
    // Recalculate screen-space attributes
    return true;
  }
  return changeFlags.dataChanged || changeFlags.propsChanged;
}
```

### 3. Viewport Comparison

deck.gl compares viewports efficiently:

```javascript
// Viewport.equals() compares:
- View matrix (16 floats)
- Projection matrix (16 floats)
- Viewport dimensions (2 ints)
- Geospatial parameters (if applicable)

// Only triggers redraw if actually different
if (!oldViewport.equals(newViewport)) {
  redraw();
}
```

### 4. Multi-Viewport Optimization

For multiple views, deck.gl optimizes:

- **Per-view controllers**: Each view has its own controller
- **Independent viewports**: Each view computes its own viewport
- **Selective redraw**: Only changed views trigger redraw
- **Layer filtering**: Layers can filter by view

```javascript
// Multiple views with independent controllers
const deck = new Deck({
  views: [
    new MapView({id: 'overview', height: '30%'}),
    new MapView({id: 'detail', y: '30%', height: '70%'})
  ],
  controllers: [
    new MapController({viewId: 'overview'}),
    new MapController({viewId: 'detail'})
  ],
  viewState: {
    overview: {zoom: 5, ...},
    detail: {zoom: 15, ...}
  },
  layerFilter: ({layer, view}) => {
    // Filter layers per view
    if (view.id === 'overview') {
      return layer.id.startsWith('overview-');
    }
    return layer.id.startsWith('detail-');
  }
});
```

### 5. Constraint Validation

View state constraints can be applied efficiently:

```javascript
// Constrain view state
function constrainViewState(viewState) {
  return {
    ...viewState,
    longitude: clamp(
      viewState.longitude,
      bounds[0][0],
      bounds[1][0]
    ),
    latitude: clamp(
      viewState.latitude,
      bounds[0][1],
      bounds[1][1]
    ),
    zoom: clamp(viewState.zoom, minZoom, maxZoom),
    pitch: clamp(viewState.pitch, 0, 60),
    bearing: normalizeAngle(viewState.bearing),
  };
}

// Apply in onViewStateChange
onViewStateChange: ({ viewState }) => {
  const constrained = constrainViewState(viewState);
  setViewState(constrained);
};
```

## Example: Complete Interaction Flow

```javascript
// 1. Initialize deck.gl
const deck = new Deck({
  initialViewState: {
    longitude: -122.4,
    latitude: 37.8,
    zoom: 12,
    pitch: 0,
    bearing: 0
  },
  controller: true,
  layers: [new ScatterplotLayer({...})]
});

// 2. User pans map
//    - Mouse down at (100, 200)
//    - Mouse move to (150, 250)
//    - EventManager normalizes to pan gesture
//    - MapController calculates delta: (50, 50) pixels

// 3. Controller updates view state
const deltaLng = pixelsToLng(50, viewport);
const deltaLat = pixelsToLat(50, viewport);
const newViewState = {
  ...currentViewState,
  longitude: currentViewState.longitude - deltaLng,
  latitude: currentViewState.latitude + deltaLat
};

// 4. onViewStateChange fires
onViewStateChange({viewState: newViewState});

// 5. Application updates (controlled mode)
setViewState(newViewState);

// 6. deck.gl receives new viewState prop
deck.setProps({viewState: newViewState});

// 7. Viewport recalculation
const oldViewport = view.makeViewport({...oldViewState});
const newViewport = view.makeViewport({...newViewState});

// 8. Viewport comparison
if (!oldViewport.equals(newViewport)) {
  // Viewport changed
  changeFlags.viewportChanged = true;
}

// 9. Layer update check
layer.shouldUpdateState({changeFlags});
// Returns: false (most layers don't need state update)

// 10. Redraw triggered
deck.redraw();

// 11. Layer draw
layer.draw({
  moduleParameters: {...},
  uniforms: {
    ...newViewport.getUniforms(),  // New projection matrices
    ...layerUniforms
  },
  context: {...}
});

// 12. GPU rendering
//    - Shader receives new viewport uniforms
//    - Transforms vertices with new matrices
//    - Renders to canvas
//    - ~16ms (60fps)
```

## Performance Characteristics

### Viewport Change Performance

- **CPU cost**: ~0.1ms (matrix computation)
- **GPU cost**: ~16ms (vertex transformation, parallel)
- **Total**: ~16ms per frame = **60fps**
- **Buffer updates**: 0 (no CPU buffer work)

### Data Change Performance

- **CPU cost**: 100-1000ms (data iteration, accessor calls, buffer building)
- **GPU cost**: 10-50ms (buffer upload)
- **Total**: 100-1000ms = **1-10fps** (causes stutter)
- **Buffer updates**: All attributes regenerated

### Interaction Performance

During continuous panning/zooming:

- **View state updates**: 60+ per second
- **Viewport recalculations**: 60+ per second
- **Redraws**: 60 per second
- **State updates**: 0 (layers don't update state)
- **Result**: Smooth 60fps interaction

## Best Practices

### 1. Use Controlled Mode for Constraints

```javascript
// Apply constraints in onViewStateChange
onViewStateChange: ({ viewState }) => {
  const constrained = applyConstraints(viewState);
  setViewState(constrained);
};
```

### 2. Minimize Viewport-Dependent Layers

Most layers should ignore viewport changes:

```javascript
// Good: Viewport-independent
shouldUpdateState({changeFlags}) {
  return changeFlags.dataChanged || changeFlags.propsChanged;
}

// Only if necessary: Viewport-dependent
shouldUpdateState({changeFlags}) {
  if (changeFlags.viewportChanged && this.needsViewportUpdate()) {
    return true;
  }
  return changeFlags.dataChanged || changeFlags.propsChanged;
}
```

### 3. Use Transitions for Programmatic Changes

```javascript
// Smooth transitions instead of instant jumps
deck.setProps({
  viewState: {
    ...targetState,
    transitionDuration: 1000,
    transitionInterpolator: new FlyToInterpolator(),
  },
});
```

### 4. Constrain View State Appropriately

```javascript
// Prevent invalid states
onViewStateChange: ({ viewState }) => {
  return {
    ...viewState,
    zoom: Math.max(0, Math.min(20, viewState.zoom)),
    pitch: Math.max(0, Math.min(60, viewState.pitch)),
  };
};
```

## Conclusion

deck.gl's view state management achieves high-performance interactive visualization through:

1. **Efficient change detection**: Viewport comparison minimizes unnecessary work
2. **GPU-optimized transformations**: All coordinate math happens on GPU
3. **Selective updates**: Layers only update state when necessary
4. **Smooth interactions**: 60fps during continuous pan/zoom
5. **Flexible architecture**: Supports multiple views, controllers, and constraints
6. **Transition system**: Smooth animations for programmatic changes

The key insight is that **viewport changes are extremely cheap** because they only require GPU matrix transformations, not CPU buffer updates. This allows deck.gl to maintain smooth 60fps performance even during rapid user interactions, making it ideal for interactive geospatial and 3D visualizations.

