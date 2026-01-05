# Napari Data Mutability and Evented Models: Preventing Silent Divergence

## Executive Summary

This document provides a detailed technical analysis of napari's data mutability model, focusing on which layer data is mutable, how mutations are tracked through evented models, and how this architecture prevents silent divergence between data and views.

**Key Insight**: Napari uses a sophisticated evented model architecture:
- **Selective Mutability**: Layer data mutability varies by layer type (Image/Labels vs Points/Shapes)
- **Event-Driven Tracking**: All mutations trigger events through EventedModel and EmitterGroup
- **Automatic Synchronization**: Evented models ensure views and plugins are notified of all changes
- **Silent Divergence Prevention**: The event system guarantees data and views never drift apart

---

## 1. Layer Data Mutability by Type

### 1.1 Immutable Layer Types

Some layer types in napari are designed with **immutable data**, meaning the underlying data array should not be modified in-place:

#### 1.1.1 Image Layer

**Data Mutability**: **Immutable** (by design)

- The `Image` layer holds n-dimensional arrays representing image data
- The underlying data array is typically treated as read-only
- To modify image data, you replace the entire dataset using `layer.set_data()` or `layer.data = new_data`
- Direct in-place modifications (e.g., `layer.data[100, 100] = 255`) are possible but not recommended

**Rationale**: Image data is often large and shared across multiple views. Immutability prevents accidental corruption and ensures consistent rendering.

```python
# Image layer - immutable data pattern
layer = viewer.add_image(image_data)

# ❌ Not recommended: Direct in-place modification
layer.data[100, 100] = 255  # Works but doesn't trigger proper events

# ✅ Recommended: Replace entire dataset
new_data = layer.data.copy()
new_data[100, 100] = 255
layer.set_data(new_data)  # Triggers events.data and events.set_data
```

#### 1.1.2 Labels Layer

**Data Mutability**: **Immutable** (by design)

- The `Labels` layer manages integer-labeled segmentation data
- Similar to Image layers, the data array is typically replaced rather than modified in-place
- Labels are meant to represent segmentation masks that are edited through specialized tools
- Direct array modification is possible but bypasses the event system

**Rationale**: Label data represents discrete regions. Replacing the dataset ensures proper event emission and view updates.

```python
# Labels layer - immutable data pattern
layer = viewer.add_labels(label_data)

# ✅ Recommended: Use layer methods or replace data
layer.set_data(new_labels)  # Proper event emission
```

### 1.2 Mutable Layer Types

Some layer types are designed for **interactive editing**, making their data inherently mutable:

#### 1.2.1 Points Layer

**Data Mutability**: **Mutable** (designed for editing)

- The `Points` layer represents collections of points in n-dimensional space
- Points can be added, removed, or modified interactively
- The layer provides methods for point manipulation that properly emit events
- Direct data modification is supported and tracked through events

**Rationale**: Points are annotation data that users frequently edit. Mutability enables interactive workflows.

```python
# Points layer - mutable data pattern
layer = viewer.add_points(points_data)

# ✅ Supported: Direct modification with event tracking
layer.data[0] = [10, 20, 30]  # Modifies point position, triggers events

# ✅ Also supported: Using layer methods
layer.add([5, 10, 15])  # Adds point, emits events
layer.remove_selected()  # Removes selected points, emits events
```

#### 1.2.2 Shapes Layer

**Data Mutability**: **Mutable** (designed for editing)

- The `Shapes` layer contains geometric shapes (rectangles, ellipses, polygons, etc.)
- Shapes can be added, removed, or modified interactively
- Vertex positions can be edited directly
- All modifications trigger appropriate events

**Rationale**: Shapes are annotation/ROI data that require frequent editing. Mutability is essential for interactive workflows.

```python
# Shapes layer - mutable data pattern
layer = viewer.add_shapes(shapes_data)

# ✅ Supported: Direct modification
layer.data[0].data = new_vertices  # Modifies shape, triggers events

# ✅ Also supported: Using layer methods
layer.add_ellipse(center, radii)  # Adds shape, emits events
layer.remove_selected()  # Removes shapes, emits events
```

