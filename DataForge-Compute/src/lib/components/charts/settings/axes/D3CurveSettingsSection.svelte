<script lang="ts">
	/**
	 * D3CurveSettingsSection - Per-curve settings for D3 Well Log charts
	 *
	 * Displays collapsible settings cards for each selected curve:
	 * - X-axis range (xMin, xMax)
	 * - Curve color
	 * - Line width
	 * - Fill direction and color
	 */
	import type { D3CurveBinding } from '$lib/panes/chart-configs'
	import ColorPresetPicker from '../ColorPresetPicker.svelte'

	interface Props {
		/** Array of curve bindings */
		curves: D3CurveBinding[]
		/** Callback when a curve's settings change */
		onCurveChange: (curveId: string, updates: Partial<D3CurveBinding>) => void
	}

	let { curves, onCurveChange }: Props = $props()

	/** Track which curves are expanded - default to all expanded */
	let expandedCurves = $state<Set<string>>(new Set())

	/** Auto-expand new curves when added */
	$effect(() => {
		const currentIds = new Set(curves.map(c => c.curveId))
		// Add any new curves to expanded set
		for (const id of currentIds) {
			if (!expandedCurves.has(id)) {
				expandedCurves = new Set([...expandedCurves, id])
			}
		}
	})

	/** Toggle expanded state for a curve */
	function toggleExpand(curveId: string): void {
		const newSet = new Set(expandedCurves)
		if (newSet.has(curveId)) {
			newSet.delete(curveId)
		} else {
			newSet.add(curveId)
		}
		expandedCurves = newSet
	}

	/** Update a numeric field */
	function updateNumber(curveId: string, field: keyof D3CurveBinding, value: string, defaultValue: number): void {
		const numValue = parseFloat(value)
		onCurveChange(curveId, { [field]: isNaN(numValue) ? defaultValue : numValue })
	}
</script>

<div class="curve-settings-section">
	<h4 class="section-title">Curve Settings</h4>

	{#if curves.length === 0}
		<div class="empty-hint">
			<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
				<path d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
			</svg>
			<span>Select curves in the Data tab</span>
		</div>
	{:else}
		{#each curves as curve (curve.curveId)}
			<div class="curve-card">
				<!-- Curve Header (clickable to expand/collapse) -->
				<button
					class="curve-header"
					class:expanded={expandedCurves.has(curve.curveId)}
					onclick={() => toggleExpand(curve.curveId)}
				>
					<span class="curve-color-dot" style="background-color: {curve.color};"></span>
					<span class="curve-name">{curve.mnemonic}</span>
					{#if curve.unit}
						<span class="curve-unit">({curve.unit})</span>
					{/if}
					<svg
						class="expand-icon"
						class:rotated={expandedCurves.has(curve.curveId)}
						width="16"
						height="16"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
					>
						<path d="M6 9l6 6 6-6" />
					</svg>
				</button>

				<!-- Curve Settings Body -->
				{#if expandedCurves.has(curve.curveId)}
					<div class="curve-body">
						<!-- X-Axis Range -->
						<div class="setting-group">
							<span class="setting-label">X-Axis Range</span>
							<div class="field-row">
								<div class="field-group">
									<label class="field-sublabel" for="xmin-{curve.curveId}">Min</label>
									<input
										id="xmin-{curve.curveId}"
										type="number"
										class="field-input"
										value={curve.xMin}
										onchange={(e) => updateNumber(curve.curveId, 'xMin', e.currentTarget.value, 0)}
									/>
								</div>
								<div class="field-group">
									<label class="field-sublabel" for="xmax-{curve.curveId}">Max</label>
									<input
										id="xmax-{curve.curveId}"
										type="number"
										class="field-input"
										value={curve.xMax}
										onchange={(e) => updateNumber(curve.curveId, 'xMax', e.currentTarget.value, 100)}
									/>
								</div>
							</div>
						</div>

						<!-- Color -->
						<div class="setting-group">
							<ColorPresetPicker
								label="Curve Color"
								selectedColor={curve.color}
								onColorChange={(color) => onCurveChange(curve.curveId, { color })}
							/>
						</div>

						<!-- Line Width -->
						<div class="setting-group">
							<label class="setting-label" for="linewidth-{curve.curveId}">Line Width</label>
							<input
								id="linewidth-{curve.curveId}"
								type="number"
								class="field-input small"
								min="0.5"
								max="5"
								step="0.5"
								value={curve.lineWidth}
								onchange={(e) => updateNumber(curve.curveId, 'lineWidth', e.currentTarget.value, 1.5)}
							/>
						</div>

						<!-- Fill Settings -->
						<div class="setting-group">
							<label class="setting-label" for="fill-{curve.curveId}">Fill Direction</label>
							<select
								id="fill-{curve.curveId}"
								class="field-select"
								value={curve.fillDirection}
								onchange={(e) => onCurveChange(curve.curveId, {
									fillDirection: e.currentTarget.value as 'left' | 'right' | 'none'
								})}
							>
								<option value="none">No Fill</option>
								<option value="left">Fill Left</option>
								<option value="right">Fill Right</option>
							</select>
						</div>

						{#if curve.fillDirection !== 'none'}
							<div class="setting-group">
								<label class="setting-label" for="fillcolor-{curve.curveId}">Fill Color</label>
								<input
									id="fillcolor-{curve.curveId}"
									type="color"
									class="color-input"
									value={curve.fillColor ?? '#ffff99'}
									onchange={(e) => onCurveChange(curve.curveId, { fillColor: e.currentTarget.value })}
								/>
							</div>
						{/if}
					</div>
				{/if}
			</div>
		{/each}
	{/if}
</div>

<style>
	.curve-settings-section {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.section-title {
		margin: 0;
		font-size: 14px;
		font-weight: 600;
		color: hsl(var(--foreground));
	}

	.empty-hint {
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

	.empty-hint svg {
		flex-shrink: 0;
	}

	.curve-card {
		border: 1px solid hsl(var(--border));
		border-radius: 8px;
		background: hsl(var(--card));
		overflow: hidden;
	}

	.curve-header {
		display: flex;
		align-items: center;
		gap: 8px;
		width: 100%;
		padding: 10px 12px;
		border: none;
		background: hsl(var(--muted) / 0.3);
		cursor: pointer;
		transition: background 0.15s;
		text-align: left;
	}

	.curve-header:hover {
		background: hsl(var(--muted) / 0.5);
	}

	.curve-header.expanded {
		border-bottom: 1px solid hsl(var(--border));
	}

	.curve-color-dot {
		width: 12px;
		height: 12px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.curve-name {
		font-size: 13px;
		font-weight: 600;
		color: hsl(var(--foreground));
	}

	.curve-unit {
		font-size: 11px;
		color: hsl(var(--muted-foreground));
	}

	.expand-icon {
		margin-left: auto;
		color: hsl(var(--muted-foreground));
		transition: transform 0.15s;
	}

	.expand-icon.rotated {
		transform: rotate(180deg);
	}

	.curve-body {
		padding: 12px;
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.setting-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.setting-label {
		font-size: 12px;
		font-weight: 500;
		color: hsl(var(--muted-foreground));
	}

	.field-row {
		display: flex;
		gap: 8px;
	}

	.field-group {
		display: flex;
		flex-direction: column;
		gap: 4px;
		flex: 1;
	}

	.field-sublabel {
		font-size: 11px;
		color: hsl(var(--muted-foreground));
	}

	.field-input,
	.field-select {
		padding: 6px 10px;
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
</style>
