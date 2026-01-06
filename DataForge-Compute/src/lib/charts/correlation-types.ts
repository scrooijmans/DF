/**
 * Well Correlation Panel Types
 *
 * Type definitions for the multi-well, multi-track correlation view.
 * Inspired by patterns from WellLogs.jl (type hierarchy), Wellioviz (depth sync),
 * and Grafana (state management).
 *
 * See plan file for architecture details.
 */

import type { DataFrameSource } from './types';

// ============================================================================
// Core Configuration Types
// ============================================================================

/**
 * CorrelationConfig - Root configuration for a well correlation panel
 *
 * Acts as the "single source of truth" (Grafana DashboardModel pattern).
 * All wells share the same depth range for synchronized scrolling.
 */
export interface CorrelationConfig {
	/** Configuration type identifier */
	type: 'correlation';
	/** Unique panel ID */
	id: string;
	/** Panel title */
	title: string;

	// Unified selection (Part 3)
	/** Selected curve types to display (unified across all wells) */
	selectedCurveTypes: SelectedCurveType[];
	/** Selected well IDs to display */
	selectedWellIds: string[];

	/** Wells displayed in this correlation view (derived from selections) */
	wells: WellCorrelationEntry[];
	/** Shared depth range across all tracks */
	depthRange: CorrelationDepthRange;
	/** Well top markers (horizontal lines across all tracks) */
	wellTops: WellTop[];

	/** Layout configuration */
	layout: CorrelationLayoutConfig;

	/** General settings */
	showLegend: boolean;
	showGrid: boolean;
	enableZoom: boolean;
	showCursor: boolean;

	/** Crossover shading configuration (e.g., NPHI/RHOB) */
	crossover?: CrossoverConfig;
}

/**
 * CorrelationDepthRange - Shared depth axis configuration
 *
 * WellLogs.jl pattern: When depth_sync=true, calculate global range from all wells.
 * Wellioviz pattern: Single depthScale passed to all tracks.
 */
export interface CorrelationDepthRange {
	/** Auto-calculate range from all loaded curves */
	autoScale: boolean;
	/** Minimum depth (used when autoScale=false) */
	min: number | null;
	/** Maximum depth (used when autoScale=false) */
	max: number | null;
	/** Invert Y-axis (depth increases downward - standard for well logs) */
	inverted: boolean;
}

// ============================================================================
// Well and Track Types
// ============================================================================

/**
 * WellCorrelationEntry - A single well in the correlation view
 *
 * WellLogs.jl pattern: Well → Tracks → Curves hierarchy with clear ownership.
 * Each well has its own color for visual distinction.
 */
export interface WellCorrelationEntry {
	/** Well ID (matches WellInfo.id from data store) */
	wellId: string;
	/** Well display name */
	wellName: string;
	/** Well header color (for visual grouping) */
	wellColor: string;
	/** Tracks (curves) to display for this well */
	tracks: CorrelationTrack[];
	/** Collapsed state for UI */
	collapsed?: boolean;
}

// ============================================================================
// Curve Display Options (OpenDTect-inspired)
// ============================================================================

/**
 * FillDirection - Direction for curve fill (OpenDTect pattern)
 *
 * - 'none': No fill
 * - 'left': Fill from curve to left edge (common for GR shading)
 * - 'right': Fill from curve to right edge
 * - 'full': Fill entire track background
 */
export type FillDirection = 'none' | 'left' | 'right' | 'full';

/**
 * CurveDisplayOptions - Styling options for a single curve
 *
 * Inspired by OpenDTect's Log 1/Log 2 configuration tabs.
 */
export interface CurveDisplayOptions {
	/** Line/fill color */
	color: string;
	/** Line width (default: 1) */
	lineWidth?: number;
	/** Fill direction (default: 'none') */
	fillDirection?: FillDirection;
	/** Fill color (defaults to curve color with opacity) */
	fillColor?: string;
	/** Fill opacity 0-1 (default: 0.3) */
	fillOpacity?: number;
	/** Mirror/flip curve horizontally (swaps xMin/xMax display) */
	mirror?: boolean;
	/** Use data min/max instead of manual range */
	autoRange?: boolean;
}

