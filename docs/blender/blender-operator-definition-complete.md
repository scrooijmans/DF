# Blender Operator Definition: Complete Guide

## Overview

This document provides a comprehensive explanation of how Blender defines operators in both Python (`bpy.types.Operator`) and C, including property declarations, poll() logic, and how context and selection type requirements are specified and encoded. Understanding operator definition is essential for creating Blender extensions.

## Operator Definition Fundamentals

### What Are Operators?

Operators are user actions that modify the scene:
- Menu items
- Keyboard shortcuts
- Tool buttons
- Python API calls

### Operator Types

1. **Python Operators**: Defined using `bpy.types.Operator`
2. **C Operators**: Defined using `wmOperatorType` structure

Both types share the same lifecycle and integration points.

## Python Operator Definition

### Basic Operator Structure

```python
import bpy
from bpy.types import Operator

class OBJECT_OT_my_operator(Operator):
    """My Custom Operator"""
    
    # Required: Unique identifier
    bl_idname = "object.my_operator"
    
    # Required: Display name
    bl_label = "My Operator"
    
    # Optional: Description (tooltip)
    bl_description = "Does something useful"
    
    # Optional: Icon
    bl_options = {'REGISTER', 'UNDO'}
    
    def execute(self, context):
        """Execute the operator"""
        # Operator logic here
        return {'FINISHED'}
    
    @classmethod
    def poll(cls, context):
        """Check if operator can run"""
        return context.active_object is not None
```

### Operator Registration

```python
# Register operator
bpy.utils.register_class(OBJECT_OT_my_operator)

# Unregister operator
bpy.utils.unregister_class(OBJECT_OT_my_operator)

# Or use decorator
@bpy.utils.register_class
class OBJECT_OT_my_operator(Operator):
    # ...
```

### Operator Class Attributes

#### Required Attributes

```python
class OBJECT_OT_my_operator(Operator):
    # Unique identifier (required)
    bl_idname = "object.my_operator"
    # Format: <category>.<name>
    # Examples: "object.delete", "mesh.subdivide", "anim.keyframe_insert"
    
    # Display name (required)
    bl_label = "My Operator"
    # Shown in UI menus
```

#### Optional Attributes

```python
class OBJECT_OT_my_operator(Operator):
    # Description (tooltip)
    bl_description = "Does something useful"
    
    # Icon (icon name from Blender's icon set)
    bl_icon = 'MESH_CUBE'
    
    # Operator options
    bl_options = {'REGISTER', 'UNDO'}
    # Options:
    # - 'REGISTER': Register operator (default)
    # - 'UNDO': Support undo/redo
    # - 'PRESET': Show preset button
    # - 'INTERNAL': Hide from search
    # - 'GRAB_CURSOR': Grab cursor during modal
    # - 'BLOCKING': Block other operators
    
    # Translation context (for i18n)
    bl_translation_context = bpy.app.translations.contexts.default
```

### Property Declarations

#### Basic Property Types

```python
class OBJECT_OT_my_operator(Operator):
    bl_idname = "object.my_operator"
    bl_label = "My Operator"
    
    # Float property
    my_float: FloatProperty(
        name="Value",
        description="A float value",
        default=1.0,
        min=0.0,
        max=10.0,
        step=0.1,
        precision=2
    )
    
    # Integer property
    my_int: IntProperty(
        name="Count",
        description="An integer value",
        default=5,
        min=1,
        max=100,
        step=1
    )
    
    # Boolean property
    my_bool: BoolProperty(
        name="Enable",
        description="A boolean flag",
        default=True
    )
    
    # String property
    my_string: StringProperty(
        name="Name",
        description="A string value",
        default="Default",
        maxlen=256
    )
    
    # Enum property
    my_enum: EnumProperty(
        name="Type",
        description="Selection type",
        items=[
            ('OPTION1', "Option 1", "First option"),
            ('OPTION2', "Option 2", "Second option"),
            ('OPTION3', "Option 3", "Third option"),
        ],
        default='OPTION1'
    )
    
    def execute(self, context):
        # Access properties
        value = self.my_float
        count = self.my_int
        enabled = self.my_bool
        name = self.my_string
        option = self.my_enum
        
        return {'FINISHED'}
```

