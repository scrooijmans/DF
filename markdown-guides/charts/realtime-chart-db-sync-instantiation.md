# Realtime Chart Database Sync: Creation, Deletion, and Name Management

## Overview

This document explains the complete end-to-end implementation of realtime chart creation, deletion, and name management in the MudRock application. It describes how charts are created with unique names, how deletions are synchronized in real-time, and how the system prevents name clashes while reusing names from deleted charts.

## Architecture Overview

The chart management system consists of several interconnected components:

```
┌─────────────────────────────────────────────────────────────┐
│                    User Interaction                          │
│  Create Chart → charts-menubar-item.svelte                  │
│  Delete Chart → chart-sidebar-item.svelte                   │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Frontend Service Layer                         │
│  createChart() → Insert into charts table                    │
│    ├─ Generate unique chart name (gap-filling logic)        │
│    └─ Insert chart with default config                      │
│  deleteChart() → DELETE from charts table                   │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              PostgreSQL Database                            │
│  INSERT INTO charts (...)                                   │
│  DELETE FROM charts WHERE id = ...                         │
│  ← Publication: supabase_realtime                           │
│  ← RLS policies enabled                                     │
│  ← Constraint: UNIQUE (project_id, name)                   │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Logical Replication (WAL)
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Supabase Realtime Service                      │
│  Detects INSERT on charts table                            │
│  Detects DELETE on charts table                            │
│  ← Extension: postgres_cdc_rls                              │
│  ← Verifies RLS policies                                    │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ WebSocket (ws://)
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Frontend Realtime Services                     │
│  RealtimeChartsService → Updates chartsState.charts         │
│  ← PostgresChartsState (Global State)                       │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              UI Components                                  │
│  charts-sidebar.svelte → Chart list updates                 │
│  chart-editor.svelte → Chart editor updates                 │
└─────────────────────────────────────────────────────────────┘
```

## Key Features

### 1. **Gap-Filling Name Generation**

- Reuses names from deleted charts (e.g., if `map_2` is deleted, next chart is `map_2`, not `map_3`)
- Finds the lowest available number to fill gaps
- Prevents duplicate key violations

### 2. **Hard Delete Pattern**

- Charts are permanently removed from the database (not soft-deleted)
- Names are immediately available for reuse
- Realtime DELETE events propagate instantly

### 3. **Optimistic UI Updates**

- Chart disappears from UI immediately when deleted (optimistic update)
- Realtime DELETE event confirms deletion
- Rollback mechanism if deletion fails

### 4. **Realtime Synchronization**

- INSERT events add charts to state automatically
- DELETE events remove charts from state automatically
- Multi-client synchronization (all users see changes simultaneously)

## Complete Call Stack & Data Flow

### Step 1: User Creates Chart

**Location**: `src/lib/components/pages/home/charts/charts-menubar/charts-menubar-item/charts-menubar-item.svelte`

**Code Flow**:

```svelte
<script lang="ts">
  import { createChart } from '$lib/services/chart-service';
  import { getPostgresProjectsState } from '$lib/state/postgres/postgres-projects-state.svelte';
  import { getPostgresChartsState } from '$lib/state/postgres/postgres-charts-state.svelte';

  async function handleClick() {
    const projectId = projectsState.currentSelectedProjectId;

    if (!projectId) {
      alert('Please select a project first');
      return;
    }

    const result = await createChart({
      chartTypeId,
      projectId,
    });

    result.match(
      (chart) => {
        // Select the newly created chart
        chartsState.setSelectedChartId(chart.id);
      },
      (error) => {
        alert(`Failed to create chart: ${error.message}`);
      },
    );
  }
</script>
```

**What Happens**:

1. Validates that a project is selected
2. Calls `createChart()` with chart type and project ID
3. Selects the newly created chart so it appears in the editor

---

### Step 2: Generate Unique Chart Name

**Location**: `src/lib/services/chart-service.ts` → `createChart()`

**Code Flow**:

```typescript
export async function createChart(params: {
  chartTypeId: string;
  projectId: string;
  displayName?: string;
}): Promise<Result<Chart, Error>> {
  // Fetch chart type to get default config
  const { data: chartType } = await supabase
    .from("chart_types")
    .select("*")
    .eq("chart_type_id", params.chartTypeId)
    .eq("is_active", true)
    .single();

  // Generate base name from display name or chart type display name
  const baseName =
    params.displayName ||
    chartType.display_name.toLowerCase().replace(/\s+/g, "_");

  // Generate unique chart name (gap-filling logic)
  const uniqueName = await generateUniqueChartName(params.projectId, baseName);

  // Create chart with default config
  const chart: Partial<Chart> = {
    project_id: params.projectId,
    name: uniqueName, // e.g., "map", "map_1", "map_2" (fills gaps)
    chart_type: params.chartTypeId,
    chart_config: chartType.default_config || {},
    data_source_config: chartType.default_data_source_config || {},
    is_active: true,
    created_by: user.id,
  };

  // Insert into database
  const { data, error } = await supabase
    .from("charts")
    .insert(chart)
    .select()
    .single();

  return ok(data as Chart);
}
```

**Database Operation**:

```sql
INSERT INTO public.charts (
    id, project_id, name, chart_type,
    chart_config, data_source_config,
    is_active, created_by, created_at, updated_at
) VALUES (
    gen_random_uuid(),
    'project-uuid',
    'map_2',  -- Gap-filled name (if map_2 was deleted)
    'map',
    '{}'::jsonb,
    '{}'::jsonb,
    true,
    'user-uuid',
    NOW(),
    NOW()
);
```

**What Happens**:

1. Fetches chart type metadata
2. Generates base name (e.g., `"map"` from `"Map Chart"`)
3. Calls `generateUniqueChartName()` to find available name
4. Creates chart record with unique name
5. Inserts chart into database

**PostgreSQL WAL**: Write-Ahead Log is updated with INSERT operation

---

### Step 3: Gap-Filling Name Generation

**Location**: `src/lib/utils/charts/chart-name-validation.ts` → `generateUniqueChartName()`

**Code Flow**:

```typescript
export async function generateUniqueChartName(
  projectId: string,
  baseName: string,
): Promise<string> {
  // Get all existing chart names in the project (only active charts)
  const existingNames = await getExistingChartNamesInProject(
    projectId,
    baseName,
  );

  // If base name doesn't exist, use it as-is
  if (!existingNames.includes(baseName)) {
    return baseName; // "map"
  }

  // Extract all suffix numbers from existing names
  const suffixNumbers: number[] = [];
  for (const name of existingNames) {
    if (name === baseName) {
      continue; // Skip base name
    } else if (name.startsWith(`${baseName}_`)) {
      const suffixPart = name.substring(baseName.length + 1);
      const suffixNum = parseInt(suffixPart, 10);
      if (!isNaN(suffixNum) && suffixNum > 0) {
        suffixNumbers.push(suffixNum);
      }
    }
  }

  // Sort suffix numbers to find gaps
  suffixNumbers.sort((a, b) => a - b);

  // Find the first gap (lowest missing number starting from 1)
  // Example: [1, 2, 4, 5] → gap at 3
  for (let i = 1; i <= suffixNumbers.length; i++) {
    if (!suffixNumbers.includes(i)) {
      return `${baseName}_${i}`; // Fill gap: "map_3"
    }
  }

  // No gaps found, use the next number after the highest
  const maxSuffix = suffixNumbers.length > 0 ? Math.max(...suffixNumbers) : 0;
  return `${baseName}_${maxSuffix + 1}`; // "map_3" if max is 2
}
```

**Name Generation Examples**:

| Existing Charts           | Next Chart Name | Reason                        |
| ------------------------- | --------------- | ----------------------------- |
| `map`, `map_1`, `map_3`   | `map_2`         | Fills gap at 2                |
| `map`, `map_1`, `map_2`   | `map_3`         | No gaps, increment            |
| `map_1`, `map_2`, `map_3` | `map`           | Base name available           |
| `map`                     | `map_1`         | Base name taken, start from 1 |

**Database Query**:

```sql
SELECT name FROM charts
WHERE project_id = 'project-uuid'
  AND is_active = true
  AND (name = 'map' OR name LIKE 'map_%');
```

