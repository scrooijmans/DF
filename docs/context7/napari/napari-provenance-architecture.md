# Napari Provenance: Limitations and Architectural Decisions

## Executive Summary

This document provides a comprehensive analysis of how napari handles provenance (data lineage and transformation history), its current limitations, and the architectural decisions that make provenance tracking easier or harder to implement and expose to users. While napari does not natively provide comprehensive provenance tracking, its event-driven, plugin-based architecture offers both opportunities and challenges for implementing provenance systems.

**Key Insight**: Napari's architecture has mixed implications for provenance:
- **Event-Driven Model**: Provides hooks for tracking changes, but events don't inherently capture history
- **Plugin Architecture**: Enables provenance plugins, but lacks standardization
- **Layer-Based Design**: Makes layer-level tracking possible, but composite operations are harder
- **In-Memory Operations**: Fast but makes persistent history tracking challenging
- **No Built-in History**: No undo/redo or operation history by default

---

## 1. Current State of Provenance in Napari

### 1.1 Native Provenance Support

**Current Status**: Napari does **not** natively provide comprehensive provenance tracking.

**What's Missing**:
- No automatic recording of data transformations
- No operation history or audit log
- No undo/redo system
- No workflow documentation
- No parameter tracking for operations
- No lineage tracking between data versions

**What Users Must Do**:
- Manually document workflows
- Use external tools for provenance
- Develop custom scripts to log operations
- Rely on plugins for specific provenance needs

### 1.2 What Provenance Would Include

If napari had comprehensive provenance tracking, it would record:

1. **Data Lineage**:
   - Source of each layer (file, plugin output, user input)
   - Transformations applied to data
   - Dependencies between layers

2. **Operation History**:
   - Sequence of operations performed
   - Parameters used for each operation
   - Timestamps of operations
   - User who performed operations

3. **Data Versions**:
   - Historical states of data
   - Ability to revert to previous versions
   - Comparison between versions

4. **Workflow Documentation**:
   - Reproducible workflow scripts
   - Exportable analysis pipelines
   - Shareable provenance records

---

## 2. Architectural Factors Affecting Provenance

### 2.1 Event-Driven Architecture

**How It Helps**:
- Events provide hooks for tracking changes
- Can intercept layer modifications
- Enables reactive provenance tracking

**How It Hinders**:
- Events are ephemeral (fire and forget)
- No built-in history of events
- Multiple simultaneous events complicate linear history
- Event order may not reflect logical operation order

**Example**:
```python
# Event-driven change
layer.opacity = 0.5
# ✅ Event emitted: layer.events.opacity(value=0.5)
# ❌ No automatic recording of:
#    - Previous value
#    - Timestamp
#    - User who made change
#    - Reason for change
```

**Architectural Impact**:
- Events could be logged for provenance, but this isn't built-in
- Would need to wrap event system to capture history
- Event listeners could record provenance, but requires custom implementation

### 2.2 Plugin-Based Architecture

**How It Helps**:
- Plugins can implement provenance tracking
- Modular design allows provenance plugins
- Extensible without modifying core

**How It Hinders**:
- No standardized provenance API
- Each plugin handles provenance differently
- Difficult to aggregate provenance across plugins
- Plugins may not report their operations

**Example**:
```python
# Plugin performs operation
@napari_hook_implementation
def filter_plugin(viewer: Viewer):
    layer = viewer.layers[0]
    filtered = apply_filter(layer.data)
    layer.set_data(filtered)
    # ❌ No automatic provenance recording:
    #    - What filter was applied?
    #    - What parameters were used?
    #    - What was the input data?
    #    - When was it applied?
```

**Architectural Impact**:
- Plugins could emit provenance events, but no standard exists
- Centralized provenance system would need plugin cooperation
- Plugin ecosystem fragmentation makes standardization difficult

### 2.3 Layer-Based Data Model

**How It Helps**:
- Each layer is a discrete unit that can be tracked
- Layer creation/modification events are observable
- Layer metadata could store provenance

