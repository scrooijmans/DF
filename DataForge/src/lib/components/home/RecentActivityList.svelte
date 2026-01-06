<script lang="ts">
	/**
	 * RecentActivityList - Display a list of recent activity items
	 *
	 * Fetches and displays recent wells and curves from the database.
	 * Shows loading and empty states appropriately.
	 */
	import { invoke } from '@tauri-apps/api/core'
	import type { RecentActivityItem } from '$lib/types'
	import RecentActivityItemComponent from './RecentActivityItem.svelte'

	interface Props {
		/** Maximum number of items to display */
		limit?: number
		/** Callback when an item is clicked */
		onItemClick?: (item: RecentActivityItem) => void
	}

	const { limit = 10, onItemClick }: Props = $props()

	let items = $state<RecentActivityItem[]>([])
	let isLoading = $state(true)
	let error = $state<string | null>(null)

	// Load when component mounts (guarded by layout's workspaceReady check)
	$effect(() => {
		loadActivity()
	})

	async function loadActivity() {
		isLoading = true
		error = null
		try {
			items = await invoke<RecentActivityItem[]>('get_recent_activity', { limit })
		} catch (e) {
			error = e instanceof Error ? e.message : String(e)
			console.error('Failed to load recent activity:', e)
		} finally {
			isLoading = false
		}
	}

	function handleItemClick(item: RecentActivityItem) {
		if (onItemClick) {
			onItemClick(item)
		}
	}
</script>

<div class="flex flex-col">
	<!-- Header -->
	<div class="flex items-center justify-between px-3 py-2">
		<h3 class="text-sm font-semibold">Recent Activity</h3>
		<button
			onclick={loadActivity}
			disabled={isLoading}
			class="text-muted-foreground hover:text-foreground rounded p-1 transition-colors disabled:opacity-50"
			title="Refresh"
		>
			<svg
				class="h-4 w-4 {isLoading ? 'animate-spin' : ''}"
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

	<!-- Content -->
	{#if isLoading && items.length === 0}
		<div class="flex items-center justify-center py-8">
			<svg class="text-muted-foreground h-6 w-6 animate-spin" fill="none" viewBox="0 0 24 24">
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
	{:else if error}
		<div class="text-muted-foreground px-3 py-4 text-center text-sm">
			<p>Failed to load activity</p>
			<button onclick={loadActivity} class="text-primary mt-1 underline">Try again</button>
		</div>
	{:else if items.length === 0}
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
					d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
				/>
			</svg>
			<p class="mt-2 text-sm">No recent activity</p>
			<p class="mt-1 text-xs">Import data to get started</p>
		</div>
	{:else}
		<div class="divide-border divide-y">
			{#each items as item (item.id)}
				<RecentActivityItemComponent {item} onClick={handleItemClick} />
			{/each}
		</div>
	{/if}
</div>
