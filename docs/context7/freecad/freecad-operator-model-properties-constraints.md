# FreeCAD Operator Model: Document Objects, Properties, and Type Constraints

This document explains in detail how FreeCAD defines "operators" as document objects/features (App::DocumentObject, PartDesign features, Python feature objects), how they declare inputs/properties, and how type constraints are represented (PropertyLink, PropertyLinkSub, PropertyFloat, etc.).

## Overview

In FreeCAD, **operators** are document objects that perform operations on other objects or compute results from inputs. Operators are defined as subclasses of `App::DocumentObject` and use FreeCAD's property system to declare inputs, outputs, and type constraints. This model enables parametric modeling where operators can be chained together through dependencies.

---

## 1. Operators as Document Objects

### What is an Operator?

An **operator** in FreeCAD is a document object that:
- **Takes inputs**: Properties that define parameters and references
- **Performs computation**: `execute()` method that computes outputs
- **Produces outputs**: Properties that store computed results (e.g., `Shape`)
- **Maintains dependencies**: Relationships to input objects

### Operator Hierarchy

```
App::DocumentObject (base class for all operators)
├── App::GeoFeature (geometric operators)
│   ├── Part::Feature (Part workbench operators)
│   │   ├── Part::Box (primitive operator)
│   │   ├── Part::Fuse (boolean operator)
│   │   ├── Part::Cut (boolean operator)
│   │   └── ...
│   └── PartDesign::Feature (PartDesign operators)
│       ├── PartDesign::Pad (extrusion operator)
│       ├── PartDesign::Pocket (cutting operator)
│       └── ...
├── App::FeaturePython (Python-scripted operators)
└── ... (other operator types)
```

### Operator Characteristics

**All operators share**:
- **Properties**: Input parameters and output results
- **Execute Method**: Computation logic
- **Dependency Tracking**: Automatic via PropertyLink properties
- **Recomputation**: Integrated with document recomputation system

---

## 2. Declaring Inputs/Properties

### Property Declaration

Operators declare inputs and outputs through **properties** added to the document object.

#### A. Property Declaration Syntax

**Python (FeaturePython)**:
```python
# Add property to object
obj.addProperty(
    "PropertyType",      # Type of property (PropertyLink, PropertyFloat, etc.)
    "PropertyName",       # Property name (used to access: obj.PropertyName)
    "Group",              # Property group (for UI organization)
    "Description"         # Property description (tooltip)
)
```

**C++ (Built-in Operators)**:
```cpp
// In class definition
ADD_PROPERTY_TYPE(
    PropertyName,         // Property name
    DefaultValue,         // Default value
    Group,                // Property group
    App::PropertyType,    // Property type
    Description           // Property description
);
```

### Input Property Types

#### A. Numeric Inputs

**PropertyFloat**: Floating-point numeric input.

```python
class MyOperator:
    def __init__(self, obj):
        # Declare float input
        obj.addProperty("App::PropertyFloat", "Length", "Inputs", "Length parameter")
        obj.Length = 10.0  # Default value
```

**PropertyLength**: Length measurement (extends PropertyFloat with units).

```python
obj.addProperty("App::PropertyLength", "Width", "Inputs", "Width parameter")
obj.Width = 5.0  # Millimeters (default unit)
obj.Width = "1 cm"  # Unit conversion supported
```

**PropertyAngle**: Angle measurement (extends PropertyFloat with units).

```python
obj.addProperty("App::PropertyAngle", "Rotation", "Inputs", "Rotation angle")
obj.Rotation = 45.0  # Degrees (default)
obj.Rotation = "90 deg"  # Explicit unit
```

**PropertyInteger**: Integer numeric input.

```python
obj.addProperty("App::PropertyInteger", "Count", "Inputs", "Count parameter")
obj.Count = 5  # Integer value
```

#### B. Reference Inputs (Dependencies)

**PropertyLink**: Reference to a single DocumentObject.

```python
class FuseOperator:
    def __init__(self, obj):
        # Declare object reference input
        obj.addProperty("App::PropertyLink", "Base", "Inputs", "Base object")
        obj.addProperty("App::PropertyLink", "Tool", "Inputs", "Tool object")
        
        # Type constraint: Base and Tool must be Part::Feature
        # (enforced at runtime, not compile-time)
```

**PropertyLinkList**: Reference to multiple DocumentObjects.

