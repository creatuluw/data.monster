<script lang="ts">
	import BarChart from '$lib/components/charts/BarChart.svelte';

	type Row = { category: string; value: number };

	// deterministic pseudo-random from grid coords — stable across reloads, no seed state
	function rnd(i: number, j: number) {
		const x = Math.sin(i * 127.1 + j * 311.7) * 43758.5453;
		return x - Math.floor(x);
	}

	// 12 months ahead of this month, inclusive
	const now = new Date();
	const monthLabels = Array.from({ length: 12 }, (_, i) =>
		new Date(now.getFullYear(), now.getMonth() + i, 1).toLocaleDateString('en-US', {
			month: 'short',
			year: '2-digit',
		}),
	);

	const CAPACITY = 36; // hours per week

	// one dimension (month) + one measure (booked hours in the month)
	const rows: Row[] = monthLabels
		.map((category, m) => ({
			category,
			value: Math.round((0.2 + rnd(m, 60) * 1.1) * CAPACITY * 4),
		}))
		// desc on the measure — component renders domain[0] topmost, so the
		// longest bar lands at the top
		.sort((a, b) => b.value - a.value);

	let selected: Row | null = $state(null);

	// live-configurable BarChart props — mirrored into the config drawer
	let chartTitle = $state('Booked hours per month');
	let chartSubtitle = $state('synthetic utilization × 4-week capacity');
	let barColor = $state('#888888');
	let heightVh = $state(0.3);
</script>

<svelte:head>
	<title>Bar chart — Labs — Data Monster</title>
</svelte:head>

<div class="bar-page">
	<div class="section-header">
		<h1 class="section-title">Bar chart</h1>
	</div>

	<BarChart
		data={rows}
		category={(d: Row) => d.category}
		value={(d: Row) => d.value}
		color={barColor}
		{heightVh}
		title={chartTitle}
		subtitle={chartSubtitle}
		labelFor={(d: Row) => `${d.category} · ${d.value}h`}
		bind:selected
	>
		{#snippet config()}
			<div class="field">
				<label class="field-label" for="cfg-title">title — string</label>
				<input class="input" id="cfg-title" type="text" bind:value={chartTitle} />
			</div>
			<div class="field">
				<label class="field-label" for="cfg-subtitle">subtitle — string</label>
				<input class="input" id="cfg-subtitle" type="text" bind:value={chartSubtitle} />
			</div>
			<div class="field">
				<label class="field-label" for="cfg-color">color — hex</label>
				<input class="input color-input" id="cfg-color" type="color" bind:value={barColor} />
				<span class="field-hint">deselected bar fill; selected stays DS green</span>
			</div>
			<div class="field">
				<label class="field-label" for="cfg-height">heightVh — viewport fraction</label>
				<input class="range-input" id="cfg-height" type="range" min="0.1" max="0.6" step="0.05" bind:value={heightVh} />
				<span class="field-hint">{Math.round(heightVh * 100)}vh</span>
			</div>
		{/snippet}
		{#snippet tooltip(d)}
			<div class="font-semibold">{d.category}</div>
			<div>Booked hours: {d.value}h</div>
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

	.section-title {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: 700;
		letter-spacing: -0.02em;
		margin: 0;
	}


	.color-input {
		padding: 2px;
		height: 36px;
		cursor: pointer;
	}

	.range-input {
		accent-color: var(--color-accent);
	}
</style>