#### Advanced Property Types

```python
class OBJECT_OT_my_operator(Operator):
    # Float vector (3D)
    my_vector: FloatVectorProperty(
        name="Location",
        description="3D location",
        default=(0.0, 0.0, 0.0),
        subtype='TRANSLATION',
        size=3
    )
    
    # Float vector (color)
    my_color: FloatVectorProperty(
        name="Color",
        description="RGB color",
        default=(1.0, 1.0, 1.0),
        subtype='COLOR',
        size=3,
        min=0.0,
        max=1.0
    )
    
    # Collection property
    my_collection: CollectionProperty(
        name="Items",
        type=SomePropertyGroup
    )
    
    # Pointer property
    my_pointer: PointerProperty(
        name="Object",
        type=bpy.types.Object
    )
```

#### Property Options

```python
class OBJECT_OT_my_operator(Operator):
    # Property with update callback
    my_float: FloatProperty(
        name="Value",
        default=1.0,
        update=lambda self, context: self.on_value_update(context)
    )
    
    def on_value_update(self, context):
        """Called when property value changes"""
        print(f"Value changed to {self.my_float}")
    
    # Property with getter/setter
    my_prop: FloatProperty(
        name="Property",
        default=1.0,
        get=lambda self: self._internal_value,
        set=lambda self, value: setattr(self, '_internal_value', value * 2)
    )
    
    # Property that's not saved
    my_temp: FloatProperty(
        name="Temporary",
        default=1.0,
        options={'SKIP_SAVE'}  # Not saved in .blend file
    )
```

### Poll() Logic

#### Basic Poll Function

```python
@classmethod
def poll(cls, context):
    """Check if operator can run in current context"""
    # Return True if operator is available
    # Return False if operator should be disabled/hidden
    return context.active_object is not None
```

#### Context-Based Poll

```python
@classmethod
def poll(cls, context):
    # Check active object
    if context.active_object is None:
        return False
    
    # Check object type
    if context.active_object.type != 'MESH':
        return False
    
    # Check mode
    if context.active_object.mode != 'OBJECT':
        return False
    
    return True
```

#### Selection-Based Poll

```python
@classmethod
def poll(cls, context):
    # Check selection count
    if len(context.selected_objects) < 2:
        return False
    
    # Check all selected objects are meshes
    for ob in context.selected_objects:
        if ob.type != 'MESH':
            return False
    
    return True
```

#### Editor-Based Poll

```python
@classmethod
def poll(cls, context):
    # Check active editor
    if context.space_data is None:
        return False
    
    # Check editor type
    if context.space_data.type != 'VIEW_3D':
        return False
    
    return True
```

#### Geometry Selection Poll

```python
@classmethod
def poll(cls, context):
    # Check active object
    ob = context.active_object
    if ob is None or ob.type != 'MESH':
        return False
    
    # Check mode
    if ob.mode != 'EDIT':
        return False
    
    # Check edit mesh
    mesh = ob.data
    if mesh is None:
        return False
    
    # Check selection (using bmesh)
    import bmesh
    bm = bmesh.from_mesh(mesh)
    
    # Check if vertices selected
    if not any(v.select for v in bm.verts):
        bm.free()
        return False
    
    bm.free()
    return True
```

### Context Requirements Encoding

#### Context Access in Poll

```python
@classmethod
def poll(cls, context):
    # Available context members:
    
    # Window manager
    wm = context.window_manager
    
    # Window
    win = context.window
    
    # Screen
    screen = context.screen
    
    # Area (editor)
    area = context.area
    space = context.space_data  # Editor type
    
    # Region
    region = context.region
    
    # Scene
    scene = context.scene
    view_layer = context.view_layer
    
    # Active/selected data
    active_object = context.active_object
    selected_objects = context.selected_objects
    selected_editable_objects = context.selected_editable_objects
    
    # Edit mode data
    edit_object = context.edit_object
    active_bone = context.active_bone
    active_pose_bone = context.active_pose_bone
    
    # Material/texture
    active_material = context.active_material
    active_texture = context.active_texture
    
    # Animation
    active_action = context.active_action
    
    return True
```

