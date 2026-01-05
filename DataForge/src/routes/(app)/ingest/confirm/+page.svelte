<script lang="ts">
	import { goto } from '$app/navigation'
	import { ingestStore } from '$lib/stores/ingest.svelte'

	let isSubmitting = $state(false)
	let submitSuccess = $state(false)

	// Redirect if no files uploaded, otherwise set step
	$effect(() => {
		if (ingestStore.files.length === 0) {
			goto('/ingest', { replaceState: true })
		} else {
			ingestStore.goToStep('confirm')
		}
	})

	async function handleSubmit() {
		isSubmitting = true
		const success = await ingestStore.submit()
		isSubmitting = false

		if (success) {
			submitSuccess = true
		}
	}

	function handleBack() {
		ingestStore.previousStep()
		goto('/ingest/mapping')
	}

	function handleNewIngest() {
		ingestStore.reset()
		goto('/ingest/source')
	}

	function handleViewData() {
		goto('/inspector')
	}

	// Calculate summary stats
	function getTotalCurves(): number {
		return ingestStore.files.reduce((total, file) => {
			const included = file.lasMetadata?.curves.filter((c) => c.include).length ?? 0
			return total + included
		}, 0)
	}

	function getTotalDataPoints(): number {
		return ingestStore.files.reduce((total, file) => {
			if (!file.lasMetadata) return total
			const curveCount = file.lasMetadata.curves.filter((c) => c.include).length
			const depthRange = file.lasMetadata.stopDepth - file.lasMetadata.startDepth
			const step = file.lasMetadata.step || 1
			const pointsPerCurve = Math.ceil(depthRange / step)
			return total + curveCount * pointsPerCurve
		}, 0)
	}
</script>

{#if submitSuccess}
	<!-- Success state -->
	<div class="flex flex-col items-center justify-center py-12 text-center">
		<div class="mb-6 flex h-16 w-16 items-center justify-center rounded-full bg-green-100">
			<svg class="h-8 w-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M5 13l4 4L19 7"
				/>
			</svg>
		</div>

		<h2 class="mb-2 text-xl font-semibold">Import Complete</h2>
		<p class="text-muted-foreground mb-8 max-w-md">
			Successfully imported {ingestStore.files.length} file{ingestStore.files.length > 1
				? 's'
				: ''} with {getTotalCurves()} curves into your workspace.
		</p>

		<div class="flex gap-4">
			<button
				onclick={handleNewIngest}
				class="border-border hover:bg-secondary rounded-md border px-4 py-2 text-sm font-medium transition-colors"
			>
				Import More Data
			</button>
			<button
				onclick={handleViewData}
				class="bg-primary text-primary-foreground hover:bg-primary/90 rounded-md px-4 py-2 text-sm font-medium transition-colors"
			>
				View Data
			</button>
		</div>
	</div>
{:else}
	<!-- Confirmation state -->
	<div class="space-y-6">
		<div>
			<h2 class="text-lg font-medium">Review & Confirm</h2>
			<p class="text-muted-foreground mt-1 text-sm">
				Review your import configuration before proceeding.
			</p>
		</div>

		<!-- Summary cards -->
		<div class="grid gap-4 sm:grid-cols-3">
			<div class="rounded-lg border p-4">
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10">
						<svg class="text-primary h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
							/>
						</svg>
					</div>
					<div>
						<p class="text-2xl font-semibold">{ingestStore.files.length}</p>
						<p class="text-muted-foreground text-sm">Files</p>
					</div>
				</div>
			</div>

			<div class="rounded-lg border p-4">
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10">
						<svg class="text-primary h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z"
							/>
						</svg>
					</div>
					<div>
						<p class="text-2xl font-semibold">{getTotalCurves()}</p>
						<p class="text-muted-foreground text-sm">Curves</p>
					</div>
				</div>
			</div>

			<div class="rounded-lg border p-4">
				<div class="flex items-center gap-3">
					<div class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10">
						<svg class="text-primary h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4"
							/>
						</svg>
					</div>
					<div>
						<p class="text-2xl font-semibold">{getTotalDataPoints().toLocaleString()}</p>
						<p class="text-muted-foreground text-sm">Data Points</p>
					</div>
				</div>
			</div>
		</div>

		<!-- Files detail -->
		<div class="space-y-2">
			<h3 class="text-sm font-medium">Files to Import</h3>
			<div class="divide-border divide-y rounded-lg border">
				{#each ingestStore.files as file}
					<div class="p-4">
						<div class="flex items-start justify-between">
							<div>
								<p class="font-medium">{file.name}</p>
								{#if file.lasMetadata}
									<p class="text-muted-foreground mt-1 text-sm">
										Well: {file.lasMetadata.wellName}
										{#if file.lasMetadata.uwi}
											({file.lasMetadata.uwi})
										{/if}
									</p>
									<p class="text-muted-foreground text-sm">
										Depth: {file.lasMetadata.startDepth} - {file.lasMetadata.stopDepth}
										{file.lasMetadata.depthUnit}
									</p>
								{/if}
							</div>
							<div class="text-right">
								<p class="text-muted-foreground text-sm">
									{file.lasMetadata?.curves.filter((c) => c.include).length ?? 0} curves
								</p>
							</div>
						</div>

						<!-- Curves list -->
						{#if file.lasMetadata}
							<div class="mt-3 flex flex-wrap gap-2">
								{#each file.lasMetadata.curves.filter((c) => c.include) as curve}
									<span class="rounded-md bg-muted px-2 py-1 font-mono text-xs">
										{curve.mnemonic}
									</span>
								{/each}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</div>

		<!-- Warning box -->
		<div class="rounded-lg border border-border bg-background p-4">
			<div class="flex gap-3">
				<svg
					class="h-5 w-5 flex-shrink-0 text-foreground"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
					/>
				</svg>
				<div class="text-sm">
					<p class="font-medium text-foreground">Before you proceed</p>
					<p class="mt-1 text-muted-foreground">
						This will create new well records and curve data in your workspace. Ensure the curve
						mappings are correct before importing.
					</p>
				</div>
			</div>
		</div>

		<!-- Error display -->
		{#if ingestStore.submitError}
			<div class="rounded-lg border border-destructive/50 bg-destructive/10 p-4">
				<div class="flex gap-3">
					<svg
						class="text-destructive h-5 w-5 flex-shrink-0"
						fill="none"
						stroke="currentColor"
						viewBox="0 0 24 24"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
						/>
					</svg>
					<div class="text-sm">
						<p class="text-destructive font-medium">Import failed</p>
						<p class="text-destructive/80 mt-1">{ingestStore.submitError}</p>
					</div>
				</div>
			</div>
		{/if}

		<!-- Navigation buttons -->
		<div class="flex justify-between pt-4">
			<button
				onclick={handleBack}
				disabled={isSubmitting}
				class="text-muted-foreground hover:text-foreground flex items-center gap-2 text-sm font-medium disabled:opacity-50"
			>
				<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M15 19l-7-7 7-7"
					/>
				</svg>
				Back
			</button>

			<button
				onclick={handleSubmit}
				disabled={isSubmitting}
				class="bg-primary text-primary-foreground hover:bg-primary/90 inline-flex items-center gap-2 rounded-md px-4 py-2 text-sm font-medium transition-colors disabled:pointer-events-none disabled:opacity-50"
			>
				{#if isSubmitting}
					<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
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
					Importing...
				{:else}
					<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M5 13l4 4L19 7"
						/>
					</svg>
					Start Import
				{/if}
			</button>
		</div>
	</div>
{/if}
