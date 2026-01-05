# Napari: Interactive Data Mutation and View Propagation

This document explains how napari allows users to create or modify data through interactive actions (e.g., drawing shapes, editing labels, selections), focusing on how these interactions mutate underlying data structures and how changes propagate to other views.

## Overview

Napari uses a **layer-based architecture** where each layer type (`Image`, `Labels`, `Points`, `Shapes`, `Vectors`, `Surface`) maintains its own data structure and provides interactive editing capabilities. The system uses an **event-driven architecture** to propagate changes from user interactions to the underlying data and then to all dependent views.

## Interactive Actions and Data Mutation

### 1. Layer Modes

Each layer type supports different interaction modes that determine how user input mutates the data:

#### Labels Layer - Paint Mode
```python
import napari
from skimage import data

viewer = napari.Viewer()
image = data.coins()[50:-50, 50:-50]
viewer.add_image(image, name='coins')

# Create labels layer
label_image = label(cleared).astype('uint8')
label_layer = viewer.add_labels(
    label_image,
    name='segmentation',
    opacity=0.7
)

# Switch to paint mode - enables interactive painting
label_layer.mode = 'paint'  # User can now paint directly on the layer
label_layer.brush_size = 10
```

**Data Mutation**: When in paint mode, mouse clicks and drags directly modify the underlying numpy array stored in `label_layer.data`. The array is mutated in-place, changing pixel values to the current label value.

#### Points Layer - Add Mode
```python
points = viewer.add_points(
    name='annotations',
    size=5,
    face_color='red'
)

# Set to add mode for manual annotation
points.mode = 'add'  # Clicking adds new points to the data
```

**Data Mutation**: In add mode, each click appends a new coordinate to the `points.data` array (shape: `(N, D)` where N is number of points, D is dimensions).

#### Shapes Layer - Selection and Modification
```python
shapes_layer = viewer.add_shapes(
    polygons,
    shape_type='polygon',
    name='regions'
)

# Select shapes programmatically
shapes_layer.selected_data = {0, 1}  # Select first two shapes

# Modify selected shapes' properties
shapes_layer.current_edge_width = 5

# Add new shapes interactively or programmatically
ellipse = np.array([[59, 222], [110, 289], [170, 243], [119, 176]])
shapes_layer.add(
    ellipse,
    shape_type='ellipse'
)
```

**Data Mutation**: Shapes are stored as arrays of vertex coordinates. When shapes are selected and modified (dragged, resized), the vertex coordinates in `shapes_layer.data` are updated directly.

### 2. Direct Data Property Mutation

The most direct way to mutate layer data is through the `data` property:

```python
# Direct mutation of image data
layer = viewer.layers.selection.active
if hasattr(layer, 'data'):
    layer.data = 255 - layer.data  # Invert image - mutates underlying array
```

**Key Point**: Assigning to `layer.data` directly mutates the underlying numpy array. This is the fundamental mechanism by which all interactive edits work - user interactions ultimately result in assignments to the `data` property.

### 3. Programmatic vs Interactive Mutations

Both programmatic and interactive edits use the same mutation mechanism:

```python
# Programmatic: Add points via code
coords = np.array([[32, 64, 64], [16, 32, 32], [48, 80, 80]])
viewer.add_points(coords, name='detected_centers')

# Interactive: User clicks in 'add' mode - same underlying mutation
points.mode = 'add'  # User clicks add points to points.data
```

## Event System and Change Propagation

### 1. Layer Events

Napari uses an event system built on `vispy` events. Each layer exposes an `events` object with various event types:

```python
points_layer = viewer.add_points(name='interactive')

# Connect to data change events
@points_layer.events.data.connect
def on_points_changed(event):
    """Called when points are added/removed/modified."""
    print(f"Points layer now has {len(points_layer.data)} points")
    # This callback is triggered whenever points_layer.data is mutated
```

**How it works**:
1. User interaction (e.g., clicking to add a point) triggers a mouse event
2. The layer's interaction handler updates `layer.data`
3. Setting `layer.data` emits a `layer.events.data` event
4. All connected callbacks are executed
5. The viewer's rendering system is notified to update the display

### 2. Viewer-Level Events

The viewer also provides event hooks for broader interactions:

```python
# Mouse drag callbacks
@viewer.mouse_drag_callbacks.append
def on_drag(viewer, event):
    """Called during mouse drag operations."""
    if event.type == 'mouse_move':
        print(f"Position: {event.position}")

# Custom keybindings that mutate data
@viewer.bind_key('Shift-I')
def invert_image(viewer):
    """Invert the active image layer."""
    layer = viewer.layers.selection.active
    if hasattr(layer, 'data'):
        layer.data = 255 - layer.data  # Mutation triggers events
        print("Image inverted!")
```

### 3. Multi-View Synchronization

When multiple views or layers depend on the same data, changes propagate through the event system:

```python
# Example: Updating layer data propagates to all views
def update_layer(image_text_tuple):
    image, text = image_text_tuple
    viewer.layers["result"].data = image  # Mutation
    viewer.text_overlay.text = text
    # The viewer automatically re-renders all affected views
```

**Propagation Flow**:
1. **Data Mutation**: `layer.data = new_data` (or interactive edit)
2. **Event Emission**: `layer.events.data.emit(event)`
3. **Viewer Notification**: Viewer's layer list is notified
4. **Rendering Update**: All views showing this layer are marked for re-render
5. **Visual Update**: Qt/OpenGL rendering pipeline updates the display

## Architecture: Separation of Model and View

Napari separates the **ViewerModel** (core logic and data) from the **Window** (Qt GUI implementation). This separation enables:

1. **Reactive Updates**: Model changes automatically propagate to views
2. **Multiple Views**: The same layer data can be displayed in different ways
3. **Programmatic Access**: Data can be mutated programmatically with the same mechanism as interactive edits

```python
# The same mutation works whether triggered by:
# 1. User interaction (mouse click in paint mode)
label_layer.data[y, x] = new_label_value

# 2. Programmatic assignment
label_layer.data = modified_array

# 3. Custom callback
@viewer.bind_key('p')
def paint_label(viewer):
    # Programmatic mutation that mirrors interactive behavior
    layer = viewer.layers.selection.active
    if isinstance(layer, napari.layers.Labels):
        # Mutate data - triggers same events as interactive edit
        layer.data[100:200, 100:200] = 5
```

## Data Structure Mutations by Layer Type

### Labels Layer
- **Data Structure**: `numpy.ndarray` of integer labels (uint8/uint16/uint32)
- **Mutations**: Direct pixel assignment `data[y, x] = label_id`
- **Paint Mode**: Brush operations modify rectangular regions of the array
- **Event**: `layer.events.data` fires on any pixel change

### Points Layer
- **Data Structure**: `numpy.ndarray` of shape `(N, D)` - N points in D dimensions
- **Mutations**: 
  - Add: `data = np.vstack([data, new_point])`
  - Remove: `data = np.delete(data, indices, axis=0)`
  - Move: `data[index] = new_coordinates`
- **Event**: `layer.events.data` fires on any array modification

### Shapes Layer
- **Data Structure**: List of `numpy.ndarray`, each shape is an array of vertices
- **Mutations**:
  - Add: `shapes_layer.add(vertices, shape_type='polygon')`
  - Modify: Vertex coordinates updated in-place
  - Remove: Shape removed from list
- **Event**: `layer.events.data` fires on shape addition/modification/removal

### Image Layer
- **Data Structure**: `numpy.ndarray` of image data
- **Mutations**: Direct array assignment `layer.data = new_array`
- **Event**: `layer.events.data` fires on data replacement

## Example: Complete Interactive Workflow

```python
import napari
from skimage import data
import numpy as np

viewer = napari.Viewer()

# 1. Add base image
image = data.camera()
viewer.add_image(image, name='base')

# 2. Add labels layer for annotation
label_layer = viewer.add_labels(
    np.zeros_like(image, dtype=np.uint8),
    name='annotations'
)

# 3. Set up interactive painting
label_layer.mode = 'paint'
label_layer.brush_size = 15
label_layer.selected_label = 1

# 4. Connect event handler to track changes
@label_layer.events.data.connect
def on_label_change(event):
    """Called whenever labels are painted/modified."""
    unique_labels = np.unique(label_layer.data)
    print(f"Labels present: {unique_labels}")
    # This could trigger analysis, save operations, etc.

# 5. User paints on the layer (interactive)
# - Mouse click/drag in paint mode
# - Directly mutates label_layer.data[y, x] = 1
# - Triggers on_label_change callback
# - Viewer re-renders the labels layer

# 6. Programmatic mutation (same mechanism)
label_layer.data[100:150, 100:150] = 2
# - Same event fires
# - Same callback executes
# - Same view update occurs

napari.run()
```

## Key Takeaways

1. **Unified Mutation Mechanism**: Both interactive and programmatic edits mutate `layer.data`, ensuring consistent behavior.

2. **Event-Driven Propagation**: All data mutations emit events (`layer.events.data`), which notify:
   - Connected callbacks (user-defined handlers)
   - The viewer's rendering system
   - Other layers that depend on this data

3. **Direct Array Mutation**: Napari layers store data as numpy arrays, and mutations happen in-place or through assignment, making the system efficient and predictable.

4. **Mode-Based Interactions**: Layer modes (`paint`, `add`, `select`, etc.) determine how user input is interpreted and translated into data mutations.

5. **Separation of Concerns**: The ViewerModel (data + logic) is separate from the Window (GUI), allowing the same mutation mechanisms to work in both interactive and programmatic contexts.

This architecture ensures that whether a user draws a shape with the mouse or a script modifies data programmatically, the same underlying mechanisms handle the mutation and propagate changes to all views consistently.

