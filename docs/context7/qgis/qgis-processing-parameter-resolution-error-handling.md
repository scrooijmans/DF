# QGIS Processing Parameter Resolution & Error Handling

This document explains in detail how QGIS Processing uses `QgsProcessingContext` and `QgsProcessingParameterDefinition` to coerce/resolve inputs (e.g., converting layer references, resolving map layers, creating feature sources/sinks), and how failures are surfaced (exceptions vs `QgsProcessingException` vs return status).

## Overview

QGIS Processing uses a **sophisticated parameter resolution system** that:

1. **Resolves Layer References**: Converts layer IDs, paths, URIs to actual layer objects
2. **Creates Feature Sources**: Converts various input types to `QgsFeatureSource` objects
3. **Creates Feature Sinks**: Creates output destinations for writing features
4. **Handles Failures**: Uses exceptions and return values to report errors

**Key Principles:**

- **Context-Based Resolution**: `QgsProcessingContext` manages layer resolution and temporary storage
- **Parameter Definition Coercion**: `QgsProcessingParameterDefinition` defines how values are converted
- **Exception-Based Errors**: Critical failures raise `QgsProcessingException`
- **Return Status**: Some methods return `None` or `False` for non-critical failures
- **Automatic Cleanup**: Context manages temporary layers and resources

## QgsProcessingContext: Layer Resolution

### Context Responsibilities

`QgsProcessingContext` manages the execution environment:

```python
from qgis.core import QgsProcessingContext

class QgsProcessingContext:
    """Manages execution environment for Processing algorithms."""

    def getMapLayer(self, identifier):
        """
        Resolves a layer identifier to a QgsMapLayer object.

        Args:
            identifier: Layer ID, name, or datasource path

        Returns:
            QgsMapLayer or None if not found
        """
        pass

    def temporaryLayerStore(self):
        """Returns temporary layer storage."""
        pass

    def project(self):
        """Returns current QgsProject."""
        pass
```

### Layer Resolution Process

**1. Layer Identifier Types**

QGIS Processing accepts multiple layer reference formats:

```python
# Layer ID (from QgsProject)
layer_id = "polygons_abc123"

# Layer name (from QgsProject)
layer_name = "My Polygon Layer"

# File path
file_path = "/path/to/layer.shp"

# URI string
uri_string = "Point?crs=EPSG:4326&field=id:integer"

# Memory layer reference
memory_ref = "memory:MyLayer"

# Database connection string
db_uri = "postgresql://host:5432/dbname?table=mytable"
```

**2. Resolution Priority**

`getMapLayer()` resolves identifiers in this order:

```python
def getMapLayer(self, identifier):
    """Resolve layer identifier to QgsMapLayer."""
    if not identifier:
        return None

    # 1. Check active project layers by ID
    project = self.project()
    if project:
        layer = project.mapLayer(identifier)
        if layer:
            return layer

        # 2. Check by layer name
        layers = project.mapLayersByName(identifier)
        if layers:
            return layers[0]  # Return first match

    # 3. Check temporary layer store
    temp_store = self.temporaryLayerStore()
    if temp_store:
        layer = temp_store.mapLayer(identifier)
        if layer:
            return layer

    # 4. Try loading from file path/URI
    # (Handled by parameterAsSource/parameterAsLayer)
    return None
```

**3. Loading External Layers**

If layer not found in project, Processing attempts to load it:

```python
from qgis.core import (
    QgsProcessingUtils,
    QgsVectorLayer,
    QgsRasterLayer
)

def loadLayerFromString(identifier, context):
    """Load layer from string identifier."""
    # Check if it's a file path
    if os.path.exists(identifier):
        # Determine layer type from file extension
        if identifier.endswith(('.shp', '.gpkg', '.geojson')):
            layer = QgsVectorLayer(identifier, "layer", "ogr")
        elif identifier.endswith(('.tif', '.tiff', '.img')):
            layer = QgsRasterLayer(identifier, "layer", "gdal")

        if layer and layer.isValid():
            # Add to temporary store
            context.temporaryLayerStore().addMapLayer(layer)
            return layer

    # Check if it's a URI
    if '?' in identifier or identifier.startswith(('Point', 'LineString', 'Polygon')):
        layer = QgsVectorLayer(identifier, "layer", "memory")
        if layer and layer.isValid():
            context.temporaryLayerStore().addMapLayer(layer)
            return layer

    return None
```

