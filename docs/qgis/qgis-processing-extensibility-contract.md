# QGIS Processing Extensibility Contract: Operators, Providers, and Core State Protection

## Executive Summary

QGIS provides a **well-defined extensibility contract** for adding new Processing operators and providers through a provider-based registration system, strict parameter typing, a structured execution lifecycle, and multiple architectural guarantees that prevent plugins from corrupting core state. This document explains in detail how plugins extend QGIS Processing, the registration mechanisms, parameter typing system, execution lifecycle, and the safeguards that ensure plugin isolation and core state protection.

---

## 1. Extensibility Architecture Overview

### 1.1 Provider-Based Extension Model

QGIS uses a **provider-based architecture** where algorithms are grouped into providers:

```
┌─────────────────────────────────────────────────────────────┐
│              QGIS Processing Framework                       │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  QgsProcessingRegistry (Global Registry)            │   │
│  │  - Manages all providers                             │   │
│  │  - Indexes algorithms by ID                          │   │
│  │  - Provides lookup and execution                     │   │
│  └──────────────┬──────────────────────────────────────┘   │
│                 │                                            │
│                 ▼                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Processing Providers                                 │   │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ │   │
│  │  │   Native     │ │    GDAL     │ │   Plugin    │ │   │
│  │  │  Provider    │ │  Provider   │ │  Provider   │ │   │
│  │  │  (Core)      │ │  (Core)     │ │  (Plugin)   │ │   │
│  │  └──────┬───────┘ └──────┬──────┘ └──────┬───────┘ │   │
│  └─────────┼─────────────────┼────────────────┼─────────┘   │
│            │                 │                 │              │
│            ▼                 ▼                 ▼              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Processing Algorithms                               │   │
│  │  - QgsProcessingAlgorithm (base class)              │   │
│  │  - All algorithms inherit from same base             │   │
│  │  - Same execution contract                          │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

**Key Principles:**
- **Provider Isolation**: Each provider is independent
- **Algorithm Uniformity**: All algorithms follow same contract
- **Registry Management**: Central registry manages all providers
- **Plugin Equality**: Plugin algorithms treated same as core algorithms

### 1.2 Extension Points

**1. Provider Registration**
- Plugins can register `QgsProcessingProvider` instances
- Providers group related algorithms
- Providers have unique IDs

**2. Algorithm Registration**
- Algorithms registered via provider's `loadAlgorithms()` method
- Algorithms have unique IDs: `{provider_id}:{algorithm_id}`
- Algorithms follow same interface as core algorithms

**3. Parameter Type Extension**
- Plugins can create custom parameter types
- Custom types must inherit from `QgsProcessingParameterDefinition`
- Custom types integrate with GUI automatically

**4. Execution Lifecycle**
- All algorithms follow same execution lifecycle
- Same validation, preparation, execution, post-processing phases
- Same error handling and feedback mechanisms

---

## 2. Provider Registration

### 2.1 Provider Class Definition

**Base Class**: `QgsProcessingProvider`

```python
from qgis.core import QgsProcessingProvider
from qgis.PyQt.QtGui import QIcon

class MyPluginProvider(QgsProcessingProvider):
    """Custom Processing provider for plugin."""
    
    def id(self):
        """
        Returns unique provider identifier.
        
        Must be unique across all providers.
        Convention: Use plugin name or namespace.
        """
        return 'myplugin'
    
    def name(self):
        """
        Returns human-readable provider name.
        
        Shown in Processing Toolbox.
        """
        return self.tr('My Plugin')
    
    def icon(self):
        """
        Returns provider icon.
        
        Optional: Can return None for default icon.
        """
        return QIcon('/path/to/icon.png')
    
    def loadAlgorithms(self):
        """
        Loads all algorithms for this provider.
        
        Must call self.addAlgorithm() for each algorithm.
        Called automatically by framework.
        """
        self.addAlgorithm(MyAlgorithm1())
        self.addAlgorithm(MyAlgorithm2())
        # Add more algorithms...
    
    def supportsNonFileBasedOutput(self):
        """
        Returns whether provider supports non-file outputs.
        
        Default: True (supports memory layers, etc.)
        """
        return True
```

### 2.2 Provider Registration in Plugin

**Plugin Initialization:**

```python
from qgis.core import QgsApplication
from .processing_provider.provider import MyPluginProvider

