# Advanced Svelte 5 Features: Reactive State Management with `$effect` and Context API

This document explains how MudRock uses advanced Svelte 5 features to create a reactive, cascading state management system where changing the selected project automatically triggers reloads of dependent data (wells, curves, selections, etc.).

## Overview

MudRock implements a **reactive dependency chain** where:

1. **User clicks a project** → `content-projects-item.svelte`
2. **Project state updates** → `PostgresProjectsState.currentSelectedProjectId` changes
3. **Dependent states react** → Wells, curves, and selections automatically reload
4. **No manual coordination needed** → Svelte's reactivity handles everything

## Advanced Svelte 5 Features Used

### 1. `$effect()` Rune - Reactive Side Effects

**Purpose:** Automatically run code when reactive dependencies change.

**How We Use It:**

- Each dependent state class (`PostgresWellsState`, `PostgresCurvesState`, `PostgresCurveMetadataState`, `ContentPipelineState`) uses `$effect()` to watch `projectsState.currentSelectedProjectId`
- When the project ID changes, the effect automatically re-runs, triggering data reloads
- Effects also watch for realtime database updates and react accordingly

**Key Pattern:**

```typescript
$effect(() => {
  // CRITICAL: Read the reactive property directly so $effect tracks it
  const projectId = this.projectsState.currentSelectedProjectId;

  // React to changes
  if (projectId) {
    void this.loadWells();
  }
});
```

**Why This Works:**

- `$effect()` creates a **tracking context** that monitors which reactive values are accessed
- Reading `currentSelectedProjectId` inside the effect makes Svelte track it
- When `currentSelectedProjectId` changes, Svelte automatically re-runs the effect

### 2. `$state()` Rune - Reactive State

**Purpose:** Create reactive state that triggers updates when changed.

**How We Use It:**

- `PostgresProjectsState.currentSelectedProjectId = $state<string | null>(null)`
- When this value changes, all `$effect()` blocks that read it automatically re-run

**Key Pattern:**

```typescript
export class PostgresProjectsState {
  currentSelectedProjectId = $state<string | null>(null);

  setSelectedProjectId(projectId: string) {
    this.currentSelectedProjectId = projectId; // Triggers reactivity
  }
}
```

### 3. Context API (`setContext` / `getContext`) - Global State Sharing

**Purpose:** Share state instances across the component tree without prop drilling.

**How We Use It:**

- States are initialized in the root layout (`+layout.svelte`) using `setContext()`
- Child components access states using `getContext()` during initialization
- States persist across navigation because they're initialized at the root level

**Key Pattern:**

```typescript
// Root layout (+layout.svelte)
setPostgresProjectsState(); // Creates and stores in context

// State class (.svelte.ts file)
export function setPostgresProjectsState() {
  if (hasContext(POSTGRES_PROJECTS_STATE_KEY)) {
    return getPostgresProjectsState(); // Idempotent - returns existing
  }
  const state = new PostgresProjectsState();
  setContext(POSTGRES_PROJECTS_STATE_KEY, state);
  return state;
}

// Dependent state class
constructor() {
  // ✅ CORRECT: getContext during initialization
  try {
    this.projectsState = getPostgresProjectsState();
  } catch (error) {
    console.warn("ProjectsState not available during initialization");
  }
}
```

**Critical Rule:** `getContext()` can **only** be called during component initialization (in constructor or `$effect` that runs synchronously). We store the reference for use in async methods.

### 4. Context Reference Storage Pattern

**Purpose:** Avoid calling `getContext()` in async functions or event handlers.

**How We Use It:**

- Store context references during initialization
- Use stored references in async methods

**Key Pattern:**

```typescript
export class PostgresWellsState {
  // Store reference during initialization
  private projectsState: ReturnType<typeof getPostgresProjectsState> | null =
    null;

  constructor() {
    // ✅ CORRECT: getContext during initialization
    this.projectsState = getPostgresProjectsState();
  }

  async loadWells(): Promise<void> {
    // ✅ CORRECT: Use stored reference, not getContext()
    const projectId = this.projectsState?.currentSelectedProjectId ?? null;
    // ... use projectId
  }
}
```

## Advanced Features We're NOT Using (And Why)

### `$effect.tracking()` - Not Needed

**What It Does:** Checks if code is running inside a tracking context (inside an `$effect()` or template).

**When You'd Use It:** When implementing a store-like pattern where you only want to subscribe to external sources if the value is accessed in a tracking context.

