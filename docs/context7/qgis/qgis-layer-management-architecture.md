# QGIS Layer Management Architecture & Call Stack

This document explains in detail the QGIS architecture and call stack for adding/removing/updating layers in `QgsProject` (layer store), how the layer tree and map canvas react (signals/slots), and how edits/commits propagate to attribute tables and renderers.

## Overview

QGIS uses a **signal-slot architecture** to coordinate layer management across multiple components:

1. **QgsProject / QgsMapLayerStore**: Central layer registry
2. **QgsLayerTreeRegistryBridge**: Synchronizes project layers with layer tree
3. **QgsLayerTreeModel**: Manages layer tree structure
4. **QgsLayerTreeMapCanvasBridge**: Synchronizes layer tree with map canvas
5. **QgsMapCanvas**: Renders layers on the map
6. **QgsVectorLayer**: Manages vector data and editing
7. **Attribute Tables**: Display and edit attribute data
8. **Renderers**: Visualize layer data

**Key Principles:**

- **Signal-Slot Communication**: Qt signals/slots connect components
- **Bridge Pattern**: Bridge classes synchronize state between components
- **Edit Buffer**: Changes buffered before commit
- **Automatic Updates**: Components automatically react to changes via signals
- **Lazy Updates**: Canvas updates are deferred/batched for performance

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    QGIS Layer Management                     │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  QgsProject / QgsMapLayerStore                      │   │
│  │  - Central layer registry                          │   │
│  │  - Signals: layersAdded, layersRemoved            │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  QgsLayerTreeRegistryBridge                         │   │
│  │  - Synchronizes project ↔ layer tree                │   │
│  │  - Listens to: layersAdded, layersRemoved          │   │
│  │  - Updates: layer tree nodes                        │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  QgsLayerTreeModel                                  │   │
│  │  - Manages layer tree structure                     │   │
│  │  - Signals: nodeAddedChildren, nodeRemovedChildren  │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  QgsLayerTreeMapCanvasBridge                       │   │
│  │  - Synchronizes layer tree ↔ map canvas             │   │
│  │  - Listens to: layer tree changes                   │   │
│  │  - Updates: canvas layer set                        │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  QgsMapCanvas                                        │   │
│  │  - Renders layers                                    │   │
│  │  - Signals: layersChanged, mapCanvasRefreshed       │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  QgsVectorLayer                                      │   │
│  │  - Manages vector data                               │   │
│  │  - Signals: featureAdded, featureDeleted,            │   │
│  │            attributeValueChanged, repaintRequested   │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│         ┌───────┼───────┐                                    │
│         ▼       ▼       ▼                                    │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐                      │
│  │Attribute│ │Renderer │ │MapCanvas│                      │
│  │  Table  │ │         │ │         │                      │
│  └─────────┘ └─────────┘ └─────────┘                      │
└─────────────────────────────────────────────────────────────┘
```

## Adding Layers: Call Stack

### 1. User Action: Add Layer

```python
from qgis.core import QgsProject, QgsVectorLayer

# User adds layer via GUI or API
layer = QgsVectorLayer("/path/to/layer.shp", "My Layer", "ogr")
QgsProject.instance().addMapLayer(layer)
```

### 2. QgsProject.addMapLayer() Call Stack

```python
class QgsProject:
    """Manages project state including layers."""

    def addMapLayer(self, mapLayer, addToLegend=True):
        """
        Add layer to project.

        Call stack:
        1. Validate layer
        2. Add to layer store
        3. Emit signals
        4. Update layer tree (via bridge)
        5. Update map canvas (via bridge)
        """
        # 1. Validate layer
        if not mapLayer or not mapLayer.isValid():
            return None

        # 2. Add to layer store
        # QgsProject contains QgsMapLayerStore
        added_layer = self.layerStore().addMapLayer(mapLayer)

        if not added_layer:
            return None

        # 3. Emit project-level signals
        self.layersAdded.emit([added_layer])
        self.layerWasAdded.emit(added_layer)

        # 4. Add to legend (layer tree) if requested
        if addToLegend:
            self.layerTreeRoot().insertLayer(0, added_layer)

        # 5. Mark project as dirty
        self.setDirty(True)

        return added_layer
