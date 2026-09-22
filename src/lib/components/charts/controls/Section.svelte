<script lang="ts">
	import type { Snippet } from 'svelte';

	/**
	 * Drawer control kit — flat section group (the /data drawer pattern): no
	 * card chrome, consecutive sections separated by a dashed hairline,
	 * optional uppercase micro-header + right-aligned action snippet.
	 */
	let {
		title,
		action,
		children
	}: {
		title?: string;
		action?: Snippet;
		children: Snippet;
	} = $props();
</script>

<section class="section">
	{#if title || action}
		<header class="section-head">
			{#if title}<span class="section-title">{title}</span>{/if}
			{@render action?.()}
		</header>
	{/if}
	{@render children()}
</section>

<style>
	.section {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	/* dashed hairline between consecutive sections — /data drawer-divider rhythm */
	.section + .section {
		border-top: 1px dashed var(--color-border);
		padding-top: var(--space-4);
	}

	.section-head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-2);
	}

	.section-title {
		font-size: 9px;
		font-weight: 700;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}
</style>
