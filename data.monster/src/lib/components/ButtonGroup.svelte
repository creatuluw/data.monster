<script lang="ts">
	import type { Snippet } from "svelte";

	interface ButtonGroupItem {
		value: string;
		label?: string;
		icon?: Snippet;
		ariaLabel?: string;
	}

	let {
		items = [],
		value = $bindable(""),
		fluid = false,
	}: {
		items: ButtonGroupItem[];
		value?: string;
		fluid?: boolean;
	} = $props();
</script>

<div class="btn-group" class:btn-group-fluid={fluid} role="group">
	{#each items as item}
		<button
			class="btn-group-item"
			class:active={value === item.value}
			onclick={() => (value = item.value)}
			aria-label={item.ariaLabel ?? item.label ?? item.value}
			aria-pressed={value === item.value}
		>
			{#if item.icon}{@render item.icon()}{/if}
			{#if item.label}{item.label}{/if}
		</button>
	{/each}
</div>

<style>
	.btn-group {
		display: flex;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		overflow: hidden;
	}

	.btn-group-item {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-1);
		height: 32px;
		padding: 0 var(--space-2);
		border: none;
		background: transparent;
		color: var(--color-text-tertiary);
		cursor: pointer;
		font-family: var(--font-body);
		font-size: var(--text-xs);
		font-weight: 500;
		transition:
			background var(--duration-fast) ease,
			color var(--duration-fast) ease;
	}

	.btn-group-item:hover {
		background: var(--color-surface-sunken);
		color: var(--color-text);
	}

	.btn-group-item.active {
		background: var(--color-accent-muted);
		color: var(--color-accent);
	}

	.btn-group-item + .btn-group-item {
		border-left: 1px solid var(--color-border);
	}

	.btn-group-item.active + .btn-group-item,
	.btn-group-item:has(+ .btn-group-item.active) {
		border-left-color: transparent;
	}

	.btn-group-fluid .btn-group-item {
		flex: 1;
	}
</style>
