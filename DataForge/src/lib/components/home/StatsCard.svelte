<script lang="ts">
	/**
	 * StatsCard - Display a single statistic with label and icon
	 *
	 * Reusable card component for showing workspace statistics.
	 */

	interface Props {
		/** The numeric value to display */
		value: number | string
		/** Label describing the statistic */
		label: string
		/** Optional icon slot */
		icon?: 'wells' | 'curves' | 'rows' | 'storage'
		/** Whether the card is in loading state */
		loading?: boolean
	}

	const { value, label, icon = 'wells', loading = false }: Props = $props()

	function formatValue(val: number | string): string {
		if (typeof val === 'string') return val
		if (val >= 1_000_000) return `${(val / 1_000_000).toFixed(1)}M`
		if (val >= 1_000) return `${(val / 1_000).toFixed(1)}K`
		return val.toLocaleString()
	}
</script>

<div class="bg-card rounded-lg border p-4">
	<div class="flex items-start justify-between">
		<div>
			{#if loading}
				<div class="bg-muted h-8 w-16 animate-pulse rounded"></div>
			{:else}
				<p class="text-2xl font-bold">{formatValue(value)}</p>
			{/if}
			<p class="text-muted-foreground mt-1 text-sm">{label}</p>
		</div>
		<div class="text-muted-foreground/50">
			{#if icon === 'wells'}
				<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="1.5"
						d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4"
					/>
				</svg>
			{:else if icon === 'curves'}
				<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="1.5"
						d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4v16"
					/>
				</svg>
			{:else if icon === 'rows'}
				<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="1.5"
						d="M4 6h16M4 10h16M4 14h16M4 18h16"
					/>
				</svg>
			{:else if icon === 'storage'}
				<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="1.5"
						d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"
					/>
				</svg>
			{/if}
		</div>
	</div>
</div>