```

### 3. QgsMapLayerStore.addMapLayer() Call Stack

```python
class QgsMapLayerStore:
    """Stores and manages map layers."""

    def addMapLayer(self, layer):
        """
        Add layer to store.

        Call stack:
        1. Check if layer already exists
        2. Add to internal map
        3. Transfer ownership
        4. Emit store signals
        """
        # 1. Check if layer already exists
        if layer.id() in self.mapLayers():
            return self.mapLayer(layer.id())

        # 2. Add to internal storage
        self._layers[layer.id()] = layer

        # 3. Transfer ownership (Qt parent-child)
        layer.setParent(self)

        # 4. Emit store signals
        self.layersAdded.emit([layer])
        self.layerWasAdded.emit(layer)

        return layer
```

### 4. Signal Propagation: layersAdded

**Signal Chain:**

```
QgsMapLayerStore.layersAdded([layer])
    ↓
QgsProject.layersAdded([layer])  (forwarded)
    ↓
QgsLayerTreeRegistryBridge.layersAdded([layer])
    ↓
QgsLayerTreeModel.nodeAddedChildren()
    ↓
QgsLayerTreeMapCanvasBridge.setCanvasLayers()
    ↓
QgsMapCanvas.setLayers()
    ↓
QgsMapCanvas.refresh()
```

**Bridge Connection:**

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
        Called when layers are added to project.

        Call stack:
        1. Get insertion point
        2. Add layer nodes to tree
        3. Emit tree signals
        """
        for layer in layers:
            # 1. Get insertion point (current selection or root)
            insertion_point = self.layerInsertionPoint()

            # 2. Create layer tree node
            layer_node = QgsLayerTreeLayer(layer)

            # 3. Insert into tree at insertion point
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

        # 4. Emit bridge signal
        self.addedLayersToLayerTreeSignal.emit(layers)
```

### 5. Layer Tree Model Update

```python
class QgsLayerTreeModel:
    """Model for layer tree view."""

    def nodeAddedChildren(self, node, indexFrom, indexTo):
        """
        Called when children added to tree node.

        Call stack:
        1. Update model structure
        2. Emit model signals
        3. Update view
        """
        # 1. Update model structure
        parent_index = self.node2index(node)

        # 2. Emit model signals (Qt Model/View)
        self.beginInsertRows(parent_index, indexFrom, indexTo)
        # ... model update ...
        self.endInsertRows()

        # 3. Emit custom signals
        self.nodeAddedChildren.emit(node, indexFrom, indexTo)
```

### 6. Map Canvas Bridge Update

```python
class QgsLayerTreeMapCanvasBridge:
    """Synchronizes layer tree with map canvas."""

    def __init__(self, root, canvas, parent=None):
        # Connect to layer tree signals
        root.addedChildren.connect(self.nodeAddedChildren)
        root.removedChildren.connect(self.nodeRemovedChildren)
        root.visibilityChanged.connect(self.nodeVisibilityChanged)

    def nodeAddedChildren(self, node, indexFrom, indexTo):
        """
        Called when layer added to tree.

        Call stack:
        1. Collect visible layers from tree
        2. Update canvas layer set
        3. Refresh canvas
        """
        # 1. Collect visible layers
        visible_layers = self.collectVisibleLayers()

        # 2. Update canvas (deferred for performance)
        self.setCanvasLayers(visible_layers)

    def setCanvasLayers(self, layers):
        """
        Update canvas layer set.

        Call stack:
        1. Set canvas layers
        2. Emit canvas signals
        3. Trigger refresh
        """
        # 1. Set layers on canvas
        self.mapCanvas().setLayers(layers)

        # 2. Emit bridge signal
        self.canvasLayersChanged.emit()

        # 3. Refresh canvas (deferred)
        self.mapCanvas().refresh()
```

