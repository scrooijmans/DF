# FreeCAD Recompute Propagation and View Refresh

This document explains in detail how a recompute (triggered by property change) propagates through dependent objects, updates shape representations, and refreshes 3D/2D views, including the complete call stack from property update to view redraw.

## Overview

When a property changes in FreeCAD, the recomputation system propagates through the dependency chain, updating all dependent objects in the correct order. After recomputation, shape representations are updated and all views (3D, 2D, tree, property editor) are refreshed to reflect the changes.

---

## 1. Complete Call Stack: Property Change to View Redraw

### High-Level Flow

```
Property Change (box.Length = 20)
    ↓
Property Setter
    ↓
onChanged(Length)
    ↓
touch()
    ↓
propagateTouched() → touches all dependents
    ↓
Document.recompute()
    ↓
Topological Sort
    ↓
For each object in order:
    ↓
    execute() → computes Shape
    ↓
    signalRecomputed() emitted
    ↓
    ViewProvider.onRecomputed()
    ↓
    ViewProvider.updateData(Shape)
    ↓
    updateVisual() → updates Coin3D scene graph
    ↓
    Viewer.redraw() → refreshes 3D view
    ↓
    Tree View updates (if needed)
    ↓
    Property Editor updates (if needed)
```

---

## 2. Property Change: Initial Trigger

### Step 1: Property Assignment

**User Action**: `box.Length = 20`

**Code Path**:
```cpp
// C++ Property Setter
void PropertyContainer::setPropertyValue(const char* name, PyObject* value)
{
    Property* prop = getPropertyByName(name);
    prop->setPyObject(value);  // Store value
    
    // Trigger change notification
    onChanged(prop);  // ← Call stack continues
}
```

**What Happens**:
- Property value stored
- `onChanged()` called with property reference
- Signal emission prepared

### Step 2: onChanged() Method

**Location**: `App::DocumentObject::onChanged()`

**Code Path**:
```cpp
void DocumentObject::onChanged(const Property* prop)
{
    // Object-specific handling
    if (prop == &Length || prop == &Width || prop == &Height) {
        // Geometric property changed - needs recomputation
        touch();  // ← Call stack continues
    }
    
    // Emit signal for observers (ViewProviders, etc.)
    signalChanged(*prop);
}
```

**What Happens**:
- Object-specific logic responds to property change
- Determines if recomputation needed
- Calls `touch()` if recomputation required
- Emits `signalChanged()` for immediate view updates (non-geometric properties)

---

## 3. Touch and Dependency Propagation

### Step 3: touch() Method

**Location**: `App::DocumentObject::touch()`

**Code Path**:
```cpp
void DocumentObject::touch()
{
    // Set touched status bit
    StatusBits.set(Touch);
    setStatus(ObjectStatus::Touched, true);
    
    // Emit signal
    signalTouched();
    
    // Propagate to dependent objects
    propagateTouched();  // ← Call stack continues
    
    // If auto-recompute enabled, trigger recomputation
    if (getDocument()->getAutoRecompute()) {
        getDocument()->recompute();  // ← May trigger here
    }
}
```

**What Happens**:
- Object marked as "Touched" (needs recomputation)
- Status bit set
- Signal emitted for observers
- Dependency propagation triggered

### Step 4: Dependency Propagation (propagateTouched)

**Location**: `App::DocumentObject::propagateTouched()`

**Code Path**:
```cpp
void DocumentObject::propagateTouched()
{
    // Get all objects that depend on this one
    std::vector<DocumentObject*> dependents = getInList();
    
    for (DocumentObject* dependent : dependents) {
        if (!dependent->isTouched()) {
            dependent->touch();  // Recursive touch
            // This recursively propagates through entire chain
        }
    }
}
```

**Example: Dependency Chain Propagation**

