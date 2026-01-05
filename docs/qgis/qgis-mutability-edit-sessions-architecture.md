# QGIS Mutability, Edit Sessions, and Commit/Rollback Architecture

## Executive Summary

QGIS employs a sophisticated mutability model that distinguishes between **mutable objects** (layers, features, attributes, styles) that can be modified through controlled edit sessions, and **immutable objects** (processing outputs, temporary layers) that are created once and never modified. This architecture enforces strict boundaries around when and how mutations are allowed, using edit sessions with commit/rollback semantics to ensure data integrity and provide transactional guarantees.

---

## 1. Mutability Classification

### 1.1 Mutable Objects

**Mutable objects** in QGIS can be modified after creation, but only under specific conditions:

#### **1.1.1 Layers (QgsMapLayer and Subclasses)**

**Mutable Properties:**
- **Symbology and styling** - Always mutable
- **Layer name** - Always mutable
- **Layer configuration** - Always mutable
- **CRS** - Mutable (with reprojection)
- **Data source** - Mutable (can be changed)
- **Feature data** - Mutable only during edit sessions (vector layers)
- **Raster data** - Generally immutable (read-only)

**Mutability Rules:**
```python
# Layer properties can always be changed
layer.setName("New Name")  # Always allowed
layer.setRenderer(renderer)  # Always allowed
layer.setCrs(new_crs)  # Always allowed

# Feature data requires edit session
layer.startEditing()  # Required for feature modifications
layer.addFeature(feature)  # Only allowed during edit session
layer.commitChanges()  # Persist changes
```

**Key Characteristics:**
- **Style mutability**: No restrictions - styles can be changed at any time
- **Data mutability**: Requires edit session for vector layers
- **Raster mutability**: Raster data is typically read-only (new rasters created instead)
- **Configuration mutability**: Layer settings can be modified without restrictions

#### **1.1.2 Features (QgsFeature)**

**Mutable Properties:**
- **Geometry** - Mutable during edit sessions
- **Attributes** - Mutable during edit sessions
- **Feature ID** - Immutable (assigned by data provider)

**Mutability Rules:**
```python
# Features are mutable only during edit sessions
layer.startEditing()

# Modify feature geometry
feature.setGeometry(new_geometry)
layer.updateFeature(feature)

# Modify feature attributes
layer.changeAttributeValue(feature_id, field_index, new_value)

# Commit or rollback
layer.commitChanges()  # Persist changes
# OR
layer.rollBack()  # Discard changes
```

**Key Characteristics:**
- **Edit session required**: Features cannot be modified outside edit sessions
- **Buffer-based**: Changes stored in edit buffer until commit
- **Transactional**: All changes committed or rolled back together
- **Undo support**: Changes can be undone before commit

#### **1.1.3 Attributes**

**Mutable Properties:**
- **Attribute values** - Mutable during edit sessions
- **Field definitions** - Mutable (but requires data provider support)

**Mutability Rules:**
```python
# Change attribute value
layer.startEditing()
layer.changeAttributeValue(feature_id, field_index, new_value)
layer.commitChanges()

# Add/remove fields (requires provider support)
layer.startEditing()
layer.addAttribute(QgsField("new_field", QVariant.String))
layer.commitChanges()
```

**Key Characteristics:**
- **Value mutability**: Attribute values can be changed during edit sessions
- **Schema mutability**: Field definitions can be modified (provider-dependent)
- **Type constraints**: Attribute types must match field definitions
- **Default values**: Can be set for new features

#### **1.1.4 Styles (Symbology, Rendering, Labeling)**

**Mutable Properties:**
- **Symbology** - Always mutable
- **Renderer** - Always mutable
- **Labeling** - Always mutable
- **Opacity** - Always mutable
- **Scale dependencies** - Always mutable

**Mutability Rules:**
```python
# Styles can be changed at any time (no edit session required)
layer.setRenderer(renderer)  # Always allowed
layer.setLabeling(label_settings)  # Always allowed
layer.setOpacity(0.5)  # Always allowed
layer.setScaleBasedVisibility(True)  # Always allowed

# Style changes are immediate and don't require commit
# Changes are saved to project file when project is saved
```

**Key Characteristics:**
- **No restrictions**: Styles can be modified at any time
- **Immediate effect**: Changes take effect immediately
- **Project persistence**: Style changes saved to project file
- **No transaction**: Style changes don't require commit/rollback
- **Signal-based**: Style changes emit signals for view updates

