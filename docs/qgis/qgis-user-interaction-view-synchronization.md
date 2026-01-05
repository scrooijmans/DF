# QGIS User Interaction and View Synchronization

## Executive Summary

QGIS enables user interactions (selection, editing, feature creation) through **map tools** that capture user input and modify the underlying data model through an **edit buffer architecture**. Changes are propagated to all active views through a **signal-based event system** that automatically synchronizes the map canvas, attribute tables, layer tree, and other views. The system uses **edit sessions** with commit/rollback semantics to ensure data integrity, and **deferred updates** to batch multiple changes efficiently.

---

## 1. Map Tools Architecture

### 1.1 Map Tool Base Class

All interactive tools inherit from `QgsMapTool`:

```python
class QgsMapTool(QObject):
    """Base class for all map tools."""
    
    def __init__(self, canvas: QgsMapCanvas):
        super().__init__()
        self._canvas = canvas
        self._layer = None  # Target layer for tool
    
    def canvasPressEvent(self, event: QgsMapMouseEvent):
        """Called when mouse button pressed."""
        pass
    
    def canvasMoveEvent(self, event: QgsMapMouseEvent):
        """Called when mouse moves."""
        pass
    
    def canvasReleaseEvent(self, event: QgsMapMouseEvent):
        """Called when mouse button released."""
        pass
    
    def activate(self):
        """Called when tool is activated."""
        pass
    
    def deactivate(self):
        """Called when tool is deactivated."""
        pass
```

### 1.2 Selection Tools

**Select Features Tool:**

```python
class QgsMapToolSelectFeatures(QgsMapTool):
    """Tool for selecting features."""
    
    def canvasReleaseEvent(self, event: QgsMapMouseEvent):
        """Handle feature selection."""
        # Get click point
        point = self.toMapCoordinates(event.pos())
        
        # Find features at point
        layer = self._canvas.currentLayer()
        if not isinstance(layer, QgsVectorLayer):
            return
        
        # Select features
        request = QgsFeatureRequest()
        request.setFilterRect(QgsRectangle(point, point))
        
        features = layer.getFeatures(request)
        selected_fids = [f.id() for f in features]
        
        # Update selection
        if event.modifiers() & Qt.ControlModifier:
            # Add to selection (Ctrl+click)
            layer.select(selected_fids)
        elif event.modifiers() & Qt.ShiftModifier:
            # Remove from selection (Shift+click)
            layer.deselect(selected_fids)
        else:
            # Replace selection
            layer.selectByIds(selected_fids)
        
        # Layer emits selectionChanged signal
        # Views automatically update
```

**Selection Rectangle Tool:**

```python
class QgsMapToolSelectRectangle(QgsMapTool):
    """Tool for selecting features by rectangle."""
    
    def canvasPressEvent(self, event: QgsMapMouseEvent):
        """Start selection rectangle."""
        self._startPoint = self.toMapCoordinates(event.pos())
        self._rubberBand = QgsRubberBand(self._canvas, QgsWkbTypes.PolygonGeometry)
    
    def canvasMoveEvent(self, event: QgsMapMouseEvent):
        """Update selection rectangle."""
        current_point = self.toMapCoordinates(event.pos())
        rect = QgsRectangle(self._startPoint, current_point)
        
        # Update rubber band
        self._rubberBand.reset()
        self._rubberBand.addPoint(QgsPointXY(rect.xMinimum(), rect.yMinimum()))
        self._rubberBand.addPoint(QgsPointXY(rect.xMaximum(), rect.yMinimum()))
        self._rubberBand.addPoint(QgsPointXY(rect.xMaximum(), rect.yMaximum()))
        self._rubberBand.addPoint(QgsPointXY(rect.xMinimum(), rect.yMaximum()))
        self._rubberBand.close()
    
    def canvasReleaseEvent(self, event: QgsMapMouseEvent):
        """Complete selection rectangle."""
        end_point = self.toMapCoordinates(event.pos())
        rect = QgsRectangle(self._startPoint, end_point)
        
        # Find features in rectangle
        layer = self._canvas.currentLayer()
        if not isinstance(layer, QgsVectorLayer):
            return
        
        request = QgsFeatureRequest()
        request.setFilterRect(rect)
        
        features = layer.getFeatures(request)
        selected_fids = [f.id() for f in features]
        
        # Update selection
        layer.selectByIds(selected_fids)
        
        # Clean up rubber band
        self._rubberBand.reset()
```