**Why We Don't Need It:**

- We're **always** accessing `currentSelectedProjectId` inside `$effect()` blocks
- We're **always** in a tracking context when reading the reactive property
- No need to check if we're in a tracking context - we know we are

**Example Where You WOULD Use It:**

```typescript
// Implementing a readable store pattern
export const readable = (initial_value, start) => {
  let value = $state(initial_value);

  return {
    get value() {
      // Only subscribe if accessed in tracking context
      if ($effect.tracking()) {
        $effect(() => {
          // Setup subscription only when tracked
          const unsubscribe = start();
          return unsubscribe;
        });
      }
      return value;
    },
  };
};
```

**Our Pattern (Simpler):**

```typescript
// We're always in $effect(), so no check needed
$effect(() => {
  const projectId = this.projectsState.currentSelectedProjectId; // Always tracked
  if (projectId) {
    void this.loadWells();
  }
});
```

### `createSubscriber()` - Not Needed

**What It Does:** Creates a subscription function that manually triggers effect re-runs when external data changes (like media queries, WebSocket messages, etc.).

**When You'd Use It:** When exposing values from external sources that change independently and you want to manually control when effects re-run.

**Why We Don't Need It:**

- We're watching **internal reactive state** (`currentSelectedProjectId`), not external sources
- Svelte's reactivity automatically handles re-runs when `$state` changes
- No manual triggering needed - Svelte does it automatically

**Example Where You WOULD Use It:**

```typescript
// Media query example (external source)
import { createSubscriber } from "svelte/reactivity";
import { on } from "svelte/events";

export class MediaQuery {
  #query = window.matchMedia("(width > 600px)");
  #subscribe = createSubscriber((update) => {
    // Setup external listener
    const off = on(this.#query, "change", update);
    return () => off(); // Cleanup
  });

  get current() {
    this.#subscribe(); // Manually trigger re-run when query changes
    return this.#query.matches;
  }
}
```

**Our Pattern (Simpler):**

```typescript
// We're watching internal $state, not external sources
export class PostgresProjectsState {
  currentSelectedProjectId = $state<string | null>(null); // Internal reactive state

  setSelectedProjectId(projectId: string) {
    this.currentSelectedProjectId = projectId; // Svelte automatically triggers effects
  }
}
```

### Summary: Why Our Pattern Is Simpler

| Feature              | Use Case                               | Our Situation                                                   |
| -------------------- | -------------------------------------- | --------------------------------------------------------------- |
| `$effect()`          | Watch reactive state                   | ✅ **We use this** - watching `currentSelectedProjectId`        |
| `$effect.tracking()` | Check if in tracking context           | ❌ **Not needed** - we're always in `$effect()`                 |
| `createSubscriber()` | Manual reactivity for external sources | ❌ **Not needed** - we're watching internal `$state`            |
| `untrack()`          | Prevent reactive loops                 | ✅ **We use this** - preventing infinite loops in complex state |

**Key Insight:** We're using the **standard reactive pattern** (`$effect()` watching `$state`), enhanced with `untrack()` to prevent loops in complex scenarios. This is simpler and more appropriate for our use case than the advanced features designed for edge cases.

## The Reactive Flow: Project Selection → Data Reload

### Step 1: User Clicks Project Item

**File:** `src/lib/components/pages/projects/content-projects/content-projects-item.svelte`

```typescript
function handleProjectSelect(event: MouseEvent | KeyboardEvent) {
  // Get global projects state
  const projectsState = getPostgresProjectsState();

  // Update selected project ID
  projectsState.setSelectedProjectId(project.id);

  // Redirect to /home
  goto("/home");
}
```

**What Happens:**

- `setSelectedProjectId()` updates `currentSelectedProjectId` from `$state`
- This change is **reactive** - Svelte detects it immediately

### Step 2: Project State Updates

**File:** `src/lib/state/postgres/postgres-projects-state.svelte.ts`

```typescript
setSelectedProjectId(projectId: string) {
  // Prevent unnecessary updates
  if (this.currentSelectedProjectId === projectId) {
    return; // Skip if already selected
  }

  // Update reactive state - triggers all dependent effects
  this.currentSelectedProjectId = projectId;
}
```

**What Happens:**

- `currentSelectedProjectId` changes from one UUID to another
- Svelte's reactivity system detects the change
- All `$effect()` blocks that read `currentSelectedProjectId` are scheduled to re-run

