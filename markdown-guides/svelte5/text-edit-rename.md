# Tree View Item Rename - End-to-End Flow

This document describes the complete flow for renaming pipelines and nodes in the tree view, including how we prevent flickering and ensure smooth user experience.

## Overview

The rename functionality allows users to rename pipelines and nodes directly in the tree view using an in-place text editor. The system ensures:

1. **Only one item can be edited at a time** - Global edit state manager prevents conflicts
2. **No flickering** - Realtime updates are skipped while an item is being edited
3. **Click-outside to cancel** - Users can cancel editing by clicking outside the item
4. **Optimized updates** - Name-only changes use O(1) Map lookups instead of full reloads

## Architecture Components

### 1. Global Edit State Manager

**File:** `src/lib/utils/tree-view/edit-state-manager.svelte.ts`

A singleton state manager that tracks which item (pipeline or node) is currently being edited across all tree views.

```typescript
export class TreeViewEditStateManager {
  private currentlyEditingId: string | null = $state(null);
  private editingType: "pipeline" | "node" | null = $state(null);

  isEditing(id: string): boolean {
    return this.currentlyEditingId === id;
  }

  startEditing(id: string, type: "pipeline" | "node"): boolean {
    if (this.currentlyEditingId !== null && this.currentlyEditingId !== id) {
      return false; // Another item is already being edited
    }
    this.currentlyEditingId = id;
    this.editingType = type;
    return true;
  }

  stopEditing(id?: string): void {
    if (id === undefined || this.currentlyEditingId === id) {
      this.currentlyEditingId = null;
      this.editingType = null;
    }
  }
}
```

**Key Features:**

- Ensures only one item can be edited at a time
- Provides reactive state that components can derive from
- Singleton pattern ensures global coordination

### 2. Optimized Data Structure

**File:** `src/lib/components/pages/home/content-main/content-dag-editor/content-pipelines/content-pipeline.svelte.ts`

Uses Map-based lookups for O(1) updates instead of O(n) array operations:

```typescript
export class ContentPipelineState {
  allPipelinesWithNodes = $state<PipelineWithNodes[]>([]);

  // Optimized lookup maps for O(1) updates
  private pipelineLookup = $state<Map<string, PipelineWithNodes>>(new Map());
  private nodeToPipelineMap = $state<Map<string, string>>(new Map());
}
```

**Benefits:**

- Name-only updates don't require full data reload
- Minimal reactive updates (only affected items)
- Better performance for large datasets

## Complete Rename Flow

### Step 1: User Initiates Rename

**File:** `src/lib/components/tree-view/pipeline-tree-item.svelte` or `node-tree-item.svelte`

**User Action:** Right-click → "Rename" from context menu

```typescript
function handleRename() {
  // Prevent if another item is being edited
  if (editStateManager.hasActiveEdit() && !editStateManager.isEditing(id)) {
    return;
  }

  if (editStateManager.startEditing(id, "pipeline")) {
    editedName = name;
    // Focus input after delay to ensure it's rendered
    setTimeout(() => {
      if (inputElementRef && inputElementRef instanceof HTMLInputElement) {
        inputElementRef.focus();
        inputElementRef.select();
      }
    }, 50);
  }
}
```

**What Happens:**

1. Edit state manager checks if another item is being edited
2. If available, marks this item as being edited
3. Sets `editedName` to current name
4. After 50ms delay, focuses and selects input text

**Why 50ms delay?**

- Ensures the Input component is fully rendered before focusing
- Prevents flickering from premature focus attempts
- Allows Svelte's reactivity to complete the render cycle

### Step 2: Input Component Renders

**File:** `src/lib/components/tree-view/pipeline-tree-item.svelte`

```svelte
{#if isEditing}
  <Input
    bind:ref={inputElementRef}
    bind:value={editedName}
    onkeydown={handleNameKeydown}
    onblur={handleNameSubmit}
    class="!text-[10px] {STYLE_CONSTANTS.SPACING.PADDING.MIN} flex-1 min-w-0 !h-6 py-0.5"
  />
{:else}
  <!-- Display name normally -->
{/if}
```

**Key Details:**

- `isEditing` is derived from edit state manager: `let isEditing = $derived(editStateManager.isEditing(id))`
- Uses `bind:ref` (not `bind:this`) because Input component exposes `ref` prop
- Input has smaller font size (`!text-[10px]`) and height (`!h-6`) for compact display

### Step 3: Click-Outside Handler

**File:** `src/lib/components/tree-view/pipeline-tree-item.svelte`

