<script lang="ts">
	import { FlaskConical } from 'lucide-svelte';
</script>

<svelte:head>
	<title>Labs — Data Monster</title>
</svelte:head>

<div class="labs-page">
	<div class="section-header">
		<span class="section-number">LABS</span>
		<h1 class="section-title">Experiments</h1>
	</div>

	<div class="labs-grid">
		<a href="/labs/chart-lib" class="lab-card">
			<div class="lab-icon">
				<FlaskConical size={18} />
			</div>
			<div class="lab-info">
				<span class="lab-title">Chart Library</span>
				<span class="lab-desc">Reusable bar chart powered by LayerChart</span>
			</div>
		</a>
	</div>
</div>

<style>
	.labs-page {
		padding: var(--space-12) var(--gutter);
		max-width: var(--max-width);
		margin: 0 auto;
	}

	.labs-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: var(--space-4);
		margin-top: var(--space-8);
	}

	.lab-card {
		display: flex;
		align-items: center;
		gap: var(--space-4);
		padding: var(--space-6);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		text-decoration: none;
		color: var(--color-text);
		background: var(--color-surface);
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.lab-card:hover {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}

	.lab-icon {
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

	.lab-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
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
	}
</style>
