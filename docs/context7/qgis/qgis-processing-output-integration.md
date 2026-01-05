# QGIS Processing Output Integration & Post-Processing

This document explains in detail how QGIS Processing outputs are added back into the project (temporary layers, sinks, `QgsProcessingOutputLayerDefinition`, post-processing steps), and how QGIS triggers map canvas refresh and layer styling updates after an algorithm finishes.

## Overview

QGIS Processing uses a **post-processing pipeline** to integrate algorithm outputs into the project:

1. **Output Creation**: Algorithms create outputs (sinks, files, memory layers)
2. **Output Registration**: Outputs registered with `QgsProcessingContext`
3. **Post-Processing**: `postProcess()` method handles finalization
4. **Layer Loading**: Layers added to project via context
5. **Canvas Refresh**: Map canvas automatically refreshes
6. **Styling Application**: Default or custom styles applied

**Key Principles:**

- **Context-Based Loading**: `QgsProcessingContext` manages output layer loading
- **Deferred Loading**: Layers loaded after algorithm completes (main thread)
- **Temporary Layer Management**: Temporary layers stored in context
- **Automatic Integration**: Outputs automatically added to project and canvas
- **Styling Support**: Default styles applied via `QgsProcessingOutputLayerDefinition`

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│              Processing Algorithm Execution                 │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  processAlgorithm()                                 │   │
│  │  - Create feature sink                               │   │
│  │  - Write features                                    │   │
│  │  - Return output dictionary                          │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  postProcess()                                       │   │
│  │  - Finalize outputs                                  │   │
│  │  - Register layers with context                      │   │
│  │  - Apply styling                                     │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  QgsProcessingContext                               │   │
│  │  - layersToLoadOnCompletion()                       │   │
│  │  - temporaryLayerStore()                            │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Processing Framework (Main Thread)                  │   │
│  │  - Load layers from context                         │   │
│  │  - Add to QgsProject                                │   │
│  │  - Apply styles                                      │   │
│  │  - Refresh map canvas                                │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  QgsProject                                         │   │
│  │  - addMapLayer()                                    │   │
│  │  - Signals: layersAdded                             │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  QgsLayerTreeMapCanvasBridge                       │   │
│  │  - Updates canvas layers                           │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  QgsMapCanvas                                        │   │
│  │  - refresh()                                        │   │
│  │  - Layers rendered                                  │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Output Types & Creation

### 1. Feature Sink Outputs

**Creation in processAlgorithm():**

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingParameterFeatureSink,
    QgsFeatureSink
)

class MyAlgorithm(QgsProcessingAlgorithm):
    """Algorithm creating feature sink output."""

    OUTPUT = 'OUTPUT'

    def processAlgorithm(self, parameters, context, feedback):
        # Get input source
        source = self.parameterAsSource(parameters, 'INPUT', context)

        # Create feature sink
        (sink, dest_id) = self.parameterAsSink(
            parameters,
            self.OUTPUT,
            context,
            source.fields(),
            source.wkbType(),
            source.sourceCrs()
        )

        # Write features
        for feature in source.getFeatures():
            sink.addFeature(feature)

        # Return destination ID
        return {self.OUTPUT: dest_id}
```

**Sink Destination Types:**

```python
# File output
dest_id = "/path/to/output.shp"
# Creates: QgsVectorFileWriter → File on disk

# Memory output
dest_id = "memory:MyLayer"
# Creates: QgsVectorLayer (memory provider) → Temporary layer

# Temporary output
dest_id = "TEMPORARY_OUTPUT"
# Creates: Temporary file → Auto-deleted on project close
```

### 2. Raster Destination Outputs

```python
from qgis.core import QgsProcessingParameterRasterDestination

def processAlgorithm(self, parameters, context, feedback):
    # Get output path
    output_path = self.parameterAsOutputLayer(
        parameters, 'OUTPUT', context
    )

    # Create raster writer
    writer = QgsRasterFileWriter(output_path)
    # ... write raster data ...

    return {'OUTPUT': output_path}
