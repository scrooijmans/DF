/**
 * D3 Utility Functions Template
 *
 * Copy and adapt these utilities for new D3 chart types.
 * Place in: src/lib/charts/[chart-type]-utils.ts
 */

import * as d3 from 'd3'

// ============================================================================
// Types
// ============================================================================

export interface DataPoint {
	x: number
	y: number
}

export interface ChartConfig {
	title: string
	xMin: number
	xMax: number
	yMin: number
	yMax: number
	color: string
	lineWidth?: number
}

export interface AxisRange {
	min: number
	max: number
}

// ============================================================================
// Scale Generators
// ============================================================================

/**
 * Create a linear X scale
 */
export function createXScale(
	domain: AxisRange,
	width: number
): d3.ScaleLinear<number, number> {
	return d3.scaleLinear()
		.domain([domain.min, domain.max])
		.range([0, width])
}

/**
 * Create a linear Y scale (inverted for SVG)
 */
export function createYScale(
	domain: AxisRange,
	height: number
): d3.ScaleLinear<number, number> {
	return d3.scaleLinear()
		.domain([domain.min, domain.max])
		.range([height, 0]) // Inverted: top is 0, bottom is height
}

/**
 * Create a logarithmic scale (for resistivity, etc.)
 */
export function createLogScale(
	domain: AxisRange,
	width: number
): d3.ScaleLogarithmic<number, number> {
	return d3.scaleLog()
		.domain([Math.max(0.001, domain.min), domain.max])
		.range([0, width])
		.clamp(true)
}

/**
 * Create an inverted depth scale (depth increases downward)
 */
export function createDepthScale(
	domain: AxisRange,
	height: number
): d3.ScaleLinear<number, number> {
	return d3.scaleLinear()
		.domain([domain.min, domain.max])
		.range([0, height]) // NOT inverted: top is min depth, bottom is max
}

// ============================================================================
// Line and Area Generators
// ============================================================================

/**
 * Create a line generator for XY data
 */
export function createLineGenerator(
	xScale: d3.ScaleLinear<number, number> | d3.ScaleLogarithmic<number, number>,
	yScale: d3.ScaleLinear<number, number>
): d3.Line<DataPoint> {
	return d3.line<DataPoint>()
		.x(d => xScale(d.x))
		.y(d => yScale(d.y))
		.defined(d => !isNaN(d.x) && !isNaN(d.y) && d.x !== null && d.y !== null)
		.curve(d3.curveLinear)
}

/**
 * Create an area generator for filled regions
 */
export function createAreaGenerator(
	xScale: d3.ScaleLinear<number, number>,
	yScale: d3.ScaleLinear<number, number>,
	baseline: 'bottom' | 'top' | number = 'bottom'
): d3.Area<DataPoint> {
	const baselineY = typeof baseline === 'number'
		? baseline
		: baseline === 'bottom'
			? yScale.range()[0]
			: yScale.range()[1]

	return d3.area<DataPoint>()
		.x(d => xScale(d.x))
		.y0(baselineY)
		.y1(d => yScale(d.y))
		.defined(d => !isNaN(d.x) && !isNaN(d.y))
		.curve(d3.curveLinear)
}

// ============================================================================
// Axis Generators
// ============================================================================

/**
 * Create a bottom axis
 */
export function createBottomAxis(
	scale: d3.ScaleLinear<number, number>,
	tickCount: number = 5
): d3.Axis<d3.NumberValue> {
	return d3.axisBottom(scale).ticks(tickCount)
}

/**
 * Create a left axis
 */
export function createLeftAxis(
	scale: d3.ScaleLinear<number, number>,
	tickCount: number = 5
): d3.Axis<d3.NumberValue> {
	return d3.axisLeft(scale).ticks(tickCount)
}

// ============================================================================
// Data Transformation
// ============================================================================

/**
 * Convert segmented data to flat array of points
 */
export function segmentsToPoints(
	segments: Array<{ x: number[]; y: number[] }>
): DataPoint[] {
	const points: DataPoint[] = []

	for (const segment of segments) {
		for (let i = 0; i < segment.x.length; i++) {
			points.push({
				x: segment.x[i],
				y: segment.y[i]
			})
		}
		// Add gap marker between segments
		if (segment !== segments[segments.length - 1]) {
			points.push({ x: NaN, y: NaN })
		}
	}

	return points
}

/**
 * Calculate extent (min/max) of data
 */
export function calculateExtent(
	data: DataPoint[],
	accessor: (d: DataPoint) => number
): [number, number] {
	const values = data.map(accessor).filter(v => !isNaN(v))
	return [Math.min(...values), Math.max(...values)]
}

/**
 * Add padding to extent
 */
export function padExtent(
	extent: [number, number],
	padding: number = 0.1
): [number, number] {
	const range = extent[1] - extent[0]
	const pad = range * padding
	return [extent[0] - pad, extent[1] + pad]
}

// ============================================================================
// Color Utilities
// ============================================================================

/** Default color palette */
export const CHART_COLORS = {
	primary: '#3b82f6',
	secondary: '#10b981',
	tertiary: '#f59e0b',
	danger: '#ef4444',
	neutral: '#6b7280'
}

/**
 * Get color from palette by index
 */
export function getColorByIndex(index: number): string {
	const colors = Object.values(CHART_COLORS)
	return colors[index % colors.length]
}

/**
 * Create a color scale for continuous data
 */
export function createColorScale(
	domain: [number, number],
	colors: [string, string] = ['#3b82f6', '#ef4444']
): d3.ScaleLinear<string, string> {
	return d3.scaleLinear<string>()
		.domain(domain)
		.range(colors)
}

// ============================================================================
// Grid Helpers
// ============================================================================

/**
 * Get grid line positions from scale
 */
export function getGridPositions(
	scale: d3.ScaleLinear<number, number>,
	count: number = 5
): number[] {
	return scale.ticks(count).map(tick => scale(tick))
}

/**
 * Generate grid line data
 */
export function createGridLines(
	scale: d3.ScaleLinear<number, number>,
	length: number,
	orientation: 'horizontal' | 'vertical',
	count: number = 5
): Array<{ position: number; label: number }> {
	return scale.ticks(count).map(tick => ({
		position: scale(tick),
		label: tick
	}))
}
