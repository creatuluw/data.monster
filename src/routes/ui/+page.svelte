<script lang="ts">
	import { onMount } from 'svelte';
	import Nav from '$lib/components/ds/Nav.svelte';
	import Hero from '$lib/components/ds/Hero.svelte';
	import ColorPalette from '$lib/components/ds/ColorPalette.svelte';
	import Typography from '$lib/components/ds/Typography.svelte';
	import Spacing from '$lib/components/ds/Spacing.svelte';
	import Buttons from '$lib/components/ds/Buttons.svelte';
	import Tags from '$lib/components/ds/Tags.svelte';
	import Inputs from '$lib/components/ds/Inputs.svelte';
	import Toggles from '$lib/components/ds/Toggles.svelte';
	import Cards from '$lib/components/ds/Cards.svelte';
	import Accordion from '$lib/components/ds/Accordion.svelte';
	import Modal from '$lib/components/ds/Modal.svelte';
	import SearchAhead from '$lib/components/ds/SearchAhead.svelte';
	import Toast from '$lib/components/ds/Toast.svelte';
	import Motion from '$lib/components/ds/Motion.svelte';
	import Principles from '$lib/components/ds/Principles.svelte';
	import Footer from '$lib/components/ds/Footer.svelte';

	onMount(() => {
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						(entry.target as HTMLElement).style.animationPlayState = 'running';
						observer.unobserve(entry.target);
					}
				});
			},
			{ threshold: 0.1 }
		);
		document.querySelectorAll('.ds-page .entrance').forEach((el) => {
			observer.observe(el);
		});
	});
</script>

<svelte:head>
	<title>Hoi — Design System + Components</title>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link
		href="https://fonts.googleapis.com/css2?family=Source+Serif+4:opsz,wght@8..60,200..900&family=Manrope:wght@200..800&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<div class="ds-breakout">
<div class="ds-page">
	<Nav />
	<Hero />

	<hr class="divider" />

	<ColorPalette />

	<hr class="divider-dashed" />

	<Typography />

	<hr class="divider-dashed" />

	<Spacing />

	<hr class="divider-dashed" />

	<section class="section" id="components" aria-labelledby="components-title">
		<div class="section-header entrance">
			<span class="section-number">04</span>
			<h2 class="section-title" id="components-title">Components</h2>
		</div>

		<div class="component-area entrance entrance-d1">
			<Buttons />
			<Tags />
			<Inputs />
			<Toggles />
			<Cards />
			<Accordion />
			<Modal />
			<SearchAhead />
			<Toast />
		</div>
	</section>

	<hr class="divider-dashed" />

	<Motion />

	<hr class="divider-dashed" />

	<Principles />

	<hr class="divider" style="margin-top: var(--space-16)" />

	<Footer />
</div>
</div>

