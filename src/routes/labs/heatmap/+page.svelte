<script lang="ts">
	import Heatmap from '$lib/components/Heatmap.svelte';

	type Row = { monthOffset: number; week: number; utilization: number; hours: number; tasks: number };
	type ActivityRow = { monthOffset: number; category: number; hours: number; tasks: number };

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
	const MONTH_FACTOR = 52 / 12;

	// heatmap 1: weekly utilization grid — x = month, y = week-of-month
	const rows: Row[] = [];
	for (let m = 0; m < 12; m++) {
		for (let w = 1; w <= 5; w++) {
			const empty = rnd(m, w) < 0.08;
			const utilization = empty ? 0 : 0.2 + rnd(m, w + 50) * 1.1;
			const hours = Math.round(utilization * CAPACITY);
			rows.push({ monthOffset: m, week: w, utilization, hours, tasks: Math.round(hours / 3.5) });
		}
	}

	// heatmap 2: activity mix — x = month, y = category
	const categoryLabels = ['Project', 'Process', 'Self-initiated', 'Recurring', '/help', 'Ad-hoc'];
	const weights = [0.55, 0.12, 0.08, 0.1, 0.06, 0.09];
	const activityRows: ActivityRow[] = [];
	for (let m = 0; m < 12; m++) {
		for (let c = 0; c < categoryLabels.length; c++) {
			const monthlyCap = CAPACITY * MONTH_FACTOR;
			const jitter = 0.75 + rnd(m + 10, c + 20) * 0.5;
			const hours = c === 4 ? 0.05 * monthlyCap : weights[c] * monthlyCap * jitter;
			activityRows.push({ monthOffset: m, category: c, hours, tasks: Math.round(hours / 4) });
		}
	}

	let selected: Row | null = $state(null);
	let activitySelected: ActivityRow | null = $state(null);
	$effect(() => {
		if (selected) activitySelected = null;
	});
	$effect(() => {
		if (activitySelected) selected = null;
	});
</script>

<svelte:head>
	<title>Heatmap — Labs — Data Monster</title>
</svelte:head>

<div class="heatmap-page">
	<div class="section-header">
		<span class="section-number">LABS</span>
		<h1 class="section-title">Heatmap</h1>
	</div>

	<p class="section-subtitle">
		Threshold-colored grid on svelteplot — in-cell labels, hover tooltips, click-to-select, no-data cells, axis flip on narrow widths. Synthetic data.
	</p>

	<Heatmap
		data={rows}
		x={(d: Row) => d.monthOffset}
		y={(d: Row) => d.week}
		value={(d: Row) => d.utilization}
		threshold={[0.5, 0.75, 0.9, 1.0, 1.25]}
		xTicks={[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]}
		formatX={(i) => monthLabels[i]}
		yTicks={[1, 2, 3, 4, 5]}
		formatY={(w) => `W${w}`}
		isEmpty={(d: Row) => d.tasks === 0}
		label={(d: Row) => `${Math.round(d.utilization * 100)}%`}
		labelFor={(d: Row) => `${monthLabels[d.monthOffset]} · Week ${d.week} · ${Math.round(d.utilization * 100)}%`}
		title="Utilization per week"
		subtitle="booked hours divided by weekly capacity"
		bind:selected
	>
		{#snippet tooltip(d)}
			<div class="font-semibold">{monthLabels[d.monthOffset]} · Week {d.week}</div>
			{#if d.tasks === 0}
				<div class="opacity-75">No tasks this week</div>
			{:else}
				<div>Utilization: {Math.round(d.utilization * 100)}%</div>
				<div>Hours: {d.hours}u / {CAPACITY}u</div>
				<div>Tasks: {d.tasks}</div>
			{/if}
		{/snippet}
	</Heatmap>

	<Heatmap
		data={activityRows}
		x={(d: ActivityRow) => d.monthOffset}
		y={(d: ActivityRow) => d.category}
		value={(d: ActivityRow) => d.hours}
		threshold={[0.5, 0.75, 0.9, 1.0, 1.25].map((f) => f * CAPACITY * MONTH_FACTOR * 0.55)}
		xTicks={[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]}
		formatX={(i) => monthLabels[i]}
		yTicks={[0, 1, 2, 3, 4, 5]}
		formatY={(c) => categoryLabels[c]}
		isEmpty={(d: ActivityRow) => d.tasks === 0 && d.category !== 4}
		label={(d: ActivityRow) => `${Math.round(d.hours)}u`}
		labelFor={(d: ActivityRow) =>
			`${monthLabels[d.monthOffset]} · ${categoryLabels[d.category]} · ${Math.round(d.hours)}u`}
		title="Activity per month"
		subtitle="sum of hours per activity type"
		bind:selected={activitySelected}
	>
		{#snippet tooltip(d)}
			<div class="font-semibold">{monthLabels[d.monthOffset]} · {categoryLabels[d.category]}</div>
			<div>Hours: {Math.round(d.hours)}u</div>
			<div>Tasks: {d.tasks}</div>
		{/snippet}
	</Heatmap>
</div>

<style>
	.heatmap-page {
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
