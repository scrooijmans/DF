# FreeCAD Extensibility Model: Workbenches, Commands, and Document Objects

This document explains in detail FreeCAD's extensibility model, including how workbenches, commands, and document objects are created, and how new operators are added without breaking recompute semantics.

## Overview

FreeCAD's extensibility model is built on three core components: **Workbenches**, **Commands**, and **Document Objects**. This modular architecture allows users and developers to extend FreeCAD's functionality while maintaining the integrity of the recomputation system and dependency management.

---

## 1. Workbenches: Modular Toolkits

### What is a Workbench?

A **Workbench** is a collection of tools and commands organized for a specific task domain (e.g., Part Design, Sketcher, Drafting). Workbenches provide:

- **Commands**: User actions (create box, extrude sketch, etc.)
- **Toolbars**: GUI organization
- **Menus**: Command organization
- **Document Objects**: Custom object types

### Workbench Architecture

```
Workbench
├── Commands (Gui::Command)
│   ├── Command1 (create object, modify property, etc.)
│   ├── Command2
│   └── ...
├── Document Objects (App::DocumentObject)
│   ├── CustomObjectType1
│   ├── CustomObjectType2
│   └── ...
└── Resources (icons, UI files, etc.)
```

### Creating a Custom Workbench

#### A. Workbench Structure

**Directory Structure**:
```
MyWorkbench/
├── Init.py              # Workbench initialization
├── InitGui.py           # GUI initialization (if Gui available)
├── MyCommand.py         # Command definitions
├── MyObject.py          # Document object definitions
└── Resources/           # Icons, UI files
    └── icons/
```

#### B. Workbench Initialization

**Init.py** (App module initialization):
```python
# Init.py - App module initialization
import FreeCAD as App

# Workbench metadata
App.appendImportPath("MyWorkbench")

# Register document object types
# (Document objects are registered when imported)
```

**InitGui.py** (Gui module initialization):
```python
# InitGui.py - Gui module initialization
import FreeCAD as App
import FreeCADGui as Gui

class MyWorkbench(Gui.Workbench):
    """Custom workbench class"""
    
    MenuText = "My Workbench"
    ToolTip = "A custom workbench for specialized tasks"
    Icon = "path/to/icon.svg"
    
    def Initialize(self):
        """Initialize workbench - register commands"""
        # Import commands
        from MyWorkbench import MyCommand1, MyCommand2
        
        # Register commands
        Gui.addCommand("MyWorkbench_Command1", MyCommand1())
        Gui.addCommand("MyWorkbench_Command2", MyCommand2())
        
        # Create toolbar
        self.appendToolbar("My Tools", [
            "MyWorkbench_Command1",
            "MyWorkbench_Command2"
        ])
        
        # Create menu
        self.appendMenu("My Menu", [
            "MyWorkbench_Command1",
            "MyWorkbench_Command2"
        ])
    
    def Activated(self):
        """Called when workbench is activated"""
        pass
    
    def Deactivated(self):
        """Called when workbench is deactivated"""
        pass

# Register workbench
Gui.addWorkbench(MyWorkbench())
```

#### C. Workbench Registration

**Registration Process**:
1. FreeCAD scans `Mod/` directory for workbenches
2. Loads `Init.py` (App module)
3. If GUI available, loads `InitGui.py` (Gui module)
4. Workbench appears in workbench selector

**Location**: Workbenches are placed in `FreeCAD/Mod/` directory.

---

## 2. Commands: User Actions

### What is a Command?

A **Command** encapsulates a user action (button click, menu selection, keyboard shortcut). Commands can:

- Create objects
- Modify properties
- Perform operations
- Interact with user (dialogs, selection)

### Command Architecture

Commands inherit from `Gui::Command` (C++) or implement command interface (Python).

### Creating a Custom Command

#### A. Python Command

**Basic Command Structure**:
```python
# MyCommand.py
import FreeCAD as App
import FreeCADGui as Gui

class MyCommand:
    """Custom command class"""
    
    def GetResources(self):
        """Return command resources (icon, menu text, tooltip)"""
        return {
            "Pixmap": "path/to/icon.svg",
            "MenuText": "My Command",
            "ToolTip": "Creates a custom object",
            "Accel": "C, M"  # Keyboard shortcut: Ctrl+M
        }
    
    def Activated(self):
        """Called when command is activated (button clicked)"""
        doc = App.ActiveDocument
        if not doc:
            App.Console.PrintError("No active document\n")
            return
        
        # Create object
        obj = doc.addObject("Part::FeaturePython", "MyObject")
        
        # Set up object (see Document Objects section)
        MyObject(obj)
        
        # Create view provider (if GUI available)
        if App.GuiUp:
            MyViewProvider(obj.ViewObject)
        
        # Update document
        doc.recompute()
        Gui.ActiveDocument.ActiveView.fitAll()
    
    def IsActive(self):
        """Check if command should be active (enabled)"""
        return App.ActiveDocument is not None

# Register command
Gui.addCommand("MyWorkbench_MyCommand", MyCommand())
```

