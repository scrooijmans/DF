<script lang="ts">
	import { authStore } from '$lib/stores'

	let name = $state(authStore.account?.name || '')
	let isSaving = $state(false)
	let saveError = $state<string | null>(null)
	let saveSuccess = $state(false)

	// Track if form has changes
	const hasChanges = $derived(name !== (authStore.account?.name || ''))

	async function handleSave() {
		if (!hasChanges) return

		isSaving = true
		saveError = null
		saveSuccess = false

		try {
			const updated = await authStore.updateAccount(name)
			if (updated) {
				saveSuccess = true
				// Reset success message after 3 seconds
				setTimeout(() => {
					saveSuccess = false
				}, 3000)
			} else {
				saveError = authStore.error || 'Failed to save changes'
			}
		} catch (e) {
			saveError = e instanceof Error ? e.message : String(e)
		} finally {
			isSaving = false
		}
	}

	function getInitials(name: string): string {
		return name
			.split(' ')
			.map((n) => n.charAt(0))
			.join('')
			.toUpperCase()
			.slice(0, 2)
	}
</script>

<div class="mx-auto max-w-2xl space-y-8">
	<!-- Profile Section -->
	<section class="border-border rounded-lg border p-6">
		<div class="space-y-6">
			<div>
				<h2 class="text-lg font-semibold">Profile</h2>
				<p class="text-muted-foreground text-sm">Manage your account information</p>
			</div>

			<!-- Avatar Preview -->
			<div class="flex items-center gap-4">
				<div
					class="bg-primary text-primary-foreground flex h-16 w-16 items-center justify-center rounded-full text-xl font-medium"
				>
					{getInitials(name || authStore.account?.name || 'U')}
				</div>
				<div>
					<p class="font-medium">{name || authStore.account?.name || 'User'}</p>
					<p class="text-muted-foreground text-sm">{authStore.account?.email || ''}</p>
				</div>
			</div>

			<div class="space-y-4">
				<!-- Name Field -->
				<div class="space-y-2">
					<label for="account-name" class="text-sm font-medium">Display Name</label>
					<input
						id="account-name"
						type="text"
						bind:value={name}
						placeholder="Your display name"
						class="border-input bg-background placeholder:text-muted-foreground focus:ring-ring w-full rounded-md border px-3 py-2 text-sm focus:outline-none focus:ring-2"
					/>
					<p class="text-muted-foreground text-xs">
						This is your display name visible to other workspace members.
					</p>
				</div>

				<!-- Email Field (Read-only) -->
				<div class="space-y-2">
					<label for="account-email" class="text-sm font-medium">Email</label>
					<input
						id="account-email"
						type="email"
						value={authStore.account?.email || ''}
						disabled
						class="border-input bg-background/50 w-full cursor-not-allowed rounded-md border px-3 py-2 text-sm opacity-60"
					/>
					<p class="text-muted-foreground text-xs">
						Email address cannot be changed at this time.
					</p>
				</div>

				{#if saveError}
					<p class="text-sm text-red-500">{saveError}</p>
				{/if}

				{#if saveSuccess}
					<p class="text-sm text-green-500">Profile updated successfully!</p>
				{/if}

				<div class="flex justify-end">
					<button
						onclick={handleSave}
						disabled={isSaving || !hasChanges}
						class="bg-primary text-primary-foreground hover:bg-primary/90 rounded-md px-4 py-2 text-sm font-medium disabled:cursor-not-allowed disabled:opacity-50"
					>
						{isSaving ? 'Saving...' : 'Save Changes'}
					</button>
				</div>
			</div>
		</div>
	</section>

	<!-- Account Info Section -->
	<section class="border-border rounded-lg border p-6">
		<div class="space-y-4">
			<div>
				<h2 class="text-lg font-semibold">Account Information</h2>
				<p class="text-muted-foreground text-sm">Details about your account</p>
			</div>

			<div class="space-y-3">
				<div class="flex items-center justify-between py-2">
					<span class="text-muted-foreground text-sm">Account ID</span>
					<span class="font-mono text-xs">{authStore.account?.id || '-'}</span>
				</div>
				<div class="border-border border-t"></div>
				<div class="flex items-center justify-between py-2">
					<span class="text-muted-foreground text-sm">Status</span>
					<span
						class="rounded-full px-2 py-0.5 text-xs font-medium {authStore.account?.status ===
						'verified'
							? 'bg-green-500/10 text-green-500'
							: authStore.account?.status === 'suspended'
								? 'bg-red-500/10 text-red-500'
								: 'bg-yellow-500/10 text-yellow-500'}"
					>
						{authStore.account?.status || 'unknown'}
					</span>
				</div>
			</div>
		</div>
	</section>
</div>