#### Context Type Requirements

```python
class OBJECT_OT_edit_mesh_operator(Operator):
    """Requires mesh object in edit mode"""
    
    @classmethod
    def poll(cls, context):
        ob = context.active_object
        return (ob is not None and 
                ob.type == 'MESH' and 
                ob.mode == 'EDIT')
    
    def execute(self, context):
        # Context guaranteed to have edit mesh
        ob = context.edit_object
        mesh = ob.data
        # ... operate on mesh
        return {'FINISHED'}
```

### Selection Type Requirements

#### Object Selection Requirements

```python
class OBJECT_OT_join_meshes(Operator):
    """Requires multiple mesh objects selected"""
    
    @classmethod
    def poll(cls, context):
        # Require at least 2 selected objects
        if len(context.selected_objects) < 2:
            return False
        
        # All must be meshes
        for ob in context.selected_objects:
            if ob.type != 'MESH':
                return False
        
        return True
    
    def execute(self, context):
        # All selected objects are guaranteed to be meshes
        meshes = [ob.data for ob in context.selected_objects]
        # ... join meshes
        return {'FINISHED'}
```

#### Geometry Selection Requirements

```python
class MESH_OT_extrude_faces(Operator):
    """Requires faces selected in edit mode"""
    
    @classmethod
    def poll(cls, context):
        ob = context.active_object
        if ob is None or ob.type != 'MESH' or ob.mode != 'EDIT':
            return False
        
        # Check face selection
        import bmesh
        mesh = ob.data
        bm = bmesh.from_mesh(mesh)
        
        has_faces = any(f.select for f in bm.faces)
        bm.free()
        
        return has_faces
    
    def execute(self, context):
        # Faces are guaranteed to be selected
        ob = context.edit_object
        mesh = ob.data
        # ... extrude faces
        return {'FINISHED'}
```

### Operator Methods

#### Execute Method

```python
def execute(self, context):
    """Main execution method"""
    # Access properties
    value = self.my_float
    
    # Access context
    ob = context.active_object
    
    # Perform operation
    # ...
    
    # Return status
    return {'FINISHED'}  # Success
    # return {'CANCELLED'}  # Cancelled
    # return {'RUNNING_MODAL'}  # Continue modal
```

#### Invoke Method

```python
def invoke(self, context, event):
    """Interactive invocation"""
    # Show file browser
    return context.window_manager.invoke_props_dialog(self)
    
    # Or show confirm dialog
    return context.window_manager.invoke_confirm(self, event)
    
    # Or enter modal
    context.window_manager.modal_handler_add(self)
    return {'RUNNING_MODAL'}
```

#### Modal Method

```python
def modal(self, context, event):
    """Modal operation (continuous)"""
    if event.type == 'MOUSEMOVE':
        # Update based on mouse
        # ...
        return {'RUNNING_MODAL'}
    
    elif event.type in {'LEFTMOUSE', 'RET'}:
        # Finish
        return {'FINISHED'}
    
    elif event.type == 'ESC':
        # Cancel
        return {'CANCELLED'}
    
    return {'PASS_THROUGH'}
```

#### Draw Method

```python
def draw(self, context):
    """Custom UI in operator dialog"""
    layout = self.layout
    
    layout.prop(self, "my_float")
    layout.prop(self, "my_enum")
    
    # Custom UI elements
    row = layout.row()
    row.label(text="Custom Label")
```

## C Operator Definition

### Basic Operator Structure

```c
// Operator type definition
void OBJECT_OT_delete(wmOperatorType *ot)
{
    // Required: Identifier
    ot->idname = "OBJECT_OT_delete";
    
    // Required: Name
    ot->name = "Delete";
    
    // Optional: Description
    ot->description = "Delete selected objects";
    
    // Required: Execute function
    ot->exec = delete_object_exec;
    
    // Optional: Poll function
    ot->poll = delete_object_poll;
    
    // Optional: Invoke function
    ot->invoke = delete_object_invoke;
    
    // Optional: Modal function
    ot->modal = delete_object_modal;
    
    // Optional: UI function
    ot->ui = delete_object_ui;
    
    // Register operator
    WM_operatortype_append(ot);
}
```

