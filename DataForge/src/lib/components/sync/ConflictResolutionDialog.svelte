<script lang="ts">
	/**
	 * ConflictResolutionDialog - Modal for resolving sync conflicts
	 *
	 * Displays a side-by-side comparison of local and remote data,
	 * allowing the user to choose which version to keep.
	 *
	 * Design principles (per software quality checklist):
	 * - Clear ownership of conflict resolution logic
	 * - User sees actual data differences before deciding
	 * - Follows ColaNode pattern: server-authoritative with client override option
	 */
	import type { SyncConflict, ConflictResolution } from '$lib/types'
	import { syncStore } from '$lib/stores'

	interface Props {
		/** The conflict to resolve */
		conflict: SyncConflict
		/** Whether the dialog is open */
		open: boolean
		/** Callback when dialog closes */
		onClose: () => void
		/** Callback when conflict is resolved */
		onResolved?: (resolution: ConflictResolution) => void
	}

	let { conflict, open, onClose, onResolved }: Props = $props()

	let isResolving = $state(false)
	let error = $state<string | null>(null)

	// Parse JSON data for display
	const localData = $derived(() => {
		try {
			return JSON.parse(conflict.local_data)
		} catch {
			return { raw: conflict.local_data }
		}
	})

	const remoteData = $derived(() => {
		try {
			return JSON.parse(conflict.remote_data)
		} catch {
			return { raw: conflict.remote_data }
		}
	})

	// Find changed fields between local and remote
	const changedFields = $derived(() => {
		const local = localData()
		const remote = remoteData()
		const allKeys = new Set([...Object.keys(local), ...Object.keys(remote)])
		const changes: Array<{ key: string; localValue: unknown; remoteValue: unknown; changed: boolean }> = []

		for (const key of allKeys) {
			const localVal = local[key]
			const remoteVal = remote[key]
			const changed = JSON.stringify(localVal) !== JSON.stringify(remoteVal)
			changes.push({ key, localValue: localVal, remoteValue: remoteVal, changed })
		}

		return changes
	})

	async function handleResolve(resolution: ConflictResolution) {
		isResolving = true
		error = null

		try {
			const success = await syncStore.resolveConflict(conflict.id, resolution)
			if (success) {
				onResolved?.(resolution)
				onClose()
			} else {
				error = syncStore.error || 'Failed to resolve conflict'
			}
		} catch (e) {
			error = e instanceof Error ? e.message : String(e)
		} finally {
			isResolving = false
		}
	}

	function formatValue(value: unknown): string {
		if (value === null) return 'null'
		if (value === undefined) return 'undefined'
		if (typeof value === 'object') return JSON.stringify(value, null, 2)
		return String(value)
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleString()
	}
</script>

{#if open}
	<!-- Modal backdrop -->
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
		onclick={(e) => {
			if (e.target === e.currentTarget) onClose()
		}}
		onkeydown={(e) => {
			if (e.key === 'Escape') onClose()
		}}
		role="dialog"
		aria-modal="true"
		aria-labelledby="conflict-dialog-title"
		tabindex="-1"
	>
		<!-- Modal content -->
		<div class="bg-background max-h-[90vh] w-full max-w-4xl overflow-hidden rounded-lg shadow-xl">
			<!-- Header -->
			<div class="border-border flex items-center justify-between border-b px-6 py-4">
				<div>
					<h2 id="conflict-dialog-title" class="text-lg font-semibold">Resolve Sync Conflict</h2>
					<p class="text-muted-foreground text-sm">
						{conflict.entity_type} - {conflict.entity_id.slice(0, 8)}...
					</p>
				</div>
				<button
					onclick={onClose}
					class="text-muted-foreground hover:text-foreground"
					aria-label="Close dialog"
				>
					<svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			</div>

			<!-- Version info -->
			<div class="border-border grid grid-cols-2 gap-4 border-b px-6 py-3">
				<div class="flex items-center gap-2">
					<span class="bg-blue-500/10 flex h-6 w-6 items-center justify-center rounded text-blue-500">
						<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
						</svg>
					</span>
					<div>
						<p class="text-sm font-medium">Local Version</p>
						<p class="text-muted-foreground text-xs">v{conflict.local_version}</p>
					</div>
				</div>
				<div class="flex items-center gap-2">
					<span class="bg-green-500/10 flex h-6 w-6 items-center justify-center rounded text-green-500">
						<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" />
						</svg>
					</span>
					<div>
						<p class="text-sm font-medium">Remote Version</p>
						<p class="text-muted-foreground text-xs">v{conflict.remote_version}</p>
					</div>
				</div>
			</div>

			<!-- Data comparison -->
			<div class="max-h-96 overflow-auto px-6 py-4">
				<table class="w-full text-sm">
					<thead>
						<tr class="border-border border-b">
							<th class="text-muted-foreground pb-2 text-left font-medium">Field</th>
							<th class="pb-2 text-left font-medium text-blue-500">Local</th>
							<th class="pb-2 text-left font-medium text-green-500">Remote</th>
						</tr>
					</thead>
					<tbody>
						{#each changedFields() as field}
							<tr class="border-border border-b {field.changed ? 'bg-yellow-500/5' : ''}">
								<td class="py-2 pr-4 font-mono text-xs">
									{field.key}
									{#if field.changed}
										<span class="ml-1 text-yellow-500">*</span>
									{/if}
								</td>
								<td class="py-2 pr-4">
									<pre class="max-w-xs truncate rounded bg-blue-500/5 px-2 py-1 font-mono text-xs {field.changed ? 'ring-1 ring-blue-500/30' : ''}">{formatValue(field.localValue)}</pre>
								</td>
								<td class="py-2">
									<pre class="max-w-xs truncate rounded bg-green-500/5 px-2 py-1 font-mono text-xs {field.changed ? 'ring-1 ring-green-500/30' : ''}">{formatValue(field.remoteValue)}</pre>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>

				{#if changedFields().filter(f => f.changed).length === 0}
					<p class="text-muted-foreground py-4 text-center text-sm">
						No visible differences in data fields. The conflict may be due to version mismatch.
					</p>
				{/if}
			</div>

			<!-- Error message -->
			{#if error}
				<div class="px-6">
					<p class="text-sm text-red-500">{error}</p>
				</div>
			{/if}

			<!-- Actions -->
			<div class="border-border flex items-center justify-between border-t px-6 py-4">
				<p class="text-muted-foreground text-xs">
					Created: {formatDate(conflict.created_at)}
				</p>
				<div class="flex gap-3">
					<button
						onclick={() => handleResolve('local')}
						disabled={isResolving}
						class="flex items-center gap-2 rounded-md bg-blue-500 px-4 py-2 text-sm font-medium text-white hover:bg-blue-600 disabled:opacity-50"
					>
						{#if isResolving}
							<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
							</svg>
						{/if}
						Keep Local
					</button>
					<button
						onclick={() => handleResolve('remote')}
						disabled={isResolving}
						class="flex items-center gap-2 rounded-md bg-green-500 px-4 py-2 text-sm font-medium text-white hover:bg-green-600 disabled:opacity-50"
					>
						{#if isResolving}
							<svg class="h-4 w-4 animate-spin" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
							</svg>
						{/if}
						Keep Remote
					</button>
					<button
						onclick={onClose}
						disabled={isResolving}
						class="bg-secondary text-secondary-foreground hover:bg-secondary/80 rounded-md px-4 py-2 text-sm font-medium disabled:opacity-50"
					>
						Cancel
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}
