<script lang="ts">
	import { syncStore, authStore } from '$lib/stores'
	import { ConflictResolutionDialog } from '$lib/components/sync'
	import type { SyncConflict } from '$lib/types'

	let serverUrl = $state(syncStore.serverUrl || '')
	let isSaving = $state(false)
	let isTesting = $state(false)
	let errorMessage = $state('')
	let successMessage = $state('')
	let connectionStatus = $state<'untested' | 'success' | 'failed'>('untested')

	// Conflict dialog state
	let selectedConflict = $state<SyncConflict | null>(null)
	let showConflictDialog = $state(false)

	// Initialize sync state when workspace is available
	$effect(() => {
		if (authStore.currentWorkspaceId) {
			syncStore.initialize(authStore.currentWorkspaceId)
		}
	})

	// Update local serverUrl when store changes
	$effect(() => {
		if (syncStore.serverUrl) {
			serverUrl = syncStore.serverUrl
		}
	})

	async function handleTestConnection() {
		if (!serverUrl) {
			errorMessage = 'Please enter a server URL'
			return
		}

		isTesting = true
		errorMessage = ''
		successMessage = ''
		connectionStatus = 'untested'

		const result = await syncStore.testConnection(serverUrl)

		if (result.success) {
			connectionStatus = 'success'
			successMessage = result.message
		} else {
			connectionStatus = 'failed'
			errorMessage = result.message
		}

		isTesting = false
	}

	async function handleSaveServer() {
		if (!authStore.currentWorkspaceId) return

		isSaving = true
		errorMessage = ''
		successMessage = ''

		const success = await syncStore.setServerUrl(authStore.currentWorkspaceId, serverUrl)

		if (success) {
			successMessage = 'Sync server URL saved successfully'
		} else {
			errorMessage = syncStore.error || 'Failed to save server URL'
		}

		isSaving = false
	}

	async function handleSync() {
		if (!authStore.currentWorkspaceId) return

		errorMessage = ''
		successMessage = ''

		const result = await syncStore.sync(authStore.currentWorkspaceId)
		if (result?.success) {
			successMessage = 'Sync completed successfully'
		} else {
			errorMessage = syncStore.error || 'Sync failed'
		}
	}

	function formatDate(dateStr: string | null): string {
		if (!dateStr) return 'Never'
		return new Date(dateStr).toLocaleString()
	}

	function getStatusColor(status: string): string {
		switch (status) {
			case 'idle':
				return 'text-green-500'
			case 'syncing':
				return 'text-blue-500'
			case 'error':
				return 'text-red-500'
			case 'offline':
				return 'text-yellow-500'
			default:
				return 'text-muted-foreground'
		}
	}

	function getConnectionStatusIcon(status: 'untested' | 'success' | 'failed'): string {
		switch (status) {
			case 'success':
				return 'text-green-500'
			case 'failed':
				return 'text-red-500'
			default:
				return 'text-muted-foreground'
		}
	}
</script>