## Parameter Resolution: parameterAs\* Methods

### parameterAsSource: Feature Source Creation

**Purpose**: Converts parameter value to `QgsFeatureSource` object

**Resolution Process:**

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingContext,
    QgsFeatureSource
)

def parameterAsSource(self, parameters, name, context):
    """
    Resolves parameter to QgsFeatureSource.

    Resolution steps:
    1. Get parameter value
    2. Resolve layer reference
    3. Create feature source
    4. Validate geometry type constraints
    5. Return source or raise exception
    """
    # 1. Get parameter value
    value = parameters.get(name)
    if not value:
        param_def = self.parameterDefinition(name)
        if param_def and not param_def.isOptional():
            raise QgsProcessingException(
                self.invalidSourceError(parameters, name)
            )
        return None

    # 2. Resolve layer reference
    layer = None

    if isinstance(value, QgsVectorLayer):
        # Already a layer object
        layer = value
    elif isinstance(value, str):
        # String identifier - resolve via context
        layer = context.getMapLayer(value)

        if not layer:
            # Try loading from file/URI
            layer = QgsProcessingUtils.mapLayerFromString(value, context)

        if not layer:
            raise QgsProcessingException(
                self.invalidSourceError(parameters, name)
            )
    elif isinstance(value, QgsFeatureSource):
        # Already a feature source
        return value

    # 3. Validate layer type
    if not isinstance(layer, QgsVectorLayer):
        raise QgsProcessingException(
            self.invalidSourceError(parameters, name)
        )

    # 4. Check geometry type constraint
    param_def = self.parameterDefinition(name)
    if isinstance(param_def, QgsProcessingParameterLimitedDataTypes):
        allowed_types = param_def.dataTypes()
        if allowed_types:
            layer_type = QgsProcessing.typeToSourceType(layer.wkbType())
            if layer_type not in allowed_types:
                raise QgsProcessingException(
                    f"Input layer geometry type ({layer_type}) is not compatible. "
                    f"Expected: {allowed_types}"
                )

    # 5. Create and return feature source
    return QgsVectorLayerFeatureSource(layer)
```

**Error Handling:**

```python
def parameterAsSource(self, parameters, name, context):
    """Resolve with error handling."""
    try:
        value = parameters.get(name)

        # Resolve layer
        layer = self.resolveLayer(value, context)
        if not layer:
            # Raise exception for missing required parameter
            raise QgsProcessingException(
                self.invalidSourceError(parameters, name)
            )

        # Create source
        source = QgsVectorLayerFeatureSource(layer)
        return source

    except QgsProcessingException:
        # Re-raise Processing exceptions
        raise
    except Exception as e:
        # Wrap other exceptions
        raise QgsProcessingException(
            f"Error resolving source parameter '{name}': {str(e)}"
        )
```

### parameterAsLayer: Layer Object Creation

**Purpose**: Converts parameter value to `QgsMapLayer` object

**Resolution Process:**

```python
def parameterAsLayer(self, parameters, name, context):
    """
    Resolves parameter to QgsMapLayer object.

    Similar to parameterAsSource but returns layer object directly.
    """
    value = parameters.get(name)
    if not value:
        param_def = self.parameterDefinition(name)
        if param_def and not param_def.isOptional():
            raise QgsProcessingException(
                f"Required parameter '{name}' is missing"
            )
        return None

    # Resolve layer
    if isinstance(value, QgsMapLayer):
        return value
    elif isinstance(value, str):
        layer = context.getMapLayer(value)
        if not layer:
            layer = QgsProcessingUtils.mapLayerFromString(value, context)
        if not layer:
            raise QgsProcessingException(
                f"Could not load layer from '{value}'"
            )
        return layer

    raise QgsProcessingException(
        f"Invalid value for parameter '{name}': {type(value)}"
    )
```

### parameterAsSink: Feature Sink Creation

**Purpose**: Creates output destination for writing features

**Resolution Process:**

```python
from qgis.core import (
    QgsFeatureSink,
    QgsFields,
    QgsCoordinateReferenceSystem
)

