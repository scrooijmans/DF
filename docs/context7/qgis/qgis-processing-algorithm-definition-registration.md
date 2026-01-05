# QGIS Processing Algorithm Definition & Registration Architecture

This document explains in detail how QGIS Processing algorithms are defined and registered, including how inputs/outputs are declared (parameter definitions, sinks, feature sources, raster inputs), and where "type requirements" are expressed in code and metadata.

## Overview

QGIS Processing algorithms are defined through a **structured class hierarchy** and registered via **provider-based architecture**:

1. **Algorithm Definition**: Subclass `QgsProcessingAlgorithm` with required methods
2. **Parameter Declaration**: Define inputs/outputs using `QgsProcessingParameterDefinition` subclasses
3. **Provider Registration**: Register algorithms via `QgsProcessingProvider` to Processing Registry
4. **Type Requirements**: Expressed through parameter class types, metadata, and validation methods

**Key Principles:**

- **Class-Based Definition**: Algorithms are Python/C++ classes inheriting from `QgsProcessingAlgorithm`
- **Parameter Type System**: Strict typing via `QgsProcessingParameterDefinition` subclasses
- **Provider Container**: Algorithms grouped into providers (e.g., "native", "gdal", "grass")
- **Metadata-Driven UI**: Parameter definitions generate GUI widgets automatically
- **Type Validation**: Type requirements enforced at parameter definition and execution time

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│              QGIS Processing Framework                       │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Processing Registry                                   │   │
│  │  - QgsProcessingRegistry                              │   │
│  │  - Manages providers and algorithms                   │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Processing Providers                                 │   │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ │   │
│  │  │   Native    │ │    GDAL     │ │    GRASS     │ │   │
│  │  │  Provider   │ │  Provider   │ │  Provider   │ │   │
│  │  └──────┬──────┘ └──────┬───────┘ └──────┬───────┘ │   │
│  └─────────┼───────────────┼────────────────┼─────────┘   │
│            │               │                 │              │
│            ▼               ▼                 ▼              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Processing Algorithms                               │   │
│  │  - QgsProcessingAlgorithm (base class)              │   │
│  │  - initAlgorithm() → Parameter definitions          │   │
│  │  - processAlgorithm() → Processing logic            │   │
│  └──────────────────────┬──────────────────────────────┘   │
│                         │                                    │
│                         ▼                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Parameter Definitions                               │   │
│  │  - QgsProcessingParameterDefinition (base)          │   │
│  │  - QgsProcessingParameterFeatureSource (input)     │   │
│  │  - QgsProcessingParameterFeatureSink (output)       │   │
│  │  - QgsProcessingParameterRasterLayer (input)        │   │
│  │  - QgsProcessingParameterRasterDestination (output) │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## Algorithm Definition: QgsProcessingAlgorithm

### Base Class Structure

All Processing algorithms inherit from `QgsProcessingAlgorithm`:

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingParameterDefinition,
    QgsProcessingContext,
    QgsProcessingFeedback
)

class MyCustomAlgorithm(QgsProcessingAlgorithm):
    """Custom Processing algorithm example."""

    # Required: Unique algorithm identifier
    def name(self):
        return "my_custom_algorithm"

    # Required: Human-readable display name
    def displayName(self):
        return "My Custom Algorithm"

    # Required: Group name in Processing Toolbox
    def group(self):
        return "My Plugin"

    # Required: Group ID
    def groupId(self):
        return "myplugin"

    # Required: Create new instance
    def createInstance(self):
        return MyCustomAlgorithm()

    # Required: Define parameters and outputs
    def initAlgorithm(self, config=None):
        # Parameter definitions go here
        pass

    # Required: Execute algorithm logic
    def processAlgorithm(self, parameters, context, feedback):
        # Processing logic goes here
        return {}
```

### Required Methods

**1. `name()` - Algorithm Identifier**

Returns a unique string identifier for the algorithm:

```python
def name(self):
    return "myplugin:buffer_features"
