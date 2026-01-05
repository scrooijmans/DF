# Blender's Runtime Type Correctness: RNA, Poll Functions, and Context Checks

## Overview

Blender is written in C, which lacks the static type system features of languages like C++ or Rust. Instead, Blender enforces type correctness at runtime through a sophisticated system combining RNA (Blender's property/type system), operator poll functions, and context validation. This document explains how these mechanisms work together to provide type safety without compile-time type checking.

## The Challenge: Type Safety in C

### Why Runtime Type Checking?

C provides minimal type safety:
- `void*` pointers can point to anything
- No generics or templates
- No inheritance or polymorphism
- Type information lost at runtime

Blender's solution: **Runtime type checking and validation**

### Blender's Approach

Instead of compile-time type checking, Blender uses:
1. **RNA**: Runtime type information and property validation
2. **Poll Functions**: Operator availability checks
3. **Context Checks**: Runtime validation of operation context
4. **ID Type System**: Runtime type identification for data-blocks

## RNA: Runtime Type Information System

### What is RNA?

RNA (RNA is Not an Acronym) is Blender's runtime type system that provides:
- Type information at runtime
- Property access and validation
- Python API integration
- UI property editing
- Type-safe property getters/setters

### RNA Structure System

RNA defines structures that describe C structs at runtime:

```c
// C struct definition
typedef struct Object {
    ID id;                    // Base ID structure
    float loc[3];            // Location
    float rot[3];            // Rotation
    float scale[3];          // Scale
    struct Mesh *data;       // Mesh pointer
} Object;

// RNA structure definition (runtime type info)
static const StructRNA rna_Object = {
    .identifier = "Object",
    .name = "Object",
    .description = "Object in 3D scene",
    .base = &RNA_ID,
    .properties = (PropertyRNA[]){
        DEFINE_PROP_FLOAT_ARRAY("location", 3, Object, loc),
        DEFINE_PROP_FLOAT_ARRAY("rotation", 3, Object, rot),
        DEFINE_PROP_FLOAT_ARRAY("scale", 3, Object, scale),
        DEFINE_PROP_POINTER("data", Object, data, "Mesh", "Mesh data"),
        {0}
    }
};
```

### RNA Type Validation

RNA validates types at runtime:

```c
// Getting property with type checking
PointerRNA ptr;
RNA_id_pointer_create(&object->id, &ptr);

// Type-safe property access
PropertyRNA *prop = RNA_struct_find_property(&ptr, "location");
if (prop && RNA_property_type(prop) == PROP_FLOAT) {
    // Safe to access as float array
    float loc[3];
    RNA_property_float_get_array(&ptr, prop, loc);
} else {
    // Type mismatch - handle error
    printf("Property 'location' is not a float array\n");
}
```

### RNA Property Types

RNA enforces property types:

```c
typedef enum {
    PROP_BOOLEAN,
    PROP_INT,
    PROP_FLOAT,
    PROP_STRING,
    PROP_ENUM,
    PROP_POINTER,      // Points to another data-block
    PROP_COLLECTION,   // Collection of items
} PropertyType;

// Type checking before access
PropertyRNA *prop = RNA_struct_find_property(&ptr, "location");
if (RNA_property_type(prop) != PROP_FLOAT) {
    return;  // Type mismatch
}
```

### RNA Pointer Validation

RNA validates pointer types:

```c
// Getting object's mesh data
PropertyRNA *data_prop = RNA_struct_find_property(&ptr, "data");
if (data_prop && RNA_property_type(data_prop) == PROP_POINTER) {
    PointerRNA data_ptr;
    RNA_property_pointer_get(&ptr, data_prop, &data_ptr);
    
    // Validate pointer type
    if (RNA_pointer_is_null(&data_ptr)) {
        printf("Object has no mesh data\n");
    } else if (RNA_struct_is_a(&data_ptr, &RNA_Mesh)) {
        // Type check passed - safe to use
        Mesh *mesh = data_ptr.data;
    } else {
        printf("Object data is not a Mesh\n");
    }
}
```

### RNA Subtype Checking

RNA supports inheritance-like type checking:

```c
// Check if pointer is of specific type or subtype
if (RNA_struct_is_a(&ptr, &RNA_Object)) {
    // Is an Object
}

// Check for base types
if (RNA_struct_is_a(&ptr, &RNA_ID)) {
    // Is any ID data-block (Object, Mesh, Material, etc.)
}
```

## Operator Poll Functions

### What Are Poll Functions?

Poll functions determine whether an operator is available in the current context. They provide **runtime type checking** before operators can execute.

### Poll Function Signature

```c
static bool operator_poll(bContext *C)
{
    // Check if operator can run in current context
    // Return false if conditions not met
}
```

### Basic Poll Pattern

```c
static bool delete_object_poll(bContext *C)
{
    // Check if there's an active object
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;  // No object selected
    }
    
    // Check object type
    if (ob->type != OB_MESH) {
        return false;  // Only works on mesh objects
    }
    
    return true;  // Operator available
}
```

### Type-Specific Poll Functions

Poll functions check for specific data types:

```c
static bool edit_mesh_poll(bContext *C)
{
    Object *ob = CTX_data_active_object(C);
    
    // Must have active object
    if (ob == NULL) {
        return false;
    }
    
    // Must be in Edit Mode
    if (ob->mode != OB_MODE_EDIT) {
        return false;
    }
    
    // Object must have mesh data
    if (ob->type != OB_MESH || ob->data == NULL) {
        return false;
    }
    
    // Mesh must be valid
    Mesh *mesh = ob->data;
    if (mesh->totvert == 0) {
        return false;  // Empty mesh
    }
    
    return true;
}
```

### Multi-Object Poll Functions

Poll functions can check multiple objects:

```c
static bool join_meshes_poll(bContext *C)
{
    // Check if multiple mesh objects selected
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

### Context-Dependent Poll Functions

Poll functions check context:

```c
static bool material_assign_poll(bContext *C)
{
    // Check active object
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;
    }
    
    // Check if in correct mode
    if (ob->mode != OB_MODE_OBJECT && ob->mode != OB_MODE_EDIT) {
        return false;  // Only in Object or Edit mode
    }
    
    // Check if object can have materials
    if (ob->type != OB_MESH && 
        ob->type != OB_CURVE && 
        ob->type != OB_SURF) {
        return false;  // Only certain object types
    }
    
    // Check if material slot exists
    if (ob->totcol == 0) {
        return false;  // No material slots
    }
    
    return true;
}
```

### Poll Function Registration

Poll functions are registered with operators:

```c
void OBJECT_OT_delete(wmOperatorType *ot)
{
    ot->name = "Delete";
    ot->idname = "OBJECT_OT_delete";
    ot->description = "Delete selected objects";
    
    ot->exec = delete_object_exec;
    ot->poll = delete_object_poll;  // Register poll function
    
    // ...
}
```

### UI Integration

Poll functions control UI availability:

```c
// In UI code
if (WM_operator_poll(C, ot)) {
    // Operator available - show in menu
    uiItemO(layout, "Delete", ICON_DELETE, "OBJECT_OT_delete");
} else {
    // Operator unavailable - gray out or hide
    uiItemO(layout, "Delete", ICON_DELETE, "OBJECT_OT_delete");
    uiItemDisable(layout, true);
}
```

## Context Checks

### What is Context?

Context (`bContext`) represents the current state:
- Active object
- Selected objects
- Current mode
- Active editor
- Current scene
- etc.

### Context Type Validation

Context provides type-safe accessors:

```c
// Get active object (with type checking)
Object *ob = CTX_data_active_object(C);
if (ob == NULL) {
    return OPERATOR_CANCELLED;  // No active object
}

