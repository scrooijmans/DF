<script lang="ts">
	import { goto } from '$app/navigation'
	import { ingestStore, type DataSourceType } from '$lib/stores/ingest.svelte'

	// Set current step when page loads
	$effect(() => {
		ingestStore.goToStep('source')
	})

	// Available data sources
	const dataSources: {
		id: DataSourceType
		name: string
		description: string
		icon: string
		available: boolean
	}[] = [
		{
			id: 'las',
			name: 'LAS Files',
			description: 'Well log data in LAS 2.0/3.0 format. Import curve data with automatic mnemonic detection.',
			icon: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z',
			available: true
		},
		{
			id: 'trajectory_csv',
			name: 'Trajectory CSV',
			description: 'Survey/directional data from CSV. Import MD, inclination, azimuth with auto-calculated TVD, NS, EW.',
			icon: 'M9 19V6l12-3v13M9 19c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zm12-3c0 1.105-1.343 2-3 2s-3-.895-3-2 1.343-2 3-2 3 .895 3 2zM9 10l12-3',
			available: true
		},
		{
			id: 'markers_csv',
			name: 'Markers/Well Tops CSV',
			description: 'Formation tops, sequence boundaries, horizon picks. Supports multi-well files with automatic well matching.',
			icon: 'M3 4h13M3 8h9m-9 4h9m5-4v12m0 0l-4-4m4 4l4-4',
			available: true
		},
		{
			id: 'surface_csv',
			name: 'Surface CSV',
			description: '3D surfaces from CSV with X, Y, Z coordinates. Horizons, faults, unconformities, and other geological surfaces.',
			icon: 'M4 5a1 1 0 011-1h14a1 1 0 011 1v2a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM4 13a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H5a1 1 0 01-1-1v-6z',
			available: true
		},
		{
			id: 'checkshot_csv',
			name: 'Checkshot CSV',
			description: 'Time-depth relationship data (MD, TVD, TWT). Velocity calibration, seismic-to-well ties.',
			icon: 'M13 10V3L4 14h7v7l9-11h-7z',
			available: true
		},
		{
			id: 'csv',
			name: 'CSV Files',
			description: 'Comma-separated values. Map columns to curve data or well properties.',
			icon: 'M3 10h18M3 14h18m-9-4v8m-7 0h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z',
			available: false
		},
		{
			id: 'excel',
			name: 'Excel Files',
			description: 'Microsoft Excel spreadsheets (.xlsx, .xls). Import tabular well data.',
			icon: 'M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z',
			available: false
		},
		{
			id: 'parquet',
			name: 'Parquet Files',
			description: 'Apache Parquet columnar format. Efficient bulk import of processed data.',
			icon: 'M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4',
			available: false
		}
	]

	function selectSource(source: (typeof dataSources)[0]) {
		if (!source.available) return
		ingestStore.setSourceType(source.id)
		ingestStore.nextStep()
		goto('/ingest/upload')
	}
</script>

<div class="space-y-6">
	<div>
		<h2 class="text-lg font-medium">Select Data Source</h2>
		<p class="text-muted-foreground mt-1 text-sm">
			Choose the type of data you want to import into your workspace.
		</p>
	</div>

	<!-- Source type cards -->
	<div class="grid gap-4 sm:grid-cols-2">
		{#each dataSources as source}
			<button
				onclick={() => selectSource(source)}
				disabled={!source.available}
				class="group relative rounded-lg border p-6 text-left transition-all
					{source.available
					? 'border-border hover:border-primary hover:bg-secondary/50 cursor-pointer'
					: 'border-border/50 bg-muted/30 cursor-not-allowed opacity-60'}
					{ingestStore.sourceType === source.id ? 'border-primary bg-secondary/50 ring-2 ring-primary/20' : ''}"
			>
				<!-- Icon -->
				<div
					class="mb-4 flex h-12 w-12 items-center justify-center rounded-lg
					{source.available ? 'bg-primary/10 text-primary' : 'bg-muted text-muted-foreground'}"
				>
					<svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={source.icon} />
					</svg>
				</div>

				<!-- Content -->
				<h3 class="font-medium {source.available ? 'text-foreground' : 'text-muted-foreground'}">
					{source.name}
				</h3>
				<p class="text-muted-foreground mt-1 text-sm">
					{source.description}
				</p>

				<!-- Coming soon badge -->
				{#if !source.available}
					<span
						class="absolute right-4 top-4 rounded-full bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground"
					>
						Coming soon
					</span>
				{/if}

				<!-- Selected indicator -->
				{#if ingestStore.sourceType === source.id}
					<div class="absolute right-4 top-4 text-primary">
						<svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
							<path
								fill-rule="evenodd"
								d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
								clip-rule="evenodd"
							/>
						</svg>
					</div>
				{/if}
			</button>
		{/each}
	</div>

	<!-- Info box -->
	<div class="bg-muted/50 rounded-lg border p-4">
		<div class="flex gap-3">
			<svg
				class="text-muted-foreground h-5 w-5 flex-shrink-0"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
				/>
			</svg>
			<div class="text-sm">
				<p class="text-muted-foreground">
					<strong class="text-foreground">LAS files</strong> are the industry standard for well log
					data. They contain curve measurements like gamma ray (GR), resistivity, and density along
					with well header information.
				</p>
			</div>
		</div>
	</div>
</div>