```

**Naming Convention:**

- Format: `{provider_id}:{algorithm_name}`
- Example: `native:buffer`, `gdal:rasterize`, `grass7:v.buffer`
- Must be unique across all providers

**2. `displayName()` - Human-Readable Name**

Returns a user-friendly name shown in the Processing Toolbox:

```python
def displayName(self):
    return self.tr("Buffer Features")
```

**3. `group()` and `groupId()` - Algorithm Grouping**

Groups algorithms in the Processing Toolbox:

```python
def group(self):
    return self.tr("Vector Geometry")

def groupId(self):
    return "vectorgeometry"
```

**4. `createInstance()` - Factory Method**

Creates a new instance of the algorithm:

```python
def createInstance(self):
    return MyCustomAlgorithm()
```

**5. `initAlgorithm()` - Parameter Definition**

Defines all input parameters and outputs:

```python
def initAlgorithm(self, config=None):
    # Input parameters
    self.addParameter(...)

    # Output definitions
    self.addOutput(...)
```

**6. `processAlgorithm()` - Execution Logic**

Contains the core algorithm logic:

```python
def processAlgorithm(self, parameters, context, feedback):
    # Retrieve parameter values
    input_layer = self.parameterAsSource(parameters, 'INPUT', context)
    buffer_distance = self.parameterAsDouble(parameters, 'DISTANCE', context)
    output_sink = self.parameterAsSink(parameters, 'OUTPUT', context, ...)

    # Process features
    for feature in input_layer.getFeatures():
        # Algorithm logic
        output_sink.addFeature(feature)

    return {'OUTPUT': output_sink.destination()}
```

## Parameter Definitions: Inputs and Outputs

### Parameter Definition Base Class

All parameters inherit from `QgsProcessingParameterDefinition`:

```python
from qgis.core import QgsProcessingParameterDefinition

class QgsProcessingParameterDefinition:
    """Base class for all Processing parameter definitions."""

    def __init__(self, name, description, defaultValue=None, optional=False):
        self.name = name
        self.description = description
        self.defaultValue = defaultValue
        self.optional = optional
```

**Common Properties:**

- **name**: Parameter identifier (string)
- **description**: Human-readable description
- **defaultValue**: Default value (optional)
- **optional**: Whether parameter is required (bool)
- **provider**: Reference to owning provider

### Input Parameter Types

#### 1. Feature Source (`QgsProcessingParameterFeatureSource`)

**Purpose**: Accepts vector layer inputs (feature sources)

**Definition:**

```python
from qgis.core import (
    QgsProcessingParameterFeatureSource,
    QgsProcessing
)

# Basic feature source
self.addParameter(
    QgsProcessingParameterFeatureSource(
        'INPUT',
        'Input vector layer'
    )
)

# With geometry type constraint
self.addParameter(
    QgsProcessingParameterFeatureSource(
        'INPUT',
        'Input polygon layer',
        types=[QgsProcessing.TypeVectorPolygon]
    )
)

# With multiple geometry types
self.addParameter(
    QgsProcessingParameterFeatureSource(
        'INPUT',
        'Input line or polygon layer',
        types=[QgsProcessing.TypeVectorLine, QgsProcessing.TypeVectorPolygon]
    )
)
```

**Type Requirements:**

- **Geometry Type Constraint**: `types` parameter restricts accepted geometry types
  - `QgsProcessing.TypeVectorPoint`
  - `QgsProcessing.TypeVectorLine`
  - `QgsProcessing.TypeVectorPolygon`
  - `QgsProcessing.TypeVectorAnyGeometry`
  - `QgsProcessing.TypeVector`

**Retrieval in `processAlgorithm()`:**

```python
def processAlgorithm(self, parameters, context, feedback):
    # Get feature source
    source = self.parameterAsSource(parameters, 'INPUT', context)

    # Check geometry type
    if source.wkbType() == QgsWkbTypes.Polygon:
        # Process polygons
        pass

    # Iterate features
    for feature in source.getFeatures():
        # Process feature
        pass
```

**Type Name:**

```python
QgsProcessingParameterFeatureSource.typeName()
# Returns: "source"
```

#### 2. Raster Layer (`QgsProcessingParameterRasterLayer`)

**Purpose**: Accepts raster layer inputs

**Definition:**

```python
from qgis.core import QgsProcessingParameterRasterLayer

