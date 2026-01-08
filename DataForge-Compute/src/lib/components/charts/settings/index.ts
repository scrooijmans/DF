/**
 * Chart Settings Components
 *
 * Reusable components for ChartSettingsDialog and its child components.
 */

// Context
export {
	setChartSettingsContext,
	getChartSettingsContext,
	getChartSettingsContextSafe,
	isLineConfig,
	isScatterConfig,
	isHistogramConfig,
	isCrossPlotConfig,
	isWellLogConfig,
	isD3WellLogConfig,
	isCorrelationConfig,
	type ChartSettingsContextValue
} from './chart-settings-context'

// Shared Components
export { default as TabNavigation } from './TabNavigation.svelte'
export { default as ColorPresetPicker } from './ColorPresetPicker.svelte'
export { default as ConfigSection } from './ConfigSection.svelte'
export { default as SelectList } from './SelectList.svelte'

// Data Tab Components
export { SingleWellDataTab, MultiWellDataTab, CorrelationDataTab } from './data-tabs'

// Style Components
export { D3WellLogStyleSection } from './style'

// Axes Components
export { DepthSettingsSection, XAxisRangeSection, D3CurveSettingsSection } from './axes'
