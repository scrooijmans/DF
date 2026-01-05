# QGIS Recomputation Triggers and Call Stacks

## Executive Summary

QGIS employs an **explicit recomputation model** where data regeneration and processing only occur when explicitly triggered by user actions. This design avoids implicit recomputation, ensuring predictable performance, clear user control, and preventing unexpected computational overhead. This document explains what events trigger recomputation, why QGIS avoids implicit recomputation, and provides detailed call stacks from UI actions to algorithm execution.

---

## 1. Events That Trigger Recomputation

### 1.1 Explicit Processing Execution

**Primary Trigger**: User explicitly runs a Processing algorithm

**When It Happens:**
- User clicks "Run" in Processing algorithm dialog
- User executes algorithm from Python console
- User runs algorithm from Processing Toolbox
- User executes a Processing model
- Batch processing execution

**What Gets Recomputed:**
- Processing algorithm creates **new output layers**
- Input layers are **never modified** (immutable)
- Output layers are **computed from scratch**
- No incremental updates or caching

**Key Characteristics:**
- **Explicit**: User must explicitly trigger execution
- **Deterministic**: Same inputs always produce same outputs
- **Isolated**: Processing doesn't affect existing layers
- **One-shot**: Algorithm runs once, produces output, completes

### 1.2 Layer Edits (Feature Modifications)

**Primary Trigger**: User edits features in a vector layer

**When It Happens:**
- User adds a feature to a layer
- User modifies feature geometry
- User changes feature attributes
- User deletes a feature
- User commits changes to data source

**What Gets Recomputed:**
- **Layer extent** - Recalculated when features change
- **Attribute table** - Updated to reflect changes
- **Map canvas rendering** - Repainted to show changes
- **Layer statistics** - Recalculated if needed
- **Spatial index** - Rebuilt if features added/removed

**What Does NOT Get Recomputed:**
- **Processing outputs** - Not automatically recomputed
- **Derived layers** - Not automatically updated
- **Analysis results** - Not automatically refreshed
- **Other layers** - Not affected by edits

**Key Characteristics:**
- **Local to layer**: Only affects the edited layer
- **Immediate visual update**: Map canvas repaints immediately
- **No cascading**: Doesn't trigger processing or other computations
- **Edit buffer**: Changes buffered until commit

### 1.3 Style Changes

**Primary Trigger**: User changes layer symbology, rendering, or labeling

**When It Happens:**
- User changes renderer (categorized, graduated, etc.)
- User modifies symbol properties (color, size, etc.)
- User changes labeling settings
- User adjusts layer opacity or blend mode
- User toggles layer visibility

**What Gets Recomputed:**
- **Map canvas rendering** - Repainted with new style
- **Legend** - Updated to show new symbology
- **Layer tree** - Updated if visibility changed
- **Renderer cache** - Cleared and regenerated

**What Does NOT Get Recomputed:**
- **Feature data** - Data unchanged
- **Layer extent** - Unchanged
- **Processing outputs** - Not affected
- **Analysis results** - Not affected

**Key Characteristics:**
- **Presentation only**: Style changes don't affect data
- **Immediate visual update**: Map canvas repaints immediately
- **No data regeneration**: No data processing occurs
- **Reversible**: Changes can be undone/redone

### 1.4 Layer Configuration Changes

**Primary Trigger**: User changes layer properties

**When It Happens:**
- User changes layer name
- User modifies layer CRS
- User applies layer filter (subset string)
- User changes layer scale visibility
- User modifies layer metadata

**What Gets Recomputed:**
- **Layer tree** - Updated to show new name
- **Map canvas** - Repainted if filter/visibility changed
- **CRS transformations** - Recalculated if CRS changed
- **Feature visibility** - Recalculated if filter changed

**What Does NOT Get Recomputed:**
- **Feature data** - Data unchanged
- **Processing outputs** - Not affected
- **Other layers** - Not affected

### 1.5 Project Changes

**Primary Trigger**: User modifies project-level settings

**When It Happens:**
- User changes project CRS
- User modifies project extent
- User adds/removes layers
- User changes layer order
- User modifies project properties

**What Gets Recomputed:**
- **Layer tree** - Updated structure
- **Map canvas** - Repainted with new layer order
- **CRS transformations** - Recalculated if project CRS changed
- **Layer visibility** - Recalculated based on new order

