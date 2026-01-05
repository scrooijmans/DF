# Napari Model-View Update Mechanism: Evented Models, Layer Events, and Viewer Refresh

## Executive Summary

This document provides a comprehensive technical analysis of napari's model-view update mechanism, explaining how evented models, layer events, viewer refresh, and canvas redraw work together to maintain synchronization between data models and UI components. It details where event emissions occur, how layer operations trigger redraws, and how multiple dock widgets stay synchronized through the event system.

**Key Insight**: Napari uses a sophisticated event-driven model-view architecture:
- **Evented Models**: Data models emit events on state changes
- **Event Propagation**: Events flow from models → viewer → canvas → UI components
- **Automatic Synchronization**: All components subscribe to events and update automatically
- **Dock Widget Coordination**: Multiple widgets stay synchronized through shared event subscriptions
- **Canvas Refresh**: Layer operations trigger canvas updates through event cascade

---

## 1. Evented Model Architecture

### 1.1 EventedModel Base Class

All napari models inherit from `EventedModel`, which provides automatic event emission:

```python
from napari.utils.events import EventedModel
from pydantic import Field

class Layer(EventedModel):
    """Base layer class with evented properties"""
    name: str = Field(default="Layer")
    opacity: float = Field(default=1.0, ge=0.0, le=1.0)
    visible: bool = Field(default=True)
    data: Any = None
    
    # EventedModel automatically:
    # 1. Creates EmitterGroup with events for each field
    # 2. Emits events when properties change
    # 3. Provides self.events attribute for subscriptions
```

**What EventedModel Provides**:
- Automatic event emission on property changes
- Type validation through Pydantic
- Consistent event API across all models
- Event source tracking

### 1.2 Event Emission Points

Events are emitted at specific points in the code:

#### 1.2.1 Property Setter Emission

```python
# When property is set, EventedModel automatically emits event
layer.opacity = 0.5
# Internal flow:
# 1. Property setter called
# 2. Pydantic validates value
# 3. Value stored: self._opacity = 0.5
# 4. Event emitted: self.events.opacity(value=0.5)
# 5. All subscribers notified
```

#### 1.2.2 Data Update Emission

```python
# When data is updated
layer.set_data(new_data)
# Internal flow:
# 1. set_data() method called
# 2. Data validated
# 3. Internal state updated: self._data = new_data
# 4. Event emitted: self.events.set_data(value=new_data)
# 5. Event emitted: self.events.data(value=new_data)
# 6. All subscribers notified
```

#### 1.2.3 LayerList Operations

```python
# When layer is added
viewer.layers.append(layer)
# Internal flow:
# 1. LayerList.append() called
# 2. Layer added to internal list
# 3. Event emitted: viewer.layers.events.inserted(layer, index)
# 4. All subscribers notified

# When layer is removed
viewer.layers.remove(layer)
# Internal flow:
# 1. LayerList.remove() called
# 2. Layer removed from internal list
# 3. Event emitted: viewer.layers.events.removed(layer, index)
# 4. All subscribers notified
```

---

## 2. Layer Events and Their Emission

### 2.1 Layer Event Types

Each layer emits various events for different property changes:

```python
# Data events
layer.events.data          # Emitted when layer.data changes
layer.events.set_data      # Emitted when set_data() is called
layer.events.refresh       # Emitted when visual refresh needed

# Visual property events
layer.events.opacity       # Emitted when opacity changes
layer.events.visible       # Emitted when visibility changes
layer.events.blending      # Emitted when blending mode changes
layer.events.colormap      # Emitted when colormap changes

# Transform events
layer.events.scale         # Emitted when scale changes
layer.events.translate     # Emitted when translation changes
layer.events.rotate        # Emitted when rotation changes
layer.events.shear         # Emitted when shear changes

# Metadata events
layer.events.name          # Emitted when name changes
layer.events.metadata      # Emitted when metadata changes
```

### 2.2 Event Emission Flow

Complete flow when a layer property changes:

```
┌─────────────────────────────────────────────────────────┐
│  Step 1: Property Change                                │
│  layer.opacity = 0.5                                    │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 2: EventedModel Property Setter                   │
│  - Pydantic validates value                              │
│  - Updates internal state: self._opacity = 0.5         │
│  - Emits event: self.events.opacity(value=0.5)          │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 3: EmitterGroup Broadcasting                      │
│  - EventEmitter for 'opacity' broadcasts                 │
│  - All connected callbacks notified                     │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Viewer       │ │ Canvas       │ │ Dock Widgets │
│ Updates      │ │ Updates      │ │ Update       │
│ State        │ │ Rendering    │ │ Displays     │
└──────────────┘ └──────────────┘ └──────────────┘
```

### 2.3 Event Emission Code Locations

**Layer Property Events**:
```python
# Location: napari/layers/base/base.py
class Layer(EventedModel):
    opacity: float = 1.0
    
    # EventedModel automatically creates:
    # @opacity.setter (implicit)
    # def opacity(self, value):
    #     self._opacity = value
    #     self.events.opacity(value=value)  # ← Event emission here
```

**Layer Data Events**:
```python
# Location: napari/layers/base/base.py
class Layer(EventedModel):
    def set_data(self, data):
        # Validate data
        self._data = data
        # Event emission
        self.events.set_data(value=data)  # ← Event emission here
        self.events.data(value=data)      # ← Event emission here
```

**LayerList Events**:
```python
# Location: napari/components/layerlist.py
class LayerList(EventedList):
    def insert(self, index, layer):
        super().insert(index, layer)
        # Event emission
        self.events.inserted(value=layer, index=index)  # ← Event emission here
    
    def remove(self, layer):
        index = self.index(layer)
        super().remove(layer)
        # Event emission
        self.events.removed(value=layer, index=index)  # ← Event emission here
```

---

## 3. Viewer Refresh Mechanism

### 3.1 Viewer Event Subscriptions

The viewer subscribes to layer events to coordinate updates:

```python
# Location: napari/components/viewer_model.py
class ViewerModel(EventedModel):
    def __init__(self):
        self.layers = LayerList()
        
        # Subscribe to layer list events
        @self.layers.events.inserted.connect
        def on_layer_added(event):
            self._on_layer_added(event.value)
        
        @self.layers.events.removed.connect
        def on_layer_removed(event):
            self._on_layer_removed(event.value)
        
        # Subscribe to individual layer events
        for layer in self.layers:
            self._subscribe_to_layer(layer)
    
    def _subscribe_to_layer(self, layer: Layer):
        """Subscribe to layer property events"""
        @layer.events.data.connect
        def on_layer_data_changed(event):
            self._on_layer_data_changed(layer, event.value)
        
        @layer.events.opacity.connect
        def on_layer_opacity_changed(event):
            self._on_layer_opacity_changed(layer, event.value)
        
        @layer.events.visible.connect
        def on_layer_visible_changed(event):
            self._on_layer_visible_changed(layer, event.value)
```

### 3.2 Viewer Update Handlers

Viewer handlers coordinate updates:

```python
class ViewerModel:
    def _on_layer_added(self, layer: Layer):
        """Handle layer addition"""
        # 1. Update internal state
        self._update_layer_order()
        
        # 2. Subscribe to new layer events
        self._subscribe_to_layer(layer)
        
        # 3. Mark canvas for update
        self._mark_canvas_dirty()
        
        # 4. Emit viewer-level events
        self.events.layers_changed()
    
    def _on_layer_data_changed(self, layer: Layer, new_data):
        """Handle layer data change"""
        # 1. Update layer bounds if needed
        self._update_layer_bounds(layer)
        
        # 2. Mark canvas for redraw
        self._mark_canvas_dirty()
        
        # 3. Update scene graph
        self._update_scene_graph()
    
    def _mark_canvas_dirty(self):
        """Mark canvas for redraw"""
        # Notify canvas to update
        if hasattr(self, '_canvas'):
            self._canvas.update()
```

### 3.3 Canvas Update Trigger

