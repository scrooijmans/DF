# Supabase Studio: Svelte Stores and Derived Stores for Multi-View Synchronization

## Executive Summary

Supabase Studio uses a sophisticated **store-based architecture** with Svelte stores and derived stores to synchronize multiple views (tables, editors, dashboards) without tight coupling or duplicated state. This architecture ensures that all views share a single source of truth while remaining decoupled and independently updateable.

**Key Design Principles:**

1. **Single Source of Truth** - Base stores hold canonical data
2. **Derived Stores for Views** - Each view derives its data from base stores
3. **Reactive Propagation** - Changes automatically flow through derived stores
4. **No Direct Coupling** - Views never directly reference each other
5. **Selective Updates** - Views only update when their derived data changes

---

## 1. Architecture Overview

### 1.1 Store Hierarchy

```
┌─────────────────────────────────────────────────────────────┐
│                    BASE STORES (Source of Truth)             │
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  TableStore  │  │ SchemaStore   │  │ QueryStore   │      │
│  │              │  │               │  │              │      │
│  │ - tables    │  │ - schemas     │  │ - queries    │      │
│  │ - rows      │  │ - columns     │  │ - results    │      │
│  │ - filters   │  │ - types       │  │ - loading    │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │                  │                 │              │
│         └──────────────────┼─────────────────┘              │
│                            │                                  │
│                            ▼                                  │
│              ┌─────────────────────────┐                     │
│              │   DERIVED STORES         │                     │
│              │   (View-Specific Data)   │                     │
│              └─────────────────────────┘                     │
│                            │                                  │
│         ┌──────────────────┼──────────────────┐            │
│         │                   │                   │            │
│         ▼                   ▼                   ▼            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ TableView    │  │ SQL Editor    │  │ Dashboard    │      │
│  │ Store        │  │ Store         │  │ Store        │      │
│  │              │  │               │  │              │      │
│  │ derived from │  │ derived from  │  │ derived from │      │
│  │ TableStore   │  │ QueryStore +  │  │ multiple     │      │
│  │              │  │ SchemaStore   │  │ stores      │      │
│  └──────────────┘  └───────────────┘  └──────────────┘      │
│         │                   │                   │            │
│         └───────────────────┼───────────────────┘            │
│                             │                                 │
│                             ▼                                 │
│                    ┌──────────────┐                          │
│                    │   VIEWS      │                          │
│                    │  (Components) │                          │
│                    └──────────────┘                          │
└─────────────────────────────────────────────────────────────┘
```

### 1.2 Data Flow Pattern

```
1. User Action (e.g., edit row in Table Editor)
   │
   ▼
2. TableStore.updateRow(rowId, changes)
   │
   ▼
3. TableStore emits change event
   │
   ▼
4. All derived stores automatically recompute
   │
   ├─► TableViewStore (filters/transforms data)
   ├─► SQL Editor Store (updates query results if querying same table)
   └─► Dashboard Store (updates charts/metrics if displaying same data)
   │
   ▼
5. Views reactively update via Svelte's reactivity system
```

---

## 2. Base Store Implementation

### 2.1 Table Store (Source of Truth)

**Inspired by Svelte 5 runes and store patterns:**