self.addParameter(
    QgsProcessingParameterRasterLayer(
        'INPUT',
        'Input raster layer'
    )
)
```

**Retrieval in `processAlgorithm()`:**

```python
def processAlgorithm(self, parameters, context, feedback):
    # Get raster layer
    raster = self.parameterAsRasterLayer(parameters, 'INPUT', context)

    # Access raster data
    provider = raster.dataProvider()
    extent = raster.extent()
    crs = raster.crs()
    band_count = raster.bandCount()
```

**Type Name:**

```python
QgsProcessingParameterRasterLayer.typeName()
# Returns: "raster"
```

#### 3. Number (`QgsProcessingParameterNumber`)

**Purpose**: Accepts numeric inputs (integer or double)

**Definition:**

```python
from qgis.core import QgsProcessingParameterNumber

# Double (floating-point)
self.addParameter(
    QgsProcessingParameterNumber(
        'DISTANCE',
        'Buffer distance',
        type=QgsProcessingParameterNumber.Double,
        defaultValue=100.0,
        minValue=0.0,
        maxValue=10000.0
    )
)

# Integer
self.addParameter(
    QgsProcessingParameterNumber(
        'ITERATIONS',
        'Number of iterations',
        type=QgsProcessingParameterNumber.Integer,
        defaultValue=1,
        minValue=1,
        maxValue=100
    )
)
```

**Type Requirements:**

- **Type**: `Double` or `Integer`
- **Min/Max Values**: Optional constraints
- **Default Value**: Optional default

**Retrieval:**

```python
distance = self.parameterAsDouble(parameters, 'DISTANCE', context)
iterations = self.parameterAsInt(parameters, 'ITERATIONS', context)
```

#### 4. String (`QgsProcessingParameterString`)

**Purpose**: Accepts text inputs

**Definition:**

```python
from qgis.core import QgsProcessingParameterString

self.addParameter(
    QgsProcessingParameterString(
        'FIELD_NAME',
        'Field name',
        defaultValue='id',
        optional=False
    )
)
```

**With Value Hints (QGIS 3.22+):**

```python
param = QgsProcessingParameterString('PRINTER_NAME', 'Printer name')
param.setMetadata({
    'widget_wrapper': {
        'value_hints': ['Printer1', 'Printer2', 'Printer3']
    }
})
self.addParameter(param)
```

#### 5. Boolean (`QgsProcessingParameterBoolean`)

**Purpose**: Accepts true/false inputs

**Definition:**

```python
from qgis.core import QgsProcessingParameterBoolean

self.addParameter(
    QgsProcessingParameterBoolean(
        'DISSOLVE',
        'Dissolve result',
        defaultValue=False
    )
)
```

**Retrieval:**

```python
dissolve = self.parameterAsBool(parameters, 'DISSOLVE', context)
```

#### 6. Enumeration (`QgsProcessingParameterEnum`)

**Purpose**: Accepts selection from predefined options

**Definition:**

```python
from qgis.core import QgsProcessingParameterEnum

self.addParameter(
    QgsProcessingParameterEnum(
        'END_CAP_STYLE',
        'End cap style',
        options=['Round', 'Flat', 'Square'],
        defaultValue=0,
        allowMultiple=False
    )
)
```

**Retrieval:**

```python
cap_style = self.parameterAsEnum(parameters, 'END_CAP_STYLE', context)
# Returns: 0, 1, or 2
```

### Output Parameter Types

#### 1. Feature Sink (`QgsProcessingParameterFeatureSink`)

**Purpose**: Defines output destination for vector features

**Definition:**

```python
from qgis.core import (
    QgsProcessingParameterFeatureSink,
    QgsProcessing
)

# Basic feature sink
self.addParameter(
    QgsProcessingParameterFeatureSink(
        'OUTPUT',
        'Output layer'
    )
)

# With geometry type specification
self.addParameter(
    QgsProcessingParameterFeatureSink(
        'OUTPUT',
        'Output polygon layer',
        type=QgsProcessing.TypeVectorPolygon
    )
)