#### B. Command Registration

**Registration in Workbench**:
```python
# In InitGui.py
from MyWorkbench import MyCommand

Gui.addCommand("MyWorkbench_MyCommand", MyCommand())
```

**Command ID Format**: `WorkbenchName_CommandName`

#### C. Command Resources

**Resources Dictionary**:
- `Pixmap`: Icon path
- `MenuText`: Menu item text
- `ToolTip`: Tooltip text
- `Accel`: Keyboard shortcut
- `Checkable`: Whether command is checkable (toggle)

#### D. Command Lifecycle

```
User clicks button/menu
    ↓
Command.Activated() called
    ↓
Command performs action
    ↓
(Optional) Command.IsActive() checked for state
```

---

## 3. Document Objects: Core Data Structures

### What is a Document Object?

A **Document Object** is a parametric entity in a FreeCAD document. It has:

- **Properties**: Parameters (dimensions, references, etc.)
- **Execute Method**: Recompute logic
- **Dependencies**: Relationships to other objects

### Document Object Types

#### A. Built-in Types

FreeCAD provides base classes for document objects:

- `App::DocumentObject`: Base class for all objects
- `App::GeoFeature`: Geometric features
- `Part::Feature`: Part workbench objects
- `App::FeaturePython`: Python-scripted objects

#### B. Custom Document Objects

Custom objects can be created in two ways:

1. **Python Objects** (FeaturePython): Fully Python-based
2. **C++ Objects**: Compiled extensions (advanced)

### Creating a Custom Document Object

#### A. Python Document Object (FeaturePython)

**Basic Structure**:
```python
# MyObject.py
import FreeCAD as App
import Part

class MyObject:
    """Custom document object"""
    
    def __init__(self, obj):
        """Initialize object properties"""
        obj.Proxy = self  # Store reference to this object
        
        # Add properties
        obj.addProperty("App::PropertyLength", "Length", "Dimensions", "Length of object")
        obj.addProperty("App::PropertyLength", "Width", "Dimensions", "Width of object")
        obj.addProperty("App::PropertyLength", "Height", "Dimensions", "Height of object")
        
        # Set default values
        obj.Length = 10.0
        obj.Width = 10.0
        obj.Height = 10.0
        
        # Add reference property (dependency)
        obj.addProperty("App::PropertyLink", "Base", "References", "Base object")
        
        # Mark object as needing recomputation
        obj.touch()
    
    def execute(self, obj):
        """Recompute object - called during doc.recompute()"""
        # Read input properties
        length = obj.Length
        width = obj.Width
        height = obj.Height
        
        # Read dependency (if exists)
        base_shape = None
        if obj.Base and hasattr(obj.Base, "Shape"):
            base_shape = obj.Base.Shape
        
        # Compute output (Shape)
        if base_shape:
            # Use base shape as reference
            obj.Shape = base_shape.copy()
        else:
            # Create new shape
            obj.Shape = Part.makeBox(length, width, height)
    
    def onChanged(self, obj, prop):
        """Called when property changes"""
        # Validate properties if needed
        if prop == "Length" and obj.Length <= 0:
            App.Console.PrintError("Length must be positive\n")
            obj.Length = 1.0  # Reset to valid value
    
    def __getstate__(self):
        """Serialize object (for saving)"""
        return None  # No custom state to save
    
    def __setstate__(self, state):
        """Deserialize object (for loading)"""
        pass  # No custom state to restore
```

#### B. ViewProvider (GUI Representation)