```python
class MultiFuseOperator:
    def __init__(self, obj):
        # Declare multiple object references
        obj.addProperty("App::PropertyLinkList", "Objects", "Inputs", "Objects to fuse")
        obj.Objects = []  # Default: empty list
```

**PropertyLinkSub**: Reference to a sub-element of a DocumentObject.

```python
class FilletOperator:
    def __init__(self, obj):
        # Declare sub-element reference
        obj.addProperty("App::PropertyLinkSub", "Edge", "Inputs", "Edge to fillet")
        # Edge = (DocumentObject, "Edge1") - tuple of object and sub-element name
```

**PropertyLinkSubList**: Reference to multiple sub-elements.

```python
class MultiFilletOperator:
    def __init__(self, obj):
        # Declare multiple sub-element references
        obj.addProperty("App::PropertyLinkSubList", "Edges", "Inputs", "Edges to fillet")
        # Edges = [(obj1, "Edge1"), (obj2, "Edge2")] - list of tuples
```

#### C. Geometric Inputs

**PropertyVector**: 3D vector input.

```python
obj.addProperty("App::PropertyVector", "Position", "Inputs", "Position vector")
obj.Position = App.Vector(10, 20, 30)
```

**PropertyPlacement**: Position and orientation input.

```python
obj.addProperty("App::PropertyPlacement", "Placement", "Inputs", "Object placement")
obj.Placement = App.Placement(
    App.Vector(10, 0, 0),
    App.Rotation(App.Vector(0, 0, 1), 45)
)
```

#### D. Other Input Types

**PropertyString**: Text string input.

```python
obj.addProperty("App::PropertyString", "Name", "Inputs", "Object name")
obj.Name = "MyObject"
```

**PropertyBool**: Boolean input.

```python
obj.addProperty("App::PropertyBool", "Enabled", "Inputs", "Enable operation")
obj.Enabled = True
```

**PropertyEnumeration**: Enumeration input.

```python
obj.addProperty("App::PropertyEnumeration", "Mode", "Inputs", "Operation mode")
obj.Mode = ["Mode1", "Mode2", "Mode3"]  # Define options
obj.Mode = "Mode1"  # Set value
```

### Output Property Types

**Shape Property**: Computed geometric result (read-only, computed in `execute()`).

```python
class MyOperator:
    def execute(self, obj):
        # Compute shape
        import Part
        obj.Shape = Part.makeBox(obj.Length, obj.Width, obj.Height)
        # Shape is a special property (not explicitly declared)
```

**Note**: `Shape` is a special property for geometric operators. It's not declared with `addProperty()` but is set in `execute()`.

---

## 3. Type Constraints Representation

### Property Types as Type Constraints

In FreeCAD, **property types themselves represent type constraints**. Each property type enforces what values can be assigned to it.

#### A. Reference Type Constraints

**PropertyLink Constraint**: Must be a DocumentObject.

```python
class FuseOperator:
    def __init__(self, obj):
        obj.addProperty("App::PropertyLink", "Base", "Inputs", "Base object")
        # Type constraint: Base must be DocumentObject (or None)
        # Runtime validation: obj.Base = "Box" → TypeError
```

**PropertyLinkSub Constraint**: Must be (DocumentObject, sub-element_name) tuple.

```python
class FilletOperator:
    def __init__(self, obj):
        obj.addProperty("App::PropertyLinkSub", "Edge", "Inputs", "Edge to fillet")
        # Type constraint: Edge must be (DocumentObject, string) tuple
        # Runtime validation: obj.Edge = box → TypeError (needs tuple)
```

#### B. Numeric Type Constraints

**PropertyFloat Constraint**: Must be floating-point number.

```python
obj.addProperty("App::PropertyFloat", "Length", "Inputs", "Length")
# Type constraint: Length must be float (or convertible to float)
# Runtime validation: obj.Length = "10" → TypeError
```

**PropertyInteger Constraint**: Must be integer.

```python
obj.addProperty("App::PropertyInteger", "Count", "Inputs", "Count")
# Type constraint: Count must be integer
# Runtime validation: obj.Count = 10.5 → TypeError
```

#### C. Geometric Type Constraints

**PropertyVector Constraint**: Must be 3-component vector.

```python
obj.addProperty("App::PropertyVector", "Position", "Inputs", "Position")
# Type constraint: Position must be Vector or (x, y, z) tuple/list
# Runtime validation: obj.Position = 10 → TypeError (scalar)
# Runtime validation: obj.Position = (10, 20) → ValueError (needs 3 components)
```

