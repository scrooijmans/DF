# FreeCAD Type Safety: Property Types and Runtime Selection Checks

This document explains in detail how FreeCAD enforces type safety for features and commands using property types (PropertyLink, PropertyFloat, etc.) and runtime selection checks, without a rigid static type system.

## Overview

FreeCAD achieves type safety through a **dynamic property type system** combined with **runtime validation**. Unlike statically-typed languages, FreeCAD doesn't use compile-time type checking. Instead, it enforces type safety at runtime through:

1. **Property Type System**: Each property has an explicit type (PropertyFloat, PropertyLink, etc.)
2. **Runtime Type Validation**: Property assignments are validated at runtime
3. **Selection Type Checking**: Commands validate selected objects match expected types
4. **Dynamic Property Management**: Properties can be added/removed at runtime while maintaining type safety

This approach provides flexibility (dynamic object creation) while ensuring safety (type validation at runtime).

---

## 1. Property Type System

### Property Type Hierarchy

FreeCAD's property system is built on a base `Property` class with specialized subclasses for each data type:

```
App::Property (base class)
├── App::PropertyBool
├── App::PropertyInteger
├── App::PropertyFloat
├── App::PropertyString
├── App::PropertyVector
├── App::PropertyPlacement
├── App::PropertyLink
├── App::PropertyLinkList
├── App::PropertyLinkSub
├── App::PropertyLinkSubList
├── App::PropertyLength (extends PropertyFloat)
├── App::PropertyAngle (extends PropertyFloat)
└── ... (many more specialized types)
```

### Property Type Enforcement

Each property type enforces type safety by accepting only compatible values.

#### A. Numeric Property Types

**PropertyFloat**: Stores floating-point numbers.

```python
import FreeCAD as App

obj = doc.addObject("Part::FeaturePython", "MyObject")
obj.addProperty("App::PropertyFloat", "Length")

# ✅ Valid assignment
obj.Length = 10.5  # Float value accepted

# ❌ Type error at runtime
obj.Length = "10"  # Raises: TypeError or PropertyError
obj.Length = [1, 2, 3]  # Raises: TypeError
```

**PropertyInteger**: Stores integer values.

```python
obj.addProperty("App::PropertyInteger", "Count")

# ✅ Valid assignment
obj.Count = 42  # Integer accepted

# ❌ Type error at runtime
obj.Count = 42.5  # Raises: TypeError (float not accepted)
obj.Count = "42"  # Raises: TypeError
```

**PropertyLength**: Specialized float for length measurements.

```python
obj.addProperty("App::PropertyLength", "Width")

# ✅ Valid assignment (accepts float, converts units)
obj.Width = 10.0  # Millimeters (default)
obj.Width = "10 mm"  # String with unit (parsed)
obj.Width = "1 cm"   # Unit conversion

# ❌ Type error
obj.Width = "invalid"  # Raises: ValueError (invalid unit string)
```

#### B. Reference Property Types

**PropertyLink**: References a single DocumentObject.

```python
obj.addProperty("App::PropertyLink", "Base")

box = doc.addObject("Part::Box", "Box")
cylinder = doc.addObject("Part::Cylinder", "Cylinder")

# ✅ Valid assignment (DocumentObject reference)
obj.Base = box  # DocumentObject accepted

# ✅ Valid assignment (None to clear reference)
obj.Base = None  # Clears reference

# ❌ Type error at runtime
obj.Base = "Box"  # Raises: TypeError (string not accepted)
obj.Base = 42     # Raises: TypeError (integer not accepted)
obj.Base = box.Shape  # Raises: TypeError (Shape not accepted)
```

**PropertyLinkList**: References multiple DocumentObjects.

```python
obj.addProperty("App::PropertyLinkList", "Tools")

box = doc.addObject("Part::Box", "Box")
cylinder = doc.addObject("Part::Cylinder", "Cylinder")
sphere = doc.addObject("Part::Sphere", "Sphere")

# ✅ Valid assignment (list of DocumentObjects)
obj.Tools = [box, cylinder, sphere]

# ✅ Valid assignment (empty list)
obj.Tools = []

# ✅ Valid assignment (single object, auto-converted to list)
obj.Tools = box  # Automatically converted to [box]

# ❌ Type error at runtime
obj.Tools = "Box"  # Raises: TypeError
obj.Tools = [box, "Cylinder"]  # Raises: TypeError (mixed types)
obj.Tools = box.Shape  # Raises: TypeError
```

