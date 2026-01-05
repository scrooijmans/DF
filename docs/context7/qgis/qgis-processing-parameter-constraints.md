# QGIS Processing Parameter Classes & Constraint Encoding

This document explains in detail how QGIS models algorithm inputs/outputs via `QgsProcessingParameter` classes (FeatureSource, VectorLayer, RasterLayer, Field, Enum, Extent, etc.), including how algorithm authors encode constraints like geometry type, layer type, and field type requirements.

## Overview

QGIS Processing uses a **hierarchical parameter class system** to model algorithm inputs and outputs, with **constraint mechanisms** encoded directly in parameter definitions:

1. **Parameter Class Hierarchy**: Different parameter classes for different data types
2. **Constraint Encoding**: Constraints specified via constructor arguments and property setters
3. **Type Validation**: Automatic validation based on constraint definitions
4. **Metadata Integration**: Constraints exposed in algorithm metadata and GUI

**Key Principles:**

- **Class-Based Typing**: Each parameter class represents a specific data type
- **Explicit Constraints**: Constraints encoded via constructor arguments (`types`, `dataType`, `layerType`)
- **Parent-Child Relationships**: Some parameters depend on others (e.g., Field → Layer)
- **Automatic Validation**: QGIS validates constraints at parameter evaluation time
- **GUI Integration**: Constraints automatically generate appropriate UI widgets

## Parameter Class Hierarchy

### Base Classes

```
QgsProcessingParameterDefinition (abstract base)
    ├── QgsProcessingParameter (base for most parameters)
    │   ├── QgsProcessingParameterFeatureSource
    │   ├── QgsProcessingParameterVectorLayer
    │   ├── QgsProcessingParameterRasterLayer
    │   ├── QgsProcessingParameterField
    │   ├── QgsProcessingParameterEnum
    │   ├── QgsProcessingParameterExtent
    │   ├── QgsProcessingParameterNumber
    │   ├── QgsProcessingParameterString
    │   └── ... (many more)
    └── QgsProcessingParameterLimitedDataTypes (mixin for type constraints)
        ├── QgsProcessingParameterFeatureSource
        ├── QgsProcessingParameterVectorLayer
        └── QgsProcessingParameterMapLayer
```

### Key Parameter Classes

#### 1. Feature Source vs Vector Layer

**QgsProcessingParameterFeatureSource:**

- **Purpose**: Accepts any feature source (vector layers, memory layers, virtual layers)
- **Flexibility**: More flexible, accepts any feature source type
- **Use Case**: When algorithm can work with any feature source

**QgsProcessingParameterVectorLayer:**

- **Purpose**: Accepts only vector layers (not other feature sources)
- **Restriction**: More restrictive, only accepts vector layers
- **Use Case**: When algorithm specifically requires a vector layer object

**Key Difference:**

```python
# FeatureSource - accepts any feature source
QgsProcessingParameterFeatureSource('INPUT', 'Input layer')

# VectorLayer - only accepts vector layers
QgsProcessingParameterVectorLayer('INPUT', 'Input vector layer')
```

## Constraint Encoding Mechanisms

### 1. Geometry Type Constraints

**Purpose**: Restrict input/output to specific geometry types (Point, Line, Polygon, etc.)

**Parameter Classes Supporting Geometry Constraints:**

- `QgsProcessingParameterFeatureSource`
- `QgsProcessingParameterVectorLayer`
- `QgsProcessingParameterFeatureSink`
- `QgsProcessingParameterVectorDestination`

**Encoding Method: `types` Parameter**

The `types` parameter accepts a list of `QgsProcessing.TypeVector*` constants:

```python
from qgis.core import (
    QgsProcessingParameterFeatureSource,
    QgsProcessing
)

# Accept only polygons
self.addParameter(
    QgsProcessingParameterFeatureSource(
        'INPUT',
        'Input polygon layer',
        types=[QgsProcessing.TypeVectorPolygon]  # Geometry type constraint
    )
)

# Accept only points
self.addParameter(
    QgsProcessingParameterFeatureSource(
        'INPUT',
        'Input point layer',
        types=[QgsProcessing.TypeVectorPoint]  # Geometry type constraint
    )
)

# Accept lines or polygons
self.addParameter(
    QgsProcessingParameterFeatureSource(
        'INPUT',
        'Input line or polygon layer',
        types=[
            QgsProcessing.TypeVectorLine,
            QgsProcessing.TypeVectorPolygon
        ]  # Multiple geometry types allowed
    )
)

# Accept any geometry type (default)
self.addParameter(
    QgsProcessingParameterFeatureSource(
        'INPUT',
        'Input layer',
        types=[QgsProcessing.TypeVectorAnyGeometry]  # Any geometry
    )
)
```

**Available Geometry Type Constants:**

```python
QgsProcessing.TypeVectorPoint          # Point geometries only
QgsProcessing.TypeVectorLine           # Line geometries only
QgsProcessing.TypeVectorPolygon        # Polygon geometries only
QgsProcessing.TypeVectorAnyGeometry    # Any geometry type
QgsProcessing.TypeVector               # Vector layer (with or without geometry)
```

**Alternative: Using `setDataTypes()` Method**

For parameters that inherit from `QgsProcessingParameterLimitedDataTypes`:

```python
from qgis.core import QgsProcessingParameterFeatureSource

param = QgsProcessingParameterFeatureSource('INPUT', 'Input layer')
param.setDataTypes([QgsProcessing.TypeVectorPolygon])  # Set constraint
self.addParameter(param)
```

**Retrieving Geometry Type Constraints:**

```python
from qgis.core import QgsProcessingParameterLimitedDataTypes

# Get parameter definition
param_def = self.parameterDefinition('INPUT')

# Check if it supports data type constraints
if isinstance(param_def, QgsProcessingParameterLimitedDataTypes):
    # Get allowed geometry types
    allowed_types = param_def.dataTypes()
    # Returns: [QgsProcessing.TypeVectorPolygon, ...]
```

**Example: Complete Geometry Type Constraint**

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingParameterFeatureSource,
    QgsProcessingParameterFeatureSink,
    QgsProcessing
)

class PolygonBufferAlgorithm(QgsProcessingAlgorithm):
    """Algorithm that only accepts polygon inputs."""

    def initAlgorithm(self, config=None):
        # Input: Only polygons accepted
        self.addParameter(
            QgsProcessingParameterFeatureSource(
                'INPUT',
                'Input polygon layer',
                types=[QgsProcessing.TypeVectorPolygon]  # Constraint: polygons only
            )
        )

        # Output: Will be polygon (same as input)
        self.addParameter(
            QgsProcessingParameterFeatureSink(
                'OUTPUT',
                'Buffered layer',
                type=QgsProcessing.TypeVectorPolygon  # Output type constraint
            )
        )

    def processAlgorithm(self, parameters, context, feedback):
        # QGIS automatically validates geometry type before this is called
        source = self.parameterAsSource(parameters, 'INPUT', context)

        # Additional runtime check (optional, but good practice)
        if source.wkbType() != QgsWkbTypes.Polygon:
            raise QgsProcessingException("Input must be a polygon layer")

        # ... processing logic ...
```

### 2. Layer Type Constraints

**Purpose**: Restrict input to specific layer types (Vector, Raster, Mesh, etc.)

**Parameter Classes Supporting Layer Type Constraints:**

- `QgsProcessingParameterMultipleLayers`
- `QgsProcessingParameterMapLayer`

**Encoding Method: `layerType` Parameter**

```python
from qgis.core import (
    QgsProcessingParameterMultipleLayers,
    QgsProcessing
)

# Accept only vector layers
self.addParameter(
    QgsProcessingParameterMultipleLayers(
        'LAYERS',
        'Input vector layers',
        layerType=QgsProcessing.TypeVectorAnyGeometry  # Layer type constraint
    )
)

