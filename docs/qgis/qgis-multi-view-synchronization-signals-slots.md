# QGIS Multi-View Synchronization: Signals, Slots, and Decoupled Updates

## Executive Summary

QGIS synchronizes multiple views (map canvas, attribute table, layer tree, processing results) using Qt's **signal/slot mechanism** combined with **bridge patterns** to achieve decoupled, event-driven updates. This architecture ensures that all views stay synchronized with the underlying data model without requiring direct references between UI components. Data updates propagate automatically through signal chains, with bridge classes managing the synchronization between different component hierarchies.

---

## 1. Signal/Slot Architecture Overview

### 1.1 Qt Signal/Slot Mechanism

**Qt's signal/slot system** provides a type-safe, decoupled communication mechanism:

```python
# Signal definition (in QgsVectorLayer)
class QgsVectorLayer(QgsMapLayer):
    # Signal declaration
    featureAdded = pyqtSignal(int, QgsFeature)  # fid, feature
    attributeValueChanged = pyqtSignal(int, int, QVariant)  # fid, field, value
    repaintRequested = pyqtSignal(bool)  # deferred flag

# Slot connection (in QgsMapCanvas)
class QgsMapCanvas(QWidget):
    def __init__(self):
        # Connect signal to slot
        layer.featureAdded.connect(self.onFeatureAdded)
        layer.repaintRequested.connect(self.refresh)
    
    def onFeatureAdded(self, fid, feature):
        """Slot called when feature is added."""
        self.refresh()
```

**Key Characteristics:**
- **Type-safe**: Signal/slot signatures must match
- **Decoupled**: Sender doesn't know about receivers
- **One-to-many**: One signal can connect to multiple slots
- **Queued connections**: Can be queued for thread-safe execution
- **Automatic cleanup**: Connections automatically cleaned up when objects destroyed

### 1.2 QGIS Signal Architecture

**QGIS uses signals at multiple levels:**

1. **Layer Signals**: Emitted by layers when data changes
2. **Project Signals**: Emitted by project when layers added/removed
3. **Tree Signals**: Emitted by layer tree when structure changes
4. **Canvas Signals**: Emitted by map canvas when rendering changes
5. **Processing Signals**: Emitted by processing framework when algorithms complete

---

## 2. View Components and Their Synchronization

### 2.1 Map Canvas

**Purpose**: Renders layers on the map

**Synchronization Mechanisms:**

#### **A. Layer Signal Connections**

```python
class QgsMapCanvas(QWidget):
    """Map rendering widget."""
    
    def setLayers(self, layers):
        """
        Set layers to render.
        
        Synchronization:
        1. Connect to layer signals
        2. Listen for repaint requests
        3. Refresh on layer changes
        """
        # 1. Disconnect old layers
        for layer in self._layers:
            layer.repaintRequested.disconnect(self.refresh)
            layer.styleChanged.disconnect(self.refresh)
            layer.extentChanged.disconnect(self.onExtentChanged)
        
        # 2. Update layer list
        self._layers = layers
        
        # 3. Connect to new layer signals
        for layer in layers:
            # Repaint when layer requests repaint
            layer.repaintRequested.connect(self.refresh)
            
            # Repaint when style changes
            layer.styleChanged.connect(self.refresh)
            
            # Update extent when layer extent changes
            layer.extentChanged.connect(self.onExtentChanged)
        
        # 4. Emit canvas signal
        self.layersChanged.emit()
        
        # 5. Refresh canvas
        self.refresh()
    
    def refresh(self):
        """Refresh canvas display."""
        # Clear render cache
        self._mapSettings.clearCache()
        
        # Trigger Qt widget repaint
        self.update()
        
        # Emit refresh signal
        self.mapCanvasRefreshed.emit()
    
    def onExtentChanged(self):
        """Called when layer extent changes."""
        # Optionally adjust canvas extent
        self.refresh()
```

**Signal Connections:**
- `layer.repaintRequested` → `canvas.refresh()` - Repaint on layer changes
- `layer.styleChanged` → `canvas.refresh()` - Repaint on style changes
- `layer.extentChanged` → `canvas.onExtentChanged()` - Update extent

**Decoupling:**
- Canvas doesn't know about other views
- Canvas only listens to layer signals
- Canvas doesn't need direct references to attribute table or layer tree

#### **B. Bridge-Based Synchronization**

