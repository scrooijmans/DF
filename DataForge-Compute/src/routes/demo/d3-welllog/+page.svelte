<script lang="ts">
	/**
	 * D3 Well Log Demo Page
	 *
	 * Demonstrates the D3WellLogTrack component with synthetic data
	 * that matches the reference screenshot (Gamma Ray with Sand/Shale).
	 */
	import D3WellLogTrack from '$lib/components/charts/D3WellLogTrack.svelte'
	import type { WellLogDataPoint, TrackConfig, DepthRange } from '$lib/charts/d3-utils'

	// =========================================================================
	// Generate Synthetic Well Log Data
	// =========================================================================

	/**
	 * Generate synthetic GR data that mimics the screenshot
	 * - High values (~100-120) = Shale
	 * - Low values (~20-40) = Sand
	 * - Smooth transitions with some noise
	 */
	function generateSyntheticGR(): WellLogDataPoint[] {
		const data: WellLogDataPoint[] = []
		const depthStart = 2000
		const depthEnd = 2200
		const depthStep = 0.5

		// Define lithology sequence (matches screenshot roughly)
		const lithologySequence = [
			{ startDepth: 2000, endDepth: 2040, baseGR: 110, type: 'shale' },
			{ startDepth: 2040, endDepth: 2150, baseGR: 30, type: 'sand' },
			{ startDepth: 2150, endDepth: 2200, baseGR: 115, type: 'shale' }
		]

		for (let depth = depthStart; depth <= depthEnd; depth += depthStep) {
			// Find current lithology
			let baseValue = 70
			for (const lith of lithologySequence) {
				if (depth >= lith.startDepth && depth < lith.endDepth) {
					baseValue = lith.baseGR

					// Add smooth transition at boundaries
					const transitionZone = 5
					if (depth < lith.startDepth + transitionZone) {
						const prevLith = lithologySequence.find(
							l => l.endDepth === lith.startDepth
						)
						if (prevLith) {
							const t = (depth - lith.startDepth) / transitionZone
							baseValue = prevLith.baseGR + t * (lith.baseGR - prevLith.baseGR)
						}
					}
					break
				}
			}

			// Add realistic noise (GR is wiggly)
			const noise = (Math.random() - 0.5) * 20
			const sineWiggle = Math.sin(depth * 0.3) * 8

			const value = Math.max(0, Math.min(150, baseValue + noise + sineWiggle))

			data.push({ depth, value })
		}

		return data
	}

	// =========================================================================
	// Chart Configuration
	// =========================================================================

	const grData = generateSyntheticGR()

	const grConfig: TrackConfig = {
		title: 'Gamma Ray',
		unit: 'gAPI',
		xMin: 0,
		xMax: 150,
		curveColor: '#22c55e', // Green (matches screenshot)
		fillColor: '#ffff99', // Light yellow for sand
		fillDirection: 'left',
		lineWidth: 1.5
	}

	const depthRange: DepthRange = {
		min: 2000,
		max: 2200
	}
</script>

<svelte:head>
	<title>D3 Well Log Demo | DataForge-Compute</title>
</svelte:head>

<div class="demo-container">
	<header class="demo-header">
		<h1>D3.js Well Log Chart Demo</h1>
		<p>
			Custom D3-based well log visualization matching the reference design.
			Features: area fill, lithology labels, grid lines.
		</p>
	</header>

	<main class="demo-content">
		<div class="chart-container">
			<D3WellLogTrack
				data={grData}
				{depthRange}
				config={grConfig}
				width={200}
				height={600}
				showLithologyLabels={true}
				grCutoff={75}
				minZoneThickness={10}
			/>
		</div>

		<aside class="demo-info">
			<h2>Chart Features</h2>
			<ul>
				<li>Header with track name, unit, and scale range</li>
				<li>Green curve line (D3 line generator)</li>
				<li>Yellow area fill from left edge to curve (sand indicator)</li>
				<li>Automatic lithology labels based on GR cutoff</li>
				<li>Vertical and horizontal grid lines</li>
				<li>Inverted depth scale (increases downward)</li>
			</ul>

			<h2>Data Summary</h2>
			<dl>
				<dt>Depth Range</dt>
				<dd>{depthRange.min} - {depthRange.max} m</dd>
				<dt>Data Points</dt>
				<dd>{grData.length}</dd>
				<dt>GR Range</dt>
				<dd>{grConfig.xMin} - {grConfig.xMax} {grConfig.unit}</dd>
			</dl>

			<h2>Configuration</h2>
			<pre>{JSON.stringify(grConfig, null, 2)}</pre>
		</aside>
	</main>
</div>

<style>
	.demo-container {
		min-height: 100vh;
		padding: 24px;
		background: #f3f4f6;
	}

	.demo-header {
		max-width: 1200px;
		margin: 0 auto 24px;
	}

	.demo-header h1 {
		font-size: 24px;
		font-weight: 700;
		color: #111827;
		margin: 0 0 8px;
	}

	.demo-header p {
		color: #6b7280;
		margin: 0;
	}

	.demo-content {
		display: flex;
		gap: 32px;
		max-width: 1200px;
		margin: 0 auto;
		align-items: flex-start;
	}

	.chart-container {
		background: white;
		border-radius: 8px;
		padding: 16px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.demo-info {
		flex: 1;
		max-width: 400px;
	}

	.demo-info h2 {
		font-size: 16px;
		font-weight: 600;
		color: #111827;
		margin: 0 0 12px;
		padding-bottom: 8px;
		border-bottom: 1px solid #e5e7eb;
	}

	.demo-info h2:not(:first-child) {
		margin-top: 24px;
	}

	.demo-info ul {
		margin: 0;
		padding-left: 20px;
		color: #374151;
	}

	.demo-info li {
		margin-bottom: 6px;
	}

	.demo-info dl {
		display: grid;
		grid-template-columns: auto 1fr;
		gap: 8px 16px;
		margin: 0;
	}

	.demo-info dt {
		font-weight: 500;
		color: #6b7280;
	}

	.demo-info dd {
		margin: 0;
		color: #111827;
	}

	.demo-info pre {
		background: #1f2937;
		color: #f9fafb;
		padding: 12px;
		border-radius: 6px;
		font-size: 12px;
		overflow-x: auto;
	}
</style>