class MyPlugin:
    """Main plugin class."""
    
    def __init__(self, iface):
        """Initialize plugin."""
        self.iface = iface
        self.provider = None
    
    def initGui(self):
        """Initialize plugin GUI."""
        # Initialize Processing provider
        self.initProcessing()
    
    def initProcessing(self):
        """
        Initialize and register Processing provider.
        
        Called during plugin initialization.
        """
        # Create provider instance
        self.provider = MyPluginProvider()
        
        # Register with Processing Registry
        QgsApplication.processingRegistry().addProvider(self.provider)
    
    def unload(self):
        """
        Unload plugin.
        
        Must unregister provider to prevent memory leaks.
        """
        if self.provider:
            # Unregister provider
            QgsApplication.processingRegistry().removeProvider(self.provider)
            self.provider = None
```

**Plugin Metadata (`metadata.txt`):**

```ini
[general]
name=My Plugin
version=1.0
qgisMinimumVersion=3.0
qgisMaximumVersion=3.99
hasProcessingProvider=yes
```

### 2.3 Processing Registry

**Registry Interface:**

```python
from qgis.core import QgsApplication

# Get Processing Registry
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

# Check if provider is registered
is_registered = registry.providerById('myplugin') is not None
```

**Registration Guarantees:**
- **Unique IDs**: Provider IDs must be unique (registry enforces)
- **Lazy Loading**: Algorithms loaded on demand via `loadAlgorithms()`
- **Thread Safety**: Registry operations are thread-safe
- **Cleanup**: Providers must be unregistered on plugin unload

---

## 3. Algorithm Registration

### 3.1 Algorithm Class Definition

**Base Class**: `QgsProcessingAlgorithm`

```python
from qgis.core import (
    QgsProcessingAlgorithm,
    QgsProcessingParameterDefinition,
    QgsProcessingContext,
    QgsProcessingFeedback,
    QgsProcessingException
)

class MyCustomAlgorithm(QgsProcessingAlgorithm):
    """Custom Processing algorithm."""
    
    # Parameter name constants
    INPUT = 'INPUT'
    OUTPUT = 'OUTPUT'
    
    def name(self):
        """
        Returns unique algorithm identifier.
        
        Format: {provider_id}:{algorithm_name}
        Must be unique across all providers.
        """
        return 'myplugin:my_algorithm'
    
    def displayName(self):
        """
        Returns human-readable algorithm name.
        
        Shown in Processing Toolbox.
        """
        return self.tr('My Custom Algorithm')
    
    def group(self):
        """
        Returns algorithm group name.
        
        Groups algorithms in Processing Toolbox.
        """
        return self.tr('My Plugin')
    
    def groupId(self):
        """
        Returns algorithm group ID.
        
        Used for grouping algorithms.
        """
        return 'myplugin'
    
    def createInstance(self):
        """
        Creates new instance of algorithm.
        
        Required for algorithm cloning.
        Must return new instance of same class.
        """
        return MyCustomAlgorithm()
    
    def initAlgorithm(self, config=None):
        """
        Defines algorithm parameters and outputs.
        
        Called once during algorithm registration.
        Must call self.addParameter() and self.addOutput().
        """
        # Input parameters
        self.addParameter(
            QgsProcessingParameterFeatureSource(
                self.INPUT,
                self.tr('Input layer')
            )
        )
        
        # Output definitions
        self.addParameter(
            QgsProcessingParameterFeatureSink(
                self.OUTPUT,
                self.tr('Output layer')
            )
        )
    
    def prepareAlgorithm(self, parameters, context, feedback):
        """
        Prepares algorithm for execution.
        
        Called before processAlgorithm().
        Should validate parameters and prepare resources.
        Returns True if preparation successful, False otherwise.
        """
        # Validate parameters
        source = self.parameterAsSource(parameters, self.INPUT, context)
        if source is None:
            return False
        
        # Prepare resources
        # ...
        
        return True
    
    def processAlgorithm(self, parameters, context, feedback):
        """
        Executes algorithm logic.
        
        Called after prepareAlgorithm() succeeds.
        Must return dictionary of output values.
        """
        # Get inputs
        source = self.parameterAsSource(parameters, self.INPUT, context)
        
        # Create output
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
            if feedback.isCanceled():
                break
            
            # Algorithm logic
            sink.addFeature(feature)
            feedback.setProgress(int(current * 100 / total))
        
        return {self.OUTPUT: dest_id}
    
    def postProcessAlgorithm(self, context, feedback):
        """
        Post-processes algorithm results.
        
        Called after processAlgorithm() completes.
        Can finalize outputs, apply styling, etc.
        """
        # Post-processing logic
        pass
