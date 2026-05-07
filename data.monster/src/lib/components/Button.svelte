<script lang="ts">
	let {
		variant = "primary",
		size,
		type,
		disabled = false,
		href,
		onclick,
		class: className,
		children,
	}: {
		variant?: "primary" | "secondary" | "ghost" | "danger" | "icon";
		size?: "sm" | "lg";
		type?: "button" | "submit" | "reset";
		disabled?: boolean;
		href?: string;
		onclick?: (e: MouseEvent) => void;
		class?: string;
		children?: import("svelte").Snippet;
	} = $props();

	const cls = $derived(
		["btn", `btn-${variant}`, size ? `btn-${size}` : "", className].filter(Boolean).join(" "),
	);
</script>

{#if href}
	<a {href} class={cls} {onclick}>
		{#if children}{@render children()}{/if}
	</a>
{:else}
	<button {type} {disabled} class={cls} {onclick}>
		{#if children}{@render children()}{/if}
	</button>
{/if}

<style>
	.btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-4);
		font-family: var(--font-body);
		font-size: var(--text-xs);
		font-weight: 600;
		line-height: 1.4;
		cursor: pointer;
		border: 1px solid transparent;
		border-radius: var(--radius-xs);
		text-decoration: none;
		transition:
			transform var(--duration-fast) var(--ease-out-expo),
			background var(--duration-fast) ease,
			color var(--duration-fast) ease,
			border-color var(--duration-fast) ease;
	}

	.btn:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 2px;
	}

	.btn:active {
		transform: scale(0.97);
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		transform: none;
	}

	.btn-primary {
		background: var(--color-accent);
		color: var(--color-text-on-accent);
		border-color: var(--color-accent);
	}

	.btn-primary:hover {
		background: var(--color-accent-dark);
		border-color: var(--color-accent-dark);
	}

	.btn-secondary {
		background: transparent;
		color: var(--color-text);
		border-color: var(--color-border-strong);
	}

	.btn-secondary:hover {
		border-color: var(--color-accent);
		color: var(--color-accent);
	}

	.btn-ghost {
		background: transparent;
		color: var(--color-text-secondary);
		border-color: transparent;
	}

	.btn-ghost:hover {
		background: var(--color-surface-sunken);
		color: var(--color-text);
	}

	.btn-danger {
		background: var(--color-danger);
		color: oklch(0.97 0.005 22);
		border-color: var(--color-danger);
	}

	.btn-danger:hover {
		background: oklch(0.48 0.15 22);
		border-color: oklch(0.48 0.15 22);
	}

	.btn-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		padding: 0;
		background: transparent;
		border: 1px solid transparent;
		border-radius: var(--radius-xs);
		cursor: pointer;
		color: var(--color-text-tertiary);
		transition:
			color var(--duration-fast) ease,
			background var(--duration-fast) ease,
			border-color var(--duration-fast) ease;
	}

	.btn-icon:hover {
		color: var(--color-text);
		background: var(--color-surface-sunken);
		border-color: var(--color-border);
	}

	.btn-icon:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	.btn-sm {
		padding: var(--space-1) var(--space-3);
		font-size: 9px;
	}

	.btn-lg {
		padding: var(--space-3) var(--space-8);
		font-size: var(--text-sm);
	}
</style>