### Step 3: Dependent States React Automatically

**File:** `src/lib/state/postgres/postgres-wells-state.svelte.ts`

```typescript
private initializeWellsLoading() {
  $effect(() => {
    // Get projectsState reference (if not already stored)
    if (!this.projectsState) {
      this.projectsState = getPostgresProjectsState();
    }

    // CRITICAL: Read reactive property directly so $effect tracks it
    const projectId = this.projectsState.currentSelectedProjectId;

    // Prevent unnecessary reloads
    if (projectId === this.lastLoadedProjectId) {
      return; // Skip if project hasn't changed
    }

    if (projectId) {
      // Clear old data immediately
      this.wells = [];
      this.lastLoadedProjectId = projectId;

      // Load new data
      void this.loadWells();
    } else {
      // Clear data if no project selected
      this.wells = [];
    }
  });
}
```

**What Happens:**

1. `$effect()` runs during component initialization
2. Reading `currentSelectedProjectId` makes Svelte track it
3. When `currentSelectedProjectId` changes, Svelte automatically re-runs this effect
4. Effect checks if project actually changed (prevents unnecessary reloads)
5. Clears old data and loads new data for the selected project

### Step 4: Same Pattern for All Dependent States

**All dependent states follow the same pattern:**

- **`PostgresWellsState`** - Watches project changes, reloads wells
- **`PostgresCurvesState`** - Watches project changes, reloads curves
- **`PostgresCurveMetadataState`** - Watches project changes, reloads curve metadata and selections
- **`ContentPipelineState`** - Watches project changes, reloads pipelines and nodes with Map-based optimizations
- **`PostgresMarkersState`** - Loads on-demand (not reactive to project changes)
- **`PostgresTrajectoriesState`** - Loads on-demand (not reactive to project changes)

### Step 5: Advanced Pattern - ContentPipelineState with Realtime Updates

**File:** `src/lib/components/pages/home/content-main/content-dag-editor/content-pipelines/content-pipeline.svelte.ts`

**This state demonstrates advanced patterns:**

1. **Map-based optimizations** for O(1) updates
2. **`untrack()` usage** to prevent reactive loops
3. **Loading flags** to prevent concurrent loads
4. **Realtime subscriptions** that update state efficiently
5. **Edit state management** to prevent flickering during user edits

```typescript
export class ContentPipelineState {
  allPipelinesWithNodes = $state<PipelineWithNodes[]>([]);
  private pipelineLookup = $state<Map<string, PipelineWithNodes>>(new Map());
  private nodeToPipelineMap = $state<Map<string, string>>(new Map());
  private isLoadingPipelines = $state(false);
  private lastLoadedProjectId: string | null = null;
  private lastRealtimeProjectId: string | null = null;

  constructor() {
    this.projectsState = getPostgresProjectsState();
    this.initializePipelinesLoading();
    this.initializeRealtimeSubscriptions();
  }

  private initializePipelinesLoading() {
    $effect(() => {
      const projectId = this.projectsState.currentSelectedProjectId;

      // Prevent reloading if project hasn't changed
      if (projectId === this.lastLoadedProjectId) {
        return;
      }

      if (projectId) {
        // Use untrack to prevent triggering effects that watch allPipelinesWithNodes
        untrack(() => {
          this.allPipelinesWithNodes = [];
          this.pipelineLookup = new Map();
          this.nodeToPipelineMap = new Map();
        });
        this.lastLoadedProjectId = projectId;
        void this.loadAllPipelinesWithNodes();
      }
    });
  }

  private initializeRealtimeSubscriptions() {
    $effect(() => {
      const projectId = this.projectsState?.currentSelectedProjectId ?? null;

      // Prevent re-subscribing if project hasn't changed
      if (projectId === this.lastRealtimeProjectId) {
        return;
      }

      this.lastRealtimeProjectId = projectId;

      // Subscribe to pipelines table changes
      const pipelinesChannel = supabase
        .channel("pipelines-changes-state")
        .on(
          "postgres_changes",
          { event: "*", schema: "public", table: "pipelines" },
          (payload: any) => {
            // Handle name-only updates with Map lookup (O(1))
            if (payload.eventType === "UPDATE" && onlyNameChanged(payload)) {
              const pipeline = this.pipelineLookup.get(payload.new.id);
              if (pipeline) {
                pipeline.name = payload.new.name;
                // Use untrack to prevent reactive loop
                untrack(() => {
                  this.allPipelinesWithNodes = Array.from(
                    this.pipelineLookup.values(),
                  );
                });
                return;
              }
            }

            // For structural changes, do full reload (but check loading flag)
            if (!this.isLoadingPipelines) {
              void this.loadAllPipelinesWithNodes();
            }
          },
        )
        .subscribe();

      return () => {
        supabase.removeChannel(pipelinesChannel);
      };
    });
  }
}
```