```

### 3.2 Algorithm Registration via Provider

**Provider's `loadAlgorithms()` Method:**

```python
class MyPluginProvider(QgsProcessingProvider):
    def loadAlgorithms(self):
        """Load all algorithms for this provider."""
        # Register algorithms
        self.addAlgorithm(MyCustomAlgorithm())
        self.addAlgorithm(AnotherAlgorithm())
        self.addAlgorithm(YetAnotherAlgorithm())
        
        # Algorithms are now registered and available
```

**Registration Flow:**

```
1. Plugin Initialization
   └─> initProcessing()
       └─> MyPluginProvider()
           └─> QgsApplication.processingRegistry().addProvider()
               └─> Provider registered
                   └─> Framework calls loadAlgorithms()
                       └─> self.addAlgorithm() for each algorithm
                           └─> Algorithms registered in registry
```

**Registration Guarantees:**
- **Unique Algorithm IDs**: Algorithm IDs must be unique (registry enforces)
- **Lazy Loading**: Algorithms instantiated on demand
- **Thread Safety**: Registration is thread-safe
- **Cleanup**: Algorithms removed when provider unregistered

---

## 4. Parameter Typing System

### 4.1 Parameter Type Hierarchy

**Base Class**: `QgsProcessingParameterDefinition`

```python
from qgis.core import QgsProcessingParameterDefinition

class QgsProcessingParameterDefinition:
    """Base class for all parameter definitions."""
    
    def __init__(self, name, description, defaultValue=None, optional=False):
        self.name = name
        self.description = description
        self.defaultValue = defaultValue
        self.optional = optional
    
    def typeName(self):
        """Returns parameter type name."""
        return "definition"
    
    def checkValueIsAcceptable(self, value, context=None):
        """Validates parameter value."""
        return True
```

**Standard Parameter Types:**

```python
# Vector inputs
QgsProcessingParameterFeatureSource  # Feature source input
QgsProcessingParameterVectorLayer   # Vector layer input
QgsProcessingParameterMultipleLayers # Multiple layers

# Raster inputs
QgsProcessingParameterRasterLayer   # Raster layer input
QgsProcessingParameterRasterBand    # Raster band

# Outputs
QgsProcessingParameterFeatureSink   # Feature sink output
QgsProcessingParameterRasterDestination # Raster output

# Primitives
QgsProcessingParameterNumber        # Number (int/double)
QgsProcessingParameterString        # String
QgsProcessingParameterBoolean       # Boolean
QgsProcessingParameterEnum          # Enumeration

# Specialized
QgsProcessingParameterField         # Field selection
QgsProcessingParameterCrs          # CRS selection
QgsProcessingParameterFile          # File/folder
QgsProcessingParameterExtent       # Extent
```

### 4.2 Parameter Type Constraints

**Geometry Type Constraints:**

```python
from qgis.core import QgsProcessing

# Only polygons accepted
self.addParameter(
    QgsProcessingParameterFeatureSource(
        'INPUT',
        'Input polygons',
        types=[QgsProcessing.TypeVectorPolygon]  # Type constraint
    )
)

# Multiple geometry types
self.addParameter(
    QgsProcessingParameterFeatureSource(
        'INPUT',
        'Input lines or polygons',
        types=[
            QgsProcessing.TypeVectorLine,
            QgsProcessing.TypeVectorPolygon
        ]
    )
)
```

**Numeric Type Constraints:**

```python
from qgis.core import QgsProcessingParameterNumber

# Double with range
self.addParameter(
    QgsProcessingParameterNumber(
        'DISTANCE',
        'Buffer distance',
        type=QgsProcessingParameterNumber.Double,  # Type: Double
        defaultValue=100.0,
        minValue=0.0,      # Minimum constraint
        maxValue=10000.0   # Maximum constraint
    )
)

