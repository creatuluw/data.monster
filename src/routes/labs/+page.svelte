<script lang="ts">
	import { BarChart2, Palette, Sparkles, Layers, FlaskConical, PieChart } from 'lucide-svelte';
</script>

<svelte:head>
	<title>Labs — Data Monster</title>
</svelte:head>

<div class="labs-page">
	<div class="section-header">
		<span class="section-number">LABS</span>
		<h1 class="section-title">Experiments</h1>
	</div>

	<p class="section-subtitle">
		Chart rendering engines under evaluation. Each library has different strengths — try all three and compare.
	</p>

	<div class="labs-grid">
		<a href="/labs/picasso-charts" class="lab-card">
			<div class="lab-icon">
				<FlaskConical size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Picasso Charts</span>
				<span class="lab-desc">Canvas-based charting by Qlik. Declarative component model with built-in brushing, interactions, and SQL-driven live data — renders to `<canvas>`.</span>
			</div>
		</a>

		<a href="/labs/observable-charts" class="lab-card">
			<div class="lab-icon">
				<FlaskConical size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Observable Charts</span>
				<span class="lab-desc">Grammar-of-graphics charting by Observable. Renders SVG with concise Plot API, faceting, and mark composition — a battle-tested standard for exploratory data visualization.</span>
			</div>
		</a>

		<a href="/labs/svelteplot-charts" class="lab-card">
			<div class="lab-icon">
				<Sparkles size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">SveltePlot</span>
				<span class="lab-desc">Svelte-native wrapper around Observable Plot. Renders SVG with idiomatic Svelte components, linked interactions, and first-class reactivity — no canvas layer.</span>
			</div>
		</a>

		<a href="/labs/chart-lib" class="lab-card">
			<div class="lab-icon">
				<Layers size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">LayerChart</span>
				<span class="lab-desc">Layered grammar-of-graphics for Svelte. Composable marks, scales, and axes that compile to SVG — built on top of D3 with full Svelte reactivity.</span>
			</div>
		</a>

		<a href="/labs/unovis-charts" class="lab-card">
			<div class="lab-icon">
				<FlaskConical size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Unovis</span>
				<span class="lab-desc">Framework-independent visualization by F5. SVG bar charts with built-in tooltip, bullet legend, and click-to-filter interactions powered by @unovis/ts.</span>
			</div>
		</a>

		<a href="/labs/echarts-charts" class="lab-card">
			<div class="lab-icon">
				<PieChart size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Apache ECharts</span>
				<span class="lab-desc">Production-grade Canvas/SVG charting by Apache. Rich interactivity, built-in tooltip, zoom, and click-to-filter with bar dimming powered by echarts.</span>
			</div>
		</a>
	</div>

	<div class="showcase-section">
		<a href="/labs/charts" class="showcase-link">
			<BarChart2 size={16} />
			<span>View chart type showcase</span>
			<span class="showcase-arrow">&rarr;</span>
		</a>
		<span class="showcase-hint">All chart types we are building across the three libraries</span>
	</div>
</div>

<style>
	.labs-page {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-6);
	}

	.section-header {
		display: flex;
		align-items: baseline;
		gap: var(--space-4);
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

	.section-title {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: 700;
		letter-spacing: -0.02em;
		margin: 0;
	}

	.section-subtitle {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: var(--space-3) 0 0 0;
		max-width: 64ch;
		line-height: var(--leading-relaxed);
	}

	.labs-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: var(--space-3);
		margin-top: var(--space-6);
	}

	.lab-card {
		display: flex;
		align-items: flex-start;
		gap: var(--space-3);
		padding: var(--space-4);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		text-decoration: none;
		color: var(--color-text);
		background: var(--color-surface);
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
		box-sizing: border-box;
	}

	.lab-card:hover {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}

	.lab-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border-radius: var(--radius-sm);
		background: var(--color-accent-muted);
		color: var(--color-accent);
		flex-shrink: 0;
	}

	.lab-info {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		flex: 1;
		min-width: 0;
	}

	.lab-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
	}

	.lab-desc {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		line-height: var(--leading-snug);
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.showcase-section {
		margin-top: var(--space-4);
		padding-top: var(--space-4);
		border-top: 1px dashed var(--color-border);
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.showcase-link {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
		text-decoration: none;
		transition: color var(--duration-fast) ease;
	}

	.showcase-link:hover {
		color: var(--color-accent);
	}

	.showcase-arrow {
		color: var(--color-text-tertiary);
		transition: transform var(--duration-fast) ease;
	}

	.showcase-link:hover .showcase-arrow {
		transform: translateX(2px);
	}

	.showcase-hint {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.02em;
		padding-left: var(--space-6);
	}
</style>
