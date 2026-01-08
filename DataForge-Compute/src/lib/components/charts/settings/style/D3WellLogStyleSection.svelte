<script lang="ts">
	/**
	 * D3WellLogStyleSection - Global style settings for D3 Well Log charts
	 *
	 * Shows global options like crossover fill configuration.
	 * Per-curve styling (color, line width, fill) is now handled in the Axes tab.
	 */
	import type { D3WellLogConfig, CrossoverFillConfig } from '$lib/panes/chart-configs'

	interface Props {
		/** Current D3 Well Log configuration */
		config: D3WellLogConfig
		/** Callback when configuration changes */
		onConfigChange: (config: D3WellLogConfig) => void
	}

	let { config, onConfigChange }: Props = $props()

	/** Check if we have at least 2 curves for crossover fill */
	let canEnableCrossover = $derived(config.curves.length >= 2)

	/** Get curve options for dropdowns */
	let curveOptions = $derived(
		config.curves.map(c => ({ id: c.curveId, label: c.mnemonic }))
	)

	/** Update crossover fill config */
	function updateCrossoverFill(updates: Partial<CrossoverFillConfig>): void {
		const currentFill = config.crossoverFill ?? {
			curve1Id: config.curves[0]?.curveId ?? '',
			curve2Id: config.curves[1]?.curveId ?? '',
			positiveColor: '#ef4444',
			negativeColor: '#22c55e',
			opacity: 0.3,
			enabled: false
		}
		onConfigChange({
			...config,
			crossoverFill: { ...currentFill, ...updates }
		})
	}

	/** Toggle crossover fill enabled */
	function toggleCrossoverFill(enabled: boolean): void {
		if (enabled && config.curves.length >= 2) {
			updateCrossoverFill({
				enabled: true,
				curve1Id: config.curves[0].curveId,
				curve2Id: config.curves[1].curveId
			})
		} else {
			updateCrossoverFill({ enabled: false })
		}
	}
</script>