# Integer
self.addParameter(
    QgsProcessingParameterNumber(
        'ITERATIONS',
        'Number of iterations',
        type=QgsProcessingParameterNumber.Integer,  # Type: Integer
        defaultValue=1,
        minValue=1,
        maxValue=100
    )
)
```

**Field Type Constraints:**

```python
from qgis.core import QgsProcessingParameterField

# Numeric field only
self.addParameter(
    QgsProcessingParameterField(
        'FIELD',
        'Numeric field',
        parentLayerParameterName='INPUT',
        type=QgsProcessingParameterField.Numeric  # Type constraint
    )
)

# String field only
self.addParameter(
    QgsProcessingParameterField(
        'NAME_FIELD',
        'Name field',
        parentLayerParameterName='INPUT',
        type=QgsProcessingParameterField.String
    )
)
```

### 4.3 Custom Parameter Types

**Creating Custom Parameter Types:**

```python
from qgis.core import QgsProcessingParameterDefinition

class MyCustomParameter(QgsProcessingParameterDefinition):
    """Custom parameter type."""
    
    def __init__(self, name, description, defaultValue=None):
        super().__init__(name, description, defaultValue)
    
    def typeName(self):
        """Returns parameter type name."""
        return "mycustom"
    
    def clone(self):
        """Creates copy of parameter."""
        return MyCustomParameter(self.name(), self.description(), self.defaultValue())
    
    def checkValueIsAcceptable(self, value, context=None):
        """Validates parameter value."""
        # Custom validation logic
        return isinstance(value, str) and value.startswith('custom:')
```

**Registering Custom Parameter Types:**

```python
from qgis.core import QgsApplication

# Register custom parameter type
registry = QgsApplication.processingRegistry()
registry.addParameterType(MyCustomParameter())
```

### 4.4 Parameter Type Safety

**Type Validation Flow:**

```
1. Parameter Definition (initAlgorithm)
   └─> Parameter type specified
       └─> Type constraints defined
           └─> Default values set

2. Parameter Input (GUI/API)
   └─> User provides value
       └─> Widget validates type
           └─> checkValueIsAcceptable() called
               └─> Type validation

3. Parameter Evaluation (prepareAlgorithm/processAlgorithm)
   └─> parameterAsSource() / parameterAsDouble() / etc.
       └─> Type conversion and validation
           └─> Raises QgsProcessingException if invalid
```

**Type Safety Guarantees:**
- **Compile-time Type Hints**: Parameter class defines type
- **Runtime Validation**: Values validated at multiple stages
- **Type Conversion**: Automatic conversion when possible
- **Exception Handling**: Invalid types raise `QgsProcessingException`

---

## 5. Execution Lifecycle

### 5.1 Lifecycle Phases

**Complete Execution Lifecycle:**

```
1. Algorithm Selection
   └─> User selects algorithm from Toolbox
       └─> Algorithm instance created (createInstance())

2. Parameter Dialog
   └─> initAlgorithm() called to get parameters
       └─> GUI widgets created from parameter definitions
           └─> User fills parameters
               └─> Parameter validation

3. Execution Preparation
   └─> prepareAlgorithm(parameters, context, feedback)
       └─> Parameter evaluation
       └─> Resource preparation
       └─> Returns True/False

4. Algorithm Execution
   └─> processAlgorithm(parameters, context, feedback)
       └─> Core algorithm logic
       └─> Returns output dictionary

5. Post-Processing
   └─> postProcessAlgorithm(context, feedback)
       └─> Output finalization
       └─> Layer registration
       └─> Styling application

6. Result Integration
   └─> Outputs added to project
       └─> Map canvas refresh
       └─> UI updates
```

### 5.2 Phase 1: Algorithm Selection

**Algorithm Instantiation:**

```python
# Framework creates algorithm instance
algorithm = registry.algorithmById('myplugin:my_algorithm')
instance = algorithm.createInstance()

# Instance is used for execution
# Each execution gets new instance (thread-safe)
```

**Guarantees:**
- **Instance Isolation**: Each execution gets new instance
- **Thread Safety**: Instances are thread-safe
- **State Isolation**: Instances don't share state

### 5.3 Phase 2: Parameter Dialog

**Parameter Definition:**

```python
def initAlgorithm(self, config=None):
    """
    Called once during algorithm registration.
    
    Defines parameters for GUI generation.
    """
    self.addParameter(...)
