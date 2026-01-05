# FreeCAD Recomputation Call Stack: From Property Change to View Refresh

This document explains in detail how recomputation is triggered in FreeCAD, including the complete call stack from property updates through `execute()` method execution to view refresh.

## Overview

FreeCAD's recomputation system is a sophisticated pipeline that ensures all dependent objects are updated when properties change. Understanding this call stack is essential for debugging, performance optimization, and developing custom FreeCAD objects.

---

## 1. Recomputation Triggers

Recomputation can be triggered in two primary ways:

### A. Property Change (Automatic Trigger)

**When**: A property of a `DocumentObject` is modified.

**How**: The property setter automatically initiates the recomputation pipeline.

**Example**:
```python
box = doc.addObject("Part::Box", "Box")
box.Length = 20  # Property change triggers recomputation pipeline
```

### B. Manual Recompute Command

**When**: User explicitly triggers recomputation (F5 key, toolbar button, or `doc.recompute()`).

**How**: Direct call to document's recompute method.

**Example**:
```python
# Manual trigger
doc.recompute()  # Explicit recomputation command

# Or via GUI:
# Tools → Recompute (F5)
```

---

## 2. Complete Call Stack

### High-Level Flow

```
Property Change / Manual Recompute
    ↓
Property Setter / Document.recompute()
    ↓
onChanged() Method
    ↓
touch() Method
    ↓
Dependency Propagation (InList traversal)
    ↓
Document.recompute() [if automatic]
    ↓
Topological Sort
    ↓
execute() Method (for each object)
    ↓
ViewProvider.updateData()
    ↓
View Refresh (3D scene update)
```

---

## 3. Detailed Call Stack: Property Change Path

### Step 1: Property Assignment

**Location**: Property setter (C++ or Python binding)

**What Happens**:
- Property value is stored
- Property change signal is emitted
- `onChanged()` method is invoked

**Code Path** (Conceptual):
```cpp
// C++ Property Setter (simplified)
void PropertyContainer::setPropertyValue(const char* propName, const PyObject* value)
{
    Property* prop = getPropertyByName(propName);
    prop->setPyObject(value);  // Store new value
    
    // Trigger change notification
    onChanged(prop);  // ← Call stack continues here
}
```

**Python Equivalent**:
```python
# When you write: box.Length = 20
# FreeCAD internally does:
box.setPropertyValue("Length", 20)  # Triggers onChanged()
```

### Step 2: onChanged() Method

**Location**: `App::DocumentObject::onChanged()`

**Purpose**: Respond to property changes, determine if recomputation is needed.

**What Happens**:
- Receives the changed property as parameter
- Object-specific logic can respond to specific property changes
- Calls `touch()` if recomputation is needed
- May trigger additional side effects

**Code Path**:
```cpp
// C++ Implementation (simplified)
void DocumentObject::onChanged(const Property* prop)
{
    // Object-specific handling
    if (prop == &Length || prop == &Width || prop == &Height) {
        // Geometric property changed - needs recomputation
        touch();  // ← Call stack continues here
    }
    
    // Emit signal for observers
    signalChanged(*prop);
}
```

**Python Custom Objects**:
```python
class MyCustomObject(App.DocumentObject):
    def onChanged(self, obj, prop):
        # Respond to specific property changes
        if prop == "Length":
            # Custom logic when Length changes
            pass
        
        # Call parent to handle standard behavior
        super().onChanged(obj, prop)  # Calls touch() if needed
```

**Key Points**:
- `onChanged()` is called **synchronously** when property changes
- It can inspect which property changed
- It determines if the change requires recomputation
- It calls `touch()` to mark object for recomputation

### Step 3: touch() Method

**Location**: `App::DocumentObject::touch()`

**Purpose**: Mark object as needing recomputation.

**What Happens**:
- Sets internal "Touched" status bit
- Marks object as invalid/stale
- Triggers dependency propagation (touches dependent objects)
- May emit signals for observers