### 1.3 Conditionally Mutable Layer Types

Some layer types have mutable data depending on their implementation:

#### 1.3.1 Vectors Layer

**Data Mutability**: **Conditionally Mutable**

- Vector fields can be updated, but the mechanism depends on the specific use case
- Typically, vector data is replaced rather than modified in-place
- Some implementations may support direct modification

#### 1.3.2 Surface Layer

**Data Mutability**: **Conditionally Mutable**

- 3D surface meshes can be updated
- Mesh vertices and faces can be modified, but typically through replacement
- Direct modification may be supported in specific implementations

#### 1.3.3 Tracks Layer

**Data Mutability**: **Conditionally Mutable**

- Trajectory data can be updated
- Track points can be modified, typically through layer methods
- Direct data modification depends on implementation

### 1.4 Mutability Summary Table

| Layer Type | Data Mutability | Recommended Modification Method | Event Tracking |
|------------|----------------|--------------------------------|----------------|
| **Image** | Immutable | `layer.set_data()` or `layer.data = new_data` | ✅ Full event emission |
| **Labels** | Immutable | `layer.set_data()` or `layer.data = new_data` | ✅ Full event emission |
| **Points** | Mutable | Direct modification or `layer.add()`/`layer.remove()` | ✅ Full event emission |
| **Shapes** | Mutable | Direct modification or `layer.add_*()`/`layer.remove()` | ✅ Full event emission |
| **Vectors** | Conditionally Mutable | `layer.set_data()` or layer methods | ✅ Full event emission |
| **Surface** | Conditionally Mutable | `layer.set_data()` or layer methods | ✅ Full event emission |
| **Tracks** | Conditionally Mutable | `layer.set_data()` or layer methods | ✅ Full event emission |

---

## 2. Mutation Tracking: Evented Models

### 2.1 EventedModel Base Class

Napari's layers are built on the `EventedModel` base class, which provides:

1. **Automatic Event Emission**: Events are emitted whenever attributes change
2. **Data Validation**: Type checking and validation using Pydantic
3. **Property Observability**: All property changes are observable
4. **Consistent API**: Uniform event handling across all layer types

### 2.2 EmitterGroup: Event Management

The `EmitterGroup` is the core mechanism for managing events in napari:

```python
from napari.utils.events import EmitterGroup

class Dims:
    def __init__(self, ndisplay):
        self._ndisplay = ndisplay
        # Create emitter group for 'ndisplay' attribute
        self.events = EmitterGroup(source=self, ndisplay=None)

    @property
    def ndisplay(self):
        return self._ndisplay

    @ndisplay.setter
    def ndisplay(self, value):
        self._ndisplay = value
        # Emit event when attribute changes
        self.events.ndisplay(value=value)
```

**Key Characteristics**:
- **Source Tracking**: Events include a reference to the source object
- **Type Safety**: Events can carry typed values
- **Multiple Listeners**: Multiple components can subscribe to the same event
- **Automatic Propagation**: Events propagate to all registered listeners

### 2.3 Layer Data Events

Layers emit specific events when their data changes:

#### 2.3.1 `events.data`

Emitted when the data array reference changes (e.g., `layer.data = new_array`):

```python
@layer.events.data.connect
def on_data_changed(event):
    """Called when layer.data is reassigned"""
    print(f"Data reference changed: {event.value}")
    # event.value contains the new data array
    # Views should update their rendering
```

**When Emitted**:
- `layer.data = new_data` is called
- `layer.set_data(new_data)` is called
- Data array is replaced (not modified in-place)

#### 2.3.2 `events.set_data`

Emitted when data is set through the `set_data()` method:

```python
@layer.events.set_data.connect
def on_set_data(event):
    """Called when layer.set_data() is called"""
    print(f"Data set via set_data(): {event.value}")
    # event.value contains the new data array
```

**When Emitted**:
- `layer.set_data(new_data)` is explicitly called
- Provides a way to distinguish between direct assignment and method calls

