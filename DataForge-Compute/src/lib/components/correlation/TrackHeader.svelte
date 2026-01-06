<script lang="ts">
	/**
	 * TrackHeader - Interactive track header component (OpenDTect-inspired)
	 *
	 * Features:
	 * - Mnemonic display with colored dot indicator
	 * - Scale range display (e.g., "0 - 150" or "0.2 - 2000")
	 * - Log/Linear toggle button
	 * - Fill direction toggle
	 * - Right-click context menu trigger
	 */
	import type { TrackCurve, TrackHeaderConfig, FillDirection } from '$lib/charts/correlation-types'
	import { DEFAULT_HEADER_CONFIG } from '$lib/charts/correlation-types'

	interface Props {
		/** Curves to display in header (normalized from track) */
		curves: TrackCurve[]
		/** X-axis minimum */
		xAxisMin: number | undefined
		/** X-axis maximum */
		xAxisMax: number | undefined
		/** Whether log scale is active */
		xAxisLogScale: boolean
		/** Header configuration */
		headerConfig?: TrackHeaderConfig
		/** Callback when scale toggle is clicked */
		onScaleToggle?: (logScale: boolean) => void
		/** Callback when fill toggle is clicked */
		onFillToggle?: (direction: FillDirection) => void
		/** Callback when right-click context menu is requested */
		onContextMenu?: (event: MouseEvent) => void
	}

	let {
		curves,
		xAxisMin,
		xAxisMax,
		xAxisLogScale,
		headerConfig = DEFAULT_HEADER_CONFIG,
		onScaleToggle,
		onFillToggle,
		onContextMenu
	}: Props = $props()

	/** Format scale range for display */
	function formatScaleRange(min: number | undefined, max: number | undefined): string {
		if (min === undefined || max === undefined) return ''
		// For log scale, format nicely
		if (xAxisLogScale) {
			return `${formatLogValue(min)} - ${formatLogValue(max)}`
		}
		// For linear, use appropriate precision
		const range = Math.abs(max - min)
		if (range >= 100) {
			return `${Math.round(min)} - ${Math.round(max)}`
		} else if (range >= 1) {
			return `${min.toFixed(1)} - ${max.toFixed(1)}`
		} else {
			return `${min.toFixed(2)} - ${max.toFixed(2)}`
		}
	}

	/** Format a value for log scale display */
	function formatLogValue(value: number): string {
		if (value >= 1000) return `${(value / 1000).toFixed(0)}k`
		if (value >= 100) return `${Math.round(value)}`
		if (value >= 1) return `${value.toFixed(0)}`
		return value.toPrecision(1)
	}

	/** Get current fill direction from first curve */
	let currentFillDirection = $derived<FillDirection>(curves[0]?.display.fillDirection ?? 'none')

	/** Cycle to next fill direction */
	function cycleFilldirection(): void {
		const order: FillDirection[] = ['none', 'left', 'right', 'full']
		const currentIndex = order.indexOf(currentFillDirection)
		const nextIndex = (currentIndex + 1) % order.length
		onFillToggle?.(order[nextIndex])
	}

	/** Get fill icon based on direction */
	function getFillIcon(direction: FillDirection): string {
		switch (direction) {
			case 'left':
				return '\u25C0' // Left triangle
			case 'right':
				return '\u25B6' // Right triangle
			case 'full':
				return '\u25AC' // Rectangle
			default:
				return '\u25CB' // Circle outline (no fill)
		}
	}

	/** Handle right-click */
	function handleContextMenu(event: MouseEvent): void {
		if (headerConfig.interactive && onContextMenu) {
			event.preventDefault()
			onContextMenu(event)
		}
	}

	/** Header height - scales with number of curves for stacked display */
	const baseHeaderHeight = 24
	let headerHeight = $derived(Math.max(baseHeaderHeight, curves.length * 16 + 8))
</script>

<div
	class="track-header"
	style="height: {headerHeight}px"
	role="banner"
	oncontextmenu={handleContextMenu}
