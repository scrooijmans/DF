# FreeCAD DocumentObject Architecture: The Unit of Truth

This document explains in detail how `App::DocumentObject` serves as the unit of truth in FreeCAD, how objects are identified and stored in documents, and how dependencies between objects form the authoritative model.

## Overview

FreeCAD is a parametric 3D modeler where all design elements are represented as **DocumentObjects**. These objects form a dependency graph that defines the authoritative model of the design. Understanding this architecture is crucial for working with FreeCAD programmatically or extending its functionality.

---

## 1. App::DocumentObject: The Unit of Truth

### What is DocumentObject?

`App::DocumentObject` is the **base class** for all objects in FreeCAD. Every element in a FreeCAD document—whether it's a geometric primitive (Box, Cylinder), a boolean operation (Fuse, Cut), a sketch, a feature, or a custom Python object—inherits from `DocumentObject`.

### Why DocumentObject is the Unit of Truth

1. **Single Source of Identity**: Every object in a document is a `DocumentObject` instance, providing a consistent interface and identity mechanism.

2. **Property Container**: All `DocumentObject` instances inherit from `PropertyContainer`, allowing them to store parametric properties (dimensions, positions, references, etc.).

3. **Transaction Support**: `DocumentObject` integrates with FreeCAD's transaction system, enabling undo/redo operations at the object level.

4. **Extension System**: Objects can be extended with additional functionality through the extension system (`ExtensionContainer`), while maintaining the core `DocumentObject` identity.

5. **Serialization**: All `DocumentObject` instances can be serialized to/from FreeCAD's document format (`.FCStd` files), ensuring persistence.

### DocumentObject Hierarchy

```
App::DocumentObject (base class)
├── App::GeoFeature (geometric features)
│   ├── Part::Feature (Part workbench objects)
│   ├── Sketcher::SketchObject (2D sketches)
│   └── ... (other geometric features)
├── App::FeaturePython (Python-scripted objects)
├── App::Link (references to other objects)
└── ... (other specialized object types)
```

---

## 2. Object Identification and Storage

### Identification Mechanisms

FreeCAD uses **multiple identification mechanisms** to reference objects:

#### A. Object Name (User-Facing Identifier)

**Purpose**: Human-readable identifier for objects in the document.

**Characteristics**:
- Set when creating objects: `doc.addObject("Part::Box", "MyBox")`
- Must be unique within a document
- Can be changed by the user
- Used in Python API: `doc.getObject("MyBox")`

**Example**:
```python
import FreeCAD as App

doc = App.newDocument()
box = doc.addObject("Part::Box", "MyBox")  # "MyBox" is the name
box.Length = 10

# Retrieve by name
retrieved_box = doc.getObject("MyBox")
```

#### B. Internal Name (Stable Identifier)

**Purpose**: Stable identifier that persists even when the user-facing name changes.

**Characteristics**:
- Automatically generated (typically based on object type + counter)
- Used internally for dependency references
- More stable than user-facing names
- Critical for maintaining dependency relationships

**Example**:
```python
# Internal name might be "Box001" even if user name is "MyBox"
# Dependencies reference internal names for stability
```

#### C. Document Object Reference

**Purpose**: Direct Python object reference.

**Characteristics**:
- Python object identity
- Used in scripts and automation
- Not serialized (only valid in current session)

**Example**:
```python
box = doc.addObject("Part::Box", "Box")
fusion = doc.addObject("Part::Fuse", "Fusion")
fusion.Base = box  # Direct object reference
```

### Storage in Documents

#### Document Structure

A FreeCAD `Document` is a container that manages all `DocumentObject` instances:

```python
# Document structure (conceptual)
class Document:
    objects: Dict[str, DocumentObject]  # Name -> Object mapping
    activeObject: DocumentObject | None
    transactions: List[Transaction]  # Undo/redo stack
```

#### Object Storage Mechanism

1. **Registration**: When an object is created, it's registered in the document:
   ```python
   doc.addObject("Part::Box", "Box")  # Registers object in document
   ```

2. **Name Management**: The document ensures name uniqueness:
   - If a name conflict occurs, FreeCAD automatically appends a number
   - Example: "Box", "Box001", "Box002" if "Box" already exists