```

### 3. Memory Layers

```python
def processAlgorithm(self, parameters, context, feedback):
    # Create memory layer
    memory_layer = QgsVectorLayer(
        "Point?crs=EPSG:4326&field=id:integer",
        "Memory Layer",
        "memory"
    )

    # Add features
    memory_layer.dataProvider().addFeatures([feature1, feature2])

    # Add to temporary store
    context.temporaryLayerStore().addMapLayer(memory_layer)

    return {'OUTPUT': memory_layer.id()}
```

## QgsProcessingOutputLayerDefinition

### Definition Structure

`QgsProcessingOutputLayerDefinition` encapsulates output layer settings:

```python
from qgis.core import QgsProcessingOutputLayerDefinition

class QgsProcessingOutputLayerDefinition:
    """Encapsulates settings for output layers."""

    def __init__(self):
        self.sink = None  # Layer identifier (path, memory ref, etc.)
        self.destinationName = ""  # Layer name for project
        self.destinationProject = None  # Target project (optional)
        self.createOptions = {}  # Layer creation options
        self.useRemapping = False  # Use remapping definition
        self.remappingDefinition = None  # Remapping definition
```

### Usage in Algorithms

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingOutputLayerDefinition,
    QgsProcessingContext
)

class MyAlgorithm(QgsProcessingAlgorithm):
    """Algorithm with output layer definition."""

    def processAlgorithm(self, parameters, context, feedback):
        # Create sink
        (sink, dest_id) = self.parameterAsSink(...)

        # Create output layer definition
        output_def = QgsProcessingOutputLayerDefinition()
        output_def.sink = dest_id
        output_def.destinationName = "My Output Layer"
        output_def.createOptions = {
            'fileEncoding': 'UTF-8',
            'layerName': 'output_features'
        }

        # Register with context
        context.addLayerToLoadOnCompletion(
            dest_id,
            QgsProcessingContext.LayerDetails(
                name=output_def.destinationName,
                crs=source.sourceCrs(),
                outputLayerDefinition=output_def
            )
        )

        return {self.OUTPUT: dest_id}
```

## Context Layer Loading Mechanism

### addLayerToLoadOnCompletion()

**Purpose**: Register output layers to be loaded after algorithm completes

```python
from qgis.core import (
    QgsProcessingContext,
    QgsProcessingContext.LayerDetails
)

class QgsProcessingContext:
    """Manages Processing execution context."""

    def addLayerToLoadOnCompletion(self, layerIdOrPath, layerDetails):
        """
        Register layer to load after algorithm completes.

        Args:
            layerIdOrPath: Layer ID, file path, or memory reference
            layerDetails: LayerDetails object with loading options
        """
        self._layersToLoad[layerIdOrPath] = layerDetails

    def layersToLoadOnCompletion(self):
        """
        Get all layers registered for loading.

        Returns:
            Dict[str, LayerDetails]: Map of layer identifiers to details
        """
        return self._layersToLoad
```

### LayerDetails Structure

```python
class QgsProcessingContext.LayerDetails:
    """Details for loading output layers."""

    def __init__(self, name="", crs=None, outputLayerDefinition=None):
        self.name = name  # Layer name in project
        self.crs = crs  # Coordinate reference system
        self.outputLayerDefinition = outputLayerDefinition  # Output definition
        self.loadAsDefaultStyle = False  # Load default style
        self.groupName = ""  # Layer tree group name
        self.forceName = False  # Force layer name
```

### Automatic Registration

**Feature Sink Registration:**

```python
def parameterAsSink(self, parameters, name, context, fields,
                    geometryType, crs, ...):
    """Create sink and register with context."""
    # ... create sink ...

    # Automatically register with context
    context.addLayerToLoadOnCompletion(
        dest_id,
        QgsProcessingContext.LayerDetails(
            name=os.path.basename(dest_id),
            crs=crs,
            outputLayerDefinition=output_def
        )
    )

    return sink, dest_id
```

## Post-Processing Steps

### postProcess() Method

**Purpose**: Finalize outputs and prepare for project integration

**Call Stack:**

