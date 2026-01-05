<script lang="ts">
	import '../app.css'
	import { authStore, themeStore } from '$lib'
	import { goto } from '$app/navigation'
	import { page } from '$app/stores'

	let { children } = $props()

	// Initialize theme store
	themeStore.initialize()

	// Track if auth has been initialized to avoid multiple calls
	let authInitialized = $state(false)

	// Initialize auth and handle routing based on auth state
	$effect(() => {
		// Initialize auth once
		if (!authInitialized) {
			authStore.initialize().then(() => {
				authInitialized = true
			})
			return
		}

		// Don't redirect while auth is still loading
		if (authStore.authStatus === 'loading') {
			return
		}

		const currentPath = $page.url.pathname
		const isAuthRoute =
			currentPath === '/login' ||
			currentPath.startsWith('/workspace/new') ||
			currentPath.startsWith('/workspace/select')

		// If not authenticated, redirect to login (unless on auth routes)
		if (authStore.authStatus === 'unauthenticated') {
			if (!isAuthRoute) {
				console.log('🔒 Redirecting to login (unauthenticated)')
				goto('/login')
			}
			return
		}

		// If authenticated, handle workspace routing
		if (authStore.authStatus === 'authenticated') {
			const status = authStore.workspaceStatus()
			const needsWorkspaceSelection =
				status === 'needs_selection' || status === 'needs_creation'

			// If on login page, redirect based on workspace status
			if (currentPath === '/login') {
				if (authStore.workspaces.length === 0) {
					console.log('✅ Authenticated, redirecting to create workspace')
					goto('/workspace/new')
				} else if (!authStore.currentWorkspaceId || !authStore.currentWorkspace) {
					console.log('✅ Authenticated, redirecting to select workspace')
					goto('/workspace/select')
				} else {
					console.log('✅ Authenticated, redirecting to home')
					goto('/')
				}
			}

			// If on an app route but workspace is invalid (deleted), redirect to selection
			if (!isAuthRoute && needsWorkspaceSelection) {
				if (authStore.workspaces.length === 0) {
					console.log('⚠️ No workspaces, redirecting to create workspace')
					goto('/workspace/new')
				} else {
					console.log('⚠️ Invalid workspace, redirecting to select workspace')
					goto('/workspace/select')
				}
			}
		}
	})
</script>

<div class="bg-background flex h-screen flex-col">
	{@render children()}
</div>
