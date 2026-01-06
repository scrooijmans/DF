<script lang="ts">
	import { authStore, membersStore, syncStore } from '$lib/stores'
	import { goto } from '$app/navigation'
	import { AppSidebar } from '$lib/components/layout'

	let { children } = $props()

	// Workspace is ready when auth is complete AND a workspace is selected
	const workspaceReady = $derived(
		authStore.authStatus === 'authenticated' && !!authStore.currentWorkspaceId
	)

	// Load members and initialize sync when workspace changes
	$effect(() => {
		if (authStore.currentWorkspaceId) {
			membersStore.loadMembers(authStore.currentWorkspaceId)
			syncStore.initialize(authStore.currentWorkspaceId)
		}
	})

	async function handleLogout() {
		membersStore.reset()
		syncStore.reset()
		await authStore.logout()
		goto('/login')
	}
</script>

<div class="flex h-screen">
	<AppSidebar onLogout={handleLogout} />

	<main class="flex-1 overflow-auto">
		{#if workspaceReady}
			{@render children()}
		{:else}
			<div class="flex h-full items-center justify-center">
				<div class="flex items-center gap-2 text-muted-foreground">
					<svg class="h-5 w-5 animate-spin" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
						<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
					</svg>
					<span>Loading...</span>
				</div>
			</div>
		{/if}
	</main>
</div>
