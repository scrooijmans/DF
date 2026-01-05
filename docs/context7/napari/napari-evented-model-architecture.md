# Napari Evented Model Architecture: Loose Coupling Through Events

## Executive Summary

This document provides a comprehensive technical analysis of napari's evented model architecture, explaining how changes to layers propagate to multiple UI components without tight coupling. The architecture uses an observer pattern with EventedModel, EmitterGroup, and event-driven communication to achieve modularity, testability, and maintainability.

**Key Insight**: Napari uses an event-driven architecture to decouple components:
- **EventedModel Base Class**: Provides automatic event emission and type validation
- **EmitterGroup/EventEmitter**: Manages event broadcasting to multiple observers
- **Observer Pattern**: UI components subscribe to events rather than directly accessing models
- **Separation of Concerns**: Models operate independently of UI, enabling headless operation and testing

---

## 1. Evented Model Architecture Overview

### 1.1 Core Principles

Napari's evented model architecture is built on several key principles:

1. **Models as Source of Truth**: Python models store application state independently of UI
2. **Event-Driven Communication**: Changes are communicated through events, not direct method calls
3. **Observer Pattern**: Components subscribe to events they care about
4. **Loose Coupling**: Components don't directly reference each other
5. **Separation of Concerns**: Models, UI, and rendering are separate layers

### 1.2 Architecture Layers

```
┌─────────────────────────────────────────────────────────┐
│              UI Layer (Qt Classes)                      │
│  - QtDims, QtLayerList, QtViewer, etc.                  │
│  - Subscribe to model events                            │
│  - Update UI in response to events                      │
└────────────────────┬────────────────────────────────────┘
                     │ Events (subscribe)
                     │
┌────────────────────┴────────────────────────────────────┐
│           Event System (EmitterGroup)                   │
│  - Manages event emitters                                │
│  - Broadcasts events to subscribers                      │
│  - Handles event connections/disconnections             │
└────────────────────┬────────────────────────────────────┘
                     │ Events (emit)
                     │
┌────────────────────┴────────────────────────────────────┐
│         Model Layer (EventedModel)                       │
│  - Dims, Layer, Viewer, etc.                            │
│  - Store application state                               │
│  - Emit events on state changes                          │
└─────────────────────────────────────────────────────────┘
```

### 1.3 Key Components

- **EventedModel**: Base class that provides automatic event emission
- **EmitterGroup**: Manages multiple event emitters for a model
- **EventEmitter**: Individual event emitter for a specific attribute
- **Qt Classes**: UI components that subscribe to model events
- **Vispy Classes**: Rendering components that subscribe to model events

---

## 2. EventedModel Base Class

### 2.1 Purpose and Features

The `EventedModel` base class provides a foundation for all napari models:

**Key Features**:
- **Automatic Event Emission**: Events emitted automatically when attributes change
- **Type Validation**: Uses Pydantic for type checking and coercion
- **Property Observability**: All property changes are observable
- **Consistent API**: Uniform event handling across all models

### 2.2 Basic Implementation

```python
from napari.utils.events import EventedModel
from pydantic import Field

class Dims(EventedModel):
    """Model for managing dimensions"""
    ndisplay: int = Field(default=2, ge=2, le=3)
    current_step: tuple = Field(default=(0,))
    
    # EventedModel automatically:
    # 1. Validates types on assignment
    # 2. Emits events when attributes change
    # 3. Provides events attribute for subscriptions
```

**What EventedModel Provides**:
- Type validation through Pydantic
- Automatic event emission on property changes
- `events` attribute containing EmitterGroup
- Consistent event API

### 2.3 Automatic Event Emission

When you inherit from `EventedModel`, property changes automatically emit events:

```python
# User code
dims = Dims()
dims.ndisplay = 3

# Internal flow:
# 1. Property setter called
# 2. Pydantic validates value (must be 2 or 3)
# 3. Value stored if valid
# 4. Event automatically emitted: dims.events.ndisplay(value=3)
# 5. All subscribers notified
```

**No Manual Event Emission Needed**:
- EventedModel handles event emission automatically
- Developers don't need to manually call `events.attribute(value)`
- Reduces boilerplate code
- Prevents forgetting to emit events

### 2.4 EventedModel vs Manual Event Handling

