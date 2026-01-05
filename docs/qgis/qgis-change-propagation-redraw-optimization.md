# QGIS Change Propagation and Redraw Optimization

## Executive Summary

QGIS propagates changes in layer data or style to active views through a **signal-based event system** that triggers redraws only when necessary. The system uses **deferred repaints**, **render caching**, **selective updates**, and **batch operations** to avoid unnecessary recomputation when multiple layers are present. Views subscribe to layer signals and refresh only when their subscribed layers change, while render caches prevent redundant rendering of unchanged layers.

---

## 1. Signal-Based Change Propagation

### 1.1 Layer Signal Architecture

All layer types emit signals when data or style changes:

```python
class QgsMapLayer(QObject):
    """Base class for all layer types."""
    
    # Common signals (all layer types)
    repaintRequested = pyqtSignal(bool)      # Request repaint (deferred flag)
    styleChanged = pyqtSignal()               # Style/symbology changed
    extentChanged = pyqtSignal()              # Spatial extent changed
    configChanged = pyqtSignal()              # Configuration changed
    nameChanged = pyqtSignal(str)            # Layer name changed
    idChanged = pyqtSignal(str)               # Layer ID changed
    legendChanged = pyqtSignal()             # Legend changed
    dataChanged = pyqtSignal()               # Data changed (generic)
```

**Vector Layer Specific Signals:**

```python
class QgsVectorLayer(QgsMapLayer):
    """Vector layer with editing support."""
    
    # Edit signals
    featureAdded = pyqtSignal(int, QgsFeature)           # fid, feature
    featureDeleted = pyqtSignal(int)                     # fid
    attributeValueChanged = pyqtSignal(int, int, QVariant)  # fid, field, value
    geometryChanged = pyqtSignal(int, QgsGeometry)       # fid, geometry
    
    # Commit signals
    committedFeaturesAdded = pyqtSignal(str, list)        # layerId, fids
    committedFeaturesRemoved = pyqtSignal(str, list)     # layerId, fids
    committedAttributeValuesChanges = pyqtSignal(str, dict)  # layerId, changes
    committedGeometriesChanges = pyqtSignal(str, dict)   # layerId, changes
    
    # Editing state signals
    editingStarted = pyqtSignal()
    editingStopped = pyqtSignal()
    beforeCommitChanges = pyqtSignal()
    afterCommitChanges = pyqtSignal()
```

**Raster Layer Specific Signals:**

```python
class QgsRasterLayer(QgsMapLayer):
    """Raster layer."""
    
    rendererChanged = pyqtSignal()
    dataChanged = pyqtSignal()
```

### 1.2 View Signal Subscriptions

Views subscribe to relevant layer signals:

```python
class QgsMapCanvas(QWidget):
    """Map canvas view."""
    
    def setLayers(self, layers: List[QgsMapLayer]):
        """Set layers to render."""
        # Disconnect old layers
        for layer in self._layers:
            layer.repaintRequested.disconnect(self.refresh)
            layer.styleChanged.disconnect(self.refresh)
            layer.extentChanged.disconnect(self.onExtentChanged)
        
        # Connect new layers
        for layer in layers:
            # Subscribe to repaint requests
            layer.repaintRequested.connect(self.refresh)
            
            # Subscribe to style changes
            layer.styleChanged.connect(self.refresh)
            
            # Subscribe to extent changes
            layer.extentChanged.connect(self.onExtentChanged)
            
            # Subscribe to type-specific signals
            if isinstance(layer, QgsVectorLayer):
                layer.featureAdded.connect(self.onFeatureAdded)
                layer.featureDeleted.connect(self.onFeatureDeleted)
```

**Attribute Table Signal Subscriptions:**

```python
class QgsAttributeTableModel(QAbstractTableModel):
    """Attribute table model."""
    
    def __init__(self, layer: QgsVectorLayer):
        super().__init__()
        self._layer = layer
        
        # Subscribe to edit signals
        layer.featureAdded.connect(self.featureAdded)
        layer.featureDeleted.connect(self.featureDeleted)
        layer.attributeValueChanged.connect(self.attributeValueChanged)
        layer.geometryChanged.connect(self.geometryChanged)
        
        # Subscribe to commit signals
        layer.committedFeaturesAdded.connect(self.committedFeaturesAdded)
        layer.committedFeaturesRemoved.connect(self.committedFeaturesRemoved)
        layer.committedAttributeValuesChanges.connect(
            self.committedAttributeValuesChanges
        )
```