**What Happens**:

1. Fetches all existing chart names matching the base name pattern
2. Extracts suffix numbers from names (e.g., `map_1` → `1`)
3. Finds the first gap in the sequence (lowest missing number)
4. Returns name with gap-filled suffix, or increments if no gaps

**Key Design Decision**: Only checks active charts (`is_active = true`) because deleted charts are hard-deleted and immediately removed from the database.

---

### Step 4: Realtime Service Detects Chart INSERT

**Location**: Supabase Realtime Service (Docker container)

**Process**:

```
PostgreSQL WAL (INSERT on charts table)
    ↓
Logical Replication → Supabase Realtime Service
    ↓
Checks RLS policies (SELECT permission required)
    ↓
Formats event for WebSocket transmission
    ↓
Queues event for WebSocket client
```

**Event Format** (Chart INSERT):

```json
{
  "schema": "public",
  "table": "charts",
  "commit_timestamp": "2025-01-20T10:14:15.444Z",
  "eventType": "INSERT",
  "new": {
    "id": "517eb7c9-21a8-4215-b501-7a2cee8ab428",
    "project_id": "project-uuid",
    "name": "map_2",
    "chart_type": "map",
    "chart_config": {},
    "data_source_config": {},
    "is_active": true,
    "created_by": "user-uuid",
    "created_at": "2025-01-20T10:14:15.441571+00:00",
    ...
  },
  "old": null
}
```

**Configuration**:

- **Publication**: `supabase_realtime` includes `public.charts` table
- **RLS Policies**: SELECT policies for `anon` and `authenticated` roles
- **Extension**: `postgres_cdc_rls` with RLS verification enabled

---

### Step 5: Frontend Receives Chart INSERT Event

**Location**: `src/lib/services/realtime-charts-service.ts` → `handleChartInsert()`

**Code Flow**:

```typescript
private async handleChartInsert(newChart: any) {
  console.log("🔄 [RealtimeChartsService] New chart inserted:", newChart);

  // Filter by current project_id if available
  const currentProjectId = this.projectsState?.currentSelectedProjectId;
  if (currentProjectId && newChart.project_id !== currentProjectId) {
    return; // Skip charts from other projects
  }

  // Convert database record to Chart type
  const chart: Chart = {
    id: newChart.id,
    project_id: newChart.project_id,
    name: newChart.name,
    chart_type: newChart.chart_type,
    chart_config: newChart.chart_config ?? {},
    data_source_config: newChart.data_source_config ?? {},
    is_active: newChart.is_active ?? true,
    // ... other fields
  };

  // Check if chart already exists (avoid duplicates)
  const exists = this.chartsState.charts.some((c: Chart) => c.id === chart.id);
  if (exists) {
    // Update existing chart
    this.chartsState.charts = this.chartsState.charts.map((c: Chart) =>
      c.id === chart.id ? chart : c,
    );
  } else {
    // Add to global state
    this.chartsState.charts = [...this.chartsState.charts, chart];
  }

  // If this newly inserted chart is the currently selected chart, load its state
  if (this.chartsState.lastSelectedChartId === chart.id) {
    this.chartsState.loadChartState(chart);
  }

  // Invalidate cache
  await invalidateTableCache("charts");
}
```

**What Happens**:

1. Receives INSERT event payload
2. Filters by current project (skips charts from other projects)
3. Converts database record to `Chart` type
4. Adds chart to `chartsState.charts` array (or updates if exists)
5. Loads chart state if it's the selected chart
6. Invalidates PostgREST cache

**State Update**:

```typescript
// Before
chartsState.charts = [
  { id: "chart-1", name: "map", ... },
  { id: "chart-2", name: "map_1", ... }
]

// After
chartsState.charts = [
  { id: "chart-1", name: "map", ... },
  { id: "chart-2", name: "map_1", ... },
  { id: "new-chart-id", name: "map_2", ... }  // ← New chart added
]
```

---

### Step 6: User Deletes Chart

**Location**: `src/lib/components/pages/home/charts/charts-sidebar/chart-sidebar-item.svelte`

