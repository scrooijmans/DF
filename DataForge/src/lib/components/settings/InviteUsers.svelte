<script lang="ts">
	import { membersStore } from '$lib/stores'
	import type { WorkspaceRole } from '$lib/types'

	let email = $state('')
	let role = $state<WorkspaceRole>('member')
	let isLoading = $state(false)
	let errorMessage = $state('')
	let successMessage = $state('')

	async function handleInvite() {
		if (!email.trim()) {
			errorMessage = 'Please enter an email address'
			return
		}

		// Basic email validation
		if (!email.includes('@') || !email.includes('.')) {
			errorMessage = 'Please enter a valid email address'
			return
		}

		isLoading = true
		errorMessage = ''
		successMessage = ''

		const result = await membersStore.addMember(email.trim(), role)

		if (result) {
			successMessage = `Successfully added ${result.name} to the workspace`
			email = ''
			role = 'member'
		} else {
			errorMessage = membersStore.error || 'Failed to add member'
		}

		isLoading = false
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter' && !isLoading) {
			handleInvite()
		}
	}
</script>

<div class="space-y-4">
	<div>
		<h2 class="text-lg font-semibold">Invite</h2>
		<p class="text-muted-foreground text-sm">
			Write the email addresses of the people you want to invite
		</p>
	</div>

	<div class="flex gap-3">
		<div class="flex-1">
			<input
				type="email"
				bind:value={email}
				onkeydown={handleKeydown}
				placeholder="Enter email address"
				disabled={isLoading}
				class="border-input bg-background placeholder:text-muted-foreground focus:ring-ring w-full rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-2 disabled:opacity-50"
			/>
		</div>
		<select
			bind:value={role}
			disabled={isLoading}
			class="border-input bg-background focus:ring-ring rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-2 disabled:opacity-50"
		>
			<option value="member">Member</option>
			<option value="admin">Admin</option>
			<option value="viewer">Viewer</option>
		</select>
		<button
			onclick={handleInvite}
			disabled={isLoading || !email.trim()}
			class="bg-secondary text-secondary-foreground hover:bg-secondary/80 disabled:bg-muted disabled:text-muted-foreground rounded-md px-6 py-2 text-sm font-medium transition-colors disabled:cursor-not-allowed"
		>
			{isLoading ? 'Adding...' : 'Invite'}
		</button>
	</div>

	{#if errorMessage}
		<p class="text-sm text-red-500">{errorMessage}</p>
	{/if}

	{#if successMessage}
		<p class="text-sm text-green-500">{successMessage}</p>
	{/if}
</div>