```python
# Dependency chain: sketch -> pad -> chamfer
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
pad = doc.addObject("PartDesign::Pad", "Pad")
chamfer = doc.addObject("PartDesign::Chamfer", "Chamfer")

pad.Profile = sketch      # pad depends on sketch
chamfer.Base = pad        # chamfer depends on pad

# User changes sketch
sketch.addGeometry(...)   # Modifies sketch
sketch.touch()            # Sketch touched

# Propagation flow:
# 1. sketch.touch()
#    - sketch marked as touched
#    - propagateTouched() called
#    - pad.InList contains sketch → pad.touch() called
#
# 2. pad.touch()
#    - pad marked as touched
#    - propagateTouched() called
#    - chamfer.InList contains pad → chamfer.touch() called
#
# 3. chamfer.touch()
#    - chamfer marked as touched
#    - propagateTouched() called
#    - chamfer.InList is empty → no further propagation
#
# Result: sketch, pad, chamfer all marked as touched
```

**Key Points**:
- Propagation is recursive
- Entire dependency chain is marked
- Prevents partial updates
- Ensures consistency

---

## 4. Document Recomputation

### Step 5: Document.recompute() Entry Point

**Location**: `App::Document::recompute()`

**Code Path**:
```cpp
DocumentObjectExecReturn* Document::recompute()
{
    // 1. Collect all touched objects
    std::vector<DocumentObject*> touchedObjects;
    for (DocumentObject* obj : Objects) {
        if (obj->isTouched()) {
            touchedObjects.push_back(obj);
        }
    }
    
    if (touchedObjects.empty()) {
        return nullptr;  // Nothing to recompute
    }
    
    // 2. Topological sort (dependency order)
    std::vector<DocumentObject*> sortedObjects = topologicalSort(touchedObjects);
    
    // 3. Execute in order
    for (DocumentObject* obj : sortedObjects) {
        // Set recomputing status
        obj->setStatus(ObjectStatus::Recomputing, true);
        
        // Call execute()
        DocumentObjectExecReturn* result = obj->execute();  // ← Call stack continues
        
        // Handle result
        if (result && result->Status == DocumentObjectExecReturn::Error) {
            obj->setStatus(ObjectStatus::Erroneous, true);
        } else {
            obj->setStatus(ObjectStatus::Touched, false);
            obj->setStatus(ObjectStatus::Erroneous, false);
        }
        
        obj->setStatus(ObjectStatus::Recomputing, false);
        
        // Emit recomputed signal
        obj->signalRecomputed();  // ← Triggers view updates
    }
    
    // 4. Emit document recomputed signal
    signalRecomputed(*this);
    
    return nullptr;
}
```

**What Happens**:
- Collects all touched objects
- Sorts by dependencies (topological sort)
- Executes each object in order
- Handles errors
- Emits signals for view updates

### Step 6: Topological Sort

**Location**: `App::Document::topologicalSort()`

**Purpose**: Order objects so dependencies are computed before dependents.

**Algorithm**: Kahn's algorithm

```cpp
std::vector<DocumentObject*> Document::topologicalSort(
    const std::vector<DocumentObject*>& objects)
{
    std::vector<DocumentObject*> result;
    std::queue<DocumentObject*> queue;
    std::map<DocumentObject*, int> inDegree;
    
    // Calculate in-degrees (number of dependencies)
    for (DocumentObject* obj : objects) {
        inDegree[obj] = obj->getOutList().size();
        if (inDegree[obj] == 0) {
            queue.push(obj);  // No dependencies - ready to execute
        }
    }
    
    // Process in topological order
    while (!queue.empty()) {
        DocumentObject* obj = queue.front();
        queue.pop();
        result.push_back(obj);
        
        // Reduce in-degree of dependents
        for (DocumentObject* dependent : obj->getInList()) {
            inDegree[dependent]--;
            if (inDegree[dependent] == 0) {
                queue.push(dependent);  // All dependencies computed
            }
        }
    }
    
    return result;
}
```

**Example**:
```python
# Dependency chain: sketch -> pad -> chamfer
# Topological sort: [sketch, pad, chamfer]

# Execution order:
# 1. sketch.execute() - no dependencies
# 2. pad.execute() - sketch already computed
# 3. chamfer.execute() - pad already computed
```

---

## 5. execute() Method: Shape Computation

### Step 7: execute() Method Execution

**Location**: `App::DocumentObject::execute()` (virtual, overridden by subclasses)

