<script lang="ts">
	import type { Workspace } from '$lib/types/auth'

	type Props = {
		workspace: Workspace
		isSelected?: boolean
		loading?: boolean
		onSelect?: (id: string) => void
	}

	let { workspace, isSelected = false, loading = false, onSelect }: Props = $props()

	function handleClick() {
		if (loading) return
		onSelect?.(workspace.id)
	}
</script>

<button
	type="button"
	onclick={handleClick}
	disabled={loading}
	class="hover:border-primary hover:bg-accent/60 flex h-full w-full flex-col rounded-xl border border-border bg-card p-4 text-left shadow-sm transition-colors disabled:opacity-50 {isSelected
		? 'border-primary bg-accent/70'
		: ''}"
>
	<div class="flex items-center gap-3">
		<div class="bg-secondary text-secondary-foreground flex h-10 w-10 items-center justify-center rounded-md text-sm font-medium">
			{workspace.name.charAt(0).toUpperCase()}
		</div>
		<div class="flex-1 truncate">
			<p class="font-medium">{workspace.name}</p>
			{#if workspace.description}
				<p class="text-muted-foreground truncate text-sm">{workspace.description}</p>
			{/if}
		</div>
		{#if isSelected && loading}
			<svg class="h-5 w-5 animate-spin text-muted-foreground" viewBox="0 0 24 24" fill="none">
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
		{/if}
	</div>
</button>


