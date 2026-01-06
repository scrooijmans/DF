<script lang="ts">
	/**
	 * TrackResizeHandle - Draggable resize handle between tracks
	 *
	 * OpenDTect-inspired visual pattern:
	 * - Subtle vertical bar between tracks
	 * - Highlights on hover with primary color
	 * - Shows width tooltip during drag
	 * - Fires callback with new width on drag end
	 */
	import { TRACK_WIDTH_CONSTRAINTS } from '$lib/charts/correlation-types'

	interface Props {
		/** Current width of the track to the left of this handle */
		currentWidth: number
		/** Track ID (for callback identification) */
		trackId: string
		/** Callback when width changes */
		onWidthChange?: (trackId: string, newWidth: number) => void
	}

	let { currentWidth, trackId, onWidthChange }: Props = $props()

	/** Is the handle currently being dragged */
	let isDragging = $state(false)

	/** Temporary width during drag (for tooltip display) */
	let dragWidth = $state(currentWidth)

	/** Starting X position for drag */
	let dragStartX = 0

	/** Starting width for drag */
	let dragStartWidth = 0

	/**
	 * Handle mouse down to start resize
	 */
	function handleMouseDown(event: MouseEvent): void {
		event.preventDefault()
		event.stopPropagation()

		isDragging = true
		dragStartX = event.clientX
		dragStartWidth = currentWidth
		dragWidth = currentWidth

		// Add global listeners
		window.addEventListener('mousemove', handleMouseMove)
		window.addEventListener('mouseup', handleMouseUp)

		// Add dragging class to body for cursor
		document.body.classList.add('track-resize-dragging')
	}

	/**
	 * Handle mouse move during drag
	 */
	function handleMouseMove(event: MouseEvent): void {
		if (!isDragging) return

		const delta = event.clientX - dragStartX
		let newWidth = dragStartWidth + delta

		// Clamp to constraints
		newWidth = Math.max(TRACK_WIDTH_CONSTRAINTS.minWidth, newWidth)
		newWidth = Math.min(TRACK_WIDTH_CONSTRAINTS.maxWidth, newWidth)

		dragWidth = newWidth
	}

	/**
	 * Handle mouse up to end resize
	 */
	function handleMouseUp(): void {
		if (!isDragging) return

		isDragging = false

		// Remove global listeners
		window.removeEventListener('mousemove', handleMouseMove)
		window.removeEventListener('mouseup', handleMouseUp)
		document.body.classList.remove('track-resize-dragging')

		// Fire callback with final width
		if (dragWidth !== currentWidth) {
			onWidthChange?.(trackId, dragWidth)
		}
	}

	/**
	 * Handle double-click to reset to default width
	 */
	function handleDoubleClick(): void {
		onWidthChange?.(trackId, TRACK_WIDTH_CONSTRAINTS.defaultWidth)
	}
</script>

<div
	class="resize-handle"
	class:dragging={isDragging}
	role="separator"
	aria-orientation="vertical"
	aria-valuenow={currentWidth}
	aria-valuemin={TRACK_WIDTH_CONSTRAINTS.minWidth}
	aria-valuemax={TRACK_WIDTH_CONSTRAINTS.maxWidth}
	tabindex="0"
	onmousedown={handleMouseDown}
	ondblclick={handleDoubleClick}
	title="Drag to resize track width. Double-click to reset."
>
	<div class="handle-line"></div>

	{#if isDragging}
		<div class="width-tooltip">
			{Math.round(dragWidth)}px
		</div>
	{/if}
</div>

<style>
	.resize-handle {
		position: relative;
		width: 6px;
		height: 100%;
		cursor: col-resize;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 10;
		background: transparent;
		transition: background 0.15s;
	}

	.resize-handle:hover,
	.resize-handle.dragging {
		background: var(--color-primary-light, rgba(59, 130, 246, 0.1));
	}

	.handle-line {
		width: 2px;
		height: 100%;
		background: var(--color-border-light, #e5e7eb);
		border-radius: 1px;
		transition:
			background 0.15s,
			width 0.15s;
	}

	.resize-handle:hover .handle-line,
	.resize-handle.dragging .handle-line {
		background: var(--color-primary, #3b82f6);
		width: 3px;
	}

	.width-tooltip {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		background: var(--color-popover, #ffffff);
		border: 1px solid var(--color-border, #d1d5db);
		border-radius: 4px;
		padding: 2px 6px;
		font-size: 10px;
		font-weight: 500;
		color: var(--color-text, #374151);
		white-space: nowrap;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		pointer-events: none;
		z-index: 100;
	}

	/* Global cursor style when dragging */
	:global(body.track-resize-dragging) {
		cursor: col-resize !important;
		user-select: none !important;
	}

	:global(body.track-resize-dragging *) {
		cursor: col-resize !important;
	}
</style>
