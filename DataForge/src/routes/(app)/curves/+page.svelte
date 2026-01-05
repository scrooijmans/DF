<script lang="ts">
	import { invoke } from '@tauri-apps/api/core'
	import DataGrid from '$lib/components/data/DataGrid.svelte'
	import { DataCoverageChart } from '$lib/components/charts'
	import type { ColDef, CellValueChangedEvent, SelectionChangedEvent } from 'ag-grid-community'

	// View mode for the main content area
	type ViewMode = 'coverage' | 'data'

	// Depth input modal state
	let showDepthModal = $state(false)
	let depthInputValue = $state('')
	let depthInputError = $state<string | null>(null)
	let depthInputRef = $state<HTMLInputElement | null>(null)

	// Types matching Rust backend
	interface WellSummary {
		id: string
		name: string
		uwi: string | null
		field: string | null
		curve_count: number
	}

	interface StoredCurveInfo {
		id: string
		mnemonic: string
		unit: string | null
		description: string | null
		sample_count: number
		min_depth: number | null
		max_depth: number | null
		min_value: number | null
		max_value: number | null
		native_parquet_hash: string | null
		quality_flag: string | null
	}

	interface CurveDataResult {
		columns: string[]
		rows: (number | null)[][]
		total_rows: number
	}

	// State
	let wells = $state<WellSummary[]>([])
	let selectedWell = $state<WellSummary | null>(null)
	let curves = $state<StoredCurveInfo[]>([])
	let selectedCurves = $state<Set<string>>(new Set())
	let curveData = $state<CurveDataResult | null>(null)

	let isLoadingWells = $state(true)
	let isLoadingCurves = $state(false)
	let isLoadingData = $state(false)
	let isSaving = $state(false)
	let error = $state<string | null>(null)
	let selectedRowCount = $state(0)

	// View mode: show coverage chart or data grid
	let viewMode = $state<ViewMode>('data')

	// AG Grid data
	let gridColumnDefs = $state<ColDef[]>([])
	let gridRowData = $state<Record<string, unknown>[]>([])
	let dataGrid = $state<DataGrid | null>(null)

	// Original data snapshot for computing diffs on save
	let originalRowData = $state<Record<string, unknown>[]>([])

	// Track pending changes count for UI (updated incrementally for performance)
	let pendingChangesCount = $state(0)

	// Computed: has unsaved changes
	let hasUnsavedChanges = $derived(pendingChangesCount > 0)

	// Compute diff between original and current state (only called on save)
	interface DataDiff {
		deletedDepths: number[] // Depths that were in original but not in current
		addedRows: Record<string, unknown>[] // Rows with depths not in original
		editedCells: {
			depth: number
			column: string
			oldValue: number | null
			newValue: number | null
		}[]
	}

	function computeDataDiff(): DataDiff {
		const depthCol = curveData?.columns[0] || 'DEPTH'

		// Build lookup maps by depth
		const originalByDepth = new Map<number, Record<string, unknown>>()
		for (const row of originalRowData) {
			const depth = row[depthCol] as number
			if (depth !== null && depth !== undefined) {
				originalByDepth.set(depth, row)
			}
		}

		const currentByDepth = new Map<number, Record<string, unknown>>()
		for (const row of gridRowData) {
			const depth = row[depthCol] as number
			if (depth !== null && depth !== undefined) {
				currentByDepth.set(depth, row)
			}
		}

		// Find deleted depths (in original but not in current)
		const deletedDepths: number[] = []
		for (const depth of originalByDepth.keys()) {
			if (!currentByDepth.has(depth)) {
				deletedDepths.push(depth)
			}
		}

		// Find added rows (in current but not in original)
		const addedRows: Record<string, unknown>[] = []
		for (const [depth, row] of currentByDepth) {
			if (!originalByDepth.has(depth)) {
				addedRows.push(row)
			}
		}

		// Find edited cells (same depth, different values)
		const editedCells: DataDiff['editedCells'] = []
		for (const [depth, currentRow] of currentByDepth) {
			const originalRow = originalByDepth.get(depth)
			if (!originalRow) continue // This is an added row, not an edit

			// Compare each column (except depth and _rowIndex)
			for (const col of curveData?.columns || []) {
				if (col === depthCol) continue // Skip depth column

				const oldVal = originalRow[col] as number | null
				const newVal = currentRow[col] as number | null

				// Compare values (treat undefined as null)
				const oldNorm = oldVal ?? null
				const newNorm = newVal ?? null

				if (oldNorm !== newNorm) {
					editedCells.push({
						depth,
						column: col,
						oldValue: oldNorm,
						newValue: newNorm
					})
				}
			}
		}

		return { deletedDepths, addedRows, editedCells }
	}

	// Load wells on component mount
	loadWells()

	async function loadWells() {
		isLoadingWells = true
		error = null
		try {
			wells = await invoke<WellSummary[]>('get_workspace_wells')
		} catch (e) {
			error = e instanceof Error ? e.message : String(e)
			console.error('Failed to load wells:', e)
		} finally {
			isLoadingWells = false
		}
	}

	async function selectWell(well: WellSummary) {
		selectedWell = well
		selectedCurves = new Set()
		curveData = null
		gridRowData = []
		clearPendingChanges()
		await loadCurvesAndData()
	}

	async function loadCurvesAndData() {
		if (!selectedWell) return

		isLoadingCurves = true
		isLoadingData = true
		error = null
		try {
			// Load curve metadata
			curves = await invoke<StoredCurveInfo[]>('get_well_curves', {
				wellId: selectedWell.id
			})

			// Auto-select all curves
			selectedCurves = new Set(curves.map((c) => c.id))

			// If there are curves, load all data
			if (curves.length > 0) {
				await loadCurveData()
			}
		} catch (e) {
			error = e instanceof Error ? e.message : String(e)
			console.error('Failed to load curves:', e)
		} finally {
			isLoadingCurves = false
			isLoadingData = false
		}
	}

	async function loadCurveData() {
		if (selectedCurves.size === 0) {
			curveData = null
			gridRowData = []
			originalRowData = []
			clearPendingChanges()
			return
		}

		isLoadingData = true
		error = null
		try {
			const result = await invoke<CurveDataResult>('query_curve_data', {
				curveIds: Array.from(selectedCurves),
				depthMin: null,
				depthMax: null,
				limit: 10000,
				offset: 0
			})
			curveData = result

			// Convert to AG Grid format
			gridColumnDefs = result.columns.map((col, index) => ({
				field: col,
				headerName: col,
				// Depth column (first) is not editable - it's the row identifier
				editable: index > 0,
				valueFormatter: (params: { value: number | null }) => {
					if (params.value === null || params.value === undefined) {
						return ''
					}
					// Format numbers with appropriate precision
					return typeof params.value === 'number' ? params.value.toFixed(4) : String(params.value)
				},
				valueParser: (params: { newValue: string }) => {
					// Parse edited values back to numbers
					if (params.newValue === '' || params.newValue === null) {
						return null
					}
					const num = parseFloat(params.newValue)
					return isNaN(num) ? null : num
				}
			}))

			// Convert rows array to objects for AG Grid
			// Add _rowIndex for tracking edits
			gridRowData = result.rows.map((row, rowIndex) => {
				const obj: Record<string, unknown> = { _rowIndex: rowIndex }
				result.columns.forEach((col, i) => {
					obj[col] = row[i]
				})
				return obj
			})

			// Store original data for discard functionality
			originalRowData = JSON.parse(JSON.stringify(gridRowData))

			// Clear any pending changes when loading new data
			clearPendingChanges()

			// Switch to data view when data is loaded
			viewMode = 'data'
		} catch (e) {
			error = e instanceof Error ? e.message : String(e)
			console.error('Failed to load curve data:', e)
		} finally {
			isLoadingData = false
		}
	}

	// Clear pending changes
	function clearPendingChanges() {
		pendingChangesCount = 0
	}

	// Get row ID for AG Grid transactions (use DEPTH as natural key)
	function getRowId(data: Record<string, unknown>): string {
		// Use the _rowIndex we added, or fall back to DEPTH column
		if (data._rowIndex !== undefined) {
			return String(data._rowIndex)
		}
		// Find depth column (first column in result)
		const depthCol = curveData?.columns[0]
		if (depthCol && data[depthCol] !== undefined) {
			return String(data[depthCol])
		}
		return JSON.stringify(data)
	}

	// Handle cell value changes - sync gridRowData and increment change count
	function handleCellValueChanged(event: CellValueChangedEvent) {
		const column = event.colDef.field as string
		const oldValue = event.oldValue as number | null
		const newValue = event.newValue as number | null

		// Don't track if value didn't actually change
		if (oldValue === newValue) return

		// Sync gridRowData with the changed cell
		// event.data contains the updated row data from AG Grid
		const depthCol = curveData?.columns[0] || 'DEPTH'
		const changedDepth = event.data[depthCol] as number
		const rowIndex = gridRowData.findIndex((row) => row[depthCol] === changedDepth)
		if (rowIndex >= 0) {
			gridRowData[rowIndex] = { ...event.data }
		}

		// Increment pending changes count
		pendingChangesCount++
		console.log('[Curves] Cell edited:', { column, oldValue, newValue })
	}

	// Handle selection changes
	function handleSelectionChanged(event: SelectionChangedEvent) {
		selectedRowCount = event.api.getSelectedRows().length
	}

	// Handle rows deleted - update gridRowData and increment change count
	function handleRowsDeleted(deletedRows: Record<string, unknown>[]) {
		const depthCol = curveData?.columns[0] || 'DEPTH'
		const deletedDepths = new Set(deletedRows.map((r) => r[depthCol] as number))

		// Remove deleted rows from gridRowData
		gridRowData = gridRowData.filter((row) => !deletedDepths.has(row[depthCol] as number))
		pendingChangesCount += deletedRows.length
		selectedRowCount = 0 // Clear selection count after delete

		console.log('[Curves] Rows deleted:', deletedRows.length)
	}

	// Delete selected rows from the grid
	function deleteSelectedRows() {
		if (!dataGrid) return
		dataGrid.deleteSelectedRows()
	}

	// Handle rows added - update gridRowData and increment change count
	function handleRowsAdded(addedRows: Record<string, unknown>[]) {
		const depthCol = curveData?.columns[0] || 'DEPTH'

		// Add new rows to gridRowData at the correct sorted position
		for (const row of addedRows) {
			const newDepth = row[depthCol] as number
			// Find insert position to maintain sorted order
			let insertIndex = gridRowData.length
			for (let i = 0; i < gridRowData.length; i++) {
				const rowDepth = gridRowData[i][depthCol] as number
				if (newDepth < rowDepth) {
					insertIndex = i
					break
				}
			}
			gridRowData = [...gridRowData.slice(0, insertIndex), row, ...gridRowData.slice(insertIndex)]
		}

		pendingChangesCount += addedRows.length
		console.log('[Curves] Rows added:', addedRows.length)
	}

	// Open the depth input modal (called from toolbar)
	function openDepthModal() {
		console.log('[Curves] openDepthModal called, dataGrid:', dataGrid, 'curveData:', curveData)
		if (!dataGrid || !curveData) {
			console.warn('[Curves] openDepthModal: dataGrid or curveData not ready')
			return
		}

		depthInputValue = ''
		depthInputError = null
		showDepthModal = true

		// Focus the input after modal opens
		setTimeout(() => {
			depthInputRef?.focus()
		}, 50)
	}

	// Handle depth input submission
	function submitDepthInput() {
		if (!dataGrid || !curveData) return

		// depthInputValue can be string or number depending on input type
		const depthStr = String(depthInputValue).trim()
		const depth = parseFloat(depthStr)
		if (isNaN(depth) || depthStr === '') {
			depthInputError = 'Please enter a valid number'
			return
		}

		// Check if depth already exists in the data
		const depthCol = curveData.columns[0]
		const existingDepths = gridRowData.map((row) => row[depthCol] as number | null)
		if (existingDepths.includes(depth)) {
			depthInputError = `Depth ${depth} already exists`
			return
		}

		// Find the correct sorted position for this depth
		// Data is sorted by depth ascending, find where this depth belongs
		let insertIndex = 0
		for (let i = 0; i < gridRowData.length; i++) {
			const rowDepth = gridRowData[i][depthCol] as number | null
			if (rowDepth !== null && depth > rowDepth) {
				insertIndex = i + 1
			} else if (rowDepth !== null && depth < rowDepth) {
				break
			}
		}

		// Create new row with the specified depth
		// Assign a unique _rowIndex for new rows (negative to distinguish from original rows)
		const newRowIndex = -(Date.now())
		const newRow: Record<string, unknown> = {
			_rowIndex: newRowIndex
		}
		curveData.columns.forEach((col, index) => {
			// First column (DEPTH) gets the user-specified value
			// Other columns start as null
			newRow[col] = index === 0 ? depth : null
		})

		console.log('[Curves] Adding new row at index', insertIndex, ':', newRow)
		const result = dataGrid.addRow(newRow, insertIndex)
		console.log('[Curves] addRow result:', result)

		// Close modal
		showDepthModal = false
		depthInputValue = ''
		depthInputError = null
	}

	// Cancel depth input
	function cancelDepthInput() {
		showDepthModal = false
		depthInputValue = ''
		depthInputError = null
	}

	// Handle Enter key in depth input
	function handleDepthInputKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			event.preventDefault()
			submitDepthInput()
		} else if (event.key === 'Escape') {
			cancelDepthInput()
		}
	}

	// Discard all pending changes
	function discardChanges() {
		if (!hasUnsavedChanges) return

		// Reset grid data to original
		gridRowData = JSON.parse(JSON.stringify(originalRowData))
		clearPendingChanges()

		console.log('[Curves] Changes discarded')
	}

	// Helper: Get curve ID by mnemonic (column name)
	function getCurveIdByMnemonic(mnemonic: string): string | undefined {
		const curve = curves.find((c) => c.mnemonic === mnemonic)
		return curve?.id
	}

	// Save all pending changes using snapshot-diff approach
	async function saveChanges() {
		if (!hasUnsavedChanges || selectedCurves.size === 0) return

		isSaving = true
		error = null

		try {
			const depthColumn = curveData?.columns[0] || 'DEPTH'
			const diff = computeDataDiff()

			console.log('[Curves] Computed diff:', {
				deletedDepths: diff.deletedDepths.length,
				addedRows: diff.addedRows.length,
				editedCells: diff.editedCells.length
			})

			// 1. Process deletions first (by depth, applied to ALL curves)
			if (diff.deletedDepths.length > 0) {
				// Find row indices in original data for each deleted depth
				const originalByDepth = new Map<number, number>() // depth -> rowIndex
				for (let i = 0; i < originalRowData.length; i++) {
					const depth = originalRowData[i][depthColumn] as number
					if (depth !== null && depth !== undefined) {
						originalByDepth.set(depth, i)
					}
				}

				const rowIndicesToDelete = diff.deletedDepths
					.map((depth) => originalByDepth.get(depth))
					.filter((idx): idx is number => idx !== undefined)

				// Delete from each curve
				for (const curveId of selectedCurves) {
					await invoke('curve_delete_rows', {
						request: {
							curve_id: curveId,
							row_indices: rowIndicesToDelete,
							reason: `Deleted ${rowIndicesToDelete.length} row(s)`
						}
					})
				}
				console.log('[Curves] Deleted rows at depths:', diff.deletedDepths)
			}

			// 2. Process cell edits (grouped by curve/mnemonic)
			if (diff.editedCells.length > 0) {
				// Build original depth -> rowIndex mapping
				const originalByDepth = new Map<number, number>()
				for (let i = 0; i < originalRowData.length; i++) {
					const depth = originalRowData[i][depthColumn] as number
					if (depth !== null && depth !== undefined) {
						originalByDepth.set(depth, i)
					}
				}

				// Group edits by column (mnemonic -> curve)
				const editsByCurve = new Map<string, typeof diff.editedCells>()
				for (const edit of diff.editedCells) {
					const curveId = getCurveIdByMnemonic(edit.column)
					if (!curveId) {
						console.warn(`[Curves] No curve found for column: ${edit.column}`)
						continue
					}
					if (!editsByCurve.has(curveId)) {
						editsByCurve.set(curveId, [])
					}
					editsByCurve.get(curveId)!.push(edit)
				}

				// Apply edits to each curve
				for (const [curveId, curveEdits] of editsByCurve) {
					const edits = curveEdits
						.map((edit) => {
							const rowIndex = originalByDepth.get(edit.depth)
							if (rowIndex === undefined) return null
							return {
								row_index: rowIndex,
								column: edit.column,
								new_value: edit.newValue
							}
						})
						.filter((e): e is NonNullable<typeof e> => e !== null)

					if (edits.length > 0) {
						await invoke('curve_update_cells', {
							request: {
								curve_id: curveId,
								edits,
								reason: `Edited ${edits.length} cell(s)`
							}
						})
						console.log(`[Curves] Saved ${edits.length} cell edits to curve ${curveId}`)
					}
				}
			}

			// 3. Process row additions (applied to ALL curves)
			if (diff.addedRows.length > 0) {
				for (const curveId of selectedCurves) {
					const curve = curves.find((c) => c.id === curveId)
					if (!curve) continue

					const rows = diff.addedRows.map((row) => ({
						[depthColumn]: row[depthColumn] as number | null,
						[curve.mnemonic]: row[curve.mnemonic] as number | null
					}))

					await invoke('curve_add_rows', {
						request: {
							curve_id: curveId,
							rows: rows,
							depth_column: depthColumn,
							reason: `Added ${rows.length} row(s)`
						}
					})
					console.log(`[Curves] Saved ${rows.length} row additions to curve ${curve.mnemonic}`)
				}
			}

			// Clear pending changes and reload data
			clearPendingChanges()
			await loadCurveData()

			console.log('[Curves] All changes saved successfully')
		} catch (e) {
			error = e instanceof Error ? e.message : String(e)
			console.error('[Curves] Failed to save changes:', e)
		} finally {
			isSaving = false
		}
	}

	// Get curve info for display
	function getCurveInfo(curveId: string): StoredCurveInfo | undefined {
		return curves.find((c) => c.id === curveId)
	}
