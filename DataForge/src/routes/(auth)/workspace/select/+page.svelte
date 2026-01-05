<script lang="ts">
	import { authStore } from '$lib'
	import { goto } from '$app/navigation'
	import WorkspaceCard from '$lib/components/workspaces/WorkspaceCard.svelte'

	let loading = $state(false)
	let selectedId = $state<string | null>(null)

	// Redirect to workspace creation if there are no workspaces
	$effect(() => {
		if (authStore.workspaces.length === 0) {
			goto('/workspace/new')
		}
	})

	async function selectWorkspace(workspaceId: string) {
		if (loading) return

		loading = true
		selectedId = workspaceId

		try {
			const success = await authStore.selectWorkspace(workspaceId)
			if (success) {
				goto('/')
			}
		} finally {
			loading = false
			selectedId = null
		}
	}

	function handleLogout() {
		authStore.logout()
		goto('/login')
	}
</script>

<div class="flex min-h-screen items-center justify-center p-4">
	<div class="bg-card w-full max-w-lg rounded-lg border p-8 shadow-lg">
		<!-- Header -->
		<div class="mb-6 text-center">
			<h1 class="text-2xl font-bold">Select Workspace</h1>
			<p class="text-muted-foreground mt-2">
				Choose a workspace to continue
			</p>
		</div>

		<!-- User Info -->
		{#if authStore.account}
			<div class="bg-muted/50 mb-6 flex items-center gap-3 rounded-md p-3">
				<div class="bg-primary text-primary-foreground flex h-10 w-10 items-center justify-center rounded-full text-sm font-medium">
					{authStore.account.name.charAt(0).toUpperCase()}
				</div>
				<div class="flex-1 truncate">
					<p class="font-medium">{authStore.account.name}</p>
					<p class="text-muted-foreground text-sm">{authStore.account.email}</p>
				</div>
			</div>
		{/if}

		<!-- Workspace Grid -->
		<div class="grid gap-3 sm:grid-cols-2">
			{#each authStore.workspaces as workspace (workspace.id)}
				<WorkspaceCard
					{workspace}
					isSelected={selectedId === workspace.id}
					loading={loading}
					onSelect={selectWorkspace}
				/>
			{/each}
		</div>

		<!-- Create New Workspace -->
		<div class="mt-6 border-t pt-6">
			<a
				href="/workspace/new"
				class="bg-primary text-primary-foreground hover:bg-primary/90 flex w-full items-center justify-center gap-2 rounded-md px-4 py-2 text-sm font-medium transition-colors"
			>
				<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
				</svg>
				Create New Workspace
			</a>
		</div>

		<!-- Logout -->
		<div class="mt-4 text-center">
			<button
				type="button"
				onclick={handleLogout}
				class="text-muted-foreground hover:text-foreground text-sm"
			>
				Sign out
			</button>
		</div>
	</div>
</div>