**Code Flow**:

```svelte
<script lang="ts">
  import { deleteChart } from '$lib/services/chart-service';
  import { getPostgresChartsState } from '$lib/state/postgres/postgres-charts-state.svelte';

  async function handleDelete() {
    const chartIdToDelete = chart.id;
    const wasSelected = chartsState.lastSelectedChartId === chartIdToDelete;

    // Optimistic update: Remove chart from state immediately
    chartsState.charts = chartsState.charts.filter((c) => c.id !== chartIdToDelete);
    console.log('[ChartSidebarItem] Optimistically removed chart from state');

    // Clear selection if this chart was selected
    if (wasSelected) {
      chartsState.setSelectedChartId(null);
    }

    // Perform actual deletion
    const result = await deleteChart(chartIdToDelete);

    result.match(
      () => {
        console.log('[ChartSidebarItem] Chart deleted successfully');
        // Realtime DELETE event will confirm and sync
      },
      (error) => {
        // Rollback optimistic update: Reload charts to restore state
        void chartsState.loadCharts();
        alert(`Failed to delete chart: ${error.message}`);
      },
    );
  }
</script>
```

**What Happens**:

1. **Optimistic Update**: Immediately removes chart from state (instant UI feedback)
2. Clears selection if deleted chart was selected
3. Calls `deleteChart()` to delete from database
4. On success: Realtime DELETE event confirms deletion
5. On failure: Reloads charts to rollback optimistic update

---

### Step 7: Delete Chart from Database

**Location**: `src/lib/services/chart-service.ts` → `deleteChart()`

**Code Flow**:

```typescript
export async function deleteChart(
  chartId: string,
): Promise<Result<void, Error>> {
  try {
    // Hard delete: Permanently remove chart from database
    const { error } = await supabase.from("charts").delete().eq("id", chartId);

    if (error) {
      return err(new Error(`Failed to delete chart: ${error.message}`));
    }

    return ok(undefined);
  } catch (error) {
    console.error("[deleteChart] Failed to delete chart:", error);
    return err(
      new Error(
        `Failed to delete chart: ${error instanceof Error ? error.message : "Unknown error"}`,
      ),
    );
  }
}
```

**Database Operation**:

```sql
DELETE FROM public.charts
WHERE id = 'chart-uuid';
```

**What Happens**:

1. Performs hard DELETE (not soft delete)
2. Chart is permanently removed from database
3. Name is immediately available for reuse
4. Foreign key constraints with CASCADE DELETE handle related records
5. PostgreSQL WAL is updated with DELETE operation

**CASCADE DELETE**: Related records in `chart_node_assignments` are automatically deleted due to `ON DELETE CASCADE` constraint.

---

### Step 8: Realtime Service Detects Chart DELETE

**Location**: Supabase Realtime Service (Docker container)

**Process**:

```
PostgreSQL WAL (DELETE on charts table)
    ↓
Logical Replication → Supabase Realtime Service
    ↓
Checks RLS policies (SELECT permission required)
    ↓
Formats event for WebSocket transmission
    ↓
Queues event for WebSocket client
```

**Event Format** (Chart DELETE):

```json
{
  "schema": "public",
  "table": "charts",
  "commit_timestamp": "2025-01-20T10:15:30.444Z",
  "eventType": "DELETE",
  "old": {
    "id": "517eb7c9-21a8-4215-b501-7a2cee8ab428",
    "project_id": "project-uuid",
    "name": "map_2",
    "chart_type": "map",
    "is_active": true,
    ...
  },
  "new": null
}
```

**Configuration**:

- **Publication**: `supabase_realtime` includes `public.charts` table
- **RLS Policies**: SELECT policies allow Realtime to read deleted records
- **Extension**: `postgres_cdc_rls` with RLS verification enabled

---

### Step 9: Frontend Receives Chart DELETE Event

**Location**: `src/lib/services/realtime-charts-service.ts` → `handleChartDelete()`

**Code Flow**:

```typescript
private async handleChartDelete(deletedChart: any) {
  console.log("🔄 [RealtimeChartsService] Chart deleted (hard delete):", deletedChart);

  if (!deletedChart || !deletedChart.id) {
    console.error("[RealtimeChartsService] DELETE event missing chart ID");
    return;
  }

  // Filter by current project_id if available
  const currentProjectId = this.projectsState?.currentSelectedProjectId;
  if (currentProjectId && deletedChart.project_id !== currentProjectId) {
    return; // Skip charts from other projects
  }

  // Check if chart exists in state before removing
  const chartExists = this.chartsState.charts.some(
    (c: Chart) => c.id === deletedChart.id,
  );
  const beforeCount = this.chartsState.charts.length;

  // Remove from global state using reactive assignment
  // CRITICAL: Create new array reference to ensure Svelte reactivity triggers
  this.chartsState.charts = [
    ...this.chartsState.charts.filter((c: Chart) => c.id !== deletedChart.id),
  ];
  const afterCount = this.chartsState.charts.length;

  console.log(
    `✅ [RealtimeChartsService] Removed chart: ${deletedChart.name} (${beforeCount} → ${afterCount} charts)`,
  );

  // If this chart was selected, clear selection
  if (this.chartsState.lastSelectedChartId === deletedChart.id) {
    this.chartsState.setSelectedChartId(null);
  }

  // Invalidate cache
  await invalidateTableCache("charts");

  // Defensive check: Verify deletion worked, reload if chart still exists
  setTimeout(() => {
    const stillExists = this.chartsState.charts.some(
      (c: Chart) => c.id === deletedChart.id,
    );
    if (stillExists) {
      console.warn("Chart still exists after DELETE, reloading from database");
      void this.chartsState.loadCharts();
    }
  }, 1000);
}
```

**What Happens**:

1. Receives DELETE event payload with `old` record
2. Filters by current project (skips deletions from other projects)
3. Removes chart from `chartsState.charts` array (idempotent - safe if already removed optimistically)
4. Clears selection if deleted chart was selected
5. Invalidates PostgREST cache
6. Defensive check: Reloads charts if deletion didn't work (fallback)

**State Update**:

```typescript
// Before
chartsState.charts = [
  { id: "chart-1", name: "map", ... },
  { id: "chart-2", name: "map_1", ... },
  { id: "chart-3", name: "map_2", ... }  // ← To be deleted
]

// After
chartsState.charts = [
  { id: "chart-1", name: "map", ... },
  { id: "chart-2", name: "map_1", ... }
  // map_2 removed, name is now available for reuse
]
```

---

### Step 10: UI Components React to State Changes

**Location**: Multiple components using `PostgresChartsState`

**Components Affected**:

1. **charts-sidebar.svelte** (Chart List):

   ```svelte
   <script lang="ts">
     const chartsState = getPostgresChartsState();
     // Filter to only show active charts (safety check)
     let charts = $derived(chartsState.charts.filter((c) => c.is_active !== false));
     let selectedChartId = $derived(chartsState.lastSelectedChartId);
   </script>

   <ChartsSidebarList {charts} selectedId={selectedChartId} onSelect={handleChartSelect} />
   ```

   - Automatically re-renders when `chartsState.charts` changes
   - Deleted chart disappears from sidebar immediately
   - New chart appears in sidebar automatically

2. **charts-sidebar-list.svelte** (Chart Items):

   ```svelte
   <script lang="ts">
     let { charts, selectedId, onSelect }: Props = $props();
   </script>

   {#each charts as chart (chart.id)}
     <ChartSidebarItem {chart} isSelected={selectedId === chart.id} onSelect={onSelect} />
   {/each}
   ```

   - Uses keyed `{#each}` block for efficient updates
   - Automatically updates when `charts` prop changes
   - Deleted chart item disappears, new chart item appears

3. **chart-editor.svelte** (Chart Editor):

   ```svelte
   <script lang="ts">
     const chartsState = getPostgresChartsState();
     let selectedChart = $derived(chartsState.getSelectedChart());
   </script>

   {#if selectedChart}
     <ChartSciChart chart={selectedChart} />
   {:else}
     <p>Select a chart from the sidebar to view it</p>
   {/if}
   ```

   - Automatically updates when selected chart changes
   - Shows empty state if selected chart is deleted

