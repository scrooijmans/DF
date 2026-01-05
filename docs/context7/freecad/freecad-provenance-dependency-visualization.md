# FreeCAD Object Provenance and Dependency Visualization

This document explains in detail how FreeCAD exposes object provenance and dependency relationships to users, and how this supports understanding "where geometry came from."

## Overview

FreeCAD's parametric modeling system maintains a complete dependency graph of all objects and their relationships. FreeCAD provides multiple user-facing tools and mechanisms to expose this information, helping users understand the provenance (origin) of geometry and trace dependencies throughout their models.

---

## 1. Dependency Graph Visualization Tool

### Dependency Graph Tool

FreeCAD provides a **Dependency Graph** tool that visually represents all dependencies in the active document.

**Access**: `Tools → Dependency graph...` (or `Std_DependencyGraph` command)

**What It Shows**:
- All objects in the document
- Dependency relationships (arrows showing dependencies)
- Object creation order (reverse chronological: newest at top, oldest at bottom)
- Circular dependency detection (highlights problematic cycles)

### Dependency Graph Display

```
Dependency Graph Visualization:

    [Fusion]  ← Newest object
      ↑ ↑
      │ └─── [Cylinder]
      │
      └────── [Box]
      
    [Pad]
      ↑
      └────── [Sketch]  ← Oldest object
```

**Key Features**:
- **Arrows**: Show dependency direction (arrow points from dependency to dependent)
- **Object Boxes**: Each object is represented as a box with its name/type
- **Color Coding**: Different colors for different object types
- **Layout**: Objects arranged to show dependency flow

### Using Dependency Graph

**Example Workflow**:
```python
# Create objects with dependencies
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
pad = doc.addObject("PartDesign::Pad", "Pad")
chamfer = doc.addObject("PartDesign::Chamfer", "Chamfer")

pad.Profile = sketch      # pad depends on sketch
chamfer.Base = pad        # chamfer depends on pad

# Open Dependency Graph: Tools → Dependency graph...
# Shows:
#   Sketch → Pad → Chamfer
```

**Benefits**:
- **Visual Understanding**: See all dependencies at a glance
- **Troubleshooting**: Identify circular dependencies or missing references
- **Documentation**: Understand model structure
- **Navigation**: Click objects to select them in the document

### Dependency Graph Export

The dependency graph can be exported as:
- **Image**: PNG, SVG, PDF
- **Graphviz DOT**: For further processing
- **Text**: Dependency list

---

## 2. Tree View: Hierarchical Dependency Display

### Tree View Structure

The **Tree View** (left panel) shows objects in reverse chronological order, but also reveals dependency relationships through object hierarchy.

**Display Order**:
- Newest objects at top
- Oldest objects at bottom
- Dependencies visible through object structure

### Tree View Features

**Object Hierarchy**:
```
Tree View:
├── Chamfer (newest)
├── Pad
├── Sketch (oldest)
└── Box
```

**Dependency Indicators**:
- Objects that depend on others may show visual indicators
- Selection highlights show relationships
- Context menu shows dependency information

### Tree View Context Menu

Right-clicking an object in tree view provides dependency information:

**Menu Options**:
- **Go to Selection**: Select objects that depend on this one
- **Select Dependencies**: Select objects this object depends on
- **Show Dependencies**: Highlight dependency chain
- **Information**: Show object details including dependencies

### Tree View Selection Synchronization

Selecting an object in tree view:
- Highlights object in 3D view
- Shows properties in property editor
- Can reveal dependencies through highlighting

---

## 3. Property Editor: Dependency Properties

### InList and OutList Properties

The **Property Editor** exposes dependency information through two key properties:

#### A. OutList Property

**Definition**: Objects that **this object depends on**.

**Display**: Read-only property showing list of object references.

**Example**:
```python
fusion = doc.addObject("Part::Fuse", "Fusion")
fusion.Base = box
fusion.Tool = cylinder

# Property Editor shows:
# OutList: [Box, Cylinder]
# (Objects fusion depends on)
```

**User Access**:
1. Select object in tree view
2. Property Editor shows `OutList` property
3. Click to expand and see dependent objects
4. Double-click object in list to select it

#### B. InList Property

**Definition**: Objects that **depend on this object**.

**Display**: Read-only property showing list of object references.

**Example**:
```python
# Continuing from above:
# Property Editor for Box shows:
# InList: [Fusion]
# (Objects that depend on Box)
```

**User Access**:
1. Select object in tree view
2. Property Editor shows `InList` property
3. See which objects use this object
4. Navigate to dependent objects

### Property Editor Dependency Navigation

**Navigation Features**:
- **Click Object in List**: Selects that object
- **Expand List**: See all dependencies
- **Context Menu**: Options to navigate to dependencies

