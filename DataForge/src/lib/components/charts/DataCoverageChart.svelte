<script lang="ts">
	/**
	 * DataCoverageChart - Visualizes data coverage/density for well curves
	 *
	 * Displays a missingno-style matrix showing where data is present or missing
	 * for each curve across the depth range. Uses native SVG for rendering.
	 *
	 * Features:
	 * - Gap detection: Shows non-contiguous segments where data exists
	 * - Depth axis: Vertical axis with depth increasing downward (geological convention)
	 * - Curve columns: Each curve gets a column showing coverage segments
	 * - Tooltips: Hover to see segment details
	 */

	import { invoke } from '@tauri-apps/api/core'

	// Types matching Rust structs
	interface DepthSegment {
		start_depth: number
		end_depth: number
		sample_count: number
	}

	interface CurveCoverage {
		id: string
		mnemonic: string
		unit: string | null
		min_depth: number | null
		max_depth: number | null
		total_samples: number
		valid_samples: number
		segments: DepthSegment[]
	}

	interface WellCoverageResult {
		well_id: string
		well_name: string
		min_depth: number
		max_depth: number
		curves: CurveCoverage[]
	}

	interface Props {
		wellId: string
		height?: number
		class?: string
	}

	let { wellId, height = 500, class: className = '' }: Props = $props()

	// State
	let coverage = $state<WellCoverageResult | null>(null)
	let loading = $state(false)
	let error = $state<string | null>(null)
	let hoveredSegment = $state<{ curve: string; segment: DepthSegment } | null>(null)

	// Dimensions
	const margin = { top: 40, right: 20, bottom: 60, left: 70 }
	const barWidth = 28
	const barGap = 4

	// Derived dimensions
	const chartWidth = $derived(
		coverage ? margin.left + coverage.curves.length * (barWidth + barGap) + margin.right : 300
	)
	const chartHeight = $derived(height)
	const plotHeight = $derived(chartHeight - margin.top - margin.bottom)
	const plotWidth = $derived(chartWidth - margin.left - margin.right)

	// Scale functions
	const depthScale = $derived(() => {
		if (!coverage || coverage.max_depth === coverage.min_depth) {
			return (_depth: number) => 0
		}
		const minDepth = coverage.min_depth
		const range = coverage.max_depth - minDepth
		return (depth: number) => ((depth - minDepth) / range) * plotHeight
	})

	// Depth ticks for axis
	const depthTicks = $derived(() => {
		if (!coverage) return []
		const range = coverage.max_depth - coverage.min_depth
		const tickCount = Math.min(10, Math.max(5, Math.floor(plotHeight / 50)))
		const step = range / (tickCount - 1)
		const ticks: number[] = []
		for (let i = 0; i < tickCount; i++) {
			ticks.push(coverage.min_depth + i * step)
		}
		return ticks
	})

	// Color palette for curves (repeating pattern)
	const colors = [
		'#3b82f6', // blue
		'#10b981', // green
		'#f59e0b', // amber
		'#ef4444', // red
		'#8b5cf6', // purple
		'#ec4899', // pink
		'#06b6d4', // cyan
		'#84cc16' // lime
	]

	function getCurveColor(index: number): string {
		return colors[index % colors.length]
	}

	// Load coverage data when wellId changes
	$effect(() => {
		if (wellId) {
			loadCoverage()
		}
	})

	async function loadCoverage() {
		loading = true
		error = null

		try {
			coverage = await invoke<WellCoverageResult>('get_curve_coverage', { wellId })
		} catch (e) {
			error = e instanceof Error ? e.message : String(e)
			console.error('Failed to load curve coverage:', e)
		} finally {
			loading = false
		}
	}

	function formatDepth(depth: number): string {
		return depth.toFixed(1)
	}

	function handleSegmentHover(curve: string, segment: DepthSegment | null) {
		if (segment) {
			hoveredSegment = { curve, segment }
		} else {
			hoveredSegment = null
		}
	}
</script>

