<script lang="ts">
	/**
	 * D3WellLogTrack - Well log track visualization using D3.js
	 *
	 * Features matching the reference screenshot:
	 * - Header with title, unit, and scale range
	 * - Vertical grid lines
	 * - Green curve line (GR style)
	 * - Yellow area fill for sand zones (low GR)
	 * - Lithology labels (Sand/Shale)
	 *
	 * Uses Svelte 5 runes and D3.js for SVG rendering.
	 */
	import { untrack } from 'svelte'
	import * as d3 from 'd3'
	import {
		createDepthScale,
		createValueScale,
		createCurveLineGenerator,
		createCurveAreaGenerator,
		segmentsToDataPoints,
		detectLithologyZones,
		getD3CurveColor,
		type WellLogDataPoint,
		type LithologyZone,
		type TrackConfig,
		type DepthRange
	} from '$lib/charts/d3-utils'

	// =========================================================================
	// Props
	// =========================================================================

	interface Props {
		/** Segmented curve data from backend */
		segments?: Array<{
			depths: number[]
			values: number[]
		}>
		/** Raw data points (alternative to segments) */
		data?: WellLogDataPoint[]
		/** Depth range for Y-axis */
		depthRange: DepthRange
		/** Track configuration */
		config: TrackConfig
		/** Track width in pixels */
		width?: number
		/** Track height in pixels */
		height?: number
		/** Show lithology labels */
		showLithologyLabels?: boolean
		/** GR cutoff for sand/shale classification */
		grCutoff?: number
		/** Minimum zone thickness for labels (in depth units) */
		minZoneThickness?: number
	}

	let {
		segments,
		data: rawData,
		depthRange,
		config,
		width = 200,
		height = 600,
		showLithologyLabels = true,
		grCutoff = 75,
		minZoneThickness = 10
	}: Props = $props()

	// =========================================================================
	// Layout Constants
	// =========================================================================

	const HEADER_HEIGHT = 40
	const PADDING = { top: 5, right: 5, bottom: 5, left: 5 }

	// =========================================================================
	// Derived Values
	// =========================================================================

	/** Convert segments to data points, or use raw data */
	let dataPoints = $derived.by((): WellLogDataPoint[] => {
		if (segments && segments.length > 0) {
			return segmentsToDataPoints(segments)
		}
		return rawData ?? []
	})

	/** Chart dimensions (excluding header and padding) */
	let chartWidth = $derived(width - PADDING.left - PADDING.right)
	let chartHeight = $derived(height - HEADER_HEIGHT - PADDING.top - PADDING.bottom)

	/** D3 Scales */
	let yScale = $derived(createDepthScale(depthRange, chartHeight))
	let xScale = $derived(createValueScale(config.xMin, config.xMax, chartWidth, config.logScale))

	/** Line generator */
	let lineGenerator = $derived(createCurveLineGenerator(xScale, yScale))

	/** Area generator (for fills) */
	let areaGenerator = $derived.by(() => {
		if (config.fillDirection === 'none' || !config.fillColor) {
			return null
		}
		return createCurveAreaGenerator(xScale, yScale, config.fillDirection, chartWidth)
	})

	/** Generate path data for curve line */
	let linePath = $derived(lineGenerator(dataPoints))

	/** Generate path data for area fill */
	let areaPath = $derived.by(() => {
		if (!areaGenerator) return null
		return areaGenerator(dataPoints)
	})

	/** Detect lithology zones */
	let lithologyZones = $derived.by((): LithologyZone[] => {
		if (!showLithologyLabels || dataPoints.length === 0) return []
		return detectLithologyZones(dataPoints, grCutoff, minZoneThickness)
	})

	/** Vertical grid line positions */
	let verticalGridLines = $derived.by(() => {
		const ticks = xScale.ticks(5)
		return ticks.map(tick => ({
			x: xScale(tick),
			label: tick
		}))
	})

	/** Horizontal grid line positions */
	let horizontalGridLines = $derived.by(() => {
		const ticks = yScale.ticks(10)
		return ticks.map(tick => ({
			y: yScale(tick),
			label: tick
		}))
	})

	// =========================================================================
	// SVG Rendering
	// =========================================================================

	let svgElement: SVGSVGElement
	// Use a regular variable instead of $state to avoid reactive loops
	let isInitialized = false

	$effect(() => {
		if (!svgElement || isInitialized) return

		untrack(() => {
			isInitialized = true
			// Initial D3 setup if needed (currently handled declaratively)
		})

		return () => {
			isInitialized = false
		}
	})