**Example Workflow**:
```
1. User selects "Fusion" object
2. Property Editor shows:
   - OutList: [Box, Cylinder]
   - InList: [] (nothing depends on Fusion)
3. User clicks "Box" in OutList
4. Box is selected, Property Editor updates
5. Box's InList shows: [Fusion]
6. User can trace: Fusion → Box → (see what Box depends on)
```

---

## 4. Expression Engine: Explicit Dependency Tracking

### Expression-Based Dependencies

FreeCAD's **Expression Engine** allows users to create explicit dependencies through mathematical expressions.

**How It Works**:
- Properties can reference other object properties via expressions
- Expressions create explicit dependencies
- Dependencies are visible in property editor

### Expression Syntax

**Basic Expression**:
```python
# In Property Editor for Pad.Length:
# Expression: Sketch.Constraints.Length * 2
# This creates explicit dependency: Pad depends on Sketch
```

**Object Identifier**:
- `ObjectName.PropertyName`: References a property
- `ObjectName.PropertyName.SubProperty`: References sub-properties
- `ObjectName.PropertyName[index]`: References array elements

### Expression Dependencies in Property Editor

**Display**:
- Properties with expressions show expression icon
- Click to edit expression
- See referenced objects in expression
- Dependencies are automatically tracked

**Example**:
```python
# Create objects
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
pad = doc.addObject("PartDesign::Pad", "Pad")

# Set expression for pad length
pad.Length = "Sketch.Constraints.Length * 2"

# Property Editor shows:
# Length: [expression icon] Sketch.Constraints.Length * 2
# OutList: [Sketch] (dependency from expression)
```

### Expression Browser

FreeCAD provides an **Expression Browser** to:
- View all expressions in document
- See expression dependencies
- Edit expressions
- Navigate to referenced objects

**Access**: Right-click property → "Expression..." → Expression Browser

---

## 5. Selection Tools: Dependency Navigation

### Go to Selection Command

**Command**: `Std_GoToSelection` (or context menu)

**Purpose**: Navigate to objects that depend on selected object.

**Usage**:
1. Select object in tree view or 3D view
2. Right-click → "Go to Selection"
3. FreeCAD selects all objects in `InList` (objects that depend on this)

**Example**:
```python
# Select Box
# Right-click → "Go to Selection"
# FreeCAD selects: [Fusion] (if Fusion depends on Box)
```

### Select Dependencies Command

**Command**: Select objects that selected object depends on.

**Usage**:
1. Select object
2. Right-click → "Select Dependencies"
3. FreeCAD selects all objects in `OutList` (objects this depends on)

**Example**:
```python
# Select Fusion
# Right-click → "Select Dependencies"
# FreeCAD selects: [Box, Cylinder] (objects Fusion depends on)
```

### Highlight Dependencies

Some workbenches provide dependency highlighting:
- **PartDesign**: Highlights dependency chain in 3D view
- **Part**: Shows dependency relationships visually
- **Custom Tools**: Can highlight dependencies

---

## 6. Object Information Dialog

### Information Dialog

**Access**: Right-click object → "Information" (or `Std_ViewObjectInfo`)

**What It Shows**:
- **Object Name**: Internal name
- **Label**: User-facing name
- **Type**: Object type (e.g., "Part::Box")
- **Dependencies**: Lists InList and OutList
- **Properties**: All properties and values
- **Shape Info**: Geometric information (if applicable)

### Information Dialog Dependencies Section

**Display Format**:
```
Dependencies:
  Depends on (OutList):
    - Box
    - Cylinder
  
  Used by (InList):
    - Fusion
```

**Navigation**:
- Click object names to select them
- Navigate through dependency chain
- Understand object relationships

---

## 7. Python Console: Programmatic Access

### Accessing Dependencies via Python

Users can access dependency information through the Python console:

**InList and OutList**:
```python
# Get dependencies
box = doc.getObject("Box")
fusion = doc.getObject("Fusion")

# See what box is used by
print(box.InList)  # Output: [<DocumentObject object>]

# See what fusion depends on
print(fusion.OutList)  # Output: [<DocumentObject object>, <DocumentObject object>]

# Navigate dependencies
for obj in fusion.OutList:
    print(f"Fusion depends on: {obj.Label}")
```

**Dependency Chain Traversal**:
```python
def trace_dependencies(obj, visited=None):
    """Trace all objects an object depends on"""
    if visited is None:
        visited = set()
    
    if obj in visited:
        return
    
    visited.add(obj)
    print(f"Object: {obj.Label}")
    print(f"  Depends on: {[o.Label for o in obj.OutList]}")
    
    for dep in obj.OutList:
        trace_dependencies(dep, visited)

# Use it
trace_dependencies(fusion)
```

**Topological Sort**:
```python
# Get objects in dependency order
sorted_objects = doc.topologicalSort()
for obj in sorted_objects:
    print(f"{obj.Label} (depends on: {[o.Label for o in obj.OutList]})")
```