Canvas updates are triggered through viewer events:

```python
# Location: napari/_qt/qt_viewer.py
class QtViewer:
    def __init__(self, viewer: ViewerModel):
        self.viewer = viewer
        self.canvas = Canvas()
        
        # Subscribe to viewer events
        @viewer.layers.events.inserted.connect
        def on_layer_added(event):
            self._add_layer_to_canvas(event.value)
            self.canvas.update()  # ← Canvas update triggered
        
        @viewer.layers.events.removed.connect
        def on_layer_removed(event):
            self._remove_layer_from_canvas(event.value)
            self.canvas.update()  # ← Canvas update triggered
        
        # Subscribe to layer property events
        for layer in viewer.layers:
            self._subscribe_to_layer_rendering(layer)
    
    def _subscribe_to_layer_rendering(self, layer: Layer):
        """Subscribe to rendering-related events"""
        @layer.events.data.connect
        def on_data_changed(event):
            self._update_layer_rendering(layer)
            self.canvas.update()  # ← Canvas update triggered
        
        @layer.events.opacity.connect
        def on_opacity_changed(event):
            self._update_layer_opacity(layer)
            self.canvas.update()  # ← Canvas update triggered
        
        @layer.events.visible.connect
        def on_visible_changed(event):
            self._update_layer_visibility(layer)
            self.canvas.update()  # ← Canvas update triggered
```

---

## 4. Layer Operations and Redraw Triggers

### 4.1 Adding a Layer

Complete flow when a layer is added:

```
┌─────────────────────────────────────────────────────────┐
│  Step 1: Layer Creation                                  │
│  layer = viewer.add_image(data)                         │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 2: LayerList.append()                              │
│  viewer.layers.append(layer)                             │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 3: Event Emission                                  │
│  viewer.layers.events.inserted(layer, index)            │
│  ← Event emission point                                 │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Viewer       │ │ QtViewer     │ │ Dock Widgets│
│ _on_layer_   │ │ on_layer_    │ │ on_layer_   │
│ added()      │ │ added()      │ │ added()     │
└──────┬───────┘ └──────┬───────┘ └──────┬───────┘
       │                │                │
       │                ▼                │
       │    ┌───────────────────────┐    │
       │    │ Canvas Update          │    │
       │    │ - Add layer to scene   │    │
       │    │ - Update scene graph   │    │
       │    │ - Mark for redraw      │    │
       │    └───────────┬────────────┘    │
       │                │                 │
       │                ▼                 │
       │    ┌───────────────────────┐     │
       │    │ Canvas Redraw          │     │
       │    │ - Rebuild render tree  │     │
       │    │ - Generate GPU commands│     │
       │    │ - Display updated view │     │
       │    └───────────────────────┘     │
       │                                   │
       └───────────────────────────────────┘
```

**Code Flow**:
```python
# User code
layer = viewer.add_image(data)

# Internal flow:
# 1. viewer.add_image() creates Image layer
# 2. viewer.layers.append(layer)
# 3. LayerList.insert() called
# 4. Event emitted: layers.events.inserted(layer, index)
# 5. Viewer._on_layer_added() called
# 6. QtViewer.on_layer_added() called
# 7. Canvas.add_layer() called
# 8. Canvas.update() called
# 9. Canvas redraws
```

### 4.2 Updating a Layer

Complete flow when a layer is updated:

```
┌─────────────────────────────────────────────────────────┐
│  Step 1: Layer Update                                   │
│  layer.set_data(new_data) or layer.opacity = 0.5      │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 2: Property Setter                                │
│  - Validates value                                       │
│  - Updates internal state                                │
│  - Emits event: layer.events.{property}(value)          │
│  ← Event emission point                                 │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Viewer       │ │ QtViewer     │ │ Dock Widgets │
│ _on_layer_   │ │ on_layer_    │ │ on_layer_   │
│ changed()    │ │ changed()    │ │ changed()   │
└──────┬───────┘ └──────┬───────┘ └──────┬───────┘
       │                │                │
       │                ▼                │
       │    ┌───────────────────────┐     │
       │    │ Canvas Update          │     │
       │    │ - Update layer data    │     │
       │    │ - Update rendering     │     │
       │    │ - Mark for redraw      │     │
       │    └───────────┬────────────┘     │
       │                │                 │
       │                ▼                 │
       │    ┌───────────────────────┐     │
       │    │ Canvas Redraw          │     │
       │    │ - Update render tree   │     │
       │    │ - Re-render layer      │     │
       │    │ - Display updated view │     │
       │    └───────────────────────┘     │
       │                                   │
       └───────────────────────────────────┘
```

