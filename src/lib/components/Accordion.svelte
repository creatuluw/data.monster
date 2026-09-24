<script lang="ts">
	import { Accordion } from 'bits-ui';

	const items = [
		{
			value: 'query-params',
			title: 'Query Parameters',
			body: {
				lead: 'Configuration options for query execution. All parameters must be validated before the analysis pipeline runs.',
				points: [
					'Time range: configurable per data source',
					'Minimum: 1 active connection per query',
					'Aggregation: configurable per metric type',
					'Precision: two decimal places for currency'
				]
			}
		},
		{
			value: 'pipeline',
			title: 'Analysis Pipeline',
			body: {
				lead: 'Standard procedure for running data analysis. Start with raw ingestion, apply transforms, then generate output.',
				points: [
					'Step 1: Select data sources and time range',
					'Step 2: Configure transforms and filters',
					'Step 3: Review sample output for accuracy',
					'Step 4: Execute and schedule delivery'
				]
			}
		},
		{
			value: 'classification',
			title: 'Data Classification',
			body: {
				lead: 'Classification taxonomy for data quality states. Every dataset maps to exactly one quality level per pipeline run.',
				points: [
					'Fresh: Ingested within last 24 hours',
					'Stale: Last refresh older than threshold',
					'Validated: Passed all quality checks',
					'Failed: Schema mismatch or missing fields'
				]
			}
		}
	];
</script>

<div class="component-group">
	<span class="component-group-label">Collapsible Content</span>
	<h3 class="component-group-title">Accordion</h3>

	<Accordion.Root type="multiple">
		{#each items as item (item.value)}
			<Accordion.Item value={item.value} class="accordion">
				<Accordion.Header>
					<Accordion.Trigger class="accordion-trigger">
						<span class="accordion-trigger-left">{item.title}</span>
						<span class="accordion-icon">▾</span>
					</Accordion.Trigger>
				</Accordion.Header>
				<Accordion.Content forceMount class="accordion-panel">
					<div class="accordion-content">
						{item.body.lead}
						<ul>
							{#each item.body.points as point (point)}
								<li>{point}</li>
							{/each}
						</ul>
					</div>
				</Accordion.Content>
			</Accordion.Item>
		{/each}
	</Accordion.Root>
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
		content: '\25A0';
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

	/* bits-ui renders item/header/trigger/content — chrome hangs off
	   :global() selectors under our .component-group wrapper */
	.component-group :global(.accordion) {
		border: 1px solid var(--color-surface-sunken);
		background: var(--color-surface);
	}

	.component-group :global(.accordion + .accordion) {
		border-top: none;
	}

	.component-group :global(.accordion-trigger) {
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
		font-weight: 700;
		color: var(--color-text);
		text-align: left;
		transition:
			background var(--duration-fast) ease,
			color var(--duration-fast) ease;
	}

	.component-group :global(.accordion-trigger:hover) {
		background: var(--color-surface-raised);
	}

	.component-group :global(.accordion-trigger:focus-visible) {
		outline: 2px solid var(--color-accent);
		outline-offset: -2px;
		z-index: 1;
	}

	.component-group :global(.accordion-trigger[aria-expanded='true']) {
		background: var(--color-surface-raised);
		border-bottom: 1px solid var(--color-border);
	}

	.component-group :global(.accordion-trigger-left) {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.component-group :global(.accordion-icon) {
		width: 16px;
		height: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: transform var(--duration-base) var(--ease-out-expo);
		color: var(--color-text-tertiary);
		flex-shrink: 0;
	}

	.component-group :global(.accordion-trigger[aria-expanded='true'] .accordion-icon) {
		transform: rotate(180deg);
	}

	/* height animation driven by the bits CSS var; forceMount keeps the panel
	   mounted so the close transition plays (bits sets hidden — we hide via
	   max-height instead, as the pre-bits demo did) */
	.component-group :global(.accordion-panel[hidden]) {
		display: block;
	}

	.component-group :global(.accordion-panel) {
		overflow: hidden;
		max-height: 0;
		transition: max-height var(--duration-slow) var(--ease-out-expo);
	}

	.component-group :global(.accordion-panel[data-state='open']) {
		max-height: var(--bits-accordion-content-height);
	}

	.component-group :global(.accordion-content) {
		padding: var(--space-6);
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		max-width: 60ch;
	}

	.component-group :global(.accordion-content ul) {
		margin-top: var(--space-3);
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		list-style: none;
	}

	.component-group :global(.accordion-content li) {
		display: flex;
		align-items: baseline;
		gap: var(--space-2);
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}

	.component-group :global(.accordion-content li::before) {
		content: '\2014';
		color: var(--color-border-strong);
		flex-shrink: 0;
	}
</style>