```python
class QgsProcessingAlgorithm:
    """Base class for Processing algorithms."""

    def postProcess(self, context, feedback, runResult=True):
        """
        Post-process algorithm outputs.

        Called in main thread after runPrepared() completes.

        Call stack:
        1. Get output dictionary from runPrepared()
        2. Process each output
        3. Load layers from context
        4. Apply styling
        5. Return final results
        """
        if not runResult:
            return {}

        # 1. Get output dictionary (from runPrepared())
        outputs = self._outputs

        # 2. Process each output
        for output_name, output_value in outputs.items():
            output_def = self.outputDefinition(output_name)

            if isinstance(output_def, QgsProcessingOutputVectorLayer):
                # Process vector layer output
                self.postProcessVectorLayer(
                    output_value, output_def, context
                )
            elif isinstance(output_def, QgsProcessingOutputRasterLayer):
                # Process raster layer output
                self.postProcessRasterLayer(
                    output_value, output_def, context
                )

        # 3. Load layers from context (main thread)
        self.loadLayersFromContext(context)

        return outputs

    def loadLayersFromContext(self, context):
        """Load all registered layers from context."""
        layers_to_load = context.layersToLoadOnCompletion()

        for layer_id, layer_details in layers_to_load.items():
            # Load layer
            layer = self.loadOutputLayer(layer_id, layer_details, context)

            if layer:
                # Add to project
                QgsProject.instance().addMapLayer(layer, False)

                # Apply styling
                self.applyOutputStyling(layer, layer_details)
```

### Loading Output Layers

**Load from File:**

```python
def loadOutputLayer(self, layer_id, layer_details, context):
    """Load output layer from identifier."""
    # Check if it's a file path
    if os.path.exists(layer_id):
        # Determine layer type
        if layer_id.endswith(('.shp', '.gpkg', '.geojson')):
            layer = QgsVectorLayer(layer_id, layer_details.name, "ogr")
        elif layer_id.endswith(('.tif', '.tiff')):
            layer = QgsRasterLayer(layer_id, layer_details.name, "gdal")

        if layer and layer.isValid():
            # Set CRS if specified
            if layer_details.crs and layer_details.crs.isValid():
                layer.setCrs(layer_details.crs)

            return layer

    # Check if it's a memory layer
    if layer_id.startswith('memory:'):
        layer_name = layer_id[7:]
        layer = context.temporaryLayerStore().mapLayer(layer_name)
        if layer:
            layer.setName(layer_details.name)
            return layer

    # Check temporary store
    layer = context.temporaryLayerStore().mapLayer(layer_id)
    if layer:
        layer.setName(layer_details.name)
        return layer

    return None
```

### Applying Styling

**Default Style Application:**

```python
def applyOutputStyling(self, layer, layer_details):
    """Apply styling to output layer."""
    output_def = layer_details.outputLayerDefinition

    if not output_def:
        return

    # Check for default style file
    if output_def.loadAsDefaultStyle:
        style_path = self.getDefaultStylePath(layer)
        if style_path and os.path.exists(style_path):
            layer.loadNamedStyle(style_path)
            return

    # Check for algorithm-specific style
    algorithm_style = self.getAlgorithmStylePath(layer)
    if algorithm_style and os.path.exists(algorithm_style):
        layer.loadNamedStyle(algorithm_style)
        return

    # Apply default QGIS style
    self.applyDefaultQGISStyle(layer)

def getDefaultStylePath(self, layer):
    """Get default style path from Processing settings."""
    from qgis.core import QgsApplication

    settings = QgsApplication.processingSettings()

    if isinstance(layer, QgsVectorLayer):
        return settings.value('defaultVectorStyle')
    elif isinstance(layer, QgsRasterLayer):
        return settings.value('defaultRasterStyle')

    return None

def getAlgorithmStylePath(self, layer):
    """Get algorithm-specific style path."""
    algorithm_id = self.name()
    output_name = layer.name()

    # Check for algorithm-specific style
    style_path = f"{algorithm_id}_{output_name}.qml"
    if os.path.exists(style_path):
        return style_path

    return None
```

## Processing Framework Integration

### Main Thread Post-Processing

**Processing Framework Call Stack:**

