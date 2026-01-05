# QGIS "Unit of Truth" Architecture: Layers, Ownership, and Consistency

## Executive Summary

In QGIS, the **QgsMapLayer** class (and its concrete subclasses like **QgsVectorLayer** and **QgsRasterLayer**) serves as the fundamental "unit of truth" for geospatial data representation. This architectural choice has profound implications for how layers are identified, owned, referenced, and maintained across the application, directly influencing undo/redo capabilities, provenance tracking, and multi-view consistency.

---

## 1. The Unit of Truth: QgsMapLayer Hierarchy

### 1.1 Base Class: QgsMapLayer

**QgsMapLayer** is an abstract base class that serves as the foundation for all map layer types in QGIS:

- **Vector layers** (`QgsVectorLayer`) - for point, line, and polygon features
- **Raster layers** (`QgsRasterLayer`) - for raster/imagery data
- **Mesh layers** (`QgsMeshLayer`) - for mesh data
- **Point cloud layers** (`QgsPointCloudLayer`) - for LiDAR and point cloud data
- **Tiled scene layers** (`QgsTiledSceneLayer`) - for 3D tiled scenes
- **Annotation layers** (`QgsAnnotationLayer`) - for annotations

Each layer type inherits from `QgsMapLayer` and must implement several virtual methods:
- `clone()` - creates a copy of the layer
- `createMapRenderer()` - creates the appropriate renderer
- `readSymbology()` / `writeSymbology()` - handles symbology persistence
- `setTransformContext()` - manages coordinate transformations

### 1.2 Why Layers Are the Unit of Truth

Layers encapsulate:
- **Data source** - the underlying geospatial data (file, database, web service)
- **Spatial reference system** (CRS) - coordinate system information
- **Symbology and styling** - how the data is rendered
- **Metadata** - descriptive information about the layer
- **Configuration** - layer-specific settings and properties

This encapsulation makes layers self-contained units that can be:
- Independently managed
- Serialized to/from project files
- Referenced by unique identifiers
- Tracked for changes and provenance

---

## 2. Layer Identification: Unique Layer IDs

### 2.1 Layer ID Generation and Management

Every `QgsMapLayer` has a **unique identifier** (layer ID) that serves as its primary key within a QGIS project:

```python
# Get layer ID
layer_id = layer.id()

# Set layer ID (must be unique)
layer.setId("my_unique_layer_id")

# Generate a unique ID based on layer name
unique_id = QgsMapLayer.generateId("My Layer Name")
```

**Key characteristics of layer IDs:**
- Must be unique within a project
- Persist across project save/load cycles
- Used for all layer references throughout the project
- Automatically generated if not explicitly set (prefixed with layer name)

### 2.2 ID-Based Layer Lookup

Layers are primarily accessed by their IDs:

```python
# Get layer by ID from project
layer = project.mapLayer(layer_id)

# Get layers by name (may return multiple)
layers = project.mapLayersByName("Layer Name")

# Get all layers as dictionary (ID -> Layer)
all_layers = project.mapLayers()
```

The ID-based lookup system ensures:
- **Stable references** - layer IDs don't change unless explicitly modified
- **Efficient access** - O(1) dictionary lookup by ID
- **Project persistence** - IDs are saved in project XML files

### 2.3 ID Change Tracking

When a layer ID changes, QGIS emits a signal:

```python
# Signal emitted when layer ID changes
layer.idChanged()
```

This signal allows dependent systems (views, expressions, relations) to update their references to maintain consistency.

---

## 3. Layer Ownership: QgsProject and QgsMapLayerStore

### 3.1 Ownership Model

**QgsProject** serves as the central container and owner of all layers in a QGIS project. Internally, QgsProject uses **QgsMapLayerStore** to manage layer ownership:

```python
class QgsMapLayerStore:
    """A storage object for map layers, in which the layers are owned 
    by the store and have their lifetime bound to the store."""
```

**Key ownership principles:**
- When a layer is added to a project, **ownership is transferred** to the project
- The project (via QgsMapLayerStore) is responsible for layer lifetime management
- Removing a layer from the project **deletes** the layer if the project owns it
- Layers can be "taken" from the project, transferring ownership back to the caller

### 3.2 Adding Layers to Project

```python
# Add a single layer (ownership transferred to project)
added_layer = project.addMapLayer(layer)

# Add multiple layers at once
added_layers = project.addMapLayers([layer1, layer2, layer3])
```