# With append support (QGIS 3.14+)
self.addParameter(
    QgsProcessingParameterFeatureSink(
        'OUTPUT',
        'Output layer',
        type=QgsProcessing.TypeVectorPolygon,
        supportsAppend=True
    )
)
```

**Type Requirements:**

- **Geometry Type**: `type` parameter specifies output geometry type
  - `QgsProcessing.TypeVectorPoint`
  - `QgsProcessing.TypeVectorLine`
  - `QgsProcessing.TypeVectorPolygon`
  - `QgsProcessing.TypeVectorAnyGeometry`

**Retrieval in `processAlgorithm()`:**

```python
def processAlgorithm(self, parameters, context, feedback):
    # Get feature sink
    (sink, dest_id) = self.parameterAsSink(
        parameters,
        'OUTPUT',
        context,
        source.fields(),  # Output fields
        source.wkbType(),  # Output geometry type
        source.sourceCrs()  # Output CRS
    )

    # Add features to sink
    for feature in source.getFeatures():
        # Modify feature if needed
        sink.addFeature(feature)

    return {'OUTPUT': dest_id}
```

**Sink Properties:**

- **Fields**: Output attribute fields
- **Geometry Type**: Output geometry type (WKB type)
- **CRS**: Coordinate reference system
- **Append Support**: Whether sink supports appending to existing layer

**Type Name:**

```python
QgsProcessingParameterFeatureSink.typeName()
# Returns: "sink"
```

#### 2. Raster Destination (`QgsProcessingParameterRasterDestination`)

**Purpose**: Defines output destination for raster layers

**Definition:**

```python
from qgis.core import QgsProcessingParameterRasterDestination

self.addParameter(
    QgsProcessingParameterRasterDestination(
        'OUTPUT',
        'Output raster'
    )
)
```

**Retrieval:**

```python
def processAlgorithm(self, parameters, context, feedback):
    # Get output path
    output_path = self.parameterAsOutputLayer(parameters, 'OUTPUT', context)

    # Create raster writer
    writer = QgsRasterFileWriter(output_path)
    # ... write raster data ...

    return {'OUTPUT': output_path}
```

**Type Name:**

```python
QgsProcessingParameterRasterDestination.typeName()
# Returns: "rasterDestination"
```

#### 3. Output Definition (`QgsProcessingOutputDefinition`)

**Purpose**: Defines non-layer outputs (values, strings, numbers)

**Definition:**

```python
from qgis.core import (
    QgsProcessingOutputNumber,
    QgsProcessingOutputString,
    QgsProcessingOutputVariant
)

# Number output
self.addOutput(
    QgsProcessingOutputNumber(
        'FEATURE_COUNT',
        'Number of features'
    )
)

# String output
self.addOutput(
    QgsProcessingOutputString(
        'RESULT_MESSAGE',
        'Result message'
    )
)
```

**Returning Values:**

```python
def processAlgorithm(self, parameters, context, feedback):
    count = 0
    for feature in source.getFeatures():
        count += 1

    return {
        'OUTPUT': output_layer,
        'FEATURE_COUNT': count,
        'RESULT_MESSAGE': f'Processed {count} features'
    }
```

### Additional Parameter Types

**File/Folder:**

```python
from qgis.core import QgsProcessingParameterFile

# File input
self.addParameter(
    QgsProcessingParameterFile(
        'INPUT_FILE',
        'Input file',
        behavior=QgsProcessingParameterFile.File
    )
)

# Folder input
self.addParameter(
    QgsProcessingParameterFile(
        'INPUT_FOLDER',
        'Input folder',
        behavior=QgsProcessingParameterFile.Folder
    )
)
```

**Multiple Layers:**

```python
from qgis.core import QgsProcessingParameterMultipleLayers

self.addParameter(
    QgsProcessingParameterMultipleLayers(
        'LAYERS',
        'Input layers',
        layerType=QgsProcessing.TypeVectorPolygon
    )
)
```

**Field:**

```python
from qgis.core import QgsProcessingParameterField

