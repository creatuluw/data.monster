<script lang="ts">
	import { BarChart2, LineChart, AreaChart, PieChart, ScatterChart } from 'lucide-svelte';

	type ChartEntry = { id: string; label: string; desc: string; icon: typeof BarChart2; status: 'available' | 'planned' };

	const charts: ChartEntry[] = [
		{ id: 'bar', label: 'Bar Chart', desc: 'Compare values across categories with stacked bars, built-in tooltip, bullet legend, and click-to-filter interactions.', icon: BarChart2, status: 'available' },
		{ id: 'line', label: 'Line Chart', desc: 'Show trends and continuity over ordered dimensions like time or sequence.', icon: LineChart, status: 'planned' },
		{ id: 'area', label: 'Area Chart', desc: 'Emphasize volume under a trend line with filled regions.', icon: AreaChart, status: 'planned' },
		{ id: 'pie', label: 'Pie / Donut', desc: 'Show part-to-whole proportions for a small number of slices.', icon: PieChart, status: 'planned' },
		{ id: 'scatter', label: 'Scatter Plot', desc: 'Reveal correlation between two numeric measures.', icon: ScatterChart, status: 'planned' },
	];
</script>

<svelte:head>
	<title>Unovis — Data Monster Labs</title>
</svelte:head>

<div class="lib-page">
	<div class="lib-header">
		<span class="section-number">UNOVIS</span>
	</div>

	<div class="lib-body">
		<div class="lib-intro">
			<h1 class="lib-title">Unovis</h1>
			<p class="lib-desc">Framework-independent visualization primitives by F5. Renders SVG with TypeScript-first APIs, built-in tooltip, legend, and click-to-filter interactions powered by <code>@unovis/ts</code>.</p>
		</div>

		<div class="chart-grid">
			{#each charts as ct (ct.id)}
				{#if ct.status === 'available'}
					<a href="/labs/unovis-charts/{ct.id}" class="chart-card">
						<div class="chart-card-icon">
							<ct.icon size={18} />
						</div>
						<div class="chart-card-info">
							<span class="chart-card-title">{ct.label}</span>
							<span class="chart-card-desc">{ct.desc}</span>
						</div>
					</a>
				{:else}
					<div class="chart-card chart-card-disabled">
						<div class="chart-card-icon">
							<ct.icon size={18} />
						</div>
						<div class="chart-card-info">
							<span class="chart-card-title">{ct.label}<span class="soon-badge">Soon</span></span>
							<span class="chart-card-desc">{ct.desc}</span>
						</div>
					</div>
				{/if}
			{/each}
		</div>
	</div>
</div>

<style>
	.lib-page {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-6);
	}

	.lib-header {
		display: flex;
		align-items: center;
		justify-content: flex-end;
	}

	.section-number {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		color: var(--color-accent);
		padding: 2px var(--space-2);
		border: 1px solid var(--color-accent-muted);
		border-radius: var(--radius-xs);
		background: var(--color-accent-muted);
		white-space: nowrap;
	}

	.lib-intro {
		margin-top: var(--space-6);
	}

	.lib-title {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: 700;
		letter-spacing: -0.02em;
		margin: 0;
	}

	.lib-desc {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: var(--space-2) 0 0 0;
		max-width: 64ch;
		line-height: var(--leading-relaxed);
	}

	.lib-desc code {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		padding: 1px 4px;
		border-radius: var(--radius-xs);
		background: var(--color-surface-sunken);
		color: var(--color-text-secondary);
	}

	.chart-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: var(--space-4);
		margin-top: var(--space-8);
	}

	.chart-card {
		display: flex;
		align-items: flex-start;
		gap: var(--space-4);
		padding: var(--space-6);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		text-decoration: none;
		color: var(--color-text);
		background: var(--color-surface);
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.chart-card:hover {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}

	.chart-card-disabled {
		opacity: 0.45;
		cursor: default;
	}

	.chart-card-disabled:hover {
		border-color: var(--color-border);
		box-shadow: none;
	}

	.chart-card-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		border-radius: var(--radius-sm);
		background: var(--color-accent-muted);
		color: var(--color-accent);
		flex-shrink: 0;
	}

	.chart-card-info {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.chart-card-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.chart-card-desc {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		line-height: var(--leading-snug);
	}

	.soon-badge {
		font-family: var(--font-mono);
		font-size: 8px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		padding: 1px 4px;
		border-radius: var(--radius-xs);
		background: var(--color-surface-sunken);
		color: var(--color-text-tertiary);
	}
</style>
