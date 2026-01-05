# Blender's Operator and Addon Extensibility Model

## Overview

Blender's extensibility model allows developers to add functionality through operators and addons. This document explains the complete lifecycle, safety constraints, and integration mechanisms that enable safe, powerful extensions while protecting Blender's core data structures.

## Extensibility Architecture

### Extension Points

Blender provides multiple extension points:

1. **Operators**: User actions (keyboard shortcuts, menu items)
2. **Addons**: Collections of operators, panels, and functionality
3. **Python Scripts**: Standalone scripts
4. **Custom Properties**: Extend data-blocks with custom data
5. **Node Groups**: Reusable node setups
6. **Modifiers**: Non-destructive operations

### Core Principles

1. **Safety First**: Extensions cannot corrupt core data structures
2. **Lifecycle Management**: Proper initialization and cleanup
3. **Integration**: Seamless integration with Blender's systems
4. **Validation**: Runtime checks prevent invalid operations

## Operator System

### What Are Operators?

Operators are user actions that modify the scene:
- Menu items
- Keyboard shortcuts
- Tool buttons
- Python API calls

### Operator Structure

```c
typedef struct wmOperatorType {
    // Identification
    char idname[64];          // Unique identifier (e.g., "OBJECT_OT_delete")
    char name[64];            // Display name
    char description[256];    // Tooltip description
    
    // Lifecycle functions
    int (*exec)(bContext *C, wmOperator *op);  // Execute operator
    int (*invoke)(bContext *C, wmOperator *op, wmEvent *event);  // Interactive invoke
    int (*modal)(bContext *C, wmOperator *op, wmEvent *event);   // Modal operation
    
    // Safety functions
    bool (*poll)(bContext *C);  // Check if operator can run
    
    // Properties
    StructRNA *srna;           // RNA properties
    PropertyRNA *prop;         // Property definitions
    
    // UI
    int (*ui)(bContext *C, wmOperator *op);  // Custom UI
    
    // Undo
    bool (*undo)(bContext *C, wmOperator *op);  // Undo support
} wmOperatorType;
```

### Operator Lifecycle

#### 1. Registration

Operators are registered at startup or when addons load:

```c
// C operator registration
void OBJECT_OT_delete(wmOperatorType *ot)
{
    ot->name = "Delete";
    ot->idname = "OBJECT_OT_delete";
    ot->description = "Delete selected objects";
    
    ot->exec = delete_object_exec;
    ot->poll = delete_object_poll;
    
    // Register with window manager
    WM_operatortype_append(ot);
}
```

```python
# Python operator registration
import bpy
from bpy.types import Operator

class OBJECT_OT_my_operator(Operator):
    bl_idname = "object.my_operator"
    bl_label = "My Operator"
    bl_description = "Does something"
    
    def execute(self, context):
        # Operator logic
        return {'FINISHED'}
    
    @classmethod
    def poll(cls, context):
        # Safety check
        return context.active_object is not None

# Register
bpy.utils.register_class(OBJECT_OT_my_operator)
```

#### 2. Poll Phase

Before execution, poll function checks if operator can run:

```c
static bool delete_object_poll(bContext *C)
{
    // Check if operator can run
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;  // No object selected
    }
    
    // Additional checks
    if (ob->type != OB_MESH) {
        return false;  // Only works on meshes
    }
    
    return true;  // Operator available
}
```

#### 3. Invoke Phase (Optional)

For interactive operations:

```c
static int delete_object_invoke(bContext *C, wmOperator *op, wmEvent *event)
{
    // Show confirmation dialog
    return WM_operator_confirm(C, op, event);
}
```

#### 4. Execute Phase

The main operation:

```c
static int delete_object_exec(bContext *C, wmOperator *op)
{
    // 1. Validate context
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        BKE_report(op->reports, RPT_ERROR, "No active object");
        return OPERATOR_CANCELLED;
    }
    
    // 2. Perform operation
    BKE_object_delete(C, ob);
    
    // 3. Update dependency graph
    DEG_id_tag_update(&ob->id, DEG_TAG_TRANSFORM);
    
    // 4. Notify UI
    WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, ob);
    
    return OPERATOR_FINISHED;
}
```

