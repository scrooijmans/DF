# Blender Operator Input Validation: Context, Poll, and Selection Checks

## Overview

Blender operators use a sophisticated validation system to ensure they only run on compatible data. This document explains how context checks, poll functions, selection validation, and RNA types work together to prevent operators from executing on incompatible data, ensuring system stability and user experience.

## Validation Architecture

### Multi-Layer Validation

Blender uses multiple layers of validation:

1. **Poll Functions**: Pre-execution availability checks
2. **Context Validation**: Runtime context state checks
3. **Selection Validation**: Selection state verification
4. **RNA Type Checking**: Runtime type information validation
5. **Defensive Checks**: Additional runtime validation in execute

### Validation Flow

```
Operator Invocation
  ↓
Poll Function Check
  ↓
Context Validation
  ↓
Selection Validation
  ↓
RNA Type Checking
  ↓
Execute (with defensive checks)
```

## Poll Functions

### What Are Poll Functions?

Poll functions determine whether an operator is available in the current context. They provide the **first line of defense** against invalid operations.

### Poll Function Signature

```c
static bool operator_poll(bContext *C)
{
    // Return true if operator can run
    // Return false if operator should be disabled/hidden
}
```

### Basic Poll Pattern

```c
static bool delete_object_poll(bContext *C)
{
    // Check if there's an active object
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;  // No object - operator unavailable
    }
    
    // Check object type
    if (ob->type != OB_MESH) {
        return false;  // Wrong type - operator unavailable
    }
    
    return true;  // Operator available
}
```

### Context-Based Poll Functions

#### Active Object Check

```c
static bool edit_mesh_poll(bContext *C)
{
    // Must have active object
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;
    }
    
    // Object must be mesh
    if (ob->type != OB_MESH) {
        return false;
    }
    
    // Object must have mesh data
    if (ob->data == NULL) {
        return false;
    }
    
    return true;
}
```

#### Mode Check

```c
static bool extrude_vertices_poll(bContext *C)
{
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;
    }
    
    // Must be in Edit Mode
    if (ob->mode != OB_MODE_EDIT) {
        return false;
    }
    
    // Must be mesh
    if (ob->type != OB_MESH) {
        return false;
    }
    
    return true;
}
```

#### Editor Check

```c
static bool view3d_operator_poll(bContext *C)
{
    // Must be in 3D Viewport
    SpaceView3D *s3d = CTX_wm_space_view3d(C);
    if (s3d == NULL) {
        return false;
    }
    
    // Must have 3D region
    RegionView3D *rv3d = CTX_wm_region_view3d(C);
    if (rv3d == NULL) {
        return false;
    }
    
    return true;
}
```

### Selection-Based Poll Functions

#### Single Selection Check

```c
static bool apply_modifier_poll(bContext *C)
{
    // Check active object
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;
    }
    
    // Check if object is selected
    if (!(ob->base_flag & BASE_SELECTED)) {
        return false;
    }
    
    // Check object type
    if (ob->type != OB_MESH) {
        return false;
    }
    
    // Check if has modifiers
    if (BLI_listbase_is_empty(&ob->modifiers)) {
        return false;
    }
    
    return true;
}
```

#### Multiple Selection Check

```c
static bool join_meshes_poll(bContext *C)
{
    // Check selection count
    int selected_count = CTX_data_selected_objects_len(C);
    if (selected_count < 2) {
        return false;  // Need at least 2 objects
    }
    
    // Check all selected objects are meshes
    Object **objects = CTX_data_selected_objects(C);
    for (int i = 0; i < selected_count; i++) {
        if (objects[i]->type != OB_MESH) {
            return false;  // All must be meshes
        }
    }
    
    return true;
}
```

#### Geometry Selection Check

