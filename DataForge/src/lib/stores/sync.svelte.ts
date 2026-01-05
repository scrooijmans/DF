// Sync store using Svelte 5 runes (ColaNode-style Git-like sync)
import { invoke } from '@tauri-apps/api/core'
import type { SyncState, SyncConflict, SyncResult, SyncStatus, ConflictResolution, TestConnectionResult } from '$lib/types'

// Sync state
let syncState = $state<SyncState | null>(null)
let conflicts = $state<SyncConflict[]>([])
let isOnline = $state(typeof navigator !== 'undefined' ? navigator.onLine : true)
let syncError = $state<string | null>(null)

// Derived state
const status = $derived<SyncStatus>(syncState?.status ?? 'idle')
const lastSyncAt = $derived(syncState?.last_sync_at ?? null)
const pendingChanges = $derived(syncState?.pending_changes ?? 0)
const conflictCount = $derived(syncState?.conflicts ?? conflicts.length)
const hasConflicts = $derived(conflictCount > 0)
const needsSync = $derived(pendingChanges > 0)
const canSync = $derived(isOnline && status !== 'syncing')
const serverUrl = $derived(syncState?.server_url ?? '')
const isConfigured = $derived(serverUrl.length > 0)

// Initialize sync state for a workspace
async function initialize(workspaceId: string): Promise<void> {
	try {
		syncError = null

		const state = await invoke<SyncState>('get_sync_state', { workspaceId })
		syncState = state

		// Load pending conflicts
		const pendingConflicts = await invoke<SyncConflict[]>('get_conflicts', { workspaceId })
		conflicts = pendingConflicts

		console.log(`Sync initialized for workspace ${workspaceId}:`, {
			status: state.status,
			serverUrl: state.server_url,
			pendingChanges: state.pending_changes,
			conflicts: pendingConflicts.length
		})
	} catch (e) {
		console.error('Failed to initialize sync state:', e)
		syncError = e instanceof Error ? e.message : String(e)
		syncState = null
		conflicts = []
	}
}

// Initialize sync for a workspace with a server URL
async function initSync(workspaceId: string, serverUrl: string): Promise<boolean> {
	try {
		syncError = null

		const state = await invoke<SyncState>('init_sync', { workspaceId, serverUrl })
		syncState = state

		console.log(`Sync configured for workspace ${workspaceId} with server ${serverUrl}`)
		return true
	} catch (e) {
		console.error('Failed to initialize sync:', e)
		syncError = e instanceof Error ? e.message : String(e)
		return false
	}
}

// Update sync server URL
async function setServerUrl(workspaceId: string, serverUrl: string): Promise<boolean> {
	try {
		syncError = null

		await invoke('set_sync_server', { workspaceId, serverUrl })

		if (syncState) {
			syncState = { ...syncState, server_url: serverUrl }
		}

		console.log(`Updated sync server for workspace ${workspaceId} to ${serverUrl}`)
		return true
	} catch (e) {
		console.error('Failed to set sync server:', e)
		syncError = e instanceof Error ? e.message : String(e)
		return false
	}
}

// Trigger manual sync
async function sync(workspaceId: string): Promise<SyncResult | null> {
	if (!canSync) {
		console.warn('Cannot sync: offline or already syncing')
		return null
	}

	try {
		syncError = null

		// Update status to syncing
		if (syncState) {
			syncState = { ...syncState, status: 'syncing' }
		}

		const result = await invoke<SyncResult>('sync_workspace', { workspaceId })

		// Update state with result
		if (syncState) {
			syncState = {
				...syncState,
				status: result.error ? 'error' : 'idle',
				pending_changes: result.pending_changes,
				conflicts: result.conflicts,
				error: result.error,
				last_sync_at: result.success ? new Date().toISOString() : syncState.last_sync_at
			}
		}

		if (result.success) {
			console.log('Sync completed successfully')
		} else {
			console.warn('Sync completed with errors:', result.error)
		}

		// Refresh conflicts
		const pendingConflicts = await invoke<SyncConflict[]>('get_conflicts', { workspaceId })
		conflicts = pendingConflicts

		return result
	} catch (e) {
		console.error('Sync failed:', e)
		syncError = e instanceof Error ? e.message : String(e)

		if (syncState) {
			syncState = { ...syncState, status: 'error', error: syncError }
		}

		return null
	}
}

