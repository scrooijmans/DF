<script lang="ts">
	/**
	 * CurveTrack - Individual curve chart within a well column
	 *
	 * Supports two modes (OpenDTect-inspired):
	 * 1. Single curve: Traditional one curve per track
	 * 2. Multi-curve: Multiple curves overlaid (e.g., NPHI/RHOB crossover)
	 *
	 * Features:
	 * - X-axis: curve values (per-curve ranges supported)
	 * - Y-axis: depth (inverted, shared via linkGroup)
	 * - Fill direction: left/right/full
	 * - Mirror option: flip curve horizontally
	 * - Interactive header with scale toggles
	 */
	import type {
		CorrelationTrack,
		CorrelationCurveData,
		TrackCurve,
		FillDirection,
		TrackHeaderConfig,
		CrossoverConfig
	} from '$lib/charts/correlation-types';
	import { normalizeTrackCurves, DEFAULT_HEADER_CONFIG, DEFAULT_CROSSOVER_CONFIG } from '$lib/charts/correlation-types';
	import EChartsChart from '$lib/components/charts/EChartsChart.svelte';
	import TrackHeader from './TrackHeader.svelte';
	import CrossoverShading from './CrossoverShading.svelte';
	import type { SegmentedCurveData } from '$lib/types';

	interface Props {
		/** Track configuration */
		track: CorrelationTrack;
		/** Loaded curve data map (trackId -> data) */
		curveData: Map<string, CorrelationCurveData>;
		/** Well ID for building track IDs */
		wellId: string;
		/** Shared depth range */
		depthRange: { min: number; max: number };
		/** Track height in pixels */
		height: number;
		/** Link group for synchronization */
		linkGroup: string;
		/** Current cursor depth */
		cursorDepth: number | null;
		/** Header configuration */
		headerConfig?: TrackHeaderConfig;
		/** Callback when scale toggle is clicked */
		onScaleToggle?: (trackId: string, logScale: boolean) => void;
		/** Callback when fill toggle is clicked */
		onFillToggle?: (trackId: string, direction: FillDirection) => void;
		/** Callback when context menu is requested */
		onContextMenu?: (trackId: string, event: MouseEvent) => void;
		/** Crossover configuration (for NPHI/RHOB shading) */
		crossover?: CrossoverConfig;
	}

	let {
		track,
		curveData,
		wellId,
		depthRange,
		height,
		linkGroup,
		cursorDepth,
		headerConfig = DEFAULT_HEADER_CONFIG,
		onScaleToggle,
		onFillToggle,
		onContextMenu,
		crossover
	}: Props = $props();

	/** Normalize track to curves array (backwards compatible) */
	let curves = $derived(normalizeTrackCurves(track));

	/** Get curve data by curve ID */
	function getCurveData(curveId: string): CorrelationCurveData | null {
		// Try wellId:curveId format first (standard)
		const trackId = `${wellId}:${curveId}`;
		return curveData.get(trackId) ?? curveData.get(curveId) ?? null;
	}

	/** Convert CorrelationCurveData to SegmentedCurveData format */
	function toSegmentedData(data: CorrelationCurveData, curve: TrackCurve): SegmentedCurveData {
		return {
			curve_id: data.trackId,
			mnemonic: curve.mnemonic,
			unit: data.unit,
			segments: data.segments.map((seg) => ({
				depth_start: seg.depthStart,
				depth_end: seg.depthEnd,
				depths: seg.depths,
				values: curve.display.mirror
					? seg.values.map((v) => mirrorValue(v, curve))
					: seg.values
			})),
			depth_range: [data.depthRange.min, data.depthRange.max] as [number, number],
			total_points: data.totalPoints
		};
	}

	/** Mirror a value within the curve's range */
	function mirrorValue(value: number, curve: TrackCurve): number {
		const min = curve.xMin ?? 0;
		const max = curve.xMax ?? 100;
		return max - (value - min);
	}

	/** Get ECharts areaStyle origin from fillDirection */
	function getFillOrigin(fillDirection: FillDirection): 'start' | 'end' | 'auto' | undefined {
		switch (fillDirection) {
			case 'left':
				return 'start';
			case 'right':
				return 'end';
			case 'full':
				return 'auto';
			default:
				return undefined;
		}
	}

	/** Build segmented data for first curve with data (for EChartsChart) */
	let primarySegmentedData = $derived.by((): SegmentedCurveData | null => {
		for (const curve of curves) {
			const data = getCurveData(curve.curveId);
			if (data && data.segments.length > 0) {
				return toSegmentedData(data, curve);
			}
		}
		return null;
	});

	/** Build series configuration for all curves */
	let seriesConfig = $derived.by(() => {
		const series: Array<{
			field: string;
			color: string;
			width: number;
			areaStyle?: { color: string; opacity: number; origin: 'start' | 'end' | 'auto' };
			z: number;
		}> = [];

		curves.forEach((curve, index) => {
			const data = getCurveData(curve.curveId);
			if (!data) return;

			const fillOrigin = getFillOrigin(curve.display.fillDirection ?? 'none');

			series.push({
				field: curve.mnemonic,
				color: curve.display.color,
				width: curve.display.lineWidth ?? 1,
				// Fill configuration
				areaStyle:
					fillOrigin && curve.display.fillDirection !== 'none'
						? {
								color: curve.display.fillColor ?? curve.display.color,
								opacity: curve.display.fillOpacity ?? 0.3,
								origin: fillOrigin
							}
						: undefined,
				// Z-index: first curve on top (higher z-index)
				z: curves.length - index
			});
		});

		return series;
	});

	/** Check if any curve has data */
	let hasAnyData = $derived(curves.some((c) => getCurveData(c.curveId) !== null));

	/** Check if still loading (no data yet but curves defined) */
	let isLoading = $derived(curves.length > 0 && !hasAnyData);

	/** Header height - scales with number of curves for stacked display */
	const baseHeaderHeight = 24;
	let headerHeight = $derived(Math.max(baseHeaderHeight, curves.length * 16 + 8));
	let chartHeight = $derived(height - headerHeight);

	/** Chart ID for registration */
	let chartId = $derived(`correlation-${track.id}`);

	/** Get X-axis range (use first curve's range or track default) */
	let xAxisMin = $derived(curves[0]?.xMin ?? track.xMin);
	let xAxisMax = $derived(curves[0]?.xMax ?? track.xMax);
	let xAxisLogScale = $derived(curves[0]?.logScale ?? track.logScale ?? false);

	/** Handle scale toggle callback */
	function handleScaleToggle(logScale: boolean): void {
		onScaleToggle?.(track.id, logScale);
	}

	/** Handle fill toggle callback */
	function handleFillToggle(direction: FillDirection): void {
		onFillToggle?.(track.id, direction);
	}

	/** Handle context menu callback */
	function handleContextMenu(event: MouseEvent): void {
		onContextMenu?.(track.id, event);
	}

	// =========================================================================
	// Crossover Shading Logic
	// =========================================================================

	/** Check if crossover shading should be shown for this track */
	let showCrossover = $derived.by(() => {
		if (!crossover?.enabled) return false;
		if (!crossover.curve1Id || !crossover.curve2Id) return false;

		// Check if this track has BOTH curves
		const curveMnemonics = curves.map((c) => c.mnemonic.toUpperCase());
		const hasCurve1 = curveMnemonics.includes(crossover.curve1Id.toUpperCase());
		const hasCurve2 = curveMnemonics.includes(crossover.curve2Id.toUpperCase());

		return hasCurve1 && hasCurve2;
	});

	/** Get TrackCurve for curve1 (if exists) */
	let crossoverCurve1 = $derived.by(() => {
		if (!crossover?.curve1Id) return null;
		return curves.find((c) => c.mnemonic.toUpperCase() === crossover.curve1Id.toUpperCase()) ?? null;
	});

	/** Get TrackCurve for curve2 (if exists) */
	let crossoverCurve2 = $derived.by(() => {
		if (!crossover?.curve2Id) return null;
		return curves.find((c) => c.mnemonic.toUpperCase() === crossover.curve2Id.toUpperCase()) ?? null;
	});

	/** Get data for curve1 */
	let crossoverData1 = $derived(crossoverCurve1 ? getCurveData(crossoverCurve1.curveId) : null);

	/** Get data for curve2 */
	let crossoverData2 = $derived(crossoverCurve2 ? getCurveData(crossoverCurve2.curveId) : null);

	/** Effective crossover config (with defaults) */
	let effectiveCrossover = $derived({
		...(crossover ?? DEFAULT_CROSSOVER_CONFIG)
	});

	/** Container width for crossover shading */
	let containerWidth = $state(140);
