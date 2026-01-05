<script lang="ts">
	import { authStore, membersStore, syncStore } from '$lib/stores'
	import { goto } from '$app/navigation'
	import { AppSidebar } from '$lib/components/layout'

	let { children } = $props()

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
		{@render children()}
	</main>
</div>