#### 2.3.3 In-Place Modification Detection

For mutable layers (Points, Shapes), in-place modifications to the data array can be detected:

```python
# Points layer - in-place modification
layer = viewer.add_points(points_data)

# Direct array modification
layer.data[0] = [10, 20, 30]

# This triggers events if the layer monitors data changes
# However, numpy array modifications don't automatically trigger events
# Layers must explicitly check for changes or use property setters
```

**Challenge**: NumPy array in-place modifications don't automatically trigger Python property setters. Napari handles this through:

1. **Explicit Checks**: Layers periodically check for data changes
2. **Property Wrappers**: Wrapping data access to detect modifications
3. **Method-Based Updates**: Encouraging use of layer methods that explicitly emit events

### 2.4 Property Event Tracking

All layer properties emit events when changed:

```python
# Opacity change
layer.opacity = 0.5
# Triggers: layer.events.opacity(value=0.5)

# Visibility change
layer.visible = False
# Triggers: layer.events.visible(value=False)

# Colormap change
layer.colormap = 'viridis'
# Triggers: layer.events.colormap(value='viridis')
```

**Event Pattern**:
- Property setter updates internal state
- Property setter emits corresponding event
- All listeners receive notification
- Views update rendering accordingly

---

## 3. Preventing Silent Divergence

### 3.1 The Problem: Silent Divergence

**Silent divergence** occurs when:
- Data is modified but views don't update
- Views display stale or incorrect data
- Multiple views show different states of the same data
- Plugins work with outdated information

**Example of Silent Divergence (Without Events)**:

```python
# ❌ Problematic: No event system
class BadLayer:
    def __init__(self, data):
        self._data = data
        self.views = []  # Views that display this layer
    
    @property
    def data(self):
        return self._data
    
    @data.setter
    def data(self, value):
        self._data = value
        # ❌ Views are not notified!
        # Views continue showing old data

# Usage
layer = BadLayer(image_data)
view1 = create_view(layer)  # Shows image_data
view2 = create_view(layer)  # Shows image_data

layer.data = new_image_data  # Data changed
# ❌ view1 and view2 still show old image_data
# Silent divergence!
```

### 3.2 The Solution: Evented Models

Napari's evented models prevent silent divergence by ensuring **all changes trigger events**:

```python
# ✅ Correct: With event system
class GoodLayer(EventedModel):
    def __init__(self, data):
        self._data = data
        self.events = EmitterGroup(source=self, data=None)
        self.views = []  # Views that display this layer
    
    @property
    def data(self):
        return self._data
    
    @data.setter
    def data(self, value):
        self._data = value
        # ✅ Emit event - all listeners notified
        self.events.data(value=value)

# Usage
layer = GoodLayer(image_data)
view1 = create_view(layer)
view1.connect_to_events(layer.events.data)  # Subscribe to events
view2 = create_view(layer)
view2.connect_to_events(layer.events.data)  # Subscribe to events

layer.data = new_image_data
# ✅ events.data emitted
# ✅ view1 receives event, updates rendering
# ✅ view2 receives event, updates rendering
# ✅ No divergence!
```

### 3.3 Event Propagation Flow

When data is modified, events propagate through the system:

```
┌─────────────────────────────────────────────────────────┐
│              Layer Data Modification                     │
│  layer.data = new_data  or  layer.set_data(new_data)   │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│           EventedModel Property Setter                   │
│  1. Validate new data                                    │
│  2. Update internal state (_data = new_data)            │
│  3. Emit event: events.data(value=new_data)             │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              EmitterGroup Event System                   │
│  Broadcasts event to all registered listeners           │
└────┬────────────┬────────────┬────────────┬────────────┘
     │            │            │            │
     ▼            ▼            ▼            ▼
┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐
│ View 1 │  │ View 2 │  │ Plugin │  │ Plugin │
│ Canvas │  │ Ortho  │  │  A     │  │  B     │
└────────┘  └────────┘  └────────┘  └────────┘
     │            │            │            │
     ▼            ▼            ▼            ▼
┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐
│ Update │  │ Update │  │ Update │  │ Update │
│Render  │  │Render  │  │ State  │  │ State  │
└────────┘  └────────┘  └────────┘  └────────┘

Result: All components synchronized, no divergence
```

