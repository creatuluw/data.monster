<script lang="ts">
	import { page as pageState } from '$app/state';
	import { loadAppComponents } from '$lib/app-components';
	import ComponentDemo from '$lib/components/ComponentDemo.svelte';
	import { ArrowLeft, Box } from 'lucide-svelte';

	const all = loadAppComponents();
	const slug = $derived(pageState.params.slug ?? '');
	const component = $derived.by(() => {
		const exact = all.find((c) => `${c.dir ? c.dir + '/' : ''}${c.name}` === slug);
		if (exact) {
			// 18 root components share a name with a ds/ one — prefer the showcase variant
			if (exact.dir !== 'ds') {
				const twin = all.find((c) => c.dir === 'ds' && c.name === exact.name);
				if (twin) return twin;
			}
			return exact;
		}
		// bare name with only a ds/ match (e.g. /components/Accordion) -> showcase variant
		if (!slug.includes('/')) {
			const twin = all.find((c) => c.dir === 'ds' && c.name === slug);
			if (twin) return twin;
		}
		return null;
	});
	const siblings = $derived(component ? all.filter((c) => c.name === component.name) : []);
</script>

<div class="detail-page">
	<a href="/components" class="back"><ArrowLeft size={14} /> All components</a>

	{#if component}
		<header class="page-hero">
			<div class="hero-classification">
				<span class="class-line"></span>
				<Box size={12} />
				{component.dir || 'components'}
			</div>
			<h1 class="hero-title">{component.name}</h1>
			<p class="hero-meta font-mono">{component.path} · {component.lines} lines</p>
			{#if siblings.length > 1}
				<p class="dup-note">
					Also at: {siblings.filter((s) => s !== component).map((s) => s.path).join(', ')}
				</p>
			{/if}
		</header>

		<hr class="divider" />

		<section class="section">
			<div class="section-header">
				<span class="section-number">01</span>
				<h2 class="section-title">Visual</h2>
			</div>
			<ComponentDemo name={component.name} />
		</section>
	{:else}
		<p class="no-live">
			Component not found. <a href="/components" class="back">Back to all components</a>
		</p>
	{/if}
</div>

<style>
	.detail-page {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-6) var(--space-8);
	}

	.back {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		text-decoration: none;
	}

	.back:hover {
		color: var(--color-text);
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

	.dup-note {
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

	.section-header {
		display: flex;
		align-items: center;
		gap: var(--space-4);
		margin-bottom: var(--space-6);
	}

	.section-number {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		letter-spacing: 0.1em;
		color: var(--color-accent);
		padding: var(--space-1) var(--space-2);
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
		color: var(--color-text);
		margin: 0;
	}

	.no-live {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		max-width: 64ch;
		line-height: var(--leading-relaxed);
	}
</style>