**Key Features:**

- **Optimized Updates**: Name-only changes use Map lookups instead of full reloads
- **Loop Prevention**: `untrack()` and loading flags prevent infinite loops
- **Efficient Reactivity**: Only updates what's necessary
- **Realtime Integration**: Handles database updates reactively

## Key Implementation Details

### 1. Idempotent Context Initialization

**Pattern:** States are initialized in root layout and persist across navigation.

```typescript
// Root layout (+layout.svelte)
setPostgresProjectsState(); // Creates once
setPostgresWellsState(); // Creates once
// ... other states

// State functions are idempotent
export function setPostgresWellsState() {
  if (hasContext(POSTGRES_WELLS_STATE_KEY)) {
    return getPostgresWellsState(); // Returns existing if already created
  }
  const state = new PostgresWellsState();
  setContext(POSTGRES_WELLS_STATE_KEY, state);
  return state;
}
```

**Benefits:**

- States persist across navigation
- No cleanup needed on route changes
- Single source of truth for each state

### 2. Preventing Unnecessary Reloads

**Pattern:** Track last loaded project ID to prevent duplicate loads.

```typescript
export class PostgresWellsState {
  private lastLoadedProjectId: string | null = null;

  $effect(() => {
    const projectId = this.projectsState.currentSelectedProjectId;

    // Skip if project hasn't changed
    if (projectId === this.lastLoadedProjectId) {
      return;
    }

    // Update tracking and reload
    this.lastLoadedProjectId = projectId;
    void this.loadWells();
  });
}
```

**Benefits:**

- Prevents duplicate API calls
- Improves performance
- Reduces unnecessary UI updates

### 3. Immediate Data Clearing

**Pattern:** Clear old data immediately when project changes to avoid showing stale data.

```typescript
if (projectId) {
  // Clear immediately - prevents showing wrong project's data
  this.wells = [];
  this.lastLoadedProjectId = projectId;
  void this.loadWells(); // Load new data asynchronously
}
```

**Benefits:**

- No stale data shown during transition
- Better UX - users see empty state while loading
- Prevents confusion about which project's data is displayed

### 4. Stored Context References for Async Methods

**Pattern:** Store context references during initialization, use in async methods.

```typescript
export class PostgresWellsState {
  private projectsState: ReturnType<typeof getPostgresProjectsState> | null =
    null;

  constructor() {
    // ✅ Store reference during initialization
    this.projectsState = getPostgresProjectsState();
  }

  async loadWells(): Promise<void> {
    // ✅ Use stored reference (not getContext())
    const projectId = this.projectsState?.currentSelectedProjectId ?? null;
    // ... use projectId
  }
}
```

**Why This Matters:**

- `getContext()` can only be called during component initialization
- Async methods run outside initialization context
- Stored references work everywhere

## Complete Flow Diagram

```
User clicks project item
    ↓
content-projects-item.svelte::handleProjectSelect()
    ↓
projectsState.setSelectedProjectId(projectId)
    ↓
PostgresProjectsState.currentSelectedProjectId = projectId (reactive update)
    ↓
┌─────────────────────────────────────────────────────────┐
│ Svelte's Reactivity System Detects Change               │
│ All $effect() blocks that read currentSelectedProjectId  │
│ are scheduled to re-run                                 │
└─────────────────────────────────────────────────────────┘
    ↓
    ├─→ PostgresWellsState::$effect() re-runs
    │       ↓
    │   Clear wells = []
    │   Update lastLoadedProjectId
    │   Call loadWells()
    │       ↓
    │   Fetch wells from database
    │   Filter by project_id
    │   Update this.wells = [...]
    │
    ├─→ PostgresCurvesState::$effect() re-runs
    │       ↓
    │   Clear curves = []
    │   Update lastLoadedProjectId
    │   Call loadCurves()
    │       ↓
    │   Fetch curves from database
    │   Filter by project's wells
    │   Update this.curves = [...]
    │
    ├─→ PostgresCurveMetadataState::$effect() re-runs
    │       ↓
    │   Clear selections = []
    │   Update lastLoadedSelectionsProjectId
    │   Call loadSelections()
    │       ↓
    │   Fetch selections from database
    │   Filter by project_id
    │   Update this.selections = [...]
    │
    └─→ ContentPipelineState::$effect() re-runs
            ↓
        Clear allPipelinesWithNodes = [] (using untrack)
        Update lastLoadedProjectId
        Call loadAllPipelinesWithNodes()
            ↓
        Fetch pipelines from database
        For each pipeline, fetch nodes
        Update allPipelinesWithNodes = [...]
        Update pipelineLookup Map (O(1) lookups)
        Update nodeToPipelineMap Map
```