### 3.4 Guarantees Provided by Evented Models

The evented model architecture provides several guarantees:

#### 3.4.1 Change Notification Guarantee

**Guarantee**: Every property change emits an event.

- Property setters always emit events
- No "silent" property changes are possible
- All listeners are guaranteed to be notified

#### 3.4.2 Consistency Guarantee

**Guarantee**: All views see the same data state.

- All views subscribe to the same events
- All views receive the same event data
- All views update based on the same information

#### 3.4.3 Atomicity Guarantee

**Guarantee**: Changes appear atomic to observers.

- Event emission happens after state update
- Listeners see the new state, not intermediate states
- No race conditions between state and events

#### 3.4.4 Ordering Guarantee

**Guarantee**: Events are emitted in the order of property changes.

- If `layer.opacity = 0.5` then `layer.visible = False`, events fire in that order
- Listeners can rely on event ordering
- State transitions are predictable

### 3.5 Handling In-Place Modifications

For mutable layers, in-place modifications to numpy arrays present a challenge:

```python
# Points layer
layer = viewer.add_points(points_data)

# In-place modification
layer.data[0] = [10, 20, 30]  # NumPy array modified directly
# ❌ This doesn't trigger Python property setter
# ❌ No event is emitted automatically
```

**Napari's Solutions**:

#### Solution 1: Explicit Event Emission

Layers can explicitly emit events after detecting in-place modifications:

```python
class Points(EventedModel):
    def modify_point(self, index, new_position):
        """Modify point and emit event"""
        self._data[index] = new_position
        # Explicitly emit event
        self.events.data(value=self._data)
```

#### Solution 2: Property Wrappers

Layers can wrap data access to detect modifications:

```python
class Points(EventedModel):
    @property
    def data(self):
        return self._data
    
    def _check_data_changed(self):
        """Check if data array was modified"""
        # Compare current data with last known state
        if not np.array_equal(self._data, self._last_data):
            self.events.data(value=self._data)
            self._last_data = self._data.copy()
```

#### Solution 3: Method-Based Updates

Encourage use of layer methods that explicitly emit events:

```python
# ✅ Preferred: Use layer methods
layer.add([10, 20, 30])  # Emits events
layer.remove_selected()  # Emits events
layer.data[0] = [10, 20, 30]  # Direct modification (use with caution)
```

### 3.6 Event Subscription Patterns

Views and plugins subscribe to events to stay synchronized:

```python
# View subscribing to layer events
class View:
    def __init__(self, layer):
        self.layer = layer
        # Subscribe to data changes
        self.layer.events.data.connect(self.on_data_changed)
        # Subscribe to property changes
        self.layer.events.opacity.connect(self.on_opacity_changed)
        self.layer.events.visible.connect(self.on_visible_changed)
    
    def on_data_changed(self, event):
        """Called when layer data changes"""
        new_data = event.value
        self.update_rendering(new_data)  # Update view
    
    def on_opacity_changed(self, event):
        """Called when layer opacity changes"""
        new_opacity = event.value
        self.update_opacity(new_opacity)  # Update view
    
    def on_visible_changed(self, event):
        """Called when layer visibility changes"""
        is_visible = event.value
        self.set_visible(is_visible)  # Update view
```

**Key Points**:
- Views subscribe to all relevant events
- Event handlers update the view's rendering
- Views never maintain their own copy of data
- Views always reflect the current layer state

---

## 4. Practical Examples

### 4.1 Image Layer Data Replacement

```python
# Create image layer
viewer = napari.Viewer()
layer = viewer.add_image(image_data)

# Subscribe to data changes
@layer.events.data.connect
def on_image_data_changed(event):
    print(f"Image data changed: shape={event.value.shape}")

# Replace data (triggers events.data)
new_image = process_image(image_data)
layer.set_data(new_image)
# ✅ Event emitted
# ✅ All views update
# ✅ Plugin listeners notified
# ✅ No silent divergence
```