**What happens when adding:**
1. Layer ownership is transferred to `QgsMapLayerStore`
2. Layer is registered in the project's layer dictionary (keyed by ID)
3. `layersAdded()` signal is emitted
4. `layerWasAdded()` signal is emitted for each layer
5. Project is marked as "dirty" (needs saving)

### 3.3 Removing Layers from Project

```python
# Remove by layer object (deletes if project owns it)
project.removeMapLayer(layer)

# Remove by layer ID
project.removeMapLayer(layer_id)

# Remove multiple layers
project.removeMapLayers([layer1, layer2])

# Remove all layers
project.removeAllMapLayers()

# Take layer (removes from project but doesn't delete)
# Caller assumes ownership
layer = project.takeMapLayer(layer_id)
```

**What happens when removing:**
1. Layer is removed from the layer registry
2. If project owns the layer, it is **deleted**
3. Project is marked as "dirty"
4. `removeAll()` signal is emitted
5. All dependent references (views, expressions, relations) are notified

### 3.4 Weak References

Some systems use **weak references** to layers to avoid ownership issues:

```python
# QgsGroupLayer stores child layers as weak references
# Child layers are NOT owned by QgsGroupLayer
# References are automatically cleaned up when child layer is deleted
```

This pattern prevents circular ownership and allows layers to be removed without breaking group layer structures.

---

## 4. Layer References in QgsProject

### 4.1 Reference Storage

Within a QGIS project, layers are referenced in multiple ways:

1. **Primary Registry**: `QgsMapLayerStore` maintains a dictionary of `{layer_id: QgsMapLayer}`
2. **Layer Tree**: `QgsLayerTree` maintains hierarchical structure (groups, visibility)
3. **Relations**: `QgsRelation` objects reference layers by ID
4. **Expressions**: Field expressions may reference other layers by ID
5. **Project XML**: Layer references stored as IDs in project file

### 4.2 Reference Resolution

When loading a project from XML, layer references are stored as IDs and must be resolved:

```python
# Layers are stored in XML with their IDs
# After reading XML, references must be resolved
def _resolveReferences(self, project: QgsProject | None):
    """Resolve references to other layers (kept as layer IDs after 
    reading XML) into layer objects."""
```

**Resolution process:**
1. Project XML is read, layer IDs are extracted
2. Layers are loaded and registered
3. `_resolveReferences()` is called on each layer
4. Layer IDs are resolved to actual `QgsMapLayer` objects
5. Cross-layer dependencies are established

### 4.3 Loose Matching

For compatibility across projects, QGIS supports "loose matching":

```python
# Loose matching: match by name, source, and provider
# rather than strict ID matching
# Useful for matching legend customizations across projects
```

This allows project components (like legend configurations) to work even when layer IDs differ between projects.

---

## 5. Undo/Redo Architecture

### 5.1 Per-Layer Undo Stacks

Each `QgsMapLayer` maintains its own **undo stack**:

```python
# Get the undo stack for a layer
undo_stack = layer.undoStack()  # Returns QUndoStack | None
```

**Key characteristics:**
- Each layer has an independent undo/redo history
- Undo operations are layer-specific
- Undo stack is managed by the layer itself
- Not all layer types support undo (depends on editability)

### 5.2 Vector Layer Edit Buffer

For `QgsVectorLayer`, editing operations use an **edit buffer** system:

```python
# Get the edit buffer (only valid during editing)
edit_buffer = vector_layer.editBuffer()

# Start editing
vector_layer.startEditing()

# Make changes (stored in edit buffer)
# ...

# Finish edit command (adds to undo stack)
vector_layer.endEditCommand()

# Commit changes
vector_layer.commitChanges()

# Or rollback changes
vector_layer.rollBack()
```

**Edit buffer workflow:**
1. `startEditing()` - enables editing mode, creates edit buffer
2. Changes are made (stored in buffer, not yet committed)
3. `beginEditCommand()` / `endEditCommand()` - groups changes into undoable commands
4. `commitChanges()` - writes buffer to data source
5. `rollBack()` - discards buffer without committing

### 5.3 Undo Command Types

QGIS provides specialized undo commands for vector layers:

```python
# Undo command classes
QgsVectorLayerUndoCommandAddFeature      # Adding features
QgsVectorLayerUndoCommandDeleteFeature   # Deleting features
QgsVectorLayerUndoCommandChangeGeometry  # Changing geometry
QgsVectorLayerUndoPassthroughCommandAddFeatures    # Batch operations
QgsVectorLayerUndoPassthroughCommandChangeGeometry
QgsVectorLayerUndoPassthroughCommandDeleteFeatures
```