```typescript
// stores/table-store.ts
import { writable, derived, get } from 'svelte/store';
import type { Table, Row, TableChange } from '$lib/types';

/**
 * Base table store - single source of truth for all table data
 * Uses Svelte 5 runes for reactivity
 */
class TableStoreImpl {
	// Core state using Svelte 5 $state rune
	private tables = $state<Map<string, Table>>(new Map());
	private rows = $state<Map<string, Map<string, Row>>>(new Map()); // tableId -> rowId -> Row
	private filters = $state<Map<string, any>>(new Map()); // tableId -> filter config

	// Change notifications for derived stores
	private changeNotifier = writable<TableChange | null>(null);

	/**
	 * Get table data (reactive)
	 */
	getTable(tableId: string): Table | undefined {
		return this.tables.get(tableId);
	}

	/**
	 * Get rows for a table (reactive)
	 */
	getRows(tableId: string): Row[] {
		const tableRows = this.rows.get(tableId);
		return tableRows ? Array.from(tableRows.values()) : [];
	}

	/**
	 * Update a row
	 */
	updateRow(tableId: string, rowId: string, changes: Partial<Row>): void {
		const tableRows = this.rows.get(tableId);
		if (!tableRows) return;

		const row = tableRows.get(rowId);
		if (!row) return;

		// Update the row
		const updatedRow = { ...row, ...changes, updated_at: new Date() };
		tableRows.set(rowId, updatedRow);

		// Notify all derived stores
		this.changeNotifier.set({
			type: 'update',
			tableId,
			rowId,
			changes,
			timestamp: Date.now()
		});
	}

	/**
	 * Add a new row
	 */
	addRow(tableId: string, row: Row): void {
		let tableRows = this.rows.get(tableId);
		if (!tableRows) {
			tableRows = new Map();
			this.rows.set(tableId, tableRows);
		}

		tableRows.set(row.id, row);

		this.changeNotifier.set({
			type: 'insert',
			tableId,
			rowId: row.id,
			changes: row,
			timestamp: Date.now()
		});
	}

	/**
	 * Delete a row
	 */
	deleteRow(tableId: string, rowId: string): void {
		const tableRows = this.rows.get(tableId);
		if (!tableRows) return;

		tableRows.delete(rowId);

		this.changeNotifier.set({
			type: 'delete',
			tableId,
			rowId,
			timestamp: Date.now()
		});
	}

	/**
	 * Set filter for a table
	 */
	setFilter(tableId: string, filter: any): void {
		this.filters.set(tableId, filter);

		this.changeNotifier.set({
			type: 'filter',
			tableId,
			timestamp: Date.now()
		});
	}

	/**
	 * Get change notifier (for derived stores to subscribe)
	 */
	get changeNotifier() {
		return this.changeNotifier;
	}
}

// Singleton instance
export const tableStore = new TableStoreImpl();
```

### 2.2 Schema Store

```typescript
// stores/schema-store.ts
import { writable } from 'svelte/store';

class SchemaStoreImpl {
	private schemas = $state<Map<string, Schema>>(new Map());
	private columns = $state<Map<string, Column[]>>(new Map()); // tableId -> columns
	private schemaNotifier = writable<SchemaChange | null>(null);

	getSchema(schemaName: string): Schema | undefined {
		return this.schemas.get(schemaName);
	}

	getColumns(tableId: string): Column[] {
		return this.columns.get(tableId) || [];
	}

	updateColumn(tableId: string, columnId: string, changes: Partial<Column>): void {
		const cols = this.columns.get(tableId);
		if (!cols) return;

		const index = cols.findIndex((c) => c.id === columnId);
		if (index === -1) return;

		cols[index] = { ...cols[index], ...changes };
		this.columns.set(tableId, [...cols]); // Trigger reactivity

		this.schemaNotifier.set({
			type: 'column_update',
			tableId,
			columnId,
			timestamp: Date.now()
		});
	}

	get changeNotifier() {
		return this.schemaNotifier;
	}
}

export const schemaStore = new SchemaStoreImpl();
```

---

## 3. Derived Stores for Views

### 3.1 Table View Store (Derived from Table Store)

**Key Pattern: Derived stores transform base store data for specific view needs**

```typescript
// stores/table-view-store.ts
import { derived, writable } from 'svelte/store';
import { tableStore } from './table-store';

/**
 * Derived store for table view
 * Automatically updates when base tableStore changes
 */
export function createTableViewStore(tableId: string) {
	// View-specific state (pagination, sorting)
	const page = writable(1);
	const pageSize = writable(50);
	const sortColumn = writable<string | null>(null);
	const sortDirection = writable<'asc' | 'desc'>('asc');

	/**
	 * Derived: Filtered and sorted rows for this view
	 * Automatically recomputes when:
	 * - tableStore rows change
	 * - filters change
	 * - sort parameters change
	 */
	const filteredRows = derived(
		[
			// Subscribe to base store changes
			tableStore.changeNotifier,
			// Subscribe to view-specific state
			page,
			pageSize,
			sortColumn,
			sortDirection
		],
		([change, $page, $pageSize, $sortCol, $sortDir]) => {
			// Get base data
			let rows = tableStore.getRows(tableId);
			const filter = tableStore.getFilter(tableId);

			// Apply filter
			if (filter) {
				rows = applyFilter(rows, filter);
			}

			// Apply sorting
			if ($sortCol) {
				rows = sortRows(rows, $sortCol, $sortDir);
			}

			// Apply pagination
			const start = ($page - 1) * $pageSize;
			const end = start + $pageSize;
			const paginatedRows = rows.slice(start, end);

			return {
				rows: paginatedRows,
				total: rows.length,
				page: $page,
				pageSize: $pageSize,
				hasMore: end < rows.length
			};
		}
	);

	/**
	 * Derived: Row count (for display in UI)
	 */
	const rowCount = derived(filteredRows, ($filteredRows) => $filteredRows.total);

	/**
	 * Derived: Loading state
	 */
	const isLoading = derived(
		tableStore.changeNotifier,
		() => false // Could check for pending operations
	);

	return {
		// State
		page,
		pageSize,
		sortColumn,
		sortDirection,

		// Derived stores
		filteredRows,
		rowCount,
		isLoading,

		// Actions
		setPage: (p: number) => page.set(p),
		setPageSize: (size: number) => pageSize.set(size),
		setSort: (col: string, dir: 'asc' | 'desc') => {
			sortColumn.set(col);
			sortDirection.set(dir);
		}
	};
}
```