### Runtime Type Validation

Type constraints are enforced at **runtime** when properties are assigned:

```python
# Property declaration creates type constraint
obj.addProperty("App::PropertyLink", "Base", "Inputs", "Base object")

# Type validation happens at assignment time
box = doc.addObject("Part::Box", "Box")

obj.Base = box      # ✅ Valid: DocumentObject assigned to PropertyLink
obj.Base = "Box"    # ❌ TypeError: string not accepted
obj.Base = 42       # ❌ TypeError: integer not accepted
obj.Base = box.Shape  # ❌ TypeError: Shape not accepted
```

### Additional Type Constraints

#### A. Object Type Constraints (Runtime Checks)

While property types enforce basic types, operators can add **additional constraints** on object types:

```python
class PartDesignPad:
    def __init__(self, obj):
        obj.addProperty("App::PropertyLink", "Profile", "Inputs", "Profile sketch")
        # Additional constraint: Profile must be Sketcher::SketchObject
        # (enforced in onChanged() or execute())
    
    def onChanged(self, obj, prop):
        if prop == "Profile" and obj.Profile:
            # Runtime type check
            if not obj.Profile.isDerivedFrom("Sketcher::SketchObject"):
                App.Console.PrintError("Profile must be a sketch\n")
                obj.Profile = None  # Clear invalid reference
```

#### B. Value Range Constraints

Operators can constrain numeric values to ranges:

```python
class MyOperator:
    def __init__(self, obj):
        obj.addProperty("App::PropertyFloat", "Length", "Inputs", "Length")
        obj.Length = 10.0
    
    def onChanged(self, obj, prop):
        if prop == "Length":
            # Value range constraint
            if obj.Length <= 0:
                App.Console.PrintError("Length must be positive\n")
                obj.Length = 1.0  # Reset to valid value
            if obj.Length > 100:
                App.Console.PrintError("Length must be <= 100\n")
                obj.Length = 100  # Clamp to maximum
```

#### C. Dependency Constraints

Operators can constrain dependencies (e.g., must have at least one input):

```python
class FuseOperator:
    def execute(self, obj):
        # Dependency constraint: Base and Tool must be set
        if not obj.Base:
            App.Console.PrintError("Base must be set\n")
            return
        if not obj.Tool:
            App.Console.PrintError("Tool must be set\n")
            return
        
        # Proceed with computation
        obj.Shape = obj.Base.Shape.fuse(obj.Tool.Shape)
```

---

## 4. Operator Examples

### Example 1: Simple Primitive Operator (Box)

**Type**: `Part::Box`

**Inputs Declared**:
```python
# C++ implementation (conceptual)
class Box : public Part::Feature {
    ADD_PROPERTY_TYPE(Length, 10.0, "Box", App::PropertyLength, "Length");
    ADD_PROPERTY_TYPE(Width, 10.0, "Box", App::PropertyLength, "Width");
    ADD_PROPERTY_TYPE(Height, 10.0, "Box", App::PropertyLength, "Height");
    ADD_PROPERTY_TYPE(Placement, App::Placement(), "Box", App::PropertyPlacement, "Placement");
};
```

**Type Constraints**:
- `Length`: PropertyLength (float with units)
- `Width`: PropertyLength (float with units)
- `Height`: PropertyLength (float with units)
- `Placement`: PropertyPlacement (position and orientation)

**Output**:
- `Shape`: Computed box geometry (read-only)

### Example 2: Boolean Operator (Fuse)

**Type**: `Part::Fuse`

**Inputs Declared**:
```python
# C++ implementation (conceptual)
class Fuse : public Part::Feature {
    ADD_PROPERTY_TYPE(Base, nullptr, "Fuse", App::PropertyLink, "Base object");
    ADD_PROPERTY_TYPE(Tool, nullptr, "Fuse", App::PropertyLink, "Tool object");
};
```

**Type Constraints**:
- `Base`: PropertyLink (must be DocumentObject, typically Part::Feature)
- `Tool`: PropertyLink (must be DocumentObject, typically Part::Feature)

**Additional Constraints** (runtime):
- Base and Tool must have `Shape` property
- Base and Tool must be valid geometric objects

**Output**:
- `Shape`: Computed union geometry (read-only)

### Example 3: PartDesign Feature (Pad)

**Type**: `PartDesign::Pad`