```python
class QgsLayerTreeMapCanvasBridge:
    """Synchronizes layer tree with map canvas."""
    
    def __init__(self, root, canvas, parent=None):
        # Connect to layer tree signals
        root.addedChildren.connect(self.nodeAddedChildren)
        root.removedChildren.connect(self.nodeRemovedChildren)
        root.visibilityChanged.connect(self.nodeVisibilityChanged)
        
        # Connect to canvas signals
        canvas.layersChanged.connect(self.onCanvasLayersChanged)
    
    def nodeVisibilityChanged(self, node):
        """
        Called when layer visibility changes in tree.
        
        Synchronization:
        1. Collect visible layers from tree
        2. Update canvas layer set
        3. Canvas automatically refreshes
        """
        # 1. Collect visible layers
        visible_layers = self.collectVisibleLayers()
        
        # 2. Update canvas (deferred for performance)
        self.setCanvasLayers(visible_layers)
    
    def setCanvasLayers(self, layers):
        """Update canvas layer set."""
        # Set layers on canvas
        # Canvas will connect to layer signals automatically
        self.mapCanvas().setLayers(layers)
```

**Bridge Pattern Benefits:**
- **Decoupling**: Tree and canvas don't know about each other
- **Single responsibility**: Bridge handles synchronization logic
- **Flexible**: Can change synchronization behavior without modifying tree or canvas

### 2.2 Attribute Table

**Purpose**: Displays and edits feature attributes

**Synchronization Mechanisms:**

#### **A. Layer Edit Signal Connections**

```python
class QgsAttributeTableModel(QAbstractTableModel):
    """Model for attribute table view."""
    
    def __init__(self, layer, parent=None):
        super().__init__(parent)
        self._layer = layer
        
        # Connect to layer edit signals
        layer.featureAdded.connect(self.featureAdded)
        layer.featureDeleted.connect(self.featureDeleted)
        layer.attributeValueChanged.connect(self.attributeValueChanged)
        layer.geometryChanged.connect(self.geometryChanged)
        
        # Connect to commit signals
        layer.committedFeaturesAdded.connect(self.committedFeaturesAdded)
        layer.committedFeaturesRemoved.connect(self.committedFeaturesRemoved)
        layer.committedAttributeValuesChanges.connect(
            self.committedAttributeValuesChanges
        )
        
        # Connect to editing state signals
        layer.editingStarted.connect(self.editingStarted)
        layer.editingStopped.connect(self.editingStopped)
    
    def featureAdded(self, fid, feature):
        """
        Called when feature added to layer.
        
        Synchronization:
        1. Add feature to model
        2. Emit model signals
        3. View automatically updates
        """
        # 1. Add feature to model
        row = self.rowCount()
        self.beginInsertRows(QModelIndex(), row, row)
        
        # 2. Update internal data
        self._features.append(feature)
        self._fidToRow[fid] = row
        
        # 3. Emit model signal (Qt Model/View)
        self.endInsertRows()
    
    def attributeValueChanged(self, fid, field, value):
        """
        Called when attribute value changes.
        
        Synchronization:
        1. Find feature row
        2. Update model data
        3. Emit model signals
        4. View automatically updates
        """
        # 1. Find feature row
        row = self._fidToRow.get(fid)
        if row is None:
            return
        
        # 2. Update model data
        field_index = self._layer.fields().indexOf(field)
        index = self.index(row, field_index)
        self.setData(index, value)
        
        # 3. Emit model signal (Qt Model/View)
        self.dataChanged.emit(index, index)
    
    def featureDeleted(self, fid):
        """
        Called when feature deleted from layer.
        
        Synchronization:
        1. Remove feature from model
        2. Emit model signals
        3. View automatically updates
        """
        # 1. Find feature row
        row = self._fidToRow.get(fid)
        if row is None:
            return
        
        # 2. Remove from model
        self.beginRemoveRows(QModelIndex(), row, row)
        
        # 3. Update internal data
        del self._features[row]
        del self._fidToRow[fid]
        
        # 4. Emit model signal (Qt Model/View)
        self.endRemoveRows()
    
    def committedAttributeValuesChanges(self, layerId, changes):
        """
        Called after attributes committed to data source.
        
        Synchronization:
        1. Refresh model to reflect committed changes
        2. Emit model signals
        3. View automatically updates
        """
        # Refresh entire model
        self.reload()
```

**Signal Connections:**
- `layer.featureAdded` → `model.featureAdded()` - Add feature to table
- `layer.featureDeleted` → `model.featureDeleted()` - Remove feature from table
- `layer.attributeValueChanged` → `model.attributeValueChanged()` - Update cell value
- `layer.committedAttributeValuesChanges` → `model.committedAttributeValuesChanges()` - Refresh after commit