### 4.2 Points Layer Interactive Editing

```python
# Create points layer
viewer = napari.Viewer()
layer = viewer.add_points(points_data)

# Subscribe to data changes
@layer.events.data.connect
def on_points_changed(event):
    print(f"Points changed: {len(event.value)} points")

# Add point (triggers events)
layer.add([10, 20, 30])
# ✅ Event emitted
# ✅ Views update to show new point

# Remove point (triggers events)
layer.remove_selected()
# ✅ Event emitted
# ✅ Views update to remove point

# Direct modification (use with caution)
layer.data[0] = [5, 10, 15]
# ⚠️ May not trigger events automatically
# Consider using layer methods instead
```

### 4.3 Multi-View Synchronization

```python
# Create viewer with layer
viewer = napari.Viewer()
layer = viewer.add_image(image_data)

# Create multiple views (e.g., main view + orthogonal views)
main_view = viewer.window._qt_viewer.canvas
ortho_view_1 = create_orthogonal_view(layer)
ortho_view_2 = create_orthogonal_view(layer)

# All views subscribe to the same layer events
# When layer data changes:
layer.set_data(new_image_data)

# ✅ main_view receives events.data, updates rendering
# ✅ ortho_view_1 receives events.data, updates rendering
# ✅ ortho_view_2 receives events.data, updates rendering
# ✅ All views show the same data
# ✅ No divergence possible
```

### 4.4 Plugin Data Modification

```python
@napari_hook_implementation
def threshold_plugin(viewer: Viewer):
    """Plugin that modifies layer data"""
    active_layer = viewer.layers.selection.active
    
    if isinstance(active_layer, Image):
        # Process data
        thresholded = active_layer.data > 128
        new_data = thresholded.astype(np.uint8) * 255
        
        # Replace data (triggers events)
        active_layer.set_data(new_data)
        # ✅ events.data emitted
        # ✅ All views update automatically
        # ✅ Other plugins monitoring this layer are notified
        # ✅ No manual view updates needed
        # ✅ No silent divergence
```

### 4.5 Property Change Synchronization

```python
# Create layer
viewer = napari.Viewer()
layer = viewer.add_image(image_data)

# Multiple views subscribe to property events
@layer.events.opacity.connect
def on_opacity_changed(event):
    print(f"Opacity changed to {event.value}")

@layer.events.visible.connect
def on_visible_changed(event):
    print(f"Visibility changed to {event.value}")

# Change properties
layer.opacity = 0.5
# ✅ events.opacity emitted
# ✅ All views update opacity
# ✅ All plugins notified

layer.visible = False
# ✅ events.visible emitted
# ✅ All views hide layer
# ✅ All plugins notified

# ✅ Properties synchronized across all components
# ✅ No divergence possible
```

---

## 5. Comparison: With vs Without Evented Models

### 5.1 Without Evented Models (Problematic)

```python
# ❌ Bad: No event system
class BadLayer:
    def __init__(self, data):
        self._data = data
        self._opacity = 1.0
        self._visible = True
        self.views = []
    
    @property
    def data(self):
        return self._data
    
    @data.setter
    def data(self, value):
        self._data = value
        # ❌ No events emitted
        # ❌ Views not notified
        # ❌ Silent divergence occurs
    
    @property
    def opacity(self):
        return self._opacity
    
    @opacity.setter
    def opacity(self, value):
        self._opacity = value
        # ❌ No events emitted
        # ❌ Views not notified
        # ❌ Silent divergence occurs

# Usage
layer = BadLayer(image_data)
view1 = create_view(layer)
view2 = create_view(layer)

layer.data = new_data
# ❌ view1 and view2 still show old data
# ❌ Silent divergence!

layer.opacity = 0.5
# ❌ view1 and view2 still show opacity=1.0
# ❌ Silent divergence!
```

