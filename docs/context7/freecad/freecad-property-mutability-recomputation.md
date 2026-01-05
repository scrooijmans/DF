# FreeCAD Property Mutability and Recomputation System

This document explains in detail which FreeCAD objects and properties are mutable, when mutation is allowed, and how property changes trigger recomputation instead of silent mutation of results.

## Overview

FreeCAD's parametric modeling system is built on a fundamental principle: **properties are mutable, but results are recomputed, not directly mutated**. This ensures consistency, enables undo/redo, and maintains the dependency graph integrity. Understanding this system is crucial for effective FreeCAD development and scripting.

---

## 1. Property Mutability: What Can Be Changed

### Mutable Properties

Most properties in FreeCAD are **mutable**—they can be changed at any time (subject to certain conditions). These include:

#### A. Input/Parameter Properties

**Definition**: Properties that define the object's parameters and inputs.

**Examples**:
- **Geometric Dimensions**: `Length`, `Width`, `Height` (for Box), `Radius` (for Cylinder)
- **Position/Orientation**: `Placement`, `Position`, `Rotation`
- **References**: `Base`, `Tool` (for boolean operations), `Profile` (for PartDesign features)
- **Custom Properties**: User-defined properties added via `addProperty()`

**Characteristics**:
- Can be modified directly: `box.Length = 50`
- Changes are tracked by the transaction system
- Modifications trigger recomputation (not immediate result updates)

**Example**:
```python
import FreeCAD as App

doc = App.newDocument()
box = doc.addObject("Part::Box", "Box")

# All these properties are mutable:
box.Length = 10.0      # ✅ Mutable
box.Width = 5.0        # ✅ Mutable
box.Height = 3.0       # ✅ Mutable
box.Placement = App.Placement(App.Vector(10, 0, 0), App.Rotation())

# Reference properties are also mutable:
fusion = doc.addObject("Part::Fuse", "Fusion")
fusion.Base = box      # ✅ Mutable (can change reference)
```

#### B. Object Metadata Properties

**Definition**: Properties that describe the object but don't affect geometry.

**Examples**:
- `Label`: User-facing name (different from internal `Name`)
- `Visibility`: Whether object is visible in 3D view
- `Transparency`: Visual transparency level
- `Color`: Display color

**Characteristics**:
- Can be modified without triggering recomputation
- Changes are immediate (visual updates)
- Still tracked by transaction system

**Example**:
```python
box.Label = "My Custom Box"  # ✅ Mutable (metadata)
box.Visibility = False       # ✅ Mutable (visual only)
box.Transparency = 50        # ✅ Mutable (visual only)
```

### Read-Only Properties

Some properties are **read-only**—they cannot be directly modified because they are computed from other properties or system state.

#### A. Computed/Output Properties

**Definition**: Properties that are derived from input properties and dependencies.

**Examples**:
- `Shape`: The computed geometric shape (for `Part::Feature` objects)
- `Area`, `Volume`: Computed geometric properties
- `InList`, `OutList`: Dependency lists (automatically maintained)
- `State`: Object state (Touched, Valid, Recomputing)

**Characteristics**:
- Cannot be directly assigned: `box.Shape = ...` ❌ (will fail or be ignored)
- Automatically computed during `execute()` method
- Updated only through recomputation

**Example**:
```python
box = doc.addObject("Part::Box", "Box")
box.Length = 10

# These are read-only (computed):
# box.Shape = ...  # ❌ Cannot assign directly
# box.Volume = ... # ❌ Cannot assign directly

# Must recompute to update:
doc.recompute()  # Shape and Volume are now updated
print(box.Shape)  # ✅ Can read computed value
print(box.Volume) # ✅ Can read computed value
```

#### B. System-Managed Properties

**Definition**: Properties maintained by FreeCAD's internal systems.

**Examples**:
- `Name`: Internal object name (system-managed, use `Label` for user-facing name)
- `TypeId`: Object type identifier (immutable)
- `Document`: Reference to containing document (immutable)
- `InList`, `OutList`: Dependency lists (automatically updated)

**Characteristics**:
- Managed by FreeCAD core
- Changes would break system integrity
- Read-only for user code

**Example**:
```python
box = doc.addObject("Part::Box", "Box")

# System-managed (read-only):
# box.Name = "NewName"      # ❌ Cannot change (use Label instead)
# box.TypeId = "..."        # ❌ Cannot change
# box.Document = other_doc  # ❌ Cannot change

# Use Label for user-facing name:
box.Label = "My Box"  # ✅ Mutable (user-facing name)
```

### Property Type Categories

FreeCAD provides various property types, each with different mutability characteristics:

#### Mutable Property Types

```python
# Numeric properties (mutable)
obj.addProperty("App::PropertyLength", "Length", "Group", "Description")
obj.addProperty("App::PropertyFloat", "Value", "Group", "Description")
obj.addProperty("App::PropertyInteger", "Count", "Group", "Description")

# String properties (mutable)
obj.addProperty("App::PropertyString", "Text", "Group", "Description")

# Boolean properties (mutable)
obj.addProperty("App::PropertyBool", "Enabled", "Group", "Description")

# Vector/Placement properties (mutable)
obj.addProperty("App::PropertyVector", "Position", "Group", "Description")
obj.addProperty("App::PropertyPlacement", "Placement", "Group", "Description")

# Reference properties (mutable)
obj.addProperty("App::PropertyLink", "Base", "Group", "Description")
obj.addProperty("App::PropertyLinkList", "Tools", "Group", "Description")

# Enumeration properties (mutable)
obj.addProperty("App::PropertyEnumeration", "Mode", "Group", "Description")
obj.Mode = ["Option1", "Option2", "Option3"]
```

#### Read-Only Property Types

```python
# Computed shape (read-only, computed in execute())
# No direct property type - Shape is a special attribute

# Dependency lists (read-only, system-managed)
# obj.InList   # Read-only
# obj.OutList  # Read-only
```

---

## 2. When Mutation is Allowed

### Conditions for Property Mutation

Property mutations are allowed under specific conditions:

#### A. Normal Operation (Default State)

**Condition**: Object is in normal state (not locked, not read-only).

**When Allowed**:
- During interactive editing
- In scripts and macros
- During transaction blocks
- When object is not currently being recomputed

**Example**:
```python
doc = App.newDocument()
box = doc.addObject("Part::Box", "Box")

# ✅ Mutation allowed in normal state:
box.Length = 10
box.Width = 5
box.Height = 3
```

#### B. Transaction Blocks

**Condition**: Property changes are wrapped in a transaction.

**When Allowed**:
- Inside `doc.openTransaction()` / `doc.commitTransaction()` blocks
- Allows grouping multiple changes for undo/redo
- Changes are deferred until transaction commit

**Example**:
```python
doc = App.newDocument()
box = doc.addObject("Part::Box", "Box")

# ✅ Mutation allowed in transaction:
doc.openTransaction("Modify Box")
try:
    box.Length = 50
    box.Width = 30
    box.Height = 20
    doc.commitTransaction()  # Changes are now committed
except Exception:
    doc.abortTransaction()   # Rollback on error
```

#### C. During Object Creation

**Condition**: Object is being initialized.

**When Allowed**:
- Immediately after `doc.addObject()`
- Before first `doc.recompute()`
- During `__init__()` or `execute()` methods (for custom objects)

**Example**:
```python
# ✅ Mutation allowed during creation:
box = doc.addObject("Part::Box", "Box")
box.Length = 10  # Set before first recompute
box.Width = 5
box.Height = 3

# First recompute computes the Shape
doc.recompute()
```

### Conditions That Prevent Mutation

#### A. Read-Only Properties

**Condition**: Property is marked as read-only or computed.

**Prevention**: Direct assignment is ignored or raises an error.

**Example**:
```python
box = doc.addObject("Part::Box", "Box")
doc.recompute()

# ❌ Cannot mutate read-only properties:
# box.Shape = ...      # Ignored or error
# box.InList = [...]    # Ignored or error
# box.Name = "NewName"  # Ignored or error
```

#### B. Locked Objects

**Condition**: Object is locked (rare, but possible in some workbenches).

**Prevention**: Property changes are blocked.

**Example**:
```python
# If object is locked (hypothetical):
# box.Length = 10  # ❌ Error: Object is locked
```

#### C. During Recomputation

**Condition**: Object is currently being recomputed.

**Prevention**: Property changes during `execute()` should be avoided (can cause inconsistencies).

**Best Practice**:
```python
class MyCustomObject(App.DocumentObject):
    def execute(self, obj):
        # ✅ Read properties (input)
        length = obj.Length
        
        # ✅ Compute and set output (Shape)
        obj.Shape = Part.makeBox(length, length, length)
        
        # ❌ Avoid: Changing input properties during execute()
        # obj.Length = 20  # Don't do this!
```

#### D. Invalid Dependency State

**Condition**: Dependency graph is invalid (circular dependencies, missing references).

**Prevention**: Some property assignments may be blocked to maintain graph integrity.

**Example**:
```python
# If circular dependency would be created:
# obj1.Reference = obj2
# obj2.Reference = obj1  # ❌ May be blocked to prevent cycle
```

