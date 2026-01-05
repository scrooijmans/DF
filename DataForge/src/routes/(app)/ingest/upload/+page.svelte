<script lang="ts">
	import { goto } from '$app/navigation'
	import { open } from '@tauri-apps/plugin-dialog'
	import { ingestStore, type IngestFile } from '$lib/stores/ingest.svelte'

	let isDragging = $state(false)
	let isSelectingFiles = $state(false)

	// Redirect if no source type selected, otherwise set step
	$effect(() => {
		if (!ingestStore.sourceType) {
			goto('/ingest', { replaceState: true })
		} else {
			ingestStore.goToStep('upload')
		}
	})

	// Get file filters for native dialog based on source type
	function getDialogFilters(): { name: string; extensions: string[] }[] {
		switch (ingestStore.sourceType) {
			case 'las':
				return [{ name: 'LAS Files', extensions: ['las', 'LAS'] }]
			case 'trajectory_csv':
				return [{ name: 'Trajectory CSV Files', extensions: ['csv', 'CSV', 'txt', 'TXT'] }]
			case 'markers_csv':
				return [{ name: 'Marker CSV Files', extensions: ['csv', 'CSV', 'txt', 'TXT'] }]
			case 'surface_csv':
				return [{ name: 'Surface CSV Files', extensions: ['csv', 'CSV', 'txt', 'TXT'] }]
			case 'checkshot_csv':
				return [{ name: 'Checkshot CSV Files', extensions: ['csv', 'CSV', 'txt', 'TXT'] }]
			case 'csv':
				return [{ name: 'CSV Files', extensions: ['csv', 'CSV'] }]
			case 'excel':
				return [{ name: 'Excel Files', extensions: ['xlsx', 'xls', 'XLSX', 'XLS'] }]
			case 'parquet':
				return [{ name: 'Parquet Files', extensions: ['parquet', 'PARQUET'] }]
			default:
				return []
		}
	}

	function generateId(): string {
		return Math.random().toString(36).substring(2, 15)
	}

	// Extract filename from path
	function getFileName(filePath: string): string {
		const parts = filePath.split(/[/\\]/)
		return parts[parts.length - 1] || filePath
	}

	// Open native file dialog and process selected files
	async function handleSelectFiles() {
		if (isSelectingFiles) return
		isSelectingFiles = true

		try {
			const selected = await open({
				multiple: true,
				filters: getDialogFilters(),
				title: `Select ${ingestStore.sourceType?.toUpperCase()} Files`
			})

			if (selected === null) {
				// User cancelled
				return
			}

			// Normalize to array
			const filePaths = Array.isArray(selected) ? selected : [selected]

			for (const filePath of filePaths) {
				const fileName = getFileName(filePath)
				const ingestFile: IngestFile = {
					id: generateId(),
					name: fileName,
					size: 0, // We don't have size from dialog, backend will provide
					type: ingestStore.sourceType || 'unknown',
					status: 'pending',
					progress: 0,
					tempPath: filePath // Store the actual file path
				}

				ingestStore.addFile(ingestFile)

				// Process file via Tauri backend
				await processFileFromPath(ingestFile, filePath)
			}
		} catch (e) {
			console.error('Error selecting files:', e)
		} finally {
			isSelectingFiles = false
		}
	}

	// Process a file given its path (uses Tauri backend)
	async function processFileFromPath(ingestFile: IngestFile, filePath: string) {
		try {
			// Use the store's processFile which calls the Tauri backend
			await ingestStore.processFile(ingestFile, filePath)
		} catch (e) {
			console.error('Error processing file:', e)
			ingestStore.updateFile(ingestFile.id, {
				status: 'error',
				error: e instanceof Error ? e.message : String(e)
			})
		}
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault()
		isDragging = false
		// Note: Drag and drop from file manager may not work in Tauri without additional setup
		// For now, recommend using the file dialog
		handleSelectFiles()
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault()
		isDragging = true
	}

	function handleDragLeave(e: DragEvent) {
		e.preventDefault()
		isDragging = false
	}

	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return bytes + ' B'
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
		return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
	}

	function getStatusIcon(status: IngestFile['status']) {
		switch (status) {
			case 'pending':
				return 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z'
			case 'uploading':
			case 'parsing':
				return 'M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15'
			case 'ready':
				return 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z'
			case 'error':
				return 'M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z'
		}
	}

	function getStatusColor(status: IngestFile['status']) {
		switch (status) {
			case 'pending':
				return 'text-muted-foreground'
			case 'uploading':
			case 'parsing':
				return 'text-primary animate-spin'
			case 'ready':
				return 'text-green-500'
			case 'error':
				return 'text-destructive'
		}
	}

	function canProceed(): boolean {
		return ingestStore.files.length > 0 && ingestStore.files.every((f) => f.status === 'ready')
	}

	function handleNext() {
		if (canProceed()) {
			ingestStore.nextStep()
			goto('/ingest/mapping')
		}
	}

	function handleBack() {
		ingestStore.previousStep()
		goto('/ingest/source')
	}