<div class="style-section">
	<!-- Info about per-curve styling -->
	<div class="info-box">
		<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
			<path d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
		</svg>
		<span>Per-curve colors, line widths, and fills are configured in the <strong>Axes</strong> tab.</span>
	</div>

	<!-- Crossover Fill Settings -->
	<div class="config-section">
		<h4 class="section-title">Crossover Fill</h4>
		<p class="section-description">
			Fill the area between two curves to highlight crossovers (e.g., density-neutron overlay).
		</p>

		{#if !canEnableCrossover}
			<div class="warning-box">
				<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
				</svg>
				<span>Select at least 2 curves to enable crossover fill</span>
			</div>
		{:else}
			<label class="checkbox-label">
				<input
					type="checkbox"
					checked={config.crossoverFill?.enabled ?? false}
					onchange={(e) => toggleCrossoverFill(e.currentTarget.checked)}
				/>
				<span>Enable Crossover Fill</span>
			</label>

			{#if config.crossoverFill?.enabled}
				<!-- Curve Selection -->
				<div class="field-row">
					<div class="field-group">
						<label class="field-label" for="crossover-curve1">Curve 1</label>
						<select
							id="crossover-curve1"
							class="field-select"
							value={config.crossoverFill.curve1Id}
							onchange={(e) => updateCrossoverFill({ curve1Id: e.currentTarget.value })}
						>
							{#each curveOptions as option}
								<option value={option.id}>{option.label}</option>
							{/each}
						</select>
					</div>
					<div class="field-group">
						<label class="field-label" for="crossover-curve2">Curve 2</label>
						<select
							id="crossover-curve2"
							class="field-select"
							value={config.crossoverFill.curve2Id}
							onchange={(e) => updateCrossoverFill({ curve2Id: e.currentTarget.value })}
						>
							{#each curveOptions as option}
								<option value={option.id}>{option.label}</option>
							{/each}
						</select>
					</div>
				</div>

				<!-- Colors -->
				<div class="field-row">
					<div class="field-group">
						<label class="field-label" for="crossover-positive">Positive (C1 > C2)</label>
						<input
							id="crossover-positive"
							type="color"
							class="color-input"
							value={config.crossoverFill.positiveColor}
							onchange={(e) => updateCrossoverFill({ positiveColor: e.currentTarget.value })}
						/>
					</div>
					<div class="field-group">
						<label class="field-label" for="crossover-negative">Negative (C2 > C1)</label>
						<input
							id="crossover-negative"
							type="color"
							class="color-input"
							value={config.crossoverFill.negativeColor}
							onchange={(e) => updateCrossoverFill({ negativeColor: e.currentTarget.value })}
						/>
					</div>
				</div>

				<!-- Opacity -->
				<div class="field-group">
					<label class="field-label" for="crossover-opacity">Fill Opacity</label>
					<input
						id="crossover-opacity"
						type="range"
						class="range-input"
						min="0.1"
						max="1"
						step="0.1"
						value={config.crossoverFill.opacity}
						oninput={(e) => updateCrossoverFill({ opacity: parseFloat(e.currentTarget.value) })}
					/>
					<span class="range-value">{Math.round((config.crossoverFill.opacity ?? 0.3) * 100)}%</span>
				</div>
			{/if}
		{/if}
	</div>
</div>

<style>
	.style-section {
		display: flex;
		flex-direction: column;
		gap: 16px;
	}

	.info-box {
		display: flex;
		align-items: flex-start;
		gap: 8px;
		padding: 12px 14px;
		background: hsl(var(--muted) / 0.3);
		border: 1px solid hsl(var(--border));
		border-radius: 8px;
		color: hsl(var(--muted-foreground));
		font-size: 12px;
		line-height: 1.4;
	}

	.info-box svg {
		flex-shrink: 0;
		margin-top: 1px;
	}

	.info-box strong {
		color: hsl(var(--foreground));
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
		line-height: 1.4;
	}

	.warning-box {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 12px;
		background: hsl(38 92% 50% / 0.1);
		border: 1px solid hsl(38 92% 50% / 0.3);
		border-radius: 6px;
		color: hsl(38 92% 40%);
		font-size: 12px;
	}

	.warning-box svg {
		flex-shrink: 0;
	}

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 13px;
		color: hsl(var(--foreground));
		cursor: pointer;
	}

	.checkbox-label input[type='checkbox'] {
		width: 16px;
		height: 16px;
		cursor: pointer;
	}

	.field-row {
		display: flex;
		gap: 12px;
	}

	.field-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
		flex: 1;
	}

	.field-label {
		font-size: 12px;
		font-weight: 500;
		color: hsl(var(--muted-foreground));
	}

	.field-select {
		padding: 8px 12px;
		border: 1px solid hsl(var(--border));
		border-radius: 6px;
		background: hsl(var(--background));
		color: hsl(var(--foreground));
		font-size: 13px;
	}

	.field-select:focus {
		outline: none;
		border-color: hsl(var(--primary));
		box-shadow: 0 0 0 2px hsl(var(--primary) / 0.2);
	}

	.color-input {
		width: 60px;
		height: 32px;
		padding: 2px;
		border: 1px solid hsl(var(--border));
		border-radius: 4px;
		cursor: pointer;
		background: transparent;
	}

	.color-input::-webkit-color-swatch-wrapper {
		padding: 2px;
	}

	.color-input::-webkit-color-swatch {
		border-radius: 2px;
		border: none;
	}

	.range-input {
		flex: 1;
		height: 6px;
		border-radius: 3px;
		background: hsl(var(--muted));
		-webkit-appearance: none;
		appearance: none;
	}

	.range-input::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 16px;
		height: 16px;
		border-radius: 50%;
		background: hsl(var(--primary));
		cursor: pointer;
	}

	.range-value {
		font-size: 12px;
		color: hsl(var(--muted-foreground));
		min-width: 40px;
		text-align: right;
	}
</style>