**What Does NOT Get Recomputed:**
- **Layer data** - Unchanged
- **Processing outputs** - Not affected
- **Analysis results** - Not affected

---

## 2. Why QGIS Avoids Implicit Recomputation

### 2.1 Performance Considerations

**Problem with Implicit Recomputation:**
- **Cascading computations**: One change triggers many recomputations
- **Unpredictable performance**: User doesn't know when computation occurs
- **Resource consumption**: Background computations consume CPU/memory
- **UI blocking**: Implicit recomputation can freeze UI

**QGIS Solution:**
- **Explicit control**: User decides when to recompute
- **Predictable performance**: Computation only when user requests it
- **Resource efficiency**: No background computations
- **Responsive UI**: UI remains responsive during edits

**Example:**
```python
# ❌ BAD: Implicit recomputation
# User edits feature → Buffer layer automatically recomputes
# → Intersection layer automatically recomputes
# → Analysis results automatically recompute
# → UI freezes for 30 seconds

# ✅ GOOD: Explicit recomputation
# User edits feature → Only layer repaints (instant)
# User explicitly runs buffer → Buffer layer recomputes (user-controlled)
# User explicitly runs intersection → Intersection recomputes (user-controlled)
```

### 2.2 User Control and Predictability

**Problem with Implicit Recomputation:**
- **Loss of control**: User can't prevent unwanted computations
- **Unpredictable behavior**: Changes trigger unexpected computations
- **Difficult debugging**: Hard to understand what triggered computation
- **Workflow disruption**: Computations interrupt user workflow

**QGIS Solution:**
- **User decides**: User explicitly chooses when to recompute
- **Predictable behavior**: Only explicit actions trigger computation
- **Clear workflow**: User controls computation sequence
- **No surprises**: No unexpected computations

**Example:**
```python
# User workflow with explicit recomputation:
# 1. Edit features in source layer (instant, no computation)
# 2. Review changes (instant, no computation)
# 3. When ready, explicitly run buffer algorithm (user-controlled)
# 4. Review buffer result (instant, no computation)
# 5. When ready, explicitly run intersection (user-controlled)

# vs. implicit recomputation:
# 1. Edit feature → Buffer automatically recomputes (unexpected)
# 2. Buffer recomputes → Intersection automatically recomputes (unexpected)
# 3. User workflow disrupted by unexpected computations
```

### 2.3 Data Integrity

**Problem with Implicit Recomputation:**
- **Race conditions**: Multiple recomputations can conflict
- **Inconsistent state**: Partial recomputations can leave inconsistent data
- **Dependency management**: Complex dependency tracking required
- **Error propagation**: Errors in one computation affect others

**QGIS Solution:**
- **Atomic operations**: Each computation is independent
- **Consistent state**: No partial recomputations
- **Simple dependencies**: User manages dependencies explicitly
- **Error isolation**: Errors don't propagate automatically

### 2.4 Resource Management

**Problem with Implicit Recomputation:**
- **Memory consumption**: Multiple recomputations consume memory
- **Disk I/O**: Frequent recomputations cause disk thrashing
- **CPU usage**: Background computations consume CPU
- **Battery drain**: Mobile devices drain battery quickly

**QGIS Solution:**
- **On-demand computation**: Computation only when needed
- **Efficient resource use**: No wasted computations
- **Battery friendly**: No background computations
- **Scalable**: Works with large datasets

### 2.5 Workflow Flexibility

**Problem with Implicit Recomputation:**
- **Rigid workflows**: Implicit recomputation enforces specific workflows
- **Limited experimentation**: Hard to experiment with different parameters
- **Version control**: Difficult to track computation versions
- **Reproducibility**: Hard to reproduce exact computation state

**QGIS Solution:**
- **Flexible workflows**: User can experiment freely
- **Parameter exploration**: Easy to try different parameters
- **Version control**: Each computation is explicit and trackable
- **Reproducibility**: Explicit computations are reproducible

---

## 3. Call Stack: UI Action to Algorithm Execution

### 3.1 Complete Call Stack Overview

