<script lang="ts">
	import { onMount } from 'svelte';
	import { app } from '$lib/stores/app.svelte';
	import { getTableMeta, extractErrorMessage } from '$lib/db-operations';
	import type { ColumnInfo } from '$lib/db-operations';
	import { BarChartCanvas } from '$lib/charts';
	import { PicassoBarChartCanvas } from '$lib/charts';
	import type { BarChartConfig } from '$lib/charts/types';
	import { Plot, BarX } from 'svelteplot';
	import { BarChart2, LineChart, PieChart, AreaChart, ScatterChart } from 'lucide-svelte';

	const NUMERIC_TYPES = new Set([
		'INTEGER', 'BIGINT', 'SMALLINT', 'TINYINT', 'INT', 'INT2', 'INT4', 'INT8',
		'DOUBLE', 'FLOAT', 'FLOAT4', 'FLOAT8', 'REAL', 'DECIMAL', 'NUMERIC',
		'HUGEINT', 'UINTEGER', 'UBIGINT', 'USMALLINT', 'UTINYINT'
	]);

	const SAMPLE_DATA = [
		{ category: 'Electronics', revenue: 48200 },
		{ category: 'Clothing', revenue: 32100 },
		{ category: 'Food & Beverage', revenue: 27800 },
		{ category: 'Home & Garden', revenue: 19400 },
		{ category: 'Sports', revenue: 15600 },
		{ category: 'Books', revenue: 12300 },
		{ category: 'Toys', revenue: 8900 },
		{ category: 'Music', revenue: 5400 }
	];

	type ChartTypeDef = {
		id: string;
		label: string;
		desc: string;
		icon: typeof BarChart2;
		status: 'available' | 'planned';
	};

	const chartTypes: ChartTypeDef[] = [
		{ id: 'bar', label: 'Bar Chart', desc: 'Compare values across categories with horizontal or vertical bars', icon: BarChart2, status: 'available' },
		{ id: 'line', label: 'Line Chart', desc: 'Show trends and continuity over ordered dimensions like time', icon: LineChart, status: 'planned' },
		{ id: 'area', label: 'Area Chart', desc: 'Emphasize volume under a trend line with filled regions', icon: AreaChart, status: 'planned' },
		{ id: 'pie', label: 'Pie / Donut', desc: 'Show part-to-whole proportions for a small number of slices', icon: PieChart, status: 'planned' },
		{ id: 'scatter', label: 'Scatter Plot', desc: 'Reveal correlation between two numeric measures', icon: ScatterChart, status: 'planned' },
	];

	let selectedChart = $state('bar');

	let dbReady = $state(false);
	let layerConfig: BarChartConfig | null = $state(null);
	let picassoConfig: BarChartConfig | null = $state(null);

	async function autoConfig() {
		if (app.tables.length === 0) return;

		const table = app.tables[0];
		try {
			const meta = await getTableMeta(table);
			const dimCol = meta.columns.find((c) => !NUMERIC_TYPES.has(c.type.toUpperCase()));
			const metCol = meta.columns.find((c) => NUMERIC_TYPES.has(c.type.toUpperCase()));
			if (!dimCol || !metCol) return;

			const cfg: BarChartConfig = {
				id: 'showcase',
				table,
				dimension: { field: dimCol.name, label: dimCol.name },
				metric: { field: metCol.name, label: metCol.name, aggregate: 'SUM' },
				orientation: 'vertical',
				sortDirection: 'DESC',
				limit: 10,
				clickToFilter: false
			};
			layerConfig = cfg;
			picassoConfig = { ...cfg, id: 'showcase-picasso' };
			dbReady = true;
		} catch (e) {
			dbReady = false;
		}
	}

	onMount(() => {
		autoConfig();
	});
</script>

<svelte:head>
	<title>Chart Showcase — Data Monster Labs</title>
</svelte:head>

