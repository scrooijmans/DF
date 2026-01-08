<script lang="ts">
	/**
	 * MultiWellDataTab - Data configuration for multi-well charts
	 *
	 * Used by: crossplot, scatter, line
	 * Features: Scrollable curve type selection (X, Y, optional Z), multi-well selection
	 */
	import type { WellInfo } from '$lib/types'
	import type {
		LineChartConfig,
		ScatterChartConfig,
		CrossPlotConfig
	} from '$lib/panes/chart-configs'
	import SelectList from '../SelectList.svelte'

	type MultiWellChartConfig = LineChartConfig | ScatterChartConfig | CrossPlotConfig

	interface Props {
		/** Current chart configuration */
		config: MultiWellChartConfig
		/** Available wells */
		wells: WellInfo[]
		/** Unique curve mnemonics across workspace */
		uniqueMnemonics: string[]
		/** Get current curve type for axis */
		getCurveType: (axis: 'x' | 'y' | 'z') => string | null
		/** Update curve type for axis */
		onCurveTypeChange: (axis: 'x' | 'y' | 'z', mnemonic: string | null) => void
		/** Get selected well IDs */
		getSelectedWellIds: () => string[]
		/** Toggle well selection */
		onToggleWell: (wellId: string) => void
		/** Get curve match count for well */
		getMatchCount: (wellId: string) => number
		/** Get required curve type count */
		getRequiredCount: () => number
	}

	let {
		config,
		wells,
		uniqueMnemonics,
		getCurveType,
		onCurveTypeChange,
		getSelectedWellIds,
		onToggleWell,
		getMatchCount,
		getRequiredCount
	}: Props = $props()

	let selectedWellIds = $derived(getSelectedWellIds())
	let requiredCount = $derived(getRequiredCount())

	/** Convert mnemonics to SelectList options */
	let curveTypeOptions = $derived(
		uniqueMnemonics.map((m) => ({
			id: m,
			label: m
		}))
	)

	/** Convert wells to SelectList options with match info */
	let wellOptions = $derived(
		wells.map((w) => {
			const matchCount = getMatchCount(w.id)
			let sublabel = `${w.curve_count} curves`
			if (requiredCount > 0) {
				sublabel = `${matchCount}/${requiredCount} matched`
			}
			return {
				id: w.id,
				label: w.name,
				sublabel
			}
		})
	)

	/** Handle curve type selection for an axis */
	function handleCurveSelect(axis: 'x' | 'y' | 'z', value: string | string[]): void {
		const mnemonic = Array.isArray(value) ? value[0] : value
		onCurveTypeChange(axis, mnemonic || null)
	}

	/** Is crossplot type */
	let isCrossplot = $derived(config.type === 'crossplot')

	/** Does crossplot use curve coloring */
	let showZAxis = $derived(
		config.type === 'crossplot' && (config as CrossPlotConfig).colorMode === 'curve'
	)
</script>

<div class="data-tab">
	<!-- Curve Type Selection -->
	<div class="config-section">
		<h4 class="section-title">Curve Types</h4>
		<p class="section-description">Select curve types for each axis</p>

		{#if uniqueMnemonics.length === 0}
			<div class="hint-box">
				<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<path d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0z" />
				</svg>
				<span>No curves available in this workspace</span>
			</div>
		{:else}
			<!-- X/Y side by side for crossplot -->
			<div class="axis-row" class:side-by-side={isCrossplot}>
				<div class="axis-column">
					<SelectList
						label="X Axis"
						options={curveTypeOptions}
						selected={getCurveType('x') ?? ''}
						onChange={(v) => handleCurveSelect('x', v)}
						maxHeight="160px"
						emptyText="No curves"
					/>
				</div>
				<div class="axis-column">
					<SelectList
						label="Y Axis"
						options={curveTypeOptions}
						selected={getCurveType('y') ?? ''}
						onChange={(v) => handleCurveSelect('y', v)}
						maxHeight="160px"
						emptyText="No curves"
					/>
				</div>
			</div>

			{#if showZAxis}
				<div class="axis-section">
					<SelectList
						label="Z Axis (Color)"
						options={curveTypeOptions}
						selected={getCurveType('z') ?? ''}
						onChange={(v) => handleCurveSelect('z', v)}
						maxHeight="120px"
						emptyText="No curves"
					/>
				</div>
			{/if}
		{/if}
	</div>

	<!-- Wells Section - multi-select -->
	<div class="config-section">
		<h4 class="section-title">Wells</h4>
		<p class="section-description">Select wells to include in the chart</p>

		<SelectList
			options={wellOptions}
			selected={selectedWellIds}
			multiSelect={true}
			onChange={(ids) => {
				const newIds = Array.isArray(ids) ? ids : [ids]
				// Determine which wells to toggle
				for (const well of wells) {
					const wasSelected = selectedWellIds.includes(well.id)
					const isNowSelected = newIds.includes(well.id)
					if (wasSelected !== isNowSelected) {
						onToggleWell(well.id)
					}
				}
			}}
			maxHeight="200px"
			emptyText="No wells available"
		/>
	</div>
</div>

<style>
	.data-tab {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.config-section {
		display: flex;
		flex-direction: column;
		gap: 12px;
		padding: 16px;
		background: hsl(var(--card));
		border: 1px solid hsl(var(--border));
		border-radius: 8px;
	}

	.section-title {
		margin: 0;
		font-size: 14px;
		font-weight: 600;
		color: hsl(var(--foreground));
	}

	.section-description {
		margin: 0;
		font-size: 12px;
		color: hsl(var(--muted-foreground));
	}

	.hint-box {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px;
		background: hsl(var(--muted) / 0.3);
		border-radius: 6px;
		color: hsl(var(--muted-foreground));
		font-size: 13px;
	}

	.hint-box svg {
		flex-shrink: 0;
	}

	.axis-row {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.axis-row.side-by-side {
		flex-direction: row;
		gap: 16px;
	}

	.axis-column {
		flex: 1;
		min-width: 0;
	}

	.axis-section {
		margin-top: 8px;
	}
</style>