// Get active object as specific type
Object *ob = CTX_data_active_object(C);
if (ob->type != OB_MESH) {
    return OPERATOR_CANCELLED;  // Wrong type
}
Mesh *mesh = ob->data;
```

### Context Data Accessors

Context provides validated data access:

```c
// Get active mesh (validates type)
Mesh *mesh = CTX_data_edit_mesh(C);
if (mesh == NULL) {
    return OPERATOR_CANCELLED;  // Not in mesh edit mode
}

// Get selected objects (validates selection)
int count = CTX_data_selected_objects_len(C);
Object **objects = CTX_data_selected_objects(C);
```

### Context Mode Checks

Context validates operation mode:

```c
// Check if in correct mode
if (CTX_data_edit_object(C) == NULL) {
    return OPERATOR_CANCELLED;  // Not in edit mode
}

// Check specific mode
Object *ob = CTX_data_active_object(C);
if (ob->mode != OB_MODE_EDIT) {
    return OPERATOR_CANCELLED;  // Not in edit mode
}
```

### Context Editor Checks

Context validates active editor:

```c
// Check if in 3D viewport
SpaceLink *sl = CTX_wm_space_data(C);
if (sl->spacetype != SPACE_VIEW3D) {
    return OPERATOR_CANCELLED;  // Not in 3D viewport
}

// Check if in specific editor
if (CTX_wm_region_view3d(C) == NULL) {
    return OPERATOR_CANCELLED;  // No 3D viewport region
}
```

## ID Type System

### Runtime Type Identification

ID data-blocks include type information:

```c
typedef struct ID {
    // ...
    short type;  // ID type code (OB_MESH, OB_CURVE, etc.)
} ID;