**ViewProvider Structure**:
```python
# MyViewProvider.py
import FreeCAD as App
import FreeCADGui as Gui

class MyViewProvider:
    """View provider for custom object"""
    
    def __init__(self, vobj):
        """Initialize view provider"""
        vobj.Proxy = self
    
    def getDefaultDisplayMode(self):
        """Return default display mode"""
        return "Flat Lines"
    
    def attach(self, vobj):
        """Attach to object"""
        self.Object = vobj.Object
    
    def updateData(self, obj, prop):
        """Called when object property changes"""
        if prop == "Shape":
            # Update visual representation
            pass
    
    def onChanged(self, vobj, prop):
        """Called when view property changes"""
        pass
    
    def __getstate__(self):
        """Serialize view provider"""
        return None
    
    def __setstate__(self, state):
        """Deserialize view provider"""
        pass
```

#### C. Object Registration

**Registration in Command**:
```python
# In MyCommand.py
from MyWorkbench import MyObject, MyViewProvider

class MyCommand:
    def Activated(self):
        doc = App.ActiveDocument
        obj = doc.addObject("Part::FeaturePython", "MyObject")
        
        # Set up object
        MyObject(obj)
        
        # Set up view provider
        if App.GuiUp:
            MyViewProvider(obj.ViewObject)
        
        doc.recompute()
```

---

## 4. Maintaining Recompute Semantics

### How New Operators Integrate

New operators (custom document objects) integrate with FreeCAD's recomputation system through:

1. **Property Framework**: Properties trigger recomputation
2. **Execute Method**: Objects implement `execute()` for recomputation
3. **Dependency Management**: Automatic dependency tracking
4. **Touch Mechanism**: Objects mark themselves for recomputation

### Property Framework Integration

**How It Works**:
- Custom objects use FreeCAD's property system
- Property changes automatically trigger `touch()`
- `touch()` marks object and dependents for recomputation
- No special code needed - property system handles it

**Example**:
```python
class MyObject:
    def __init__(self, obj):
        obj.addProperty("App::PropertyLength", "Length")
        obj.Length = 10.0  # Property change triggers touch()
    
    def execute(self, obj):
        # Called during recomputation
        obj.Shape = Part.makeBox(obj.Length, 10, 10)
```

**Key Point**: Property changes automatically integrate with recomputation system.

### Execute Method Contract

**Execute Method Requirements**:

1. **Read Input Properties**: Read object's own properties
2. **Read Dependencies**: Read dependency objects (already computed)
3. **Compute Output**: Compute output properties (e.g., Shape)
4. **No Side Effects**: Don't modify input properties or dependencies

**Example**:
```python
def execute(self, obj):
    # ✅ Read input properties
    length = obj.Length
    width = obj.Width
    
    # ✅ Read dependencies (already computed in topological order)
    if obj.Base:
        base_shape = obj.Base.Shape  # Dependency was computed first
    
    # ✅ Compute output
    if obj.Base:
        obj.Shape = base_shape.copy()
    else:
        obj.Shape = Part.makeBox(length, width, 10)
    
    # ❌ Don't modify input properties
    # obj.Length = 20  # Don't do this!
    
    # ❌ Don't modify dependencies
    # obj.Base.Length = 20  # Don't do this!
```

### Dependency Management Integration

**Automatic Dependency Tracking**:

When custom objects reference other objects, FreeCAD automatically tracks dependencies:

```python
class MyObject:
    def __init__(self, obj):
        obj.addProperty("App::PropertyLink", "Base")
    
    def execute(self, obj):
        if obj.Base:
            # FreeCAD automatically:
            # - Adds obj.Base to obj.OutList
            # - Adds obj to obj.Base.InList
            # - Ensures obj.Base is computed before obj
            base_shape = obj.Base.Shape
```

**Key Point**: Dependencies are tracked automatically - no manual dependency management needed.

### Touch Mechanism Integration

**How Touch Works**:

1. **Property Change**: Property setter calls `touch()`
2. **Object Marked**: Object marked as needing recomputation
3. **Dependency Propagation**: Dependents are also touched
4. **Recomputation**: `doc.recompute()` processes touched objects

**Custom Objects Automatically Integrate**:
```python
class MyObject:
    def __init__(self, obj):
        obj.addProperty("App::PropertyLength", "Length")
    
    # When user changes Length:
    # obj.Length = 20
    # → Property setter calls obj.touch()
    # → Object marked for recomputation
    # → doc.recompute() calls obj.execute()
```

**Key Point**: Touch mechanism works automatically for custom objects.

### Topological Sort Integration

**How It Works**:

1. **Dependency Graph**: FreeCAD builds graph from OutList relationships
2. **Topological Sort**: Objects sorted by dependencies
3. **Execute Order**: Objects executed in dependency order
4. **Custom Objects Included**: Custom objects automatically included in sort