### 3.2 SQL Editor Store (Derived from Multiple Stores)

**Key Pattern: Derived stores can combine data from multiple base stores**

```typescript
// stores/sql-editor-store.ts
import { derived, writable } from 'svelte/store';
import { tableStore } from './table-store';
import { schemaStore } from './schema-store';

export function createSQLEditorStore() {
	const query = writable<string>('');
	const queryResults = writable<any[]>([]);
	const isExecuting = writable(false);

	/**
	 * Derived: Available tables for autocomplete
	 * Updates when schema changes
	 */
	const availableTables = derived(schemaStore.changeNotifier, () => {
		// Get all tables from schema store
		return Array.from(schemaStore.getAllTables());
	});

	/**
	 * Derived: Query suggestions based on current schema
	 */
	const querySuggestions = derived(
		[query, availableTables, schemaStore.changeNotifier],
		([$query, $tables]) => {
			// Generate autocomplete suggestions based on:
			// - Current query text
			// - Available tables and columns
			return generateSuggestions($query, $tables);
		}
	);

	/**
	 * Derived: Query results with reactive updates
	 * If query references a table that changes, results update
	 */
	const reactiveResults = derived(
		[queryResults, tableStore.changeNotifier, schemaStore.changeNotifier],
		([$results, $tableChange, $schemaChange]) => {
			// Check if the change affects our query results
			if ($tableChange && affectsQuery($tableChange, $query)) {
				// Re-execute query or refresh results
				return refreshQueryResults($query);
			}
			return $results;
		}
	);

	return {
		query,
		queryResults: reactiveResults,
		isExecuting,
		availableTables,
		querySuggestions,

		// Actions
		setQuery: (q: string) => query.set(q),
		executeQuery: async () => {
			isExecuting.set(true);
			try {
				const results = await executeSQL(get(query));
				queryResults.set(results);
			} finally {
				isExecuting.set(false);
			}
		}
	};
}
```

### 3.3 Dashboard Store (Derived from Multiple Sources)

```typescript
// stores/dashboard-store.ts
import { derived } from 'svelte/store';
import { tableStore } from './table-store';
import { queryStore } from './query-store';

export function createDashboardStore(dashboardId: string) {
	const config = writable<DashboardConfig>(loadDashboardConfig(dashboardId));

	/**
	 * Derived: Chart data from multiple table stores
	 * Each chart widget derives from different tables
	 */
	const chartData = derived(
		[config, tableStore.changeNotifier, queryStore.changeNotifier],
		([$config, $tableChange, $queryChange]) => {
			const charts = $config.widgets
				.filter((w) => w.type === 'chart')
				.map((widget) => {
					// Each chart derives its data from the appropriate store
					if (widget.sourceType === 'table') {
						const rows = tableStore.getRows(widget.tableId);
						return {
							widgetId: widget.id,
							data: transformToChartData(rows, widget.config)
						};
					} else if (widget.sourceType === 'query') {
						const results = queryStore.getResults(widget.queryId);
						return {
							widgetId: widget.id,
							data: transformToChartData(results, widget.config)
						};
					}
				});

			return charts;
		}
	);

	/**
	 * Derived: Metrics (aggregated data)
	 */
	const metrics = derived([config, tableStore.changeNotifier], ([$config]) => {
		return $config.metricWidgets.map((metric) => {
			const rows = tableStore.getRows(metric.tableId);
			return {
				metricId: metric.id,
				value: calculateMetric(rows, metric.aggregation)
			};
		});
	});

	return {
		config,
		chartData,
		metrics
	};
}
```

---

## 4. View Components (Consuming Derived Stores)

### 4.1 Table View Component

**Key Pattern: Views subscribe to derived stores, never base stores directly**

