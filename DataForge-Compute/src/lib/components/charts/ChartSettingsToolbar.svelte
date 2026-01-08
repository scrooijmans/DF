<script lang="ts">
	/**
	 * ChartSettingsToolbar - Top toolbar with chart info
	 *
	 * Displays:
	 * - Chart type label
	 * - Selected chart pane title
	 *
	 * Settings are now inline in the right sidebar (ContextToolbar → ChartSettingsPanel)
	 */

	import { selectionContext } from '$lib/panes/selection-context';
	import { PaneType } from '$lib/panes/layout-model';

	/** Selection stores */
	let selectionType = selectionContext.selectionType;
	let selectedPane = selectionContext.selectedPane;

	/** Whether a chart pane is currently selected */
	let hasSelectedChart = $derived($selectionType === 'pane' && $selectedPane !== null);

	/** Get the chart type name for display */
	function getChartTypeName(paneType: PaneType | undefined): string {
		switch (paneType) {
			case PaneType.LineChart:
				return 'Line Chart';
			case PaneType.ScatterChart:
				return 'Scatter Chart';
			case PaneType.Histogram:
				return 'Histogram';
			case PaneType.CrossPlot:
				return 'Cross Plot';
			case PaneType.WellLog:
				return 'Well Log';
			case PaneType.D3WellLog:
				return 'D3 Well Log';
			case PaneType.LinkedCharts:
				return 'Linked Charts';
			case PaneType.Correlation:
				return 'Correlation';
			case PaneType.DataGrid:
				return 'Data Grid';
			case PaneType.Table:
				return 'Table';
			default:
				return 'Chart';
		}
	}

	/** Chart type name */
	let chartTypeName = $derived(
		hasSelectedChart ? getChartTypeName($selectedPane?.paneNode.paneType) : 'No Chart Selected'
	);

	/** Chart title */
	let chartTitle = $derived(hasSelectedChart ? ($selectedPane?.paneNode.title ?? '') : '');
</script>

<div class="chart-settings-toolbar">
	<div class="toolbar-info">
		<span class="chart-type">{chartTypeName}</span>
		{#if chartTitle}
			<span class="separator">|</span>
			<span class="chart-title">{chartTitle}</span>
		{/if}
	</div>

	{#if hasSelectedChart}
		<div class="settings-hint">
			<svg viewBox="0 0 24 24" class="hint-icon" aria-hidden="true">
				<path
					fill="currentColor"
					d="M11 7h2v2h-2zm0 4h2v6h-2zm1-9C6.48 2 2 6.48 2 12s4.48 10 10 10s10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8s8 3.59 8 8s-3.59 8-8 8z"
				/>
			</svg>
			<span class="hint-text">Settings in right panel</span>
		</div>
	{/if}
</div>

<style>
	.chart-settings-toolbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 12px;
		padding: 6px 12px;
		background: hsl(var(--muted));
		border-bottom: 1px solid hsl(var(--border));
		min-height: 36px;
	}

	.toolbar-info {
		display: flex;
		align-items: center;
		gap: 8px;
		min-width: 0;
		flex: 1;
	}

	.chart-type {
		font-size: 11px;
		font-weight: 600;
		color: hsl(var(--muted-foreground));
		text-transform: uppercase;
		letter-spacing: 0.05em;
		white-space: nowrap;
	}

	.separator {
		color: hsl(var(--border));
	}

	.chart-title {
		font-size: 13px;
		font-weight: 500;
		color: hsl(var(--foreground));
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.settings-hint {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 4px 8px;
		border-radius: 4px;
		background: hsl(var(--background));
		color: hsl(var(--muted-foreground));
	}

	.hint-icon {
		width: 14px;
		height: 14px;
		opacity: 0.6;
	}

	.hint-text {
		font-size: 11px;
		white-space: nowrap;
	}
</style>