**Svelte Reactivity**:

- `$state` runes trigger automatic re-renders
- `$derived` runes recompute when dependencies change
- Components automatically update without manual refresh
- Keyed `{#each}` blocks efficiently update DOM

---

## Complete Call Stack Summary

### Chart Creation Flow

```
1. User clicks chart type icon
   ↓
2. charts-menubar-item.svelte::handleClick()
   ↓
3. chart-service.ts::createChart()
   ├─ Fetch chart type metadata
   ├─ Generate base name (e.g., "map")
   ├─ generateUniqueChartName()
   │   ├─ Query database for existing names
   │   ├─ Extract suffix numbers
   │   ├─ Find first gap (e.g., map_2 if map_2 was deleted)
   │   └─ Return unique name
   └─ Insert chart into database
       └─ PostgreSQL WAL updated
   ↓
4. Supabase Realtime Service detects INSERT
   ├─ Checks RLS policies
   └─ Sends WebSocket message
   ↓
5. Frontend receives INSERT event
   ├─ RealtimeChartsService::handleChartInsert()
   ├─ Adds chart to chartsState.charts array
   └─ Loads chart state if selected
   ↓
6. UI Components react
   ├─ charts-sidebar.svelte → Chart appears in list
   └─ chart-editor.svelte → Chart appears in editor (if selected)
```

### Chart Deletion Flow

```
1. User right-clicks chart and selects "Delete"
   ↓
2. chart-sidebar-item.svelte::handleDelete()
   ├─ Optimistic update: Remove chart from state immediately
   ├─ Clear selection if deleted chart was selected
   └─ Call deleteChart()
   ↓
3. chart-service.ts::deleteChart()
   └─ DELETE FROM charts WHERE id = ...
       └─ PostgreSQL WAL updated
   ↓
4. Supabase Realtime Service detects DELETE
   ├─ Checks RLS policies
   └─ Sends WebSocket message
   ↓
5. Frontend receives DELETE event
   ├─ RealtimeChartsService::handleChartDelete()
   ├─ Removes chart from chartsState.charts array (idempotent)
   ├─ Clears selection if deleted chart was selected
   └─ Defensive check: Reload if chart still exists
   ↓
6. UI Components react
   ├─ charts-sidebar.svelte → Chart disappears from list
   └─ chart-editor.svelte → Shows empty state (if deleted chart was selected)
```

---

## Name Generation Algorithm

### Gap-Filling Strategy

The name generation algorithm uses a **gap-filling strategy** to reuse names from deleted charts:

1. **Check Base Name**: If `baseName` (e.g., `"map"`) is available, use it
2. **Extract Suffixes**: Collect all suffix numbers from existing names (e.g., `map_1` → `1`, `map_3` → `3`)
3. **Find Gaps**: Look for the first missing number starting from `1`
4. **Fill Gap**: If gap found, use that number (e.g., gap at `2` → `map_2`)
5. **Increment**: If no gaps, use `maxSuffix + 1` (e.g., `[1, 2, 3]` → `map_4`)

### Examples

| Scenario           | Existing Charts           | Next Chart | Reason                   |
| ------------------ | ------------------------- | ---------- | ------------------------ |
| **Gap Filling**    | `map`, `map_1`, `map_3`   | `map_2`    | Fills gap at 2           |
| **No Gaps**        | `map`, `map_1`, `map_2`   | `map_3`    | No gaps, increment       |
| **Base Available** | `map_1`, `map_2`, `map_3` | `map`      | Base name available      |
| **First Chart**    | (none)                    | `map`      | Base name available      |
| **Second Chart**   | `map`                     | `map_1`    | Base taken, start from 1 |

### Database Query

```sql
-- Get all existing chart names matching the pattern
SELECT name FROM charts
WHERE project_id = 'project-uuid'
  AND is_active = true
  AND (name = 'map' OR name LIKE 'map_%')
ORDER BY name;
```

**Result**: `['map', 'map_1', 'map_3']`

**Processing**:

1. Base name `map` exists → skip
2. Extract suffixes: `[1, 3]`
3. Find gaps: Missing `2` → return `map_2`

---

## Database Schema

### Charts Table

```sql
CREATE TABLE public.charts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES public.projects(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    chart_type TEXT NOT NULL,
    chart_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    data_source_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    rendering_config JSONB DEFAULT '{}'::jsonb,
    created_by UUID NOT NULL REFERENCES auth.users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    tags TEXT[] DEFAULT '{}',
    is_active BOOLEAN DEFAULT true,

    -- CRITICAL: Unique constraint ensures no duplicate names per project
    CONSTRAINT charts_project_name_unique UNIQUE (project_id, name)
);
```

### Key Constraints

- **`charts_project_name_unique`**: Ensures chart names are unique within a project
- **`ON DELETE CASCADE`**: Deleting a project deletes all its charts
- **Foreign Keys**: `created_by` references `auth.users`, `project_id` references `projects`

### Related Tables

**chart_node_assignments** (Many-to-many: charts ↔ nodes):

```sql
CREATE TABLE public.chart_node_assignments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chart_id UUID NOT NULL REFERENCES charts(id) ON DELETE CASCADE,
    node_id UUID NOT NULL REFERENCES nodes(id) ON DELETE CASCADE,
    assignment_config JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chart_node_assignments_unique UNIQUE (chart_id, node_id)
);
```

**CASCADE DELETE**: When a chart is deleted, all related `chart_node_assignments` are automatically deleted.

---

## Realtime Configuration

### SQL Migration

```sql
-- Add charts table to realtime publication
ALTER PUBLICATION supabase_realtime ADD TABLE public.charts;

-- Enable RLS
ALTER TABLE public.charts ENABLE ROW LEVEL SECURITY;

-- Create RLS policies for SELECT (required for Realtime)
CREATE POLICY "Users can view charts in their projects"
    ON public.charts
    FOR SELECT
    USING (
        EXISTS (
            SELECT 1 FROM public.projects
            WHERE projects.id = charts.project_id
            AND (
                projects.owner_id = auth.uid()
                OR EXISTS (
                    SELECT 1 FROM public.project_permissions
                    WHERE project_permissions.project_id = projects.id
                    AND project_permissions.user_id = auth.uid()
                )
            )
        )
    );

-- Create RLS policy for DELETE
CREATE POLICY "Users can delete charts in their projects"
    ON public.charts
    FOR DELETE
    USING (
        EXISTS (
            SELECT 1 FROM public.projects
            WHERE projects.id = charts.project_id
            AND (
                projects.owner_id = auth.uid()
                OR EXISTS (
                    SELECT 1 FROM public.project_permissions
                    WHERE project_permissions.project_id = projects.id
                    AND project_permissions.user_id = auth.uid()
                    AND project_permissions.role IN ('owner', 'editor', 'admin')
                )
            )
        )
    );
```

### Frontend Initialization

**Location**: `src/lib/state/postgres/postgres-charts-state.svelte.ts`

```typescript
export class PostgresChartsState {
  charts = $state<Chart[]>([]);
  lastSelectedChartId = $state<string | null>(null);

  constructor() {
    // Initialize realtime service
    requestAnimationFrame(() => {
      setTimeout(() => {
        this.initializeRealtime();
      }, 200);
    });
  }

  private async initializeRealtime() {
    realtimeChartsService.initialize(this, this.projectsState || undefined);
    await realtimeChartsService.connect();
  }
}
```

---

## Benefits

### ✅ **Gap-Filling Name Reuse**

- Deleted chart names are immediately available for reuse
- Prevents name inflation (no `map_100` when `map_2` is available)
- Clean, predictable naming scheme

### ✅ **Real-time Updates**

- UI updates automatically when charts are created/deleted
- No manual refresh required
- Consistent state across all components
- Multi-client synchronization (all users see changes simultaneously)

### ✅ **Optimistic UI Updates**

- Instant feedback when deleting charts
- Better user experience (no waiting for database)
- Rollback mechanism if deletion fails

### ✅ **Type Safety**

- TypeScript types ensure correct data structures
- `Chart` type matches backend schema
- Compile-time validation prevents errors

