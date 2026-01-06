<script lang="ts">
	/**
	 * TrackContextMenu - Right-click configuration menu for tracks
	 *
	 * OpenDTect-inspired pattern:
	 * - Scale mode: Linear / Log
	 * - Fill direction: Left / Right / None
	 * - Future: Color picker, line width
	 *
	 * Appears at mouse position on right-click, closes on click outside.
	 */
	import type { TrackContextMenuState, FillDirection } from '$lib/charts/correlation-types'

	interface Props {
		/** Menu state */
		state: TrackContextMenuState
		/** Current log scale setting for the track */
		isLogScale: boolean
		/** Current fill direction for the track */
		fillDirection: FillDirection
		/** Callback when scale changes */
		onScaleChange?: (trackId: string, logScale: boolean) => void
		/** Callback when fill direction changes */
		onFillChange?: (trackId: string, direction: FillDirection) => void
		/** Callback to close the menu */
		onClose: () => void
	}

	let {
		state,
		isLogScale,
		fillDirection,
		onScaleChange,
		onFillChange,
		onClose
	}: Props = $props()

	/**
	 * Handle scale button click
	 */
	function handleScaleClick(logScale: boolean): void {
		if (state.trackId) {
			onScaleChange?.(state.trackId, logScale)
		}
	}

	/**
	 * Handle fill button click
	 */
	function handleFillClick(direction: FillDirection): void {
		if (state.trackId) {
			onFillChange?.(state.trackId, direction)
		}
	}

	/**
	 * Handle click outside to close menu
	 */
	function handleBackdropClick(event: MouseEvent): void {
		event.preventDefault()
		event.stopPropagation()
		onClose()
	}

	/**
	 * Prevent clicks inside menu from closing it
	 */
	function handleMenuClick(event: MouseEvent): void {
		event.stopPropagation()
	}
</script>

{#if state.isOpen}
	<!-- Backdrop to catch outside clicks -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div class="context-menu-backdrop" onclick={handleBackdropClick}>
		<!-- Menu container -->
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="context-menu"
			style="left: {state.position.x}px; top: {state.position.y}px"
			onclick={handleMenuClick}
		>
			<!-- Scale Section -->
			<div class="menu-section">
				<div class="section-label">Scale</div>
				<div class="button-group">
					<button
						class="menu-btn"
						class:active={!isLogScale}
						onclick={() => handleScaleClick(false)}
					>
						Linear
					</button>
					<button
						class="menu-btn"
						class:active={isLogScale}
						onclick={() => handleScaleClick(true)}
					>
						Log
					</button>
				</div>
			</div>

			<!-- Fill Section -->
			<div class="menu-section">
				<div class="section-label">Fill</div>
				<div class="button-group">
					<button
						class="menu-btn"
						class:active={fillDirection === 'none'}
						onclick={() => handleFillClick('none')}
						title="No fill"
					>
						&#x25CB;
					</button>
					<button
						class="menu-btn"
						class:active={fillDirection === 'left'}
						onclick={() => handleFillClick('left')}
						title="Fill left"
					>
						&#x25C0;
					</button>
					<button
						class="menu-btn"
						class:active={fillDirection === 'right'}
						onclick={() => handleFillClick('right')}
						title="Fill right"
					>
						&#x25B6;
					</button>
					<button
						class="menu-btn"
						class:active={fillDirection === 'full'}
						onclick={() => handleFillClick('full')}
						title="Full fill"
					>
						&#x25AC;
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.context-menu-backdrop {
		position: fixed;
		inset: 0;
		z-index: 1000;
		background: transparent;
	}

	.context-menu {
		position: fixed;
		background: var(--color-popover, #ffffff);
		border: 1px solid var(--color-border, #e5e7eb);
		border-radius: 6px;
		box-shadow:
			0 4px 6px -1px rgba(0, 0, 0, 0.1),
			0 2px 4px -1px rgba(0, 0, 0, 0.06);
		padding: 8px;
		min-width: 120px;
		z-index: 1001;
	}

	.menu-section {
		margin-bottom: 8px;
	}

	.menu-section:last-child {
		margin-bottom: 0;
	}

	.section-label {
		font-size: 10px;
		font-weight: 600;
		color: var(--color-text-muted, #9ca3af);
		margin-bottom: 4px;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.button-group {
		display: flex;
		gap: 4px;
	}

	.menu-btn {
		flex: 1;
		padding: 4px 8px;
		font-size: 11px;
		font-weight: 500;
		border: 1px solid var(--color-border-light, #e5e7eb);
		border-radius: 4px;
		background: var(--color-bg, #ffffff);
		color: var(--color-text-muted, #6b7280);
		cursor: pointer;
		transition:
			background 0.15s,
			border-color 0.15s,
			color 0.15s;
	}

	.menu-btn:hover {
		background: var(--color-bg-hover, #f3f4f6);
		border-color: var(--color-border, #d1d5db);
		color: var(--color-text, #374151);
	}

	.menu-btn.active {
		background: var(--color-primary-light, #dbeafe);
		border-color: var(--color-primary, #3b82f6);
		color: var(--color-primary, #3b82f6);
	}
</style>
