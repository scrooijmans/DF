# QGIS Processing Parameter Validation & Call Chain

This document explains in detail how QGIS validates Processing parameters at runtime before execution, including parameter checks, compatible layer checks, geometry/field compatibility, CRS handling, and the complete call chain from GUI parameter widgets to `QgsProcessingAlgorithm::prepareAlgorithm` / `processAlgorithm`.

## Overview

QGIS Processing performs **multi-stage parameter validation** before algorithm execution:

1. **GUI Widget Validation**: Initial validation in parameter widgets
2. **Parameter Definition Validation**: Type and constraint checking
3. **Preprocessing**: Parameter normalization and cleaning
4. **Comprehensive Validation**: `checkParameterValues()` method
5. **CRS Validation**: Coordinate reference system compatibility
6. **Preparation**: `prepareAlgorithm()` setup
7. **Execution**: `processAlgorithm()` with runtime checks

**Key Principles:**

- **Progressive Validation**: Validation occurs at multiple stages
- **Early Failure**: Invalid parameters fail fast with clear error messages
- **Context-Aware**: Validation considers algorithm context and dependencies
- **Type Safety**: Strong type checking at parameter evaluation time
- **Compatibility Checks**: Layer, geometry, and field compatibility validated

## Validation Call Chain

### Complete Call Flow

```
User Interaction (GUI)
    ↓
1. Parameter Widget Validation
   - QgsAbstractProcessingParameterWidgetWrapper
   - checkValueIsAcceptable()
   - compatibleLayerTypes()
   - compatibleFieldTypes()
    ↓
2. Parameter Preprocessing
   - preprocessParameters()
   - Parameter normalization
   - Default value application
    ↓
3. Parameter Definition Validation
   - QgsProcessingParameterDefinition::checkValueIsAcceptable()
   - Type checking
   - Constraint validation
    ↓
4. Algorithm-Level Validation
   - checkParameterValues()
   - validateInputCrs()
   - Custom validation logic
    ↓
5. Algorithm Preparation
   - prepareAlgorithm()
   - Parameter evaluation (parameterAs* methods)
   - Resource setup
    ↓
6. Algorithm Execution
   - processAlgorithm()
   - Runtime validation
   - Error handling
```

## Stage 1: GUI Parameter Widget Validation

### Widget Wrapper System

Each parameter type has a corresponding widget wrapper that provides GUI validation:

```python
from qgis.gui import QgsAbstractProcessingParameterWidgetWrapper

class QgsAbstractProcessingParameterWidgetWrapper:
    """Base class for parameter widget wrappers."""

    def createWidget(self, context):
        """Creates the GUI widget for the parameter."""
        pass

    def setValue(self, value):
        """Sets the parameter value in the widget."""
        pass

    def value(self):
        """Gets the current parameter value from the widget."""
        pass
```

### Widget-Level Validation

**1. Value Acceptability Check**

Widgets validate values using parameter definition methods:

```python
from qgis.core import QgsProcessingParameterDefinition

# Widget checks if value is acceptable
param_def = self.parameterDefinition()
is_acceptable = param_def.checkValueIsAcceptable(user_input, context)

if not is_acceptable:
    # Show error message in widget
    self.showError("Invalid value for parameter")
```

**2. Compatible Layer Types**

For layer parameters, widgets filter available layers:

```python
from qgis.gui import QgsProcessingParameterWidgetWrapper

class VectorLayerWidgetWrapper(QgsProcessingParameterWidgetWrapper):
    """Widget wrapper for vector layer parameters."""

    def compatibleLayerTypes(self):
        """Returns compatible layer types for this parameter."""
        param_def = self.parameterDefinition()

        # Get geometry type constraints
        if isinstance(param_def, QgsProcessingParameterLimitedDataTypes):
            data_types = param_def.dataTypes()
            # Filter layers by geometry type
            return data_types

        return [QgsProcessing.TypeVectorAnyGeometry]

    def populateLayerComboBox(self):
        """Populates layer dropdown with compatible layers only."""
        compatible_types = self.compatibleLayerTypes()

        # Filter project layers
        for layer in QgsProject.instance().mapLayers().values():
            if self.isLayerCompatible(layer, compatible_types):
                self.addLayerToComboBox(layer)
```