/**
 * TrackCurve - A single curve within a multi-curve track
 *
 * OpenDTect supports Log 1 + Log 2 per panel. This extends that to N curves.
 * Each curve can have its own X-axis range (for overlays like NPHI/RHOB).
 */
export interface TrackCurve {
	/** Curve ID for data loading */
	curveId: string;
	/** Curve mnemonic (e.g., "GR", "NPHI") */
	mnemonic: string;
	/** Display options */
	display: CurveDisplayOptions;
	/** X-axis minimum (overrides track default) */
	xMin?: number;
	/** X-axis maximum (overrides track default) */
	xMax?: number;
	/** Use logarithmic scale (overrides track default) */
	logScale?: boolean;
}

/**
 * CorrelationTrack - A single curve track within a well column
 *
 * Each track has:
 * - Its own X-axis range (independent per curve type)
 * - Shared Y-axis (depth) with all other tracks
 * - Optional depth zones for lithology/facies coloring
 *
 * Supports two modes (backwards compatible):
 * 1. Single curve: Use curveId, mnemonic, color directly
 * 2. Multi-curve: Use curves[] array (OpenDTect Log 1 + Log 2 pattern)
 */
export interface CorrelationTrack {
	/** Unique track ID (typically wellId:curveId or wellId:trackIndex) */
	id: string;

	// --- Single curve mode (backwards compatible) ---
	/** Curve mnemonic (e.g., "GR", "NPHI", "RHOB") */
	mnemonic?: string;
	/** Curve ID for data loading */
	curveId?: string;
	/** Line/fill color */
	color?: string;
	/** Line width */
	lineWidth?: number;
	/** Fill area under curve (legacy - use fillDirection for new code) */
	fillArea?: boolean;

	// --- Multi-curve mode (OpenDTect-inspired) ---
	/** Multiple curves to display in this track (first renders on top) */
	curves?: TrackCurve[];

	// --- Shared track settings ---
	/** X-axis minimum (optional, auto-scaled if not set) */
	xMin?: number;
	/** X-axis maximum (optional, auto-scaled if not set) */
	xMax?: number;
	/** Use logarithmic scale for X-axis (e.g., resistivity) */
	logScale?: boolean;
	/** Depth zones for this track */
	zones?: DepthZone[];
	/** Track width in pixels (default: auto) */
	width?: number;
}

// ============================================================================
// Annotation Types
// ============================================================================

/**
 * WellTop - A stratigraphic marker or formation top
 *
 * Renders as a horizontal line across all tracks at a specific depth.
 * Can be global (all wells) or well-specific.
 */
export interface WellTop {
	/** Marker name (e.g., "Top Sand A", "Base Shale") */
	name: string;
	/** Depth value */
	depth: number;
	/** Line color */
	color: string;
	/** Line style */
	lineStyle?: 'solid' | 'dashed' | 'dotted';
	/** Line width */
	lineWidth?: number;
	/** Well ID if well-specific (undefined = global) */
	wellId?: string;
	/** Show label */
	showLabel?: boolean;
	/** Label position */
	labelPosition?: 'left' | 'right' | 'center';
}

/**
 * DepthZone - A colored depth interval for lithology/facies visualization
 *
 * Renders as a colored rectangle behind the curve.
 */
export interface DepthZone {
	/** Zone name (e.g., "Sand", "Shale", "Limestone") */
	name: string;
	/** Start depth (top) */
	depthStart: number;
	/** End depth (bottom) */
	depthEnd: number;
	/** Fill color */
	color: string;
	/** Fill opacity (0-1) */
	opacity?: number;
	/** Show zone label */
	showLabel?: boolean;
}

// ============================================================================
// Event Types (Grafana EventBus pattern)
// ============================================================================

/**
 * DepthRangeChangedEvent - Emitted when depth range changes
 */
export interface DepthRangeChangedEvent {
	/** Event source: 'user' (pan/zoom) or 'auto' (data loaded) */
	source: 'user' | 'auto';
	/** New depth range */
	range: { min: number; max: number };
	/** Source chart ID if user-triggered */
	sourceChartId?: string;
}

/**
 * CursorDepthChangedEvent - Emitted when cursor moves
 */
