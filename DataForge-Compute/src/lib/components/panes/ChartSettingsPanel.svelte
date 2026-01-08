<script lang="ts">
	/**
	 * ChartSettingsPanel - Inline chart settings for right sidebar
	 *
	 * Displays chart configuration directly in the right panel when a chart pane
	 * is selected. Uses tabs for Data/Style/Axes navigation.
	 *
	 * Uses extracted components from settings/:
	 * - SingleWellDataTab (histogram, welllog, d3-welllog)
	 * - MultiWellDataTab (crossplot, scatter, line)
	 * - D3WellLogStyleSection
	 * - DepthSettingsSection, XAxisRangeSection
	 */
	import { onDestroy } from 'svelte'
	import type { CurveInfo, WellInfo, SegmentedCurveData, CurveInfoWithWell } from '$lib/types'
	import type { PaneNode, MultiCurveSegmentedData } from '$lib/panes/layout-model'
	import { PaneType } from '$lib/panes/layout-model'
	import type {
		ChartConfiguration,
		HistogramConfig,
		WellLogConfig,
		D3WellLogConfig,
		D3CurveBinding,
		AxisBinding
	} from '$lib/panes/chart-configs'
	import {
		createDefaultHistogramConfig,
		createDefaultWellLogConfig,
		createDefaultD3WellLogConfig,
		getChartTypeName
	} from '$lib/panes/chart-configs'
	import { loadSegmentedCurveData } from '$lib/stores/dataStore'
	import { allWorkspaceCurves } from '$lib/stores/compute'
	import {
		SingleWellDataTab,
		D3WellLogStyleSection,
		DepthSettingsSection,
		D3CurveSettingsSection
	} from '$lib/components/charts/settings'

	interface Props {
		/** Selected pane node */
		pane: PaneNode
		/** Current chart configuration */
		config: ChartConfiguration | null
		/** Available wells */
		wells: WellInfo[]
		/** Available curves for the selected well */
		curves: CurveInfo[]
		/** Selected well info */
		well: WellInfo | null
		/** Callback when well selection changes */
		onWellChange?: (wellId: string) => void
		/** Callback when configuration changes */
		onConfigChange: (config: ChartConfiguration) => void
		/** Callback when segmented chart data changes (single curve) */
		onSegmentedDataChange?: (data: SegmentedCurveData | null) => void
		/** Callback when multi-curve segmented data changes (D3 multi-curve) */
		onMultiCurveDataChange?: (data: MultiCurveSegmentedData | null) => void
	}

	let {
		pane,
		config,
		wells,
		curves,
		well,
		onWellChange,
		onConfigChange,
		onSegmentedDataChange,
		onMultiCurveDataChange
	}: Props = $props()

	/** Loading state */
	let isLoadingData = $state(false)

	/** Active tab */
	type TabId = 'data' | 'style' | 'axes'
	let activeTab = $state<TabId>('data')

	/** Workspace curves for multi-well charts */
	let workspaceCurves: CurveInfoWithWell[] = $state([])
	const unsubscribeWorkspaceCurves = allWorkspaceCurves.subscribe((value) => {
		workspaceCurves = value
	})
	onDestroy(() => unsubscribeWorkspaceCurves())

	/** Initialize config if not present - and propagate default to parent */
	let chartConfig = $derived.by(() => {
		if (config) return config
		// Create default config based on pane type
		let defaultConfig: ChartConfiguration
		switch (pane.paneType) {
			case PaneType.Histogram:
				defaultConfig = createDefaultHistogramConfig()
				break
			case PaneType.WellLog:
				defaultConfig = createDefaultWellLogConfig()
				break
			case PaneType.D3WellLog:
				defaultConfig = createDefaultD3WellLogConfig()
				break
			default:
				defaultConfig = createDefaultHistogramConfig()
		}
		return defaultConfig
	})

	// When config is missing, propagate the default to parent on first render
	$effect(() => {
		if (!config && chartConfig) {
			// Use setTimeout to avoid updating during render
			setTimeout(() => onConfigChange(chartConfig), 0)
		}
	})

	/** Check chart type */
	let isSingleWellChart = $derived(
		pane.paneType === PaneType.Histogram ||
		pane.paneType === PaneType.WellLog ||
		pane.paneType === PaneType.D3WellLog
	)

	let isD3WellLog = $derived(pane.paneType === PaneType.D3WellLog)

	/** D3 Well Log specific config */
	let d3Config = $derived(
		chartConfig?.type === 'd3-welllog' ? chartConfig as D3WellLogConfig : null
	)

	/** Available tabs based on chart type */
	let availableTabs = $derived.by(() => {
		const tabs: { id: TabId; label: string; icon: string }[] = [
			{ id: 'data', label: 'Data', icon: 'M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z' }
		]
		if (isD3WellLog) {
			tabs.push(
				{ id: 'style', label: 'Style', icon: 'M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01' },
				{ id: 'axes', label: 'Axes', icon: 'M3 3v18h18M3 15h4v6H3v-6zm6-5h4v11H9V10zm6-7h4v18h-4V3z' }
			)
		}
		return tabs
	})

	/**
	 * Handle axis binding change from SingleWellDataTab
	 */
	async function handleAxisChange(key: string, binding: AxisBinding | null): Promise<void> {
		if (!chartConfig || !binding) return

		console.log('[ChartSettingsPanel] handleAxisChange:', { key, binding, chartType: chartConfig.type })

		if (chartConfig.type === 'histogram' && key === 'dataCurve') {
			const newConfig = { ...chartConfig, dataCurve: binding } as HistogramConfig
			onConfigChange(newConfig)
		} else if (chartConfig.type === 'welllog' && key === 'curve') {
			const newConfig = { ...chartConfig, curve: binding } as WellLogConfig
			onConfigChange(newConfig)
			await loadWellLogData(newConfig)
		} else if (chartConfig.type === 'd3-welllog' && key === 'curve') {
			const newConfig = { ...chartConfig, curve: binding } as D3WellLogConfig
			console.log('[ChartSettingsPanel] D3 Well Log curve selected, loading data for curve:', binding.curveId)
			onConfigChange(newConfig)
			await loadD3WellLogData(newConfig)
		}
	}

	/**
	 * Load D3 well log data (segmented format)
	 */
	async function loadD3WellLogData(currentConfig: D3WellLogConfig): Promise<void> {
		const curveId = currentConfig.curve?.curveId
		console.log('[ChartSettingsPanel] loadD3WellLogData called:', { curveId })

		if (!curveId) {
			console.log('[ChartSettingsPanel] No curve ID, clearing data')
			onSegmentedDataChange?.(null)
			return
		}

		isLoadingData = true
		try {
			console.log('[ChartSettingsPanel] Fetching segmented data for curve:', curveId)
			const segmentedData = await loadSegmentedCurveData(curveId)
			console.log('[ChartSettingsPanel] Received segmented data:', {
				hasData: !!segmentedData,
				segmentCount: segmentedData?.segments?.length ?? 0,
				depthRange: segmentedData?.depth_range,
				mnemonic: segmentedData?.mnemonic
			})

			if (segmentedData && segmentedData.segments.length > 0) {
				console.log('[ChartSettingsPanel] Calling onSegmentedDataChange with data')
				onSegmentedDataChange?.(segmentedData)
			} else {
				console.log('[ChartSettingsPanel] No valid data, calling onSegmentedDataChange with null')
				onSegmentedDataChange?.(null)
			}
		} catch (error) {
			console.error('[ChartSettingsPanel] Failed to load D3 well log data:', error)
			onSegmentedDataChange?.(null)
		} finally {
			isLoadingData = false
		}
	}

	/**
	 * Load well log data (segmented format)
	 */
	async function loadWellLogData(currentConfig: WellLogConfig): Promise<void> {
		const curveId = currentConfig.curve?.curveId
		if (!curveId) {
			onSegmentedDataChange?.(null)
			return
		}

		isLoadingData = true
		try {
			const segmentedData = await loadSegmentedCurveData(curveId)
			if (segmentedData && segmentedData.segments.length > 0) {
				onSegmentedDataChange?.(segmentedData)
			} else {
				onSegmentedDataChange?.(null)
			}
		} catch (error) {
			console.error('[ChartSettingsPanel] Failed to load well log data:', error)
			onSegmentedDataChange?.(null)
		} finally {
			isLoadingData = false
		}
	}

	/**
	 * Update D3 well log style
	 */
	function handleD3StyleChange(newConfig: D3WellLogConfig): void {
		console.log('[ChartSettingsPanel] D3 style change:', newConfig)
		onConfigChange(newConfig as ChartConfiguration)
	}

	/**
	 * Update depth range
	 */
	function handleDepthRangeChange(depthRange: Partial<{ autoScale: boolean; min: number | null; max: number | null }>): void {
		if (!d3Config) return
		onConfigChange({
			...d3Config,
			depthRange: { ...d3Config.depthRange, ...depthRange }
		} as ChartConfiguration)
	}

	/**
	 * Update individual curve settings (xMin, xMax, color, lineWidth, fill)
	 */
	function handleCurveSettingChange(curveId: string, updates: Partial<D3CurveBinding>): void {
		if (!d3Config) return
		const updatedCurves = d3Config.curves.map(c =>
			c.curveId === curveId ? { ...c, ...updates } : c
		)
		onConfigChange({ ...d3Config, curves: updatedCurves } as ChartConfiguration)
	}

	/**
	 * Handle D3 multi-curve selection change from SingleWellDataTab
	 */
	async function handleD3CurvesChange(curveBindings: D3CurveBinding[]): Promise<void> {
		if (!chartConfig || chartConfig.type !== 'd3-welllog') return

		console.log('[ChartSettingsPanel] handleD3CurvesChange:', curveBindings)

		// Update config with new curves array
		const newConfig: D3WellLogConfig = {
			...(chartConfig as D3WellLogConfig),
			curves: curveBindings
		}
		onConfigChange(newConfig)

		// Load data for all selected curves
		await loadMultiCurveData(curveBindings)
	}

	/**
	 * Load data for multiple curves (D3 multi-curve mode)
	 */
	async function loadMultiCurveData(curveBindings: D3CurveBinding[]): Promise<void> {
		if (curveBindings.length === 0) {
			onMultiCurveDataChange?.(null)
			return
		}

		isLoadingData = true
		try {
			// Load data for each curve in parallel
			const loadPromises = curveBindings.map(async (binding) => {
				const segmentedData = await loadSegmentedCurveData(binding.curveId)
				return {
					curveId: binding.curveId,
					mnemonic: binding.mnemonic,
					unit: binding.unit ?? '',
					segments: segmentedData?.segments ?? [],
					depth_range: segmentedData?.depth_range ?? [0, 0] as [number, number]
				}
			})

			const curveResults = await Promise.all(loadPromises)

			// Filter out curves with no data
			const validCurves = curveResults.filter(c => c.segments.length > 0)

			if (validCurves.length === 0) {
				console.log('[ChartSettingsPanel] No valid curve data loaded')
				onMultiCurveDataChange?.(null)
				return
			}

			// Calculate combined depth range
			let minDepth = Infinity
			let maxDepth = -Infinity
			for (const curve of validCurves) {
				if (curve.depth_range[0] < minDepth) minDepth = curve.depth_range[0]
				if (curve.depth_range[1] > maxDepth) maxDepth = curve.depth_range[1]
			}

			const multiCurveData: MultiCurveSegmentedData = {
				curves: validCurves,
				combined_depth_range: [minDepth, maxDepth]
			}

			console.log('[ChartSettingsPanel] Multi-curve data loaded:', {
				curveCount: validCurves.length,
				combinedDepthRange: [minDepth, maxDepth]
			})

			onMultiCurveDataChange?.(multiCurveData)
		} catch (error) {
			console.error('[ChartSettingsPanel] Failed to load multi-curve data:', error)
			onMultiCurveDataChange?.(null)
		} finally {
			isLoadingData = false
		}
	}
