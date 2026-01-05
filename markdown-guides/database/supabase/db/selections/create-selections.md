# Selection Creation and Loading with Supabase PostgreSQL

This guide explains the end-to-end process of creating and loading selections in our database-driven architecture using Supabase PostgreSQL, global state management, and realtime updates.

## Architecture Overview

### Current Implementation: JavaScript-Based Approach

**Frontend-First**: We use the JavaScript Supabase client directly from the frontend for database operations. This provides:

- Direct access to Supabase's PostgREST API
- Built-in realtime subscriptions
- Row Level Security (RLS) enforcement
- Type-safe error handling with `neverthrow`

### Database Schema

**Two-Table Design**:

- `selections` table: Metadata about selections (name, description, data_type, project_id, owner_id)
- `selection_items` table: Polymorphic references to actual items (wells, curves, nodes, pipelines, projects)

**Key Features**:

- Polymorphic associations: Single table handles multiple data types
- Project-scoped: Selections belong to projects
- Owner-based: Each selection has an owner
- Realtime-enabled: Changes propagate via PostgreSQL logical replication

## End-to-End Flow

### Flow Diagram

```
User Input (UI)
    ↓
selections-new-selection.svelte (Component)
    ↓
saveSelection() (Service - neverthrow)
    ↓
Supabase Client → PostgreSQL
    ├─ INSERT selections
    └─ INSERT selection_items
    ↓
PostgreSQL Logical Replication
    ↓
Supabase Realtime Service
    ↓
RealtimeSelectionsService (WebSocket)
    ↓
PostgresWellsState / PostgresCurveMetadataState (Global State)
    ↓
UI Updates Automatically
```

## Step-by-Step Implementation

### Step 1: User Input in Frontend Component

**Component**: `src/lib/components/pages/home/selections-page/selections-new-selection/selections-new-selection.svelte`

**Responsibilities**:

- Display form for creating new selections
- Show available items (wells or curves) from global state
- Handle user input and validation
- Call service to save selection

**Key Code**:

```typescript
// Get global state instances (during component initialization)
const wellsState = getPostgresWellsState();
const curvesState = getPostgresCurveMetadataState();
const projectsState = getPostgresProjectsState();

// Get available items based on data type
const availableItems = $derived(
  selectedDataType === "wells"
    ? wellsState.wells.filter(
        (w) => w.project_id === projectsState.currentSelectedProjectId,
      )
    : curvesState.curveMetadata,
);

// Convert to multi-select options
const itemOptions: MultiSelectOption[] = $derived(
  selectedDataType === "wells"
    ? availableItems.map((well) => ({
        label: well.name,
        value: well.id.toString(),
      }))
    : availableItems.map((curve) => ({
        label: curve.display_name,
        value: curve.id.toString(),
      })),
);

// Save handler
async function handleSave() {
  if (!isValid || !projectsState.currentSelectedProjectId) return;

  isSaving = true;
  try {
    const result = await saveSelection({
      name: selectionName.trim(),
      description: selectionDescription.trim() || null,
      data_type: selectedDataType as "wells" | "curves",
      project_id: projectsState.currentSelectedProjectId,
      item_ids: selectedItemIds,
    });

    if (result.isOk()) {
      // Reset form and reload selections
      selectionName = "";
      selectionDescription = "";
      selectedItemIds = [];

      // Reload selections - pass projectId explicitly to avoid getContext issues
      const projectId = projectsState.currentSelectedProjectId;
      if (selectedDataType === "wells") {
        await wellsState.loadSelections(projectId);
      } else {
        await curvesState.loadSelections(projectId);
      }
    } else {
      console.error("❌ Failed to save selection:", result.error);
    }
  } catch (error) {
    console.error("❌ Unexpected error saving selection:", error);
  } finally {
    isSaving = false;
  }
}
```

**Key Points**:

- Uses `$derived` runes for reactive item options
- Accesses global state during component initialization (when `getContext()` is allowed)
- Passes `projectId` explicitly to async methods to avoid `getContext()` in async handlers

### Step 2: Service Layer - Save Selection