```
User Action (GUI)
    ↓
1. UI Event Handler
   - QgsProcessingAlgorithmDialog::accept()
   - QgsProcessingToolbox::executeAlgorithm()
   - Python: processing.run()
    ↓
2. Processing Framework Entry Point
   - QgsProcessingAlgorithm::run()
   - QgsProcessingRegistry::runAlgorithm()
    ↓
3. Parameter Validation
   - QgsProcessingAlgorithm::checkParameterValues()
   - QgsProcessingParameterDefinition::checkValueIsAcceptable()
   - QgsProcessingAlgorithm::validateInputCrs()
    ↓
4. Algorithm Preparation
   - QgsProcessingAlgorithm::prepareAlgorithm()
   - parameterAsSource() / parameterAsLayer() / etc.
   - Resource setup
    ↓
5. Algorithm Execution
   - QgsProcessingAlgorithm::processAlgorithm()
   - Feature processing / Raster processing
   - Output creation
    ↓
6. Post-Processing
   - QgsProcessingAlgorithm::postProcessAlgorithm()
   - Layer registration
   - Map canvas refresh
    ↓
7. Result Return
   - Return output dictionary
   - Update UI
```

### 3.2 Detailed Call Stack: Processing Algorithm Execution

#### **Stage 1: UI Action**

```python
# User clicks "Run" button in Processing dialog
class QgsProcessingAlgorithmDialog(QDialog):
    """Processing algorithm parameter dialog."""
    
    def accept(self):
        """
        Called when user clicks "Run" button.
        
        Call stack:
        1. Validate parameters
        2. Get parameter values
        3. Create processing context
        4. Execute algorithm
        """
        # 1. Validate all parameters
        if not self.validateParameters():
            return  # Show error, don't execute
        
        # 2. Get parameter values from widgets
        parameters = self.getParameterValues()
        
        # 3. Create processing context
        context = QgsProcessingContext()
        context.setProject(QgsProject.instance())
        
        # 4. Create feedback object
        feedback = QgsProcessingFeedback()
        
        # 5. Execute algorithm
        result = processing.run(
            self.algorithm().id(),
            parameters,
            context=context,
            feedback=feedback
        )
        
        # 6. Handle result
        self.handleResult(result)
```

#### **Stage 2: Processing Framework Entry**

```python
# processing.run() is called
def run(algorithmOrId, parameters, onFinish=None, feedback=None, 
        isChildAlgorithm=False, context=None):
    """
    Execute a processing algorithm.
    
    Call stack:
    1. Get algorithm instance
    2. Validate parameters
    3. Prepare algorithm
    4. Execute algorithm
    5. Post-process
    """
    # 1. Get algorithm instance
    if isinstance(algorithmOrId, str):
        algorithm = QgsApplication.processingRegistry().algorithmById(algorithmOrId)
    else:
        algorithm = algorithmOrId
    
    if not algorithm:
        raise QgsProcessingException(f"Algorithm not found: {algorithmOrId}")
    
    # 2. Create context if not provided
    if context is None:
        context = QgsProcessingContext()
        context.setProject(QgsProject.instance())
    
    # 3. Create feedback if not provided
    if feedback is None:
        feedback = QgsProcessingFeedback()
    
    # 4. Preprocess parameters
    parameters = algorithm.preprocessParameters(parameters, context)
    
    # 5. Validate parameters
    issues = algorithm.checkParameterValues(parameters, context)
    if issues:
        raise QgsProcessingException(f"Parameter validation failed: {issues}")
    
    # 6. Prepare algorithm
    if not algorithm.prepareAlgorithm(parameters, context, feedback):
        raise QgsProcessingException("Algorithm preparation failed")
    
    # 7. Execute algorithm
    try:
        result = algorithm.processAlgorithm(parameters, context, feedback)
    except Exception as e:
        feedback.reportError(str(e))
        raise
    
    # 8. Post-process
    algorithm.postProcessAlgorithm(context, feedback)
    
    return result
```

#### **Stage 3: Parameter Validation**