self.addParameter(
    QgsProcessingParameterField(
        'FIELD',
        'Field name',
        parentLayerParameterName='INPUT',
        type=QgsProcessingParameterField.Numeric
    )
)
```

**Coordinate Reference System:**

```python
from qgis.core import QgsProcessingParameterCrs

self.addParameter(
    QgsProcessingParameterCrs(
        'CRS',
        'Output CRS',
        defaultValue='EPSG:4326'
    )
)
```

## Type Requirements Expression

### 1. In Code: Parameter Class Types

**Type Requirements Expressed Through:**

**A. Parameter Class Selection**

The choice of parameter class inherently defines the type:

```python
# Vector input → QgsProcessingParameterFeatureSource
self.addParameter(
    QgsProcessingParameterFeatureSource('INPUT', 'Input layer')
)

# Raster input → QgsProcessingParameterRasterLayer
self.addParameter(
    QgsProcessingParameterRasterLayer('INPUT', 'Input raster')
)

# Number input → QgsProcessingParameterNumber
self.addParameter(
    QgsProcessingParameterNumber('DISTANCE', 'Distance')
)
```

**B. Geometry Type Constraints**

For feature sources, geometry types are explicitly constrained:

```python
# Only polygons accepted
self.addParameter(
    QgsProcessingParameterFeatureSource(
        'INPUT',
        'Input polygons',
        types=[QgsProcessing.TypeVectorPolygon]  # Type constraint
    )
)
```

**C. Numeric Type Constraints**

For numbers, type (Integer/Double) and range constraints:

```python
self.addParameter(
    QgsProcessingParameterNumber(
        'DISTANCE',
        'Distance',
        type=QgsProcessingParameterNumber.Double,  # Type: Double
        minValue=0.0,  # Minimum constraint
        maxValue=10000.0  # Maximum constraint
    )
)
```

**D. Field Type Constraints**

For field parameters, data type constraints:

```python
self.addParameter(
    QgsProcessingParameterField(
        'FIELD',
        'Numeric field',
        parentLayerParameterName='INPUT',
        type=QgsProcessingParameterField.Numeric  # Type constraint
    )
)
```

### 2. In Metadata: Parameter Type Information

**Type Name Method**

Each parameter class provides a `typeName()` static method:

```python
# Feature source type name
QgsProcessingParameterFeatureSource.typeName()
# Returns: "source"

# Feature sink type name
QgsProcessingParameterFeatureSink.typeName()
# Returns: "sink"

# Raster layer type name
QgsProcessingParameterRasterLayer.typeName()
# Returns: "raster"
```

**Parameter Type Registry**

QGIS maintains a registry of parameter types:

```python
from qgis.core import QgsApplication

registry = QgsApplication.processingRegistry()

# Get parameter type
param_type = registry.parameterType('source')

# Get all parameter types
all_types = registry.parameterTypes()
```

**Parameter Type Metadata**

Parameter types expose metadata about accepted types:

```python
from qgis.core import QgsProcessingParameterType

# Get accepted parameter types
accepted_params = param_type.acceptedParameterTypes()

# Get accepted output types
accepted_outputs = param_type.acceptedOutputTypes()

# Get accepted data types
accepted_data = param_type.acceptedDataTypes(parameter_definition)
```

### 3. Type Validation

**Automatic Validation**

QGIS automatically validates parameter types:

1. **GUI Validation**: Processing dialog validates inputs before execution
2. **Parameter Evaluation**: `parameterAs*()` methods validate and convert types
3. **Error Handling**: Invalid types raise `QgsProcessingException`

**Example Validation:**

```python
def processAlgorithm(self, parameters, context, feedback):
    # This will raise QgsProcessingException if INPUT is not a valid source
    source = self.parameterAsSource(parameters, 'INPUT', context)

    # This will raise if DISTANCE is not a valid number
    distance = self.parameterAsDouble(parameters, 'DISTANCE', context)

    # This validates geometry type matches constraint
    if source.wkbType() != QgsWkbTypes.Polygon:
        raise QgsProcessingException("Input must be a polygon layer")
