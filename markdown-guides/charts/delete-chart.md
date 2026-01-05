# Chart Deletion with PostgreSQL and Realtime Sync

This guide explains how we delete charts from our application using PostgreSQL for metadata storage and Supabase Realtime for reactive UI updates.

## Architecture Overview

### Third-Party APIs Used

1. **Supabase JavaScript Client** (`@supabase/supabase-js`)
   - Official Supabase client for PostgreSQL operations
   - Handles authentication and RLS policies
   - Provides type safety and error handling

2. **Supabase Realtime** (via Supabase JavaScript Client)
   - WebSocket-based realtime subscriptions
   - Automatic UI updates when database changes occur
   - Respects RLS policies for secure updates

3. **PostgreSQL with RLS** (via Supabase)
   - Row Level Security for data isolation
   - Soft delete pattern (`is_active = false`)
   - Foreign key constraints with CASCADE DELETE

### Soft Delete Pattern

**Soft Delete**: We mark charts as inactive instead of hard-deleting them:

- ✅ **Database**: Chart record remains but `is_active = false`
- ✅ **UI**: Chart immediately disappears from sidebar
- ✅ **State**: Chart removed from reactive state array
- ✅ **Selection**: Selected chart cleared if deleted
- ✅ **Realtime**: Changes propagate automatically via WebSocket

## Step-by-Step Implementation Process

### Step 1: User Interaction

**Component**: `chart-sidebar-item.svelte`
**Third-Party APIs**: None (Pure SvelteKit)

```typescript
// User right-clicks chart item and selects "Delete" from context menu
async function handleDelete() {
  try {
    const result = await deleteChart(chart.id);

    result.match(
      () => {
        console.log("[ChartSidebarItem] Chart deleted:", chart.id);
        // If this chart was selected, clear selection
        if (chartsState.lastSelectedChartId === chart.id) {
          chartsState.setSelectedChartId(null);
        }
        // Realtime updates will propagate via RealtimeChartsService
      },
      (error) => {
        console.error("[ChartSidebarItem] Failed to delete chart:", error);
        alert(`Failed to delete chart: ${error.message}`);
      },
    );
  } catch (error) {
    console.error("[ChartSidebarItem] Unexpected error deleting chart:", error);
    alert(
      `Failed to delete chart: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
}
```

**What Happens**:

1. User clicks "Delete" in context menu
2. `handleDelete()` is called immediately (no confirmation dialog)
3. Calls `deleteChart()` service function
4. Handles success/error with `neverthrow` Result type
5. Clears selection if deleted chart was selected

### Step 2: Chart Deletion Service

**Component**: `src/lib/services/chart-service.ts`
**Third-Party APIs**: Supabase JavaScript Client

```typescript
import { supabase } from "$lib/supabase";
import { err, ok, type Result } from "neverthrow";

/**
 * Delete a chart (soft delete: mark as inactive)
 */