</script>

<div class="space-y-6">
	<div>
		<h2 class="text-lg font-medium">Upload Files</h2>
		<p class="text-muted-foreground mt-1 text-sm">
			{#if ingestStore.sourceType === 'las'}
				Drag and drop your LAS files here, or click to browse.
			{:else if ingestStore.sourceType === 'trajectory_csv'}
				Select trajectory CSV files with MD, inclination, and azimuth columns.
			{:else if ingestStore.sourceType === 'markers_csv'}
				Select marker/well tops CSV files with marker name and depth columns.
			{:else if ingestStore.sourceType === 'surface_csv'}
				Select surface CSV files with X, Y, Z coordinate columns.
			{:else if ingestStore.sourceType === 'checkshot_csv'}
				Select checkshot CSV files with MD, TVD, and TWT columns.
			{:else}
				Drag and drop your files here, or click to browse.
			{/if}
		</p>
	</div>

	<!-- Dropzone -->
	<div
		role="button"
		tabindex="0"
		ondrop={handleDrop}
		ondragover={handleDragOver}
		ondragleave={handleDragLeave}
		onclick={handleSelectFiles}
		onkeydown={(e) => e.key === 'Enter' && handleSelectFiles()}
		class="relative flex min-h-[200px] cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed p-8 transition-colors
			{isDragging
			? 'border-primary bg-primary/5'
			: 'border-border hover:border-primary/50 hover:bg-secondary/30'}
			{isSelectingFiles ? 'pointer-events-none opacity-50' : ''}"
	>
		{#if isSelectingFiles}
			<svg class="mb-4 h-12 w-12 animate-spin text-primary" fill="none" viewBox="0 0 24 24">
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
			<p class="text-center text-sm text-muted-foreground">Processing files...</p>
		{:else}
			<svg
				class="mb-4 h-12 w-12 {isDragging ? 'text-primary' : 'text-muted-foreground'}"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
				/>
			</svg>

			<p class="text-center text-sm">
				<span class="text-foreground font-medium">Click to browse files</span>
			</p>
			<p class="text-muted-foreground mt-1 text-xs">
				{#if ingestStore.sourceType === 'las'}
					LAS 2.0 and 3.0 files supported
				{:else if ingestStore.sourceType === 'trajectory_csv'}
					CSV files with MD, INC, AZI columns (comma, tab, or semicolon delimited)
				{:else if ingestStore.sourceType === 'markers_csv'}
					CSV files with marker name and depth columns (supports multi-well files)
				{:else if ingestStore.sourceType === 'surface_csv'}
					CSV files with X, Y, Z columns (supports multi-surface files)
				{:else if ingestStore.sourceType === 'checkshot_csv'}
					CSV files with MD, TVD, TWT columns (time-depth relationships)
				{:else if ingestStore.sourceType === 'csv'}
					CSV files with headers
				{:else if ingestStore.sourceType === 'excel'}
					.xlsx and .xls files
				{:else}
					Parquet files
				{/if}
			</p>
		{/if}
	</div>

	<!-- File list -->
	{#if ingestStore.files.length > 0}
		<div class="space-y-2">
			<h3 class="text-sm font-medium">Uploaded Files ({ingestStore.files.length})</h3>

			<div class="divide-border divide-y rounded-lg border">
				{#each ingestStore.files as file}
					<div class="flex items-center gap-4 p-4">
						<!-- Status icon -->
						<svg
							class="h-5 w-5 flex-shrink-0 {getStatusColor(file.status)}"
							fill="none"
							stroke="currentColor"
							viewBox="0 0 24 24"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d={getStatusIcon(file.status)}
							/>
						</svg>

						<!-- File info -->
						<div class="min-w-0 flex-1">
							<p class="truncate text-sm font-medium">{file.name}</p>
							<p class="text-muted-foreground text-xs">
								{formatFileSize(file.size)}
								{#if file.status === 'uploading'}
									- Uploading {file.progress}%
								{:else if file.status === 'parsing'}
									- Parsing...
								{:else if file.status === 'ready' && file.lasMetadata}
									- {file.lasMetadata.curves.length} curves, {file.lasMetadata.startDepth} to {file.lasMetadata.stopDepth} {file.lasMetadata.depthUnit}
								{:else if file.status === 'ready' && file.trajectoryMetadata}
									- {file.trajectoryMetadata.rowCount} stations, {file.trajectoryMetadata.columns.length} columns
									{#if !file.trajectoryMetadata.hasRequiredColumns}
										<span class="text-destructive"> (Missing: {file.trajectoryMetadata.missingRequired.join(', ')})</span>
									{/if}
								{:else if file.status === 'ready' && file.markerMetadata}
									- {file.markerMetadata.rowCount} markers, {file.markerMetadata.columns.length} columns
									{#if file.markerMetadata.isMultiWell}
										<span class="text-primary"> ({file.markerMetadata.wellNames.length} wells)</span>
									{/if}
									{#if !file.markerMetadata.hasRequiredColumns}
										<span class="text-destructive"> (Missing: {file.markerMetadata.missingRequired.join(', ')})</span>
									{/if}
								{:else if file.status === 'ready' && file.surfaceMetadata}
									- {file.surfaceMetadata.rowCount} points
									{#if file.surfaceMetadata.isMultiSurface}
										<span class="text-primary"> ({file.surfaceMetadata.surfaceNames.length} surfaces)</span>
									{/if}
									{#if !file.surfaceMetadata.hasRequiredColumns}
										<span class="text-destructive"> (Missing: {file.surfaceMetadata.missingRequired.join(', ')})</span>
									{/if}
								{:else if file.status === 'ready' && file.checkMetadata}
									- {file.checkMetadata.rowCount} stations, {file.checkMetadata.columns.length} columns
									{#if !file.checkMetadata.hasRequiredColumns}
										<span class="text-destructive"> (Missing: {file.checkMetadata.missingRequired.join(', ')})</span>
									{/if}
								{:else if file.status === 'error'}
									- {file.error}
								{/if}
							</p>

							<!-- Progress bar -->
							{#if file.status === 'uploading'}
								<div class="mt-2 h-1 w-full overflow-hidden rounded-full bg-muted">
									<div
										class="bg-primary h-full transition-all duration-300"
										style="width: {file.progress}%"
									></div>
								</div>
							{/if}
						</div>

						<!-- Remove button -->
						<button
							onclick={() => ingestStore.removeFile(file.id)}
							class="text-muted-foreground hover:text-foreground flex-shrink-0 p-1"
							title="Remove file"
						>
							<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path
									stroke-linecap="round"
									stroke-linejoin="round"
									stroke-width="2"
									d="M6 18L18 6M6 6l12 12"
								/>
							</svg>
						</button>
					</div>
				{/each}
			</div>
		</div>
	{/if}

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
			disabled={!canProceed()}
			class="bg-primary text-primary-foreground hover:bg-primary/90 inline-flex items-center gap-2 rounded-md px-4 py-2 text-sm font-medium transition-colors disabled:pointer-events-none disabled:opacity-50"
		>
			Continue
			<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
			</svg>
		</button>
	</div>
</div>