**3. Compatible Field Types**

For field parameters, widgets filter available fields:

```python
class FieldWidgetWrapper(QgsProcessingParameterWidgetWrapper):
    """Widget wrapper for field parameters."""

    def compatibleFieldTypes(self):
        """Returns compatible field types for this parameter."""
        param_def = self.parameterDefinition()

        # Get field type constraint
        if isinstance(param_def, QgsProcessingParameterField):
            field_type = param_def.dataType()
            return field_type

        return QgsProcessingParameterField.Any

    def populateFieldComboBox(self, layer):
        """Populates field dropdown with compatible fields only."""
        compatible_type = self.compatibleFieldTypes()

        # Filter layer fields
        for field in layer.fields():
            if self.isFieldCompatible(field, compatible_type):
                self.addFieldToComboBox(field)
```

### Widget Validation Example

```python
from qgis.core import (
    QgsProcessingParameterFeatureSource,
    QgsProcessing
)
from qgis.gui import QgsProcessingParameterWidgetWrapper

class CustomVectorLayerWidget(QgsProcessingParameterWidgetWrapper):
    """Custom widget for vector layer parameter."""

    def createWidget(self, context):
        """Create layer selection widget."""
        widget = QComboBox()

        # Populate with compatible layers only
        param_def = self.parameterDefinition()
        if isinstance(param_def, QgsProcessingParameterFeatureSource):
            # Get geometry type constraint
            data_types = param_def.dataTypes()

            # Filter layers
            for layer in QgsProject.instance().mapLayers().values():
                if isinstance(layer, QgsVectorLayer):
                    # Check geometry type compatibility
                    if self.isGeometryTypeCompatible(layer, data_types):
                        widget.addItem(layer.name(), layer.id())

        # Connect validation signal
        widget.currentIndexChanged.connect(self.validateValue)

        return widget

    def isGeometryTypeCompatible(self, layer, allowed_types):
        """Check if layer geometry type is compatible."""
        layer_type = QgsProcessing.typeToSourceType(layer.wkbType())
        return layer_type in allowed_types

    def validateValue(self):
        """Validate current widget value."""
        current_value = self.value()
        param_def = self.parameterDefinition()

        # Check value acceptability
        if not param_def.checkValueIsAcceptable(current_value, self.context()):
            self.showError("Selected layer is not compatible")
            return False

        return True
```

## Stage 2: Parameter Preprocessing

### Preprocessing Method

After user input, parameters are preprocessed:

```python
from qgis.core import QgsProcessingAlgorithm

class MyAlgorithm(QgsProcessingAlgorithm):
    """Example algorithm with preprocessing."""

    def preprocessParameters(self, parameters):
        """
        Pre-processes parameters before validation.
        Called automatically after user input.
        """
        processed = {}

        for name, value in parameters.items():
            # Normalize parameter values
            if name == 'INPUT':
                # Convert layer ID to layer object if needed
                if isinstance(value, str):
                    layer = QgsProject.instance().mapLayer(value)
                    processed[name] = layer
                else:
                    processed[name] = value

            elif name == 'DISTANCE':
                # Ensure numeric value
                try:
                    processed[name] = float(value)
                except (ValueError, TypeError):
                    processed[name] = self.parameterDefinition(name).defaultValue()

            else:
                processed[name] = value

        return processed
```

### Default Value Application

Preprocessing applies default values for optional parameters:

```python
def preprocessParameters(self, parameters):
    """Apply default values for missing optional parameters."""
    processed = parameters.copy()

    for param_name in self.parameterDefinitions():
        param_def = self.parameterDefinition(param_name)

        # Apply default if parameter is optional and missing
        if param_def.isOptional() and param_name not in processed:
            processed[param_name] = param_def.defaultValue()

    return processed
```

## Stage 3: Parameter Definition Validation

### checkValueIsAcceptable Method

Each parameter definition validates its own values:

```python
from qgis.core import (
    QgsProcessingParameterDefinition,
    QgsProcessingContext
)

class QgsProcessingParameterDefinition:
    """Base class for parameter definitions."""

    def checkValueIsAcceptable(self, value, context=None):
        """
        Validates if a value is acceptable for this parameter.

        Args:
            value: The value to check
            context: Optional Processing context

        Returns:
            bool: True if value is acceptable
        """
        # Base implementation checks:
        # 1. Required parameters are not None/empty
        # 2. Value type matches parameter type
        # 3. Constraints are satisfied

        if not self.isOptional() and (value is None or value == ''):
            return False

        # Type-specific validation in subclasses
        return self._checkValueIsAcceptable(value, context)
```

### Feature Source Parameter Validation

```python
from qgis.core import (
    QgsProcessingParameterFeatureSource,
    QgsProcessingParameterLimitedDataTypes
)

class QgsProcessingParameterFeatureSource(QgsProcessingParameterLimitedDataTypes):
    """Feature source parameter with geometry type constraints."""

    def _checkValueIsAcceptable(self, value, context=None):
        """Validate feature source parameter."""
        # Check if value is a valid layer/source
        if isinstance(value, str):
            # Layer ID - check if layer exists
            if context:
                layer = context.getMapLayer(value)
                if not layer:
                    return False
                value = layer

        if isinstance(value, QgsVectorLayer):
            # Check geometry type constraint
            data_types = self.dataTypes()
            if data_types:
                layer_type = QgsProcessing.typeToSourceType(value.wkbType())
                if layer_type not in data_types:
                    return False

        return True
```

### Field Parameter Validation

```python
from qgis.core import QgsProcessingParameterField

class QgsProcessingParameterField(QgsProcessingParameter):
    """Field parameter with type constraints."""

    def _checkValueIsAcceptable(self, value, context=None):
        """Validate field parameter."""
        # Get parent layer
        parent_name = self.parentLayerParameterName()
        if not parent_name:
            return False

        # Get parent layer from context
        if context:
            parent_layer = context.getMapLayer(parent_name)
            if not parent_layer:
                return False

            # Check if field exists
            field_index = parent_layer.fields().indexOf(value)
            if field_index < 0:
                return False

            # Check field type constraint
            field_def = parent_layer.fields().at(field_index)
            required_type = self.dataType()

            if required_type != QgsProcessingParameterField.Any:
                if not self.isFieldTypeCompatible(field_def, required_type):
                    return False

        return True

    def isFieldTypeCompatible(self, field_def, required_type):
        """Check if field type matches requirement."""
        if required_type == QgsProcessingParameterField.Numeric:
            return field_def.isNumeric()
        elif required_type == QgsProcessingParameterField.String:
            return field_def.type() == QVariant.String
        elif required_type == QgsProcessingParameterField.DateTime:
            return field_def.type() in [QVariant.Date, QVariant.DateTime]
        # ... other types

        return True
```

## Stage 4: Algorithm-Level Validation

### checkParameterValues Method

Comprehensive validation of all parameters:

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingContext,
    QgsProcessingException
)

class MyAlgorithm(QgsProcessingAlgorithm):
    """Algorithm with custom validation."""

    def checkParameterValues(self, parameters, context):
        """
        Validates all parameter values.

        Args:
            parameters: Dictionary of parameter values
            context: Processing context

        Returns:
            List[str]: List of validation error messages (empty if valid)
        """
        issues = []

        # Call base class validation
        base_issues = super().checkParameterValues(parameters, context)
        issues.extend(base_issues)

        # Custom validation

        # 1. Check required parameters
        if 'INPUT' not in parameters or not parameters['INPUT']:
            issues.append("Input layer is required")

        # 2. Check layer compatibility
        source = self.parameterAsSource(parameters, 'INPUT', context)
        if source:
            # Check geometry type
            if source.wkbType() != QgsWkbTypes.Polygon:
                issues.append("Input layer must contain polygon geometries")

            # Check feature count
            if source.featureCount() == 0:
                issues.append("Input layer is empty")

        # 3. Check field compatibility
        field_name = self.parameterAsString(parameters, 'FIELD', context)
        if field_name:
            field_index = source.fields().indexOf(field_name)
            if field_index >= 0:
                field_def = source.fields().at(field_index)
                if not field_def.isNumeric():
                    issues.append("Selected field must be numeric")

        # 4. Check numeric range
        distance = self.parameterAsDouble(parameters, 'DISTANCE', context)
        if distance <= 0:
            issues.append("Distance must be greater than 0")

        return issues
