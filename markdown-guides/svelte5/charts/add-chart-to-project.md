# Realtime Chart Creation: Adding Charts to Project

## Overview

This document explains the complete end-to-end implementation of realtime chart creation in the MudRock application. It describes how chart type options are displayed to users, how clicking a chart type icon creates a new chart instance in the database, and how the UI automatically updates in real-time through Supabase Realtime subscriptions.

## Architecture Overview

The chart creation system consists of several interconnected components:

```
┌─────────────────────────────────────────────────────────────┐
│                    User Interaction                          │
│  Click Chart Type Icon → charts-menubar-item.svelte         │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Frontend Service Layer                         │
│  createChart() → Insert into charts table                    │
│    ├─ Get authenticated user ID                             │
│    ├─ Fetch chart type from chart_types table                │
│    ├─ Generate unique chart name (with suffix if needed)    │
│    └─ Insert chart with default config                      │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              PostgreSQL Database                            │
│  INSERT INTO charts (...)                                   │
│  ← Publication: supabase_realtime                           │
│  ← RLS policies enabled                                     │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Logical Replication (WAL)
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Supabase Realtime Service                      │
│  Detects INSERT on charts table                             │
│  ← Extension: postgres_cdc_rls                              │
│  ← Verifies RLS policies                                    │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ WebSocket (ws://)
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Frontend Realtime Services                     │
│  RealtimeChartsService → Updates PostgresChartsState.charts  │
│  ← PostgresChartsState (Global State)                      │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              UI Components                                  │
│  charts-sidebar.svelte → Chart list updates                 │
│  charts-list.svelte → New chart appears in sidebar          │
└─────────────────────────────────────────────────────────────┘
```

## Complete Call Stack & Data Flow

### Step 1: Load Chart Types from Database

**Location**: `src/lib/components/pages/home/charts/charts-menubar/charts-menubar.svelte`

**Code Flow**:

```svelte
<script lang="ts">
  import { getChartTypes } from '$lib/services/chart-service';

  let chartTypes = $state<ChartType[]>([]);

  $effect(() => {
    void loadChartTypes();
  });

  async function loadChartTypes() {
    const result = await getChartTypes();
    result.match(
      (types) => { chartTypes = types; },
      (error) => { console.error('Failed to load chart types:', error); }
    );
  }
</script>
```

**What Happens**:

1. Component mounts and `$effect()` triggers `loadChartTypes()`
2. Calls `getChartTypes()` from `chart-service.ts`
3. Fetches active chart types from `chart_types` table using `getTableData("chart_types")`
4. Filters for `is_active = true` chart types
5. Sorts by `sort_order`
6. Displays chart types as icons in menubar

**Database Query**:

```sql
SELECT * FROM chart_types
WHERE is_active = true
ORDER BY sort_order ASC;
```

**Example Chart Types**:

- `xy` - XY Plot (LineChart icon)
- `well_log` - Well Log (BarChart icon)

---

### Step 2: User Clicks Chart Type Icon

**Location**: `src/lib/components/pages/home/charts/charts-menubar/charts-menubar-item/charts-menubar-item.svelte`

**Code Flow**:

```svelte
<script lang="ts">
  import { createChart } from '$lib/services/chart-service';
  import { getPostgresProjectsState } from '$lib/state/postgres/postgres-projects-state.svelte';

  const projectsState = getPostgresProjectsState();
  let isCreating = $state(false);

  async function handleClick() {
    const projectId = projectsState.currentSelectedProjectId;

    if (!projectId) {
      alert('Please select a project first');
      return;
    }

    try {
      isCreating = true;

      const result = await createChart({
        chartTypeId,
        projectId,
      });

      result.match(
        (chart) => {
          console.log('Chart created:', chart.id);
          onSelect?.(chartTypeId);
        },
        (error) => {
          alert(`Failed to create chart: ${error.message}`);
        },
      );
    } finally {
      isCreating = false;
    }
  }
</script>

<button onclick={handleClick} disabled={isCreating}>
  <!-- Icon and display name -->
</button>
```

**What Happens**:

1. Validates that a project is selected (`currentSelectedProjectId`)
2. Shows loading state (`isCreating = true`) - displays spinner
3. Calls `createChart()` with `chartTypeId` and `projectId`
4. Handles success/error using `neverthrow` Result type
5. Calls `onSelect` callback to update UI state

