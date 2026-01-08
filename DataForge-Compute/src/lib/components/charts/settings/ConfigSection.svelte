<script lang="ts">
	/**
	 * ConfigSection - Wrapper for configuration sections
	 *
	 * Provides consistent styling for titled sections within the settings dialog.
	 */
	import type { Snippet } from 'svelte'

	interface Props {
		/** Section title */
		title: string
		/** Optional description text */
		description?: string
		/** Content slot */
		children: Snippet
		/** Optional collapsible state */
		collapsible?: boolean
		/** Initial collapsed state */
		collapsed?: boolean
	}

	let {
		title,
		description,
		children,
		collapsible = false,
		collapsed = false
	}: Props = $props()

	let isCollapsed = $state(collapsed)
</script>

<section class="config-section">
	<header class="section-header" class:collapsible onclick={() => collapsible && (isCollapsed = !isCollapsed)}>
		<h3 class="section-title">{title}</h3>
		{#if collapsible}
			<svg
				class="collapse-icon"
				class:collapsed={isCollapsed}
				viewBox="0 0 24 24"
				width="16"
				height="16"
			>
				<path fill="currentColor" d="M7 10l5 5 5-5z" />
			</svg>
		{/if}
	</header>

	{#if description && !isCollapsed}
		<p class="section-description">{description}</p>
	{/if}

	{#if !isCollapsed}
		<div class="section-content">
			{@render children()}
		</div>
	{/if}
</section>

<style>
	.config-section {
		display: flex;
		flex-direction: column;
		gap: 12px;
		padding: 16px;
		background: hsl(var(--card));
		border: 1px solid hsl(var(--border));
		border-radius: 8px;
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 8px;
	}

	.section-header.collapsible {
		cursor: pointer;
		user-select: none;
	}

	.section-header.collapsible:hover {
		opacity: 0.8;
	}

	.section-title {
		margin: 0;
		font-size: 14px;
		font-weight: 600;
		color: hsl(var(--foreground));
	}

	.section-description {
		margin: 0;
		font-size: 12px;
		color: hsl(var(--muted-foreground));
		line-height: 1.5;
	}

	.section-content {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.collapse-icon {
		color: hsl(var(--muted-foreground));
		transition: transform 0.2s;
	}

	.collapse-icon.collapsed {
		transform: rotate(-90deg);
	}
</style>