### ✅ **Performance**

- Cache invalidation ensures fresh data
- Realtime subscriptions avoid polling
- Efficient WebSocket communication
- Defensive reloads only when needed

### ✅ **Scalability**

- Same pattern can be extended to other tables
- Realtime service is reusable
- RLS policies ensure security
- Gap-filling algorithm scales to thousands of charts

---

## Troubleshooting

### Chart Not Appearing in UI After Creation

1. **Check Realtime Connection**:

   ```typescript
   console.log(realtimeChartsService.getConnectionStatus());
   ```

2. **Verify Chart in Database**:

   ```sql
   SELECT * FROM charts WHERE id = 'chart-id';
   ```

3. **Check RLS Policies**:

   ```sql
   SELECT * FROM pg_policies WHERE tablename = 'charts';
   ```

4. **Check Browser Console**: Look for `[RealtimeChartsService]` logs

### Chart Not Disappearing After Deletion

1. **Check Optimistic Update**: Chart should disappear immediately
2. **Check Realtime DELETE Event**: Look for `[RealtimeChartsService] Chart deleted` log
3. **Check Defensive Reload**: After 1 second, check if chart still exists
4. **Verify Database**: Confirm chart is actually deleted:
   ```sql
   SELECT * FROM charts WHERE id = 'chart-id';
   ```

### Duplicate Key Constraint Error

1. **Check Name Generation**: Verify `generateUniqueChartName()` is finding gaps correctly
2. **Check Database State**: Ensure deleted charts are actually removed:
   ```sql
   SELECT name FROM charts WHERE project_id = 'project-id' AND name LIKE 'map%';
   ```
3. **Check Race Conditions**: Multiple users creating charts simultaneously might cause conflicts

### Name Not Reusing After Deletion

1. **Verify Hard Delete**: Ensure chart is actually deleted (not soft-deleted):
   ```sql
   SELECT COUNT(*) FROM charts WHERE id = 'deleted-chart-id';
   -- Should return 0
   ```
2. **Check Name Generation Query**: Verify it only checks active charts:
   ```sql
   SELECT name FROM charts
   WHERE project_id = 'project-id'
     AND is_active = true
     AND (name = 'map' OR name LIKE 'map_%');
   ```
3. **Check Gap-Filling Logic**: Verify algorithm finds gaps correctly

### Realtime Not Working

1. **Verify Publication**:

   ```sql
   SELECT * FROM pg_publication_tables WHERE tablename = 'charts';
   ```

2. **Check Realtime Service Logs**: Look for "SUBSCRIBED" status in console

3. **Verify Tenant Configuration**: Ensure tenant exists in `_realtime.tenants`

4. **Check WebSocket Connection**: Verify WebSocket is connected in browser DevTools

---

## Key Files Reference

| File                              | Purpose                                        |
| --------------------------------- | ---------------------------------------------- |
| `chart-service.ts`                | Chart creation and deletion functions          |
| `chart-name-validation.ts`        | Gap-filling name generation logic              |
| `realtime-charts-service.ts`      | Realtime subscription service for charts table |
| `postgres-charts-state.svelte.ts` | Global state management for charts             |
| `charts-sidebar.svelte`           | Chart list component with reactive updates     |
| `chart-sidebar-item.svelte`       | Individual chart item with delete handler      |
| `charts-menubar-item.svelte`      | Chart creation handler                         |

---

## Summary

The realtime chart database sync system provides seamless, reactive chart management with intelligent name reuse. By leveraging Supabase Realtime subscriptions, gap-filling name generation, and optimistic UI updates, the system ensures:

- ✅ **Instant UI Updates**: Charts appear/disappear immediately
- ✅ **Name Reuse**: Deleted chart names are reused intelligently
- ✅ **Multi-Client Sync**: All users see changes simultaneously
- ✅ **Error Recovery**: Rollback mechanisms prevent data loss
- ✅ **Type Safety**: TypeScript ensures correct data structures
- ✅ **Performance**: Efficient WebSocket communication and caching

This architecture ensures type safety, performance, and maintainability throughout the chart creation and deletion flow.





