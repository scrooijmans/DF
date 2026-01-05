# Napari Rendering and Recomputation: Event Propagation Chain

## Executive Summary

This document provides a comprehensive technical analysis of what triggers recomputation or re-rendering in napari, including layer data changes, property modifications, plugin executions, and the complete event propagation chain that ensures the viewer stays synchronized with the underlying data.

**Key Insight**: Napari uses a hierarchical event-driven rendering system:
- **Layer-Level Events**: Data and property changes emit events at the layer level
- **LayerList-Level Events**: Layer collection changes propagate through LayerList
- **Viewer-Level Events**: Viewer coordinates recomputation and re-rendering
- **Automatic Synchronization**: Event propagation ensures all components update together

---

## 1. Triggers for Recomputation and Re-rendering

### 1.1 Layer Data Changes

Layer data modifications are one of the primary triggers for recomputation and re-rendering in napari.

#### 1.1.1 Data Replacement Events

When layer data is replaced or significantly modified, napari emits specific events:

**`layer.events.set_data`**
- **Triggered when**: The primary data array of a layer is set or replaced
- **Example**: `layer.set_data(new_data)` or `layer.data = new_data`
- **Effect**: Viewer recomputes layer bounds, scaling, and rendering parameters
- **Rendering impact**: Full re-render of the affected layer

```python
# Example: Image layer data replacement
viewer = napari.Viewer()
layer = viewer.add_image(image_data)

# Replace data - triggers set_data event
new_image = process_image(image_data)
layer.set_data(new_image)
# ✅ layer.events.set_data emitted
# ✅ Viewer recomputes layer properties
# ✅ Canvas re-renders the layer
```

**`layer.events.data`**
- **Triggered when**: The data property is directly assigned
- **Example**: `layer.data = new_data`
- **Effect**: Similar to `set_data`, but may have different internal handling
- **Rendering impact**: Full re-render of the affected layer

#### 1.1.2 Data Refresh Events

**`layer.events.refresh`**
- **Triggered when**: The layer's visual representation needs to be refreshed
- **Example**: After data updates, when rendering cache needs invalidation
- **Effect**: Forces a visual refresh without full recomputation
- **Rendering impact**: Re-render using existing computed parameters

```python
# Example: Triggering a refresh
layer.events.refresh()
# ✅ Canvas updates rendering
# ✅ No recomputation of bounds/scaling
```

#### 1.1.3 In-Place Data Modifications

For mutable layers (Points, Shapes), in-place modifications can trigger updates:

```python
# Points layer - in-place modification
layer = viewer.add_points(points_data)

# Direct array modification
layer.data[0] = [10, 20, 30]
# ⚠️ May not automatically trigger events
# ✅ Should use layer methods or explicitly emit events
```

**Best Practice**: Use layer methods that properly emit events:
- `layer.add()` for adding points
- `layer.remove()` for removing points
- `layer.set_data()` for replacing data

### 1.2 Property Changes

Layer property modifications trigger property-specific events that lead to re-rendering.

#### 1.2.1 Visual Properties

**`layer.events.opacity`**
- **Triggered when**: `layer.opacity = value`
- **Effect**: Updates blending calculations, re-renders layer with new opacity
- **Rendering impact**: Re-render of affected layer (and potentially underlying layers)

```python
# Example: Opacity change
layer.opacity = 0.5
# ✅ layer.events.opacity(value=0.5) emitted
# ✅ Viewer updates blending calculations
# ✅ Canvas re-renders layer with new opacity
```

**`layer.events.visible`**
- **Triggered when**: `layer.visible = True/False`
- **Effect**: Shows or hides layer in rendering
- **Rendering impact**: Re-render to show/hide layer

```python
# Example: Visibility change
layer.visible = False
# ✅ layer.events.visible(value=False) emitted
# ✅ Viewer excludes layer from rendering
# ✅ Canvas re-renders without this layer
```

**`layer.events.blending`**
- **Triggered when**: `layer.blending = 'additive'` or other blending modes
- **Effect**: Updates color blending calculations
- **Rendering impact**: Re-render of affected layer and layers below it

```python
# Example: Blending mode change
layer.blending = 'additive'
# ✅ layer.events.blending(value='additive') emitted
# ✅ Viewer updates blending calculations
# ✅ Canvas re-renders with new blending
```

#### 1.2.2 Transform Properties