# Accept only raster layers
self.addParameter(
    QgsProcessingParameterMultipleLayers(
        'RASTERS',
        'Input raster layers',
        layerType=QgsProcessing.TypeRaster  # Layer type constraint
    )
)
```

**Using `setLayerType()` Method:**

```python
param = QgsProcessingParameterMultipleLayers('LAYERS', 'Input layers')
param.setLayerType(QgsProcessing.TypeVectorPolygon)  # Set constraint
self.addParameter(param)
```

**Available Layer Type Constants:**

```python
QgsProcessing.TypeVector              # Vector layers
QgsProcessing.TypeVectorPoint        # Point vector layers
QgsProcessing.TypeVectorLine         # Line vector layers
QgsProcessing.TypeVectorPolygon      # Polygon vector layers
QgsProcessing.TypeVectorAnyGeometry  # Any geometry type
QgsProcessing.TypeRaster             # Raster layers
QgsProcessing.TypeMesh               # Mesh layers
QgsProcessing.TypePointCloud         # Point cloud layers
QgsProcessing.TypeAnnotation         # Annotation layers
```

**Example: Layer Type Constraint**

```python
from qgis.core import (
    QgsProcessingParameterMultipleLayers,
    QgsProcessing
)

class MergeVectorLayersAlgorithm(QgsProcessingAlgorithm):
    """Algorithm that merges multiple vector layers."""

    def initAlgorithm(self, config=None):
        # Only accept vector layers
        self.addParameter(
            QgsProcessingParameterMultipleLayers(
                'LAYERS',
                'Input vector layers',
                layerType=QgsProcessing.TypeVectorAnyGeometry  # Constraint: vector only
            )
        )

        # ... output parameter ...
```

### 3. Field Type Constraints

**Purpose**: Restrict field selection to specific data types (Numeric, String, Date, etc.)

**Parameter Class: `QgsProcessingParameterField`**

**Encoding Method: `type` Parameter**

```python
from qgis.core import (
    QgsProcessingParameterField,
    QgsProcessingParameterField
)

# Accept only numeric fields
self.addParameter(
    QgsProcessingParameterField(
        'FIELD',
        'Numeric field',
        parentLayerParameterName='INPUT',
        type=QgsProcessingParameterField.Numeric  # Field type constraint
    )
)

# Accept only string fields
self.addParameter(
    QgsProcessingParameterField(
        'FIELD',
        'String field',
        parentLayerParameterName='INPUT',
        type=QgsProcessingParameterField.String  # Field type constraint
    )
)

# Accept only date/time fields
self.addParameter(
    QgsProcessingParameterField(
        'FIELD',
        'Date field',
        parentLayerParameterName='INPUT',
        type=QgsProcessingParameterField.DateTime  # Field type constraint
    )
)

# Accept any field type (default)
self.addParameter(
    QgsProcessingParameterField(
        'FIELD',
        'Field',
        parentLayerParameterName='INPUT',
        type=QgsProcessingParameterField.Any  # No constraint
    )
)
```

**Available Field Type Constants:**

```python
QgsProcessingParameterField.Any          # Any field type
QgsProcessingParameterField.Numeric       # Numeric fields (int, double)
QgsProcessingParameterField.String        # String/text fields
QgsProcessingParameterField.Date          # Date fields
QgsProcessingParameterField.DateTime      # DateTime fields
QgsProcessingParameterField.Boolean       # Boolean fields
QgsProcessingParameterField.Binary        # Binary/BLOB fields
```

**Parent Layer Relationship:**

Field parameters must reference a parent layer parameter:

```python
# Define input layer first
self.addParameter(
    QgsProcessingParameterFeatureSource('INPUT', 'Input layer')
)

# Then define field parameter that references the layer
self.addParameter(
    QgsProcessingParameterField(
        'FIELD',
        'Numeric field from input',
        parentLayerParameterName='INPUT',  # Links to INPUT parameter
        type=QgsProcessingParameterField.Numeric
    )
)
```

**Using `setDataType()` Method:**

```python
param = QgsProcessingParameterField('FIELD', 'Field', parentLayerParameterName='INPUT')
param.setDataType(QgsProcessingParameterField.Numeric)  # Set constraint
self.addParameter(param)
```

**Retrieving Field Type Constraints:**

```python
# Get parameter definition
param_def = self.parameterDefinition('FIELD')

