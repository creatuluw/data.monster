<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		variant = 'primary',
		size = 'md',
		disabled = false,
		onclick,
		children
	}: {
		variant?: 'primary' | 'secondary' | 'ghost' | 'danger';
		size?: 'sm' | 'md';
		disabled?: boolean;
		onclick?: (e: MouseEvent) => void;
		children: Snippet;
	} = $props();
</script>

<button type="button" class="btn btn-{variant} btn-{size}" {disabled} {onclick}>
	{@render children()}
</button>

<style>
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-1);
		border: 1px solid transparent;
		border-radius: var(--radius-sm);
		font-family: var(--font-body);
		font-weight: 500;
		cursor: pointer;
		white-space: nowrap;
		transition:
			background 120ms ease,
			color 120ms ease,
			border-color 120ms ease;
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn:focus-visible {
		outline: none;
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}

	.btn-md {
		height: 30px;
		padding: 0 var(--space-3);
		font-size: var(--text-sm);
	}

	.btn-sm {
		height: 24px;
		padding: 0 var(--space-2);
		font-size: var(--text-xs);
	}

	.btn-primary {
		background: var(--color-accent);
		color: var(--color-text-on-accent);
	}

	.btn-primary:hover:not(:disabled) {
		background: var(--color-accent-dark);
	}

	.btn-secondary {
		background: var(--color-surface);
		border-color: var(--color-border-strong);
		color: var(--color-text);
	}

	.btn-secondary:hover:not(:disabled) {
		background: var(--color-surface-sunken);
	}

	.btn-ghost {
		background: none;
		color: var(--color-text-secondary);
	}

	.btn-ghost:hover:not(:disabled) {
		color: var(--color-text);
	}

	.btn-danger {
		background: var(--color-danger);
		color: white;
	}

	.btn-danger:hover:not(:disabled) {
		background: oklch(0.48 0.16 25);
	}
</style>