### 1.3 Editing Tools

**Add Feature Tool:**

```python
class QgsMapToolAddFeature(QgsMapTool):
    """Tool for adding new features."""
    
    def __init__(self, canvas: QgsMapCanvas, layer: QgsVectorLayer):
        super().__init__(canvas)
        self._layer = layer
        self._capturing = False
        self._points = []
    
    def canvasPressEvent(self, event: QgsMapMouseEvent):
        """Handle click to add vertex."""
        if event.button() == Qt.LeftButton:
            point = self.toMapCoordinates(event.pos())
            self._points.append(point)
            
            # Update rubber band
            if not self._capturing:
                self._capturing = True
                self._rubberBand = QgsRubberBand(
                    self._canvas,
                    self._layer.geometryType()
                )
            
            self._rubberBand.addPoint(point)
    
    def canvasReleaseEvent(self, event: QgsMapMouseEvent):
        """Handle release to complete feature."""
        if event.button() == Qt.RightButton:
            # Right-click completes feature
            if len(self._points) >= self._minVertices():
                self._createFeature()
            else:
                # Cancel if not enough vertices
                self._cancel()
    
    def _createFeature(self):
        """Create feature from captured points."""
        if not self._layer.isEditable():
            return
        
        # Create geometry
        if self._layer.geometryType() == QgsWkbTypes.PointGeometry:
            geometry = QgsGeometry.fromPointXY(self._points[0])
        elif self._layer.geometryType() == QgsWkbTypes.LineGeometry:
            geometry = QgsGeometry.fromPolylineXY(self._points)
        elif self._layer.geometryType() == QgsWkbTypes.PolygonGeometry:
            geometry = QgsGeometry.fromPolygonXY([self._points])
        
        # Create feature
        feature = QgsFeature(self._layer.fields())
        feature.setGeometry(geometry)
        
        # Add to layer (triggers edit buffer update)
        fid = self._layer.addFeature(feature)
        
        # Clean up
        self._rubberBand.reset()
        self._points = []
        self._capturing = False
        
        # Layer emits featureAdded signal
        # Views automatically update
```

**Edit Feature Tool:**

```python
class QgsMapToolEditFeature(QgsMapTool):
    """Tool for editing existing features."""
    
    def canvasReleaseEvent(self, event: QgsMapMouseEvent):
        """Handle feature editing."""
        point = self.toMapCoordinates(event.pos())
        
        # Find feature at point
        layer = self._canvas.currentLayer()
        if not isinstance(layer, QgsVectorLayer) or not layer.isEditable():
            return
        
        # Get feature at point
        request = QgsFeatureRequest()
        request.setFilterRect(QgsRectangle(point, point))
        request.setLimit(1)
        
        features = list(layer.getFeatures(request))
        if not features:
            return
        
        feature = features[0]
        
        # Start editing feature
        self._editingFeature = feature
        self._editingFid = feature.id()
        
        # Show vertex editor
        self._showVertexEditor(feature)
    
    def _showVertexEditor(self, feature: QgsFeature):
        """Show vertex editor for feature."""
        # Create vertex editor panel
        editor = QgsVertexEditor()
        editor.setFeature(feature)
        
        # Connect to vertex changes
        editor.vertexChanged.connect(self._onVertexChanged)
        
        # Show editor
        editor.show()
    
    def _onVertexChanged(self, vertex_index: int, point: QgsPointXY):
        """Handle vertex coordinate change."""
        # Update feature geometry
        geometry = self._editingFeature.geometry()
        geometry.moveVertex(point.x(), point.y(), vertex_index)
        
        # Update feature in edit buffer
        self._layer.changeGeometry(self._editingFid, geometry)
        
        # Layer emits geometryChanged signal
        # Views automatically update
```