---

### Step 3: Create Chart in Database

**Location**: `src/lib/services/chart-service.ts` → `createChart()`

**Code Flow**:

```typescript
export async function createChart(params: {
  chartTypeId: string;
  projectId: string;
  displayName?: string;
}): Promise<Result<Chart, Error>> {
  // Step 1: Get authenticated user ID
  const {
    data: { user },
    error: userError,
  } = await supabase.auth.getUser();
  if (userError || !user) {
    return err(new Error("User not authenticated"));
  }

  // Step 2: Fetch chart type metadata
  const { data: chartType } = await supabase
    .from("chart_types")
    .select("*")
    .eq("chart_type_id", params.chartTypeId)
    .eq("is_active", true)
    .single();

  // Step 3: Generate unique chart name
  const baseName = chartType.display_name.toLowerCase().replace(/\s+/g, "_");
  const uniqueName = await generateUniqueChartName(params.projectId, baseName);

  // Step 4: Create chart with default config
  const chart = {
    project_id: params.projectId,
    name: uniqueName, // e.g., "xy_plot", "xy_plot_1", "xy_plot_2"
    description: chartType.description || null,
    chart_type: params.chartTypeId, // "xy", "well_log", etc.
    chart_config: chartType.default_config || {},
    data_source_config: chartType.default_data_source_config || {},
    rendering_config: null,
    tags: null,
    is_active: true,
    created_by: user.id, // Current authenticated user
  };

  // Step 5: Insert into database
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
    id, project_id, name, description, chart_type,
    chart_config, data_source_config, rendering_config,
    tags, is_active, created_by, created_at, updated_at
) VALUES (
    gen_random_uuid(),
    'project-uuid',
    'xy_plot',  -- or 'xy_plot_1', 'xy_plot_2', etc.
    'Basic XY scatter or line plot for comparing two variables',
    'xy',
    '{"xAxis": {...}, "yAxes": [...], "series": []}'::jsonb,
    '{"type": "node", "nodeIds": []}'::jsonb,
    NULL,
    '{}'::text[],
    true,
    'user-uuid',  -- from auth.users
    NOW(),
    NOW()
);
```

**What Happens**:

1. Gets current authenticated user ID from Supabase auth
2. Fetches chart type metadata from `chart_types` table
3. Generates unique chart name (checks for duplicates, adds `_1`, `_2` suffix if needed)
4. Creates chart record with default configuration from chart type
5. Inserts chart into `charts` table via Supabase PostgREST
6. Returns `Chart` object with created chart data

**PostgreSQL WAL**: Write-Ahead Log is updated with INSERT operation

---

### Step 4: Chart Name Validation

**Location**: `src/lib/utils/charts/chart-name-validation.ts`

**Code Flow**:

```typescript
export async function generateUniqueChartName(
  projectId: string,
  baseName: string,
): Promise<string> {
  // Get existing chart names in the same project
  const existingNames = await getExistingChartNamesInProject(
    projectId,
    baseName,
  );

  // If base name doesn't exist, use it as-is
  if (!existingNames.includes(baseName)) {
    return baseName; // "xy_plot"
  }

  // Find highest suffix number
  let maxSuffix = 0;
  for (const name of existingNames) {
    if (name === baseName) {
      maxSuffix = Math.max(maxSuffix, 0);
    } else if (name.startsWith(`${baseName}_`)) {
      const suffixNum = parseInt(name.substring(baseName.length + 1), 10);
      if (!isNaN(suffixNum)) {
        maxSuffix = Math.max(maxSuffix, suffixNum);
      }
    }
  }

  // Return name with incremented suffix
  return `${baseName}_${maxSuffix + 1}`; // "xy_plot_1", "xy_plot_2", etc.
}
```

**Database Query**:

```sql
SELECT name FROM charts
WHERE project_id = 'project-uuid'
  AND is_active = true
  AND (name = 'xy_plot' OR name LIKE 'xy_plot_%');
```

**Name Generation Examples**:

- First chart: `xy_plot`
- Second chart: `xy_plot_1`
- Third chart: `xy_plot_2`
- Well log charts: `well_log`, `well_log_1`, `well_log_2`, etc.

**Key Features**:

- ✅ Only checks names within the same `project_id`
- ✅ Different projects can have charts with the same name
- ✅ Automatically increments suffix (`_1`, `_2`, `_3`, etc.)
- ✅ Handles edge cases (missing base name, non-numeric suffixes)

---

### Step 5: Realtime Service Detects Changes

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
    "name": "xy_plot",
    "chart_type": "xy",
    "chart_config": {
      "xAxis": {"type": "numeric", "title": "X Axis"},
      "yAxes": [{"id": "left", "type": "numeric", "title": "Y Axis"}],
      "series": []
    },
    "data_source_config": {"type": "node", "nodeIds": []},
    "rendering_config": null,
    "created_by": "user-uuid",
    "is_active": true,
    "created_at": "2025-01-20T10:14:15.441571+00:00",
    ...
  },
  "old": null
}
```

**Configuration**:

- **Publication**: `supabase_realtime` includes `public.charts` table
- **RLS Policies**: SELECT policies for `anon` and `authenticated` roles (see `013-charts-schema.sql`)
- **Extension**: `postgres_cdc_rls` with RLS verification enabled

---

### Step 6: Frontend Receives Chart INSERT Event

**Location**: `src/lib/services/realtime-charts-service.ts` → `handleChartInsert()`

**Code Flow**:

```typescript
private async handleChartInsert(newChart: any) {
  // Filter by current project_id
  const currentProjectId = this.projectsState?.currentSelectedProjectId;
  if (currentProjectId && newChart.project_id !== currentProjectId) {
    console.log('Skipping chart (different project)');
    return;
  }

  // Convert database record to Chart type
  const chart: Chart = {
    id: newChart.id,
    project_id: newChart.project_id,
    name: newChart.name,
    description: newChart.description ?? null,
    chart_type: newChart.chart_type,
    chart_config: newChart.chart_config ?? {},
    data_source_config: newChart.data_source_config ?? {},
    rendering_config: newChart.rendering_config ?? null,
    created_by: newChart.created_by,
    created_at: newChart.created_at ?? new Date().toISOString(),
    updated_at: newChart.updated_at ?? new Date().toISOString(),
    tags: newChart.tags ?? [],
    is_active: newChart.is_active ?? true,
  };

  // Check if chart already exists (avoid duplicates)
  const exists = this.chartsState.charts.some((c: Chart) => c.id === chart.id);
  if (exists) {
    // Update existing chart
    this.chartsState.charts = this.chartsState.charts.map((c: Chart) =>
      c.id === chart.id ? chart : c,
    );
  } else {
    // Add new chart to global state
    this.chartsState.charts = [...this.chartsState.charts, chart];
    console.log('Added new chart to global state:', chart.name);
  }

  // Invalidate cache
  await invalidateTableCache("charts");
}
```

**What Happens**:

1. Filters by current project ID (only processes charts for selected project)
2. Converts database record to `Chart` type
3. Checks if chart already exists in state (avoid duplicates)
4. Updates `PostgresChartsState.charts` array
5. Invalidates PostgREST cache for "charts" table

**State Update**:

```typescript
// Before
charts = [
  { id: "chart-1", name: "xy_plot", chart_type: "xy", ... }
]

