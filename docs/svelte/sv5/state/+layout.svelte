<script lang="ts">
	import '../app.css';
	import { invalidate } from '$app/navigation';
	import { onMount } from 'svelte';
	import { auth, updateSession } from '$lib/stores/auth.svelte';
	import type { User } from '@supabase/supabase-js';
	import { appState } from '$lib/stores/app-state';
	import { initSessionManager } from '$lib/utils/session-manager';
	import { setTreeState } from '$lib/state/gridspace/tree-state.svelte.js';
	import { setChartDataStore } from '$lib/state/data/chart-data-store';
	import { setGridspaceQuickFilter } from '$lib/state/gridspace-manager/filters/gridspace-manager-quick-filter.svelte.js';
	import { setGridspaceState } from '$lib/state/gridspace/gridspace.svelte.ts';
	import { setOperatorsManager } from '$lib/state/gridspace/operators/operators.svelte.js';
	import { setChartsManager } from '$lib/classes/scichart/charts-manager/charts-manager.svelte';
	import { setGridspacesManager } from '$lib/state/gridspace-manager/gridspaces/gridspaces.svelte';
	import { setViewsManager } from '$lib/state/gridspace/views/views.svelte';
	import { setFlow } from '$lib/state/gridspace/flow/gridspace-flow.svelte.ts';

	//******************* STATE INITIAILIZATION ********************/
	// Initialize base states
	setTreeState();

	// Initialize chart-related states in correct order
	setChartDataStore(); // Initialize data store first
	setGridspaceQuickFilter();
	setGridspaceState();
	setOperatorsManager();
	setChartsManager(); // Initialize chart manager after data store
	setGridspacesManager();
	// Initialize ViewsManager
	setViewsManager();
	// Initialize setFlow
	setFlow();

	//******************* VARIABLES ********************/

	let { data, children } = $props();
	let supabase = $derived(data.supabase);
	let currentSession = $derived(data.session);
	let verifiedUser = $state<User | null>(null);

	//******************* FUNCTIONS ********************/

	$effect(() => {
		updateSession(currentSession);
	});

	onMount(() => {
		// WARNING: Do not trust the user/session from onAuthStateChange directly!
		// Always use supabase.auth.getUser() to validate the JWT and get the real user if you need to trust it on the client.
		const { data } = supabase.auth.onAuthStateChange((event, newSession) => {
			if (newSession?.expires_at !== auth.session?.expires_at) {
				invalidate('supabase:auth');
			}
		});

		// Initialize session manager with cleanup
		const cleanupSession = initSessionManager();

		// Handle window focus/blur
		const handleFocus = () => appState.set('active');
		const handleBlur = () => appState.set('inactive');

		window.addEventListener('focus', handleFocus);
		window.addEventListener('blur-sm', handleBlur);

		// Enhanced cleanup
		return () => {
			data.subscription.unsubscribe();
			window.removeEventListener('focus', handleFocus);
			window.removeEventListener('blur-sm', handleBlur);
			cleanupSession(); // Ensure refresh is stopped on unmount
		};
	});
</script>

<svelte:head>
	<title>SV5_SB</title>
</svelte:head>

<div class="flex min-h-screen flex-col">
	<main class="flex-1">
		<div>
			{#if import.meta.env.DEV}
				<pre class="debug">
					Auth Debug:
					Verified User: {JSON.stringify(verifiedUser?.email ?? 'none', null, 2)}
					Current Session: {JSON.stringify(currentSession?.user?.email ?? 'none', null, 2)}
				</pre>
			{/if}
			{@render children?.()}
		</div>
	</main>
</div>

<style>
	.debug {
		position: fixed;
		bottom: 0;
		right: 0;
		background: #333;
		color: #fff;
		padding: 10px;
		font-size: 12px;
		max-width: 400px;
		z-index: 1000;
	}
</style>
