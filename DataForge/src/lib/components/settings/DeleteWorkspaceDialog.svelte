<script lang="ts">
	/**
	 * DeleteWorkspaceDialog - Confirmation modal for deleting a workspace
	 *
	 * Requires user to type the workspace name to confirm deletion.
	 * Only workspace owners can delete workspaces.
	 */
	import { goto } from '$app/navigation'
	import { authStore } from '$lib/stores'

	interface Props {
		/** Whether the dialog is open */
		open: boolean
		/** Callback when dialog closes */
		onClose: () => void
	}

	let { open = $bindable(), onClose }: Props = $props()

	let confirmationInput = $state('')
	let isDeleting = $state(false)
	let error = $state<string | null>(null)

	const workspaceName = $derived(authStore.currentWorkspace?.name || '')
	const workspaceId = $derived(authStore.currentWorkspaceId || '')
	const isConfirmed = $derived(confirmationInput === workspaceName)

	async function handleDelete() {
		if (!isConfirmed || !workspaceId) return

		isDeleting = true
		error = null

		try {
			const success = await authStore.deleteWorkspace(workspaceId)
			if (success) {
				onClose()
				await goto('/workspace/select')
			} else {
				error = authStore.error || 'Failed to delete workspace'
			}
		} catch (e) {
			error = e instanceof Error ? e.message : String(e)
		} finally {
			isDeleting = false
		}
	}

	function handleClose() {
		confirmationInput = ''
		error = null
		onClose()
	}
</script>

{#if open}
	<!-- Modal backdrop -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
		onclick={(e) => {
			if (e.target === e.currentTarget) handleClose()
		}}
		onkeydown={(e) => {
			if (e.key === 'Escape') handleClose()
		}}
		role="dialog"
		aria-modal="true"
		aria-labelledby="delete-dialog-title"
		tabindex="-1"
	>
		<!-- Modal content -->
		<div class="bg-background w-full max-w-md rounded-lg shadow-xl">
			<!-- Header -->
			<div class="border-border flex items-center justify-between border-b px-6 py-4">
				<div>
					<h2 id="delete-dialog-title" class="text-lg font-semibold text-red-500">
						Delete Workspace
					</h2>
					<p class="text-muted-foreground text-sm">This action cannot be undone</p>
				</div>
				<button
					onclick={handleClose}
					class="text-muted-foreground hover:text-foreground"
					aria-label="Close dialog"
				>
					<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M6 18L18 6M6 6l12 12"
						/>
					</svg>
				</button>
			</div>

			<!-- Content -->
			<div class="space-y-4 px-6 py-4">
				<div class="rounded-md border border-red-500/30 bg-red-500/10 p-4">
					<p class="text-sm">
						<strong>Warning:</strong> Deleting this workspace will permanently remove all associated data
						including:
					</p>
					<ul class="mt-2 list-inside list-disc text-sm text-red-400">
						<li>All wells and curve data</li>
						<li>All trajectories and markers</li>
						<li>All surfaces and checkshots</li>
						<li>All member associations</li>
					</ul>
				</div>

				<div class="space-y-2">
					<label for="confirm-workspace-name" class="text-sm font-medium">
						Type <span class="font-mono text-red-500">{workspaceName}</span> to confirm
					</label>
					<input
						id="confirm-workspace-name"
						type="text"
						bind:value={confirmationInput}
						placeholder="Enter workspace name"
						class="border-input bg-background placeholder:text-muted-foreground focus:ring-ring w-full rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-2"
						disabled={isDeleting}
					/>
				</div>

				{#if error}
					<p class="text-sm text-red-500">{error}</p>
				{/if}
			</div>

			<!-- Actions -->
			<div class="border-border flex justify-end gap-3 border-t px-6 py-4">
				<button
					onclick={handleClose}
					disabled={isDeleting}
					class="bg-secondary text-secondary-foreground hover:bg-secondary/80 rounded-md px-4 py-2 text-sm font-medium disabled:opacity-50"
				>
					Cancel
				</button>
				<button
					onclick={handleDelete}
					disabled={!isConfirmed || isDeleting}
					class="flex items-center gap-2 rounded-md bg-red-500 px-4 py-2 text-sm font-medium text-white hover:bg-red-600 disabled:cursor-not-allowed disabled:opacity-50"
				>
					{#if isDeleting}
						<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
							<circle
								class="opacity-25"
								cx="12"
								cy="12"
								r="10"
								stroke="currentColor"
								stroke-width="4"
							></circle>
							<path
								class="opacity-75"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							></path>
						</svg>
					{/if}
					Delete Workspace
				</button>
			</div>
		</div>
	</div>
{/if}