**Without EventedModel (Manual)**:
```python
class Dims:
    def __init__(self):
        self._ndisplay = 2
        self.events = EmitterGroup(source=self, ndisplay=None)
    
    @property
    def ndisplay(self):
        return self._ndisplay
    
    @ndisplay.setter
    def ndisplay(self, value):
        # Manual validation
        if not isinstance(value, int):
            raise TypeError("ndisplay must be int")
        if value not in [2, 3]:
            raise ValueError("ndisplay must be 2 or 3")
        
        # Manual event emission
        self._ndisplay = value
        self.events.ndisplay(value=value)  # Easy to forget!
```

**With EventedModel (Automatic)**:
```python
class Dims(EventedModel):
    ndisplay: int = Field(default=2, ge=2, le=3)
    # That's it! Event emission is automatic
```

---

## 3. EmitterGroup and EventEmitter

### 3.1 EmitterGroup

`EmitterGroup` manages multiple event emitters for a single model:

```python
from napari.utils.events import EmitterGroup

class Layer(EventedModel):
    # EventedModel automatically creates EmitterGroup
    # with emitters for each field
    
    # Equivalent to:
    # self.events = EmitterGroup(
    #     source=self,
    #     data=None,
    #     opacity=None,
    #     visible=None,
    #     ...
    # )
```

**EmitterGroup Responsibilities**:
- Manages multiple EventEmitter instances
- Provides access to individual emitters
- Handles event source tracking
- Manages connection/disconnection lifecycle

### 3.2 EventEmitter

Each `EventEmitter` handles events for a specific attribute:

```python
# Access individual emitter
layer.events.opacity  # EventEmitter for opacity attribute

# Connect callback
layer.events.opacity.connect(callback_function)

# Emit event (usually automatic via EventedModel)
layer.events.opacity(value=0.5)
```

**EventEmitter Features**:
- **Connect**: Subscribe callback functions to events
- **Disconnect**: Unsubscribe callbacks
- **Emit**: Broadcast event to all connected callbacks
- **Source Tracking**: Events include reference to source object

### 3.3 Event Object

Events carry information about the change:

```python
@layer.events.opacity.connect
def on_opacity_changed(event):
    """Callback receives event object"""
    print(f"Source: {event.source}")      # The layer object
    print(f"Value: {event.value}")        # New opacity value
    print(f"Type: {event.type}")         # Event type
    print(f"Old value: {event.old_value}") # Previous value (if available)
```

**Event Object Properties**:
- `source`: The object that emitted the event
- `value`: The new value
- `type`: The type of event
- `old_value`: Previous value (if tracked)

---

## 4. How Layer Changes Propagate

### 4.1 Complete Propagation Flow

When a layer property changes, the event propagates through the system:

```
┌─────────────────────────────────────────────────────────┐
│  Step 1: Property Change                                │
│  layer.opacity = 0.5                                    │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  Step 2: EventedModel Processing                        │
│  - Pydantic validates value                             │
│  - Value stored if valid                                │
│  - Event automatically emitted                           │
│  layer.events.opacity(value=0.5)                        │
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
│ QtLayer      │ │ VispyLayer   │ │ Plugin       │
│ (UI)         │ │ (Rendering)  │ │ (Analysis)   │
│              │ │              │ │              │
│ Updates      │ │ Updates      │ │ Updates      │
│ opacity      │ │ rendering    │ │ state        │
│ slider       │ │ with new     │ │              │
│              │ │ opacity      │ │              │
└──────────────┘ └──────────────┘ └──────────────┘
```

### 4.2 Multiple Subscribers

Multiple components can subscribe to the same event:

```python
# Layer property change
layer = viewer.add_image(image_data)

# Component 1: UI slider subscribes
@layer.events.opacity.connect
def update_opacity_slider(event):
    """Update UI slider to reflect new opacity"""
    opacity_slider.setValue(event.value)

# Component 2: Rendering engine subscribes
@layer.events.opacity.connect
def update_rendering(event):
    """Update rendering with new opacity"""
    vispy_layer.opacity = event.value
    canvas.update()

# Component 3: Plugin subscribes
@layer.events.opacity.connect
def log_opacity_change(event):
    """Log opacity changes for analysis"""
    logger.info(f"Opacity changed to {event.value}")

# All three components automatically updated when:
layer.opacity = 0.5
# ✅ Slider updates
# ✅ Rendering updates
# ✅ Plugin logs change
```

### 4.3 No Direct Dependencies

Components don't need to know about each other:

```python
# ❌ Tight Coupling (Bad)
class OpacitySlider:
    def __init__(self, layer, renderer, logger):
        self.layer = layer
        self.renderer = renderer  # Direct dependency
        self.logger = logger      # Direct dependency
    
    def on_slider_changed(self, value):
        self.layer.opacity = value
        self.renderer.update()    # Direct call
        self.logger.log(value)    # Direct call

# ✅ Loose Coupling (Good)
class OpacitySlider:
    def __init__(self, layer):
        self.layer = layer
        # No dependencies on renderer or logger
    
    def on_slider_changed(self, value):
        self.layer.opacity = value
        # Event automatically propagates to all subscribers
        # Slider doesn't need to know who else is listening
```

---

## 5. Qt Classes and Model Connection

### 5.1 Qt Class Pattern

Each Python model has a corresponding Qt class for UI representation:

```python
# Python Model
class Dims(EventedModel):
    ndisplay: int = 2

# Qt UI Class
class QtDims(QWidget):
    def __init__(self, dims: Dims):
        self.dims = dims
        
        # Connect to model events
        self.dims.events.ndisplay.connect(self._update_display)
        
        # Create UI elements
        self.combo_box = QComboBox()
        self.combo_box.addItems(['2D', '3D'])
        self.combo_box.currentIndexChanged.connect(self._on_combo_changed)
    
    def _update_display(self, event):
        """Called when model's ndisplay changes"""
        # Update UI to reflect model state
        if event.value == 2:
            self.combo_box.setCurrentIndex(0)
        else:
            self.combo_box.setCurrentIndex(1)
    
    def _on_combo_changed(self, index):
        """Called when user changes UI"""
        # Update model (triggers event)
        self.dims.ndisplay = 2 if index == 0 else 3
```

### 5.2 Bidirectional Updates

The connection enables bidirectional updates:

```
User changes UI → Model updates → Event emitted → UI updates (if needed)
Model changes → Event emitted → UI updates
```

**Example Flow**:
```python
# User drags slider
opacity_slider.setValue(0.5)
# → QtSlider emits signal
# → QtLayer._on_opacity_changed() called
# → layer.opacity = 0.5
# → EventedModel emits event
# → All subscribers notified (including slider itself)
# → Slider updates if needed (prevents feedback loops)
```

### 5.3 Preventing Feedback Loops

Qt classes must prevent infinite update loops:

```python
class QtLayer(QWidget):
    def __init__(self, layer: Layer):
        self.layer = layer
        self._updating = False  # Flag to prevent loops
        
        # Connect to model events
        self.layer.events.opacity.connect(self._on_model_opacity_changed)
        
        # Connect UI to model
        self.opacity_slider.valueChanged.connect(self._on_slider_changed)
    
    def _on_model_opacity_changed(self, event):
        """Called when model opacity changes"""
        if not self._updating:
            self._updating = True
            self.opacity_slider.setValue(event.value)
            self._updating = False
    
    def _on_slider_changed(self, value):
        """Called when user changes slider"""
        if not self._updating:
            self._updating = True
            self.layer.opacity = value  # Triggers model event
            self._updating = False
```

---

## 6. Observer Pattern Implementation

### 6.1 Observer Pattern in Napari

Napari implements the observer pattern through events:

**Traditional Observer Pattern**:
```python
# Subject (Observable)
class Subject:
    def __init__(self):
        self._observers = []
    
    def attach(self, observer):
        self._observers.append(observer)
    
    def notify(self):
        for observer in self._observers:
            observer.update()

# Observer
class Observer:
    def update(self):
        pass
```

**Napari's Event-Driven Pattern**:
```python
# Subject (EventedModel)
class Layer(EventedModel):
    opacity: float = 1.0
    # Automatically provides events.opacity emitter

# Observer (Subscriber)
def update_ui(event):
    """Observer callback"""
    ui_element.value = event.value

# Connection (Subscription)
layer.events.opacity.connect(update_ui)
```

### 6.2 Advantages of Event-Driven Approach

**Flexibility**:
- Multiple callbacks can subscribe to same event
- Callbacks can be added/removed dynamically
- No need to modify model to add new observers

**Decoupling**:
- Observers don't need to know about each other
- Model doesn't need to know about observers
- Easy to add new UI components or plugins

**Testability**:
- Models can be tested without UI
- UI can be tested with mock models
- Events can be tested independently

### 6.3 Multiple Observers Example