# Get field type constraint
field_type = param_def.dataType()
# Returns: QgsProcessingParameterField.Numeric, etc.

# Get parent layer parameter name
parent_layer = param_def.parentLayerParameterName()
# Returns: 'INPUT'
```

**Example: Field Type Constraint**

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingParameterFeatureSource,
    QgsProcessingParameterField,
    QgsProcessingParameterFeatureSink
)

class CalculateStatisticsAlgorithm(QgsProcessingAlgorithm):
    """Algorithm that calculates statistics on a numeric field."""

    def initAlgorithm(self, config=None):
        # Input layer
        self.addParameter(
            QgsProcessingParameterFeatureSource('INPUT', 'Input layer')
        )

        # Numeric field only
        self.addParameter(
            QgsProcessingParameterField(
                'FIELD',
                'Numeric field for statistics',
                parentLayerParameterName='INPUT',
                type=QgsProcessingParameterField.Numeric  # Constraint: numeric only
            )
        )

        # ... output parameter ...

    def processAlgorithm(self, parameters, context, feedback):
        source = self.parameterAsSource(parameters, 'INPUT', context)
        field_name = self.parameterAsString(parameters, 'FIELD', context)

        # Get field definition
        field_index = source.fields().indexOf(field_name)
        field_def = source.fields().at(field_index)

        # Runtime validation (optional, but good practice)
        if not field_def.isNumeric():
            raise QgsProcessingException("Field must be numeric")

        # ... processing logic ...
```

### 4. Enumeration Constraints

**Purpose**: Restrict selection to predefined options

**Parameter Class: `QgsProcessingParameterEnum`**

**Encoding Method: `options` Parameter**

```python
from qgis.core import QgsProcessingParameterEnum

# Define enum with options
self.addParameter(
    QgsProcessingParameterEnum(
        'OPERATION',
        'Operation type',
        options=['Add', 'Subtract', 'Multiply', 'Divide'],  # Constraint: only these options
        defaultValue=0,
        allowMultiple=False
    )
)

# Multiple selection allowed
self.addParameter(
    QgsProcessingParameterEnum(
        'OPERATIONS',
        'Operations',
        options=['Buffer', 'Simplify', 'Smooth'],
        defaultValue=[0],  # Default to first option
        allowMultiple=True  # Allow multiple selections
    )
)
```

**Retrieving Enum Value:**

```python
def processAlgorithm(self, parameters, context, feedback):
    # Get enum value (returns index: 0, 1, 2, ...)
    operation = self.parameterAsEnum(parameters, 'OPERATION', context)

    # Map to actual value
    options = ['Add', 'Subtract', 'Multiply', 'Divide']
    operation_name = options[operation]

    # ... use operation_name ...
```

**Example: Enum Constraint**

```python
from qgis.core import QgsProcessingParameterEnum

class BufferAlgorithm(QgsProcessingAlgorithm):
    """Algorithm with buffer cap style selection."""

    def initAlgorithm(self, config=None):
        # ... other parameters ...

        # Cap style enum
        self.addParameter(
            QgsProcessingParameterEnum(
                'END_CAP_STYLE',
                'End cap style',
                options=['Round', 'Flat', 'Square'],  # Constraint: only these options
                defaultValue=0
            )
        )

    def processAlgorithm(self, parameters, context, feedback):
        cap_style = self.parameterAsEnum(parameters, 'END_CAP_STYLE', context)
        # cap_style: 0=Round, 1=Flat, 2=Square

        # ... use cap_style in processing ...
```

### 5. Extent Constraints

**Purpose**: Define spatial extent parameters

**Parameter Class: `QgsProcessingParameterExtent`**

**Encoding Method: Default Extent**

```python
from qgis.core import QgsProcessingParameterExtent

# Basic extent parameter
self.addParameter(
    QgsProcessingParameterExtent(
        'EXTENT',
        'Processing extent'
    )
)

# With default extent
from qgis.core import QgsRectangle

default_extent = QgsRectangle(0, 0, 100, 100)
self.addParameter(
    QgsProcessingParameterExtent(
        'EXTENT',
        'Processing extent',
        defaultValue=default_extent
    )
)
```

