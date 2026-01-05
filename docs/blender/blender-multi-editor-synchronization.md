# Blender Multi-Editor Synchronization: Dependency Graph Updates and Redraw Notifications

## Overview

Blender's multi-editor architecture allows users to work with the same data in different views simultaneously (3D Viewport, Graph Editor, Outliner, Properties Panel, etc.). This document explains how Blender keeps all editors synchronized through dependency graph updates and a sophisticated redraw notification system.

## The Multi-Editor Challenge

### Problem Statement

When a user modifies data in one editor, all other editors showing that data must update:
- **3D Viewport**: Shows 3D representation of objects
- **Graph Editor**: Shows animation curves (F-curves)
- **Outliner**: Shows scene hierarchy
- **Properties Panel**: Shows object properties
- **Material Editor**: Shows material settings
- **Texture Editor**: Shows texture data
- **Timeline**: Shows keyframes and playback

### Synchronization Requirements

1. **Consistency**: All editors show the same data state
2. **Performance**: Updates should be efficient (not redraw everything)
3. **Responsiveness**: Updates should be immediate or near-immediate
4. **Selective**: Only affected editors should update

## Architecture Overview

### Synchronization Flow

```
User Action (e.g., move object)
  ↓
Data-Block Modified
  ↓
Dependency Graph Tagged
  ↓
DEG Evaluation Scheduled
  ↓
Notifiers Sent to All Editors
  ↓
Editors Check Notifiers
  ↓
Affected Editors Redraw
```

### Key Components

1. **Dependency Graph (DEG)**: Tracks data dependencies and evaluates changes
2. **Notifier System**: Broadcasts change notifications to editors
3. **Editor Update Handlers**: Each editor listens for relevant notifications
4. **Redraw System**: Manages when and how editors redraw

## Dependency Graph Updates

### DEG as Central Coordinator

The Dependency Graph serves as the central coordinator for all data changes:

```c
// When data changes
object->loc[0] = 1.0;

// DEG is notified
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);

// DEG evaluates changes
DEG_evaluate_on_framechange(depsgraph, scene, frame);

// DEG triggers notifiers
WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, object);
```

### DEG Evaluation and Editor Updates

DEG evaluation produces evaluated data that editors use:

```c
// Original data (user edits this)
Object *object_orig = scene->objects.first;

// Evaluated data (editors read this)
Object *object_eval = DEG_get_evaluated_object(depsgraph, object_orig);

// All editors use evaluated data
// - 3D Viewport: object_eval->loc
// - Properties Panel: object_eval->loc
// - Outliner: object_eval->name
```

### DEG Update Propagation

When DEG updates, it propagates to all dependent editors:

```
DEG Evaluation
  ↓
Evaluated Data Updated
  ↓
Notifiers Sent
  ↓
3D Viewport: Updates object position
  ↓
Properties Panel: Updates location fields
  ↓
Outliner: Updates object name/icon if needed
  ↓
Graph Editor: Updates F-curve display if animated
```

## Notifier System

### What Are Notifiers?

Notifiers are messages that inform editors about data changes. They're the communication mechanism between the data layer and UI layer.

### Notifier Structure

```c
typedef struct wmNotifier {
    struct wmNotifier *next, *prev;
    
    // Notifier category and action
    unsigned int category;  // NC_OBJECT, NC_SCENE, etc.
    unsigned int action;    // NA_EDITED, NA_ADDED, etc.
    unsigned int data;      // ND_TRANSFORM, ND_GEOMETRY, etc.
    
    // Reference to changed data
    void *reference;       // Pointer to changed object/data-block
    
    // Additional data
    int subtype;           // Sub-category of change
} wmNotifier;
```

### Notifier Categories

```c
typedef enum {
    NC_WM = 0,           // Window manager
    NC_SCENE = 1,        // Scene changes
    NC_OBJECT = 2,       // Object changes
    NC_MATERIAL = 3,     // Material changes
    NC_TEXTURE = 4,      // Texture changes
    NC_GEOM = 5,         // Geometry changes
    NC_ANIMATION = 6,    // Animation changes
    NC_SPACE = 7,        // Space (editor) changes
    NC_WINDOW = 8,       // Window changes
    NC_ID = 9,           // ID data-block changes
    // ... more categories
} NotifierCategory;
```

