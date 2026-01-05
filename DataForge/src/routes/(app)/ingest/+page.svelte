<script lang="ts">
	import { goto } from '$app/navigation'
	import { ingestStore } from '$lib/stores/ingest.svelte'

	// Reset wizard state and redirect to source selection
	$effect(() => {
		ingestStore.reset()
		goto('/ingest/source', { replaceState: true })
	})
</script>

<!-- Redirect to source selection -->
<div class="flex items-center justify-center p-8">
	<p class="text-muted-foreground text-sm">Loading...</p>
</div>
