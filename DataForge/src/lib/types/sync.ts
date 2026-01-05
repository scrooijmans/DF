// Sync types matching Rust backend (ColaNode-style Git-like sync)

export type SyncStatus = 'idle' | 'syncing' | 'error' | 'offline'

export interface SyncState {
	workspace_id: string
	server_url: string
	status: SyncStatus
	last_sync_at: string | null
	pending_changes: number
	conflicts: number
	error: string | null
}

export interface SyncConflict {
	id: number
	workspace_id: string
	entity_type: string
	entity_id: string
	local_version: number
	local_data: string
	remote_version: number
	remote_data: string
	created_at: string
}

export interface SyncResult {
	success: boolean
	pending_changes: number
	conflicts: number
	error: string | null
}

export type ConflictResolution = 'local' | 'remote' | 'merged'

export interface TestConnectionResult {
	success: boolean
	message: string
	server_version: string | null
}
