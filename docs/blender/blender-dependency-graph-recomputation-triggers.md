# Blender Dependency Graph Recomputation Triggers

## Overview

The Dependency Graph (DEG) in Blender is responsible for maintaining consistency between data-blocks and their evaluated results. Understanding what triggers recomputation is essential for performance optimization, debugging update issues, and writing efficient Blender add-ons. This document details all the mechanisms that trigger dependency graph recomputation.

## Dependency Graph Recomputation Fundamentals

### What is Recomputation?

Recomputation is the process by which Blender:
1. Detects changes to original data-blocks
2. Marks affected nodes in the dependency graph as invalid
3. Re-evaluates invalidated nodes in the correct order
4. Updates evaluated data used by viewport, rendering, and other systems

### Recomputation vs. Evaluation

- **Recomputation**: The full cycle of detecting changes and re-evaluating
- **Evaluation**: The actual computation of evaluated data from original data

## Trigger Categories

### 1. Operator Execution

Operators are user actions (keyboard shortcuts, menu items, etc.) that modify the scene.

#### Operator Lifecycle

```c
// Operator execution flow
static int operator_execute(bContext *C, wmOperator *op) {
    // 1. Operator modifies data-blocks
    object->loc[0] = 1.0;
    
    // 2. Operator tags updates (explicit)
    DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
    
    // 3. Operator notifies UI
    WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, object);
    
    return OPERATOR_FINISHED;
}
```

#### Automatic DEG Updates from Operators

Most operators automatically trigger DEG updates:

```c
// Built-in operators automatically tag updates
// Example: Transform operator
WM_operatortype_append(TRANSFORM_OT_translate);
// When executed, automatically:
// - Tags object with DEG_TAG_TRANSFORM
// - Triggers DEG update
// - Updates viewport
```

#### Custom Operator Updates

When writing custom operators, you must explicitly trigger updates:

```c
static int my_custom_operator_exec(bContext *C, wmOperator *op) {
    Object *ob = CTX_data_active_object(C);
    Mesh *mesh = ob->data;
    
    // Modify mesh
    mesh->mvert[0].co[0] = 1.0;
    
    // REQUIRED: Tag for DEG update
    BKE_mesh_tag_update(mesh);
    DEG_id_tag_update(&mesh->id, DEG_TAG_GEOMETRY);
    
    // REQUIRED: Notify UI
    WM_event_add_notifier(C, NC_GEOM | ND_DATA, mesh);
    
    return OPERATOR_FINISHED;
}
```

#### Operator Update Timing

Operators trigger updates at different times:

1. **Immediate Updates**: Critical changes (deletion, structural changes)
   ```c
   // Immediate update
   DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
   DEG_evaluate_on_framechange(depsgraph, scene, scene->r.cfra);
   ```

2. **Deferred Updates**: Non-critical changes (property tweaks)
   ```c
   // Deferred until next update cycle
   DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
   // Update happens in next WM_main event loop iteration
   ```

### 2. Property Changes

