<script lang="ts">
	/**
	 * SelectList - Scrollable selection list component
	 *
	 * A visual list that displays all options in a scrollable container.
	 * Supports single and multi-select modes.
	 *
	 * Used by: SingleWellDataTab, MultiWellDataTab, CorrelationDataTab
	 */

	interface Option {
		/** Unique identifier for the option */
		id: string
		/** Display label */
		label: string
		/** Optional secondary text (e.g., curve count) */
		sublabel?: string
	}

	interface Props {
		/** List of options to display */
		options: Option[]
		/** Selected option ID(s) - string for single, string[] for multi */
		selected: string | string[]
		/** Whether multiple selections are allowed */
		multiSelect?: boolean
		/** Callback when selection changes */
		onChange: (selected: string | string[]) => void
		/** Maximum height of the list */
		maxHeight?: string
		/** Placeholder when no options */
		emptyText?: string
		/** Optional label above the list */
		label?: string
	}

	let {
		options,
		selected,
		multiSelect = false,
		onChange,
		maxHeight = '200px',
		emptyText = 'No options available',
		label
	}: Props = $props()

	/** Get selected IDs as array for easier checking */
	let selectedIds = $derived(
		Array.isArray(selected) ? selected : (selected ? [selected] : [])
	)

	/** Check if an option is selected */
	function isSelected(id: string): boolean {
		return selectedIds.includes(id)
	}

	/** Handle option click */
	function handleClick(id: string): void {
		if (multiSelect) {
			// Toggle selection for multi-select
			const currentSelected = Array.isArray(selected) ? selected : []
			if (isSelected(id)) {
				onChange(currentSelected.filter((s) => s !== id))
			} else {
				onChange([...currentSelected, id])
			}
		} else {
			// Single select - just set the value
			onChange(id)
		}
	}

	/** Handle keyboard navigation */
	function handleKeydown(event: KeyboardEvent, id: string): void {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault()
			handleClick(id)
		}
	}
</script>

{#if label}
	<span class="list-label">{label}</span>
{/if}

<div class="select-list" style="max-height: {maxHeight}">
	{#if options.length === 0}
		<div class="empty-state">{emptyText}</div>
	{:else}
		{#each options as option (option.id)}
			<button
				type="button"
				class="list-item"
				class:selected={isSelected(option.id)}
				onclick={() => handleClick(option.id)}
				onkeydown={(e) => handleKeydown(e, option.id)}
			>
				{#if multiSelect}
					<span class="checkbox" class:checked={isSelected(option.id)}>
						{#if isSelected(option.id)}
							<svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M2 6l3 3 5-6" />
							</svg>
						{/if}
					</span>
				{/if}
				<span class="item-content">
					<span class="item-label">{option.label}</span>
					{#if option.sublabel}
						<span class="item-sublabel">{option.sublabel}</span>
					{/if}
				</span>
			</button>
		{/each}
	{/if}
</div>

<style>
	.list-label {
		display: block;
		font-size: 12px;
		font-weight: 500;
		color: hsl(var(--muted-foreground));
		margin-bottom: 6px;
	}

	.select-list {
		display: flex;
		flex-direction: column;
		gap: 2px;
		overflow-y: auto;
		border: 1px solid hsl(var(--border));
		border-radius: 6px;
		background: hsl(var(--background));
		padding: 4px;
	}

	.empty-state {
		padding: 24px 16px;
		text-align: center;
		color: hsl(var(--muted-foreground));
		font-size: 13px;
	}

	.list-item {
		display: flex;
		align-items: center;
		gap: 8px;
		width: 100%;
		padding: 8px 12px;
		border: none;
		border-radius: 4px;
		background: transparent;
		color: hsl(var(--foreground));
		font-size: 13px;
		text-align: left;
		cursor: pointer;
		transition: background-color 0.1s ease;
	}

	.list-item:hover {
		background: hsl(var(--accent));
	}

	.list-item:focus-visible {
		outline: 2px solid hsl(var(--primary));
		outline-offset: -2px;
	}

	.list-item.selected {
		background: hsl(var(--primary) / 0.1);
		color: hsl(var(--primary));
	}

	.list-item.selected:hover {
		background: hsl(var(--primary) / 0.15);
	}

	.checkbox {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 16px;
		height: 16px;
		border: 1.5px solid hsl(var(--border));
		border-radius: 3px;
		background: hsl(var(--background));
		flex-shrink: 0;
		transition: background-color 0.1s ease, border-color 0.1s ease;
	}

	.checkbox.checked {
		background: hsl(var(--primary));
		border-color: hsl(var(--primary));
		color: white;
	}

	.item-content {
		display: flex;
		flex-direction: column;
		gap: 2px;
		min-width: 0;
		flex: 1;
	}

	.item-label {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.item-sublabel {
		font-size: 11px;
		color: hsl(var(--muted-foreground));
	}
</style>