### Notifier Actions

```c
typedef enum {
    NA_EDITED = 0,        // Data was edited
    NA_ADDED = 1,        // Data was added
    NA_REMOVED = 2,      // Data was removed
    NA_SELECTED = 3,     // Selection changed
    NA_ACTIVATED = 4,    // Object activated
    NA_RENAME = 5,       // Name changed
    NA_DEACTIVATED = 6,  // Object deactivated
    // ... more actions
} NotifierAction;
```

### Notifier Data Types

```c
// For NC_OBJECT category
typedef enum {
    ND_TRANSFORM = 0,    // Transform changed
    ND_GEOMETRY = 1,     // Geometry changed
    ND_SHADING = 2,      // Shading changed
    ND_DRAW = 3,         // Draw/display changed
    ND_MODIFIER = 4,     // Modifier changed
    ND_KEYS = 5,         // Keys changed
    ND_CONSTRAINT = 6,   // Constraint changed
    ND_POSE = 7,         // Pose changed
    ND_LIGHT = 8,        // Light changed
    ND_LAYER = 9,        // Layer changed
    // ... more data types
} NotifierData;
```

### Sending Notifiers

```c
// Basic notifier
WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, object);

// Notifier with action
WM_event_add_notifier_ex(C, 
    &(const wmNotifier){
        .category = NC_OBJECT,
        .action = NA_EDITED,
        .data = ND_TRANSFORM,
        .reference = object
    });

// Multiple notifiers
WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, object);
WM_event_add_notifier(C, NC_SCENE | ND_LAYER, scene);
```

## Editor Update Handlers

### How Editors Listen

Each editor registers handlers for specific notifiers:

```c
// Editor registration
static void view3d_listener(wmSpaceType *st)
{
    // Listen for object transform changes
    wmWindowManager *wm = st->wm;
    LISTENER(OBJECT, TRANSFORM, view3d_object_transform_update);
    LISTENER(OBJECT, GEOMETRY, view3d_object_geometry_update);
    LISTENER(SCENE, LAYER, view3d_scene_layer_update);
    // ... more listeners
}
```

### Editor Update Functions

Each editor has update functions for different notifier types:

```c
// 3D Viewport update handler
static void view3d_object_transform_update(bContext *C, wmNotifier *notifier)
{
    // Check if this editor is affected
    SpaceView3D *s3d = CTX_wm_space_view3d(C);
    if (s3d == NULL) {
        return;  // Not in 3D viewport
    }
    
    // Get changed object
    Object *ob = notifier->reference;
    if (ob == NULL) {
        return;
    }
    
    // Check if object is visible in this viewport
    if (!ED_view3d_object_visible(s3d, ob)) {
        return;  // Not visible, no update needed
    }
    
    // Mark region for redraw
    ARegion *region = CTX_wm_region(C);
    ED_region_tag_redraw(region);
}
```

### Selective Updates

Editors only update when relevant:

```c
// Graph Editor update handler
static void graph_editor_object_transform_update(bContext *C, wmNotifier *notifier)
{
    Object *ob = notifier->reference;
    
    // Only update if object has animation
    if (ob->adt == NULL || ob->adt->action == NULL) {
        return;  // No animation, no update needed
    }
    
    // Check if transform F-curves exist
    if (!graph_editor_has_transform_curves(ob)) {
        return;  // No transform curves, no update
    }
    
    // Update graph editor
    ARegion *region = CTX_wm_region(C);
    ED_region_tag_redraw(region);
}
```

## Editor-Specific Synchronization

### 3D Viewport Synchronization

The 3D Viewport shows the 3D representation of objects:

```c
// 3D Viewport update flow
static void view3d_draw(bContext *C, ARegion *region)
{
    // Get evaluated data from DEG
    Depsgraph *depsgraph = CTX_data_depsgraph(C);
    Scene *scene = CTX_data_scene(C);
    
    // Draw all objects using evaluated data
    Object *ob_eval;
    for (ob_eval = scene->objects.first; ob_eval; ob_eval = ob_eval->id.next) {
        // Use evaluated transform
        float mat[4][4];
        BKE_object_matrix_get(ob_eval, mat);
        
        // Draw object with evaluated transform
        draw_object(ob_eval, mat);
    }
}

// When object transforms change
static void view3d_object_transform_update(bContext *C, wmNotifier *notifier)
{
    // DEG already evaluated new transform
    // Just redraw viewport
    ARegion *region = CTX_wm_region(C);
    ED_region_tag_redraw(region);
}
```

### Graph Editor Synchronization

The Graph Editor shows animation curves (F-curves):

```c
// Graph Editor update flow
static void graph_editor_draw(bContext *C, ARegion *region)
{
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL || ob->adt == NULL) {
        return;
    }
    
    // Get F-curves from animation data
    FCurve *fcu;
    for (fcu = ob->adt->action->curves.first; fcu; fcu = fcu->next) {
        // Draw F-curve
        draw_fcurve(fcu);
    }
}

// When object transforms change
static void graph_editor_object_transform_update(bContext *C, wmNotifier *notifier)
{
    Object *ob = notifier->reference;
    
    // Check if object has transform F-curves
    if (ob->adt && ob->adt->action) {
        // Update F-curve display
        ARegion *region = CTX_wm_region(C);
        ED_region_tag_redraw(region);
    }
}

// When animation data changes
static void graph_editor_animation_update(bContext *C, wmNotifier *notifier)
{
    // Animation changed - update graph editor
    ARegion *region = CTX_wm_region(C);
    ED_region_tag_redraw(region);
}
```

### Outliner Synchronization

The Outliner shows the scene hierarchy:

```c
// Outliner update flow
static void outliner_draw(bContext *C, ARegion *region)
{
    Scene *scene = CTX_data_scene(C);
    
    // Draw scene hierarchy
    draw_scene_tree(scene);
}

// When objects are added/removed
static void outliner_object_update(bContext *C, wmNotifier *notifier)
{
    // Object hierarchy changed
    ARegion *region = CTX_wm_region(C);
    ED_region_tag_redraw(region);
}

// When object names change
static void outliner_object_rename_update(bContext *C, wmNotifier *notifier)
{
    // Object renamed - update outliner
    ARegion *region = CTX_wm_region(C);
    ED_region_tag_redraw(region);
}
```

### Properties Panel Synchronization

The Properties Panel shows object properties:

```c
// Properties Panel update flow
static void properties_panel_draw(bContext *C, ARegion *region)
{
    Object *ob = CTX_data_active_object(C);
    if (ob == NULL) {
        return;
    }
    
    // Get evaluated object
    Depsgraph *depsgraph = CTX_data_depsgraph(C);
    Object *ob_eval = DEG_get_evaluated_object(depsgraph, ob);
    
    // Draw properties using evaluated data
    draw_object_properties(ob_eval);
}

// When object properties change
static void properties_panel_object_update(bContext *C, wmNotifier *notifier)
{
    // Check if changed object is active
    Object *ob = notifier->reference;
    Object *active_ob = CTX_data_active_object(C);
    
    if (ob == active_ob) {
        // Active object changed - update properties panel
        ARegion *region = CTX_wm_region(C);
        ED_region_tag_redraw(region);
    }
}
```

## Redraw System

### Redraw Tagging

Editors don't redraw immediately. Instead, they're tagged for redraw:

```c
// Tag region for redraw
ARegion *region = CTX_wm_region(C);
ED_region_tag_redraw(region);

// Tag entire window for redraw
wmWindow *win = CTX_wm_window(C);
WM_event_add_notifier(C, NC_WINDOW | ND_REDRAW, win);
```

### Redraw Scheduling

Redraws are scheduled and batched:

```c
// Main event loop
while (!WM_window_should_close(window)) {
    // 1. Process events
    wm_event_do_handlers(C);
    
    // 2. Process DEG updates
    if (DEG_needs_update(depsgraph)) {
        DEG_evaluate_on_framechange(depsgraph, scene, frame);
    }
    
    // 3. Process notifiers
    wm_notifier_do_handlers(C);
    
    // 4. Redraw tagged regions
    wm_draw_update(C);
}
```