**Service**: `src/lib/services/selections-service.ts`

**Responsibilities**:

- Handle authentication (with fallback for CORS issues)
- Create selection record in database
- Create selection_items records
- Provide rollback on failure
- Use `neverthrow` for type-safe error handling

**Key Code**:

```typescript
export function saveSelection(
  params: CreateSelectionParams,
): ResultAsync<Selection, Error> {
  const supabase = getSupabaseClient();

  // Temporary hardcoded owner_id (until CORS/auth is configured)
  const ownerId = "29ade0b8-b81d-4475-bc4d-87f76553e5bb";

  // Try to get user from auth, but silently fall back if CORS/auth fails
  return ResultAsync.fromPromise(
    supabase.auth.getUser(),
    () => null, // Silently catch errors (CORS/auth not configured)
  )
    .map((result) => result?.data?.user?.id || ownerId)
    .andThen((finalOwnerId) =>
      ResultAsync.fromPromise(
        (async () => {
          // 1. Create the selection record
          const { data: selection, error: selectionError } = await supabase
            .from("selections")
            .insert({
              name: params.name,
              description: params.description,
              data_type: params.data_type,
              owner_id: finalOwnerId,
              project_id: params.project_id,
              is_public: false,
              is_favorite: false,
            })
            .select()
            .single();

          if (selectionError) {
            throw new Error(
              `Failed to create selection: ${selectionError.message}`,
            );
          }

          // 2. Create selection items based on data type
          const selectionItems = params.item_ids.map((itemId, index) => {
            const baseItem = {
              selection_id: selection.id,
              item_order: index,
            };

            if (params.data_type === "wells") {
              return {
                ...baseItem,
                well_id: parseInt(itemId),
              };
            } else if (params.data_type === "curves") {
              return {
                ...baseItem,
                curve_id: itemId,
              };
            }

            throw new Error(`Unsupported data type: ${params.data_type}`);
          });

          // 3. Insert all selection items
          const { error: itemsError } = await supabase
            .from("selection_items")
            .insert(selectionItems);

          if (itemsError) {
            // Rollback: delete the selection if items insertion fails
            await supabase.from("selections").delete().eq("id", selection.id);
            throw new Error(
              `Failed to create selection items: ${itemsError.message}`,
            );
          }

          // 4. Return selection with item count
          return {
            ...selection,
            item_count: params.item_ids.length,
          };
        })(),
        (error) => (error instanceof Error ? error : new Error(String(error))),
      ),
    );
}
```

**Key Points**:

- Uses `neverthrow`'s `ResultAsync` for type-safe error handling
- Handles auth gracefully with fallback to hardcoded `owner_id`
- Implements transaction-like behavior with rollback on failure
- Creates polymorphic `selection_items` based on `data_type`

### Step 3: Database Operations

**PostgreSQL Tables**:

1. **`selections` table**:

   ```sql
   INSERT INTO selections (name, description, data_type, owner_id, project_id, is_public, is_favorite)
   VALUES ('My Selection', 'Description', 'wells', 'owner-uuid', 'project-uuid', false, false)
   RETURNING *;
   ```

2. **`selection_items` table**:
   ```sql
   INSERT INTO selection_items (selection_id, item_order, well_id)
   VALUES
     ('selection-uuid', 0, 1),
     ('selection-uuid', 1, 2),
     ('selection-uuid', 2, 3);
   ```

**PostgreSQL Features Used**:

- Foreign key constraints with `ON DELETE CASCADE`
- Check constraints for polymorphic associations
- Partial unique indexes to prevent duplicate items
- Row Level Security (RLS) for access control

### Step 4: Realtime Updates

**Service**: `src/lib/services/realtime-selections-service.ts`

**How It Works**:

1. **Initialization**: Service is initialized with state objects during component initialization
2. **Connection**: Connects to Supabase Realtime via WebSocket
3. **Subscription**: Subscribes to `postgres_changes` events on `selections` table
4. **Event Handling**: Updates global state when changes occur

**Key Code**:

```typescript
// Subscribe to selections table changes
this.channel = this.supabase
  .channel("selections-changes")
  .on(
    "postgres_changes",
    {
      event: "*", // INSERT, UPDATE, DELETE
      schema: "public",
      table: "selections",
    },
    async (payload: any) => {
      await this.handleSelectionsChange(payload);
    },
  )
  .subscribe(async (status: any) => {
    if (status === "SUBSCRIBED") {
      this.isConnected = true;
    }
  });

// Handle INSERT events
private async handleSelectionInsert(newSelection: any) {
  const selection: Selection = {
    id: newSelection.id,
    name: newSelection.name,
    // ... other fields
  };

  // Update the appropriate state based on data_type
  if (newSelection.data_type === "wells" && this.wellsState) {
    const exists = this.wellsState.selections.some(
      (s: Selection) => s.id === selection.id,
    );
    if (!exists) {
      this.wellsState.selections = [...this.wellsState.selections, selection];
    }
  } else if (newSelection.data_type === "curves" && this.curvesState) {
    // Similar for curves
  }

  // Invalidate cache
  await invalidateTableCache("selections");
}
```

**Key Points**:

- Uses PostgreSQL logical replication (built into Supabase)
- Automatically updates UI when database changes occur
- Handles INSERT, UPDATE, and DELETE events
- Updates appropriate state based on `data_type`

### Step 5: Global State Management

**State Classes**:

- `PostgresWellsState` (`src/lib/state/postgres/postgres-wells-state.svelte.ts`)
- `PostgresCurveMetadataState` (`src/lib/state/postgres/postgres-curve-metadata-state.svelte.ts`)

**Responsibilities**:

- Store selections in reactive state (`$state`)
- Watch for project changes and auto-load selections
- Provide methods to load selections manually
- Integrate with realtime service for automatic updates

**Key Code**:

```typescript
export class PostgresWellsState {
  // Reactive state for selections
  selections = $state<Selection[]>([]);

  // Store projectsState reference (set during initialization)
  private projectsState: ReturnType<typeof getPostgresProjectsState> | null =
    null;

  constructor() {
    // Get projectsState reference during initialization (when getContext is allowed)
    try {
      this.projectsState = getPostgresProjectsState();
    } catch (error) {
      console.warn("ProjectsState not available during initialization");
    }

    // Initialize selections loading - watch for project changes
    this.initializeSelectionsLoading();
  }

  // Initialize selections loading by watching project changes
  private initializeSelectionsLoading() {
    $effect(() => {
      // Get or update projectsState reference (runs during component initialization)
      if (!this.projectsState) {
        try {
          this.projectsState = getPostgresProjectsState();
        } catch (error) {
          return;
        }
      }

      const projectId = this.projectsState.currentSelectedProjectId;

      if (projectId) {
        console.log(`Project changed to ${projectId}, loading selections...`);
        void this.loadSelections(projectId);
      } else {
        this.selections = [];
      }
    });
  }

  // Load selections for wells
  async loadSelections(projectId?: string | null): Promise<void> {
    // Use provided projectId or get from stored projectsState reference
    const finalProjectId =
      projectId ?? this.projectsState?.currentSelectedProjectId ?? null;

    if (!finalProjectId) {
      this.selections = [];
      return;
    }

    console.log(`Loading selections for project: ${finalProjectId}...`);

    const result = await loadSelections("wells", finalProjectId);

    if (result.isOk()) {
      this.selections = result.value;
      console.log(`Loaded ${result.value.length} selections`);
    } else {
      console.error("Failed to load selections:", result.error);
      this.selections = [];
    }
  }
}
```

**Key Points**:

- **CRITICAL**: Stores `projectsState` reference during initialization to avoid `getContext()` in async methods
- Uses `$effect` to watch for project changes and auto-load selections
- Accepts `projectId` as parameter to avoid calling `getContext()` in async handlers
- Uses `neverthrow`'s `ResultAsync` for type-safe error handling

### Step 6: Loading Selections

**Service Method**: `loadSelections()` in `selections-service.ts`

**How It Works**:

1. Query `selections_view` (database view with `item_count`)
2. Filter by `data_type` and optionally `project_id`
3. Return selections ordered by `created_at`

**Key Code**:

```typescript
export function loadSelections(
  dataType: "wells" | "curves",
  projectId: string | null,
): ResultAsync<Selection[], Error> {
  const supabase = getSupabaseClient();

  return ResultAsync.fromPromise(
    (async () => {
      let query = supabase
        .from("selections_view") // Use the view to get item_count
        .select("*")
        .eq("data_type", dataType)
        .order("created_at", { ascending: false });

      if (projectId) {
        query = query.eq("project_id", projectId);
      }

      const { data, error } = await query;

      if (error) {
        throw new Error(`Failed to load selections: ${error.message}`);
      }

      // Transform the data - the view already includes item_count
      return (data || []).map((selection: any) => ({
        ...selection,
        item_count: selection.item_count || 0,
      }));
    })(),
    (error) => (error instanceof Error ? error : new Error(String(error))),
  );
}
```

**Key Points**:

- Uses database view (`selections_view`) for efficient querying with `item_count`
- Filters by `data_type` and `project_id`
- Returns `ResultAsync` for type-safe error handling

## Context API Best Practices

**CRITICAL**: When using `getContext()` with global state classes:

1. **Store Reference During Initialization**:

   ```typescript
   class MyState {
     private projectsState: ReturnType<typeof getPostgresProjectsState> | null =
       null;

     constructor() {
       // ✅ CORRECT: getContext during initialization
       try {
         this.projectsState = getPostgresProjectsState();
       } catch (error) {
         console.warn("ProjectsState not available");
       }
     }
   }
   ```

2. **Use Stored Reference in Async Methods**:

   ```typescript
   async loadSelections(projectId?: string | null): Promise<void> {
     // ✅ CORRECT: Use stored reference, not getContext()
     const finalProjectId =
       projectId ?? this.projectsState?.currentSelectedProjectId ?? null;
   }
   ```

3. **Pass Values as Parameters**:

   ```typescript
   // ✅ CORRECT: Pass projectId as parameter
   await wellsState.loadSelections(projectId);

   // ❌ AVOID: Calling getContext() in async handler
   async function handler() {
     const projectsState = getPostgresProjectsState(); // ERROR!
   }
   ```

## Error Handling

**Using `neverthrow`**:

- All service methods return `ResultAsync<T, Error>` for type-safe error handling
- Components check `result.isOk()` before accessing `result.value`
- Errors are logged but don't crash the application

**Example**:

```typescript
const result = await saveSelection(params);

if (result.isOk()) {
  console.log("✅ Selection saved:", result.value);
  // Use result.value
} else {
  console.error("❌ Failed:", result.error);
  // Handle error
}
```

## Realtime Updates Flow

1. **Database Change**: User creates selection → PostgreSQL INSERT
2. **Logical Replication**: PostgreSQL streams change to Supabase Realtime
3. **WebSocket**: Realtime service sends change to connected clients
4. **RealtimeSelectionsService**: Receives change event
5. **State Update**: Updates `wellsState.selections` or `curvesState.selections`
6. **UI Update**: Svelte reactivity automatically updates UI

## Summary

**Creating Selections**:

1. User fills form in `selections-new-selection.svelte`
2. Component calls `saveSelection()` service
3. Service creates `selections` and `selection_items` records
4. Database change triggers realtime update
5. `RealtimeSelectionsService` updates global state
6. UI updates automatically via Svelte reactivity

**Loading Selections**:

1. Global state watches for project changes via `$effect`
2. Calls `loadSelections()` service when project changes
3. Service queries `selections_view` filtered by `data_type` and `project_id`
4. Results stored in `selections` state array
5. UI displays selections reactively

**Key Architecture Decisions**:

- ✅ Frontend-first: Direct Supabase client usage
- ✅ Type-safe errors: `neverthrow` for error handling
- ✅ Realtime updates: Automatic UI synchronization
- ✅ Context safety: Store references during initialization
- ✅ Project-scoped: Selections belong to projects
- ✅ Polymorphic design: Single schema handles multiple data types