---

## 2. Edit Buffer Architecture

### 2.1 Edit Buffer Structure

The edit buffer stores all uncommitted changes:

```python
class QgsVectorLayerEditBuffer:
    """Edit buffer for vector layer."""
    
    def __init__(self, layer: QgsVectorLayer):
        self._layer = layer
        self._addedFeatures = []  # New features
        self._deletedFeatureIds = set()  # Deleted feature IDs
        self._changedAttributeValues = {}  # {fid: {field_index: value}}
        self._changedGeometries = {}  # {fid: geometry}
    
    def addFeature(self, feature: QgsFeature) -> int:
        """Add feature to buffer."""
        # Generate temporary ID
        fid = self._generateTemporaryId()
        feature.setId(fid)
        
        # Store in buffer
        self._addedFeatures.append(feature)
        
        return fid
    
    def deleteFeature(self, fid: int) -> bool:
        """Mark feature for deletion."""
        # Remove from added features if not yet committed
        self._addedFeatures = [f for f in self._addedFeatures if f.id() != fid]
        
        # Mark for deletion
        self._deletedFeatureIds.add(fid)
        
        return True
    
    def changeAttributeValue(self, fid: int, field_index: int, value: QVariant) -> bool:
        """Change attribute value in buffer."""
        if fid not in self._changedAttributeValues:
            self._changedAttributeValues[fid] = {}
        
        self._changedAttributeValues[fid][field_index] = value
        return True
    
    def changeGeometry(self, fid: int, geometry: QgsGeometry) -> bool:
        """Change geometry in buffer."""
        self._changedGeometries[fid] = geometry
        return True
```

### 2.2 Feature Retrieval with Edit Buffer

Features are retrieved from both provider and edit buffer:

```python
class QgsVectorLayer(QgsMapLayer):
    """Vector layer with edit buffer support."""
    
    def getFeature(self, fid: int) -> QgsFeature:
        """Get feature (from provider or edit buffer)."""
        # Check if feature is deleted
        if self._editBuffer and fid in self._editBuffer._deletedFeatureIds:
            return QgsFeature()  # Invalid feature
        
        # Check if feature is new (in edit buffer)
        if self._editBuffer:
            for feature in self._editBuffer._addedFeatures:
                if feature.id() == fid:
                    # Return feature from buffer (with any modifications)
                    return self._getModifiedFeature(feature)
        
        # Get from provider
        feature = self._dataProvider.getFeature(fid)
        
        # Apply modifications from buffer
        if self._editBuffer:
            feature = self._applyBufferModifications(feature)
        
        return feature
    
    def _applyBufferModifications(self, feature: QgsFeature) -> QgsFeature:
        """Apply edit buffer modifications to feature."""
        fid = feature.id()
        
        # Apply attribute changes
        if fid in self._editBuffer._changedAttributeValues:
            attrs = feature.attributes()
            for field_index, value in self._editBuffer._changedAttributeValues[fid].items():
                attrs[field_index] = value
            feature.setAttributes(attrs)
        
        # Apply geometry changes
        if fid in self._editBuffer._changedGeometries:
            feature.setGeometry(self._editBuffer._changedGeometries[fid])
        
        return feature
```

---

## 3. Signal-Based View Synchronization

### 3.1 Selection Signals

**Layer Selection Signals:**

```python
class QgsVectorLayer(QgsMapLayer):
    """Vector layer with selection support."""
    
    # Selection signals
    selectionChanged = pyqtSignal()  # Selection changed
    
    def selectByIds(self, fids: List[int]):
        """Select features by IDs."""
        self._selectedFeatureIds = set(fids)
        self.selectionChanged.emit()
    
    def select(self, fids: List[int]):
        """Add features to selection."""
        self._selectedFeatureIds.update(fids)
        self.selectionChanged.emit()
    
    def deselect(self, fids: List[int]):
        """Remove features from selection."""
        self._selectedFeatureIds.difference_update(fids)
        self.selectionChanged.emit()
```

**View Subscriptions:**

