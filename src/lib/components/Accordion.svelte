<script lang="ts">
	let panels = $state([
		{ id: '1', open: false },
		{ id: '2', open: false },
		{ id: '3', open: false }
	]);

	function toggle(id: string) {
		panels = panels.map(p => p.id === id ? { ...p, open: !p.open } : p);
	}
</script>

<div class="component-group">
	<span class="component-group-label">Collapsible Content</span>
	<h3 class="component-group-title">Accordion</h3>

	{#each panels as panel, i}
		<div class="accordion">
			<button
				class="accordion-trigger"
				aria-expanded={panel.open}
				aria-controls="acc-panel-{panel.id}"
				id="acc-trigger-{panel.id}"
				onclick={() => toggle(panel.id)}
			>
				<span class="accordion-trigger-left">
				{#if i === 0}Query Parameters{:else if i === 1}Analysis Pipeline{:else}Data Classification{/if}
			</span>
				<span class="accordion-icon" style="transform: rotate({panel.open ? 180 : 0}deg)">▾</span>
			</button>
			<div
				class="accordion-panel"
				id="acc-panel-{panel.id}"
				role="region"
				aria-labelledby="acc-trigger-{panel.id}"
				aria-hidden={!panel.open}
				style="max-height: {panel.open ? '500px' : '0'}"
			>
				<div class="accordion-content">
					{#if i === 0}
						Configuration options for query execution. All parameters
						must be validated before the analysis pipeline runs.
						<ul>
							<li>Time range: configurable per data source</li>
							<li>Minimum: 1 active connection per query</li>
							<li>Aggregation: configurable per metric type</li>
							<li>Precision: two decimal places for currency</li>
						</ul>
					{:else if i === 1}
						Standard procedure for running data analysis. Start with
						raw ingestion, apply transforms, then generate output.
						<ul>
							<li>Step 1: Select data sources and time range</li>
							<li>Step 2: Configure transforms and filters</li>
							<li>Step 3: Review sample output for accuracy</li>
							<li>Step 4: Execute and schedule delivery</li>
						</ul>
					{:else}
						Classification taxonomy for data quality states. Every
						dataset maps to exactly one quality level per pipeline run.
						<ul>
							<li>Fresh: Ingested within last 24 hours</li>
							<li>Stale: Last refresh older than threshold</li>
							<li>Validated: Passed all quality checks</li>
							<li>Failed: Schema mismatch or missing fields</li>
						</ul>
					{/if}
				</div>
			</div>
		</div>
	{/each}
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

	.accordion {
		border: 1px solid var(--color-surface-sunken);
		background: var(--color-surface);
	}

	.accordion + .accordion {
		border-top: none;
	}

	.accordion-trigger {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-4) var(--space-6);
		background: var(--color-surface);
		border: none;
		cursor: pointer;
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
		text-align: left;
		transition:
			background var(--duration-fast) ease,
			color var(--duration-fast) ease;
	}

	.accordion-trigger:hover {
		background: var(--color-surface-raised);
	}

	.accordion-trigger:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: -2px;
		z-index: 1;
	}

	.accordion-trigger[aria-expanded="true"] {
		background: var(--color-surface-raised);
		border-bottom: 1px solid var(--color-border);
	}

	.accordion-trigger-left {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.accordion-ref {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.08em;
		color: var(--color-accent);
		min-width: 48px;
	}

	.accordion-icon {
		width: 16px;
		height: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: transform var(--duration-base) var(--ease-out-expo);
		color: var(--color-text-tertiary);
		flex-shrink: 0;
	}

	.accordion-panel {
		overflow: hidden;
		transition: max-height var(--duration-slow) var(--ease-out-expo);
	}

	.accordion-content {
		padding: var(--space-6);
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		max-width: 60ch;
	}

	.accordion-content ul {
		margin-top: var(--space-3);
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		list-style: none;
	}

	.accordion-content li {
		display: flex;
		align-items: baseline;
		gap: var(--space-2);
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}

	.accordion-content li::before {
		content: "\2014";
		color: var(--color-border-strong);
		flex-shrink: 0;
	}
</style>