```c
static bool extrude_faces_poll(bContext *C)
{
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL || ob->type != OB_MESH) {
        return false;
    }
    
    // Must be in Edit Mode
    if (ob->mode != OB_MODE_EDIT) {
        return false;
    }
    
    // Get edit mesh
    Mesh *mesh = ob->data;
    BMEditMesh *em = mesh->edit_mesh;
    if (em == NULL) {
        return false;
    }
    
    // Check if faces are selected
    if (em->bm->totfacesel == 0) {
        return false;  // No faces selected
    }
    
    return true;
}
```

### Data Validation in Poll

#### Data Existence Check

```c
static bool material_assign_poll(bContext *C)
{
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;
    }
    
    // Check if object can have materials
    if (ob->type != OB_MESH && 
        ob->type != OB_CURVE && 
        ob->type != OB_SURF) {
        return false;
    }
    
    // Check if material slot exists
    if (ob->totcol == 0) {
        return false;  // No material slots
    }
    
    return true;
}
```

#### Data Validity Check

```c
static bool subdivide_mesh_poll(bContext *C)
{
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL || ob->type != OB_MESH) {
        return false;
    }
    
    Mesh *mesh = ob->data;
    if (mesh == NULL) {
        return false;
    }
    
    // Check mesh has geometry
    if (mesh->totvert == 0) {
        return false;  // Empty mesh
    }
    
    if (mesh->totedge == 0 && mesh->totface == 0) {
        return false;  // No edges or faces
    }
    
    return true;
}
```

## Context Validation

### Context Structure

Context (`bContext`) represents the current state:

```c
typedef struct bContext {
    // Window manager
    wmWindowManager *wm;
    wmWindow *win;
    
    // Screen and area
    ScrArea *area;
    SpaceLink *space_data;
    ARegion *region;
    
    // Scene data
    Scene *scene;
    ViewLayer *view_layer;
    
    // Active/selected data
    Base *active_base;      // Active object base
    Object *active_object;   // Active object
    // ... more context data
} bContext;
```

### Context Accessors

Context provides validated accessors:

#### Active Object

```c
// Get active object (validated)
Object *ob = CTX_data_active_object(C);
if (ob == NULL) {
    // No active object
    return OPERATOR_CANCELLED;
}

// Additional validation
if (ob->type != OB_MESH) {
    // Wrong type
    return OPERATOR_CANCELLED;
}
```

#### Selected Objects

```c
// Get selected objects count
int count = CTX_data_selected_objects_len(C);
if (count == 0) {
    return OPERATOR_CANCELLED;
}

// Get selected objects array
Object **objects = CTX_data_selected_objects(C);
for (int i = 0; i < count; i++) {
    // Validate each object
    if (objects[i]->type != OB_MESH) {
        return OPERATOR_CANCELLED;
    }
}
```

#### Edit Mesh

```c
// Get edit mesh (validates mode and type)
Mesh *mesh = CTX_data_edit_mesh(C);
if (mesh == NULL) {
    // Not in edit mode or not a mesh
    return OPERATOR_CANCELLED;
}

// Additional validation
BMEditMesh *em = mesh->edit_mesh;
if (em == NULL || em->bm == NULL) {
    return OPERATOR_CANCELLED;
}
```

#### Active Material

```c
// Get active material
Material *mat = CTX_data_active_material(C);
if (mat == NULL) {
    // No active material
    return OPERATOR_CANCELLED;
}

// Validate material type
if (mat->id.lib != NULL) {
    // External material - may need special handling
}
```

### Context Mode Validation

```c
// Check current mode
Object *ob = CTX_data_active_object(C);
if (ob == NULL) {
    return OPERATOR_CANCELLED;
}

// Validate mode
switch (ob->mode) {
    case OB_MODE_OBJECT:
        // Object mode operations
        break;
        
    case OB_MODE_EDIT:
        // Edit mode operations
        break;
        
    case OB_MODE_SCULPT:
        // Sculpt mode operations
        break;
        
    default:
        // Unsupported mode
        return OPERATOR_CANCELLED;
}
```

### Context Editor Validation

