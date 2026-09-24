<script lang="ts">
	import { loadAppComponents } from '$lib/app-components';
	import { Box } from 'lucide-svelte';

	const components = loadAppComponents();
	// 18 root components share a name with a ds/ one — link to the ds/ (showcase) variant
	const dsNames = new Set(components.filter((c) => c.dir === 'ds').map((c) => c.name));

	function hrefFor(c: (typeof components)[number]): string {
		const dir = c.dir === 'ds' || !dsNames.has(c.name) ? c.dir : 'ds';
		return `/components/${dir ? dir + '/' : ''}${c.name}`;
	}

	function dirLabel(dir: string): string {
		return dir || 'components';
	}
</script>

<svelte:head>
	<title>Components — Data Monster</title>
</svelte:head>

<div class="page">
	<div class="section-header">
		<h1 class="section-title page-title">Components</h1>
	</div>

	<p class="section-subtitle">
		Every component with a live showcase, from <code>src/lib/components/</code>.
		Click a card to see it rendered.
	</p>

	<div class="grid">
		{#each components as c (c.path)}
			<a class="card" href={hrefFor(c)}>
				<div class="card-icon"><Box size={18} /></div>
				<div class="card-body">
					<span class="card-title">{c.name}</span>
					<span class="card-desc">{c.lines} lines · {dirLabel(c.dir)}</span>
				</div>
			</a>
		{/each}
	</div>
</div>

<style>
	.page {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-6);
	}

	.section-header {
		display: flex;
		align-items: baseline;
		gap: var(--space-4);
	}

	.section-title {
		margin: 0;
	}

	.section-subtitle {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: var(--space-1) 0 var(--space-6) 0;
		max-width: 64ch;
		line-height: var(--leading-relaxed);
	}

	.section-subtitle code {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: var(--space-4);
	}

	/* same card as /labs and /library */
	.card {
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

	.card:hover {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}

	.card-icon {
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

	.card-body {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		flex: 1;
		min-width: 0;
	}

	.card-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 700;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.card-desc {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		font-family: var(--font-mono);
		line-height: var(--leading-snug);
	}
</style>
