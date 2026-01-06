<script lang="ts">
	import { invoke } from '@tauri-apps/api/core'
	import { goto } from '$app/navigation'
	import { authStore } from '$lib'
	import { RecentActivityList, StatsCard } from '$lib/components/home'
	import type { WorkspaceStats, RecentActivityItem } from '$lib/types'

	let stats = $state<WorkspaceStats | null>(null)
	let isLoadingStats = $state(true)
	let statsError = $state<string | null>(null)

	// Load stats when component mounts (guarded by layout's workspaceReady check)
	$effect(() => {
		loadStats()
	})

	async function loadStats() {
		isLoadingStats = true
		statsError = null
		try {
			stats = await invoke<WorkspaceStats>('get_workspace_stats')
		} catch (e) {
			statsError = e instanceof Error ? e.message : String(e)
			console.error('Failed to load workspace stats:', e)
		} finally {
			isLoadingStats = false
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

	function handleActivityClick(item: RecentActivityItem) {
		// Navigate to the appropriate detail page based on item type
		if (item.item_type === 'well') {
			goto(`/wells/${item.id}`)
		} else if (item.item_type === 'curve' && item.parent_id) {
			// Navigate to well detail page for curves
			goto(`/wells/${item.parent_id}`)
		} else {
			goto('/inspector')
		}
	}
</script>

<div class="flex h-full flex-col">
	<!-- Page Header -->
	<header class="border-border border-b px-6 py-4">
		<h1 class="text-2xl font-bold">Home</h1>
		<p class="text-muted-foreground text-sm">
			Welcome to {authStore.currentWorkspace?.name || 'DataForge'}
		</p>
	</header>

	<!-- Main Content -->
	<div class="flex-1 overflow-auto p-6">
		<div class="mx-auto max-w-4xl space-y-6">
			<!-- First-run / Empty workspace guidance -->
			{#if !isLoadingStats && stats && stats.well_count === 0}
				<section class="rounded-lg border border-blue-200 bg-gradient-to-r from-blue-50 to-indigo-50 p-6 dark:border-blue-900 dark:from-blue-950 dark:to-indigo-950">
					<div class="flex items-start gap-4">
						<div class="rounded-full bg-blue-100 p-3 dark:bg-blue-900">
							<svg class="h-6 w-6 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
							</svg>
						</div>
						<div class="flex-1">
							<h2 class="text-lg font-semibold text-blue-900 dark:text-blue-100">
								Get started with your workspace
							</h2>
							<p class="mt-1 text-sm text-blue-700 dark:text-blue-300">
								Your workspace is empty. Import some data to get started!
							</p>
							<div class="mt-4 flex flex-wrap gap-3">
								<a
									href="/ingest"
									class="inline-flex items-center gap-2 rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-blue-700"
								>
									<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
									</svg>
									Import LAS Files
								</a>
								<a
									href="/ingest/trajectory"
									class="inline-flex items-center gap-2 rounded-md border border-blue-300 bg-white px-4 py-2 text-sm font-medium text-blue-700 transition-colors hover:bg-blue-50 dark:border-blue-700 dark:bg-blue-900 dark:text-blue-200 dark:hover:bg-blue-800"
								>
									Import Trajectories
								</a>
							</div>
						</div>
					</div>
				</section>
			{/if}

			<!-- Stats Grid -->
			<section>
				<h2 class="mb-3 text-sm font-semibold uppercase tracking-wide text-muted-foreground">
					Workspace Overview
				</h2>
				<div class="grid grid-cols-2 gap-4 lg:grid-cols-4">
					<StatsCard
						value={stats?.well_count ?? 0}
						label="Wells"
						icon="wells"
						loading={isLoadingStats}
					/>
					<StatsCard
						value={stats?.curve_count ?? 0}
						label="Curves"
						icon="curves"
						loading={isLoadingStats}
					/>
					<StatsCard
						value={stats?.total_data_points ?? 0}
						label="Data Points"
						icon="rows"
						loading={isLoadingStats}
					/>
					<StatsCard
						value={stats ? formatBytes(stats.total_size_bytes) : '0 B'}
						label="Storage Used"
						icon="storage"
						loading={isLoadingStats}
					/>
				</div>
			</section>

			<!-- Quick Actions -->
			<section>
				<h2 class="mb-3 text-sm font-semibold uppercase tracking-wide text-muted-foreground">
					Quick Actions
				</h2>
				<div class="flex flex-wrap gap-3">
					<a
						href="/ingest"
						class="bg-primary text-primary-foreground hover:bg-primary/90 inline-flex items-center gap-2 rounded-md px-4 py-2 text-sm font-medium transition-colors"
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
					<a
						href="/inspector"
						class="bg-secondary text-secondary-foreground hover:bg-secondary/80 inline-flex items-center gap-2 rounded-md px-4 py-2 text-sm font-medium transition-colors"
					>
						<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4"
							/>
						</svg>
						Browse Database
					</a>
				</div>
			</section>

			<!-- Recent Activity -->
			<section>
				<h2 class="mb-3 text-sm font-semibold uppercase tracking-wide text-muted-foreground">
					Recent Activity
				</h2>
				<div class="bg-card rounded-lg border">
					<RecentActivityList limit={15} onItemClick={handleActivityClick} />
				</div>
			</section>
		</div>
	</div>
</div>