```python
class QgsProcessingAlgorithm:
    """Base class for processing algorithms."""
    
    def checkParameterValues(self, parameters, context):
        """
        Validate all parameter values.
        
        Call stack:
        1. Check required parameters
        2. Validate parameter types
        3. Check layer compatibility
        4. Check field compatibility
        5. Check CRS compatibility
        6. Custom validation
        """
        issues = []
        
        # 1. Check required parameters
        for param_name, param_def in self.parameterDefinitions().items():
            if not param_def.isOptional():
                if param_name not in parameters or not parameters[param_name]:
                    issues.append(f"Required parameter '{param_name}' is missing")
        
        # 2. Validate parameter types
        for param_name, value in parameters.items():
            param_def = self.parameterDefinition(param_name)
            if param_def:
                if not param_def.checkValueIsAcceptable(value, context):
                    issues.append(f"Parameter '{param_name}' has invalid value")
        
        # 3. Check layer compatibility
        for param_name, value in parameters.items():
            param_def = self.parameterDefinition(param_name)
            if isinstance(param_def, QgsProcessingParameterFeatureSource):
                source = self.parameterAsSource(parameters, param_name, context)
                if source:
                    # Check geometry type
                    if isinstance(param_def, QgsProcessingParameterLimitedDataTypes):
                        allowed_types = param_def.dataTypes()
                        layer_type = QgsProcessing.typeToSourceType(source.wkbType())
                        if layer_type not in allowed_types:
                            issues.append(
                                f"Parameter '{param_name}': geometry type "
                                f"{layer_type} not compatible"
                            )
        
        # 4. Check field compatibility
        for param_name, value in parameters.items():
            param_def = self.parameterDefinition(param_name)
            if isinstance(param_def, QgsProcessingParameterField):
                field_name = self.parameterAsString(parameters, param_name, context)
                parent_name = param_def.parentLayerParameterName()
                if parent_name and field_name:
                    parent_source = self.parameterAsSource(
                        parameters, parent_name, context
                    )
                    if parent_source:
                        field_index = parent_source.fields().indexOf(field_name)
                        if field_index < 0:
                            issues.append(
                                f"Field '{field_name}' not found in parent layer"
                            )
                        else:
                            field_def = parent_source.fields().at(field_index)
                            required_type = param_def.dataType()
                            if not self.isFieldTypeCompatible(field_def, required_type):
                                issues.append(
                                    f"Field '{field_name}' type not compatible"
                                )
        
        # 5. Check CRS compatibility
        if not self.validateInputCrs(parameters, context):
            issues.append("Input CRS validation failed")
        
        # 6. Custom validation (subclass can override)
        custom_issues = self.checkParameterValuesCustom(parameters, context)
        issues.extend(custom_issues)
        
        return issues
```

#### **Stage 4: Algorithm Preparation**

```python
class QgsProcessingAlgorithm:
    """Base class for processing algorithms."""
    
    def prepareAlgorithm(self, parameters, context, feedback):
        """
        Prepare algorithm for execution.
        
        Call stack:
        1. Evaluate parameters
        2. Validate CRS
        3. Setup resources
        4. Pre-compute values
        """
        # 1. Evaluate parameters (with automatic validation)
        try:
            self.input_source = self.parameterAsSource(
                parameters, self.INPUT, context
            )
            if not self.input_source:
                feedback.reportError("Invalid input layer")
                return False
            
            self.distance = self.parameterAsDouble(
                parameters, self.DISTANCE, context
            )
            if self.distance <= 0:
                feedback.reportError("Distance must be greater than 0")
                return False
        except QgsProcessingException as e:
            feedback.reportError(str(e))
            return False
        
        # 2. Validate CRS
        if not self.validateInputCrs(parameters, context):
            feedback.reportError("CRS validation failed")
            return False
        
        # 3. Setup resources
        self.setupProcessingResources(parameters, context)
        
        # 4. Create output sink
        (self.sink, self.dest_id) = self.parameterAsSink(
            parameters,
            self.OUTPUT,
            context,
            self.input_source.fields(),
            self.input_source.wkbType(),
            self.input_source.sourceCrs()
        )
        
        return True
```

#### **Stage 5: Algorithm Execution**

```python
class QgsProcessingAlgorithm:
    """Base class for processing algorithms."""
    
    def processAlgorithm(self, parameters, context, feedback):
        """
        Execute algorithm.
        
        Call stack:
        1. Get validated parameters
        2. Process features/raster
        3. Write to output
        4. Return result
        """
        # Parameters already validated in prepareAlgorithm()
        
        # 1. Get validated parameters
        source = self.parameterAsSource(parameters, self.INPUT, context)
        distance = self.parameterAsDouble(parameters, self.DISTANCE, context)
        
        # 2. Get output sink (created in prepareAlgorithm)
        sink = self.sink
        dest_id = self.dest_id
        
        # 3. Process features
        total = 100.0 / source.featureCount() if source.featureCount() else 0
        
        for current, feature in enumerate(source.getFeatures()):
            # Check for cancellation
            if feedback.isCanceled():
                break
            
            # Process feature
            if feature.geometry():
                # Buffer geometry
                buffered = feature.geometry().buffer(distance, 5)
                feature.setGeometry(buffered)
            
            # Add to output
            sink.addFeature(feature)
            
            # Update progress
            feedback.setProgress(int(current * total))
        
        # 4. Return result
        return {self.OUTPUT: dest_id}
```

