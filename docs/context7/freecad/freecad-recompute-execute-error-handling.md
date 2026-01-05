# FreeCAD Recompute Mechanism: execute() Calls, Dependency Checking, and Error Handling

This document explains in detail how FreeCAD's recompute mechanism calls object `execute()` methods, how objects check dependencies and input validity during `execute()`, and how errors are communicated to the user and document state.

## Overview

FreeCAD's recompute mechanism is a sophisticated system that ensures all objects in a document are updated in the correct order when changes occur. The system calls `execute()` methods on objects, validates dependencies and inputs, and communicates errors through multiple channels to maintain document integrity and provide user feedback.

---

## 1. Recompute Mechanism Overview

### Recompute Flow

```
Property Change / Manual Recompute
    ↓
Document.recompute()
    ↓
Collect Touched Objects
    ↓
Topological Sort (dependency order)
    ↓
For each object in sorted order:
    ↓
    execute() called
    ↓
    Dependency Validation
    ↓
    Input Validation
    ↓
    Computation
    ↓
    Error Handling
    ↓
    Update Document State
    ↓
Update Views
```

### Key Components

1. **Document.recompute()**: Main entry point for recomputation
2. **Topological Sort**: Orders objects by dependencies
3. **execute() Method**: Performs computation for each object
4. **Error Handling**: Manages errors and communicates to user
5. **Document State**: Tracks object status (touched, erroneous, etc.)

---

## 2. Document.recompute() Mechanism

### Recompute Entry Point

**Location**: `App::Document::recompute()`

**Purpose**: Orchestrate recomputation of all touched objects.

**Process**:
1. Collect all touched objects
2. Perform topological sort
3. Execute objects in dependency order
4. Handle errors
5. Update document state

### Implementation (Conceptual)

```cpp
// C++ Implementation (simplified)
DocumentObjectExecReturn* Document::recompute()
{
    // 1. Collect touched objects
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
        DocumentObjectExecReturn* result = obj->execute();
        
        // Handle result
        if (result && result->Status == DocumentObjectExecReturn::Error) {
            // Error occurred
            obj->setStatus(ObjectStatus::Erroneous, true);
            obj->setStatus(ObjectStatus::Recomputing, false);
            
            // Log error
            Console().Error("Recompute failed for %s: %s\n", 
                          obj->Label.getValue(), result->Why);
            
            // Continue with other objects (don't stop entire recompute)
        } else {
            // Success
            obj->setStatus(ObjectStatus::Touched, false);
            obj->setStatus(ObjectStatus::Erroneous, false);
            obj->setStatus(ObjectStatus::Recomputing, false);
        }
    }
    
    // 4. Update views
    signalRecomputed();
    
    return nullptr;
}
```

### Python API

```python
# Manual recomputation
doc.recompute()  # Recomputes all touched objects

# Recomputation with specific object
box.recomputeFeature()  # Recomputes single object and dependents
```

---

## 3. Topological Sort and Execution Order

### Topological Sort Algorithm

**Purpose**: Order objects so dependencies are computed before dependents.

**Algorithm**: Kahn's algorithm (or DFS-based)

```cpp
// Topological Sort (Kahn's algorithm)
std::vector<DocumentObject*> Document::topologicalSort(
    const std::vector<DocumentObject*>& objects)
{
    std::vector<DocumentObject*> result;
    std::queue<DocumentObject*> queue;
    std::map<DocumentObject*, int> inDegree;
    
    // Calculate in-degrees (number of dependencies)
    for (DocumentObject* obj : objects) {
        inDegree[obj] = obj->getOutList().size();  // Dependencies count
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

### Execution Order Example

```python
# Dependency chain: sketch -> pad -> chamfer
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
pad = doc.addObject("PartDesign::Pad", "Pad")
chamfer = doc.addObject("PartDesign::Chamfer", "Chamfer")

pad.Profile = sketch      # pad depends on sketch
chamfer.Base = pad        # chamfer depends on pad

# Topological sort result: [sketch, pad, chamfer]
# Execution order:
# 1. sketch.execute() - no dependencies
# 2. pad.execute() - sketch already computed
# 3. chamfer.execute() - pad already computed
```

**Key Point**: Dependencies are always computed before dependents, ensuring inputs are ready when `execute()` is called.

---

## 4. execute() Method Call Mechanism

### execute() Signature

**C++**:
```cpp
virtual App::DocumentObjectExecReturn* execute();
```

**Python**:
```python
def execute(self, obj):
    # Computation logic
    return None  # Success