#### 5. Modal Phase (Optional)

For continuous operations:

```c
static int transform_modal(bContext *C, wmOperator *op, wmEvent *event)
{
    switch (event->type) {
        case MOUSEMOVE:
            // Update transform based on mouse
            update_transform(op, event);
            return OPERATOR_RUNNING_MODAL;
            
        case LEFTMOUSE:
        case RETKEY:
        case SPACEKEY:
            // Finish operation
            return OPERATOR_FINISHED;
            
        case ESCKEY:
            // Cancel operation
            return OPERATOR_CANCELLED;
    }
    
    return OPERATOR_RUNNING_MODAL;
}
```

#### 6. Cleanup

Operators clean up resources:

```c
static void delete_object_free(wmOperator *op)
{
    // Free operator-specific data
    if (op->customdata) {
        MEM_freeN(op->customdata);
    }
}
```

### Operator Return Codes

```c
typedef enum {
    OPERATOR_FINISHED = 1,      // Operation completed successfully
    OPERATOR_CANCELLED = 2,     // Operation cancelled
    OPERATOR_RUNNING_MODAL = 3, // Modal operation continues
    OPERATOR_PASS_THROUGH = 4,  // Pass event to next handler
    OPERATOR_INTERFACE = 5,     // Show interface
} wmOperatorReturn;
```

### Operator Properties

Operators can have properties:

```c
// C operator properties
static void delete_object_props(wmOperatorType *ot)
{
    PropertyRNA *prop;
    
    prop = RNA_def_boolean(ot->srna, "confirm", true, "Confirm", "Ask for confirmation");
    RNA_def_property_flag(prop, PROP_SKIP_SAVE);
}
```

```python
# Python operator properties
class OBJECT_OT_my_operator(Operator):
    bl_idname = "object.my_operator"
    bl_label = "My Operator"
    
    # Properties
    my_float: FloatProperty(
        name="Value",
        description="Some value",
        default=1.0,
        min=0.0,
        max=10.0
    )
    
    my_enum: EnumProperty(
        name="Type",
        items=[
            ('OPTION1', "Option 1", "First option"),
            ('OPTION2', "Option 2", "Second option"),
        ],
        default='OPTION1'
    )
    
    def execute(self, context):
        # Access properties
        value = self.my_float
        option = self.my_enum
        return {'FINISHED'}
```

## Addon System

### What Are Addons?

Addons are collections of:
- Operators
- Panels (UI)
- Menu items
- Keymaps
- Properties
- Custom functionality

### Addon Structure

```python
bl_info = {
    "name": "My Addon",
    "author": "Your Name",
    "version": (1, 0, 0),
    "blender": (3, 0, 0),  # Minimum Blender version
    "location": "View3D > Sidebar",
    "description": "Adds useful functionality",
    "category": "Mesh",
}

# Addon classes
classes = [
    OBJECT_OT_my_operator,
    VIEW3D_PT_my_panel,
]

def register():
    # Register all classes
    for cls in classes:
        bpy.utils.register_class(cls)
    
    # Register keymaps
    register_keymaps()
    
    # Initialize addon data
    init_addon_data()

def unregister():
    # Unregister in reverse order
    unregister_keymaps()
    
    for cls in reversed(classes):
        bpy.utils.unregister_class(cls)
    
    # Cleanup addon data
    cleanup_addon_data()
```

### Addon Lifecycle

#### 1. Discovery

Blender discovers addons in:
- User addons directory: `~/.config/blender/X.X/scripts/addons/`
- System addons directory: `$BLENDER_SYSTEM_SCRIPTS/addons/`

#### 2. Loading

Addons are loaded when:
- Blender starts (if enabled)
- User enables addon in preferences
- User reloads addon

```python
# Addon loading
def register():
    # Called when addon is enabled
    print("Addon loading...")
    
    # Register operators
    bpy.utils.register_class(MY_OT_operator)
    
    # Register panels
    bpy.utils.register_class(MY_PT_panel)
    
    # Register menus
    bpy.types.VIEW3D_MT_object.append(menu_func)
    
    print("Addon loaded")
```

#### 3. Activation

Addon becomes active after registration:

```python
# Addon is now active
# Operators available in UI
# Panels visible in interface
# Keymaps active
```

#### 4. Runtime

Addon runs during normal Blender operation:

```python
# Operators can be called
# Panels can be drawn
# Event handlers active
```

#### 5. Unloading

Addon unloads when:
- User disables addon
- Blender shuts down
- Addon is reloaded

```python
def unregister():
    # Called when addon is disabled
    print("Addon unloading...")
    
    # Unregister in reverse order
    bpy.types.VIEW3D_MT_object.remove(menu_func)
    bpy.utils.unregister_class(MY_PT_panel)
    bpy.utils.unregister_class(MY_OT_operator)
    
    # Cleanup resources
    cleanup_resources()
    
    print("Addon unloaded")
```

### Addon Safety Constraints

#### 1. Namespace Isolation

Addons use namespaces to avoid conflicts:

```python
# Good: Namespaced
bl_idname = "myaddon.operator_name"

# Bad: Generic name (conflicts possible)
bl_idname = "operator_name"
```

#### 2. Resource Management

Addons must clean up resources:

```python
def register():
    # Allocate resources
    global my_data
    my_data = {}
    
    # Register classes
    bpy.utils.register_class(MY_OT_operator)

def unregister():
    # Unregister classes
    bpy.utils.unregister_class(MY_OT_operator)
    
    # Cleanup resources
    global my_data
    my_data = None
```

#### 3. Error Handling

Addons must handle errors gracefully:

```python
def execute(self, context):
    try:
        # Operation that might fail
        perform_operation()
        return {'FINISHED'}
    except Exception as e:
        self.report({'ERROR'}, f"Operation failed: {e}")
        return {'CANCELLED'}
```

## Safety Constraints

### Data Structure Protection

#### 1. Read-Only Access During Evaluation

Operators cannot modify data during dependency graph evaluation:

```c
// ❌ WRONG: Modifying during evaluation
void modifier_callback(Object *ob) {
    ob->loc[0] = 1.0;  // Not allowed during eval
}

// ✅ CORRECT: Modify original data, not evaluated
void operator_exec(bContext *C, wmOperator *op) {
    Object *ob = CTX_data_active_object(C);
    ob->loc[0] = 1.0;  // OK: Not during evaluation
    DEG_id_tag_update(&ob->id, DEG_TAG_TRANSFORM);
}
```

#### 2. Copy-on-Write Protection

Shared data must be copied before modification:

```c
// Check if data is shared
if (mesh->id.us > 1) {
    // Create copy
    mesh = BKE_mesh_copy_for_eval(mesh);
    object->data = mesh;
}

// Now safe to modify
mesh->mvert[0].co[0] = 1.0;
```

#### 3. ID Reference Safety

Operators must respect ID reference counting:

```c
// Increment reference when storing
id_us_plus(&material->id);  // Increment usage count
object->mat[0] = material;

// Decrement when removing
id_us_min(&material->id);  // Decrement usage count
object->mat[0] = NULL;
```

### Context Validation

#### 1. Poll Functions

Operators must validate context before execution:

```c
static bool operator_poll(bContext *C)
{
    // Validate context
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;
    }
    
    // Validate type
    if (ob->type != OB_MESH) {
        return false;
    }
    
    return true;
}
```

#### 2. Runtime Checks

Operators must check context in execute:

```c
static int operator_exec(bContext *C, wmOperator *op)
{
    // Double-check context (defensive programming)
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        BKE_report(op->reports, RPT_ERROR, "No active object");
        return OPERATOR_CANCELLED;
    }
    
    // Validate type
    if (ob->type != OB_MESH) {
        BKE_report(op->reports, RPT_ERROR, "Active object is not a mesh");
        return OPERATOR_CANCELLED;
    }
    
    // Safe to proceed
    return OPERATOR_FINISHED;
}
```

### Memory Safety

#### 1. Allocation/Deallocation

Operators must properly manage memory:

```c
// Allocate
void *data = MEM_mallocN(sizeof(MyData), "MyData");

// Use data
// ...

// Free
MEM_freeN(data);
```

#### 2. No Memory Leaks

Operators must clean up allocated resources:

```c
static void operator_free(wmOperator *op)
{
    if (op->customdata) {
        MyData *data = op->customdata;
        // Free nested allocations
        if (data->array) {
            MEM_freeN(data->array);
        }
        // Free main structure
        MEM_freeN(data);
        op->customdata = NULL;
    }
}
```

### Thread Safety

#### 1. Main Thread Only

Most operations must be on main thread:

```c
// ❌ WRONG: From worker thread
void worker_thread() {
    object->loc[0] = 1.0;  // Not thread-safe
}

// ✅ CORRECT: Queue for main thread
void worker_thread() {
    WM_jobs_customdata_set(job, object, new_value);
}
```

#### 2. Lock-Free Operations

Some operations are lock-free:

```c
// Reading evaluated data (lock-free)
Object *ob_eval = DEG_get_evaluated_object(depsgraph, ob);
float loc[3];
copy_v3_v3(loc, ob_eval->loc);  // Safe to read
```

## Integration with Core Data Structures

### ID Data-Block Integration

#### 1. Extending ID Data-Blocks

Addons can add custom properties to ID data-blocks:

```python
# Add custom property to Object
import bpy

# Define custom property
bpy.types.Object.my_custom_prop = bpy.props.FloatProperty(
    name="My Custom Property",
    default=1.0
)

# Use in operator
class OBJECT_OT_use_custom_prop(Operator):
    bl_idname = "object.use_custom_prop"
    bl_label = "Use Custom Property"
    
    def execute(self, context):
        ob = context.active_object
        value = ob.my_custom_prop  # Access custom property
        return {'FINISHED'}
```

#### 2. ID Property System

Use ID properties for structured data:

```c
// C: Add ID property
IDProperty *group = IDP_NewGroup("my_group");
IDProperty *prop = IDP_NewFloat("my_value", 1.0);
IDP_AddToGroup(group, prop);
IDP_AddToGroup(object->id.properties, group);

// Access
IDProperty *prop = IDP_GetPropertyFromGroup(object->id.properties, "my_group.my_value");
float value = IDP_Float(prop);
```

### Dependency Graph Integration

#### 1. Tagging Updates

Operators must tag DEG updates:

```c
// After modifying data
object->loc[0] = 1.0;
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
```

#### 2. Using Evaluated Data

Operators should use evaluated data when appropriate:

```c
// Get evaluated object
Depsgraph *depsgraph = CTX_data_depsgraph(C);
Object *ob_eval = DEG_get_evaluated_object(depsgraph, ob);

// Use evaluated data
float loc[3];
copy_v3_v3(loc, ob_eval->loc);
```

### UI Integration

#### 1. Panels

Addons can add custom panels:

```python
class VIEW3D_PT_my_panel(Panel):
    bl_label = "My Panel"
    bl_idname = "VIEW3D_PT_my_panel"
    bl_space_type = 'VIEW_3D'
    bl_region_type = 'UI'
    bl_category = "My Addon"
    
    def draw(self, context):
        layout = self.layout
        ob = context.active_object
        
        layout.prop(ob, "location")
        layout.operator("object.my_operator")
```

#### 2. Menus

Addons can extend menus:

```python
def menu_func(self, context):
    self.layout.operator("object.my_operator")

# Add to menu
bpy.types.VIEW3D_MT_object.append(menu_func)

# Remove from menu (in unregister)
bpy.types.VIEW3D_MT_object.remove(menu_func)
```

#### 3. Keymaps

Addons can register keymaps:

```python
addon_keymaps = []

def register_keymaps():
    wm = bpy.context.window_manager
    kc = wm.keyconfigs.addon
    
    if kc:
        km = kc.keymaps.new(name='3D View', space_type='VIEW_3D')
        kmi = km.keymap_items.new("object.my_operator", 'E', 'PRESS', ctrl=True)
        addon_keymaps.append((km, kmi))

def unregister_keymaps():
    for km, kmi in addon_keymaps:
        km.keymap_items.remove(kmi)
    addon_keymaps.clear()
```

### Event System Integration

#### 1. Event Handlers

Addons can register event handlers:

```python
@persistent
def load_handler(dummy):
    print("File loaded")

# Register handler
bpy.app.handlers.load_post.append(load_handler)

# Unregister handler
bpy.app.handlers.load_post.remove(load_handler)
```

#### 2. Timers

Addons can use timers:

```python
def timer_func():
    # Called periodically
    print("Timer tick")
    return 1.0  # Return interval in seconds

# Register timer
bpy.app.timers.register(timer_func)
```

## Best Practices

### 1. Proper Registration Order

```python
def register():
    # Register in dependency order
    # 1. Properties first
    bpy.utils.register_class(MY_PROP_GROUP)
    
    # 2. Operators
    bpy.utils.register_class(MY_OT_operator)
    
    # 3. Panels
    bpy.utils.register_class(MY_PT_panel)
    
    # 4. Menus
    bpy.types.VIEW3D_MT_object.append(menu_func)

def unregister():
    # Unregister in reverse order
    bpy.types.VIEW3D_MT_object.remove(menu_func)
    bpy.utils.unregister_class(MY_PT_panel)
    bpy.utils.unregister_class(MY_OT_operator)
    bpy.utils.unregister_class(MY_PROP_GROUP)
```

### 2. Error Handling

```python
def execute(self, context):
    try:
        # Operation
        result = perform_operation()
        self.report({'INFO'}, f"Success: {result}")
        return {'FINISHED'}
    except ValueError as e:
        self.report({'ERROR'}, f"Invalid value: {e}")
        return {'CANCELLED'}
    except Exception as e:
        self.report({'ERROR'}, f"Unexpected error: {e}")
        import traceback
        traceback.print_exc()
        return {'CANCELLED'}
```

### 3. Resource Cleanup

```python
# Store resources globally
addon_data = {}

def register():
    global addon_data
    addon_data = {
        'cache': {},
        'handlers': [],
    }

def unregister():
    global addon_data
    # Cleanup handlers
    for handler in addon_data['handlers']:
        bpy.app.handlers.load_post.remove(handler)
    
    # Clear cache
    addon_data['cache'].clear()
    addon_data = None
```

### 4. Version Compatibility

```python
bl_info = {
    "blender": (3, 0, 0),  # Minimum version
}

def register():
    # Check Blender version
    if bpy.app.version < (3, 0, 0):
        raise Exception("Addon requires Blender 3.0 or later")
    
    # Check for required features
    if not hasattr(bpy.types.Object, 'some_property'):
        raise Exception("Required feature not available")
```

## Common Pitfalls

### 1. Forgetting to Unregister

```python
# ❌ BAD: Classes remain registered
def unregister():
    pass  # Nothing unregistered!

# ✅ GOOD: Proper cleanup
def unregister():
    bpy.utils.unregister_class(MY_OT_operator)
```

### 2. Memory Leaks

```python
# ❌ BAD: Global data never cleared
my_data = []

def register():
    global my_data
    my_data = [1, 2, 3]  # Never freed

# ✅ GOOD: Cleanup in unregister
def unregister():
    global my_data
    my_data = None
```

### 3. Not Tagging DEG Updates

```python
# ❌ BAD: No DEG update
ob.location.x = 1.0  # Viewport won't update

# ✅ GOOD: Tag DEG update
ob.location.x = 1.0
bpy.context.view_layer.update()  # Or use operator
```

### 4. Modifying During Evaluation

```python
# ❌ BAD: Modifying during evaluation
def modifier_callback(ob):
    ob.location.x = 1.0  # Not allowed

# ✅ GOOD: Modify original data
def operator_exec(self, context):
    ob = context.active_object
    ob.location.x = 1.0  # OK in operator
```

## Conclusion

Blender's operator and addon extensibility model provides:

1. **Lifecycle Management**: Proper registration, activation, and cleanup
2. **Safety Constraints**: Poll functions, context validation, memory safety
3. **Integration**: Seamless integration with core data structures
4. **Flexibility**: Multiple extension points and customization options

Understanding this model is essential for:
- Writing reliable Blender addons
- Integrating with Blender's systems
- Maintaining data structure integrity
- Creating professional extensions

The system balances flexibility with safety, allowing powerful extensions while protecting Blender's core functionality.

