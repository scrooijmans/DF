<script lang="ts">
	import { authStore } from '$lib'
	import { goto } from '$app/navigation'

	let name = $state('')
	let description = $state('')
	let loading = $state(false)
	let error = $state<string | null>(null)

	const isValid = $derived(name.trim().length >= 2)

	async function handleSubmit(e: Event) {
		e.preventDefault()
		if (!isValid || loading) return

		loading = true
		error = null

		try {
			const workspace = await authStore.createWorkspace(name.trim(), description.trim() || undefined)

			if (workspace) {
				goto('/')
			} else {
				error = authStore.error || 'Failed to create workspace'
			}
		} finally {
			loading = false
		}
	}

	function handleBack() {
		if (authStore.workspaces.length > 0) {
			goto('/workspace/select')
		} else {
			authStore.logout()
			goto('/login')
		}
	}
</script>

<div class="flex min-h-screen items-center justify-center p-4">
	<div class="bg-card w-full max-w-md rounded-lg border p-8 shadow-lg">
		<!-- Header -->
		<div class="mb-6 text-center">
			<h1 class="text-2xl font-bold">Create Workspace</h1>
			<p class="text-muted-foreground mt-2">
				Set up a workspace to organize your projects
			</p>
		</div>

		<!-- Form -->
		<form onsubmit={handleSubmit} class="space-y-4">
			<div>
				<label for="name" class="text-sm font-medium">Workspace Name</label>
				<input
					id="name"
					type="text"
					bind:value={name}
					placeholder="My Workspace"
					class="border-input bg-background mt-1 w-full rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
					required
					disabled={loading}
					minlength={2}
				/>
				<p class="text-muted-foreground mt-1 text-xs">
					At least 2 characters
				</p>
			</div>

			<div>
				<label for="description" class="text-sm font-medium">
					Description
					<span class="text-muted-foreground font-normal">(optional)</span>
				</label>
				<textarea
					id="description"
					bind:value={description}
					placeholder="A brief description of this workspace..."
					rows={3}
					class="border-input bg-background mt-1 w-full resize-none rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
					disabled={loading}
				></textarea>
			</div>

			{#if error}
				<div class="bg-destructive/10 text-destructive rounded-md p-3 text-sm">
					{error}
				</div>
			{/if}

			<button
				type="submit"
				disabled={!isValid || loading}
				class="bg-primary text-primary-foreground hover:bg-primary/90 w-full rounded-md px-4 py-2 text-sm font-medium transition-colors disabled:cursor-not-allowed disabled:opacity-50"
			>
				{#if loading}
					<span class="inline-flex items-center gap-2">
						<svg class="h-4 w-4 animate-spin" viewBox="0 0 24 24" fill="none">
							<circle
								class="opacity-25"
								cx="12"
								cy="12"
								r="10"
								stroke="currentColor"
								stroke-width="4"
							/>
							<path
								class="opacity-75"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
							/>
						</svg>
						Creating...
					</span>
				{:else}
					Create Workspace
				{/if}
			</button>
		</form>

		<!-- Back -->
		<div class="mt-6 text-center">
			<button
				type="button"
				onclick={handleBack}
				class="text-muted-foreground hover:text-foreground text-sm"
			>
				{authStore.workspaces.length > 0 ? 'Back to workspace selection' : 'Sign out'}
			</button>
		</div>

		<!-- Info -->
		<div class="text-muted-foreground mt-8 border-t pt-6 text-center text-xs">
			<p>Workspaces help you organize projects and collaborate with team members.</p>
		</div>
	</div>
</div>