**Code Path**:
```cpp
// Example: Part::Box::execute()
App::DocumentObjectExecReturn* Part::Box::execute()
{
    // 1. Read input properties
    double length = Length.getValue();
    double width = Width.getValue();
    double height = Height.getValue();
    Base::Placement placement = Placement.getValue();
    
    // 2. Read dependencies (if any)
    // (For Box, no dependencies - it's a primitive)
    
    // 3. Compute output (Shape)
    TopoDS_Shape boxShape = BRepPrimAPI_MakeBox(length, width, height);
    boxShape = placement.transform(boxShape);
    
    // 4. Update output property
    Shape.setValue(boxShape);  // ← Shape updated
    
    // 5. Clear touched status
    StatusBits.reset(Touch);
    setStatus(ObjectStatus::Touched, false);
    
    // 6. Emit signal (triggers view updates)
    signalChanged(Shape);  // ← Signal emitted
    
    return nullptr;  // Success
}
```

**For Objects with Dependencies**:
```cpp
// Example: Part::Fuse::execute()
App::DocumentObjectExecReturn* Part::Fuse::execute()
{
    // 1. Validate dependencies
    if (!Base || !Tool) {
        return new DocumentObjectExecReturn(
            DocumentObjectExecReturn::Error,
            "Base and Tool must be set"
        );
    }
    
    // 2. Read dependency shapes (already computed)
    TopoDS_Shape baseShape = Base->Shape.getValue();
    TopoDS_Shape toolShape = Tool->Shape.getValue();
    
    // 3. Validate shapes
    if (baseShape.IsNull() || toolShape.IsNull()) {
        return new DocumentObjectExecReturn(
            DocumentObjectExecReturn::Error,
            "Base or Tool has null Shape"
        );
    }
    
    // 4. Compute output (fuse operation)
    TopoDS_Shape fusedShape;
    try {
        fusedShape = BRepAlAPI_Fuse(baseShape, toolShape);
    } catch (const Standard_Failure& e) {
        return new DocumentObjectExecReturn(
            DocumentObjectExecReturn::Error,
            e.GetMessageString()
        );
    }
    
    // 5. Update output property
    Shape.setValue(fusedShape);  // ← Shape updated
    
    // 6. Clear touched status
    StatusBits.reset(Touch);
    
    // 7. Emit signal
    signalChanged(Shape);  // ← Signal emitted
    
    return nullptr;  // Success
}
```

**What Happens**:
- Input properties read
- Dependencies read (already computed in topological order)
- Shape computed from inputs and dependencies
- Shape property updated
- Status cleared
- Signal emitted for view updates

---

## 6. Signal Emission: Triggering View Updates

### Step 8: signalRecomputed() and signalChanged()

**After execute() completes**:

```cpp
// In execute()
Shape.setValue(newShape);  // Shape updated
signalChanged(Shape);      // Signal emitted

// After execute() returns
obj->signalRecomputed();   // Signal emitted
```

**Signal Observers**:
- ViewProvider (observes object signals)
- Tree View Observer (observes document signals)
- Property Editor Observer (observes object signals)

---

## 7. ViewProvider: Shape Representation Update

### Step 9: ViewProvider.onRecomputed()

**Location**: `Gui::ViewProvider::onRecomputed()`

**Code Path**:
```cpp
void ViewProvider::onRecomputed()
{
    // Object was recomputed - update visual representation
    updateData(&pcObject->Shape);  // ← Call stack continues
}
```

**What Happens**:
- Called after `execute()` completes
- Triggers visual update
- Calls `updateData()` with Shape property

### Step 10: ViewProvider.updateData()

**Location**: `Gui::ViewProvider::updateData()`

**Code Path**:
```cpp
void ViewProvider::updateData(const App::Property* prop)
{
    if (prop == &pcObject->Shape) {
        // Shape changed - update 3D representation
        updateVisual();  // ← Call stack continues
    } else if (prop == &pcObject->Placement) {
        // Placement changed - update position
        updatePlacement();
    } else if (prop == &pcObject->Visibility) {
        // Visibility changed
        if (pcObject->Visibility.getValue()) {
            show();
        } else {
            hide();
        }
    }
    
    // Emit signal for other observers
    signalUpdateData(prop);
}
```

**What Happens**:
- Receives property that changed
- Determines if visual update needed
- Calls appropriate update method
- Emits signal for other observers

### Step 11: ViewProvider.updateVisual()