### 1.2 Immutable Objects

**Immutable objects** in QGIS are created once and never modified:

#### **1.2.1 Processing Outputs**

**Immutable Characteristics:**
- **Processing algorithms create new layers** - Inputs are never modified
- **Output layers are immutable** - Once created, cannot be modified
- **File-based outputs** - New files created, originals unchanged
- **Memory outputs** - Temporary layers created in memory

**Immutability Rules:**
```python
# Processing framework creates new layers (inputs immutable)
result = processing.run('native:buffer', {
    'INPUT': input_layer,  # Input is NOT modified
    'DISTANCE': 100,
    'OUTPUT': 'memory:'  # New layer created
})

output_layer = result['OUTPUT']  # New immutable layer

# Attempting to modify processing output
output_layer.startEditing()  # May fail or create edit buffer
# But original processing output remains unchanged
```

**Key Characteristics:**
- **Input immutability**: Processing algorithms never modify input layers
- **Output immutability**: Processing outputs are created once and not modified
- **Copy semantics**: Processing creates copies, not references
- **No edit sessions**: Processing outputs don't support edit sessions by default
- **Temporary nature**: Many processing outputs are temporary

**Exceptions:**
- **In-place algorithms**: Some algorithms can modify inputs (e.g., field calculator)
- **User override**: Users can specify existing layers as outputs (with confirmation)

#### **1.2.2 Temporary Layers**

**Immutable Characteristics:**
- **Memory layers** - Created in memory, not persisted
- **Processing intermediates** - Temporary results from algorithms
- **Virtual layers** - SQL-based views (read-only)

**Immutability Rules:**
```python
# Memory layer (temporary, immutable)
memory_layer = QgsVectorLayer('memory:', 'Temp Layer', 'memory')
# Can add features, but layer itself is temporary
# Not saved to project file

# Virtual layer (read-only, immutable)
virtual_layer = QgsVectorLayer(
    '?query=SELECT * FROM layer1',
    'Virtual Layer',
    'virtual'
)
# Cannot be edited - read-only
```

**Key Characteristics:**
- **Temporary storage**: Not persisted to disk
- **No persistence**: Removed when project closes (unless explicitly saved)
- **Read-only**: Many temporary layers are read-only
- **No edit sessions**: Temporary layers often don't support editing

#### **1.2.3 Raster Data**

**Immutable Characteristics:**
- **Raster pixels** - Cannot be modified in-place
- **Raster bands** - Read-only
- **Raster values** - Cannot be changed directly

**Immutability Rules:**
```python
# Raster layers are read-only
raster_layer = QgsRasterLayer('/path/to/raster.tif', 'Raster')

# Cannot modify raster data directly
# Must create new raster for modifications
processing.run('gdal:rastercalculator', {
    'INPUT_A': raster_layer,
    'FORMULA': 'A * 2',  # Creates new raster
    'OUTPUT': '/path/to/output.tif'
})
```

**Key Characteristics:**
- **Read-only data**: Raster data cannot be modified
- **New rasters**: Modifications create new raster files
- **No edit sessions**: Raster layers don't support edit sessions
- **Processing-based**: Changes made through processing algorithms

---

## 2. When Mutation is Allowed

### 2.1 Style Mutations (Always Allowed)

**No Restrictions:**
- Styles can be modified at any time
- No edit session required
- Changes are immediate
- No commit/rollback needed

```python
# Style changes are always allowed
layer.setRenderer(renderer)
layer.setLabeling(label_settings)
layer.setOpacity(0.5)
layer.setBlendMode(QPainter.CompositionMode_Multiply)
```

