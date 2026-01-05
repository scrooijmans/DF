<script lang="ts">
	/**
	 * SyncStatusIndicator - Global sync status display component
	 *
	 * Shows current sync state with visual indicators:
	 * - Idle (green checkmark) - synced and up to date
	 * - Syncing (blue spinner) - sync in progress
	 * - Pending (orange dot) - local changes waiting to sync
	 * - Conflict (yellow warning) - conflicts need resolution
	 * - Error (red x) - sync error occurred
	 * - Offline (gray cloud) - no network connection
	 *
	 * Clicking navigates to sync settings for detailed management.
	 */
	import { syncStore } from '$lib/stores'
	import { goto } from '$app/navigation'

	interface Props {
		/** Show expanded view with text labels */
		expanded?: boolean
		/** Custom class for the container */
		class?: string
	}

	let { expanded = false, class: className = '' }: Props = $props()

	// Computed display state
	const displayState = $derived(() => {
		if (!syncStore.isOnline) {
			return {
				status: 'offline',
				label: 'Offline',
				color: 'text-muted-foreground',
				bgColor: 'bg-muted',
				animate: false
			}
		}
		if (syncStore.status === 'syncing') {
			return {
				status: 'syncing',
				label: 'Syncing...',
				color: 'text-blue-500',
				bgColor: 'bg-blue-500/10',
				animate: true
			}
		}
		if (syncStore.status === 'error') {
			return {
				status: 'error',
				label: 'Sync Error',
				color: 'text-red-500',
				bgColor: 'bg-red-500/10',
				animate: false
			}
		}
		if (syncStore.hasConflicts) {
			return {
				status: 'conflict',
				label: `${syncStore.conflictCount} Conflict${syncStore.conflictCount > 1 ? 's' : ''}`,
				color: 'text-yellow-500',
				bgColor: 'bg-yellow-500/10',
				animate: false
			}
		}
		if (syncStore.needsSync) {
			return {
				status: 'pending',
				label: `${syncStore.pendingChanges} Pending`,
				color: 'text-orange-500',
				bgColor: 'bg-orange-500/10',
				animate: false
			}
		}
		return {
			status: 'idle',
			label: 'Synced',
			color: 'text-green-500',
			bgColor: 'bg-green-500/10',
			animate: false
		}
	})

	function handleClick() {
		goto('/settings/sync')
	}

	function formatLastSync(dateStr: string | null): string {
		if (!dateStr) return 'Never synced'
		const date = new Date(dateStr)
		const now = new Date()
		const diffMs = now.getTime() - date.getTime()
		const diffMins = Math.floor(diffMs / 60000)

		if (diffMins < 1) return 'Just now'
		if (diffMins < 60) return `${diffMins}m ago`

		const diffHours = Math.floor(diffMins / 60)
		if (diffHours < 24) return `${diffHours}h ago`

		const diffDays = Math.floor(diffHours / 24)
		return `${diffDays}d ago`
	}
</script>

{#if syncStore.isConfigured}
	<button
		onclick={handleClick}
		class="flex items-center gap-2 rounded-md p-2 transition-colors hover:bg-secondary {className}"
		title="{displayState().label} - Click to manage sync"
	>
		<!-- Status Icon -->
		<div class="relative flex h-5 w-5 items-center justify-center {displayState().color}">
			{#if displayState().status === 'offline'}
				<!-- Cloud with slash -->
				<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z"
					/>
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4l16 16" />
				</svg>
			{:else if displayState().status === 'syncing'}
				<!-- Spinning arrows -->
				<svg class="h-5 w-5 animate-spin" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
					/>
				</svg>
			{:else if displayState().status === 'error'}
				<!-- X circle -->
				<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
					/>
				</svg>
			{:else if displayState().status === 'conflict'}
				<!-- Warning triangle -->
				<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
					/>
				</svg>
			{:else if displayState().status === 'pending'}
				<!-- Cloud upload -->
				<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
					/>
				</svg>
			{:else}
				<!-- Check circle (idle/synced) -->
				<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
					/>
				</svg>
			{/if}

			<!-- Notification badge for conflicts/pending -->
			{#if (displayState().status === 'conflict' || displayState().status === 'pending') && !expanded}
				<span
					class="absolute -right-1 -top-1 flex h-3 w-3 items-center justify-center rounded-full bg-current text-[8px] font-bold text-white"
				>
					{displayState().status === 'conflict' ? syncStore.conflictCount : syncStore.pendingChanges}
				</span>
			{/if}
		</div>

		<!-- Expanded label -->
		{#if expanded}
			<div class="hidden min-w-0 flex-1 text-left lg:block">
				<p class="truncate text-sm font-medium {displayState().color}">{displayState().label}</p>
				<p class="text-muted-foreground truncate text-xs">
					{formatLastSync(syncStore.lastSyncAt)}
				</p>
			</div>
		{/if}
	</button>
{:else if expanded}
	<button
		onclick={handleClick}
		class="flex items-center gap-2 rounded-md p-2 text-muted-foreground transition-colors hover:bg-secondary {className}"
		title="Configure sync server"
	>
		<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z"
			/>
		</svg>
		<span class="hidden text-sm lg:block">Setup Sync</span>
	</button>
{/if}
