<script lang="ts">
	/**
	 * RecentActivityItem - Display a single recent activity entry (well or curve)
	 *
	 * Shows item name, type icon, parent info (for curves), size, and timestamps.
	 * Designed to be used in a list context.
	 */
	import type { RecentActivityItem } from '$lib/types'

	interface Props {
		item: RecentActivityItem
		onClick?: (item: RecentActivityItem) => void
	}

	const { item, onClick }: Props = $props()

	function formatBytes(bytes: number | null): string {
		if (bytes === null || bytes === 0) return '-'
		const units = ['B', 'KB', 'MB', 'GB']
		let unitIndex = 0
		let size = bytes
		while (size >= 1024 && unitIndex < units.length - 1) {
			size /= 1024
			unitIndex++
		}
		return `${size.toFixed(size < 10 ? 1 : 0)} ${units[unitIndex]}`
	}

	function formatNumber(num: number | null): string {
		if (num === null) return '-'
		return num.toLocaleString()
	}

	function formatRelativeTime(dateString: string): string {
		const date = new Date(dateString)
		const now = new Date()
		const diffMs = now.getTime() - date.getTime()
		const diffSecs = Math.floor(diffMs / 1000)
		const diffMins = Math.floor(diffSecs / 60)
		const diffHours = Math.floor(diffMins / 60)
		const diffDays = Math.floor(diffHours / 24)

		if (diffSecs < 60) return 'just now'
		if (diffMins < 60) return `${diffMins}m ago`
		if (diffHours < 24) return `${diffHours}h ago`
		if (diffDays < 7) return `${diffDays}d ago`

		return date.toLocaleDateString(undefined, { month: 'short', day: 'numeric' })
	}

	function handleClick() {
		if (onClick) {
			onClick(item)
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			e.preventDefault()
			handleClick()
		}
	}
</script>

<button
	type="button"
	onclick={handleClick}
	onkeydown={handleKeydown}
	class="hover:bg-secondary/50 focus:bg-secondary/50 group flex w-full items-center gap-3 rounded-md px-3 py-2.5 text-left transition-colors focus:outline-none"
>
	<!-- Icon -->
	<div
		class="flex h-9 w-9 flex-shrink-0 items-center justify-center rounded-md {item.item_type ===
		'well'
			? 'bg-blue-500/10 text-blue-500'
			: 'bg-emerald-500/10 text-emerald-500'}"
	>
		{#if item.item_type === 'well'}
			<!-- Well icon (database/cylinder) -->
			<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="1.5"
					d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4"
				/>
			</svg>
		{:else}
			<!-- Curve icon (chart/line) -->
			<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="1.5"
					d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4v16"
				/>
			</svg>
		{/if}
	</div>

	<!-- Content -->
	<div class="min-w-0 flex-1">
		<div class="flex items-center gap-2">
			<span class="truncate font-medium">{item.name}</span>
			{#if item.item_type === 'curve' && item.parent_name}
				<span class="text-muted-foreground truncate text-xs">
					in {item.parent_name}
				</span>
			{/if}
		</div>
		<div class="text-muted-foreground mt-0.5 flex items-center gap-2 text-xs">
			{#if item.row_count !== null}
				<span>{formatNumber(item.row_count)} rows</span>
				<span class="text-border">|</span>
			{/if}
			<span>{formatBytes(item.size_bytes)}</span>
		</div>
	</div>

	<!-- Timestamp -->
	<div class="text-muted-foreground flex-shrink-0 text-xs">
		{formatRelativeTime(item.updated_at)}
	</div>
</button>