**Inputs Declared**:
```python
# C++ implementation (conceptual)
class Pad : public PartDesign::Feature {
    ADD_PROPERTY_TYPE(Profile, nullptr, "Pad", App::PropertyLink, "Profile sketch");
    ADD_PROPERTY_TYPE(Length, 10.0, "Pad", App::PropertyLength, "Extrusion length");
    ADD_PROPERTY_TYPE(Length2, 0.0, "Pad", App::PropertyLength, "Second length");
    ADD_PROPERTY_TYPE(Type, 0, "Pad", App::PropertyEnumeration, "Extrusion type");
    ADD_PROPERTY_TYPE(UpToFace, nullptr, "Pad", App::PropertyLinkSub, "Up to face");
    // ... more properties
};
```

**Type Constraints**:
- `Profile`: PropertyLink (must be Sketcher::SketchObject)
- `Length`: PropertyLength (float with units)
- `Length2`: PropertyLength (float with units)
- `Type`: PropertyEnumeration (enumeration value)
- `UpToFace`: PropertyLinkSub (must be (DocumentObject, face_name) tuple)

**Additional Constraints** (runtime):
- Profile must be Sketcher::SketchObject
- Profile must be in same Body
- Length must be positive

**Output**:
- `Shape`: Computed extruded geometry (read-only)

### Example 4: Python Feature Operator

**Type**: `App::FeaturePython`

**Inputs Declared**:
```python
class MyCustomOperator:
    def __init__(self, obj):
        obj.Proxy = self
        
        # Declare inputs
        obj.addProperty("App::PropertyLink", "Base", "Inputs", "Base object")
        obj.addProperty("App::PropertyFloat", "Offset", "Inputs", "Offset distance")
        obj.addProperty("App::PropertyBool", "Invert", "Inputs", "Invert direction")
        
        # Set defaults
        obj.Offset = 1.0
        obj.Invert = False
    
    def execute(self, obj):
        # Read inputs
        if not obj.Base or not hasattr(obj.Base, "Shape"):
            return
        
        base_shape = obj.Base.Shape
        offset = obj.Offset
        
        # Compute output
        import Part
        if obj.Invert:
            offset = -offset
        
        obj.Shape = base_shape.makeOffsetShape(offset, 0.1)
```

**Type Constraints**:
- `Base`: PropertyLink (must be DocumentObject with Shape)
- `Offset`: PropertyFloat (must be float)
- `Invert`: PropertyBool (must be boolean)

**Output**:
- `Shape`: Computed offset geometry (read-only)

---

## 5. Property Declaration Patterns

### Pattern 1: Single Object Reference

**Use Case**: Operator takes one input object.

```python
class MyOperator:
    def __init__(self, obj):
        obj.addProperty("App::PropertyLink", "Input", "Inputs", "Input object")
```

**Type Constraint**: `Input` must be DocumentObject (or None).

### Pattern 2: Multiple Object References

**Use Case**: Operator takes multiple input objects.

```python
class MyOperator:
    def __init__(self, obj):
        obj.addProperty("App::PropertyLinkList", "Inputs", "Inputs", "Input objects")
```

**Type Constraint**: `Inputs` must be list of DocumentObjects (or empty list).

### Pattern 3: Sub-Element Reference

**Use Case**: Operator operates on specific face/edge/vertex.

```python
class MyOperator:
    def __init__(self, obj):
        obj.addProperty("App::PropertyLinkSub", "Face", "Inputs", "Face to process")
```

**Type Constraint**: `Face` must be (DocumentObject, sub_element_name) tuple.

### Pattern 4: Mixed Inputs

**Use Case**: Operator takes both object references and numeric parameters.

```python
class MyOperator:
    def __init__(self, obj):
        # Object reference
        obj.addProperty("App::PropertyLink", "Base", "Inputs", "Base object")
        
        # Numeric parameters
        obj.addProperty("App::PropertyFloat", "Factor", "Parameters", "Scale factor")
        obj.addProperty("App::PropertyBool", "Enabled", "Parameters", "Enable operation")
        
        # Defaults
        obj.Factor = 1.0
        obj.Enabled = True
```

### Pattern 5: Optional Inputs

**Use Case**: Operator has optional inputs.

```python
class MyOperator:
    def __init__(self, obj):
        # Required input
        obj.addProperty("App::PropertyLink", "Base", "Inputs", "Base object")
        
        # Optional input
        obj.addProperty("App::PropertyLink", "Tool", "Inputs", "Tool object (optional)")
        obj.Tool = None  # Default: not set
    
    def execute(self, obj):
        # Handle optional input
        if obj.Tool:
            # Use tool
            pass
        else:
            # No tool, use default behavior
            pass
```

