# Blender's Data-Block Architecture and Complete Update Flow

## Overview

This document provides a comprehensive explanation of Blender's data-block (ID) architecture, dependency graph update flow, and the complete call chain from operator execution to UI redraw across multiple editors. Understanding this flow is essential for comprehending how Blender maintains consistency and performance.

## Data-Block (ID) Architecture

### ID Data-Block Fundamentals

ID data-blocks are Blender's fundamental unit of data organization. Every entity in a Blender scene is an ID data-block.

#### ID Structure

```c
typedef struct ID {
    // Linked list management
    void *next, *prev;
    
    // ID remapping (for undo/redo)
    struct ID *newid;
    
    // Library reference (for linked data)
    struct Library *lib;
    
    // Name (64 chars + 2-char type prefix)
    char name[66];
    
    // Status flags
    short flag;  // LIB_FAKEUSER, LIB_EXTERN, etc.
    
    // Runtime tags
    int tag;  // For dependency graph invalidation
    
    // Reference count (usage count)
    int us;
    
    // Custom properties
    IDProperty *properties;
} ID;
```

#### ID Type System

```c
// ID type codes (2-character prefixes)
typedef enum {
    ID_SCE = MAKE_ID('S', 'C'),  // Scene
    ID_OB = MAKE_ID('O', 'B'),    // Object
    ID_ME = MAKE_ID('M', 'E'),    // Mesh
    ID_CU = MAKE_ID('C', 'U'),    // Curve
    ID_MB = MAKE_ID('M', 'B'),    // Metaball
    ID_MA = MAKE_ID('M', 'A'),    // Material
    ID_TE = MAKE_ID('T', 'E'),    // Texture
    ID_IM = MAKE_ID('I', 'M'),    // Image
    ID_AR = MAKE_ID('A', 'R'),    // Armature
    ID_AC = MAKE_ID('A', 'C'),    // Action
    ID_NT = MAKE_ID('N', 'T'),    // Node Tree
    // ... more types
} ID_Type;

// Type checking macro
#define GS(id) ((id)->name[0] << 8 | (id)->name[1])
#define ID_TYPE(id) GS(id)
```

#### ID Data-Block Hierarchy

```
Scene (ID_SCE)
  └─> Objects (ID_OB)
        └─> Mesh (ID_ME)
              └─> Material (ID_MA)
                    └─> Texture (ID_TE)
                          └─> Image (ID_IM)
```

### ID Reference System

#### Reference Counting

```c
// Increment reference count
id_us_plus(&material->id);  // material->id.us++
object->mat[0] = material;

// Decrement reference count
id_us_min(&material->id);  // material->id.us--
object->mat[0] = NULL;

// Check if data-block can be deleted
if (material->id.us == 0 && !(material->id.flag & LIB_FAKEUSER)) {
    // Can be deleted
}
```

#### Reference Types

1. **Direct References**: Explicit pointer assignments
   ```c
   object->data = mesh;  // Object references mesh
   ```

2. **Indirect References**: Through other data-blocks
   ```c
   object->data = mesh;
   mesh->mat[0] = material;  // Object indirectly references material
   ```

3. **Cyclic References**: Circular dependencies (handled carefully)
   ```c
   node_group_a->nodetree->nodes[0] = node_group_b;
   node_group_b->nodetree->nodes[0] = node_group_a;
   ```

### ID Data-Block Lifecycle

#### Creation

```c
// Create new ID data-block
Mesh *mesh = BKE_mesh_add(bmain, "MyMesh");
// mesh->id.us = 1 (initial reference)
// mesh->id.name = "MEMyMesh"
```

#### Linking

```c
// Link ID to scene
BKE_id_link(bmain, &mesh->id);
// Reference count may increase
```

#### Unlinking

```c
// Unlink ID from scene
BKE_id_unlink(bmain, &mesh->id);
// Reference count decreases
// If us == 0, may be deleted
```

#### Deletion

```c
// Delete ID data-block
BKE_id_delete(bmain, &mesh->id);
// All references become NULL
// Memory freed
```

## Dependency Graph Architecture

### Dependency Graph Structure

The Dependency Graph (DEG) maintains relationships between data-blocks and computes evaluated results.

#### Node Types

```c
typedef enum {
    DEG_NODE_TYPE_OPERATION,     // Operation node (modifier, constraint, etc.)
    DEG_NODE_TYPE_ID,            // ID data-block node
    DEG_NODE_TYPE_PARAMETERS,    // Parameters node
    DEG_NODE_TYPE_TIMESOURCE,    // Time source node
} DEGNodeType;
```