```c
// Check active editor
SpaceLink *sl = CTX_wm_space_data(C);
if (sl == NULL) {
    return OPERATOR_CANCELLED;
}

// Validate editor type
switch (sl->spacetype) {
    case SPACE_VIEW3D:
        // 3D Viewport operations
        break;
        
    case SPACE_PROPERTIES:
        // Properties editor operations
        break;
        
    default:
        // Unsupported editor
        return OPERATOR_CANCELLED;
}
```

## Selection Validation

### Selection State Checks

#### Object Selection

```c
// Check if object is selected
static bool is_object_selected(Object *ob)
{
    return (ob->base_flag & BASE_SELECTED) != 0;
}

// Check selection count
static int get_selected_count(bContext *C)
{
    int count = 0;
    Object *ob;
    for (ob = CTX_data_selected_objects(C); ob; ob = ob->id.next) {
        if (is_object_selected(ob)) {
            count++;
        }
    }
    return count;
}
```

#### Geometry Selection

```c
// Check vertex selection
static bool has_selected_vertices(bContext *C)
{
    Mesh *mesh = CTX_data_edit_mesh(C);
    if (mesh == NULL) {
        return false;
    }
    
    BMEditMesh *em = mesh->edit_mesh;
    if (em == NULL || em->bm == NULL) {
        return false;
    }
    
    return em->bm->totvertsel > 0;
}

// Check edge selection
static bool has_selected_edges(bContext *C)
{
    Mesh *mesh = CTX_data_edit_mesh(C);
    if (mesh == NULL) {
        return false;
    }
    
    BMEditMesh *em = mesh->edit_mesh;
    if (em == NULL || em->bm == NULL) {
        return false;
    }
    
    return em->bm->totedgesel > 0;
}

// Check face selection
static bool has_selected_faces(bContext *C)
{
    Mesh *mesh = CTX_data_edit_mesh(C);
    if (mesh == NULL) {
        return false;
    }
    
    BMEditMesh *em = mesh->edit_mesh;
    if (em == NULL || em->bm == NULL) {
        return false;
    }
    
    return em->bm->totfacesel > 0;
}
```

### Selection Type Validation

```c
// Validate selection type matches operator requirements
static bool validate_selection_type(bContext *C, int required_type)
{
    Mesh *mesh = CTX_data_edit_mesh(C);
    if (mesh == NULL) {
        return false;
    }
    
    BMEditMesh *em = mesh->edit_mesh;
    if (em == NULL || em->bm == NULL) {
        return false;
    }
    
    switch (required_type) {
        case BM_VERT:
            return em->bm->totvertsel > 0;
            
        case BM_EDGE:
            return em->bm->totedgesel > 0;
            
        case BM_FACE:
            return em->bm->totfacesel > 0;
            
        default:
            return false;
    }
}
```

## RNA Type Validation

### RNA Type System

RNA provides runtime type information for validation:

```c
// Get RNA pointer
PointerRNA ptr;
RNA_id_pointer_create(&object->id, &ptr);

// Check RNA type
if (!RNA_struct_is_a(&ptr, &RNA_Object)) {
    // Not an Object
    return OPERATOR_CANCELLED;
}
```

### RNA Property Type Checking

```c
// Validate property exists and has correct type
PropertyRNA *prop = RNA_struct_find_property(&ptr, "location");
if (prop == NULL) {
    // Property doesn't exist
    return OPERATOR_CANCELLED;
}

// Check property type
if (RNA_property_type(prop) != PROP_FLOAT) {
    // Wrong type
    return OPERATOR_CANCELLED;
}

// Check array length
if (RNA_property_array_length(&ptr, prop) != 3) {
    // Wrong array size
    return OPERATOR_CANCELLED;
}
```

### RNA Pointer Validation