```

**GUI Generation:**
- Framework automatically generates GUI widgets
- Widgets based on parameter type
- Validation happens in widgets

**Guarantees:**
- **Automatic GUI**: GUI generated from parameter definitions
- **Type-Safe Widgets**: Widgets match parameter types
- **Validation**: Widgets validate input

### 5.4 Phase 3: Execution Preparation

**prepareAlgorithm() Method:**

```python
def prepareAlgorithm(self, parameters, context, feedback):
    """
    Prepares algorithm for execution.
    
    Responsibilities:
    - Validate parameters
    - Evaluate parameter values
    - Prepare resources
    - Check prerequisites
    
    Returns:
    - True: Preparation successful, proceed to execution
    - False: Preparation failed, abort execution
    """
    # 1. Validate parameters
    source = self.parameterAsSource(parameters, self.INPUT, context)
    if source is None:
        feedback.reportError("Invalid input layer")
        return False
    
    # 2. Check prerequisites
    if source.featureCount() == 0:
        feedback.reportError("Input layer is empty")
        return False
    
    # 3. Prepare resources
    # ...
    
    return True
```

**Preparation Guarantees:**
- **Early Validation**: Parameters validated before execution
- **Resource Preparation**: Resources prepared in advance
- **Failure Handling**: Preparation failures abort execution cleanly
- **Feedback Support**: Progress and errors reported via feedback

### 5.5 Phase 4: Algorithm Execution

**processAlgorithm() Method:**

```python
def processAlgorithm(self, parameters, context, feedback):
    """
    Executes algorithm logic.
    
    Responsibilities:
    - Get input data
    - Create output sinks
    - Process data
    - Return outputs
    
    Returns:
    - Dictionary of output values
    - Keys match output parameter names
    
    Exceptions:
    - QgsProcessingException: Algorithm errors
    """
    # 1. Get inputs (already validated in prepareAlgorithm)
    source = self.parameterAsSource(parameters, self.INPUT, context)
    distance = self.parameterAsDouble(parameters, self.DISTANCE, context)
    
    # 2. Create output sink
    (sink, dest_id) = self.parameterAsSink(
        parameters,
        self.OUTPUT,
        context,
        source.fields(),
        source.wkbType(),
        source.sourceCrs()
    )
    
    # 3. Process features
    total = source.featureCount()
    for current, feature in enumerate(source.getFeatures()):
        # Check for cancellation
        if feedback.isCanceled():
            break
        
        # Process feature
        processed_feature = self.processFeature(feature, distance)
        sink.addFeature(processed_feature)
        
        # Report progress
        feedback.setProgress(int(current * 100 / total))
    
    # 4. Return outputs
    return {self.OUTPUT: dest_id}
```

**Execution Guarantees:**
- **Input Validation**: Inputs already validated in prepareAlgorithm
- **Output Creation**: Outputs created via framework methods
- **Cancellation Support**: Algorithm checks for cancellation
- **Progress Reporting**: Progress reported via feedback
- **Exception Handling**: Errors raise QgsProcessingException

### 5.6 Phase 5: Post-Processing

**postProcessAlgorithm() Method:**

```python
def postProcessAlgorithm(self, context, feedback):
    """
    Post-processes algorithm results.
    
    Responsibilities:
    - Finalize outputs
    - Apply styling
    - Register layers
    - Cleanup resources
    
    Called after processAlgorithm() completes successfully.
    """
    # 1. Get output layers from context
    layers = context.layersToLoadOnCompletion()
    
    # 2. Apply styling
    for layer_id, details in layers.items():
        layer = context.temporaryLayerStore().mapLayer(layer_id)
        if layer:
            # Apply default styling
            self.applyDefaultStyling(layer)
    
    # 3. Cleanup
    # ...
