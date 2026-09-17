<script lang="ts">
	import { setupChartRegistry } from '$lib/charts/registry-setup.svelte';
	import { getLibraryComponents } from '$lib/library/registry';
	import { ChartColumn, Wrench } from 'lucide-svelte';

	setupChartRegistry();
	const components = getLibraryComponents();
</script>

<svelte:head>
	<title>Library — Data Monster</title>
</svelte:head>

<div class="page">
	<div class="section-header">
		<h1 class="section-title">Library</h1>
	</div>

	<p class="section-subtitle">
		Extension components available to the whole app — every chart here is registered for use in
		/pages. Devs add new components as packages under <code>src/lib/library/components/</code>.
	</p>

	<div class="grid">
		{#each components as c (c.def.type)}
			<a class="card" href="/library/{c.def.type}">
				<div class="card-icon"><ChartColumn size={18} /></div>
				<div class="card-body">
					<span class="card-title">{c.def.label}</span>
					<span class="card-desc">{c.description}</span>
				</div>
			</a>
		{/each}
		<a class="card" href="/library/dev">
			<div class="card-icon"><Wrench size={18} /></div>
				<div class="card-body">
					<span class="card-title">For developers</span>
					<span class="card-desc">Create components: the package contract, test-first build, e2e report — or let the library-component-builder skill do it.</span>
				</div>
			</a>
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
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: 700;
		letter-spacing: -0.02em;
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

	/* same card as /labs */
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
	}

	.card-desc {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		line-height: var(--leading-snug);
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}
</style>
