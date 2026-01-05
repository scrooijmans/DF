<script lang="ts">
	import { authStore, membersStore } from '$lib/stores'
	import { DeleteWorkspaceDialog, TransferOwnershipDialog } from '$lib/components/settings'

	let workspaceName = $state(authStore.currentWorkspace?.name || '')
	let workspaceDescription = $state(authStore.currentWorkspace?.description || '')
	let isSaving = $state(false)
	let saveError = $state<string | null>(null)
	let showDeleteDialog = $state(false)
	let showTransferDialog = $state(false)

	// Only owner and admin can edit workspace
	const canEdit = $derived(membersStore.isOwner || membersStore.canManageMembers)

	async function handleSave() {
		const workspaceId = authStore.currentWorkspaceId
		if (!workspaceId) return

		isSaving = true
		saveError = null

		try {
			const updated = await authStore.updateWorkspace(workspaceId, workspaceName, workspaceDescription)
			if (!updated) {
				saveError = authStore.error || 'Failed to save changes'
			}
		} catch (e) {
			saveError = e instanceof Error ? e.message : String(e)
		} finally {
			isSaving = false
		}
	}
</script>

<div class="mx-auto max-w-2xl space-y-8">
	<section class="border-border rounded-lg border p-6">
		<div class="space-y-6">
			<div>
				<h2 class="text-lg font-semibold">General Settings</h2>
				<p class="text-muted-foreground text-sm">Manage your workspace settings</p>
			</div>

			<div class="space-y-4">
				<div class="space-y-2">
					<label for="workspace-name" class="text-sm font-medium">Workspace Name</label>
					<input
						id="workspace-name"
						type="text"
						bind:value={workspaceName}
						disabled={!canEdit}
						class="border-input bg-background placeholder:text-muted-foreground focus:ring-ring w-full rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-2 disabled:cursor-not-allowed disabled:opacity-50"
					/>
				</div>

				<div class="space-y-2">
					<label for="workspace-description" class="text-sm font-medium">Description</label>
					<textarea
						id="workspace-description"
						bind:value={workspaceDescription}
						rows={3}
						disabled={!canEdit}
						class="border-input bg-background placeholder:text-muted-foreground focus:ring-ring w-full resize-none rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-2 disabled:cursor-not-allowed disabled:opacity-50"
						placeholder="Optional description for your workspace"
					></textarea>
				</div>

				{#if saveError}
					<p class="text-sm text-red-500">{saveError}</p>
				{/if}

				{#if canEdit}
					<div class="flex justify-end">
						<button
							onclick={handleSave}
							disabled={isSaving}
							class="bg-primary text-primary-foreground hover:bg-primary/90 rounded-md px-4 py-2 text-sm font-medium disabled:opacity-50"
						>
							{isSaving ? 'Saving...' : 'Save Changes'}
						</button>
					</div>
				{:else}
					<p class="text-muted-foreground text-sm">
						Only workspace owners and admins can edit these settings.
					</p>
				{/if}
			</div>
		</div>
	</section>

	<!-- Danger Zone -->
	{#if membersStore.isOwner}
		<section class="rounded-lg border border-red-500/50 p-6">
			<div class="space-y-4">
				<div>
					<h2 class="text-lg font-semibold text-red-500">Danger Zone</h2>
					<p class="text-muted-foreground text-sm">
						Irreversible actions for this workspace
					</p>
				</div>

				<!-- Transfer Ownership -->
				<div class="flex items-center justify-between">
					<div>
						<p class="text-sm font-medium">Transfer Ownership</p>
						<p class="text-muted-foreground text-xs">
							Transfer this workspace to an admin
						</p>
					</div>
					<button
						onclick={() => (showTransferDialog = true)}
						class="border-border hover:bg-secondary rounded-md border px-4 py-2 text-sm font-medium"
					>
						Transfer Ownership
					</button>
				</div>

				<!-- Delete Workspace -->
				<div class="flex items-center justify-between">
					<div>
						<p class="text-sm font-medium">Delete Workspace</p>
						<p class="text-muted-foreground text-xs">
							Permanently delete this workspace and all its data
						</p>
					</div>
					<button
						onclick={() => (showDeleteDialog = true)}
						class="rounded-md border border-red-500 px-4 py-2 text-sm font-medium text-red-500 hover:bg-red-500/10"
					>
						Delete Workspace
					</button>
				</div>
			</div>
		</section>
	{/if}
</div>

<!-- Dialogs -->
<DeleteWorkspaceDialog bind:open={showDeleteDialog} onClose={() => (showDeleteDialog = false)} />
<TransferOwnershipDialog bind:open={showTransferDialog} onClose={() => (showTransferDialog = false)} />
