<script lang="ts">
	let {
		variant = "default",
		children,
	}: {
		variant?: "default" | "accent" | "success" | "warning" | "danger";
		children?: import("svelte").Snippet;
	} = $props();
</script>

<span class="tag tag-{variant}">
	{#if children}{@render children()}{/if}
</span>

<style>
	.tag {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		padding: 2px var(--space-2);
		border: 1px solid;
		font-family: var(--font-body);
		font-size: 9px;
		font-weight: 600;
		line-height: 1.5;
		border-radius: var(--radius-xs);
	}

	.tag-default {
		background: var(--color-surface-sunken);
		color: var(--color-text-secondary);
		border-color: var(--color-border);
	}

	.tag-accent {
		background: var(--color-accent-muted);
		color: var(--color-accent-dark);
		border-color: oklch(0.82 0.03 41);
	}

	.tag-success {
		background: oklch(0.95 0.03 160);
		color: oklch(0.4 0.1 160);
		border-color: oklch(0.88 0.04 160);
	}

	.tag-warning {
		background: oklch(0.96 0.03 80);
		color: oklch(0.42 0.1 80);
		border-color: oklch(0.9 0.03 80);
	}

	.tag-danger {
		background: oklch(0.95 0.03 22);
		color: oklch(0.38 0.12 22);
		border-color: oklch(0.9 0.04 22);
	}
</style>
