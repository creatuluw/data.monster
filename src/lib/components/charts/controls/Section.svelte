<script lang="ts">
	import type { Snippet } from 'svelte';

	/**
	 * Drawer control kit — section card: hairline panel with optional uppercase
	 * micro-header and a right-aligned action snippet (e.g. "+ add").
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
		padding: var(--space-3) var(--space-4) var(--space-4);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		background: var(--color-surface);
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