```

**Custom Validation**

Algorithms can implement custom validation:

```python
def checkParameterValues(self, parameters, context):
    """Custom parameter validation."""
    issues = []

    source = self.parameterAsSource(parameters, 'INPUT', context)
    if source.featureCount() == 0:
        issues.append("Input layer is empty")

    distance = self.parameterAsDouble(parameters, 'DISTANCE', context)
    if distance <= 0:
        issues.append("Distance must be greater than 0")

    return issues
```

## Provider Registration

### Provider Class Definition

Processing providers group related algorithms:

```python
from qgis.core import QgsProcessingProvider
from qgis.PyQt.QtGui import QIcon

class MyProcessingProvider(QgsProcessingProvider):
    """Custom Processing provider."""

    def id(self):
        """Unique provider identifier."""
        return 'myplugin'

    def name(self):
        """Human-readable provider name."""
        return self.tr('My Plugin')

    def icon(self):
        """Provider icon."""
        return QgsProcessingProvider.icon(self)

    def loadAlgorithms(self):
        """Load all algorithms for this provider."""
        self.addAlgorithm(MyCustomAlgorithm())
        self.addAlgorithm(AnotherAlgorithm())
        # Add more algorithms...
```

### Required Provider Methods

**1. `id()` - Provider Identifier**

Returns unique string identifier:

```python
def id(self):
    return 'myplugin'
```

**2. `name()` - Provider Display Name**

Returns human-readable name:

```python
def name(self):
    return self.tr('My Plugin')
```

**3. `loadAlgorithms()` - Algorithm Loading**

Loads all algorithms for the provider:

```python
def loadAlgorithms(self):
    """Must call self.addAlgorithm() for each algorithm."""
    self.addAlgorithm(MyCustomAlgorithm())
    self.addAlgorithm(AnotherAlgorithm())
```

### Registering Provider

**In Plugin Initialization:**

```python
from qgis.core import QgsApplication
from .processing_provider.provider import MyProcessingProvider

class MyPlugin:
    def __init__(self):
        self.provider = None

    def initGui(self):
        """Initialize plugin GUI."""
        # Initialize Processing provider
        self.initProcessing()

    def initProcessing(self):
        """Initialize and register Processing provider."""
        self.provider = MyProcessingProvider()
        QgsApplication.processingRegistry().addProvider(self.provider)

    def unload(self):
        """Unload plugin."""
        if self.provider:
            QgsApplication.processingRegistry().removeProvider(self.provider)
            self.provider = None
```

**Plugin Metadata (`metadata.txt`):**

```ini
[general]
name=My Plugin
version=1.0
hasProcessingProvider=yes
```

### Processing Registry

**Registry Methods:**

```python
from qgis.core import QgsApplication

registry = QgsApplication.processingRegistry()

# Add provider
registry.addProvider(my_provider)

# Remove provider
registry.removeProvider(my_provider)

# Get provider by ID
provider = registry.providerById('myplugin')

# Get all providers
providers = registry.providers()

# Get algorithm by ID
algorithm = registry.algorithmById('myplugin:my_algorithm')

