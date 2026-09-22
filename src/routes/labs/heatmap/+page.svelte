<script lang="ts">
	import HeatmapRenderer from '$lib/components/charts/renderers/HeatmapRenderer.svelte';
import Section from '$lib/components/charts/controls/Section.svelte';
import Field from '$lib/components/charts/controls/Field.svelte';
import Select from '$lib/components/charts/controls/Select.svelte';
	import { setupChartRegistry } from '$lib/charts/registry-setup.svelte';

	setupChartRegistry();

	// engine-shaped rows: two dimensions (month, week) + one measure (utilization)
	type Row = { month: string; week: string; utilization: number };

	function rnd(i: number, j: number) {
		const x = Math.sin(i * 127.1 + j * 311.7) * 43758.5453;
		return x - Math.floor(x);
	}

	const now = new Date();
	const monthLabels = Array.from({ length: 12 }, (_, i) =>
		new Date(now.getFullYear(), now.getMonth() + i, 1).toLocaleDateString('en-US', {
			month: 'short',
			year: '2-digit',
		}),
	);

	const weeks = ['W1', 'W2', 'W3', 'W4', 'W5'];

	// holes (no rows) render as no-data cells in the renderer's full grid
	const rows: Row[] = [];
	for (let m = 0; m < 12; m++) {
		for (let w = 0; w < weeks.length; w++) {
			if (rnd(m, w + 1) < 0.08) continue; // no-data week
			const utilization = 0.2 + rnd(m, w + 50) * 1.1;
			rows.push({ month: monthLabels[m], week: weeks[w], utilization });
		}
	}

	let selected = $state<{ dimension: string; value: string } | null>(null);
	let scheme = $state<'orrd' | 'blues' | 'greens'>('orrd');
	let heightVh = $state(0.3);
</script>

<svelte:head>
	<title>Heatmap — Labs — Data Monster</title>
</svelte:head>

<div class="heatmap-page">
	<div class="section-header">
		<h1 class="section-title">Heatmap</h1>
	</div>

	<p class="section-subtitle">
		Registry entry <code>heatmap</code> — pivot hook grid, threshold colors from the data max, no-data holes, click-to-select. Synthetic data.
	</p>

	<HeatmapRenderer
		{rows}
		dimensionAliases={['month', 'week']}
		measureAliases={['utilization']}
		options={{ scheme, threshold: 5 }}
		annotations={[]}
		title="Utilization per week"
		subtitle="booked hours divided by weekly capacity"
		tooltip={{ template: '{month} · {week}: {utilization}' }}
		{selected}
		onSelect={(s) => (selected = s)}
		colorScale={{ colorOf: () => '#888888' }}
		fmts={{ utilization: (v) => `${Math.round(Number(v) * 100)}%` }}
		{heightVh}
	>
		{#snippet config()}
			<Section>
				<Field label="Scheme">
					<Select bind:value={scheme}>
						<option value="orrd">orrd</option>
						<option value="blues">blues</option>
						<option value="greens">greens</option>
					</Select>
				</Field>
				<Field label="Height" hint="viewport fraction">
					<input class="range-input" type="range" min="0.1" max="0.6" step="0.05" bind:value={heightVh} />
					<span class="field-hint">{Math.round(heightVh * 100)}vh</span>
				</Field>
			</Section>
		{/snippet}
	</HeatmapRenderer>
</div>

<style>
	.heatmap-page {
		max-width: 48rem;
		margin: 0 auto;
		padding: 2rem 1.5rem 4rem;
	}

	.section-header {
		margin-bottom: 1rem;
	}

	.section-title {
		font-family: var(--font-display);
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--color-text);
	}

	.section-subtitle {
		color: #71717a;
		font-size: 0.875rem;
		margin-bottom: 1.5rem;
	}

	.code {
		font-family: var(--font-mono);
		font-size: 0.8em;
	}
</style>