// Type checking macros
#define GS(id) ((id)->name[0] << 8 | (id)->name[1])
#define ID_TYPE(id) GS(id)

// Type checking
if (ID_TYPE(&object->id) == ID_OB) {
    // Is an Object
}
```

### ID Type Codes

```c
typedef enum {
    ID_SCE = MAKE_ID('S', 'C'),  // Scene
    ID_OB = MAKE_ID('O', 'B'),   // Object
    ID_ME = MAKE_ID('M', 'E'),   // Mesh
    ID_CU = MAKE_ID('C', 'U'),   // Curve
    ID_MB = MAKE_ID('M', 'B'),   // Metaball
    ID_MA = MAKE_ID('M', 'A'),   // Material
    ID_TE = MAKE_ID('T', 'E'),   // Texture
    ID_IM = MAKE_ID('I', 'M'),   // Image
    // ... more types
} ID_Type;
```

### Type-Safe ID Casting

```c
// Type-safe casting
ID *id = &object->id;
if (ID_TYPE(id) == ID_OB) {
    Object *ob = (Object *)id;  // Safe cast
}

// Generic ID handling
ID *id = some_id_pointer;
switch (ID_TYPE(id)) {
    case ID_OB:
        Object *ob = (Object *)id;
        // Handle object
        break;
    case ID_ME:
        Mesh *me = (Mesh *)id;
        // Handle mesh
        break;
    default:
        // Unknown type
        break;
}
```

## Combined Type Checking Patterns

### Pattern 1: Operator with Full Validation

```c
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
    
    // 4. Data validation
    Mesh *mesh = ob->data;
    if (mesh == NULL || mesh->totvert == 0) {
        return false;
    }
    
    // 5. Selection check
    if (mesh->edit_mesh->bm->totvertsel == 0) {
        return false;  // No vertices selected
    }
    
    return true;
}

static int extrude_vertices_exec(bContext *C, wmOperator *op)
{
    // Additional runtime checks (defensive programming)
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL || ob->type != OB_MESH) {
        return OPERATOR_CANCELLED;
    }
    
    Mesh *mesh = ob->data;
    if (mesh == NULL) {
        return OPERATOR_CANCELLED;
    }
    
    // Safe to proceed
    // ... perform operation
}
```

### Pattern 2: RNA Property Access with Validation

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
    
    // Validate property exists and is correct type
    PropertyRNA *prop = RNA_struct_find_property(&ptr, "location");
    if (prop == NULL) {
        return OPERATOR_CANCELLED;  // Property doesn't exist
    }
    
    if (RNA_property_type(prop) != PROP_FLOAT) {
        return OPERATOR_CANCELLED;  // Wrong type
    }
    
    if (RNA_property_array_length(&ptr, prop) != 3) {
        return OPERATOR_CANCELLED;  // Wrong array size
    }
    
    // Safe to set property
    float loc[3] = {1.0, 2.0, 3.0};
    RNA_property_float_set_array(&ptr, prop, loc);
    
    return OPERATOR_FINISHED;
}
```

### Pattern 3: Multi-Level Type Checking

```c
static bool apply_modifier_poll(bContext *C)
{
    // Level 1: Context
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;
    }
    
    // Level 2: Object type
    if (ob->type != OB_MESH) {
        return false;
    }
    
    // Level 3: Data existence
    Mesh *mesh = ob->data;
    if (mesh == NULL) {
        return false;
    }
    
    // Level 4: Modifier existence
    if (BLI_listbase_is_empty(&ob->modifiers)) {
        return false;  // No modifiers
    }
    
    // Level 5: Modifier type check
    ModifierData *md = ob->modifiers.first;
    if (md->type == eModifierType_None) {
        return false;  // Invalid modifier
    }
    
    return true;
}
```

## Type Safety Layers

### Layer 1: Poll Functions (Prevention)

Poll functions **prevent** invalid operations from being available:

```c
// Operator not shown in UI if poll returns false
if (!WM_operator_poll(C, ot)) {
    return;  // Don't show operator
}
```

### Layer 2: Context Checks (Validation)

Context checks **validate** current state:

```c
// Validate context before operation
Object *ob = CTX_data_active_object(C);
if (ob == NULL || ob->type != OB_MESH) {
    return OPERATOR_CANCELLED;
}
```

### Layer 3: RNA Validation (Type Safety)

RNA provides **type-safe** property access:

```c
// Type-checked property access
PropertyRNA *prop = RNA_struct_find_property(&ptr, "location");
if (RNA_property_type(prop) != PROP_FLOAT) {
    return OPERATOR_CANCELLED;
}
```