### 7. Map Canvas Update

```python
class QgsMapCanvas:
    """Map rendering widget."""

    def setLayers(self, layers):
        """
        Set layers to render.

        Call stack:
        1. Update layer list
        2. Connect to layer signals
        3. Emit canvas signals
        4. Schedule refresh
        """
        # 1. Update layer list
        self._layers = layers

        # 2. Connect to layer signals for auto-refresh
        for layer in layers:
            layer.repaintRequested.connect(self.refresh)
            layer.styleChanged.connect(self.refresh)

        # 3. Emit canvas signal
        self.layersChanged.emit()

        # 4. Schedule refresh (deferred)
        self.refresh()

    def refresh(self):
        """Refresh canvas display."""
        # Clear cache and repaint
        self._mapSettings.setLayers(self._layers)
        self.update()  # Qt widget update
```

## Removing Layers: Call Stack

### 1. User Action: Remove Layer

```python
# User removes layer via GUI or API
QgsProject.instance().removeMapLayer(layer_id)
```

### 2. QgsProject.removeMapLayer() Call Stack

```python
class QgsProject:
    """Manages project state including layers."""

    def removeMapLayer(self, layerId):
        """
        Remove layer from project.

        Call stack:
        1. Get layer from store
        2. Emit will-be-removed signals
        3. Remove from layer tree
        4. Remove from layer store
        5. Emit removed signals
        6. Update map canvas (via bridge)
        """
        # 1. Get layer
        layer = self.mapLayer(layerId)
        if not layer:
            return False

        # 2. Emit will-be-removed signals
        self.layerWillBeRemoved.emit(layerId)
        self.layersWillBeRemoved.emit([layerId])

        # 3. Remove from layer tree
        self.layerTreeRoot().findLayer(layerId).parent().removeChildNode(
            self.layerTreeRoot().findLayer(layerId)
        )

        # 4. Remove from layer store
        self.layerStore().removeMapLayer(layerId)

        # 5. Mark project as dirty
        self.setDirty(True)

        return True
```

### 3. QgsMapLayerStore.removeMapLayer() Call Stack

```python
class QgsMapLayerStore:
    """Stores and manages map layers."""

    def removeMapLayer(self, layerId):
        """
        Remove layer from store.

        Call stack:
        1. Get layer
        2. Emit will-be-removed signals
        3. Remove from internal map
        4. Delete layer (if owned)
        5. Emit removed signals
        """
        # 1. Get layer
        layer = self.mapLayer(layerId)
        if not layer:
            return False

        # 2. Emit will-be-removed signals
        self.layerWillBeRemoved.emit(layer)
        self.layersWillBeRemoved.emit([layerId])

        # 3. Remove from internal storage
        del self._layers[layerId]

        # 4. Delete layer (Qt parent-child deletion)
        layer.deleteLater()

        # 5. Emit removed signals
        self.layerRemoved.emit(layerId)
        self.layersRemoved.emit([layerId])

        return True
```

### 4. Signal Propagation: layersRemoved

**Signal Chain:**

```
QgsMapLayerStore.layersWillBeRemoved([layerId])
    ↓
QgsProject.layersWillBeRemoved([layerId])  (forwarded)
    ↓
QgsLayerTreeRegistryBridge.layersRemoved([layerId])
    ↓
QgsLayerTreeModel.nodeRemovedChildren()
    ↓
QgsLayerTreeMapCanvasBridge.setCanvasLayers()
    ↓
QgsMapCanvas.setLayers()
    ↓
QgsMapCanvas.refresh()
```

**Bridge Connection:**

