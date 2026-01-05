# FreeCAD Selection and Input Validation

This document explains in detail how FreeCAD validates selections/inputs for commands and features (Gui::Selection, selection gates/filters, dynamic type checking, exceptions), including typical patterns used to reject invalid inputs before recompute.

## Overview

FreeCAD uses a multi-layered validation system to ensure commands and features receive valid inputs before performing operations. This validation happens at multiple stages: selection filtering, command validation, property validation, and feature validation. Invalid inputs are rejected early to prevent errors during recomputation.

---

## 1. Gui::Selection Validation

### Selection Retrieval Methods

FreeCAD provides multiple methods to retrieve and validate selections:

#### A. Basic Selection Retrieval

**getSelection()**: Get all selected objects.

```python
import FreeCAD as App
import FreeCADGui as Gui

# Get selection
selection = Gui.Selection.getSelection()

# Validate: Check if selection exists
if len(selection) == 0:
    App.Console.PrintError("No objects selected\n")
    return
```

**getSelectionEx()**: Get selection with sub-elements.

```python
# Get selection with sub-elements
selection = Gui.Selection.getSelectionEx()

# Each item in selection has:
# - sel.Object: The selected object
# - sel.SubElementNames: List of sub-element names (Edge1, Face1, etc.)
# - sel.PickedPoints: List of picked points
```

**getSelectionObject()**: Get first selected object.

```python
# Get first selected object
obj = Gui.Selection.getSelectionObject()
if obj is None:
    App.Console.PrintError("No object selected\n")
    return
```

#### B. Type-Specific Selection Methods

**countObjectsOfType(type)**: Count selected objects of specific type.

```python
# Count Part::Feature objects in selection
count = Gui.Selection.countObjectsOfType("Part::Feature")
if count < 2:
    App.Console.PrintError("Select at least 2 Part features\n")
    return
```

**getObjectsOfType(type)**: Get selected objects of specific type.

```python
# Get only Part::Feature objects from selection
part_features = Gui.Selection.getObjectsOfType("Part::Feature")
if len(part_features) < 2:
    App.Console.PrintError("Select at least 2 Part features\n")
    return
```

### Selection Validation Patterns

#### Pattern 1: Count Validation

```python
def validate_selection_count(selection, min_count, max_count=None):
    """Validate selection count"""
    count = len(selection)
    
    if count < min_count:
        App.Console.PrintError(
            f"Select at least {min_count} object(s)\n")
        return False
    
    if max_count is not None and count > max_count:
        App.Console.PrintError(
            f"Select at most {max_count} object(s)\n")
        return False
    
    return True

# Usage
selection = Gui.Selection.getSelection()
if not validate_selection_count(selection, min_count=2, max_count=2):
    return
```

#### Pattern 2: Type Validation

```python
def validate_selection_types(selection, required_type):
    """Validate all selected objects are of required type"""
    invalid = []
    for obj in selection:
        if not obj.isDerivedFrom(required_type):
            invalid.append(obj.Label)
    
    if invalid:
        App.Console.PrintError(
            f"Invalid object types: {', '.join(invalid)}\n"
            f"All objects must be {required_type}\n")
        return False
    
    return True

# Usage
selection = Gui.Selection.getSelection()
if not validate_selection_types(selection, "Part::Feature"):
    return
```

#### Pattern 3: Sub-Element Validation

```python
def validate_sub_elements(selection, allowed_types):
    """Validate sub-elements are of allowed types"""
    valid_subs = []
    
    for sel in selection:
        obj = sel.Object
        sub_elements = sel.SubElementNames
        
        # Validate object type
        if not obj.isDerivedFrom("Part::Feature"):
            App.Console.PrintError(
                f"Object '{obj.Label}' is not a Part feature\n")
            continue
        
        # Validate sub-elements
        if len(sub_elements) == 0:
            App.Console.PrintError(
                f"No sub-elements selected on '{obj.Label}'\n")
            continue
        
        # Validate sub-element types
        for sub in sub_elements:
            sub_type = sub.split()[0] if ' ' in sub else sub.split('e')[0] + 'e'
            if sub_type not in allowed_types:
                App.Console.PrintError(
                    f"Sub-element '{sub}' is not of allowed type\n")
                continue
            
            valid_subs.append((obj, sub))
    
    if len(valid_subs) == 0:
        App.Console.PrintError("No valid sub-elements selected\n")
        return False
    
    return valid_subs

# Usage
selection = Gui.Selection.getSelectionEx()
valid_edges = validate_sub_elements(selection, ["Edge", "Face"])
if not valid_edges:
    return
```