### Redraw Optimization

Redraws are optimized to avoid unnecessary work:

```c
// Check if redraw is needed
if (region->do_draw == RGN_DRAW) {
    // Region needs redraw
    view3d_draw(C, region);
    region->do_draw = RGN_DRAW_NONE;  // Clear flag
}
```

## Synchronization Patterns

### Pattern 1: Transform Change

```
User moves object in 3D Viewport
  ↓
Object transform modified
  ↓
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM)
  ↓
DEG evaluates new transform
  ↓
WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, object)
  ↓
Notifiers sent to all editors:
  - 3D Viewport: Updates object position (already showing new position)
  - Properties Panel: Updates location fields
  - Graph Editor: Updates F-curve display (if animated)
  - Outliner: No update (name/icon unchanged)
```

### Pattern 2: Geometry Change

```
User edits mesh in Edit Mode
  ↓
Mesh geometry modified
  ↓
BKE_mesh_tag_update(mesh)
DEG_id_tag_update(&mesh->id, DEG_TAG_GEOMETRY)
  ↓
DEG evaluates new geometry
  ↓
WM_event_add_notifier(C, NC_GEOM | ND_DATA, mesh)
  ↓
Notifiers sent to all editors:
  - 3D Viewport: Updates mesh display
  - Properties Panel: Updates geometry info
  - Outliner: No update (structure unchanged)
  - Material Editor: No update (material unchanged)
```

### Pattern 3: Material Change

```
User changes material color
  ↓
Material property modified
  ↓
DEG_id_tag_update(&material->id, DEG_TAG_SHADING)
  ↓
DEG evaluates new shading
  ↓
WM_event_add_notifier(C, NC_MATERIAL | ND_SHADING, material)
  ↓
Notifiers sent to all editors:
  - 3D Viewport: Updates material display
  - Material Editor: Updates color display
  - Properties Panel: Updates material properties
  - Outliner: No update (material name unchanged)
```

### Pattern 4: Selection Change

```
User selects different object
  ↓
Selection changed
  ↓
WM_event_add_notifier(C, NC_OBJECT | ND_SELECT, object)
  ↓
No DEG update needed (no data change)
  ↓
Notifiers sent to all editors:
  - 3D Viewport: Updates selection highlight
  - Properties Panel: Shows new object properties
  - Outliner: Updates selection highlight
  - Graph Editor: Shows new object's F-curves
```

## Update Coordination

### DEG as Single Source of Truth

The Dependency Graph ensures all editors see the same evaluated data:

```c
// All editors read from DEG
Object *ob_orig = scene->objects.first;

// 3D Viewport
Object *ob_eval_3d = DEG_get_evaluated_object(depsgraph_3d, ob_orig);

// Properties Panel
Object *ob_eval_props = DEG_get_evaluated_object(depsgraph_props, ob_orig);

// Both see same evaluated data (assuming same depsgraph)
// ob_eval_3d->loc == ob_eval_props->loc
```

### Per-Editor Depsgraphs

Each editor can have its own depsgraph for optimization:

```c
// 3D Viewport depsgraph (full evaluation)
Depsgraph *depsgraph_view3d = CTX_data_depsgraph_view3d(C);

// Properties Panel depsgraph (minimal evaluation)
Depsgraph *depsgraph_props = CTX_data_depsgraph_props(C);

// Both evaluate from same original data
// But may have different evaluation settings
```

### Update Batching

Updates are batched to avoid excessive redraws:

```c
// Multiple changes
for (int i = 0; i < 100; i++) {
    objects[i]->loc[0] = i;
    DEG_id_tag_update(&objects[i]->id, DEG_TAG_TRANSFORM);
}

// Single DEG evaluation for all changes
DEG_evaluate_on_framechange(depsgraph, scene, frame);

// Single redraw for all affected editors
WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, NULL);
```

## Performance Optimizations

### Selective Redraws

Only affected regions redraw:

```c
// Check if editor is affected
static void editor_update(bContext *C, wmNotifier *notifier)
{
    // Check if this editor shows the changed data
    if (!editor_shows_data(notifier->reference)) {
        return;  // Not affected, skip redraw
    }
    
    // Only redraw if visible
    ARegion *region = CTX_wm_region(C);
    if (region->visible) {
        ED_region_tag_redraw(region);
    }
}
```