```svelte
<!-- components/TableEditor.svelte -->
<script lang="ts">
  import { createTableViewStore } from '$lib/stores/table-view-store';
  import { tableStore } from '$lib/stores/table-store';

  interface Props {
    tableId: string;
  }

  let { tableId }: Props = $props();

  // Create view-specific derived store
  const viewStore = createTableViewStore(tableId);

  // Reactive values from derived store
  let filteredRows = $state<any[]>([]);
  let rowCount = $state(0);

  // Subscribe to derived store (automatic cleanup)
  $effect(() => {
    const unsubscribe = viewStore.filteredRows.subscribe(value => {
      filteredRows = value.rows;
      rowCount = value.total;
    });

    return unsubscribe;
  });

  // Handle row edit
  function handleRowEdit(rowId: string, field: string, value: any) {
    // Update base store (not derived store!)
    tableStore.updateRow(tableId, rowId, { [field]: value });
    // Derived store automatically updates, triggering view refresh
  }
</script>

<div class="table-editor">
  <div class="table-header">
    <h2>Table: {tableId}</h2>
    <span>Total rows: {rowCount}</span>
  </div>

  <table>
    <thead>
      <tr>
        <!-- Column headers -->
      </tr>
    </thead>
    <tbody>
      {#each filteredRows as row (row.id)}
        <tr>
          {#each Object.entries(row) as [key, value]}
            <td contenteditable
                onblur={(e) => handleRowEdit(row.id, key, e.target.textContent)}>
              {value}
            </td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>

  <!-- Pagination controls -->
  <div class="pagination">
    <button onclick={() => viewStore.setPage($viewStore.page - 1)}>
      Previous
    </button>
    <span>Page {viewStore.page}</span>
    <button onclick={() => viewStore.setPage($viewStore.page + 1)}>
      Next
    </button>
  </div>
</div>
```

### 4.2 SQL Editor Component

```svelte
<!-- components/SQLEditor.svelte -->
<script lang="ts">
  import { createSQLEditorStore } from '$lib/stores/sql-editor-store';

  const editorStore = createSQLEditorStore();

  // Reactive query results
  let results = $state<any[]>([]);

  $effect(() => {
    const unsubscribe = editorStore.queryResults.subscribe(value => {
      results = value;
    });
    return unsubscribe;
  });

  // Autocomplete suggestions
  let suggestions = $state<string[]>([]);

  $effect(() => {
    const unsubscribe = editorStore.querySuggestions.subscribe(value => {
      suggestions = value;
    });
    return unsubscribe;
  });
</script>

<div class="sql-editor">
  <textarea
    bind:value={$editorStore.query}
    onkeydown={handleAutocomplete}
  />

  <button onclick={() => editorStore.executeQuery()}>
    Execute
  </button>

  {#if $editorStore.isExecuting}
    <div>Executing query...</div>
  {:else}
    <table>
      {#each results as row}
        <tr>
          {#each Object.values(row) as cell}
            <td>{cell}</td>
          {/each}
        </tr>
      {/each}
    </table>
  {/if}
</div>
```

### 4.3 Dashboard Component

```svelte
<!-- components/Dashboard.svelte -->
<script lang="ts">
  import { createDashboardStore } from '$lib/stores/dashboard-store';

  interface Props {
    dashboardId: string;
  }

  let { dashboardId }: Props = $props();

  const dashboardStore = createDashboardStore(dashboardId);

  // Reactive chart data
  let chartData = $state<any[]>([]);
  let metrics = $state<any[]>([]);

  $effect(() => {
    const unsubscribe1 = dashboardStore.chartData.subscribe(value => {
      chartData = value;
    });

    const unsubscribe2 = dashboardStore.metrics.subscribe(value => {
      metrics = value;
    });

    return () => {
      unsubscribe1();
      unsubscribe2();
    };
  });
</script>

<div class="dashboard">
  <!-- Metrics -->
  <div class="metrics">
    {#each metrics as metric}
      <div class="metric-card">
        <h3>{metric.metricId}</h3>
        <p>{metric.value}</p>
      </div>
    {/each}
  </div>

  <!-- Charts -->
  <div class="charts">
    {#each chartData as chart}
      <ChartComponent data={chart.data} />
    {/each}
  </div>
</div>
```

---

## 5. Synchronization Without Tight Coupling

### 5.1 How Views Stay Synchronized

**Example Scenario: User edits a row in Table Editor**