def parameterAsSink(self, parameters, name, context, fields,
                    geometryType, crs, sinkFlags=None,
                    createOptions=None, datasourceOptions=None,
                    layerOptions=None):
    """
    Creates feature sink for output.

    Resolution steps:
    1. Get output destination value
    2. Determine output format
    3. Create sink (file, memory, or temporary)
    4. Register with context
    5. Return sink and destination ID
    """
    # 1. Get parameter value
    value = parameters.get(name)
    if not value:
        param_def = self.parameterDefinition(name)
        if param_def and not param_def.isOptional():
            raise QgsProcessingException(
                self.invalidSinkError(parameters, name)
            )
        return None, None

    # 2. Determine output format
    destination = str(value)

    # Check for special destinations
    if destination.startswith('memory:'):
        # Memory layer
        layer_name = destination[7:] if len(destination) > 7 else 'output'
        sink, dest_id = self.createMemorySink(
            context, fields, geometryType, crs, layer_name
        )
    elif destination.startswith('TEMPORARY_OUTPUT'):
        # Temporary file
        sink, dest_id = self.createTemporarySink(
            context, fields, geometryType, crs, createOptions
        )
    else:
        # File output
        sink, dest_id = self.createFileSink(
            context, destination, fields, geometryType, crs,
            createOptions, datasourceOptions, layerOptions
        )

    if not sink:
        raise QgsProcessingException(
            self.invalidSinkError(parameters, name)
        )

    return sink, dest_id

def createFileSink(self, context, path, fields, geometryType, crs,
                   createOptions, datasourceOptions, layerOptions):
    """Create file-based feature sink."""
    from qgis.core import QgsVectorFileWriter

    # Determine format from extension
    format = QgsVectorFileWriter.driverForExtension(
        os.path.splitext(path)[1]
    )

    # Create writer
    writer = QgsVectorFileWriter(
        path,
        context.defaultEncoding(),
        fields,
        geometryType,
        crs,
        format,
        datasourceOptions,
        layerOptions
    )

    if writer.hasError():
        error_msg = writer.errorMessage()
        return None, None

    # Create sink wrapper
    sink = QgsVectorFileWriterFeatureSink(writer)
    dest_id = path

    # Register with context for loading
    context.addLayerToLoadOnCompletion(
        dest_id,
        QgsProcessingContext.LayerDetails(
            name=os.path.basename(path),
            crs=crs
        )
    )

    return sink, dest_id

def createMemorySink(self, context, fields, geometryType, crs, name):
    """Create memory layer sink."""
    from qgis.core import QgsVectorLayer, QgsMemoryProviderUtils

    # Create memory layer
    uri = QgsMemoryProviderUtils.createUri(
        fields, geometryType, crs
    )
    layer = QgsVectorLayer(uri, name, "memory")

    if not layer.isValid():
        return None, None

    # Add to temporary store
    context.temporaryLayerStore().addMapLayer(layer)

    # Create sink
    sink = QgsVectorLayerFeatureSink(layer)
    dest_id = f"memory:{name}"

    return sink, dest_id
```

**Error Handling:**

```python
def parameterAsSink(self, parameters, name, context, ...):
    """Create sink with error handling."""
    try:
        value = parameters.get(name)

        # Create sink
        sink, dest_id = self.createSink(value, context, ...)

        if not sink:
            raise QgsProcessingException(
                self.invalidSinkError(parameters, name)
            )

        return sink, dest_id

    except QgsProcessingException:
        raise
    except Exception as e:
        raise QgsProcessingException(
            f"Error creating sink for parameter '{name}': {str(e)}"
        )
```

## Parameter Definition Coercion

### checkValueIsAcceptable: Value Validation

**Purpose**: Validates if a value is acceptable for a parameter

```python
from qgis.core import QgsProcessingParameterDefinition

class QgsProcessingParameterDefinition:
    """Base class for parameter definitions."""

    def checkValueIsAcceptable(self, value, context=None):
        """
        Validates if value is acceptable.

        Returns:
            bool: True if acceptable, False otherwise
        """
        # Base checks
        if not self.isOptional() and (value is None or value == ''):
            return False

        # Type-specific validation
        return self._checkValueIsAcceptable(value, context)
```

### Feature Source Parameter Coercion

```python
from qgis.core import QgsProcessingParameterFeatureSource

