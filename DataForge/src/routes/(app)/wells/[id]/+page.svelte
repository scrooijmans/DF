<script lang="ts">
	/**
	 * Well Detail Page - Dashboard view for a specific well
	 *
	 * Displays well metadata, statistics, and recent activity (curves modified, added, etc.)
	 */
	import { page } from '$app/stores'
	import { goto } from '$app/navigation'
	import { invoke } from '@tauri-apps/api/core'
	import { StatsCard, RecentActivityItem } from '$lib/components/home'
	import type { WellDetails, RecentActivityItem as RecentActivityItemType } from '$lib/types'

	// Get well ID from route params
	let wellId = $derived($page.params.id)

	// State
	let well = $state<WellDetails | null>(null)
	let activity = $state<RecentActivityItemType[]>([])
	let isLoadingWell = $state(true)
	let isLoadingActivity = $state(true)
	let wellError = $state<string | null>(null)
	let activityError = $state<string | null>(null)

	// Load data when wellId changes
	$effect(() => {
		if (wellId) {
			loadWellDetails()
			loadWellActivity()
		}
	})

	async function loadWellDetails() {
		isLoadingWell = true
		wellError = null
		try {
			well = await invoke<WellDetails>('get_well_details', { wellId })
		} catch (e) {
			wellError = e instanceof Error ? e.message : String(e)
			console.error('Failed to load well details:', e)
		} finally {
			isLoadingWell = false
		}
	}

	async function loadWellActivity() {
		isLoadingActivity = true
		activityError = null
		try {
			activity = await invoke<RecentActivityItemType[]>('get_well_activity', {
				wellId,
				limit: 20
			})
		} catch (e) {
			activityError = e instanceof Error ? e.message : String(e)
			console.error('Failed to load well activity:', e)
		} finally {
			isLoadingActivity = false
		}
	}

	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B'
		const units = ['B', 'KB', 'MB', 'GB', 'TB']
		let unitIndex = 0
		let size = bytes
		while (size >= 1024 && unitIndex < units.length - 1) {
			size /= 1024
			unitIndex++
		}
		return `${size.toFixed(size < 10 && unitIndex > 0 ? 1 : 0)} ${units[unitIndex]}`
	}

	function formatDepth(depth: number | null, unit: string): string {
		if (depth === null) return '-'
		return `${depth.toLocaleString()} ${unit}`
	}

	function formatDate(dateString: string): string {
		const date = new Date(dateString)
		return date.toLocaleDateString(undefined, {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		})
	}

	function handleActivityClick(item: RecentActivityItemType) {
		// Navigate to curves page with this well and curve selected
		goto(`/curves?well=${wellId}`)
	}

	function navigateToCurves() {
		goto(`/curves?well=${wellId}`)
	}
</script>