---

## 8. Shape Binder: Geometry Provenance

### Shape Binder Tool

**Purpose**: Reference geometry from one object in another, maintaining provenance.

**How It Works**:
- Creates a reference to geometry (faces, edges, vertices)
- Maintains link to original object
- Shows provenance in property editor

**Example**:
```python
# Create base object
box = doc.addObject("Part::Box", "Box")
doc.recompute()

# Create Shape Binder
binder = doc.addObject("Part::FeaturePython", "ShapeBinder")
# ... configure binder to reference Box.Face1

# Property Editor shows:
# Source: Box
# Shape: (reference to Box geometry)
```

**Provenance Display**:
- **Source Property**: Shows original object
- **Shape Property**: Shows referenced geometry
- **Dependency**: Automatically tracked in OutList

### Shape Binder in Property Editor

**Properties**:
- `Source`: Original object (provenance)
- `Shape`: Referenced geometry
- `OutList`: Shows dependency on source object

**User Benefits**:
- **Clear Provenance**: Know where geometry came from
- **Dependency Tracking**: Automatic dependency management
- **Flexibility**: Can change source without breaking references

---

## 9. App Link: Object Provenance

### App Link Feature

**Purpose**: Create lightweight references to objects, sharing geometry and properties.

**How It Works**:
- Links to original object
- Shares same geometry
- Maintains provenance information

**Example**:
```python
# Create original object
box = doc.addObject("Part::Box", "Box")

# Create link
link = doc.addObject("App::Link", "BoxLink")
link.LinkedObject = box

# Property Editor shows:
# LinkedObject: Box (provenance)
# OutList: [Box] (dependency)
```

### App Link Provenance Display

**Properties**:
- `LinkedObject`: Original object (provenance)
- `LinkTransform`: Transformation relative to original
- `OutList`: Shows dependency on linked object

**User Benefits**:
- **Clear Origin**: Know where linked object came from
- **Dependency Tracking**: Automatic dependency management
- **Efficiency**: Share geometry without duplication

---

## 10. PartDesign Workbench: Feature History

### PartDesign Body Feature History

In **PartDesign Workbench**, features within a Body form a linear dependency chain.

**Feature Chain**:
```
Body
├── Sketch001 (base)
├── Pad001 (depends on Sketch001)
├── Sketch002 (depends on Pad001)
├── Pocket001 (depends on Sketch002)
└── Chamfer001 (depends on Pocket001)
```

### Feature History Display

**Tree View**:
- Features listed in creation order
- Dependency chain visible through hierarchy
- BaseFeature property shows dependency

**Property Editor**:
- `BaseFeature`: Previous feature in chain
- `Profile`: Sketch or face used
- `OutList`: Shows all dependencies

**Example**:
```python
# PartDesign feature chain
body = doc.addObject("PartDesign::Body", "Body")
sketch = body.newObject("Sketcher::SketchObject", "Sketch")
pad = body.newObject("PartDesign::Pad", "Pad")

pad.Profile = sketch

# Property Editor for Pad shows:
# BaseFeature: (empty, first feature)
# Profile: Sketch
# OutList: [Sketch]
```

### Feature History Navigation

**Navigation Tools**:
- **Tree View**: Click features to see chain
- **Property Editor**: See BaseFeature and Profile
- **3D View**: Visual representation of feature chain

**User Benefits**:
- **Clear History**: See how feature was created
- **Dependency Chain**: Understand feature order
- **Troubleshooting**: Identify which feature caused issue

---

## 11. Sketch Constraints: External Geometry Dependencies

### External Geometry in Sketcher

**Sketcher** allows referencing geometry from other objects (external geometry).

**How It Works**:
- Sketch can reference edges, faces, vertices from other objects
- Creates dependency on referenced object
- Provenance shown in constraint list

**Example**:
```python
# Create base object
box = doc.addObject("Part::Box", "Box")
doc.recompute()

# Create sketch
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
sketch.MapMode = "FlatFace"
sketch.AttachmentOffset = App.Placement(App.Vector(0, 0, 10), App.Rotation())

# Add external geometry reference
# (In Sketcher, use "External Geometry" tool)
# Select Box.Face1 edge
# Creates dependency: Sketch depends on Box
```

### External Geometry Display

**Constraint List**:
- External geometry constraints listed
- Show referenced object and element
- Click to select referenced object

**Property Editor**:
- `External Geometry`: List of external references
- `OutList`: Shows dependency on referenced objects

**User Benefits**:
- **Clear References**: See what external geometry is used
- **Dependency Tracking**: Automatic dependency management
- **Navigation**: Click to go to referenced object

---

## 12. Complete Provenance Example

### Scenario: Complex Model