3. **Persistence**: Objects are serialized to `.FCStd` files:
   - XML-based format (ZIP archive)
   - Contains object properties, dependencies, and metadata
   - Preserves the complete dependency graph

#### Accessing Objects

```python
# Multiple ways to access objects:

# 1. By name (most common)
box = doc.getObject("Box")

# 2. From active document
doc = App.activeDocument()
box = doc.getObject("Box")

# 3. List all objects
all_objects = doc.Objects  # Returns list of all DocumentObjects

# 4. Iterate through objects
for obj in doc.Objects:
    print(f"{obj.Name}: {obj.TypeId}")
```

---

## 3. Dependencies: The Authoritative Model

### Dependency Graph Architecture

FreeCAD's parametric modeling relies on a **directed acyclic graph (DAG)** of dependencies. This graph defines the authoritative model—the single source of truth for how objects relate to each other.

### Dependency Properties

Every `DocumentObject` has two key properties that define its position in the dependency graph:

#### A. OutList (Outgoing Dependencies)

**Definition**: List of objects that **this object depends on**.

**Meaning**: "I need these objects to compute my result."

**Example**:
```python
box = doc.addObject("Part::Box", "Box")
cylinder = doc.addObject("Part::Cylinder", "Cylinder")

fusion = doc.addObject("Part::Fuse", "Fusion")
fusion.Base = box      # box is in fusion's OutList
fusion.Tool = cylinder # cylinder is in fusion's OutList

# fusion.OutList = [box, cylinder]
```

#### B. InList (Incoming Dependencies)

**Definition**: List of objects that **depend on this object**.

**Meaning**: "These objects need me to compute their results."

**Example**:
```python
# Continuing from above:
# box.InList = [fusion]      # fusion depends on box
# cylinder.InList = [fusion] # fusion depends on cylinder
```

### How Dependencies Form the Authoritative Model

#### 1. Dependency Declaration

Dependencies are declared through **property assignments**:

```python
# Create base objects
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
pad = doc.addObject("PartDesign::Pad", "Pad")

# Declare dependency: Pad depends on Sketch
pad.Profile = sketch  # This creates the dependency

# The dependency graph now shows:
# sketch -> pad
```

#### 2. Automatic Dependency Tracking

FreeCAD automatically tracks dependencies when properties reference other objects:

```python
# Property assignment automatically creates dependency
fusion.Base = box  # FreeCAD records: fusion depends on box

# This is tracked internally, so:
assert box in fusion.OutList
assert fusion in box.InList
```

#### 3. Recomputation Order

The dependency graph determines the **recomputation order**:

```python
# FreeCAD uses topological sort to determine order
doc.recompute()  # Recomputes objects in dependency order

# Order: box -> cylinder -> fusion
# (base objects first, then dependent objects)
```

#### 4. Topological Sort

FreeCAD provides `topologicalSort()` to get objects in dependency order:

```python
# Get objects sorted by dependencies
sorted_objects = doc.topologicalSort()

# Returns objects in order: [box, cylinder, fusion]
# Ensures dependencies are computed before dependents
```

### Dependency Graph Example

Consider a complex model:

```python
# Create objects
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
pad = doc.addObject("PartDesign::Pad", "Pad")
chamfer = doc.addObject("PartDesign::Chamfer", "Chamfer")
mirror = doc.addObject("PartDesign::Mirrored", "Mirror")

# Establish dependencies
pad.Profile = sketch           # pad depends on sketch
chamfer.Base = pad             # chamfer depends on pad
mirror.Origin = chamfer        # mirror depends on chamfer

# Dependency graph:
# sketch -> pad -> chamfer -> mirror
```

**Dependency Properties**:
- `sketch.OutList = [pad]`
- `sketch.InList = []`
- `pad.OutList = [chamfer]`
- `pad.InList = [sketch]`
- `chamfer.OutList = [mirror]`
- `chamfer.InList = [pad]`
- `mirror.OutList = []`
- `mirror.InList = [chamfer]`

### Why This Forms the Authoritative Model

1. **Single Source of Truth**: The dependency graph is the **only** source of truth for object relationships. No separate relationship table or metadata is needed.

2. **Automatic Consistency**: When an object changes, FreeCAD automatically knows which objects need recomputation by traversing `InList`.