---

## 3. How Property Changes Trigger Recomputation

### The Recomputation Pipeline

FreeCAD uses a **lazy recomputation system** where property changes mark objects for recomputation, but don't immediately update results. This ensures:

1. **Consistency**: All dependent objects are updated together
2. **Performance**: Multiple changes can be batched
3. **Correctness**: Dependency order is respected

### Step 1: Property Change Detection

When a property is modified, FreeCAD automatically detects the change:

```python
box = doc.addObject("Part::Box", "Box")
doc.recompute()  # Initial computation

# Property change is detected:
box.Length = 20  # FreeCAD detects: Length property changed
```

**What Happens**:
- Property setter records the change
- Object is marked as "touched" (needs recomputation)
- Dependent objects are also marked (via `InList` traversal)

### Step 2: Touch Mechanism

The `touch()` mechanism marks objects for recomputation:

```python
# Manual touch (explicit):
box.touch()  # Marks box as needing recomputation

# Automatic touch (on property change):
box.Length = 20  # Automatically calls touch() internally
```

**Touch Propagation**:
- When an object is touched, all objects in its `InList` are also touched
- This ensures dependent objects are recomputed when dependencies change

**Example**:
```python
box = doc.addObject("Part::Box", "Box")
cylinder = doc.addObject("Part::Cylinder", "Cylinder")
fusion = doc.addObject("Part::Fuse", "Fusion")
fusion.Base = box
fusion.Tool = cylinder

doc.recompute()  # Initial computation

# Changing box touches box and fusion:
box.Length = 30  # Touches: box, fusion (via InList)

# State after touch:
# box: Touched (needs recomputation)
# cylinder: Valid (unchanged)
# fusion: Touched (depends on box)
```

### Step 3: Recomputation Trigger

Recomputation can be triggered manually or automatically:

#### A. Manual Recomputation

**Explicit Trigger**: User or script calls `doc.recompute()`

```python
# Manual recomputation:
box.Length = 20
doc.recompute()  # Explicitly triggers recomputation
```

**When to Use**:
- After making multiple property changes
- In scripts where you want to control when recomputation happens
- For performance (batch multiple changes before recomputing)

**Example**:
```python
# Efficient: Batch changes, then recompute once
box.Length = 20
box.Width = 15
box.Height = 10
# ... many more changes ...
doc.recompute()  # Single recomputation for all changes
```

#### B. Automatic Recomputation

**Automatic Trigger**: FreeCAD may automatically recompute in certain situations:

- After certain GUI operations
- When document is saved
- When dependency graph changes significantly

**Note**: Automatic recomputation is not guaranteed—always call `doc.recompute()` explicitly in scripts.

### Step 4: Topological Sort and Execution Order

When `doc.recompute()` is called, FreeCAD:

1. **Identifies Touched Objects**: Finds all objects marked for recomputation
2. **Topological Sort**: Orders objects by dependency (dependencies before dependents)
3. **Executes in Order**: Calls `execute()` method on each object in sorted order

**Example**:
```python
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
pad = doc.addObject("PartDesign::Pad", "Pad")
chamfer = doc.addObject("PartDesign::Chamfer", "Chamfer")

pad.Profile = sketch
chamfer.Base = pad

# Dependency order: sketch -> pad -> chamfer

# Change sketch:
sketch.addGeometry(...)  # Touches: sketch, pad, chamfer

# Recomputation order (topological sort):
doc.recompute()
# 1. execute() on sketch (no dependencies)
# 2. execute() on pad (depends on sketch)
# 3. execute() on chamfer (depends on pad)
```

### Step 5: Execute Method and Result Computation

For each object in recomputation order, FreeCAD calls the `execute()` method:

```python
class MyCustomObject(App.DocumentObject):
    def execute(self, obj):
        # ✅ Read input properties:
        length = obj.Length
        width = obj.Width
        height = obj.Height
        
        # ✅ Read dependency objects:
        if obj.Base:
            base_shape = obj.Base.Shape  # Read computed shape from dependency
        
        # ✅ Compute output (Shape):
        obj.Shape = Part.makeBox(length, width, height)
        
        # ✅ Compute other output properties:
        obj.Volume = obj.Shape.Volume  # Derived from Shape
```

**Key Points**:
- `execute()` is called during recomputation
- Input properties are read (mutable, already set)
- Dependencies are read (their `Shape` is already computed)
- Output properties (like `Shape`) are computed and assigned
- **Results are not mutated directly**—they are recomputed from inputs

### Step 6: Result Update (Not Direct Mutation)

**Critical Principle**: Results are **recomputed**, not directly mutated.