**`layer.events.scale`**
- **Triggered when**: `layer.scale = [sx, sy, sz]`
- **Effect**: Recomputes spatial transformations
- **Rendering impact**: Re-render with new scale

**`layer.events.translate`**
- **Triggered when**: `layer.translate = [tx, ty, tz]`
- **Effect**: Recomputes spatial position
- **Rendering impact**: Re-render at new position

**`layer.events.rotate`**
- **Triggered when**: `layer.rotate = angle`
- **Effect**: Recomputes rotation transformation
- **Rendering impact**: Re-render with new rotation

**`layer.events.shear`**
- **Triggered when**: `layer.shear = shear_matrix`
- **Effect**: Recomputes shear transformation
- **Rendering impact**: Re-render with new shear

```python
# Example: Transform property changes
layer.scale = [2.0, 2.0, 1.0]
# ✅ layer.events.scale(value=[2.0, 2.0, 1.0]) emitted
# ✅ Viewer recomputes transformation matrix
# ✅ Canvas re-renders with new scale

layer.translate = [10, 20, 0]
# ✅ layer.events.translate(value=[10, 20, 0]) emitted
# ✅ Viewer recomputes position
# ✅ Canvas re-renders at new position
```

#### 1.2.3 Colormap and Color Properties

**`layer.events.colormap`**
- **Triggered when**: `layer.colormap = 'viridis'`
- **Effect**: Updates color mapping calculations
- **Rendering impact**: Re-render with new colormap

**`layer.events.gamma`**
- **Triggered when**: `layer.gamma = value`
- **Effect**: Updates gamma correction
- **Rendering impact**: Re-render with new gamma

**`layer.events.contrast_limits`**
- **Triggered when**: `layer.contrast_limits = [min, max]`
- **Effect**: Updates intensity scaling
- **Rendering impact**: Re-render with new contrast

```python
# Example: Colormap and color properties
layer.colormap = 'viridis'
# ✅ layer.events.colormap(value='viridis') emitted
# ✅ Viewer updates color mapping
# ✅ Canvas re-renders with new colormap

layer.contrast_limits = [0, 255]
# ✅ layer.events.contrast_limits(value=[0, 255]) emitted
# ✅ Viewer updates intensity scaling
# ✅ Canvas re-renders with new contrast
```

### 1.3 Plugin Execution

Plugins can trigger recomputation and re-rendering by modifying layers or viewer state.

#### 1.3.1 Layer Modifications by Plugins

When plugins modify layer data or properties, they should emit appropriate events:

```python
@napari_hook_implementation
def filter_plugin(viewer: Viewer):
    """Plugin that applies a filter to an image layer"""
    active_layer = viewer.layers.selection.active
    
    if isinstance(active_layer, Image):
        # Process data
        filtered_data = apply_filter(active_layer.data)
        
        # Update layer data - triggers set_data event
        active_layer.set_data(filtered_data)
        # ✅ layer.events.set_data emitted
        # ✅ Viewer recomputes layer
        # ✅ Canvas re-renders layer
```

#### 1.3.2 Adding/Removing Layers

Plugins that add or remove layers trigger LayerList events:

**`viewer.layers.events.inserted`**
- **Triggered when**: `viewer.add_image()`, `viewer.add_points()`, etc.
- **Effect**: Viewer incorporates new layer into rendering pipeline
- **Rendering impact**: Re-render to include new layer

**`viewer.layers.events.removed`**
- **Triggered when**: `viewer.layers.remove(layer)` or layer deleted
- **Effect**: Viewer removes layer from rendering pipeline
- **Rendering impact**: Re-render without removed layer

```python
# Example: Plugin adding a layer
@napari_hook_implementation
def analysis_plugin(viewer: Viewer):
    """Plugin that adds a new analysis layer"""
    # Add new layer
    result_layer = viewer.add_image(analysis_result)
    # ✅ viewer.layers.events.inserted emitted
    # ✅ Viewer incorporates new layer
    # ✅ Canvas re-renders with new layer
```

#### 1.3.3 Viewer Property Changes

Plugins can modify viewer-level properties that trigger re-rendering:

**`viewer.dims.events.ndisplay`**
- **Triggered when**: `viewer.dims.ndisplay = 2` or `3`
- **Effect**: Changes 2D/3D display mode
- **Rendering impact**: Full re-render in new display mode

**`viewer.dims.events.current_step`**
- **Triggered when**: User navigates through dimensions
- **Effect**: Updates which slice/timepoint is displayed
- **Rendering impact**: Re-render with new slice