```

### execute() Call Flow

```
Document.recompute()
    ↓
For each object in topological order:
    ↓
    Set status: Recomputing = true
    ↓
    Call obj.execute()
    ↓
    execute() performs:
        - Dependency validation
        - Input validation
        - Computation
        - Error handling
    ↓
    Check return value
    ↓
    Update document state
```

### execute() Implementation Pattern

```python
class MyFeature:
    def execute(self, obj):
        """Recompute object"""
        # 1. Validate dependencies
        if not self.validate_dependencies(obj):
            return  # Early return on invalid dependencies
        
        # 2. Validate inputs
        if not self.validate_inputs(obj):
            return  # Early return on invalid inputs
        
        # 3. Read dependencies (already computed)
        dependencies = self.read_dependencies(obj)
        
        # 4. Read inputs
        inputs = self.read_inputs(obj)
        
        # 5. Compute output
        try:
            output = self.compute(inputs, dependencies)
            obj.Shape = output
        except Exception as e:
            # Error during computation
            App.Console.PrintError(f"Computation error: {e}\n")
            return
        
        # 6. Success - object is now valid
        return None
```

---

## 5. Dependency Checking in execute()

### Dependency Validation

Objects check dependencies in `execute()` to ensure they're valid and ready.

#### Pattern 1: Basic Dependency Check

```python
class MyFeature:
    def execute(self, obj):
        """Check dependencies exist"""
        # Check Base dependency
        if not obj.Base:
            App.Console.PrintError("Base is not set\n")
            return  # Early return
        
        # Check Base is valid object
        if not obj.Base.isDerivedFrom("Part::Feature"):
            App.Console.PrintError("Base must be Part::Feature\n")
            return
        
        # Check Base has Shape (dependency was computed)
        if not hasattr(obj.Base, "Shape"):
            App.Console.PrintError("Base has no Shape\n")
            return
        
        # Use dependency
        base_shape = obj.Base.Shape
        obj.Shape = base_shape.copy()
```

#### Pattern 2: Multiple Dependencies

```python
class MyFeature:
    def execute(self, obj):
        """Check multiple dependencies"""
        # Check Base
        if not obj.Base:
            App.Console.PrintError("Base is not set\n")
            return
        
        if not obj.Base.isDerivedFrom("Part::Feature"):
            App.Console.PrintError("Base must be Part::Feature\n")
            return
        
        # Check Tool (optional dependency)
        if obj.Tool:
            if not obj.Tool.isDerivedFrom("Part::Feature"):
                App.Console.PrintError("Tool must be Part::Feature\n")
                return
        
        # Use dependencies
        base_shape = obj.Base.Shape
        if obj.Tool:
            tool_shape = obj.Tool.Shape
            obj.Shape = base_shape.fuse(tool_shape)
        else:
            obj.Shape = base_shape.copy()
```

#### Pattern 3: Dependency Shape Validation

```python
class MyFeature:
    def execute(self, obj):
        """Validate dependency shapes"""
        if not obj.Base:
            return
        
        try:
            base_shape = obj.Base.Shape
            
            # Validate shape is not null
            if base_shape.isNull():
                App.Console.PrintError("Base Shape is null\n")
                return
            
            # Validate shape is valid
            if not base_shape.isValid():
                App.Console.PrintError("Base Shape is invalid\n")
                return
            
            # Validate shape type (e.g., must be solid)
            if base_shape.ShapeType != "Solid":
                App.Console.PrintError("Base Shape must be a Solid\n")
                return
            
            # Use valid shape
            obj.Shape = base_shape.copy()
        
        except Exception as e:
            App.Console.PrintError(f"Error accessing Base Shape: {e}\n")
            return
```

#### Pattern 4: Dependency State Check

```python
class MyFeature:
    def execute(self, obj):
        """Check dependency state"""
        if not obj.Base:
            return
        
        # Check dependency is not erroneous
        if obj.Base.isErroneous():
            App.Console.PrintError(
                f"Base '{obj.Base.Label}' has errors\n")
            return
        
        # Check dependency is not being recomputed
        if obj.Base.isRecomputing():
            # This shouldn't happen (topological sort ensures order)
            # But check anyway for safety
            App.Console.PrintError(
                f"Base '{obj.Base.Label}' is still recomputing\n")
            return
        
        # Check dependency is valid (not touched)
        if obj.Base.isTouched():
            # Dependency should have been computed already
            # This indicates a problem with topological sort
            App.Console.PrintError(
                f"Base '{obj.Base.Label}' is still touched\n")
            return
        
        # Dependency is valid - use it
        obj.Shape = obj.Base.Shape.copy()
