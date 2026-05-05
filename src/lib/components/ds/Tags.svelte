<div class="component-group">
	<span class="component-group-label">Status Indicators</span>
	<h3 class="component-group-title">Tags</h3>
	<div class="tag-row">
		<span class="tag tag-default">Draft</span>
		<span class="tag tag-accent">Active</span>
		<span class="tag tag-success">On Track</span>
		<span class="tag tag-warning">At Risk</span>
		<span class="tag tag-danger">Overdue</span>
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

	.tag-row {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-2);
	}

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