#### Graph Structure

```
Dependency Graph
  ├─> Scene Node
  │     └─> Object Node "Cube"
  │           ├─> Transform Operation
  │           ├─> Mesh Node "Cube_Mesh"
  │           │     ├─> Subdivision Modifier Operation
  │           │     └─> Material Node "Red_Material"
  │           │           └─> Texture Node "Brick_Texture"
  │           │                 └─> Image Node "brick.png"
  │           └─> Constraint Operation
  └─> Time Source Node
```

### Dependency Graph Evaluation

#### Evaluation Phases

1. **Tagging Phase**: Mark what needs updating
   ```c
   DEG_id_tag_update(&mesh->id, DEG_TAG_GEOMETRY);
   ```

2. **Scheduling Phase**: Determine evaluation order
   ```c
   DEG_graph_tag_update(depsgraph);
   DEG_graph_update(depsgraph);
   ```

3. **Evaluation Phase**: Compute evaluated data
   ```c
   DEG_evaluate_on_framechange(depsgraph, scene, frame);
   ```

#### Evaluated Data Storage

```c
// Original data (user edits this)
Object *object_orig = scene->objects.first;

// Evaluated data (computed by DEG)
Object *object_eval = DEG_get_evaluated_object(depsgraph, object_orig);

// Editors use evaluated data
// - 3D Viewport: object_eval->loc
// - Properties Panel: object_eval->loc
// - Graph Editor: object_eval->adt (animation data)
```

## Complete Update Flow: Operator to UI Redraw

### Call Chain Overview

```
User Action (e.g., move object)
  ↓
Operator Execution
  ↓
Data-Block Modification
  ↓
DEG Tagging
  ↓
DEG Evaluation
  ↓
Notifier System
  ↓
Editor Handlers
  ↓
Redraw Tagging
  ↓
UI Redraw
```

### Detailed Call Chain

#### Phase 1: User Input

```c
// User presses G (grab/move) key
// Event system captures input
wmEvent *event = window->event_queue.first;

// Event type: GKEY
// Event value: PRESS
```

#### Phase 2: Operator Invocation

```c
// Window manager processes event
WM_handle_event(C, event);

// Finds operator for G key
wmOperatorType *ot = WM_operatortype_find("TRANSFORM_OT_translate", false);

// Checks if operator can run (poll)
if (WM_operator_poll(C, ot)) {
    // Operator available
    wmOperator *op = WM_operator_new(ot);
    
    // Invoke operator
    WM_operator_invoke(C, ot, event);
}
```

#### Phase 3: Operator Poll

```c
// Poll function checks context
static bool transform_poll(bContext *C)
{
    // Check if object is selected
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return false;
    }
    
    // Check if in correct mode
    if (ob->mode != OB_MODE_OBJECT) {
        return false;
    }
    
    return true;  // Operator can run
}
```

#### Phase 4: Operator Invoke

```c
// Interactive invoke (for modal operations)
static int transform_invoke(bContext *C, wmOperator *op, wmEvent *event)
{
    // Initialize transform operation
    TransformData *tdata = transform_data_create(C, op);
    op->customdata = tdata;
    
    // Enter modal mode
    WM_event_add_modal_handler(C, op);
    
    return OPERATOR_RUNNING_MODAL;
}
```

#### Phase 5: Modal Operation

```c
// Modal operation (continuous updates)
static int transform_modal(bContext *C, wmOperator *op, wmEvent *event)
{
    TransformData *tdata = op->customdata;
    
    switch (event->type) {
        case MOUSEMOVE:
            // Update transform based on mouse
            transform_update_from_mouse(C, op, event);
            
            // Apply transform immediately
            transform_apply(C, op);
            
            return OPERATOR_RUNNING_MODAL;
            
        case LEFTMOUSE:
        case RETKEY:
            // Finish operation
            transform_finish(C, op);
            return OPERATOR_FINISHED;
            
        case ESCKEY:
            // Cancel operation
            transform_cancel(C, op);
            return OPERATOR_CANCELLED;
    }
    
    return OPERATOR_RUNNING_MODAL;
}
```

#### Phase 6: Data Modification

```c
// Apply transform to object
static void transform_apply(bContext *C, wmOperator *op)
{
    TransformData *tdata = op->customdata;
    Object *ob = tdata->ob;
    
    // Modify object transform
    copy_v3_v3(ob->loc, tdata->loc);
    copy_v3_v3(ob->rot, tdata->rot);
    copy_v3_v3(ob->scale, tdata->scale);
    
    // CRITICAL: Tag DEG update
    DEG_id_tag_update(&ob->id, DEG_TAG_TRANSFORM);
    
    // CRITICAL: Send notifier
    WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, ob);
}
```