export interface CursorDepthChangedEvent {
	/** Source chart ID */
	sourceChartId: string;
	/** Cursor depth (null when cursor leaves chart) */
	depth: number | null;
	/** Data value at cursor position */
	value?: number;
}

/**
 * TrackChangedEvent - Emitted when tracks are added/removed
 */
export interface TrackChangedEvent {
	/** Action type */
	action: 'add' | 'remove' | 'update';
	/** Well ID */
	wellId: string;
	/** Track info */
	track: CorrelationTrack;
}

// ============================================================================
// Loaded Data Types
// ============================================================================

/**
 * CorrelationCurveData - Loaded curve data for a track
 *
 * Extends the segment-based architecture for correlation panels.
 */
export interface CorrelationCurveData {
	/** Track ID (wellId:curveId) */
	trackId: string;
	/** Curve mnemonic */
	mnemonic: string;
	/** Unit of measurement */
	unit: string | null;
	/** Segments of valid data */
	segments: Array<{
		depthStart: number;
		depthEnd: number;
		depths: number[];
		values: number[];
	}>;
	/** Total depth range */
	depthRange: { min: number; max: number };
	/** Total valid points */
	totalPoints: number;
	/** Data source */
	source: DataFrameSource;
}

// ============================================================================
// Factory Functions
// ============================================================================

/**
 * Create a default correlation configuration
 */
export function createDefaultCorrelationConfig(id?: string): CorrelationConfig {
	return {
		type: 'correlation',
		id: id ?? `correlation-${Date.now()}`,
		title: 'Well Correlation',
		selectedCurveTypes: [],
		selectedWellIds: [],
		wells: [],
		depthRange: {
			autoScale: true,
			min: null,
			max: null,
			inverted: true
		},
		wellTops: [],
		layout: { ...DEFAULT_LAYOUT },
		showLegend: true,
		showGrid: true,
		enableZoom: true,
		showCursor: true,
		crossover: { ...DEFAULT_CROSSOVER_CONFIG }
	};
}

/**
 * Create a default well entry
 */
export function createWellEntry(
	wellId: string,
	wellName: string,
	wellColor: string
): WellCorrelationEntry {
	return {
		wellId,
		wellName,
		wellColor,
		tracks: [],
		collapsed: false
	};
}

/**
 * Create a default track (single curve mode)
 */
export function createTrack(
	wellId: string,
	curveId: string,
	mnemonic: string,
	color: string
): CorrelationTrack {
	return {
		id: `${wellId}:${curveId}`,
		mnemonic,
		curveId,
		color,
		lineWidth: 1,
		fillArea: false
	};
}

/**
 * Create a multi-curve track (OpenDTect-inspired)
 *
 * @param wellId - Well identifier
 * @param trackIndex - Track index within the well (for ID generation)
 * @param curves - Array of curves to display (first renders on top)
 */
export function createMultiCurveTrack(
	wellId: string,
	trackIndex: number,
	curves: TrackCurve[]
): CorrelationTrack {
	return {
		id: `${wellId}:track-${trackIndex}`,
		curves
	};
}

/**
 * Create a TrackCurve with display options
 */
export function createTrackCurve(
	curveId: string,
	mnemonic: string,
	options?: Partial<CurveDisplayOptions & { xMin?: number; xMax?: number; logScale?: boolean }>
): TrackCurve {
	const defaultRange = getDefaultCurveRange(mnemonic);
	return {
		curveId,
		mnemonic,
		display: {
			color: options?.color ?? getDefaultCurveColor(mnemonic),
			lineWidth: options?.lineWidth ?? 1,
			fillDirection: options?.fillDirection ?? 'none',
			fillColor: options?.fillColor,
			fillOpacity: options?.fillOpacity ?? 0.3,
			mirror: options?.mirror ?? false,
			autoRange: options?.autoRange ?? false
		},
		xMin: options?.xMin ?? defaultRange?.min,
		xMax: options?.xMax ?? defaultRange?.max,
		logScale: options?.logScale ?? defaultRange?.logScale ?? false
	};
}

/**
 * Normalize a CorrelationTrack to always have curves[] array
 *
 * Converts single-curve mode to multi-curve mode for consistent handling.
 * This enables backwards compatibility while using multi-curve rendering.
 */
