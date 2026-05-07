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
	import Icons from '$lib/components/ds/Icons.svelte';
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

	<Icons />

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

	:global(:root) {
		--grid-color: oklch(0.9 0.004 250 / 0.35);
		--grid-spacing: 24px;
	}

	:global(.ds-page) {
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
