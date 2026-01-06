<script lang="ts">
	/**
	 * WellColumn - Container for a single well's curve tracks
	 *
	 * Displays:
	 * - Well header (name, color indicator)
	 * - Multiple CurveTrack components (one per track/curve)
	 * - Well tops overlay
	 */
	import type {
		WellCorrelationEntry,
		CorrelationCurveData,
		WellTop,
		FillDirection,
		CrossoverConfig
	} from '$lib/charts/correlation-types';
	import { TRACK_WIDTH_CONSTRAINTS } from '$lib/charts/correlation-types';
	import CurveTrack from './CurveTrack.svelte';
	import WellTopOverlay from './WellTopOverlay.svelte';
	import TrackResizeHandle from './TrackResizeHandle.svelte';

	interface Props {
		/** Well entry with track configuration */
		well: WellCorrelationEntry;
		/** Loaded curve data by track ID */
		curveData: Map<string, CorrelationCurveData>;
		/** Shared depth range */
		depthRange: { min: number; max: number };
		/** Column height in pixels */
		height: number;
		/** Link group for chart synchronization */
		linkGroup: string;
		/** Well tops to display */
		wellTops: WellTop[];
		/** Current cursor depth for crosshair */
		cursorDepth: number | null;
		/** Fixed width per track in pixels (optional, defaults to percentage-based) */
		trackWidth?: number;
		/** Per-track width overrides (trackId -> width in pixels) */
		trackWidths?: Record<string, number>;
		/** Enable resize handles between tracks */
		enableResize?: boolean;
		/** Callback when track width changes */
		onTrackWidthChange?: (trackId: string, newWidth: number) => void;
		/** Callback when scale toggle is clicked */
		onScaleToggle?: (trackId: string, logScale: boolean) => void;
		/** Callback when fill toggle is clicked */
		onFillToggle?: (trackId: string, direction: FillDirection) => void;
		/** Callback when context menu is requested */
		onContextMenu?: (trackId: string, event: MouseEvent) => void;
		/** Crossover configuration (for NPHI/RHOB shading) */
		crossover?: CrossoverConfig;
	}

	let {
		well,
		curveData,
		depthRange,
		height,
		linkGroup,
		wellTops,
		cursorDepth,
		trackWidth,
		trackWidths = {},
		enableResize = true,
		onTrackWidthChange,
		onScaleToggle,
		onFillToggle,
		onContextMenu,
		crossover
	}: Props = $props();

	/**
	 * Get width for a specific track
	 * Priority: trackWidths[trackId] > trackWidth > default
	 */
	function getTrackWidth(trackId: string): number {
		if (trackWidths[trackId] !== undefined) {
			return trackWidths[trackId];
		}
		if (trackWidth !== undefined) {
			return trackWidth;
		}
		return TRACK_WIDTH_CONSTRAINTS.defaultWidth;
	}

	/**
	 * Handle track width change from resize handle
	 */
	function handleTrackWidthChange(trackId: string, newWidth: number): void {
		onTrackWidthChange?.(trackId, newWidth);
	}

	/** Header height for calculations */
	const headerHeight = 36;
	const trackAreaHeight = $derived(height - headerHeight);
</script>

<div class="well-column">
	<!-- Well Header -->
	<div class="well-header" style="border-left: 3px solid {well.wellColor}">
		<span class="well-name" title={well.wellName}>{well.wellName}</span>
		<span class="track-count">{well.tracks.length} track{well.tracks.length !== 1 ? 's' : ''}</span>
	</div>

	<!-- Tracks Container -->
	<div class="tracks-container" style="height: {trackAreaHeight}px">
		{#if well.tracks.length === 0}
			<div class="empty-tracks">
				<p>No curves selected</p>
			</div>
		{:else}
			<div class="tracks-row">
				{#each well.tracks as track, index (track.id)}
					<div
						class="track-wrapper"
						style="width: {getTrackWidth(track.id)}px; min-width: {TRACK_WIDTH_CONSTRAINTS.minWidth}px"
					>
						<CurveTrack
							{track}
							{curveData}
							wellId={well.wellId}
							{depthRange}
							height={trackAreaHeight}
							{linkGroup}
							{cursorDepth}
							{onScaleToggle}
							{onFillToggle}
							{onContextMenu}
							{crossover}
						/>
					</div>

					{#if enableResize && index < well.tracks.length - 1}
						<TrackResizeHandle
							trackId={track.id}
							currentWidth={getTrackWidth(track.id)}
							onWidthChange={handleTrackWidthChange}
						/>
					{/if}
				{/each}
			</div>

			<!-- Well Tops Overlay (spans all tracks) -->
			{#if wellTops.length > 0}
				<WellTopOverlay
					tops={wellTops}
					{depthRange}
					height={trackAreaHeight}
				/>
			{/if}
		{/if}
	</div>
</div>

<style>
	.well-column {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		overflow: hidden;
	}

	.well-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 8px 12px;
		background: var(--color-bg-secondary, #f9fafb);
		border-bottom: 1px solid var(--color-border, #e5e7eb);
		flex-shrink: 0;
		height: 36px;
		box-sizing: border-box;
	}

	.well-name {
		font-size: 12px;
		font-weight: 600;
		color: var(--color-text, #111827);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		max-width: 70%;
	}

	.track-count {
		font-size: 10px;
		color: var(--color-text-tertiary, #9ca3af);
	}

	.tracks-container {
		flex: 1;
		position: relative;
		overflow: hidden;
	}

	.tracks-row {
		display: flex;
		width: 100%;
		height: 100%;
	}

	.track-wrapper {
		flex-shrink: 0;
		border-right: 1px solid var(--color-border-light, #f3f4f6);
		height: 100%;
	}

	.track-wrapper:last-child {
		border-right: none;
	}

	.empty-tracks {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: var(--color-text-tertiary, #9ca3af);
		font-size: 12px;
	}
</style>