```python
class QgsLayerTreeRegistryBridge:
    """Synchronizes project layers with layer tree."""

    def layersRemoved(self, layerIds):
        """
        Called when layers removed from project.

        Call stack:
        1. Find layer nodes in tree
        2. Remove nodes from tree
        3. Emit tree signals
        """
        for layerId in layerIds:
            # 1. Find layer node
            layer_node = self.root().findLayer(layerId)
            if layer_node:
                # 2. Remove from tree
                parent = layer_node.parent()
                parent.removeChildNode(layer_node)
```

## Updating Layers: Edit & Commit Flow

### 1. Starting Edit Session

```python
from qgis.core import QgsVectorLayer

layer = QgsProject.instance().mapLayer(layer_id)
layer.startEditing()
```

**Call Stack:**

```python
class QgsVectorLayer:
    """Vector layer with editing support."""

    def startEditing(self):
        """
        Start editing session.

        Call stack:
        1. Check if editing supported
        2. Create edit buffer
        3. Emit signals
        """
        # 1. Check provider support
        if not self.dataProvider().capabilities() & QgsVectorDataProvider.EditCapabilities:
            return False

        # 2. Create edit buffer
        self._editBuffer = QgsVectorLayerEditBuffer(self)

        # 3. Emit signal
        self.beforeEditingStarted.emit()
        self.editingStarted.emit()

        return True
```

### 2. Making Edits

**Adding Feature:**

```python
feature = QgsFeature(layer.fields())
feature.setGeometry(geometry)
feature.setAttributes([value1, value2, ...])
layer.addFeature(feature)
```

**Call Stack:**

```python
class QgsVectorLayer:
    """Vector layer with editing support."""

    def addFeature(self, feature):
        """
        Add feature to edit buffer.

        Call stack:
        1. Add to edit buffer
        2. Emit signals
        3. Trigger repaint
        """
        # 1. Add to edit buffer
        fid = self._editBuffer.addFeature(feature)

        # 2. Emit signals
        self.featureAdded.emit(fid, feature)

        # 3. Request repaint
        self.repaintRequested.emit()

        return fid
```

**Changing Attribute:**

```python
layer.changeAttributeValue(feature_id, field_index, new_value)
```

**Call Stack:**

```python
class QgsVectorLayer:
    """Vector layer with editing support."""

    def changeAttributeValue(self, fid, field, newValue):
        """
        Change attribute value in edit buffer.

        Call stack:
        1. Update edit buffer
        2. Emit signals
        3. Trigger repaint
        """
        # 1. Update edit buffer
        success = self._editBuffer.changeAttributeValue(fid, field, newValue)

        if success:
            # 2. Emit signals
            self.attributeValueChanged.emit(fid, field, newValue)

            # 3. Request repaint
            self.repaintRequested.emit()

        return success
```

**Deleting Feature:**

```python
layer.deleteFeature(feature_id)
```

**Call Stack:**

```python
class QgsVectorLayer:
    """Vector layer with editing support."""

    def deleteFeature(self, fid):
        """
        Delete feature from edit buffer.

        Call stack:
        1. Mark for deletion in edit buffer
        2. Emit signals
        3. Trigger repaint
        """
        # 1. Mark for deletion
        success = self._editBuffer.deleteFeature(fid)

        if success:
            # 2. Emit signals
            self.featureDeleted.emit(fid)

            # 3. Request repaint
            self.repaintRequested.emit()

        return success
```

### 3. Committing Changes

```python
layer.commitChanges()
```

**Call Stack:**