<div class="flex flex-col {className}">
	{#if loading}
		<div class="flex h-64 items-center justify-center">
			<div class="text-muted-foreground flex items-center gap-2">
				<svg class="h-5 w-5 animate-spin" fill="none" viewBox="0 0 24 24">
					<circle
						class="opacity-25"
						cx="12"
						cy="12"
						r="10"
						stroke="currentColor"
						stroke-width="4"
					></circle>
					<path
						class="opacity-75"
						fill="currentColor"
						d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
					></path>
				</svg>
				<span>Loading coverage data...</span>
			</div>
		</div>
	{:else if error}
		<div class="flex h-64 items-center justify-center">
			<div class="text-destructive text-center">
				<p class="font-medium">Failed to load coverage</p>
				<p class="text-sm opacity-70">{error}</p>
			</div>
		</div>
	{:else if coverage && coverage.curves.length > 0}
		<div class="overflow-x-auto">
			<svg width={chartWidth} height={chartHeight} class="font-sans">
				<!-- Background -->
				<rect width={chartWidth} height={chartHeight} fill="transparent" />

				<!-- Title -->
				<text
					x={chartWidth / 2}
					y={20}
					text-anchor="middle"
					class="fill-foreground text-sm font-medium"
				>
					Data Coverage: {coverage.well_name}
				</text>

				<!-- Plot area -->
				<g transform="translate({margin.left}, {margin.top})">
					<!-- Y-axis (Depth) -->
					<line x1={0} y1={0} x2={0} y2={plotHeight} class="stroke-border" stroke-width="1" />

					<!-- Depth ticks and labels -->
					{#each depthTicks() as tick}
						{@const y = depthScale()(tick)}
						<line x1={-5} y1={y} x2={0} y2={y} class="stroke-muted-foreground" stroke-width="1" />
						<text
							x={-10}
							y={y}
							text-anchor="end"
							dominant-baseline="middle"
							class="fill-muted-foreground text-xs"
						>
							{formatDepth(tick)}
						</text>
					{/each}

					<!-- Y-axis label -->
					<text
						x={-50}
						y={plotHeight / 2}
						text-anchor="middle"
						transform="rotate(-90, -50, {plotHeight / 2})"
						class="fill-muted-foreground text-xs"
					>
						Depth
					</text>

					<!-- Curve columns -->
					{#each coverage.curves as curve, i}
						{@const x = i * (barWidth + barGap)}
						{@const color = getCurveColor(i)}

						<!-- Column background (shows gaps as empty) -->
						<rect
							{x}
							y={0}
							width={barWidth}
							height={plotHeight}
							class="fill-muted/30"
							rx="2"
						/>

						<!-- Data segments -->
						{#each curve.segments as segment}
							{@const y1 = depthScale()(segment.start_depth)}
							{@const y2 = depthScale()(segment.end_depth)}
							{@const segmentHeight = Math.max(2, y2 - y1)}

							<rect
								x={x}
								y={y1}
								width={barWidth}
								height={segmentHeight}
								fill={color}
								rx="2"
								class="cursor-pointer opacity-80 transition-opacity hover:opacity-100"
								role="img"
								aria-label="{curve.mnemonic}: {formatDepth(segment.start_depth)} - {formatDepth(
									segment.end_depth
								)}"
								onmouseenter={() => handleSegmentHover(curve.mnemonic, segment)}
								onmouseleave={() => handleSegmentHover(curve.mnemonic, null)}
							/>
						{/each}

						<!-- Curve label (bottom) -->
						<text
							x={x + barWidth / 2}
							y={plotHeight + 15}
							text-anchor="middle"
							class="fill-foreground text-xs font-medium"
						>
							{curve.mnemonic}
						</text>

						<!-- Unit label (below mnemonic) -->
						{#if curve.unit}
							<text
								x={x + barWidth / 2}
								y={plotHeight + 28}
								text-anchor="middle"
								class="fill-muted-foreground text-xs"
							>
								({curve.unit})
							</text>
						{/if}

						<!-- Sample count indicator -->
						<text
							x={x + barWidth / 2}
							y={plotHeight + 45}
							text-anchor="middle"
							class="fill-muted-foreground text-xs"
						>
							{curve.valid_samples.toLocaleString()}
						</text>
					{/each}
				</g>
			</svg>
		</div>

		<!-- Tooltip -->
		{#if hoveredSegment}
			<div
				class="bg-popover text-popover-foreground border-border fixed z-50 rounded-md border px-3 py-2 text-sm shadow-md"
				style="pointer-events: none; top: 50%; left: 50%; transform: translate(-50%, -50%);"
			>
				<div class="font-medium">{hoveredSegment.curve}</div>
				<div class="text-muted-foreground text-xs">
					{formatDepth(hoveredSegment.segment.start_depth)} - {formatDepth(
						hoveredSegment.segment.end_depth
					)}
				</div>
				<div class="text-muted-foreground text-xs">
					{hoveredSegment.segment.sample_count.toLocaleString()} samples
				</div>
			</div>
		{/if}

		<!-- Legend / Summary -->
		<div class="border-border mt-4 border-t pt-4">
			<div class="flex flex-wrap gap-4 text-xs">
				<div class="text-muted-foreground">
					<span class="font-medium">Depth Range:</span>
					{formatDepth(coverage.min_depth)} - {formatDepth(coverage.max_depth)}
				</div>
				<div class="text-muted-foreground">
					<span class="font-medium">Curves:</span>
					{coverage.curves.length}
				</div>
				<div class="flex items-center gap-2">
					<div class="h-3 w-3 rounded bg-blue-500"></div>
					<span class="text-muted-foreground">Data present</span>
				</div>
				<div class="flex items-center gap-2">
					<div class="bg-muted/30 h-3 w-3 rounded"></div>
					<span class="text-muted-foreground">No data / Gap</span>
				</div>
			</div>
		</div>
	{:else if coverage}
		<div class="text-muted-foreground flex h-64 items-center justify-center">
			<p>No curves found for this well</p>
		</div>
	{/if}
</div>