#### **Stage 6: Post-Processing**

```python
class QgsProcessingAlgorithm:
    """Base class for processing algorithms."""
    
    def postProcessAlgorithm(self, context, feedback):
        """
        Post-process after algorithm execution.
        
        Call stack:
        1. Get output layer
        2. Register in project
        3. Refresh map canvas
        """
        # 1. Get output layer from context
        output_layer = context.takeResultLayer(self.OUTPUT)
        
        if output_layer:
            # 2. Register in project
            if context.project():
                context.project().addMapLayer(output_layer)
            
            # 3. Refresh map canvas
            if context.feedback():
                context.feedback().pushInfo(
                    f"Output layer '{output_layer.name()}' added to project"
                )
        
        return {}
```

### 3.3 Call Stack: Layer Edit Operations

#### **Complete Call Stack for Feature Edit**

```
User Action (Edit Feature)
    ↓
1. UI Event Handler
   - QgsMapToolEdit::canvasReleaseEvent()
   - QgsMapToolAddFeature::canvasReleaseEvent()
   - Attribute table edit
    ↓
2. Layer Edit Operation
   - QgsVectorLayer::addFeature()
   - QgsVectorLayer::changeAttributeValue()
   - QgsVectorLayer::deleteFeature()
    ↓
3. Edit Buffer Update
   - QgsVectorLayerEditBuffer::addFeature()
   - QgsVectorLayerEditBuffer::changeAttributeValue()
   - QgsVectorLayerEditBuffer::deleteFeature()
    ↓
4. Signal Emission
   - featureAdded(fid, feature)
   - attributeValueChanged(fid, field, value)
   - featureDeleted(fid)
   - repaintRequested()
    ↓
5. View Updates
   - QgsMapCanvas::refresh() (via repaintRequested signal)
   - QgsAttributeTable::refresh() (via featureAdded signal)
   - QgsLayerTreeModel::update() (via layer signals)
    ↓
6. Commit (User Action)
   - QgsVectorLayer::commitChanges()
   - QgsVectorLayerEditBuffer::commitChanges()
   - Data provider write
   - Committed signals emitted
```

#### **Detailed Call Stack: Adding a Feature**

```python
# User clicks on map to add feature
class QgsMapToolAddFeature(QgsMapTool):
    """Map tool for adding features."""
    
    def canvasReleaseEvent(self, event):
        """
        Called when user clicks on map.
        
        Call stack:
        1. Get click point
        2. Create feature
        3. Add to layer
        4. Update UI
        """
        # 1. Get click point
        point = self.toMapCoordinates(event.pos())
        
        # 2. Create feature
        feature = QgsFeature(self.layer.fields())
        feature.setGeometry(QgsGeometry.fromPointXY(point))
        feature.setAttributes([...])
        
        # 3. Add to layer (triggers edit buffer update)
        if self.layer.isEditable():
            fid = self.layer.addFeature(feature)
            # Triggers: featureAdded signal, repaintRequested signal
        
        # 4. Update UI (handled by signals)
        # Map canvas repaints automatically via repaintRequested signal
```

```python
class QgsVectorLayer:
    """Vector layer with editing support."""
    
    def addFeature(self, feature):
        """
        Add feature to edit buffer.
        
        Call stack:
        1. Check edit session active
        2. Add to edit buffer
        3. Emit signals
        4. Request repaint
        """
        # 1. Check edit session active
        if not self.isEditable():
            return False
        
        # 2. Add to edit buffer
        fid = self._editBuffer.addFeature(feature)
        
        # 3. Emit signals
        self.featureAdded.emit(fid, feature)
        
        # 4. Request repaint
        self.repaintRequested.emit()
        
        return fid
```

