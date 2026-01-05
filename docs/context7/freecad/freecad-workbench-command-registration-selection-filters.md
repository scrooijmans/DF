# FreeCAD Workbench Command Registration and Selection Filters

This document explains in detail how FreeCAD workbenches register commands/operators and expose them in the UI, including how commands determine expected selection/data types through selection filters and accepted object types.

## Overview

FreeCAD workbenches register commands that appear in menus, toolbars, and can be triggered via keyboard shortcuts. Commands use selection filters and type validation to ensure they operate on appropriate object types, providing a robust and user-friendly interface.

---

## 1. Workbench Command Registration

### Workbench Initialization

Workbenches register commands during initialization in `InitGui.py`:

```python
# InitGui.py
import FreeCAD as App
import FreeCADGui as Gui

class MyWorkbench(Gui.Workbench):
    MenuText = "My Workbench"
    ToolTip = "Custom workbench"
    Icon = "path/to/icon.svg"
    
    def Initialize(self):
        """Initialize workbench - register commands"""
        # Import command modules
        from MyWorkbench import CreateBox, CreateCylinder, FuseObjects
        
        # Register commands
        Gui.addCommand("MyWorkbench_CreateBox", CreateBox.Command())
        Gui.addCommand("MyWorkbench_CreateCylinder", CreateCylinder.Command())
        Gui.addCommand("MyWorkbench_FuseObjects", FuseObjects.Command())
        
        # Create toolbar
        self.appendToolbar("My Tools", [
            "MyWorkbench_CreateBox",
            "MyWorkbench_CreateCylinder",
            "MyWorkbench_FuseObjects"
        ])
        
        # Create menu
        self.appendMenu("My Menu", [
            "MyWorkbench_CreateBox",
            "MyWorkbench_CreateCylinder",
            "MyWorkbench_FuseObjects"
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

### Command Registration Process

**Registration Steps**:
1. **Import Command**: Import command class/module
2. **Create Instance**: Instantiate command class
3. **Register with Gui**: `Gui.addCommand(command_id, command_instance)`
4. **Add to UI**: Add to toolbar/menu in `Initialize()`

**Command ID Format**: `WorkbenchName_CommandName`

**Example**:
```python
# Command ID: MyWorkbench_CreateBox
Gui.addCommand("MyWorkbench_CreateBox", CreateBox.Command())
```

---

## 2. Command Structure and UI Exposure

### Command Resources (GetResources)

Commands define their UI appearance through `GetResources()`:

```python
class MyCommand:
    def GetResources(self):
        """Return command resources for UI"""
        return {
            "Pixmap": "MyWorkbench/icons/MyCommand.svg",  # Icon path
            "MenuText": "My Command",                      # Menu text
            "ToolTip": "Creates a custom object",         # Tooltip
            "Accel": "C, M",                              # Keyboard shortcut (Ctrl+M)
            "Checkable": False                            # Toggle command (optional)
        }
```

**Resource Dictionary Keys**:
- **Pixmap**: Icon path (SVG, PNG, XPM)
- **MenuText**: Text shown in menu
- **ToolTip**: Tooltip text when hovering
- **Accel**: Keyboard shortcut (format: "Key1, Key2" for Ctrl+Key2)
- **Checkable**: Whether command is toggleable (boolean)

### UI Exposure Methods

#### A. Toolbar

**Add to Toolbar**:
```python
self.appendToolbar("Toolbar Name", [
    "CommandID1",
    "CommandID2",
    "Separator",  # Visual separator
    "CommandID3"
])
```

**Example**:
```python
self.appendToolbar("Part Tools", [
    "Part_Box",
    "Part_Cylinder",
    "Separator",
    "Part_Fuse",
    "Part_Cut"
])
```

#### B. Menu

**Add to Menu**:
```python
self.appendMenu("Menu Name", [
    "CommandID1",
    "CommandID2",
    "Separator",
    "CommandID3"
])
```

**Example**:
```python
self.appendMenu("Part", [
    "Part_Box",
    "Part_Cylinder",
    "Separator",
    "Part_Fuse",
    "Part_Cut"
])
```

#### C. Context Menu

**Add to Context Menu** (right-click menu):
```python
self.appendContextMenu("Context Menu Name", [
    "CommandID1",
    "CommandID2"
])
```

#### D. Keyboard Shortcuts

**Define in GetResources()**:
```python
def GetResources(self):
    return {
        "Accel": "C, M"  # Ctrl+M
    }
