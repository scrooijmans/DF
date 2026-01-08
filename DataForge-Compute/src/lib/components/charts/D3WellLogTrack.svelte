<script lang="ts">
	/**
	 * D3WellLogTrack - Multi-curve well log track visualization using D3.js
	 *
	 * Features:
	 * - Multiple curves with independent X-axes (like density-neutron overlay)
	 * - Stacked header rows showing scale range per curve
	 * - Vertical grid lines
	 * - Optional crossover fill between curves
	 * - Shared depth (Y) axis with visible labels
	 * - Interactive crosshair with value tooltips
	 * - Zoom/pan support for depth range
	 *
	 * Uses Svelte 5 runes and D3.js for SVG rendering.
	 */
	import { untrack } from 'svelte'
	import * as d3 from 'd3'
	import {
		createDepthScale,
		createValueScale,
		segmentsToDataPoints,
		type WellLogDataPoint,
		type DepthRange
	} from '$lib/charts/d3-utils'
	import type { D3CurveBinding, CrossoverFillConfig } from '$lib/panes/chart-configs'

	// =========================================================================
	// Types
	// =========================================================================

	interface CurveData {
		/** Curve binding configuration */
		config: D3CurveBinding
		/** Segmented curve data from backend */
		segments: Array<{
			depths: number[]
			values: number[]
		}>
	}

	interface CrosshairValue {
		mnemonic: string
		value: number
		color: string
		unit: string
	}

	// =========================================================================
	// Props
	// =========================================================================

	interface Props {
		/** Array of curve data (config + segments) */
		curvesData: CurveData[]
		/** Depth range for Y-axis */
		depthRange: DepthRange
		/** Track width in pixels */
		width?: number
		/** Track height in pixels */
		height?: number
		/** Crossover fill configuration (optional) */
		crossoverFill?: CrossoverFillConfig
		/** Well name to display in header */
		wellName?: string
		/** Callback when depth range changes via zoom/pan */
		onDepthRangeChange?: (range: DepthRange) => void
	}

	let {
		curvesData,
		depthRange,
		width = 200,
		height = 600,
		crossoverFill,
		wellName,
		onDepthRangeChange
	}: Props = $props()

	// =========================================================================
	// Layout Constants
	// =========================================================================

	const HEADER_ROW_HEIGHT = 40
	const WELL_NAME_HEIGHT = 32
	const DEPTH_AXIS_WIDTH = 45
	const VALUE_LABEL_WIDTH = 70
	const PADDING = { top: 5, right: VALUE_LABEL_WIDTH + 5, bottom: 5, left: DEPTH_AXIS_WIDTH }

	// =========================================================================
	// Derived Values
	// =========================================================================

	/** Total header height based on number of curves + optional well name */
	let headerHeight = $derived(
		(wellName ? WELL_NAME_HEIGHT : 0) +
		Math.max(HEADER_ROW_HEIGHT, curvesData.length * HEADER_ROW_HEIGHT)
	)

	/** Chart dimensions (excluding header and padding) */
	let chartWidth = $derived(width - PADDING.left - PADDING.right)
	let chartHeight = $derived(height - headerHeight - PADDING.top - PADDING.bottom)

	/** Shared Y scale (depth) */
	let yScale = $derived(createDepthScale(depthRange, chartHeight))

	/** Per-curve X scales */
	let curveScales = $derived.by(() => {
		return curvesData.map(curve => ({
			curveId: curve.config.curveId,
			xScale: createValueScale(
				curve.config.xMin,
				curve.config.xMax,
				chartWidth,
				false // linear scale
			)
		}))
	})

	/** Per-curve data points */
	let curveDataPoints = $derived.by(() => {
		return curvesData.map(curve => ({
			curveId: curve.config.curveId,
			config: curve.config,
			points: segmentsToDataPoints(curve.segments)
		}))
	})

	/** Per-curve line paths */
	let curvePaths = $derived.by(() => {
		return curveDataPoints.map(curve => {
			const scaleInfo = curveScales.find(s => s.curveId === curve.curveId)
			if (!scaleInfo) return { curveId: curve.curveId, path: null, areaPath: null, config: curve.config }

			const lineGenerator = d3.line<WellLogDataPoint>()
				.x(d => scaleInfo.xScale(d.value))
				.y(d => yScale(d.depth))
				.defined(d => !isNaN(d.value) && d.value !== null)
				.curve(d3.curveLinear)

			const path = lineGenerator(curve.points)

			// Area fill if configured
			let areaPath: string | null = null
			if (curve.config.fillDirection !== 'none' && curve.config.fillColor) {
				const baseline = curve.config.fillDirection === 'left' ? 0 : chartWidth
				const areaGenerator = d3.area<WellLogDataPoint>()
					.x0(baseline)
					.x1(d => scaleInfo.xScale(d.value))
					.y(d => yScale(d.depth))
					.defined(d => !isNaN(d.value) && d.value !== null)
					.curve(d3.curveLinear)
				areaPath = areaGenerator(curve.points)
			}

			return {
				curveId: curve.curveId,
				config: curve.config,
				path,
				areaPath
			}
		})
	})

	/** Crossover fill paths (when two curves overlap) */
	let crossoverPaths = $derived.by(() => {
		if (!crossoverFill?.enabled || curvesData.length < 2) return null

		const curve1Data = curveDataPoints.find(c => c.curveId === crossoverFill.curve1Id)
		const curve2Data = curveDataPoints.find(c => c.curveId === crossoverFill.curve2Id)
		const curve1Scale = curveScales.find(s => s.curveId === crossoverFill.curve1Id)
		const curve2Scale = curveScales.find(s => s.curveId === crossoverFill.curve2Id)

		if (!curve1Data || !curve2Data || !curve1Scale || !curve2Scale) return null

		// Build merged data points at same depths
		// For simplicity, we'll use one curve's depths and interpolate the other
		const mergedPoints: Array<{ depth: number; x1: number; x2: number }> = []

		for (const p1 of curve1Data.points) {
			if (isNaN(p1.value)) continue
			// Find closest point in curve2 at same depth (or interpolate)
			const p2 = curve2Data.points.find(p => Math.abs(p.depth - p1.depth) < 0.5)
			if (p2 && !isNaN(p2.value)) {
				mergedPoints.push({
					depth: p1.depth,
					x1: curve1Scale.xScale(p1.value),
					x2: curve2Scale.xScale(p2.value)
				})
			}
		}

		if (mergedPoints.length < 2) return null

		// Create area generator for crossover fill
		// Positive: x1 > x2, Negative: x2 > x1
		const positivePath = d3.area<{ depth: number; x1: number; x2: number }>()
			.x0(d => Math.min(d.x1, d.x2))
			.x1(d => Math.max(d.x1, d.x2))
			.y(d => yScale(d.depth))
			.defined(d => d.x1 > d.x2) // Only where curve1 > curve2
			.curve(d3.curveLinear)

		const negativePath = d3.area<{ depth: number; x1: number; x2: number }>()
			.x0(d => Math.min(d.x1, d.x2))
			.x1(d => Math.max(d.x1, d.x2))
			.y(d => yScale(d.depth))
			.defined(d => d.x2 >= d.x1) // Only where curve2 >= curve1
			.curve(d3.curveLinear)

		return {
			positive: positivePath(mergedPoints),
			negative: negativePath(mergedPoints),
			positiveColor: crossoverFill.positiveColor,
			negativeColor: crossoverFill.negativeColor,
			opacity: crossoverFill.opacity
		}
	})

	/** Vertical grid line positions (use first curve's scale) */
	let verticalGridLines = $derived.by(() => {
		if (curveScales.length === 0) return []
		const firstScale = curveScales[0].xScale
		const ticks = firstScale.ticks(5)
		return ticks.map(tick => ({
			x: firstScale(tick),
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
	// Crosshair State
	// =========================================================================

	let crosshairVisible = $state(false)
	let crosshairX = $state(0)
	let crosshairY = $state(0)
	let crosshairDepth = $state(0)
	let crosshairValues = $state<CrosshairValue[]>([])

	/** Handle mouse move for crosshair tracking */
	function handleMouseMove(event: MouseEvent) {
		const target = event.currentTarget as SVGRectElement
		const [mouseX, mouseY] = d3.pointer(event, target)

		crosshairX = mouseX
		crosshairY = mouseY

		// Convert Y position to depth
		crosshairDepth = yScale.invert(mouseY)

		// Find value at this depth for each curve using bisect
		crosshairValues = curveDataPoints.map(curve => {
			const bisect = d3.bisector((d: WellLogDataPoint) => d.depth).center
			const index = bisect(curve.points, crosshairDepth)
			const point = curve.points[index]
			return {
				mnemonic: curve.config.mnemonic,
				value: point?.value ?? NaN,
				color: curve.config.color,
				unit: curve.config.unit ?? ''
			}
		})
	}

	// =========================================================================
	// SVG Rendering & Zoom
	// =========================================================================

	let svgElement: SVGSVGElement
	let depthAxisEl: SVGGElement
	let isInitialized = false

	// Render depth axis using D3
	$effect(() => {
		if (!depthAxisEl || chartHeight <= 0) return

		const axis = d3.axisLeft(yScale)
			.ticks(Math.floor(chartHeight / 50))
			.tickFormat(d => `${d}`)

		d3.select(depthAxisEl)
			.call(axis)
			.selectAll('text')
			.attr('fill', '#6b7280')
			.attr('font-size', '10px')

		d3.select(depthAxisEl)
			.selectAll('line')
			.attr('stroke', '#d1d5db')

		d3.select(depthAxisEl)
			.select('.domain')
			.attr('stroke', '#d1d5db')
	})

	// Set up zoom behavior
	$effect(() => {
		if (!svgElement || !onDepthRangeChange) return

		const zoom = d3.zoom<SVGSVGElement, unknown>()
			.scaleExtent([0.5, 10])
			.on('zoom', (event) => {
				const newYScale = event.transform.rescaleY(yScale)
				const newMin = newYScale.domain()[0]
				const newMax = newYScale.domain()[1]
				onDepthRangeChange({ min: newMin, max: newMax })
			})

		d3.select(svgElement).call(zoom)

		return () => {
			d3.select(svgElement).on('.zoom', null)
		}
	})

	$effect(() => {
		if (!svgElement || isInitialized) return

		untrack(() => {
			isInitialized = true
		})

		return () => {
			isInitialized = false
		}
	})
</script>

<div class="d3-welllog-track" style="width: {width}px; height: {height}px;">
	<!-- Stacked Headers (one per curve) -->
	<div class="track-headers" style="height: {headerHeight}px;">
		<!-- Well Name Header (if provided) -->
		{#if wellName}
			<div class="well-name-header">
				{wellName}
			</div>
		{/if}

		<!-- Curve Headers -->
		{#each curvesData as curve, i (curve.config.curveId)}
			<div class="header-row" style="border-bottom-color: {curve.config.color};">
				<div class="header-title">{curve.config.mnemonic}</div>
				<div class="header-scale">
					<span class="scale-min">{curve.config.xMin}</span>
					<span class="scale-unit">{curve.config.unit ?? ''}</span>
					<span class="scale-max">{curve.config.xMax}</span>
				</div>
				<div class="header-line" style="background-color: {curve.config.color};"></div>
			</div>
		{/each}
		{#if curvesData.length === 0}
			<div class="header-row empty">
				<div class="header-title">No curves selected</div>
			</div>
		{/if}
	</div>

	<!-- Chart Area -->
	<svg
		bind:this={svgElement}
		width={width}
		height={height - headerHeight}
		class="track-chart"
	>
		<!-- Depth Axis -->
		<g
			bind:this={depthAxisEl}
			class="depth-axis"
			transform="translate({PADDING.left - 2}, {PADDING.top})"
		></g>

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

			<!-- Crossover fill (if enabled) -->
			{#if crossoverPaths}
				{#if crossoverPaths.positive}
					<path
						d={crossoverPaths.positive}
						fill={crossoverPaths.positiveColor}
						fill-opacity={crossoverPaths.opacity}
						stroke="none"
						class="crossover-fill positive"
					/>
				{/if}
				{#if crossoverPaths.negative}
					<path
						d={crossoverPaths.negative}
						fill={crossoverPaths.negativeColor}
						fill-opacity={crossoverPaths.opacity}
						stroke="none"
						class="crossover-fill negative"
					/>
				{/if}
			{/if}

			<!-- Area fills (per curve) -->
			{#each curvePaths as curve (curve.curveId)}
				{#if curve.areaPath && curve.config.fillColor}
					<path
						d={curve.areaPath}
						fill={curve.config.fillColor}
						fill-opacity={curve.config.fillOpacity}
						stroke="none"
						class="curve-area"
					/>
				{/if}
			{/each}

			<!-- Curve lines -->
			{#each curvePaths as curve (curve.curveId)}
				{#if curve.path}
					<path
						d={curve.path}
						fill="none"
						stroke={curve.config.color}
						stroke-width={curve.config.lineWidth}
						class="curve-line"
					/>
				{/if}
			{/each}

			<!-- Crosshair (when visible) -->
			{#if crosshairVisible && chartHeight > 0}
				<!-- Horizontal crosshair line -->
				<line
					class="crosshair-line"
					x1={0}
					y1={crosshairY}
					x2={chartWidth}
					y2={crosshairY}
					stroke="#6b7280"
					stroke-width="1"
					stroke-dasharray="4,2"
					pointer-events="none"
				/>
				<!-- Vertical crosshair line -->
				<line
					class="crosshair-line"
					x1={crosshairX}
					y1={0}
					x2={crosshairX}
					y2={chartHeight}
					stroke="#6b7280"
					stroke-width="1"
					stroke-dasharray="4,2"
					pointer-events="none"
				/>

				<!-- Depth label on Y-axis -->
				<g class="depth-label" transform="translate({-DEPTH_AXIS_WIDTH + 2}, {crosshairY})">
					<rect
						x={-2}
						y={-10}
						width={DEPTH_AXIS_WIDTH - 4}
						height={20}
						fill="#1f2937"
						rx="2"
					/>
					<text
						fill="white"
						font-size="10"
						text-anchor="end"
						x={DEPTH_AXIS_WIDTH - 10}
						dy="4"
					>{crosshairDepth.toFixed(1)}</text>
				</g>

				<!-- Value labels for each curve (on right side) -->
				{#each crosshairValues as cv, i}
					{#if !isNaN(cv.value)}
						<g class="value-label" transform="translate({chartWidth + 4}, {8 + i * 18})">
							<rect
								x={0}
								y={-8}
								width={VALUE_LABEL_WIDTH - 8}
								height={16}
								fill={cv.color}
								rx="2"
							/>
							<text fill="white" font-size="9" font-weight="500" dx="4" dy="4">
								{cv.mnemonic}: {cv.value.toFixed(1)}
							</text>
						</g>
					{/if}
				{/each}
			{/if}

			<!-- Mouse capture overlay (must be last for pointer events) -->
			<rect
				class="mouse-overlay"
				width={chartWidth}
				height={chartHeight}
				fill="transparent"
				onmousemove={handleMouseMove}
				onmouseenter={() => crosshairVisible = true}
				onmouseleave={() => crosshairVisible = false}
			/>
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

	.track-headers {
		display: flex;
		flex-direction: column;
		border-bottom: 1px solid #e5e7eb;
		background: #f9fafb;
		overflow: hidden;
	}

	.well-name-header {
		background: white;
		color: #111827;
		font-size: 12px;
		font-weight: 600;
		padding: 8px 12px;
		text-align: center;
		border-bottom: 1px solid #e5e7eb;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.header-row {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4px 8px;
		border-bottom: 2px solid transparent;
		min-height: 40px;
	}

	.header-row.empty {
		color: #9ca3af;
		font-size: 11px;
	}

	.header-title {
		font-size: 11px;
		font-weight: 600;
		color: #111827;
		text-align: center;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		max-width: 100%;
	}

	.header-scale {
		display: flex;
		justify-content: space-between;
		width: 100%;
		font-size: 9px;
		color: #6b7280;
		margin-top: 1px;
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
		margin-top: 1px;
	}

	.track-chart {
		flex: 1;
		cursor: crosshair;
	}

	.curve-line {
		vector-effect: non-scaling-stroke;
	}

	.crossover-fill {
		pointer-events: none;
	}

	.crosshair-line {
		pointer-events: none;
	}

	.depth-label,
	.value-label {
		pointer-events: none;
	}

	.mouse-overlay {
		cursor: crosshair;
	}

	/* Style depth axis */
	:global(.depth-axis .tick line) {
		stroke: #d1d5db;
	}

	:global(.depth-axis .tick text) {
		fill: #6b7280;
		font-size: 10px;
	}

	:global(.depth-axis .domain) {
		stroke: #d1d5db;
	}
</style>
