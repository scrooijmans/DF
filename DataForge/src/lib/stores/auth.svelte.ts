// Authentication store using Svelte 5 runes
import { invoke } from '@tauri-apps/api/core'
import type { Account, Workspace, AuthResponse, SessionState, AuthStatus, WorkspaceStatus } from '$lib/types'

// Auth state
let account = $state<Account | null>(null)
let workspaces = $state<Workspace[]>([])
let currentWorkspaceId = $state<string | null>(null)
let authStatus = $state<AuthStatus>('loading')
let error = $state<string | null>(null)

// Derived state
const isAuthenticated = $derived(authStatus === 'authenticated')
const currentWorkspace = $derived(
	workspaces.find((w) => w.id === currentWorkspaceId) ?? null
)
const workspaceStatus = $derived((): WorkspaceStatus => {
	if (authStatus === 'loading') return 'loading'
	if (!isAuthenticated) return 'loading'
	if (currentWorkspaceId && currentWorkspace) return 'selected'
	if (workspaces.length > 0) return 'needs_selection'
	return 'needs_creation'
})

// Initialize auth state from backend (ColaNode pattern: wait for backend readiness)
async function initialize(): Promise<void> {
	try {
		authStatus = 'loading'
		error = null

		// ColaNode pattern: Wait for backend to be ready before querying session
		// Optimized polling with exponential backoff (max 2 seconds)
		let ready = false
		const maxAttempts = 10
		let attempts = 0
		let delay = 50 // Start with 50ms

		while (!ready && attempts < maxAttempts) {
			try {
				ready = await invoke<boolean>('is_ready')
				if (!ready) {
					await new Promise(resolve => setTimeout(resolve, delay))
					delay = Math.min(delay * 1.5, 200) // Exponential backoff, max 200ms
					attempts++
				}
			} catch (e) {
				// Backend might not be ready yet, continue polling
				await new Promise(resolve => setTimeout(resolve, delay))
				delay = Math.min(delay * 1.5, 200)
				attempts++
			}
		}

		if (!ready) {
			console.warn('⚠️ Backend not ready after polling, proceeding anyway')
		}

		// Now query session state (ColaNode pattern: backend validates before returning)
		const session = await invoke<SessionState>('get_session')

		if (session.authenticated && session.account) {
			account = session.account
			workspaces = session.workspaces
			currentWorkspaceId = session.current_workspace_id
			authStatus = 'authenticated'
			console.log('✅ Auth initialized: authenticated as', session.account.email)
			console.log(`📦 Found ${workspaces.length} workspace(s)`)
		} else {
			account = null
			workspaces = []
			currentWorkspaceId = null
			authStatus = 'unauthenticated'
			console.log('ℹ️ Auth initialized: not authenticated')
		}
	} catch (e) {
		console.error('❌ Failed to initialize auth:', e)
		error = e instanceof Error ? e.message : String(e)
		// On error, assume unauthenticated rather than staying in loading state
		account = null
		workspaces = []
		currentWorkspaceId = null
		authStatus = 'unauthenticated'
	}
}

// Register a new account
async function register(email: string, password: string, name: string): Promise<boolean> {
	try {
		error = null

		const result = await invoke<AuthResponse>('register', { email, password, name })

		account = result.account
		workspaces = result.workspaces
		currentWorkspaceId = result.current_workspace_id
		authStatus = 'authenticated'

		return true
	} catch (e) {
		console.error('Registration failed:', e)
		error = e instanceof Error ? e.message : String(e)
		return false
	}
}

// Login with email and password
async function login(email: string, password: string): Promise<boolean> {
	try {
		error = null

		const result = await invoke<AuthResponse>('login', { email, password })

		account = result.account
		workspaces = result.workspaces
		currentWorkspaceId = result.current_workspace_id
		authStatus = 'authenticated'

		return true
	} catch (e) {
		console.error('Login failed:', e)
		error = e instanceof Error ? e.message : String(e)
		return false
	}
}

// Logout
async function logout(): Promise<void> {
	try {
		await invoke('logout')
	} catch (e) {
		console.error('Logout error:', e)
	} finally {
		account = null
		workspaces = []
		currentWorkspaceId = null
		authStatus = 'unauthenticated'
		error = null
	}
}

// Create a new workspace
async function createWorkspace(name: string, description?: string): Promise<Workspace | null> {
	try {
		error = null

		const workspace = await invoke<Workspace>('create_workspace', { name, description })

		workspaces = [...workspaces, workspace]
		currentWorkspaceId = workspace.id

		return workspace
	} catch (e) {
		console.error('Failed to create workspace:', e)
		error = e instanceof Error ? e.message : String(e)
		return null
	}
}

// Select a workspace
async function selectWorkspace(workspaceId: string): Promise<boolean> {
	try {
		error = null

		await invoke('select_workspace', { workspaceId })
		currentWorkspaceId = workspaceId

		return true
	} catch (e) {
		console.error('Failed to select workspace:', e)
		error = e instanceof Error ? e.message : String(e)
		return false
	}
}

// Refresh workspaces list
async function refreshWorkspaces(): Promise<void> {
	try {
		const result = await invoke<Workspace[]>('get_workspaces')
		workspaces = result
	} catch (e) {
		console.error('Failed to refresh workspaces:', e)
	}
}

// Delete a workspace (owner only)
async function deleteWorkspace(workspaceId: string): Promise<boolean> {
	try {
		error = null

		await invoke('delete_workspace', { workspaceId })

		// Remove from local list
		workspaces = workspaces.filter((w) => w.id !== workspaceId)

		// If deleted workspace was current, clear selection
		if (currentWorkspaceId === workspaceId) {
			currentWorkspaceId = null
		}

		return true
	} catch (e) {
		console.error('Failed to delete workspace:', e)
		error = e instanceof Error ? e.message : String(e)
		return false
	}
}

// Update a workspace
async function updateWorkspace(
	workspaceId: string,
	name?: string,
	description?: string
): Promise<Workspace | null> {
	try {
		error = null

		const updated = await invoke<Workspace>('update_workspace', {
			workspaceId,
			name,
			description
		})

		// Update in local list
		workspaces = workspaces.map((w) => (w.id === workspaceId ? { ...w, ...updated } : w))

		return updated
	} catch (e) {
		console.error('Failed to update workspace:', e)
		error = e instanceof Error ? e.message : String(e)
		return null
	}
}

// Update account profile
async function updateAccount(name?: string): Promise<Account | null> {
	try {
		error = null

		const updated = await invoke<Account>('update_account', { name })

		// Update local account
		account = updated

		return updated
	} catch (e) {
		console.error('Failed to update account:', e)
		error = e instanceof Error ? e.message : String(e)
		return null
	}
}

// Clear error
function clearError(): void {
	error = null
}

// Export the store
export const authStore = {
	// State getters (reactive)
	get account() {
		return account
	},
	get workspaces() {
		return workspaces
	},
	get currentWorkspaceId() {
		return currentWorkspaceId
	},
	get currentWorkspace() {
		return currentWorkspace
	},
	get authStatus() {
		return authStatus
	},
	get workspaceStatus() {
		return workspaceStatus
	},
	get isAuthenticated() {
		return isAuthenticated
	},
	get error() {
		return error
	},

	// Actions
	initialize,
	register,
	login,
	logout,
	createWorkspace,
	selectWorkspace,
	refreshWorkspaces,
	deleteWorkspace,
	updateWorkspace,
	updateAccount,
	clearError
}