---

## 2. Selection Gates and Filters

### Selection Gate Concept

**Selection gates** restrict what users can select in the 3D view, preventing invalid selections before they occur.

### Setting Selection Gates

#### A. Object Type Gate

**Filter to specific object types**:

```python
# Filter to Part::Feature objects only
Gui.Selection.addSelectionGate("SELECT Part::Feature")

# Filter to Sketcher::SketchObject only
Gui.Selection.addSelectionGate("SELECT Sketcher::SketchObject")

# Clear gate when done
Gui.Selection.clearSelectionGate()
```

#### B. Sub-Element Gate

**Filter to specific sub-elements**:

```python
# Filter to edges only
Gui.Selection.addSelectionGate("SELECT Part::Feature SUBELEMENT Edge")

# Filter to faces only
Gui.Selection.addSelectionGate("SELECT Part::Feature SUBELEMENT Face")

# Filter to vertices only
Gui.Selection.addSelectionGate("SELECT Part::Feature SUBELEMENT Vertex")

# Clear gate
Gui.Selection.clearSelectionGate()
```

#### C. Multiple Type Gate

**Filter to multiple object types** (advanced):

```python
# Custom selection gate (C++ implementation)
# Python can use basic gates, complex gates require C++
```

### Selection Gate Patterns

#### Pattern 1: Temporary Gate

```python
class FilletCommand:
    def Activated(self):
        try:
            # Set selection gate
            Gui.Selection.addSelectionGate("SELECT Part::Feature SUBELEMENT Edge")
            
            # Wait for user selection (via dialog or callback)
            # ...
            
        finally:
            # Always clear gate
            Gui.Selection.clearSelectionGate()
```

#### Pattern 2: Gate with Validation

```python
class SelectEdgeCommand:
    def Activated(self):
        # Set gate to restrict selection
        Gui.Selection.addSelectionGate("SELECT Part::Feature SUBELEMENT Edge")
        
        try:
            # Get selection (user selects with gate active)
            selection = Gui.Selection.getSelectionEx()
            
            # Additional validation
            if len(selection) == 0:
                App.Console.PrintError("Select an edge\n")
                return
            
            # Process selection
            for sel in selection:
                obj = sel.Object
                for sub in sel.SubElementNames:
                    if sub.startswith("Edge"):
                        # Process edge
                        pass
        
        finally:
            # Clear gate
            Gui.Selection.clearSelectionGate()
```

### Selection Filter Commands

FreeCAD provides built-in selection filter commands:

- **Part Select Filter**: Filter to vertices, edges, or faces
- **Custom Filters**: Workbenches can create custom filters

---

## 3. Dynamic Type Checking

### Type Checking Methods

#### A. isDerivedFrom()

**Check if object is instance of or derived from type**:

```python
def validate_object_type(obj, required_type):
    """Validate object is of required type"""
    if not obj.isDerivedFrom(required_type):
        App.Console.PrintError(
            f"Object '{obj.Label}' is not a {required_type}\n")
        return False
    return True

# Usage
box = doc.addObject("Part::Box", "Box")
if not validate_object_type(box, "Part::Feature"):
    return
```

**Inheritance Check**:
```python
# Box is derived from Part::Feature
box.isDerivedFrom("Part::Feature")      # True
box.isDerivedFrom("App::DocumentObject")  # True
box.isDerivedFrom("Part::Box")          # True (exact type)

# Box is not derived from Cylinder
box.isDerivedFrom("Part::Cylinder")     # False
```

#### B. TypeId Property

**Check exact object type**:

```python
def validate_exact_type(obj, required_type):
    """Validate object is exact type"""
    if obj.TypeId != required_type:
        App.Console.PrintError(
            f"Object '{obj.Label}' is not {required_type}\n")
        return False
    return True

# Usage
if not validate_exact_type(box, "Part::Box"):
    return
```

**Base Type Check**:
```python
# Check if object is Part workbench object
if obj.TypeId.startswith("Part::"):
    # Part workbench object
    pass

# Check if object is PartDesign feature
if obj.TypeId.startswith("PartDesign::"):
    # PartDesign feature
    pass
```

#### C. hasExtension()

**Check if object has specific extension**:

```python
def validate_extension(obj, extension_type):
    """Validate object has required extension"""
    if not obj.hasExtension(extension_type):
        App.Console.PrintError(
            f"Object '{obj.Label}' does not have {extension_type}\n")
        return False
    return True

# Usage
if not validate_extension(obj, "App::GroupExtension"):
    return
```

#### D. hasattr() Check

**Check if object has specific attribute**:

```python
def validate_attribute(obj, attribute_name):
    """Validate object has required attribute"""
    if not hasattr(obj, attribute_name):
        App.Console.PrintError(
            f"Object '{obj.Label}' does not have {attribute_name}\n")
        return False
    return True

# Usage
if not validate_attribute(obj, "Shape"):
    return
```

### Type Checking Patterns

#### Pattern 1: Multiple Type Validation

```python
def validate_multiple_types(obj, allowed_types):
    """Validate object is one of allowed types"""
    for allowed_type in allowed_types:
        if obj.isDerivedFrom(allowed_type):
            return True
    
    App.Console.PrintError(
        f"Object '{obj.Label}' must be one of: {', '.join(allowed_types)}\n")
    return False

# Usage
if not validate_multiple_types(obj, ["Part::Feature", "Part::Compound"]):
    return
```

#### Pattern 2: Type with Attribute Check

```python
def validate_type_with_shape(obj):
    """Validate object is Part::Feature and has Shape"""
    if not obj.isDerivedFrom("Part::Feature"):
        App.Console.PrintError("Object must be Part::Feature\n")
        return False
    
    if not hasattr(obj, "Shape"):
        App.Console.PrintError("Object must have Shape property\n")
        return False
    
    if not obj.Shape or obj.Shape.isNull():
        App.Console.PrintError("Object Shape is null\n")
        return False
    
    return True

# Usage
if not validate_type_with_shape(obj):
    return
```

---

## 4. Exception Handling

### Exception Types

FreeCAD uses standard Python exceptions and custom exceptions:

#### A. Standard Exceptions

**TypeError**: Type mismatch.

```python
try:
    obj.Base = "Box"  # String instead of DocumentObject
except TypeError as e:
    App.Console.PrintError(f"Type error: {e}\n")
```

**ValueError**: Invalid value.

```python
try:
    obj.Position = (10, 20)  # Needs 3 components
except ValueError as e:
    App.Console.PrintError(f"Value error: {e}\n")
```

**AttributeError**: Missing attribute.

```python
try:
    shape = obj.Shape  # Object might not have Shape
except AttributeError as e:
    App.Console.PrintError(f"Attribute error: {e}\n")
```

#### B. FreeCAD Custom Exceptions

**PropertyError**: Property-related error.

```python
try:
    obj.Length = "invalid"
except App.PropertyError as e:
    App.Console.PrintError(f"Property error: {e}\n")
```

### Exception Handling Patterns

#### Pattern 1: Try-Except with Validation

```python
def safe_operation(obj):
    """Perform operation with exception handling"""
    try:
        # Validate input
        if not obj.isDerivedFrom("Part::Feature"):
            raise ValueError("Object must be Part::Feature")
        
        # Perform operation
        shape = obj.Shape
        if shape.isNull():
            raise ValueError("Shape is null")
        
        # Success
        return shape
    
    except ValueError as e:
        App.Console.PrintError(f"Validation error: {e}\n")
        return None
    
    except AttributeError as e:
        App.Console.PrintError(f"Attribute error: {e}\n")
        return None
    
    except Exception as e:
        App.Console.PrintError(f"Unexpected error: {e}\n")
        return None
```

#### Pattern 2: Validation Before Operation

```python
def validate_before_operation(obj):
    """Validate before performing operation"""
    # Validate type
    if not obj.isDerivedFrom("Part::Feature"):
        App.Console.PrintError("Invalid object type\n")
        return False
    
    # Validate attribute
    if not hasattr(obj, "Shape"):
        App.Console.PrintError("Object missing Shape\n")
        return False
    
    # Validate value
    if obj.Shape.isNull():
        App.Console.PrintError("Shape is null\n")
        return False
    
    # All validations passed
    return True

# Usage
if not validate_before_operation(obj):
    return

# Safe to perform operation
shape = obj.Shape
```

---

## 5. Command Validation Patterns

### Complete Command Validation

#### Pattern 1: Early Return Validation