### 5.4 Edit Buffer Groups

Multiple layers can be grouped for coordinated editing:

```python
# Create edit buffer group
edit_group = QgsVectorLayerEditBufferGroup()

# Add layers to group
edit_group.addLayer(layer1)
edit_group.addLayer(layer2)

# Operations apply to all layers in group
edit_group.commit()  # Commit all layers
edit_group.rollBack()  # Rollback all layers
```

This enables **transactional editing** across multiple related layers.

### 5.5 Impact of Layer-as-Unit-of-Truth on Undo/Redo

**Advantages:**
- **Isolation**: Undo operations on one layer don't affect others
- **Granularity**: Fine-grained control over what can be undone
- **Performance**: Only affected layers need to track undo history
- **Simplicity**: Each layer manages its own undo state

**Considerations:**
- Cross-layer operations require coordination (edit buffer groups)
- Project-level undo would require aggregating layer undo stacks
- Undo state is lost when layer is removed from project

---

## 6. Provenance Tracking

### 6.1 Configuration Change Signals

Layers emit signals when their configuration changes:

```python
# Signal emitted when layer configuration changes
layer.configChanged()

# The project listens to this signal to mark itself as dirty
# This tracks that the project has unsaved changes
```

**What triggers configChanged:**
- Symbology changes
- Style changes
- Labeling changes
- Rendering settings
- Any layer property modification

### 6.2 Project Dirty State

The project tracks whether it has unsaved changes:

```python
# Mark project as dirty (modified)
project.setDirty(True)

# Check if project is dirty
is_dirty = project.isDirty()

# Signal emitted when dirty state is set
project.dirtySet()
```

**Dirty state is set when:**
- Layers are added/removed
- Layer configuration changes (via `configChanged` signal)
- Project properties change
- Any project-level modification occurs

### 6.3 XML Serialization for Provenance

Layers serialize their complete state to XML for project files:

```python
# Write layer state to XML
layer.writeLayerXml(layer_element, document, context)

# Read layer state from XML
layer.readXml(layer_node, context)
```

**What is serialized:**
- Layer ID
- Data source (with path resolution)
- CRS information
- Symbology and styling
- Custom properties
- Metadata
- Layer-specific configuration

### 6.4 Change Tracking in Edit Buffer

For vector layers, the edit buffer tracks detailed change information:

```python
# Signals emitted after committing changes
edit_buffer.committedGeometriesChanges(layerId, changedGeometries)
edit_buffer.committedAttributeValuesChanges(layerId, changedAttributes)
```

**Change tracking includes:**
- Which features were modified
- What geometry changes occurred
- What attribute values changed
- Layer ID for cross-layer provenance

### 6.5 Impact of Layer-as-Unit-of-Truth on Provenance

**Advantages:**
- **Self-contained**: Each layer knows its own provenance
- **Granular tracking**: Changes tracked at layer level
- **Serialization**: Complete layer state can be saved/restored
- **Signal-based**: Changes propagate via Qt signals

**Considerations:**
- Cross-layer provenance requires coordination
- Project-level provenance aggregates layer-level information
- Historical provenance requires external tracking (not built-in)

---

## 7. Multi-View Consistency

### 7.1 Signal-Based Synchronization

QGIS uses Qt's signal/slot mechanism to maintain consistency across multiple views:

```python
# Layer signals that trigger view updates
layer.configChanged()        # Configuration changed
layer.idChanged()            # ID changed
layer.nameChanged()          # Name changed
layer.extentChanged()        # Spatial extent changed
layer.legendChanged()        # Legend/symbology changed
layer.reload()               # Data reloaded
```

**Views subscribe to these signals:**
- Map canvas (rendering)
- Layer tree (hierarchical view)
- Attribute table
- Layer properties dialog
- Legend widget
- Any custom views

### 7.2 Project-Level Signals

The project also emits signals for layer changes:

```python
# Project signals
project.layersAdded([layers])           # Layers added
project.layersRemoved([layer_ids])      # Layers removed
project.layerWasAdded(layer)            # Single layer added
project.removeAll()                      # All layers removed
project.dirtySet()                       # Project modified
```

### 7.3 Current Layer Tracking

QGIS tracks the "current" (active) layer:

```python
# Set current layer
project.setCurrentLayer(layer)

# Get current layer
current = project.currentLayer()

# Signal emitted when current layer changes
interface.currentLayerChanged(layer)
```

**Current layer affects:**
- Which layer is shown in attribute table
- Which layer receives edit operations
- Which layer properties are displayed
- Tool behavior and context