// Resolve a conflict
async function resolveConflict(
	conflictId: number,
	resolution: ConflictResolution
): Promise<boolean> {
	try {
		syncError = null

		await invoke('resolve_conflict', { conflictId, resolution })

		// Remove from local list
		conflicts = conflicts.filter((c) => c.id !== conflictId)

		// Update conflict count in state
		if (syncState) {
			syncState = { ...syncState, conflicts: conflicts.length }
		}

		console.log(`Resolved conflict ${conflictId} with resolution: ${resolution}`)
		return true
	} catch (e) {
		console.error('Failed to resolve conflict:', e)
		syncError = e instanceof Error ? e.message : String(e)
		return false
	}
}

// Refresh conflicts list
async function refreshConflicts(workspaceId: string): Promise<void> {
	try {
		const pendingConflicts = await invoke<SyncConflict[]>('get_conflicts', { workspaceId })
		conflicts = pendingConflicts
	} catch (e) {
		console.error('Failed to refresh conflicts:', e)
	}
}

// Get pending sync count
async function getPendingCount(workspaceId: string): Promise<number> {
	try {
		return await invoke<number>('get_pending_sync_count', { workspaceId })
	} catch (e) {
		console.error('Failed to get pending count:', e)
		return 0
	}
}

// Clear synced entries from queue
async function clearSyncedQueue(workspaceId: string): Promise<void> {
	try {
		await invoke('clear_synced_queue', { workspaceId })
		console.log('Cleared synced queue entries')
	} catch (e) {
		console.error('Failed to clear synced queue:', e)
	}
}

// Test connection to sync server
async function testConnection(serverUrl: string): Promise<TestConnectionResult> {
	try {
		const result = await invoke<TestConnectionResult>('test_sync_connection', { serverUrl })
		console.log('Connection test result:', result)
		return result
	} catch (e) {
		console.error('Connection test failed:', e)
		return {
			success: false,
			message: e instanceof Error ? e.message : String(e),
			server_version: null
		}
	}
}

// Clear error
function clearError(): void {
	syncError = null
	if (syncState) {
		syncState = { ...syncState, error: null }
	}
}

// Reset store (on workspace change or logout)
function reset(): void {
	syncState = null
	conflicts = []
	syncError = null
}

// Listen for online/offline events (browser-side only)
if (typeof window !== 'undefined') {
	window.addEventListener('online', () => {
		isOnline = true
		if (syncState?.status === 'offline') {
			syncState = { ...syncState, status: 'idle' }
		}
		console.log('Network connection restored')
	})

	window.addEventListener('offline', () => {
		isOnline = false
		if (syncState) {
			syncState = { ...syncState, status: 'offline' }
		}
		console.log('Network connection lost')
	})
}

// Export the store
export const syncStore = {
	// State getters (reactive)
	get syncState() {
		return syncState
	},
	get status() {
		return status
	},
	get lastSyncAt() {
		return lastSyncAt
	},
	get pendingChanges() {
		return pendingChanges
	},
	get conflicts() {
		return conflicts
	},
	get conflictCount() {
		return conflictCount
	},
	get hasConflicts() {
		return hasConflicts
	},
	get needsSync() {
		return needsSync
	},
	get canSync() {
		return canSync
	},
	get isOnline() {
		return isOnline
	},
	get serverUrl() {
		return serverUrl
	},
	get isConfigured() {
		return isConfigured
	},
	get error() {
		return syncError
	},

	// Actions
	initialize,
	initSync,
	setServerUrl,
	sync,
	testConnection,
	resolveConflict,
	refreshConflicts,
	getPendingCount,
	clearSyncedQueue,
	clearError,
	reset
}
