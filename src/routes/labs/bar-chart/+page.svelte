<script lang="ts">
	import BarChartRenderer from '$lib/components/charts/renderers/BarChartRenderer.svelte';
import Section from '$lib/components/charts/controls/Section.svelte';
import Field from '$lib/components/charts/controls/Field.svelte';
import Select from '$lib/components/charts/controls/Select.svelte';
import TextInput from '$lib/components/charts/controls/TextInput.svelte';
	import { setupChartRegistry } from '$lib/charts/registry-setup.svelte';

	setupChartRegistry();

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

	// engine-shaped rows: one dimension (month) + one measure (booked hours)
	const rows: Row[] = monthLabels
		.map((category, m) => ({
			category,
			value: Math.round((0.2 + rnd(m, 60) * 1.1) * CAPACITY * 4),
		}))
		.sort((a, b) => b.value - a.value);

	// registry selection model: {dimension, value}, not the row object
	let selected = $state<{ dimension: string; value: string } | null>(null);

	let chartTitle = $state('Booked hours per month');
	let chartSubtitle = $state('synthetic utilization × 4-week capacity');
	let orientation = $state<'horizontal' | 'vertical'>('horizontal');
	let heightVh = $state(0.3);
</script>

<svelte:head>
	<title>Bar chart — Labs — Data Monster</title>
</svelte:head>

<div class="bar-page">
	<div class="section-header">
		<h1 class="section-title">Bar chart</h1>
	</div>

	<p class="section-subtitle">
		Registry entry <code>bar</code> on the chart fundament — thin renderer, engine-shaped rows, cross-filter selection model. Synthetic data.
	</p>

	<BarChartRenderer
		{rows}
		dimensionAliases={['category']}
		measureAliases={['value']}
		options={{ orientation }}
		annotations={[]}
		title={chartTitle}
		subtitle={chartSubtitle}
		tooltip={{ template: '{category}: {value}h' }}
		{selected}
		onSelect={(s) => (selected = s)}
		colorScale={{ colorOf: () => '#888888' }}
		fmts={{ value: (v) => `${v}h` }}
		{heightVh}
	>
		{#snippet config()}
			<Section>
				<Field label="Title">
					<TextInput bind:value={chartTitle} />
				</Field>
				<Field label="Subtitle">
					<TextInput bind:value={chartSubtitle} />
				</Field>
				<Field label="Orientation">
					<Select bind:value={orientation}>
						<option value="horizontal">horizontal</option>
						<option value="vertical">vertical</option>
					</Select>
				</Field>
				<Field label="Height" hint="viewport fraction">
					<input class="range-input" type="range" min="0.1" max="0.6" step="0.05" bind:value={heightVh} />
					<span class="field-hint">{Math.round(heightVh * 100)}vh</span>
				</Field>
			</Section>
		{/snippet}
	</BarChartRenderer>
</div>

<style>
	.bar-page {
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
