# Global Wells Fetch Flow

## Overview

This document explains how wells data is fetched and managed globally using the state class pattern in Svelte 5, similar to the UDF pattern.

## Architecture Flow

### 1. Global State Management (`postgres-wells-state.svelte.ts`)

The `PostgresWellsState` class manages the global state for all wells:

- **State Properties**:
  - `wells`: Array of `Well[]` - All wells filtered by current project
  - `selectedWells`: Array of `string[]` - Names of currently selected wells
  - `selectedWellId`: `string` - ID of currently selected well (for single-select operations)
  - `isLoading`: Boolean flag for loading state

- **Initialization**:
  - Called via `setPostgresWellsState()` in the parent layout (`+layout.svelte`)
  - Constructor sets up two initialization flows:
    1. **Realtime Service**: Initializes realtime updates for wells (deferred via `requestAnimationFrame`)
    2. **Wells Loading**: Sets up reactive loading via `initializeWellsLoading()`

### 2. Reactive Loading Pattern (`initializeWellsLoading()`)

The state class uses Svelte 5's `$effect()` to reactively load wells when project changes:

- **Method**: `initializeWellsLoading()`
- **Reactive Dependency**: Watches `projectsState.currentSelectedProjectId`
- **Behavior**:
  - When project changes → automatically calls `loadWells()`
  - When no project selected → clears wells array
  - Runs automatically whenever `currentSelectedProjectId` changes

### 3. Data Fetching (`loadWells()`)

The `loadWells()` method handles fetching and filtering wells:

- **Backend Call**: Uses `getTableData("wells")` Tauri command to fetch from PostgreSQL via PostgREST
- **Filtering**: Filters wells by `project_id` matching `currentSelectedProjectId`
- **State Update**: Calls `setWells()` to update state (with merge logic for realtime updates)
- **Error Handling**: Catches errors, logs them, and clears wells on failure

### 4. State Update Pattern (`setWells()`)

The `setWells()` method handles merging new data with existing state:

- **Merge Logic**: Merges new wells from DB with existing wells (preserves realtime updates)
- **Deduplication**: Uses `Map` to avoid duplicate wells by ID
- **Precedence**: DB data takes precedence for existing IDs, but preserves realtime-added wells

### 5. Realtime Updates (`realtimeWellsService`)

The realtime service provides live updates:

- **Initialization**: Deferred in constructor to avoid initialization order issues
- **Connection**: Connects to PostgREST realtime subscription
- **Updates**: Automatically updates `wells` array when changes occur in database
- **Status**: Provides `getRealtimeStatus()` method to check connection status

## Data Flow Summary

```
+layout.svelte
    ↓
setPostgresWellsState() → Creates PostgresWellsState instance
    ↓
Constructor:
    ├─→ initializeRealtime() (deferred)
    └─→ initializeWellsLoading()
        ↓
        $effect() watches projectsState.currentSelectedProjectId
            ↓
            When project changes → loadWells()
                ↓
                getTableData("wells") → PostgreSQL via PostgREST
                    ↓
                    Filter by project_id
                        ↓
                        setWells() → Update state (with merge)
                            ↓
                            UI components reactively update
```

## Key Points

1. **Self-Contained State**: The state class is responsible for its own data loading, similar to UDF pattern
2. **Reactive Loading**: Uses `$effect()` to automatically load wells when project changes
3. **Project-Dependent**: Wells are filtered by `project_id`, so loading is triggered by project selection
4. **Realtime Integration**: Merges realtime updates with DB fetches to keep data current
5. **Global State**: Uses Svelte 5's `$state` and Context API for reactive updates across components
6. **No Component Logic**: Components like `home-page.svelte` and `content-data.svelte` only consume the state, they don't manage loading

## Comparison with UDF Pattern

| Aspect              | UDFs                          | Wells                                       |
| ------------------- | ----------------------------- | ------------------------------------------- |
| **Loading Trigger** | Once on initialization        | When project changes                        |
| **Dependency**      | None (global)                 | `projectsState.currentSelectedProjectId`    |
| **Filtering**       | None                          | By `project_id`                             |
| **Realtime**        | No                            | Yes (via `realtimeWellsService`)            |
| **Pattern**         | `initialize()` in constructor | `initializeWellsLoading()` with `$effect()` |
