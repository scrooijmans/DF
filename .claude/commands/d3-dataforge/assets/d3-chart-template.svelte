<script lang="ts">
	/**
	 * D3 Chart Template
	 *
	 * Copy this file as a starting point for new D3 charts.
	 * Replace [ChartName] with your chart name.
	 *
	 * Key patterns:
	 * - Use $derived for D3 scales and generators (pure functions)
	 * - Use regular variables for initialization guards (NOT $state)
	 * - Wrap imperative D3 code in untrack()
	 * - Return cleanup from $effect
	 */
	import { untrack } from 'svelte'
	import * as d3 from 'd3'

	// =========================================================================
	// Types
	// =========================================================================

	interface DataPoint {
		x: number
		y: number
	}

	interface ChartConfig {
		title: string
		xMin: number
		xMax: number
		yMin: number
		yMax: number
		color: string
		lineWidth?: number
	}

	// =========================================================================
	// Props
	// =========================================================================

	interface Props {
		/** Data points to visualize */
		data: DataPoint[]
		/** Chart configuration */
		config: ChartConfig
		/** Chart width in pixels */
		width?: number
		/** Chart height in pixels */
		height?: number
	}

	let {
		data,
		config,
		width = 400,
		height = 300
	}: Props = $props()

	// =========================================================================
	// Layout Constants
	// =========================================================================

	const MARGIN = { top: 20, right: 20, bottom: 30, left: 40 }

	// =========================================================================
	// Derived Values (Safe - Pure Functions)
	// =========================================================================

	/** Chart dimensions (excluding margins) */
	let chartWidth = $derived(width - MARGIN.left - MARGIN.right)
	let chartHeight = $derived(height - MARGIN.top - MARGIN.bottom)

	/** D3 Scales - pure functions, safe to derive */
	let xScale = $derived(
		d3.scaleLinear()
			.domain([config.xMin, config.xMax])
			.range([0, chartWidth])
	)

	let yScale = $derived(
		d3.scaleLinear()
			.domain([config.yMin, config.yMax])
			.range([chartHeight, 0]) // Inverted for SVG coordinates
	)

	/** Line generator - pure function, safe to derive */
	let lineGenerator = $derived(
		d3.line<DataPoint>()
			.x(d => xScale(d.x))
			.y(d => yScale(d.y))
			.defined(d => !isNaN(d.x) && !isNaN(d.y))
	)

	/** Generated path data */
	let linePath = $derived(lineGenerator(data))

	/** Axis ticks */
	let xTicks = $derived(xScale.ticks(5))
	let yTicks = $derived(yScale.ticks(5))

	// =========================================================================
	// Imperative D3 Setup (For interactions like zoom, brush)
	// =========================================================================

	let svgElement: SVGSVGElement

	// CRITICAL: Use regular variable, NOT $state
	let isInitialized = false

	$effect(() => {
		// Guard: wait for element, skip if already initialized
		if (!svgElement || isInitialized) return

		// CRITICAL: Wrap D3 code in untrack to prevent reactive tracking
		untrack(() => {
			isInitialized = true

			// Add imperative D3 features here (brush, zoom, etc.)
			// Example:
			// const svg = d3.select(svgElement)
			// const brush = d3.brush()
			//   .extent([[0, 0], [chartWidth, chartHeight]])
			//   .on('brush', handleBrush)
			// svg.select('.brush-container').call(brush)
		})

		// CRITICAL: Return cleanup function
		return () => {
			isInitialized = false
			// Clean up D3 elements/listeners
			// d3.select(svgElement).select('.brush-container').selectAll('*').remove()
		}
	})
</script>

<!-- ======================================================================= -->
<!-- Template: Declarative SVG (Svelte renders, D3 calculates) -->
<!-- ======================================================================= -->

<div class="d3-chart" style="width: {width}px; height: {height}px;">
	<!-- Header (optional) -->
	<div class="chart-header">
		<span class="chart-title">{config.title}</span>
	</div>

	<!-- SVG Chart -->
	<svg
		bind:this={svgElement}
		{width}
		{height}
		class="chart-svg"
	>
		<g transform="translate({MARGIN.left}, {MARGIN.top})">
			<!-- X-axis -->
			<g class="axis x-axis" transform="translate(0, {chartHeight})">
				<line x1={0} y1={0} x2={chartWidth} y2={0} stroke="currentColor" />
				{#each xTicks as tick}
					<g transform="translate({xScale(tick)}, 0)">
						<line y2={6} stroke="currentColor" />
						<text y={9} dy="0.71em" text-anchor="middle" class="tick-label">
							{tick}
						</text>
					</g>
				{/each}
			</g>

			<!-- Y-axis -->
			<g class="axis y-axis">
				<line x1={0} y1={0} x2={0} y2={chartHeight} stroke="currentColor" />
				{#each yTicks as tick}
					<g transform="translate(0, {yScale(tick)})">
						<line x2={-6} stroke="currentColor" />
						<text x={-9} dy="0.32em" text-anchor="end" class="tick-label">
							{tick}
						</text>
					</g>
				{/each}
			</g>

			<!-- Grid lines (optional) -->
			<g class="grid">
				{#each yTicks as tick}
					<line
						x1={0}
						y1={yScale(tick)}
						x2={chartWidth}
						y2={yScale(tick)}
						stroke="#e5e7eb"
						stroke-dasharray="2,2"
					/>
				{/each}
			</g>

			<!-- Data visualization -->
			{#if linePath}
				<path
					d={linePath}
					fill="none"
					stroke={config.color}
					stroke-width={config.lineWidth ?? 1.5}
					class="data-line"
				/>
			{/if}

			<!-- Data points (optional) -->
			{#each data as point}
				{#if !isNaN(point.x) && !isNaN(point.y)}
					<circle
						cx={xScale(point.x)}
						cy={yScale(point.y)}
						r={3}
						fill={config.color}
						class="data-point"
					/>
				{/if}
			{/each}

			<!-- Brush container (for imperative D3) -->
			<g class="brush-container"></g>
		</g>
	</svg>
</div>

<style>
	.d3-chart {
		display: flex;
		flex-direction: column;
		background: #ffffff;
		border: 1px solid #e5e7eb;
		border-radius: 4px;
		overflow: hidden;
	}

	.chart-header {
		padding: 8px 12px;
		border-bottom: 1px solid #e5e7eb;
		background: #f9fafb;
	}

	.chart-title {
		font-size: 14px;
		font-weight: 600;
		color: #111827;
	}

	.chart-svg {
		flex: 1;
	}

	.tick-label {
		font-size: 10px;
		fill: #6b7280;
	}

	.data-line {
		vector-effect: non-scaling-stroke;
	}

	.data-point {
		transition: r 0.1s ease;
	}

	.data-point:hover {
		r: 5;
	}
</style>