```python
class QgsProcessingAlgRunnerTask:
    """Task for running Processing algorithms."""

    def run(self):
        """Run algorithm in background thread."""
        # 1. Prepare algorithm
        success = algorithm.prepare(parameters, context, feedback)

        if not success:
            return False

        # 2. Run algorithm (background thread)
        run_result = algorithm.runPrepared(parameters, context, feedback)

        # 3. Store results for post-processing
        self._runResult = run_result
        self._context = context

        return True

    def finished(self, result):
        """Called in main thread after run() completes."""
        if not result:
            return

        # 4. Post-process in main thread
        final_outputs = algorithm.postProcess(
            self._context,
            self._feedback,
            self._runResult
        )

        # 5. Load layers from context
        self.loadLayersFromContext(self._context)

        # 6. Emit completion signal
        self.executed.emit(final_outputs)
```

### Loading Layers from Context

**Framework Layer Loading:**

```python
def loadLayersFromContext(context):
    """Load all layers registered in context."""
    from qgis.core import QgsProject, QgsProcessingUtils

    layers_to_load = context.layersToLoadOnCompletion()

    loaded_layers = []

    for layer_id, layer_details in layers_to_load.items():
        # Load layer
        layer = QgsProcessingUtils.mapLayerFromString(
            layer_id,
            context,
            layer_details.outputLayerDefinition
        )

        if not layer or not layer.isValid():
            continue

        # Set layer name
        if layer_details.name:
            layer.setName(layer_details.name)

        # Set CRS
        if layer_details.crs and layer_details.crs.isValid():
            layer.setCrs(layer_details.crs)

        # Add to project
        project = context.project() or QgsProject.instance()
        added_layer = project.addMapLayer(layer, False)

        if added_layer:
            loaded_layers.append(added_layer)

            # Apply styling
            applyOutputLayerStyling(added_layer, layer_details)

    return loaded_layers
```

## Map Canvas Refresh

### Automatic Refresh Trigger

**Signal Chain:**

```
QgsProject.addMapLayer(layer)
    ↓
QgsProject.layersAdded.emit([layer])
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
    ↓
QgsMapCanvas.mapCanvasRefreshed.emit()
```

### Canvas Refresh Implementation

```python
class QgsMapCanvas:
    """Map rendering widget."""

    def setLayers(self, layers):
        """
        Set layers to render.

        Call stack:
        1. Update layer list
        2. Connect to layer signals
        3. Emit layersChanged signal
        4. Schedule refresh
        """
        # 1. Update layer list
        self._layers = layers

        # 2. Connect to layer signals
        for layer in layers:
            layer.repaintRequested.connect(self.refresh)
            layer.styleChanged.connect(self.refresh)

        # 3. Emit signal
        self.layersChanged.emit()

        # 4. Schedule refresh
        self.refresh()

    def refresh(self):
        """
        Refresh canvas display.

        Call stack:
        1. Clear render cache
        2. Update map settings
        3. Trigger Qt widget update
        4. Emit refresh signal
        """
        # 1. Clear render cache
        self._mapSettings.clearCache()

        # 2. Update map settings
        self._mapSettings.setLayers(self._layers)

        # 3. Trigger Qt widget update (repaint)
        self.update()

        # 4. Emit signal
        self.mapCanvasRefreshed.emit()
```

### Deferred Refresh

**Performance Optimization:**

```python
class QgsLayerTreeMapCanvasBridge:
    """Synchronizes layer tree with map canvas."""

    def __init__(self, root, canvas, parent=None):
        # Use timer for deferred updates
        self._updateTimer = QTimer()
        self._updateTimer.setSingleShot(True)
        self._updateTimer.timeout.connect(self.updateCanvasLayers)

    def nodeAddedChildren(self, node, indexFrom, indexTo):
        """Called when layer added to tree."""
        # Defer update to batch multiple changes
        self._updateTimer.start(100)  # 100ms delay

    def updateCanvasLayers(self):
        """Update canvas layer set."""
        # Collect visible layers
        visible_layers = self.collectVisibleLayers()

        # Update canvas
        self.mapCanvas().setLayers(visible_layers)
```

## Layer Styling Updates

### Style Application Flow

**Style Loading:**

