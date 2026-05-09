<script lang="ts">
	import * as Plot from '@observablehq/plot';
	import type { BarChartConfig, BarChartData } from './types';
	import { executeBarChartQuery } from './engine/DataModelConnector';
	import { PALETTE, DIMMED } from './chart-helpers';
	import { extractErrorMessage } from '$lib/db-operations';

	interface Props {
		config: BarChartConfig;
	}

	let { config }: Props = $props();

	let data: BarChartData[] = $state([]);
	let selectedGroups: Set<string> = $state(new Set());
	let isLoading: boolean = $state(true);
	let error: string | null = $state(null);
	let containerEl: HTMLDivElement | undefined = $state();

	let dimField = $derived(config.dimension.field);
	let isHorizontal = $derived((config.orientation ?? 'vertical') === 'horizontal');

	let title = $derived.by(() => {
		const agg = config.metric.aggregate ?? 'SUM';
		const metricLabel = config.metric.label ?? config.metric.field;
		const dimLabel = config.dimension.label ?? config.dimension.field;
		return `${agg} of ${metricLabel} by ${dimLabel}`;
	});

	async function fetchData() {
		isLoading = true;
		error = null;
		try {
			const result = await executeBarChartQuery(config);
			data = result.rows;
		} catch (e: unknown) {
			error = extractErrorMessage(e);
			data = [];
		} finally {
			isLoading = false;
		}
	}

	function plotData(): Array<{ category: string; value: number }> {
		return data.map((d) => ({
			category: String(d[dimField] ?? ''),
			value: typeof d.value === 'number' ? d.value : (Number(d.value) || 0)
		}));
	}

	function renderPlot() {
		if (!containerEl) return;
		if (isLoading || error || data.length === 0) return;

		const sel = selectedGroups;
		const isSel = sel.size > 0;

		const pd = plotData();
		const sortDir = config.sortDirection ?? 'DESC';

		const marks: any[] = [];

		if (isHorizontal) {
			marks.push(
				Plot.barX(pd, {
					y: 'category',
					x: 'value',
					fill: (d: { category: string }) => {
						if (!isSel) return PALETTE[0];
						const idx = pd.findIndex((r) => r.category === d.category);
						return sel.has(d.category) ? PALETTE[idx % PALETTE.length] : DIMMED;
					},
					sort: { y: sortDir === 'ASC' ? 'x' : '-x' },
					tip: true
				})
			);
		} else {
			marks.push(
				Plot.barY(pd, {
					x: 'category',
					y: 'value',
					fill: (d: { category: string }) => {
						if (!isSel) return PALETTE[0];
						const idx = pd.findIndex((r) => r.category === d.category);
						return sel.has(d.category) ? PALETTE[idx % PALETTE.length] : DIMMED;
					},
					sort: { x: sortDir === 'ASC' ? 'y' : '-y' },
					tip: true
				})
			);
		}

		containerEl.firstChild?.remove();

		const plot = Plot.plot({
			marks,
			marginLeft: 80,
			marginBottom: 60,
			style: {
				fontFamily: 'var(--font-mono)',
				fontSize: '11px',
				color: 'var(--color-text-secondary)',
				background: 'transparent',
				overflow: 'visible'
			}
		});

		containerEl.append(plot);

		if (config.clickToFilter !== false) {
			const rects = plot.querySelectorAll<SVGRectElement>('[aria-label="bar"]');
			rects.forEach((rect) => {
				const cat = pd[Array.from(rect.parentElement?.parentElement?.children ?? [])
					.indexOf(rect.closest('g')!) % pd.length]?.category;
				if (!cat) return;
				rect.style.cursor = 'pointer';
				rect.addEventListener('click', () => {
					const next = new Set(selectedGroups);
					if (next.has(cat)) {
						next.delete(cat);
					} else {
						next.add(cat);
					}
					selectedGroups = next;
				});
			});
		}
	}

	$effect(() => {
		void config.table;
		void config.dimension.field;
		void config.metric.field;
		void config.metric.aggregate;
		void config.sortDirection;
		void config.limit;
		void config.orientation;
		fetchData();
	});

	$effect(() => {
		void data.length;
		void selectedGroups.size;
		renderPlot();
	});

	function removeGroup(group: string) {
		const next = new Set(selectedGroups);
		next.delete(group);
		selectedGroups = next;
	}

	function clearAll() {
		selectedGroups = new Set();
	}
</script>

<div class="chart-shell">
	<div class="chart-header">
		<h3 class="chart-title-text">{title}</h3>
	</div>

	{#if selectedGroups.size > 0}
		<div class="filter-chips">
			{#each [...selectedGroups] as group (group)}
				<button class="filter-chip" onclick={() => removeGroup(group)}>
					{group}
					<span class="chip-x">&times;</span>
				</button>
			{/each}
			<button class="clear-all-btn" onclick={clearAll}>Clear all</button>
		</div>
	{/if}

	<div class="chart-body" bind:this={containerEl}>
		{#if isLoading}
			<div class="chart-state">
				<svg class="spinner" viewBox="0 0 24 24" fill="none">
					<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
					<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
				</svg>
				<span>Loading chart data...</span>
			</div>
		{:else if error}
			<div class="chart-state chart-error">
				<span>Query error: {error}</span>
			</div>
		{:else if data.length === 0}
			<div class="chart-state">
				<span>No data available</span>
			</div>
		{/if}
	</div>
</div>

<style>
	.chart-shell {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-lg);
		height: 100%;
		display: flex;
		flex-direction: column;
		min-height: 0;
	}

	.chart-header {
		padding: var(--space-4) var(--space-6) 0;
		flex-shrink: 0;
	}

	.chart-title-text {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text-secondary);
		letter-spacing: -0.01em;
		margin: 0;
	}

	.filter-chips {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-2);
		padding: var(--space-3) var(--space-6);
		flex-shrink: 0;
	}

	.filter-chip {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 2px var(--space-2);
		border: 1px solid var(--color-accent-light);
		border-radius: var(--radius-xs);
		background: var(--color-accent-muted);
		color: var(--color-accent-dark);
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		cursor: pointer;
		transition: background var(--duration-fast) ease, border-color var(--duration-fast) ease;
	}

	.filter-chip:hover {
		background: oklch(0.88 0.03 41);
	}

	.chip-x {
		font-weight: 600;
		opacity: 0.6;
	}

	.clear-all-btn {
		padding: 2px var(--space-2);
		border: 1px solid transparent;
		border-radius: var(--radius-xs);
		background: transparent;
		color: var(--color-text-tertiary);
		font-family: var(--font-body);
		font-size: var(--text-xs);
		cursor: pointer;
		transition: color var(--duration-fast) ease;
	}

	.clear-all-btn:hover {
		color: var(--color-text);
	}

	.chart-body {
		flex: 1;
		min-height: 0;
		padding: var(--space-4);
		overflow: hidden;
	}

	.chart-body :global(svg) {
		overflow: visible;
		width: 100%;
		height: 100%;
	}

	.chart-state {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-3);
		height: 100%;
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}

	.chart-error {
		color: var(--color-danger);
	}
</style>