</script>

<div class="settings-panel">
	<!-- Header -->
	<div class="panel-header">
		<div class="header-icon">
			<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<path d="M3 3v18h18M7 16l4-4 4 4 4-8" />
			</svg>
		</div>
		<div class="header-content">
			<h3 class="header-title">{getChartTypeName(chartConfig?.type ?? 'line')}</h3>
			<span class="header-subtitle">{pane.title}</span>
		</div>
	</div>

	<!-- Loading indicator -->
	{#if isLoadingData}
		<div class="loading-bar">
			<div class="loading-progress"></div>
		</div>
	{/if}

	<!-- Tab navigation -->
	<div class="tab-nav" role="tablist" aria-label="Chart settings tabs">
		{#each availableTabs as tab (tab.id)}
			<button
				class="tab-button"
				class:active={activeTab === tab.id}
				onclick={() => activeTab = tab.id}
				role="tab"
				aria-selected={activeTab === tab.id}
			>
				<svg viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true">
					<path d={tab.icon} />
				</svg>
				<span>{tab.label}</span>
			</button>
		{/each}
	</div>

	<!-- Tab content -->
	<div class="tab-content">
		{#if activeTab === 'data'}
			<!-- DATA Tab -->
			{#if isSingleWellChart && chartConfig}
				<SingleWellDataTab
					config={chartConfig as HistogramConfig | WellLogConfig | D3WellLogConfig}
					{wells}
					{curves}
					selectedWell={well}
					onWellChange={(wellId) => onWellChange?.(wellId)}
					onAxisChange={handleAxisChange}
					onCurvesChange={isD3WellLog ? handleD3CurvesChange : undefined}
				/>
			{:else}
				<div class="coming-soon">
					<svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
						<path d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
					</svg>
					<p>Multi-well chart settings coming soon</p>
				</div>
			{/if}
		{:else if activeTab === 'style' && d3Config}
			<!-- STYLE Tab -->
			<D3WellLogStyleSection
				config={d3Config}
				onConfigChange={handleD3StyleChange}
			/>
		{:else if activeTab === 'axes' && d3Config}
			<!-- AXES Tab -->
			<div class="axes-content">
				<DepthSettingsSection
					depthRange={d3Config.depthRange}
					onDepthRangeChange={handleDepthRangeChange}
				/>
				<div class="section-spacer"></div>
				<D3CurveSettingsSection
					curves={d3Config.curves}
					onCurveChange={handleCurveSettingChange}
				/>
			</div>
		{/if}
	</div>
</div>

<style>
	.settings-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		overflow: hidden;
	}

	.panel-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		border-bottom: 1px solid hsl(var(--border));
		background: hsl(var(--muted) / 0.3);
	}

	.header-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border-radius: 8px;
		background: hsl(var(--primary) / 0.1);
		color: hsl(var(--primary));
		flex-shrink: 0;
	}

	.header-content {
		flex: 1;
		min-width: 0;
	}

	.header-title {
		margin: 0;
		font-size: 14px;
		font-weight: 600;
		color: hsl(var(--foreground));
	}

	.header-subtitle {
		font-size: 12px;
		color: hsl(var(--muted-foreground));
	}

	.loading-bar {
		height: 2px;
		background: hsl(var(--muted));
		overflow: hidden;
	}

	.loading-progress {
		height: 100%;
		width: 30%;
		background: hsl(var(--primary));
		animation: loading 1s ease-in-out infinite;
	}

	@keyframes loading {
		0% { transform: translateX(-100%); }
		100% { transform: translateX(400%); }
	}

	/* Tab Navigation */
	.tab-nav {
		display: flex;
		gap: 4px;
		padding: 8px 12px;
		border-bottom: 1px solid hsl(var(--border));
		background: hsl(var(--muted) / 0.2);
	}

	.tab-button {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 6px 12px;
		border: none;
		border-radius: 6px;
		background: transparent;
		color: hsl(var(--muted-foreground));
		font-size: 12px;
		font-weight: 500;
		cursor: pointer;
		transition: background 0.15s, color 0.15s;
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

	/* Tab Content */
	.tab-content {
		flex: 1;
		overflow-y: auto;
		padding: 12px;
	}

	.axes-content {
		display: flex;
		flex-direction: column;
		gap: 0;
	}

	.section-spacer {
		height: 16px;
	}

	.coming-soon {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 12px;
		padding: 32px 24px;
		text-align: center;
		color: hsl(var(--muted-foreground));
	}

	.coming-soon svg {
		opacity: 0.4;
	}

	.coming-soon p {
		margin: 0;
		font-size: 13px;
	}
</style>
