<script lang="ts">
	import { goto } from '$app/navigation'
	import { onMount } from 'svelte'

	// Redirect to general settings page by default
	onMount(() => {
		goto('/settings/general', { replaceState: true })
	})
</script>

<div class="text-muted-foreground py-8 text-center">Redirecting...</div>
