<script lang="ts">
	/**
	 * CorrelationDataTab - Data configuration for correlation charts
	 *
	 * Used by: correlation
	 * Features: Curve type toggles with settings, multi-well checkbox selection
	 */
	import type { WellInfo } from '$lib/types'
	import type { CorrelationConfig, SelectedCurveType } from '$lib/charts/correlation-types'
	import { getDefaultCurveColor } from '$lib/charts/correlation-types'

	interface Props {
		/** Current chart configuration */
		config: CorrelationConfig
		/** Available wells */
		wells: WellInfo[]
		/** Unique curve mnemonics across workspace */
		uniqueMnemonics: string[]
		/** Toggle curve type selection */
		onToggleCurveType: (mnemonic: string) => void
		/** Update curve type settings */
		onUpdateCurveTypeSettings: (mnemonic: string, updates: Partial<SelectedCurveType>) => void
		/** Toggle well selection */
		onToggleWell: (wellId: string) => void
		/** Get curve match count for well */
		getWellCurveMatchCount: (wellId: string) => number
	}

	let {
		config,
		wells,
		uniqueMnemonics,
		onToggleCurveType,
		onUpdateCurveTypeSettings,
		onToggleWell,
		getWellCurveMatchCount
	}: Props = $props()

	let selectedCurveTypes = $derived(config.selectedCurveTypes ?? [])
	let selectedWellIds = $derived(config.selectedWellIds ?? [])

	function isCurveTypeSelected(mnemonic: string): boolean {
		return selectedCurveTypes.some(ct => ct.mnemonic.toUpperCase() === mnemonic)
	}
</script>