### Update Throttling

Updates are throttled to avoid excessive redraws:

```c
// Throttle rapid updates
static double last_update_time = 0.0;
double current_time = PIL_check_seconds_timer();

if (current_time - last_update_time < 0.016) {  // ~60 FPS
    return;  // Skip update (too soon)
}

last_update_time = current_time;
ED_region_tag_redraw(region);
```

### Partial Updates

Some editors support partial updates:

```c
// Partial redraw (only changed area)
static void view3d_partial_redraw(bContext *C, ARegion *region, Object *ob)
{
    // Calculate bounding box of changed object
    BoundBox *bb = BKE_object_boundbox_get(ob);
    
    // Only redraw affected area
    ED_region_tag_redraw_partial(region, bb);
}
```

## Debugging Synchronization

### Checking Notifiers

```c
// Print all notifiers
void debug_print_notifiers(wmWindowManager *wm)
{
    wmNotifier *notifier;
    for (notifier = wm->notifier_queue.first; notifier; notifier = notifier->next) {
        printf("Notifier: category=%d, action=%d, data=%d\n",
               notifier->category, notifier->action, notifier->data);
    }
}
```

### Checking Redraw Tags

```c
// Check if region needs redraw
if (region->do_draw != RGN_DRAW_NONE) {
    printf("Region %s needs redraw\n", region->type->name);
}
```

### Forcing Updates

```c
// Force update all editors
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
DEG_evaluate_on_framechange(depsgraph, scene, scene->r.cfra);
WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, object);
WM_main_add_notifier(C, NC_WINDOW | ND_REDRAW, NULL);
```

## Best Practices

### 1. Always Tag DEG Updates

```c
// After modifying data
object->loc[0] = 1.0;
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);  // REQUIRED
```

### 2. Always Send Notifiers

```c
// After DEG update
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, object);  // REQUIRED
```

### 3. Use Appropriate Notifier Types

```c
// Use specific notifier types
WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, object);  // For transforms
WM_event_add_notifier(C, NC_GEOM | ND_DATA, mesh);            // For geometry
```

### 4. Batch Updates

```c
// Group related changes
for (int i = 0; i < count; i++) {
    modify_data(i);
    DEG_id_tag_update(&data[i]->id, tag);
}
// Single notifier for all
WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, NULL);
```

## Common Issues

### Issue 1: Editor Not Updating

**Problem**: Editor doesn't update after data change.

**Solution**: Check notifier is sent:
```c
// Missing notifier
DEG_id_tag_update(&object->id, DEG_TAG_TRANSFORM);
// Missing: WM_event_add_notifier(C, NC_OBJECT | ND_TRANSFORM, object);
```

### Issue 2: Excessive Redraws

**Problem**: Editors redraw too frequently.

**Solution**: Batch updates and throttle:
```c
// Batch multiple changes
// Send single notifier at end
```

### Issue 3: Stale Data

**Problem**: Editor shows old data.

**Solution**: Use evaluated data from DEG:
```c
// Wrong: Use original data
Object *ob = scene->objects.first;

// Correct: Use evaluated data
Object *ob_eval = DEG_get_evaluated_object(depsgraph, ob);
```

## Conclusion

Blender synchronizes multiple editors through:

1. **Dependency Graph**: Central coordinator for data evaluation
2. **Notifier System**: Broadcasts change notifications
3. **Editor Handlers**: Each editor listens for relevant notifications
4. **Redraw System**: Manages when and how editors redraw

This system provides:
- **Consistency**: All editors show same data state
- **Performance**: Selective updates, batching, throttling
- **Responsiveness**: Immediate or near-immediate updates
- **Flexibility**: Editors can optimize their updates independently

Understanding this synchronization system is essential for:
- Writing Blender add-ons that integrate properly
- Debugging update issues
- Optimizing performance
- Understanding Blender's architecture

The combination of DEG updates and redraw notifications creates a robust, efficient system for keeping multiple editors synchronized while maintaining good performance.