## Why This Pattern Works

### 1. **Declarative Reactivity**

- We declare **what** should happen (reload when project changes)
- Svelte handles **when** it happens (automatically)
- No manual event listeners or coordination needed

### 2. **Single Source of Truth**

- `PostgresProjectsState.currentSelectedProjectId` is the single source
- All dependent states read from this one value
- No synchronization issues or race conditions

### 3. **Automatic Cleanup**

- `$effect()` cleanup runs automatically when component unmounts
- No manual cleanup needed for effects
- States persist across navigation (initialized in root layout)

### 4. **Performance Optimized**

- Prevents unnecessary reloads with `lastLoadedProjectId` tracking
- Clears data immediately to prevent stale data display
- Uses stored context references to avoid repeated `getContext()` calls
- Map-based lookups for O(1) updates instead of O(n) array operations
- Loading flags prevent concurrent operations and infinite loops

### 5. **Loop Prevention**

- Uses `untrack()` to prevent reactive loops when effects read and write the same state
- Loading flags prevent concurrent loads that could trigger loops
- Edit state management prevents realtime updates from interfering with user edits

## Comparison to Alternative Approaches

### ❌ **Alternative 1: Manual Event Listeners**

```typescript
// BAD: Manual coordination
projectsState.on("projectChanged", (projectId) => {
  wellsState.loadWells(projectId);
  curvesState.loadCurves(projectId);
});
```

**Problems:**

- Requires manual subscription/unsubscription
- Easy to forget cleanup
- More boilerplate code

### ❌ **Alternative 2: Polling**

```typescript
// BAD: Polling for changes
setInterval(() => {
  if (projectId !== lastProjectId) {
    loadWells();
  }
}, 1000);
```

**Problems:**

- Wastes resources
- Delayed updates
- Not reactive

### ✅ **Our Approach: `$effect()` Reactivity**

```typescript
// GOOD: Declarative reactivity
$effect(() => {
  const projectId = this.projectsState.currentSelectedProjectId;
  if (projectId) {
    void this.loadWells();
  }
});
```

**Benefits:**

- Automatic tracking
- Immediate updates
- No manual coordination
- Svelte handles everything

## References

- **Svelte 5 `$effect()` Documentation:** https://svelte.dev/docs/svelte/$effect
- **Svelte 5 `$state()` Documentation:** https://svelte.dev/docs/svelte/$state
- **Svelte 5 `untrack()` Documentation:** https://svelte.dev/docs/svelte/untrack
- **Svelte Context API:** https://svelte.dev/docs/svelte/setContext
- **Matt Simon's Blog on `$effect.tracking`:** See `docs/supabase/advanced/createSubscriber/blog/matt-simon.md`
- **Text Edit Rename Flow:** See `markdown-guides/svelte5/text-edit-rename.md` for details on edit state management

## Summary

MudRock uses Svelte 5's advanced reactivity features (`$effect()`, `$state()`, `untrack()`, Context API) to create a **declarative, reactive state management system** where:

1. **User actions** (clicking a project) update reactive state
2. **Dependent states** automatically react via `$effect()` tracking
3. **Data reloads** happen automatically without manual coordination
4. **Performance** is optimized with:
   - Change detection and immediate clearing
   - Map-based lookups for O(1) updates
   - Loading flags to prevent concurrent operations
5. **Loop prevention** using `untrack()` when effects read and write the same state
6. **Realtime updates** integrated seamlessly with reactive state
7. **No cleanup** needed because states persist and effects handle their own lifecycle

This pattern follows Svelte 5 best practices and leverages the framework's reactivity system to its full potential, while avoiding common pitfalls like infinite loops and unnecessary re-renders.