```c
// Get pointer property
PropertyRNA *data_prop = RNA_struct_find_property(&ptr, "data");
if (data_prop == NULL) {
    return OPERATOR_CANCELLED;
}

// Check property type
if (RNA_property_type(data_prop) != PROP_POINTER) {
    return OPERATOR_CANCELLED;
}

// Get pointer value
PointerRNA data_ptr;
RNA_property_pointer_get(&ptr, data_prop, &data_ptr);

// Validate pointer type
if (RNA_pointer_is_null(&data_ptr)) {
    // NULL pointer
    return OPERATOR_CANCELLED;
}

// Check if pointer is of expected type
if (!RNA_struct_is_a(&data_ptr, &RNA_Mesh)) {
    // Not a Mesh
    return OPERATOR_CANCELLED;
}

// Safe to use
Mesh *mesh = data_ptr.data;
```

### RNA Subtype Checking

```c
// Check for base types
if (RNA_struct_is_a(&ptr, &RNA_ID)) {
    // Is any ID data-block
}

// Check for specific types
if (RNA_struct_is_a(&ptr, &RNA_Object)) {
    // Is an Object
}

// Check inheritance-like relationships
if (RNA_struct_is_a(&ptr, &RNA_ID) && 
    RNA_struct_is_a(&ptr, &RNA_Object)) {
    // Is an Object (which is also an ID)
}
```

## Combined Validation Patterns

### Pattern 1: Full Validation Chain

```c
// Poll function
static bool extrude_vertices_poll(bContext *C)
{
    // 1. Context check
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;
    }
    
    // 2. Mode check
    if (ob->mode != OB_MODE_EDIT) {
        return false;
    }
    
    // 3. Type check
    if (ob->type != OB_MESH) {
        return false;
    }
    
    // 4. Data check
    Mesh *mesh = ob->data;
    if (mesh == NULL) {
        return false;
    }
    
    // 5. Selection check
    BMEditMesh *em = mesh->edit_mesh;
    if (em == NULL || em->bm == NULL) {
        return false;
    }
    
    if (em->bm->totvertsel == 0) {
        return false;  // No vertices selected
    }
    
    return true;
}

// Execute with defensive checks
static int extrude_vertices_exec(bContext *C, wmOperator *op)
{
    // Defensive checks (even though poll passed)
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL || ob->type != OB_MESH) {
        BKE_report(op->reports, RPT_ERROR, "No active mesh object");
        return OPERATOR_CANCELLED;
    }
    
    if (ob->mode != OB_MODE_EDIT) {
        BKE_report(op->reports, RPT_ERROR, "Not in edit mode");
        return OPERATOR_CANCELLED;
    }
    
    Mesh *mesh = ob->data;
    if (mesh == NULL) {
        BKE_report(op->reports, RPT_ERROR, "Object has no mesh data");
        return OPERATOR_CANCELLED;
    }
    
    BMEditMesh *em = mesh->edit_mesh;
    if (em == NULL || em->bm == NULL) {
        BKE_report(op->reports, RPT_ERROR, "No edit mesh");
        return OPERATOR_CANCELLED;
    }
    
    if (em->bm->totvertsel == 0) {
        BKE_report(op->reports, RPT_ERROR, "No vertices selected");
        return OPERATOR_CANCELLED;
    }
    
    // Safe to proceed
    // ... perform operation
    return OPERATOR_FINISHED;
}
```

### Pattern 2: RNA-Based Validation

