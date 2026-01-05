# Napari Architecture: Layer Objects as Unit of Truth

## Executive Summary

This document provides a comprehensive architectural analysis of napari's core design principles, focusing on how **Layer objects** serve as the fundamental unit of truth, how the **Viewer model** owns and manages these layers, and how this architecture ensures consistency across views and plugins.

**Key Insight**: Napari uses a centralized layer-based architecture:
- **Layer Objects as Unit of Truth**: Each Layer encapsulates both data and metadata, serving as the authoritative source
- **Viewer Ownership Model**: The Viewer model owns and manages all Layer objects through a centralized layer list
- **Consistent State Management**: All views and plugins interact with the same Layer objects, ensuring synchronization
- **Event-Driven Updates**: Changes to layers propagate automatically through the Viewer's event system

---

## 1. The Unit of Truth: Layer Objects

### 1.1 Layer as Data Container

In napari, the **Layer object** is the fundamental unit of truth. Each Layer serves as a self-contained entity that encapsulates:

1. **Underlying Data**: The actual data array (numpy arrays, zarr arrays, etc.)
2. **Metadata**: Properties such as:
   - Spatial coordinates and scaling
   - Color mapping and colormaps
   - Visibility and opacity
   - Blending modes
   - Rendering parameters
3. **Type-Specific Behavior**: Each layer type (Image, Labels, Points, Shapes, Surfaces, Tracks, Vectors) provides specialized functionality

### 1.2 Layer Types and Their Data

Napari supports multiple layer types, each with its own data structure:

- **Image Layer**: 2D/3D/4D image data with intensity values
- **Labels Layer**: Integer-labeled segmentation data
- **Points Layer**: Coordinate-based point annotations
- **Shapes Layer**: Geometric shapes (polygons, rectangles, ellipses, etc.)
- **Surfaces Layer**: 3D mesh data
- **Tracks Layer**: Time-series tracking data
- **Vectors Layer**: Vector field visualizations

### 1.3 Data Encapsulation Principle

The Layer object maintains a direct reference to the underlying data array through its `data` property. This design ensures:

- **Single Source of Truth**: The data array exists in one location, referenced by the Layer
- **Immediate Consistency**: Any modification to the Layer's data is immediately reflected
- **No Data Duplication**: Views and plugins reference the same data object, preventing inconsistencies

```python
# Conceptual representation
layer = Image(data=my_array)  # Layer owns the data reference
layer.data[100, 100] = 255    # Direct modification
# All views immediately see the change
```

---

## 2. Viewer Model: Centralized Layer Management

### 2.1 Viewer as Layer Owner

The **Viewer model** acts as the central hub that owns and manages all Layer objects. When a layer is added to the viewer, it becomes part of the viewer's `layers` collection (a `LayerList`).

**Key Ownership Characteristics**:

1. **Centralized Registry**: `viewer.layers` maintains the complete list of all layers
2. **Order Management**: The layer list maintains the rendering order (z-order)
3. **Lifecycle Control**: The viewer controls layer addition, removal, and updates
4. **State Coordination**: The viewer synchronizes layer properties across all views

### 2.2 Layer Identification

Layers are identified and managed through several mechanisms:

1. **Layer List Index**: Position in `viewer.layers` list
2. **Layer Name**: Unique or user-assigned names (accessible via `layer.name`)
3. **Layer Object Reference**: Direct Python object reference
4. **Layer Type**: Classification by layer type (Image, Labels, etc.)

```python
# Adding a layer to viewer
viewer.add_image(data)  # Creates Image layer, adds to viewer.layers

# Accessing layers
layer = viewer.layers[0]           # By index
layer = viewer.layers['my_layer']  # By name
layer = viewer.layers.selection.active  # Currently selected layer
```

### 2.3 LayerList: The Container

The `LayerList` is a specialized collection that:

- **Maintains Order**: Preserves layer order for rendering (bottom to top)
- **Emits Events**: Notifies observers of layer additions, removals, and modifications
- **Provides Selection**: Manages which layer is currently active/selected
- **Enables Synchronization**: Coordinates updates across all registered views