```python
class FuseCommand:
    def Activated(self):
        doc = App.ActiveDocument
        if not doc:
            App.Console.PrintError("No active document\n")
            return
        
        selection = Gui.Selection.getSelection()
        
        # Validate count
        if len(selection) < 2:
            App.Console.PrintError("Select at least 2 objects\n")
            return
        
        # Validate types
        for obj in selection:
            if not obj.isDerivedFrom("Part::Feature"):
                App.Console.PrintError(
                    f"Object '{obj.Label}' is not a Part feature\n")
                return
            
            if not hasattr(obj, "Shape"):
                App.Console.PrintError(
                    f"Object '{obj.Label}' has no Shape\n")
                return
            
            if obj.Shape.isNull():
                App.Console.PrintError(
                    f"Object '{obj.Label}' has null Shape\n")
                return
        
        # All validations passed - create fusion
        fusion = doc.addObject("Part::Fuse", "Fusion")
        fusion.Base = selection[0]
        fusion.Tool = selection[1]
        doc.recompute()
```

#### Pattern 2: Validation Function

```python
def validate_fuse_selection(selection):
    """Validate selection for Fuse command"""
    # Count validation
    if len(selection) < 2:
        App.Console.PrintError("Select at least 2 objects\n")
        return False, None
    
    # Type validation
    valid_objects = []
    for obj in selection:
        if not obj.isDerivedFrom("Part::Feature"):
            App.Console.PrintError(
                f"Object '{obj.Label}' is not a Part feature\n")
            continue
        
        if not hasattr(obj, "Shape") or obj.Shape.isNull():
            App.Console.PrintError(
                f"Object '{obj.Label}' has invalid Shape\n")
            continue
        
        valid_objects.append(obj)
    
    if len(valid_objects) < 2:
        App.Console.PrintError("Need at least 2 valid objects\n")
        return False, None
    
    return True, valid_objects

class FuseCommand:
    def Activated(self):
        selection = Gui.Selection.getSelection()
        valid, objects = validate_fuse_selection(selection)
        
        if not valid:
            return
        
        # Create fusion
        fusion = doc.addObject("Part::Fuse", "Fusion")
        fusion.Base = objects[0]
        fusion.Tool = objects[1]
        doc.recompute()
```

---

## 6. Feature Validation (onChanged)

### onChanged Validation

**onChanged()** is called when properties change, providing an opportunity to validate inputs before recomputation.

#### Pattern 1: Type Validation

```python
class MyFeature:
    def onChanged(self, obj, prop):
        """Validate property changes"""
        if prop == "Base" and obj.Base:
            # Validate type
            if not obj.Base.isDerivedFrom("Part::Feature"):
                App.Console.PrintError(
                    "Base must be a Part::Feature\n")
                obj.Base = None  # Clear invalid reference
                return
            
            # Validate attribute
            if not hasattr(obj.Base, "Shape"):
                App.Console.PrintError(
                    "Base must have Shape property\n")
                obj.Base = None
                return
```

#### Pattern 2: Value Range Validation

```python
class MyFeature:
    def onChanged(self, obj, prop):
        """Validate property values"""
        if prop == "Length":
            # Validate range
            if obj.Length <= 0:
                App.Console.PrintError("Length must be positive\n")
                obj.Length = 1.0  # Reset to valid value
                return
            
            if obj.Length > 1000:
                App.Console.PrintError("Length must be <= 1000\n")
                obj.Length = 1000  # Clamp to maximum
                return
```

#### Pattern 3: Dependency Validation

```python
class MyFeature:
    def onChanged(self, obj, prop):
        """Validate dependencies"""
        if prop == "Base" and obj.Base:
            # Check for circular dependency
            if obj.Base == obj:
                App.Console.PrintError("Cannot reference self\n")
                obj.Base = None
                return
            
            # Check if Base depends on this (would create cycle)
            if obj in obj.Base.InList:
                App.Console.PrintError("Circular dependency detected\n")
                obj.Base = None
                return
```

#### Pattern 4: Multiple Property Validation

```python
class MyFeature:
    def onChanged(self, obj, prop):
        """Validate multiple properties"""
        # Validate Base
        if prop == "Base" and obj.Base:
            if not obj.Base.isDerivedFrom("Part::Feature"):
                App.Console.PrintError("Base must be Part::Feature\n")
                obj.Base = None
                return
        
        # Validate Length
        if prop == "Length":
            if obj.Length <= 0:
                App.Console.PrintError("Length must be positive\n")
                obj.Length = 1.0
                return
        
        # Cross-property validation
        if prop in ["Base", "Length"]:
            if obj.Base and obj.Length > 100:
                # Check if Base can handle large length
                if hasattr(obj.Base, "Shape"):
                    bbox = obj.Base.Shape.BoundBox
                    if bbox.XLength < obj.Length:
                        App.Console.PrintWarning(
                            "Length exceeds base object size\n")
```