```typescript
$effect(() => {
  if (!isEditing) return;

  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;

    // Don't close if clicking on context menu
    if (
      target.closest("[data-radix-context-menu-content]") ||
      target.closest("[data-radix-context-menu-trigger]") ||
      target.closest("[data-radix-context-menu-item]")
    ) {
      return;
    }

    if (containerElementRef && !containerElementRef.contains(target)) {
      // Clicked outside, cancel editing
      editStateManager.stopEditing(id);
      editedName = "";
    }
  }

  // Use setTimeout to avoid immediate trigger from context menu click
  const timeoutId = setTimeout(() => {
    document.addEventListener("mousedown", handleClickOutside, true);
  }, 150);

  return () => {
    clearTimeout(timeoutId);
    document.removeEventListener("mousedown", handleClickOutside, true);
  };
});
```

**Key Features:**

- Only active when `isEditing` is true
- 150ms delay prevents immediate trigger from context menu click
- Uses capture phase (`true`) for reliable detection
- Excludes context menu elements from triggering cancel
- Properly cleans up event listener on effect cleanup

### Step 4: User Submits Rename

**File:** `src/lib/components/tree-view/pipeline-tree-item.svelte`

**User Action:** Press Enter or blur the input

```typescript
async function handleNameSubmit() {
  if (!editedName.trim() || editedName.trim() === name) {
    editStateManager.stopEditing(id);
    editedName = "";
    return;
  }

  try {
    await updatePipelineName({
      pipelineId: id,
      name: editedName.trim(),
    });
    console.log("[PipelineTreeItem] Pipeline name updated:", editedName.trim());
    editStateManager.stopEditing(id);
    editedName = "";
    // Realtime updates will propagate via ContentPipelineState
  } catch (error) {
    console.error("[PipelineTreeItem] Failed to update pipeline name:", error);
    alert(
      `Failed to update pipeline name: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
    editStateManager.stopEditing(id);
  }
}
```

**What Happens:**

1. Validates the new name (not empty, different from current)
2. Calls `updatePipelineName()` service function
3. Updates database via Supabase
4. Stops editing state
5. Realtime update will arrive shortly

### Step 5: Database Update

**File:** `src/lib/services/pipeline-service.ts` or `node-service.ts`

```typescript
export async function updatePipelineName(params: {
  pipelineId: string;
  name: string;
}): Promise<void> {
  const { error } = await supabase
    .from("pipelines")
    .update({ name: params.name, updated_at: new Date().toISOString() })
    .eq("id", params.pipelineId);
  if (error) {
    throw new Error(`Failed to update pipeline name: ${error.message}`);
  }
}
```

**What Happens:**

- Updates the `name` column in the database
- Updates `updated_at` timestamp
- Supabase triggers a realtime UPDATE event

### Step 6: Realtime Update Arrives

**File:** `src/lib/components/pages/home/content-main/content-dag-editor/content-pipelines/content-pipeline.svelte.ts`

```typescript
// Subscribe to pipelines table changes
const pipelinesChannel = supabase
  .channel("pipelines-changes-state")
  .on(
    "postgres_changes",
    {
      event: "*",
      schema: "public",
      table: "pipelines",
    },
    (payload: any) => {
      // ... check if for current project ...

      if (payload.eventType === "UPDATE") {
        const oldData = payload.old || {};
        const newData = payload.new || {};

        const dagDefinitionChanged =
          JSON.stringify(oldData.dag_definition || {}) !==
          JSON.stringify(newData.dag_definition || {});

        // If only name changed (not dag_definition), update in-place
        if (!dagDefinitionChanged && oldData.name !== newData.name) {
          // ✅ CRITICAL: Check if this pipeline is currently being edited
          const editStateManager = getTreeViewEditStateManager();
          if (editStateManager.isEditing(changedPipelineId)) {
            console.log(
              "[ContentPipelineState] Skipping in-place update - pipeline is being edited:",
              changedPipelineId,
            );
            return; // Skip update to prevent flickering
          }

          // Update using Map for O(1) access
          const pipeline = this.pipelineLookup.get(changedPipelineId);
          if (pipeline) {
            pipeline.name = newData.name;
            // Update the array reference to trigger reactivity
            this.allPipelinesWithNodes = Array.from(
              this.pipelineLookup.values(),
            );
          }
          return;
        }
      }

      // For structural changes, do full reload
      void this.loadAllPipelinesWithNodes();
    },
  )
  .subscribe();
```

**Key Anti-Flickering Mechanism:**

1. **Check Edit State First**: Before updating in-place, check if the item is currently being edited
2. **Skip Update if Editing**: If the item is being edited, skip the realtime update entirely
3. **Why This Works**: The user's edit is already complete and saved. The realtime update is just confirming what we already know. Skipping it prevents:
   - The input from disappearing
   - The name from flickering
   - The focus from being lost

### Step 7: Edit Completes, Next Update Processes Normally

After `editStateManager.stopEditing(id)` is called:

1. The item is no longer marked as being edited
2. The next realtime update (if any) will process normally
3. The in-place update will work correctly
4. The UI reflects the new name smoothly

## Preventing Flickering: Complete Flow

### Scenario: User Renames Pipeline "A" to "B"

```
Time 0ms:   User clicks "Rename"
            → editStateManager.startEditing("pipeline-A", 'pipeline')
            → isEditing = true
            → Input component renders