</script>

<div class="flex h-full">
	<!-- Well/Curve selector sidebar -->
	<div class="border-border flex w-72 shrink-0 flex-col border-r">
		<div class="border-border flex items-center justify-between border-b p-3">
			<h2 class="text-sm font-semibold">Wells</h2>
			<button
				onclick={loadWells}
				disabled={isLoadingWells}
				class="rounded p-1 text-muted-foreground hover:bg-secondary/50 hover:text-foreground disabled:opacity-50"
				title="Refresh wells"
			>
				<svg
					class="h-4 w-4 {isLoadingWells ? 'animate-spin' : ''}"
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

		<div class="flex-1 overflow-auto">
			{#if isLoadingWells && wells.length === 0}
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
			{:else if wells.length === 0}
				<div class="p-4 text-sm text-muted-foreground">No wells found</div>
			{:else}
				<ul class="divide-y divide-border">
					{#each wells as well}
						<li>
							<button
								onclick={() => selectWell(well)}
								class="flex w-full items-center justify-between px-3 py-2 text-sm transition-colors hover:bg-secondary/50 {selectedWell?.id ===
								well.id
									? 'bg-secondary'
									: ''}"
							>
								<span class="flex flex-col items-start">
									<span class="font-medium truncate max-w-[180px]">{well.name}</span>
									{#if well.uwi}
										<span class="text-xs text-muted-foreground">{well.uwi}</span>
									{/if}
								</span>
								<span class="text-xs text-muted-foreground">{well.curve_count} curves</span>
							</button>
						</li>
					{/each}
				</ul>
			{/if}
		</div>

		<!-- Curve list (when well is selected) -->
		{#if selectedWell}
			<div class="border-border border-t">
				<div class="border-border flex items-center justify-between border-b p-3">
					<h2 class="text-sm font-semibold">Curves ({curves.length})</h2>
				</div>
				<div class="max-h-64 overflow-auto">
					{#if isLoadingCurves}
						<div class="flex items-center justify-center p-4">
							<svg
								class="h-5 w-5 animate-spin text-muted-foreground"
								fill="none"
								viewBox="0 0 24 24"
							>
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
					{:else if curves.length === 0}
						<div class="p-4 text-sm text-muted-foreground">No curves found</div>
					{:else}
						<ul class="divide-y divide-border">
							{#each curves as curve}
								<li class="flex items-center gap-2 px-3 py-2 text-sm">
									<span class="flex flex-1 flex-col items-start">
										<span class="font-medium">{curve.mnemonic}</span>
										<span class="text-xs text-muted-foreground">
											{curve.sample_count} pts
											{#if curve.unit}
												| {curve.unit}
											{/if}
										</span>
									</span>
								</li>
							{/each}
						</ul>
					{/if}
				</div>
			</div>
		{/if}
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

		{#if !selectedWell}
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
							d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
						/>
					</svg>
					<h3 class="mt-4 text-lg font-medium">Curve Viewer</h3>
					<p class="mt-1 text-sm text-muted-foreground">
						Select a well from the sidebar to view its curves
					</p>
				</div>
			</div>
		{:else if viewMode === 'coverage'}
			<!-- Data Coverage Chart -->
			<div class="border-border flex items-center justify-between border-b px-4 py-2">
				<div>
					<h2 class="font-semibold">{selectedWell.name}</h2>
					<p class="text-muted-foreground text-xs">Data Coverage</p>
				</div>
				<div class="flex items-center gap-2">
					<button
						onclick={() => (viewMode = 'data')}
						class="border-border flex items-center gap-1 rounded border px-2 py-1 text-xs hover:bg-secondary"
					>
						<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M3 10h18M3 14h18m-9-4v8m-7 0h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"
							/>
						</svg>
						View Data Table
					</button>
				</div>
			</div>
			<div class="flex-1 overflow-auto p-4">
				<DataCoverageChart wellId={selectedWell.id} height={500} />
			</div>
		{:else}
			<!-- Curve data header -->
			<div class="flex items-center justify-between border-b border-border px-4 py-2">
				<div>
					<h2 class="font-semibold">{selectedWell.name}</h2>
					{#if curveData}
						<p class="text-xs text-muted-foreground">
							{curveData.total_rows} rows, {curveData.columns.length} columns
							{#if selectedRowCount > 0}
								<span class="text-primary">• {selectedRowCount} selected</span>
							{/if}
							{#if hasUnsavedChanges}
								<span class="ml-2 text-amber-500">
									({pendingChangesCount} unsaved change{pendingChangesCount !== 1 ? 's' : ''})
								</span>
							{/if}
						</p>
					{/if}
				</div>

				<div class="flex items-center gap-2">
					<!-- Save/Discard buttons (when editing) -->
					{#if hasUnsavedChanges}
						<button
							onclick={discardChanges}
							disabled={isSaving}
							class="flex items-center gap-1 rounded border border-border px-2 py-1 text-xs hover:bg-secondary disabled:opacity-50"
							title="Discard all changes"
						>
							<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M6 18L18 6M6 6l12 12"
								/>
							</svg>
							Discard
						</button>
						<button
							onclick={saveChanges}
							disabled={isSaving}
							class="flex items-center gap-1 rounded bg-primary px-2 py-1 text-xs text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
							title="Save all changes"
						>
							{#if isSaving}
								<svg class="h-3 w-3 animate-spin" fill="none" viewBox="0 0 24 24">
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
								Saving...
							{:else}
								<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M5 13l4 4L19 7"
									/>
								</svg>
								Save ({pendingChangesCount})
							{/if}
						</button>
					{/if}

					<!-- Insert at Depth button -->
					{#if gridRowData.length > 0}
						<button
							onclick={openDepthModal}
							disabled={isSaving}
							class="flex items-center gap-1 rounded border border-border px-2 py-1 text-xs hover:bg-secondary disabled:opacity-50"
							title="Insert a new row at a specific depth"
						>
							<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M12 4v16m8-8H4"
								/>
							</svg>
							Insert at Depth
						</button>

						<!-- Delete Selected button -->
						<button
							onclick={deleteSelectedRows}
							disabled={isSaving || selectedRowCount === 0}
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
							Delete ({selectedRowCount})
						</button>
					{/if}

					<button
						onclick={() => (viewMode = 'coverage')}
						class="flex items-center gap-1 rounded border border-border px-2 py-1 text-xs hover:bg-secondary"
						title="Show data coverage chart"
					>
						<svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
							/>
						</svg>
						Coverage
					</button>

					<button
						onclick={loadCurveData}
						disabled={isLoadingData || hasUnsavedChanges}
						class="flex items-center gap-1 rounded border border-border px-2 py-1 text-xs hover:bg-secondary disabled:opacity-50"
						title={hasUnsavedChanges ? 'Save or discard changes before refreshing' : 'Refresh data'}
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
						editable={true}
						multiSelect={true}
						enableKeyboardDelete={true}
						{getRowId}
						onCellValueChanged={handleCellValueChanged}
						onSelectionChanged={handleSelectionChanged}
						onRowsDeleted={handleRowsDeleted}
						onRowsAdded={handleRowsAdded}
					/>
				{:else if curveData}
					<div class="flex h-32 items-center justify-center text-muted-foreground">
						No data available for selected curves
					</div>
				{:else}
					<div class="flex h-32 items-center justify-center text-muted-foreground">
						Click "View" to load curve data
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>

<!-- Depth Input Modal -->
{#if showDepthModal}
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
		onclick={cancelDepthInput}
		onkeydown={(e) => e.key === 'Escape' && cancelDepthInput()}
		role="dialog"
		aria-modal="true"
		aria-labelledby="depth-modal-title"
		tabindex="-1"
	>
		<div
			class="w-80 rounded-lg border border-border bg-background p-4 shadow-lg"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="presentation"
		>
			<h3 id="depth-modal-title" class="mb-3 text-sm font-semibold">Insert Row at Depth</h3>
			<div class="mb-3">
				<label for="depth-input" class="mb-1 block text-xs text-muted-foreground">
					Enter depth value:
				</label>
				<input
					id="depth-input"
					type="number"
					step="any"
					bind:this={depthInputRef}
					bind:value={depthInputValue}
					onkeydown={handleDepthInputKeydown}
					class="w-full rounded border border-border bg-background px-3 py-2 text-sm focus:border-primary focus:outline-none focus:ring-1 focus:ring-primary"
					placeholder="e.g., 1500.5"
				/>
				{#if depthInputError}
					<p class="mt-1 text-xs text-destructive">{depthInputError}</p>
				{/if}
			</div>
			<div class="flex justify-end gap-2">
				<button
					onclick={cancelDepthInput}
					class="rounded border border-border px-3 py-1.5 text-xs hover:bg-secondary"
				>
					Cancel
				</button>
				<button
					onclick={submitDepthInput}
					class="rounded bg-primary px-3 py-1.5 text-xs text-primary-foreground hover:bg-primary/90"
				>
					Insert Row
				</button>
			</div>
		</div>
	</div>
{/if}