**Code Flow**:
```python
# User code
layer.opacity = 0.5

# Internal flow:
# 1. Property setter called
# 2. EventedModel validates and stores value
# 3. Event emitted: layer.events.opacity(value=0.5)
# 4. Viewer._on_layer_opacity_changed() called
# 5. QtViewer.on_layer_opacity_changed() called
# 6. Canvas.update_layer_opacity() called
# 7. Canvas.update() called
# 8. Canvas redraws
```

### 4.3 Removing a Layer

Complete flow when a layer is removed:

```
┌─────────────────────────────────────────────────────────┐
│  Step 1: Layer Removal                                  │
│  viewer.layers.remove(layer)                            │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 2: LayerList.remove()                             │
│  - Removes layer from list                              │
│  - Emits event: layers.events.removed(layer, index)     │
│  ← Event emission point                                 │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Viewer       │ │ QtViewer     │ │ Dock Widgets │
│ _on_layer_   │ │ on_layer_    │ │ on_layer_   │
│ removed()    │ │ removed()    │ │ removed()   │
└──────┬───────┘ └──────┬───────┘ └──────┬───────┘
       │                │                │
       │                ▼                │
       │    ┌───────────────────────┐     │
       │    │ Canvas Update          │     │
       │    │ - Remove layer from    │     │
       │    │   scene                │     │
       │    │ - Update scene graph   │     │
       │    │ - Mark for redraw      │     │
       │    └───────────┬────────────┘     │
       │                │                 │
       │                ▼                 │
       │    ┌───────────────────────┐     │
       │    │ Canvas Redraw          │     │
       │    │ - Rebuild render tree  │     │
       │    │ - Re-render scene      │     │
       │    │ - Display updated view │     │
       │    └───────────────────────┘     │
       │                                   │
       └───────────────────────────────────┘
```

**Code Flow**:
```python
# User code
viewer.layers.remove(layer)

# Internal flow:
# 1. LayerList.remove() called
# 2. Layer removed from internal list
# 3. Event emitted: layers.events.removed(layer, index)
# 4. Viewer._on_layer_removed() called
# 5. QtViewer.on_layer_removed() called
# 6. Canvas.remove_layer() called
# 7. Canvas.update() called
# 8. Canvas redraws
```

---

## 5. Canvas Redraw Mechanism

### 5.1 Canvas Update Cycle

The canvas update cycle coordinates rendering:

```python
# Location: napari/_vispy/vispy_canvas.py
class VispyCanvas:
    def __init__(self, viewer: ViewerModel):
        self.viewer = viewer
        self.scene = Scene()
        self._dirty = False  # Dirty flag
        
        # Subscribe to viewer events
        @viewer.layers.events.inserted.connect
        def on_layer_added(event):
            self._add_layer_to_scene(event.value)
            self.update()  # ← Mark for redraw
        
        @viewer.layers.events.removed.connect
        def on_layer_removed(event):
            self._remove_layer_from_scene(event.value)
            self.update()  # ← Mark for redraw
    
    def update(self):
        """Mark canvas for redraw"""
        self._dirty = True
        # Schedule redraw in next event loop iteration
        QTimer.singleShot(0, self._redraw)
    
    def _redraw(self):
        """Perform actual redraw"""
        if not self._dirty:
            return
        
        # 1. Rebuild scene graph
        self._rebuild_scene_graph()
        
        # 2. Update render tree
        self._update_render_tree()
        
        # 3. Generate GPU commands
        self._generate_gpu_commands()
        
        # 4. Display
        self._display()
        
        self._dirty = False
```

