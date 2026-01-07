<script lang="ts">
	import { goto } from '$app/navigation'
	import { invoke } from '@tauri-apps/api/core'
	import { ingestStore } from '$lib/stores/ingest.svelte'
	import type { WellSummary, MainCurveType } from '$lib/types/ingest'
	import type { TrajectoryColumnType, SurveyType, AzimuthReference } from '$lib/types/trajectory'
	import { getColumnDisplayName, getSurveyTypeDisplayName } from '$lib/types/trajectory'
	import type { MarkerColumnType, InterpretationType, ConfidenceLevel } from '$lib/types/markers'
	import { getMarkerColumnDisplayName, getInterpretationTypeDisplayName } from '$lib/types/markers'
	import type { SurfaceColumnType, SurfaceType, ZPositiveDirection } from '$lib/types/surfaces'
	import { getSurfaceColumnDisplayName, getSurfaceTypeDisplayName, SURFACE_TYPE_OPTIONS, XY_UNIT_OPTIONS, Z_UNIT_OPTIONS, Z_DIRECTION_OPTIONS, COMMON_CRS_OPTIONS } from '$lib/types/surfaces'
	import type { CheckshotColumnType, CheckshotSurveyType } from '$lib/types/checkshots'
	import { getCheckColumnDisplayName, getCheckSurveyTypeDisplayName, CHECKSHOT_SURVEY_TYPE_OPTIONS, DEPTH_UNIT_OPTIONS, TIME_UNIT_OPTIONS, DATUM_OPTIONS } from '$lib/types/checkshots'

	// Standard curve types for dropdown
	const CURVE_TYPES: { value: MainCurveType; label: string; description: string }[] = [
		{ value: 'GR', label: 'GR', description: 'Gamma Ray' },
		{ value: 'RT', label: 'RT', description: 'Resistivity' },
		{ value: 'RHOB', label: 'RHOB', description: 'Bulk Density' },
		{ value: 'NPHI', label: 'NPHI', description: 'Neutron Porosity' },
		{ value: 'CALI', label: 'CALI', description: 'Caliper' },
		{ value: 'DT', label: 'DT', description: 'Sonic' },
		{ value: 'SP', label: 'SP', description: 'Spontaneous Potential' },
		{ value: 'PE', label: 'PE', description: 'Photoelectric' },
		{ value: 'DEPTH', label: 'DEPTH', description: 'Depth Index' },
		{ value: 'OTHER', label: 'OTHER', description: 'Other/Unknown' }
	]

	// Trajectory column types for dropdown (matches Rust snake_case serialization)
	const TRAJECTORY_COLUMN_TYPES: { value: TrajectoryColumnType; label: string }[] = [
		{ value: 'measured_depth', label: 'Measured Depth (MD)' },
		{ value: 'inclination', label: 'Inclination (INC)' },
		{ value: 'azimuth', label: 'Azimuth (AZI)' },
		{ value: 'true_vertical_depth', label: 'True Vertical Depth (TVD)' },
		{ value: 'north_south', label: 'North-South (NS)' },
		{ value: 'east_west', label: 'East-West (EW)' },
		{ value: 'dogleg_severity', label: 'Dogleg Severity (DLS)' },
		{ value: 'unknown', label: 'Unknown' }
	]

	// Survey types for dropdown
	const SURVEY_TYPES: { value: SurveyType; label: string }[] = [
		{ value: 'definitive', label: 'Definitive' },
		{ value: 'mwd', label: 'MWD (Measurement While Drilling)' },
		{ value: 'gyro', label: 'Gyroscopic' },
		{ value: 'preliminary', label: 'Preliminary' },
		{ value: 'composite', label: 'Composite' }
	]

	// Marker column types for dropdown (matches Rust snake_case serialization)
	const MARKER_COLUMN_TYPES: { value: MarkerColumnType; label: string }[] = [
		{ value: 'well_name', label: 'Well Name' },
		{ value: 'well_uwi', label: 'Well UWI/API' },
		{ value: 'marker_name', label: 'Marker Name' },
		{ value: 'measured_depth', label: 'Measured Depth' },
		{ value: 'true_vertical_depth', label: 'True Vertical Depth' },
		{ value: 'true_vertical_depth_ss', label: 'TVD Sub-Sea' },
		{ value: 'marker_type', label: 'Marker Type' },
		{ value: 'thickness', label: 'Thickness' },
		{ value: 'quality', label: 'Quality' },
		{ value: 'interpreter', label: 'Interpreter' },
		{ value: 'pick_date', label: 'Pick Date' },
		{ value: 'comments', label: 'Comments' },
		{ value: 'color', label: 'Color' },
		{ value: 'unknown', label: 'Unknown' }
	]

	// Interpretation types for dropdown
	const INTERPRETATION_TYPES: { value: InterpretationType; label: string }[] = [
		{ value: 'formation', label: 'Formation Tops' },
		{ value: 'sequence', label: 'Sequence Boundaries' },
		{ value: 'zone', label: 'Zone Boundaries' },
		{ value: 'horizon', label: 'Seismic Horizons' },
		{ value: 'other', label: 'Other' }
	]

	// Surface column types for dropdown
	const SURFACE_COLUMN_TYPES: { value: SurfaceColumnType; label: string }[] = [
		{ value: 'x', label: 'X (Easting)' },
		{ value: 'y', label: 'Y (Northing)' },
		{ value: 'z', label: 'Z (Depth/Elevation)' },
		{ value: 'surface_name', label: 'Surface Name' },
		{ value: 'quality', label: 'Quality' },
		{ value: 'attribute', label: 'Attribute' },
		{ value: 'unknown', label: 'Unknown' }
	]

	// Checkshot column types for dropdown (matches Rust snake_case serialization)
	const CHECKSHOT_COLUMN_TYPES: { value: CheckshotColumnType; label: string }[] = [
		{ value: 'measured_depth', label: 'Measured Depth (MD)' },
		{ value: 'true_vertical_depth', label: 'True Vertical Depth (TVD)' },
		{ value: 'two_way_time', label: 'Two-Way Time (TWT)' },
		{ value: 'velocity', label: 'Interval Velocity' },
		{ value: 'one_way_time', label: 'One-Way Time (OWT)' },
		{ value: 'quality', label: 'Quality' },
		{ value: 'unknown', label: 'Unknown' }
	]

	// Check if we're in trajectory mode
	const isTrajectoryMode = $derived(ingestStore.sourceType === 'trajectory_csv')

	// Check if we're in markers mode
	const isMarkersMode = $derived(ingestStore.sourceType === 'markers_csv')

	// Check if we're in surfaces mode
	const isSurfacesMode = $derived(ingestStore.sourceType === 'surface_csv')

	// Check if we're in checkshots mode
	const isCheckshotsMode = $derived(ingestStore.sourceType === 'checkshot_csv')

	// Well selection state
	let existingWells = $state<WellSummary[]>([])
	let wellSelectionMode = $state<'existing' | 'new'>('new')
	let selectedWellId = $state<string | null>(null)
	let newWellName = $state('')
	let isLoadingWells = $state(true)

	// Load existing wells on mount
	$effect(() => {
		loadWells()
	})

	async function loadWells() {
		isLoadingWells = true
		try {
			existingWells = await invoke<WellSummary[]>('get_workspace_wells')
		} catch (e) {
			console.error('Failed to load wells:', e)
			existingWells = []
		} finally {
			isLoadingWells = false
		}
	}

	// Redirect if no files uploaded, otherwise set step
	$effect(() => {
		if (ingestStore.files.length === 0) {
			goto('/ingest', { replaceState: true })
		} else {
			ingestStore.goToStep('mapping')
		}
	})

	// Sync well selection with store when selection changes
	function updateWellSelection() {
		if (wellSelectionMode === 'existing' && selectedWellId) {
			ingestStore.setTargetWell(selectedWellId, false)
		} else if (wellSelectionMode === 'new') {
			// For trajectory and checkshot, use file name as default well name if not specified
			const defaultName = isTrajectoryMode || isCheckshotsMode
				? ingestStore.files[0]?.name?.replace(/\.[^/.]+$/, '') || ''
				: ingestStore.files[0]?.lasMetadata?.wellName || ''
			const name = newWellName || defaultName
			ingestStore.setTargetWell(null, true, name || undefined)
		}
	}

	// Trajectory-specific: toggle column include
	function toggleTrajectoryColumnInclude(fileId: string, columnIndex: number) {
		const file = ingestStore.files.find((f) => f.id === fileId)
		const col = file?.trajectoryMetadata?.columns.find((c) => c.index === columnIndex)
		if (col) {
			ingestStore.updateTrajectoryColumnMapping(fileId, columnIndex, { include: !col.include })
		}
	}

	// Trajectory-specific: set column type
	function setTrajectoryColumnType(fileId: string, columnIndex: number, type: string) {
		ingestStore.updateTrajectoryColumnMapping(fileId, columnIndex, { mappedType: type })
	}

	// Get trajectory metadata stats
	function getTrajectoryStats() {
		return ingestStore.files.reduce(
			(stats, file) => {
				const meta = file.trajectoryMetadata
				if (meta) {
					stats.stationCount += meta.rowCount
					stats.columnCount += meta.columns.filter((c) => c.include).length
				}
				return stats
			},
			{ stationCount: 0, columnCount: 0 }
		)
	}

	// Marker-specific: toggle column include
	function toggleMarkerColumnInclude(fileId: string, columnIndex: number) {
		const file = ingestStore.files.find((f) => f.id === fileId)
		const col = file?.markerMetadata?.columns.find((c) => c.index === columnIndex)
		if (col) {
			ingestStore.updateMarkerColumnMapping(fileId, columnIndex, { include: !col.include })
		}
	}

	// Marker-specific: set column type
	function setMarkerColumnType(fileId: string, columnIndex: number, type: MarkerColumnType) {
		ingestStore.updateMarkerColumnMapping(fileId, columnIndex, { mappedType: type })
	}

	// Get marker metadata stats
	function getMarkerStats() {
		return ingestStore.files.reduce(
			(stats, file) => {
				const meta = file.markerMetadata
				if (meta) {
					stats.markerCount += meta.rowCount
					stats.wellCount += meta.wellNames.length
					stats.columnCount += meta.columns.filter((c) => c.include).length
				}
				return stats
			},
			{ markerCount: 0, wellCount: 0, columnCount: 0 }
		)
	}

	// Checkshot-specific: toggle column include
	function toggleCheckColumnInclude(fileId: string, columnIndex: number) {
		const file = ingestStore.files.find((f) => f.id === fileId)
		const col = file?.checkMetadata?.columns.find((c) => c.index === columnIndex)
		if (col) {
			ingestStore.updateCheckColumnMapping(fileId, columnIndex, { include: !col.include })
		}
	}

	// Checkshot-specific: set column type
	function setCheckColumnType(fileId: string, columnIndex: number, type: CheckshotColumnType) {
		ingestStore.updateCheckColumnMapping(fileId, columnIndex, { mappedType: type })
	}

	// Get checkshot metadata stats
	function getCheckStats() {
		return ingestStore.files.reduce(
			(stats, file) => {
				const meta = file.checkMetadata
				if (meta) {
					stats.stationCount += meta.rowCount
					stats.columnCount += meta.columns.filter((c) => c.include).length
				}
				return stats
			},
			{ stationCount: 0, columnCount: 0 }
		)
	}

	// Common curve mnemonics for suggestions
	const commonMnemonics: Record<string, { name: string; unit: string; description: string }> = {
		DEPT: { name: 'DEPT', unit: 'M', description: 'Measured Depth' },
		GR: { name: 'GR', unit: 'API', description: 'Gamma Ray' },
		RHOB: { name: 'RHOB', unit: 'G/C3', description: 'Bulk Density' },
		NPHI: { name: 'NPHI', unit: 'V/V', description: 'Neutron Porosity' },
		DT: { name: 'DT', unit: 'US/F', description: 'Sonic Transit Time' },
		RT: { name: 'RT', unit: 'OHMM', description: 'Deep Resistivity' },
		CALI: { name: 'CALI', unit: 'IN', description: 'Caliper' },
		SP: { name: 'SP', unit: 'MV', description: 'Spontaneous Potential' },
		PE: { name: 'PE', unit: 'B/E', description: 'Photoelectric Factor' }
	}

	// Currently expanded file (for accordion)
	let expandedFileId = $state<string | null>(ingestStore.files[0]?.id ?? null)

	function toggleFile(fileId: string) {
		expandedFileId = expandedFileId === fileId ? null : fileId
	}

	function toggleCurveInclude(fileId: string, curveIndex: number) {
		ingestStore.updateCurveMapping(fileId, curveIndex, {
			include: !ingestStore.files.find((f) => f.id === fileId)?.lasMetadata?.curves[curveIndex]
				?.include
		})
	}

	function setCurveType(fileId: string, curveIndex: number, type: MainCurveType) {
		ingestStore.updateCurveMapping(fileId, curveIndex, { mappedType: type })
	}

	function handleNext() {
		updateWellSelection()
		ingestStore.nextStep()
		goto('/ingest/confirm')
	}

	function handleBack() {
		ingestStore.previousStep()
		goto('/ingest/upload')
	}

	// Count included curves across all files
	function getIncludedCurvesCount(): number {
		return ingestStore.files.reduce((total, file) => {
			const included = file.lasMetadata?.curves.filter((c) => c.include).length ?? 0
			return total + included
		}, 0)
	}
