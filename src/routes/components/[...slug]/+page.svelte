<script lang="ts">
	import { page as pageState } from '$app/state';
	import { loadAppComponents } from '$lib/app-components';
	import ComponentDemo from '$lib/components/ComponentDemo.svelte';
	import { Box } from 'lucide-svelte';

	const all = loadAppComponents();
	const slug = $derived(pageState.params.slug ?? '');
	// catalog is deduped to one active component per name; accept legacy bare-name URLs
	const component = $derived(
		all.find((c) => `${c.dir ? c.dir + '/' : ''}${c.name}` === slug) ??
			(slug.includes('/') ? null : all.find((c) => c.name === slug)) ??
			null
	);
</script>

<div class="detail-page">

	{#if component}
		<header class="page-hero">
			<div class="hero-classification">
				<span class="class-line"></span>
				<Box size={12} />
				{component.dir || 'components'}
			</div>
			<h1 class="hero-title">{component.name}</h1>
			<p class="hero-meta font-mono">{component.path} · {component.lines} lines</p>
		</header>

		<hr class="divider" />

		<section class="section">
			<ComponentDemo name={component.name} />
		</section>
	{:else}
		<p class="no-live">
			Component not found.
		</p>
	{/if}
</div>

<style>
	.detail-page {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-6) var(--space-8);
	}

	/* reference: page-hero */
	.page-hero {
		padding: var(--space-8) 0 0 0;
	}

	.hero-classification {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.class-line {
		display: inline-block;
		width: 24px;
		height: 1px;
		background: var(--color-accent);
	}

	.hero-title {
		font-family: var(--font-display);
		font-size: var(--text-2xl, 2rem);
		font-weight: 700;
		letter-spacing: -0.02em;
		margin: var(--space-3) 0 0 0;
		color: var(--color-text);
	}

	.hero-meta {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		margin: var(--space-2) 0 0 0;
	}

	.divider {
		border: none;
		border-top: 1px solid var(--color-border);
		margin: var(--space-6) 0 0 0;
	}

	/* reference: numbered section header */
	.section {
		padding: var(--space-8) 0;
	}

	.no-live {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		max-width: 64ch;
		line-height: var(--leading-relaxed);
	}
</style>