export async function deleteChart(
  chartId: string,
): Promise<Result<void, Error>> {
  try {
    const { error } = await supabase
      .from("charts")
      .update({
        is_active: false,
        updated_at: new Date().toISOString(),
      })
      .eq("id", chartId);

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

**What Happens**:

1. Updates chart record in `charts` table
2. Sets `is_active = false` (soft delete)
3. Updates `updated_at` timestamp
4. Returns `Result<void, Error>` for type-safe error handling
5. PostgreSQL WAL (Write-Ahead Log) is updated with UPDATE operation

**Database Query**:

```sql
UPDATE public.charts
SET is_active = false,
    updated_at = NOW()
WHERE id = 'chart-uuid';
```

### Step 3: Supabase Realtime Detection

**Component**: `src/lib/services/realtime-charts-service.ts`
**Third-Party APIs**: Supabase Realtime (WebSocket)

```typescript
// Realtime subscription listens for UPDATE events on charts table
this.channel = this.supabase
  .channel("charts-changes", {
    config: {
      // postgres_changes respects RLS policies automatically
    },
  })
  .on(
    "postgres_changes",
    {
      event: "*", // Listen to INSERT, UPDATE, DELETE
      schema: "public",
      table: "charts",
    },
    async (payload: any) => {
      console.log("🔄 [RealtimeChartsService] Received change:", payload);
      await this.handleChartsChange(payload);
    },
  )
  .subscribe();
```

**What Happens**:

1. PostgreSQL WAL change triggers Supabase Realtime
2. WebSocket message sent to connected clients
3. `handleChartsChange()` processes the event
4. Event type is `UPDATE` (not `DELETE` because we're soft-deleting)

### Step 4: Realtime Update Handler

**Component**: `src/lib/services/realtime-charts-service.ts`
**Method**: `handleChartUpdate()`

```typescript
private async handleChartUpdate(newChart: any, oldChart: any) {
  // Filter by current project_id if available
  const currentProjectId = this.projectsState?.currentSelectedProjectId;
  if (currentProjectId && newChart.project_id !== currentProjectId) {
    return; // Skip charts from other projects
  }

  // Check if chart was soft-deleted (is_active changed from true to false)
  const wasActive = oldChart?.is_active !== false;
  const isNowActive = newChart.is_active !== false;

  if (wasActive && !isNowActive) {
    // Chart was soft-deleted, remove from state
    console.log(
      "🔄 [RealtimeChartsService] Chart soft-deleted, removing from state:",
      newChart.name,
    );

    // Remove from global state
    this.chartsState.charts = this.chartsState.charts.filter(
      (c: Chart) => c.id !== newChart.id,
    );

    // If this chart was selected, clear selection
    if (this.chartsState.lastSelectedChartId === newChart.id) {
      console.log(
        "🔄 [RealtimeChartsService] Deleted chart was selected, clearing selection",
      );
      this.chartsState.setSelectedChartId(null);
    }

    // Invalidate cache for "charts" table to ensure fresh data on next fetch
    await invalidateTableCache("charts");
    return;
  }

  // ... handle other update cases (reactivation, normal updates)
}
```

**What Happens**:

1. Receives UPDATE event payload with `old` and `new` records
2. Detects soft delete: `old.is_active = true` → `new.is_active = false`
3. Removes chart from `chartsState.charts` array
4. Clears selection if deleted chart was selected
5. Invalidates PostgREST cache for fresh data on next fetch
6. UI automatically updates via Svelte reactivity

### Step 5: Frontend State Update

**Component**: `src/lib/state/postgres/postgres-charts-state.svelte.ts`
**Property**: `charts = $state<Chart[]>([])`

```typescript
export class PostgresChartsState {
  // All available chart objects from the backend (filtered by current project)
  charts = $state<Chart[]>([]);
  lastSelectedChartId = $state<string | null>(null);

  // Charts are filtered by is_active when loaded
  async loadCharts(projectId: string) {
    const { data, error } = await supabase
      .from("charts")
      .select("*")
      .eq("project_id", projectId)
      .eq("is_active", true) // Only load active charts
      .order("created_at", { ascending: false });

    // ... convert and set charts
  }
}
```

**What Happens**:

1. `charts` array is reactive (`$state`)
2. When chart is removed from array, Svelte automatically updates UI
3. `charts-sidebar.svelte` displays filtered charts
4. Deleted chart disappears immediately from sidebar

### Step 6: UI Component Reactivity

**Component**: `src/lib/components/pages/home/charts/charts-sidebar/charts-sidebar.svelte`

```svelte
<script lang="ts">
  const chartsState = getPostgresChartsState();
  // Filter to only show active charts (safety check - realtime service also filters)
  let charts = $derived(chartsState.charts.filter((c) => c.is_active !== false));
  let selectedChartId = $derived(chartsState.lastSelectedChartId);
</script>

<div class="h-full bg-white border-r {STYLE_CONSTANTS.SIDEBAR.WIDTH} flex flex-col">
  <ChartsSidebarList
    {charts}
    selectedId={selectedChartId}
    onSelect={handleChartSelect}
  />
</div>
```

**What Happens**:

1. `$derived` automatically recomputes when `chartsState.charts` changes
2. Filter ensures only active charts are displayed
3. `ChartsSidebarList` receives updated chart array
4. Deleted chart item disappears from UI

## Complete Data Flow

```
User clicks "Delete" in context menu
    ↓
chart-sidebar-item.svelte::handleDelete()
    ↓
chart-service.ts::deleteChart()
    ↓
Supabase Client: UPDATE charts SET is_active = false
    ↓
PostgreSQL: WAL updated with UPDATE operation
    ↓
Supabase Realtime: WebSocket message sent
    ↓
realtime-charts-service.ts::handleChartUpdate()
    ├─ Detects soft delete (is_active: true → false)
    ├─ Removes chart from chartsState.charts array
    ├─ Clears selection if deleted chart was selected
    └─ Invalidates cache
    ↓
PostgresChartsState.charts updated (reactive)
    ↓
charts-sidebar.svelte::charts $derived recomputes
    ↓
ChartsSidebarList receives filtered charts
    ↓
UI updates: Deleted chart disappears from sidebar
```

## Key Features

### 1. Soft Delete Pattern

- **Why**: Preserves data for audit trails and potential recovery
- **How**: Sets `is_active = false` instead of DELETE
- **Benefits**:
  - Can restore deleted charts if needed
  - Maintains referential integrity
  - Preserves chart history

### 2. Realtime Synchronization

- **Immediate Updates**: UI updates instantly via WebSocket
- **Multi-Client**: All connected clients see deletion simultaneously
- **RLS Security**: Realtime respects Row Level Security policies
- **Project Filtering**: Only processes charts from current project

### 3. State Management

- **Reactive Arrays**: Svelte `$state` ensures automatic UI updates
- **Selection Clearing**: Automatically clears selection if deleted chart was selected
- **Cache Invalidation**: Ensures fresh data on next fetch
- **Type Safety**: Uses `neverthrow` Result types for error handling

### 4. Error Handling

- **Service Layer**: Returns `Result<void, Error>` for type-safe errors
- **Component Layer**: Handles errors with user-friendly alerts
- **Realtime Layer**: Logs errors but continues processing other events
- **Database Layer**: RLS policies prevent unauthorized deletions

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
    is_active BOOLEAN DEFAULT true, -- Soft delete flag

    CONSTRAINT charts_project_name_unique UNIQUE (project_id, name)
);
```

### Indexes

```sql
CREATE INDEX idx_charts_project_id ON public.charts(project_id);
CREATE INDEX idx_charts_chart_type ON public.charts(chart_type);
CREATE INDEX idx_charts_is_active ON public.charts(is_active) WHERE is_active = true;
```

### RLS Policies

```sql
-- Users can delete charts in their projects
CREATE POLICY "Users can delete charts in their projects"
    ON public.charts
    FOR DELETE
    USING (
        -- Allow service role (Tauri backend)
        auth.uid() IS NULL
        OR
        -- Allow users with project access
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

## Realtime Configuration

### Supabase Realtime Publication

```sql
-- Charts table must be added to realtime publication
ALTER PUBLICATION supabase_realtime ADD TABLE public.charts;
```

### Realtime Subscription

```typescript
// Subscribe to all events (INSERT, UPDATE, DELETE)
this.channel = this.supabase
  .channel("charts-changes")
  .on(
    "postgres_changes",
    {
      event: "*",
      schema: "public",
      table: "charts",
    },
    async (payload: any) => {
      await this.handleChartsChange(payload);
    },
  )
  .subscribe();
```

## Comparison with Hard Delete

| Feature            | Hard Delete (DELETE) | Soft Delete (UPDATE) |
| ------------------ | -------------------- | -------------------- |
| **Database**       | Record removed       | Record remains       |
| **Recovery**       | Not possible         | Possible             |
| **Audit Trail**    | Lost                 | Preserved            |
| **Performance**    | Faster               | Slightly slower      |
| **Referential**    | CASCADE required     | Maintained           |
| **Realtime Event** | DELETE event         | UPDATE event         |

## Troubleshooting

### Common Issues

1. **Chart doesn't disappear from UI**
   - Check if Realtime subscription is connected
   - Verify `is_active` changed to `false` in database
   - Check browser console for Realtime errors
   - Ensure project filtering is correct

2. **Duplicate key constraint error**
   - Old chart wasn't removed from state before creating new one
   - Check if `handleChartUpdate` is detecting soft delete correctly
   - Verify `charts` array is being filtered properly

3. **Selection not cleared**
   - Check if `lastSelectedChartId` matches deleted chart ID
   - Verify `setSelectedChartId(null)` is being called
   - Check browser console for errors

4. **Realtime not working**
   - Verify charts table is in `supabase_realtime` publication
   - Check RLS policies allow SELECT for Realtime
   - Ensure WebSocket connection is established
   - Check Supabase dashboard for Realtime logs

## Key Third-Party Dependencies

1. **Supabase JavaScript Client** (`@supabase/supabase-js`)
   - **Purpose**: PostgreSQL client with authentication and RLS
   - **Used For**: Chart deletion, realtime subscriptions
   - **Why**: Official client, RLS integration, type safety

2. **Supabase Realtime** (via Supabase JavaScript Client)
   - **Purpose**: WebSocket-based realtime updates
   - **Used For**: Automatic UI synchronization
   - **Why**: Built-in, secure, respects RLS policies

3. **neverthrow** (`neverthrow`)
   - **Purpose**: Type-safe error handling
   - **Used For**: `Result<T, E>` return types
   - **Why**: Functional error handling, type safety

4. **PostgreSQL with RLS**
   - **Purpose**: Database backend with security policies
   - **Used For**: Data storage, soft delete pattern
   - **Why**: ACID compliance, RLS security, foreign key constraints

## Conclusion

This implementation provides a secure, reactive, and user-friendly way to delete charts from our application. By leveraging soft deletes, Supabase Realtime, and Svelte reactivity, we ensure:

- **Immediate UI Updates**: Charts disappear instantly from sidebar
- **Multi-Client Sync**: All connected clients see changes simultaneously
- **Data Preservation**: Soft delete maintains audit trail
- **Type Safety**: `neverthrow` Result types prevent runtime errors
- **Security**: RLS policies prevent unauthorized deletions

**Key Benefits**:

- ✅ **Reactive**: UI updates automatically via Svelte `$state`
- ✅ **Realtime**: Changes propagate via WebSocket
- ✅ **Secure**: RLS policies enforce access control
- ✅ **Recoverable**: Soft delete allows restoration
- ✅ **Performant**: Efficient filtering and cache invalidation