```python
class QgsVectorLayer:
    """Vector layer with editing support."""

    def commitChanges(self):
        """
        Commit changes to data provider.

        Call stack:
        1. Emit before-commit signal
        2. Commit edit buffer
        3. Emit committed signals
        4. Clear edit buffer
        5. Emit after-commit signal
        6. Trigger repaint
        """
        # 1. Emit before-commit signal
        self.beforeCommitChanges.emit()

        # 2. Commit edit buffer to provider
        success, errors = self._editBuffer.commitChanges()

        if not success:
            return False

        # 3. Emit committed signals
        self.committedFeaturesAdded.emit(self.id(), added_features)
        self.committedFeaturesRemoved.emit(self.id(), removed_fids)
        self.committedAttributeValuesChanges.emit(
            self.id(), changed_attributes
        )
        self.committedGeometriesChanges.emit(self.id(), changed_geometries)

        # 4. Clear edit buffer
        self._editBuffer = None

        # 5. Emit after-commit signal
        self.afterCommitChanges.emit()
        self.editingStopped.emit()

        # 6. Request repaint
        self.repaintRequested.emit()

        return True
```

### 4. Edit Buffer Commit Details

```python
class QgsVectorLayerEditBuffer:
    """Manages uncommitted edits."""

    def commitChanges(self):
        """
        Commit all edits to provider.

        Call stack:
        1. Commit added features
        2. Commit deleted features
        3. Commit attribute changes
        4. Commit geometry changes
        5. Return success status
        """
        errors = []

        # 1. Commit added features
        for feature in self._addedFeatures:
            if not self._layer.dataProvider().addFeatures([feature]):
                errors.append(f"Failed to add feature {feature.id()}")

        # 2. Commit deleted features
        for fid in self._deletedFeatureIds:
            if not self._layer.dataProvider().deleteFeatures([fid]):
                errors.append(f"Failed to delete feature {fid}")

        # 3. Commit attribute changes
        for fid, attrs in self._changedAttributeValues.items():
            if not self._layer.dataProvider().changeAttributeValues(
                {fid: attrs}
            ):
                errors.append(f"Failed to update attributes for {fid}")

        # 4. Commit geometry changes
        for fid, geom in self._changedGeometries.items():
            if not self._layer.dataProvider().changeGeometryValues(
                {fid: geom}
            ):
                errors.append(f"Failed to update geometry for {fid}")

        return len(errors) == 0, errors
```

## Edit Propagation: Signals & Slots

### Signal Chain for Attribute Change

```
User edits attribute in attribute table
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
│  Connected Slots:                        │
│  - QgsAttributeTableModel: update row   │
│  - QgsMapCanvas: refresh()              │
│  - QgsRenderer: update symbol           │
│  - Custom plugins: handle change        │
└─────────────────────────────────────────┘
```

### Attribute Table Update

```python
class QgsAttributeTableModel:
    """Model for attribute table view."""

    def __init__(self, layer, parent=None):
        # Connect to layer signals
        layer.attributeValueChanged.connect(self.attributeValueChanged)
        layer.featureAdded.connect(self.featureAdded)
        layer.featureDeleted.connect(self.featureDeleted)
        layer.committedAttributeValuesChanges.connect(
            self.committedAttributeValuesChanges
        )

    def attributeValueChanged(self, fid, field, value):
        """
        Called when attribute value changes.

        Call stack:
        1. Find feature row
        2. Update model data
        3. Emit model signals
        """
        # 1. Find feature row
        row = self._fidToRow.get(fid)
        if row is None:
            return

        # 2. Update model data
        field_index = self._layer.fields().indexFromName(field)
        index = self.index(row, field_index)
        self.setData(index, value)

        # 3. Emit model signal (Qt Model/View)
        self.dataChanged.emit(index, index)

    def featureAdded(self, fid, feature):
        """Called when feature added."""
        # Refresh model to include new feature
        self.reload()

    def featureDeleted(self, fid):
        """Called when feature deleted."""
        # Remove row from model
        row = self._fidToRow.get(fid)
        if row is not None:
            self.beginRemoveRows(QModelIndex(), row, row)
            # ... remove row ...
            self.endRemoveRows()

    def committedAttributeValuesChanges(self, layerId, changes):
        """Called after commit."""
        # Refresh model to reflect committed changes
        self.reload()
```

### Renderer Update

