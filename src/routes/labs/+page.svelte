<script lang="ts">
	import { Grid3x3, BarChart3 } from 'lucide-svelte';
</script>

<svelte:head>
	<title>Labs — Data Monster</title>
</svelte:head>

<div class="labs-page">
	<div class="section-header">
		<span class="section-number">LABS</span>
		<h1 class="section-title">Chart types</h1>
	</div>

	<p class="section-subtitle">
		One card per chart type we are building here, each with its own playground. Heatmap first.
	</p>

	<div class="labs-grid">
		<a href="/labs/heatmap" class="lab-card">
			<div class="lab-icon">
				<Grid3x3 size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Heatmap</span>
				<span class="lab-desc">Threshold-colored grid with in-cell labels, hover tooltips, click-to-select and no-data cells. Built on svelteplot.</span>
			</div>
		</a>

		<a href="/labs/bar-chart" class="lab-card">
			<div class="lab-icon">
				<BarChart3 size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Bar chart</span>
				<span class="lab-desc">Aggregation via buildBars (top N + Other), svelteplot BarY rendering, hover tooltip, click-to-select. First chart on the shared fundament.</span>
			</div>
		</a>
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
</style>