```python
# Example: Plugin changing viewer properties
@napari_hook_implementation
def display_plugin(viewer: Viewer):
    """Plugin that changes display mode"""
    # Change to 3D mode
    viewer.dims.ndisplay = 3
    # ✅ viewer.dims.events.ndisplay(value=3) emitted
    # ✅ Viewer switches to 3D rendering
    # ✅ Canvas re-renders in 3D mode
```

### 1.4 User Interactions

User interactions can trigger re-rendering through various mechanisms:

#### 1.4.1 Camera/Viewport Changes

**`viewer.camera.events.center`**
- **Triggered when**: User pans the view
- **Effect**: Updates camera center
- **Rendering impact**: Re-render from new viewpoint

**`viewer.camera.events.zoom`**
- **Triggered when**: User zooms in/out
- **Effect**: Updates zoom level
- **Rendering impact**: Re-render at new zoom

**`viewer.camera.events.angles`**
- **Triggered when**: User rotates 3D view
- **Effect**: Updates camera angles
- **Rendering impact**: Re-render from new angle

#### 1.4.2 Dimension Navigation

**`viewer.dims.events.current_step`**
- **Triggered when**: User moves slider or uses arrow keys
- **Effect**: Updates current slice/timepoint
- **Rendering impact**: Re-render with new slice

```python
# Example: User interaction triggering re-render
# User drags dimension slider
viewer.dims.current_step = (10, 20, 0)
# ✅ viewer.dims.events.current_step emitted
# ✅ Viewer updates displayed slice
# ✅ Canvas re-renders with new slice
```

---

## 2. Event Propagation Chain

The event propagation in napari follows a hierarchical structure, ensuring that changes at any level are properly communicated and acted upon.

### 2.1 Layer-Level Event Emission

Events originate at the layer level when data or properties change:

```
┌─────────────────────────────────────────────────────────┐
│                    Layer Object                         │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Property Setter (e.g., opacity, data, visible)   │  │
│  │  1. Update internal state                         │  │
│  │  2. Emit event: layer.events.{property}           │  │
│  └───────────────────────────────────────────────────┘  │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
         ┌───────────────────────┐
         │  Event Emission       │
         │  - events.data        │
         │  - events.opacity     │
         │  - events.visible     │
         │  - events.set_data    │
         │  - events.refresh    │
         └───────────────────────┘
```

**Example Flow**:
```python
# User or plugin modifies layer property
layer.opacity = 0.5

# Layer property setter executes:
# 1. Updates internal state: self._opacity = 0.5
# 2. Emits event: self.events.opacity(value=0.5)
# 3. Event propagates to listeners
```

### 2.2 LayerList-Level Event Handling

The `LayerList` listens to individual layer events and can emit higher-level events:

```
┌─────────────────────────────────────────────────────────┐
│                    LayerList                            │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Event Listeners                                  │  │
│  │  - Listens to layer.events.*                      │  │
│  │  - Emits layers.events.changed                    │  │
│  │  - Emits layers.events.inserted                   │  │
│  │  - Emits layers.events.removed                    │  │
│  └───────────────────────────────────────────────────┘  │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
         ┌───────────────────────┐
         │  LayerList Events     │
         │  - events.inserted    │
         │  - events.removed     │
         │  - events.changed    │
         │  - events.reordered  │
         └───────────────────────┘
```

**Example Flow**:
```python
# Layer emits event
layer.opacity = 0.5  # Emits layer.events.opacity

# LayerList receives event
# LayerList can:
# 1. Track layer state changes
# 2. Emit layers.events.changed if needed
# 3. Update layer ordering/selection if needed
```

### 2.3 Viewer-Level Event Handling

The `Viewer` listens to events from both individual layers and the LayerList:

```
┌─────────────────────────────────────────────────────────┐
│                    Viewer Object                        │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Event Listeners                                  │  │
│  │  - Listens to layer.events.*                      │  │
│  │  - Listens to layers.events.*                     │  │
│  │  - Listens to dims.events.*                       │  │
│  │  - Listens to camera.events.*                     │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Rendering Coordinator                            │  │
│  │  - Determines what needs re-rendering             │  │
│  │  - Triggers canvas update                         │  │
│  │  - Manages rendering pipeline                     │  │
│  └───────────────────────────────────────────────────┘  │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
         ┌───────────────────────┐
         │  Canvas Update        │
         │  - Recompute bounds  │
         │  - Update scene graph │
         │  - Trigger redraw     │
         └───────────────────────┘
```