</script>

<div class="curve-track">
	<!-- Track Header (Interactive) -->
	<TrackHeader
		{curves}
		{xAxisMin}
		{xAxisMax}
		{xAxisLogScale}
		{headerConfig}
		onScaleToggle={handleScaleToggle}
		onFillToggle={handleFillToggle}
		onContextMenu={handleContextMenu}
	/>

	<!-- Chart Area -->
	<div class="chart-container" style="height: {chartHeight}px" bind:clientWidth={containerWidth}>
		{#if hasAnyData && primarySegmentedData}
			<EChartsChart
				id={chartId}
				data={null}
				segmentedData={primarySegmentedData}
				type="welllog"
				title=""
				height={chartHeight}
				invertY={true}
				showCursor={true}
				enableZoom={true}
				{linkGroup}
				syncCursor={true}
				syncViewport={true}
				yAxisMin={depthRange.min}
				yAxisMax={depthRange.max}
				hideYAxis={true}
				{xAxisMin}
				{xAxisMax}
				{xAxisLogScale}
				xAxisPosition="top"
				disableDownsampling={true}
				yAxisOnlyZoom={true}
				series={seriesConfig}
			/>
		{:else}
			<div class="loading-state">
				{#if isLoading}
					<span class="loading-text">Loading...</span>
				{:else}
					<span class="no-data-text">No data</span>
				{/if}
			</div>
		{/if}

		<!-- Crossover shading overlay (NPHI/RHOB gas indicator) -->
		{#if showCrossover && crossoverCurve1 && crossoverCurve2}
			<CrossoverShading
				curve1={crossoverCurve1}
				curve2={crossoverCurve2}
				curve1Data={crossoverData1}
				curve2Data={crossoverData2}
				{depthRange}
				trackWidth={containerWidth}
				trackHeight={chartHeight}
				config={effectiveCrossover}
			/>
		{/if}

		<!-- Cursor crosshair overlay -->
		{#if cursorDepth !== null}
			{@const yRatio = (cursorDepth - depthRange.min) / (depthRange.max - depthRange.min)}
			{@const yPos = yRatio * chartHeight}
			{#if yPos >= 0 && yPos <= chartHeight}
				<div class="cursor-line" style="top: {yPos}px"></div>
			{/if}
		{/if}
	</div>
</div>

<style>
	.curve-track {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		overflow: hidden;
	}

	.chart-container {
		flex: 1;
		position: relative;
		overflow: hidden;
	}

	.loading-state {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		background: var(--color-bg-tertiary, #f3f4f6);
	}

	.loading-text {
		font-size: 11px;
		color: var(--color-text-tertiary, #9ca3af);
		animation: pulse 1.5s infinite;
	}

	.no-data-text {
		font-size: 11px;
		color: var(--color-text-tertiary, #9ca3af);
	}

	.cursor-line {
		position: absolute;
		left: 0;
		right: 0;
		height: 1px;
		background: rgba(59, 130, 246, 0.5);
		pointer-events: none;
		z-index: 10;
	}

	@keyframes pulse {
		0%,
		100% {
			opacity: 0.5;
		}
		50% {
			opacity: 1;
		}
	}
</style>