```c
static int set_location_exec(bContext *C, wmOperator *op)
{
    // Get RNA pointer
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return OPERATOR_CANCELLED;
    }
    
    PointerRNA ptr;
    RNA_id_pointer_create(&ob->id, &ptr);
    
    // Validate RNA type
    if (!RNA_struct_is_a(&ptr, &RNA_Object)) {
        BKE_report(op->reports, RPT_ERROR, "Active object is not an Object");
        return OPERATOR_CANCELLED;
    }
    
    // Validate property
    PropertyRNA *prop = RNA_struct_find_property(&ptr, "location");
    if (prop == NULL) {
        BKE_report(op->reports, RPT_ERROR, "Object has no location property");
        return OPERATOR_CANCELLED;
    }
    
    if (RNA_property_type(prop) != PROP_FLOAT) {
        BKE_report(op->reports, RPT_ERROR, "Location is not a float property");
        return OPERATOR_CANCELLED;
    }
    
    if (RNA_property_array_length(&ptr, prop) != 3) {
        BKE_report(op->reports, RPT_ERROR, "Location array has wrong size");
        return OPERATOR_CANCELLED;
    }
    
    // Safe to set property
    float loc[3] = {1.0, 2.0, 3.0};
    RNA_property_float_set_array(&ptr, prop, loc);
    
    return OPERATOR_FINISHED;
}
```

### Pattern 3: Multi-Object Validation

```c
static bool join_meshes_poll(bContext *C)
{
    // Check selection count
    int count = CTX_data_selected_objects_len(C);
    if (count < 2) {
        return false;
    }
    
    // Validate all selected objects
    Object **objects = CTX_data_selected_objects(C);
    for (int i = 0; i < count; i++) {
        // Type check
        if (objects[i]->type != OB_MESH) {
            return false;
        }
        
        // Data check
        if (objects[i]->data == NULL) {
            return false;
        }
        
        // Selection check
        if (!(objects[i]->base_flag & BASE_SELECTED)) {
            return false;
        }
    }
    
    return true;
}

static int join_meshes_exec(bContext *C, wmOperator *op)
{
    // Defensive validation
    int count = CTX_data_selected_objects_len(C);
    if (count < 2) {
        BKE_report(op->reports, RPT_ERROR, "Need at least 2 selected objects");
        return OPERATOR_CANCELLED;
    }
    
    Object **objects = CTX_data_selected_objects(C);
    for (int i = 0; i < count; i++) {
        if (objects[i]->type != OB_MESH) {
            BKE_reportf(op->reports, RPT_ERROR, 
                       "Object '%s' is not a mesh", objects[i]->id.name + 2);
            return OPERATOR_CANCELLED;
        }
        
        if (objects[i]->data == NULL) {
            BKE_reportf(op->reports, RPT_ERROR, 
                       "Object '%s' has no mesh data", objects[i]->id.name + 2);
            return OPERATOR_CANCELLED;
        }
    }
    
    // Safe to proceed
    // ... perform join operation
    return OPERATOR_FINISHED;
}
```

## Python Operator Validation

### Python Poll Functions

```python
import bpy
from bpy.types import Operator

class OBJECT_OT_my_operator(Operator):
    bl_idname = "object.my_operator"
    bl_label = "My Operator"
    
    @classmethod
    def poll(cls, context):
        # Context validation
        if context.active_object is None:
            return False
        
        # Type validation
        if context.active_object.type != 'MESH':
            return False
        
        # Mode validation
        if context.active_object.mode != 'OBJECT':
            return False
        
        # Data validation
        if context.active_object.data is None:
            return False
        
        return True
    
    def execute(self, context):
        # Defensive checks
        ob = context.active_object
        if ob is None or ob.type != 'MESH':
            self.report({'ERROR'}, "No active mesh object")
            return {'CANCELLED'}
        
        # Safe to proceed
        # ... perform operation
        return {'FINISHED'}
```

### Python Selection Validation

```python
@classmethod
def poll(cls, context):
    # Check selection count
    if len(context.selected_objects) < 2:
        return False
    
    # Validate all selected objects
    for ob in context.selected_objects:
        if ob.type != 'MESH':
            return False
        
        if ob.data is None:
            return False
    
    return True
```

### Python RNA Validation