```

---

## 6. Input Validity Checking in execute()

### Input Validation Patterns

#### Pattern 1: Property Value Validation

```python
class MyFeature:
    def execute(self, obj):
        """Validate input properties"""
        # Validate Length
        if obj.Length <= 0:
            App.Console.PrintError("Length must be positive\n")
            return
        
        if obj.Length > 1000:
            App.Console.PrintError("Length must be <= 1000\n")
            return
        
        # Validate Width
        if obj.Width <= 0:
            App.Console.PrintError("Width must be positive\n")
            return
        
        # All inputs valid - compute
        import Part
        obj.Shape = Part.makeBox(obj.Length, obj.Width, obj.Height)
```

#### Pattern 2: Combined Input Validation

```python
class MyFeature:
    def execute(self, obj):
        """Validate all inputs together"""
        errors = []
        
        # Collect validation errors
        if not obj.Base:
            errors.append("Base is not set")
        
        if obj.Base and not obj.Base.isDerivedFrom("Part::Feature"):
            errors.append("Base must be Part::Feature")
        
        if obj.Length <= 0:
            errors.append("Length must be positive")
        
        if obj.Width <= 0:
            errors.append("Width must be positive")
        
        # Report all errors
        if errors:
            App.Console.PrintError("Validation errors:\n")
            for error in errors:
                App.Console.PrintError(f"  - {error}\n")
            return
        
        # All valid - compute
        import Part
        obj.Shape = Part.makeBox(obj.Length, obj.Width, obj.Height)
```

#### Pattern 3: Input Range Validation

```python
class MyFeature:
    def execute(self, obj):
        """Validate input ranges"""
        # Validate numeric ranges
        if obj.Length < 0.1 or obj.Length > 1000:
            App.Console.PrintError(
                f"Length {obj.Length} out of range [0.1, 1000]\n")
            return
        
        # Validate enum values
        if obj.Mode not in ["Mode1", "Mode2", "Mode3"]:
            App.Console.PrintError(f"Invalid Mode: {obj.Mode}\n")
            return
        
        # Validate vector components
        if obj.Position.Length < 0 or obj.Position.Length > 1000:
            App.Console.PrintError("Position out of range\n")
            return
        
        # All valid - compute
        pass
```

---

## 7. Error Communication to User

### Error Communication Channels

FreeCAD communicates errors through multiple channels:

#### A. Console Messages

**App.Console.PrintError()**: Error messages in Report View.

```python
def execute(self, obj):
    if not obj.Base:
        App.Console.PrintError("Base is not set\n")
        return
    
    # Error appears in Report View
```

**App.Console.PrintWarning()**: Warning messages.

```python
def execute(self, obj):
    if obj.Length > 100:
        App.Console.PrintWarning("Length is very large\n")
        # Continue computation
```

**App.Console.PrintMessage()**: Informational messages.

```python
def execute(self, obj):
    App.Console.PrintMessage(f"Computing {obj.Label}\n")
```

#### B. Document Object Status

**Status Bits**: Objects have status bits indicating state.

```python
# Status bits (C++ concept, exposed in Python)
obj.setStatus(ObjectStatus::Erroneous, True)  # Mark as erroneous
obj.setStatus(ObjectStatus::Touched, False)    # Clear touched status
```

**Status Properties**:
- **Touched**: Object needs recomputation
- **Erroneous**: Object has errors
- **Recomputing**: Object is currently being recomputed
- **Valid**: Object is up-to-date

#### C. Tree View Indicators

**Visual Indicators**: Objects show status in tree view.

- **Blue Icon**: Object is touched (needs recomputation)
- **Red Icon**: Object has errors
- **Tooltip**: Hovering shows error message

#### D. DocumentObjectExecReturn

**Return Value**: `execute()` can return error information.

**C++**:
```cpp
// Return error
return new DocumentObjectExecReturn(
    DocumentObjectExecReturn::Error,
    "Error message"
);

// Return success
return nullptr;
```

**Python**:
```python
def execute(self, obj):
    # Python execute() typically returns None for success
    # Errors are communicated via Console and status bits
    if error:
        App.Console.PrintError("Error message\n")
        return  # Early return indicates error
    return None  # Success