```python
class QgsMapCanvas(QWidget):
    """Map canvas view."""
    
    def __init__(self):
        super().__init__()
        self._layer = None
        
        # Connect to selection signals
        if self._layer:
            self._layer.selectionChanged.connect(self._onSelectionChanged)
    
    def _onSelectionChanged(self):
        """Handle selection change."""
        # Highlight selected features
        self._updateSelectionHighlight()
        
        # Repaint canvas
        self.refresh()
    
    def _updateSelectionHighlight(self):
        """Update selection highlight rendering."""
        if not self._layer:
            return
        
        # Get selected features
        selected_fids = self._layer.selectedFeatureIds()
        
        # Update selection renderer
        self._selectionRenderer.setSelectedFeatures(selected_fids)
```

**Attribute Table Selection Synchronization:**

```python
class QgsAttributeTableModel(QAbstractTableModel):
    """Attribute table model."""
    
    def __init__(self, layer: QgsVectorLayer):
        super().__init__()
        self._layer = layer
        
        # Connect to selection signals
        layer.selectionChanged.connect(self._onSelectionChanged)
    
    def _onSelectionChanged(self):
        """Handle selection change."""
        # Update selected rows
        selected_fids = self._layer.selectedFeatureIds()
        
        # Emit dataChanged for selected rows
        for fid in selected_fids:
            row = self._fidToRow.get(fid)
            if row is not None:
                index = self.index(row, 0)
                self.dataChanged.emit(index, index)
        
        # View automatically updates selection highlight
```

### 3.2 Edit Signals

**Feature Edit Signals:**

```python
class QgsVectorLayer(QgsMapLayer):
    """Vector layer with editing support."""
    
    # Edit signals
    featureAdded = pyqtSignal(int, QgsFeature)  # fid, feature
    featureDeleted = pyqtSignal(int)  # fid
    attributeValueChanged = pyqtSignal(int, int, QVariant)  # fid, field, value
    geometryChanged = pyqtSignal(int, QgsGeometry)  # fid, geometry
    
    def addFeature(self, feature: QgsFeature) -> int:
        """Add feature to edit buffer."""
        if not self.isEditable():
            return False
        
        # Add to edit buffer
        fid = self._editBuffer.addFeature(feature)
        
        # Emit signal
        self.featureAdded.emit(fid, feature)
        self.repaintRequested.emit()
        
        return fid
    
    def changeAttributeValue(self, fid: int, field_index: int, value: QVariant) -> bool:
        """Change attribute value in edit buffer."""
        if not self.isEditable():
            return False
        
        # Update edit buffer
        self._editBuffer.changeAttributeValue(fid, field_index, value)
        
        # Emit signal
        self.attributeValueChanged.emit(fid, field_index, value)
        self.repaintRequested.emit()
        
        return True
    
    def changeGeometry(self, fid: int, geometry: QgsGeometry) -> bool:
        """Change geometry in edit buffer."""
        if not self.isEditable():
            return False
        
        # Update edit buffer
        self._editBuffer.changeGeometry(fid, geometry)
        
        # Emit signal
        self.geometryChanged.emit(fid, geometry)
        self.repaintRequested.emit()
        
        return True
```

**View Updates from Edit Signals:**

```python
class QgsMapCanvas(QWidget):
    """Map canvas view."""
    
    def __init__(self):
        super().__init__()
        self._layer = None
        
        # Connect to edit signals
        if self._layer:
            self._layer.featureAdded.connect(self._onFeatureAdded)
            self._layer.featureDeleted.connect(self._onFeatureDeleted)
            self._layer.attributeValueChanged.connect(self._onAttributeChanged)
            self._layer.geometryChanged.connect(self._onGeometryChanged)
    
    def _onFeatureAdded(self, fid: int, feature: QgsFeature):
        """Handle feature addition."""
        # Canvas automatically repaints via repaintRequested signal
        # No additional action needed
        pass
    
    def _onFeatureDeleted(self, fid: int):
        """Handle feature deletion."""
        # Canvas automatically repaints via repaintRequested signal
        pass
    
    def _onAttributeChanged(self, fid: int, field_index: int, value: QVariant):
        """Handle attribute change."""
        # Check if attribute affects symbology
        if self._fieldAffectsSymbology(field_index):
            # Clear render cache for this feature
            self._clearFeatureCache(fid)
        
        # Canvas automatically repaints via repaintRequested signal
        pass
    
    def _onGeometryChanged(self, fid: int, geometry: QgsGeometry):
        """Handle geometry change."""
        # Clear render cache for this feature
        self._clearFeatureCache(fid)
        
        # Canvas automatically repaints via repaintRequested signal
        pass
```