**Decoupling:**
- Attribute table doesn't know about map canvas
- Attribute table only listens to layer signals
- Attribute table doesn't need direct references to other views

#### **B. Bidirectional Updates**

```python
class QgsAttributeTableModel:
    """Model for attribute table view."""
    
    def setData(self, index, value, role=Qt.EditRole):
        """
        Called when user edits cell in table.
        
        Synchronization:
        1. Update layer (triggers layer signals)
        2. Layer signals propagate to other views
        3. All views automatically update
        """
        if role == Qt.EditRole:
            # 1. Get feature and field
            row = index.row()
            field_index = index.column()
            feature = self._features[row]
            fid = feature.id()
            
            # 2. Update layer (triggers layer signals)
            if self._layer.isEditable():
                self._layer.changeAttributeValue(fid, field_index, value)
                # This emits attributeValueChanged signal
                # Which propagates to other views
            
            return True
        
        return False
```

**Update Flow:**
1. User edits cell in attribute table
2. Model updates layer via `changeAttributeValue()`
3. Layer emits `attributeValueChanged` signal
4. Signal propagates to:
   - Attribute table model (updates display)
   - Map canvas (refreshes rendering)
   - Renderer (updates symbology if field affects style)
   - Other connected views

### 2.3 Layer Tree (Legend)

**Purpose**: Hierarchical view of layers and groups

**Synchronization Mechanisms:**

#### **A. Project Signal Connections**

```python
class QgsLayerTreeRegistryBridge:
    """Synchronizes project layers with layer tree."""
    
    def __init__(self, root, project, parent=None):
        # Connect to project signals
        project.layersAdded.connect(self.layersAdded)
        project.layersRemoved.connect(self.layersRemoved)
        project.layerWasAdded.connect(self.layerWasAdded)
        project.layerWillBeRemoved.connect(self.layerWillBeRemoved)
    
    def layersAdded(self, layers):
        """
        Called when layers added to project.
        
        Synchronization:
        1. Create layer tree nodes
        2. Insert into tree structure
        3. Tree model automatically updates
        4. View automatically updates
        """
        for layer in layers:
            # 1. Create layer tree node
            layer_node = QgsLayerTreeLayer(layer)
            
            # 2. Insert into tree
            insertion_point = self.layerInsertionPoint()
            if insertion_point.group:
                insertion_point.group.insertChildNode(
                    insertion_point.index,
                    layer_node
                )
            else:
                self.root().insertChildNode(
                    insertion_point.index,
                    layer_node
                )
    
    def layersRemoved(self, layerIds):
        """
        Called when layers removed from project.
        
        Synchronization:
        1. Find layer nodes in tree
        2. Remove from tree structure
        3. Tree model automatically updates
        4. View automatically updates
        """
        for layerId in layerIds:
            # 1. Find layer node
            layer_node = self.root().findLayer(layerId)
            if layer_node:
                # 2. Remove from tree
                parent = layer_node.parent()
                parent.removeChildNode(layer_node)
```

**Signal Connections:**
- `project.layersAdded` → `bridge.layersAdded()` - Add layers to tree
- `project.layersRemoved` → `bridge.layersRemoved()` - Remove layers from tree
- `project.layerWasAdded` → `bridge.layerWasAdded()` - Handle single layer addition

**Decoupling:**
- Layer tree doesn't know about project internals
- Bridge handles synchronization logic
- Tree only knows about tree structure

#### **B. Tree Model Updates**

```python
class QgsLayerTreeModel(QAbstractItemModel):
    """Model for layer tree view."""
    
    def __init__(self, root, parent=None):
        super().__init__(parent)
        self._root = root
        
        # Connect to tree signals
        root.addedChildren.connect(self.nodeAddedChildren)
        root.removedChildren.connect(self.nodeRemovedChildren)
        root.visibilityChanged.connect(self.nodeVisibilityChanged)
        root.nameChanged.connect(self.nodeNameChanged)
    
    def nodeAddedChildren(self, node, indexFrom, indexTo):
        """
        Called when children added to tree node.
        
        Synchronization:
        1. Update model structure
        2. Emit model signals
        3. View automatically updates
        """
        # 1. Get parent index
        parent_index = self.node2index(node)
        
        # 2. Emit model signals (Qt Model/View)
        self.beginInsertRows(parent_index, indexFrom, indexTo)
        
        # 3. Update internal structure
        # ... model update ...
        
        # 4. Emit model signal (Qt Model/View)
        self.endInsertRows()
    
    def nodeVisibilityChanged(self, node):
        """
        Called when node visibility changes.
        
        Synchronization:
        1. Update model data
        2. Emit model signals
        3. View automatically updates
        4. Canvas bridge updates canvas
        """
        # 1. Get node index
        index = self.node2index(node)
        
        # 2. Emit model signal (Qt Model/View)
        self.dataChanged.emit(index, index)
```

