// Workspace members store using Svelte 5 runes
import { invoke } from '@tauri-apps/api/core'
import type { WorkspaceMember, WorkspaceRole } from '$lib/types'

// State
let members = $state<WorkspaceMember[]>([])
let currentWorkspaceId = $state<string | null>(null)
let myRole = $state<WorkspaceRole | null>(null)
let isLoading = $state(false)
let error = $state<string | null>(null)

// Derived state
const canManageMembers = $derived(myRole === 'owner' || myRole === 'admin')
const canChangeRoles = $derived(myRole === 'owner' || myRole === 'admin')
const isOwner = $derived(myRole === 'owner')
const memberCount = $derived(members.length)
const admins = $derived(members.filter((m) => m.role === 'admin'))
const hasAdmins = $derived(admins.length > 0)

// Load members for a workspace
async function loadMembers(workspaceId: string): Promise<void> {
	try {
		isLoading = true
		error = null
		currentWorkspaceId = workspaceId

		const [memberList, role] = await Promise.all([
			invoke<WorkspaceMember[]>('get_workspace_members', { workspaceId }),
			invoke<string>('get_my_workspace_role', { workspaceId })
		])

		members = memberList
		myRole = role as WorkspaceRole

		console.log(`Loaded ${memberList.length} members for workspace ${workspaceId}`)
	} catch (e) {
		console.error('Failed to load workspace members:', e)
		error = e instanceof Error ? e.message : String(e)
		members = []
		myRole = null
	} finally {
		isLoading = false
	}
}

// Add a member by email
async function addMember(
	email: string,
	role: WorkspaceRole = 'member'
): Promise<WorkspaceMember | null> {
	if (!currentWorkspaceId) {
		error = 'No workspace selected'
		return null
	}

	try {
		error = null

		const newMember = await invoke<WorkspaceMember>('add_workspace_member', {
			workspaceId: currentWorkspaceId,
			email,
			role
		})

		// Add to local list
		members = [...members, newMember]

		console.log(`Added member ${email} to workspace`)
		return newMember
	} catch (e) {
		console.error('Failed to add member:', e)
		error = e instanceof Error ? e.message : String(e)
		return null
	}
}

// Update a member's role
async function updateMemberRole(
	targetAccountId: string,
	newRole: WorkspaceRole
): Promise<boolean> {
	if (!currentWorkspaceId) {
		error = 'No workspace selected'
		return false
	}

	try {
		error = null

		await invoke('update_workspace_member_role', {
			workspaceId: currentWorkspaceId,
			targetAccountId,
			newRole
		})

		// Update local list
		members = members.map((m) =>
			m.account_id === targetAccountId ? { ...m, role: newRole } : m
		)

		console.log(`Updated member ${targetAccountId} role to ${newRole}`)
		return true
	} catch (e) {
		console.error('Failed to update member role:', e)
		error = e instanceof Error ? e.message : String(e)
		return false
	}
}

// Remove a member
async function removeMember(targetAccountId: string): Promise<boolean> {
	if (!currentWorkspaceId) {
		error = 'No workspace selected'
		return false
	}

	try {
		error = null

		await invoke('remove_workspace_member', {
			workspaceId: currentWorkspaceId,
			targetAccountId
		})

		// Remove from local list
		members = members.filter((m) => m.account_id !== targetAccountId)

		console.log(`Removed member ${targetAccountId} from workspace`)
		return true
	} catch (e) {
		console.error('Failed to remove member:', e)
		error = e instanceof Error ? e.message : String(e)
		return false
	}
}

// Transfer workspace ownership to an admin
async function transferOwnership(newOwnerAccountId: string): Promise<boolean> {
	if (!currentWorkspaceId) {
		error = 'No workspace selected'
		return false
	}

	try {
		error = null

		await invoke('transfer_workspace_ownership', {
			workspaceId: currentWorkspaceId,
			newOwnerAccountId
		})

		// Update local member roles
		members = members.map((m) => {
			if (m.role === 'owner') {
				return { ...m, role: 'admin' as WorkspaceRole }
			}
			if (m.account_id === newOwnerAccountId) {
				return { ...m, role: 'owner' as WorkspaceRole }
			}
			return m
		})

		// Update myRole if I was the owner
		if (myRole === 'owner') {
			myRole = 'admin'
		}

		console.log(`Transferred ownership to ${newOwnerAccountId}`)
		return true
	} catch (e) {
		console.error('Failed to transfer ownership:', e)
		error = e instanceof Error ? e.message : String(e)
		return false
	}
}

// Clear error
function clearError(): void {
	error = null
}

// Reset store (on workspace change or logout)
function reset(): void {
	members = []
	currentWorkspaceId = null
	myRole = null
	error = null
	isLoading = false
}

// Export the store
export const membersStore = {
	// State getters (reactive)
	get members() {
		return members
	},
	get myRole() {
		return myRole
	},
	get isLoading() {
		return isLoading
	},
	get error() {
		return error
	},
	get canManageMembers() {
		return canManageMembers
	},
	get canChangeRoles() {
		return canChangeRoles
	},
	get isOwner() {
		return isOwner
	},
	get memberCount() {
		return memberCount
	},
	get currentWorkspaceId() {
		return currentWorkspaceId
	},
	get admins() {
		return admins
	},
	get hasAdmins() {
		return hasAdmins
	},

	// Actions
	loadMembers,
	addMember,
	updateMemberRole,
	removeMember,
	transferOwnership,
	clearError,
	reset
}