**Location**: `Gui::ViewProvider::updateVisual()`

**Code Path**:
```cpp
void ViewProvider::updateVisual()
{
    // Get updated shape from object
    TopoDS_Shape shape = pcObject->Shape.getValue();
    
    if (shape.IsNull()) {
        // Shape is null - hide object
        hide();
        return;
    }
    
    // Convert OpenCASCADE shape to Coin3D representation
    SoShape* shapeNode = getShapeNode();
    
    // Update Coin3D shape node
    shapeNode->shape.setValue(convertToCoin3D(shape));
    
    // Update placement node
    SoTransform* transform = getTransformNode();
    Base::Placement placement = pcObject->Placement.getValue();
    transform->translation.setValue(
        placement.getPosition().x,
        placement.getPosition().y,
        placement.getPosition().z
    );
    
    // Update rotation
    Base::Rotation rotation = placement.getRotation();
    // ... set rotation in transform node
    
    // Touch Coin3D scene graph (trigger update)
    SoSeparator* root = getRootNode();
    root->touch();  // Coin3D touch - marks scene graph as changed
    
    // Request redraw
    getViewer()->redraw();  // ← Call stack continues
}
```

**What Happens**:
- Gets updated Shape from object
- Converts OpenCASCADE shape to Coin3D representation
- Updates Coin3D scene graph nodes
- Touches scene graph (marks as changed)
- Requests viewer redraw

---

## 8. View Redraw: 3D/2D View Refresh

### Step 12: Viewer.redraw()

**Location**: `Gui::View3DInventor::redraw()`

**Code Path**:
```cpp
void View3DInventor::redraw()
{
    // Coin3D automatically redraws when scene graph is touched
    // But we can force immediate redraw
    
    // Get Coin3D viewer
    SoQtExaminerViewer* viewer = getViewer();
    
    // Schedule redraw
    viewer->scheduleRedraw();
    
    // Or force immediate redraw
    viewer->render();
}
```

**What Happens**:
- Coin3D detects scene graph changes
- Schedules redraw
- Renders updated scene
- 3D view displays updated geometry

### Step 13: Multiple Views Update

**All 3D views** using the same scene graph update:

```cpp
// Coin3D scene graph is shared
// When scene graph is touched, all views update automatically

// View 1: Front view
// View 2: Top view
// View 3: Isometric view
// All update when scene graph changes
```

---

## 9. Complete Example: Dependency Chain Recompute

### Scenario: Complex Dependency Chain

```python
# Create objects with dependencies
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
pad = doc.addObject("PartDesign::Pad", "Pad")
chamfer = doc.addObject("PartDesign::Chamfer", "Chamfer")

pad.Profile = sketch      # pad depends on sketch
chamfer.Base = pad        # chamfer depends on pad

# Initial state: all objects computed
doc.recompute()
```

### User Changes Sketch

```python
# User modifies sketch geometry
sketch.addGeometry(Part.LineSegment(...))
```

### Complete Call Stack

```
1. sketch.addGeometry() modifies sketch
    ↓
2. sketch.touch()
    - sketch.StatusBits.set(Touch)
    - sketch.propagateTouched()
      - pad.touch() (pad.InList contains sketch)
        - pad.StatusBits.set(Touch)
        - pad.propagateTouched()
          - chamfer.touch() (chamfer.InList contains pad)
            - chamfer.StatusBits.set(Touch)
            - chamfer.propagateTouched() (no dependents)
    ↓
3. doc.recompute() [if auto-recompute enabled]
    ↓
4. Document::recompute()
    - Collect touched: [sketch, pad, chamfer]
    - Topological sort: [sketch, pad, chamfer]
    ↓
5. sketch.execute()
    - Read: sketch geometry
    - Compute: sketch.Shape
    - Update: sketch.Shape = newShape
    - Emit: sketch.signalChanged(Shape)
    - Emit: sketch.signalRecomputed()
    ↓
6. ViewProvider::onRecomputed() for sketch
    - ViewProvider.updateData(&sketch.Shape)
    - ViewProvider.updateVisual()
    - Update Coin3D scene graph
    - Viewer.redraw()
    ↓
7. pad.execute()
    - Read: pad.Profile (sketch)
    - Read: sketch.Shape (already computed)
    - Compute: pad.Shape from sketch.Shape
    - Update: pad.Shape = newShape
    - Emit: pad.signalChanged(Shape)
    - Emit: pad.signalRecomputed()
    ↓
8. ViewProvider::onRecomputed() for pad
    - ViewProvider.updateData(&pad.Shape)
    - ViewProvider.updateVisual()
    - Update Coin3D scene graph
    - Viewer.redraw()
    ↓
9. chamfer.execute()
    - Read: chamfer.Base (pad)
    - Read: pad.Shape (already computed)
    - Compute: chamfer.Shape from pad.Shape
    - Update: chamfer.Shape = newShape
    - Emit: chamfer.signalChanged(Shape)
    - Emit: chamfer.signalRecomputed()
    ↓
10. ViewProvider::onRecomputed() for chamfer
    - ViewProvider.updateData(&chamfer.Shape)
    - ViewProvider.updateVisual()
    - Update Coin3D scene graph
    - Viewer.redraw()
    ↓
11. All views updated:
    - 3D views show updated geometry
    - Tree view shows updated status
    - Property editor shows updated values
```

