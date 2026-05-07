<div class="component-group">
	<span class="component-group-label">Information Panels</span>
	<h3 class="component-group-title">Cards</h3>
	<div class="card-grid">
		<article class="card">
			<span class="card-id">GL-001</span>
			<h4 class="card-title">Structured Queries</h4>
			<p class="card-body">
				Every analysis starts with a well-defined query. Named templates
				and parameterized filters ensure reproducibility across dashboards
				and scheduled reports.
			</p>
			<div class="card-footer">
				<span class="tag tag-accent">Core</span>
				<button class="btn btn-ghost btn-sm">Details</button>
			</div>
		</article>
		<article class="card">
			<span class="card-id">GL-002</span>
			<h4 class="card-title">Live Refresh</h4>
			<p class="card-body">
				Data sources update in real time. When a pipeline finishes or
				an ingestion completes, dashboards reflect the change immediately.
				No manual reload. No stale numbers.
			</p>
			<div class="card-footer">
				<span class="tag tag-default">Motion</span>
				<button class="btn btn-ghost btn-sm">Details</button>
			</div>
		</article>
		<article class="card">
			<span class="card-id">GL-003</span>
			<h4 class="card-title">Perceptual Accuracy</h4>
			<p class="card-body">
				OKLCH ensures colors look equally distinct in charts and heatmaps.
				Category palettes, status indicators, and alert levels are
				perceptually uniform across the entire interface.
			</p>
			<div class="card-footer">
				<span class="tag tag-default">Color</span>
				<button class="btn btn-ghost btn-sm">Details</button>
			</div>
		</article>
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

	.card-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: var(--space-6);
	}

	.card {
		background: var(--color-surface);
		border: 1px solid var(--color-surface-sunken);
		border-radius: var(--radius-md);
		padding: var(--space-8);
		position: relative;
		transition: box-shadow var(--duration-fast) ease, border-color var(--duration-fast) ease;
	}

	.card:hover {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}

	.card-id {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		color: var(--color-text-tertiary);
		margin-bottom: var(--space-4);
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.card-id::before {
		content: "";
		width: 4px;
		height: 4px;
		background: var(--color-accent);
	}

	.card-title {
		font-family: var(--font-display);
		font-size: var(--text-lg);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
		margin-bottom: var(--space-3);
	}

	.card-body {
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		max-width: 45ch;
	}

	.card-footer {
		margin-top: var(--space-6);
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding-top: var(--space-4);
		border-top: 1px dashed var(--color-border);
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

	.btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		font-family: var(--font-body);
		font-weight: 600;
		line-height: 1.4;
		cursor: pointer;
		border: 1px solid transparent;
		border-radius: var(--radius-xs);
		transition:
			transform var(--duration-fast) var(--ease-out-expo),
			background var(--duration-fast) ease,
			color var(--duration-fast) ease;
		background: transparent;
		color: var(--color-text-secondary);
	}

	.btn:hover {
		background: var(--color-surface-sunken);
		color: var(--color-text);
	}

	.btn:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 2px;
	}

	.btn:active {
		transform: scale(0.97);
	}

	.btn-sm {
		padding: var(--space-1) var(--space-3);
		font-size: 9px;
	}
</style>