```python
class QgsFeatureRenderer:
    """Base class for feature renderers."""

    def __init__(self):
        # Connect to layer signals
        self._layer.attributeValueChanged.connect(self.attributeValueChanged)
        self._layer.featureAdded.connect(self.featureAdded)
        self._layer.featureDeleted.connect(self.featureDeleted)
        self._layer.repaintRequested.connect(self.repaintRequested)

    def attributeValueChanged(self, fid, field, value):
        """
        Called when attribute value changes.

        Call stack:
        1. Check if field affects symbology
        2. Update symbol for feature
        3. Request repaint
        """
        # 1. Check if field affects symbology
        if self.usesAttribute(field):
            # 2. Update symbol for feature
            symbol = self.symbolForFeature(feature)
            # ... update symbol based on new value ...

            # 3. Request repaint
            self._layer.repaintRequested.emit()

    def featureAdded(self, fid, feature):
        """Called when feature added."""
        # Renderer automatically includes new features
        # Request repaint to show new feature
        self._layer.repaintRequested.emit()

    def featureDeleted(self, fid):
        """Called when feature deleted."""
        # Renderer automatically excludes deleted features
        # Request repaint to remove feature
        self._layer.repaintRequested.emit()
```

### Map Canvas Update

```python
class QgsMapCanvas:
    """Map rendering widget."""

    def __init__(self):
        # Connect to layer signals
        for layer in self._layers:
            layer.repaintRequested.connect(self.refresh)
            layer.styleChanged.connect(self.refresh)

    def refresh(self):
        """
        Refresh canvas display.

        Call stack:
        1. Clear render cache
        2. Trigger repaint
        3. Emit refresh signal
        """
        # 1. Clear render cache
        self._mapSettings.clearCache()

        # 2. Trigger repaint (Qt widget update)
        self.update()

        # 3. Emit signal
        self.mapCanvasRefreshed.emit()
```

## Complete Edit Flow Example

### Full Edit & Commit Flow

```python
from qgis.core import (
    QgsVectorLayer,
    QgsFeature,
    QgsProject
)

# 1. Get layer
layer = QgsProject.instance().mapLayer(layer_id)

# 2. Start editing
layer.startEditing()
# Emits: beforeEditingStarted, editingStarted

# 3. Add feature
feature = QgsFeature(layer.fields())
feature.setGeometry(geometry)
feature.setAttributes([1, "Name", 100.0])
fid = layer.addFeature(feature)
# Emits: featureAdded(fid, feature)
# Triggers: repaintRequested
# Updates: Attribute table (via featureAdded signal)
# Updates: Renderer (via featureAdded signal)
# Updates: Map canvas (via repaintRequested signal)

# 4. Change attribute
layer.changeAttributeValue(fid, 1, "New Name")
# Emits: attributeValueChanged(fid, field, value)
# Triggers: repaintRequested
# Updates: Attribute table (via attributeValueChanged signal)
# Updates: Renderer (if field affects symbology)
# Updates: Map canvas (via repaintRequested signal)

# 5. Commit changes
success = layer.commitChanges()
# Emits: beforeCommitChanges
# Commits: Edit buffer to provider
# Emits: committedFeaturesAdded, committedAttributeValuesChanges
# Emits: afterCommitChanges, editingStopped
# Triggers: repaintRequested
# Updates: Attribute table (via committed signals)
# Updates: Renderer (via repaintRequested signal)
# Updates: Map canvas (via repaintRequested signal)
```

## Signal Summary

### QgsProject / QgsMapLayerStore Signals

| Signal                | Emitted When                | Parameters              |
| --------------------- | --------------------------- | ----------------------- |
| `layersAdded`         | Layers added to project     | `List[QgsMapLayer]`     |
| `layersRemoved`       | Layers removed from project | `List[str]` (layer IDs) |
| `layersWillBeRemoved` | Before layers removed       | `List[str]` (layer IDs) |
| `layerWasAdded`       | Single layer added          | `QgsMapLayer`           |
| `layerWillBeRemoved`  | Before single layer removed | `str` (layer ID)        |