Property changes are modifications to RNA properties (Blender's property system).

#### RNA Property System

RNA (RNA = "RNA is Not an Acronym") is Blender's property system that provides:
- Python API access
- UI property editing
- Automatic update notifications

#### Property Change Flow

```
User edits property in UI
  ↓
RNA property setter called
  ↓
Property value changed
  ↓
RNA notifier triggered
  ↓
DEG update tagged
  ↓
DEG recomputation scheduled
```

#### Automatic Property Updates

RNA properties automatically trigger DEG updates:

```python
# Python API
import bpy
obj = bpy.data.objects["Cube"]
obj.location.x = 1.0  # Automatically triggers DEG update
```

```c
// C API
PointerRNA ptr;
RNA_id_pointer_create(&object->id, &ptr);
RNA_float_set(&ptr, "location", 0, 1.0);  // Triggers DEG update
```

#### Property Update Types

Different property types trigger different update tags:

```c
// Transform properties
RNA_float_set(&ptr, "location", 0, 1.0);
// → DEG_TAG_TRANSFORM

// Geometry properties
RNA_int_set(&ptr, "subdivision_level", 2);
// → DEG_TAG_GEOMETRY

// Material properties
RNA_float_set(&ptr, "base_color", 0, 1.0);
// → DEG_TAG_SHADING
```

#### Custom Property Updates

For custom properties, you must manually trigger updates:

```c
// Set custom property
IDProperty *prop = IDP_GetProperty(object->id.properties, "my_prop");
IDP_Float(prop) = 1.0;

// Manually trigger update
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
```

### 3. Scene Updates

Scene updates are periodic refresh cycles that check for changes.

#### Main Event Loop

Blender's main event loop continuously processes updates:

```c
// Simplified main loop
while (!WM_window_should_close(window)) {
    // 1. Process events
    wm_event_do_handlers(C);
    
    // 2. Check for DEG updates
    if (DEG_needs_update(depsgraph)) {
        DEG_evaluate_on_framechange(depsgraph, scene, frame);
    }
    
    // 3. Redraw viewports
    wm_draw_update(C);
}
```

#### Frame Change Updates

Frame changes trigger automatic DEG updates:

```c
// When frame changes
scene->r.cfra = 25;

// Automatically triggers
DEG_evaluate_on_framechange(depsgraph, scene, 25);
```

#### Animation System Updates

The animation system triggers updates when:
- Keyframes are added/removed
- F-curves are modified
- Drivers are evaluated
- Constraints are updated

```c
// Animation evaluation
BKE_animsys_evaluate_animdata(&object->id, animdata, ctime, adt_flag);
// Automatically tags DEG updates for affected objects
```

#### Time-Based Updates

Time-dependent nodes trigger updates:
- Drivers with time-based expressions
- Animation nodes
- Procedural textures with time input

```c
// Driver evaluation
float driver_value = BKE_driver_evaluate(driver, ctime);
// If driver affects object property, DEG update triggered
```

### 4. Modifier Updates

Modifiers are non-destructive operations that modify geometry.

#### Modifier Evaluation

Modifiers are evaluated as part of the dependency graph:

```
Original Mesh
  ↓
[Subdivision Modifier]
  ↓
[Array Modifier]
  ↓
Evaluated Mesh
```

#### Modifier Change Detection

When modifiers change, DEG updates are triggered:

```c
// Adding modifier
ModifierData *md = BKE_modifier_new(eModifierType_Subsurf);
BLI_addtail(&object->modifiers, md);
DEG_id_tag_update(&object->id, DEG_TAG_GEOMETRY);

// Modifying modifier settings
SubsurfModifierData *smd = (SubsurfModifierData *)md;
smd->levels = 3;
DEG_id_tag_update(&object->id, DEG_TAG_GEOMETRY);
```

#### Modifier Stack Invalidation

Changing one modifier invalidates all subsequent modifiers:

```
Mesh → Modifier1 → Modifier2 → Modifier3
                    ↑
              Change Modifier2
                    ↓
        Modifier2 and Modifier3 invalidated
```

### 5. Constraint Updates

Constraints modify object transforms based on relationships.

#### Constraint Types

- **Transform Constraints**: Location, rotation, scale
- **Relationship Constraints**: Parent, follow path
- **Animation Constraints**: Copy transforms, limit rotation

#### Constraint Evaluation

Constraints are evaluated during DEG evaluation:

```c
// Constraint evaluation
BKE_constraints_evaluate(&object->constraints, object, ctime);
// Automatically triggers DEG_TAG_TRANSFORM
```

#### Constraint Change Detection

```c
// Adding constraint
bConstraint *con = BKE_constraint_add_for_object(object, CONSTRAINT_TYPE_TRACKTO);
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);

// Modifying constraint
con->influence = 0.5;
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
```

### 6. Material and Shader Updates

Material changes affect shading and rendering.

#### Material Property Updates

```c
// Changing material color
Material *mat = object->mat[0];
mat->r = 1.0;
mat->g = 0.0;
mat->b = 0.0;
DEG_id_tag_update(&mat->id, DEG_TAG_SHADING);
```

#### Node-Based Materials

Node materials trigger updates when nodes change:

```c
// Modifying shader node
bNode *node = material->nodetree->nodes.first;
node->custom1 = 0.5;  // Some node property
DEG_id_tag_update(&material->id, DEG_TAG_SHADING);
```

#### Texture Updates

Texture changes propagate to materials:

```c
// Changing texture
Tex *tex = material->tex;
tex->noise_depth = 5;
DEG_id_tag_update(&tex->id, DEG_TAG_SHADING);
// Also invalidates materials using this texture
```

### 7. Geometry Changes

Direct geometry modifications trigger DEG updates.

#### Mesh Editing

```c
// Adding vertex
BMVert *v = BM_vert_create(bm, co);
BKE_mesh_tag_update(mesh);
DEG_id_tag_update(&mesh->id, DEG_TAG_GEOMETRY);

// Removing face
BM_face_kill(bm, f);
BKE_mesh_tag_update(mesh);
DEG_id_tag_update(&mesh->id, DEG_TAG_GEOMETRY);
```

#### Curve Editing

```c
// Modifying curve
Curve *cu = object->data;
cu->bevobj = other_object;
DEG_id_tag_update(&cu->id, DEG_TAG_GEOMETRY);
```

#### Armature Updates

```c
// Modifying bone
Bone *bone = armature->bonebase.first;
bone->head[0] = 1.0;
DEG_id_tag_update(&armature->id, DEG_TAG_GEOMETRY);
```

### 8. Relationship Changes

Changes to object hierarchies and references.

#### Parenting

```c
// Setting parent
object->parent = parent_object;
object->partype = PAROBJECT;
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
```

#### Material Assignment

```c
// Assigning material
object->mat[0] = material;
DEG_id_tag_update(&object->id, DEG_TAG_SHADING);
```

#### Object Linking

```c
// Linking object to different mesh
object->data = new_mesh;
DEG_id_tag_update(&object->id, DEG_TAG_GEOMETRY);
```

### 9. External File Updates

Changes to linked external files.

#### Library Reload

```c
// Reloading external library
BKE_library_reload(library, NULL, NULL);
// Triggers DEG update for all linked data-blocks
```

#### Image Reload

```c
// Reloading image
Image *ima = image;
BKE_image_reload(ima);
DEG_id_tag_update(&ima->id, DEG_TAG_SHADING);
```

### 10. Manual Update Triggers

Explicit calls to trigger DEG updates.

#### Direct DEG Updates

```c
// Force immediate update
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
DEG_evaluate_on_framechange(depsgraph, scene, scene->r.cfra);
```

#### Scene Graph Updates

```c
// Update entire scene
DEG_graph_tag_update(depsgraph);
DEG_evaluate_on_framechange(depsgraph, scene, scene->r.cfra);
```

## Update Tag System

### Invalidation Tags

Different types of changes use different tags:

```c
typedef enum {
    DEG_TAG_INVALID = 0,
    DEG_TAG_DO_NOT_TAG = 1,
    DEG_TAG_TRANSFORM = (1 << 1),      // Object transforms
    DEG_TAG_GEOMETRY = (1 << 2),       // Mesh/curve geometry
    DEG_TAG_TIME = (1 << 3),           // Time-dependent changes
    DEG_TAG_COPY_ON_WRITE = (1 << 4),  // COW needed
    DEG_TAG_SHADING = (1 << 5),        // Material/shader changes
    DEG_TAG_ANIMATION = (1 << 6),      // Animation changes
    DEG_TAG_SELECT = (1 << 7),         // Selection changes
} DEGUpdateTag;
```

### Tag Propagation

Tags propagate through the dependency graph:

```
Mesh tagged with DEG_TAG_GEOMETRY
  ↓
All objects using mesh tagged
  ↓
All modifiers on those objects invalidated
  ↓
All constraints on those objects invalidated
```

### Tag Combination

Multiple tags can be combined:

```c
// Combine tags
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM | DEG_TAG_GEOMETRY);
```

## Update Scheduling

### Immediate Updates

Some changes require immediate evaluation:

```c
// Critical operations
if (critical_change) {
    DEG_id_tag_update(&object->id, DEG_TAG_GEOMETRY);
    DEG_evaluate_on_framechange(depsgraph, scene, scene->r.cfra);
    WM_event_add_notifier(C, NC_GEOM | ND_DATA, object);
}
```

### Deferred Updates

Most updates are deferred to the next update cycle:

```c
// Non-critical operations
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
// Update happens in next event loop iteration
```

### Batch Updates

Multiple changes are batched together:

```c
// Multiple property changes
for (int i = 0; i < 100; i++) {
    objects[i]->loc[0] = i;
    DEG_id_tag_update(&objects[i]->id, DEG_TAG_TRANSFORM);
}
// Single evaluation for all changes
DEG_evaluate_on_framechange(depsgraph, scene, scene->r.cfra);
```

## Update Optimization

### Change Detection

Blender minimizes updates by detecting actual changes:

```c
// Only update if value changed
float old_value = object->loc[0];
float new_value = compute_new_value();

if (old_value != new_value) {
    object->loc[0] = new_value;
    DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
}
```

### Dirty Flags

Data-blocks use dirty flags to track what changed:

```c
// Mesh dirty flags
typedef enum {
    ME_FLAG_DIRTY = 1,
    ME_FLAG_DIRTY_NORMALS = 2,
    ME_FLAG_DIRTY_VERTEX_COLORS = 4,
} MeshFlag;

// Only recompute what's dirty
if (mesh->flag & ME_FLAG_DIRTY_NORMALS) {
    BKE_mesh_calc_normals(mesh);
    mesh->flag &= ~ME_FLAG_DIRTY_NORMALS;
}
```

### Evaluation Caching

Evaluated results are cached:

```
Original data unchanged
  ↓
Cached evaluated data reused
  ↓
No recomputation needed
```

## Update Notifiers

### Notifier System

Notifiers inform UI components of changes:

```c
// Notifier types
typedef enum {
    NC_WM = 0,           // Window manager
    NC_SCENE = 1,        // Scene
    NC_OBJECT = 2,       // Object
    NC_MATERIAL = 3,     // Material
    NC_TEXTURE = 4,      // Texture
    NC_GEOM = 5,         // Geometry
    // ... more
} NotifierType;

// Notifier actions
typedef enum {
    NA_EDITED = 0,       // Edited
    NA_ADDED = 1,        // Added
    NA_REMOVED = 2,      // Removed
    NA_SELECTED = 3,     // Selected
    // ... more
} NotifierAction;
```

### Notifier Usage

```c
// Notify UI of change
WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, object);
// UI components listening to this notifier update
```

## Common Update Patterns

### Pattern 1: Property Edit

```
User edits property in UI
  ↓
RNA property setter
  ↓
DEG_id_tag_update()
  ↓
DEG recomputation (deferred)
  ↓
Viewport update
```

### Pattern 2: Operator Execution

```
User executes operator
  ↓
Operator modifies data
  ↓
Operator calls DEG_id_tag_update()
  ↓
Operator calls WM_event_add_notifier()
  ↓
DEG recomputation (immediate or deferred)
  ↓
UI updates
```

### Pattern 3: Animation Playback

```
Frame changes
  ↓
Animation system evaluates
  ↓
Property values updated
  ↓
DEG_id_tag_update() (automatic)
  ↓
DEG recomputation
  ↓
Viewport redraw
```

### Pattern 4: Modifier Change

```
User changes modifier setting
  ↓
Modifier data modified
  ↓
DEG_id_tag_update(&object->id, DEG_TAG_GEOMETRY)
  ↓
DEG recomputation
  ↓
Evaluated mesh updated
  ↓
Viewport shows new geometry
```

## Debugging Update Issues

### Checking Update Tags

```c
// Check if object needs update
if (DEG_id_type_tagged(&object->id)) {
    printf("Object tagged for update\n");
}
```

### Forcing Updates

```c
// Force update
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
DEG_evaluate_on_framechange(depsgraph, scene, scene->r.cfra);
```

### Update Timing

```c
// Check update timing
double start = PIL_check_seconds_timer();
DEG_evaluate_on_framechange(depsgraph, scene, scene->r.cfra);
double end = PIL_check_seconds_timer();
printf("Update took %f seconds\n", end - start);
```

## Best Practices

### 1. Always Tag Updates

```c
// After modifying data
mesh->mvert[0].co[0] = 1.0;
DEG_id_tag_update(&mesh->id, DEG_TAG_GEOMETRY);  // REQUIRED
```

### 2. Use Appropriate Tags

```c
// Use specific tags
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);  // For transforms
DEG_id_tag_update(&mesh->id, DEG_TAG_GEOMETRY);     // For geometry
```

### 3. Batch Updates

```c
// Group related changes
for (int i = 0; i < count; i++) {
    modify_data(i);
    DEG_id_tag_update(&data[i]->id, tag);
}
// Single evaluation
DEG_evaluate_on_framechange(depsgraph, scene, scene->r.cfra);
```

### 4. Use Notifiers

```c
// Notify UI
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, object);
```

## Conclusion

Dependency graph recomputation is triggered by:

1. **Operator Execution**: User actions that modify the scene
2. **Property Changes**: RNA property modifications
3. **Scene Updates**: Frame changes, animation evaluation
4. **Modifier Updates**: Non-destructive operation changes
5. **Constraint Updates**: Relationship constraint changes
6. **Material Updates**: Shading and texture changes
7. **Geometry Changes**: Direct mesh/curve modifications
8. **Relationship Changes**: Parenting, linking, assignments
9. **External Updates**: Library and image reloads
10. **Manual Triggers**: Explicit update calls

Understanding these triggers helps:
- Write efficient Blender add-ons
- Debug update issues
- Optimize performance
- Understand Blender's update cycle

The system is designed to minimize unnecessary recomputation while ensuring consistency across all editors and systems.