**PropertyLinkSub**: References a sub-element of a DocumentObject.

```python
obj.addProperty("App::PropertyLinkSub", "Face")

box = doc.addObject("Part::Box", "Box")
doc.recompute()

# ✅ Valid assignment (object and sub-element)
obj.Face = (box, "Face1")  # Tuple: (DocumentObject, sub-element name)

# ✅ Valid assignment (object only, no sub-element)
obj.Face = (box, "")  # Empty sub-element

# ❌ Type error at runtime
obj.Face = box  # Raises: TypeError (needs tuple)
obj.Face = ("Box", "Face1")  # Raises: TypeError (string not DocumentObject)
obj.Face = (box.Shape, "Face1")  # Raises: TypeError (Shape not DocumentObject)
```

**PropertyLinkSubList**: References multiple sub-elements.

```python
obj.addProperty("App::PropertyLinkSubList", "Faces")

box = doc.addObject("Part::Box", "Box")
doc.recompute()

# ✅ Valid assignment (list of tuples)
obj.Faces = [(box, "Face1"), (box, "Face2")]

# ✅ Valid assignment (mixed: some with sub-elements, some without)
obj.Faces = [(box, "Face1"), (box, "")]

# ❌ Type error at runtime
obj.Faces = box  # Raises: TypeError (needs list of tuples)
obj.Faces = [(box, "Face1"), "invalid"]  # Raises: TypeError
```

#### C. Geometric Property Types

**PropertyVector**: Stores 3D vectors.

```python
obj.addProperty("App::PropertyVector", "Position")

# ✅ Valid assignment (Vector object)
obj.Position = App.Vector(10, 20, 30)

# ✅ Valid assignment (tuple/list, auto-converted)
obj.Position = (10, 20, 30)  # Converted to Vector
obj.Position = [10, 20, 30]  # Converted to Vector

# ❌ Type error at runtime
obj.Position = 10  # Raises: TypeError (scalar not accepted)
obj.Position = (10, 20)  # Raises: ValueError (needs 3 components)
obj.Position = "10,20,30"  # Raises: TypeError
```

**PropertyPlacement**: Stores position and orientation.

```python
obj.addProperty("App::PropertyPlacement", "Placement")

# ✅ Valid assignment (Placement object)
obj.Placement = App.Placement(
    App.Vector(10, 0, 0),
    App.Rotation(App.Vector(0, 0, 1), 45)
)

# ❌ Type error at runtime
obj.Placement = App.Vector(10, 0, 0)  # Raises: TypeError
obj.Placement = "10,0,0"  # Raises: TypeError
```

#### D. Other Property Types

**PropertyString**: Text strings.

```python
obj.addProperty("App::PropertyString", "Name")

# ✅ Valid assignment
obj.Name = "MyObject"

# ✅ Valid assignment (number converted to string)
obj.Name = 42  # Automatically converted to "42"

# ❌ Type error (rare, most types convert to string)
obj.Name = [1, 2, 3]  # May raise: TypeError
```

**PropertyBool**: Boolean values.

```python
obj.addProperty("App::PropertyBool", "Enabled")

# ✅ Valid assignment
obj.Enabled = True
obj.Enabled = False

# ✅ Valid assignment (truthy/falsy values)
obj.Enabled = 1  # Converts to True
obj.Enabled = 0  # Converts to False

# ❌ Type error (ambiguous values)
obj.Enabled = "True"  # May raise: TypeError (string not boolean)
```

---

## 2. Runtime Type Validation

### Property Assignment Interception

When a property is assigned in Python, FreeCAD intercepts the assignment and validates the type before storing the value.

#### A. Property Setter Mechanism

**Location**: `PropertyContainer::__setattr__()` (Python binding)

**What Happens**:
1. Assignment intercepted: `obj.Length = 20`
2. Property lookup: Find property named "Length"
3. Type validation: Check if value matches property type
4. Value conversion: Attempt type conversion if possible
5. Store or error: Store value or raise TypeError