**Signal Chain:**
1. Project adds layer → `project.layersAdded` signal
2. Bridge receives signal → `bridge.layersAdded()` slot
3. Bridge adds node to tree → `tree.insertChildNode()`
4. Tree emits signal → `tree.addedChildren` signal
5. Model receives signal → `model.nodeAddedChildren()` slot
6. Model emits Qt signal → `model.beginInsertRows()` / `endInsertRows()`
7. View receives Qt signal → View automatically updates

### 2.4 Processing Results

**Purpose**: Display processing algorithm outputs

**Synchronization Mechanisms:**

#### **A. Processing Completion Signals**

```python
class QgsProcessingAlgorithmDialog(QDialog):
    """Processing algorithm parameter dialog."""
    
    def accept(self):
        """
        Called when user clicks "Run".
        
        Synchronization:
        1. Execute algorithm
        2. Get output layer
        3. Add to project (triggers project signals)
        4. All views automatically update
        """
        # 1. Execute algorithm
        result = processing.run(
            self.algorithm().id(),
            self.getParameterValues(),
            context=self.context(),
            feedback=self.feedback()
        )
        
        # 2. Get output layer
        output_layer_id = result.get('OUTPUT')
        if output_layer_id:
            # 3. Load layer into project
            layer = self.context().takeResultLayer(output_layer_id)
            if layer:
                # 4. Add to project (triggers project signals)
                self.context().project().addMapLayer(layer)
                # This emits project.layersAdded signal
                # Which propagates to all views via bridges
```

**Signal Chain:**
1. Processing completes → Algorithm returns result
2. Result layer added to project → `project.addMapLayer()`
3. Project emits signal → `project.layersAdded` signal
4. Bridge receives signal → `bridge.layersAdded()` slot
5. Bridge adds to tree → Tree structure updated
6. Tree emits signal → `tree.addedChildren` signal
7. Canvas bridge receives signal → `canvasBridge.setCanvasLayers()`
8. Canvas receives signal → `canvas.setLayers()` → `canvas.refresh()`
9. All views updated → Synchronized automatically

#### **B. Processing Context Integration**

```python
class QgsProcessingContext:
    """Processing execution context."""
    
    def addLayerToLoadOnCompletion(self, layerId, details):
        """
        Schedule layer to be loaded after algorithm completion.
        
        Synchronization:
        1. Store layer details
        2. Load after completion
        3. Add to project (triggers signals)
        """
        self._layersToLoad[layerId] = details
    
    def takeResultLayer(self, layerId):
        """
        Get result layer from context.
        
        Synchronization:
        1. Get layer from context
        2. Remove from context
        3. Return layer (caller adds to project)
        """
        return self._resultLayers.pop(layerId, None)
```

**Integration Points:**
- Processing context manages result layers
- Result layers added to project after completion
- Project signals propagate to all views
- No direct coupling between processing and views

---

## 3. Bridge Pattern for Decoupling

### 3.1 Layer Tree Registry Bridge

**Purpose**: Synchronizes project layer registry with layer tree

```python
class QgsLayerTreeRegistryBridge(QObject):
    """Synchronizes project layers with layer tree."""
    
    def __init__(self, root, project, parent=None):
        super().__init__(parent)
        self._root = root
        self._project = project
        
        # Connect to project signals
        project.layersAdded.connect(self.layersAdded)
        project.layersRemoved.connect(self.layersRemoved)
        project.layerWasAdded.connect(self.layerWasAdded)
        project.layerWillBeRemoved.connect(self.layerWillBeRemoved)
        
        # Connect to tree signals (for reverse synchronization)
        root.addedChildren.connect(self.nodeAddedChildren)
        root.removedChildren.connect(self.nodeRemovedChildren)
    
    def layersAdded(self, layers):
        """
        Called when layers added to project.
        
        Decoupling:
        - Bridge doesn't know about canvas or attribute table
        - Bridge only synchronizes project ↔ tree
        - Other bridges handle tree ↔ canvas synchronization
        """
        for layer in layers:
            # Create tree node
            layer_node = QgsLayerTreeLayer(layer)
            
            # Insert into tree
            insertion_point = self.layerInsertionPoint()
            if insertion_point.group:
                insertion_point.group.insertChildNode(
                    insertion_point.index,
                    layer_node
                )
            else:
                self._root.insertChildNode(
                    insertion_point.index,
                    layer_node
                )
```