**Code Path**:
```cpp
// C++ Implementation (simplified)
void DocumentObject::touch()
{
    // Set touched status bit
    StatusBits.set(Touch);
    
    // Mark as needing recomputation
    setStatus(ObjectStatus::Touched, true);
    
    // Propagate to dependent objects
    propagateTouched();  // ← Touches all objects in InList
    
    // Emit signal
    signalTouched();
    
    // If auto-recompute is enabled, trigger recomputation
    if (getDocument()->getAutoRecompute()) {
        getDocument()->recompute();  // ← May trigger here
    }
}
```

**Python API**:
```python
# Manual touch
box.touch()  # Marks box and dependents for recomputation

# Automatic touch (happens in onChanged())
box.Length = 20  # Automatically calls touch()
```

**Key Points**:
- `touch()` sets the "Touched" status bit
- It propagates to dependent objects (via `InList`)
- It may trigger automatic recomputation if enabled
- Object remains "Touched" until `execute()` completes

### Step 4: Dependency Propagation

**Location**: `App::DocumentObject::propagateTouched()`

**Purpose**: Mark all dependent objects as needing recomputation.

**What Happens**:
- Traverses `InList` (objects that depend on this object)
- Recursively calls `touch()` on each dependent
- Ensures entire dependency chain is marked

**Code Path**:
```cpp
// C++ Implementation (simplified)
void DocumentObject::propagateTouched()
{
    // Get all objects that depend on this one
    std::vector<DocumentObject*> dependents = getInList();
    
    for (DocumentObject* dependent : dependents) {
        if (!dependent->isTouched()) {
            dependent->touch();  // Recursive touch
        }
    }
}
```

**Example**:
```python
# Dependency chain: sketch -> pad -> chamfer
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
pad = doc.addObject("PartDesign::Pad", "Pad")
chamfer = doc.addObject("PartDesign::Chamfer", "Chamfer")

pad.Profile = sketch      # pad depends on sketch
chamfer.Base = pad        # chamfer depends on pad

# When sketch is touched:
sketch.touch()
# Propagates to:
#   - pad.touch() (because pad.InList contains sketch)
#   - chamfer.touch() (because chamfer depends on pad)
```

**Key Points**:
- Dependency propagation ensures consistency
- All dependent objects are marked before recomputation
- Prevents partial updates (some objects updated, others not)

### Step 5: Document.recompute() Entry Point

**Location**: `App::Document::recompute()`

**Purpose**: Orchestrate recomputation of all touched objects.

**What Happens**:
- Collects all touched objects
- Performs topological sort
- Executes objects in dependency order
- Updates views after execution

**Code Path**:
```cpp
// C++ Implementation (simplified)
void Document::recompute()
{
    // Collect all touched objects
    std::vector<DocumentObject*> touchedObjects;
    for (DocumentObject* obj : Objects) {
        if (obj->isTouched()) {
            touchedObjects.push_back(obj);
        }
    }
    
    // Topological sort (dependency order)
    std::vector<DocumentObject*> sortedObjects = topologicalSort(touchedObjects);
    
    // Execute in order
    for (DocumentObject* obj : sortedObjects) {
        executeObject(obj);  // ← Call stack continues here
    }
    
    // Update views
    updateViews();
}
```

**Python API**:
```python
# Manual recomputation
doc.recompute()  # Recomputes all touched objects

# Recomputation with specific object
box.recomputeFeature()  # Recomputes single object and dependents
```

**Key Points**:
- `Document.recompute()` is the main entry point
- It handles topological sorting
- It executes objects in correct order
- It coordinates view updates

### Step 6: Topological Sort

**Location**: `App::Document::topologicalSort()`

**Purpose**: Order objects by dependencies (dependencies before dependents).