**Code Path** (Conceptual):
```cpp
// C++ Property Setter (simplified)
PyObject* PropertyContainer::__setattr__(PyObject* args)
{
    const char* propName;
    PyObject* value;
    PyArg_ParseTuple(args, "sO", &propName, &value);
    
    // Find property
    Property* prop = getPropertyByName(propName);
    if (!prop) {
        // Not a property, use standard attribute
        return PyObject_GenericSetAttr(self, propName, value);
    }
    
    // Validate type
    if (!prop->isValidType(value)) {
        // Type mismatch - raise error
        PyErr_SetString(PyExc_TypeError, 
            "Property 'Length' expects type 'PropertyFloat', "
            "got type 'str'");
        return nullptr;
    }
    
    // Convert and store
    prop->setPyObject(value);
    return Py_None;
}
```

**Python Example**:
```python
obj = doc.addObject("Part::Box", "Box")

# ✅ Valid: Float assigned to PropertyFloat
obj.Length = 10.5  # Type check passes

# ❌ Invalid: String assigned to PropertyFloat
try:
    obj.Length = "10.5"  # Raises: TypeError
except TypeError as e:
    print(f"Type error: {e}")
    # Output: "Property 'Length' expects type 'PropertyFloat', got type 'str'"
```

#### B. Type Conversion Rules

FreeCAD attempts **safe type conversions** before raising errors:

**Numeric Conversions**:
```python
obj.addProperty("App::PropertyFloat", "Value")

# ✅ Automatic conversions
obj.Value = 10      # int → float (10.0)
obj.Value = "10.5"  # string → float (10.5, if numeric)

# ❌ Invalid conversions
obj.Value = "abc"   # Raises: ValueError (not numeric)
obj.Value = [1, 2]  # Raises: TypeError (list not convertible)
```

**Reference Conversions**:
```python
obj.addProperty("App::PropertyLink", "Base")

box = doc.addObject("Part::Box", "Box")

# ✅ Valid: Direct object reference
obj.Base = box

# ❌ Invalid: No conversion from string to DocumentObject
obj.Base = "Box"  # Raises: TypeError (no string→object conversion)
```

**Vector Conversions**:
```python
obj.addProperty("App::PropertyVector", "Position")

# ✅ Automatic conversions
obj.Position = (10, 20, 30)     # tuple → Vector
obj.Position = [10, 20, 30]     # list → Vector
obj.Position = App.Vector(10, 20, 30)  # Vector → Vector

# ❌ Invalid conversions
obj.Position = 10              # Raises: TypeError (scalar)
obj.Position = (10, 20)        # Raises: ValueError (needs 3 components)
```

#### C. Custom Exception Types

FreeCAD defines custom exception types for property errors:

```python
# PropertyError: General property-related error
try:
    obj.Length = "invalid"
except App.PropertyError as e:
    print(f"Property error: {e}")

# TypeError: Type mismatch
try:
    obj.Base = "Box"
except TypeError as e:
    print(f"Type error: {e}")

# ValueError: Invalid value (e.g., wrong number of components)
try:
    obj.Position = (10, 20)  # Needs 3 components
except ValueError as e:
    print(f"Value error: {e}")
```

---

## 3. Runtime Selection Checks

### Command Type Validation

FreeCAD commands validate selected objects at runtime to ensure they match expected types.

#### A. Selection Validation Pattern

**Location**: Command `Activated()` method

**What Happens**:
1. Command activated (user clicks button or menu)
2. Get selection: Retrieve selected objects from GUI
3. Type validation: Check if selected objects match expected types
4. Execute or error: Proceed with command or show error message

**Example: Fuse Command**:
```python
class FuseCommand:
    def Activated(self):
        # Get selection
        selection = Gui.Selection.getSelection()
        
        # Validate: Need at least 2 objects
        if len(selection) < 2:
            App.Console.PrintError("Select at least 2 objects\n")
            return
        
        # Validate: All must be Part::Feature objects
        for obj in selection:
            if not obj.isDerivedFrom("Part::Feature"):
                App.Console.PrintError(
                    f"Object '{obj.Label}' is not a Part feature\n")
                return
        
        # Type check passed - create fusion
        fusion = doc.addObject("Part::Fuse", "Fusion")
        fusion.Base = selection[0]
        fusion.Tool = selection[1]
        doc.recompute()
```