**Retrieving Extent:**

```python
def processAlgorithm(self, parameters, context, feedback):
    # Get extent as QgsRectangle
    extent = self.parameterAsExtent(parameters, 'EXTENT', context)

    # Get extent with CRS
    extent, crs = self.parameterAsExtentCrs(parameters, 'EXTENT', context)

    # ... use extent ...
```

### 6. Numeric Constraints

**Purpose**: Restrict numeric inputs to ranges and types

**Parameter Class: `QgsProcessingParameterNumber`**

**Encoding Method: Type, Min, Max Parameters**

```python
from qgis.core import QgsProcessingParameterNumber

# Double with range constraint
self.addParameter(
    QgsProcessingParameterNumber(
        'DISTANCE',
        'Buffer distance',
        type=QgsProcessingParameterNumber.Double,  # Type constraint: double
        defaultValue=100.0,
        minValue=0.0,      # Minimum constraint
        maxValue=10000.0   # Maximum constraint
    )
)

# Integer with range constraint
self.addParameter(
    QgsProcessingParameterNumber(
        'ITERATIONS',
        'Number of iterations',
        type=QgsProcessingParameterNumber.Integer,  # Type constraint: integer
        defaultValue=1,
        minValue=1,        # Minimum constraint
        maxValue=100      # Maximum constraint
    )
)
```

**Available Number Types:**

```python
QgsProcessingParameterNumber.Integer  # Integer only
QgsProcessingParameterNumber.Double   # Double (floating-point)
```

**Example: Numeric Constraints**

```python
from qgis.core import QgsProcessingParameterNumber

class SmoothAlgorithm(QgsProcessingAlgorithm):
    """Algorithm with iteration count constraint."""

    def initAlgorithm(self, config=None):
        # Iterations: integer, 1-100
        self.addParameter(
            QgsProcessingParameterNumber(
                'ITERATIONS',
                'Number of iterations',
                type=QgsProcessingParameterNumber.Integer,  # Type: integer
                defaultValue=5,
                minValue=1,   # Constraint: minimum 1
                maxValue=100  # Constraint: maximum 100
            )
        )
```

## Constraint Validation

### Automatic Validation

QGIS automatically validates constraints:

1. **GUI Validation**: Processing dialog validates inputs before execution
2. **Parameter Evaluation**: `parameterAs*()` methods validate types
3. **Error Messages**: Invalid inputs generate user-friendly error messages

### Validation Flow

```
User Input (GUI/API)
    ↓
Parameter Definition (with constraints)
    ↓
Constraint Check
    ├─ Geometry Type Match? → Yes → Continue
    │                        → No  → Error: "Input must be polygon layer"
    ├─ Layer Type Match?     → Yes → Continue
    │                        → No  → Error: "Input must be vector layer"
    ├─ Field Type Match?     → Yes → Continue
    │                        → No  → Error: "Field must be numeric"
    └─ Numeric Range?        → Yes → Continue
                             → No  → Error: "Value must be between 0 and 100"
    ↓
Parameter Evaluation (parameterAs* methods)
    ↓
Algorithm Execution
```

### Custom Validation

Algorithms can implement custom validation:

```python
def checkParameterValues(self, parameters, context):
    """Custom parameter validation."""
    issues = []

    # Check geometry type
    source = self.parameterAsSource(parameters, 'INPUT', context)
    if source.wkbType() != QgsWkbTypes.Polygon:
        issues.append("Input layer must contain polygon geometries")

    # Check field type
    field_name = self.parameterAsString(parameters, 'FIELD', context)
    field_index = source.fields().indexOf(field_name)
    if field_index >= 0:
        field_def = source.fields().at(field_index)
        if not field_def.isNumeric():
            issues.append("Selected field must be numeric")

    # Check numeric range
    distance = self.parameterAsDouble(parameters, 'DISTANCE', context)
    if distance <= 0:
        issues.append("Distance must be greater than 0")

    return issues  # Empty list = no issues
```