**Decoupling Benefits:**
- **Project doesn't know about tree**: Project only emits signals
- **Tree doesn't know about project**: Tree only manages tree structure
- **Bridge handles synchronization**: Bridge translates between project and tree
- **Other views unaffected**: Changes to bridge don't affect other views

### 3.2 Layer Tree Map Canvas Bridge

**Purpose**: Synchronizes layer tree with map canvas

```python
class QgsLayerTreeMapCanvasBridge(QObject):
    """Synchronizes layer tree with map canvas."""
    
    def __init__(self, root, canvas, parent=None):
        super().__init__(parent)
        self._root = root
        self._canvas = canvas
        
        # Connect to tree signals
        root.addedChildren.connect(self.nodeAddedChildren)
        root.removedChildren.connect(self.nodeRemovedChildren)
        root.visibilityChanged.connect(self.nodeVisibilityChanged)
        
        # Connect to canvas signals (for reverse synchronization)
        canvas.layersChanged.connect(self.onCanvasLayersChanged)
    
    def nodeVisibilityChanged(self, node):
        """
        Called when layer visibility changes in tree.
        
        Decoupling:
        - Bridge doesn't know about project or attribute table
        - Bridge only synchronizes tree ↔ canvas
        - Other bridges handle project ↔ tree synchronization
        """
        # Collect visible layers from tree
        visible_layers = self.collectVisibleLayers()
        
        # Update canvas
        self._canvas.setLayers(visible_layers)
        # Canvas will connect to layer signals automatically
```

**Decoupling Benefits:**
- **Tree doesn't know about canvas**: Tree only emits signals
- **Canvas doesn't know about tree**: Canvas only manages rendering
- **Bridge handles synchronization**: Bridge translates between tree and canvas
- **Other views unaffected**: Changes to bridge don't affect other views

### 3.3 Bridge Chain Pattern

**Complete Synchronization Chain:**

```
Project (Layer Registry)
    ↓ (project.layersAdded signal)
QgsLayerTreeRegistryBridge
    ↓ (tree.insertChildNode())
Layer Tree
    ↓ (tree.addedChildren signal)
QgsLayerTreeMapCanvasBridge
    ↓ (canvas.setLayers())
Map Canvas
    ↓ (canvas connects to layer signals)
Layer Signals
    ↓ (layer.repaintRequested signal)
Canvas Refresh
```

**Benefits:**
- **Each bridge handles one synchronization**: Project↔Tree, Tree↔Canvas
- **Bridges are independent**: Can modify one bridge without affecting others
- **No circular dependencies**: Signals flow in one direction
- **Easy to extend**: Add new bridges for new view types

---

## 4. Signal Propagation Patterns

### 4.1 Direct Signal Connections

**Pattern**: Component directly connects to layer signals

```python
# Map canvas directly connects to layer
layer.repaintRequested.connect(canvas.refresh)

# Attribute table directly connects to layer
layer.featureAdded.connect(tableModel.featureAdded)
layer.attributeValueChanged.connect(tableModel.attributeValueChanged)
```

**Use Cases:**
- **Layer-specific updates**: Views that need immediate updates from specific layers
- **Performance-critical**: Direct connections avoid bridge overhead
- **Simple relationships**: One-to-one or one-to-few relationships

**Benefits:**
- **Low latency**: Direct signal propagation
- **Simple**: No intermediate components
- **Efficient**: Minimal overhead

**Drawbacks:**
- **Tight coupling**: View must know about layer
- **Hard to extend**: Adding new views requires modifying layer connections

### 4.2 Bridge-Mediated Signal Connections

**Pattern**: Bridge connects signals between components

```python
# Bridge connects project signals to tree
project.layersAdded.connect(bridge.layersAdded)
bridge.layersAdded() → tree.insertChildNode()

# Bridge connects tree signals to canvas
tree.addedChildren.connect(bridge.nodeAddedChildren)
bridge.nodeAddedChildren() → canvas.setLayers()
```

**Use Cases:**
- **Complex synchronization**: Multiple components need coordination
- **Hierarchical relationships**: Components in different hierarchies
- **Flexible architecture**: Need to change synchronization behavior

**Benefits:**
- **Decoupling**: Components don't know about each other
- **Flexible**: Can change bridge behavior without modifying components
- **Extensible**: Easy to add new bridges for new relationships