export function normalizeTrackCurves(track: CorrelationTrack): TrackCurve[] {
	// If already multi-curve mode, return as-is
	if (track.curves && track.curves.length > 0) {
		return track.curves;
	}

	// Convert single-curve mode to TrackCurve
	if (track.curveId && track.mnemonic) {
		return [
			{
				curveId: track.curveId,
				mnemonic: track.mnemonic,
				display: {
					color: track.color ?? getDefaultCurveColor(track.mnemonic),
					lineWidth: track.lineWidth ?? 1,
					fillDirection: track.fillArea ? 'left' : 'none',
					fillOpacity: 0.3,
					mirror: false,
					autoRange: false
				},
				xMin: track.xMin,
				xMax: track.xMax,
				logScale: track.logScale
			}
		];
	}

	// Empty track
	return [];
}

/**
 * Get all curve IDs from a track (handles both modes)
 */
export function getTrackCurveIds(track: CorrelationTrack): string[] {
	const curves = normalizeTrackCurves(track);
	return curves.map((c) => c.curveId);
}

/**
 * Create a classic NPHI/RHOB overlay track (industry standard)
 *
 * NPHI is plotted reversed (0.45 to -0.15) and RHOB normal (1.95 to 2.95)
 * so they crossover indicates gas-bearing zones.
 */
export function createNphiRhobOverlayTrack(
	wellId: string,
	trackIndex: number,
	nphiCurveId: string,
	rhobCurveId: string
): CorrelationTrack {
	return createMultiCurveTrack(wellId, trackIndex, [
		createTrackCurve(nphiCurveId, 'NPHI', {
			color: '#3b82f6', // Blue
			fillDirection: 'right',
			fillOpacity: 0.2
		}),
		createTrackCurve(rhobCurveId, 'RHOB', {
			color: '#ef4444', // Red
			fillDirection: 'left',
			fillOpacity: 0.2
		})
	]);
}

/**
 * Create a track with default X-axis ranges applied
 *
 * Uses industry-standard ranges from DEFAULT_CURVE_RANGES if available,
 * or falls back to the curveTypeConfig if provided.
 */
export function createTrackWithDefaults(
	wellId: string,
	curveId: string,
	mnemonic: string,
	curveTypeConfig?: SelectedCurveType
): CorrelationTrack {
	const defaultRange = getDefaultCurveRange(mnemonic);
	return {
		id: `${wellId}:${curveId}`,
		mnemonic,
		curveId,
		color: curveTypeConfig?.color ?? getDefaultCurveColor(mnemonic),
		xMin: curveTypeConfig?.xMin ?? defaultRange?.min,
		xMax: curveTypeConfig?.xMax ?? defaultRange?.max,
		logScale: curveTypeConfig?.logScale ?? defaultRange?.logScale ?? false,
		lineWidth: 1,
		fillArea: false
	};
}

/**
 * Create a default well top marker
 */
export function createWellTop(
	name: string,
	depth: number,
	color: string = '#ff0000'
): WellTop {
	return {
		name,
		depth,
		color,
		lineStyle: 'solid',
		lineWidth: 1,
		showLabel: true,
		labelPosition: 'left'
	};
}

// ============================================================================
// Utility Functions
// ============================================================================

// ============================================================================
// Default Curve X-Axis Ranges (Industry Standard)
// ============================================================================

/**
 * CurveTypeRange - Default X-axis range for a curve type
 */
export interface CurveTypeRange {
	min: number;
	max: number;
	logScale: boolean;
}

/**
 * Default X-axis ranges by curve type (industry standard)
 * Note: Reversed min/max indicates the curve should be plotted right-to-left
 */
