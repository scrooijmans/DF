<script lang="ts">
	/**
	 * TransferOwnershipDialog - Modal for transferring workspace ownership
	 *
	 * Allows the current owner to transfer ownership to an admin.
	 * The current owner will become an admin after transfer.
	 */
	import { membersStore } from '$lib/stores'
	import type { WorkspaceMember } from '$lib/types'

	interface Props {
		/** Whether the dialog is open */
		open: boolean
		/** Callback when dialog closes */
		onClose: () => void
		/** Callback when transfer is complete */
		onTransferred?: () => void
	}

	let { open = $bindable(), onClose, onTransferred }: Props = $props()

	let selectedAdminId = $state<string | null>(null)
	let isTransferring = $state(false)
	let error = $state<string | null>(null)

	const admins = $derived(membersStore.admins)
	const hasAdmins = $derived(membersStore.hasAdmins)

	function getSelectedAdmin(): WorkspaceMember | undefined {
		return admins.find((a) => a.account_id === selectedAdminId)
	}

	async function handleTransfer() {
		if (!selectedAdminId) return

		isTransferring = true
		error = null

		try {
			const success = await membersStore.transferOwnership(selectedAdminId)
			if (success) {
				onTransferred?.()
				handleClose()
			} else {
				error = membersStore.error || 'Failed to transfer ownership'
			}
		} catch (e) {
			error = e instanceof Error ? e.message : String(e)
		} finally {
			isTransferring = false
		}
	}

	function handleClose() {
		selectedAdminId = null
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
		aria-labelledby="transfer-dialog-title"
		tabindex="-1"
	>
		<!-- Modal content -->
		<div class="bg-background w-full max-w-md rounded-lg shadow-xl">
			<!-- Header -->
			<div class="border-border flex items-center justify-between border-b px-6 py-4">
				<div>
					<h2 id="transfer-dialog-title" class="text-lg font-semibold">Transfer Ownership</h2>
					<p class="text-muted-foreground text-sm">Transfer workspace to an admin</p>
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
				{#if !hasAdmins}
					<div class="rounded-md border border-yellow-500/30 bg-yellow-500/10 p-4">
						<p class="text-sm text-yellow-400">
							<strong>No admins available.</strong> You must have at least one admin in the workspace
							before you can transfer ownership. Invite a user or promote an existing member to admin
							first.
						</p>
					</div>
				{:else}
					<div class="rounded-md border border-blue-500/30 bg-blue-500/10 p-4">
						<p class="text-sm">
							<strong>Note:</strong> After transferring ownership, you will become an admin of this
							workspace. This action can only be reversed by the new owner.
						</p>
					</div>

					<div class="space-y-2">
						<label for="select-admin" class="text-sm font-medium">Select New Owner</label>
						<select
							id="select-admin"
							bind:value={selectedAdminId}
							class="border-input bg-background focus:ring-ring w-full rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-2"
							disabled={isTransferring}
						>
							<option value={null}>Choose an admin...</option>
							{#each admins as admin}
								<option value={admin.account_id}>
									{admin.email}
								</option>
							{/each}
						</select>
					</div>

					{#if selectedAdminId}
						{@const selected = getSelectedAdmin()}
						{#if selected}
							<div class="bg-secondary/50 rounded-md p-3">
								<p class="text-sm">
									<strong>{selected.email}</strong> will become the new owner of this workspace.
								</p>
							</div>
						{/if}
					{/if}
				{/if}

				{#if error}
					<p class="text-sm text-red-500">{error}</p>
				{/if}
			</div>

			<!-- Actions -->
			<div class="border-border flex justify-end gap-3 border-t px-6 py-4">
				<button
					onclick={handleClose}
					disabled={isTransferring}
					class="bg-secondary text-secondary-foreground hover:bg-secondary/80 rounded-md px-4 py-2 text-sm font-medium disabled:opacity-50"
				>
					Cancel
				</button>
				<button
					onclick={handleTransfer}
					disabled={!selectedAdminId || isTransferring || !hasAdmins}
					class="bg-primary text-primary-foreground hover:bg-primary/90 flex items-center gap-2 rounded-md px-4 py-2 text-sm font-medium disabled:cursor-not-allowed disabled:opacity-50"
				>
					{#if isTransferring}
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
					Transfer Ownership
				</button>
			</div>
		</div>
	</div>
{/if}