```

### validateInputCrs Method

CRS compatibility validation:

```python
from qgis.core import QgsProcessingAlgorithm

class MyAlgorithm(QgsProcessingAlgorithm):
    """Algorithm with CRS validation."""

    def validateInputCrs(self, parameters, context):
        """
        Validates coordinate reference systems of input parameters.

        Args:
            parameters: Dictionary of parameter values
            context: Processing context

        Returns:
            bool: True if CRS are valid
        """
        # Base implementation checks all input CRSs are equal
        crs_list = []

        # Collect CRSs from all vector/raster inputs
        for param_name, param_value in parameters.items():
            param_def = self.parameterDefinition(param_name)

            if isinstance(param_def, (QgsProcessingParameterFeatureSource,
                                      QgsProcessingParameterRasterLayer)):
                source = self.parameterAsSource(parameters, param_name, context)
                if source and source.sourceCrs().isValid():
                    crs_list.append(source.sourceCrs())

        # Check if all CRSs are equal (base implementation)
        if len(crs_list) > 1:
            first_crs = crs_list[0]
            for crs in crs_list[1:]:
                if crs != first_crs:
                    # CRS mismatch - can override to allow different CRSs
                    return False

        return True

    def validateInputCrs(self, parameters, context):
        """Override to allow different CRSs with automatic reprojection."""
        # Allow different CRSs - algorithm will handle reprojection
        return True
```

### Compatible Layer Checks

Check layer compatibility with algorithm requirements:

```python
def checkParameterValues(self, parameters, context):
    """Check layer compatibility."""
    issues = []

    # Get input layer
    source = self.parameterAsSource(parameters, 'INPUT', context)
    if not source:
        return issues

    # Check layer type compatibility
    param_def = self.parameterDefinition('INPUT')
    if isinstance(param_def, QgsProcessingParameterLimitedDataTypes):
        allowed_types = param_def.dataTypes()
        layer_type = QgsProcessing.typeToSourceType(source.wkbType())

        if layer_type not in allowed_types:
            issues.append(
                f"Input layer geometry type ({layer_type}) is not compatible. "
                f"Allowed types: {allowed_types}"
            )

    # Check layer has required fields
    required_fields = ['id', 'name', 'value']
    for field_name in required_fields:
        if source.fields().indexOf(field_name) < 0:
            issues.append(f"Input layer must have '{field_name}' field")

    return issues
```

### Geometry Compatibility Checks

Validate geometry compatibility:

```python
def checkParameterValues(self, parameters, context):
    """Check geometry compatibility."""
    issues = []

    source = self.parameterAsSource(parameters, 'INPUT', context)
    if not source:
        return issues

    # Check geometry type
    if source.wkbType() == QgsWkbTypes.NoGeometry:
        issues.append("Input layer must have geometries")

    # Check for invalid geometries (if required)
    if self.needsValidGeometries():
        invalid_count = 0
        for feature in source.getFeatures():
            if not feature.geometry().isGeosValid():
                invalid_count += 1
                if invalid_count > 10:  # Limit check to avoid performance issues
                    break

        if invalid_count > 0:
            issues.append(
                f"Input layer contains {invalid_count}+ invalid geometries. "
                "Please fix geometries before running this algorithm."
            )

    return issues
