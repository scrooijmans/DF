<script lang="ts">
	/**
	 * TabNavigation - Tab bar for ChartSettingsDialog
	 *
	 * Displays available tabs with icons and handles tab switching.
	 */

	type TabId = 'data' | 'axes' | 'style' | 'display' | 'advanced' | 'wellTops'

	interface Tab {
		id: TabId
		label: string
		icon: string
	}

	interface Props {
		/** Available tabs to display */
		tabs: Tab[]
		/** Currently active tab ID */
		activeTab: TabId
		/** Callback when tab changes */
		onTabChange: (tabId: TabId) => void
	}

	let { tabs, activeTab, onTabChange }: Props = $props()
</script>

<div class="tab-nav" role="tablist" aria-label="Chart settings tabs">
	{#each tabs as tab (tab.id)}
		<button
			class="tab-button"
			class:active={activeTab === tab.id}
			onclick={() => onTabChange(tab.id)}
			role="tab"
			aria-selected={activeTab === tab.id}
		>
			<svg viewBox="0 0 24 24" width="16" height="16" aria-hidden="true">
				<path fill="currentColor" d={tab.icon} />
			</svg>
			<span>{tab.label}</span>
		</button>
	{/each}
</div>

<style>
	.tab-nav {
		display: flex;
		gap: 4px;
		padding: 12px 16px;
		border-bottom: 1px solid hsl(var(--border));
		background: hsl(var(--muted) / 0.3);
		overflow-x: auto;
	}

	.tab-button {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 8px 12px;
		border: none;
		border-radius: 6px;
		background: transparent;
		color: hsl(var(--muted-foreground));
		font-size: 13px;
		font-weight: 500;
		cursor: pointer;
		transition:
			background 0.15s,
			color 0.15s;
		white-space: nowrap;
	}

	.tab-button:hover {
		background: hsl(var(--accent));
		color: hsl(var(--accent-foreground));
	}

	.tab-button.active {
		background: hsl(var(--primary));
		color: hsl(var(--primary-foreground));
	}

	.tab-button svg {
		flex-shrink: 0;
	}
</style>