```python
# Layer property change
layer = viewer.add_image(image_data)

# Observer 1: UI Slider
@layer.events.opacity.connect
def update_slider(event):
    slider.setValue(event.value)

# Observer 2: Rendering Engine
@layer.events.opacity.connect
def update_rendering(event):
    vispy_layer.opacity = event.value
    canvas.update()

# Observer 3: Status Bar
@layer.events.opacity.connect
def update_status(event):
    status_bar.setText(f"Opacity: {event.value:.2f}")

# Observer 4: Plugin
@layer.events.opacity.connect
def plugin_handler(event):
    plugin_state['opacity'] = event.value

# All observers automatically notified when:
layer.opacity = 0.5

# None of these observers know about each other
# All are decoupled from the layer model
```

---

## 7. Loose Coupling Benefits

### 7.1 Modularity

Components can be developed and modified independently:

```python
# Add new UI component without modifying model
class NewOpacityWidget(QWidget):
    def __init__(self, layer: Layer):
        self.layer = layer
        # Subscribe to events
        self.layer.events.opacity.connect(self._update)
    
    def _update(self, event):
        # Update widget
        pass

# Model doesn't need to know about this widget
# Widget doesn't need to know about other UI components
```

### 7.2 Testability

Models can be tested without UI:

```python
# Test model in isolation
def test_layer_opacity():
    layer = Image(data=image_data)
    
    # Track events
    events_received = []
    @layer.events.opacity.connect
    def track_event(event):
        events_received.append(event.value)
    
    # Change opacity
    layer.opacity = 0.5
    
    # Verify event was emitted
    assert events_received == [0.5]
    assert layer.opacity == 0.5
```

### 7.3 Extensibility

Plugins can extend functionality without modifying core code:

```python
# Plugin subscribes to existing events
@napari_hook_implementation
def my_plugin(viewer: Viewer):
    # Subscribe to layer events
    @viewer.layers.events.inserted.connect
    def on_layer_added(event):
        new_layer = event.value
        # Plugin-specific handling
        setup_plugin_for_layer(new_layer)
    
    # No modification to napari core needed
    # Plugin works with existing event system
```

### 7.4 Headless Operation

Models can operate without UI:

```python
# Create viewer and layers without UI
viewer = napari.Viewer()  # Can run headless
layer = viewer.add_image(image_data)

# Process data programmatically
layer.opacity = 0.5
layer.visible = False

# Events still work, but no UI to update
# Useful for:
# - Automated processing
# - Testing
# - Server-side operations
```

---

## 8. Practical Examples

### 8.1 Complete Example: Layer Opacity Change

```python
import napari
from napari.layers import Image

# Step 1: Create viewer and layer
viewer = napari.Viewer()
layer = viewer.add_image(image_data)

# Step 2: UI components subscribe to events
# (This happens automatically in napari, but shown for clarity)

# QtLayer subscribes
@layer.events.opacity.connect
def qt_layer_update(event):
    """Qt UI updates opacity slider"""
    qt_opacity_slider.setValue(event.value)

# VispyLayer subscribes
@layer.events.opacity.connect
def vispy_layer_update(event):
    """Rendering engine updates opacity"""
    vispy_layer.opacity = event.value
    canvas.update()

# Plugin subscribes
@layer.events.opacity.connect
def plugin_update(event):
    """Plugin tracks opacity changes"""
    plugin_state['opacity'] = event.value

# Step 3: User changes opacity
layer.opacity = 0.5

# Step 4: Event propagation
# - EventedModel emits layer.events.opacity(value=0.5)
# - EmitterGroup broadcasts to all subscribers
# - QtLayer updates slider
# - VispyLayer updates rendering
# - Plugin updates state
# All happen automatically, no direct dependencies
```

### 8.2 Example: Adding New UI Component

```python
# New UI component can be added without modifying existing code
class CustomOpacityDisplay(QWidget):
    """Custom widget that displays opacity"""
    
    def __init__(self, layer: Layer):
        super().__init__()
        self.layer = layer
        
        # Subscribe to events
        self.layer.events.opacity.connect(self._update_display)
        
        # Create UI
        self.label = QLabel()
        self._update_display(None)  # Initial update
    
    def _update_display(self, event):
        """Update display when opacity changes"""
        opacity = self.layer.opacity if event is None else event.value
        self.label.setText(f"Opacity: {opacity:.1%}")
    
    # No need to:
    # - Modify Layer class
    # - Modify other UI components
    # - Know about other observers
    # Just subscribe to events and it works!
```

