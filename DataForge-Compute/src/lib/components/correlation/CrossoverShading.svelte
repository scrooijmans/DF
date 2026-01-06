<script lang="ts">
	/**
	 * CrossoverShading - SVG overlay for NPHI/RHOB crossover visualization
	 *
	 * OpenDTect-inspired pattern:
	 * - Detects crossover regions between two curves
	 * - Shades regions where curve1 > curve2 (when normalized)
	 * - Common use: NPHI/RHOB crossover indicates gas-bearing zones
	 *
	 * Algorithm:
	 * 1. Normalize both curves to 0-1 using their display ranges
	 * 2. Find sign changes in (normalized_curve1 - normalized_curve2)
	 * 3. Build SVG polygon regions between crossover points
	 * 4. Shade where curve1 > curve2
	 */
	import type {
		CrossoverConfig,
		CrossoverRegion,
		CorrelationCurveData,
		TrackCurve
	} from '$lib/charts/correlation-types'

	interface Props {
		/** First curve configuration (e.g., NPHI) */
		curve1: TrackCurve
		/** Second curve configuration (e.g., RHOB) */
		curve2: TrackCurve
		/** Loaded data for curve1 */
		curve1Data: CorrelationCurveData | null
		/** Loaded data for curve2 */
		curve2Data: CorrelationCurveData | null
		/** Shared depth range for the track */
		depthRange: { min: number; max: number }
		/** Track width in pixels */
		trackWidth: number
		/** Track height in pixels */
		trackHeight: number
		/** Crossover configuration */
		config: CrossoverConfig
	}

	let {
		curve1,
		curve2,
		curve1Data,
		curve2Data,
		depthRange,
		trackWidth,
		trackHeight,
		config
	}: Props = $props()

	/**
	 * Normalize a value to 0-1 range based on curve display settings
	 */
	function normalizeValue(value: number, curve: TrackCurve): number {
		const min = curve.xMin ?? 0
		const max = curve.xMax ?? 1
		// Handle reversed ranges (like NPHI: 0.45 to -0.15)
		if (min > max) {
			return (min - value) / (min - max)
		}
		return (value - min) / (max - min)
	}

	/**
	 * Convert depth to Y pixel position
	 */
	function depthToY(depth: number): number {
		const ratio = (depth - depthRange.min) / (depthRange.max - depthRange.min)
		return ratio * trackHeight
	}

	/**
	 * Convert normalized value (0-1) to X pixel position
	 */
	function normalizedToX(normalized: number): number {
		return normalized * trackWidth
	}

	/**
	 * Interpolate value at a specific depth from curve data
	 */
	function interpolateAtDepth(
		curveData: CorrelationCurveData,
		depth: number
	): number | null {
		for (const segment of curveData.segments) {
			if (depth < segment.depthStart || depth > segment.depthEnd) continue

			// Find surrounding points
			for (let i = 0; i < segment.depths.length - 1; i++) {
				const d1 = segment.depths[i]
				const d2 = segment.depths[i + 1]
				if (depth >= d1 && depth <= d2) {
					// Linear interpolation
					const t = (depth - d1) / (d2 - d1)
					return segment.values[i] + t * (segment.values[i + 1] - segment.values[i])
				}
			}
		}
		return null
	}

	/**
	 * Find crossover regions between two curves
	 */
	function findCrossoverRegions(): CrossoverRegion[] {
		if (!curve1Data || !curve2Data) return []
		if (!config.enabled) return []

		const regions: CrossoverRegion[] = []

		// Sample at regular depth intervals
		const depthStep = (depthRange.max - depthRange.min) / 500
		let inCrossover = false
		let regionStart = 0
		let regionPoints: Array<{ x1: number; x2: number; depth: number }> = []

		for (let depth = depthRange.min; depth <= depthRange.max; depth += depthStep) {
			const v1 = interpolateAtDepth(curve1Data, depth)
			const v2 = interpolateAtDepth(curve2Data, depth)

			if (v1 === null || v2 === null) {
				// Gap in data - close any open region
				if (inCrossover && regionPoints.length > 0) {
					regions.push(buildPolygonRegion(regionStart, depth - depthStep, regionPoints))
					regionPoints = []
				}
				inCrossover = false
				continue
			}

			const n1 = normalizeValue(v1, curve1)
			const n2 = normalizeValue(v2, curve2)

			// Crossover when curve1's normalized value > curve2's normalized value
			const isCrossover = n1 > n2

			if (isCrossover && !inCrossover) {
				// Start of crossover region
				inCrossover = true
				regionStart = depth
				regionPoints = []
			} else if (!isCrossover && inCrossover) {
				// End of crossover region
				if (regionPoints.length > 0) {
					regions.push(buildPolygonRegion(regionStart, depth, regionPoints))
				}
				inCrossover = false
				regionPoints = []
			}

			if (inCrossover) {
				regionPoints.push({
					x1: normalizedToX(n1),
					x2: normalizedToX(n2),
					depth
				})
			}
		}

		// Close final region if still in crossover at end
		if (inCrossover && regionPoints.length > 0) {
			regions.push(
				buildPolygonRegion(regionStart, depthRange.max, regionPoints)
			)
		}

		return regions
	}

	/**
	 * Build a polygon region from sampled points
	 */
	function buildPolygonRegion(
		startDepth: number,
		endDepth: number,
		points: Array<{ x1: number; x2: number; depth: number }>
	): CrossoverRegion {
		// Build polygon: curve1 forward, curve2 backward
		const polygon: Array<{ x: number; depth: number }> = []

		// Curve1 points (top to bottom)
		for (const p of points) {
			polygon.push({ x: p.x1, depth: p.depth })
		}

		// Curve2 points (bottom to top)
		for (let i = points.length - 1; i >= 0; i--) {
			polygon.push({ x: points[i].x2, depth: points[i].depth })
		}

		return {
			startDepth,
			endDepth,
			polygon
		}
	}

	/**
	 * Convert polygon to SVG path points string
	 */
	function polygonToPathPoints(region: CrossoverRegion): string {
		if (region.polygon.length === 0) return ''

		const points = region.polygon.map(
			(p) => `${p.x.toFixed(1)},${depthToY(p.depth).toFixed(1)}`
		)

		return points.join(' ')
	}

	/** Computed crossover regions */
	let crossoverRegions = $derived(findCrossoverRegions())
</script>

{#if config.enabled && crossoverRegions.length > 0}
	<svg
		class="crossover-overlay"
		width={trackWidth}
		height={trackHeight}
		style="z-index: {config.zIndex}"
	>
		{#each crossoverRegions as region, index (index)}
			<polygon
				points={polygonToPathPoints(region)}
				fill={config.fillColor}
				fill-opacity={config.fillOpacity}
				stroke="none"
			/>
		{/each}
	</svg>
{/if}

<style>
	.crossover-overlay {
		position: absolute;
		top: 0;
		left: 0;
		pointer-events: none;
		overflow: hidden;
	}
</style>
