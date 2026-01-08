/**
 * D3.js Utility Functions for Well Log Charts
 *
 * Provides scale generators, axis formatters, and area generators
 * optimized for geoscience well log visualization.
 */

import * as d3 from 'd3'

// ============================================================================
// Types
// ============================================================================

export interface WellLogDataPoint {
	depth: number
	value: number
}

export interface LithologyZone {
	name: string
	depthStart: number
	depthEnd: number
	color: string
	textColor?: string
}

export interface TrackConfig {
	/** Track title (e.g., "Gamma Ray") */
	title: string
	/** Unit of measurement (e.g., "gAPI") */
	unit: string
	/** Minimum value for X-axis */
	xMin: number
	/** Maximum value for X-axis */
	xMax: number
	/** Curve line color */
	curveColor: string
	/** Fill color for area (null for no fill) */
	fillColor: string | null
	/** Fill direction: 'left' fills from left edge to curve */
	fillDirection: 'left' | 'right' | 'none'
	/** Use logarithmic scale */
	logScale?: boolean
	/** Line width */
	lineWidth?: number
}

export interface DepthRange {
	min: number
	max: number
}

// ============================================================================
// Scale Generators
// ============================================================================

/**
 * Create an inverted Y scale for depth (increases downward)
 */
export function createDepthScale(
	depthRange: DepthRange,
	height: number
): d3.ScaleLinear<number, number> {
	return d3.scaleLinear()
		.domain([depthRange.min, depthRange.max])
		.range([0, height]) // Top to bottom (inverted)
}

/**
 * Create an X scale for curve values (linear or log)
 */
export function createValueScale(
	xMin: number,
	xMax: number,
	width: number,
	logScale: boolean = false
): d3.ScaleLinear<number, number> | d3.ScaleLogarithmic<number, number> {
	if (logScale) {
		return d3.scaleLog()
			.domain([Math.max(0.1, xMin), xMax])
			.range([0, width])
			.clamp(true)
	}
	return d3.scaleLinear()
		.domain([xMin, xMax])
		.range([0, width])
}

// ============================================================================
// Line and Area Generators
// ============================================================================

/**
 * Create a D3 line generator for well log curves
 */
export function createCurveLineGenerator(
	xScale: d3.ScaleLinear<number, number> | d3.ScaleLogarithmic<number, number>,
	yScale: d3.ScaleLinear<number, number>
): d3.Line<WellLogDataPoint> {
	return d3.line<WellLogDataPoint>()
		.x(d => xScale(d.value))
		.y(d => yScale(d.depth))
		.defined(d => !isNaN(d.value) && d.value !== null)
		.curve(d3.curveLinear)
}

/**
 * Create a D3 area generator for well log fills
 * Fills from baseline (left or right edge) to the curve
 */
export function createCurveAreaGenerator(
	xScale: d3.ScaleLinear<number, number> | d3.ScaleLogarithmic<number, number>,
	yScale: d3.ScaleLinear<number, number>,
	fillDirection: 'left' | 'right',
	width: number
): d3.Area<WellLogDataPoint> {
	const baseline = fillDirection === 'left' ? 0 : width

	return d3.area<WellLogDataPoint>()
		.x0(baseline) // Baseline (left or right edge)
		.x1(d => xScale(d.value)) // Curve value
		.y(d => yScale(d.depth)) // Depth position
		.defined(d => !isNaN(d.value) && d.value !== null)
		.curve(d3.curveLinear)
}

// ============================================================================
// Axis Generators
// ============================================================================

/**
 * Create a top axis for curve values (X-axis at top of track)
 */
export function createTopAxis(
	xScale: d3.ScaleLinear<number, number> | d3.ScaleLogarithmic<number, number>,
	logScale: boolean = false
): d3.Axis<d3.NumberValue> {
	const axis = d3.axisTop(xScale)

	if (logScale) {
		axis.ticks(3, '~s')
	} else {
		axis.ticks(3)
	}

	return axis
}

/**
 * Create a left axis for depth values (Y-axis)
 */
export function createDepthAxis(
	yScale: d3.ScaleLinear<number, number>
): d3.Axis<d3.NumberValue> {
	return d3.axisLeft(yScale)
		.ticks(10)
}