### Operator Type Structure

```c
typedef struct wmOperatorType {
    // Identification
    char idname[64];          // Unique identifier
    char name[64];            // Display name
    char description[256];    // Description
    
    // Lifecycle functions
    int (*exec)(bContext *C, wmOperator *op);      // Execute
    int (*invoke)(bContext *C, wmOperator *op, wmEvent *event);  // Invoke
    int (*modal)(bContext *C, wmOperator *op, wmEvent *event);    // Modal
    
    // Safety functions
    bool (*poll)(bContext *C);  // Poll function
    
    // Properties
    StructRNA *srna;           // RNA structure
    PropertyRNA *prop;          // Property definitions
    
    // UI
    int (*ui)(bContext *C, wmOperator *op);  // Custom UI
    
    // Undo
    bool (*undo)(bContext *C, wmOperator *op);  // Undo support
    
    // Options
    int flag;  // Operator flags
} wmOperatorType;
```

### Property Declarations in C

#### Basic Properties

```c
// Define operator properties
static void delete_object_props(wmOperatorType *ot)
{
    PropertyRNA *prop;
    
    // Boolean property
    prop = RNA_def_boolean(ot->srna, "confirm", true, 
                          "Confirm", "Ask for confirmation");
    RNA_def_property_flag(prop, PROP_SKIP_SAVE);
    
    // Integer property
    prop = RNA_def_int(ot->srna, "count", 1, 1, 100,
                      "Count", "Number of items", 1, 100);
    
    // Float property
    prop = RNA_def_float(ot->srna, "value", 1.0f, 0.0f, 10.0f,
                        "Value", "Some value", 0.0f, 10.0f);
    
    // String property
    prop = RNA_def_string(ot->srna, "name", "Default", 256,
                         "Name", "Object name");
    
    // Enum property
    static const EnumPropertyItem type_items[] = {
        {OPTION1, "OPTION1", 0, "Option 1", "First option"},
        {OPTION2, "OPTION2", 0, "Option 2", "Second option"},
        {0, NULL, 0, NULL, NULL}
    };
    prop = RNA_def_enum(ot->srna, "type", type_items, OPTION1,
                       "Type", "Selection type");
}
```

#### Advanced Properties

```c
static void transform_props(wmOperatorType *ot)
{
    PropertyRNA *prop;
    
    // Float vector (3D)
    prop = RNA_def_float_vector(ot->srna, "location", 3, NULL, -FLT_MAX, FLT_MAX,
                               "Location", "3D location", -FLT_MAX, FLT_MAX);
    RNA_def_property_subtype(prop, PROP_TRANSLATION);
    
    // Float vector (color)
    prop = RNA_def_float_vector(ot->srna, "color", 3, NULL, 0.0f, 1.0f,
                               "Color", "RGB color", 0.0f, 1.0f);
    RNA_def_property_subtype(prop, PROP_COLOR);
    
    // Pointer property
    prop = RNA_def_pointer(ot->srna, "object", "Object",
                          "Object", "Target object");
}
```

### Poll() Logic in C

#### Basic Poll Function

```c
static bool delete_object_poll(bContext *C)
{
    // Check if operator can run
    Object *ob = CTX_data_active_object(C);
    return (ob != NULL);
}
```

#### Context-Based Poll

```c
static bool edit_mesh_poll(bContext *C)
{
    // Check active object
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;
    }
    
    // Check object type
    if (ob->type != OB_MESH) {
        return false;
    }
    
    // Check mode
    if (ob->mode != OB_MODE_EDIT) {
        return false;
    }
    
    return true;
}
```

#### Selection-Based Poll

```c
static bool join_meshes_poll(bContext *C)
{
    // Check selection count
    int count = CTX_data_selected_objects_len(C);
    if (count < 2) {
        return false;
    }
    
    // Check all selected objects are meshes
    Object **objects = CTX_data_selected_objects(C);
    for (int i = 0; i < count; i++) {
        if (objects[i]->type != OB_MESH) {
            return false;
        }
    }
    
    return true;
}
```