</script>

<div class="space-y-6">
	<div>
		<h2 class="text-lg font-medium">
			{#if isMarkersMode}
				Map Marker Data
			{:else if isTrajectoryMode}
				Map Trajectory Data
			{:else if isCheckshotsMode}
				Map Checkshot Data
			{:else}
				Map Data
			{/if}
		</h2>
		<p class="text-muted-foreground mt-1 text-sm">
			{#if isMarkersMode}
				Review columns, map wells, and configure marker set settings.
			{:else if isTrajectoryMode}
				Select a target well, review columns, and configure survey metadata.
			{:else if isCheckshotsMode}
				Select a target well, review columns, and configure time-depth settings.
			{:else}
				Select a target well, review curves, and configure how your data will be imported.
			{/if}
		</p>
	</div>

	<!-- Well Selection (not shown for markers - they have their own well mapping) -->
	{#if !isMarkersMode}
		<div class="rounded-lg border p-4">
			<h3 class="mb-3 text-sm font-medium">Target Well</h3>

			<!-- Mode selector -->
			<div class="mb-4 flex gap-4">
				<label class="flex items-center gap-2">
					<input
						type="radio"
						name="wellMode"
						value="new"
						checked={wellSelectionMode === 'new'}
						onchange={() => {
							wellSelectionMode = 'new'
							selectedWellId = null
						}}
						class="accent-primary h-4 w-4"
					/>
					<span class="text-sm">Create new well</span>
				</label>
				<label class="flex items-center gap-2">
					<input
						type="radio"
						name="wellMode"
						value="existing"
						checked={wellSelectionMode === 'existing'}
						onchange={() => (wellSelectionMode = 'existing')}
						class="accent-primary h-4 w-4"
						disabled={existingWells.length === 0}
					/>
					<span class="text-sm {existingWells.length === 0 ? 'text-muted-foreground' : ''}"
						>Add to existing well
						{#if existingWells.length === 0}
							<span class="text-xs">(no wells)</span>
						{/if}
					</span>
				</label>
			</div>

			{#if wellSelectionMode === 'new'}
				<!-- New well name input -->
				<div>
					<label for="newWellName" class="text-muted-foreground mb-1 block text-xs">Well Name</label>
					<input
						id="newWellName"
						type="text"
						bind:value={newWellName}
						placeholder={isTrajectoryMode || isCheckshotsMode
							? ingestStore.files[0]?.name?.replace(/\.[^/.]+$/, '') || 'Enter well name'
							: ingestStore.files[0]?.lasMetadata?.wellName || 'Enter well name'}
						class="border-border bg-background focus:border-primary w-full rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
					/>
					<p class="text-muted-foreground mt-1 text-xs">
						{#if isTrajectoryMode || isCheckshotsMode}
							Leave blank to use the file name
						{:else}
							Leave blank to use the well name from the LAS file
						{/if}
					</p>
				</div>
			{:else}
				<!-- Existing well selection -->
				{#if isLoadingWells}
					<div class="flex items-center gap-2 text-sm text-muted-foreground">
						<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
							<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
							></circle>
							<path
								class="opacity-75"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							></path>
						</svg>
						Loading wells...
					</div>
				{:else}
					<select
						bind:value={selectedWellId}
						class="border-border bg-background focus:border-primary w-full rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
					>
						<option value={null}>Select a well...</option>
						{#each existingWells as well}
							<option value={well.id}>
								{well.name}
								{#if well.uwi}({well.uwi}){/if}
								- {well.curve_count} curves
							</option>
						{/each}
					</select>
				{/if}
			{/if}
		</div>
	{/if}

	<!-- Summary stats -->
	{#if isMarkersMode}
		{@const stats = getMarkerStats()}
		<div class="bg-muted/30 grid grid-cols-4 gap-4 rounded-lg border p-4">
			<div class="text-center">
				<p class="text-2xl font-semibold">{ingestStore.files.length}</p>
				<p class="text-muted-foreground text-sm">Files</p>
			</div>
			<div class="text-center">
				<p class="text-2xl font-semibold">{stats.markerCount}</p>
				<p class="text-muted-foreground text-sm">Markers</p>
			</div>
			<div class="text-center">
				<p class="text-2xl font-semibold">{stats.wellCount}</p>
				<p class="text-muted-foreground text-sm">Wells</p>
			</div>
			<div class="text-center">
				<p class="text-2xl font-semibold">{stats.columnCount}</p>
				<p class="text-muted-foreground text-sm">Columns</p>
			</div>
		</div>
	{:else if isTrajectoryMode}
		{@const stats = getTrajectoryStats()}
		<div class="bg-muted/30 grid grid-cols-3 gap-4 rounded-lg border p-4">
			<div class="text-center">
				<p class="text-2xl font-semibold">{ingestStore.files.length}</p>
				<p class="text-muted-foreground text-sm">Files</p>
			</div>
			<div class="text-center">
				<p class="text-2xl font-semibold">{stats.stationCount}</p>
				<p class="text-muted-foreground text-sm">Stations</p>
			</div>
			<div class="text-center">
				<p class="text-2xl font-semibold">{stats.columnCount}</p>
				<p class="text-muted-foreground text-sm">Columns Selected</p>
			</div>
		</div>
	{:else if isCheckshotsMode}
		{@const stats = getCheckStats()}
		<div class="bg-muted/30 grid grid-cols-3 gap-4 rounded-lg border p-4">
			<div class="text-center">
				<p class="text-2xl font-semibold">{ingestStore.files.length}</p>
				<p class="text-muted-foreground text-sm">Files</p>
			</div>
			<div class="text-center">
				<p class="text-2xl font-semibold">{stats.stationCount}</p>
				<p class="text-muted-foreground text-sm">Stations</p>
			</div>
			<div class="text-center">
				<p class="text-2xl font-semibold">{stats.columnCount}</p>
				<p class="text-muted-foreground text-sm">Columns Selected</p>
			</div>
		</div>
	{:else}
		<div class="bg-muted/30 grid grid-cols-3 gap-4 rounded-lg border p-4">
			<div class="text-center">
				<p class="text-2xl font-semibold">{ingestStore.files.length}</p>
				<p class="text-muted-foreground text-sm">Files</p>
			</div>
			<div class="text-center">
				<p class="text-2xl font-semibold">{getIncludedCurvesCount()}</p>
				<p class="text-muted-foreground text-sm">Curves Selected</p>
			</div>
			<div class="text-center">
				<p class="text-2xl font-semibold">
					{ingestStore.files.reduce((sum, f) => sum + (f.lasMetadata?.curves.length ?? 0), 0)}
				</p>
				<p class="text-muted-foreground text-sm">Total Curves</p>
			</div>
		</div>
	{/if}

	<!-- File accordion -->
	<div class="space-y-2">
		{#each ingestStore.files as file}
			<div class="rounded-lg border">
				<!-- File header -->
				<button
					onclick={() => toggleFile(file.id)}
					class="hover:bg-secondary/50 flex w-full items-center justify-between p-4 text-left transition-colors"
				>
					<div class="flex items-center gap-3">
						<svg
							class="text-muted-foreground h-5 w-5"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
							/>
						</svg>
						<div>
							<p class="font-medium">{file.name}</p>
							{#if file.lasMetadata}
								<p class="text-muted-foreground text-sm">
									{file.lasMetadata.wellName} - {file.lasMetadata.curves.filter((c) => c.include)
										.length} of {file.lasMetadata.curves.length} curves selected
								</p>
							{:else if file.trajectoryMetadata}
								<p class="text-muted-foreground text-sm">
									{file.trajectoryMetadata.rowCount} stations, {file.trajectoryMetadata.columns.filter((c) => c.include).length} columns selected
									{#if !file.trajectoryMetadata.hasRequiredColumns}
										<span class="text-destructive"> - Missing required columns</span>
									{/if}
								</p>
							{:else if file.markerMetadata}
								<p class="text-muted-foreground text-sm">
									{file.markerMetadata.rowCount} markers
									{#if file.markerMetadata.isMultiWell}
										from {file.markerMetadata.wellNames.length} wells
									{/if}
									{#if !file.markerMetadata.hasRequiredColumns}
										<span class="text-destructive"> - Missing required columns</span>
									{/if}
								</p>
							{:else if file.checkMetadata}
								<p class="text-muted-foreground text-sm">
									{file.checkMetadata.rowCount} stations, {file.checkMetadata.columns.filter((c) => c.include).length} columns selected
									{#if !file.checkMetadata.hasRequiredColumns}
										<span class="text-destructive"> - Missing required columns</span>
									{/if}
								</p>
							{/if}
						</div>
					</div>
					<svg
						class="h-5 w-5 transition-transform {expandedFileId === file.id ? 'rotate-180' : ''}"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
					</svg>
				</button>

				<!-- Expanded content for LAS files -->
				{#if expandedFileId === file.id && file.lasMetadata}
					<div class="border-t px-4 pb-4">
						<!-- Well metadata -->
						<div class="bg-muted/30 mt-4 grid grid-cols-2 gap-4 rounded-lg p-4 sm:grid-cols-4">
							<div>
								<p class="text-muted-foreground text-xs uppercase">Well Name</p>
								<p class="font-medium">{file.lasMetadata.wellName}</p>
							</div>
							<div>
								<p class="text-muted-foreground text-xs uppercase">UWI</p>
								<p class="font-medium">{file.lasMetadata.uwi || '-'}</p>
							</div>
							<div>
								<p class="text-muted-foreground text-xs uppercase">Depth Range</p>
								<p class="font-medium">
									{file.lasMetadata.startDepth} - {file.lasMetadata.stopDepth}
									{file.lasMetadata.depthUnit}
								</p>
							</div>
							<div>
								<p class="text-muted-foreground text-xs uppercase">Step</p>
								<p class="font-medium">{file.lasMetadata.step} {file.lasMetadata.depthUnit}</p>
							</div>
						</div>

						<!-- Curve selection table -->
						<div class="mt-4">
							<h4 class="mb-2 text-sm font-medium">Curves</h4>
							<div class="rounded-lg border overflow-x-auto">
								<table class="w-full text-sm">
									<thead>
										<tr class="border-b bg-muted/30">
											<th class="w-12 px-4 py-2 text-left">
												<span class="sr-only">Include</span>
											</th>
											<th class="px-4 py-2 text-left font-medium">Mnemonic</th>
											<th class="px-4 py-2 text-left font-medium">Unit</th>
											<th class="px-4 py-2 text-left font-medium">Curve Type</th>
											<th class="px-4 py-2 text-left font-medium">Description</th>
										</tr>
									</thead>
									<tbody class="divide-y">
										{#each file.lasMetadata.curves as curve, index}
											<tr class="hover:bg-muted/20">
												<td class="px-4 py-2">
													<input
														type="checkbox"
														checked={curve.include}
														onchange={() => toggleCurveInclude(file.id, index)}
														class="accent-primary h-4 w-4 rounded"
													/>
												</td>
												<td class="px-4 py-2 font-mono text-xs">{curve.mnemonic}</td>
												<td class="px-4 py-2 font-mono text-xs">{curve.unit}</td>
												<td class="px-4 py-2">
													<div class="flex items-center gap-2">
														<select
															value={curve.mappedType || curve.detectedType}
															onchange={(e) =>
																setCurveType(
																	file.id,
																	index,
																	(e.target as HTMLSelectElement).value as MainCurveType
																)}
															class="border-border bg-background focus:border-primary rounded border px-2 py-1 text-xs focus:outline-none focus:ring-1 focus:ring-primary/50"
														>
															{#each CURVE_TYPES as type}
																<option value={type.value}>{type.label}</option>
															{/each}
														</select>
														{#if curve.mappedType && curve.mappedType !== curve.detectedType}
															<span class="text-xs text-blue-500" title="Manually mapped">
																(mapped)
															</span>
														{/if}
													</div>
												</td>
												<td class="text-muted-foreground px-4 py-2 text-xs">{curve.description}</td>
											</tr>
										{/each}
									</tbody>
								</table>
							</div>
						</div>
					</div>
				{/if}

				<!-- Expanded content for Trajectory files -->
				{#if expandedFileId === file.id && file.trajectoryMetadata}
					<div class="border-t px-4 pb-4">
						<!-- Survey metadata -->
						<div class="bg-muted/30 mt-4 grid grid-cols-2 gap-4 rounded-lg p-4 sm:grid-cols-4">
							<div>
								<p class="text-muted-foreground text-xs uppercase">Stations</p>
								<p class="font-medium">{file.trajectoryMetadata.rowCount}</p>
							</div>
							<div>
								<p class="text-muted-foreground text-xs uppercase">Delimiter</p>
								<p class="font-medium">{file.trajectoryMetadata.delimiter}</p>
							</div>
							<div>
								<p class="text-muted-foreground text-xs uppercase">Columns</p>
								<p class="font-medium">{file.trajectoryMetadata.columns.length}</p>
							</div>
							<div>
								<p class="text-muted-foreground text-xs uppercase">Status</p>
								<p class="font-medium {file.trajectoryMetadata.hasRequiredColumns ? 'text-green-500' : 'text-destructive'}">
									{file.trajectoryMetadata.hasRequiredColumns ? 'Ready' : 'Missing columns'}
								</p>
							</div>
						</div>

						<!-- Survey Settings (OSDU WellboreTrajectory fields) -->
						<div class="mt-4">
							<h4 class="mb-2 text-sm font-medium">Survey Settings</h4>
							<div class="grid grid-cols-2 gap-4 sm:grid-cols-4">
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Survey Type</label>
									<select
										value={file.trajectoryMetadata.surveyType || ''}
										onchange={(e) =>
											ingestStore.updateTrajectorySurveyMetadata(file.id, {
												surveyType: (e.target as HTMLSelectElement).value as SurveyType || undefined
											})}
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									>
										<option value="">Not specified</option>
										{#each SURVEY_TYPES as st}
											<option value={st.value}>{st.label}</option>
										{/each}
									</select>
								</div>
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Azimuth Reference</label>
									<select
										value={file.trajectoryMetadata.azimuthReference || ''}
										onchange={(e) =>
											ingestStore.updateTrajectorySurveyMetadata(file.id, {
												azimuthReference: (e.target as HTMLSelectElement).value as AzimuthReference || undefined
											})}
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									>
										<option value="">Not specified</option>
										<option value="true_north">True North</option>
										<option value="grid_north">Grid North</option>
										<option value="magnetic_north">Magnetic North</option>
									</select>
								</div>
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Depth Unit</label>
									<select
										value={file.trajectoryMetadata.mdUnit}
										onchange={(e) =>
											ingestStore.updateTrajectorySurveyMetadata(file.id, {
												mdUnit: (e.target as HTMLSelectElement).value
											})}
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									>
										<option value="ft">Feet (ft)</option>
										<option value="m">Meters (m)</option>
									</select>
								</div>
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Angle Unit</label>
									<select
										value={file.trajectoryMetadata.angleUnit}
										onchange={(e) =>
											ingestStore.updateTrajectorySurveyMetadata(file.id, {
												angleUnit: (e.target as HTMLSelectElement).value
											})}
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									>
										<option value="deg">Degrees (deg)</option>
										<option value="rad">Radians (rad)</option>
									</select>
								</div>
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Survey Company</label>
									<input
										type="text"
										value={file.trajectoryMetadata.surveyCompany || ''}
										onchange={(e) =>
											ingestStore.updateTrajectorySurveyMetadata(file.id, {
												surveyCompany: (e.target as HTMLInputElement).value || undefined
											})}
										placeholder="Optional"
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									/>
								</div>
							</div>
						</div>

						<!-- Column mapping table -->
						<div class="mt-4">
							<h4 class="mb-2 text-sm font-medium">Columns</h4>
							<div class="rounded-lg border overflow-x-auto">
								<table class="w-full text-sm">
									<thead>
										<tr class="border-b bg-muted/30">
											<th class="w-12 px-4 py-2 text-left">
												<span class="sr-only">Include</span>
											</th>
											<th class="px-4 py-2 text-left font-medium">Header</th>
											<th class="px-4 py-2 text-left font-medium">Column Type</th>
											<th class="px-4 py-2 text-left font-medium">Unit</th>
											<th class="px-4 py-2 text-left font-medium">Range</th>
										</tr>
									</thead>
									<tbody class="divide-y">
										{#each file.trajectoryMetadata.columns as col}
											<tr class="hover:bg-muted/20 {col.isInput ? 'bg-primary/5' : ''}">
												<td class="px-4 py-2">
													<input
														type="checkbox"
														checked={col.include}
														onchange={() => toggleTrajectoryColumnInclude(file.id, col.index)}
														class="accent-primary h-4 w-4 rounded"
													/>
												</td>
												<td class="px-4 py-2 font-mono text-xs">
													{col.header}
													{#if col.isInput}
														<span class="ml-1 rounded bg-primary/20 px-1 text-primary text-[10px]">INPUT</span>
													{/if}
												</td>
												<td class="px-4 py-2">
													<div class="flex items-center gap-2">
														<select
															value={col.mappedType || col.detectedType}
															onchange={(e) =>
																setTrajectoryColumnType(
																	file.id,
																	col.index,
																	(e.target as HTMLSelectElement).value
																)}
															class="border-border bg-background focus:border-primary rounded border px-2 py-1 text-xs focus:outline-none focus:ring-1 focus:ring-primary/50"
														>
															{#each TRAJECTORY_COLUMN_TYPES as type}
																<option value={type.value}>{type.label}</option>
															{/each}
														</select>
														{#if col.mappedType && col.mappedType !== col.detectedType}
															<span class="text-xs text-blue-500" title="Manually mapped">
																(mapped)
															</span>
														{/if}
													</div>
												</td>
												<td class="px-4 py-2 font-mono text-xs">{col.unit || '-'}</td>
												<td class="text-muted-foreground px-4 py-2 text-xs">
													{#if col.minValue !== null && col.maxValue !== null}
														{col.minValue.toFixed(2)} - {col.maxValue.toFixed(2)}
													{:else}
														-
													{/if}
												</td>
											</tr>
										{/each}
									</tbody>
								</table>
							</div>
						</div>

						<!-- Warnings -->
						{#if file.trajectoryMetadata.warnings.length > 0}
							<div class="mt-4 rounded-lg border border-yellow-500/50 bg-yellow-500/10 p-3">
								<h4 class="mb-2 text-sm font-medium text-yellow-600">Warnings</h4>
								<ul class="list-inside list-disc text-xs text-yellow-600">
									{#each file.trajectoryMetadata.warnings as warning}
										<li>{warning}</li>
									{/each}
								</ul>
							</div>
						{/if}
					</div>
				{/if}

				<!-- Expanded content for Marker files -->
				{#if expandedFileId === file.id && file.markerMetadata}
					<div class="border-t px-4 pb-4">
						<!-- File metadata -->
						<div class="bg-muted/30 mt-4 grid grid-cols-2 gap-4 rounded-lg p-4 sm:grid-cols-4">
							<div>
								<p class="text-muted-foreground text-xs uppercase">Markers</p>
								<p class="font-medium">{file.markerMetadata.rowCount}</p>
							</div>
							<div>
								<p class="text-muted-foreground text-xs uppercase">Delimiter</p>
								<p class="font-medium">{file.markerMetadata.delimiter}</p>
							</div>
							<div>
								<p class="text-muted-foreground text-xs uppercase">Wells</p>
								<p class="font-medium">{file.markerMetadata.isMultiWell ? file.markerMetadata.wellNames.length : 1}</p>
							</div>
							<div>
								<p class="text-muted-foreground text-xs uppercase">Status</p>
								<p class="font-medium {file.markerMetadata.hasRequiredColumns ? 'text-green-500' : 'text-destructive'}">
									{file.markerMetadata.hasRequiredColumns ? 'Ready' : 'Missing columns'}
								</p>
							</div>
						</div>

						<!-- Marker Set Settings -->
						<div class="mt-4">
							<h4 class="mb-2 text-sm font-medium">Marker Set Settings</h4>
							<div class="grid grid-cols-2 gap-4 sm:grid-cols-4">
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Set Name</label>
									<input
										type="text"
										value={file.markerMetadata.setName || ''}
										onchange={(e) =>
											ingestStore.updateMarkerSetMetadata(file.id, {
												setName: (e.target as HTMLInputElement).value || undefined
											})}
										placeholder="Formation Tops"
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									/>
								</div>
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Interpretation Type</label>
									<select
										value={file.markerMetadata.interpretationType || ''}
										onchange={(e) =>
											ingestStore.updateMarkerSetMetadata(file.id, {
												interpretationType: (e.target as HTMLSelectElement).value as InterpretationType || undefined
											})}
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									>
										<option value="">Not specified</option>
										{#each INTERPRETATION_TYPES as it}
											<option value={it.value}>{it.label}</option>
										{/each}
									</select>
								</div>
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Depth Unit</label>
									<select
										value={file.markerMetadata.depthUnit}
										onchange={(e) =>
											ingestStore.updateMarkerSetMetadata(file.id, {
												depthUnit: (e.target as HTMLSelectElement).value
											})}
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									>
										<option value="ft">Feet (ft)</option>
										<option value="m">Meters (m)</option>
									</select>
								</div>
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Interpreter</label>
									<input
										type="text"
										value={file.markerMetadata.interpreter || ''}
										onchange={(e) =>
											ingestStore.updateMarkerSetMetadata(file.id, {
												interpreter: (e.target as HTMLInputElement).value || undefined
											})}
										placeholder="Optional"
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									/>
								</div>
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Interpretation Date</label>
									<input
										type="date"
										value={file.markerMetadata.interpretationDate || ''}
										onchange={(e) =>
											ingestStore.updateMarkerSetMetadata(file.id, {
												interpretationDate: (e.target as HTMLInputElement).value || undefined
											})}
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									/>
								</div>
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Confidence Level</label>
									<select
										value={file.markerMetadata.confidenceLevel || ''}
										onchange={(e) =>
											ingestStore.updateMarkerSetMetadata(file.id, {
												confidenceLevel: (e.target as HTMLSelectElement).value as ConfidenceLevel || undefined
											})}
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									>
										<option value="">Not specified</option>
										<option value="High">High</option>
										<option value="Medium">Medium</option>
										<option value="Low">Low</option>
									</select>
								</div>
							</div>
						</div>

						<!-- Well Mapping Options -->
						{#if file.markerMetadata.isMultiWell && file.markerMetadata.wellNames.length > 0}
							<div class="mt-4">
								<div class="flex items-center justify-between mb-2">
									<h4 class="text-sm font-medium">Well Mapping</h4>
									<div class="flex items-center gap-4">
										<label class="flex items-center gap-2">
											<input
												type="checkbox"
												checked={file.markerMetadata.allowUnmappedWells}
												onchange={(e) =>
													ingestStore.updateMarkerSetMetadata(file.id, {
														allowUnmappedWells: (e.target as HTMLInputElement).checked
													})}
												class="accent-primary h-4 w-4 rounded"
											/>
											<span class="text-xs text-muted-foreground">Import without mapping (map later)</span>
										</label>
										{#if !file.markerMetadata.allowUnmappedWells}
											<label class="flex items-center gap-2">
												<input
													type="checkbox"
													checked={file.markerMetadata.autoCreateWells}
													onchange={(e) =>
														ingestStore.updateMarkerSetMetadata(file.id, {
															autoCreateWells: (e.target as HTMLInputElement).checked
														})}
													class="accent-primary h-4 w-4 rounded"
												/>
												<span class="text-xs text-muted-foreground">Auto-create unmatched wells</span>
											</label>
										{/if}
									</div>
								</div>

								{#if file.markerMetadata.allowUnmappedWells}
									<!-- Unmapped wells info box -->
									<div class="rounded-lg border border-blue-500/50 bg-blue-500/10 p-3">
										<p class="text-xs text-blue-600">
											<strong>Import without mapping:</strong> Markers will be imported with well names stored as text.
											You can map them to actual wells later from the Inspector.
											Found {file.markerMetadata.wellNames.length} unique well name{file.markerMetadata.wellNames.length === 1 ? '' : 's'}:
											<span class="font-mono">{file.markerMetadata.wellNames.slice(0, 5).join(', ')}{file.markerMetadata.wellNames.length > 5 ? '...' : ''}</span>
										</p>
									</div>
								{:else}
									<!-- Well mapping table -->
									<div class="rounded-lg border overflow-x-auto">
										<table class="w-full text-sm">
											<thead>
												<tr class="border-b bg-muted/30">
													<th class="px-4 py-2 text-left font-medium">Well Name (from file)</th>
													<th class="px-4 py-2 text-left font-medium">Match To</th>
													<th class="px-4 py-2 text-left font-medium">Status</th>
												</tr>
											</thead>
											<tbody class="divide-y">
												{#each file.markerMetadata.wellMappings as mapping}
													<tr class="hover:bg-muted/20">
														<td class="px-4 py-2 font-mono text-xs">{mapping.well_name}</td>
														<td class="px-4 py-2">
															<select
																value={mapping.well_id || ''}
																onchange={(e) => {
																	const value = (e.target as HTMLSelectElement).value
																	ingestStore.updateMarkerWellMapping(file.id, mapping.well_name, {
																		well_id: value || null,
																		create_new: !value && file.markerMetadata?.autoCreateWells
																	})
																}}
																class="border-border bg-background focus:border-primary rounded border px-2 py-1 text-xs focus:outline-none focus:ring-1 focus:ring-primary/50 min-w-[150px]"
															>
																<option value="">
																	{file.markerMetadata.autoCreateWells ? 'Create new well' : 'No match'}
																</option>
																{#each existingWells as well}
																	<option value={well.id}>
																		{well.name}
																		{#if well.uwi}({well.uwi}){/if}
																	</option>
																{/each}
															</select>
														</td>
														<td class="px-4 py-2 text-xs">
															{#if mapping.well_id}
																<span class="text-green-500">Matched</span>
															{:else if file.markerMetadata.autoCreateWells}
																<span class="text-blue-500">Will create</span>
															{:else}
																<span class="text-yellow-500">Unmatched</span>
															{/if}
														</td>
													</tr>
												{/each}
											</tbody>
										</table>
									</div>
								{/if}
							</div>
						{/if}

						<!-- Column mapping table -->
						<div class="mt-4">
							<h4 class="mb-2 text-sm font-medium">Columns</h4>
							<div class="rounded-lg border overflow-x-auto">
								<table class="w-full text-sm">
									<thead>
										<tr class="border-b bg-muted/30">
											<th class="w-12 px-4 py-2 text-left">
												<span class="sr-only">Include</span>
											</th>
											<th class="px-4 py-2 text-left font-medium">Header</th>
											<th class="px-4 py-2 text-left font-medium">Column Type</th>
											<th class="px-4 py-2 text-left font-medium">Unit</th>
											<th class="px-4 py-2 text-left font-medium">Sample Values</th>
										</tr>
									</thead>
									<tbody class="divide-y">
										{#each file.markerMetadata.columns as col}
											<tr class="hover:bg-muted/20 {col.detectedType === 'marker_name' || col.detectedType === 'measured_depth' ? 'bg-primary/5' : ''}">
												<td class="px-4 py-2">
													<input
														type="checkbox"
														checked={col.include}
														onchange={() => toggleMarkerColumnInclude(file.id, col.index)}
														class="accent-primary h-4 w-4 rounded"
													/>
												</td>
												<td class="px-4 py-2 font-mono text-xs">
													{col.header}
													{#if col.detectedType === 'marker_name' || col.detectedType === 'measured_depth'}
														<span class="ml-1 rounded bg-primary/20 px-1 text-primary text-[10px]">REQUIRED</span>
													{/if}
												</td>
												<td class="px-4 py-2">
													<div class="flex items-center gap-2">
														<select
															value={col.mappedType || col.detectedType}
															onchange={(e) =>
																setMarkerColumnType(
																	file.id,
																	col.index,
																	(e.target as HTMLSelectElement).value as MarkerColumnType
																)}
															class="border-border bg-background focus:border-primary rounded border px-2 py-1 text-xs focus:outline-none focus:ring-1 focus:ring-primary/50"
														>
															{#each MARKER_COLUMN_TYPES as type}
																<option value={type.value}>{type.label}</option>
															{/each}
														</select>
														{#if col.mappedType && col.mappedType !== col.detectedType}
															<span class="text-xs text-blue-500" title="Manually mapped">
																(mapped)
															</span>
														{/if}
													</div>
												</td>
												<td class="px-4 py-2 font-mono text-xs">{col.unit || '-'}</td>
												<td class="text-muted-foreground px-4 py-2 text-xs max-w-[200px] truncate">
													{col.sampleValues.slice(0, 3).join(', ')}
													{#if col.sampleValues.length > 3}...{/if}
												</td>
											</tr>
										{/each}
									</tbody>
								</table>
							</div>
						</div>

						<!-- Warnings -->
						{#if file.markerMetadata.warnings.length > 0}
							<div class="mt-4 rounded-lg border border-yellow-500/50 bg-yellow-500/10 p-3">
								<h4 class="mb-2 text-sm font-medium text-yellow-600">Warnings</h4>
								<ul class="list-inside list-disc text-xs text-yellow-600">
									{#each file.markerMetadata.warnings as warning}
										<li>{warning}</li>
									{/each}
								</ul>
							</div>
						{/if}
					</div>
				{/if}

				<!-- Expanded content for Checkshot files -->
				{#if expandedFileId === file.id && file.checkMetadata}
					<div class="border-t px-4 pb-4">
						<!-- File metadata -->
						<div class="bg-muted/30 mt-4 grid grid-cols-2 gap-4 rounded-lg p-4 sm:grid-cols-4">
							<div>
								<p class="text-muted-foreground text-xs uppercase">Stations</p>
								<p class="font-medium">{file.checkMetadata.rowCount}</p>
							</div>
							<div>
								<p class="text-muted-foreground text-xs uppercase">Delimiter</p>
								<p class="font-medium">{file.checkMetadata.delimiter}</p>
							</div>
							<div>
								<p class="text-muted-foreground text-xs uppercase">Columns</p>
								<p class="font-medium">{file.checkMetadata.columns.length}</p>
							</div>
							<div>
								<p class="text-muted-foreground text-xs uppercase">Status</p>
								<p class="font-medium {file.checkMetadata.hasRequiredColumns ? 'text-green-500' : 'text-destructive'}">
									{file.checkMetadata.hasRequiredColumns ? 'Ready' : 'Missing columns'}
								</p>
							</div>
						</div>

						<!-- Checkshot Settings -->
						<div class="mt-4">
							<h4 class="mb-2 text-sm font-medium">Checkshot Settings</h4>
							<div class="grid grid-cols-2 gap-4 sm:grid-cols-4">
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Survey Type</label>
									<select
										value={file.checkMetadata.surveyType || ''}
										onchange={(e) =>
											ingestStore.updateCheckMetadata(file.id, {
												surveyType: (e.target as HTMLSelectElement).value as CheckshotSurveyType || undefined
											})}
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									>
										<option value="">Not specified</option>
										{#each CHECKSHOT_SURVEY_TYPE_OPTIONS as st}
											<option value={st.value}>{st.label}</option>
										{/each}
									</select>
								</div>
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Depth Unit</label>
									<select
										value={file.checkMetadata.depthUnit}
										onchange={(e) =>
											ingestStore.updateCheckMetadata(file.id, {
												depthUnit: (e.target as HTMLSelectElement).value
											})}
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									>
										{#each DEPTH_UNIT_OPTIONS as opt}
											<option value={opt.value}>{opt.label}</option>
										{/each}
									</select>
								</div>
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Time Unit</label>
									<select
										value={file.checkMetadata.timeUnit}
										onchange={(e) =>
											ingestStore.updateCheckMetadata(file.id, {
												timeUnit: (e.target as HTMLSelectElement).value
											})}
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									>
										{#each TIME_UNIT_OPTIONS as opt}
											<option value={opt.value}>{opt.label}</option>
										{/each}
									</select>
								</div>
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Datum</label>
									<select
										value={file.checkMetadata.datum || ''}
										onchange={(e) =>
											ingestStore.updateCheckMetadata(file.id, {
												datum: (e.target as HTMLSelectElement).value || undefined
											})}
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									>
										<option value="">Not specified</option>
										{#each DATUM_OPTIONS as opt}
											<option value={opt.value}>{opt.label}</option>
										{/each}
									</select>
								</div>
							</div>
							<div class="mt-3 grid grid-cols-2 gap-4 sm:grid-cols-4">
								<div>
									<label class="text-muted-foreground mb-1 block text-xs">Survey Company</label>
									<input
										type="text"
										value={file.checkMetadata.surveyCompany || ''}
										onchange={(e) =>
											ingestStore.updateCheckMetadata(file.id, {
												surveyCompany: (e.target as HTMLInputElement).value || undefined
											})}
										placeholder="Optional"
										class="border-border bg-background focus:border-primary w-full rounded border px-2 py-1 text-sm focus:outline-none focus:ring-1 focus:ring-primary/50"
									/>
								</div>
								<div class="flex items-end">
									<label class="flex items-center gap-2">
										<input
											type="checkbox"
											checked={file.checkMetadata.calculateVelocity}
											onchange={(e) =>
												ingestStore.updateCheckMetadata(file.id, {
													calculateVelocity: (e.target as HTMLInputElement).checked
												})}
											class="accent-primary h-4 w-4 rounded"
										/>
										<span class="text-sm">Calculate interval velocity</span>
									</label>
								</div>
							</div>
						</div>

						<!-- Column mapping table -->
						<div class="mt-4">
							<h4 class="mb-2 text-sm font-medium">Columns</h4>
							<div class="rounded-lg border overflow-x-auto">
								<table class="w-full text-sm">
									<thead>
										<tr class="border-b bg-muted/30">
											<th class="w-12 px-4 py-2 text-left">
												<span class="sr-only">Include</span>
											</th>
											<th class="px-4 py-2 text-left font-medium">Header</th>
											<th class="px-4 py-2 text-left font-medium">Column Type</th>
											<th class="px-4 py-2 text-left font-medium">Unit</th>
											<th class="px-4 py-2 text-left font-medium">Range</th>
										</tr>
									</thead>
									<tbody class="divide-y">
										{#each file.checkMetadata.columns as col}
											<tr class="hover:bg-muted/20 {col.isRequired ? 'bg-primary/5' : ''}">
												<td class="px-4 py-2">
													<input
														type="checkbox"
														checked={col.include}
														onchange={() => toggleCheckColumnInclude(file.id, col.index)}
														class="accent-primary h-4 w-4 rounded"
													/>
												</td>
												<td class="px-4 py-2 font-mono text-xs">
													{col.header}
													{#if col.isRequired}
														<span class="ml-1 rounded bg-primary/20 px-1 text-primary text-[10px]">REQUIRED</span>
													{/if}
												</td>
												<td class="px-4 py-2">
													<div class="flex items-center gap-2">
														<select
															value={col.mappedType || col.detectedType}
															onchange={(e) =>
																setCheckColumnType(
																	file.id,
																	col.index,
																	(e.target as HTMLSelectElement).value as CheckshotColumnType
																)}
															class="border-border bg-background focus:border-primary rounded border px-2 py-1 text-xs focus:outline-none focus:ring-1 focus:ring-primary/50"
														>
															{#each CHECKSHOT_COLUMN_TYPES as type}
																<option value={type.value}>{type.label}</option>
															{/each}
														</select>
														{#if col.mappedType && col.mappedType !== col.detectedType}
															<span class="text-xs text-blue-500" title="Manually mapped">
																(mapped)
															</span>
														{/if}
													</div>
												</td>
												<td class="px-4 py-2 font-mono text-xs">{col.unit || '-'}</td>
												<td class="text-muted-foreground px-4 py-2 text-xs">
													{#if col.minValue !== null && col.maxValue !== null}
														{col.minValue.toFixed(2)} - {col.maxValue.toFixed(2)}
													{:else}
														-
													{/if}
												</td>
											</tr>
										{/each}
									</tbody>
								</table>
							</div>
						</div>

						<!-- Warnings -->
						{#if file.checkMetadata.warnings.length > 0}
							<div class="mt-4 rounded-lg border border-yellow-500/50 bg-yellow-500/10 p-3">
								<h4 class="mb-2 text-sm font-medium text-yellow-600">Warnings</h4>
								<ul class="list-inside list-disc text-xs text-yellow-600">
									{#each file.checkMetadata.warnings as warning}
										<li>{warning}</li>
									{/each}
								</ul>
							</div>
						{/if}
					</div>
				{/if}
			</div>
		{/each}
	</div>

	<!-- Navigation buttons -->
	<div class="flex justify-between pt-4">
		<button
			onclick={handleBack}
			class="text-muted-foreground hover:text-foreground flex items-center gap-2 text-sm font-medium"
		>
			<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
			</svg>
			Back
		</button>

		<button
			onclick={handleNext}
			disabled={!isMarkersMode && wellSelectionMode === 'existing' && !selectedWellId}
			class="bg-primary text-primary-foreground hover:bg-primary/90 inline-flex items-center gap-2 rounded-md px-4 py-2 text-sm font-medium transition-colors disabled:pointer-events-none disabled:opacity-50"
		>
			Continue
			<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
			</svg>
		</button>
	</div>
</div>