---

## 7. Feature Validation (execute)

### execute Validation

**execute()** validates inputs before computation, rejecting invalid inputs to prevent recomputation errors.

#### Pattern 1: Early Return on Invalid Input

```python
class MyFeature:
    def execute(self, obj):
        """Recompute with validation"""
        # Validate Base
        if not obj.Base:
            App.Console.PrintError("Base is not set\n")
            return  # Early return - no computation
        
        # Validate Base type
        if not obj.Base.isDerivedFrom("Part::Feature"):
            App.Console.PrintError("Base must be Part::Feature\n")
            return
        
        # Validate Base has Shape
        if not hasattr(obj.Base, "Shape"):
            App.Console.PrintError("Base has no Shape\n")
            return
        
        # Validate Base Shape is valid
        try:
            base_shape = obj.Base.Shape
            if base_shape.isNull():
                App.Console.PrintError("Base Shape is null\n")
                return
        except Exception as e:
            App.Console.PrintError(f"Error accessing Base Shape: {e}\n")
            return
        
        # Validate Length
        if obj.Length <= 0:
            App.Console.PrintError("Length must be positive\n")
            return
        
        # All validations passed - compute
        import Part
        obj.Shape = base_shape.copy()
```

#### Pattern 2: Validation with Error State

```python
class MyFeature:
    def execute(self, obj):
        """Recompute with error state"""
        # Validate inputs
        errors = []
        
        if not obj.Base:
            errors.append("Base is not set")
        
        if obj.Base and not obj.Base.isDerivedFrom("Part::Feature"):
            errors.append("Base must be Part::Feature")
        
        if obj.Length <= 0:
            errors.append("Length must be positive")
        
        # If errors, set error state and return
        if errors:
            App.Console.PrintError("Validation errors:\n")
            for error in errors:
                App.Console.PrintError(f"  - {error}\n")
            return  # Don't compute
        
        # All valid - compute
        import Part
        obj.Shape = obj.Base.Shape.copy()
```

#### Pattern 3: Graceful Degradation

```python
class MyFeature:
    def execute(self, obj):
        """Recompute with graceful degradation"""
        # Try to use Base if available
        if obj.Base and obj.Base.isDerivedFrom("Part::Feature"):
            try:
                base_shape = obj.Base.Shape
                if not base_shape.isNull():
                    # Use Base
                    obj.Shape = base_shape.copy()
                    return
            except Exception:
                pass
        
        # Fallback: Create default shape
        App.Console.PrintWarning("Using default shape (Base invalid)\n")
        import Part
        obj.Shape = Part.makeBox(obj.Length, obj.Width, obj.Height)
```

---

## 8. Complete Validation Example

### Example: Fuse Command with Full Validation

```python
import FreeCAD as App
import FreeCADGui as Gui

class FuseCommand:
    def GetResources(self):
        return {
            "Pixmap": "Part/icons/Part_Fuse.svg",
            "MenuText": "Fuse",
            "ToolTip": "Fuses selected objects"
        }
    
    def IsActive(self):
        """Command active only if valid selection"""
        selection = Gui.Selection.getSelection()
        if len(selection) < 2:
            return False
        
        # Check all are Part::Feature
        return all(
            obj.isDerivedFrom("Part::Feature") and
            hasattr(obj, "Shape") and
            not obj.Shape.isNull()
            for obj in selection
        )
    
    def Activated(self):
        """Command activation with full validation"""
        doc = App.ActiveDocument
        if not doc:
            App.Console.PrintError("No active document\n")
            return
        
        # Get selection
        selection = Gui.Selection.getSelection()
        
        # Validate count
        if len(selection) < 2:
            App.Console.PrintError("Select at least 2 objects\n")
            return
        
        # Validate types and shapes
        valid_objects = []
        for obj in selection:
            # Type check
            if not obj.isDerivedFrom("Part::Feature"):
                App.Console.PrintError(
                    f"Object '{obj.Label}' is not a Part feature\n")
                continue
            
            # Attribute check
            if not hasattr(obj, "Shape"):
                App.Console.PrintError(
                    f"Object '{obj.Label}' has no Shape property\n")
                continue
            
            # Shape validity check
            try:
                if obj.Shape.isNull():
                    App.Console.PrintError(
                        f"Object '{obj.Label}' has null Shape\n")
                    continue
            except Exception as e:
                App.Console.PrintError(
                    f"Error checking Shape of '{obj.Label}': {e}\n")
                continue
            
            valid_objects.append(obj)
        
        # Check we have enough valid objects
        if len(valid_objects) < 2:
            App.Console.PrintError("Need at least 2 valid objects\n")
            return
        
        # Create fusion
        try:
            fusion = doc.addObject("Part::Fuse", "Fusion")
            fusion.Base = valid_objects[0]
            fusion.Tool = valid_objects[1]
            
            # Additional objects go to Tools (if PropertyLinkList exists)
            if len(valid_objects) > 2 and hasattr(fusion, "Tools"):
                fusion.Tools = valid_objects[2:]
            
            doc.recompute()
        
        except Exception as e:
            App.Console.PrintError(f"Error creating fusion: {e}\n")

Gui.addCommand("Part_Fuse", FuseCommand())
```