---

## 2. Redraw Trigger Mechanisms

### 2.1 Immediate vs Deferred Repaints

QGIS uses **deferred repaints** to batch multiple updates:

```python
class QgsMapLayer(QObject):
    """Base class for all layer types."""
    
    def triggerRepaint(self, deferred: bool = False):
        """
        Request repaint of views.
        
        Args:
            deferred: If True, repaint is deferred to next event loop iteration
        """
        # Emit repaint request signal
        self.repaintRequested.emit(deferred)
        
        # If not deferred, also emit style changed
        if not deferred:
            self.styleChanged.emit()
```

**Deferred Repaint Benefits:**

```python
# Multiple rapid changes
layer.addFeature(feature1)  # Emits repaintRequested(deferred=True)
layer.addFeature(feature2)  # Emits repaintRequested(deferred=True)
layer.addFeature(feature3)  # Emits repaintRequested(deferred=True)

# Canvas receives multiple signals but only repaints once
# All three features appear in single repaint
```

### 2.2 Canvas Refresh Process

When a repaint is triggered, the canvas follows this process:

```python
class QgsMapCanvas(QWidget):
    """Map rendering widget."""
    
    def refresh(self):
        """
        Refresh canvas display.
        
        Process:
        1. Clear render cache (invalidates cached renders)
        2. Update map settings
        3. Trigger Qt widget repaint (schedules paintEvent)
        4. Emit refresh signal
        """
        # 1. Clear render cache
        self._mapSettings.clearCache()
        
        # 2. Update map settings with current layers
        self._mapSettings.setLayers(self._layers)
        
        # 3. Trigger Qt widget repaint (deferred to event loop)
        self.update()  # Schedules paintEvent() call
        
        # 4. Emit refresh signal
        self.mapCanvasRefreshed.emit()
    
    def paintEvent(self, event: QPaintEvent):
        """
        Qt paint event - actually renders layers.
        
        Called by Qt event loop after update() is called.
        """
        # Create render context
        context = QgsRenderContext.fromMapSettings(self._mapSettings)
        
        # Render all layers
        for layer in self._layers:
            if not layer.isVisible():
                continue
            
            # Create layer renderer
            renderer = layer.createMapRenderer(context)
            
            # Render layer (uses cache if available)
            renderer.render()
```

### 2.3 Signal Chain Example

**Complete change propagation flow:**

```
1. User edits feature
   ↓
2. QgsVectorLayer.changeAttributeValue(fid, field, value)
   ↓
3. QgsVectorLayerEditBuffer.changeAttributeValue()
   ↓
4. Layer emits signals:
   - attributeValueChanged.emit(fid, field, value)
   - repaintRequested.emit(deferred=True)
   ↓
5. Signal propagation:
   ├─> QgsMapCanvas.refresh() (via repaintRequested signal)
   │   └─> Canvas clears cache
   │   └─> Canvas.update() (schedules paintEvent)
   │
   ├─> QgsAttributeTableModel.attributeValueChanged() (via attributeValueChanged signal)
   │   └─> Model updates cell value
   │   └─> Model emits dataChanged signal
   │   └─> View updates cell display
   │
   └─> QgsFeatureRenderer.attributeValueChanged() (via attributeValueChanged signal)
       └─> Renderer updates symbol if field affects style
       └─> Renderer emits repaintRequested
       └─> Canvas refreshes again
   ↓
6. Qt event loop processes update()
   ↓
7. paintEvent() called
   ↓
8. Layers rendered with updated data
```

---

## 3. Avoiding Unnecessary Recomputation

### 3.1 Render Caching

QGIS caches rendered layers to avoid redundant rendering:

```python
class QgsMapSettings:
    """Map rendering settings with cache support."""
    
    def __init__(self):
        self._cache = {}  # Cache of rendered layers
        self._cacheKey = None  # Cache key (extent, scale, etc.)
    
    def clearCache(self):
        """Clear render cache."""
        self._cache.clear()
        self._cacheKey = None
    
    def cacheKey(self) -> str:
        """Generate cache key from current settings."""
        # Key includes: extent, scale, CRS, DPI, layer IDs
        key_parts = [
            str(self.extent()),
            str(self.scale()),
            str(self.destinationCrs().authid()),
            str(self.outputDpi()),
            ','.join([layer.id() for layer in self.layers()])
        ]
        return '|'.join(key_parts)
    
    def getCachedRender(self, layer: QgsMapLayer) -> Optional[QImage]:
        """Get cached render for layer."""
        cache_key = self.cacheKey()
        layer_cache = self._cache.get(cache_key, {})
        return layer_cache.get(layer.id())
    
    def setCachedRender(self, layer: QgsMapLayer, image: QImage):
        """Cache rendered layer."""
        cache_key = self.cacheKey()
        if cache_key not in self._cache:
            self._cache[cache_key] = {}
        self._cache[cache_key][layer.id()] = image
```

**Cache Usage in Rendering:**

```python
class QgsMapCanvas(QWidget):
    """Map rendering widget."""
    
    def paintEvent(self, event: QPaintEvent):
        """Render layers with caching."""
        context = QgsRenderContext.fromMapSettings(self._mapSettings)
        
        for layer in self._layers:
            if not layer.isVisible():
                continue
            
            # Check cache first
            cached_image = self._mapSettings.getCachedRender(layer)
            
            if cached_image and not layer.needsRepaint():
                # Use cached render
                painter = QPainter(self)
                painter.drawImage(0, 0, cached_image)
            else:
                # Render layer
                renderer = layer.createMapRenderer(context)
                renderer.render()
                
                # Cache rendered result
                rendered_image = self.captureLayerRender(layer)
                self._mapSettings.setCachedRender(layer, rendered_image)
```

### 3.2 Selective Layer Updates

Only changed layers trigger repaints:

```python
class QgsMapLayer(QObject):
    """Base class for all layer types."""
    
    def __init__(self):
        super().__init__()
        self._needsRepaint = False  # Flag indicating layer needs repaint
    
    def needsRepaint(self) -> bool:
        """Check if layer needs repaint."""
        return self._needsRepaint
    
    def setNeedsRepaint(self, needs: bool = True):
        """Set repaint flag."""
        self._needsRepaint = needs
    
    def triggerRepaint(self, deferred: bool = False):
        """Request repaint."""
        self._needsRepaint = True
        self.repaintRequested.emit(deferred)
```

**Canvas Only Repaints Changed Layers:**

```python
class QgsMapCanvas(QWidget):
    """Map rendering widget."""
    
    def refresh(self):
        """Refresh canvas - only repaints layers that need it."""
        # Clear cache for layers that need repaint
        for layer in self._layers:
            if layer.needsRepaint():
                self._mapSettings.clearCacheForLayer(layer)
                layer.setNeedsRepaint(False)
        
        # Trigger repaint
        self.update()
    
    def paintEvent(self, event: QPaintEvent):
        """Render layers - uses cache for unchanged layers."""
        context = QgsRenderContext.fromMapSettings(self._mapSettings)
        
        for layer in self._layers:
            if not layer.isVisible():
                continue
            
            # Check if layer has cached render
            cached_image = self._mapSettings.getCachedRender(layer)
            
            if cached_image:
                # Use cached render (no recomputation)
                painter = QPainter(self)
                painter.drawImage(0, 0, cached_image)
            else:
                # Render layer (only if not cached)
                renderer = layer.createMapRenderer(context)
                renderer.render()
                
                # Cache result
                rendered_image = self.captureLayerRender(layer)
                self._mapSettings.setCachedRender(layer, rendered_image)
```

### 3.3 Batch Signal Processing

Multiple signals are batched to avoid multiple repaints:

```python
class QgsMapCanvas(QWidget):
    """Map rendering widget."""
    
    def __init__(self, parent=None):
        super().__init__(parent)
        self._refreshPending = False  # Flag for pending refresh
        self._refreshTimer = QTimer()
        self._refreshTimer.setSingleShot(True)
        self._refreshTimer.timeout.connect(self._doRefresh)
    
    def refresh(self):
        """
        Schedule refresh (batches multiple refresh requests).
        
        If multiple layers emit repaintRequested in quick succession,
        only one repaint occurs.
        """
        # Mark refresh as pending
        self._refreshPending = True
        
        # Start/restart timer (batches rapid updates)
        self._refreshTimer.start(10)  # 10ms delay
    
    def _doRefresh(self):
        """Actually perform refresh (called by timer)."""
        if not self._refreshPending:
            return
        
        # Clear pending flag
        self._refreshPending = False
        
        # Clear cache
        self._mapSettings.clearCache()
        
        # Trigger repaint
        self.update()
```