#### Phase 7: DEG Tagging

```c
// Tag object for update
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);

// Internal DEG processing
void DEG_id_tag_update(ID *id, int flag)
{
    // Find node in dependency graph
    DEGNode *node = DEG_get_node(depsgraph, id);
    if (node == NULL) {
        return;
    }
    
    // Mark node as needing update
    node->flag |= flag;
    
    // Propagate to dependent nodes
    DEG_node_tag_update(depsgraph, node);
}
```

#### Phase 8: DEG Propagation

```c
// Propagate invalidation to dependents
void DEG_node_tag_update(Depsgraph *depsgraph, DEGNode *node)
{
    // Traverse outgoing edges (dependents)
    DEGNodeRelation *rel;
    for (rel = node->outlinks.first; rel; rel = rel->next) {
        DEGNode *dependent = rel->to;
        
        // Mark dependent as invalid
        dependent->flag |= DEG_NODE_FLAG_NEEDS_UPDATE;
        
        // Recursively propagate
        DEG_node_tag_update(depsgraph, dependent);
    }
}
```

#### Phase 9: DEG Evaluation Scheduling

```c
// Main event loop checks for DEG updates
void WM_main(bContext *C)
{
    while (!WM_window_should_close(window)) {
        // Process events
        wm_event_do_handlers(C);
        
        // Check if DEG needs update
        Depsgraph *depsgraph = CTX_data_depsgraph(C);
        if (DEG_needs_update(depsgraph)) {
            // Schedule DEG evaluation
            DEG_evaluate_on_framechange(depsgraph, scene, scene->r.cfra);
        }
        
        // Process notifiers
        wm_notifier_do_handlers(C);
        
        // Redraw
        wm_draw_update(C);
    }
}
```

#### Phase 10: DEG Evaluation

```c
// Evaluate dependency graph
void DEG_evaluate_on_framechange(Depsgraph *depsgraph, Scene *scene, int frame)
{
    // 1. Build evaluation schedule
    DEG_graph_build(depsgraph);
    
    // 2. Evaluate nodes in order
    DEGNode *node;
    for (node = depsgraph->nodes.first; node; node = node->next) {
        if (node->flag & DEG_NODE_FLAG_NEEDS_UPDATE) {
            // Evaluate node
            DEG_node_evaluate(depsgraph, node);
            
            // Clear update flag
            node->flag &= ~DEG_NODE_FLAG_NEEDS_UPDATE;
        }
    }
}

// Evaluate specific node
void DEG_node_evaluate(Depsgraph *depsgraph, DEGNode *node)
{
    switch (node->type) {
        case DEG_NODE_TYPE_ID:
            // Evaluate ID data-block
            DEG_id_node_evaluate(depsgraph, node);
            break;
            
        case DEG_NODE_TYPE_OPERATION:
            // Evaluate operation (modifier, constraint, etc.)
            DEG_operation_node_evaluate(depsgraph, node);
            break;
    }
}

// Evaluate ID data-block
void DEG_id_node_evaluate(Depsgraph *depsgraph, DEGNode *node)
{
    ID *id_orig = node->id_orig;
    ID *id_eval = node->id_eval;
    
    // Copy original to evaluated
    BKE_id_copy_ex(bmain, id_orig, id_eval, 0);
    
    // Apply modifiers, constraints, etc.
    // (handled by operation nodes)
}
```

#### Phase 11: Notifier System

```c
// Send notifier
WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, object);

// Internal notifier processing
void WM_event_add_notifier(bContext *C, unsigned int type, void *reference)
{
    wmWindowManager *wm = CTX_wm_manager(C);
    
    // Create notifier
    wmNotifier *notifier = MEM_callocN(sizeof(wmNotifier), "wmNotifier");
    notifier->category = type & 0xFF;
    notifier->action = (type >> 8) & 0xFF;
    notifier->data = (type >> 16) & 0xFF;
    notifier->reference = reference;
    
    // Add to queue
    BLI_addtail(&wm->notifier_queue, notifier);
}
```

#### Phase 12: Notifier Processing