**Attribute Table Updates:**

```python
class QgsAttributeTableModel(QAbstractTableModel):
    """Attribute table model."""
    
    def __init__(self, layer: QgsVectorLayer):
        super().__init__()
        self._layer = layer
        
        # Connect to edit signals
        layer.featureAdded.connect(self._onFeatureAdded)
        layer.featureDeleted.connect(self._onFeatureDeleted)
        layer.attributeValueChanged.connect(self._onAttributeChanged)
    
    def _onFeatureAdded(self, fid: int, feature: QgsFeature):
        """Handle feature addition."""
        # Add feature to model
        row = self.rowCount()
        self.beginInsertRows(QModelIndex(), row, row)
        
        self._features.append(feature)
        self._fidToRow[fid] = row
        
        self.endInsertRows()
    
    def _onFeatureDeleted(self, fid: int):
        """Handle feature deletion."""
        # Remove feature from model
        row = self._fidToRow.get(fid)
        if row is None:
            return
        
        self.beginRemoveRows(QModelIndex(), row, row)
        
        del self._features[row]
        del self._fidToRow[fid]
        
        # Update row mappings
        for fid, r in self._fidToRow.items():
            if r > row:
                self._fidToRow[fid] = r - 1
        
        self.endRemoveRows()
    
    def _onAttributeChanged(self, fid: int, field_index: int, value: QVariant):
        """Handle attribute change."""
        # Update model data
        row = self._fidToRow.get(fid)
        if row is None:
            return
        
        # Update feature attributes
        feature = self._features[row]
        attrs = feature.attributes()
        attrs[field_index] = value
        feature.setAttributes(attrs)
        
        # Emit dataChanged signal
        index = self.index(row, field_index)
        self.dataChanged.emit(index, index)
```

---

## 4. Complete Interaction Flow

### 4.1 Feature Creation Flow

**Complete flow from user click to view update:**

```
1. User clicks on map with Add Feature tool
   ↓
2. QgsMapToolAddFeature.canvasReleaseEvent()
   ↓
3. Tool creates QgsFeature with geometry
   ↓
4. Tool calls layer.addFeature(feature)
   ↓
5. QgsVectorLayer.addFeature()
   ├─> Checks layer.isEditable()
   ├─> Calls editBuffer.addFeature()
   └─> Emits signals:
       - featureAdded.emit(fid, feature)
       - repaintRequested.emit()
   ↓
6. Signal propagation:
   ├─> QgsMapCanvas.refresh() (via repaintRequested)
   │   └─> Canvas clears cache
   │   └─> Canvas.update() (schedules paintEvent)
   │
   ├─> QgsAttributeTableModel._onFeatureAdded() (via featureAdded)
   │   └─> Model adds feature to internal list
   │   └─> Model emits beginInsertRows() / endInsertRows()
   │   └─> View adds new row
   │
   └─> QgsFeatureRenderer._onFeatureAdded() (via featureAdded)
       └─> Renderer updates symbol cache
       └─> Renderer emits repaintRequested
   ↓
7. Qt event loop processes update()
   ↓
8. paintEvent() called
   ↓
9. Canvas renders all layers including new feature
   ↓
10. All views synchronized:
    - Map canvas shows new feature
    - Attribute table shows new row
    - Selection highlights new feature (if auto-selected)
```

### 4.2 Feature Editing Flow

**Complete flow from user edit to view update:**

