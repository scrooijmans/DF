<script lang="ts">
	/**
	 * ContextToolbar - Context-sensitive toolbar panel
	 *
	 * Displays different content based on what is currently selected:
	 * - UDF selected: Shows UDF parameters and execution controls
	 * - Chart pane selected: Shows inline chart settings (ChartSettingsPanel)
	 * - Nothing selected: Shows placeholder with instructions
	 */
	import type { CurveInfo, WellInfo, SegmentedCurveData } from '$lib/types'
	import type { ChartConfiguration } from '$lib/panes/chart-configs'
	import type { MultiCurveSegmentedData } from '$lib/panes/layout-model'
	import { selectionContext } from '$lib/panes/selection-context'
	import ParameterForm from '$lib/components/ParameterForm.svelte'
	import ChartSettingsPanel from './ChartSettingsPanel.svelte'
	import { PaneType } from '$lib/panes/layout-model'

	interface Props {
		/** Available wells */
		wells: WellInfo[]
		/** Available curves for the selected well */
		curves: CurveInfo[]
		/** Selected well info */
		well: WellInfo | null
		/** Callback when well selection changes */
		onWellChange?: (wellId: string) => void
		/** Callback when chart configuration changes */
		onConfigChange?: (config: ChartConfiguration) => void
		/** Callback when segmented data changes (single curve) */
		onSegmentedDataChange?: (data: SegmentedCurveData | null) => void
		/** Callback when multi-curve data changes (D3 multi-curve) */
		onMultiCurveDataChange?: (data: MultiCurveSegmentedData | null) => void
	}

	let {
		wells,
		curves,
		well,
		onWellChange,
		onConfigChange,
		onSegmentedDataChange,
		onMultiCurveDataChange
	}: Props = $props()

	/** Selection stores */
	let selectionType = selectionContext.selectionType
	let selectedPane = selectionContext.selectedPane
	let selectedUdf = selectionContext.selectedUdf

	/** Check if selected pane is a chart type that supports inline settings */
	let isChartPane = $derived(
		$selectedPane?.paneNode.paneType === PaneType.Histogram ||
		$selectedPane?.paneNode.paneType === PaneType.WellLog ||
		$selectedPane?.paneNode.paneType === PaneType.D3WellLog ||
		$selectedPane?.paneNode.paneType === PaneType.LineChart ||
		$selectedPane?.paneNode.paneType === PaneType.ScatterChart ||
		$selectedPane?.paneNode.paneType === PaneType.CrossPlot ||
		$selectedPane?.paneNode.paneType === PaneType.Correlation
	)

	/** Get chart type name for display */
	function getChartTypeName(paneType: PaneType | undefined): string {
		switch (paneType) {
			case PaneType.LineChart:
				return 'Line Chart'
			case PaneType.ScatterChart:
				return 'Scatter Chart'
			case PaneType.Histogram:
				return 'Histogram'
			case PaneType.CrossPlot:
				return 'Cross Plot'
			case PaneType.WellLog:
				return 'Well Log'
			case PaneType.D3WellLog:
				return 'D3 Well Log'
			case PaneType.LinkedCharts:
				return 'Linked Charts'
			case PaneType.Correlation:
				return 'Correlation'
			case PaneType.DataGrid:
				return 'Data Grid'
			default:
				return 'Chart'
		}
	}
</script>