```

**Post-Processing Guarantees:**
- **Output Finalization**: Outputs finalized in main thread
- **Layer Registration**: Layers registered with project
- **Styling Application**: Default styles applied
- **Resource Cleanup**: Temporary resources cleaned up

### 5.7 Lifecycle Guarantees

**Execution Isolation:**
- **Instance Isolation**: Each execution gets new algorithm instance
- **Parameter Isolation**: Parameters isolated per execution
- **Context Isolation**: Each execution has separate context
- **State Isolation**: No shared mutable state between executions

**Error Handling:**
- **Preparation Failures**: Return False, abort cleanly
- **Execution Failures**: Raise QgsProcessingException
- **Post-Processing Failures**: Logged, don't affect execution
- **Cancellation**: Checked at multiple points, aborts cleanly

**Thread Safety:**
- **Background Execution**: Algorithms can run in background threads
- **Main Thread Post-Processing**: Post-processing in main thread
- **Context Thread Safety**: Context is thread-safe
- **Feedback Thread Safety**: Feedback is thread-safe

---

## 6. Core State Protection Guarantees

### 6.1 Input Layer Immutability

**Guarantee**: Processing algorithms **never modify input layers**

**Enforcement:**

```python
def processAlgorithm(self, parameters, context, feedback):
    # Get input source (read-only)
    source = self.parameterAsSource(parameters, self.INPUT, context)
    
    # ❌ CANNOT modify input layer
    # source.addFeature(feature)  # Not possible
    # source.changeAttributeValue(...)  # Not possible
    
    # ✅ CAN only read from input
    for feature in source.getFeatures():
        # Read feature data
        geometry = feature.geometry()
        attributes = feature.attributes()
    
    # ✅ MUST create new output
    (sink, dest_id) = self.parameterAsSink(...)
    sink.addFeature(new_feature)
```

**Architectural Mechanisms:**
- **Read-Only Sources**: `parameterAsSource()` returns read-only source
- **No Edit Access**: Sources don't provide edit methods
- **Immutable Inputs**: Input layers are immutable during processing
- **New Outputs Only**: Algorithms create new outputs, never modify inputs

### 6.2 Context Isolation

**Guarantee**: Each algorithm execution has **isolated context**

**QgsProcessingContext Isolation:**

```python
class QgsProcessingContext:
    """Execution context for algorithm."""
    
    def __init__(self):
        # Isolated per execution
        self._project = None
        self._temporaryStore = QgsMapLayerStore()
        self._layersToLoad = {}
        self._transformContext = QgsCoordinateTransformContext()
    
    def setProject(self, project):
        """Set active project (read-only reference)."""
        self._project = project  # Reference, not copy
    
    def temporaryLayerStore(self):
        """Get temporary layer store (isolated per execution)."""
        return self._temporaryStore  # Isolated store
    
    def layersToLoadOnCompletion(self):
        """Get layers to load after execution (isolated per execution)."""
        return self._layersToLoad  # Isolated list
```

**Isolation Guarantees:**
- **Separate Contexts**: Each execution has separate context
- **Temporary Isolation**: Temporary layers isolated per execution
- **No Cross-Execution State**: Contexts don't share state
- **Cleanup on Completion**: Context cleaned up after execution

### 6.3 Project State Protection

**Guarantee**: Algorithms **cannot directly modify project state**

**Project Access:**

```python
def processAlgorithm(self, parameters, context, feedback):
    # Get project (read-only access)
    project = context.project()
    
    # ❌ CANNOT directly modify project
    # project.addMapLayer(layer)  # Not allowed during execution
    # project.removeMapLayer(layer_id)  # Not allowed
    
    # ✅ CAN read project state
    layers = project.mapLayers()
    current_layer = project.currentLayer()
    
    # ✅ MUST register outputs via context
    context.addLayerToLoadOnCompletion(
        layer_id,
        QgsProcessingContext.LayerDetails(name="Output")
    )
    # Framework adds layer to project in post-processing (main thread)
```

**Protection Mechanisms:**
- **Read-Only Project Access**: Project accessed read-only during execution
- **Deferred Layer Addition**: Layers added in post-processing (main thread)
- **No Direct Modification**: Algorithms cannot directly modify project
- **Framework-Managed Integration**: Framework manages project integration

### 6.4 Temporary Resource Management

**Guarantee**: Temporary resources are **automatically managed and cleaned up**

**Temporary Layer Management:**

```python
class QgsProcessingContext:
    def temporaryLayerStore(self):
        """
        Returns temporary layer store.
        
        Temporary layers are:
        - Automatically cleaned up after execution
        - Isolated per execution
        - Not added to project automatically
        """
        return self._temporaryStore
    
    def addLayerToLoadOnCompletion(self, layerId, details):
        """
        Schedule layer to be loaded after execution.
        
        Layers are:
        - Loaded in main thread (post-processing)
        - Added to project by framework
        - Styled according to details
        """
        self._layersToLoad[layerId] = details