### Layer 4: Runtime Assertions (Defensive)

Runtime assertions catch programming errors:

```c
// Defensive checks
BLI_assert(object != NULL);
BLI_assert(object->type == OB_MESH);
BLI_assert(object->data != NULL);
```

## Error Handling

### Graceful Degradation

When type checks fail, operators fail gracefully:

```c
static int operator_exec(bContext *C, wmOperator *op)
{
    // Type check
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        BKE_report(op->reports, RPT_ERROR, "No active object");
        return OPERATOR_CANCELLED;
    }
    
    if (ob->type != OB_MESH) {
        BKE_report(op->reports, RPT_ERROR, "Active object is not a mesh");
        return OPERATOR_CANCELLED;
    }
    
    // Operation proceeds
    return OPERATOR_FINISHED;
}
```

### User Feedback

Type check failures provide user feedback:

```c
// In poll function
if (ob->type != OB_MESH) {
    // Operator grayed out in UI
    // Tooltip shows: "Requires mesh object"
    return false;
}
```

## Performance Considerations

### Poll Function Efficiency

Poll functions should be fast (called frequently):

```c
// ✅ FAST: Simple checks
static bool simple_poll(bContext *C)
{
    return CTX_data_active_object(C) != NULL;
}

// ❌ SLOW: Complex operations
static bool slow_poll(bContext *C)
{
    // Don't do expensive operations here
    Mesh *mesh = CTX_data_edit_mesh(C);
    if (mesh) {
        // Don't iterate all vertices in poll!
        for (int i = 0; i < mesh->totvert; i++) {
            // Too expensive for poll
        }
    }
    return true;
}
```

### Caching Type Checks

Some type checks can be cached:

```c
// Cache RNA type info
static StructRNA *cached_struct = NULL;
if (cached_struct == NULL) {
    cached_struct = &RNA_Object;
}
```

## Best Practices

### 1. Always Use Poll Functions

```c
// ✅ GOOD: Poll function validates
static bool operator_poll(bContext *C) {
    return CTX_data_active_object(C) != NULL;
}

// ❌ BAD: No poll function
// Operator might fail at runtime
```

### 2. Validate in Exec Too

```c
// ✅ GOOD: Double-check in exec
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
    // No check - might crash if poll didn't run
    ob->loc[0] = 1.0;  // Potential crash
}
```

### 3. Use RNA for Property Access

```c
// ✅ GOOD: Type-safe RNA access
PropertyRNA *prop = RNA_struct_find_property(&ptr, "location");
if (prop && RNA_property_type(prop) == PROP_FLOAT) {
    RNA_property_float_get_array(&ptr, prop, loc);
}

// ❌ BAD: Direct struct access (no type checking)
float *loc = object->loc;  // No validation
```

### 4. Check Context Before Use

```c
// ✅ GOOD: Validate context
Object *ob = CTX_data_active_object(C);
if (ob == NULL || ob->type != OB_MESH) {
    return OPERATOR_CANCELLED;
}

// ❌ BAD: Assume context is valid
Object *ob = CTX_data_active_object(C);
Mesh *mesh = ob->data;  // Might be NULL or wrong type
```

## Comparison with Static Type Systems

### Static Type System (e.g., Rust, C++)

```rust
// Compile-time type checking
fn delete_object(obj: &Object) {
    // Type guaranteed at compile time
    // Compiler prevents type errors
}
```

### Blender's Runtime System

```c
// Runtime type checking
static bool delete_object_poll(bContext *C) {
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL || ob->type != OB_MESH) {
        return false;  // Type checked at runtime
    }
    return true;
}
```

### Trade-offs

**Static Type System:**
- ✅ Compile-time safety
- ✅ Better performance (no runtime checks)
- ❌ Less flexible
- ❌ Requires language support

**Blender's Runtime System:**
- ✅ Works in C
- ✅ Flexible and dynamic
- ✅ Can adapt to user actions
- ❌ Runtime overhead
- ❌ Errors caught at runtime

## Conclusion

Blender enforces type correctness through:

1. **RNA**: Runtime type information and property validation
2. **Poll Functions**: Pre-execution availability checks
3. **Context Checks**: Runtime state validation
4. **ID Type System**: Runtime type identification

This system provides:
- **Type Safety**: Prevents invalid operations
- **User Experience**: Operators only available when valid
- **Error Prevention**: Catches errors before execution
- **Flexibility**: Adapts to dynamic context

While not as strong as compile-time type checking, this runtime system provides effective type safety in C and enables Blender's flexible, user-driven workflow.