```python
class QgsVectorLayerEditBuffer:
    """Edit buffer for vector layer."""
    
    def addFeature(self, feature):
        """
        Add feature to buffer.
        
        Call stack:
        1. Generate temporary feature ID
        2. Store in buffer
        3. Return feature ID
        """
        # 1. Generate temporary feature ID
        fid = self._nextTemporaryFeatureId
        self._nextTemporaryFeatureId -= 1
        
        # 2. Store in buffer
        self._addedFeatures[fid] = feature
        
        return fid
```

### 3.4 Call Stack: Style Changes

#### **Complete Call Stack for Style Change**

```
User Action (Change Style)
    ↓
1. UI Event Handler
   - QgsRendererPropertiesWidget::apply()
   - QgsSymbolSelectorWidget::apply()
   - Layer properties dialog
    ↓
2. Layer Style Update
   - QgsVectorLayer::setRenderer()
   - QgsVectorLayer::setLabeling()
   - QgsVectorLayer::setOpacity()
    ↓
3. Signal Emission
   - rendererChanged()
   - labelingChanged()
   - configChanged()
   - repaintRequested()
    ↓
4. View Updates
   - QgsMapCanvas::refresh() (via repaintRequested signal)
   - QgsLayerTreeModel::update() (via configChanged signal)
   - QgsLegendModel::update() (via rendererChanged signal)
    ↓
5. Renderer Cache Clear
   - QgsFeatureRenderer::clearCache()
   - QgsMapCanvas::clearCache()
```

#### **Detailed Call Stack: Changing Renderer**

```python
# User changes renderer in layer properties
class QgsRendererPropertiesWidget(QWidget):
    """Widget for configuring layer renderer."""
    
    def apply(self):
        """
        Apply renderer changes to layer.
        
        Call stack:
        1. Create renderer from widget
        2. Set renderer on layer
        3. Update UI
        """
        # 1. Create renderer from widget
        renderer = self.createRenderer()
        
        # 2. Set renderer on layer
        self.layer.setRenderer(renderer)
        # Triggers: rendererChanged signal, configChanged signal, repaintRequested signal
        
        # 3. Update UI (handled by signals)
        # Map canvas repaints automatically via repaintRequested signal
```

```python
class QgsVectorLayer:
    """Vector layer with rendering support."""
    
    def setRenderer(self, renderer):
        """
        Set renderer for layer.
        
        Call stack:
        1. Store renderer
        2. Clear renderer cache
        3. Emit signals
        4. Request repaint
        """
        # 1. Store renderer
        self._renderer = renderer
        
        # 2. Clear renderer cache
        if renderer:
            renderer.clearCache()
        
        # 3. Emit signals
        self.rendererChanged.emit()
        self.configChanged.emit()
        
        # 4. Request repaint
        self.repaintRequested.emit()
```

---

## 4. Comparison: Explicit vs Implicit Recomputation

### 4.1 Explicit Recomputation (QGIS Model)

**Characteristics:**
- User explicitly triggers computation
- Computation occurs immediately when triggered
- No background computations
- Predictable performance
- User controls when computation happens

**Example:**
```python
# User workflow:
# 1. Edit source layer (instant, no computation)
# 2. Review changes (instant, no computation)
# 3. Explicitly run buffer algorithm (user-controlled, explicit)
# 4. Review buffer result (instant, no computation)
# 5. Explicitly run intersection (user-controlled, explicit)
```

**Benefits:**
- **Performance**: No unexpected computations
- **Control**: User decides when to compute
- **Predictability**: Clear when computation occurs
- **Resource efficiency**: No wasted computations

### 4.2 Implicit Recomputation (Hypothetical Model)

**Characteristics:**
- Computation triggered automatically by changes
- Computation occurs in background
- Cascading computations possible
- Unpredictable performance
- System controls when computation happens

**Example:**
```python
# Hypothetical implicit recomputation:
# 1. User edits source layer
#    → Buffer layer automatically recomputes (implicit, unexpected)
#    → Intersection layer automatically recomputes (implicit, unexpected)
#    → Analysis results automatically recompute (implicit, unexpected)
#    → UI freezes for 30 seconds (unexpected)
```

**Problems:**
- **Performance**: Unexpected computations freeze UI
- **Control**: User can't prevent computations
- **Predictability**: Hard to know when computation occurs
- **Resource waste**: Unnecessary computations