export const DEFAULT_CURVE_RANGES: Record<string, CurveTypeRange> = {
	// Gamma Ray
	GR: { min: 0, max: 150, logScale: false },
	SGR: { min: 0, max: 150, logScale: false },

	// Neutron Porosity (reversed for overlay with density)
	NPHI: { min: 0.45, max: -0.15, logScale: false },
	TNPH: { min: 0.45, max: -0.15, logScale: false },

	// Density
	RHOB: { min: 1.95, max: 2.95, logScale: false },

	// Resistivity (log scale)
	ILD: { min: 0.2, max: 2000, logScale: true },
	ILM: { min: 0.2, max: 2000, logScale: true },
	RT: { min: 0.2, max: 2000, logScale: true },
	LLD: { min: 0.2, max: 2000, logScale: true },
	LLS: { min: 0.2, max: 2000, logScale: true },
	MSFL: { min: 0.2, max: 2000, logScale: true },

	// Sonic (reversed - faster rocks on right)
	DT: { min: 140, max: 40, logScale: false },
	DTC: { min: 140, max: 40, logScale: false },
	DTS: { min: 300, max: 100, logScale: false },

	// Porosity
	PHIE: { min: 0, max: 0.4, logScale: false },
	PHIT: { min: 0, max: 0.4, logScale: false },
	DPHI: { min: 0, max: 0.4, logScale: false },

	// Caliper
	CALI: { min: 6, max: 16, logScale: false },
	HCAL: { min: 6, max: 16, logScale: false },

	// Spontaneous Potential
	SP: { min: -100, max: 100, logScale: false },

	// Photoelectric Factor
	PEF: { min: 0, max: 10, logScale: false },
	PE: { min: 0, max: 10, logScale: false }
};

/**
 * Get default X-axis range for a curve mnemonic
 */
export function getDefaultCurveRange(mnemonic: string): CurveTypeRange | null {
	return DEFAULT_CURVE_RANGES[mnemonic.toUpperCase()] ?? null;
}

// ============================================================================
// Unified Selection Types (Part 3)
// ============================================================================

/**
 * SelectedCurveType - A curve type selected for display in correlation view
 *
 * Users select curve types once, and they are applied to all wells that have them.
 */
export interface SelectedCurveType {
	/** Curve mnemonic (e.g., "GR", "NPHI") */
	mnemonic: string;
	/** Display color */
	color: string;
	/** X-axis minimum (override or default) */
	xMin?: number;
	/** X-axis maximum (override or default) */
	xMax?: number;
	/** Use logarithmic scale */
	logScale?: boolean;
}

/**
 * CorrelationLayoutConfig - Layout settings for the correlation panel
 */
export interface CorrelationLayoutConfig {
	/** Width of each track in pixels */
	trackWidth: number;
	/** Width of the depth track in pixels */
	depthTrackWidth: number;
	/** Per-track width overrides (trackId -> width) */
	trackWidths?: Record<string, number>;
	/** Header configuration */
	headerConfig?: TrackHeaderConfig;
}

/**
 * Default layout configuration
 */
export const DEFAULT_LAYOUT: CorrelationLayoutConfig = {
	trackWidth: 140,
	depthTrackWidth: 60,
	trackWidths: {},
	headerConfig: undefined
};

// ============================================================================
// Color Defaults
// ============================================================================

/**
 * Default curve colors by mnemonic (industry convention)
 */
export const DEFAULT_CURVE_COLORS: Record<string, string> = {
	GR: '#22c55e', // Green for Gamma Ray
	NPHI: '#3b82f6', // Blue for Neutron Porosity
	RHOB: '#ef4444', // Red for Density
	DT: '#f59e0b', // Orange for Sonic
	ILD: '#8b5cf6', // Purple for Deep Resistivity
	ILM: '#ec4899', // Pink for Medium Resistivity
	SP: '#6b7280', // Gray for Spontaneous Potential
	CALI: '#06b6d4', // Cyan for Caliper
	PEF: '#84cc16', // Lime for Photoelectric Factor
	DPHI: '#f97316' // Dark Orange for Density Porosity
};

/**
 * Default well colors for visual distinction
 */
export const DEFAULT_WELL_COLORS: string[] = [
	'#3b82f6', // Blue
	'#22c55e', // Green
	'#ef4444', // Red
	'#f59e0b', // Amber
	'#8b5cf6', // Purple
	'#06b6d4', // Cyan
	'#ec4899', // Pink
	'#84cc16' // Lime
];

/**
 * Get default color for a curve mnemonic
 */
export function getDefaultCurveColor(mnemonic: string): string {
	const upper = mnemonic.toUpperCase();
	return DEFAULT_CURVE_COLORS[upper] ?? '#6b7280';
}

/**
 * Get next well color based on index
 */
