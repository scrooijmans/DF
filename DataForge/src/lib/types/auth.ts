// Authentication types matching Rust backend

export interface Account {
	id: string
	email: string
	name: string
	avatar_url: string | null
	status: 'unverified' | 'verified' | 'suspended'
}

export interface Workspace {
	id: string
	name: string
	description: string | null
	avatar_url: string | null
	owner_id: string
	role: 'owner' | 'admin' | 'member' | 'viewer' | null
}

export interface AuthResponse {
	account: Account
	workspaces: Workspace[]
	current_workspace_id: string | null
}

export interface SessionState {
	authenticated: boolean
	account: Account | null
	workspaces: Workspace[]
	current_workspace_id: string | null
}

export type AuthStatus = 'loading' | 'authenticated' | 'unauthenticated'
export type WorkspaceStatus = 'loading' | 'selected' | 'needs_selection' | 'needs_creation'
export type WorkspaceRole = 'owner' | 'admin' | 'member' | 'viewer'

export interface WorkspaceMember {
	member_id: string
	account_id: string
	email: string
	name: string
	avatar_url: string | null
	role: WorkspaceRole
	joined_at: string
}
