<script lang="ts">
	import type { Snippet } from 'svelte';

	/**
	 * Drawer section — flat (no box): dashed hairline separators between
	 * sections, uppercase micro-header, optional right-aligned action.
	 * Motif shared with the skeleton cards and the role-picker modal.
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
		padding-top: var(--space-4);
		border-top: 1px dashed var(--color-border-strong);
	}

	.section:first-child {
		border-top: none;
		padding-top: 0;
	}

	.section-head {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-2);
	}

	.section-title {
		font-size: 10px;
		font-weight: 600;
		letter-spacing: 0.08em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}
</style>