```python
def applyOutputLayerStyling(layer, layer_details):
    """Apply styling to output layer."""
    output_def = layer_details.outputLayerDefinition

    if not output_def:
        # Apply default QGIS style
        applyDefaultStyle(layer)
        return

    # 1. Check for remapping definition
    if output_def.useRemapping and output_def.remappingDefinition:
        applyRemappingStyle(layer, output_def.remappingDefinition)
        return

    # 2. Check for default style file
    if output_def.loadAsDefaultStyle:
        style_path = getDefaultStylePath(layer)
        if style_path:
            layer.loadNamedStyle(style_path)
            layer.triggerRepaint()
            return

    # 3. Check for algorithm-specific style
    algorithm_style = getAlgorithmStylePath(layer)
    if algorithm_style:
        layer.loadNamedStyle(algorithm_style)
        layer.triggerRepaint()
        return

    # 4. Apply default QGIS style
    applyDefaultStyle(layer)

def applyDefaultStyle(layer):
    """Apply default QGIS style based on layer type."""
    if isinstance(layer, QgsVectorLayer):
        # Apply default vector style
        renderer = QgsSingleSymbolRenderer(
            QgsSymbol.defaultSymbol(layer.geometryType())
        )
        layer.setRenderer(renderer)
    elif isinstance(layer, QgsRasterLayer):
        # Apply default raster style
        layer.setDefaultContrastEnhancement()

    layer.triggerRepaint()
```

### Style Change Signals

**Renderer Update:**

```python
class QgsVectorLayer:
    """Vector layer with styling support."""

    def setRenderer(self, renderer):
        """
        Set layer renderer.

        Call stack:
        1. Set renderer
        2. Emit styleChanged signal
        3. Request repaint
        """
        # 1. Set renderer
        self._renderer = renderer

        # 2. Emit signal
        self.styleChanged.emit()

        # 3. Request repaint
        self.repaintRequested.emit()

def styleChanged(self):
    """
    Emitted when layer style changes.

    Connected to:
    - QgsMapCanvas: refresh()
    - QgsLayerTreeModel: updateLayerStyle()
    - Attribute table: update conditional styles
    """
    pass
```

## Complete Output Integration Example

### Full Algorithm with Output Integration

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingParameterFeatureSource,
    QgsProcessingParameterFeatureSink,
    QgsProcessingContext,
    QgsProcessingContext.LayerDetails,
    QgsProcessingOutputLayerDefinition
)

class BufferAlgorithm(QgsProcessingAlgorithm):
    """Algorithm demonstrating complete output integration."""

    INPUT = 'INPUT'
    OUTPUT = 'OUTPUT'

    def initAlgorithm(self, config=None):
        self.addParameter(
            QgsProcessingParameterFeatureSource(self.INPUT, 'Input layer')
        )
        self.addParameter(
            QgsProcessingParameterFeatureSink(
                self.OUTPUT,
                'Buffered layer'
            )
        )

    def processAlgorithm(self, parameters, context, feedback):
        """Process algorithm and create output."""
        # Get input
        source = self.parameterAsSource(parameters, self.INPUT, context)

        # Create sink (automatically registered with context)
        (sink, dest_id) = self.parameterAsSink(
            parameters,
            self.OUTPUT,
            context,
            source.fields(),
            source.wkbType(),
            source.sourceCrs()
        )

        # Process features
        for feature in source.getFeatures():
            # Buffer geometry
            if feature.geometry():
                buffered = feature.geometry().buffer(100, 5)
                feature.setGeometry(buffered)

            sink.addFeature(feature)

        # Create output layer definition
        output_def = QgsProcessingOutputLayerDefinition()
        output_def.sink = dest_id
        output_def.destinationName = "Buffered Features"
        output_def.loadAsDefaultStyle = True

        # Register with context (if not already registered by parameterAsSink)
        context.addLayerToLoadOnCompletion(
            dest_id,
            QgsProcessingContext.LayerDetails(
                name=output_def.destinationName,
                crs=source.sourceCrs(),
                outputLayerDefinition=output_def
            )
        )

        return {self.OUTPUT: dest_id}

    def postProcess(self, context, feedback, runResult=True):
        """Post-process outputs."""
        if not runResult:
            return {}

        # Get outputs
        outputs = self._outputs

        # Framework will automatically:
        # 1. Load layers from context
        # 2. Add to project
        # 3. Apply styling
        # 4. Refresh canvas

        return outputs