**What This Means**:
- You cannot directly assign to `obj.Shape`
- `Shape` is computed in `execute()` from input properties
- Changing `Length` doesn't mutate `Shape`—it triggers recomputation that computes a new `Shape`

**Example**:
```python
box = doc.addObject("Part::Box", "Box")
box.Length = 10
doc.recompute()  # Shape is computed from Length=10

# ❌ WRONG: Direct mutation (doesn't work):
# box.Shape = Part.makeBox(20, 20, 20)  # Ignored or error

# ✅ CORRECT: Change input, trigger recomputation:
box.Length = 20
doc.recompute()  # Shape is recomputed from Length=20
```

**Why This Matters**:
- **Consistency**: Results always match inputs
- **Dependency Tracking**: Dependent objects know when to recompute
- **Undo/Redo**: Transaction system can track input changes, not result mutations
- **Parametric**: Changing one parameter automatically updates all dependent results

---

## 4. Recomputation vs. Direct Mutation: Detailed Comparison

### Direct Mutation (What FreeCAD Does NOT Do)

**If FreeCAD used direct mutation** (hypothetical, incorrect approach):

```python
# ❌ Hypothetical direct mutation (NOT how FreeCAD works):
box = doc.addObject("Part::Box", "Box")
box.Length = 10
box.Shape = Part.makeBox(10, 10, 10)  # Direct assignment

box.Length = 20
box.Shape = Part.makeBox(20, 20, 20)  # Direct mutation of result
```