---

## 6. Type Constraint Enforcement

### Compile-Time vs Runtime

FreeCAD uses **runtime type constraints**, not compile-time:

- **Property Types**: Enforced at runtime when properties are assigned
- **Additional Constraints**: Enforced in `onChanged()` or `execute()`
- **No Static Types**: No compile-time type checking

### Type Validation Flow

```
Property Assignment
    ↓
Property Type Check (runtime)
    ↓
✅ Valid Type → Store Value
❌ Invalid Type → Raise TypeError
    ↓
(If valid) onChanged() called
    ↓
Additional Constraints Checked
    ↓
✅ Valid → Property Set
❌ Invalid → Reset/Clear Property
```

### Example: Type Constraint Enforcement

```python
class MyOperator:
    def __init__(self, obj):
        obj.addProperty("App::PropertyLink", "Base", "Inputs", "Base object")
        obj.addProperty("App::PropertyFloat", "Factor", "Parameters", "Scale factor")
    
    def onChanged(self, obj, prop):
        # Property type already validated (PropertyLink, PropertyFloat)
        # Additional constraints:
        
        if prop == "Base" and obj.Base:
            # Constraint: Base must have Shape property
            if not hasattr(obj.Base, "Shape"):
                App.Console.PrintError("Base must have Shape property\n")
                obj.Base = None
        
        if prop == "Factor":
            # Constraint: Factor must be positive
            if obj.Factor <= 0:
                App.Console.PrintError("Factor must be positive\n")
                obj.Factor = 1.0
```

---

## 7. Operator Input/Output Contract

### Input Contract

**Inputs are**:
- **Declared**: Via `addProperty()` or `ADD_PROPERTY_TYPE()`
- **Typed**: Property type enforces type constraint
- **Validated**: Additional constraints in `onChanged()`
- **Read in execute()**: Inputs are read, not modified

### Output Contract

**Outputs are**:
- **Computed**: In `execute()` method
- **Read-only**: Cannot be directly assigned (computed properties)
- **Dependent on inputs**: Outputs computed from inputs and dependencies

### Execute Method Contract

```python
def execute(self, obj):
    # ✅ Read input properties
    length = obj.Length
    base = obj.Base
    
    # ✅ Read dependencies (already computed)
    if base:
        base_shape = base.Shape  # Dependency was computed first
    
    # ✅ Compute output
    obj.Shape = compute_shape(length, base_shape)
    
    # ❌ Don't modify inputs
    # obj.Length = 20  # Don't do this!
    
    # ❌ Don't modify dependencies
    # obj.Base.Length = 20  # Don't do this!
```

---

## 8. Summary

### Operator Definition

1. **Operators are DocumentObjects**: All operators inherit from `App::DocumentObject`
2. **Properties Declare Inputs**: `addProperty()` or `ADD_PROPERTY_TYPE()` declare inputs
3. **Type Constraints via Property Types**: Property types enforce type constraints
4. **Execute Method Computes Outputs**: `execute()` reads inputs and computes outputs

### Input Declaration

- **Property Types**: PropertyLink, PropertyFloat, PropertyVector, etc.
- **Property Declaration**: `obj.addProperty("PropertyType", "Name", "Group", "Description")`
- **Default Values**: Set after declaration
- **Type Constraints**: Enforced by property type

### Type Constraints

- **Property Types**: Basic type constraints (PropertyLink = DocumentObject, etc.)
- **Runtime Validation**: Type checking happens at assignment time
- **Additional Constraints**: Object type checks, value ranges, dependency requirements
- **Validation Points**: Property assignment, `onChanged()`, `execute()`

### Key Principles

1. **Properties = Inputs**: Properties declare operator inputs
2. **Property Types = Type Constraints**: Property types enforce type safety
3. **Runtime Validation**: Type checking happens at runtime, not compile-time
4. **Execute = Computation**: `execute()` method computes outputs from inputs

---

## References

- Document Objects: `App::DocumentObject`, `App::FeaturePython`
- Property System: `App::PropertyContainer`, `addProperty()`
- Property Types: `App::PropertyLink`, `App::PropertyFloat`, `App::PropertyLinkSub`
- Operators: `Part::Feature`, `PartDesign::Feature`
- Recomputation: `DocumentObject::execute()`

