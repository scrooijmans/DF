<script lang="ts">
	import { InviteUsers, UsersList } from '$lib/components/settings'
	import { membersStore } from '$lib/stores'
</script>

<div class="mx-auto max-w-3xl space-y-8">
	<!-- Invite Section -->
	{#if membersStore.canManageMembers}
		<section class="border-border rounded-lg border p-6">
			<InviteUsers />
		</section>
	{/if}

	<!-- Users List Section -->
	<section class="border-border rounded-lg border p-6">
		<UsersList />
	</section>
</div>