<div class="context-toolbar">
	{#if $selectionType === 'none'}
		<!-- No Selection -->
		<div class="empty-state">
			<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
				<path d="M15 15l-2 5L9 9l11 4-5 2zm0 0l5 5M7.188 2.239l.777 2.897M5.136 7.965l-2.898-.777M13.95 4.05l-2.122 2.122m-5.657 5.656l-2.12 2.122" />
			</svg>
			<h3>Select an Item</h3>
			<p>
				Click on a <strong>chart pane</strong> to configure its settings, or select a <strong>tool</strong> from the toolbox to configure parameters.
			</p>
		</div>
	{:else if $selectionType === 'udf' && $selectedUdf}
		<!-- UDF Selected - Show Parameter Form -->
		<div class="toolbar-section">
			<div class="section-header">
				<span class="section-icon udf-icon">
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
					</svg>
				</span>
				<div class="section-title-group">
					<h3 class="section-title">{$selectedUdf.name}</h3>
					<span class="section-subtitle">{$selectedUdf.category}</span>
				</div>
			</div>
			<div class="section-content">
				<ParameterForm />
			</div>
		</div>
	{:else if $selectionType === 'pane' && $selectedPane && isChartPane}
		<!-- Chart Pane Selected - Show Inline Settings -->
		<ChartSettingsPanel
			pane={$selectedPane.paneNode}
			config={$selectedPane.chartConfig ?? null}
			{wells}
			{curves}
			{well}
			onWellChange={(wellId) => onWellChange?.(wellId)}
			onConfigChange={(config) => onConfigChange?.(config)}
			onSegmentedDataChange={(data) => onSegmentedDataChange?.(data)}
			onMultiCurveDataChange={(data) => onMultiCurveDataChange?.(data)}
		/>
	{:else if $selectionType === 'pane' && $selectedPane}
		<!-- Non-chart Pane Selected - Show basic info -->
		<div class="toolbar-section">
			<div class="section-header">
				<span class="section-icon chart-icon">
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M3 3v18h18M7 16l4-4 4 4 4-8" />
					</svg>
				</span>
				<div class="section-title-group">
					<h3 class="section-title">{getChartTypeName($selectedPane.paneNode.paneType)}</h3>
					<span class="section-subtitle">{$selectedPane.paneNode.title}</span>
				</div>
			</div>
			<div class="section-content">
				<div class="settings-hint">
					<p>This pane type doesn't have configurable settings.</p>
				</div>
			</div>
		</div>
	{:else if $selectionType === 'curve'}
		<!-- Curve Selected - Show Curve Info (placeholder for now) -->
		<div class="toolbar-section">
			<div class="section-header">
				<span class="section-icon curve-icon">
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
					</svg>
				</span>
				<div class="section-title-group">
					<h3 class="section-title">Curve Details</h3>
					<span class="section-subtitle">View curve information</span>
				</div>
			</div>
			<div class="section-content">
				<p class="placeholder-text">Curve details panel coming soon...</p>
			</div>
		</div>
	{/if}
</div>

<style>
	.context-toolbar {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: hsl(var(--background));
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 32px 24px;
		text-align: center;
		color: hsl(var(--muted-foreground));
		height: 100%;
	}

	.empty-state svg {
		margin-bottom: 16px;
		opacity: 0.4;
	}

	.empty-state h3 {
		font-size: 14px;
		font-weight: 600;
		color: hsl(var(--foreground));
		margin: 0 0 8px 0;
	}

	.empty-state p {
		font-size: 13px;
		line-height: 1.5;
		margin: 0;
		max-width: 240px;
	}

	.empty-state strong {
		color: hsl(var(--foreground));
	}

	.toolbar-section {
		display: flex;
		flex-direction: column;
		height: 100%;
		overflow: hidden;
	}

	.section-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		border-bottom: 1px solid hsl(var(--border));
		background: hsl(var(--muted) / 0.3);
	}

	.section-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border-radius: 8px;
		flex-shrink: 0;
	}

	.udf-icon {
		background: hsl(var(--primary) / 0.1);
		color: hsl(var(--primary));
	}

	.chart-icon {
		background: hsl(142 76% 36% / 0.1);
		color: hsl(142 76% 36%);
	}

	.curve-icon {
		background: hsl(38 92% 50% / 0.1);
		color: hsl(38 92% 50%);
	}

	.section-title-group {
		flex: 1;
		min-width: 0;
	}

	.section-title {
		font-size: 14px;
		font-weight: 600;
		color: hsl(var(--foreground));
		margin: 0;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.section-subtitle {
		font-size: 12px;
		color: hsl(var(--muted-foreground));
	}

	.section-content {
		flex: 1;
		overflow-y: auto;
	}

	.settings-hint {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 32px 24px;
		text-align: center;
		color: hsl(var(--muted-foreground));
		height: 100%;
	}

	.settings-hint p {
		font-size: 13px;
		line-height: 1.5;
		margin: 0;
		max-width: 200px;
	}

	.placeholder-text {
		padding: 24px;
		font-size: 13px;
		color: hsl(var(--muted-foreground));
		text-align: center;
	}
</style>