### 7.4 Layer Tree Synchronization

The layer tree (legend) is synchronized with the layer registry:

```python
# Get layer tree registry bridge
bridge = project.layerTreeRegistryBridge()

# This bridge synchronizes:
# - Layer registry <-> Layer tree
# - Layer additions/removals
# - Layer visibility
# - Layer group structure
```

### 7.5 Impact of Layer-as-Unit-of-Truth on Multi-View Consistency

**Advantages:**
- **Single source of truth**: All views reference the same layer objects
- **Automatic updates**: Signal-based propagation ensures consistency
- **Decoupled views**: Views don't need to know about each other
- **Efficient**: Only affected views update when layers change

**Mechanisms:**
- **Qt signals**: Decoupled, event-driven updates
- **Weak references**: Prevent dangling pointers
- **ID-based lookup**: Stable references even if layer objects change
- **Registry pattern**: Centralized layer management

**Considerations:**
- Views must properly subscribe to signals
- Race conditions possible if signals processed out of order
- Performance considerations with many views/layers

---

## 8. Architectural Implications

### 8.1 Design Patterns

The layer-as-unit-of-truth architecture employs several design patterns:

1. **Registry Pattern**: `QgsMapLayerStore` acts as a central registry
2. **Observer Pattern**: Signal/slot mechanism for view updates
3. **Factory Pattern**: Layer creation through providers
4. **Strategy Pattern**: Different renderers for different layer types
5. **Command Pattern**: Undo/redo command objects

### 8.2 Benefits of This Architecture

1. **Modularity**: Layers are independent, self-contained units
2. **Extensibility**: New layer types can be added by subclassing
3. **Consistency**: Single source of truth prevents data divergence
4. **Performance**: Efficient lookups and updates
5. **Persistence**: Complete state can be serialized
6. **Undo/Redo**: Natural fit for per-layer undo stacks

### 8.3 Trade-offs and Limitations

1. **Cross-layer operations**: Require explicit coordination
2. **Project-level undo**: Not directly supported (would need aggregation)
3. **Distributed editing**: Each layer manages its own state
4. **Memory management**: Ownership must be carefully managed
5. **Reference stability**: Layer IDs must remain stable

---

## 9. Practical Examples

### 9.1 Creating and Adding a Layer

```python
from qgis.core import QgsVectorLayer, QgsProject

# Create a vector layer
layer = QgsVectorLayer(
    "/path/to/data.shp",
    "My Layer",
    "ogr"
)

# Set a unique ID
layer.setId("my_layer_001")

# Add to project (ownership transferred)
project = QgsProject.instance()
project.addMapLayer(layer)

# Layer is now owned by project
# Removing it will delete it
```

### 9.2 Referencing Layers in Expressions

```python
# Expression can reference other layers by ID
expression = '"field1" + layer_property(\'other_layer_id\', \'feature_count\')'

# Layer IDs are resolved when expression is evaluated
```

### 9.3 Undo/Redo Workflow

```python
# Start editing
layer.startEditing()

# Begin edit command (groups changes)
layer.beginEditCommand("Add feature")

# Make changes
feature = QgsFeature(layer.fields())
feature.setGeometry(geometry)
feature.setAttributes([...])
layer.addFeature(feature)

# End edit command (adds to undo stack)
layer.endEditCommand()

# Can now undo this operation
layer.undoStack().undo()

# Or commit changes
layer.commitChanges()
```

### 9.4 Tracking Changes

```python
# Connect to configuration change signal
def on_config_changed():
    print("Layer configuration changed")
    print(f"Project dirty: {project.isDirty()}")

layer.configChanged.connect(on_config_changed)

# Any change to layer will trigger this
layer.setName("New Name")  # Triggers configChanged
```

---

## 10. Conclusion

The choice of **QgsMapLayer** (and its subclasses) as the "unit of truth" in QGIS creates a robust, extensible architecture for managing geospatial data. This design:

- **Enables** efficient layer management through unique IDs and centralized ownership
- **Supports** fine-grained undo/redo at the layer level
- **Facilitates** provenance tracking through signals and serialization
- **Ensures** multi-view consistency through Qt's signal/slot mechanism
- **Provides** a foundation for extensibility and modularity

Understanding this architecture is crucial for:
- Developing QGIS plugins
- Integrating QGIS into larger systems
- Implementing custom layer types
- Building applications on top of QGIS
- Debugging layer-related issues

The layer-centric design, while having some limitations for cross-layer operations, provides a solid foundation for a complex GIS application where layers are the natural unit of organization and management.