### QgsVectorLayer Signals

| Signal                            | Emitted When                | Parameters                                     |
| --------------------------------- | --------------------------- | ---------------------------------------------- |
| `beforeEditingStarted`            | Before editing starts       | None                                           |
| `editingStarted`                  | Editing started             | None                                           |
| `editingStopped`                  | Editing stopped             | None                                           |
| `beforeCommitChanges`             | Before commit               | None                                           |
| `afterCommitChanges`              | After commit                | None                                           |
| `featureAdded`                    | Feature added to buffer     | `int` (fid), `QgsFeature`                      |
| `featureDeleted`                  | Feature deleted from buffer | `int` (fid)                                    |
| `attributeValueChanged`           | Attribute value changed     | `int` (fid), `int` (field), `QVariant` (value) |
| `geometryChanged`                 | Geometry changed            | `int` (fid), `QgsGeometry`                     |
| `repaintRequested`                | Layer needs repaint         | `bool` (deferred)                              |
| `committedFeaturesAdded`          | Features committed          | `str` (layerId), `List[QgsFeature]`            |
| `committedFeaturesRemoved`        | Features committed          | `str` (layerId), `List[int]` (fids)            |
| `committedAttributeValuesChanges` | Attributes committed        | `str` (layerId), `QgsChangedAttributesMap`     |
| `committedGeometriesChanges`      | Geometries committed        | `str` (layerId), `QgsChangedGeometriesMap`     |

### QgsLayerTreeModel Signals

| Signal                  | Emitted When               | Parameters                       |
| ----------------------- | -------------------------- | -------------------------------- |
| `nodeAddedChildren`     | Children added to node     | `QgsLayerTreeNode`, `int`, `int` |
| `nodeRemovedChildren`   | Children removed from node | `QgsLayerTreeNode`, `int`, `int` |
| `nodeVisibilityChanged` | Node visibility changed    | `QgsLayerTreeNode`               |
| `dataChanged`           | Model data changed         | `QModelIndex`, `QModelIndex`     |

### QgsMapCanvas Signals

| Signal               | Emitted When       | Parameters |
| -------------------- | ------------------ | ---------- |
| `layersChanged`      | Layer set changed  | None       |
| `mapCanvasRefreshed` | Canvas refreshed   | None       |
| `extentsChanged`     | Map extent changed | None       |

## Architecture Summary

### Component Responsibilities

1. **QgsProject / QgsMapLayerStore**
   - Central layer registry
   - Layer ownership management
   - Signal emission for layer changes

2. **QgsLayerTreeRegistryBridge**
   - Synchronizes project layers ↔ layer tree
   - Listens to project signals
   - Updates layer tree structure

3. **QgsLayerTreeModel**
   - Manages layer tree data model
   - Provides Qt Model/View interface
   - Emits tree change signals

4. **QgsLayerTreeMapCanvasBridge**
   - Synchronizes layer tree ↔ map canvas
   - Listens to tree signals
   - Updates canvas layer set

5. **QgsMapCanvas**
   - Renders layers
   - Listens to layer signals
   - Manages render cache

6. **QgsVectorLayer**
   - Manages vector data
   - Provides editing API
   - Emits edit signals

7. **QgsAttributeTableModel**
   - Manages attribute table data
   - Listens to layer edit signals
   - Updates table display

8. **QgsFeatureRenderer**
   - Visualizes features
   - Listens to layer signals
   - Updates symbology

### Signal Flow Patterns

1. **Layer Addition/Removal**: Project → Bridge → Tree → Bridge → Canvas
2. **Edit Changes**: Layer → Attribute Table / Renderer / Canvas
3. **Commit Changes**: Layer → Provider → Committed Signals → All Components
4. **Repaint Requests**: Layer → Canvas (direct connection)

This architecture ensures **decoupled components**, **automatic synchronization**, and **efficient updates** across the QGIS application.