**Example Flow**:
```python
# Layer emits event
layer.opacity = 0.5  # Emits layer.events.opacity

# Viewer receives event
# Viewer:
# 1. Determines affected layers
# 2. Updates blending calculations
# 3. Triggers canvas update
# 4. Canvas re-renders affected layers
```

### 2.4 Complete Event Propagation Chain

The complete chain from property change to rendered output:

```
┌─────────────────────────────────────────────────────────────┐
│  Step 1: Property Change                                     │
│  layer.opacity = 0.5                                         │
└────────────────────┬──────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  Step 2: Layer Event Emission                                │
│  layer.events.opacity(value=0.5)                            │
│  - Event object created with value=0.5                       │
│  - Event broadcast to all registered listeners              │
└────────────────────┬──────────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ LayerList    │ │ Viewer       │ │ Plugins      │
│ Listener     │ │ Listener     │ │ Listeners    │
└──────┬───────┘ └──────┬───────┘ └──────┬───────┘
       │                │                │
       │                ▼                │
       │    ┌───────────────────────┐    │
       │    │ Viewer Processing     │    │
       │    │ - Update blending     │    │
       │    │ - Mark layer dirty    │    │
       │    │ - Schedule redraw     │    │
       │    └───────────┬───────────┘    │
       │                │                │
       │                ▼                │
       │    ┌───────────────────────┐    │
       │    │ Canvas Update         │    │
       │    │ - Recompute scene     │    │
       │    │ - Update render tree  │    │
       │    │ - Trigger redraw      │    │
       │    └───────────┬───────────┘    │
       │                │                │
       │                ▼                │
       │    ┌───────────────────────┐    │
       │    │ GPU Rendering         │    │
       │    │ - Draw layers         │    │
       │    │ - Apply blending      │    │
       │    │ - Display result      │    │
       │    └───────────────────────┘    │
       │                                 │
       └─────────────────────────────────┘
```

### 2.5 Event Subscription and Handling

Components subscribe to events and handle them appropriately:

#### 2.5.1 Viewer Event Subscriptions

```python
# Viewer subscribes to layer events
@layer.events.opacity.connect
def on_layer_opacity_changed(event):
    """Viewer handler for opacity changes"""
    # Update blending calculations
    self._update_blending()
    # Mark canvas for redraw
    self._canvas.update()

@layer.events.data.connect
def on_layer_data_changed(event):
    """Viewer handler for data changes"""
    # Recompute layer bounds
    self._recompute_bounds(layer)
    # Update scene graph
    self._update_scene_graph()
    # Mark canvas for redraw
    self._canvas.update()
```

#### 2.5.2 Canvas Event Subscriptions

```python
# Canvas subscribes to viewer events
@viewer.layers.events.changed.connect
def on_layers_changed(event):
    """Canvas handler for layer changes"""
    # Rebuild scene graph
    self._rebuild_scene_graph()
    # Schedule redraw
    self._schedule_redraw()

@viewer.camera.events.center.connect
def on_camera_moved(event):
    """Canvas handler for camera movement"""
    # Update viewport
    self._update_viewport()
    # Schedule redraw
    self._schedule_redraw()
```

#### 2.5.3 Plugin Event Subscriptions

```python
# Plugin subscribes to layer events
@layer.events.data.connect
def on_data_changed(event):
    """Plugin handler for data changes"""
    # Update plugin's internal state
    self._update_state(event.value)
    # Potentially trigger additional processing
    self._process_if_needed()
```

---

## 3. Rendering Pipeline

### 3.1 Rendering Stages

The rendering process consists of several stages:

#### 3.1.1 Stage 1: Event Detection

- Events are emitted by layers, LayerList, or viewer
- Event listeners are notified
- Affected components are identified

#### 3.1.2 Stage 2: State Update

- Layer properties are updated
- Viewer state is updated
- Scene graph is marked as dirty

#### 3.1.3 Stage 3: Recomputation

- Layer bounds are recomputed (if data changed)
- Transformation matrices are recalculated (if transforms changed)
- Blending parameters are updated (if visual properties changed)

#### 3.1.4 Stage 4: Scene Graph Update

- Scene graph is rebuilt or updated
- Layer order is applied
- Visibility is checked

#### 3.1.5 Stage 5: Rendering

- GPU commands are generated
- Layers are drawn in order
- Blending is applied
- Result is displayed