class QgsProcessingParameterFeatureSource(QgsProcessingParameterLimitedDataTypes):
    """Feature source parameter with coercion."""

    def _checkValueIsAcceptable(self, value, context=None):
        """Validate and coerce feature source value."""
        if not value:
            return self.isOptional()

        # Coerce string to layer
        if isinstance(value, str):
            if context:
                # Try resolving via context
                layer = context.getMapLayer(value)
                if layer:
                    value = layer
                else:
                    # Try loading from file
                    layer = QgsProcessingUtils.mapLayerFromString(value, context)
                    if layer:
                        value = layer

        # Validate layer type
        if isinstance(value, QgsVectorLayer):
            # Check geometry type constraint
            data_types = self.dataTypes()
            if data_types:
                layer_type = QgsProcessing.typeToSourceType(value.wkbType())
                return layer_type in data_types
            return True

        # Accept QgsFeatureSource directly
        if isinstance(value, QgsFeatureSource):
            return True

        return False
```

## Error Handling Mechanisms

### QgsProcessingException: Critical Errors

**Purpose**: Raised for fatal errors that prevent algorithm execution

**Usage:**

```python
from qgis.core import QgsProcessingException

# Raise exception for invalid parameter
if not source:
    raise QgsProcessingException(
        self.invalidSourceError(parameters, 'INPUT')
    )

# Raise exception for invalid value
if distance <= 0:
    raise QgsProcessingException(
        "Distance must be greater than 0"
    )

# Raise exception with context
try:
    sink.addFeature(feature)
except Exception as e:
    raise QgsProcessingException(
        f"Error writing feature: {str(e)}"
    )
```

**Exception Hierarchy:**

```python
class QgsProcessingException(Exception):
    """Base exception for Processing errors."""
    pass

# Specific exception types
class QgsProcessingParameterException(QgsProcessingException):
    """Exception for parameter-related errors."""
    pass
```

### Return Status: Non-Critical Failures

**Purpose**: Return `None` or `False` for recoverable errors

**Usage:**

```python
# Return None for optional parameter
def parameterAsSource(self, parameters, name, context):
    value = parameters.get(name)
    if not value:
        param_def = self.parameterDefinition(name)
        if param_def and param_def.isOptional():
            return None  # Optional parameter missing - not an error
        raise QgsProcessingException(...)  # Required parameter missing - error

# Return False for validation
def checkParameterValues(self, parameters, context):
    issues = []
    # ... validation checks ...
    return issues  # Empty list = success, non-empty = failure

# Return status for preparation
def prepareAlgorithm(self, parameters, context, feedback):
    try:
        # ... preparation ...
        return True  # Success
    except Exception as e:
        feedback.reportError(str(e))
        return False  # Failure (non-exception)
```

### Error Message Helpers

**Purpose**: Generate user-friendly error messages

```python
from qgis.core import QgsProcessingAlgorithm

class MyAlgorithm(QgsProcessingAlgorithm):
    """Algorithm with error message helpers."""

    def invalidSourceError(self, parameters, name):
        """
        Generate error message for invalid source.

        Returns:
            str: User-friendly error message
        """
        param_def = self.parameterDefinition(name)
        value = parameters.get(name, '')

        if isinstance(value, str):
            return f"Could not load source layer from '{value}'"
        else:
            return f"Invalid source for parameter '{name}'"

    def invalidSinkError(self, parameters, name):
        """Generate error message for invalid sink."""
        param_def = self.parameterDefinition(name)
        value = parameters.get(name, '')

        return f"Could not create output layer at '{value}'"

    def invalidRasterError(self, parameters, name):
        """Generate error message for invalid raster."""
        value = parameters.get(name, '')
        return f"Could not load raster layer from '{value}'"
