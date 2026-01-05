<script lang="ts">
	import { ingestStore, type IngestStep } from '$lib/stores/ingest.svelte'
	import { goto } from '$app/navigation'
	import { page } from '$app/stores'

	const { children } = $props()

	// Step configuration
	const steps: { id: IngestStep; label: string; path: string }[] = [
		{ id: 'source', label: 'Select Source', path: '/ingest/source' },
		{ id: 'upload', label: 'Upload Files', path: '/ingest/upload' },
		{ id: 'mapping', label: 'Map Data', path: '/ingest/mapping' },
		{ id: 'confirm', label: 'Confirm', path: '/ingest/confirm' }
	]

	function handleStepClick(step: (typeof steps)[0]) {
		if (ingestStore.canAccessStep(step.id)) {
			ingestStore.goToStep(step.id)
			goto(step.path)
		}
	}

	function getStepStatus(step: (typeof steps)[0]): 'completed' | 'current' | 'upcoming' {
		const currentIndex = ingestStore.getStepIndex(ingestStore.currentStep)
		const stepIndex = ingestStore.getStepIndex(step.id)

		if (stepIndex < currentIndex) return 'completed'
		if (stepIndex === currentIndex) return 'current'
		return 'upcoming'
	}
</script>

<div class="flex h-full flex-col">
	<!-- Header with progress stepper -->
	<div class="border-border border-b bg-background px-6 py-4">
		<div class="mx-auto max-w-4xl">
			<h1 class="mb-4 text-xl font-semibold">Data Ingestion</h1>

			<!-- Progress Stepper -->
			<nav aria-label="Progress">
				<ol class="flex items-center">
					{#each steps as step, index}
						{@const status = getStepStatus(step)}
						{@const canAccess = ingestStore.canAccessStep(step.id)}

						<li class="relative {index !== steps.length - 1 ? 'flex-1' : ''}">
							<div class="flex items-center">
								<!-- Step circle -->
								<button
									onclick={() => handleStepClick(step)}
									disabled={!canAccess}
									class="group flex items-center"
									aria-current={status === 'current' ? 'step' : undefined}
								>
									<span
										class="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full border-2 text-sm font-medium transition-colors
										{status === 'completed'
											? 'border-primary bg-primary text-primary-foreground'
											: status === 'current'
												? 'border-primary bg-background text-primary'
												: canAccess
													? 'border-muted-foreground/30 bg-background text-muted-foreground hover:border-muted-foreground'
													: 'border-muted-foreground/20 bg-muted text-muted-foreground/50'}"
									>
										{#if status === 'completed'}
											<svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
												<path
													fill-rule="evenodd"
													d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
													clip-rule="evenodd"
												/>
											</svg>
										{:else}
											{index + 1}
										{/if}
									</span>
									<span
										class="ml-3 text-sm font-medium
										{status === 'current'
											? 'text-foreground'
											: status === 'completed'
												? 'text-muted-foreground'
												: 'text-muted-foreground/50'}"
									>
										{step.label}
									</span>
								</button>

								<!-- Connector line -->
								{#if index !== steps.length - 1}
									<div class="ml-4 flex-1">
										<div
											class="h-0.5 {status === 'completed'
												? 'bg-primary'
												: 'bg-muted-foreground/20'}"
										></div>
									</div>
								{/if}
							</div>
						</li>
					{/each}
				</ol>
			</nav>
		</div>
	</div>

	<!-- Content area -->
	<div class="flex-1 overflow-auto">
		<div class="mx-auto max-w-4xl px-6 py-6">
			{@render children()}
		</div>
	</div>
</div>