### 3.2 Rendering Optimization

Napari optimizes rendering through several mechanisms:

#### 3.2.1 Dirty Flagging

Layers and the canvas maintain "dirty" flags to track what needs updating:

```python
# Conceptual representation
class Layer:
    def __init__(self):
        self._dirty = True  # Needs recomputation
        self._render_dirty = True  # Needs re-rendering
    
    @property
    def opacity(self):
        return self._opacity
    
    @opacity.setter
    def opacity(self, value):
        self._opacity = value
        self._render_dirty = True  # Mark for re-render
        self.events.opacity(value=value)
```

#### 3.2.2 Batching Updates

Multiple property changes can be batched to reduce rendering overhead:

```python
# Multiple changes
layer.opacity = 0.5
layer.visible = True
layer.colormap = 'viridis'

# All trigger events, but rendering can be batched
# Canvas may wait for event loop to batch redraws
```

#### 3.2.3 Selective Re-rendering

Only affected layers are re-rendered:

```python
# Only layer 0 changes
viewer.layers[0].opacity = 0.5

# Canvas re-renders:
# - Layer 0 (changed)
# - Layers above layer 0 (affected by blending)
# - Does NOT re-render layers below layer 0 (not affected)
```

---

## 4. Practical Examples

### 4.1 Complete Example: Image Layer Data Update

```python
# Step 1: Create viewer and layer
viewer = napari.Viewer()
layer = viewer.add_image(image_data)

# Step 2: Replace data
new_image = process_image(image_data)
layer.set_data(new_image)

# Event propagation:
# 1. layer.set_data() called
# 2. layer.events.set_data(new_image) emitted
# 3. Viewer receives event
#    - Recomputes layer bounds
#    - Updates scene graph
#    - Marks canvas dirty
# 4. Canvas receives update signal
#    - Rebuilds render tree
#    - Generates GPU commands
#    - Displays new image
# 5. All views update (main view, orthogonal views, etc.)
```

### 4.2 Complete Example: Property Change Cascade

```python
# Step 1: Create viewer with multiple layers
viewer = napari.Viewer()
layer1 = viewer.add_image(image1)
layer2 = viewer.add_image(image2)

# Step 2: Change layer1 opacity
layer1.opacity = 0.5

# Event propagation:
# 1. layer1.opacity = 0.5
# 2. layer1.events.opacity(0.5) emitted
# 3. Viewer receives event
#    - Updates blending calculations for layer1
#    - Marks layer1 and layer2 for re-render (layer2 affected by blending)
# 4. Canvas receives update signal
#    - Re-renders layer1 with new opacity
#    - Re-renders layer2 (affected by layer1's blending)
#    - Displays composite result
```

### 4.3 Complete Example: Plugin Adding Layer

```python
@napari_hook_implementation
def analysis_plugin(viewer: Viewer):
    """Plugin that performs analysis and adds result layer"""
    
    # Step 1: Get active layer
    active_layer = viewer.layers.selection.active
    
    # Step 2: Perform analysis
    result = perform_analysis(active_layer.data)
    
    # Step 3: Add result as new layer
    result_layer = viewer.add_image(result, name='Analysis Result')
    
    # Event propagation:
    # 1. viewer.add_image() called
    # 2. New Image layer created
    # 3. Layer added to viewer.layers
    # 4. viewer.layers.events.inserted(result_layer) emitted
    # 5. Viewer receives event
    #    - Adds layer to scene graph
    #    - Updates layer list UI
    #    - Marks canvas dirty
    # 6. Canvas receives update signal
    #    - Rebuilds render tree with new layer
    #    - Re-renders all layers
    #    - Displays result
```

### 4.4 Complete Example: Dimension Navigation

```python
# User drags dimension slider
viewer.dims.current_step = (10, 20, 0)

# Event propagation:
# 1. viewer.dims.current_step = (10, 20, 0)
# 2. viewer.dims.events.current_step((10, 20, 0)) emitted
# 3. Viewer receives event
#    - Updates displayed slice
#    - Marks all layers for re-render (new slice)
# 4. Canvas receives update signal
#    - Updates slice extraction for all layers
#    - Re-renders all layers with new slice
#    - Displays new slice
```

---

## 5. Event Reference Summary

### 5.1 Layer Events

