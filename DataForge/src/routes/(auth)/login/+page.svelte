<script lang="ts">
	import { authStore } from '$lib'
	import { goto } from '$app/navigation'

	let mode = $state<'login' | 'register'>('login')
	let email = $state('')
	let password = $state('')
	let name = $state('')
	let loading = $state(false)
	let localError = $state<string | null>(null)

	// Wait for backend to be ready before allowing login
	const backendReady = $derived(authStore.authStatus !== 'loading')

	const isValid = $derived(
		backendReady &&
			(mode === 'login'
				? email.trim() !== '' && password.trim() !== ''
				: email.trim() !== '' && password.trim() !== '' && name.trim() !== '')
	)

	async function handleSubmit(e: Event) {
		e.preventDefault()
		if (!isValid || loading) return

		loading = true
		localError = null
		authStore.clearError()

		try {
			let success: boolean

			if (mode === 'login') {
				success = await authStore.login(email, password)
			} else {
				success = await authStore.register(email, password, name)
			}

			if (success) {
				// Navigate based on workspace status
				if (authStore.workspaces.length === 0) {
					goto('/workspace/new')
				} else if (!authStore.currentWorkspaceId) {
					goto('/workspace/select')
				} else {
					goto('/')
				}
			} else {
				localError = authStore.error
			}
		} finally {
			loading = false
		}
	}

	function toggleMode() {
		mode = mode === 'login' ? 'register' : 'login'
		localError = null
		authStore.clearError()
	}
</script>

<div class="flex min-h-screen items-center justify-center p-4">
	<div class="bg-card w-full max-w-md rounded-lg border p-8 shadow-lg">
		<!-- Logo / Title -->
		<div class="mb-8 text-center">
			<h1 class="text-3xl font-bold">DataForge</h1>
			<p class="text-muted-foreground mt-2">
				{mode === 'login' ? 'Sign in to your account' : 'Create a new account'}
			</p>
		</div>

		<!-- Backend Loading Indicator -->
		{#if !backendReady}
			<div class="mb-4 flex items-center justify-center gap-2 text-muted-foreground">
				<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
					<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
					<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
				</svg>
				<span class="text-sm">Initializing...</span>
			</div>
		{/if}

		<!-- Form -->
		<form onsubmit={handleSubmit} class="space-y-4">
			{#if mode === 'register'}
				<div>
					<label for="name" class="text-sm font-medium">Name</label>
					<input
						id="name"
						type="text"
						bind:value={name}
						placeholder="Your name"
						class="border-input bg-background mt-1 w-full rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
						required
						disabled={loading || !backendReady}
					/>
				</div>
			{/if}

			<div>
				<label for="email" class="text-sm font-medium">Email</label>
				<input
					id="email"
					type="email"
					bind:value={email}
					placeholder="you@example.com"
					class="border-input bg-background mt-1 w-full rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
					required
					disabled={loading || !backendReady}
				/>
			</div>

			<div>
				<label for="password" class="text-sm font-medium">Password</label>
				<input
					id="password"
					type="password"
					bind:value={password}
					placeholder="Enter your password"
					class="border-input bg-background mt-1 w-full rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
					required
					disabled={loading || !backendReady}
					minlength={6}
				/>
			</div>

			{#if localError || authStore.error}
				<div class="bg-destructive/10 text-destructive rounded-md p-3 text-sm">
					{localError || authStore.error}
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
						{mode === 'login' ? 'Signing in...' : 'Creating account...'}
					</span>
				{:else}
					{mode === 'login' ? 'Sign in' : 'Create account'}
				{/if}
			</button>
		</form>

		<!-- Toggle Mode -->
		<div class="mt-6 text-center">
			<p class="text-muted-foreground text-sm">
				{mode === 'login' ? "Don't have an account?" : 'Already have an account?'}
				<button
					type="button"
					onclick={toggleMode}
					class="text-primary ml-1 font-medium hover:underline"
					disabled={loading || !backendReady}
				>
					{mode === 'login' ? 'Sign up' : 'Sign in'}
				</button>
			</p>
		</div>

		<!-- Info -->
		<div class="text-muted-foreground mt-8 border-t pt-6 text-center text-xs">
			<p>Offline-first geoscience data platform</p>
			<p class="mt-1">Your data stays local and syncs when you're ready.</p>
		</div>
	</div>
</div>