## Complete Constraint Examples

### Example 1: Polygon Buffer with Field Constraint

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingParameterFeatureSource,
    QgsProcessingParameterField,
    QgsProcessingParameterNumber,
    QgsProcessingParameterFeatureSink,
    QgsProcessing,
    QgsProcessingParameterField
)

class PolygonBufferWithFieldAlgorithm(QgsProcessingAlgorithm):
    """Buffer polygons using distance from numeric field."""

    INPUT = 'INPUT'
    FIELD = 'FIELD'
    OUTPUT = 'OUTPUT'

    def initAlgorithm(self, config=None):
        # Input: Only polygons
        self.addParameter(
            QgsProcessingParameterFeatureSource(
                self.INPUT,
                'Input polygon layer',
                types=[QgsProcessing.TypeVectorPolygon]  # Geometry constraint
            )
        )

        # Field: Numeric only, from INPUT layer
        self.addParameter(
            QgsProcessingParameterField(
                self.FIELD,
                'Buffer distance field',
                parentLayerParameterName=self.INPUT,  # Parent relationship
                type=QgsProcessingParameterField.Numeric  # Field type constraint
            )
        )

        # Output: Polygons
        self.addParameter(
            QgsProcessingParameterFeatureSink(
                self.OUTPUT,
                'Buffered layer',
                type=QgsProcessing.TypeVectorPolygon  # Output type constraint
            )
        )

    def processAlgorithm(self, parameters, context, feedback):
        # Constraints automatically validated by QGIS
        source = self.parameterAsSource(parameters, self.INPUT, context)
        field_name = self.parameterAsString(parameters, self.FIELD, context)

        # ... processing logic ...
```

### Example 2: Multiple Layer Type Constraint

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingParameterMultipleLayers,
    QgsProcessingParameterFeatureSink,
    QgsProcessing
)

class MergePolygonLayersAlgorithm(QgsProcessingAlgorithm):
    """Merge multiple polygon layers."""

    LAYERS = 'LAYERS'
    OUTPUT = 'OUTPUT'

    def initAlgorithm(self, config=None):
        # Input: Multiple polygon layers only
        self.addParameter(
            QgsProcessingParameterMultipleLayers(
                self.LAYERS,
                'Input polygon layers',
                layerType=QgsProcessing.TypeVectorPolygon  # Layer type constraint
            )
        )

        # Output: Polygon
        self.addParameter(
            QgsProcessingParameterFeatureSink(
                self.OUTPUT,
                'Merged layer',
                type=QgsProcessing.TypeVectorPolygon  # Output type constraint
            )
        )

    def processAlgorithm(self, parameters, context, feedback):
        # Get list of layers (all validated as polygons)
        layers = self.parameterAsLayerList(parameters, self.LAYERS, context)

        # ... merge logic ...
```

### Example 3: Complex Constraints

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingParameterFeatureSource,
    QgsProcessingParameterField,
    QgsProcessingParameterEnum,
    QgsProcessingParameterNumber,
    QgsProcessingParameterFeatureSink,
    QgsProcessing,
    QgsProcessingParameterField
)

