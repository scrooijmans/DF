<script lang="ts">
	import { themeStore, type ThemeMode } from '$lib/stores/theme.svelte'

	const options: { mode: ThemeMode; label: string; description: string; icon: string }[] = [
		{
			mode: 'light',
			label: 'Light',
			description: 'Use light theme regardless of system settings',
			icon: 'sun'
		},
		{
			mode: 'dark',
			label: 'Dark',
			description: 'Use dark theme regardless of system settings',
			icon: 'moon'
		},
		{
			mode: 'system',
			label: 'System',
			description: 'Automatically match your operating system theme',
			icon: 'monitor'
		}
	]

	function handleSelect(mode: ThemeMode) {
		themeStore.setMode(mode)
	}
</script>

<div class="mx-auto max-w-2xl space-y-8">
	<!-- Theme Section -->
	<section class="border-border rounded-lg border p-6">
		<div class="space-y-6">
			<div>
				<h2 class="text-lg font-semibold">Theme</h2>
				<p class="text-muted-foreground text-sm">
					Choose how DataForge looks to you. Select a theme or sync with your system settings.
				</p>
			</div>

			<div class="space-y-3">
				{#each options as option}
					{@const isSelected = themeStore.mode === option.mode}
					<button
						onclick={() => handleSelect(option.mode)}
						class="flex w-full items-center gap-4 rounded-lg border p-4 text-left transition-colors
							{isSelected
							? 'border-primary bg-primary/5'
							: 'border-border hover:bg-secondary/50'}"
					>
						<!-- Icon -->
						<div
							class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg {isSelected
								? 'bg-primary/10 text-primary'
								: 'bg-secondary text-muted-foreground'}"
						>
							{#if option.icon === 'sun'}
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="20"
									height="20"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<circle cx="12" cy="12" r="4" />
									<path d="M12 2v2" />
									<path d="M12 20v2" />
									<path d="m4.93 4.93 1.41 1.41" />
									<path d="m17.66 17.66 1.41 1.41" />
									<path d="M2 12h2" />
									<path d="M20 12h2" />
									<path d="m6.34 17.66-1.41 1.41" />
									<path d="m19.07 4.93-1.41 1.41" />
								</svg>
							{:else if option.icon === 'moon'}
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="20"
									height="20"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />
								</svg>
							{:else}
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="20"
									height="20"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
								>
									<rect width="20" height="14" x="2" y="3" rx="2" />
									<line x1="8" x2="16" y1="21" y2="21" />
									<line x1="12" x2="12" y1="17" y2="21" />
								</svg>
							{/if}
						</div>

						<!-- Content -->
						<div class="flex-1">
							<p class="font-medium">{option.label}</p>
							<p class="text-muted-foreground text-sm">{option.description}</p>
						</div>

						<!-- Radio indicator -->
						<div
							class="flex h-5 w-5 shrink-0 items-center justify-center rounded-full border-2 {isSelected
								? 'border-primary'
								: 'border-muted-foreground/30'}"
						>
							{#if isSelected}
								<div class="bg-primary h-2.5 w-2.5 rounded-full"></div>
							{/if}
						</div>
					</button>
				{/each}
			</div>
		</div>
	</section>

	<!-- Current Theme Info -->
	<section class="border-border rounded-lg border p-6">
		<div class="space-y-4">
			<div>
				<h2 class="text-lg font-semibold">Current Theme</h2>
				<p class="text-muted-foreground text-sm">The active theme being applied</p>
			</div>

			<div class="flex items-center gap-3">
				<div
					class="flex h-10 w-10 items-center justify-center rounded-lg {themeStore.isDark
						? 'bg-slate-800 text-slate-200'
						: 'bg-amber-100 text-amber-600'}"
				>
					{#if themeStore.isDark}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="20"
							height="20"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />
						</svg>
					{:else}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							width="20"
							height="20"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							stroke-linecap="round"
							stroke-linejoin="round"
						>
							<circle cx="12" cy="12" r="4" />
							<path d="M12 2v2" />
							<path d="M12 20v2" />
							<path d="m4.93 4.93 1.41 1.41" />
							<path d="m17.66 17.66 1.41 1.41" />
							<path d="M2 12h2" />
							<path d="M20 12h2" />
							<path d="m6.34 17.66-1.41 1.41" />
							<path d="m19.07 4.93-1.41 1.41" />
						</svg>
					{/if}
				</div>
				<div>
					<p class="font-medium capitalize">{themeStore.resolvedTheme}</p>
					<p class="text-muted-foreground text-sm">
						{#if themeStore.mode === 'system'}
							Following system preference
						{:else}
							Manually set to {themeStore.mode}
						{/if}
					</p>
				</div>
			</div>
		</div>
	</section>
</div>
