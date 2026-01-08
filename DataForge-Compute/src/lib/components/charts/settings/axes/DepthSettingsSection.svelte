<script lang="ts">
	/**
	 * DepthSettingsSection - Depth range configuration
	 *
	 * Used by: welllog, d3-welllog, correlation
	 * Features: Auto-scale toggle, manual min/max depth inputs
	 */

	interface DepthRange {
		autoScale: boolean
		min: number | null
		max: number | null
	}

	interface Props {
		/** Current depth range configuration */
		depthRange: DepthRange
		/** Callback when depth range changes */
		onDepthRangeChange: (depthRange: Partial<DepthRange>) => void
		/** Optional ID prefix for form inputs */
		idPrefix?: string
		/** Auto-scale label text */
		autoScaleLabel?: string
	}

	let {
		depthRange,
		onDepthRangeChange,
		idPrefix = 'depth',
		autoScaleLabel = 'Auto-scale Depth'
	}: Props = $props()
</script>

<div class="config-section">
	<h4 class="section-title">Depth Settings</h4>

	<label class="checkbox-label">
		<input
			type="checkbox"
			checked={depthRange.autoScale}
			onchange={(e) => onDepthRangeChange({ autoScale: e.currentTarget.checked })}
		/>
		<span>{autoScaleLabel}</span>
	</label>

	{#if !depthRange.autoScale}
		<div class="field-row">
			<div class="field-group">
				<label class="field-label" for="{idPrefix}-min">Min Depth</label>
				<input
					id="{idPrefix}-min"
					type="number"
					class="field-input"
					value={depthRange.min ?? ''}
					onchange={(e) => onDepthRangeChange({
						min: e.currentTarget.value ? parseFloat(e.currentTarget.value) : null
					})}
				/>
			</div>
			<div class="field-group">
				<label class="field-label" for="{idPrefix}-max">Max Depth</label>
				<input
					id="{idPrefix}-max"
					type="number"
					class="field-input"
					value={depthRange.max ?? ''}
					onchange={(e) => onDepthRangeChange({
						max: e.currentTarget.value ? parseFloat(e.currentTarget.value) : null
					})}
				/>
			</div>
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