**How It Hinders**:
- Composite operations across layers are harder to track
- Layer relationships (dependencies) not explicitly modeled
- In-place modifications don't create new layers
- Layer deletion loses provenance

**Example**:
```python
# Layer creation - could track provenance
layer1 = viewer.add_image(image_data)
# ✅ Could record: source file, timestamp, user

# Layer modification - harder to track
layer1.data = processed_data
# ❌ No automatic recording of:
#    - What processing was applied?
#    - What was the previous state?
#    - What parameters were used?

# Composite operation - very hard to track
result = combine_layers(layer1, layer2, layer3)
# ❌ No automatic recording of:
#    - Which layers were combined?
#    - How were they combined?
#    - What was the operation?
```

**Architectural Impact**:
- Layer-level tracking is feasible
- Operation-level tracking requires additional infrastructure
- Cross-layer operations need explicit provenance support

### 2.4 In-Memory Data Operations

**How It Helps**:
- Fast operations
- Real-time updates
- Interactive workflows

**How It Hinders**:
- No persistent history by default
- Data modifications overwrite previous state
- No automatic versioning
- Memory constraints limit history retention

**Example**:
```python
# In-place modification
layer.data[100, 100] = 255
# ❌ Previous value lost
# ❌ No history of change
# ❌ Cannot undo

# Data replacement
layer.set_data(new_data)
# ❌ Previous data lost (unless explicitly saved)
# ❌ No version history
```

**Architectural Impact**:
- Would need copy-on-write or versioning system
- History storage could be memory-intensive
- Would require explicit save points or checkpoints

### 2.5 No Built-in Undo/Redo

**Current State**: Napari does not provide undo/redo functionality.

**Impact on Provenance**:
- No operation history maintained
- No way to revert changes
- No audit trail of modifications
- Users must manually track changes

**What Would Be Needed**:
- Command pattern for operations
- History stack for undo/redo
- State snapshots or diffs
- UI for navigating history

---

## 3. Architectural Decisions That Help Provenance

### 3.1 EventedModel Architecture

**Benefit**: Events provide hooks for provenance tracking.

**How It Could Be Leveraged**:
```python
# Provenance plugin could subscribe to all events
class ProvenanceTracker:
    def __init__(self, viewer: Viewer):
        self.viewer = viewer
        self.history = []
        
        # Subscribe to layer events
        @viewer.layers.events.inserted.connect
        def on_layer_added(event):
            self.record('layer_added', {
                'layer': event.value,
                'timestamp': datetime.now(),
                'source': 'user'
            })
        
        @viewer.layers.events.removed.connect
        def on_layer_removed(event):
            self.record('layer_removed', {
                'layer': event.value,
                'timestamp': datetime.now()
            })
        
        # Subscribe to property changes
        for layer in viewer.layers:
            self._track_layer(layer)
    
    def _track_layer(self, layer):
        @layer.events.data.connect
        def on_data_changed(event):
            self.record('data_changed', {
                'layer': layer,
                'old_shape': self._get_previous_shape(layer),
                'new_shape': event.value.shape,
                'timestamp': datetime.now()
            })
```

