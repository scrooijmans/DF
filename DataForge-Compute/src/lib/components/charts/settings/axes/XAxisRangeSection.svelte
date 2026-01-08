<script lang="ts">
	/**
	 * XAxisRangeSection - X-axis range configuration
	 *
	 * Used by: d3-welllog (curve value range)
	 * Features: Manual min/max value inputs
	 */

	interface Props {
		/** X-axis minimum value */
		xMin: number
		/** X-axis maximum value */
		xMax: number
		/** Callback when X min changes */
		onXMinChange: (value: number) => void
		/** Callback when X max changes */
		onXMaxChange: (value: number) => void
		/** Section title */
		title?: string
		/** Optional ID prefix for form inputs */
		idPrefix?: string
	}

	let {
		xMin,
		xMax,
		onXMinChange,
		onXMaxChange,
		title = 'X-Axis Range',
		idPrefix = 'x-axis'
	}: Props = $props()
</script>

<div class="config-section">
	<h4 class="section-title">{title}</h4>

	<div class="field-row">
		<div class="field-group">
			<label class="field-label" for="{idPrefix}-min">Min</label>
			<input
				id="{idPrefix}-min"
				type="number"
				class="field-input"
				value={xMin}
				onchange={(e) => onXMinChange(parseFloat(e.currentTarget.value) || 0)}
			/>
		</div>
		<div class="field-group">
			<label class="field-label" for="{idPrefix}-max">Max</label>
			<input
				id="{idPrefix}-max"
				type="number"
				class="field-input"
				value={xMax}
				onchange={(e) => onXMaxChange(parseFloat(e.currentTarget.value) || 150)}
			/>
		</div>
	</div>
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

	.field-input {
		padding: 8px 12px;
		border: 1px solid hsl(var(--border));
		border-radius: 6px;
		background: hsl(var(--background));
		color: hsl(var(--foreground));
		font-size: 13px;
	}

	.field-input:focus {
		outline: none;
		border-color: hsl(var(--primary));
		box-shadow: 0 0 0 2px hsl(var(--primary) / 0.2);
	}
</style>
