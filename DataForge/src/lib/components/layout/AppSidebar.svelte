<script lang="ts">
	import { page } from '$app/stores'
	import { authStore } from '$lib/stores'
	import { goto } from '$app/navigation'
	import SyncStatusIndicator from './SyncStatusIndicator.svelte'

	const { onLogout }: { onLogout?: () => void } = $props()

	function isActive(href: string): boolean {
		const path = $page.url.pathname
		if (href === '/') {
			return path === '/'
		}
		return path.startsWith(href)
	}

	function getInitials(name: string): string {
		return name
			.split(' ')
			.map((n) => n.charAt(0))
			.join('')
			.toUpperCase()
			.slice(0, 2)
	}

	async function handleLogout() {
		if (onLogout) {
			onLogout()
		}
	}

	function switchWorkspace() {
		goto('/workspace/select')
	}
</script>

<aside class="border-border flex w-16 flex-col border-r lg:w-64">
	<!-- Workspace Switcher -->
	<div class="border-border border-b p-3">
		{#if authStore.currentWorkspace}
			<button
				onclick={switchWorkspace}
				class="hover:bg-secondary flex w-full items-center gap-3 rounded-md p-2 transition-colors"
				title={authStore.currentWorkspace.name}
			>
				<span
					class="bg-primary text-primary-foreground flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-md text-sm font-medium"
				>
					{authStore.currentWorkspace.name.charAt(0).toUpperCase()}
				</span>
				<span class="hidden truncate text-sm font-medium lg:block">
					{authStore.currentWorkspace.name}
				</span>
				<svg
					class="text-muted-foreground ml-auto hidden h-4 w-4 lg:block"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M8 9l4-4 4 4m0 6l-4 4-4-4"
					/>
				</svg>
			</button>
		{/if}
	</div>

	<!-- Main Navigation -->
	<nav class="flex-1 space-y-1 p-3">
		<!-- Home -->
		<a
			href="/"
			class="flex items-center gap-3 rounded-md p-2 text-sm transition-colors {isActive('/')
				? 'bg-secondary text-foreground'
				: 'text-muted-foreground hover:bg-secondary/50 hover:text-foreground'}"
			title="Home"
		>
			<svg class="h-5 w-5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"
				/>
			</svg>
		<span class="hidden lg:block">Home</span>
	</a>

	<!-- Inspector (DBeaver-style SQLite browser) -->
		<a
			href="/inspector"
			class="flex items-center gap-3 rounded-md p-2 text-sm transition-colors {isActive(
				'/inspector'
			)
				? 'bg-secondary text-foreground'
				: 'text-muted-foreground hover:bg-secondary/50 hover:text-foreground'}"
			title="Inspector"
		>
			<svg class="h-5 w-5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4"
				/>
			</svg>
			<span class="hidden lg:block">Inspector</span>
		</a>

		<!-- Wells -->
		<a
			href="/wells"
			class="flex items-center gap-3 rounded-md p-2 text-sm transition-colors {isActive('/wells')
				? 'bg-secondary text-foreground'
				: 'text-muted-foreground hover:bg-secondary/50 hover:text-foreground'}"
			title="Wells"
		>
			<!-- Wellhead/derrick icon -->
			<svg class="h-5 w-5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<!-- Horizontal beam at top -->
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 6h12" />
				<!-- Left support arm -->
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 6v4" />
				<!-- Right support arm -->
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 6v4" />
				<!-- Central shaft going down -->
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v14" />
				<!-- Circle at bottom (wellbore) -->
				<circle cx="12" cy="20" r="1.5" fill="currentColor" />
			</svg>
			<span class="hidden lg:block">Wells</span>
		</a>

		<!-- Curve Viewer -->
		<a
			href="/curves"
			class="flex items-center gap-3 rounded-md p-2 text-sm transition-colors {isActive('/curves')
				? 'bg-secondary text-foreground'
				: 'text-muted-foreground hover:bg-secondary/50 hover:text-foreground'}"
			title="Curve Viewer"
		>
			<!-- Vertical wavy log track icon -->
			<svg class="h-5 w-5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<!-- Vertical wavy line representing a log curve -->
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M12 3c-2 2-2 4 0 6s2 4 0 6-2 4 0 6"
				/>
			</svg>
			<span class="hidden lg:block">Curves</span>
		</a>

		<!-- Data Ingestion -->
		<a
			href="/ingest"
			class="flex items-center gap-3 rounded-md p-2 text-sm transition-colors {isActive('/ingest')
				? 'bg-secondary text-foreground'
				: 'text-muted-foreground hover:bg-secondary/50 hover:text-foreground'}"
			title="Data Ingestion"
		>
			<svg class="h-5 w-5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
				/>
			</svg>
			<span class="hidden lg:block">Data Ingestion</span>
		</a>

		<!-- Divider -->
		<div class="border-border my-4 border-t"></div>

		<!-- Settings -->
		<a
			href="/settings"
			class="flex items-center gap-3 rounded-md p-2 text-sm transition-colors {isActive(
				'/settings'
			)
				? 'bg-secondary text-foreground'
				: 'text-muted-foreground hover:bg-secondary/50 hover:text-foreground'}"
			title="Settings"
		>
			<svg class="h-5 w-5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
				/>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
				/>
			</svg>
			<span class="hidden lg:block">Settings</span>
		</a>
	</nav>

	<!-- Sync Status -->
	<div class="border-border border-t p-3">
		<SyncStatusIndicator expanded={true} />
	</div>

	<!-- User Profile -->
	<div class="border-border border-t p-3">
		{#if authStore.account}
			<div class="flex items-center gap-3">
				<div
					class="bg-secondary text-foreground flex h-8 w-8 flex-shrink-0 items-center justify-center rounded-full text-xs font-medium"
				>
					{getInitials(authStore.account.name)}
				</div>
				<div class="hidden min-w-0 flex-1 lg:block">
					<p class="truncate text-sm font-medium">{authStore.account.name}</p>
					<p class="text-muted-foreground truncate text-xs">{authStore.account.email}</p>
				</div>
				<button
					onclick={handleLogout}
					class="text-muted-foreground hover:text-foreground hidden p-1 lg:block"
					title="Sign out"
				>
					<svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"
						/>
					</svg>
				</button>
			</div>
		{/if}
	</div>
</aside>