---

## 5. Practical Examples

### 5.1 Processing Algorithm Execution

```python
# User clicks "Run" in Processing dialog
# Complete call stack:

# 1. UI Action
QgsProcessingAlgorithmDialog.accept()
    ↓
# 2. Processing Framework
processing.run('native:buffer', {
    'INPUT': layer_id,
    'DISTANCE': 100,
    'OUTPUT': 'memory:'
})
    ↓
# 3. Parameter Validation
QgsProcessingAlgorithm.checkParameterValues()
    ↓
# 4. Algorithm Preparation
QgsProcessingAlgorithm.prepareAlgorithm()
    ↓
# 5. Algorithm Execution
QgsProcessingAlgorithm.processAlgorithm()
    ↓
# 6. Post-Processing
QgsProcessingAlgorithm.postProcessAlgorithm()
    ↓
# 7. Result
# New layer added to project
```

### 5.2 Layer Edit Operation

```python
# User adds feature to layer
# Complete call stack:

# 1. UI Action
QgsMapToolAddFeature.canvasReleaseEvent()
    ↓
# 2. Layer Edit
QgsVectorLayer.addFeature(feature)
    ↓
# 3. Edit Buffer
QgsVectorLayerEditBuffer.addFeature(feature)
    ↓
# 4. Signal Emission
featureAdded.emit(fid, feature)
repaintRequested.emit()
    ↓
# 5. View Updates
QgsMapCanvas.refresh()  # Via repaintRequested signal
QgsAttributeTable.refresh()  # Via featureAdded signal
    ↓
# 6. Commit (User Action)
QgsVectorLayer.commitChanges()
    ↓
# 7. Data Persistence
QgsVectorDataProvider.addFeatures()
```

### 5.3 Style Change

```python
# User changes layer renderer
# Complete call stack:

# 1. UI Action
QgsRendererPropertiesWidget.apply()
    ↓
# 2. Layer Style Update
QgsVectorLayer.setRenderer(renderer)
    ↓
# 3. Signal Emission
rendererChanged.emit()
configChanged.emit()
repaintRequested.emit()
    ↓
# 4. View Updates
QgsMapCanvas.refresh()  # Via repaintRequested signal
QgsLegendModel.update()  # Via rendererChanged signal
    ↓
# 5. Renderer Cache Clear
QgsFeatureRenderer.clearCache()
```

---

## 6. Key Architectural Principles

### 6.1 Explicit Control

**Principle**: User explicitly controls when computation occurs

**Implementation:**
- Processing algorithms only run when user explicitly triggers them
- No automatic recomputation of derived layers
- No background computations

**Benefits:**
- Predictable performance
- User control
- Resource efficiency

### 6.2 Isolation

**Principle**: Changes to one layer don't automatically affect others

**Implementation:**
- Layer edits only affect the edited layer
- Processing outputs are independent
- No cascading computations

**Benefits:**
- Clear boundaries
- Predictable behavior
- Easy debugging

### 6.3 Signal-Based Updates

**Principle**: Visual updates use signals, not recomputation

**Implementation:**
- Layer edits emit signals
- Style changes emit signals
- Views subscribe to signals for updates

**Benefits:**
- Efficient updates
- Decoupled architecture
- Responsive UI

### 6.4 Immutable Processing Outputs

**Principle**: Processing outputs are immutable

**Implementation:**
- Processing creates new layers
- Input layers never modified
- Outputs don't automatically update

**Benefits:**
- Data integrity
- Reproducibility
- Version control

---

## 7. Conclusion

QGIS's explicit recomputation model provides:

1. **Predictable Performance**: Computation only occurs when explicitly triggered
2. **User Control**: Users decide when to recompute
3. **Resource Efficiency**: No wasted background computations
4. **Clear Workflows**: Explicit computation steps are easy to understand
5. **Data Integrity**: Immutable processing outputs ensure consistency

The call stacks from UI actions to algorithm execution demonstrate the multi-stage validation and execution process, ensuring robust and reliable computation while maintaining user control and predictable performance.

This architecture is particularly important for:
- **Large datasets**: Avoiding unexpected computations on large datasets
- **Complex workflows**: Managing complex multi-step processing workflows
- **Performance-critical applications**: Ensuring responsive UI
- **Reproducibility**: Making computations explicit and trackable