```python
def execute(self, context):
    ob = context.active_object
    if ob is None:
        return {'CANCELLED'}
    
    # RNA type check
    if not isinstance(ob, bpy.types.Object):
        self.report({'ERROR'}, "Active object is not an Object")
        return {'CANCELLED'}
    
    # RNA property check
    if 'location' not in ob.bl_rna.properties:
        self.report({'ERROR'}, "Object has no location property")
        return {'CANCELLED'}
    
    # Safe to access property
    ob.location.x = 1.0
    return {'FINISHED'}
```

## Validation Best Practices

### 1. Always Use Poll Functions

```c
// ✅ GOOD: Poll function validates
static bool operator_poll(bContext *C) {
    return CTX_data_active_object(C) != NULL;
}

// ❌ BAD: No poll function
// Operator might fail at runtime
```

### 2. Validate in Execute Too

```c
// ✅ GOOD: Double-check in execute
static int operator_exec(bContext *C, wmOperator *op) {
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return OPERATOR_CANCELLED;
    }
    // ...
}

// ❌ BAD: Assume poll passed
static int operator_exec(bContext *C, wmOperator *op) {
    Object *ob = CTX_data_active_object(C);
    ob->loc[0] = 1.0;  // Might crash if ob is NULL
}
```

### 3. Use Context Accessors

```c
// ✅ GOOD: Use validated accessors
Object *ob = CTX_data_active_object(C);
Mesh *mesh = CTX_data_edit_mesh(C);

// ❌ BAD: Direct access (no validation)
Object *ob = C->active_object;  // Might be NULL
```

### 4. Check RNA Types

```c
// ✅ GOOD: RNA type checking
PointerRNA ptr;
RNA_id_pointer_create(&object->id, &ptr);
if (!RNA_struct_is_a(&ptr, &RNA_Object)) {
    return OPERATOR_CANCELLED;
}

// ❌ BAD: Assume type
Object *ob = (Object *)some_id;  // Might not be Object
```

### 5. Provide User Feedback

```c
// ✅ GOOD: Informative error messages
if (ob == NULL) {
    BKE_report(op->reports, RPT_ERROR, "No active object");
    return OPERATOR_CANCELLED;
}

// ❌ BAD: Silent failure
if (ob == NULL) {
    return OPERATOR_CANCELLED;  // User doesn't know why
}
```

## Common Validation Patterns

### Pattern 1: Edit Mode Operator

```c
static bool edit_mode_operator_poll(bContext *C)
{
    Object *ob = CTX_data_active_object(C);
    return (ob != NULL && 
            ob->type == OB_MESH && 
            ob->mode == OB_MODE_EDIT &&
            ob->data != NULL);
}
```

### Pattern 2: Material Operator

```c
static bool material_operator_poll(bContext *C)
{
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;
    }
    
    // Check object can have materials
    if (ob->type != OB_MESH && 
        ob->type != OB_CURVE) {
        return false;
    }
    
    // Check has material slots
    if (ob->totcol == 0) {
        return false;
    }
    
    return true;
}
```

### Pattern 3: Animation Operator

```c
static bool animation_operator_poll(bContext *C)
{
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;
    }
    
    // Check has animation data
    if (ob->adt == NULL || ob->adt->action == NULL) {
        return false;
    }
    
    return true;
}
```

## Conclusion

Blender's operator input validation system provides:

1. **Poll Functions**: Pre-execution availability checks
2. **Context Validation**: Runtime context state verification
3. **Selection Validation**: Selection state checks
4. **RNA Type Checking**: Runtime type information validation
5. **Defensive Checks**: Additional validation in execute

This multi-layer approach ensures:
- **Safety**: Operators don't run on incompatible data
- **User Experience**: Operators are disabled when unavailable
- **Stability**: Prevents crashes from invalid operations
- **Clarity**: Users understand why operations aren't available

Understanding this validation system is essential for:
- Writing reliable Blender operators
- Debugging operator availability issues
- Creating user-friendly interfaces
- Maintaining system stability

The combination of poll functions, context checks, selection validation, and RNA type checking creates a robust system that prevents invalid operations while providing clear feedback to users.