---

## 3. Consistency Across Views and Plugins

### 3.1 Single Source of Truth Architecture

The Layer-as-unit-of-truth design ensures consistency through:

#### 3.1.1 Unified Data Representation

Since each Layer encapsulates its data and metadata, any changes to the data are immediately reflected in all views that display the layer:

```
┌─────────────────────────────────────────────────────────┐
│                    Layer Object                         │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Data Array (numpy/zarr)                         │  │
│  │  Metadata (coordinates, colormap, opacity, etc.) │  │
│  └───────────────────────────────────────────────────┘  │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
   ┌────────┐  ┌────────┐  ┌────────┐
   │ View 1 │  │ View 2 │  │ Plugin │
   │ (Main) │  │ (Ortho)│  │ (Edit) │
   └────────┘  └────────┘  └────────┘
   
All reference the same Layer object → Automatic consistency
```

#### 3.1.2 Property Synchronization

Layer properties (visibility, opacity, colormap, etc.) are stored in the Layer object itself, not in individual views:

- **Visibility**: `layer.visible = False` hides the layer in ALL views
- **Opacity**: `layer.opacity = 0.5` applies to ALL views
- **Colormap**: `layer.colormap = 'viridis'` updates ALL views simultaneously

### 3.2 Plugin Interoperability

Plugins interact with the Viewer and its layers through a standardized API:

#### 3.2.1 Layer Access Pattern

```python
# Plugin accessing layers
@napari_hook_implementation
def my_plugin_function(viewer: Viewer):
    # Access all layers through viewer
    for layer in viewer.layers:
        # Process layer data
        process(layer.data)
    
    # Modify layer properties
    active_layer = viewer.layers.selection.active
    active_layer.opacity = 0.7  # Change visible in all views
```

#### 3.2.2 Standardized Interaction

By adhering to the layer model, plugins can:

- **Read Data**: Access `layer.data` to read the underlying array
- **Modify Data**: Update `layer.data` directly (changes propagate automatically)
- **Add Layers**: Create new layers via `viewer.add_*()` methods
- **Remove Layers**: Remove layers via `viewer.layers.remove()`
- **Listen to Events**: Subscribe to layer events for reactive behavior

#### 3.2.3 Conflict Prevention

The centralized ownership model prevents conflicts:

- **No Duplicate Data**: Plugins work with the same Layer objects, not copies
- **Atomic Updates**: Changes to layer data are immediately visible to all plugins
- **Event Coordination**: The Viewer's event system ensures all plugins are notified of changes

### 3.3 Event-Driven Synchronization

The Viewer model manages events related to layer changes:

#### 3.3.1 Event Types

- **Layer Added**: `viewer.layers.events.inserted`
- **Layer Removed**: `viewer.layers.events.removed`
- **Layer Updated**: `layer.events.data`, `layer.events.set_data`
- **Property Changed**: `layer.events.opacity`, `layer.events.visible`, etc.

#### 3.3.2 Real-Time Updates

The event-driven architecture enables:

```python
# View or plugin subscribing to layer changes
@viewer.layers.selection.events.active.connect
def on_active_layer_changed(event):
    # React to layer selection change
    new_layer = event.value
    update_ui_for_layer(new_layer)

@layer.events.data.connect
def on_data_changed(event):
    # React to data modification
    # All views automatically update their rendering
    pass
```

#### 3.3.3 Cross-View Synchronization

Plugins like `napari-orthogonal-views` leverage this architecture:

- **Shared Layer References**: All orthogonal views reference the same Layer objects
- **Event Propagation**: Interactions in one view (panning, zooming) trigger events
- **Automatic Updates**: Other views receive events and update accordingly
- **Consistent State**: All views always show the same data state

---

## 4. Architectural Benefits

### 4.1 Data Integrity

- **No Stale Data**: Since all components reference the same Layer objects, there's no risk of displaying outdated data
- **Atomic Operations**: Data modifications are immediately visible everywhere
- **Consistent Metadata**: Properties like coordinates and scaling are stored once in the Layer