```
1. User edits row in TableEditor component
   │
   ▼
2. TableEditor calls tableStore.updateRow(tableId, rowId, changes)
   │
   ▼
3. tableStore updates internal state and emits change notification
   │
   ├─► TableViewStore.filteredRows (derived) automatically recomputes
   │   └─► TableEditor view updates (shows edited value)
   │
   ├─► SQLEditorStore.reactiveResults (derived) checks if change affects query
   │   └─► If query references same table → results refresh
   │   └─► SQLEditor view updates (shows updated data)
   │
   └─► DashboardStore.chartData (derived) checks if any charts use this table
       └─► If chart uses this table → chart data recomputes
       └─► Dashboard view updates (chart redraws)
```

**Key Points:**

- **No direct communication** between views
- **All synchronization** happens through base stores
- **Derived stores** automatically recompute when dependencies change
- **Views** reactively update via Svelte's reactivity system

### 5.2 Avoiding Duplicated State

**Problem: Without derived stores, each view would need its own copy of data**

```typescript
// ❌ BAD: Each view maintains its own state
class TableEditor {
	private rows = []; // Duplicate!
}

class SQLEditor {
	private queryResults = []; // Duplicate!
}

class Dashboard {
	private chartData = []; // Duplicate!
}
```

**Solution: Derived stores compute view-specific data on-demand**

```typescript
// ✅ GOOD: Views derive from single source
const tableStore = new TableStore(); // Single source

const tableViewStore = derived(tableStore, ...); // Computed
const sqlEditorStore = derived(tableStore, ...); // Computed
const dashboardStore = derived(tableStore, ...); // Computed
```

### 5.3 Selective Updates

**Derived stores only recompute when their dependencies change:**

```typescript
// This derived store only recomputes when:
// 1. tableStore.changeNotifier emits (any table change)
// 2. AND the change affects tableId='users'
const userTableViewStore = derived(tableStore.changeNotifier, ($change) => {
	if ($change?.tableId === 'users') {
		return tableStore.getRows('users');
	}
	return getCurrentValue(); // Return cached value if not relevant
});
```

---

## 6. Advanced Patterns

### 6.1 Composed Derived Stores

**Derived stores can depend on other derived stores:**

```typescript
// Base store
const tableStore = new TableStore();

// First-level derived store
const filteredTableStore = derived(tableStore, ($store) => applyFilters($store.getRows()));

// Second-level derived store (depends on first-level)
const paginatedTableStore = derived(
	[filteredTableStore, page, pageSize],
	([$filtered, $page, $size]) => paginate($filtered, $page, $size)
);
```

### 6.2 Store Factories

**Create view-specific stores on demand:**

```typescript
// Store factory pattern
export function createViewStore(viewId: string, config: ViewConfig) {
	const baseStore = getBaseStore(config.dataSource);

	return derived(baseStore.changeNotifier, () => transformData(baseStore.getData(), config));
}

// Usage in component
const viewStore = createViewStore('view-1', {
	dataSource: 'table:users',
	filters: { status: 'active' },
	sort: { column: 'created_at', direction: 'desc' }
});
```

### 6.3 Store Composition with Svelte 5 Runes

**Using Svelte 5 runes for more direct reactivity:**

```typescript
// Using Svelte 5 $state and $derived runes
class TableStore {
	private tables = $state<Map<string, Table>>(new Map());

	// Derived using $derived rune
	getTableRows = $derived.by((tableId: string) => {
		const table = this.tables.get(tableId);
		return table ? Array.from(table.rows.values()) : [];
	});
}

// Usage in component (no subscription needed!)
const rows = tableStore.getTableRows('users'); // Automatically reactive
```

---

## 7. Benefits of This Architecture

### 7.1 Decoupling

- **Views don't know about each other** - They only know about base stores
- **Easy to add new views** - Just create a new derived store
- **Views can be removed** - No impact on other views

### 7.2 Performance

- **Selective updates** - Only affected derived stores recompute
- **Memoization** - Svelte automatically memoizes derived store computations
- **Lazy evaluation** - Derived stores only compute when subscribed to

### 7.3 Maintainability

- **Single source of truth** - All data in base stores
- **Clear data flow** - Base stores → Derived stores → Views
- **Easy to debug** - Can inspect store state at any level

### 7.4 Testability

- **Stores can be tested independently** - No need for full component setup
- **Derived stores are pure functions** - Easy to unit test
- **Mock base stores** - Test derived stores with mock data

---

## 8. Comparison with Other Patterns

### 8.1 vs. Event Bus Pattern

