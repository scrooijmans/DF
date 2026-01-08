<script lang="ts">
	/**
	 * D3WellLogStyleSection - Style settings for D3 Well Log charts
	 *
	 * Includes curve color, line width, fill settings, and lithology labels
	 */
	import type { D3WellLogConfig } from '$lib/panes/chart-configs'
	import { COLOR_PRESETS } from '$lib/panes/chart-configs'
	import ColorPresetPicker from '../ColorPresetPicker.svelte'

	interface Props {
		/** Current D3 Well Log configuration */
		config: D3WellLogConfig
		/** Callback when configuration changes */
		onConfigChange: (config: D3WellLogConfig) => void
	}

	let { config, onConfigChange }: Props = $props()

	function updateConfig<K extends keyof D3WellLogConfig>(key: K, value: D3WellLogConfig[K]): void {
		onConfigChange({ ...config, [key]: value })
	}
</script>

<!-- Curve Style -->
<div class="config-section">
	<h4 class="section-title">Curve Style</h4>

	<ColorPresetPicker
		label="Curve Color"
		selectedColor={config.curveColor ?? '#22c55e'}
		onColorChange={(color) => updateConfig('curveColor', color)}
	/>

	<div class="field-group">
		<label class="field-label" for="d3-line-width">Line Width</label>
		<input
			id="d3-line-width"
			type="number"
			class="field-input small"
			min="0.5"
			max="5"
			step="0.5"
			value={config.lineWidth ?? 1.5}
			onchange={(e) => updateConfig('lineWidth', parseFloat(e.currentTarget.value) || 1.5)}
		/>
	</div>
</div>

<!-- Fill Settings -->
<div class="config-section">
	<h4 class="section-title">Fill Settings</h4>

	<div class="field-group">
		<label class="field-label">Fill Direction</label>
		<select
			class="field-select"
			value={config.fillDirection ?? 'left'}
			onchange={(e) => updateConfig('fillDirection', e.currentTarget.value as 'left' | 'right' | 'none')}
		>
			<option value="left">Left (Sand)</option>
			<option value="right">Right (Shale)</option>
			<option value="none">None</option>
		</select>
	</div>

	{#if config.fillDirection !== 'none'}
		<div class="field-group">
			<label class="field-label">Fill Color</label>
			<input
				type="color"
				class="color-input"
				value={config.fillColor ?? '#ffff99'}
				onchange={(e) => updateConfig('fillColor', e.currentTarget.value)}
			/>
		</div>
	{/if}
</div>

<!-- Lithology Labels -->
<div class="config-section">
	<h4 class="section-title">Lithology Labels</h4>

	<label class="checkbox-label">
		<input
			type="checkbox"
			checked={config.showLithologyLabels ?? false}
			onchange={(e) => updateConfig('showLithologyLabels', e.currentTarget.checked)}
		/>
		<span>Show Lithology Labels</span>
	</label>

	{#if config.showLithologyLabels}
		<div class="field-group">
			<label class="field-label" for="d3-gr-cutoff">GR Cutoff (Sand/Shale)</label>
			<input
				id="d3-gr-cutoff"
				type="number"
				class="field-input small"
				min="0"
				max="200"
				value={config.grCutoff ?? 75}
				onchange={(e) => updateConfig('grCutoff', parseFloat(e.currentTarget.value) || 75)}
			/>
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

	.field-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.field-label {
		font-size: 12px;
		font-weight: 500;
		color: hsl(var(--muted-foreground));
	}

	.field-input,
	.field-select {
		padding: 8px 12px;
		border: 1px solid hsl(var(--border));
		border-radius: 6px;
		background: hsl(var(--background));
		color: hsl(var(--foreground));
		font-size: 13px;
	}

	.field-input.small {
		width: 80px;
	}

	.field-input:focus,
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
</style>