**Why Always Allowed:**
- Styles are presentation-only (don't affect data)
- Changes are reversible (undo/redo at project level)
- No data integrity concerns
- Performance impact is minimal

### 2.2 Feature Mutations (Edit Session Required)

**Restrictions:**
- **Edit session must be active** - `startEditing()` must be called
- **Layer must support editing** - Provider must have edit capabilities
- **Data source must be writable** - File must exist and be writable
- **No concurrent editing** - Only one edit session per layer

```python
# Check if editing is possible
if layer.isEditable():
    # Already in edit session
    pass
elif layer.dataProvider().capabilities() & QgsVectorDataProvider.EditCapabilities:
    # Can start editing
    layer.startEditing()
else:
    # Cannot edit (read-only source)
    pass
```

**Edit Capability Check:**
```python
# Check provider capabilities
capabilities = layer.dataProvider().capabilities()

# Edit capabilities
if capabilities & QgsVectorDataProvider.AddFeatures:
    # Can add features
    pass

if capabilities & QgsVectorDataProvider.DeleteFeatures:
    # Can delete features
    pass

if capabilities & QgsVectorDataProvider.ChangeAttributeValues:
    # Can change attributes
    pass

if capabilities & QgsVectorDataProvider.ChangeGeometries:
    # Can change geometries
    pass
```

### 2.3 Layer Configuration Mutations (Always Allowed)

**No Restrictions:**
- Layer name, CRS, extent can be changed
- Layer properties can be modified
- Layer visibility can be toggled
- Layer filters can be applied

```python
# Configuration changes are always allowed
layer.setName("New Name")
layer.setCrs(new_crs)
layer.setSubsetString("field > 100")
layer.setOpacity(0.8)
```

### 2.4 Processing Output Mutations (Generally Not Allowed)

**Restrictions:**
- Processing outputs are immutable by design
- Cannot modify processing outputs directly
- Must create new layers for modifications

**Exceptions:**
```python
# Some processing outputs can be made editable
result = processing.run('native:buffer', {...})
output_layer = result['OUTPUT']

# If output is a file-based layer, can start editing
if isinstance(output_layer, str):
    layer = QgsVectorLayer(output_layer, 'Output', 'ogr')
    if layer.dataProvider().capabilities() & QgsVectorDataProvider.EditCapabilities:
        layer.startEditing()  # Now mutable
```

---

## 3. Edit Sessions Architecture

### 3.1 Edit Session Lifecycle

**Edit sessions** provide controlled access to mutable operations:

```python
# 1. Start edit session
layer.startEditing()

# 2. Make changes (stored in edit buffer)
layer.addFeature(feature)
layer.changeAttributeValue(fid, field, value)
layer.deleteFeature(fid)

# 3. End edit session (commit or rollback)
layer.commitChanges()  # Persist changes
# OR
layer.rollBack()  # Discard changes
```

**Lifecycle States:**
1. **Not Editing** - Layer is read-only
2. **Editing** - Edit buffer active, changes buffered
3. **Committed** - Changes persisted to data source
4. **Rolled Back** - Changes discarded, buffer cleared

### 3.2 Edit Buffer Architecture

**QgsVectorLayerEditBuffer** manages uncommitted changes:

```python
class QgsVectorLayerEditBuffer:
    """Manages uncommitted changes to vector layer."""
    
    # Added features (not yet in data source)
    _addedFeatures: Dict[int, QgsFeature]
    
    # Modified features (geometry/attributes changed)
    _changedGeometries: Dict[int, QgsGeometry]
    _changedAttributes: Dict[int, Dict[int, Any]]
    
    # Deleted features (marked for deletion)
    _deletedFeatureIds: Set[int]
    
    # Original feature states (for rollback)
    _originalFeatures: Dict[int, QgsFeature]
```

**Buffer Operations:**
- **Add Feature**: Stored in `_addedFeatures`
- **Modify Feature**: Changes stored in `_changedGeometries` / `_changedAttributes`
- **Delete Feature**: ID stored in `_deletedFeatureIds`
- **Original State**: Preserved in `_originalFeatures` for rollback

### 3.3 Edit Session Enforcement

**QGIS enforces edit sessions through:**

1. **State Checking:**
```python
def addFeature(self, feature):
    if not self.isEditable():
        return False  # Cannot add feature without edit session
    return self._editBuffer.addFeature(feature)
```

2. **Provider Capability Checking:**
```python
def startEditing(self):
    if not self.dataProvider().capabilities() & QgsVectorDataProvider.EditCapabilities:
        return False  # Provider doesn't support editing
    self._editBuffer = QgsVectorLayerEditBuffer(self)
    return True
```

3. **Data Source Writable Checking:**
```python
def startEditing(self):
    if not self.dataProvider().isEditable():
        return False  # Data source is read-only
    # ... create edit buffer
```

### 3.4 Edit Session Signals

**Signals emitted during edit sessions:**
- `editingStarted()` - Edit session started
- `beforeEditingStarted()` - Before edit session starts
- `featureAdded(fid, feature)` - Feature added to buffer
- `featureDeleted(fid)` - Feature marked for deletion
- `attributeValueChanged(fid, field, value)` - Attribute changed
- `geometryChanged(fid, geometry)` - Geometry changed
- `beforeCommitChanges()` - Before committing
- `afterCommitChanges()` - After committing
- `committedFeaturesAdded(layerId, features)` - Features committed
- `committedFeaturesRemoved(layerId, fids)` - Features removed
- `committedAttributeValuesChanges(layerId, changes)` - Attributes committed
- `committedGeometriesChanges(layerId, geometries)` - Geometries committed
- `editingStopped()` - Edit session ended
- `beforeRollBack()` - Before rollback
- `afterRollBack()` - After rollback

---

## 4. Commit/Rollback Semantics

### 4.1 Commit Semantics

**Commit** persists buffered changes to the data source:

```python
def commitChanges(self):
    """
    Commit changes to data provider.
    
    Process:
    1. Emit before-commit signal
    2. Validate changes
    3. Write changes to data provider
    4. Clear edit buffer
    5. Emit committed signals
    6. Emit after-commit signal
    """
    # 1. Before commit
    self.beforeCommitChanges.emit()
    
    # 2. Commit to provider
    success, errors = self._editBuffer.commitChanges()
    
    if not success:
        return False
    
    # 3. Clear buffer
    self._editBuffer = None
    
    # 4. After commit
    self.afterCommitChanges.emit()
    self.editingStopped.emit()
    
    return True
```

**Commit Process:**
1. **Validation** - Check if changes are valid
2. **Provider Write** - Write changes to data provider
3. **Transaction Commit** - Commit database transaction (if applicable)
4. **Buffer Clear** - Clear edit buffer
5. **Signal Emission** - Notify listeners of committed changes

**Commit Guarantees:**
- **Atomicity** - All changes committed or none
- **Durability** - Changes persisted to data source
- **Consistency** - Data integrity maintained
- **Isolation** - Changes visible after commit

### 4.2 Rollback Semantics

**Rollback** discards buffered changes without persisting:

```python
def rollBack(self, deleteBuffer=True):
    """
    Rollback changes, discarding edit buffer.
    
    Process:
    1. Emit before-rollback signal
    2. Discard edit buffer
    3. Restore original state
    4. Emit after-rollback signal
    """
    # 1. Before rollback
    self.beforeRollBack.emit()
    
    # 2. Discard buffer
    if deleteBuffer:
        self._editBuffer = None
    
    # 3. After rollback
    self.afterRollBack.emit()
    self.editingStopped.emit()
    
    return True
```

**Rollback Process:**
1. **Signal Emission** - Emit before-rollback signal
2. **Buffer Discard** - Discard edit buffer
3. **State Restoration** - Restore original feature states
4. **Signal Emission** - Emit after-rollback signal

**Rollback Guarantees:**
- **Complete Discard** - All buffered changes discarded
- **State Restoration** - Layer returns to pre-edit state
- **No Persistence** - No changes written to data source
- **Immediate Effect** - Changes disappear immediately

### 4.3 Transaction Boundaries

**File-Based Layers:**
- **No Transactions** - File operations are not transactional
- **Atomic Writes** - OS-level atomic file operations
- **No Rollback** - Cannot rollback file changes
- **Backup Required** - Users must manage backups

**Database Layers (PostGIS, SpatiaLite):**
- **Transactions Supported** - Database transactions available
- **Commit Required** - Changes committed to database
- **Rollback Supported** - Can rollback database transactions
- **ACID Guarantees** - Full ACID transaction support

```python
# Database layer with transaction support
layer = QgsVectorLayer(uri.uri(), "Layer", "postgres")

layer.startEditing()
layer.addFeature(feature)

# Commit creates database transaction
if layer.commitChanges():
    # Transaction committed
    pass
else:
    # Transaction rolled back automatically
    layer.rollBack()
```

### 4.4 Edit Buffer Groups

**Multiple layers can be grouped for coordinated commit/rollback:**

```python
# Create edit buffer group
edit_group = QgsVectorLayerEditBufferGroup()

# Add layers to group
edit_group.addLayer(layer1)
edit_group.addLayer(layer2)
edit_group.addLayer(layer3)

# Start editing on all layers
for layer in [layer1, layer2, layer3]:
    layer.startEditing()

# Make changes across layers
layer1.addFeature(feature1)
layer2.changeAttributeValue(fid, field, value)
layer3.deleteFeature(fid)

# Commit all layers atomically
if edit_group.commit():
    # All layers committed
    pass
else:
    # All layers rolled back
    edit_group.rollBack()
```

**Edit Buffer Group Guarantees:**
- **Atomic Operations** - All layers committed or none
- **Coordinated Rollback** - All layers rolled back together
- **Cross-Layer Consistency** - Maintains consistency across layers
- **Transaction Support** - Database transactions coordinated

---

## 5. Core Architecture Enforcement

### 5.1 Mutability Enforcement Mechanisms

**QGIS enforces mutability rules through:**

1. **State Checking:**
```python
# Check if layer is editable
if not layer.isEditable():
    return False  # Cannot modify without edit session

# Check provider capabilities
if not layer.dataProvider().capabilities() & QgsVectorDataProvider.EditCapabilities:
    return False  # Provider doesn't support editing
```

2. **Edit Buffer Isolation:**
```python
# Changes stored in buffer, not data source
def addFeature(self, feature):
    if not self.isEditable():
        return False
    # Store in buffer, not data source
    return self._editBuffer.addFeature(feature)
```

3. **Provider Abstraction:**
```python
# Provider enforces data source mutability
class QgsVectorDataProvider:
    def isEditable(self) -> bool:
        """Check if data source is writable."""
        pass
    
    def capabilities(self) -> int:
        """Return provider capabilities."""
        pass
```

### 5.2 Immutability Enforcement Mechanisms

**QGIS enforces immutability through:**

1. **Processing Framework:**
```python
# Processing creates new layers, never modifies inputs
def processAlgorithm(self, parameters, context, feedback):
    input_layer = self.parameterAsSource(parameters, 'INPUT', context)
    # Input is read-only
    
    # Create new output layer
    output_layer = self.parameterAsOutputLayer(parameters, 'OUTPUT', context)
    # Output is new, immutable layer
```

2. **Read-Only Providers:**
```python
# Some providers are read-only
virtual_layer = QgsVectorLayer('?query=...', 'Virtual', 'virtual')
# Provider doesn't support editing
if not virtual_layer.dataProvider().capabilities() & QgsVectorDataProvider.EditCapabilities:
    # Cannot edit virtual layer
    pass
```

3. **Temporary Layer Semantics:**
```python
# Memory layers are temporary
memory_layer = QgsVectorLayer('memory:', 'Temp', 'memory')
# Can be edited, but not persisted
# Removed when project closes
```

### 5.3 Edit Session Enforcement

**Edit sessions are enforced at multiple levels:**

1. **Layer Level:**
```python
class QgsVectorLayer:
    def addFeature(self, feature):
        if not self.isEditable():
            return False  # Enforced at layer level
        return self._editBuffer.addFeature(feature)
```

2. **Provider Level:**
```python
class QgsVectorDataProvider:
    def addFeatures(self, features):
        if not self.isEditable():
            return False  # Enforced at provider level
        # Write to data source
```

3. **Data Source Level:**
```python
# File permissions, database transactions, etc.
# Enforced at OS/database level
```

### 5.4 Commit/Rollback Enforcement

**Commit/rollback is enforced through:**

1. **Edit Buffer State:**
```python
# Changes only committed if buffer exists
def commitChanges(self):
    if not self._editBuffer:
        return False  # No changes to commit
    return self._editBuffer.commitChanges()
```

2. **Transaction Management:**
```python
# Database transactions managed by provider
class QgsVectorDataProvider:
    def commitChanges(self):
        # Begin transaction
        # Write changes
        # Commit or rollback
        pass
```

3. **Error Handling:**
```python
# Automatic rollback on error
try:
    success = layer.commitChanges()
    if not success:
        layer.rollBack()  # Automatic rollback
except Exception as e:
    layer.rollBack()  # Rollback on exception
```

---

## 6. Practical Examples

### 6.1 Mutable Layer Operations

```python
from qgis.core import QgsVectorLayer, QgsFeature, QgsGeometry, QgsPointXY

# Load layer
layer = QgsVectorLayer('/path/to/data.shp', 'Layer', 'ogr')

# Style changes (always allowed)
layer.setRenderer(renderer)
layer.setOpacity(0.8)

# Configuration changes (always allowed)
layer.setName("New Name")
layer.setCrs(new_crs)

# Feature changes (require edit session)
if layer.dataProvider().capabilities() & QgsVectorDataProvider.EditCapabilities:
    layer.startEditing()
    
    # Add feature
    feature = QgsFeature(layer.fields())
    feature.setGeometry(QgsGeometry.fromPointXY(QgsPointXY(0, 0)))
    feature.setAttributes([1, "Value"])
    layer.addFeature(feature)
    
    # Modify feature
    layer.changeAttributeValue(feature_id, 0, "New Value")
    
    # Delete feature
    layer.deleteFeature(feature_id)
    
    # Commit or rollback
    if layer.commitChanges():
        print("Changes committed")
    else:
        layer.rollBack()
        print("Changes rolled back")
```

### 6.2 Immutable Processing Outputs

```python
from qgis.core import QgsProcessing

# Processing creates new layers (inputs immutable)
result = processing.run('native:buffer', {
    'INPUT': input_layer,  # Input NOT modified
    'DISTANCE': 100,
    'OUTPUT': 'memory:'  # New immutable layer
})

output_layer = result['OUTPUT']  # New layer

# Output is immutable (cannot modify directly)
# Must create new layer for modifications
if isinstance(output_layer, str):
    layer = QgsVectorLayer(output_layer, 'Output', 'ogr')
    # Can make editable if file-based
    if layer.dataProvider().capabilities() & QgsVectorDataProvider.EditCapabilities:
        layer.startEditing()
        # Now can modify
```

### 6.3 Transactional Editing

```python
from qgis.core import QgsVectorLayerEditBufferGroup

# Create edit buffer group
edit_group = QgsVectorLayerEditBufferGroup()

# Add layers
layers = [layer1, layer2, layer3]
for layer in layers:
    layer.startEditing()
    edit_group.addLayer(layer)

# Make changes across layers
layer1.addFeature(feature1)
layer2.changeAttributeValue(fid, field, value)
layer3.deleteFeature(fid)

# Commit all layers atomically
if edit_group.commit():
    print("All layers committed")
else:
    edit_group.rollBack()
    print("All layers rolled back")
```

### 6.4 Style Mutations (No Restrictions)

```python
# Styles can be changed at any time
layer.setRenderer(renderer)
layer.setLabeling(label_settings)
layer.setOpacity(0.5)
layer.setBlendMode(QPainter.CompositionMode_Multiply)

# Changes are immediate, no commit needed
# Changes saved to project file when project is saved
```

---

## 7. Architectural Implications

### 7.1 Design Patterns

**QGIS mutability architecture employs:**

1. **Command Pattern** - Edit commands for undo/redo
2. **Memento Pattern** - Edit buffer stores original state
3. **State Pattern** - Edit session states (editing/not editing)
4. **Strategy Pattern** - Different providers have different capabilities
5. **Observer Pattern** - Signals notify of changes

### 7.2 Benefits

1. **Data Integrity** - Edit sessions prevent accidental modifications
2. **Transactional Guarantees** - Commit/rollback provides ACID-like properties
3. **Undo/Redo Support** - Edit buffer enables undo/redo
4. **Performance** - Buffered changes reduce I/O operations
5. **Consistency** - Immutable processing outputs prevent side effects

### 7.3 Trade-offs

1. **Complexity** - Edit session management adds complexity
2. **Performance** - Edit buffers consume memory
3. **Concurrency** - Only one edit session per layer
4. **Persistence** - File-based layers lack transaction support
5. **Immutability** - Processing outputs cannot be modified directly

---

## 8. Conclusion

QGIS's mutability architecture provides a robust foundation for geospatial data management:

- **Mutable objects** (layers, features, attributes, styles) can be modified through controlled edit sessions
- **Immutable objects** (processing outputs, temporary layers) are created once and never modified
- **Edit sessions** enforce when mutations are allowed, using edit buffers to manage uncommitted changes
- **Commit/rollback semantics** provide transactional guarantees, ensuring data integrity and enabling undo/redo

This architecture balances flexibility (allowing modifications where needed) with safety (preventing accidental changes), while providing the transactional guarantees necessary for reliable geospatial data management.