#### B. Type Checking Methods

FreeCAD provides methods to check object types at runtime:

**isDerivedFrom()**: Check if object is instance of or derived from a type.

```python
box = doc.addObject("Part::Box", "Box")

# ✅ Type checks
box.isDerivedFrom("Part::Feature")      # True (Box is a Feature)
box.isDerivedFrom("App::DocumentObject")  # True (all objects derive from this)
box.isDerivedFrom("Part::Box")          # True (exact type)

# ❌ Type checks
box.isDerivedFrom("Part::Cylinder")     # False
box.isDerivedFrom("Sketcher::SketchObject")  # False
```

**TypeId Property**: Get object's type identifier.

```python
box = doc.addObject("Part::Box", "Box")

# Check exact type
if box.TypeId == "Part::Box":
    print("This is a Box")

# Check base type
if box.TypeId.startswith("Part::"):
    print("This is a Part workbench object")
```

**hasExtension()**: Check if object has a specific extension.

```python
# Check for GroupExtension
if obj.hasExtension("App::GroupExtension"):
    group = obj.getExtensionByType("App::GroupExtension")
    # Use group functionality
```

#### C. Selection Filtering

Commands can filter selections to only accept specific types:

```python
class ChamferCommand:
    def Activated(self):
        # Get selection
        selection = Gui.Selection.getSelection()
        
        # Filter: Only Part::Feature objects
        valid_objects = [
            obj for obj in selection
            if obj.isDerivedFrom("Part::Feature")
        ]
        
        if len(valid_objects) == 0:
            App.Console.PrintError("Select a Part feature\n")
            return
        
        # Use first valid object
        obj = valid_objects[0]
        chamfer = doc.addObject("PartDesign::Chamfer", "Chamfer")
        chamfer.Base = obj
        doc.recompute()
```

#### D. Sub-Element Selection Validation

Commands can validate sub-element selections (faces, edges, vertices):

```python
class FilletCommand:
    def Activated(self):
        # Get selection with sub-elements
        selection = Gui.Selection.getSelectionEx()
        
        if len(selection) == 0:
            App.Console.PrintError("Select edges or faces\n")
            return
        
        # Validate: Must have sub-elements
        for sel in selection:
            obj = sel.Object
            sub_elements = sel.SubElementNames
            
            if not obj.isDerivedFrom("Part::Feature"):
                App.Console.PrintError(
                    f"Object '{obj.Label}' is not a Part feature\n")
                return
            
            if len(sub_elements) == 0:
                App.Console.PrintError(
                    f"Select edges or faces on '{obj.Label}'\n")
                return
            
            # Validate sub-element types
            for sub in sub_elements:
                if not (sub.startswith("Edge") or sub.startswith("Face")):
                    App.Console.PrintError(
                        f"Sub-element '{sub}' is not an edge or face\n")
                    return
        
        # Type checks passed - create fillet
        fillet = doc.addObject("Part::Fillet", "Fillet")
        # ... set up fillet with selected sub-elements
        doc.recompute()
```

---

## 4. Property Type Constraints

### Property Type Restrictions

Some property types have additional constraints beyond basic type checking:

#### A. PropertyLink Type Constraints

**PropertyLink** can be constrained to specific object types:

```python
# Standard PropertyLink (accepts any DocumentObject)
obj.addProperty("App::PropertyLink", "Base")

# Constrained PropertyLink (only accepts Part::Feature)
# Note: This is typically done in C++ code, but can be validated in Python
class MyFeature(App.DocumentObject):
    def __init__(self, obj):
        obj.addProperty("App::PropertyLink", "Base")
        # Constraint: Base must be Part::Feature
    
    def onChanged(self, obj, prop):
        if prop == "Base" and obj.Base:
            if not obj.Base.isDerivedFrom("Part::Feature"):
                App.Console.PrintError(
                    "Base must be a Part::Feature\n")
                obj.Base = None  # Clear invalid reference
```