```c
// Process notifiers in main loop
void wm_notifier_do_handlers(bContext *C)
{
    wmWindowManager *wm = CTX_wm_manager(C);
    wmNotifier *notifier;
    
    // Process all queued notifiers
    for (notifier = wm->notifier_queue.first; notifier; notifier = notifier->next) {
        // Call handlers for this notifier type
        wm_notifier_call_handlers(C, notifier);
    }
    
    // Clear queue
    BLI_freelistN(&wm->notifier_queue);
}

// Call handlers
void wm_notifier_call_handlers(bContext *C, wmNotifier *notifier)
{
    // Find all registered handlers for this notifier type
    wmEventHandler *handler;
    for (handler = wm->handlers.first; handler; handler = handler->next) {
        if (handler->type == notifier->category) {
            // Call handler
            handler->func(C, notifier);
        }
    }
}
```

#### Phase 13: Editor Handlers

```c
// 3D Viewport handler
static void view3d_object_transform_update(bContext *C, wmNotifier *notifier)
{
    SpaceView3D *s3d = CTX_wm_space_view3d(C);
    if (s3d == NULL) {
        return;
    }
    
    Object *ob = notifier->reference;
    
    // Check if object is visible
    if (!ED_view3d_object_visible(s3d, ob)) {
        return;
    }
    
    // Tag region for redraw
    ARegion *region = CTX_wm_region(C);
    ED_region_tag_redraw(region);
}

// Properties Panel handler
static void properties_panel_object_transform_update(bContext *C, wmNotifier *notifier)
{
    Object *ob = notifier->reference;
    Object *active_ob = CTX_data_active_object(C);
    
    // Only update if changed object is active
    if (ob == active_ob) {
        ARegion *region = CTX_wm_region(C);
        ED_region_tag_redraw(region);
    }
}

// Graph Editor handler
static void graph_editor_object_transform_update(bContext *C, wmNotifier *notifier)
{
    Object *ob = notifier->reference;
    
    // Only update if object has animation
    if (ob->adt == NULL || ob->adt->action == NULL) {
        return;
    }
    
    // Check if transform F-curves exist
    if (!graph_editor_has_transform_curves(ob)) {
        return;
    }
    
    ARegion *region = CTX_wm_region(C);
    ED_region_tag_redraw(region);
}

// Outliner handler
static void outliner_object_transform_update(bContext *C, wmNotifier *notifier)
{
    // Outliner doesn't need update for transform changes
    // (only updates for hierarchy/name changes)
    // But we could update selection highlight if needed
}
```

#### Phase 14: Redraw Tagging

```c
// Tag region for redraw
void ED_region_tag_redraw(ARegion *region)
{
    // Set redraw flag
    region->do_draw = RGN_DRAW;
    
    // Mark window as needing redraw
    wmWindow *win = region->win;
    win->redraws |= WIN_REDRAW_REGIONS;
}
```

#### Phase 15: Redraw Processing

```c
// Main loop redraw
void wm_draw_update(bContext *C)
{
    wmWindowManager *wm = CTX_wm_manager(C);
    wmWindow *win;
    
    // Check all windows
    for (win = wm->windows.first; win; win = win->next) {
        if (win->redraws) {
            // Redraw window
            wm_draw_window(C, win);
            win->redraws = 0;
        }
    }
}

// Redraw window
void wm_draw_window(bContext *C, wmWindow *win)
{
    // Redraw all regions that need it
    ARegion *region;
    for (region = win->screen->regionbase.first; region; region = region->next) {
        if (region->do_draw == RGN_DRAW) {
            // Redraw region
            region->type->draw(C, region);
            region->do_draw = RGN_DRAW_NONE;
        }
    }
}
```

#### Phase 16: Editor Redraw

```c
// 3D Viewport redraw
static void view3d_draw(bContext *C, ARegion *region)
{
    // Get evaluated data from DEG
    Depsgraph *depsgraph = CTX_data_depsgraph(C);
    Scene *scene = CTX_data_scene(C);
    
    // Draw all objects using evaluated data
    Object *ob_orig;
    for (ob_orig = scene->objects.first; ob_orig; ob_orig = ob_orig->id.next) {
        // Get evaluated object
        Object *ob_eval = DEG_get_evaluated_object(depsgraph, ob_orig);
        
        // Get evaluated transform
        float mat[4][4];
        BKE_object_matrix_get(ob_eval, mat);
        
        // Draw object
        draw_object_3d(ob_eval, mat);
    }
}

// Properties Panel redraw
static void properties_panel_draw(bContext *C, ARegion *region)
{
    Object *ob_orig = CTX_data_active_object(C);
    if (ob_orig == NULL) {
        return;
    }
    
    // Get evaluated object
    Depsgraph *depsgraph = CTX_data_depsgraph(C);
    Object *ob_eval = DEG_get_evaluated_object(depsgraph, ob_orig);
    
    // Draw properties using evaluated data
    uiLayout *layout = uiLayoutColumn(region->ui, true);
    uiItemR(layout, ob_eval, "location", 0, "Location", ICON_NONE);
    uiItemR(layout, ob_eval, "rotation", 0, "Rotation", ICON_NONE);
    uiItemR(layout, ob_eval, "scale", 0, "Scale", ICON_NONE);
}

// Graph Editor redraw
static void graph_editor_draw(bContext *C, ARegion *region)
{
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL || ob->adt == NULL) {
        return;
    }
    
    // Draw F-curves
    FCurve *fcu;
    for (fcu = ob->adt->action->curves.first; fcu; fcu = fcu->next) {
        // Draw F-curve
        draw_fcurve(region, fcu);
    }
}
```