---

## 9. Validation Best Practices

### Best Practices

#### A. Validate Early

**✅ DO**: Validate inputs as early as possible.

```python
def Activated(self):
    # ✅ Validate immediately
    selection = Gui.Selection.getSelection()
    if len(selection) == 0:
        return  # Early return
    
    # Continue only if valid
```

**❌ DON'T**: Validate after performing operations.

```python
def Activated(self):
    # ❌ Bad: Create object first, then validate
    fusion = doc.addObject("Part::Fuse", "Fusion")
    fusion.Base = selection[0]  # Might be invalid
    # Validation happens too late
```

#### B. Clear Error Messages

**✅ DO**: Provide specific, actionable error messages.

```python
App.Console.PrintError(
    f"Object '{obj.Label}' is not a Part feature. "
    f"Select a Part::Feature object.\n")
```

**❌ DON'T**: Provide vague error messages.

```python
App.Console.PrintError("Invalid selection\n")  # Too vague
```

#### C. Validate All Inputs

**✅ DO**: Validate all inputs before using them.

```python
# Validate Base
if not obj.Base:
    return
if not obj.Base.isDerivedFrom("Part::Feature"):
    return

# Validate Length
if obj.Length <= 0:
    return

# All validated - safe to use
```

**❌ DON'T**: Assume inputs are valid.

```python
# ❌ Bad: No validation
base_shape = obj.Base.Shape  # Might fail
```

#### D. Use Selection Gates

**✅ DO**: Use selection gates to prevent invalid selections.

```python
Gui.Selection.addSelectionGate("SELECT Part::Feature")
try:
    # User can only select Part::Feature
    pass
finally:
    Gui.Selection.clearSelectionGate()
```

#### E. Handle Exceptions

**✅ DO**: Handle exceptions gracefully.

```python
try:
    shape = obj.Shape
except AttributeError:
    App.Console.PrintError("Object has no Shape\n")
    return
except Exception as e:
    App.Console.PrintError(f"Unexpected error: {e}\n")
    return
```

---

## 10. Summary

### Validation Layers

1. **Selection Gates**: Prevent invalid selections in UI
2. **Command Validation**: Validate selection in `Activated()`
3. **Property Validation**: Validate in `onChanged()`
4. **Feature Validation**: Validate in `execute()`

### Validation Methods

1. **Type Checking**: `isDerivedFrom()`, `TypeId`, `hasExtension()`
2. **Attribute Checking**: `hasattr()`
3. **Value Checking**: Range validation, null checks
4. **Exception Handling**: Try-except blocks

### Validation Patterns

1. **Early Return**: Return immediately on invalid input
2. **Validation Functions**: Separate validation logic
3. **Error State**: Set error state instead of computing
4. **Graceful Degradation**: Fallback to default behavior

### Key Principles

1. **Validate Early**: Check inputs as soon as possible
2. **Clear Messages**: Provide specific error messages
3. **Handle Exceptions**: Catch and handle errors gracefully
4. **Prevent Recompute Errors**: Reject invalid inputs before recomputation

---

## References

- Selection: `Gui.Selection.getSelection()`, `Gui.Selection.getSelectionEx()`
- Selection Gates: `Gui.Selection.addSelectionGate()`, `Gui.Selection.clearSelectionGate()`
- Type Checking: `isDerivedFrom()`, `TypeId`, `hasExtension()`
- Validation: `onChanged()`, `execute()`
- Exceptions: `TypeError`, `ValueError`, `AttributeError`, `App.PropertyError`