#### Geometry Selection Poll

```c
static bool extrude_faces_poll(bContext *C)
{
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL || ob->type != OB_MESH) {
        return false;
    }
    
    if (ob->mode != OB_MODE_EDIT) {
        return false;
    }
    
    // Check face selection
    Mesh *mesh = ob->data;
    if (mesh == NULL) {
        return false;
    }
    
    BMEditMesh *em = mesh->edit_mesh;
    if (em == NULL || em->bm == NULL) {
        return false;
    }
    
    return (em->bm->totfacesel > 0);
}
```

### Context Requirements Encoding in C

#### Context Access

```c
static bool operator_poll(bContext *C)
{
    // Available context accessors:
    
    // Active object
    Object *ob = CTX_data_active_object(C);
    
    // Selected objects
    int count = CTX_data_selected_objects_len(C);
    Object **objects = CTX_data_selected_objects(C);
    
    // Edit mesh
    Mesh *mesh = CTX_data_edit_mesh(C);
    
    // Active material
    Material *mat = CTX_data_active_material(C);
    
    // Scene
    Scene *scene = CTX_data_scene(C);
    ViewLayer *view_layer = CTX_data_view_layer(C);
    
    // Editor
    SpaceLink *space = CTX_wm_space_data(C);
    ARegion *region = CTX_wm_region(C);
    
    return true;
}
```

#### Context Type Requirements

```c
static bool edit_mesh_operator_poll(bContext *C)
{
    // Require mesh object in edit mode
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL || ob->type != OB_MESH) {
        return false;
    }
    
    if (ob->mode != OB_MODE_EDIT) {
        return false;
    }
    
    Mesh *mesh = CTX_data_edit_mesh(C);
    if (mesh == NULL) {
        return false;
    }
    
    return true;
}
```

### Selection Type Requirements in C

#### Object Selection Requirements

```c
static bool join_meshes_poll(bContext *C)
{
    // Require multiple mesh objects selected
    int count = CTX_data_selected_objects_len(C);
    if (count < 2) {
        return false;
    }
    
    Object **objects = CTX_data_selected_objects(C);
    for (int i = 0; i < count; i++) {
        if (objects[i]->type != OB_MESH) {
            return false;
        }
    }
    
    return true;
}
```

#### Geometry Selection Requirements

```c
static bool extrude_faces_poll(bContext *C)
{
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL || ob->type != OB_MESH || ob->mode != OB_MODE_EDIT) {
        return false;
    }
    
    Mesh *mesh = CTX_data_edit_mesh(C);
    if (mesh == NULL) {
        return false;
    }
    
    BMEditMesh *em = mesh->edit_mesh;
    if (em == NULL || em->bm == NULL) {
        return false;
    }
    
    // Require faces selected
    return (em->bm->totfacesel > 0);
}
```

### Operator Functions

#### Execute Function

```c
static int delete_object_exec(bContext *C, wmOperator *op)
{
    // Get properties
    bool confirm = RNA_boolean_get(op->ptr, "confirm");
    
    // Get context
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        BKE_report(op->reports, RPT_ERROR, "No active object");
        return OPERATOR_CANCELLED;
    }
    
    // Perform operation
    BKE_object_delete(C, ob);
    
    // Update dependency graph
    DEG_id_tag_update(&ob->id, DEG_TAG_TRANSFORM);
    
    // Notify UI
    WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, ob);
    
    return OPERATOR_FINISHED;
}
```

#### Poll Function

```c
static bool delete_object_poll(bContext *C)
{
    return CTX_data_active_object(C) != NULL;
}
```

#### Invoke Function

```c
static int delete_object_invoke(bContext *C, wmOperator *op, wmEvent *event)
{
    // Show confirmation dialog
    return WM_operator_confirm(C, op, event);
}
```

#### Modal Function

