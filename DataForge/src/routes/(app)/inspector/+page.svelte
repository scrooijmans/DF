<script lang="ts">
	import { invoke } from '@tauri-apps/api/core'
	import DataGrid from '$lib/components/data/DataGrid.svelte'
	import type { ColDef, CellValueChangedEvent, SelectionChangedEvent } from 'ag-grid-community'

	// Types matching Rust backend
	interface TableInfo {
		name: string
		row_count: number
		is_editable: boolean
	}

	interface ForeignKeyInfo {
		target_table: string
		target_column: string
	}

	interface ColumnInfo {
		name: string
		data_type: string
		nullable: boolean
		primary_key: boolean
		foreign_key: ForeignKeyInfo | null
	}

	interface ForeignKeyLookupValue {
		id: unknown
		display: string
	}

	interface QueryResult {
		columns: ColumnInfo[]
		rows: (string | number | null)[][]
		total_rows: number
	}

	interface OrphanCleanupResult {
		orphan_count: number
		deleted_count: number
		bytes_freed: number
		errors: string[]
	}

	// State
	let tables = $state<TableInfo[]>([])
	let selectedTable = $state<string | null>(null)
	let isTableEditable = $state(true)
	let columns = $state<ColumnInfo[]>([])
	let queryResult = $state<QueryResult | null>(null)
	let isLoadingTables = $state(true)
	let isLoadingData = $state(false)
	let isSaving = $state(false)
	let error = $state<string | null>(null)
	let updateError = $state<string | null>(null)
	let selectedRowCount = $state(0)

	// AG Grid data
	let gridColumnDefs = $state<ColDef[]>([])
	let gridRowData = $state<Record<string, unknown>[]>([])

	// Reference to the DataGrid component
	let dataGrid: DataGrid | undefined = $state()

	// Track pending new rows (not yet inserted into database)
	// These are rows that have been added to the grid but not yet saved
	let pendingNewRows = $state<Set<string>>(new Set())

	// Check if we have unsaved new rows
	let hasUnsavedRows = $derived(pendingNewRows.size > 0)

	// Cache for foreign key lookup values (column_name -> lookup values)
	let fkLookupCache = $state<Map<string, ForeignKeyLookupValue[]>>(new Map())

	// Orphaned blob cleanup state
	let showCleanupDialog = $state(false)
	let cleanupPreview = $state<OrphanCleanupResult | null>(null)
	let isPreviewingOrphans = $state(false)
	let isCleaningOrphans = $state(false)
	let cleanupResult = $state<OrphanCleanupResult | null>(null)

	// Load tables immediately when component initializes
	loadTables()

	async function loadTables() {
		isLoadingTables = true
		error = null
		try {
			tables = await invoke<TableInfo[]>('inspector_get_tables')
		} catch (e) {
			error = e instanceof Error ? e.message : String(e)
			console.error('Failed to load tables:', e)
		} finally {
			isLoadingTables = false
		}
	}

	async function selectTable(tableName: string) {
		// Clear pending rows when switching tables
		pendingNewRows = new Set()
		selectedTable = tableName
		// Set editability based on table info
		const tableInfo = tables.find((t) => t.name === tableName)
		isTableEditable = tableInfo?.is_editable ?? true
		await loadTableData()
	}

	async function loadTableData() {
		if (!selectedTable) return

		isLoadingData = true
		error = null
		// Clear pending rows when reloading data
		pendingNewRows = new Set()
		// Clear FK lookup cache when switching tables
		fkLookupCache = new Map()
		try {
			// Load columns and data - get all rows for AG Grid client-side model
			const [cols, result] = await Promise.all([
				invoke<ColumnInfo[]>('inspector_get_columns', { tableName: selectedTable }),
				invoke<QueryResult>('inspector_query_table', {
					tableName: selectedTable,
					offset: 0,
					limit: 10000 // Get more rows for AG Grid pagination
				})
			])
			columns = cols
			queryResult = result

			// Fetch FK lookup values for columns that have foreign keys
			const fkColumns = cols.filter((col) => col.foreign_key)
			const fkPromises = fkColumns.map(async (col) => {
				if (!col.foreign_key) return
				try {
					const lookupValues = await invoke<ForeignKeyLookupValue[]>('inspector_get_fk_lookup_values', {
						targetTable: col.foreign_key.target_table,
						targetColumn: col.foreign_key.target_column
					})
					fkLookupCache.set(col.name, lookupValues)
				} catch (e) {
					console.warn(`Failed to load FK lookup values for ${col.name}:`, e)
				}
			})
			await Promise.all(fkPromises)

			// Convert to AG Grid format with dropdown editors for FK columns
			gridColumnDefs = cols.map((col) => {
				const colDef: ColDef = {
					field: col.name,
					headerName: col.name,
					headerTooltip: `${col.data_type}${col.primary_key ? ' (PK)' : ''}${col.nullable ? '' : ' NOT NULL'}${col.foreign_key ? ` → ${col.foreign_key.target_table}.${col.foreign_key.target_column}` : ''}`,
					cellClass: col.primary_key ? 'font-semibold' : '',
					cellRenderer: (params: { value: unknown }) => {
						if (params.value === null) {
							return '<span class="text-muted-foreground italic">NULL</span>'
						}
						// For FK columns, show the display value if available
						if (col.foreign_key) {
							const lookupValues = fkLookupCache.get(col.name)
							if (lookupValues) {
								const match = lookupValues.find((lv) => lv.id === params.value)
								if (match && match.display !== String(params.value)) {
									return `${match.display} <span class="text-muted-foreground">(${params.value})</span>`
								}
							}
						}
						const val = String(params.value)
						// Truncate long values
						return val.length > 200 ? val.slice(0, 200) + '...' : val
					}
				}

				// Add dropdown cell editor for foreign key columns
				if (col.foreign_key) {
					const lookupValues = fkLookupCache.get(col.name)
					if (lookupValues && lookupValues.length > 0) {
						colDef.cellEditor = 'agSelectCellEditor'
						colDef.cellEditorParams = {
							values: lookupValues.map((lv) => lv.id)
						}
						// Show display value in dropdown
						colDef.valueFormatter = (params: { value: unknown }) => {
							if (params.value === null) return 'NULL'
							const match = lookupValues.find((lv) => lv.id === params.value)
							return match ? `${match.display} (${params.value})` : String(params.value)
						}
					}
				}

				return colDef
			})

			// Convert rows array to objects for AG Grid
			gridRowData = result.rows.map((row) => {
				const obj: Record<string, unknown> = {}
				cols.forEach((col, i) => {
					obj[col.name] = row[i]
				})
				return obj
			})
		} catch (e) {
			error = e instanceof Error ? e.message : String(e)
			console.error('Failed to load table data:', e)
		} finally {
			isLoadingData = false
		}
	}

	async function handleCellValueChanged(event: CellValueChangedEvent) {
		if (!selectedTable) return

		// Clear any previous update error
		updateError = null

		// Get the field (column name) that was edited
		const columnName = event.colDef.field
		if (!columnName) {
			console.error('No column name in cell edit event')
			return
		}

		// Check if this is a pending row (not in DB yet)
		const rowId = getRowId(event.data)
		if (pendingNewRows.has(rowId)) {
			// For pending rows, just update in grid (no DB call needed)
			// The row will be inserted with all values when user clicks Save
			console.log(`[Inspector] Updated pending row ${rowId}.${columnName} = ${JSON.stringify(event.newValue)}`)
			return
		}

		// Get the new value
		const newValue = event.newValue

		// Find primary key columns for this table
		const primaryKeyColumns = columns.filter((col) => col.primary_key)

		if (primaryKeyColumns.length === 0) {
			// No primary key - use rowid if available, or warn user
			updateError = 'Cannot update: Table has no primary key defined'
			// Revert the change in the grid
			event.api.applyTransaction({
				update: [{ ...event.data, [columnName]: event.oldValue }]
			})
			return
		}

		// Build primary key values from the row data
		const primaryKeys: [string, unknown][] = primaryKeyColumns.map((col) => [
			col.name,
			event.data[col.name]
		])

		try {
			await invoke('inspector_update_cell', {
				tableName: selectedTable,
				columnName,
				newValue,
				primaryKeys
			})

			console.log(
				`Updated ${selectedTable}.${columnName} = ${JSON.stringify(newValue)} where ${primaryKeys.map(([k, v]) => `${k}=${v}`).join(' AND ')}`
			)
		} catch (e) {
			updateError = e instanceof Error ? e.message : String(e)
			console.error('Failed to update cell:', e)

			// Revert the change in the grid
			event.api.applyTransaction({
				update: [{ ...event.data, [columnName]: event.oldValue }]
			})
		}
	}

	function handleSelectionChanged(event: SelectionChangedEvent) {
		selectedRowCount = event.api.getSelectedRows().length
	}

	async function handleRowsDeleted(deletedRows: Record<string, unknown>[]) {
		if (!selectedTable || deletedRows.length === 0) return

		// Separate pending rows (not in DB yet) from existing rows
		const pendingToRemove: string[] = []
		const existingToDelete: Record<string, unknown>[] = []

		for (const row of deletedRows) {
			const rowId = getRowId(row)
			if (pendingNewRows.has(rowId)) {
				pendingToRemove.push(rowId)
			} else {
				existingToDelete.push(row)
			}
		}

		// Remove pending rows from tracking (no DB delete needed)
		if (pendingToRemove.length > 0) {
			pendingNewRows = new Set([...pendingNewRows].filter((id) => !pendingToRemove.includes(id)))
			console.log(`[Inspector] Removed ${pendingToRemove.length} pending rows from tracking`)
		}

		// Delete existing rows from database
		if (existingToDelete.length > 0) {
			const primaryKeyColumns = columns.filter((col) => col.primary_key)

			if (primaryKeyColumns.length === 0) {
				updateError = 'Cannot delete: Table has no primary key defined'
				// Reload data to restore deleted rows in grid
				await loadTableData()
				return
			}

			for (const row of existingToDelete) {
				const primaryKeys: [string, unknown][] = primaryKeyColumns.map((col) => [col.name, row[col.name]])

				try {
					await invoke('inspector_delete_row', {
						tableName: selectedTable,
						primaryKeys
					})
					console.log(`Deleted row from ${selectedTable} where ${primaryKeys.map(([k, v]) => `${k}=${v}`).join(' AND ')}`)
				} catch (e) {
					updateError = e instanceof Error ? e.message : String(e)
					console.error('Failed to delete row:', e)
					// Reload data to restore the grid
					await loadTableData()
					return
				}
			}

			// Refresh table list to update row counts
			loadTables()
		}
	}

	async function handleRowsAdded(addedRows: Record<string, unknown>[]) {
		if (!selectedTable || addedRows.length === 0) return

		// Don't insert immediately - just track as pending
		// This avoids foreign key constraint errors for incomplete rows
		for (const row of addedRows) {
			const rowId = getRowId(row)
			pendingNewRows = new Set([...pendingNewRows, rowId])
		}
		console.log('[Inspector] Rows staged for insert:', addedRows.length, 'Total pending:', pendingNewRows.size)
	}

	async function saveNewRows() {
		if (!selectedTable || !dataGrid || pendingNewRows.size === 0) return

		isSaving = true
		updateError = null

		// Get all rows from grid and filter to pending ones
		const allRows = dataGrid.getAllRows()
		const rowsToInsert = allRows.filter((row) => pendingNewRows.has(getRowId(row)))

		console.log('[Inspector] Saving', rowsToInsert.length, 'pending rows')

		let insertedCount = 0
		const failedRowIds: string[] = []

		for (const row of rowsToInsert) {
			const rowId = getRowId(row)
			try {
				await invoke('inspector_insert_row', {
					tableName: selectedTable,
					rowData: row
				})
				console.log(`[Inspector] Inserted row into ${selectedTable}:`, row)
				// Remove from pending set on success
				pendingNewRows = new Set([...pendingNewRows].filter((id) => id !== rowId))
				insertedCount++
			} catch (e) {
				updateError = e instanceof Error ? e.message : String(e)
				console.error('[Inspector] Failed to insert row:', e)
				failedRowIds.push(rowId)
				// Continue trying other rows
			}
		}

		isSaving = false

		if (insertedCount > 0) {
			// Refresh table list to update row counts
			loadTables()
		}

		if (failedRowIds.length > 0 && insertedCount > 0) {
			updateError = `Inserted ${insertedCount} rows. ${failedRowIds.length} rows failed - check foreign key values.`
		} else if (failedRowIds.length > 0) {
			updateError = `Failed to insert rows. Check foreign key values and required fields.`
		}
	}

	function discardNewRows() {
		if (!dataGrid || pendingNewRows.size === 0) return

		// Get all rows from grid and find the pending ones to delete
		const allRows = dataGrid.getAllRows()
		const rowsToDelete = allRows.filter((row) => pendingNewRows.has(getRowId(row)))

		console.log('[Inspector] Discarding', rowsToDelete.length, 'pending rows')

		// Remove from grid
		dataGrid.deleteRows(rowsToDelete)

		// Clear pending set
		pendingNewRows = new Set()
	}

	// Generate a simple UUID for new rows
	function generateTempId(): string {
		return 'new-' + Math.random().toString(36).substring(2, 11) + '-' + Date.now().toString(36)
	}

	// Generate ISO timestamp for datetime columns
	function getCurrentISOTimestamp(): string {
		return new Date().toISOString()
	}

	// Check if column name is a timestamp column
	function isTimestampColumn(name: string): boolean {
		const lower = name.toLowerCase()
		return (
			lower === 'created_at' ||
			lower === 'updated_at' ||
			lower === 'createdat' ||
			lower === 'updatedat' ||
			lower === 'created' ||
			lower === 'updated' ||
			lower === 'timestamp' ||
			lower === 'date_created' ||
			lower === 'date_updated' ||
			lower === 'modification_date' ||
			lower === 'creation_date'
		)
	}

	// Check if column type is a date/time type
	function isDateTimeType(type: string): boolean {
		const upper = type.toUpperCase()
		return (
			upper.includes('DATE') ||
			upper.includes('TIME') ||
			upper.includes('TIMESTAMP')
		)
	}

	function addNewRow() {
		console.log('[Inspector] addNewRow called, dataGrid:', dataGrid, 'columns:', columns.length)
		if (!dataGrid || columns.length === 0) {
			console.warn('[Inspector] addNewRow: dataGrid or columns not ready')
			return
		}

		const now = getCurrentISOTimestamp()

		// Create an empty row with default values based on column types
		const newRow: Record<string, unknown> = {}
		columns.forEach((col) => {
			const type = col.data_type.toUpperCase()

			if (col.primary_key) {
				// For primary keys, generate a temporary ID
				// - For TEXT/VARCHAR type: generate a UUID-like string
				// - For INTEGER type: generate a negative temp ID (to be replaced on insert)
				if (type.includes('INT')) {
					// Negative number as temporary ID - will be replaced by SQLite autoincrement
					newRow[col.name] = -Math.floor(Math.random() * 1000000)
				} else {
					newRow[col.name] = generateTempId()
				}
			} else if (isTimestampColumn(col.name) || isDateTimeType(type)) {
				// Auto-fill timestamp columns with current time
				newRow[col.name] = now
			} else if (col.foreign_key) {
				// For foreign key columns, try to set the first available value or null
				const lookupValues = fkLookupCache.get(col.name)
				if (lookupValues && lookupValues.length > 0 && !col.nullable) {
					// If column is NOT NULL, default to the first available FK value
					newRow[col.name] = lookupValues[0].id
				} else {
					newRow[col.name] = null
				}
			} else if (col.nullable) {
				newRow[col.name] = null
			} else {
				// Default values based on type for NOT NULL columns
				if (type.includes('INT')) {
					newRow[col.name] = 0
				} else if (type.includes('REAL') || type.includes('FLOAT') || type.includes('DOUBLE')) {
					newRow[col.name] = 0.0
				} else if (type.includes('BOOL')) {
					newRow[col.name] = false
				} else {
					newRow[col.name] = ''
				}
			}
		})

		console.log('[Inspector] Adding new row at index 0:', newRow)
		// Add at index 0 to insert at the top of the table
		const result = dataGrid.addRow(newRow, 0)
		console.log('[Inspector] addRow result:', result)
	}

	function deleteSelectedRows() {
		if (!dataGrid) return
		const deleted = dataGrid.deleteSelectedRows()
		if (deleted.length === 0) {
			updateError = 'No rows selected to delete'
		}
	}

	function getRowId(data: Record<string, unknown>): string {
		// Use primary key(s) as row ID for transactions
		const primaryKeyColumns = columns.filter((col) => col.primary_key)
		if (primaryKeyColumns.length > 0) {
			return primaryKeyColumns.map((col) => String(data[col.name])).join('-')
		}
		// Fallback: use all column values (not ideal but works)
		return JSON.stringify(data)
	}

	// Format bytes for display
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B'
		const k = 1024
		const sizes = ['B', 'KB', 'MB', 'GB']
		const i = Math.floor(Math.log(bytes) / Math.log(k))
		return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
	}

	// Open cleanup dialog and preview orphaned blobs
	async function openCleanupDialog() {
		showCleanupDialog = true
		cleanupPreview = null
		cleanupResult = null
		isPreviewingOrphans = true

		try {
			cleanupPreview = await invoke<OrphanCleanupResult>('inspector_preview_orphaned_blobs')
		} catch (e) {
			console.error('Failed to preview orphaned blobs:', e)
			updateError = e instanceof Error ? e.message : String(e)
		} finally {
			isPreviewingOrphans = false
		}
	}

	// Execute orphaned blob cleanup
	async function executeCleanup() {
		isCleaningOrphans = true
		cleanupResult = null

		try {
			cleanupResult = await invoke<OrphanCleanupResult>('inspector_cleanup_orphaned_blobs')
			// Refresh table list to update blob_registry row count
			await loadTables()
			// If we're viewing blob_registry, reload its data
			if (selectedTable === 'blob_registry') {
				await loadTableData()
			}
		} catch (e) {
			console.error('Failed to cleanup orphaned blobs:', e)
			updateError = e instanceof Error ? e.message : String(e)
		} finally {
			isCleaningOrphans = false
		}
	}

	function closeCleanupDialog() {
		showCleanupDialog = false
		cleanupPreview = null
		cleanupResult = null
	}