// After
charts = [
  { id: "chart-1", name: "xy_plot", chart_type: "xy", ... },
  { id: "new-chart-id", name: "xy_plot_1", chart_type: "xy", ... }  // ← New chart added
]
```

---

### Step 7: UI Components React to State Changes

**Location**: Multiple components using `PostgresChartsState`

**Components Affected**:

1. **charts-sidebar.svelte** (Chart List):

   ```svelte
   <script lang="ts">
     const chartsState = getPostgresChartsState();
     let charts = $derived(chartsState.charts);
   </script>

   <ChartsList {charts} selectedId={selectedChartId} onSelect={handleChartSelect} />
   ```

   - Automatically re-renders when `charts` array changes
   - New chart appears in sidebar list

2. **charts-list.svelte** (Chart Items):

   ```svelte
   <script lang="ts">
     let { charts, selectedId, onSelect }: Props = $props();
   </script>

   {#each charts as chart (chart.id)}
     <ChartItem {chart} isSelected={selectedId === chart.id} onSelect={onSelect} />
   {/each}
   ```

   - Automatically updates when `charts` prop changes
   - New chart item appears in list

3. **chart-item.svelte** (Individual Chart):

   ```svelte
   <script lang="ts">
     let { chart, isSelected, onSelect }: Props = $props();
   </script>

   <button onclick={() => onSelect?.(chart.id)}>
     {chart.name}
   </button>
   ```

   - Displays chart name and handles selection

**Svelte Reactivity**:

- `$state` runes trigger automatic re-renders
- `$derived` runes recompute when dependencies change
- Components automatically update without manual refresh

---

## Complete Call Stack Summary

```
1. Component mounts
   ↓
2. charts-menubar.svelte::loadChartTypes()
   ↓
3. chart-service.ts::getChartTypes()
   └─ getTableData("chart_types") → PostgreSQL SELECT
   └─ Filter is_active = true, sort by sort_order
   ↓
4. User clicks chart type icon
   ↓
5. charts-menubar-item.svelte::handleClick()
   ↓
6. chart-service.ts::createChart()
   ├─ supabase.auth.getUser() → Get user ID
   ├─ supabase.from('chart_types').select() → Get chart type metadata
   ├─ generateUniqueChartName() → Check for duplicates, add suffix
   │   └─ getExistingChartNamesInProject() → Query charts table
   └─ supabase.from('charts').insert() → PostgreSQL INSERT
       └─ PostgreSQL WAL updated
   ↓
7. Supabase Realtime Service detects changes
   └─ Chart INSERT event → WebSocket message
   ↓
8. Frontend receives event
   └─ RealtimeChartsService::handleChartInsert()
       └─ Updates PostgresChartsState.charts array
   ↓
9. UI Components react
   ├─ charts-sidebar.svelte → Chart list updates
   ├─ charts-list.svelte → New chart item appears
   └─ chart-item.svelte → Chart name displayed
```

---

## Key Files Reference

| File                              | Purpose                                        |
| --------------------------------- | ---------------------------------------------- |
| `charts-menubar.svelte`           | Loads and displays chart type options          |
| `charts-menubar-item.svelte`      | Individual chart type icon with click handler  |
| `chart-service.ts`                | Chart creation and chart type fetching         |
| `chart-name-validation.ts`        | Unique name generation with suffix increment   |
| `realtime-charts-service.ts`      | Realtime subscription service for charts table |
| `postgres-charts-state.svelte.ts` | Global state management for charts             |
| `charts-sidebar.svelte`           | Sidebar component displaying chart list        |
| `013-charts-schema.sql`           | SQL migration for charts table schema          |
| `014-chart-types-registry.sql`    | SQL migration for chart_types registry         |
| `015-add-xy-chart-type.sql`       | SQL migration adding 'xy' to charts constraint |

---

## Database Schema

### Chart Types Table (`chart_types`)

```sql
CREATE TABLE public.chart_types (
    id UUID PRIMARY KEY,
    chart_type_id TEXT NOT NULL UNIQUE, -- "xy", "well_log", etc.
    display_name TEXT NOT NULL, -- "XY Plot", "Well Log"
    description TEXT,
    category TEXT, -- "basic", "advanced", etc.
    icon_name TEXT, -- "LineChart", "BarChart"
    default_config JSONB DEFAULT '{}'::jsonb,
    default_data_source_config JSONB DEFAULT '{}'::jsonb,
    is_active BOOLEAN DEFAULT true,
    sort_order INTEGER DEFAULT 0
);
```

### Charts Table (`charts`)

```sql
CREATE TABLE public.charts (
    id UUID PRIMARY KEY,
    project_id UUID NOT NULL REFERENCES projects(id),
    name TEXT NOT NULL,
    description TEXT,
    chart_type TEXT NOT NULL CHECK (chart_type IN (
        'xy', 'well_log', 'cross_well', 'statistical', 'time_series', 'custom'
    )),
    chart_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    data_source_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    rendering_config JSONB,
    created_by UUID NOT NULL REFERENCES auth.users(id),
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    tags TEXT[] DEFAULT '{}',
    is_active BOOLEAN DEFAULT true,

    CONSTRAINT charts_project_name_unique UNIQUE (project_id, name)
);
```

**Key Constraints**:

- ✅ `project_id` + `name` must be unique (allows same name in different projects)
- ✅ `chart_type` must be one of valid types (`xy`, `well_log`, etc.)
- ✅ `created_by` must reference authenticated user

---

## Realtime Configuration

### SQL Migration (`013-charts-schema.sql`)

1. **Add to Publication**:

   ```sql
   ALTER PUBLICATION supabase_realtime ADD TABLE public.charts;
   ```

2. **Enable RLS**:

   ```sql
   ALTER TABLE public.charts ENABLE ROW LEVEL SECURITY;
   ```

3. **Create RLS Policies**:
   - SELECT policies for `anon` and `authenticated`
   - INSERT, UPDATE, DELETE policies based on project permissions

### Frontend Initialization

**Location**: `postgres-charts-state.svelte.ts`

```typescript
export class PostgresChartsState {
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

## Name Validation Details

### Validation Rules

**Rule**: Chart names must be unique per `project_id`.

**Rationale**:

- Same chart name can exist in different projects
- Same project cannot have duplicate chart names
- Automatic suffix increment prevents conflicts

**Validation Points**:

1. **Before Insert**: Check existing names in same project
2. **Suffix Generation**: Increment `_1`, `_2`, `_3`, etc.
3. **Database Constraint**: `UNIQUE (project_id, name)` ensures uniqueness

**Example**:

- ✅ Project A + `xy_plot` → Allowed
- ✅ Project B + `xy_plot` → Allowed (different project)
- ❌ Project A + `xy_plot` (duplicate) → Auto-renamed to `xy_plot_1`
- ✅ Project A + `xy_plot_1` → Allowed (different name)

---

## Benefits

### ✅ **Real-time Updates**

- UI updates automatically when charts are created
- No manual refresh required
- Consistent state across all components

### ✅ **Type Safety**

- TypeScript types ensure correct data structures
- `Chart` type matches backend schema
- Compile-time validation prevents errors

### ✅ **Name Uniqueness**

- Automatic suffix generation prevents conflicts
- Project-scoped validation (different projects can have same names)
- Database constraint ensures uniqueness

### ✅ **Performance**

- Cache invalidation ensures fresh data
- Realtime subscriptions avoid polling
- Efficient WebSocket communication

### ✅ **Scalability**

- Same pattern can be extended to other tables
- Realtime service is reusable
- RLS policies ensure security

---

## Troubleshooting

### Chart Not Appearing in UI

1. **Check Realtime Connection**:

   ```typescript
   console.log(chartsState.getRealtimeStatus());
   ```

2. **Verify Chart in Database**:

   ```sql
   SELECT * FROM charts WHERE project_id = 'project-uuid' ORDER BY created_at DESC;
   ```

3. **Check RLS Policies**:

   ```sql
   SELECT * FROM pg_policies WHERE tablename = 'charts';
   ```

### Realtime Not Working

1. **Verify Publication**:

   ```sql
   SELECT * FROM pg_publication_tables WHERE tablename = 'charts';
   ```

2. **Check Realtime Service Logs**:
   - Look for "SUBSCRIBED" status in console
   - Check for RLS policy errors

3. **Verify Project Filtering**:
   - Ensure `currentSelectedProjectId` is set
   - Check that realtime service receives `projectsState`

### Name Validation Issues

1. **Check Existing Names**:

   ```sql
   SELECT name FROM charts
   WHERE project_id = 'project-uuid'
   AND (name = 'xy_plot' OR name LIKE 'xy_plot_%');
   ```

2. **Verify Suffix Logic**:
   - Check `generateUniqueChartName()` function
   - Ensure project_id is correctly passed

---

## Summary

The realtime chart creation system provides a seamless, reactive experience for adding charts to projects. By leveraging Supabase Realtime subscriptions, the UI automatically updates when charts are created, ensuring users always see the latest state without manual refresh.

**Key Design Decisions**:

- ✅ **Project-scoped validation**: Names unique per project, not globally
- ✅ **Automatic suffix generation**: Prevents naming conflicts
- ✅ **RLS policies**: Secure access control for realtime subscriptions
- ✅ **State management**: Global state ensures consistency across components
- ✅ **Type safety**: TypeScript types match backend structures
- ✅ **Performance**: Cache invalidation + realtime subscriptions
- ✅ **Error handling**: Uses `neverthrow` Result type for type-safe error handling

This architecture ensures type safety, performance, and maintainability throughout the chart creation flow.