```

### Error Communication Patterns

#### Pattern 1: Console Error with Status

```python
class MyFeature:
    def execute(self, obj):
        if not obj.Base:
            # Console error
            App.Console.PrintError(
                f"Object '{obj.Label}': Base is not set\n")
            
            # Set erroneous status (if available)
            # obj.setStatus(ObjectStatus::Erroneous, True)
            
            return  # Early return
```

#### Pattern 2: Detailed Error Messages

```python
class MyFeature:
    def execute(self, obj):
        errors = []
        
        if not obj.Base:
            errors.append("Base is not set")
        
        if obj.Length <= 0:
            errors.append(f"Length ({obj.Length}) must be positive")
        
        if errors:
            # Detailed error message
            App.Console.PrintError(
                f"Object '{obj.Label}' errors:\n")
            for error in errors:
                App.Console.PrintError(f"  - {error}\n")
            return
```

#### Pattern 3: Exception Handling

```python
class MyFeature:
    def execute(self, obj):
        try:
            # Computation that might fail
            import Part
            obj.Shape = Part.makeBox(obj.Length, obj.Width, obj.Height)
        
        except Part.OCCError as e:
            # OCC (OpenCASCADE) error
            App.Console.PrintError(
                f"Geometric error in '{obj.Label}': {e}\n")
            return
        
        except Exception as e:
            # Unexpected error
            App.Console.PrintError(
                f"Unexpected error in '{obj.Label}': {e}\n")
            return
```

---

## 8. Document State Management

### Object Status Bits

Objects have status bits that track their state:

**Status Bits**:
- **Touch**: Object needs recomputation
- **Erroneous**: Object has errors
- **Recompute**: Object is currently being recomputed
- **New**: Object is newly created
- **Restore**: Object is being restored

### Status Management During Recomputation

```cpp
// C++ Status Management (conceptual)
void Document::recomputeObject(DocumentObject* obj)
{
    // Set recomputing status
    obj->setStatus(ObjectStatus::Recomputing, true);
    obj->setStatus(ObjectStatus::Erroneous, false);
    
    // Call execute()
    DocumentObjectExecReturn* result = obj->execute();
    
    if (result && result->Status == DocumentObjectExecReturn::Error) {
        // Error occurred
        obj->setStatus(ObjectStatus::Erroneous, true);
        obj->setStatus(ObjectStatus::Touched, true);  // Still needs recomputation
    } else {
        // Success
        obj->setStatus(ObjectStatus::Touched, false);
        obj->setStatus(ObjectStatus::Erroneous, false);
    }
    
    // Clear recomputing status
    obj->setStatus(ObjectStatus::Recomputing, false);
}
```

### Document State After Recomputation

**Success State**:
- Object status: Touched = false, Erroneous = false
- Object Shape: Valid and computed
- Dependents: Can now use this object

**Error State**:
- Object status: Touched = true, Erroneous = true
- Object Shape: May be invalid or null
- Dependents: May also be erroneous (depends on error handling)

### Error Propagation

**Error Propagation Pattern**:
```
Object A (erroneous)
    ↓
Object B depends on A
    ↓
Object B.execute() checks A
    ↓
Object B detects A is erroneous
    ↓
Object B sets itself erroneous
    ↓
Object C depends on B
    ↓
(Error propagates through dependency chain)
```

**Example**:
```python
class MyFeature:
    def execute(self, obj):
        # Check if dependency is erroneous
        if obj.Base and obj.Base.isErroneous():
            App.Console.PrintError(
                f"Base '{obj.Base.Label}' has errors\n")
            # This object also becomes erroneous
            # (status set by recompute mechanism)
            return
        
        # Continue computation
        obj.Shape = obj.Base.Shape.copy()
```

---

## 9. Complete Recompute Example

### Example: Complex Dependency Chain with Errors

```python
# Create objects with dependencies
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
pad = doc.addObject("PartDesign::Pad", "Pad")
chamfer = doc.addObject("PartDesign::Chamfer", "Chamfer")

pad.Profile = sketch
chamfer.Base = pad

# Modify sketch (invalid geometry)
sketch.addGeometry(Part.Circle())  # Invalid - no constraints
doc.recompute()