Time 50ms:  Input receives focus
            → User starts typing "B"

Time 200ms: User presses Enter
            → handleNameSubmit() called
            → updatePipelineName() called
            → Database updated
            → editStateManager.stopEditing("pipeline-A")
            → isEditing = false
            → Input component unmounts

Time 250ms: Realtime UPDATE event arrives
            → ContentPipelineState receives event
            → Checks: editStateManager.isEditing("pipeline-A")?
            → Returns: false (edit already completed)
            → Proceeds with in-place update
            → Updates pipelineLookup Map
            → Updates allPipelinesWithNodes array
            → UI updates smoothly ✅
```

### Scenario: Realtime Update Arrives During Edit

```
Time 0ms:   User clicks "Rename"
            → editStateManager.startEditing("pipeline-A", 'pipeline')
            → isEditing = true

Time 100ms: User is typing new name
            → Realtime UPDATE event arrives (from another user/session)
            → ContentPipelineState receives event
            → Checks: editStateManager.isEditing("pipeline-A")?
            → Returns: true (currently being edited)
            → Skips update ✅
            → User's edit continues uninterrupted

Time 500ms: User presses Enter
            → Edit completes
            → editStateManager.stopEditing("pipeline-A")
            → Next realtime update will process normally
```

## Optimized Update Strategy

### Name-Only Updates (O(1))

When only the name changes (not structural changes like `dag_definition`):

1. **Skip Full Reload**: Don't call `loadAllPipelinesWithNodes()`
2. **Use Map Lookup**: `pipelineLookup.get(id)` - O(1) access
3. **Update In-Place**: Modify the object directly
4. **Trigger Reactivity**: Create new array reference from Map values
5. **Minimal Impact**: Only the affected item updates, not the entire tree

### Structural Updates (Full Reload)

When `dag_definition` or other structural changes occur:

1. **Full Reload**: Call `loadAllPipelinesWithNodes()`
2. **Complete Refresh**: Fetches all pipelines and nodes from database
3. **Rebuild Maps**: Updates both `pipelineLookup` and `nodeToPipelineMap`
4. **Necessary**: Structural changes require complete data refresh

## Key Implementation Details

### 1. Edit State Check Prevents Flickering

```typescript
// ✅ CORRECT: Check edit state before updating
const editStateManager = getTreeViewEditStateManager();
if (editStateManager.isEditing(changedPipelineId)) {
  return; // Skip update - prevents flickering
}
```

**Why This Works:**

- The user's edit is already complete
- The realtime update is redundant
- Skipping prevents UI disruption

### 2. Focus Delay Ensures Proper Rendering

```typescript
setTimeout(() => {
  if (inputElementRef && inputElementRef instanceof HTMLInputElement) {
    inputElementRef.focus();
    inputElementRef.select();
  }
}, 50);
```

**Why 50ms:**

- Gives Svelte time to render the Input component
- Ensures the DOM element exists before focusing
- Prevents focus errors and flickering

### 3. Click-Outside Delay Prevents Immediate Cancel

```typescript
const timeoutId = setTimeout(() => {
  document.addEventListener("mousedown", handleClickOutside, true);
}, 150);
```

**Why 150ms:**

- Context menu click triggers `mousedown` event
- Without delay, click-outside would fire immediately
- Delay allows context menu to close first

### 4. Map-Based Updates for Performance

```typescript
// O(1) lookup instead of O(n) array search
const pipeline = this.pipelineLookup.get(changedPipelineId);
if (pipeline) {
  pipeline.name = newData.name;
  this.allPipelinesWithNodes = Array.from(this.pipelineLookup.values());
}
```

**Benefits:**

- Fast updates even with many pipelines
- Minimal reactive updates
- Better performance for large datasets

## Error Handling

### Validation

- Empty names are rejected
- Names unchanged from current are rejected
- Database errors are caught and displayed to user

### Edge Cases

- **Multiple rapid renames**: Edit state manager prevents simultaneous edits
- **Realtime update during edit**: Edit state check skips update
- **Click-outside during edit**: Properly cancels without saving
- **Network errors**: User sees error message, edit state cleared

## Summary

The rename flow uses a combination of:

1. **Global Edit State Manager** - Ensures only one item edited at a time
2. **Edit State Checks** - Prevents realtime updates from interfering with active edits
3. **Optimized Updates** - O(1) Map lookups for name-only changes
4. **Proper Timing** - Delays for focus and click-outside prevent race conditions
5. **Click-Outside Support** - Natural UX for canceling edits

This architecture ensures smooth, flicker-free renaming with optimal performance.