```

## Complete Resolution Example

### Full Parameter Resolution Flow

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingParameterFeatureSource,
    QgsProcessingParameterFeatureSink,
    QgsProcessingContext,
    QgsProcessingException,
    QgsProcessingFeedback
)

class BufferAlgorithm(QgsProcessingAlgorithm):
    """Algorithm demonstrating parameter resolution."""

    INPUT = 'INPUT'
    OUTPUT = 'OUTPUT'

    def initAlgorithm(self, config=None):
        self.addParameter(
            QgsProcessingParameterFeatureSource(
                self.INPUT,
                'Input layer'
            )
        )
        self.addParameter(
            QgsProcessingParameterFeatureSink(
                self.OUTPUT,
                'Output layer'
            )
        )

    def processAlgorithm(self, parameters, context, feedback):
        """
        Process algorithm with parameter resolution.

        This demonstrates:
        1. Layer reference resolution
        2. Feature source creation
        3. Feature sink creation
        4. Error handling
        """
        # 1. Resolve input source
        # parameterAsSource() handles:
        # - String ID → QgsMapLayer → QgsFeatureSource
        # - File path → Load layer → QgsFeatureSource
        # - URI → Create layer → QgsFeatureSource
        # - QgsVectorLayer → QgsFeatureSource
        # Raises QgsProcessingException on failure
        try:
            source = self.parameterAsSource(parameters, self.INPUT, context)
        except QgsProcessingException as e:
            # Exception raised for:
            # - Missing required parameter
            # - Invalid layer reference
            # - Geometry type mismatch
            feedback.reportError(str(e))
            raise

        if not source:
            # None returned for optional parameter
            raise QgsProcessingException("Input source is required")

        # 2. Create output sink
        # parameterAsSink() handles:
        # - File path → Create QgsVectorFileWriter → QgsFeatureSink
        # - Memory reference → Create memory layer → QgsFeatureSink
        # - Temporary output → Create temp file → QgsFeatureSink
        # Raises QgsProcessingException on failure
        try:
            (sink, dest_id) = self.parameterAsSink(
                parameters,
                self.OUTPUT,
                context,
                source.fields(),
                source.wkbType(),
                source.sourceCrs()
            )
        except QgsProcessingException as e:
            feedback.reportError(str(e))
            raise

        if not sink:
            raise QgsProcessingException(
                self.invalidSinkError(parameters, self.OUTPUT)
            )

        # 3. Process features
        total = 100.0 / source.featureCount() if source.featureCount() else 0
        for current, feature in enumerate(source.getFeatures()):
            if feedback.isCanceled():
                break

            # Process feature
            # ...

            # Write to sink
            if not sink.addFeature(feature):
                # Sink write failure
                raise QgsProcessingException(
                    f"Error writing feature to output: {sink.errorMessage()}"
                )

            feedback.setProgress(int(current * total))

        return {self.OUTPUT: dest_id}
```

## Resolution Summary

### Resolution Flow

```
Parameter Value (various types)
    ↓
1. Type Check
   - QgsMapLayer → Use directly
   - QgsFeatureSource → Use directly
   - String → Resolve
   - None → Check optional
    ↓
2. String Resolution (if needed)
   - Layer ID → context.getMapLayer()
   - File path → QgsProcessingUtils.mapLayerFromString()
   - URI → Create layer
   - Memory ref → Create memory layer
    ↓
3. Validation
   - Geometry type constraints
   - Layer type constraints
   - Field type constraints
    ↓
4. Object Creation
   - Feature Source → QgsVectorLayerFeatureSource
   - Feature Sink → QgsVectorFileWriterFeatureSink
   - Layer → QgsVectorLayer/QgsRasterLayer
    ↓
5. Context Registration
   - Temporary layers → context.temporaryLayerStore()
   - Output layers → context.addLayerToLoadOnCompletion()
    ↓
6. Return or Exception
   - Success → Return object
   - Failure → Raise QgsProcessingException
```

### Error Handling Strategy

| Error Type                     | Mechanism                | When Used                               |
| ------------------------------ | ------------------------ | --------------------------------------- |
| **Missing Required Parameter** | `QgsProcessingException` | Required parameter not provided         |
| **Invalid Layer Reference**    | `QgsProcessingException` | Cannot resolve layer ID/path            |
| **Geometry Type Mismatch**     | `QgsProcessingException` | Layer geometry doesn't match constraint |
| **Sink Creation Failure**      | `QgsProcessingException` | Cannot create output file/layer         |
| **Optional Parameter Missing** | Return `None`            | Optional parameter not provided         |
| **Validation Issues**          | Return `List[str]`       | `checkParameterValues()` returns issues |
| **Preparation Failure**        | Return `False`           | `prepareAlgorithm()` returns False      |
| **Runtime Errors**             | `QgsProcessingException` | Errors during feature processing        |

### Best Practices

1. **Use parameterAs\* Methods**: Always use these for parameter resolution
2. **Handle Exceptions**: Catch `QgsProcessingException` for user-friendly errors
3. **Check Return Values**: Check for `None` for optional parameters
4. **Use Error Helpers**: Use `invalidSourceError()`, `invalidSinkError()` for messages
5. **Register Outputs**: Let context manage temporary layers
6. **Validate Early**: Check parameters in `checkParameterValues()`
7. **Provide Context**: Include parameter name and value in error messages

This architecture ensures **robust parameter resolution**, **clear error messages**, and **reliable algorithm execution** in QGIS Processing.
