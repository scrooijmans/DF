<script lang="ts">
	import { setContext } from 'svelte';
	import { onMount } from 'svelte';
	import { getGridspaceState } from '$lib/state/gridspace/gridspace.svelte';
	import { getOperatorsManager } from '$lib/state/gridspace/operators/operators.svelte';
	import OperatorMenubarActions from '$lib/components/gridspace/gridspace-content/gridspace-content-operators/gridspace-content-operator/operator-menubar-actions/operator-menubar-actions.svelte';
	import BlockAvoLayeredModel from '$lib/components/gridspace/operators/blocky-avo/block-avo-layered-model/block-avo-layered-model.svelte';
	import CalculateVclFromGr from '$lib/components/gridspace/operators/calculate_vcl_from_gr/calculate_vcl_from_gr.svelte';
	import GridspaceContentOperatorSectionsHeader from '$lib/components/gridspace/gridspace-content/gridspace-content-operators/gridspace-content-operator/gridspace-content-operator-item-sections-header/gridspace-content-operator-item-sections-header.svelte';
	import { getFlow } from '$lib/state/gridspace/flow/gridspace-flow.svelte.ts';

	// Props
	const { id } = $props<{
		id: string;
	}>();

	// Get managers from context
	const gridspaceState = getGridspaceState();
	const operatorsManager = getOperatorsManager();

	// Get the operator instance
	const operator = $derived(operatorsManager.getOperator(id));

	// Set operator ID in context for the menubar actions
	setContext('operatorId', id);
	setContext('operatorsManager', {
		getOperatorState: (operatorId: string) => {
			const op = operatorsManager.getOperator(operatorId);
			return {
				operator: op,
				// Add any methods that might be needed to interact with the operator
				toggleSelection: () => {
					console.log(`Toggle selection for operator: ${operatorId}`);
					// Implement selection functionality for operators
				},
				toggleInteraction: () => {
					console.log(`Toggle interaction for operator: ${operatorId}`);
					// Implement interaction functionality for operators
				}
			};
		}
	});

	// Handle item selection
	function selectItem() {
		gridspaceState.setActiveItem(id);
		// Also set this as the active operator
		operatorsManager.setActiveOperator(id);
	}

	// Function to handle item removal
	function removeItem() {
		operatorsManager.removeOperator(id);
	}

	// Function to get the appropriate component based on operator id
	function getOperatorComponent(operatorId: string) {
		switch (operatorId) {
			case 'blocky-avo-layered-model':
				return BlockAvoLayeredModel;
			case 'calculate-vcl-from-gr':
				return CalculateVclFromGr;
			default:
				return null;
		}
	}

</script>

<div
	class="item flex h-full w-full flex-col overflow-hidden rounded-md border bg-card shadow-xs"
	class:ring-2={id === gridspaceState.activeItemId}
	class:ring-primary={id === gridspaceState.activeItemId}
	on:click={selectItem}
	role="button"
	tabindex="0"
>
	<!-- Item content with menubar on the left -->
	<div class="content-area flex flex-1">
		<!-- Vertical menubar on the left -->
		<div class="menubar-container shrink-0 bg-muted/20 p-1">
			<OperatorMenubarActions vertical={true} />
		</div>

		<!-- Operator content -->
		<div class="flex-1 overflow-hidden bg-muted/50 p-2">
			{#if operator}
				<div class="flex flex-col h-full">
					<div class="flex items-center mb-2 justify-between">
						<div class="flex items-center">
							<h3 class="text-lg font-medium">{operator.label}</h3>
						</div>
						<button
							class="ml-4 p-2 rounded-full hover:bg-muted transition-colors flex items-center justify-center"
							aria-label="Run Operator"
						>
							<svg
								width="24"
								height="24"
								viewBox="0 0 24 24"
								fill="none"
								stroke="#444"
								stroke-width="2"
								stroke-linecap="round"
								stroke-linejoin="round"><polygon points="6 4 20 12 6 20 6 4" /></svg
							>
						</button>
					</div>

					<!-- Operator content area with component based on id -->
					<div class="flex-1 border rounded-md p-2 bg-white overflow-auto">
						<GridspaceContentOperatorSectionsHeader />

						{#if getOperatorComponent(operator.operatorId)}
							<svelte:component
								this={getOperatorComponent(operator.operatorId)}
								{...{ operatorId: id } as Record<string, unknown>}
							/>
						{:else}
							<div class="flex flex-col h-full">
								<p>Operator ID: {operator.operatorId}</p>
								<p>Instance ID: {operator.id}</p>

								<!-- Generic placeholder for unimplemented operator types -->
								<div class="mt-4 border-t pt-4">
									<p class="text-muted-foreground">
										No specific implementation available for operator id: {operator.operatorId}
									</p>
								</div>
							</div>
						{/if}
					</div>

					<!-- Action buttons -->
					<div class="mt-2 flex justify-end space-x-2">
						<button
							class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-9 px-4 py-2"
							on:click|stopPropagation={() => {
								// Implement any primary action for the operator
								console.log(`Primary action for operator: ${id}`);
							}}
						>
							Run
						</button>
						<button
							class="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 border border-input bg-background hover:bg-accent hover:text-accent-foreground h-9 px-4 py-2"
							on:click|stopPropagation={removeItem}
						>
							Remove
						</button>
					</div>
				</div>
			{:else}
				<div class="flex items-center justify-center h-full">
					<p class="text-muted-foreground">Operator not found.</p>
				</div>
			{/if}
		</div>
	</div>
</div>

<style>
	.item {
		transition: box-shadow 0.2s ease;
		background-color: white;
		border-radius: 0.375rem;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		position: relative;
		display: flex;
		flex-direction: column;
	}

	.item:hover {
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
	}

	.content-area {
		pointer-events: auto;
		flex: 1;
		overflow: hidden;
	}

	.menubar-container {
		border-right: 1px solid rgba(0, 0, 0, 0.1);
		display: flex;
		align-items: flex-start;
		padding-top: 0.5rem;
	}
</style>