<!-- Curve Types Section -->
<div class="config-section">
	<h4 class="section-title">Curve Types</h4>
	<p class="section-description">Select which curve types to display across all wells</p>

	{#if uniqueMnemonics.length === 0}
		<div class="hint-box">
			<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
				<path d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
			</svg>
			<span>No curves available in this workspace</span>
		</div>
	{:else}
		<div class="curve-type-list">
			{#each uniqueMnemonics as mnemonic (mnemonic)}
				{@const isSelected = isCurveTypeSelected(mnemonic)}
				<label class="curve-type-item">
					<input
						type="checkbox"
						checked={isSelected}
						onchange={() => onToggleCurveType(mnemonic)}
					/>
					<span class="curve-type-dot" style="background: {getDefaultCurveColor(mnemonic)}"></span>
					<span class="curve-type-name">{mnemonic}</span>
				</label>
			{/each}
		</div>

		{#if selectedCurveTypes.length > 0}
			<div class="curve-type-settings-list">
				<p class="settings-hint">Configure X-axis range for each curve type:</p>
				{#each selectedCurveTypes as curveType (curveType.mnemonic)}
					<div class="curve-type-settings">
						<span class="curve-type-label" style="color: {curveType.color}">{curveType.mnemonic}</span>
						<div class="curve-type-inputs">
							<input
								type="number"
								class="field-input small"
								placeholder="Min"
								value={curveType.xMin ?? ''}
								onchange={(e) => onUpdateCurveTypeSettings(curveType.mnemonic, {
									xMin: e.currentTarget.value ? parseFloat(e.currentTarget.value) : undefined
								})}
							/>
							<span class="input-separator">-</span>
							<input
								type="number"
								class="field-input small"
								placeholder="Max"
								value={curveType.xMax ?? ''}
								onchange={(e) => onUpdateCurveTypeSettings(curveType.mnemonic, {
									xMax: e.currentTarget.value ? parseFloat(e.currentTarget.value) : undefined
								})}
							/>
							<label class="checkbox-label compact">
								<input
									type="checkbox"
									checked={curveType.logScale ?? false}
									onchange={(e) => onUpdateCurveTypeSettings(curveType.mnemonic, {
										logScale: e.currentTarget.checked
									})}
								/>
								<span>Log</span>
							</label>
						</div>
					</div>
				{/each}
			</div>
		{/if}
	{/if}
</div>

<!-- Wells Section -->
<div class="config-section">
	<h4 class="section-title">Wells</h4>
	<p class="section-description">Select which wells to include in the correlation</p>

	{#if wells.length === 0}
		<div class="hint-box">
			<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
				<path d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
			</svg>
			<span>No wells available in workspace</span>
		</div>
	{:else}
		<div class="wells-checklist">
			{#each wells as wellInfo (wellInfo.id)}
				{@const isSelected = selectedWellIds.includes(wellInfo.id)}
				{@const matchCount = getWellCurveMatchCount(wellInfo.id)}
				{@const totalSelected = selectedCurveTypes.length}
				<label class="well-checkbox-item">
					<input
						type="checkbox"
						checked={isSelected}
						onchange={() => onToggleWell(wellInfo.id)}
					/>
					<span class="well-checkbox-name">{wellInfo.name}</span>
					{#if totalSelected > 0}
						<span class="match-badge" class:partial={matchCount < totalSelected && matchCount > 0} class:none={matchCount === 0}>
							{matchCount}/{totalSelected}
						</span>
					{:else}
						<span class="curve-count">({wellInfo.curve_count} curves)</span>
					{/if}
				</label>
			{/each}
		</div>
	{/if}
</div>

<style>
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

	.curve-type-list {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
	}

	.curve-type-item {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 6px 10px;
		border: 1px solid hsl(var(--border));
		border-radius: 6px;
		cursor: pointer;
		transition: background 0.1s, border-color 0.1s;
	}

	.curve-type-item:hover {
		background: hsl(var(--accent));
	}

	.curve-type-item:has(input:checked) {
		border-color: hsl(var(--primary));
		background: hsl(var(--primary) / 0.1);
	}

	.curve-type-dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
	}

	.curve-type-name {
		font-size: 13px;
		font-weight: 500;
	}

	.curve-type-settings-list {
		display: flex;
		flex-direction: column;
		gap: 8px;
		margin-top: 8px;
		padding-top: 12px;
		border-top: 1px solid hsl(var(--border));
	}

	.settings-hint {
		margin: 0;
		font-size: 12px;
		color: hsl(var(--muted-foreground));
	}

	.curve-type-settings {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.curve-type-label {
		min-width: 60px;
		font-size: 13px;
		font-weight: 600;
	}

	.curve-type-inputs {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.field-input {
		padding: 6px 10px;
		border: 1px solid hsl(var(--border));
		border-radius: 4px;
		background: hsl(var(--background));
		color: hsl(var(--foreground));
		font-size: 13px;
	}

	.field-input.small {
		width: 70px;
	}

	.field-input:focus {
		outline: none;
		border-color: hsl(var(--primary));
	}

	.input-separator {
		color: hsl(var(--muted-foreground));
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 12px;
		color: hsl(var(--muted-foreground));
		cursor: pointer;
	}

	.checkbox-label.compact {
		margin-left: 8px;
	}

	.wells-checklist {
		display: flex;
		flex-direction: column;
		gap: 4px;
		max-height: 200px;
		overflow-y: auto;
	}

	.well-checkbox-item {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 8px;
		border-radius: 4px;
		cursor: pointer;
		transition: background 0.1s;
	}

	.well-checkbox-item:hover {
		background: hsl(var(--accent));
	}

	.well-checkbox-name {
		flex: 1;
		font-size: 13px;
		color: hsl(var(--foreground));
	}

	.curve-count {
		font-size: 12px;
		color: hsl(var(--muted-foreground));
	}

	.match-badge {
		padding: 2px 6px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 500;
		background: hsl(var(--primary));
		color: hsl(var(--primary-foreground));
	}

	.match-badge.partial {
		background: hsl(var(--warning, 45 93% 47%));
		color: hsl(var(--warning-foreground, 0 0% 0%));
	}

	.match-badge.none {
		background: hsl(var(--muted));
		color: hsl(var(--muted-foreground));
	}
</style>