### 5.2 Scene Graph Updates

Scene graph is updated when layers change:

```python
class VispyCanvas:
    def _rebuild_scene_graph(self):
        """Rebuild scene graph from layers"""
        # Clear existing scene
        self.scene.clear()
        
        # Add layers in order
        for layer in self.viewer.layers:
            if layer.visible:
                vispy_node = self._create_vispy_node(layer)
                self.scene.add(vispy_node)
    
    def _create_vispy_node(self, layer: Layer):
        """Create vispy node for layer"""
        if isinstance(layer, Image):
            return VispyImageNode(layer)
        elif isinstance(layer, Labels):
            return VispyLabelsNode(layer)
        # ... other layer types
```

### 5.3 Render Tree Updates

Render tree is updated for efficient rendering:

```python
class VispyCanvas:
    def _update_render_tree(self):
        """Update render tree"""
        # Update each layer's rendering
        for layer in self.viewer.layers:
            if layer.visible:
                vispy_node = self._get_vispy_node(layer)
                # Update node properties
                vispy_node.opacity = layer.opacity
                vispy_node.data = layer.data
                vispy_node.transform = layer.transform_matrix
```

---

## 6. Dock Widget Synchronization

### 6.1 Dock Widget Event Subscriptions

Dock widgets subscribe to viewer and layer events:

```python
# Example: Dock widget that displays layer properties
class LayerPropertiesWidget(QWidget):
    """Dock widget showing layer properties"""
    
    def __init__(self, viewer: ViewerModel):
        super().__init__()
        self.viewer = viewer
        
        # Subscribe to layer selection
        @viewer.layers.selection.events.active.connect
        def on_active_layer_changed(event):
            self._update_for_layer(event.value)
        
        # Subscribe to layer list changes
        @viewer.layers.events.inserted.connect
        def on_layer_added(event):
            self._refresh_layer_list()
        
        @viewer.layers.events.removed.connect
        def on_layer_removed(event):
            self._refresh_layer_list()
        
        # Subscribe to property changes for all layers
        for layer in viewer.layers:
            self._subscribe_to_layer_properties(layer)
    
    def _subscribe_to_layer_properties(self, layer: Layer):
        """Subscribe to layer property events"""
        @layer.events.opacity.connect
        def on_opacity_changed(event):
            if layer == self.viewer.layers.selection.active:
                self._update_opacity_display(event.value)
        
        @layer.events.visible.connect
        def on_visible_changed(event):
            if layer == self.viewer.layers.selection.active:
                self._update_visibility_display(event.value)
        
        @layer.events.name.connect
        def on_name_changed(event):
            self._refresh_layer_list()
```

### 6.2 Multiple Dock Widgets

Multiple dock widgets stay synchronized through shared events:

```python
# Widget 1: Layer List Widget
class LayerListWidget(QWidget):
    def __init__(self, viewer: ViewerModel):
        self.viewer = viewer
        
        # Subscribe to layer list events
        @viewer.layers.events.inserted.connect
        def on_layer_added(event):
            self._add_layer_item(event.value)
        
        @viewer.layers.events.removed.connect
        def on_layer_removed(event):
            self._remove_layer_item(event.index)

# Widget 2: Property Panel Widget
class PropertyPanelWidget(QWidget):
    def __init__(self, viewer: ViewerModel):
        self.viewer = viewer
        
        # Subscribe to selection changes
        @viewer.layers.selection.events.active.connect
        def on_active_layer_changed(event):
            self._update_for_layer(event.value)
        
        # Subscribe to property changes
        for layer in viewer.layers:
            @layer.events.opacity.connect
            def on_opacity_changed(event):
                if layer == viewer.layers.selection.active:
                    self._update_opacity_slider(event.value)

# Widget 3: Histogram Widget
class HistogramWidget(QWidget):
    def __init__(self, viewer: ViewerModel):
        self.viewer = viewer
        
        # Subscribe to data changes
        @viewer.layers.selection.events.active.connect
        def on_active_layer_changed(event):
            self._update_histogram(event.value)
        
        # Subscribe to data changes for active layer
        for layer in viewer.layers:
            @layer.events.data.connect
            def on_data_changed(event):
                if layer == viewer.layers.selection.active:
                    self._update_histogram(layer)

# All three widgets:
# - Subscribe to the same viewer events
# - Receive the same event notifications
# - Update independently but stay synchronized
# - No direct dependencies between widgets
```

