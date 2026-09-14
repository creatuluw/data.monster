<script lang="ts">
	import BarChart from '$lib/components/charts/BarChart.svelte';
	import { buildBars, type BarDatum } from '$lib/charts/fundament';

	type Sale = { region: string; channel: string; amount: number };

	const REGIONS = ['EU', 'US', 'APAC', 'LATAM', 'MEA', 'CA', 'OCE', 'NORDICS'];
	const CHANNELS = ['Online', 'Retail', 'Partner'];

	// deterministic pseudo-random from coords — stable across reloads
	function rnd(i: number, j: number) {
		const x = Math.sin(i * 127.1 + j * 311.7) * 43758.5453;
		return x - Math.floor(x);
	}

	const sales: Sale[] = [];
	for (let i = 0; i < REGIONS.length; i++) {
		for (let j = 0; j < CHANNELS.length; j++) {
			const n = 1 + Math.floor(rnd(i, j) * 3);
			for (let k = 0; k < n; k++) {
				sales.push({
					region: REGIONS[i],
					channel: CHANNELS[j],
					amount: Math.round(rnd(i * 7 + k, j * 13 + k) * 90_000 + 10_000),
				});
			}
		}
	}

	// data prep happens in the pure fundament — the component only renders
	const bars: BarDatum[] = buildBars(
		sales,
		(d) => d.region,
		(d) => d.amount,
		{ topN: 5 }
	);

	const fmt = (v: number) => `${Math.round(v / 1000)}k`;
	const fmtFull = (v: number) =>
		new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', maximumFractionDigits: 0 }).format(v);

	let selected: BarDatum | null = $state(null);
</script>

<svelte:head>
	<title>Bar chart — Labs — Data Monster</title>
</svelte:head>

<div class="bar-page">
	<div class="section-header">
		<span class="section-number">LABS</span>
		<h1 class="section-title">Bar chart</h1>
	</div>

	<p class="section-subtitle">
		Aggregation via <code>buildBars</code> (top 5 + Other), rendering via svelteplot <code>BarY</code> — hover tooltip, click-to-select, mono chart labels, empty-data guard. Synthetic data.
	</p>

	<BarChart
		data={bars}
		category={(d: BarDatum) => d.category}
		value={(d: BarDatum) => d.value}
		formatY={fmt}
		labelFor={(d: BarDatum) => `${d.category} · ${fmtFull(d.value)}`}
		title="Revenue by region"
		subtitle="top 5 regions, rest lumped into Other — aggregated from raw sales rows"
		bind:selected
	>
		{#snippet tooltip(d)}
			<div class="font-semibold">{d.category}</div>
			<div>Revenue: {fmtFull(d.value)}</div>
			<div>Share: {Math.round((d.value / bars.reduce((s, b) => s + b.value, 0)) * 100)}%</div>
		{/snippet}
	</BarChart>
</div>

<style>
	.bar-page {
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
		margin: var(--space-3) 0 var(--space-4) 0;
		max-width: 64ch;
		line-height: var(--leading-relaxed);
	}

	.section-subtitle code {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-accent-dark);
	}
</style>