# Get all algorithms
algorithms = registry.algorithms()
```

## Complete Example: Custom Algorithm

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingParameterFeatureSource,
    QgsProcessingParameterNumber,
    QgsProcessingParameterFeatureSink,
    QgsProcessing,
    QgsProcessingException,
    QgsWkbTypes
)
from qgis import processing

class BufferAlgorithm(QgsProcessingAlgorithm):
    """Example buffer algorithm."""

    INPUT = 'INPUT'
    DISTANCE = 'DISTANCE'
    OUTPUT = 'OUTPUT'

    def name(self):
        return 'myplugin:buffer'

    def displayName(self):
        return self.tr('Buffer Features')

    def group(self):
        return self.tr('Vector Geometry')

    def groupId(self):
        return 'vectorgeometry'

    def createInstance(self):
        return BufferAlgorithm()

    def initAlgorithm(self, config=None):
        # Input vector layer (any geometry type)
        self.addParameter(
            QgsProcessingParameterFeatureSource(
                self.INPUT,
                self.tr('Input layer'),
                types=[QgsProcessing.TypeVectorAnyGeometry]
            )
        )

        # Buffer distance (double, positive)
        self.addParameter(
            QgsProcessingParameterNumber(
                self.DISTANCE,
                self.tr('Buffer distance'),
                type=QgsProcessingParameterNumber.Double,
                defaultValue=100.0,
                minValue=0.0
            )
        )

        # Output layer (same geometry type as input)
        self.addParameter(
            QgsProcessingParameterFeatureSink(
                self.OUTPUT,
                self.tr('Buffered layer')
            )
        )

    def processAlgorithm(self, parameters, context, feedback):
        # Retrieve inputs (type validation happens here)
        source = self.parameterAsSource(parameters, self.INPUT, context)
        if source is None:
            raise QgsProcessingException(
                self.invalidSourceError(parameters, self.INPUT)
            )

        distance = self.parameterAsDouble(parameters, self.DISTANCE, context)

        # Create output sink
        (sink, dest_id) = self.parameterAsSink(
            parameters,
            self.OUTPUT,
            context,
            source.fields(),
            source.wkbType(),
            source.sourceCrs()
        )

        if sink is None:
            raise QgsProcessingException(
                self.invalidSinkError(parameters, self.OUTPUT)
            )

        # Process features
        total = 100.0 / source.featureCount() if source.featureCount() else 0
        for current, feature in enumerate(source.getFeatures()):
            if feedback.isCanceled():
                break

            # Buffer geometry
            if feature.geometry():
                buffered = feature.geometry().buffer(distance, 5)
                feature.setGeometry(buffered)

            sink.addFeature(feature)
            feedback.setProgress(int(current * total))

        return {self.OUTPUT: dest_id}
```

## Type Requirements Summary

### Where Type Requirements Are Expressed

1. **Parameter Class Selection**
   - `QgsProcessingParameterFeatureSource` → Vector input
   - `QgsProcessingParameterRasterLayer` → Raster input
   - `QgsProcessingParameterNumber` → Numeric input

2. **Parameter Constructor Arguments**
   - `types=[...]` → Geometry type constraints
   - `type=Double/Integer` → Numeric type
   - `minValue/maxValue` → Range constraints

3. **Parameter Type Metadata**
   - `typeName()` → String type identifier
   - `acceptedParameterTypes()` → Compatible parameter types
   - `acceptedOutputTypes()` → Compatible output types

4. **Parameter Evaluation Methods**
   - `parameterAsSource()` → Validates feature source
   - `parameterAsRasterLayer()` → Validates raster layer
   - `parameterAsDouble()` → Validates and converts to double

5. **Algorithm Validation**
   - `checkParameterValues()` → Custom validation
   - Runtime type checking in `processAlgorithm()`

### Type Requirement Flow

```
User Input (GUI/API)
    ↓
Parameter Definition (initAlgorithm)
    ↓
Type Validation (parameterAs* methods)
    ↓
Type Conversion (if needed)
    ↓
Algorithm Execution (processAlgorithm)
    ↓
Output Type Validation (sink/destination)
    ↓
Result Return
```

## Summary

QGIS Processing algorithms are defined and registered through:

1. **Algorithm Definition**: Subclass `QgsProcessingAlgorithm` with required methods
2. **Parameter Declaration**: Use `QgsProcessingParameterDefinition` subclasses to define inputs/outputs
3. **Type Requirements**: Expressed through:
   - Parameter class selection (FeatureSource, RasterLayer, Number, etc.)
   - Constructor arguments (types, minValue, maxValue, etc.)
   - Parameter type metadata (typeName, acceptedParameterTypes, etc.)
   - Parameter evaluation methods (parameterAsSource, parameterAsDouble, etc.)
4. **Provider Registration**: Register algorithms via `QgsProcessingProvider` to Processing Registry
5. **Automatic UI Generation**: Parameter definitions automatically generate GUI widgets
6. **Type Validation**: Automatic validation at parameter evaluation and execution time

This architecture ensures **type safety**, **user-friendly interfaces**, and **extensibility** for Processing algorithms in QGIS.