```

## Temporary Layer Management

### Temporary Layer Store

```python
class QgsProcessingContext:
    """Manages temporary layers."""

    def temporaryLayerStore(self):
        """
        Returns temporary layer store.

        Temporary layers are:
        - Memory layers
        - Temporary files
        - Layers created during processing
        """
        return self._temporaryStore

    def takeResultLayer(self, layerId):
        """
        Take ownership of result layer.

        Removes layer from temporary store and transfers ownership.
        """
        layer = self._temporaryStore.takeMapLayer(layerId)
        return layer
```

### Temporary File Outputs

```python
def createTemporarySink(context, fields, geometryType, crs):
    """Create temporary file sink."""
    # Generate temporary file path
    temp_file = QgsProcessingUtils.generateTempFilename(
        'output', 'shp', context
    )

    # Create sink
    writer = QgsVectorFileWriter(
        temp_file,
        context.defaultEncoding(),
        fields,
        geometryType,
        crs,
        "ESRI Shapefile"
    )

    sink = QgsVectorFileWriterFeatureSink(writer)
    dest_id = temp_file

    # Register as temporary (auto-deleted)
    context.addLayerToLoadOnCompletion(
        dest_id,
        QgsProcessingContext.LayerDetails(
            name="Temporary Output",
            crs=crs
        )
    )

    return sink, dest_id
```

### Memory Layer Outputs

```python
def createMemorySink(context, fields, geometryType, crs, name):
    """Create memory layer sink."""
    # Create memory layer
    uri = QgsMemoryProviderUtils.createUri(fields, geometryType, crs)
    layer = QgsVectorLayer(uri, name, "memory")

    if not layer.isValid():
        return None, None

    # Add to temporary store
    context.temporaryLayerStore().addMapLayer(layer)

    # Create sink
    sink = QgsVectorLayerFeatureSink(layer)
    dest_id = f"memory:{name}"

    # Register with context
    context.addLayerToLoadOnCompletion(
        dest_id,
        QgsProcessingContext.LayerDetails(
            name=name,
            crs=crs
        )
    )

    return sink, dest_id
```

## Complete Integration Flow

### End-to-End Flow

```
1. Algorithm Execution (Background Thread)
   processAlgorithm()
       ↓
   Create feature sink
       ↓
   Write features to sink
       ↓
   Return output dictionary
       ↓
   Register with context (addLayerToLoadOnCompletion)

2. Post-Processing (Main Thread)
   postProcess()
       ↓
   Get layers from context (layersToLoadOnCompletion)
       ↓
   For each layer:
       - Load layer (file/memory/temporary)
       - Set layer name and CRS
       - Add to QgsProject
       - Apply styling

3. Project Integration
   QgsProject.addMapLayer(layer)
       ↓
   QgsProject.layersAdded.emit([layer])
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
       ↓
   Canvas repainted with new layer

4. Styling Application
   applyOutputLayerStyling(layer, layer_details)
       ↓
   Load style file (if specified)
       ↓
   layer.loadNamedStyle(style_path)
       ↓
   layer.styleChanged.emit()
       ↓
   QgsMapCanvas.refresh() (via signal connection)
       ↓
   Canvas repainted with new style
```

## Summary

### Output Integration Components

1. **QgsProcessingContext**
   - Manages output layer registration
   - Stores temporary layers
   - Provides layer loading mechanism

2. **QgsProcessingOutputLayerDefinition**
   - Encapsulates output settings
   - Defines styling options
   - Specifies layer properties

3. **postProcess() Method**
   - Finalizes outputs
   - Loads layers from context
   - Applies styling

4. **Processing Framework**
   - Coordinates post-processing
   - Loads layers in main thread
   - Integrates with project

5. **QgsProject**
   - Adds layers to project
   - Emits layer signals
   - Triggers canvas updates

6. **QgsMapCanvas**
   - Refreshes on layer addition
   - Renders new layers
   - Updates on style changes

### Key Mechanisms

- **Deferred Loading**: Layers loaded in main thread after algorithm completes
- **Automatic Integration**: Outputs automatically added to project and canvas
- **Styling Support**: Default and custom styles applied automatically
- **Temporary Management**: Temporary layers managed by context
- **Signal Propagation**: Changes propagate via Qt signals/slots

This architecture ensures **seamless output integration**, **automatic canvas updates**, and **consistent styling** for Processing algorithm outputs in QGIS.
