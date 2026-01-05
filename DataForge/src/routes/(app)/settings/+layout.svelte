<script lang="ts">
	import { SettingsSidebar } from '$lib/components/settings'
	import { page } from '$app/stores'

	let { children } = $props()

	const sections = [
		{
			title: 'Workspace settings',
			items: [
				{ label: 'General', href: '/settings/general', icon: 'settings' as const },
				{ label: 'Users', href: '/settings/users', icon: 'users' as const },
				{ label: 'Sync', href: '/settings/sync', icon: 'sync' as const }
			]
		},
		{
			title: 'Account settings',
			items: [{ label: 'Profile', href: '/settings/profile', icon: 'user' as const }]
		},
		{
			title: 'App settings',
			items: [{ label: 'Appearance', href: '/settings/appearance', icon: 'palette' as const }]
		}
	]

	// Get current page title
	const pageTitle = $derived(() => {
		const path = $page.url.pathname
		if (path.includes('/users')) return 'Workspace Users'
		if (path.includes('/general')) return 'General Settings'
		if (path.includes('/sync')) return 'Sync Settings'
		if (path.includes('/profile')) return 'Profile Settings'
		if (path.includes('/appearance')) return 'Appearance'
		return 'Settings'
	})
</script>

<div class="flex h-full flex-col">
	<!-- Page Header -->
	<header class="border-border border-b px-6 py-4">
		<h1 class="text-2xl font-bold">{pageTitle()}</h1>
		<p class="text-muted-foreground text-sm">Manage workspace and app settings</p>
	</header>

	<!-- Settings content with sub-sidebar -->
	<div class="flex flex-1 overflow-hidden">
		<SettingsSidebar {sections} />

		<div class="flex-1 overflow-auto p-6">
			{@render children()}
		</div>
	</div>
</div>