<style>
	.ds-breakout {
		width: 100vw;
		position: relative;
		left: 50%;
		right: 50%;
		margin-left: -50vw;
		margin-right: -50vw;
		background: white;
	}

	:global(.flex.h-screen.flex-col > header) {
		display: none;
	}

	:global(.flex.h-screen.flex-col .flex.flex-1.overflow-hidden) {
		padding: 0;
	}

	:global(.flex.h-screen.flex-col .flex.flex-1.overflow-hidden main) {
		padding: 0;
		max-width: 100%;
		overflow-x: hidden;
	}

	:global(html) {
		overflow-x: clip;
	}

	:global(body) {
		overflow-x: clip;
	}

	:global(.ds-page) {
		--color-accent: oklch(0.69 0.16 41);
		--color-accent-light: oklch(0.78 0.09 41);
		--color-accent-dark: oklch(0.44 0.14 41);
		--color-accent-muted: oklch(0.93 0.03 41);

		--color-surface: #ffffff;
		--color-surface-raised: oklch(0.96 0.005 250);
		--color-surface-sunken: oklch(0.935 0.006 250);

		--color-text: oklch(0.14 0.01 250);
		--color-text-secondary: oklch(0.4 0.012 250);
		--color-text-tertiary: oklch(0.58 0.01 250);
		--color-text-on-accent: oklch(0.97 0.005 250);

		--color-border: oklch(0.86 0.006 250);
		--color-border-strong: oklch(0.72 0.012 250);
		--color-grid: oklch(0.9 0.004 250);

		--color-success: oklch(0.68 0.14 160);
		--color-warning: oklch(0.76 0.13 80);
		--color-danger: oklch(0.58 0.17 22);

		--font-display: "Source Serif 4", serif;
		--font-body: "Manrope", sans-serif;
		--font-mono: "Courier New", Courier, monospace;

		--text-xs: 0.6875rem;
		--text-sm: 0.8125rem;
		--text-base: 1rem;
		--text-md: 1.25rem;
		--text-lg: 1.563rem;
		--text-xl: 1.953rem;
		--text-2xl: 2.441rem;
		--text-3xl: clamp(2.8rem, 5.5vw, 3.815rem);
		--text-4xl: clamp(3.2rem, 7vw, 5.06rem);

		--leading-tight: 1.08;
		--leading-snug: 1.25;
		--leading-normal: 1.6;
		--leading-relaxed: 1.8;

		--space-1: 0.25rem;
		--space-2: 0.5rem;
		--space-3: 0.75rem;
		--space-4: 1rem;
		--space-6: 1.5rem;
		--space-8: 2rem;
		--space-12: 3rem;
		--space-16: 4rem;
		--space-24: 6rem;

		--radius-xs: 2px;
		--radius-sm: 3px;
		--radius-md: 4px;
		--radius-lg: 6px;
		--radius-xl: 10px;
		--radius-full: 9999px;

		--shadow-sm: 0 1px 2px oklch(0.22 0.005 250 / 0.07);
		--shadow-md: 0 2px 8px oklch(0.22 0.005 250 / 0.09);
		--shadow-lg: 0 6px 24px oklch(0.22 0.005 250 / 0.12);

		--ease-out-expo: cubic-bezier(0.16, 1, 0.3, 1);
		--ease-out-quart: cubic-bezier(0.25, 1, 0.5, 1);
		--ease-industrial: cubic-bezier(0.22, 0.61, 0.36, 1);
		--duration-fast: 120ms;
		--duration-base: 250ms;
		--duration-slow: 450ms;
		--duration-enter: 600ms;

		--max-width: 72rem;
		--gutter: clamp(1rem, 3vw, 2rem);

		--grid-color: oklch(0.9 0.004 250 / 0.35);
		--grid-spacing: 24px;

		font-family: var(--font-body);
		font-size: var(--text-base);
		line-height: var(--leading-normal);
		color: var(--color-text);
		background: var(--color-surface);
	}

	:global(.ds-page .section) {
		max-width: var(--max-width);
		margin: 0 auto;
		padding: var(--space-16) var(--gutter);
		position: relative;
	}

	:global(.ds-page .section + .section) {
		padding-top: 0;
	}

	:global(.ds-page .section-header) {
		display: flex;
		align-items: baseline;
		gap: var(--space-4);
		margin-bottom: var(--space-12);
	}

	:global(.ds-page .section-number) {
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

	:global(.ds-page .section-title) {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: 700;
		letter-spacing: -0.02em;
		line-height: var(--leading-snug);
		color: var(--color-text);
	}

	:global(.ds-page .divider) {
		border: none;
		height: 1px;
		background: var(--color-border);
		max-width: var(--max-width);
		margin: 0 auto;
	}

	:global(.ds-page .divider-dashed) {
		border: none;
		height: 0;
		border-top: 1px dashed var(--color-border);
		max-width: var(--max-width);
		margin: 0 auto;
	}

	:global(.ds-page .component-area) {
		display: flex;
		flex-direction: column;
		gap: var(--space-16);
	}

	@keyframes slideUp {
		from { opacity: 0; transform: translateY(12px); }
		to { opacity: 1; transform: translateY(0); }
	}

	:global(.ds-page .entrance) {
		animation: slideUp var(--duration-enter) var(--ease-out-expo) both;
		animation-play-state: paused;
	}

	:global(.ds-page .entrance-d1) { animation-delay: 60ms; }
	:global(.ds-page .entrance-d2) { animation-delay: 120ms; }
	:global(.ds-page .entrance-d3) { animation-delay: 180ms; }
	:global(.ds-page .entrance-d4) { animation-delay: 240ms; }
	:global(.ds-page .entrance-d5) { animation-delay: 300ms; }

	@media (prefers-reduced-motion: reduce) {
		:global(.ds-page .entrance),
		:global(.ds-page .entrance-d1),
		:global(.ds-page .entrance-d2),
		:global(.ds-page .entrance-d3),
		:global(.ds-page .entrance-d4),
		:global(.ds-page .entrance-d5) {
			animation: none;
			opacity: 1;
		}
	}
</style>