```

### Field Compatibility Checks

Validate field compatibility:

```python
def checkParameterValues(self, parameters, context):
    """Check field compatibility."""
    issues = []

    source = self.parameterAsSource(parameters, 'INPUT', context)
    field_name = self.parameterAsString(parameters, 'FIELD', context)

    if not source or not field_name:
        return issues

    # Check field exists
    field_index = source.fields().indexOf(field_name)
    if field_index < 0:
        issues.append(f"Field '{field_name}' does not exist in input layer")
        return issues

    # Check field type
    field_def = source.fields().at(field_index)
    param_def = self.parameterDefinition('FIELD')

    if isinstance(param_def, QgsProcessingParameterField):
        required_type = param_def.dataType()

        if required_type == QgsProcessingParameterField.Numeric:
            if not field_def.isNumeric():
                issues.append(
                    f"Field '{field_name}' must be numeric. "
                    f"Current type: {field_def.typeName()}"
                )

        elif required_type == QgsProcessingParameterField.String:
            if field_def.type() != QVariant.String:
                issues.append(
                    f"Field '{field_name}' must be string type. "
                    f"Current type: {field_def.typeName()}"
                )

    return issues
```

## Stage 5: Algorithm Preparation

### prepareAlgorithm Method

Prepares algorithm for execution:

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingContext,
    QgsProcessingFeedback
)

class MyAlgorithm(QgsProcessingAlgorithm):
    """Algorithm with preparation step."""

    def prepareAlgorithm(self, parameters, context, feedback):
        """
        Prepares algorithm for execution.

        Args:
            parameters: Dictionary of parameter values
            context: Processing context
            feedback: Progress feedback object

        Returns:
            bool: True if preparation successful
        """
        # 1. Evaluate parameters (with validation)
        try:
            self.input_source = self.parameterAsSource(
                parameters, 'INPUT', context
            )
            if not self.input_source:
                feedback.reportError("Invalid input layer")
                return False

            self.field_name = self.parameterAsString(
                parameters, 'FIELD', context
            )
            self.distance = self.parameterAsDouble(
                parameters, 'DISTANCE', context
            )
        except QgsProcessingException as e:
            feedback.reportError(str(e))
            return False

        # 2. Validate CRS
        if not self.validateInputCrs(parameters, context):
            feedback.reportError("CRS validation failed")
            return False

        # 3. Setup resources
        self.setupProcessingResources(parameters, context)

        # 4. Pre-compute values if needed
        self.precomputeValues(parameters, context, feedback)

        return True

    def setupProcessingResources(self, parameters, context):
        """Setup processing resources."""
        # Create temporary files
        # Initialize data structures
        # Load reference data
        pass

    def precomputeValues(self, parameters, context, feedback):
        """Pre-compute values for efficiency."""
        # Calculate statistics
        # Build spatial indexes
        # Cache frequently accessed data
        pass
```

### Parameter Evaluation in prepareAlgorithm

Parameters are evaluated with automatic validation:

```python
def prepareAlgorithm(self, parameters, context, feedback):
    """Evaluate parameters with validation."""

    # parameterAsSource() validates:
    # - Layer exists
    # - Geometry type matches constraint
    # - Layer is accessible
    source = self.parameterAsSource(parameters, 'INPUT', context)
    # Raises QgsProcessingException if invalid

    # parameterAsLayer() validates:
    # - Layer is a QgsVectorLayer/QgsRasterLayer
    # - Layer is loaded and valid
    layer = self.parameterAsLayer(parameters, 'INPUT', context)

    # parameterAsDouble() validates:
    # - Value is numeric
    # - Value is within min/max range (if specified)
    distance = self.parameterAsDouble(parameters, 'DISTANCE', context)

    # parameterAsString() validates:
    # - Value is a string
    # - Value is not empty (if required)
    field_name = self.parameterAsString(parameters, 'FIELD', context)

    # parameterAsSink() validates:
    # - Output path is valid
    # - Output format is supported
    # - Output can be created
    sink, dest_id = self.parameterAsSink(
        parameters, 'OUTPUT', context,
        source.fields(),
        source.wkbType(),
        source.sourceCrs()
    )

    return True
```

## Stage 6: Algorithm Execution

### processAlgorithm Method

Final execution with runtime validation:

```python
def processAlgorithm(self, parameters, context, feedback):
    """
    Execute algorithm with runtime validation.

    Args:
        parameters: Dictionary of parameter values
        context: Processing context
        feedback: Progress feedback object

    Returns:
        Dict[str, Any]: Output values
    """
    # Parameters already validated in prepareAlgorithm()
    # But can perform additional runtime checks

    # 1. Get validated parameters
    source = self.parameterAsSource(parameters, 'INPUT', context)
    field_name = self.parameterAsString(parameters, 'FIELD', context)
    distance = self.parameterAsDouble(parameters, 'DISTANCE', context)

    # 2. Runtime validation
    if source.featureCount() == 0:
        raise QgsProcessingException("Input layer is empty")

    # 3. Check field exists (runtime check)
    field_index = source.fields().indexOf(field_name)
    if field_index < 0:
        raise QgsProcessingException(f"Field '{field_name}' not found")

    # 4. Process features
    total = 100.0 / source.featureCount() if source.featureCount() else 0
    for current, feature in enumerate(source.getFeatures()):
        if feedback.isCanceled():
            break

        # Process feature
        # ...

        feedback.setProgress(int(current * total))

    return {'OUTPUT': output_path}
```

## Complete Validation Example

### Full Algorithm with All Validation Stages

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingParameterFeatureSource,
    QgsProcessingParameterField,
    QgsProcessingParameterNumber,
    QgsProcessingParameterFeatureSink,
    QgsProcessing,
    QgsProcessingException,
    QgsProcessingContext,
    QgsProcessingFeedback,
    QgsWkbTypes
)

class ValidatedBufferAlgorithm(QgsProcessingAlgorithm):
    """Algorithm demonstrating complete validation chain."""

    INPUT = 'INPUT'
    FIELD = 'FIELD'
    DISTANCE = 'DISTANCE'
    OUTPUT = 'OUTPUT'

    def initAlgorithm(self, config=None):
        # Input: Polygons only
        self.addParameter(
            QgsProcessingParameterFeatureSource(
                self.INPUT,
                'Input polygon layer',
                types=[QgsProcessing.TypeVectorPolygon]  # Constraint
            )
        )

        # Field: Numeric only
        self.addParameter(
            QgsProcessingParameterField(
                self.FIELD,
                'Buffer distance field',
                parentLayerParameterName=self.INPUT,
                type=QgsProcessingParameterField.Numeric  # Constraint
            )
        )

        # Distance: Positive number
        self.addParameter(
            QgsProcessingParameterNumber(
                self.DISTANCE,
                'Additional buffer distance',
                type=QgsProcessingParameterNumber.Double,
                defaultValue=0.0,
                minValue=0.0  # Constraint
            )
        )

        # Output
        self.addParameter(
            QgsProcessingParameterFeatureSink(
                self.OUTPUT,
                'Buffered layer',
                type=QgsProcessing.TypeVectorPolygon
            )
        )

    def preprocessParameters(self, parameters):
        """Stage 2: Preprocess parameters."""
        processed = parameters.copy()

        # Apply defaults
        if self.DISTANCE not in processed:
            processed[self.DISTANCE] = 0.0

        return processed

    def checkParameterValues(self, parameters, context):
        """Stage 4: Comprehensive validation."""
        issues = []

        # Check input layer
        source = self.parameterAsSource(parameters, self.INPUT, context)
        if not source:
            issues.append("Input layer is required")
            return issues

        # Check geometry type (constraint validation)
        if source.wkbType() != QgsWkbTypes.Polygon:
            issues.append("Input layer must contain polygon geometries")

        # Check feature count
        if source.featureCount() == 0:
            issues.append("Input layer is empty")

        # Check field
        field_name = self.parameterAsString(parameters, self.FIELD, context)
        if field_name:
            field_index = source.fields().indexOf(field_name)
            if field_index < 0:
                issues.append(f"Field '{field_name}' does not exist")
            else:
                field_def = source.fields().at(field_index)
                if not field_def.isNumeric():
                    issues.append(f"Field '{field_name}' must be numeric")

        # Check distance
        distance = self.parameterAsDouble(parameters, self.DISTANCE, context)
        if distance < 0:
            issues.append("Distance must be non-negative")

        return issues

    def validateInputCrs(self, parameters, context):
        """Stage 4: CRS validation."""
        # Base implementation checks all CRSs are equal
        return super().validateInputCrs(parameters, context)

    def prepareAlgorithm(self, parameters, context, feedback):
        """Stage 5: Prepare algorithm."""
        # Evaluate parameters (automatic validation)
        self.source = self.parameterAsSource(parameters, self.INPUT, context)
        self.field_name = self.parameterAsString(parameters, self.FIELD, context)
        self.distance = self.parameterAsDouble(parameters, self.DISTANCE, context)

        # Create output sink
        (self.sink, self.dest_id) = self.parameterAsSink(
            parameters, self.OUTPUT, context,
            self.source.fields(),
            self.source.wkbType(),
            self.source.sourceCrs()
        )

        return True

    def processAlgorithm(self, parameters, context, feedback):
        """Stage 6: Execute algorithm."""
        # Parameters already validated and evaluated

        # Runtime check
        if self.source.featureCount() == 0:
            raise QgsProcessingException("Input layer is empty")

        # Process features
        total = 100.0 / self.source.featureCount() if self.source.featureCount() else 0
        field_index = self.source.fields().indexOf(self.field_name)

        for current, feature in enumerate(self.source.getFeatures()):
            if feedback.isCanceled():
                break

            # Get buffer distance from field
            field_value = feature.attribute(field_index)
            buffer_distance = float(field_value) + self.distance

            # Buffer geometry
            if feature.geometry():
                buffered = feature.geometry().buffer(buffer_distance, 5)
                feature.setGeometry(buffered)

            self.sink.addFeature(feature)
            feedback.setProgress(int(current * total))

        return {self.OUTPUT: self.dest_id}