**What Happens**:
- Builds dependency graph from `OutList` relationships
- Performs topological sort (Kahn's algorithm or DFS-based)
- Returns objects in execution order

**Algorithm** (Conceptual):
```cpp
// Topological Sort (Kahn's algorithm)
std::vector<DocumentObject*> Document::topologicalSort(
    const std::vector<DocumentObject*>& objects)
{
    std::vector<DocumentObject*> result;
    std::queue<DocumentObject*> queue;
    std::map<DocumentObject*, int> inDegree;
    
    // Calculate in-degrees
    for (DocumentObject* obj : objects) {
        inDegree[obj] = obj->getOutList().size();
        if (inDegree[obj] == 0) {
            queue.push(obj);  // No dependencies
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
                queue.push(dependent);
            }
        }
    }
    
    return result;
}
```

**Example**:
```python
# Dependency chain: sketch -> pad -> chamfer
# Topological sort result: [sketch, pad, chamfer]

sorted_objects = doc.topologicalSort()
# Executes in order: sketch first, then pad, then chamfer
```

**Key Points**:
- Ensures dependencies are computed before dependents
- Prevents executing objects before their inputs are ready
- Handles complex dependency graphs

### Step 7: execute() Method Execution

**Location**: `App::DocumentObject::execute()` (virtual method, overridden by subclasses)

**Purpose**: Recompute object's output properties from input properties.

**What Happens**:
- Object reads its input properties
- Object reads dependency objects' computed properties
- Object computes its output properties (e.g., `Shape`)
- Object updates its state

**Code Path**:
```cpp
// C++ Implementation (simplified, for Part::Box)
App::DocumentObjectExecReturn* Part::Box::execute()
{
    // Read input properties
    double length = Length.getValue();
    double width = Width.getValue();
    double height = Height.getValue();
    Base::Placement placement = Placement.getValue();
    
    // Compute output (Shape)
    TopoDS_Shape boxShape = BRepPrimAPI_MakeBox(length, width, height);
    boxShape = placement.transform(boxShape);
    
    // Update output property
    Shape.setValue(boxShape);  // ← Computed result
    
    // Clear touched status
    StatusBits.reset(Touch);
    
    return nullptr;  // Success
}
```

**Python Custom Objects**:
```python
class MyCustomObject(App.DocumentObject):
    def execute(self, obj):
        # Read input properties
        length = obj.Length
        width = obj.Width
        
        # Read dependencies (already computed)
        if obj.Base:
            base_shape = obj.Base.Shape  # Dependency was computed first
        
        # Compute output
        import Part
        obj.Shape = Part.makeBox(length, width, 10)
        
        # Output properties are now updated
        return None  # Success
```

**Execution Order Example**:
```python
# Objects: sketch, pad, chamfer
# Dependency: sketch -> pad -> chamfer

doc.recompute()
# 1. execute() on sketch:
#    - Reads: sketch geometry
#    - Computes: sketch.Shape
#    - Status: sketch no longer touched

# 2. execute() on pad:
#    - Reads: pad.Profile (sketch)
#    - Reads: sketch.Shape (already computed)
#    - Computes: pad.Shape from sketch.Shape
#    - Status: pad no longer touched

# 3. execute() on chamfer:
#    - Reads: chamfer.Base (pad)
#    - Reads: pad.Shape (already computed)
#    - Computes: chamfer.Shape from pad.Shape
#    - Status: chamfer no longer touched
```

**Key Points**:
- `execute()` is called in topological order
- Dependencies are already computed when `execute()` runs
- Output properties (like `Shape`) are computed here
- Object is marked as valid after `execute()` completes

### Step 8: ViewProvider.updateData()

**Location**: `Gui::ViewProvider::updateData()`

**Purpose**: Update visual representation after object recomputation.

**What Happens**:
- Receives notification that object data changed
- Updates 3D representation (mesh, display properties)
- Refreshes visual state

**Code Path**:
```cpp
// C++ Implementation (simplified)
void ViewProvider::updateData(const App::Property* prop)
{
    if (prop == &pcObject->Shape) {
        // Shape changed - update 3D representation
        updateVisual();
    } else if (prop == &pcObject->Visibility) {
        // Visibility changed - show/hide
        show();
    }
    
    // Emit signal for view update
    signalUpdateData(prop);
}
```

**Python ViewProviders**:
```python
class MyViewProvider:
    def updateData(self, obj, prop):
        # Called when object property changes
        if prop == "Shape":
            # Update 3D representation
            self.updateVisual(obj.Shape)
        elif prop == "Visibility":
            # Update visibility
            self.show() if obj.Visibility else self.hide()
```

**When Called**:
- After `execute()` completes successfully
- When specific properties change (Shape, Placement, etc.)
- During view refresh operations

**Key Points**:
- `updateData()` is called after recomputation
- It updates the visual representation
- It's separate from geometric computation
- Multiple `updateData()` calls may occur for different properties

### Step 9: View Refresh (3D Scene Update)

**Location**: Coin3D scene graph update (FreeCAD's 3D rendering)

**Purpose**: Refresh the 3D view to show updated geometry.

**What Happens**:
- Coin3D scene graph is updated with new geometry
- OpenGL rendering pipeline processes changes
- View is redrawn with updated objects

**Code Path** (Conceptual):
```cpp
// View update (simplified)
void ViewProvider::updateVisual()
{
    // Update Coin3D scene graph
    SoSeparator* root = getRootNode();
    
    // Update shape node
    SoShape* shapeNode = getShapeNode();
    shapeNode->shape.setValue(convertToCoin3D(pcObject->Shape));
    
    // Trigger scene graph update
    root->touch();  // Coin3D touch
    
    // Request redraw
    getViewer()->redraw();
}
```

**Python View Update**:
```python
class MyViewProvider:
    def updateVisual(self, shape):
        # Update Coin3D representation
        self.ShapeNode.Shape = shape
        
        # Trigger view refresh
        self.ViewObject.update()
```

**Key Points**:
- View refresh happens after `updateData()`
- Uses Coin3D for 3D rendering
- May be batched for performance
- Separate from geometric computation

---

## 4. Complete Call Stack Example

### Scenario: Changing Box Length

```python
import FreeCAD as App

doc = App.newDocument()
box = doc.addObject("Part::Box", "Box")
box.Length = 10
doc.recompute()  # Initial computation

# User changes Length property:
box.Length = 20  # ← START OF CALL STACK
```

### Call Stack Trace

```
1. Property Assignment
   └─ box.Length = 20
      └─ PropertyContainer::setPropertyValue("Length", 20)
         └─ Property::setPyObject(20)
            └─ Property value stored
               └─ onChanged(prop)  ← Step 2

2. onChanged() Method
   └─ DocumentObject::onChanged(&Length)
      └─ Detects Length property changed
         └─ touch()  ← Step 3

3. touch() Method
   └─ DocumentObject::touch()
      └─ StatusBits.set(Touch)  // Mark as touched
         └─ propagateTouched()  ← Step 4

4. Dependency Propagation
   └─ DocumentObject::propagateTouched()
      └─ For each obj in InList:
         └─ obj.touch()  // Recursive touch
            └─ (If fusion depends on box, fusion.touch() is called)
               └─ Document::recompute()  ← Step 5 (if auto-recompute enabled)

5. Document.recompute()
   └─ Document::recompute()
      └─ Collect touched objects: [box, fusion]
         └─ topologicalSort([box, fusion])  ← Step 6
            └─ Returns: [box, fusion]  // box before fusion
               └─ For each obj in sorted order:
                  └─ executeObject(obj)  ← Step 7

6. Topological Sort
   └─ Document::topologicalSort(objects)
      └─ Build dependency graph
         └─ Sort by dependencies
            └─ Return: [box, fusion]

7. execute() Method (for box)
   └─ Part::Box::execute()
      └─ Read: Length=20, Width=10, Height=10
         └─ Compute: Shape = makeBox(20, 10, 10)
            └─ Shape.setValue(newShape)
               └─ StatusBits.reset(Touch)  // Mark as valid
                  └─ ViewProvider::updateData(&Shape)  ← Step 8

8. ViewProvider.updateData()
   └─ ViewProvider::updateData(&Shape)
      └─ Update Coin3D scene graph
         └─ updateVisual()  ← Step 9

9. View Refresh
   └─ ViewProvider::updateVisual()
      └─ Update Coin3D Shape node
         └─ getViewer()->redraw()
            └─ OpenGL rendering
               └─ 3D view updated

10. execute() Method (for fusion, if touched)
    └─ Part::Fuse::execute()
       └─ Read: Base=box, Tool=cylinder
          └─ Read: box.Shape (already computed)
             └─ Compute: Shape = fuse(box.Shape, cylinder.Shape)
                └─ Shape.setValue(newShape)
                   └─ ViewProvider::updateData(&Shape)
                      └─ View refresh
```

---

## 5. Manual Recompute Command Path

### Scenario: User Presses F5 (Recompute)

```python
# User action: Press F5 or click "Recompute" button
# Or script: doc.recompute()
```

### Call Stack Trace

```
1. User Action / Script Call
   └─ doc.recompute()  ← Direct entry point
      └─ Document::recompute()
         └─ Collect all touched objects
            └─ (No property change, just manual trigger)
               └─ topologicalSort(touchedObjects)  ← Step 2

2. Topological Sort
   └─ Document::topologicalSort(objects)
      └─ Sort by dependencies
         └─ Return sorted list
            └─ For each obj: executeObject(obj)  ← Step 3

3. execute() Method
   └─ (Same as Step 7 in property change path)
      └─ ViewProvider::updateData()
         └─ View refresh
```

**Key Difference**: Manual recompute skips Steps 1-4 (property change, onChanged, touch, propagation) and goes directly to topological sort and execution.

---

## 6. Signal/Slot Mechanism

FreeCAD uses Qt's signal/slot system for asynchronous notifications:

### Signals Emitted During Recomputation

```cpp
// Signals emitted during recomputation process:

// 1. Property change signal
signalChanged(Property* prop)  // When property changes

// 2. Touch signal
signalTouched()  // When object is touched

// 3. Recomputation signal
signalRecomputed(DocumentObject* obj)  // After execute() completes

// 4. View update signal
signalUpdateData(Property* prop)  // When view needs update
```

### Observers and Listeners

```python
# Python observers can listen to signals:
class MyObserver:
    def slotChanged(self, obj, prop):
        print(f"Property {prop} changed on {obj.Name}")
    
    def slotRecomputed(self, obj):
        print(f"Object {obj.Name} recomputed")

# Register observer
observer = MyObserver()
App.addDocumentObserver(observer)
```

---

## 7. Performance Considerations

### Batching Property Changes

**Inefficient**:
```python
# ❌ Bad: Recomputes after each change
box.Length = 20
doc.recompute()  # Recompute 1
box.Width = 15
doc.recompute()  # Recompute 2
box.Height = 10
doc.recompute()  # Recompute 3
```

**Efficient**:
```python
# ✅ Good: Batch changes, single recompute
box.Length = 20
box.Width = 15
box.Height = 10
doc.recompute()  # Single recompute for all changes
```

### Auto-Recompute Control

```python
# Disable auto-recompute for performance
doc.setAutoRecompute(False)

# Make many changes
box.Length = 20
box.Width = 15
cylinder.Radius = 5
# ... many more changes ...

# Manual recompute once
doc.recompute()  # Single recomputation

# Re-enable auto-recompute
doc.setAutoRecompute(True)
```

---

## 8. Summary

### Key Call Stack Points

1. **Property Change** → `onChanged()` → `touch()` → Dependency propagation
2. **Document.recompute()** → Topological sort → `execute()` in order
3. **execute()** → Compute outputs → `updateData()` → View refresh

### Critical Methods

- **`onChanged()`**: Responds to property changes
- **`touch()`**: Marks object for recomputation
- **`Document.recompute()`**: Orchestrates recomputation
- **`topologicalSort()`**: Orders objects by dependencies
- **`execute()`**: Computes output properties
- **`updateData()`**: Updates visual representation
- **View refresh**: Updates 3D scene

### Principles

1. **Property changes trigger recomputation**, not direct result mutation
2. **Dependencies are respected** through topological sorting
3. **Views are updated** after geometric computation completes
4. **Batching is important** for performance

---

## References

- FreeCAD Source: `src/App/DocumentObject.cpp`
- FreeCAD Source: `src/App/Document.cpp`
- FreeCAD Source: `src/Gui/ViewProvider.cpp`
- Property System: `App::PropertyContainer`
- Recomputation: `Document::recompute()`, `DocumentObject::execute()`
- View System: `Gui::ViewProvider::updateData()`