```c
static int transform_modal(bContext *C, wmOperator *op, wmEvent *event)
{
    switch (event->type) {
        case MOUSEMOVE:
            // Update transform
            return OPERATOR_RUNNING_MODAL;
            
        case LEFTMOUSE:
        case RETKEY:
            return OPERATOR_FINISHED;
            
        case ESCKEY:
            return OPERATOR_CANCELLED;
    }
    
    return OPERATOR_RUNNING_MODAL;
}
```

## Operator Registration

### Python Registration

```python
# Manual registration
bpy.utils.register_class(OBJECT_OT_my_operator)

# Unregistration
bpy.utils.unregister_class(OBJECT_OT_my_operator)

# Registration in addon
def register():
    bpy.utils.register_class(OBJECT_OT_my_operator)

def unregister():
    bpy.utils.unregister_class(OBJECT_OT_my_operator)
```

### C Registration

```c
// Register in initialization
void OBJECT_OT_delete(wmOperatorType *ot)
{
    // ... define operator
    WM_operatortype_append(ot);
}

// Called during Blender startup
void ED_operators_init(void)
{
    WM_operatortype_append(OBJECT_OT_delete);
    // ... register more operators
}
```

## Operator Integration

### Menu Integration

```python
# Python: Add to menu
def menu_func(self, context):
    self.layout.operator("object.my_operator")

bpy.types.VIEW3D_MT_object.append(menu_func)
```

```c
// C: Add to menu
void VIEW3D_MT_object_editmode(wmMenuType *mt)
{
    uiItemO(layout, "My Operator", ICON_NONE, "OBJECT_OT_my_operator");
}
```

### Keymap Integration

```python
# Python: Register keymap
addon_keymaps = []

def register_keymaps():
    wm = bpy.context.window_manager
    kc = wm.keyconfigs.addon
    
    if kc:
        km = kc.keymaps.new(name='3D View', space_type='VIEW_3D')
        kmi = km.keymap_items.new("object.my_operator", 'E', 'PRESS', ctrl=True)
        addon_keymaps.append((km, kmi))
```

```c
// C: Register keymap
void ED_keymap_object(wmKeyConfig *keyconf)
{
    wmKeyMap *keymap = WM_keymap_ensure(keyconf, "Object Mode", 0, 0);
    WM_keymap_add_item(keymap, "OBJECT_OT_my_operator", EKEY, KM_PRESS, KM_CTRL, 0);
}
```

## Best Practices

### 1. Use Descriptive IDs

```python
# ✅ GOOD: Clear, namespaced ID
bl_idname = "mesh.subdivide_smooth"

# ❌ BAD: Generic ID
bl_idname = "subdivide"
```

### 2. Always Define Poll

```python
# ✅ GOOD: Poll function
@classmethod
def poll(cls, context):
    return context.active_object is not None

# ❌ BAD: No poll function
# Operator might fail at runtime
```

### 3. Validate in Execute Too

```python
# ✅ GOOD: Defensive checks
def execute(self, context):
    ob = context.active_object
    if ob is None:
        self.report({'ERROR'}, "No active object")
        return {'CANCELLED'}
    # ...

# ❌ BAD: Assume poll passed
def execute(self, context):
    ob = context.active_object
    ob.location.x = 1.0  # Might crash
```

### 4. Use Appropriate Property Types

```python
# ✅ GOOD: Specific property types
my_vector: FloatVectorProperty(subtype='TRANSLATION')
my_color: FloatVectorProperty(subtype='COLOR')

# ❌ BAD: Generic types
my_vector: FloatProperty()  # Wrong type
```

## Conclusion

Blender operators are defined through:

1. **Python**: `bpy.types.Operator` class with properties, poll(), and execute()
2. **C**: `wmOperatorType` structure with function pointers

Both support:
- **Property Declarations**: Various types with validation
- **Poll() Logic**: Context and selection validation
- **Context Requirements**: Encoded through poll() and context accessors
- **Selection Requirements**: Validated in poll() functions

Understanding operator definition is essential for:
- Creating Blender extensions
- Integrating with Blender's UI
- Ensuring operator safety and availability
- Providing good user experience

The system provides flexibility while maintaining safety through poll() functions and context validation.