// ============================================================================
// Grid Line Generators
// ============================================================================

/**
 * Generate vertical grid line positions
 */
export function getVerticalGridPositions(
	xScale: d3.ScaleLinear<number, number> | d3.ScaleLogarithmic<number, number>,
	count: number = 5
): number[] {
	const ticks = xScale.ticks(count)
	return ticks.map(tick => xScale(tick))
}

/**
 * Generate horizontal grid line positions (depth intervals)
 */
export function getHorizontalGridPositions(
	yScale: d3.ScaleLinear<number, number>,
	count: number = 10
): number[] {
	const ticks = yScale.ticks(count)
	return ticks.map(tick => yScale(tick))
}

// ============================================================================
// Data Transformation
// ============================================================================

/**
 * Convert SegmentedCurveData segments to flat array of data points
 */
export function segmentsToDataPoints(
	segments: Array<{
		depths: number[]
		values: number[]
	}>
): WellLogDataPoint[] {
	const points: WellLogDataPoint[] = []

	for (const segment of segments) {
		for (let i = 0; i < segment.depths.length; i++) {
			points.push({
				depth: segment.depths[i],
				value: segment.values[i]
			})
		}
		// Add a gap marker between segments (NaN value)
		if (segment !== segments[segments.length - 1]) {
			const lastDepth = segment.depths[segment.depths.length - 1]
			points.push({ depth: lastDepth + 0.001, value: NaN })
		}
	}

	return points
}

/**
 * Detect lithology zones based on GR cutoff
 * Low GR (<threshold) = Sand, High GR (>threshold) = Shale
 */
export function detectLithologyZones(
	data: WellLogDataPoint[],
	grCutoff: number = 75,
	minZoneThickness: number = 5
): LithologyZone[] {
	if (data.length === 0) return []

	const zones: LithologyZone[] = []
	let currentZone: {
		type: 'sand' | 'shale'
		depthStart: number
		depthEnd: number
	} | null = null

	for (const point of data) {
		if (isNaN(point.value)) continue

		const zoneType = point.value < grCutoff ? 'sand' : 'shale'

		if (!currentZone) {
			currentZone = {
				type: zoneType,
				depthStart: point.depth,
				depthEnd: point.depth
			}
		} else if (currentZone.type === zoneType) {
			currentZone.depthEnd = point.depth
		} else {
			// Zone transition - save current zone if thick enough
			const thickness = currentZone.depthEnd - currentZone.depthStart
			if (thickness >= minZoneThickness) {
				zones.push({
					name: currentZone.type === 'sand' ? 'Sand' : 'Shale',
					depthStart: currentZone.depthStart,
					depthEnd: currentZone.depthEnd,
					color: currentZone.type === 'sand' ? '#ffff99' : 'transparent',
					textColor: '#666666'
				})
			}
			// Start new zone
			currentZone = {
				type: zoneType,
				depthStart: point.depth,
				depthEnd: point.depth
			}
		}
	}

	// Handle last zone
	if (currentZone) {
		const thickness = currentZone.depthEnd - currentZone.depthStart
		if (thickness >= minZoneThickness) {
			zones.push({
				name: currentZone.type === 'sand' ? 'Sand' : 'Shale',
				depthStart: currentZone.depthStart,
				depthEnd: currentZone.depthEnd,
				color: currentZone.type === 'sand' ? '#ffff99' : 'transparent',
				textColor: '#666666'
			})
		}
	}

	return zones
}

// ============================================================================
// Color Utilities
// ============================================================================

/** Default curve colors by mnemonic */
export const D3_CURVE_COLORS: Record<string, string> = {
	GR: '#22c55e', // Green for Gamma Ray
	NPHI: '#3b82f6', // Blue for Neutron Porosity
	RHOB: '#ef4444', // Red for Density
	DT: '#f59e0b', // Orange for Sonic
	ILD: '#8b5cf6', // Purple for Deep Resistivity
	SP: '#6b7280', // Gray for Spontaneous Potential
	CALI: '#06b6d4' // Cyan for Caliper
}

/**
 * Get default color for a curve mnemonic
 */
export function getD3CurveColor(mnemonic: string): string {
	return D3_CURVE_COLORS[mnemonic.toUpperCase()] ?? '#22c55e'
}