**Example: Batch Processing**

```python
# Multiple rapid changes
layer1.addFeature(feature1)  # Emits repaintRequested → refresh() called
layer2.changeAttribute(...)   # Emits repaintRequested → refresh() called (restarts timer)
layer3.setRenderer(...)       # Emits repaintRequested → refresh() called (restarts timer)

# Timer fires after 10ms
# → Only one repaint occurs
# → All three layers rendered in single paintEvent()
```

### 3.4 Extent-Based Optimization

Only layers intersecting visible extent are rendered:

```python
class QgsMapCanvas(QWidget):
    """Map rendering widget."""
    
    def paintEvent(self, event: QPaintEvent):
        """Render only visible layers in visible extent."""
        context = QgsRenderContext.fromMapSettings(self._mapSettings)
        visible_extent = context.extent()
        
        for layer in self._layers:
            # Skip invisible layers
            if not layer.isVisible():
                continue
            
            # Skip layers outside visible extent
            layer_extent = layer.extent()
            if not layer_extent.intersects(visible_extent):
                continue  # Layer not visible, skip rendering
            
            # Render layer
            renderer = layer.createMapRenderer(context)
            renderer.render()
```

### 3.5 Scale-Dependent Rendering

Layers can be skipped at certain scales:

```python
class QgsMapCanvas(QWidget):
    """Map rendering widget."""
    
    def paintEvent(self, event: QPaintEvent):
        """Render layers with scale-dependent visibility."""
        context = QgsRenderContext.fromMapSettings(self._mapSettings)
        scale = context.scale()
        
        for layer in self._layers:
            if not layer.isVisible():
                continue
            
            # Check scale-dependent visibility
            if hasattr(layer, 'minimumScale'):
                if scale < layer.minimumScale():
                    continue  # Layer not visible at this scale
            
            if hasattr(layer, 'maximumScale'):
                if scale > layer.maximumScale():
                    continue  # Layer not visible at this scale
            
            # Render layer
            renderer = layer.createMapRenderer(context)
            renderer.render()
```

---

## 4. Multi-Layer Optimization Strategies

### 4.1 Layer-Level Caching

Each layer maintains its own render cache:

```python
class QgsVectorLayer(QgsMapLayer):
    """Vector layer with render caching."""
    
    def __init__(self):
        super().__init__()
        self._renderCache = {}  # Cache of rendered features
    
    def clearRenderCache(self):
        """Clear layer render cache."""
        self._renderCache.clear()
    
    def getCachedFeatureRender(self, feature_id: int, 
                               context: QgsRenderContext) -> Optional[QImage]:
        """Get cached render for feature."""
        cache_key = self._generateCacheKey(feature_id, context)
        return self._renderCache.get(cache_key)
    
    def cacheFeatureRender(self, feature_id: int, context: QgsRenderContext, 
                          image: QImage):
        """Cache feature render."""
        cache_key = self._generateCacheKey(feature_id, context)
        self._renderCache[cache_key] = image
```

### 4.2 Incremental Updates

Only changed features are re-rendered:

```python
class QgsVectorLayerRenderer(QgsMapRendererJob):
    """Vector layer renderer with incremental updates."""
    
    def render(self):
        """Render layer with incremental updates."""
        context = self.context()
        layer = self.layer()
        
        # Get changed features since last render
        changed_features = layer.getChangedFeatures()
        
        if not changed_features:
            # No changes, use cached render
            cached_image = self.getCachedRender()
            if cached_image:
                self.drawCachedImage(cached_image)
                return
        
        # Render only changed features
        for feature in changed_features:
            # Clear old render for feature
            self.clearFeatureCache(feature.id())
            
            # Render feature
            self.renderFeature(feature, context)
            
            # Cache new render
            self.cacheFeatureRender(feature.id(), context)
```

### 4.3 Parallel Layer Rendering

Multiple layers can be rendered in parallel:

```python
class QgsMapCanvas(QWidget):
    """Map rendering widget with parallel rendering."""
    
    def paintEvent(self, event: QPaintEvent):
        """Render layers in parallel when possible."""
        context = QgsRenderContext.fromMapSettings(self._mapSettings)
        
        # Group layers by renderer type
        render_jobs = []
        for layer in self._layers:
            if not layer.isVisible():
                continue
            
            # Create render job
            renderer = layer.createMapRenderer(context)
            render_jobs.append(renderer)
        
        # Render in parallel (if supported)
        if self.supportsParallelRendering():
            # Use QThreadPool for parallel rendering
            from PyQt5.QtCore import QThreadPool
            
            pool = QThreadPool.globalInstance()
            for job in render_jobs:
                pool.start(job)
            
            # Wait for completion
            pool.waitForDone()
        else:
            # Sequential rendering
            for job in render_jobs:
                job.render()
```

### 4.4 Dirty Region Tracking

Only changed regions are repainted:

```python
class QgsMapCanvas(QWidget):
    """Map rendering widget with dirty region tracking."""
    
    def __init__(self, parent=None):
        super().__init__(parent)
        self._dirtyRegions = []  # List of dirty regions
    
    def markDirtyRegion(self, rect: QgsRectangle):
        """Mark region as dirty (needs repaint)."""
        self._dirtyRegions.append(rect)
        self.update()  # Trigger repaint
    
    def paintEvent(self, event: QPaintEvent):
        """Repaint only dirty regions."""
        if not self._dirtyRegions:
            # No dirty regions, repaint everything
            self._paintAllLayers()
            return
        
        # Repaint only dirty regions
        for dirty_rect in self._dirtyRegions:
            # Set clip region
            painter = QPainter(self)
            painter.setClipRect(self.mapRectToWidget(dirty_rect))
            
            # Render layers in dirty region
            self._paintLayersInRegion(dirty_rect)
        
        # Clear dirty regions
        self._dirtyRegions.clear()
```

---

## 5. Change Propagation Examples

### 5.1 Feature Edit Propagation

**Complete flow when editing a feature:**

```python
# 1. User edits feature attribute
layer.changeAttributeValue(fid, field_index, new_value)

# 2. Layer updates edit buffer
#    - QgsVectorLayerEditBuffer.changeAttributeValue()
#    - Updates feature in edit buffer

# 3. Layer emits signals
layer.attributeValueChanged.emit(fid, field_index, new_value)
layer.repaintRequested.emit(deferred=True)

# 4. Signal propagation
#    ├─> QgsMapCanvas.refresh()
#    │   └─> Canvas marks refresh as pending
#    │   └─> Timer starts (10ms delay)
#    │
#    ├─> QgsAttributeTableModel.attributeValueChanged()
#    │   └─> Model updates cell value
#    │   └─> Model emits dataChanged signal
#    │   └─> View updates cell (immediate)
#    │
#    └─> QgsFeatureRenderer.attributeValueChanged()
#        └─> Renderer checks if field affects symbology
#        └─> If yes, updates symbol and emits repaintRequested
#        └─> Canvas refreshes again

# 5. Timer fires (10ms later)
#    └─> Canvas._doRefresh() called
#    └─> Canvas clears cache for changed layer
#    └─> Canvas.update() called (schedules paintEvent)

# 6. Qt event loop processes update
#    └─> paintEvent() called

# 7. Rendering
#    └─> Canvas checks cache for each layer
#    └─> Changed layer: cache cleared, re-rendered
#    └─> Other layers: use cached renders (no recomputation)
#    └─> All layers composited and displayed
```

### 5.2 Style Change Propagation

**Complete flow when changing layer style:**

```python
# 1. User changes renderer
layer.setRenderer(new_renderer)

# 2. Layer updates renderer
#    - Stores new renderer
#    - Clears renderer cache

# 3. Layer emits signals
layer.rendererChanged.emit()
layer.configChanged.emit()
layer.repaintRequested.emit(deferred=True)

# 4. Signal propagation
#    ├─> QgsMapCanvas.refresh()
#    │   └─> Canvas clears cache for this layer
#    │   └─> Canvas schedules repaint
#    │
#    ├─> QgsLayerTreeModel.rendererChanged()
#    │   └─> Model updates legend icon
#    │   └─> View updates legend display
#    │
#    └─> QgsFeatureRenderer.clearCache()
#        └─> Renderer clears feature cache

# 5. Rendering
#    └─> Changed layer: re-rendered with new style
#    └─> Other layers: use cached renders (no recomputation)
```