---

## 10. Shape Representation Update Details

### OpenCASCADE to Coin3D Conversion

**Shape Conversion Process**:

```cpp
// ViewProvider converts OpenCASCADE shape to Coin3D
SoShape* ViewProvider::convertToCoin3D(const TopoDS_Shape& shape)
{
    // Create Coin3D shape node
    SoShape* shapeNode = new SoShape();
    
    // Convert OpenCASCADE shape to Coin3D mesh
    // (OpenCASCADE provides tessellation)
    BRepMesh_IncrementalMesh mesh(shape, 0.1);  // Mesh with tolerance
    
    // Extract mesh data
    // ... convert to Coin3D format ...
    
    // Set in shape node
    shapeNode->shape.setValue(coinShape);
    
    return shapeNode;
}
```

**What Happens**:
- OpenCASCADE shape is tessellated (converted to mesh)
- Mesh data extracted
- Converted to Coin3D format
- Set in Coin3D scene graph node

### Scene Graph Update

**Coin3D Scene Graph Structure**:

```
SoSeparator (root)
├── SoTransform (placement)
│   ├── translation
│   └── rotation
└── SoShape (geometry)
    └── mesh data
```

**Update Process**:
1. Shape node updated with new mesh
2. Transform node updated with new placement
3. Scene graph touched (marks as changed)
4. Coin3D automatically detects change
5. Viewer redraws

---

## 11. View Refresh: Multiple View Types

### 3D View Refresh

**Process**:
```
Shape updated in execute()
    ↓
ViewProvider.updateData(Shape)
    ↓
ViewProvider.updateVisual()
    ↓
Coin3D scene graph updated
    ↓
Viewer.redraw()
    ↓
3D view displays updated geometry
```

### Tree View Refresh

**Process**:
```
Object recomputed
    ↓
signalRecomputed() emitted
    ↓
Tree View Observer receives signal
    ↓
Tree view item updated (status icon, etc.)
```

### Property Editor Refresh

**Process**:
```
Property changed
    ↓
signalChanged(property) emitted
    ↓
Property Editor Observer receives signal
    ↓
Property editor display updated
```

### 2D View Refresh (Sketcher)

**Process**:
```
Sketch recomputed
    ↓
sketch.Shape updated
    ↓
ViewProvider.updateData(Shape)
    ↓
2D view (Sketcher view) updated
    ↓
Constraints, geometry redrawn
```

---

## 12. Batch Updates and Performance

### Batch Property Changes

**Efficient Pattern**:
```python
# ✅ Good: Batch changes, single recompute
box.Length = 20
box.Width = 15
box.Height = 10
doc.recompute()  # Single recomputation

# Each property change:
# - touch() called
# - propagateTouched() called
# - But recomputation happens once
```

**Inefficient Pattern**:
```python
# ❌ Bad: Recompute after each change
box.Length = 20
doc.recompute()  # Recomputation 1
box.Width = 15
doc.recompute()  # Recomputation 2
box.Height = 10
doc.recompute()  # Recomputation 3
```

### View Update Batching