#### B. PropertyLinkList Constraints

**PropertyLinkList** can validate all objects in the list:

```python
class MyFeature(App.DocumentObject):
    def __init__(self, obj):
        obj.addProperty("App::PropertyLinkList", "Tools")
    
    def onChanged(self, obj, prop):
        if prop == "Tools":
            # Validate all tools are Part::Feature
            for tool in obj.Tools:
                if tool and not tool.isDerivedFrom("Part::Feature"):
                    App.Console.PrintError(
                        f"Tool '{tool.Label}' is not a Part::Feature\n")
                    # Remove invalid reference
                    obj.Tools = [t for t in obj.Tools if t != tool]
```

#### C. Numeric Range Constraints

Properties can validate numeric ranges:

```python
class MyFeature(App.DocumentObject):
    def __init__(self, obj):
        obj.addProperty("App::PropertyFloat", "Length")
        obj.Length = 10.0
    
    def onChanged(self, obj, prop):
        if prop == "Length":
            # Constraint: Length must be positive
            if obj.Length <= 0:
                App.Console.PrintError("Length must be positive\n")
                obj.Length = 1.0  # Reset to valid value
```

---

## 5. Dynamic Property Management

### Adding Properties at Runtime

Properties can be added dynamically while maintaining type safety:

```python
# Create object
obj = doc.addObject("Part::FeaturePython", "MyObject")

# Add properties with explicit types
obj.addProperty("App::PropertyFloat", "Length")
obj.addProperty("App::PropertyLink", "Base")
obj.addProperty("App::PropertyLinkList", "Tools")

# Type safety is enforced immediately
obj.Length = 10.5  # ✅ Valid
obj.Length = "10"  # ❌ TypeError

box = doc.addObject("Part::Box", "Box")
obj.Base = box      # ✅ Valid
obj.Base = "Box"    # ❌ TypeError
```

### Property Type Information

Properties store their type information:

```python
obj = doc.addObject("Part::Box", "Box")

# Get property type
prop = obj.getPropertyByName("Length")
print(prop.Type)  # Output: "App::PropertyLength"

# Check if property exists
if obj.getPropertyByName("Length"):
    print("Length property exists")

# List all properties
for prop_name in obj.PropertiesList:
    prop = obj.getPropertyByName(prop_name)
    print(f"{prop_name}: {prop.Type}")
```

### Removing Properties

Properties can be removed (though this is less common):

```python
# Remove property (if it exists)
if obj.getPropertyByName("CustomProperty"):
    obj.removeProperty("CustomProperty")
```

---

## 6. Feature Type Safety

### Feature Property Validation

Features validate their properties in `onChanged()`:

```python
class MyCustomFeature(App.DocumentObject):
    def __init__(self, obj):
        obj.addProperty("App::PropertyLink", "Base")
        obj.addProperty("App::PropertyFloat", "Offset")
    
    def onChanged(self, obj, prop):
        # Validate Base property
        if prop == "Base":
            if obj.Base:
                # Type check: Base must be Part::Feature
                if not obj.Base.isDerivedFrom("Part::Feature"):
                    App.Console.PrintError(
                        "Base must be a Part::Feature\n")
                    obj.Base = None  # Clear invalid reference
        
        # Validate Offset property
        if prop == "Offset":
            if obj.Offset < 0:
                App.Console.PrintError("Offset must be non-negative\n")
                obj.Offset = 0  # Reset to valid value
    
    def execute(self, obj):
        # Execute only if Base is valid
        if not obj.Base or not obj.Base.isDerivedFrom("Part::Feature"):
            return
        
        # Use Base.Shape (type-safe: we validated Base is Part::Feature)
        base_shape = obj.Base.Shape
        # ... compute result
```

### Feature Dependency Validation

Features validate dependencies in `execute()`:

```python
class MyCustomFeature(App.DocumentObject):
    def execute(self, obj):
        # Validate Base exists and has Shape
        if not obj.Base:
            return  # No base - cannot compute
        
        if not obj.Base.isDerivedFrom("Part::Feature"):
            return  # Invalid type
        
        if not hasattr(obj.Base, "Shape"):
            return  # Base has no Shape property
        
        # Type-safe access: we validated Base is Part::Feature
        base_shape = obj.Base.Shape
        
        # Compute result
        import Part
        obj.Shape = base_shape.copy()  # Example: copy shape
```

---

## 7. Command Type Safety

### Command Selection Validation

Commands validate selections before execution:

```python
class ExtrudeCommand:
    def Activated(self):
        # Get selection
        selection = Gui.Selection.getSelection()
        
        # Type validation: Need exactly 1 sketch
        if len(selection) != 1:
            App.Console.PrintError("Select exactly 1 sketch\n")
            return
        
        sketch = selection[0]
        
        # Type check: Must be a sketch
        if not sketch.isDerivedFrom("Sketcher::SketchObject"):
            App.Console.PrintError("Selected object is not a sketch\n")
            return
        
        # Type-safe: we validated sketch is Sketcher::SketchObject
        pad = doc.addObject("PartDesign::Pad", "Pad")
        pad.Profile = sketch  # Type-safe assignment
        doc.recompute()
```

### Command Property Assignment Validation

Commands validate property assignments:

```python
class OffsetCommand:
    def Activated(self):
        selection = Gui.Selection.getSelectionEx()
        
        if len(selection) == 0:
            return
        
        sel = selection[0]
        obj = sel.Object
        
        # Validate object type
        if not obj.isDerivedFrom("Part::Feature"):
            App.Console.PrintError("Select a Part feature\n")
            return
        
        # Create offset feature
        offset = doc.addObject("Part::Offset", "Offset")
        
        # Type-safe assignment: obj is Part::Feature
        offset.Base = obj
        
        # Validate and set offset distance
        try:
            offset.Value = 1.0  # PropertyFloat - type-safe
        except TypeError as e:
            App.Console.PrintError(f"Invalid offset value: {e}\n")
            return
        
        doc.recompute()
```

---

## 8. Type Safety Without Static Types

### Advantages of Dynamic Type System

1. **Flexibility**: Objects can be created and modified at runtime
2. **Extensibility**: New property types can be added without recompiling
3. **Python Integration**: Natural Python API (no type annotations needed)
4. **User-Friendly**: Users can create custom objects without type system knowledge

### Runtime Type Safety Mechanisms

1. **Property Type Enforcement**: Each property validates assignments
2. **Selection Validation**: Commands check object types before execution
3. **onChanged() Validation**: Features validate property changes
4. **execute() Validation**: Features validate dependencies before computation

### Error Handling

Type errors are caught and reported at runtime:

```python
# Type error caught at assignment time
try:
    box.Length = "invalid"
except (TypeError, App.PropertyError) as e:
    App.Console.PrintError(f"Type error: {e}\n")

# Selection error caught at command execution
try:
    # Command validates selection
    Gui.runCommand("Part_Fuse", 0)
except Exception as e:
    App.Console.PrintError(f"Command error: {e}\n")
```

---

## 9. Summary

### Key Mechanisms

1. **Property Types**: Each property has an explicit type (PropertyFloat, PropertyLink, etc.)
2. **Runtime Validation**: Property assignments are validated at runtime
3. **Type Conversion**: Safe conversions are attempted (int→float, tuple→Vector)
4. **Selection Checks**: Commands validate selected objects match expected types
5. **Feature Validation**: Features validate properties and dependencies in onChanged() and execute()

### Type Safety Without Static Types

FreeCAD achieves type safety through:
- **Explicit Property Types**: Properties declare their types
- **Runtime Validation**: Type checks happen when properties are assigned
- **Selection Validation**: Commands check object types before execution
- **Error Reporting**: Type errors are caught and reported clearly

This provides the safety of static types with the flexibility of dynamic systems.

---

## References

- FreeCAD Property System: `App::PropertyContainer`
- Property Types: `App::PropertyFloat`, `App::PropertyLink`, etc.
- Type Checking: `isDerivedFrom()`, `TypeId`
- Selection: `Gui.Selection.getSelection()`, `Gui.Selection.getSelectionEx()`
- Validation: `onChanged()`, `execute()`