```

**Shortcut Format**: `"Key1, Key2"` where:
- `Key1` = Modifier (C=Ctrl, A=Alt, S=Shift)
- `Key2` = Key (M, B, etc.)

---

## 3. Command Activation and Selection

### Command Lifecycle

```
User Action (click button, menu, shortcut)
    ↓
Command.IsActive() checked (optional)
    ↓
Command.Activated() called
    ↓
Command gets selection
    ↓
Command validates selection
    ↓
Command performs operation
```

### Getting Selection

**Basic Selection Retrieval**:
```python
import FreeCAD as App
import FreeCADGui as Gui

class MyCommand:
    def Activated(self):
        # Get selected objects
        selection = Gui.Selection.getSelection()
        
        # Process selection
        for obj in selection:
            print(f"Selected: {obj.Label}")
```

**Selection Methods**:
- `Gui.Selection.getSelection()`: Get all selected objects
- `Gui.Selection.getSelectionEx()`: Get selection with sub-elements
- `Gui.Selection.getSelectionObject()`: Get first selected object
- `Gui.Selection.countObjectsOfType(type)`: Count selected objects of type

---

## 4. Selection Filters and Type Validation

### Selection Filter Concept

**Selection filters** restrict what users can select in the 3D view, ensuring commands only receive appropriate object types.

### Filter Types

#### A. Sub-Element Filters

**Filter to specific sub-elements** (vertices, edges, faces):

```python
# Enable edge selection filter
Gui.Selection.addSelectionGate("SELECT Part::Feature SUBELEMENT Edge")

# Enable face selection filter
Gui.Selection.addSelectionGate("SELECT Part::Feature SUBELEMENT Face")

# Enable vertex selection filter
Gui.Selection.addSelectionGate("SELECT Part::Feature SUBELEMENT Vertex")

# Clear filter
Gui.Selection.clearSelectionGate()
```

**Filter Syntax**: `"SELECT ObjectType SUBELEMENT ElementType"`

**Example**: `"SELECT Part::Feature SUBELEMENT Edge"` allows only edge selection on Part features.

#### B. Object Type Filters

**Filter to specific object types**:

```python
# Filter to Part::Feature objects only
Gui.Selection.addSelectionGate("SELECT Part::Feature")

# Filter to Sketcher::SketchObject only
Gui.Selection.addSelectionGate("SELECT Sketcher::SketchObject")

# Clear filter
Gui.Selection.clearSelectionGate()
```

### Selection Validation in Commands

Commands validate selection to ensure correct object types:

#### A. Type Checking Methods

**isDerivedFrom()**: Check if object is instance of or derived from type.

```python
class FuseCommand:
    def Activated(self):
        selection = Gui.Selection.getSelection()
        
        # Validate: Need at least 2 objects
        if len(selection) < 2:
            App.Console.PrintError("Select at least 2 objects\n")
            return
        
        # Validate: All must be Part::Feature
        for obj in selection:
            if not obj.isDerivedFrom("Part::Feature"):
                App.Console.PrintError(
                    f"Object '{obj.Label}' is not a Part feature\n")
                return
        
        # Type checks passed - create fusion
        fusion = doc.addObject("Part::Fuse", "Fusion")
        fusion.Base = selection[0]
        fusion.Tool = selection[1]
        doc.recompute()
```

**TypeId Property**: Check exact object type.

```python
# Check exact type
if obj.TypeId == "Part::Box":
    print("This is a Box")

# Check base type
if obj.TypeId.startswith("Part::"):
    print("This is a Part workbench object")
```

**hasExtension()**: Check if object has specific extension.

```python
# Check for GroupExtension
if obj.hasExtension("App::GroupExtension"):
    group = obj.getExtensionByType("App::GroupExtension")
    # Use group functionality
```

#### B. Selection Filtering

**Filter selection to valid objects**:

```python
class ChamferCommand:
    def Activated(self):
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

#### C. Sub-Element Selection Validation

**Validate sub-element selections** (faces, edges, vertices):