```

**Cleanup Guarantees:**
- **Automatic Cleanup**: Temporary layers cleaned up after execution
- **Isolated Storage**: Temporary layers isolated per execution
- **No Leaks**: Framework ensures cleanup
- **Explicit Loading**: Layers only loaded if explicitly registered

### 6.5 Error Isolation

**Guarantee**: Algorithm errors **do not corrupt core state**

**Error Handling:**

```python
def processAlgorithm(self, parameters, context, feedback):
    try:
        # Algorithm logic
        result = self.executeLogic(parameters, context, feedback)
        return result
    except Exception as e:
        # ❌ DON'T catch and suppress
        # ❌ DON'T modify project state
        # ✅ DO raise QgsProcessingException
        raise QgsProcessingException(f"Algorithm failed: {str(e)}")
```

**Error Isolation Mechanisms:**
- **Exception Propagation**: Errors propagate to framework
- **State Rollback**: Framework rolls back on errors
- **No State Corruption**: Errors don't leave state corrupted
- **Cleanup on Error**: Framework cleans up on error

### 6.6 Thread Safety

**Guarantee**: Algorithms can run in **background threads safely**

**Thread Safety Mechanisms:**

```python
# Algorithm execution in background thread
def executeAlgorithm(algorithm, parameters, context, feedback):
    # 1. Create isolated context (thread-safe)
    execution_context = QgsProcessingContext()
    execution_context.setProject(context.project())  # Read-only reference
    
    # 2. Execute in background thread
    result = algorithm.processAlgorithm(parameters, execution_context, feedback)
    
    # 3. Post-process in main thread
    QTimer.singleShot(0, lambda: postProcess(result, execution_context))
```

**Thread Safety Guarantees:**
- **Background Execution**: Algorithms can run in background threads
- **Main Thread Post-Processing**: Post-processing in main thread
- **No Shared Mutable State**: No shared mutable state between threads
- **Thread-Safe Context**: Context is thread-safe

### 6.7 Parameter Validation

**Guarantee**: Parameters are **validated before execution**

**Multi-Stage Validation:**

```python
# Stage 1: GUI validation
def validateParameters(self):
    """Validate parameters in GUI."""
    for param in self.parameters():
        if not param.checkValueIsAcceptable(value, context):
            return False
    return True

# Stage 2: Algorithm validation
def checkParameterValues(self, parameters, context):
    """Algorithm-level parameter validation."""
    issues = []
    source = self.parameterAsSource(parameters, self.INPUT, context)
    if source.featureCount() == 0:
        issues.append("Input layer is empty")
    return issues

# Stage 3: Preparation validation
def prepareAlgorithm(self, parameters, context, feedback):
    """Final validation before execution."""
    source = self.parameterAsSource(parameters, self.INPUT, context)
    if source is None:
        return False  # Abort execution
    return True