**Problems with Direct Mutation**:
- No dependency tracking (dependent objects don't know to update)
- Inconsistent state (inputs and outputs can diverge)
- No undo/redo support (can't track result mutations easily)
- Breaks parametric modeling (changing one thing doesn't update dependents)

### Recomputation (What FreeCAD Actually Does)

**How FreeCAD actually works**:

```python
# ✅ Recomputation-based (how FreeCAD works):
box = doc.addObject("Part::Box", "Box")
box.Length = 10
doc.recompute()  # execute() computes: obj.Shape = Part.makeBox(10, 10, 10)

box.Length = 20
doc.recompute()  # execute() recomputes: obj.Shape = Part.makeBox(20, 20, 20)
```

**Benefits of Recomputation**:
- **Dependency Tracking**: Changing `box.Length` touches `box` and all objects in `box.InList`
- **Consistency**: `Shape` is always computed from current input properties
- **Undo/Redo**: Transaction system tracks input property changes
- **Parametric**: Changing `box.Length` automatically triggers recomputation of dependent objects

### Complete Example: Dependency Chain

```python
import FreeCAD as App
import Part

doc = App.newDocument()

# Create dependency chain:
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
pad = doc.addObject("PartDesign::Pad", "Pad")
chamfer = doc.addObject("PartDesign::Chamfer", "Chamfer")

pad.Profile = sketch      # pad depends on sketch
chamfer.Base = pad        # chamfer depends on pad

doc.recompute()  # Initial computation

# Change sketch (input property):
sketch.addGeometry(Part.LineSegment(...))  # Modifies sketch
# FreeCAD automatically:
# 1. Touches: sketch
# 2. Touches: pad (via sketch.InList)
# 3. Touches: chamfer (via pad.InList)

# Recomputation order (topological sort):
doc.recompute()
# 1. execute() on sketch: computes sketch.Shape from geometry
# 2. execute() on pad: computes pad.Shape from sketch.Shape
# 3. execute() on chamfer: computes chamfer.Shape from pad.Shape

# Results are recomputed, not mutated:
# - sketch.Shape is recomputed from sketch geometry
# - pad.Shape is recomputed from sketch.Shape
# - chamfer.Shape is recomputed from pad.Shape
```

---

## 5. Property Change Lifecycle

### Complete Lifecycle Example

```python
import FreeCAD as App

doc = App.newDocument()
box = doc.addObject("Part::Box", "Box")

# === PHASE 1: Initial Setup ===
box.Length = 10
box.Width = 5
box.Height = 3
# State: box is "Touched" (needs recomputation)
# box.Shape: Not yet computed (or invalid)

# === PHASE 2: First Recomputation ===
doc.recompute()
# 1. FreeCAD identifies: box is touched
# 2. Topological sort: [box] (no dependencies)
# 3. Calls box.execute():
#    - Reads: Length=10, Width=5, Height=3
#    - Computes: box.Shape = Part.makeBox(10, 5, 3)
# 4. State: box is "Valid" (up-to-date)
# 5. box.Shape: Valid geometric shape

# === PHASE 3: Property Change ===
box.Length = 20
# 1. Property setter detects change
# 2. Automatically calls: box.touch()
# 3. State: box is "Touched" (needs recomputation)
# 4. box.Shape: Still exists but is now "stale" (doesn't match Length=20)

# === PHASE 4: Recomputation ===
doc.recompute()
# 1. FreeCAD identifies: box is touched
# 2. Topological sort: [box]
# 3. Calls box.execute():
#    - Reads: Length=20, Width=5, Height=3
#    - Computes: box.Shape = Part.makeBox(20, 5, 3)  # NEW shape
# 4. State: box is "Valid"
# 5. box.Shape: Updated geometric shape (recomputed, not mutated)
```

### State Transitions

```
[Object Created]
    ↓
[Touched] ←── Property Changed / touch() called
    ↓
[Recomputing] ←── doc.recompute() called
    ↓
[Valid] ←── execute() completed successfully
    ↓
[Touched] ←── Property changed again
    ...
```

---

## 6. Best Practices

### For Scripting

#### ✅ DO: Change Input Properties, Then Recompute

```python
# ✅ Good: Change inputs, then recompute
box.Length = 20
box.Width = 15
doc.recompute()  # Results are recomputed
```

#### ❌ DON'T: Try to Mutate Results Directly

```python
# ❌ Bad: Trying to mutate results
box.Shape = Part.makeBox(20, 20, 20)  # Doesn't work!
```

#### ✅ DO: Batch Changes Before Recomputation

```python
# ✅ Good: Batch changes
box.Length = 20
box.Width = 15
box.Height = 10
cylinder.Radius = 5
cylinder.Height = 10
# ... many more changes ...
doc.recompute()  # Single recomputation for all
```

#### ❌ DON'T: Recompute After Every Change

```python
# ❌ Bad: Inefficient
box.Length = 20
doc.recompute()  # Unnecessary
box.Width = 15
doc.recompute()  # Unnecessary
box.Height = 10
doc.recompute()  # Unnecessary
```

### For Custom Objects

#### ✅ DO: Compute Results in execute()

```python
class MyCustomObject(App.DocumentObject):
    def execute(self, obj):
        # ✅ Read inputs
        length = obj.Length
        width = obj.Width
        
        # ✅ Compute output
        obj.Shape = Part.makeBox(length, width, 10)
```

#### ❌ DON'T: Change Input Properties in execute()

```python
class MyCustomObject(App.DocumentObject):
    def execute(self, obj):
        # ❌ Bad: Changing inputs during recomputation
        obj.Length = 20  # Don't do this!
        obj.Shape = Part.makeBox(20, 20, 10)
```

#### ✅ DO: Read Dependencies in execute()

```python
class MyCustomObject(App.DocumentObject):
    def execute(self, obj):
        # ✅ Read dependency (already computed)
        if obj.Base:
            base_shape = obj.Base.Shape  # Dependency was recomputed first
            obj.Shape = base_shape.copy()
```

---

## 7. Summary

### Key Principles

1. **Properties are Mutable, Results are Recomputed**:
   - Input properties (Length, Width, etc.) can be changed
   - Output properties (Shape, Volume) are computed, not mutated
   - Changes trigger recomputation, not direct result updates

2. **Mutation is Allowed When**:
   - Object is in normal state
   - Property is not read-only
   - Not during recomputation (for input properties)
   - Inside transaction blocks

3. **Recomputation Pipeline**:
   - Property change → `touch()` → Mark for recomputation
   - `doc.recompute()` → Topological sort → `execute()` in order
   - `execute()` reads inputs, computes outputs
   - Results are updated through recomputation, not direct mutation

4. **Why Recomputation, Not Mutation**:
   - Maintains dependency graph integrity
   - Ensures consistency (results always match inputs)
   - Enables parametric modeling (automatic updates)
   - Supports undo/redo (tracks inputs, not results)
   - Allows batching (multiple changes, single recomputation)

### The Fundamental Rule

**In FreeCAD, you change inputs (properties), and FreeCAD recomputes outputs (results).**

You never directly mutate results like `obj.Shape`—you change `obj.Length` and let FreeCAD recompute `obj.Shape` through the `execute()` method.

This is the core of parametric modeling: **parameters drive geometry, not the other way around**.

---

## References

- FreeCAD Property System: `App::PropertyContainer`
- Recomputation: `doc.recompute()`, `obj.touch()`, `obj.execute()`
- Property Types: `App::PropertyLength`, `App::PropertyLink`, etc.
- Transaction System: `doc.openTransaction()`, `doc.commitTransaction()`
- Dependency Tracking: `obj.InList`, `obj.OutList`