3. **Parametric Updates**: Changing a base object (e.g., `sketch`) automatically triggers recomputation of all dependent objects (`pad`, `chamfer`, `mirror`).

4. **Serialization**: The dependency graph is preserved in `.FCStd` files, ensuring the model can be reconstructed exactly.

5. **Validation**: FreeCAD can detect circular dependencies and prevent invalid models.

### Circular Dependency Detection

FreeCAD prevents circular dependencies:

```python
# This would create a cycle (invalid):
obj1.Reference = obj2
obj2.Reference = obj1  # Error: Circular dependency detected
```

The dependency graph must remain **acyclic** (DAG) for the model to be valid.

---

## 4. Transaction System and Object State

### Transactions

FreeCAD uses a transaction system to manage object state changes:

```python
# Open transaction
doc.openTransaction("Modify Box")

try:
    box.Length = 50
    box.Width = 30
    doc.commitTransaction()
except Exception:
    doc.abortTransaction()
```

**How it works**:
- All property changes within a transaction are grouped
- The transaction system tracks `DocumentObject` state changes
- Undo/redo operates at the transaction level

### Object State Management

Each `DocumentObject` tracks its state:

- **Touched**: Object has pending changes (needs recomputation)
- **Valid**: Object is up-to-date
- **Recomputing**: Object is currently being recomputed

```python
# Mark object for recomputation
box.touch()  # Marks box as modified
doc.recompute()  # Recomputes touched objects in dependency order
```

---

## 5. Practical Implications

### For Scripting

When scripting FreeCAD, always work with `DocumentObject` references:

```python
# ✅ Good: Use object references
box = doc.addObject("Part::Box", "Box")
fusion = doc.addObject("Part::Fuse", "Fusion")
fusion.Base = box  # Direct reference

# ❌ Avoid: Relying only on names (fragile)
box_name = "Box"
fusion.Base = doc.getObject(box_name)  # Can fail if name changes
```

### For Extensions

When creating custom objects, inherit from `DocumentObject`:

```python
class MyCustomObject(App.DocumentObject):
    def __init__(self, obj):
        super().__init__(obj)
        obj.addProperty("App::PropertyLength", "Length", "MyGroup", "Length description")
    
    def execute(self, obj):
        # Recompute logic here
        pass
```

### For Dependency Management

Always establish dependencies explicitly:

```python
# ✅ Good: Clear dependency chain
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
pad = doc.addObject("PartDesign::Pad", "Pad")
pad.Profile = sketch  # Explicit dependency

# ❌ Bad: Missing dependency (pad won't update if sketch changes)
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
pad = doc.addObject("PartDesign::Pad", "Pad")
# Missing: pad.Profile = sketch
```

---

## 6. Summary

### Key Concepts

1. **DocumentObject is the Unit of Truth**: Every object in FreeCAD is a `DocumentObject`, providing consistent identity, properties, and behavior.

2. **Identification**: Objects are identified by:
   - **Name**: User-facing identifier (can change)
   - **Internal Name**: Stable identifier for dependencies
   - **Object Reference**: Python object identity

3. **Storage**: Objects are stored in `Document` containers, registered by name, and serialized to `.FCStd` files.

4. **Dependencies Form the Model**: The `InList`/`OutList` properties create a DAG that defines:
   - Which objects depend on which
   - Recomputation order
   - The authoritative model structure

5. **Automatic Management**: FreeCAD automatically:
   - Tracks dependencies when properties are assigned
   - Determines recomputation order via topological sort
   - Prevents circular dependencies
   - Maintains consistency across the model

### The Authoritative Model

The **authoritative model** in FreeCAD is the **dependency graph** formed by `DocumentObject` instances and their `InList`/`OutList` relationships. This graph:

- Defines what the model **is** (structure)
- Determines how it **updates** (recomputation order)
- Preserves **consistency** (single source of truth)
- Enables **parametrics** (automatic updates)

No separate metadata, relationship tables, or external state is needed—the dependency graph **is** the model.

---

## References

- FreeCAD Python API: `App::DocumentObject`
- FreeCAD Document Management: `App::Document`
- Dependency Properties: `InList`, `OutList`
- Recomputation: `doc.recompute()`, `obj.touch()`
- Topological Sort: `doc.topologicalSort()`