</script>

<div class="flex h-full">
	<!-- Table list sidebar -->
	<div class="border-border flex w-64 shrink-0 flex-col border-r">
		<div class="border-border flex items-center justify-between border-b p-3">
			<h2 class="text-sm font-semibold">Tables</h2>
			<div class="flex items-center gap-1">
				<button
					onclick={openCleanupDialog}
					class="rounded p-1 text-muted-foreground hover:bg-secondary/50 hover:text-foreground"
					title="Clean up orphaned blobs"
				>
					<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
						/>
					</svg>
				</button>
				<button
					onclick={loadTables}
					disabled={isLoadingTables}
					class="rounded p-1 text-muted-foreground hover:bg-secondary/50 hover:text-foreground disabled:opacity-50"
					title="Refresh tables"
				>
					<svg
						class="h-4 w-4 {isLoadingTables ? 'animate-spin' : ''}"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
						/>
					</svg>
				</button>
			</div>
		</div>

		<div class="flex-1 overflow-auto">
			{#if isLoadingTables && tables.length === 0}
				<div class="flex items-center justify-center p-4">
					<svg class="h-5 w-5 animate-spin text-muted-foreground" fill="none" viewBox="0 0 24 24">
						<circle
							class="opacity-25"
							cx="12"
							cy="12"
							r="10"
							stroke="currentColor"
							stroke-width="4"
						></circle>
						<path
							class="opacity-75"
							fill="currentColor"
							d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
						></path>
					</svg>
				</div>
			{:else if tables.length === 0}
				<div class="p-4 text-sm text-muted-foreground">No tables found</div>
			{:else}
				<ul class="divide-y divide-border">
					{#each tables as table}
						<li>
							<button
								onclick={() => selectTable(table.name)}
								class="flex w-full items-center justify-between px-3 py-2 text-sm transition-colors hover:bg-secondary/50 {selectedTable ===
								table.name
									? 'bg-secondary'
									: ''} {!table.is_editable ? 'text-muted-foreground' : ''}"
							>
								<span class="flex items-center gap-2">
									{#if table.is_editable}
										<svg
											class="h-4 w-4 text-muted-foreground"
											fill="none"
											stroke="currentColor"
											viewBox="0 0 24 24"
										>
											<path
												stroke-linecap="round"
												stroke-linejoin="round"
												stroke-width="2"
												d="M3 10h18M3 14h18m-9-4v8m-7 0h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
											/>
										</svg>
									{:else}
										<!-- Lock icon for read-only tables -->
										<span title="Read-only table">
											<svg
												class="h-4 w-4 text-muted-foreground/60"
												fill="none"
												stroke="currentColor"
												viewBox="0 0 24 24"
											>
												<path
													stroke-linecap="round"
													stroke-linejoin="round"
													stroke-width="2"
													d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
												/>
											</svg>
										</span>
									{/if}
									<span class="truncate">{table.name}</span>
								</span>
								<span class="text-xs text-muted-foreground">{table.row_count}</span>
							</button>
						</li>
					{/each}
				</ul>
			{/if}
		</div>
	</div>

	<!-- Main content -->
	<div class="flex flex-1 flex-col overflow-hidden">
		{#if error}
			<div
				class="bg-destructive/10 border-destructive/20 m-4 rounded-lg border p-4 text-sm text-destructive"
			>
				{error}
			</div>
		{/if}

		{#if updateError}
			<div
				class="bg-destructive/10 border-destructive/20 mx-4 mt-2 rounded-lg border p-3 text-sm text-destructive"
			>
				<span class="font-medium">Update failed:</span>
				{updateError}
				<button
					onclick={() => (updateError = null)}
					class="ml-2 text-xs underline hover:no-underline"
				>
					Dismiss
				</button>
			</div>
		{/if}

		{#if !selectedTable}
			<div class="flex flex-1 items-center justify-center">
				<div class="text-center">
					<svg
						class="mx-auto h-12 w-12 text-muted-foreground/50"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4"
						/>
					</svg>
					<h3 class="mt-4 text-lg font-medium">Select a table</h3>
					<p class="mt-1 text-sm text-muted-foreground">
						Choose a table from the sidebar to view its data
					</p>
				</div>
			</div>
		{:else}
			<!-- Table header -->
			<div class="flex items-center justify-between border-b border-border px-4 py-2">
				<div>
					<div class="flex items-center gap-2">
						<h2 class="font-semibold">{selectedTable}</h2>
						{#if !isTableEditable}
							<span class="inline-flex items-center gap-1 rounded bg-muted px-1.5 py-0.5 text-xs text-muted-foreground">
								<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
									/>
								</svg>
								Read-Only
							</span>
						{/if}
					</div>
					{#if queryResult}
						<p class="text-xs text-muted-foreground">
							{queryResult.total_rows} rows, {columns.length} columns
							{#if selectedRowCount > 0}
								<span class="text-primary">• {selectedRowCount} selected</span>
							{/if}
						</p>
					{/if}
				</div>

				<div class="flex items-center gap-2">
					{#if hasUnsavedRows && isTableEditable}
						<!-- Save Pending Rows Button -->
						<button
							onclick={saveNewRows}
							disabled={isSaving}
							class="flex items-center gap-1 rounded border border-primary bg-primary px-2 py-1 text-xs text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
							title="Save new rows to database"
						>
							{#if isSaving}
								<svg class="h-3 w-3 animate-spin" fill="none" viewBox="0 0 24 24">
									<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
									<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
								</svg>
							{:else}
								<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
								</svg>
							{/if}
							Save ({pendingNewRows.size})
						</button>

						<!-- Discard Pending Rows Button -->
						<button
							onclick={discardNewRows}
							disabled={isSaving}
							class="flex items-center gap-1 rounded border border-border px-2 py-1 text-xs hover:bg-destructive/10 hover:border-destructive/50 hover:text-destructive disabled:opacity-50"
							title="Discard unsaved new rows"
						>
							<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
							</svg>
							Discard
						</button>

						<div class="h-4 w-px bg-border"></div>
					{/if}

					{#if isTableEditable}
						<!-- Add Row Button -->
						<button
							onclick={addNewRow}
							disabled={isLoadingData}
							class="flex items-center gap-1 rounded border border-border px-2 py-1 text-xs hover:bg-secondary disabled:opacity-50"
							title="Add new row"
						>
							<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
							</svg>
							Add Row
						</button>

						<!-- Delete Selected Button -->
						<button
							onclick={deleteSelectedRows}
							disabled={isLoadingData || selectedRowCount === 0}
							class="flex items-center gap-1 rounded border border-destructive/50 px-2 py-1 text-xs text-destructive hover:bg-destructive/10 disabled:opacity-50 disabled:border-border disabled:text-muted-foreground"
							title="Delete selected rows (Delete key)"
						>
							<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
								/>
							</svg>
							Delete{#if selectedRowCount > 0} ({selectedRowCount}){/if}
						</button>
					{/if}

					<!-- Refresh Button -->
					<button
						onclick={loadTableData}
						disabled={isLoadingData}
						class="flex items-center gap-1 rounded border border-border px-2 py-1 text-xs hover:bg-secondary disabled:opacity-50"
						title="Refresh table data"
					>
						<svg
							class="h-3 w-3 {isLoadingData ? 'animate-spin' : ''}"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
							/>
						</svg>
						Refresh
					</button>
				</div>
			</div>

			<!-- Selection hint -->
			<div class="bg-muted/30 border-b border-border px-4 py-1 text-xs text-muted-foreground">
				{#if isTableEditable}
					Tip: Cmd+Click (Mac) or Ctrl+Click (Windows) to select multiple rows. Press Delete to remove selected rows.
				{:else}
					This is a system table and cannot be modified. You can view the data but not edit, add, or delete rows.
				{/if}
			</div>

			<!-- AG Grid Data Grid -->
			<div class="flex-1 overflow-hidden">
				{#if isLoadingData}
					<div class="flex h-full items-center justify-center">
						<svg class="h-8 w-8 animate-spin text-muted-foreground" fill="none" viewBox="0 0 24 24">
							<circle
								class="opacity-25"
								cx="12"
								cy="12"
								r="10"
								stroke="currentColor"
								stroke-width="4"
							></circle>
							<path
								class="opacity-75"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							></path>
						</svg>
					</div>
				{:else if gridRowData.length > 0}
					<DataGrid
						bind:this={dataGrid}
						columnDefs={gridColumnDefs}
						rowData={gridRowData}
						autoSizeColumns={true}
						height="100%"
						editable={isTableEditable}
						multiSelect={true}
						showCheckboxes={false}
						enableKeyboardDelete={isTableEditable}
						{getRowId}
						onCellValueChanged={handleCellValueChanged}
						onSelectionChanged={handleSelectionChanged}
						onRowsDeleted={isTableEditable ? handleRowsDeleted : undefined}
						onRowsAdded={isTableEditable ? handleRowsAdded : undefined}
					/>
				{:else if queryResult}
					<div class="flex h-32 items-center justify-center text-muted-foreground">
						No data in this table
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>

<!-- Orphaned Blob Cleanup Dialog -->
{#if showCleanupDialog}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onclick={closeCleanupDialog}>
		<div
			class="w-full max-w-md rounded-lg border border-border bg-background p-6 shadow-xl"
			onclick={(e) => e.stopPropagation()}
		>
			<div class="mb-4 flex items-center gap-3">
				<div class="flex h-10 w-10 items-center justify-center rounded-full bg-muted">
					<svg class="h-5 w-5 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
						/>
					</svg>
				</div>
				<div>
					<h3 class="text-lg font-semibold">Clean Up Orphaned Blobs</h3>
					<p class="text-sm text-muted-foreground">Remove unreferenced parquet files</p>
				</div>
			</div>

			{#if isPreviewingOrphans}
				<div class="flex items-center justify-center py-8">
					<svg class="h-6 w-6 animate-spin text-muted-foreground" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
						<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
					</svg>
					<span class="ml-2 text-sm text-muted-foreground">Scanning for orphaned blobs...</span>
				</div>
			{:else if cleanupResult}
				<!-- Cleanup completed -->
				<div class="rounded-lg border border-green-500/20 bg-green-500/10 p-4">
					<div class="flex items-center gap-2 text-green-600 dark:text-green-400">
						<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
						</svg>
						<span class="font-medium">Cleanup Complete</span>
					</div>
					<div class="mt-2 space-y-1 text-sm">
						<p>Deleted <strong>{cleanupResult.deleted_count}</strong> orphaned blobs</p>
						<p>Freed <strong>{formatBytes(cleanupResult.bytes_freed)}</strong> of storage</p>
					</div>
					{#if cleanupResult.errors.length > 0}
						<div class="mt-3 rounded border border-amber-500/20 bg-amber-500/10 p-2 text-xs text-amber-600 dark:text-amber-400">
							<p class="font-medium">{cleanupResult.errors.length} errors occurred:</p>
							<ul class="mt-1 list-inside list-disc">
								{#each cleanupResult.errors.slice(0, 3) as error}
									<li class="truncate">{error}</li>
								{/each}
								{#if cleanupResult.errors.length > 3}
									<li>...and {cleanupResult.errors.length - 3} more</li>
								{/if}
							</ul>
						</div>
					{/if}
				</div>
				<div class="mt-4 flex justify-end">
					<button
						onclick={closeCleanupDialog}
						class="rounded bg-primary px-4 py-2 text-sm text-primary-foreground hover:bg-primary/90"
					>
						Done
					</button>
				</div>
			{:else if cleanupPreview}
				<!-- Preview results -->
				{#if cleanupPreview.orphan_count === 0}
					<div class="rounded-lg border border-border bg-muted/50 p-4 text-center">
						<svg class="mx-auto h-8 w-8 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
						</svg>
						<p class="mt-2 text-sm text-muted-foreground">No orphaned blobs found</p>
						<p class="text-xs text-muted-foreground">All blob registry entries are properly referenced</p>
					</div>
					<div class="mt-4 flex justify-end">
						<button
							onclick={closeCleanupDialog}
							class="rounded bg-secondary px-4 py-2 text-sm hover:bg-secondary/80"
						>
							Close
						</button>
					</div>
				{:else}
					<div class="rounded-lg border border-amber-500/20 bg-amber-500/10 p-4">
						<div class="flex items-center gap-2 text-amber-600 dark:text-amber-400">
							<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
							</svg>
							<span class="font-medium">Orphaned Blobs Found</span>
						</div>
						<div class="mt-2 space-y-1 text-sm">
							<p><strong>{cleanupPreview.orphan_count}</strong> orphaned blobs detected</p>
							<p><strong>{formatBytes(cleanupPreview.bytes_freed)}</strong> can be freed</p>
						</div>
						<p class="mt-3 text-xs text-muted-foreground">
							These are parquet files in blob_registry that are no longer referenced by any curves, trajectories, or other data.
						</p>
					</div>
					<div class="mt-4 flex justify-end gap-2">
						<button
							onclick={closeCleanupDialog}
							disabled={isCleaningOrphans}
							class="rounded border border-border px-4 py-2 text-sm hover:bg-secondary disabled:opacity-50"
						>
							Cancel
						</button>
						<button
							onclick={executeCleanup}
							disabled={isCleaningOrphans}
							class="flex items-center gap-2 rounded bg-destructive px-4 py-2 text-sm text-destructive-foreground hover:bg-destructive/90 disabled:opacity-50"
						>
							{#if isCleaningOrphans}
								<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
									<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
									<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
								</svg>
								Cleaning...
							{:else}
								Delete Orphaned Blobs
							{/if}
						</button>
					</div>
				{/if}
			{/if}
		</div>
	</div>
{/if}
