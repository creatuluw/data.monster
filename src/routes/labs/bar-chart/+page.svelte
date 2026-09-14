<script lang="ts">
	import BarChart from '$lib/components/charts/BarChart.svelte';

	// mirrors /labs/heatmap page logic: everything inline, only the chart
	// component imported — no $lib/charts imports from the page

	type Bar = { category: string; value: number };

	// deterministic pseudo-random — stable across reloads
	function rnd(i: number) {
		const x = Math.sin(i * 127.1 + 311.7) * 43758.5453;
		return x - Math.floor(x);
	}

	// aggregate synthetic sales into bars: top 5 + Other (kept last)
	const REGIONS = ['EU', 'US', 'APAC', 'LATAM', 'MEA', 'CA', 'OCE', 'NORDICS'];
	const sums = new Map<string, number>();
	for (let i = 0; i < REGIONS.length; i++) {
		for (let j = 0; j < 3; j++) {
			sums.set(REGIONS[i], (sums.get(REGIONS[i]) ?? 0) + Math.round(rnd(i * 7 + j) * 90_000 + 10_000));
		}
	}
	const sorted = [...sums.entries()].map(([category, value]) => ({ category, value })).sort((a, b) => b.value - a.value);
	const bars: Bar[] = [...sorted.slice(0, 5), { category: 'Other', value: sorted.slice(5).reduce((s, b) => s + b.value, 0) }];
	const total = bars.reduce((s, b) => s + b.value, 0);

	const fmt = (v: number) => `${Math.round(v / 1000)}k`;
	const fmtFull = (v: number) =>
		new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', maximumFractionDigits: 0 }).format(v);

	let selected: Bar | null = $state(null);
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
		svelteplot BarY — hover tooltip, click-to-select, mono chart labels, empty-data guard. Top 5 regions + Other, aggregated from synthetic sales. Same interaction model as the heatmap.
	</p>

	<BarChart
		data={bars}
		category={(d: Bar) => d.category}
		value={(d: Bar) => d.value}
		formatY={fmt}
		labelFor={(d: Bar) => `${d.category} · ${fmtFull(d.value)}`}
		title="Revenue by region"
		subtitle="top 5 regions, rest lumped into Other"
		bind:selected
	>
		{#snippet tooltip(d)}
			<div class="font-semibold">{d.category}</div>
			<div>Revenue: {fmtFull(d.value)}</div>
			<div>Share: {Math.round((d.value / total) * 100)}%</div>
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
</style>