```

**Validation Guarantees:**
- **Early Validation**: Parameters validated before execution
- **Multiple Stages**: Validation at multiple stages
- **Type Safety**: Type validation at each stage
- **Constraint Checking**: Constraints validated

---

## 7. Extensibility Contract Summary

### 7.1 Registration Contract

**Provider Registration:**
- ✅ Must implement `QgsProcessingProvider`
- ✅ Must provide unique provider ID
- ✅ Must implement `loadAlgorithms()`
- ✅ Must register via `QgsApplication.processingRegistry().addProvider()`
- ✅ Must unregister on plugin unload

**Algorithm Registration:**
- ✅ Must implement `QgsProcessingAlgorithm`
- ✅ Must provide unique algorithm ID (`{provider_id}:{algorithm_id}`)
- ✅ Must implement required methods (`name()`, `displayName()`, `createInstance()`, etc.)
- ✅ Must register via provider's `addAlgorithm()`

### 7.2 Parameter Typing Contract

**Parameter Definition:**
- ✅ Must use `QgsProcessingParameterDefinition` subclasses
- ✅ Must define type constraints (geometry types, numeric ranges, etc.)
- ✅ Must provide parameter metadata (name, description, default value)

**Parameter Validation:**
- ✅ Parameters validated at multiple stages
- ✅ Type validation enforced
- ✅ Constraints validated
- ✅ Invalid parameters raise `QgsProcessingException`

### 7.3 Execution Lifecycle Contract

**Required Methods:**
- ✅ `initAlgorithm()`: Define parameters
- ✅ `prepareAlgorithm()`: Prepare execution
- ✅ `processAlgorithm()`: Execute algorithm
- ✅ `postProcessAlgorithm()`: Post-process results

**Lifecycle Guarantees:**
- ✅ Each execution gets new instance
- ✅ Parameters isolated per execution
- ✅ Context isolated per execution
- ✅ Errors handled cleanly

### 7.4 Core State Protection Contract

**Input Immutability:**
- ✅ Algorithms never modify input layers
- ✅ Inputs accessed read-only
- ✅ New outputs created, inputs unchanged

**Context Isolation:**
- ✅ Each execution has isolated context
- ✅ Temporary resources isolated per execution
- ✅ No cross-execution state sharing

**Project Protection:**
- ✅ Algorithms cannot directly modify project
- ✅ Layers added via context (deferred to main thread)
- ✅ Framework manages project integration

**Error Isolation:**
- ✅ Errors don't corrupt core state
- ✅ Framework handles error cleanup
- ✅ State rolled back on errors

**Thread Safety:**
- ✅ Algorithms can run in background threads
- ✅ Post-processing in main thread
- ✅ No shared mutable state

---

## 8. Best Practices for Plugin Developers

### 8.1 Provider Implementation

**Do:**
- ✅ Use unique provider IDs
- ✅ Implement all required methods
- ✅ Unregister provider on plugin unload
- ✅ Handle errors gracefully

**Don't:**
- ❌ Modify core providers
- ❌ Share state between algorithm instances
- ❌ Access project directly during execution
- ❌ Suppress exceptions

### 8.2 Algorithm Implementation

**Do:**
- ✅ Follow execution lifecycle contract
- ✅ Validate parameters thoroughly
- ✅ Check for cancellation regularly
- ✅ Report progress via feedback
- ✅ Create new outputs, never modify inputs

**Don't:**
- ❌ Modify input layers
- ❌ Access project directly
- ❌ Share mutable state between executions
- ❌ Suppress errors
- ❌ Block main thread

### 8.3 Parameter Definition

**Do:**
- ✅ Use appropriate parameter types
- ✅ Define type constraints
- ✅ Provide clear descriptions
- ✅ Set reasonable defaults

**Don't:**
- ❌ Use generic types when specific types available
- ❌ Skip validation
- ❌ Use ambiguous parameter names

### 8.4 Error Handling

**Do:**
- ✅ Raise `QgsProcessingException` for algorithm errors
- ✅ Return `False` from `prepareAlgorithm()` for preparation failures
- ✅ Provide clear error messages
- ✅ Let framework handle cleanup

**Don't:**
- ❌ Catch and suppress exceptions
- ❌ Modify project state on error
- ❌ Leave resources in inconsistent state

---

## 9. Conclusion

QGIS provides a **comprehensive extensibility contract** for adding Processing operators and providers through:

1. **Provider-Based Registration**: Providers group algorithms and register with global registry
2. **Strict Parameter Typing**: Type-safe parameter system with validation at multiple stages
3. **Structured Execution Lifecycle**: Well-defined phases (preparation, execution, post-processing)
4. **Core State Protection**: Multiple guarantees prevent plugins from corrupting core state

**Key Architectural Guarantees:**
- **Input Immutability**: Algorithms never modify input layers
- **Context Isolation**: Each execution has isolated context
- **Project Protection**: Algorithms cannot directly modify project
- **Temporary Resource Management**: Automatic cleanup of temporary resources
- **Error Isolation**: Errors don't corrupt core state
- **Thread Safety**: Safe execution in background threads

This contract enables **safe, extensible Processing** where plugins can add new algorithms without risking core state corruption, while maintaining **type safety**, **error handling**, and **performance** guarantees.