**Architectural Advantage**:
- Event system is already in place
- Can be extended without modifying core
- Non-invasive (doesn't break existing code)

### 3.2 Separation of Models and Views

**Benefit**: Models operate independently of UI, enabling headless provenance tracking.

**How It Could Be Leveraged**:
```python
# Provenance can be tracked at model level
class ProvenanceLayer(Layer):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.provenance = ProvenanceRecord()
    
    def set_data(self, data, **kwargs):
        # Record provenance before change
        self.provenance.record_operation(
            operation='set_data',
            previous_data=self.data,
            new_data=data,
            parameters=kwargs,
            timestamp=datetime.now()
        )
        super().set_data(data, **kwargs)
```

**Architectural Advantage**:
- Models can be extended with provenance
- Works in headless mode
- UI doesn't need to know about provenance

### 3.3 Plugin System

**Benefit**: Plugins can implement provenance without modifying core.

**How It Could Be Leveraged**:
```python
# Provenance plugin
@napari_hook_implementation
def provenance_plugin(viewer: Viewer):
    """Plugin that tracks all operations"""
    tracker = ProvenanceTracker(viewer)
    
    # Wrap viewer methods to track operations
    original_add_image = viewer.add_image
    def tracked_add_image(data, **kwargs):
        layer = original_add_image(data, **kwargs)
        tracker.record('add_image', {
            'layer': layer,
            'data_shape': data.shape,
            'parameters': kwargs
        })
        return layer
    
    viewer.add_image = tracked_add_image
```

**Architectural Advantage**:
- Can be added as plugin
- Doesn't require core changes
- Can be optional (users opt-in)

---

## 4. Architectural Decisions That Hinder Provenance

### 4.1 No Operation Abstraction

**Problem**: Operations are not abstracted as first-class objects.

**Current State**:
```python
# Direct property assignment
layer.opacity = 0.5
# No operation object
# No way to undo
# No way to record parameters
```

**What Would Help**:
```python
# Operation abstraction (not in napari)
operation = SetOpacityOperation(layer, 0.5)
viewer.execute(operation)
# ✅ Operation can be:
#    - Recorded in history
#    - Undone
#    - Replayed
#    - Documented
```

**Architectural Impact**:
- Would require command pattern
- Significant architectural change
- Would need to wrap all operations

### 4.2 In-Place Modifications

**Problem**: Direct data modifications don't create history.

**Current State**:
```python
# In-place modification
layer.data[100, 100] = 255
# ❌ No record of:
#    - Previous value
#    - Operation performed
#    - Cannot undo
```

**What Would Help**:
```python
# Copy-on-write (not in napari)
layer.data[100, 100] = 255
# ✅ Would:
#    - Create new version
#    - Record previous value
#    - Enable undo
```

**Architectural Impact**:
- Would require versioning system
- Memory overhead
- Performance implications

### 4.3 No Standardized Plugin Interface for Provenance

**Problem**: Plugins don't have standard way to report operations.

**Current State**:
```python
# Plugin performs operation
@napari_hook_implementation
def my_plugin(viewer: Viewer):
    layer = viewer.layers[0]
    result = process(layer.data)
    layer.set_data(result)
    # ❌ No standard way to report:
    #    - What operation was performed
    #    - What parameters were used
    #    - What the input was
```

**What Would Help**:
```python
# Standardized provenance interface (not in napari)
@napari_hook_implementation
def my_plugin(viewer: Viewer):
    layer = viewer.layers[0]
    result = process(layer.data, param1=10, param2=20)
    
    # Report to provenance system
    viewer.provenance.record_operation(
        plugin='my_plugin',
        operation='process',
        input_layer=layer,
        output_data=result,
        parameters={'param1': 10, 'param2': 20}
    )
    
    layer.set_data(result)
```

**Architectural Impact**:
- Would require provenance API
- Plugins would need to adopt it
- Backward compatibility concerns

### 4.4 No Persistent State Management

**Problem**: No built-in way to save/load application state with history.

**Current State**:
```python
# No state persistence
viewer = napari.Viewer()
# ... perform operations ...
# ❌ No way to save:
#    - Layer history
#    - Operation sequence
#    - Provenance record
```

**What Would Help**:
```python
# State persistence (not in napari)
viewer.save_state('session.napari')
# ✅ Would save:
#    - All layers
#    - Operation history
#    - Provenance record
#    - UI state
```

**Architectural Impact**:
- Would require serialization system
- File format design
- Version compatibility

---

## 5. Provenance Implementation Strategies

### 5.1 Event-Based Provenance Tracking

**Approach**: Subscribe to events and record them.

**Implementation**:
```python
class EventBasedProvenance:
    def __init__(self, viewer: Viewer):
        self.viewer = viewer
        self.history = []
        
        # Track all layer events
        @viewer.layers.events.inserted.connect
        def on_insert(event):
            self.record('layer_inserted', {
                'layer': event.value,
                'index': event.index,
                'timestamp': datetime.now()
            })
        
        # Track property changes
        for layer in viewer.layers:
            self._track_layer_properties(layer)
    
    def _track_layer_properties(self, layer):
        @layer.events.opacity.connect
        def on_opacity(event):
            self.record('opacity_changed', {
                'layer': layer,
                'old_value': self._get_previous_value(layer, 'opacity'),
                'new_value': event.value,
                'timestamp': datetime.now()
            })
```

**Advantages**:
- Non-invasive
- Works with existing architecture
- Can be added as plugin

**Limitations**:
- Only tracks what emits events
- May miss in-place modifications
- Events don't capture operation context

### 5.2 Wrapper-Based Provenance

**Approach**: Wrap layer methods to track operations.

**Implementation**:
```python
class ProvenanceLayerWrapper:
    def __init__(self, layer: Layer, tracker):
        self._layer = layer
        self._tracker = tracker
    
    def set_data(self, data, **kwargs):
        # Record before change
        self._tracker.record('set_data', {
            'layer': self._layer,
            'previous_data': self._layer.data,
            'new_data': data,
            'parameters': kwargs,
            'timestamp': datetime.now()
        })
        
        # Perform operation
        self._layer.set_data(data, **kwargs)
    
    def __getattr__(self, name):
        # Delegate to wrapped layer
        return getattr(self._layer, name)
```

**Advantages**:
- Captures operation context
- Can record parameters
- Works with existing code

**Limitations**:
- Requires wrapping all operations
- May not catch all modifications
- Performance overhead

### 5.3 Command Pattern for Provenance

**Approach**: Use command pattern to abstract operations.

**Implementation**:
```python
class Command:
    def execute(self):
        raise NotImplementedError
    
    def undo(self):
        raise NotImplementedError
    
    def to_provenance_record(self):
        raise NotImplementedError

class SetOpacityCommand(Command):
    def __init__(self, layer: Layer, opacity: float):
        self.layer = layer
        self.new_opacity = opacity
        self.old_opacity = layer.opacity
    
    def execute(self):
        self.layer.opacity = self.new_opacity
    
    def undo(self):
        self.layer.opacity = self.old_opacity
    
    def to_provenance_record(self):
        return {
            'operation': 'set_opacity',
            'layer': self.layer,
            'old_value': self.old_opacity,
            'new_value': self.new_opacity,
            'timestamp': datetime.now()
        }

class CommandManager:
    def __init__(self):
        self.history = []
        self.provenance = []
    
    def execute(self, command: Command):
        command.execute()
        self.history.append(command)
        self.provenance.append(command.to_provenance_record())
    
    def undo(self):
        if self.history:
            command = self.history.pop()
            command.undo()
```

**Advantages**:
- Enables undo/redo
- Captures full operation context
- Standardized operation representation

**Limitations**:
- Requires significant architectural changes
- All operations must be commands
- More complex implementation

---

## 6. Architectural Recommendations for Provenance

### 6.1 What Would Make Provenance Easier

**1. Operation Abstraction**:
- Command pattern for all operations
- Standardized operation interface
- Operation history stack

**2. Provenance API**:
- Standard interface for reporting operations
- Centralized provenance manager
- Plugin integration points

**3. State Versioning**:
- Copy-on-write for data modifications
- Version snapshots
- Diff-based history

**4. Persistent State**:
- Save/load application state
- Include provenance in saved state
- Export provenance records

**5. Event History**:
- Persistent event log
- Event replay capability
- Event filtering and querying

### 6.2 What Makes Provenance Harder

**1. In-Place Modifications**:
- Direct array access bypasses tracking
- No automatic versioning
- Previous state lost

**2. Plugin Fragmentation**:
- No standard for operation reporting
- Each plugin handles differently
- Difficult to aggregate

**3. Performance Constraints**:
- History storage overhead
- Versioning memory cost
- Real-time tracking impact

**4. UI Complexity**:
- Exposing provenance to users
- History navigation interface
- Workflow visualization

---

## 7. Existing Provenance Solutions

### 7.1 Plugin-Based Solutions

**napari-stracking**:
- Tracks particle tracking operations
- Separates steps into plugins
- Enables reproducible pipelines

**napari-arboretum**:
- Tracks cell lineage
- Visualizes data relationships
- Domain-specific provenance

**Limitations**:
- Domain-specific
- Not general-purpose
- Don't track all operations

### 7.2 External Tools

**Manual Documentation**:
- Users document workflows manually
- Scripts to log operations
- External provenance systems

**Limitations**:
- Error-prone
- Time-consuming
- Not automatic

---

## 8. Practical Examples

### 8.1 Tracking Layer Creation

```python
class ProvenanceTracker:
    def __init__(self, viewer: Viewer):
        self.viewer = viewer
        self.history = []
        
        @viewer.layers.events.inserted.connect
        def on_layer_added(event):
            self.record({
                'operation': 'layer_added',
                'layer_type': type(event.value).__name__,
                'layer_name': event.value.name,
                'data_shape': event.value.data.shape if hasattr(event.value, 'data') else None,
                'timestamp': datetime.now(),
                'source': 'user'  # Could be 'plugin', 'script', etc.
            })
```

### 8.2 Tracking Data Modifications

```python
class DataProvenanceTracker:
    def __init__(self, layer: Layer):
        self.layer = layer
        self.history = []
        
        @layer.events.data.connect
        def on_data_changed(event):
            # Capture change
            self.record({
                'operation': 'data_changed',
                'previous_shape': self._get_previous_shape(),
                'new_shape': event.value.shape,
                'previous_dtype': self._get_previous_dtype(),
                'new_dtype': event.value.dtype,
                'timestamp': datetime.now()
            })
```

### 8.3 Tracking Plugin Operations

```python
# Plugin that reports its operations
@napari_hook_implementation
def tracked_filter_plugin(viewer: Viewer):
    """Plugin that tracks its operations"""
    layer = viewer.layers.selection.active
    
    # Get provenance tracker (if available)
    tracker = getattr(viewer, 'provenance', None)
    
    # Record operation start
    if tracker:
        operation_id = tracker.start_operation(
            plugin='filter_plugin',
            operation='apply_filter',
            input_layer=layer,
            parameters={'threshold': 0.5}
        )
    
    # Perform operation
    filtered = apply_filter(layer.data, threshold=0.5)
    layer.set_data(filtered)
    
    # Record operation completion
    if tracker:
        tracker.complete_operation(operation_id, {
            'output_shape': filtered.shape,
            'output_dtype': filtered.dtype
        })
```

---

## 9. Summary

Napari's architecture has mixed implications for provenance tracking:

### Current Limitations

- **No Native Provenance**: No built-in provenance tracking system
- **No Operation History**: No undo/redo or operation history
- **No Standardized API**: No standard way for plugins to report operations
- **In-Place Modifications**: Direct modifications don't create history
- **No Persistent State**: No built-in state saving with history

### Architectural Advantages

- **Event System**: Events provide hooks for tracking
- **Plugin Architecture**: Enables provenance plugins
- **Layer Model**: Layer-level tracking is feasible
- **Separation of Concerns**: Models can be extended independently

### What Would Help

- **Operation Abstraction**: Command pattern for operations
- **Provenance API**: Standardized interface for reporting
- **State Versioning**: Copy-on-write or versioning system
- **Persistent State**: Save/load with provenance
- **Event History**: Persistent event log

### Current Workarounds

- **Event-Based Tracking**: Subscribe to events and record
- **Wrapper-Based Tracking**: Wrap operations to track
- **Plugin Solutions**: Domain-specific provenance plugins
- **External Tools**: Manual documentation or external systems

While napari's architecture doesn't make provenance impossible, it doesn't make it easy either. The event-driven, plugin-based design provides opportunities for provenance tracking, but significant work is needed to implement comprehensive provenance systems. The lack of operation abstraction, standardized APIs, and persistent state management are the main architectural barriers to easy provenance implementation.

