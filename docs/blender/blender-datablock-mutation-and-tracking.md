# Blender Data-Block Mutation, Change Tracking, and Restrictions

## Overview

Blender's data-block mutation system is tightly integrated with the Dependency Graph (DEG) to ensure consistency, performance, and correctness. Understanding how mutations are tracked, when they're allowed, and when they're restricted or deferred is crucial for understanding Blender's architecture.

## Data-Block Mutation Fundamentals

### Direct Mutation Model

Blender uses a **direct mutation model** where data-blocks are modified in-place:

```c
// Example: Modifying mesh vertices
Mesh *mesh = object->data;
MVert *vert = &mesh->mvert[0];
vert->co[0] = 1.0f;  // Direct mutation
```

### Mutation Categories

1. **Structural Mutations**: Changes to data-block structure
   - Adding/removing vertices, edges, faces
   - Adding/removing material slots
   - Modifying array sizes

2. **Value Mutations**: Changes to data values
   - Vertex coordinates
   - Material properties (color, roughness, etc.)
   - Object transforms

3. **Reference Mutations**: Changes to pointers/references
   - Assigning new materials
   - Linking objects to different meshes
   - Changing parent relationships

## Dependency Graph Change Tracking

### Invalidation System

The Dependency Graph tracks changes through an **invalidation system**:

#### Invalidation Tags

```c
typedef enum {
    DEG_TAG_INVALID = 0,
    DEG_TAG_DO_NOT_TAG = 1,
    DEG_TAG_TRANSFORM = (1 << 1),
    DEG_TAG_GEOMETRY = (1 << 2),
    DEG_TAG_TIME = (1 << 3),
    DEG_TAG_COPY_ON_WRITE = (1 << 4),
    // ... more tags
} DEGUpdateTag;
```

#### Invalidation Propagation

When a data-block is mutated, the DEG marks it and its dependents as invalid:

```
Mutation: Mesh vertex moved
  ↓
Mesh marked with DEG_TAG_GEOMETRY
  ↓
DEG traverses dependency graph
  ↓
All dependent objects marked invalid
  ↓
Evaluation scheduled for next update
```

### Change Detection Mechanisms

#### 1. Explicit Invalidation

Code explicitly marks data-blocks as changed:

```c
// After modifying mesh geometry
BKE_mesh_tag_update(mesh);
// or
DEG_id_tag_update(&mesh->id, DEG_TAG_GEOMETRY);
```

#### 2. Automatic Detection