**Problems**:
- Views don't know when data changes
- Views display stale data
- Multiple views can show different states
- Plugins can't react to changes
- Manual synchronization required (error-prone)

### 5.2 With Evented Models (Napari's Approach)

```python
# ✅ Good: Event system
class GoodLayer(EventedModel):
    def __init__(self, data):
        self._data = data
        self._opacity = 1.0
        self._visible = True
        self.events = EmitterGroup(
            source=self,
            data=None,
            opacity=None,
            visible=None
        )
    
    @property
    def data(self):
        return self._data
    
    @data.setter
    def data(self, value):
        self._data = value
        # ✅ Emit event
        self.events.data(value=value)
    
    @property
    def opacity(self):
        return self._opacity
    
    @opacity.setter
    def opacity(self, value):
        self._opacity = value
        # ✅ Emit event
        self.events.opacity(value=value)

# Usage
layer = GoodLayer(image_data)
view1 = create_view(layer)
view1.subscribe_to_events(layer.events)  # Subscribe
view2 = create_view(layer)
view2.subscribe_to_events(layer.events)  # Subscribe

layer.data = new_data
# ✅ events.data emitted
# ✅ view1 receives event, updates
# ✅ view2 receives event, updates
# ✅ No divergence!

layer.opacity = 0.5
# ✅ events.opacity emitted
# ✅ view1 receives event, updates
# ✅ view2 receives event, updates
# ✅ No divergence!
```

**Benefits**:
- Views automatically notified of changes
- Views always show current data
- All views show the same state
- Plugins can react to changes
- Automatic synchronization (no manual work)

---

## 6. Advanced Topics

### 6.1 Event Batching

Multiple property changes can be batched to reduce event overhead:

```python
# Multiple changes
layer.opacity = 0.5      # Emits events.opacity
layer.visible = False    # Emits events.visible
layer.colormap = 'gray'  # Emits events.colormap

# Views receive three separate events
# Can be batched for efficiency if needed
```

### 6.2 Event Filtering

Listeners can filter events to only react to specific changes:

```python
@layer.events.data.connect
def on_data_changed(event):
    new_data = event.value
    # Only update if shape changed
    if new_data.shape != self.last_shape:
        self.update_rendering(new_data)
        self.last_shape = new_data.shape
```

### 6.3 Conditional Event Emission

Layers can conditionally emit events to avoid unnecessary updates:

```python
@opacity.setter
def opacity(self, value):
    # Only emit if value actually changed
    if value != self._opacity:
        self._opacity = value
        self.events.opacity(value=value)
```

### 6.4 Event Chaining

Events can trigger other events, creating chains of updates:

```python
@layer.events.data.connect
def on_data_changed(event):
    # Data changed, may need to update other properties
    new_data = event.value
    if new_data.dtype != self._last_dtype:
        # Trigger colormap update
        self.update_colormap_for_dtype(new_data.dtype)
```

---

## 7. Summary

Napari's evented model architecture provides a robust solution for maintaining consistency between data and views:

### Key Principles

1. **Selective Mutability**: Layer data mutability varies by type (Image/Labels immutable, Points/Shapes mutable)
2. **Event-Driven Tracking**: All property changes emit events through EventedModel and EmitterGroup
3. **Automatic Synchronization**: Views and plugins subscribe to events and update automatically
4. **Silent Divergence Prevention**: The event system guarantees data and views never drift apart

### Guarantees

- **Change Notification**: Every property change emits an event
- **Consistency**: All views see the same data state
- **Atomicity**: Changes appear atomic to observers
- **Ordering**: Events are emitted in the order of property changes

### Best Practices

1. **Use `set_data()` for Image/Labels**: Replace data rather than modifying in-place
2. **Use Layer Methods for Points/Shapes**: Prefer `layer.add()` over direct array modification
3. **Subscribe to Events**: Views and plugins should subscribe to relevant events
4. **Never Maintain Copies**: Views should reference layer data, not copy it

This architecture elegantly solves the challenge of maintaining consistency in a multi-view, multi-plugin scientific visualization application by ensuring that all changes are tracked and propagated automatically.