</script>

<div class="d3-welllog-track" style="width: {width}px; height: {height}px;">
	<!-- Header -->
	<div class="track-header" style="height: {HEADER_HEIGHT}px;">
		<div class="header-title">{config.title}</div>
		<div class="header-scale">
			<span class="scale-min">{config.xMin}</span>
			<span class="scale-unit">{config.unit}</span>
			<span class="scale-max">{config.xMax}</span>
		</div>
		<div class="header-line" style="background-color: {config.curveColor};"></div>
	</div>

	<!-- Chart Area -->
	<svg
		bind:this={svgElement}
		width={width}
		height={height - HEADER_HEIGHT}
		class="track-chart"
	>
		<g transform="translate({PADDING.left}, {PADDING.top})">
			<!-- Background grid -->
			<g class="grid vertical-grid">
				{#each verticalGridLines as line}
					<line
						x1={line.x}
						y1={0}
						x2={line.x}
						y2={chartHeight}
						stroke="#e5e7eb"
						stroke-width="1"
					/>
				{/each}
			</g>

			<g class="grid horizontal-grid">
				{#each horizontalGridLines as line}
					<line
						x1={0}
						y1={line.y}
						x2={chartWidth}
						y2={line.y}
						stroke="#e5e7eb"
						stroke-width="1"
					/>
				{/each}
			</g>

			<!-- Lithology zones (background fills + labels) -->
			{#if showLithologyLabels}
				{#each lithologyZones as zone}
					{@const y1 = yScale(zone.depthStart)}
					{@const y2 = yScale(zone.depthEnd)}
					{@const zoneHeight = y2 - y1}
					{@const centerY = y1 + zoneHeight / 2}

					<!-- Zone fill (only for sand zones with color) -->
					{#if zone.color && zone.color !== 'transparent'}
						<!-- This is handled by the area fill below, but we can add zone-specific backgrounds here -->
					{/if}

					<!-- Zone label -->
					<text
						x={chartWidth / 2}
						y={centerY}
						text-anchor="middle"
						dominant-baseline="middle"
						class="zone-label"
						fill={zone.textColor ?? '#666'}
					>
						{zone.name}
					</text>
				{/each}
			{/if}

			<!-- Area fill (sand indicator) -->
			{#if areaPath && config.fillColor}
				<path
					d={areaPath}
					fill={config.fillColor}
					fill-opacity="0.5"
					stroke="none"
					class="curve-area"
				/>
			{/if}

			<!-- Curve line -->
			{#if linePath}
				<path
					d={linePath}
					fill="none"
					stroke={config.curveColor}
					stroke-width={config.lineWidth ?? 1.5}
					class="curve-line"
				/>
			{/if}
		</g>
	</svg>
</div>

<style>
	.d3-welllog-track {
		display: flex;
		flex-direction: column;
		background: #ffffff;
		border: 1px solid #e5e7eb;
		overflow: hidden;
	}

	.track-header {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4px 8px;
		border-bottom: 1px solid #e5e7eb;
		background: #f9fafb;
	}

	.header-title {
		font-size: 12px;
		font-weight: 600;
		color: #111827;
		text-align: center;
	}

	.header-scale {
		display: flex;
		justify-content: space-between;
		width: 100%;
		font-size: 10px;
		color: #6b7280;
		margin-top: 2px;
	}

	.scale-min {
		text-align: left;
	}

	.scale-unit {
		text-align: center;
		font-weight: 500;
	}

	.scale-max {
		text-align: right;
	}

	.header-line {
		width: 100%;
		height: 2px;
		margin-top: 2px;
	}

	.track-chart {
		flex: 1;
	}

	.curve-line {
		vector-effect: non-scaling-stroke;
	}

	.zone-label {
		font-size: 11px;
		font-family: system-ui, -apple-system, sans-serif;
		pointer-events: none;
	}
</style>