**Drawbacks:**
- **Additional overhead**: Bridge adds indirection
- **More complex**: More components to understand

### 4.3 Cascading Signal Propagation

**Pattern**: Signals cascade through multiple components

```python
# Signal cascade: Layer → Canvas → Renderer
layer.repaintRequested.emit()
    ↓
canvas.refresh() (slot)
    ↓
canvas.mapCanvasRefreshed.emit()
    ↓
renderer.onCanvasRefreshed() (slot)
    ↓
renderer.updateCache()
```

**Use Cases:**
- **Dependent updates**: One component's update triggers another's update
- **Optimization**: Batch updates through cascade
- **Coordination**: Multiple components need to update together

**Benefits:**
- **Automatic coordination**: Updates propagate automatically
- **Efficient**: Can batch updates in cascade
- **Flexible**: Can add/remove cascade steps

**Drawbacks:**
- **Hard to debug**: Signal chains can be complex
- **Performance**: Long chains can be slow
- **Order dependency**: Update order matters

---

## 5. Data Update Propagation

### 5.1 Feature Edit Propagation

**Complete Update Flow:**

```
User edits feature in attribute table
    ↓
QgsAttributeTableModel.setData()
    ↓
QgsVectorLayer.changeAttributeValue()
    ↓
QgsVectorLayerEditBuffer.changeAttributeValue()
    ↓
QgsVectorLayer.attributeValueChanged.emit(fid, field, value)
    ↓
┌─────────────────────────────────────────┐
│  Signal Propagation:                    │
│                                         │
│  1. Attribute Table Model               │
│     → model.attributeValueChanged()     │
│     → model.dataChanged.emit()          │
│     → View updates cell                 │
│                                         │
│  2. Map Canvas                          │
│     → canvas.refresh()                  │
│     → Canvas repaints                   │
│                                         │
│  3. Renderer                             │
│     → renderer.attributeValueChanged()  │
│     → renderer.updateSymbol()           │
│     → renderer.repaintRequested.emit()  │
│     → Canvas refreshes                  │
│                                         │
│  4. Custom Plugins                      │
│     → plugin.onAttributeChanged()       │
│     → Plugin-specific updates           │
└─────────────────────────────────────────┘
```

**Key Points:**
- **Single source of truth**: Layer is the source of truth
- **Automatic propagation**: Signals automatically propagate to all connected views
- **No direct coupling**: Views don't know about each other
- **Efficient**: Only affected views update

### 5.2 Layer Addition Propagation

**Complete Update Flow:**

```
Processing algorithm completes
    ↓
QgsProcessingContext.takeResultLayer()
    ↓
QgsProject.addMapLayer(layer)
    ↓
QgsMapLayerStore.addMapLayer(layer)
    ↓
QgsMapLayerStore.layersAdded.emit([layer])
    ↓
QgsProject.layersAdded.emit([layer])  (forwarded)
    ↓
┌─────────────────────────────────────────┐
│  Signal Propagation:                    │
│                                         │
│  1. Layer Tree Registry Bridge          │
│     → bridge.layersAdded()              │
│     → tree.insertChildNode()            │
│     → tree.addedChildren.emit()         │
│                                         │
│  2. Layer Tree Model                    │
│     → model.nodeAddedChildren()         │
│     → model.beginInsertRows()           │
│     → View adds tree item               │
│                                         │
│  3. Layer Tree Map Canvas Bridge        │
│     → bridge.nodeAddedChildren()        │
│     → bridge.setCanvasLayers()          │
│     → canvas.setLayers()                │
│                                         │
│  4. Map Canvas                          │
│     → canvas.setLayers()                │
│     → canvas connects to layer signals  │
│     → canvas.refresh()                  │
│     → Canvas renders new layer          │
│                                         │
│  5. Attribute Table                     │
│     → (No direct connection)             │
│     → User must manually select layer   │
└─────────────────────────────────────────┘
```

**Key Points:**
- **Cascading updates**: Updates cascade through bridge chain
- **Automatic synchronization**: All views update automatically
- **No manual coordination**: No need to manually update each view
- **Efficient**: Updates batched where possible

### 5.3 Style Change Propagation

**Complete Update Flow:**