## Multi-Editor Synchronization Flow

### Example: Moving an Object

```
User Action: Press G, move mouse, release
  ↓
[Phase 1] Event: GKEY PRESS
  ↓
[Phase 2] Operator: TRANSFORM_OT_translate invoked
  ↓
[Phase 3] Poll: Check if object selected (✓)
  ↓
[Phase 4] Invoke: Enter modal mode
  ↓
[Phase 5] Modal: MOUSEMOVE events
  ↓
[Phase 6] Data Modification: object->loc[0] = new_x
  ↓
[Phase 7] DEG Tagging: DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM)
  ↓
[Phase 8] DEG Propagation: Mark dependent nodes invalid
  ↓
[Phase 9] DEG Scheduling: DEG_needs_update() returns true
  ↓
[Phase 10] DEG Evaluation: Compute evaluated transform
  ↓
[Phase 11] Notifier: WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, object)
  ↓
[Phase 12] Notifier Processing: wm_notifier_do_handlers(C)
  ↓
[Phase 13] Editor Handlers:
  - view3d_object_transform_update() → ED_region_tag_redraw(region_3d)
  - properties_panel_object_transform_update() → ED_region_tag_redraw(region_props)
  - graph_editor_object_transform_update() → ED_region_tag_redraw(region_graph)
  ↓
[Phase 14] Redraw Tagging: region->do_draw = RGN_DRAW
  ↓
[Phase 15] Redraw Processing: wm_draw_update(C)
  ↓
[Phase 16] Editor Redraw:
  - view3d_draw() → Shows object at new position
  - properties_panel_draw() → Shows new location values
  - graph_editor_draw() → Updates F-curve display (if animated)
```

### Synchronization Points

1. **DEG Evaluation**: Single source of truth for evaluated data
2. **Notifier System**: Broadcasts changes to all editors
3. **Editor Handlers**: Each editor decides if it needs to update
4. **Redraw System**: Batches redraws for efficiency

## Performance Optimizations

### Update Batching

```c
// Multiple changes batched together
for (int i = 0; i < 100; i++) {
    objects[i]->loc[0] = i;
    DEG_id_tag_update(&objects[i]->id, DEG_TAG_TRANSFORM);
}

// Single DEG evaluation
DEG_evaluate_on_framechange(depsgraph, scene, frame);

// Single notifier
WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, NULL);
```

### Selective Updates

```c
// Editors only update if relevant
static void editor_update(bContext *C, wmNotifier *notifier)
{
    // Check if editor shows this data
    if (!editor_shows_data(notifier->reference)) {
        return;  // Skip update
    }
    
    ED_region_tag_redraw(region);
}
```

### Update Throttling

```c
// Throttle rapid updates
static double last_update = 0.0;
double now = PIL_check_seconds_timer();

if (now - last_update < 0.016) {  // ~60 FPS
    return;  // Skip update
}

last_update = now;
ED_region_tag_redraw(region);
```

## Key Takeaways

1. **ID Data-Blocks**: Fundamental unit of data organization
2. **Dependency Graph**: Maintains relationships and computes evaluated data
3. **Notifier System**: Broadcasts changes to all editors
4. **Editor Handlers**: Selective updates based on relevance
5. **Redraw System**: Batched, efficient UI updates

The complete flow ensures:
- **Consistency**: All editors see same evaluated data
- **Performance**: Batched updates, selective redraws
- **Responsiveness**: Immediate or near-immediate updates
- **Safety**: Proper validation and error handling

Understanding this flow is essential for:
- Writing Blender addons
- Debugging update issues
- Optimizing performance
- Understanding Blender's architecture