```python
# Create objects with dependencies
sketch = doc.addObject("Sketcher::SketchObject", "BaseSketch")
pad = doc.addObject("PartDesign::Pad", "BasePad")
pad.Profile = sketch

box = doc.addObject("Part::Box", "ReferenceBox")
doc.recompute()

sketch2 = doc.addObject("Sketcher::SketchObject", "SecondSketch")
# Add external geometry reference to box
# (via Sketcher external geometry tool)

pocket = doc.addObject("PartDesign::Pocket", "Pocket")
pocket.Profile = sketch2

fusion = doc.addObject("Part::Fuse", "Fusion")
fusion.Base = pad
fusion.Tool = box
```

### Provenance Tracking

**Dependency Graph Shows**:
```
    [Fusion]
      ↑ ↑
      │ └─── [ReferenceBox]
      │
      └────── [BasePad]
                ↑
                └─── [BaseSketch]
                
    [Pocket]
      ↑
      └────── [SecondSketch]
                ↑
                └─── [ReferenceBox] (external geometry)
```

**Property Editor Shows**:

**Fusion**:
- `OutList`: [BasePad, ReferenceBox]
- User can see: Fusion depends on BasePad and ReferenceBox

**BasePad**:
- `OutList`: [BaseSketch]
- `InList`: [Fusion]
- User can see: BasePad depends on BaseSketch, used by Fusion

**SecondSketch**:
- `OutList`: [ReferenceBox] (from external geometry)
- `InList`: [Pocket]
- User can see: SecondSketch references ReferenceBox, used by Pocket

### User Workflow: Tracing Provenance

1. **User selects Fusion in tree view**
   - Property Editor shows: `OutList: [BasePad, ReferenceBox]`
   - User clicks "BasePad" in OutList
   - BasePad is selected

2. **User examines BasePad**
   - Property Editor shows: `OutList: [BaseSketch]`
   - User clicks "BaseSketch" in OutList
   - BaseSketch is selected

3. **User traces complete chain**:
   - Fusion → BasePad → BaseSketch
   - User understands: Fusion geometry comes from BasePad, which comes from BaseSketch

4. **User opens Dependency Graph**
   - Visual representation shows complete dependency structure
   - User can see all relationships at once

---

## 13. Troubleshooting with Provenance

### Circular Dependency Detection

**Problem**: Circular dependencies cause recomputation errors.

**Solution**: Use Dependency Graph to identify cycles.

**Example**:
```
Dependency Graph shows:
  ObjectA → ObjectB → ObjectC → ObjectA (cycle!)
  
FreeCAD highlights cycle in red
User can identify and fix the circular reference
```

### Missing Dependency Detection

**Problem**: Object fails to recompute because dependency is missing.

**Solution**: Check OutList property to see what object depends on.

**Example**:
```python
# Pad fails to recompute
# Property Editor shows:
# OutList: [Sketch] (but Sketch was deleted)
# User can see: Pad depends on missing Sketch
# Solution: Recreate Sketch or change Pad.Profile
```

### Understanding Change Propagation

**Problem**: Changing one object affects unexpected objects.

**Solution**: Use InList to see what depends on changed object.

**Example**:
```python
# User changes Sketch
# Wants to know what will be affected
# Property Editor for Sketch shows:
# InList: [Pad, Pocket, Chamfer]
# User can see: Changing Sketch affects Pad, Pocket, and Chamfer
```

---

## 14. Summary

### Key Tools for Provenance

1. **Dependency Graph**: Visual representation of all dependencies
2. **Property Editor**: Shows InList and OutList properties
3. **Tree View**: Hierarchical display with dependency indicators
4. **Expression Browser**: Shows expression-based dependencies
5. **Information Dialog**: Complete object information including dependencies
6. **Selection Tools**: Navigate dependencies (Go to Selection, Select Dependencies)
7. **Python Console**: Programmatic access to dependency information

### How Provenance Supports Understanding

1. **Tracing Origins**: Users can trace geometry back to its source
2. **Understanding Relationships**: See how objects are connected
3. **Troubleshooting**: Identify problems in dependency chain
4. **Change Impact**: Understand what will be affected by changes
5. **Documentation**: Dependency graph serves as model documentation

### User Benefits

- **Transparency**: Clear visibility into object relationships
- **Debugging**: Easy to identify dependency issues
- **Learning**: Understand how parametric modeling works
- **Maintenance**: Easier to modify and maintain models
- **Collaboration**: Share dependency information with others

---

## References

- Dependency Graph: `Tools → Dependency graph...` (`Std_DependencyGraph`)
- Property Editor: InList, OutList properties
- Expression Engine: Object identifiers and expressions
- Shape Binder: Geometry provenance tracking
- App Link: Object reference provenance
- PartDesign: Feature history and BaseFeature
- Sketcher: External geometry dependencies

