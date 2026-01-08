<script lang="ts">
	/**
	 * SingleWellDataTab - Data configuration for single-well charts
	 *
	 * Used by: histogram, welllog, d3-welllog
	 * Features: Scrollable well list, scrollable curve list
	 */
	import type { CurveInfo, WellInfo } from '$lib/types'
	import type {
		HistogramConfig,
		WellLogConfig,
		D3WellLogConfig,
		AxisBinding
	} from '$lib/panes/chart-configs'
	import SelectList from '../SelectList.svelte'

	type SingleWellChartConfig = HistogramConfig | WellLogConfig | D3WellLogConfig

	interface Props {
		/** Current chart configuration */
		config: SingleWellChartConfig
		/** Available wells */
		wells: WellInfo[]
		/** Available curves for selected well */
		curves: CurveInfo[]
		/** Selected well */
		selectedWell: WellInfo | null
		/** Callback when well changes */
		onWellChange: (wellId: string) => void
		/** Callback when axis binding changes */
		onAxisChange: (key: string, binding: AxisBinding | null) => void
	}

	let {
		config,
		wells,
		curves,
		selectedWell,
		onWellChange,
		onAxisChange
	}: Props = $props()

	/** Convert wells to SelectList options */
	let wellOptions = $derived(
		wells.map((w) => ({
			id: w.id,
			label: w.name,
			sublabel: `${w.curve_count} curves`
		}))
	)

	/** Convert curves to SelectList options */
	let curveOptions = $derived(
		curves.map((c) => ({
			id: c.id,
			label: c.mnemonic,
			sublabel: c.main_curve_type ?? c.unit ?? undefined
		}))
	)

	/** Get the currently selected curve ID based on chart type */
	let selectedCurveId = $derived.by(() => {
		if (config.type === 'histogram') {
			return (config as HistogramConfig).dataCurve?.curveId ?? ''
		} else if (config.type === 'welllog') {
			return (config as WellLogConfig).curve?.curveId ?? ''
		} else if (config.type === 'd3-welllog') {
			return (config as D3WellLogConfig).curve?.curveId ?? ''
		}
		return ''
	})

	/** Handle curve selection */
	function handleCurveSelect(curveId: string | string[]): void {
		const id = Array.isArray(curveId) ? curveId[0] : curveId
		if (!id) return

		const curve = curves.find((c) => c.id === id)
		if (!curve) return

		const binding: AxisBinding = {
			curveId: curve.id,
			mnemonic: curve.mnemonic,
			autoScale: true
		}

		if (config.type === 'histogram') {
			onAxisChange('dataCurve', binding)
		} else {
			onAxisChange('curve', binding)
		}
	}
</script>

<div class="data-tab">
	<!-- Well Selection -->
	<div class="config-section">
		<h4 class="section-title">Well</h4>
		<SelectList
			options={wellOptions}
			selected={selectedWell?.id ?? ''}
			onChange={(id) => onWellChange(Array.isArray(id) ? id[0] : id)}
			maxHeight="160px"
			emptyText="No wells available"
		/>
	</div>

	<!-- Curve Selection -->
	{#if selectedWell}
		<div class="config-section">
			<h4 class="section-title">Curve</h4>
			<SelectList
				options={curveOptions}
				selected={selectedCurveId}
				onChange={handleCurveSelect}
				maxHeight="200px"
				emptyText="No curves in this well"
			/>
		</div>
	{:else}
		<div class="hint-box">
			<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
				<path d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0z" />
			</svg>
			<span>Select a well to choose a curve for plotting</span>
		</div>
	{/if}
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

	.hint-box {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 16px;
		background: hsl(var(--muted) / 0.3);
		border: 1px solid hsl(var(--border));
		border-radius: 8px;
		color: hsl(var(--muted-foreground));
		font-size: 13px;
	}

	.hint-box svg {
		flex-shrink: 0;
	}
</style>