**Example**:
```python
# Create objects
sketch = doc.addObject("Sketcher::SketchObject", "Sketch")
custom = doc.addObject("Part::FeaturePython", "Custom")
custom.Base = sketch  # Custom depends on Sketch

# FreeCAD automatically:
# - Tracks dependency: custom.OutList = [sketch]
# - Topological sort: [sketch, custom]
# - Execute order: sketch.execute() first, then custom.execute()
```

**Key Point**: Custom objects are automatically included in topological sort.

---

## 5. Complete Example: Custom Workbench

### Example: "MyTools" Workbench

**Directory Structure**:
```
MyTools/
├── Init.py
├── InitGui.py
├── CreateBox.py          # Command
├── MyBox.py              # Document object
└── Resources/
    └── icons/
        └── CreateBox.svg
```

**Init.py**:
```python
# Init.py
import FreeCAD as App

# Workbench metadata
App.appendImportPath("MyTools")
```

**InitGui.py**:
```python
# InitGui.py
import FreeCADGui as Gui

class MyToolsWorkbench(Gui.Workbench):
    MenuText = "My Tools"
    ToolTip = "Custom tools workbench"
    Icon = "path/to/icon.svg"
    
    def Initialize(self):
        from MyTools import CreateBox
        Gui.addCommand("MyTools_CreateBox", CreateBox.Command())
        self.appendToolbar("My Tools", ["MyTools_CreateBox"])
        self.appendMenu("My Menu", ["MyTools_CreateBox"])
    
    def Activated(self):
        pass
    
    def Deactivated(self):
        pass

Gui.addWorkbench(MyToolsWorkbench())
```

**CreateBox.py** (Command):
```python
# CreateBox.py
import FreeCAD as App
import FreeCADGui as Gui
from MyTools import MyBox

class Command:
    def GetResources(self):
        return {
            "Pixmap": "MyTools/CreateBox",
            "MenuText": "Create Box",
            "ToolTip": "Creates a custom box object"
        }
    
    def Activated(self):
        doc = App.ActiveDocument
        if not doc:
            doc = App.newDocument()
        
        obj = doc.addObject("Part::FeaturePython", "MyBox")
        MyBox.MyBox(obj)
        
        if App.GuiUp:
            from MyTools import MyBoxViewProvider
            MyBoxViewProvider.MyBoxViewProvider(obj.ViewObject)
        
        doc.recompute()
        Gui.ActiveDocument.ActiveView.fitAll()
    
    def IsActive(self):
        return True

Gui.addCommand("MyTools_CreateBox", Command())
```

**MyBox.py** (Document Object):
```python
# MyBox.py
import FreeCAD as App
import Part

class MyBox:
    def __init__(self, obj):
        obj.Proxy = self
        obj.addProperty("App::PropertyLength", "Length", "Dimensions", "Length")
        obj.addProperty("App::PropertyLength", "Width", "Dimensions", "Width")
        obj.addProperty("App::PropertyLength", "Height", "Dimensions", "Height")
        obj.Length = 10.0
        obj.Width = 10.0
        obj.Height = 10.0
        obj.touch()
    
    def execute(self, obj):
        obj.Shape = Part.makeBox(obj.Length, obj.Width, obj.Height)
    
    def onChanged(self, obj, prop):
        if prop in ["Length", "Width", "Height"]:
            if obj.Length <= 0 or obj.Width <= 0 or obj.Height <= 0:
                App.Console.PrintError("Dimensions must be positive\n")
    
    def __getstate__(self):
        return None
    
    def __setstate__(self, state):
        pass
```

**MyBoxViewProvider.py** (View Provider):
```python
# MyBoxViewProvider.py
import FreeCADGui as Gui

class MyBoxViewProvider:
    def __init__(self, vobj):
        vobj.Proxy = self
    
    def getDefaultDisplayMode(self):
        return "Flat Lines"
    
    def attach(self, vobj):
        self.Object = vobj.Object
    
    def updateData(self, obj, prop):
        pass
    
    def onChanged(self, vobj, prop):
        pass
    
    def __getstate__(self):
        return None
    
    def __setstate__(self, state):
        pass
```

### Integration with Recompute System

**How It Works**:

1. **User clicks "Create Box" command**
   - Command creates `MyBox` object
   - Object has properties: Length, Width, Height

2. **User changes Length property**
   - `obj.Length = 20` triggers property setter
   - Property setter calls `obj.touch()`
   - Object marked for recomputation