### 6.3 Synchronization Flow

How multiple widgets stay synchronized:

```
┌─────────────────────────────────────────────────────────┐
│  Layer Property Change                                  │
│  layer.opacity = 0.5                                   │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Event Emission                                         │
│  layer.events.opacity(value=0.5)                        │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Widget 1     │ │ Widget 2     │ │ Widget 3     │
│ (Layer List) │ │ (Properties)  │ │ (Histogram)  │
│              │ │              │ │              │
│ Receives     │ │ Receives     │ │ Receives     │
│ Event        │ │ Event        │ │ Event        │
└──────┬───────┘ └──────┬───────┘ └──────┬───────┘
       │                │                │
       ▼                ▼                ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Updates      │ │ Updates      │ │ Updates      │
│ Layer List   │ │ Opacity      │ │ Histogram    │
│ Display      │ │ Slider       │ │ (if active)  │
└──────────────┘ └──────────────┘ └──────────────┘

All widgets update independently but stay synchronized
through shared event subscriptions
```

### 6.4 Preventing Race Conditions

Napari prevents race conditions through event ordering:

```python
# Events are emitted synchronously in order
layer.opacity = 0.5
# 1. Property setter executes
# 2. Event emitted
# 3. All subscribers notified (in order)
# 4. Each subscriber updates
# 5. Next event can be emitted

# No race conditions because:
# - Events are processed synchronously
# - Each subscriber completes before next
# - State changes are atomic
```

---

## 7. Complete Update Cycle Example

### 7.1 Adding a Layer

Complete cycle from user action to UI update:

```python
# User action
layer = viewer.add_image(image_data)

# Step 1: Layer creation
# Location: napari/viewer.py
def add_image(self, data, ...):
    layer = Image(data, ...)
    self.layers.append(layer)  # ← Triggers LayerList operation
    return layer

# Step 2: LayerList.append()
# Location: napari/components/layerlist.py
def append(self, layer):
    self.insert(len(self), layer)

def insert(self, index, layer):
    super().insert(index, layer)
    # ← Event emission point
    self.events.inserted(value=layer, index=index)

# Step 3: Event propagation
# All subscribers receive events.inserted:
# - ViewerModel._on_layer_added()
# - QtViewer.on_layer_added()
# - LayerListWidget.on_layer_added()
# - PropertyPanelWidget.on_layer_added()
# - HistogramWidget.on_layer_added()
# - Canvas.on_layer_added()

# Step 4: Viewer updates
# Location: napari/components/viewer_model.py
def _on_layer_added(self, layer):
    self._update_layer_order()
    self._subscribe_to_layer(layer)
    self._mark_canvas_dirty()

# Step 5: Canvas updates
# Location: napari/_qt/qt_viewer.py
def on_layer_added(self, event):
    self._add_layer_to_canvas(event.value)
    self.canvas.update()  # ← Triggers redraw

# Step 6: Canvas redraw
# Location: napari/_vispy/vispy_canvas.py
def update(self):
    self._dirty = True
    QTimer.singleShot(0, self._redraw)

def _redraw(self):
    self._rebuild_scene_graph()
    self._update_render_tree()
    self._generate_gpu_commands()
    self._display()

# Step 7: UI updates
# - Layer list widget adds layer item
# - Property panel updates for new layer
# - Canvas displays new layer
# - All widgets synchronized
```

### 7.2 Updating Layer Property

Complete cycle for property update:

```python
# User action
layer.opacity = 0.5

# Step 1: Property setter
# Location: napari/utils/events/evented_model.py
# EventedModel automatically:
# - Validates value
# - Stores value: self._opacity = 0.5
# - Emits event: self.events.opacity(value=0.5)
# ← Event emission point

# Step 2: Event propagation
# All subscribers receive events.opacity:
# - ViewerModel._on_layer_opacity_changed()
# - QtViewer.on_layer_opacity_changed()
# - LayerListWidget.on_opacity_changed()
# - PropertyPanelWidget.on_opacity_changed()
# - Canvas.on_opacity_changed()

# Step 3: Viewer updates
# Location: napari/components/viewer_model.py
def _on_layer_opacity_changed(self, layer, opacity):
    self._update_blending_calculations()
    self._mark_canvas_dirty()

# Step 4: Canvas updates
# Location: napari/_qt/qt_viewer.py
def on_layer_opacity_changed(self, event):
    self._update_layer_opacity(event.source, event.value)
    self.canvas.update()  # ← Triggers redraw

# Step 5: Canvas redraw
# Location: napari/_vispy/vispy_canvas.py
def _redraw(self):
    # Update opacity in render tree
    vispy_node.opacity = layer.opacity
    # Re-render
    self._display()

# Step 6: UI updates
# - Property panel updates opacity slider
# - Canvas re-renders with new opacity
# - All widgets synchronized
```

---

## 8. Event Emission Points Summary

### 8.1 Layer Property Events

| Property | Event | Emission Point |
|----------|-------|----------------|
| `opacity` | `layer.events.opacity` | Property setter (EventedModel) |
| `visible` | `layer.events.visible` | Property setter (EventedModel) |
| `name` | `layer.events.name` | Property setter (EventedModel) |
| `data` | `layer.events.data` | `set_data()` method |
| `data` | `layer.events.set_data` | `set_data()` method |
| `scale` | `layer.events.scale` | Property setter (EventedModel) |
| `translate` | `layer.events.translate` | Property setter (EventedModel) |

### 8.2 LayerList Events

| Operation | Event | Emission Point |
|-----------|-------|----------------|
| Add layer | `layers.events.inserted` | `LayerList.insert()` |
| Remove layer | `layers.events.removed` | `LayerList.remove()` |
| Reorder | `layers.events.reordered` | `LayerList.move()` |

### 8.3 Viewer Events

| Change | Event | Emission Point |
|--------|-------|----------------|
| Layer added | `viewer.events.layers_changed` | `ViewerModel._on_layer_added()` |
| Layer removed | `viewer.events.layers_changed` | `ViewerModel._on_layer_removed()` |
| Selection changed | `layers.selection.events.active` | `LayerList.selection.active` setter |

---

## 9. Summary

Napari's model-view update mechanism provides a robust, event-driven architecture:

### Key Components

1. **Evented Models**: All models emit events on state changes
2. **Event Propagation**: Events flow from models → viewer → canvas → UI
3. **Automatic Synchronization**: Components subscribe to events and update automatically
4. **Canvas Refresh**: Layer operations trigger canvas updates through event cascade

### Event Emission Points

- **Property Setters**: EventedModel automatically emits events
- **Layer Methods**: `set_data()` and other methods emit events
- **LayerList Operations**: `insert()`, `remove()` emit events
- **Viewer Handlers**: Viewer emits events for coordination

### Update Flow

1. **User Action**: Property change or layer operation
2. **Event Emission**: Model emits appropriate event
3. **Event Propagation**: All subscribers notified
4. **Viewer Update**: Viewer coordinates updates
5. **Canvas Update**: Canvas marks for redraw
6. **UI Update**: All widgets update independently
7. **Rendering**: Canvas redraws with new state

### Dock Widget Synchronization

- **Shared Events**: All widgets subscribe to same events
- **Independent Updates**: Each widget updates independently
- **No Direct Dependencies**: Widgets don't reference each other
- **Consistent State**: All widgets see same state through events

This architecture ensures that all components stay synchronized while maintaining loose coupling and modularity.

