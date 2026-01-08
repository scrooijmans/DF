<script lang="ts">
	/**
	 * ColorPresetPicker - Reusable color selector with presets
	 *
	 * Displays a row of color swatches from presets with optional custom color input.
	 */
	import { COLOR_PRESETS } from '$lib/panes/chart-configs'

	interface Props {
		/** Currently selected color */
		selectedColor: string
		/** Callback when color changes */
		onColorChange: (color: string) => void
		/** Custom color presets (defaults to COLOR_PRESETS) */
		presets?: string[]
		/** Label for the picker */
		label?: string
		/** Show custom color input */
		showCustomInput?: boolean
	}

	let {
		selectedColor,
		onColorChange,
		presets = COLOR_PRESETS,
		label = 'Color',
		showCustomInput = true
	}: Props = $props()
</script>

<div class="color-picker">
	{#if label}
		<span class="color-label">{label}</span>
	{/if}

	<div class="color-swatches">
		{#each presets as color}
			<button
				class="color-swatch"
				class:selected={selectedColor === color}
				style="background-color: {color}"
				onclick={() => onColorChange(color)}
				aria-label="Select color {color}"
				title={color}
			></button>
		{/each}

		{#if showCustomInput}
			<input
				type="color"
				value={selectedColor}
				onchange={(e) => onColorChange(e.currentTarget.value)}
				class="color-input"
				title="Custom color"
			/>
		{/if}
	</div>
</div>

<style>
	.color-picker {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.color-label {
		font-size: 12px;
		font-weight: 500;
		color: hsl(var(--muted-foreground));
	}

	.color-swatches {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
		align-items: center;
	}

	.color-swatch {
		width: 24px;
		height: 24px;
		border-radius: 4px;
		border: 2px solid transparent;
		cursor: pointer;
		transition:
			transform 0.1s,
			border-color 0.1s;
	}

	.color-swatch:hover {
		transform: scale(1.1);
	}

	.color-swatch.selected {
		border-color: hsl(var(--foreground));
		box-shadow: 0 0 0 2px hsl(var(--background));
	}

	.color-input {
		width: 24px;
		height: 24px;
		padding: 0;
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
</style>