<div class="mx-auto max-w-2xl space-y-8">
	<!-- Server Configuration -->
	<section class="border-border rounded-lg border p-6">
		<div class="space-y-6">
			<div>
				<h2 class="text-lg font-semibold">Sync Server</h2>
				<p class="text-muted-foreground text-sm">Configure the server to sync your workspace data</p>
			</div>

			<div class="space-y-4">
				<div class="space-y-2">
					<label for="server-url" class="text-sm font-medium">Server URL</label>
					<input
						id="server-url"
						type="url"
						bind:value={serverUrl}
						placeholder="https://sync.example.com"
						class="border-input bg-background placeholder:text-muted-foreground focus:ring-ring w-full rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-2"
					/>
					<p class="text-muted-foreground text-xs">
						Enter the URL of your sync server. For local testing, use http://localhost:3000
					</p>
				</div>

				<div class="flex gap-3">
					<button
						onclick={handleTestConnection}
						disabled={isTesting || !serverUrl}
						class="bg-secondary text-secondary-foreground hover:bg-secondary/80 rounded-md px-4 py-2 text-sm font-medium disabled:opacity-50"
					>
						{isTesting ? 'Testing...' : 'Test Connection'}
					</button>
					<button
						onclick={handleSaveServer}
						disabled={isSaving}
						class="bg-primary text-primary-foreground hover:bg-primary/90 rounded-md px-4 py-2 text-sm font-medium disabled:opacity-50"
					>
						{isSaving ? 'Saving...' : 'Save'}
					</button>
					{#if connectionStatus === 'success'}
						<span class="flex items-center gap-1 text-sm text-green-500">
							<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
							</svg>
							Connected
						</span>
					{:else if connectionStatus === 'failed'}
						<span class="flex items-center gap-1 text-sm text-red-500">
							<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
							</svg>
							Failed
						</span>
					{/if}
				</div>

				{#if errorMessage}
					<p class="text-sm text-red-500">{errorMessage}</p>
				{/if}

				{#if successMessage}
					<p class="text-sm text-green-500">{successMessage}</p>
				{/if}
			</div>
		</div>
	</section>

	<!-- Sync Status -->
	<section class="border-border rounded-lg border p-6">
		<div class="space-y-6">
			<div>
				<h2 class="text-lg font-semibold">Sync Status</h2>
				<p class="text-muted-foreground text-sm">Current sync state and statistics</p>
			</div>

			<div class="grid grid-cols-2 gap-4">
				<div class="bg-secondary/30 rounded-lg p-4">
					<p class="text-muted-foreground text-xs uppercase tracking-wider">Status</p>
					<p class="text-lg font-medium capitalize {getStatusColor(syncStore.status)}">
						{syncStore.status}
					</p>
				</div>

				<div class="bg-secondary/30 rounded-lg p-4">
					<p class="text-muted-foreground text-xs uppercase tracking-wider">Connection</p>
					<p class="text-lg font-medium {syncStore.isOnline ? 'text-green-500' : 'text-red-500'}">
						{syncStore.isOnline ? 'Online' : 'Offline'}
					</p>
				</div>

				<div class="bg-secondary/30 rounded-lg p-4">
					<p class="text-muted-foreground text-xs uppercase tracking-wider">Pending Changes</p>
					<p class="text-lg font-medium">{syncStore.pendingChanges}</p>
				</div>

				<div class="bg-secondary/30 rounded-lg p-4">
					<p class="text-muted-foreground text-xs uppercase tracking-wider">Conflicts</p>
					<p class="text-lg font-medium {syncStore.conflictCount > 0 ? 'text-yellow-500' : ''}">
						{syncStore.conflictCount}
					</p>
				</div>

				<div class="bg-secondary/30 col-span-2 rounded-lg p-4">
					<p class="text-muted-foreground text-xs uppercase tracking-wider">Last Sync</p>
					<p class="font-medium">{formatDate(syncStore.lastSyncAt)}</p>
				</div>
			</div>

			<div class="flex gap-3">
				<button
					onclick={handleSync}
					disabled={!syncStore.canSync || !syncStore.isConfigured}
					class="bg-secondary text-secondary-foreground hover:bg-secondary/80 rounded-md px-4 py-2 text-sm font-medium disabled:opacity-50"
				>
					{syncStore.status === 'syncing' ? 'Syncing...' : 'Sync Now'}
				</button>

				{#if !syncStore.isConfigured}
					<p class="text-muted-foreground flex items-center text-sm">
						Configure a server URL to enable sync
					</p>
				{/if}
			</div>

			{#if syncStore.error}
				<p class="text-sm text-red-500">{syncStore.error}</p>
			{/if}
		</div>
	</section>

	<!-- Conflicts -->
	{#if syncStore.hasConflicts}
		<section class="rounded-lg border border-yellow-500/50 p-6">
			<div class="space-y-4">
				<div>
					<h2 class="text-lg font-semibold text-yellow-500">Sync Conflicts</h2>
					<p class="text-muted-foreground text-sm">
						{syncStore.conflictCount} conflict(s) require resolution
					</p>
				</div>

				<div class="space-y-2">
					{#each syncStore.conflicts as conflict}
						<div class="bg-secondary/30 flex items-center justify-between rounded-lg p-3">
							<div>
								<p class="text-sm font-medium">{conflict.entity_type}</p>
								<p class="text-muted-foreground font-mono text-xs">
									{conflict.entity_id.slice(0, 8)}...
								</p>
								<p class="text-muted-foreground text-xs">
									Local v{conflict.local_version} vs Remote v{conflict.remote_version}
								</p>
							</div>
							<div class="flex gap-2">
								<button
									onclick={() => {
										selectedConflict = conflict
										showConflictDialog = true
									}}
									class="bg-secondary hover:bg-secondary/80 rounded px-3 py-1.5 text-xs font-medium"
								>
									View Details
								</button>
								<button
									onclick={() => syncStore.resolveConflict(conflict.id, 'local')}
									class="rounded px-3 py-1.5 text-xs font-medium text-blue-500 hover:bg-blue-500/10"
								>
									Keep Local
								</button>
								<button
									onclick={() => syncStore.resolveConflict(conflict.id, 'remote')}
									class="rounded px-3 py-1.5 text-xs font-medium text-green-500 hover:bg-green-500/10"
								>
									Keep Remote
								</button>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</section>
	{/if}
</div>

<!-- Conflict Resolution Dialog -->
{#if selectedConflict}
	<ConflictResolutionDialog
		conflict={selectedConflict}
		open={showConflictDialog}
		onClose={() => {
			showConflictDialog = false
			selectedConflict = null
		}}
		onResolved={() => {
			// Refresh conflicts after resolution
			if (authStore.currentWorkspaceId) {
				syncStore.refreshConflicts(authStore.currentWorkspaceId)
			}
		}}
	/>
{/if}