```
User changes layer renderer
    ↓
QgsVectorLayer.setRenderer(renderer)
    ↓
QgsVectorLayer.rendererChanged.emit()
    ↓
QgsVectorLayer.configChanged.emit()
    ↓
QgsVectorLayer.repaintRequested.emit()
    ↓
┌─────────────────────────────────────────┐
│  Signal Propagation:                    │
│                                         │
│  1. Map Canvas                          │
│     → canvas.refresh()                  │
│     → Canvas repaints with new style    │
│                                         │
│  2. Layer Tree (Legend)                 │
│     → tree.model.dataChanged()          │
│     → View updates legend icon          │
│                                         │
│  3. Renderer                             │
│     → renderer.clearCache()             │
│     → renderer.repaintRequested.emit()  │
│     → Canvas refreshes                  │
│                                         │
│  4. Custom Plugins                      │
│     → plugin.onStyleChanged()           │
│     → Plugin-specific updates           │
└─────────────────────────────────────────┘
```

**Key Points:**
- **Style-only updates**: No data changes, only presentation
- **Fast propagation**: Style changes propagate quickly
- **Visual updates**: Only visual components update
- **No data reload**: Data doesn't need to be reloaded

---

## 6. Decoupling Mechanisms

### 6.1 Signal-Based Communication

**Principle**: Components communicate only through signals

```python
# ❌ BAD: Direct coupling
class QgsMapCanvas:
    def addLayer(self, layer):
        # Direct reference to attribute table
        self.attributeTable.addLayer(layer)  # Tight coupling

# ✅ GOOD: Signal-based
class QgsMapCanvas:
    def addLayer(self, layer):
        # Emit signal, don't know about receivers
        self.layersChanged.emit()  # Decoupled
```

**Benefits:**
- **No direct references**: Components don't hold references to each other
- **Flexible**: Can add/remove signal connections dynamically
- **Testable**: Easy to test components in isolation
- **Extensible**: Easy to add new signal receivers

### 6.2 Bridge Pattern

**Principle**: Bridges handle synchronization between components

```python
# Bridge handles synchronization
class QgsLayerTreeMapCanvasBridge:
    def __init__(self, tree, canvas):
        # Bridge connects signals
        tree.addedChildren.connect(self.nodeAddedChildren)
        # Bridge doesn't expose tree or canvas to each other
```

**Benefits:**
- **Separation of concerns**: Each component has single responsibility
- **Independent evolution**: Components can evolve independently
- **Easy to modify**: Can change bridge behavior without modifying components
- **Reusable**: Bridges can be reused for different component pairs

### 6.3 Weak References

**Principle**: Use weak references to prevent circular dependencies

```python
# Weak reference prevents circular dependency
class QgsGroupLayer:
    def __init__(self):
        # Child layers stored as weak references
        self._childLayers = []  # Weak references
    
    def addChildLayer(self, layer):
        # Store weak reference
        weak_ref = weakref.ref(layer)
        self._childLayers.append(weak_ref)
```

**Benefits:**
- **No circular dependencies**: Weak references don't prevent garbage collection
- **Automatic cleanup**: Objects can be garbage collected when not needed
- **Memory efficient**: Prevents memory leaks from circular references

### 6.4 ID-Based Lookup

**Principle**: Use IDs instead of direct object references

```python
# ID-based lookup instead of direct reference
class QgsProject:
    def mapLayer(self, layerId):
        """Get layer by ID."""
        return self._layers.get(layerId)
    
    def removeMapLayer(self, layerId):
        """Remove layer by ID."""
        layer = self.mapLayer(layerId)
        if layer:
            del self._layers[layerId]
```

**Benefits:**
- **Stable references**: IDs don't change when objects are recreated
- **Serializable**: IDs can be saved to project files
- **Flexible**: Can resolve IDs to objects when needed
- **No dangling pointers**: IDs are safe even if objects are deleted

---

## 7. Complete Synchronization Example

### 7.1 Adding a Processing Result Layer

**Complete Flow:**

```python
# 1. Processing algorithm completes
result = processing.run('native:buffer', {
    'INPUT': input_layer_id,
    'DISTANCE': 100,
    'OUTPUT': 'memory:'
})

# 2. Get output layer
output_layer = result['OUTPUT']
layer = QgsVectorLayer(output_layer, 'Buffer', 'ogr')

# 3. Add to project
project = QgsProject.instance()
project.addMapLayer(layer)
# Emits: project.layersAdded([layer])

# 4. Layer Tree Registry Bridge receives signal
# bridge.layersAdded([layer])
#   → Creates QgsLayerTreeLayer node
#   → Inserts into tree
#   → Emits: tree.addedChildren signal

# 5. Layer Tree Model receives signal
# model.nodeAddedChildren()
#   → Updates model structure
#   → Emits: model.beginInsertRows() / endInsertRows()
#   → View adds tree item

# 6. Layer Tree Map Canvas Bridge receives signal
# bridge.nodeAddedChildren()
#   → Collects visible layers
#   → Calls: canvas.setLayers(visible_layers)

# 7. Map Canvas receives update
# canvas.setLayers(layers)
#   → Connects to layer signals
#   → layer.repaintRequested.connect(canvas.refresh)
#   → canvas.refresh()
#   → Canvas renders new layer

# 8. All views now synchronized:
#   - Layer tree shows new layer
#   - Map canvas renders new layer
#   - Attribute table can show new layer (when selected)
#   - Processing results dialog shows output
```