<div class="flex h-full flex-col">
	<!-- Page Header -->
	<header class="border-border flex items-center gap-4 border-b px-6 py-4">
		<a
			href="/"
			class="text-muted-foreground hover:text-foreground rounded p-1 transition-colors"
			title="Back to Home"
		>
			<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M15 19l-7-7 7-7"
				/>
			</svg>
		</a>
		<div class="flex-1">
			{#if isLoadingWell}
				<div class="bg-muted h-7 w-48 animate-pulse rounded"></div>
			{:else if well}
				<h1 class="text-2xl font-bold">{well.name}</h1>
				{#if well.uwi}
					<p class="text-muted-foreground text-sm">UWI: {well.uwi}</p>
				{/if}
			{:else}
				<h1 class="text-2xl font-bold">Well Not Found</h1>
			{/if}
		</div>
		{#if well}
			<button
				onclick={navigateToCurves}
				class="bg-primary text-primary-foreground hover:bg-primary/90 inline-flex items-center gap-2 rounded-md px-4 py-2 text-sm font-medium transition-colors"
			>
				<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4v16"
					/>
				</svg>
				View Curves
			</button>
		{/if}
	</header>

	<!-- Main Content -->
	<div class="flex-1 overflow-auto p-6">
		{#if wellError}
			<div class="bg-destructive/10 text-destructive rounded-lg border border-destructive/20 p-6">
				<h2 class="font-semibold">Error Loading Well</h2>
				<p class="mt-1 text-sm">{wellError}</p>
				<button
					onclick={loadWellDetails}
					class="mt-3 text-sm underline hover:no-underline"
				>
					Try again
				</button>
			</div>
		{:else}
			<div class="mx-auto max-w-4xl space-y-6">
				<!-- Stats Grid -->
				<section>
					<h2 class="text-muted-foreground mb-3 text-sm font-semibold uppercase tracking-wide">
						Well Statistics
					</h2>
					<div class="grid grid-cols-2 gap-4 lg:grid-cols-4">
						<StatsCard
							value={well?.curve_count ?? 0}
							label="Curves"
							icon="curves"
							loading={isLoadingWell}
						/>
						<StatsCard
							value={well?.total_data_points ?? 0}
							label="Data Points"
							icon="rows"
							loading={isLoadingWell}
						/>
						<StatsCard
							value={well ? formatBytes(well.total_size_bytes) : '0 B'}
							label="Storage Used"
							icon="storage"
							loading={isLoadingWell}
						/>
						<StatsCard
							value={well?.depth_step ?? 0}
							label="Depth Step ({well?.depth_unit ?? 'ft'})"
							icon="wells"
							loading={isLoadingWell}
						/>
					</div>
				</section>

				<!-- Well Metadata -->
				<section>
					<h2 class="text-muted-foreground mb-3 text-sm font-semibold uppercase tracking-wide">
						Well Information
					</h2>
					<div class="bg-card divide-border divide-y rounded-lg border">
						{#if isLoadingWell}
							<div class="space-y-3 p-4">
								{#each Array(6) as _}
									<div class="flex justify-between">
										<div class="bg-muted h-4 w-24 animate-pulse rounded"></div>
										<div class="bg-muted h-4 w-32 animate-pulse rounded"></div>
									</div>
								{/each}
							</div>
						{:else if well}
							<div class="grid grid-cols-2 gap-x-8 gap-y-1 p-4">
								{#if well.field}
									<div class="flex justify-between py-1.5">
										<span class="text-muted-foreground text-sm">Field</span>
										<span class="text-sm font-medium">{well.field}</span>
									</div>
								{/if}
								{#if well.company}
									<div class="flex justify-between py-1.5">
										<span class="text-muted-foreground text-sm">Company</span>
										<span class="text-sm font-medium">{well.company}</span>
									</div>
								{/if}
								{#if well.location}
									<div class="flex justify-between py-1.5">
										<span class="text-muted-foreground text-sm">Location</span>
										<span class="text-sm font-medium">{well.location}</span>
									</div>
								{/if}
								{#if well.x !== null && well.y !== null}
									<div class="flex justify-between py-1.5">
										<span class="text-muted-foreground text-sm">Coordinates</span>
										<span class="text-sm font-medium">
											{well.x.toFixed(4)}, {well.y.toFixed(4)}
										</span>
									</div>
								{/if}
								<div class="flex justify-between py-1.5">
									<span class="text-muted-foreground text-sm">Depth Range</span>
									<span class="text-sm font-medium">
										{formatDepth(well.min_depth, well.depth_unit)} - {formatDepth(well.max_depth, well.depth_unit)}
									</span>
								</div>
								<div class="flex justify-between py-1.5">
									<span class="text-muted-foreground text-sm">Depth Origin</span>
									<span class="text-sm font-medium">
										{formatDepth(well.depth_origin, well.depth_unit)}
									</span>
								</div>
							</div>
							<div class="text-muted-foreground flex gap-4 px-4 py-3 text-xs">
								<span>Created: {formatDate(well.created_at)}</span>
								<span>Updated: {formatDate(well.updated_at)}</span>
							</div>
						{/if}
					</div>
				</section>

				<!-- Recent Activity -->
				<section>
					<div class="mb-3 flex items-center justify-between">
						<h2 class="text-muted-foreground text-sm font-semibold uppercase tracking-wide">
							Recent Curve Activity
						</h2>
						<button
							onclick={loadWellActivity}
							disabled={isLoadingActivity}
							class="text-muted-foreground hover:text-foreground rounded p-1 transition-colors disabled:opacity-50"
							title="Refresh"
						>
							<svg
								class="h-4 w-4 {isLoadingActivity ? 'animate-spin' : ''}"
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
					<div class="bg-card rounded-lg border">
						{#if isLoadingActivity && activity.length === 0}
							<div class="flex items-center justify-center py-8">
								<svg
									class="text-muted-foreground h-6 w-6 animate-spin"
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
						{:else if activityError}
							<div class="text-muted-foreground px-3 py-4 text-center text-sm">
								<p>Failed to load activity</p>
								<button
									onclick={loadWellActivity}
									class="text-primary mt-1 underline"
								>
									Try again
								</button>
							</div>
						{:else if activity.length === 0}
							<div class="text-muted-foreground px-3 py-8 text-center">
								<svg
									class="mx-auto h-10 w-10 opacity-50"
									fill="none"
									stroke="currentColor"
									viewBox="0 0 24 24"
								>
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="1.5"
										d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4v16"
									/>
								</svg>
								<p class="mt-2 text-sm">No curves yet</p>
								<p class="mt-1 text-xs">Import curve data for this well to see activity</p>
							</div>
						{:else}
							<div class="divide-border divide-y">
								{#each activity as item (item.id)}
									<RecentActivityItem {item} onClick={handleActivityClick} />
								{/each}
							</div>
						{/if}
					</div>
				</section>
			</div>
		{/if}
	</div>
</div>