### 8.3 Example: Plugin Integration

```python
@napari_hook_implementation
def analysis_plugin(viewer: Viewer):
    """Plugin that analyzes layer changes"""
    
    # Subscribe to layer events
    @viewer.layers.events.inserted.connect
    def on_layer_added(event):
        layer = event.value
        
        # Subscribe to layer property events
        @layer.events.opacity.connect
        def on_opacity_changed(event):
            # Plugin-specific processing
            analyze_opacity_change(layer, event.value)
        
        @layer.events.data.connect
        def on_data_changed(event):
            # Plugin-specific processing
            analyze_data_change(layer, event.value)
    
    # Plugin works with existing event system
    # No modification to napari core needed
    # Completely decoupled from other components
```

### 8.4 Example: Testing Models

```python
def test_layer_events():
    """Test layer events without UI"""
    # Create layer (no UI needed)
    layer = Image(data=image_data)
    
    # Track events
    opacity_events = []
    data_events = []
    
    @layer.events.opacity.connect
    def track_opacity(event):
        opacity_events.append(event.value)
    
    @layer.events.data.connect
    def track_data(event):
        data_events.append(event.value.shape)
    
    # Make changes
    layer.opacity = 0.5
    layer.data = new_image_data
    
    # Verify events
    assert opacity_events == [0.5]
    assert data_events == [new_image_data.shape]
    
    # Model tested in isolation
    # No UI dependencies
    # Fast and reliable tests
```

---

## 9. Comparison: Tight vs Loose Coupling

### 9.1 Tight Coupling (Problematic)

```python
# ❌ Tight Coupling
class Layer:
    def __init__(self):
        self.opacity = 1.0
        self.slider = None      # Direct reference
        self.renderer = None    # Direct reference
        self.status_bar = None  # Direct reference
    
    def set_opacity(self, value):
        self.opacity = value
        # Must manually update all dependent components
        if self.slider:
            self.slider.setValue(value)
        if self.renderer:
            self.renderer.set_opacity(value)
        if self.status_bar:
            self.status_bar.setText(f"Opacity: {value}")

# Problems:
# - Layer must know about all UI components
# - Adding new component requires modifying Layer
# - Hard to test (need all components)
# - Hard to extend (tight dependencies)
```

### 9.2 Loose Coupling (Napari's Approach)

```python
# ✅ Loose Coupling
class Layer(EventedModel):
    opacity: float = 1.0
    # That's it! No references to UI components

# UI components subscribe to events
class QtLayer:
    def __init__(self, layer: Layer):
        self.layer = layer
        self.layer.events.opacity.connect(self._update_slider)
    
    def _update_slider(self, event):
        self.slider.setValue(event.value)

class VispyLayer:
    def __init__(self, layer: Layer):
        self.layer = layer
        self.layer.events.opacity.connect(self._update_rendering)
    
    def _update_rendering(self, event):
        self.vispy_layer.opacity = event.value
        self.canvas.update()

# Benefits:
# - Layer doesn't know about UI components
# - Adding new component doesn't require modifying Layer
# - Easy to test (components isolated)
# - Easy to extend (just subscribe to events)
```

---

## 10. Summary

Napari's evented model architecture provides a robust foundation for building maintainable, testable, and extensible applications:

### Key Components

1. **EventedModel**: Base class providing automatic event emission and type validation
2. **EmitterGroup**: Manages multiple event emitters for a model
3. **EventEmitter**: Individual emitter for specific attributes
4. **Observer Pattern**: Components subscribe to events rather than directly accessing models

### Architecture Benefits

- **Loose Coupling**: Components don't directly reference each other
- **Modularity**: Components can be developed and modified independently
- **Testability**: Models can be tested without UI dependencies
- **Extensibility**: Plugins can extend functionality without modifying core code
- **Headless Operation**: Models can operate without UI

### Event Propagation Flow

1. **Property Change**: `layer.opacity = 0.5`
2. **Event Emission**: `EventedModel` automatically emits `layer.events.opacity(value=0.5)`
3. **Broadcasting**: `EmitterGroup` broadcasts to all subscribers
4. **Observer Notification**: All connected callbacks are invoked
5. **UI Updates**: Each observer updates its component independently

This architecture ensures that changes to layers propagate efficiently to multiple UI components while maintaining loose coupling, enabling napari to be modular, testable, and extensible.

