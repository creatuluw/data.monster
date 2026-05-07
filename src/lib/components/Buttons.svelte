<div class="component-group">
	<span class="component-group-label">Interactive Elements</span>
	<h3 class="component-group-title">Buttons</h3>
	<div class="button-row">
		<button class="btn btn-primary btn-lg">Register</button>
		<button class="btn btn-primary">Confirm</button>
		<button class="btn btn-secondary">Edit</button>
		<button class="btn btn-ghost">Cancel</button>
		<button class="btn btn-danger btn-sm">Remove</button>
	</div>
</div>

<style>
	.component-group {
		position: relative;
	}

	.component-group-label {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.14em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
		margin-bottom: var(--space-6);
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.component-group-label::before {
		content: "\25A0";
		color: var(--color-accent);
		font-size: 7px;
	}

	.component-group-title {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
		margin-bottom: var(--space-6);
	}

	.button-row {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: var(--space-3);
	}

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

	.btn-sm {
		padding: var(--space-1) var(--space-3);
		font-size: 9px;
	}

	.btn-lg {
		padding: var(--space-3) var(--space-8);
		font-size: var(--text-sm);
	}
</style>