3. **Document recomputation**
   - `doc.recompute()` collects touched objects
   - Topological sort includes `MyBox` object
   - `MyBox.execute()` called in correct order

4. **Execute method runs**
   - Reads Length, Width, Height properties
   - Computes Shape from properties
   - Updates `obj.Shape` property

5. **View updates**
   - ViewProvider receives update signal
   - 3D view refreshes with new geometry

**Key Point**: Custom object integrates seamlessly with recomputation system - no special code needed.

---

## 6. Advanced Extensibility Patterns

### A. Extending Existing Objects

**Pattern**: Extend built-in objects with custom behavior.

**Example**:
```python
class EnhancedBox(Part.Box):
    def __init__(self, obj):
        # Call parent constructor
        Part.Box.__init__(self, obj)
        
        # Add custom properties
        obj.addProperty("App::PropertyBool", "ShowWireframe", "Display", "Show wireframe")
    
    def execute(self, obj):
        # Call parent execute
        Part.Box.execute(self, obj)
        
        # Add custom logic
        if obj.ShowWireframe:
            # Custom wireframe display
            pass
```

### B. Object Extensions

**Pattern**: Use extension system to add functionality.

**Example**:
```python
from App import Extension, ExtensionContainer

class MyExtension(Extension):
    def __init__(self, obj):
        Extension.__init__(self, obj)
        obj.addExtension("MyExtension", self)
    
    def customMethod(self):
        # Custom functionality
        pass
```

### C. Command Macros

**Pattern**: Create command that executes multiple operations.

**Example**:
```python
class ComplexCommand:
    def Activated(self):
        # Create multiple objects
        obj1 = doc.addObject("Part::Box", "Box1")
        obj2 = doc.addObject("Part::Box", "Box2")
        
        # Perform operations
        fusion = doc.addObject("Part::Fuse", "Fusion")
        fusion.Base = obj1
        fusion.Tool = obj2
        
        doc.recompute()
```

---

## 7. Best Practices for Extensibility

### A. Property Design

**✅ DO**:
- Use appropriate property types (PropertyLength, PropertyLink, etc.)
- Set default values
- Validate properties in `onChanged()`
- Document properties with descriptions

**❌ DON'T**:
- Modify properties in `execute()`
- Create circular dependencies
- Store computed values as input properties

### B. Execute Method Design

**✅ DO**:
- Read input properties
- Read dependencies (already computed)
- Compute output properties
- Handle missing dependencies gracefully

**❌ DON'T**:
- Modify input properties
- Modify dependencies
- Create side effects
- Assume execution order (use dependencies)

### C. Dependency Management

**✅ DO**:
- Use PropertyLink for object references
- Let FreeCAD track dependencies automatically
- Validate dependencies in `onChanged()`
- Handle missing dependencies in `execute()`

**❌ DON'T**:
- Manually manage dependency lists
- Create circular dependencies
- Assume dependencies exist without checking

### D. Error Handling

**✅ DO**:
- Validate properties in `onChanged()`
- Check dependencies exist in `execute()`
- Provide clear error messages
- Reset invalid values to defaults

**❌ DON'T**:
- Let errors propagate silently
- Crash on invalid input
- Leave object in invalid state

---

## 8. Summary

### Key Components

1. **Workbenches**: Modular collections of tools and commands
2. **Commands**: User actions (buttons, menus, shortcuts)
3. **Document Objects**: Parametric entities with properties and execute methods

### Extensibility Model

- **Modular**: Workbenches are independent modules
- **Dynamic**: Commands and objects registered at runtime
- **Integrated**: New operators automatically integrate with recomputation system

### Recompute Semantics Preservation

1. **Property Framework**: Properties automatically trigger recomputation
2. **Execute Method**: Objects implement recomputation logic
3. **Dependency Management**: Automatic dependency tracking
4. **Touch Mechanism**: Automatic recomputation marking
5. **Topological Sort**: Automatic execution order

### Benefits

- **Extensibility**: Easy to add new functionality
- **Safety**: Recompute semantics preserved automatically
- **Consistency**: All objects follow same patterns
- **Flexibility**: Python and C++ extensions supported

---

## References

- FreeCAD Workbench Development: `Gui::Workbench`
- Command System: `Gui::Command`
- Document Objects: `App::DocumentObject`, `App::FeaturePython`
- Property System: `App::PropertyContainer`
- Recomputation: `Document::recompute()`, `DocumentObject::execute()`
- Extensions: `App::Extension`, `App::ExtensionContainer`