```
1. User edits attribute in attribute table
   ↓
2. QgsAttributeTableModel.setData(index, value)
   ↓
3. Model calls layer.changeAttributeValue(fid, field_index, value)
   ↓
4. QgsVectorLayer.changeAttributeValue()
   ├─> Checks layer.isEditable()
   ├─> Calls editBuffer.changeAttributeValue()
   └─> Emits signals:
       - attributeValueChanged.emit(fid, field_index, value)
       - repaintRequested.emit()
   ↓
5. Signal propagation:
   ├─> QgsMapCanvas.refresh() (via repaintRequested)
   │   └─> Canvas clears cache if field affects symbology
   │   └─> Canvas.update()
   │
   ├─> QgsAttributeTableModel._onAttributeChanged() (via attributeValueChanged)
   │   └─> Model updates feature attributes
   │   └─> Model emits dataChanged(index, index)
   │   └─> View updates cell (already updated, confirms change)
   │
   └─> QgsFeatureRenderer._onAttributeChanged() (via attributeValueChanged)
       └─> Renderer checks if field affects symbology
       └─> If yes, updates symbol and emits repaintRequested
   ↓
6. Qt event loop processes update()
   ↓
7. paintEvent() called
   ↓
8. Canvas renders with updated attribute (if affects symbology)
   ↓
9. All views synchronized:
    - Map canvas shows updated feature (if symbology changed)
    - Attribute table shows updated value
    - Labels update (if field used for labeling)
```

### 4.3 Selection Flow

**Complete flow from user selection to view update:**

```
1. User clicks on map with Select tool
   ↓
2. QgsMapToolSelectFeatures.canvasReleaseEvent()
   ↓
3. Tool finds features at click point
   ↓
4. Tool calls layer.selectByIds(fids)
   ↓
5. QgsVectorLayer.selectByIds()
   ├─> Updates _selectedFeatureIds
   └─> Emits signal:
       - selectionChanged.emit()
   ↓
6. Signal propagation:
   ├─> QgsMapCanvas._onSelectionChanged() (via selectionChanged)
   │   └─> Canvas updates selection highlight
   │   └─> Canvas.refresh()
   │
   ├─> QgsAttributeTableModel._onSelectionChanged() (via selectionChanged)
   │   └─> Model emits dataChanged for selected rows
   │   └─> View highlights selected rows
   │
   └─> QgsFeatureForm._onSelectionChanged() (via selectionChanged)
       └─> Form updates to show selected feature
   ↓
7. All views synchronized:
    - Map canvas highlights selected features
    - Attribute table highlights selected rows
    - Feature form shows selected feature attributes
```

---

## 5. Commit and Rollback

### 5.1 Committing Changes

**Commit process:**

```python
class QgsVectorLayer(QgsMapLayer):
    """Vector layer with commit/rollback support."""
    
    def commitChanges(self) -> bool:
        """Commit edit buffer to data provider."""
        if not self.isEditable():
            return False
        
        # Emit before-commit signal
        self.beforeCommitChanges.emit()
        
        # Commit edit buffer
        success, errors = self._editBuffer.commitChanges()
        
        if not success:
            return False
        
        # Emit committed signals
        self.committedFeaturesAdded.emit(self.id(), added_fids)
        self.committedFeaturesRemoved.emit(self.id(), deleted_fids)
        self.committedAttributeValuesChanges.emit(self.id(), attribute_changes)
        self.committedGeometriesChanges.emit(self.id(), geometry_changes)
        
        # Clear edit buffer
        self._editBuffer = None
        
        # Emit after-commit signal
        self.afterCommitChanges.emit()
        self.editingStopped.emit()
        
        # Request repaint
        self.repaintRequested.emit()
        
        return True
```

**Edit Buffer Commit:**

