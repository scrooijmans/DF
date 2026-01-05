# AG Grid Multi-Row Select Database Sync

## Overview

This document describes the end-to-end process of how the `ag-data-table-multi-row-select.svelte` component keeps its selection state synchronized with the database, ensuring the database is the single source of truth for selection state.

## Architecture

The component uses a **database-first** approach where:

1. **Database is the single source of truth** - Selection state is stored in the database (e.g., `charts.data_source_config`)
2. **AG Grid syncs with database state** - AG Grid's visual selection reflects the database state
3. **User clicks update database first** - When a user clicks a row, the database is updated, then AG Grid syncs

## Complete Data Flow

### Step 1: Component Initialization

**Location**: `ag-data-table-multi-row-select.svelte`

**Process**:

```typescript
// Component receives selectedRowIds prop (bound from parent)
let selectedRowIds = $bindable([]);

// AG Grid is configured with disabled automatic selection
const gridOptions = $state<GridOptions>({
  rowSelection: {
    mode: "multiRow",
    checkboxes: false,
    enableClickSelection: false, // ✅ Disabled - we handle manually
  },
  suppressRowClickSelection: true, // ✅ Suppressed - we handle manually
  onGridReady: (params) => {
    gridApi = params.api;
    // Initial sync: Set AG Grid selection to match selectedRowIds
    if (selectedRowIds.length > 0) {
      params.api.forEachNode((node: any) => {
        const rowId = getRowId({ data: node.data });
        if (selectedRowIds.includes(rowId)) {
          node.setSelected(true);
        }
      });
    }
  },
});
```

**What Happens**:

1. Component receives `selectedRowIds` prop (initially empty or populated from database)
2. AG Grid is configured with `enableClickSelection: false` to prevent automatic selection
3. On `onGridReady`, AG Grid's selection is synced to match `selectedRowIds`

---

### Step 2: User Clicks a Row

**Location**: `ag-data-table-multi-row-select.svelte::onRowClicked`

**Process**:

```typescript
onRowClicked: (params: any) => {
  const rowId = getRowId({ data: params.data });
  const isSelected = params.node.isSelected();

  // Set flag to prevent effect from syncing during this update
  isUpdatingFromClick = true;

  // Toggle selection in AG Grid
  if (isSelected) {
    params.node.setSelected(false, false);
    const newSelectedIds = selectedRowIds.filter((id) => id !== rowId);
    selectedRowIds = newSelectedIds; // ✅ Update bound prop
    onSelectionChange?.(newSelectedIds); // ✅ Notify parent
  } else {
    params.node.setSelected(true, false);
    if (!selectedRowIds.includes(rowId)) {
      const newSelectedIds = [...selectedRowIds, rowId];
      selectedRowIds = newSelectedIds; // ✅ Update bound prop
      onSelectionChange?.(newSelectedIds); // ✅ Notify parent
    }
  }

  // Reset flag after state settles
  setTimeout(() => {
    isUpdatingFromClick = false;
  }, 0);
};
```

**What Happens**:

1. User clicks a row in AG Grid
2. `onRowClicked` handler fires (AG Grid's automatic selection is disabled)
3. Component toggles AG Grid's visual selection manually
4. Component updates `selectedRowIds` prop (bound to parent component)
5. Component calls `onSelectionChange` callback to notify parent
6. `isUpdatingFromClick` flag is set to prevent `$effect` from syncing during this update

---

### Step 3: Parent Component Updates Database

**Location**: `chart-data-source-curve-selector.svelte` (or other selector components)

**Process**:

```typescript
// Parent component receives selection change
<AgDataTableMultiRowSelect
  bind:selectedRowIds={selectedCurveIds}
  onSelectionChange={(ids) => {
    selectedCurveIds = ids; // Update local state
  }}
/>

// When user clicks "Add" or dialog closes
export async function handleAdd() {
  // Compare current selection with existing database state
  const existingIds = new Set(existingCurveIds);
  const selectedIds = new Set(selectedCurveIds);

  const toAdd = selectedCurveIds.filter((id) => !existingIds.has(id));
  const toRemove = existingCurveIds.filter((id) => !selectedIds.has(id));

  // Update database
  if (toAdd.length > 0) {
    await addDataSourcesToChart(chart.id, newSources);
  }
  if (toRemove.length > 0) {
    await removeDataSourcesFromChart(chart.id, sourceIdsToRemove);
  }
}
```

**What Happens**:

1. Parent component receives `onSelectionChange` callback with new selection
2. Parent updates local `selectedCurveIds` state
3. When user confirms (clicks "Add" or closes dialog), `handleAdd()` is called
4. Parent compares current selection with existing database state
5. Parent calls `addDataSourcesToChart()` or `removeDataSourcesFromChart()` to update database

---

### Step 4: Database Update Triggers Realtime

**Location**: `chart-data-source-service.ts`

**Process**:

```typescript
export async function addDataSourcesToChart(
  chartId: string,
  newSources: ChartDataSourceEntry[],
): Promise<Result<void, Error>> {
  // Fetch current chart
  const { data: chart } = await supabase
    .from("charts")
    .select("data_source_config")
    .eq("id", chartId)
    .single();

  // Merge new sources with existing ones
  const currentConfig = Array.isArray(chart.data_source_config)
    ? (chart.data_source_config as ChartDataSourceEntry[])
    : [];
  const updatedConfig = [...currentConfig, ...newSources];

  // Update database
  const { error } = await supabase
    .from("charts")
    .update({
      data_source_config: updatedConfig,
      updated_at: new Date().toISOString(),
    })
    .eq("id", chartId);

  // ✅ Database update triggers PostgreSQL WAL
  // ✅ Supabase Realtime detects change
  // ✅ WebSocket event is sent to frontend
}
```

**What Happens**:

1. Service updates `charts.data_source_config` JSONB column
2. PostgreSQL WAL (Write-Ahead Log) records the UPDATE
3. Supabase Realtime service detects the change via logical replication
4. Realtime service verifies RLS policies
5. Realtime service sends WebSocket event to frontend

---

### Step 5: Frontend Receives Realtime Update

**Location**: `realtime-charts-service.ts::handleChartUpdate()`

**Process**:

```typescript
private async handleChartUpdate(newChart: any, oldChart) {
  // Convert database record to Chart type
  const chart: Chart = {
    id: newChart.id,
    data_source_config: newChart.data_source_config ?? {},
    // ... other fields
  };

  // Update global state
  this.chartsState.charts = this.chartsState.charts.map((c: Chart) =>
    c.id === chart.id ? chart : c,
  );

  // ✅ This triggers reactive updates in all components using chartsState
}
```

**What Happens**:

1. Realtime service receives UPDATE event from WebSocket
2. Service converts database record to `Chart` type
3. Service updates `PostgresChartsState.charts` array
4. Svelte reactivity triggers updates in all components using `chartsState`

---

### Step 6: Selector Component Reacts to Database Update

**Location**: `chart-data-source-curve-selector.svelte`

**Process**:

```typescript
// Extract existing curve IDs from chart.data_source_config (reactive)
const existingCurveIds = $derived.by(() => {
  if (!chart.data_source_config || !Array.isArray(chart.data_source_config)) {
    return [];
  }

  return chart.data_source_config
    .filter(
      (entry: any) => entry.source?.type === "curve" && entry.source?.curveId,
    )
    .map((entry: any) => String(entry.source.curveId));
});

// Initialize selectedCurveIds when chart changes
$effect(() => {
  const existing = existingCurveIds;
  const currentChartId = chart.id;

  if (currentChartId !== lastChartId) {
    selectedCurveIds = [...existing]; // ✅ Sync with database
    lastChartId = currentChartId;
  }
});
```

**What Happens**:

1. `chart.data_source_config` is updated via realtime (from Step 5)
2. `existingCurveIds` `$derived` recomputes automatically
3. `$effect` detects change and updates `selectedCurveIds` to match database
4. `selectedCurveIds` prop is updated (bound to AG Grid component)

---

### Step 7: AG Grid Syncs with Database State

**Location**: `ag-data-table-multi-row-select.svelte::$effect`

**Process**:

```typescript
$effect(() => {
  const ids = selectedRowIds; // From database via parent component
  const existing = existingRowIds; // For highlighting

  // Create stable string representation for comparison
  const idsString = ids.slice().sort().join(",");

  // Skip if selection hasn't actually changed
  if (idsString === lastSyncedSelectedIds && gridApi) {
    return;
  }

  if (gridApi && !isUpdatingFromClick) {
    // Batch all updates together
    let nodesUpdated = 0;
    let needsRefresh = false;

    // Update row class rules for highlighting existing rows
    gridOptions.rowClassRules = {
      "ag-row-selected-curve": (params: any) => {
        if (!existing || existing.length === 0) return false;
        const rowId = getRowId({ data: params.data });
        return existing.includes(rowId);
      },
    };
    needsRefresh = true;

    // Sync AG Grid selection to match database state
    gridApi.forEachNode((node: any) => {
      const rowId = getRowId({ data: node.data });
      const shouldBeSelected = ids.includes(rowId);
      const isSelected = node.isSelected();

      if (shouldBeSelected !== isSelected) {
        node.setSelected(shouldBeSelected, false);
        nodesUpdated++;
        needsRefresh = true;
      }
    });

    // Only refresh if something actually changed
    if (needsRefresh) {
      requestAnimationFrame(() => {
        if (gridApi) {
          gridApi.refreshCells({ force: true });
        }
      });
    }

    // Update last synced state
    lastSyncedSelectedIds = idsString;
  }
});
```

**What Happens**:

1. `$effect` runs when `selectedRowIds` changes (from database update)
2. Effect checks if selection actually changed (compares string representations)
3. If `isUpdatingFromClick` is false (not a user click), sync AG Grid selection
4. Effect updates AG Grid's visual selection to match `selectedRowIds`
5. Effect refreshes cells to update row highlighting
6. Effect updates `lastSyncedSelectedIds` to prevent unnecessary refreshes

---

## Key Design Decisions

### 1. Database as Single Source of Truth

**Why**: Ensures consistency across multiple clients and prevents conflicts.

**Implementation**:

- Selection state is stored in `charts.data_source_config` JSONB column
- AG Grid's selection is derived from database state, not the other way around
- User clicks update database first, then AG Grid syncs

### 2. Disabled AG Grid Automatic Selection

**Why**: Prevents conflicts between AG Grid's internal selection state and database state.

**Implementation**:

```typescript
rowSelection: {
  enableClickSelection: false, // ✅ Disabled
},
suppressRowClickSelection: true, // ✅ Suppressed
```

**Benefits**:

- Full control over when selection changes
- No conflicts with database-driven selection
- Predictable behavior

### 3. Manual Selection Handling

**Why**: Allows us to update database before AG Grid syncs.

**Implementation**:

```typescript
onRowClicked: (params: any) => {
  // Set flag to prevent effect from syncing
  isUpdatingFromClick = true;

  // Update database state (via prop binding)
  selectedRowIds = newSelectedIds;
  onSelectionChange?.(newSelectedIds);

  // Reset flag after state settles
  setTimeout(() => {
    isUpdatingFromClick = false;
  }, 0);
};
```

**Benefits**:

- Database is updated immediately on click
- Effect doesn't interfere with user-initiated changes
- Clear separation between user actions and reactive sync

### 4. Batched Sync Updates

**Why**: Prevents multiple refreshes and improves performance.

**Implementation**:

```typescript
// Track last synced state
let lastSyncedSelectedIds = $state<string>("");

$effect(() => {
  const idsString = ids.slice().sort().join(",");

  // Skip if selection hasn't actually changed
  if (idsString === lastSyncedSelectedIds && gridApi) {
    return;
  }

  // Batch all updates
  let needsRefresh = false;
  // ... update row class rules ...
  // ... sync selection ...

  // Only refresh if something changed
  if (needsRefresh) {
    requestAnimationFrame(() => {
      gridApi.refreshCells({ force: true });
    });
  }

  lastSyncedSelectedIds = idsString;
});
```

**Benefits**:

- Prevents unnecessary cell refreshes
- Improves performance
- Reduces visual flicker

### 5. Realtime Synchronization

**Why**: Ensures all clients see the same selection state.

**Implementation**:

- Supabase Realtime subscriptions detect database changes
- Frontend receives WebSocket events
- Components reactively update when `chart.data_source_config` changes

**Benefits**:

- Multi-user collaboration support
- Consistent state across clients
- No manual refresh needed

---

## Complete Call Stack

```
1. User clicks row in AG Grid
   ↓
2. onRowClicked fires
   ├─ Sets isUpdatingFromClick = true
   ├─ Updates selectedRowIds prop (bound to parent)
   └─ Calls onSelectionChange callback
   ↓
3. Parent component receives callback
   ├─ Updates local selectedCurveIds state
   └─ (User confirms selection)
   ↓
4. Parent calls handleAdd()
   ├─ Compares selectedCurveIds with existingCurveIds
   ├─ Calls addDataSourcesToChart() or removeDataSourcesFromChart()
   └─ Updates charts.data_source_config in database
   ↓
5. Database UPDATE triggers PostgreSQL WAL
   ↓
6. Supabase Realtime detects change
   ├─ Verifies RLS policies
   └─ Sends WebSocket event to frontend
   ↓
7. RealtimeChartsService receives event
   ├─ Converts to Chart type
   └─ Updates PostgresChartsState.charts array
   ↓
8. Selector component reacts (via $derived)
   ├─ existingCurveIds recomputes from chart.data_source_config
   └─ $effect updates selectedCurveIds to match database
   ↓
9. AG Grid component reacts (via $effect)
   ├─ selectedRowIds prop changes (from parent)
   ├─ Effect detects change (if not isUpdatingFromClick)
   ├─ Syncs AG Grid selection to match selectedRowIds
   └─ Refreshes cells to update highlighting
   ↓
10. Visual state matches database state ✅
```

---

## Preventing Conflicts

### Conflict Prevention Mechanisms

1. **`isUpdatingFromClick` Flag**:
   - Set to `true` during user click handling
   - Prevents `$effect` from syncing during user-initiated updates
   - Reset after state settles

2. **Selection Change Detection**:
   - Compares string representations of selection arrays
   - Only syncs if selection actually changed
   - Prevents unnecessary refreshes

3. **Batched Updates**:
   - Collects all changes before refreshing
   - Uses `requestAnimationFrame` to batch DOM updates
   - Reduces visual flicker

4. **Disabled Automatic Selection**:
   - AG Grid's `enableClickSelection: false`
   - `suppressRowClickSelection: true`
   - Full manual control over selection

---

## Benefits

### ✅ **Consistency**

- Database is always the source of truth
- All clients see the same selection state
- No conflicts between AG Grid and database

### ✅ **Performance**

- Batched updates prevent multiple refreshes
- Only syncs when selection actually changes
- Efficient reactive updates

### ✅ **Reliability**

- Realtime synchronization ensures consistency
- No manual refresh needed
- Works across multiple clients

### ✅ **User Experience**

- Immediate visual feedback on click
- Smooth updates without flicker
- Predictable behavior

---

## Troubleshooting

### Issue: Selection Not Syncing

**Symptoms**: AG Grid selection doesn't match database state

**Debugging**:

1. Check `isUpdatingFromClick` flag - should be `false` when syncing
2. Check `lastSyncedSelectedIds` - should match current selection string
3. Check `selectedRowIds` prop - should match database state
4. Check `$effect` dependencies - should track `selectedRowIds`

**Solution**: Ensure `isUpdatingFromClick` is reset after user clicks

### Issue: Infinite Sync Loop

**Symptoms**: `$effect` runs repeatedly without stopping

**Debugging**:

1. Check `lastSyncedSelectedIds` comparison logic
2. Check if `selectedRowIds` is being mutated (should be reassigned)
3. Check if `onSelectionChange` is causing reactive updates

**Solution**: Use string comparison to detect actual changes, not reference equality

### Issue: Selection Lost on Tab Switch

**Symptoms**: Selection disappears when switching tabs in dialog

**Debugging**:

1. Check if `selectedRowIds` is being reset in parent component
2. Check if `existingCurveIds` is correctly derived from `chart.data_source_config`
3. Check if `$effect` initialization logic preserves selections

**Solution**: Only sync when chart changes, not when switching tabs

---

## Summary

The AG Grid multi-row select component maintains synchronization with the database through a carefully orchestrated flow:

1. **User clicks** → Update database state (via props)
2. **Database updates** → Trigger realtime events
3. **Realtime events** → Update global state
4. **Global state updates** → Trigger reactive updates
5. **Reactive updates** → Sync AG Grid selection

The database is always the single source of truth, and AG Grid's visual selection reflects the database state. This ensures consistency, reliability, and a smooth user experience.