class ComplexAlgorithm(QgsProcessingAlgorithm):
    """Algorithm with multiple constraint types."""

    INPUT = 'INPUT'
    FIELD = 'FIELD'
    OPERATION = 'OPERATION'
    FACTOR = 'FACTOR'
    OUTPUT = 'OUTPUT'

    def initAlgorithm(self, config=None):
        # Input: Lines or polygons
        self.addParameter(
            QgsProcessingParameterFeatureSource(
                self.INPUT,
                'Input layer',
                types=[
                    QgsProcessing.TypeVectorLine,
                    QgsProcessing.TypeVectorPolygon
                ]  # Multiple geometry types
            )
        )

        # Field: Numeric, from INPUT
        self.addParameter(
            QgsProcessingParameterField(
                self.FIELD,
                'Numeric field',
                parentLayerParameterName=self.INPUT,
                type=QgsProcessingParameterField.Numeric  # Field type constraint
            )
        )

        # Operation: Enum constraint
        self.addParameter(
            QgsProcessingParameterEnum(
                self.OPERATION,
                'Operation',
                options=['Multiply', 'Divide', 'Add', 'Subtract'],  # Enum constraint
                defaultValue=0
            )
        )

        # Factor: Double, 0.1 to 10.0
        self.addParameter(
            QgsProcessingParameterNumber(
                self.FACTOR,
                'Factor',
                type=QgsProcessingParameterNumber.Double,  # Type constraint
                defaultValue=1.0,
                minValue=0.1,   # Range constraint
                maxValue=10.0   # Range constraint
            )
        )

        # Output: Same as input
        self.addParameter(
            QgsProcessingParameterFeatureSink(
                self.OUTPUT,
                'Output layer'
            )
        )

    def processAlgorithm(self, parameters, context, feedback):
        # All constraints validated automatically
        source = self.parameterAsSource(parameters, self.INPUT, context)
        field_name = self.parameterAsString(parameters, self.FIELD, context)
        operation = self.parameterAsEnum(parameters, self.OPERATION, context)
        factor = self.parameterAsDouble(parameters, self.FACTOR, context)

        # ... processing logic ...
```

## Constraint Encoding Summary

### Where Constraints Are Encoded

1. **Constructor Arguments**
   - `types=[...]` → Geometry type constraints
   - `layerType=...` → Layer type constraints
   - `type=...` → Field type, number type constraints
   - `minValue/maxValue` → Numeric range constraints
   - `options=[...]` → Enum constraints

2. **Property Setters**
   - `setDataTypes([...])` → Set geometry type constraints
   - `setLayerType(...)` → Set layer type constraints
   - `setDataType(...)` → Set field/number type constraints

3. **Parent Relationships**
   - `parentLayerParameterName='INPUT'` → Links field to layer parameter

### Constraint Types by Parameter Class

| Parameter Class                           | Constraint Type    | Encoding Method                        |
| ----------------------------------------- | ------------------ | -------------------------------------- |
| `QgsProcessingParameterFeatureSource`     | Geometry type      | `types=[...]` or `setDataTypes([...])` |
| `QgsProcessingParameterVectorLayer`       | Geometry type      | `types=[...]` or `setDataTypes([...])` |
| `QgsProcessingParameterFeatureSink`       | Geometry type      | `type=...` or `setDataType(...)`       |
| `QgsProcessingParameterVectorDestination` | Geometry type      | `type=...` or `setDataType(...)`       |
| `QgsProcessingParameterMultipleLayers`    | Layer type         | `layerType=...` or `setLayerType(...)` |
| `QgsProcessingParameterField`             | Field type         | `type=...` or `setDataType(...)`       |
| `QgsProcessingParameterNumber`            | Number type, range | `type=...`, `minValue`, `maxValue`     |
| `QgsProcessingParameterEnum`              | Options            | `options=[...]`                        |

### Constraint Validation Points

1. **GUI Level**: Processing dialog validates before execution
2. **Parameter Evaluation**: `parameterAs*()` methods validate types
3. **Algorithm Level**: `checkParameterValues()` for custom validation
4. **Runtime Level**: Algorithm code can perform additional checks

## Best Practices

1. **Use Appropriate Parameter Classes**
   - Use `FeatureSource` for flexibility
   - Use `VectorLayer` when layer object is required
   - Use specific types when constraints are needed

2. **Encode Constraints Explicitly**
   - Always specify geometry type constraints when algorithm requires specific types
   - Use field type constraints to guide users
   - Set numeric ranges to prevent invalid inputs

3. **Document Constraints**
   - Use clear parameter descriptions
   - Mention constraints in algorithm help text
   - Provide examples in documentation

4. **Validate at Multiple Levels**
   - Rely on automatic validation
   - Add custom validation in `checkParameterValues()`
   - Perform runtime checks in `processAlgorithm()`

5. **Use Parent Relationships**
   - Link field parameters to layer parameters
   - This ensures field selection is contextually relevant

This architecture ensures **type safety**, **user guidance**, and **robust algorithm execution** through explicit constraint encoding in parameter definitions.
