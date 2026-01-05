# Global Curves Fetch Flow

## Overview

This document explains how curves data is fetched and managed globally using the state class pattern in Svelte 5, similar to the wells and UDF patterns.

## Architecture Flow

### 1. Global State Management (`postgres-curve-state.svelte.ts`)

The `PostgresCurvesState` class manages the global state for all curves:

- **State Properties**:
  - `curves`: Array of `Curve[]` - All curves filtered by current project's wells
  - `isLoading`: Boolean flag for loading state

- **Initialization**:
  - Called via `setPostgresCurvesState()` in the parent layout (`+layout.svelte`)
  - Constructor sets up two initialization flows:
    1. **Realtime Service**: Initializes realtime updates for curves (deferred via `requestAnimationFrame`)
    2. **Curves Loading**: Sets up reactive loading via `initializeCurvesLoading()`

### 2. Reactive Loading Mechanism

The `initializeCurvesLoading()` method sets up an `$effect()` that watches `projectsState.currentSelectedProjectId`:

```typescript
$effect(() => {
  const projectsState = getPostgresProjectsState();
  const projectId = projectsState.currentSelectedProjectId;

  if (projectId) {
    console.log(`[PostgresCurvesState] Project changed to ${projectId}, loading curves...`);
    void this.loadCurves();
  } else {
    console.warn("[PostgresCurvesState] No project selected, clearing curves...");
    this.curves = [];
  }
});
```

### 3. Data Loading Flow

When a project is selected:

1. **Project Change Detected**: `$effect()` detects `currentSelectedProjectId` change
2. **Load Curves**: Calls `loadCurves()` method
3. **Fetch Data**: Uses `getTableData("curves")` to fetch all curves from PostgreSQL
4. **Filter by Project Wells**: Filters curves to only include those belonging to wells in the selected project
5. **Update State**: Calls `setCurves()` to update the global state

### 4. Realtime Updates

Similar to wells, curves support realtime updates:

- **Service**: `RealtimeCurvesService` handles Supabase Realtime subscriptions
- **Events**: Listens for INSERT, UPDATE, DELETE events on the `curves` table
- **State Updates**: Automatically updates `curves` array when changes occur
- **Cache Invalidation**: Invalidates table cache to ensure fresh data

### 5. Data Structure

Each `Curve` object contains:

```typescript
interface Curve {
  id: string;                    // UUID primary key
  well_id: number;              // Reference to wells table
  curve_id: string;             // Unique identifier (e.g., "gr1", "gr2", "dtc1")
  curve_name: string;           // Display name
  curve_metadata_id: string;    // Reference to curve_metadata table
  parquet_file_path: string | null;
  parquet_column_name: string | null;
  created_at: string;
  updated_at: string;
}
```

## Comparison with Other Patterns

### Similarities to Wells Pattern:
- ✅ Reactive loading based on project selection
- ✅ Realtime updates via Supabase
- ✅ State class with `$state` runes
- ✅ Context-based global state management
- ✅ Automatic filtering by project

### Differences:
- **Dependency on Wells**: Curves are filtered by wells in the selected project, so curves loading depends on wells being loaded first
- **Well State Reference**: Stores `wellsState` reference during initialization to avoid `getContext()` errors in `$effect()` blocks

## Data Flow Diagram

```
User selects project
    ↓
PostgresProjectsState.currentSelectedProjectId changes
    ↓
PostgresCurvesState.$effect() triggered
    ↓
loadCurves() called
    ↓
Fetch all curves from PostgreSQL (getTableData("curves"))
    ↓
Get wells for current project (from PostgresWellsState)
    ↓
Filter curves by project's well IDs
    ↓
Update PostgresCurvesState.curves
    ↓
UI components reactively update
```

## Integration Points

### LAS File Upload Flow

When LAS files are uploaded and processed:

1. **LAS Parsing**: Backend parses LAS file and detects curves
2. **Curve Creation**: Backend creates entries in `curves` table for each detected curve
3. **Realtime Update**: Supabase Realtime triggers INSERT event
4. **State Update**: `RealtimeCurvesService` receives event and updates `PostgresCurvesState.curves`
5. **UI Update**: Components using `getPostgresCurvesState()` automatically see new curves

### Curve Metadata Mapping

Each curve in the `curves` table references a `curve_metadata` entry:

- **Curve Metadata**: Defines the curve type (GR, RHOB, RT, etc.) and properties
- **Curve Instance**: Represents a specific curve from a LAS file (e.g., "gr1", "gr2") mapped to metadata
- **Well Association**: Links the curve to a specific well

## Usage Example

```typescript
// In a component
import { getPostgresCurvesState } from '$lib/state/postgres/postgres-curve-state.svelte';

const curvesState = getPostgresCurvesState();

// Access curves for current project
const projectCurves = curvesState.curves;

// Get curves for a specific well
const wellCurves = curvesState.getCurvesForWell(wellId);

// Get curves by metadata type
const grCurves = curvesState.getCurvesByMetadata(grMetadataId);
```

## Notes

- Curves are automatically loaded when a project is selected
- Curves are filtered by wells in the selected project
- Realtime updates ensure curves stay synchronized across multiple clients
- The `wellsState` reference is stored during initialization to avoid `getContext()` errors