```python
class FilletCommand:
    def Activated(self):
        # Get selection with sub-elements
        selection = Gui.Selection.getSelectionEx()
        
        if len(selection) == 0:
            App.Console.PrintError("Select edges or faces\n")
            return
        
        # Validate selections
        edges = []
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
                    f"Select edges or faces on '{obj.Label}'\n")
                continue
            
            # Validate sub-element types
            for sub in sub_elements:
                if not (sub.startswith("Edge") or sub.startswith("Face")):
                    App.Console.PrintError(
                        f"Sub-element '{sub}' is not an edge or face\n")
                    continue
                
                edges.append((obj, sub))
        
        if len(edges) == 0:
            App.Console.PrintError("No valid edges or faces selected\n")
            return
        
        # Create fillet with selected edges
        fillet = doc.addObject("Part::Fillet", "Fillet")
        fillet.Edges = edges
        doc.recompute()
```

---

## 5. Command State Management (IsActive)

### IsActive Method

**Purpose**: Determine if command should be enabled/disabled.

**Called**: Before command activation, and periodically to update UI state.

**Example**:
```python
class MyCommand:
    def IsActive(self):
        """Check if command should be active (enabled)"""
        # Command active only if document exists
        return App.ActiveDocument is not None
```

### Common IsActive Patterns

#### A. Document Check

```python
def IsActive(self):
    return App.ActiveDocument is not None
```

#### B. Selection Check

```python
def IsActive(self):
    selection = Gui.Selection.getSelection()
    return len(selection) > 0
```

#### C. Selection Type Check

```python
def IsActive(self):
    selection = Gui.Selection.getSelection()
    if len(selection) == 0:
        return False
    
    # Check if all selected objects are Part::Feature
    return all(obj.isDerivedFrom("Part::Feature") for obj in selection)
```

#### D. Workbench Check

```python
def IsActive(self):
    return Gui.ActiveWorkbench.Name == "MyWorkbench"
```

#### E. Complex Conditions

```python
def IsActive(self):
    # Multiple conditions
    if not App.ActiveDocument:
        return False
    
    selection = Gui.Selection.getSelection()
    if len(selection) < 2:
        return False
    
    # All must be Part::Feature
    return all(obj.isDerivedFrom("Part::Feature") for obj in selection)
```

---

## 6. Selection Gate Usage

### Setting Selection Gate

**Selection gates** restrict what users can select in the 3D view:

```python
class FilletCommand:
    def Activated(self):
        # Set selection gate: only edges
        Gui.Selection.addSelectionGate("SELECT Part::Feature SUBELEMENT Edge")
        
        # Wait for user selection
        # (Command may use dialog or callback)
        
        # Clear selection gate when done
        Gui.Selection.clearSelectionGate()
```

### Selection Gate Patterns

#### A. Edge Selection

```python
# Allow only edge selection on Part features
Gui.Selection.addSelectionGate("SELECT Part::Feature SUBELEMENT Edge")
```

#### B. Face Selection

```python
# Allow only face selection on Part features
Gui.Selection.addSelectionGate("SELECT Part::Feature SUBELEMENT Face")
```

#### C. Object Selection

```python
# Allow only Part::Feature object selection
Gui.Selection.addSelectionGate("SELECT Part::Feature")
```

#### D. Sketch Selection

```python
# Allow only sketch selection
Gui.Selection.addSelectionGate("SELECT Sketcher::SketchObject")
```

### Clearing Selection Gate

**Always clear selection gate** when command finishes:

```python
class MyCommand:
    def Activated(self):
        try:
            # Set selection gate
            Gui.Selection.addSelectionGate("SELECT Part::Feature")
            
            # Do work...
            
        finally:
            # Always clear gate
            Gui.Selection.clearSelectionGate()
```

---

## 7. Complete Command Examples

### Example 1: Simple Object Creation Command

**Command**: Creates a box without selection requirements.

```python
import FreeCAD as App
import FreeCADGui as Gui

class CreateBoxCommand:
    def GetResources(self):
        return {
            "Pixmap": "Part/icons/Part_Box.svg",
            "MenuText": "Box",
            "ToolTip": "Creates a box",
            "Accel": "B, O"  # Ctrl+O
        }
    
    def Activated(self):
        doc = App.ActiveDocument
        if not doc:
            doc = App.newDocument()
        
        # Create box
        box = doc.addObject("Part::Box", "Box")
        box.Length = 10
        box.Width = 10
        box.Height = 10
        
        doc.recompute()
        Gui.ActiveDocument.ActiveView.fitAll()
    
    def IsActive(self):
        return True  # Always active

Gui.addCommand("Part_Box", CreateBoxCommand())
```

### Example 2: Selection-Based Command (Fuse)

**Command**: Requires 2 Part::Feature objects selected.