**Coin3D automatically batches** scene graph updates:
- Multiple scene graph changes
- Single redraw at end
- Efficient rendering

---

## 13. Error Handling in Recompute Chain

### Error During execute()

**If execute() fails**:

```cpp
App::DocumentObjectExecReturn* MyFeature::execute()
{
    // Error occurs
    if (error) {
        // Set error status
        setStatus(ObjectStatus::Erroneous, true);
        setStatus(ObjectStatus::Touched, true);  // Still needs recomputation
        
        // Emit error signal
        signalRecomputed();  // Still emitted (with error state)
        
        return new DocumentObjectExecReturn(
            DocumentObjectExecReturn::Error,
            "Error message"
        );
    }
    
    // Success
    return nullptr;
}
```

**Error Propagation**:
```
Object A.execute() fails
    ↓
Object A marked as Erroneous
    ↓
Object B.execute() checks A
    ↓
Object B detects A is erroneous
    ↓
Object B also marked as erroneous
    ↓
(Error propagates through dependency chain)
```

### View Update on Error

**If object has error**:
- ViewProvider may show error indicator
- 3D view may not update (if Shape is invalid)
- Tree view shows error icon
- Property editor may show error message

---

## 14. Complete Call Stack Summary

### Full Call Stack Trace

```
Property Change: box.Length = 20
    ↓
1. PropertyContainer::setPropertyValue("Length", 20)
    ↓
2. Property::setPyObject(20)
    ↓
3. DocumentObject::onChanged(&Length)
    ↓
4. DocumentObject::touch()
    ↓
5. DocumentObject::propagateTouched()
    - For each obj in InList:
      - obj.touch() (recursive)
    ↓
6. Document::recompute() [if auto-recompute]
    ↓
7. Document::collectTouchedObjects()
    ↓
8. Document::topologicalSort(touchedObjects)
    ↓
9. For each obj in sorted order:
    ↓
    a. obj.setStatus(Recomputing, true)
    ↓
    b. obj.execute()
        - Read input properties
        - Read dependencies
        - Compute Shape
        - Shape.setValue(newShape)
        - signalChanged(Shape) emitted
    ↓
    c. obj.setStatus(Touched, false)
    ↓
    d. obj.signalRecomputed() emitted
    ↓
    e. ViewProvider::onRecomputed()
        - ViewProvider::updateData(&Shape)
        - ViewProvider::updateVisual()
            - Get Shape from object
            - Convert to Coin3D
            - Update scene graph
            - Scene graph touched
    ↓
    f. Viewer::redraw()
        - Coin3D detects scene graph change
        - Renders updated scene
    ↓
    g. Tree View Observer::slotRecomputed()
        - Update tree view item
    ↓
    h. Property Editor Observer::slotChangedObject()
        - Update property display
```

---

## 15. Summary

### Recompute Propagation

1. **Property Change**: Triggers `onChanged()` → `touch()`
2. **Dependency Propagation**: `propagateTouched()` marks all dependents
3. **Topological Sort**: Objects ordered by dependencies
4. **Execute in Order**: Each object's `execute()` called
5. **Shape Update**: Shape computed and updated in each object

### View Refresh

1. **Signal Emission**: `signalRecomputed()` and `signalChanged()` emitted
2. **ViewProvider Update**: `updateData()` and `updateVisual()` called
3. **Scene Graph Update**: Coin3D scene graph updated
4. **Viewer Redraw**: 3D view refreshed
5. **Other Views**: Tree view, property editor updated

### Key Principles

1. **Dependency Order**: Dependencies computed before dependents
2. **Automatic Propagation**: Changes propagate through dependency chain
3. **Signal-Based Updates**: Views update via signals
4. **Batch Updates**: Multiple changes batched for efficiency
5. **Error Handling**: Errors propagate and are communicated

---

## References

- Recomputation: `Document::recompute()`, `DocumentObject::execute()`
- Dependency Propagation: `DocumentObject::touch()`, `DocumentObject::propagateTouched()`
- Topological Sort: `Document::topologicalSort()`
- View Updates: `ViewProvider::updateData()`, `ViewProvider::updateVisual()`
- View Refresh: `Viewer::redraw()`, Coin3D scene graph

