<script lang="ts">
	let {
		count = 0,
		variant = "default",
	}: {
		count?: number;
		variant?: "default" | "accent" | "danger";
	} = $props();
</script>

{#if count > 0}
	<span class="badge badge-{variant}">{count > 99 ? '99+' : count}</span>
{/if}

<style>
	.badge {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 18px;
		height: 18px;
		padding: 0 5px;
		border-radius: var(--radius-full);
		font-family: var(--font-body);
		font-size: 10px;
		font-weight: 700;
		line-height: 1;
		letter-spacing: -0.02em;
	}

	.badge-default {
		background: var(--color-surface-sunken);
		color: var(--color-text-secondary);
		border: 1px solid var(--color-border);
	}

	.badge-accent {
		background: var(--color-accent);
		color: var(--color-text-on-accent);
	}

	.badge-danger {
		background: oklch(0.55 0.2 22);
		color: white;
	}
</style>