| Aspect          | Event Bus                   | Derived Stores                  |
| --------------- | --------------------------- | ------------------------------- |
| **Coupling**    | Views must know event names | Views only know stores          |
| **Type Safety** | String-based events         | Type-safe store subscriptions   |
| **Debugging**   | Hard to trace event flow    | Clear dependency graph          |
| **Performance** | All listeners notified      | Only dependent stores recompute |

### 8.2 vs. Redux/Flux Pattern

| Aspect             | Redux                        | Derived Stores          |
| ------------------ | ---------------------------- | ----------------------- |
| **Boilerplate**    | Actions, reducers, selectors | Just stores             |
| **Learning Curve** | Steeper                      | Simpler (native Svelte) |
| **Performance**    | Manual optimization needed   | Automatic memoization   |
| **DevTools**       | Redux DevTools               | Svelte DevTools         |

### 8.3 vs. Direct Props/State

| Aspect              | Direct State                 | Derived Stores         |
| ------------------- | ---------------------------- | ---------------------- |
| **Synchronization** | Manual prop drilling         | Automatic              |
| **Duplication**     | Each component has own state | Single source of truth |
| **Updates**         | Manual coordination          | Automatic propagation  |

---

## 9. Real-World Example: Table Edit Synchronization

### Scenario

User has three views open:

1. **Table Editor** - Editing row in `users` table
2. **SQL Editor** - Running query: `SELECT * FROM users WHERE status = 'active'`
3. **Dashboard** - Chart showing user count by status

### Step-by-Step Flow

```typescript
// 1. User edits row in Table Editor
tableStore.updateRow('users', 'user-123', { status: 'inactive' });

// 2. Base store updates and notifies
tableStore.changeNotifier.set({
	type: 'update',
	tableId: 'users',
	rowId: 'user-123',
	changes: { status: 'inactive' }
});

// 3. Derived stores automatically recompute:

// TableViewStore.filteredRows
// - Checks if change affects 'users' table → YES
// - Recomputes filtered rows
// - Table Editor view updates

// SQLEditorStore.reactiveResults
// - Checks if query references 'users' table → YES
// - Checks if row 'user-123' matches WHERE status = 'active' → NO (now inactive)
// - Recomputes results (removes this row from results)
// - SQL Editor view updates

// DashboardStore.chartData
// - Checks if any charts use 'users' table → YES
// - Recomputes user count by status
// - Chart data updates (inactive count +1, active count -1)
// - Dashboard view updates
```

**Result:** All three views update automatically, showing consistent data, without any direct communication between them.

---

## 10. Best Practices

### 10.1 Store Design

1. **Keep base stores focused** - One store per domain (tables, schemas, queries)
2. **Derived stores for views** - Each view gets its own derived store
3. **Avoid circular dependencies** - Derived stores should not depend on each other
4. **Use factories** - Create view stores on demand, don't pre-create all

### 10.2 Component Design

1. **Subscribe in $effect** - Always clean up subscriptions
2. **Use derived stores directly** - Don't cache derived store values unnecessarily
3. **Update base stores only** - Never update derived stores directly
4. **Handle loading states** - Derived stores can expose loading state

### 10.3 Performance

1. **Limit derived store dependencies** - Only subscribe to what you need
2. **Use $derived.by for expensive computations** - Memoize based on inputs
3. **Batch updates** - Group multiple base store updates together
4. **Lazy load** - Only create derived stores when views are mounted

---

## 11. Summary

Supabase Studio's store architecture provides:

✅ **Single Source of Truth** - Base stores hold canonical data  
✅ **Automatic Synchronization** - Derived stores automatically update  
✅ **No Tight Coupling** - Views never directly reference each other  
✅ **No Duplicated State** - Data computed on-demand from base stores  
✅ **Type Safety** - Full TypeScript support throughout  
✅ **Performance** - Selective updates and automatic memoization  
✅ **Maintainability** - Clear data flow and easy to extend

This architecture enables Supabase Studio to maintain multiple synchronized views (tables, editors, dashboards) while keeping the codebase maintainable and performant.

---

## 12. References

- [Svelte Stores Documentation](https://svelte.dev/docs/svelte-store)
- [Svelte 5 Runes](https://svelte.dev/docs/svelte-5-preview)
- [Svelte Derived Stores](https://svelte.dev/docs/svelte-store#derived)
- [Mat Simon: Svelte in Depth - createSubscriber()](https://matsimon.dev/blog/svelte-in-depth-create-subscriber) - Advanced reactivity patterns
- [DataForge Multi-View Synchronization](./DFC-model-view-sync.md) - Similar patterns in DataForge
