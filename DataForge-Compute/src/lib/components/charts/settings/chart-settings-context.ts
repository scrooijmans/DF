/**
 * Chart Settings Context
 *
 * Provides shared state and actions for ChartSettingsDialog child components.
 * Uses Svelte's context API to avoid prop drilling.
 */

import { getContext, setContext } from 'svelte'
import type { CurveInfo, WellInfo, SegmentedCurveData, MultiWellCurveData } from '$lib/types'
import type { CurveInfoWithWell } from '$lib/types'
import type { PaneNode } from '$lib/panes/layout-model'
import type {
	ChartConfiguration,
	AxisBinding,
	SeriesStyle
} from '$lib/panes/chart-configs'
import type { ChartDataFrame } from '$lib/charts/types'
import type { CorrelationCurveData } from '$lib/charts/correlation-types'

// ============================================================================
// Context Key
// ============================================================================

const CHART_SETTINGS_CONTEXT_KEY = Symbol('chart-settings-context')

// ============================================================================
// Context Interface
// ============================================================================

export interface ChartSettingsContextValue {
	// ----- Reactive State (read-only from child components) -----

	/** Selected pane node */
	pane: PaneNode | null
	/** Current chart configuration */
	config: ChartConfiguration | null
	/** Available wells */
	wells: WellInfo[]
	/** Available curves for selected well */
	curves: CurveInfo[]
	/** Selected well */
	selectedWell: WellInfo | null
	/** All workspace curves (for correlation) */
	workspaceCurves: CurveInfoWithWell[]
	/** Loading state */
	isLoadingData: boolean

	// ----- Actions (functions to update state) -----

	/** Update chart configuration */
	updateConfig: (newConfig: ChartConfiguration) => void
	/** Update a specific config field */
	updateConfigField: <K extends keyof ChartConfiguration>(
		key: K,
		value: ChartConfiguration[K]
	) => void
	/** Change selected well */
	changeWell: (wellId: string) => void
	/** Close the dialog */
	close: () => void

	// ----- Data Loading Actions -----

	/** Load chart data frame */
	loadChartData: (curveId: string) => Promise<ChartDataFrame | null>
	/** Load segmented curve data */
	loadSegmentedData: (curveId: string) => Promise<SegmentedCurveData | null>
	/** Notify data change */
	notifyDataChange: (data: ChartDataFrame | null) => void
	/** Notify segmented data change */
	notifySegmentedDataChange: (data: SegmentedCurveData | null) => void
	/** Notify multi-well data change */
	notifyMultiWellDataChange: (data: MultiWellCurveData[]) => void
	/** Notify correlation curve data change */
	notifyCorrelationCurveDataChange: (trackId: string, data: CorrelationCurveData | null) => void
}

// ============================================================================
// Context Getters and Setters
// ============================================================================

/**
 * Set the chart settings context (called in ChartSettingsDialog)
 */
export function setChartSettingsContext(value: ChartSettingsContextValue): void {
	setContext(CHART_SETTINGS_CONTEXT_KEY, value)
}

/**
 * Get the chart settings context (called in child components)
 * @throws Error if context is not set
 */
export function getChartSettingsContext(): ChartSettingsContextValue {
	const ctx = getContext<ChartSettingsContextValue>(CHART_SETTINGS_CONTEXT_KEY)
	if (!ctx) {
		throw new Error(
			'getChartSettingsContext must be called within a component that is a child of ChartSettingsDialog'
		)
	}
	return ctx
}

/**
 * Safely get context (returns null if not set)
 */
export function getChartSettingsContextSafe(): ChartSettingsContextValue | null {
	return getContext<ChartSettingsContextValue | null>(CHART_SETTINGS_CONTEXT_KEY)
}

// ============================================================================
// Helper Types for Specific Chart Types
// ============================================================================

/** Helper to narrow config type */
export function isLineConfig(
	config: ChartConfiguration | null
): config is ChartConfiguration & { type: 'line' } {
	return config?.type === 'line'
}

export function isScatterConfig(
	config: ChartConfiguration | null
): config is ChartConfiguration & { type: 'scatter' } {
	return config?.type === 'scatter'
}

export function isHistogramConfig(
	config: ChartConfiguration | null
): config is ChartConfiguration & { type: 'histogram' } {
	return config?.type === 'histogram'
}

export function isCrossPlotConfig(
	config: ChartConfiguration | null
): config is ChartConfiguration & { type: 'crossplot' } {
	return config?.type === 'crossplot'
}

export function isWellLogConfig(
	config: ChartConfiguration | null
): config is ChartConfiguration & { type: 'welllog' } {
	return config?.type === 'welllog'
}

export function isD3WellLogConfig(
	config: ChartConfiguration | null
): config is ChartConfiguration & { type: 'd3-welllog' } {
	return config?.type === 'd3-welllog'
}

export function isCorrelationConfig(
	config: ChartConfiguration | null
): config is ChartConfiguration & { type: 'correlation' } {
	return config?.type === 'correlation'
}