```python
class FuseCommand:
    def GetResources(self):
        return {
            "Pixmap": "Part/icons/Part_Fuse.svg",
            "MenuText": "Fuse",
            "ToolTip": "Fuses selected objects"
        }
    
    def Activated(self):
        selection = Gui.Selection.getSelection()
        
        # Validate: Need exactly 2 objects
        if len(selection) != 2:
            App.Console.PrintError("Select exactly 2 objects\n")
            return
        
        # Validate: Both must be Part::Feature
        for obj in selection:
            if not obj.isDerivedFrom("Part::Feature"):
                App.Console.PrintError(
                    f"Object '{obj.Label}' is not a Part feature\n")
                return
        
        # Create fusion
        doc = App.ActiveDocument
        fusion = doc.addObject("Part::Fuse", "Fusion")
        fusion.Base = selection[0]
        fusion.Tool = selection[1]
        doc.recompute()
    
    def IsActive(self):
        # Active only if 2 Part::Feature objects selected
        selection = Gui.Selection.getSelection()
        if len(selection) != 2:
            return False
        return all(obj.isDerivedFrom("Part::Feature") for obj in selection)

Gui.addCommand("Part_Fuse", FuseCommand())
```

### Example 3: Sub-Element Selection Command (Fillet)

**Command**: Requires edges or faces selected.

```python
class FilletCommand:
    def GetResources(self):
        return {
            "Pixmap": "Part/icons/Part_Fillet.svg",
            "MenuText": "Fillet",
            "ToolTip": "Fillets selected edges"
        }
    
    def Activated(self):
        # Get selection with sub-elements
        selection = Gui.Selection.getSelectionEx()
        
        if len(selection) == 0:
            App.Console.PrintError("Select edges or faces\n")
            return
        
        # Collect valid edges/faces
        edges = []
        for sel in selection:
            obj = sel.Object
            sub_elements = sel.SubElementNames
            
            # Validate object type
            if not obj.isDerivedFrom("Part::Feature"):
                continue
            
            # Validate sub-elements
            for sub in sub_elements:
                if sub.startswith("Edge") or sub.startswith("Face"):
                    edges.append((obj, sub))
        
        if len(edges) == 0:
            App.Console.PrintError("Select edges or faces\n")
            return
        
        # Create fillet
        doc = App.ActiveDocument
        fillet = doc.addObject("Part::Fillet", "Fillet")
        fillet.Edges = edges
        fillet.Radius = 1.0
        doc.recompute()
    
    def IsActive(self):
        # Active if edges or faces selected
        selection = Gui.Selection.getSelectionEx()
        if len(selection) == 0:
            return False
        
        # Check if any valid sub-elements selected
        for sel in selection:
            if not sel.Object.isDerivedFrom("Part::Feature"):
                continue
            for sub in sel.SubElementNames:
                if sub.startswith("Edge") or sub.startswith("Face"):
                    return True
        return False

Gui.addCommand("Part_Fillet", FilletCommand())
```

### Example 4: Command with Selection Gate

**Command**: Uses selection gate to restrict selection.

```python
class SelectEdgeCommand:
    def GetResources(self):
        return {
            "Pixmap": "MyWorkbench/icons/SelectEdge.svg",
            "MenuText": "Select Edge",
            "ToolTip": "Select an edge"
        }
    
    def Activated(self):
        # Set selection gate: only edges
        Gui.Selection.addSelectionGate("SELECT Part::Feature SUBELEMENT Edge")
        
        # Show dialog or wait for selection
        # (In practice, this might use a dialog or callback)
        
        # Get selection
        selection = Gui.Selection.getSelectionEx()
        
        # Clear selection gate
        Gui.Selection.clearSelectionGate()
        
        if len(selection) == 0:
            App.Console.PrintMessage("No edge selected\n")
            return
        
        # Process selected edges
        for sel in selection:
            obj = sel.Object
            for sub in sel.SubElementNames:
                if sub.startswith("Edge"):
                    App.Console.PrintMessage(f"Selected edge: {sub} on {obj.Label}\n")
    
    def IsActive(self):
        return App.ActiveDocument is not None

Gui.addCommand("MyWorkbench_SelectEdge", SelectEdgeCommand())
```

---

## 8. PartDesign Workbench: Advanced Selection Patterns

### PartDesign Feature Selection

PartDesign features often require specific object types:

```python
class PadCommand:
    def Activated(self):
        # PartDesign Pad requires a sketch
        selection = Gui.Selection.getSelection()
        
        if len(selection) == 0:
            App.Console.PrintError("Select a sketch\n")
            return
        
        sketch = selection[0]
        
        # Validate: Must be sketch
        if not sketch.isDerivedFrom("Sketcher::SketchObject"):
            App.Console.PrintError("Select a sketch\n")
            return
        
        # Validate: Sketch must be in active Body
        body = Gui.ActiveDocument.ActiveView.getActiveObject("pdbody")
        if not body:
            App.Console.PrintError("No active Body\n")
            return
        
        if sketch not in body.Group:
            App.Console.PrintError("Sketch must be in active Body\n")
            return
        
        # Create pad
        pad = body.newObject("PartDesign::Pad", "Pad")
        pad.Profile = sketch
        pad.Length = 10.0
        App.ActiveDocument.recompute()
    
    def IsActive(self):
        # Active only if sketch selected and Body active
        selection = Gui.Selection.getSelection()
        if len(selection) != 1:
            return False
        
        if not selection[0].isDerivedFrom("Sketcher::SketchObject"):
            return False
        
        body = Gui.ActiveDocument.ActiveView.getActiveObject("pdbody")
        return body is not None
```

### TaskFeaturePick Dialog

PartDesign uses `TaskFeaturePick` dialog for feature selection:

```python
# PartDesign uses TaskFeaturePick dialog
# This dialog filters objects based on:
# - Object type (e.g., must be sketch)
# - Object status (valid, used, etc.)
# - Object location (in Body, external, etc.)

# The dialog handles selection filtering automatically
```

---

## 9. Selection Filter Best Practices

### Best Practices

#### A. Always Validate Selection

```python
def Activated(self):
    selection = Gui.Selection.getSelection()
    
    # ✅ Always validate
    if len(selection) == 0:
        App.Console.PrintError("Select objects\n")
        return
    
    # ✅ Validate types
    for obj in selection:
        if not obj.isDerivedFrom("Part::Feature"):
            App.Console.PrintError("Invalid object type\n")
            return
```

#### B. Clear Selection Gates

```python
def Activated(self):
    try:
        Gui.Selection.addSelectionGate("SELECT Part::Feature")
        # Do work...
    finally:
        # ✅ Always clear gate
        Gui.Selection.clearSelectionGate()
```

#### C. Provide Clear Error Messages

```python
# ✅ Good: Clear error message
if not obj.isDerivedFrom("Part::Feature"):
    App.Console.PrintError(
        f"Object '{obj.Label}' must be a Part feature\n")

# ❌ Bad: Vague error message
if not obj.isDerivedFrom("Part::Feature"):
    App.Console.PrintError("Invalid selection\n")
```

#### D. Use IsActive for Better UX

```python
def IsActive(self):
    # ✅ Disable command if requirements not met
    selection = Gui.Selection.getSelection()
    return len(selection) >= 2 and all(
        obj.isDerivedFrom("Part::Feature") for obj in selection
    )
```

---

## 10. Summary

### Command Registration

1. **Workbench Initialize()**: Registers commands via `Gui.addCommand()`
2. **UI Exposure**: Commands added to toolbars, menus via `appendToolbar()`, `appendMenu()`
3. **Command Resources**: `GetResources()` defines icon, text, tooltip, shortcut

### Selection and Type Validation

1. **Selection Retrieval**: `Gui.Selection.getSelection()` or `getSelectionEx()`
2. **Type Checking**: `isDerivedFrom()`, `TypeId`, `hasExtension()`
3. **Selection Filtering**: Filter selection to valid objects
4. **Selection Gates**: Restrict what users can select in 3D view

### Command State Management

1. **IsActive()**: Determines if command should be enabled
2. **Common Patterns**: Document check, selection check, type check
3. **Dynamic State**: Command state updates based on selection

### Key Principles

1. **Validate Selection**: Always validate selection in `Activated()`
2. **Clear Gates**: Always clear selection gates when done
3. **Clear Errors**: Provide clear error messages
4. **Use IsActive**: Disable commands when requirements not met

---

## References

- Command Registration: `Gui.addCommand()`, `Gui.Workbench.Initialize()`
- Selection: `Gui.Selection.getSelection()`, `Gui.Selection.getSelectionEx()`
- Selection Gates: `Gui.Selection.addSelectionGate()`, `Gui.Selection.clearSelectionGate()`
- Type Checking: `isDerivedFrom()`, `TypeId`, `hasExtension()`
- Command Resources: `GetResources()`, `IsActive()`, `Activated()`