# Recomputation flow:
# 1. Document.recompute() called
# 2. Topological sort: [sketch, pad, chamfer]
# 3. sketch.execute():
#    - Validates inputs
#    - Computes sketch.Shape
#    - Fails: sketch has invalid geometry
#    - Sets sketch status: Erroneous = true
#    - Console: "Sketch has invalid geometry"
# 4. pad.execute():
#    - Checks sketch dependency
#    - Detects sketch is erroneous
#    - Sets pad status: Erroneous = true
#    - Console: "Base 'Sketch' has errors"
#    - Returns early (no computation)
# 5. chamfer.execute():
#    - Checks pad dependency
#    - Detects pad is erroneous
#    - Sets chamfer status: Erroneous = true
#    - Console: "Base 'Pad' has errors"
#    - Returns early (no computation)
# 6. Document state:
#    - sketch: Erroneous = true, Touched = true
#    - pad: Erroneous = true, Touched = true
#    - chamfer: Erroneous = true, Touched = true
# 7. Tree view shows red icons on all objects
# 8. User sees error messages in Report View
```

---

## 10. Best Practices

### Best Practices for execute()

#### A. Validate Early

**✅ DO**: Validate dependencies and inputs at the start of `execute()`.

```python
def execute(self, obj):
    # ✅ Validate immediately
    if not obj.Base:
        return
    if not obj.Base.isDerivedFrom("Part::Feature"):
        return
```

**❌ DON'T**: Validate after starting computation.

```python
def execute(self, obj):
    # ❌ Bad: Start computation, then validate
    shape = obj.Base.Shape  # Might fail
    if not obj.Base:
        return  # Too late
```

#### B. Clear Error Messages

**✅ DO**: Provide specific, actionable error messages.

```python
App.Console.PrintError(
    f"Object '{obj.Label}': Base '{obj.Base.Label}' is not Part::Feature\n")
```

**❌ DON'T**: Provide vague error messages.

```python
App.Console.PrintError("Error\n")  # Too vague
```

#### C. Handle Exceptions

**✅ DO**: Catch and handle exceptions gracefully.

```python
try:
    shape = obj.Base.Shape
except AttributeError:
    App.Console.PrintError("Base has no Shape\n")
    return
except Exception as e:
    App.Console.PrintError(f"Unexpected error: {e}\n")
    return
```

#### D. Check Dependency State

**✅ DO**: Check if dependencies are valid before using them.

```python
if obj.Base.isErroneous():
    App.Console.PrintError("Base has errors\n")
    return
```

#### E. Return Early on Errors

**✅ DO**: Return immediately when errors are detected.

```python
if not obj.Base:
    App.Console.PrintError("Base not set\n")
    return  # Early return
```

---

## 11. Summary

### Recompute Mechanism

1. **Document.recompute()**: Main entry point
2. **Topological Sort**: Orders objects by dependencies
3. **execute() Calls**: Each object's `execute()` called in order
4. **Dependency Validation**: Objects check dependencies are valid
5. **Input Validation**: Objects validate input properties
6. **Error Handling**: Errors caught and communicated
7. **State Update**: Document state updated based on results

### Dependency Checking

1. **Existence Check**: Dependency exists
2. **Type Check**: Dependency is correct type
3. **State Check**: Dependency is not erroneous
4. **Shape Check**: Dependency has valid Shape
5. **Readiness Check**: Dependency was computed (not touched)

### Input Validation

1. **Property Values**: Validate numeric ranges, enums, etc.
2. **Required Properties**: Check required properties are set
3. **Property Types**: Validate property types
4. **Cross-Property**: Validate relationships between properties

### Error Communication

1. **Console Messages**: `App.Console.PrintError()`
2. **Status Bits**: Object status (Erroneous, Touched)
3. **Tree View**: Visual indicators (red icons)
4. **Tooltips**: Error messages on hover
5. **Report View**: Detailed error messages

### Document State

1. **Status Bits**: Track object state (Touched, Erroneous, Recomputing)
2. **Error Propagation**: Errors propagate through dependency chain
3. **State After Recompute**: Objects marked as valid or erroneous
4. **User Feedback**: Visual and textual error indicators

---

## References

- Recomputation: `Document::recompute()`, `DocumentObject::execute()`
- Topological Sort: `Document::topologicalSort()`
- Status Bits: `ObjectStatus::Touched`, `ObjectStatus::Erroneous`
- Error Communication: `App.Console.PrintError()`, `DocumentObjectExecReturn`
- Dependency Checking: `isDerivedFrom()`, `isErroneous()`, `isTouched()`