```

## Validation Summary

### Validation Stages

| Stage                | Method                     | Purpose                       | Validation Type                 |
| -------------------- | -------------------------- | ----------------------------- | ------------------------------- |
| **1. GUI Widget**    | Widget validation          | Initial user input validation | Type, format, range             |
| **2. Preprocessing** | `preprocessParameters()`   | Parameter normalization       | Default values, type conversion |
| **3. Definition**    | `checkValueIsAcceptable()` | Parameter-level validation    | Type, constraints               |
| **4. Algorithm**     | `checkParameterValues()`   | Comprehensive validation      | Completeness, compatibility     |
| **4. CRS**           | `validateInputCrs()`       | CRS compatibility             | Coordinate systems              |
| **5. Preparation**   | `prepareAlgorithm()`       | Parameter evaluation          | Resource setup                  |
| **6. Execution**     | `processAlgorithm()`       | Runtime checks                | Final validation                |

### Validation Checks

1. **Parameter Completeness**: All required parameters provided
2. **Type Compatibility**: Parameter types match definitions
3. **Constraint Validation**: Geometry types, field types, ranges
4. **Layer Compatibility**: Layer types match requirements
5. **Geometry Compatibility**: Geometry types and validity
6. **Field Compatibility**: Field existence and types
7. **CRS Compatibility**: Coordinate reference systems
8. **Value Ranges**: Numeric values within min/max
9. **Resource Availability**: Files, layers accessible
10. **Runtime Checks**: Feature counts, data validity

### Error Handling

Validation errors are reported at each stage:

```python
# GUI Widget Level
widget.showError("Invalid value")

# Parameter Definition Level
if not param_def.checkValueIsAcceptable(value):
    return False  # Widget shows error

# Algorithm Level
issues = self.checkParameterValues(parameters, context)
if issues:
    # Show error dialog with all issues
    return False

# Preparation Level
if not self.prepareAlgorithm(parameters, context, feedback):
    # Show error message
    return False

# Execution Level
try:
    result = self.processAlgorithm(parameters, context, feedback)
except QgsProcessingException as e:
    # Show exception message
    feedback.reportError(str(e))
```

This multi-stage validation ensures **robust parameter handling**, **clear error messages**, and **reliable algorithm execution** in QGIS Processing.