### 4.2 Plugin Ecosystem

- **Standardized API**: Plugins interact with a well-defined Layer/Viewer interface
- **Composability**: Multiple plugins can work on the same layers without conflicts
- **Extensibility**: New layer types can be added while maintaining the same architecture

### 4.3 Performance

- **Efficient Rendering**: Views can share rendering caches and resources
- **Lazy Evaluation**: Data is only loaded/processed when needed
- **Event Batching**: Multiple changes can be batched for efficient updates

### 4.4 User Experience

- **Predictable Behavior**: Changes in one view are immediately visible in others
- **Unified Interface**: All views and plugins work with the same layer properties
- **Undo/Redo**: Layer state can be tracked and restored consistently

---

## 5. Practical Examples

### 5.1 Multi-View Consistency

```python
# Create viewer with image layer
viewer = napari.Viewer()
layer = viewer.add_image(image_data)

# Create orthogonal views plugin
# All views reference the same 'layer' object
# Panning in one view updates all views because they share:
# - Same layer.data
# - Same layer.visible
# - Same layer.opacity
# - Same layer.colormap
```

### 5.2 Plugin Data Modification

```python
@napari_hook_implementation
def threshold_plugin(viewer: Viewer):
    """Plugin that thresholds an image layer"""
    active_layer = viewer.layers.selection.active
    
    if isinstance(active_layer, Image):
        # Modify the layer's data directly
        thresholded = active_layer.data > 128
        active_layer.data = thresholded.astype(np.uint8) * 255
        
        # This change is immediately visible in:
        # - Main viewer canvas
        # - All orthogonal views
        # - Any other views displaying this layer
        # - Any other plugins monitoring this layer
```

### 5.3 Layer Property Synchronization

```python
# User adjusts opacity slider in UI
layer.opacity = 0.5

# This single assignment triggers:
# 1. Layer object updates its internal state
# 2. Layer emits 'opacity' event
# 3. All views receive event and update rendering
# 4. All plugins monitoring opacity are notified
# 5. UI elements (sliders, displays) update

# All happen automatically through the event system
```

---

## 6. Comparison with Alternative Architectures

### 6.1 What Napari Does NOT Do

Napari avoids these patterns that could lead to inconsistency:

- **View-Owned Data**: Views don't maintain their own copies of data
- **Plugin Data Copies**: Plugins don't create separate data structures
- **Decentralized State**: Layer properties aren't stored in multiple places

### 6.2 Why This Matters

If napari used a different architecture (e.g., views owning data copies):

```
❌ Problematic Architecture:
View 1 → Data Copy 1
View 2 → Data Copy 2
Plugin → Data Copy 3

Result: Inconsistency, synchronization issues, memory waste
```

```
✅ Napari Architecture:
Layer → Single Data Object
    ↓
View 1, View 2, Plugin (all reference same Layer)

Result: Consistency, efficiency, simplicity
```

---

## 7. Summary

Napari's architecture is built on a simple but powerful principle: **Layer objects are the unit of truth, and the Viewer model owns and manages them**.

### Key Takeaways

1. **Layer = Unit of Truth**: Each Layer encapsulates data and metadata, serving as the authoritative source
2. **Viewer = Central Manager**: The Viewer owns all layers through `viewer.layers` (LayerList)
3. **Consistency Through Reference**: All views and plugins reference the same Layer objects
4. **Event-Driven Updates**: Changes propagate automatically through napari's event system
5. **Plugin Standardization**: Plugins interact with layers through a well-defined API

This architecture ensures that:
- Data modifications are immediately visible everywhere
- Layer properties are synchronized across all views
- Plugins can work together without conflicts
- The user experience is consistent and predictable
- The codebase remains maintainable and extensible

The design elegantly solves the challenge of maintaining consistency in a multi-view, multi-plugin scientific visualization application by centralizing data ownership and using an event-driven update mechanism.