**Synchronization Points:**
1. **Project signal**: `project.layersAdded` - Central synchronization point
2. **Bridge connections**: Bridges translate between component hierarchies
3. **Layer signals**: Layer signals propagate to connected views
4. **Automatic updates**: All views update automatically

### 7.2 Editing a Feature

**Complete Flow:**

```python
# 1. User edits attribute in attribute table
tableModel.setData(index, new_value)
#   → Calls: layer.changeAttributeValue(fid, field, new_value)

# 2. Layer updates edit buffer
layer.changeAttributeValue(fid, field, new_value)
#   → Updates: editBuffer.changeAttributeValue()
#   → Emits: layer.attributeValueChanged(fid, field, new_value)
#   → Emits: layer.repaintRequested()

# 3. Attribute Table Model receives signal
# model.attributeValueChanged(fid, field, new_value)
#   → Updates model data
#   → Emits: model.dataChanged(index, index)
#   → View updates cell (already updated, but confirms change)

# 4. Map Canvas receives signal
# canvas.refresh() (via repaintRequested signal)
#   → Clears render cache
#   → Triggers repaint
#   → Canvas shows updated feature

# 5. Renderer receives signal (if field affects symbology)
# renderer.attributeValueChanged(fid, field, new_value)
#   → Updates symbol for feature
#   → Emits: renderer.repaintRequested()
#   → Canvas refreshes with new symbol

# 6. All views now synchronized:
#   - Attribute table shows new value
#   - Map canvas shows updated feature
#   - Renderer shows updated symbology (if applicable)
```

**Synchronization Points:**
1. **Layer signal**: `layer.attributeValueChanged` - Central synchronization point
2. **Direct connections**: Views directly connected to layer signals
3. **Automatic updates**: All connected views update automatically
4. **No manual coordination**: No need to manually update each view

---

## 8. Benefits of Decoupled Architecture

### 8.1 Modularity

**Benefits:**
- **Independent components**: Each view is independent
- **Easy to add/remove**: Can add/remove views without affecting others
- **Clear boundaries**: Clear separation between components
- **Reusable**: Components can be reused in different contexts

### 8.2 Maintainability

**Benefits:**
- **Single responsibility**: Each component has one job
- **Easy to understand**: Clear component boundaries
- **Easy to modify**: Changes to one component don't affect others
- **Easy to test**: Components can be tested in isolation

### 8.3 Extensibility

**Benefits:**
- **Easy to add views**: Add new views by connecting to signals
- **Easy to add bridges**: Add new bridges for new synchronization needs
- **Plugin support**: Plugins can connect to signals without modifying core
- **Flexible architecture**: Architecture adapts to new requirements

### 8.4 Performance

**Benefits:**
- **Efficient updates**: Only affected views update
- **Batched updates**: Updates can be batched for performance
- **Lazy updates**: Updates can be deferred when appropriate
- **Minimal overhead**: Signal/slot mechanism is efficient

---

## 9. Conclusion

QGIS achieves **decoupled multi-view synchronization** through:

1. **Signal/Slot Communication**: Qt signals/slots provide decoupled communication
2. **Bridge Pattern**: Bridges handle synchronization between component hierarchies
3. **ID-Based References**: IDs provide stable references without tight coupling
4. **Weak References**: Weak references prevent circular dependencies
5. **Progressive Updates**: Updates propagate through signal chains automatically

This architecture provides:
- **Decoupling**: Views don't know about each other
- **Automatic synchronization**: All views stay synchronized automatically
- **Flexibility**: Easy to add/remove views and modify synchronization
- **Performance**: Efficient updates with minimal overhead
- **Maintainability**: Clear component boundaries and responsibilities

The signal/slot architecture, combined with bridge patterns, enables QGIS to maintain **consistent state across multiple views** while keeping components **loosely coupled** and **independently evolvable**. This design is particularly important for a complex GIS application where multiple views of the same data must stay synchronized without creating tight dependencies that would make the system brittle and hard to maintain.

