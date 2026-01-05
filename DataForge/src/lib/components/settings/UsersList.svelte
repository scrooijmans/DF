<script lang="ts">
	import { membersStore, authStore } from '$lib/stores'
	import type { WorkspaceMember, WorkspaceRole } from '$lib/types'

	let editingMemberId = $state<string | null>(null)
	let confirmRemoveId = $state<string | null>(null)

	function getInitials(name: string): string {
		return name
			.split(' ')
			.map((n) => n.charAt(0))
			.join('')
			.toUpperCase()
			.slice(0, 2)
	}

	function getAvatarColor(name: string): string {
		const colors = [
			'bg-orange-500',
			'bg-green-500',
			'bg-blue-500',
			'bg-purple-500',
			'bg-pink-500',
			'bg-yellow-500',
			'bg-teal-500',
			'bg-indigo-500'
		]
		const index = name.charCodeAt(0) % colors.length
		return colors[index]
	}

	function getRoleLabel(role: WorkspaceRole): string {
		switch (role) {
			case 'owner':
				return 'Owner'
			case 'admin':
				return 'Admin'
			case 'member':
				return 'Member'
			case 'viewer':
				return 'Viewer'
			default:
				return role
		}
	}

	async function handleRoleChange(member: WorkspaceMember, newRole: WorkspaceRole) {
		await membersStore.updateMemberRole(member.account_id, newRole)
		editingMemberId = null
	}

	async function handleRemove(member: WorkspaceMember) {
		await membersStore.removeMember(member.account_id)
		confirmRemoveId = null
	}

	function canModifyMember(member: WorkspaceMember): boolean {
		// Cannot modify owner
		if (member.role === 'owner') return false
		// Cannot modify yourself
		if (member.account_id === authStore.account?.id) return false
		// Only owners and admins can modify
		return membersStore.canManageMembers
	}

	function canRemoveMember(member: WorkspaceMember): boolean {
		// Cannot remove owner
		if (member.role === 'owner') return false
		// Cannot remove yourself (use leave workspace instead)
		if (member.account_id === authStore.account?.id) return false
		// Admins cannot remove other admins
		if (membersStore.myRole === 'admin' && member.role === 'admin') return false
		// Only owners and admins can remove
		return membersStore.canManageMembers
	}
</script>

<div class="space-y-4">
	<div>
		<h2 class="text-lg font-semibold">Users</h2>
		<p class="text-muted-foreground text-sm">The list of all users on the workspace</p>
	</div>

	{#if membersStore.isLoading}
		<div class="text-muted-foreground py-8 text-center">Loading members...</div>
	{:else if membersStore.members.length === 0}
		<div class="text-muted-foreground py-8 text-center">No members found</div>
	{:else}
		<div class="space-y-2">
			{#each membersStore.members as member (member.member_id)}
				<div
					class="hover:bg-secondary/30 flex items-center justify-between rounded-lg px-3 py-3 transition-colors"
				>
					<div class="flex items-center gap-3">
						<!-- Avatar -->
						{#if member.avatar_url}
							<img
								src={member.avatar_url}
								alt={member.name}
								class="h-10 w-10 rounded-full object-cover"
							/>
						{:else}
							<div
								class="text-primary-foreground flex h-10 w-10 items-center justify-center rounded-full text-sm font-medium {getAvatarColor(
									member.name
								)}"
							>
								{getInitials(member.name)}
							</div>
						{/if}

						<!-- Name and Email -->
						<div>
							<p class="text-sm font-medium">
								{member.name}
								{#if member.account_id === authStore.account?.id}
									<span class="text-muted-foreground text-xs">(you)</span>
								{/if}
							</p>
							<p class="text-muted-foreground text-xs">{member.email}</p>
						</div>
					</div>

					<!-- Role Selector / Remove -->
					<div class="flex items-center gap-2">
						{#if confirmRemoveId === member.member_id}
							<!-- Confirm remove -->
							<span class="text-muted-foreground mr-2 text-xs">Remove?</span>
							<button
								onclick={() => handleRemove(member)}
								class="rounded px-2 py-1 text-xs text-red-500 hover:bg-red-500/10"
							>
								Yes
							</button>
							<button
								onclick={() => (confirmRemoveId = null)}
								class="text-muted-foreground hover:bg-secondary rounded px-2 py-1 text-xs"
							>
								No
							</button>
						{:else if editingMemberId === member.member_id && canModifyMember(member)}
							<!-- Role dropdown -->
							<select
								value={member.role}
								onchange={(e) => handleRoleChange(member, (e.target as HTMLSelectElement).value as WorkspaceRole)}
								class="border-input bg-background focus:ring-ring rounded-md border px-2 py-1 text-xs focus:outline-none focus:ring-2"
							>
								<option value="admin">Admin</option>
								<option value="member">Member</option>
								<option value="viewer">Viewer</option>
							</select>
							<button
								onclick={() => (editingMemberId = null)}
								class="text-muted-foreground hover:text-foreground ml-1"
								title="Cancel"
							>
								<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
									<path
										stroke-linecap="round"
										stroke-linejoin="round"
										stroke-width="2"
										d="M6 18L18 6M6 6l12 12"
									/>
								</svg>
							</button>
						{:else}
							<!-- Role display -->
							<button
								onclick={() => canModifyMember(member) && (editingMemberId = member.member_id)}
								disabled={!canModifyMember(member)}
								class="text-muted-foreground flex items-center gap-1 text-sm disabled:cursor-default {canModifyMember(
									member
								)
									? 'hover:text-foreground cursor-pointer'
									: ''}"
							>
								{getRoleLabel(member.role)}
								{#if canModifyMember(member)}
									<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M19 9l-7 7-7-7"
										/>
									</svg>
								{/if}
							</button>

							<!-- Remove button -->
							{#if canRemoveMember(member)}
								<button
									onclick={() => (confirmRemoveId = member.member_id)}
									class="text-muted-foreground hover:text-red-500 ml-2"
									title="Remove member"
								>
									<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											stroke-width="2"
											d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
										/>
									</svg>
								</button>
							{/if}
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{/if}

	{#if membersStore.error}
		<p class="text-sm text-red-500">{membersStore.error}</p>
	{/if}
</div>