```python
class QgsVectorLayerEditBuffer:
    """Edit buffer with commit support."""
    
    def commitChanges(self) -> Tuple[bool, List[str]]:
        """Commit all changes to data provider."""
        errors = []
        
        # 1. Commit added features
        if self._addedFeatures:
            success = self._layer.dataProvider().addFeatures(self._addedFeatures)
            if not success:
                errors.append("Failed to add features")
        
        # 2. Commit deleted features
        if self._deletedFeatureIds:
            success = self._layer.dataProvider().deleteFeatures(
                list(self._deletedFeatureIds)
            )
            if not success:
                errors.append("Failed to delete features")
        
        # 3. Commit attribute changes
        for fid, attrs in self._changedAttributeValues.items():
            success = self._layer.dataProvider().changeAttributeValues(
                {fid: attrs}
            )
            if not success:
                errors.append(f"Failed to update attributes for {fid}")
        
        # 4. Commit geometry changes
        for fid, geometry in self._changedGeometries.items():
            success = self._layer.dataProvider().changeGeometryValues(
                {fid: geometry}
            )
            if not success:
                errors.append(f"Failed to update geometry for {fid}")
        
        return len(errors) == 0, errors
```

### 5.2 Rollback

**Rollback process:**

```python
class QgsVectorLayer(QgsMapLayer):
    """Vector layer with rollback support."""
    
    def rollBack(self) -> bool:
        """Rollback edit buffer (discard changes)."""
        if not self.isEditable():
            return False
        
        # Emit before-rollback signal
        self.beforeRollBack.emit()
        
        # Clear edit buffer
        self._editBuffer = None
        
        # Emit after-rollback signal
        self.afterRollBack.emit()
        
        # Request repaint
        self.repaintRequested.emit()
        
        return True
```

**View Updates on Rollback:**

```python
class QgsAttributeTableModel(QAbstractTableModel):
    """Attribute table model."""
    
    def __init__(self, layer: QgsVectorLayer):
        super().__init__()
        self._layer = layer
        
        # Connect to rollback signal
        layer.afterRollBack.connect(self._onRollBack)
    
    def _onRollBack(self):
        """Handle rollback."""
        # Reload all features from provider
        self.reload()
        
        # Emit modelReset signal
        self.beginResetModel()
        # ... reload features ...
        self.endResetModel()
        
        # View automatically updates
```

---

## 6. Best Practices

### 6.1 Edit Session Management

```python
# ✅ GOOD: Use context manager for automatic commit/rollback
with edit(layer):
    layer.addFeature(feature1)
    layer.addFeature(feature2)
    layer.changeAttributeValue(fid, field, value)
    # Automatically commits on success
    # Automatically rolls back on exception

# ❌ BAD: Manual commit without error handling
layer.startEditing()
layer.addFeature(feature)
layer.commitChanges()  # May fail, no error handling
```

### 6.2 Signal Connections

```python
# ✅ GOOD: Connect to signals for view updates
layer.featureAdded.connect(self._onFeatureAdded)
layer.selectionChanged.connect(self._onSelectionChanged)

# ❌ BAD: Polling for changes
def checkForChanges(self):
    # Inefficient polling
    if layer.featureCount() != self._lastCount:
        self.refresh()
```

### 6.3 Batch Operations

```python
# ✅ GOOD: Batch multiple changes
canvas.freeze(True)
with edit(layer):
    for feature in features:
        layer.addFeature(feature)
canvas.freeze(False)
# Single repaint after all changes

# ❌ BAD: Individual commits
for feature in features:
    with edit(layer):
        layer.addFeature(feature)
    # Multiple repaints (very slow)
```

---

## 7. Conclusion

QGIS user interaction and view synchronization uses:

1. **Map Tools**: Capture user input (clicks, drags, keyboard)
2. **Edit Buffer**: Stores uncommitted changes in memory
3. **Signal-Based Propagation**: Changes propagate through Qt signals
4. **Automatic View Updates**: Views subscribe to signals and update automatically
5. **Commit/Rollback**: Changes committed or rolled back atomically

This architecture provides:
- **Responsiveness**: Changes visible immediately during editing
- **Data Integrity**: Edit buffer ensures atomic commits
- **View Consistency**: All views stay synchronized automatically
- **User Control**: Users decide when to commit changes
- **Error Handling**: Failed commits preserve state for correction

The key insight is that **user interactions modify the data model through edit buffers**, and **views automatically stay synchronized through signal subscriptions**. This decoupled architecture ensures that all views (map canvas, attribute tables, forms, panels) remain consistent without requiring direct references between components.