### 5.3 Multiple Layer Changes

**When multiple layers change simultaneously:**

```python
# Multiple layers change
layer1.addFeature(feature1)    # Emits repaintRequested
layer2.changeAttribute(...)   # Emits repaintRequested
layer3.setRenderer(...)       # Emits repaintRequested

# Canvas receives three signals
#    └─> refresh() called three times
#    └─> Timer restarted each time (batches updates)
#    └─> After 10ms, timer fires once

# Single repaint occurs
#    └─> Cache cleared for layer1, layer2, layer3
#    └─> Other layers use cached renders
#    └─> Only changed layers re-rendered
```

---

## 6. Performance Optimizations Summary

### 6.1 Caching Strategies

1. **Render Cache**: Cached rendered layers prevent redundant rendering
2. **Feature Cache**: Cached feature renders for incremental updates
3. **Symbol Cache**: Cached symbols prevent redundant symbol creation
4. **Tile Cache**: Cached raster tiles for faster rendering

### 6.2 Update Batching

1. **Deferred Repaints**: Multiple repaint requests batched into single repaint
2. **Timer-Based Batching**: 10ms delay batches rapid updates
3. **Signal Queuing**: Qt signal queuing batches signal processing

### 6.3 Selective Updates

1. **Layer-Level**: Only changed layers trigger repaints
2. **Feature-Level**: Only changed features re-rendered
3. **Region-Level**: Only dirty regions repainted
4. **Extent-Based**: Only visible layers rendered

### 6.4 Parallel Processing

1. **Parallel Rendering**: Multiple layers rendered in parallel
2. **Thread Pool**: QThreadPool for concurrent render jobs
3. **Async Loading**: Data loaded asynchronously while rendering

---

## 7. Best Practices

### 7.1 Signal Emission

```python
# ✅ GOOD: Use deferred repaints for rapid changes
for i in range(100):
    layer.addFeature(features[i])
    # Each emits repaintRequested(deferred=True)
    # Canvas batches all updates into single repaint

# ❌ BAD: Immediate repaints for rapid changes
for i in range(100):
    layer.addFeature(features[i])
    layer.repaintRequested.emit(deferred=False)
    # Canvas repaints 100 times (very slow)
```

### 7.2 Cache Management

```python
# ✅ GOOD: Clear cache only when necessary
layer.setRenderer(new_renderer)
# Layer automatically clears cache

# ❌ BAD: Clear cache unnecessarily
layer.setRenderer(new_renderer)
canvas.clearCache()  # Clears cache for all layers (unnecessary)
```

### 7.3 Batch Operations

```python
# ✅ GOOD: Batch multiple changes
layer.startEditing()
layer.beginEditCommand("Batch edit")
for feature in features:
    layer.changeAttributeValue(...)
layer.endEditCommand()
layer.commitChanges()
# Single repaint after commit

# ❌ BAD: Individual commits
for feature in features:
    layer.startEditing()
    layer.changeAttributeValue(...)
    layer.commitChanges()
    # Multiple repaints (very slow)
```

---

## 8. Conclusion

QGIS change propagation and redraw optimization uses:

1. **Signal-Based Propagation**: Changes propagate through Qt signals
2. **Deferred Repaints**: Multiple updates batched into single repaint
3. **Render Caching**: Cached renders prevent redundant rendering
4. **Selective Updates**: Only changed layers/features/regions updated
5. **Batch Processing**: Rapid changes batched together
6. **Extent-Based Optimization**: Only visible layers rendered

This architecture provides:
- **Efficiency**: Minimal recomputation through caching
- **Responsiveness**: Deferred repaints keep UI responsive
- **Scalability**: Works well with many layers
- **Flexibility**: Views subscribe only to relevant signals
- **Performance**: Parallel rendering and selective updates

The key insight is that **QGIS avoids unnecessary recomputation** by:
- Caching rendered layers and only re-rendering when cache is invalid
- Batching multiple updates into single repaints
- Only updating changed layers/features/regions
- Using deferred repaints to batch rapid changes

This ensures that even with many layers, only the necessary work is performed, keeping the UI responsive and efficient.