>
	<!-- Mnemonic Display -->
	{#if headerConfig.showMnemonic}
		<div class="mnemonic-section">
			{#if curves.length === 1}
				<!-- Single curve: horizontal layout -->
				<div class="mnemonic-row">
					<span class="color-dot" style="background: {curves[0].display.color}"></span>
					<span class="mnemonic-text" style="color: {curves[0].display.color}">
						{curves[0].mnemonic}
					</span>
				</div>
			{:else}
				<!-- Multi-curve: stacked display -->
				<div class="mnemonic-stack">
					{#each curves as curve}
						<div class="mnemonic-row stacked">
							<span class="color-dot small" style="background: {curve.display.color}"></span>
							<span class="mnemonic-text small" style="color: {curve.display.color}">
								{curve.mnemonic}
							</span>
						</div>
					{/each}
				</div>
			{/if}
		</div>
	{/if}

	<!-- Scale Range and Controls -->
	<div class="controls-section">
		{#if headerConfig.showScaleRange}
			<span class="scale-range">
				{formatScaleRange(xAxisMin, xAxisMax)}
			</span>
		{/if}

		<div class="toggle-buttons">
			{#if headerConfig.showScaleToggle}
				<button
					class="toggle-btn"
					class:active={xAxisLogScale}
					onclick={() => onScaleToggle?.(!xAxisLogScale)}
					title={xAxisLogScale ? 'Switch to Linear scale' : 'Switch to Log scale'}
				>
					{xAxisLogScale ? 'LOG' : 'LIN'}
				</button>
			{/if}

			{#if headerConfig.showFillToggle}
				<button
					class="toggle-btn fill-btn"
					onclick={cycleFilldirection}
					title="Fill direction: {currentFillDirection}"
				>
					{getFillIcon(currentFillDirection)}
				</button>
			{/if}
		</div>
	</div>
</div>

<style>
	.track-header {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2px 4px;
		min-height: 24px;
		border-bottom: 1px solid var(--color-border-light, #e5e7eb);
		background: var(--color-bg, #ffffff);
		flex-shrink: 0;
		box-sizing: border-box;
		gap: 2px;
		cursor: default;
	}

	.track-header:hover {
		background: var(--color-bg-hover, #f9fafb);
	}

	/* Mnemonic Section */
	.mnemonic-section {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
	}

	.mnemonic-row {
		display: flex;
		align-items: center;
		gap: 4px;
	}

	.mnemonic-row.stacked {
		gap: 2px;
	}

	.mnemonic-stack {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1px;
	}

	.color-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.color-dot.small {
		width: 5px;
		height: 5px;
	}

	.mnemonic-text {
		font-size: 11px;
		font-weight: 600;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.mnemonic-text.small {
		font-size: 9px;
		font-weight: 500;
	}

	/* Controls Section */
	.controls-section {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		gap: 4px;
	}

	.scale-range {
		font-size: 8px;
		color: var(--color-text-muted, #9ca3af);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		flex: 1;
		text-align: center;
	}

	.toggle-buttons {
		display: flex;
		gap: 2px;
		flex-shrink: 0;
	}

	.toggle-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 20px;
		height: 14px;
		padding: 0;
		border: 1px solid var(--color-border-light, #e5e7eb);
		border-radius: 2px;
		background: var(--color-bg, #ffffff);
		color: var(--color-text-muted, #9ca3af);
		font-size: 7px;
		font-weight: 600;
		cursor: pointer;
		transition:
			background 0.15s,
			color 0.15s,
			border-color 0.15s;
	}

	.toggle-btn:hover {
		background: var(--color-bg-hover, #f3f4f6);
		border-color: var(--color-border, #d1d5db);
		color: var(--color-text, #374151);
	}

	.toggle-btn.active {
		background: var(--color-primary-light, #dbeafe);
		border-color: var(--color-primary, #3b82f6);
		color: var(--color-primary, #3b82f6);
	}

	.fill-btn {
		font-size: 9px;
		width: 16px;
	}
</style>