<div class="showcase">
	<div class="showcase-header">
		<span class="section-number">CHART SHOWCASE</span>
	</div>

	<div class="showcase-body">
		<nav class="chart-nav">
			{#each chartTypes as ct (ct.id)}
				<button
					class="nav-item"
					class:nav-item-active={selectedChart === ct.id}
					class:nav-item-disabled={ct.status === 'planned'}
					onclick={() => { if (ct.status === 'available') selectedChart = ct.id; }}
					disabled={ct.status === 'planned'}
				>
					<ct.icon size={16} />
					<span class="nav-item-label">{ct.label}</span>
					{#if ct.status === 'planned'}
						<span class="nav-badge">Soon</span>
					{/if}
				</button>
			{/each}
		</nav>

		{#if selectedChart === 'bar'}
			<div class="chart-gallery">
				<div class="gallery-card">
					<div class="gallery-card-header">
						<div class="gallery-card-title-row">
							<span class="gallery-lib">LayerChart</span>
							<a href="/labs/chart-lib/bar" class="gallery-link">Interactive &rarr;</a>
						</div>
						<span class="gallery-render">SVG &middot; D3 grammar-of-graphics &middot; Svelte reactivity</span>
					</div>
					<div class="gallery-card-body">
						{#if dbReady && layerConfig}
							<BarChartCanvas config={layerConfig} />
						{:else}
							<div class="gallery-placeholder">
								<span>Connect a database to preview</span>
							</div>
						{/if}
					</div>
				</div>

				<div class="gallery-card">
					<div class="gallery-card-header">
						<div class="gallery-card-title-row">
							<span class="gallery-lib">Picasso.js</span>
							<a href="/labs/picasso-charts/bar" class="gallery-link">Interactive &rarr;</a>
						</div>
						<span class="gallery-render">Canvas &middot; Qlik declarative model &middot; built-in brushing</span>
					</div>
					<div class="gallery-card-body">
						{#if dbReady && picassoConfig}
							<PicassoBarChartCanvas config={picassoConfig} />
						{:else}
							<div class="gallery-placeholder">
								<span>Connect a database to preview</span>
							</div>
						{/if}
					</div>
				</div>

				<div class="gallery-card">
					<div class="gallery-card-header">
						<div class="gallery-card-title-row">
							<span class="gallery-lib">SveltePlot</span>
							<a href="/labs/svelteplot-charts/bar" class="gallery-link">Interactive &rarr;</a>
						</div>
						<span class="gallery-render">SVG &middot; Observable Plot &middot; idiomatic Svelte</span>
					</div>
					<div class="gallery-card-body">
						<Plot y={SAMPLE_DATA.map((d) => d.category)}>
							<BarX data={SAMPLE_DATA} x="revenue" y="category" />
						</Plot>
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.showcase {
		display: flex;
		flex-direction: column;
		height: 100%;
		overflow: hidden;
	}

	.showcase-header {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
		height: 41px;
		box-sizing: border-box;
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

	.showcase-body {
		flex: 1;
		min-height: 0;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
	}

	.chart-nav {
		display: flex;
		gap: var(--space-1);
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
		overflow-x: auto;
	}

	.nav-item {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		border: 1px solid transparent;
		border-radius: var(--radius-xs);
		background: none;
		font-family: var(--font-display);
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-text);
		cursor: pointer;
		white-space: nowrap;
		transition: all var(--duration-fast) ease;
	}

	.nav-item:hover:not(:disabled) {
		background: var(--color-surface-sunken);
		border-color: var(--color-border);
	}

	.nav-item:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	.nav-item-active {
		background: var(--color-accent-muted);
		border-color: var(--color-accent-muted);
		color: var(--color-accent);
	}

	.nav-item-disabled {
		color: var(--color-text-tertiary);
		opacity: 0.5;
		cursor: default;
	}

	.nav-item-label {
		letter-spacing: -0.01em;
	}

	.nav-badge {
		font-family: var(--font-mono);
		font-size: 8px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		padding: 1px 4px;
		border-radius: var(--radius-xs);
		background: var(--color-surface-sunken);
		color: var(--color-text-tertiary);
	}

	.chart-gallery {
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
		padding: var(--space-6) var(--space-6);
	}

	.gallery-card {
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		overflow: hidden;
		background: var(--color-surface);
	}

	.gallery-card-header {
		display: flex;
		flex-direction: column;
		gap: 2px;
		padding: var(--space-4) var(--space-5);
		border-bottom: 1px solid var(--color-border);
	}

	.gallery-card-title-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.gallery-lib {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 700;
		letter-spacing: -0.01em;
	}

	.gallery-link {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.02em;
		color: var(--color-accent);
		text-decoration: none;
		transition: opacity var(--duration-fast) ease;
	}

	.gallery-link:hover {
		opacity: 0.75;
	}

	.gallery-render {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.02em;
	}

	.gallery-card-body {
		min-height: 280px;
		position: relative;
	}

	.gallery-card-body :global(.chart-shell) {
		border: none;
		border-radius: 0;
	}

	.gallery-placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 280px;
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}
</style>