| Event | Trigger | Effect |
|-------|---------|--------|
| `events.data` | `layer.data = value` | Full recomputation and re-render |
| `events.set_data` | `layer.set_data(value)` | Full recomputation and re-render |
| `events.refresh` | `layer.events.refresh()` | Visual refresh only |
| `events.opacity` | `layer.opacity = value` | Re-render with new opacity |
| `events.visible` | `layer.visible = value` | Show/hide layer |
| `events.blending` | `layer.blending = value` | Re-render with new blending |
| `events.scale` | `layer.scale = value` | Recompute transforms, re-render |
| `events.translate` | `layer.translate = value` | Recompute position, re-render |
| `events.rotate` | `layer.rotate = value` | Recompute rotation, re-render |
| `events.colormap` | `layer.colormap = value` | Re-render with new colormap |
| `events.contrast_limits` | `layer.contrast_limits = value` | Re-render with new contrast |

### 5.2 LayerList Events

| Event | Trigger | Effect |
|-------|---------|--------|
| `events.inserted` | `viewer.add_*()` | Add layer to rendering |
| `events.removed` | `viewer.layers.remove()` | Remove layer from rendering |
| `events.changed` | Layer modified | Update rendering |
| `events.reordered` | Layer order changed | Re-render in new order |

### 5.3 Viewer Events

| Event | Trigger | Effect |
|-------|---------|--------|
| `dims.events.ndisplay` | `viewer.dims.ndisplay = value` | Switch 2D/3D mode |
| `dims.events.current_step` | `viewer.dims.current_step = value` | Update displayed slice |
| `camera.events.center` | Camera pan | Re-render from new position |
| `camera.events.zoom` | Camera zoom | Re-render at new zoom |
| `camera.events.angles` | Camera rotation | Re-render from new angle |

---

## 6. Best Practices

### 6.1 For Plugin Developers

1. **Use Layer Methods**: Prefer `layer.set_data()` over direct assignment when possible
2. **Emit Events Properly**: Ensure events are emitted after modifications
3. **Batch Updates**: Group multiple property changes together when possible
4. **Subscribe to Events**: Listen to relevant events to stay synchronized

```python
# ✅ Good: Use layer methods
layer.set_data(new_data)  # Properly emits events

# ❌ Avoid: Direct assignment (may not emit events properly)
layer.data = new_data  # May work but less reliable
```

### 6.2 For View Developers

1. **Subscribe to All Relevant Events**: Don't miss events that affect rendering
2. **Handle Events Efficiently**: Update only what's necessary
3. **Use Dirty Flags**: Track what needs updating to avoid unnecessary work
4. **Batch Redraws**: Group multiple updates into single redraw when possible

```python
# ✅ Good: Subscribe to all relevant events
@layer.events.data.connect
@layer.events.opacity.connect
@layer.events.visible.connect
def on_layer_changed(event):
    self._update_rendering()
```

### 6.3 For Performance

1. **Minimize Property Changes**: Batch changes when possible
2. **Use Refresh for Visual-Only Updates**: Use `events.refresh()` when recomputation isn't needed
3. **Avoid Unnecessary Re-renders**: Check if value actually changed before emitting event
4. **Optimize Event Handlers**: Keep event handlers lightweight

```python
# ✅ Good: Check if value changed
@opacity.setter
def opacity(self, value):
    if value != self._opacity:  # Only update if changed
        self._opacity = value
        self.events.opacity(value=value)
```

---

## 7. Summary

Napari's rendering and recomputation system is built on a hierarchical event-driven architecture:

### Key Principles

1. **Event-Driven Updates**: All changes trigger events that propagate through the system
2. **Hierarchical Propagation**: Events flow from layers → LayerList → Viewer → Canvas
3. **Automatic Synchronization**: All components stay synchronized through event subscriptions
4. **Selective Re-rendering**: Only affected components are updated

### Triggers

- **Layer Data Changes**: `events.set_data`, `events.data`, `events.refresh`
- **Property Changes**: `events.opacity`, `events.visible`, `events.blending`, etc.
- **Plugin Execution**: Layer modifications, additions, removals
- **User Interactions**: Camera movements, dimension navigation

### Event Propagation Chain

1. **Layer Level**: Property changes emit layer events
2. **LayerList Level**: Layer events propagate to LayerList
3. **Viewer Level**: Viewer coordinates recomputation and rendering
4. **Canvas Level**: Canvas updates scene graph and triggers GPU rendering

This architecture ensures that napari remains responsive and accurate, with all components automatically staying synchronized through the event system.

