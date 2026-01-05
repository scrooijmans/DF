<script lang="ts">
	/**
	 * Wells List Page - Browse all wells in the workspace
	 *
	 * Displays a list of wells with quick access to well detail pages.
	 */
	import { goto } from '$app/navigation'
	import { invoke } from '@tauri-apps/api/core'
	import type { WellSummary } from '$lib/types'

	// State
	let wells = $state<WellSummary[]>([])
	let isLoading = $state(true)
	let error = $state<string | null>(null)

	// Load wells on mount
	loadWells()

	async function loadWells() {
		isLoading = true
		error = null
		try {
			wells = await invoke<WellSummary[]>('get_workspace_wells')
		} catch (e) {
			error = e instanceof Error ? e.message : String(e)
			console.error('Failed to load wells:', e)
		} finally {
			isLoading = false
		}
	}

	function navigateToWell(wellId: string) {
		goto(`/wells/${wellId}`)
	}
</script>

<div class="flex h-full flex-col">
	<!-- Page Header -->
	<header class="border-border flex items-center justify-between border-b px-6 py-4">
		<div>
			<h1 class="text-2xl font-bold">Wells</h1>
			<p class="text-muted-foreground text-sm">
				{wells.length} well{wells.length !== 1 ? 's' : ''} in workspace
			</p>
		</div>
		<div class="flex items-center gap-2">
			<button
				onclick={loadWells}
				disabled={isLoading}
				class="text-muted-foreground hover:text-foreground rounded p-2 transition-colors disabled:opacity-50"
				title="Refresh"
			>
				<svg
					class="h-5 w-5 {isLoading ? 'animate-spin' : ''}"
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
			<a
				href="/ingest"
				class="bg-primary text-primary-foreground hover:bg-primary/90 inline-flex items-center gap-2 rounded-md px-4 py-2 text-sm font-medium transition-colors"
			>
				<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 4v16m8-8H4"
					/>
				</svg>
				Import Data
			</a>
		</div>
	</header>

	<!-- Main Content -->
	<div class="flex-1 overflow-auto p-6">
		{#if error}
			<div class="bg-destructive/10 text-destructive rounded-lg border border-destructive/20 p-6">
				<h2 class="font-semibold">Error Loading Wells</h2>
				<p class="mt-1 text-sm">{error}</p>
				<button onclick={loadWells} class="mt-3 text-sm underline hover:no-underline">
					Try again
				</button>
			</div>
		{:else if isLoading && wells.length === 0}
			<div class="flex items-center justify-center py-16">
				<svg class="text-muted-foreground h-8 w-8 animate-spin" fill="none" viewBox="0 0 24 24">
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
			<div class="text-muted-foreground py-16 text-center">
				<svg
					class="mx-auto h-16 w-16 opacity-50"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="1.5"
						d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4"
					/>
				</svg>
				<h3 class="mt-4 text-lg font-medium text-foreground">No wells yet</h3>
				<p class="mt-2 text-sm">Import LAS files to create wells and curves</p>
				<a
					href="/ingest"
					class="bg-primary text-primary-foreground hover:bg-primary/90 mt-4 inline-flex items-center gap-2 rounded-md px-4 py-2 text-sm font-medium transition-colors"
				>
					<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
						/>
					</svg>
					Import Data
				</a>
			</div>
		{:else}
			<div class="mx-auto max-w-4xl">
				<div class="bg-card divide-border divide-y rounded-lg border">
					{#each wells as well (well.id)}
						<button
							onclick={() => navigateToWell(well.id)}
							class="hover:bg-secondary/50 flex w-full items-center gap-4 px-4 py-3 text-left transition-colors"
						>
							<!-- Icon -->
							<div
								class="bg-blue-500/10 text-blue-500 flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-md"
							>
								<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="1.5"
										d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4"
									/>
								</svg>
							</div>

							<!-- Info -->
							<div class="min-w-0 flex-1">
								<div class="flex items-center gap-2">
									<span class="truncate font-medium">{well.name}</span>
								</div>
								<div class="text-muted-foreground mt-0.5 flex items-center gap-2 text-xs">
									{#if well.uwi}
										<span>UWI: {well.uwi}</span>
										<span class="text-border">|</span>
									{/if}
									{#if well.field}
										<span>{well.field}</span>
										<span class="text-border">|</span>
									{/if}
									<span>{well.curve_count} curve{well.curve_count !== 1 ? 's' : ''}</span>
								</div>
							</div>

							<!-- Arrow -->
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
									d="M9 5l7 7-7 7"
								/>
							</svg>
						</button>
					{/each}
				</div>
			</div>
		{/if}
	</div>
</div>
