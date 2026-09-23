<script lang="ts">
	import { page as pageState } from '$app/state';
	import { loadAppComponents } from '$lib/app-components';
	import { highlightHTML } from '@speed-highlight/core';
	import '@speed-highlight/core/themes/github-light.css';
	import { Copy, Check, ArrowLeft, Box } from 'lucide-svelte';
	// ds/ components are self-showcasing (each renders its own demo, no props) — /ui renders them the same way
	import Accordion from '$lib/components/ds/Accordion.svelte';
	import Buttons from '$lib/components/ds/Buttons.svelte';
	import Cards from '$lib/components/ds/Cards.svelte';
	import ColorPalette from '$lib/components/ds/ColorPalette.svelte';
	import Footer from '$lib/components/ds/Footer.svelte';
	import Hero from '$lib/components/ds/Hero.svelte';
	import Icons from '$lib/components/ds/Icons.svelte';
	import Inputs from '$lib/components/ds/Inputs.svelte';
	import Modal from '$lib/components/ds/Modal.svelte';
	import Motion from '$lib/components/ds/Motion.svelte';
	import Nav from '$lib/components/ds/Nav.svelte';
	import Principles from '$lib/components/ds/Principles.svelte';
	import SearchAhead from '$lib/components/ds/SearchAhead.svelte';
	import Spacing from '$lib/components/ds/Spacing.svelte';
	import Tags from '$lib/components/ds/Tags.svelte';
	import Toast from '$lib/components/ds/Toast.svelte';
	import Toggles from '$lib/components/ds/Toggles.svelte';
	import Typography from '$lib/components/ds/Typography.svelte';

	// eslint-disable-next-line @typescript-eslint/no-explicit-any -- mixed svelte component types don't unify
	const DS_SHOWCASE: Record<string, any> = {
		Accordion, Buttons, Cards, ColorPalette, Footer, Hero, Icons, Inputs, Modal,
		Motion, Nav, Principles, SearchAhead, Spacing, Tags, Toast, Toggles, Typography
	};

	const all = loadAppComponents();
	const slug = $derived(pageState.params.slug ?? '');
	const component = $derived(all.find((c) => `${c.dir ? c.dir + '/' : ''}${c.name}` === slug) ?? null);
	const Demo = $derived(
		component && component.dir === 'ds' ? (DS_SHOWCASE[component.name] ?? null) : null
	);
	const siblings = $derived(component ? all.filter((c) => c.name === component.name) : []);

	let highlighted = $state('');
	let copied = $state(false);

	$effect(() => {
		if (!component) {
			highlighted = '';
			return;
		}
		let alive = true;
		highlightHTML(component.source, 'xml', { block: true, showLineNumbers: true })
			.then((html) => {
				if (alive) highlighted = html;
			})
			.catch(() =>
				highlightHTML(component!.source, 'plain', { block: true }).then((html) => {
					if (alive) highlighted = html;
				})
			);
		return () => {
			alive = false;
		};
	});

	async function copy() {
		if (!component) return;
		try {
			await navigator.clipboard.writeText(component.source);
			copied = true;
			setTimeout(() => (copied = false), 1500);
		} catch {
			// clipboard unavailable — the text is selectable anyway
		}
	}
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
			{#if Demo}
				<!-- eslint-disable-next-line svelte/no-at-html-tags -->
				<Demo />
			{:else}
				<p class="no-live">
					This component needs app data/props, so there is no standalone preview — see it
					working in the app, or read the source below. Components under
					<code class="font-mono">ds/</code> render live here.
				</p>
			{/if}
		</section>

		<hr class="divider-dashed" />

		<section class="section source-section">
			<div class="section-header">
				<span class="section-number">02</span>
				<h2 class="section-title">Source</h2>
				<button class="copy-btn" onclick={copy} title={copied ? 'Copied' : 'Copy source'}>
					{#if copied}<Check size={14} class="text-green-600" /> Copied{:else}<Copy size={14} /> Copy{/if}
				</button>
			</div>
			<div class="source-wrap">
				<!-- eslint-disable-next-line svelte/no-at-html-tags -->
				{@html highlighted}
			</div>
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

	.divider-dashed {
		border: none;
		border-top: 1px dashed var(--color-border);
		margin: 0;
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

	.copy-btn {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 6px 10px;
		border-radius: var(--radius-md, 6px);
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-accent-dark, #1e3a5f);
		background: var(--color-accent-muted, #eef4fa);
	}

	.copy-btn:hover {
		background: var(--color-accent, #d7e6f5);
	}

	.source-section .copy-btn {
		margin-left: auto;
	}

	.source-wrap {
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		overflow: auto;
		max-height: 60vh;
	}

	.source-wrap :global(.shj) {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		line-height: var(--leading-relaxed);
		margin: 0;
	}
</style>