Some operations automatically trigger invalidation:
- Property updates via RNA (Blender's Python API)
- Operator execution
- Animation system updates

#### 3. Copy-on-Write Detection

Copy-on-Write (COW) system detects when shared data needs duplication:

```c
// When multiple objects share a mesh
// and one needs modification:
Mesh *mesh = object->data;
if (mesh->id.us > 1) {
    // Create copy for this object
    mesh = BKE_mesh_copy_for_eval(mesh);
    object->data = mesh;
}
```

### Dependency Graph Evaluation

#### Evaluation Phases

1. **Tagging Phase**: Mark what needs updating
   ```c
   DEG_id_tag_update(&mesh->id, DEG_TAG_GEOMETRY);
   ```

2. **Scheduling Phase**: Determine evaluation order
   ```c
   DEG_evaluate_on_framechange(depsgraph, scene, frame);
   ```

3. **Evaluation Phase**: Actually compute new values
   ```c
   // Evaluated data stored separately from original
   Object *object_eval = DEG_get_evaluated_object(depsgraph, object);
   ```

#### Evaluated vs. Original Data

Blender maintains **two versions** of data:

- **Original Data**: The source data-blocks (user-editable)
- **Evaluated Data**: Computed results (for display/rendering)

```
Original Mesh (user edits this)
  ↓
[Modifiers applied]
  ↓
Evaluated Mesh (viewport shows this)
```

## Mutation Restrictions

### Read-Only Contexts

#### During Evaluation

Data-blocks are **read-only** during dependency graph evaluation:

```c
// ❌ NOT ALLOWED during evaluation
void modifier_callback(Object *ob) {
    Mesh *mesh = ob->data;
    mesh->mvert[0].co[0] = 1.0;  // Mutation during eval!
}

// ✅ CORRECT: Write to evaluated data
void modifier_callback(Object *ob) {
    Mesh *mesh_eval = ob->runtime.mesh_eval;
    mesh_eval->mvert[0].co[0] = 1.0;  // OK: evaluated data
}
```

#### During Rendering

Original data-blocks cannot be mutated during rendering:
- Render engine reads evaluated data
- Original data must remain stable
- Changes deferred until render completes

### Thread Safety Restrictions

#### Main Thread Only

Most data-block mutations must occur on the **main thread**:

```c
// ❌ NOT ALLOWED from worker thread
void worker_thread_function() {
    mesh->mvert[0].co[0] = 1.0;  // Thread safety violation!
}

// ✅ CORRECT: Queue mutation for main thread
void worker_thread_function() {
    // Queue update for main thread
    WM_jobs_customdata_set(job, mesh, vertex_index, new_value);
}
```

#### Protected Operations

Certain operations require exclusive access:
- Structural changes (add/remove geometry)
- Reference changes (assigning materials)
- ID data-block deletion

### Library-Linked Data Restrictions

#### External Data-Blocks

Data-blocks linked from external files (`LIB_EXTERN` flag) are **read-only**:

```c
if (mesh->id.lib != NULL) {
    // Cannot directly modify external mesh
    // Must create local override first
    mesh = BKE_mesh_make_local(mesh);
}
```

#### Override System

To modify external data, Blender creates **local overrides**:

```
External Mesh (read-only)
  ↓
Create Override
  ↓
Local Override Mesh (mutable)
  ↓
Changes stored separately
```

## Mutation Deferral

### Deferred Updates

Blender defers many mutations to optimize performance:

#### 1. Batch Updates

Multiple mutations are batched together:

```c
// Multiple vertex moves
for (int i = 0; i < 1000; i++) {
    mesh->mvert[i].co[0] += 1.0;
}
// Only one invalidation at the end
BKE_mesh_tag_update(mesh);
```

#### 2. Frame-Based Deferral

Some mutations are deferred until frame change:

```c
// Animation updates
// Changes accumulate during frame
// Evaluated once per frame
DEG_evaluate_on_framechange(depsgraph, scene, frame);
```

#### 3. Viewport Update Deferral

Viewport updates are deferred to avoid excessive redraws:

```c
// Multiple property changes
RNA_float_set(ptr, "location_x", 1.0);
RNA_float_set(ptr, "location_y", 2.0);
RNA_float_set(ptr, "location_z", 3.0);
// Single viewport update at end
WM_main_add_notifier(NC_OBJECT | ND_TRANSFORM, object);
```

### Update Batching

#### Notifier System

Blender uses a **notifier system** to batch UI updates:

```c
// Multiple changes
object->loc[0] = 1.0;
object->loc[1] = 2.0;
object->rot[0] = 0.5;

// Single notifier at end
WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, object);
```

#### Update Regions

Updates are batched by region:
- 3D Viewport updates
- Properties panel updates
- Outliner updates
- All batched per region

### Lazy Evaluation

#### On-Demand Computation

Some data is computed **lazily** (only when needed):

```c
// Bounding box not computed until requested
BoundBox *bb = BKE_object_boundbox_get(object);
// Computed on first access, cached afterward
```

#### Cache Invalidation

Cached computed data is invalidated when dependencies change:

```
Mesh modified
  ↓
Bounding box cache invalidated
  ↓
Next access recomputes bounding box
```

## Mutation Patterns

### Safe Mutation Pattern

The standard pattern for safe mutations:

```c
// 1. Check if mutation is allowed
if (mesh->id.lib != NULL) {
    mesh = BKE_mesh_make_local(mesh);
}

// 2. Perform mutation
mesh->mvert[0].co[0] = new_value;

// 3. Tag for update
BKE_mesh_tag_update(mesh);
DEG_id_tag_update(&mesh->id, DEG_TAG_GEOMETRY);

// 4. Notify UI
WM_event_add_notifier(C, NC_GEOM | ND_DATA, mesh);
```

### Copy-on-Write Pattern

When sharing needs to be broken:

```c
// Check if data is shared
if (mesh->id.us > 1) {
    // Create copy
    mesh = BKE_mesh_copy_for_eval(mesh);
    // Update reference
    object->data = mesh;
}

// Now safe to mutate
mesh->mvert[0].co[0] = new_value;
```

### Deferred Evaluation Pattern

For expensive operations:

```c
// Mark as needing update
DEG_id_tag_update(&mesh->id, DEG_TAG_GEOMETRY);

// Don't evaluate immediately
// Evaluation happens later in update cycle
// User can continue working
```

## Performance Optimizations

### Update Minimization

#### Change Detection

Blender minimizes updates by detecting actual changes:

```c
float old_value = mesh->mvert[0].co[0];
float new_value = compute_new_value();

if (old_value != new_value) {
    mesh->mvert[0].co[0] = new_value;
    BKE_mesh_tag_update(mesh);
}
```

#### Dirty Flags

Data-blocks use **dirty flags** to track what changed:

```c
typedef struct Mesh {
    // ...
    int flag;  // ME_FLAG_DIRTY, ME_FLAG_DIRTY_NORMALS, etc.
} Mesh;

// Only recompute what's dirty
if (mesh->flag & ME_FLAG_DIRTY_NORMALS) {
    BKE_mesh_calc_normals(mesh);
    mesh->flag &= ~ME_FLAG_DIRTY_NORMALS;
}
```

### Evaluation Caching

#### Result Caching

Evaluated results are cached:

```
Original Mesh
  ↓
[Evaluation]
  ↓
Evaluated Mesh (cached)
  ↓
If original unchanged, reuse cached result
```

#### Cache Invalidation

Cache invalidated when dependencies change:

```
Dependency changed
  ↓
Cache marked invalid
  ↓
Next evaluation recomputes
```

## Threading Considerations

### Main Thread Mutations

Most mutations must be on main thread:

```c
// ✅ Main thread
void operator_execute(bContext *C) {
    // Safe to mutate
    object->loc[0] = 1.0;
}

// ❌ Worker thread
void modifier_thread() {
    // Cannot mutate original data
    // Must write to evaluated data only
}
```

### Parallel Evaluation

Dependency graph can evaluate in parallel:

```
Independent branches evaluated in parallel:
  Object A → Mesh A → Material A  } Parallel
  Object B → Mesh B → Material B  } evaluation
  Object C → Mesh C → Material C  }
```

But mutations to original data remain main-thread only.

## Undo/Redo Integration

### Undo System

Mutations are tracked for undo:

```c
// Before mutation
UndoStack_push(undo_stack, mesh);

// Perform mutation
mesh->mvert[0].co[0] = new_value;

// Mutation tracked automatically
// Undo restores previous state
```

### Undo Granularity

Undo operates at the **data-block level**:

```
User action: Move 1000 vertices
  ↓
Single undo step created
  ↓
Undo restores entire mesh state
```

## Best Practices

### Mutation Guidelines

1. **Always tag updates**: After mutation, tag for DEG update
2. **Check restrictions**: Verify mutation is allowed in current context
3. **Handle sharing**: Use COW when data is shared
4. **Batch operations**: Group related mutations
5. **Use notifiers**: Notify UI of changes

### Performance Tips

1. **Minimize invalidations**: Only tag what actually changed
2. **Batch updates**: Group multiple changes
3. **Use dirty flags**: Track what needs recomputation
4. **Leverage caching**: Let DEG cache evaluated results
5. **Defer when possible**: Use deferred evaluation for expensive ops

## Common Pitfalls

### Mutation During Evaluation

```c
// ❌ WRONG
void modifier_apply(Object *ob) {
    Mesh *mesh = ob->data;
    mesh->mvert[0].co[0] = 1.0;  // Mutating original during eval!
}

// ✅ CORRECT
void modifier_apply(Object *ob) {
    Mesh *mesh_eval = ob->runtime.mesh_eval;
    mesh_eval->mvert[0].co[0] = 1.0;  // Mutate evaluated data
}
```

### Forgetting to Tag Updates

```c
// ❌ WRONG
mesh->mvert[0].co[0] = 1.0;
// No update tag - viewport won't refresh!

// ✅ CORRECT
mesh->mvert[0].co[0] = 1.0;
BKE_mesh_tag_update(mesh);
DEG_id_tag_update(&mesh->id, DEG_TAG_GEOMETRY);
```

### Mutating Shared Data

```c
// ❌ WRONG
Mesh *mesh = object1->data;  // Also used by object2
mesh->mvert[0].co[0] = 1.0;  // Affects both objects!

// ✅ CORRECT
if (mesh->id.us > 1) {
    mesh = BKE_mesh_copy_for_eval(mesh);
    object1->data = mesh;
}
mesh->mvert[0].co[0] = 1.0;  // Only affects object1
```

## Conclusion

Blender's mutation system provides:

1. **Direct Mutation**: In-place modification of data-blocks
2. **Change Tracking**: DEG tracks all changes through invalidation
3. **Restrictions**: Mutations restricted during evaluation, rendering, and from worker threads
4. **Deferral**: Updates batched and deferred for performance
5. **Safety**: Copy-on-Write protects shared data
6. **Consistency**: Undo system ensures reversible changes

Understanding these mechanisms is essential for:
- Writing Blender add-ons
- Understanding performance characteristics
- Debugging update issues
- Implementing custom operators

The system balances flexibility (direct mutation) with safety (restrictions and tracking) to maintain data consistency while providing responsive user interaction.