export function getNextWellColor(index: number): string {
	return DEFAULT_WELL_COLORS[index % DEFAULT_WELL_COLORS.length];
}

/**
 * Calculate global depth range from all loaded curves
 */
export function calculateGlobalDepthRange(
	curveDataMap: Map<string, CorrelationCurveData>
): { min: number; max: number } {
	let globalMin = Infinity;
	let globalMax = -Infinity;

	for (const curveData of curveDataMap.values()) {
		if (curveData.depthRange.min < globalMin) {
			globalMin = curveData.depthRange.min;
		}
		if (curveData.depthRange.max > globalMax) {
			globalMax = curveData.depthRange.max;
		}
	}

	// Return sensible defaults if no data
	if (globalMin === Infinity || globalMax === -Infinity) {
		return { min: 0, max: 1000 };
	}

	return { min: globalMin, max: globalMax };
}

// ============================================================================
// Track Header Types (OpenDTect-inspired)
// ============================================================================

/**
 * TrackHeaderConfig - Configuration for interactive track headers
 *
 * Inspired by OpenDTect's Log 1/Log 2 configuration interface.
 */
export interface TrackHeaderConfig {
	/** Show mnemonic with color indicator */
	showMnemonic: boolean;
	/** Show scale range (e.g., "0 - 150") */
	showScaleRange: boolean;
	/** Show log/linear toggle */
	showScaleToggle: boolean;
	/** Show fill direction toggle */
	showFillToggle: boolean;
	/** Interactive header (enables right-click menu) */
	interactive: boolean;
}

/**
 * Default header configuration
 */
export const DEFAULT_HEADER_CONFIG: TrackHeaderConfig = {
	showMnemonic: true,
	showScaleRange: true,
	showScaleToggle: true,
	showFillToggle: true,
	interactive: true
};

// ============================================================================
// Track Width Resizing
// ============================================================================

/**
 * Track width constraints for resizing
 */
export const TRACK_WIDTH_CONSTRAINTS = {
	minWidth: 80,
	maxWidth: 400,
	defaultWidth: 140
};

// ============================================================================
// Log Scale Configuration
// ============================================================================

/**
 * Log scale tick values for resistivity tracks (industry standard)
 * Covers range from 0.2 to 2000 ohm-m
 */
export const LOG_SCALE_TICK_VALUES = [
	0.2, 0.5, 1, 2, 5, 10, 20, 50, 100, 200, 500, 1000, 2000
];

// ============================================================================
// Crossover Shading Types
// ============================================================================

/**
 * CrossoverConfig - Configuration for NPHI/RHOB crossover shading
 *
 * Crossover shading highlights regions where curves cross, commonly used
 * to identify gas-bearing zones (NPHI > RHOB when normalized).
 */
export interface CrossoverConfig {
	/** Enable automatic crossover detection */
	enabled: boolean;
	/** First curve ID (typically NPHI) */
	curve1Id: string;
	/** Second curve ID (typically RHOB) */
	curve2Id: string;
	/** Shading color for crossover regions */
	fillColor: string;
	/** Fill opacity (0-1) */
	fillOpacity: number;
	/** Z-index (below curves, above background) */
	zIndex: number;
}

/**
 * CrossoverRegion - A detected crossover region between two curves
 */
export interface CrossoverRegion {
	/** Start depth of crossover region */
	startDepth: number;
	/** End depth of crossover region */
	endDepth: number;
	/** Polygon points for SVG rendering */
	polygon: Array<{ x: number; depth: number }>;
}

/**
 * Default crossover configuration (gas indication)
 */
export const DEFAULT_CROSSOVER_CONFIG: CrossoverConfig = {
	enabled: false,
	curve1Id: '',
	curve2Id: '',
	fillColor: '#ffff00', // Yellow for gas indication
	fillOpacity: 0.3,
	zIndex: 2
};

// ============================================================================
// Context Menu Types
// ============================================================================

/**
 * TrackContextMenuState - State for the track context menu
 */
export interface TrackContextMenuState {
	/** Whether the menu is open */
	isOpen: boolean;
	/** Track ID that was right-clicked */
	trackId: string | null;
	/** Curve ID if a specific curve was targeted */
	curveId: string | null;
	/** Menu position */
	position: { x: number; y: number };
}
