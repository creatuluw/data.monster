<script lang="ts">
	import { LineChart, AreaChart } from 'layerchart';
	import { curveMonotoneX, curveLinear } from 'd3-shape';
	import type { LineAreaChartConfig, BarChartData } from './types';
	import { executeLineAreaChartQuery } from './engine/DataModelConnector';
	import { PALETTE, DIMMED, BLACK } from './chart-helpers';
	import { extractErrorMessage } from '$lib/db-operations';

	interface Props {
		config: LineAreaChartConfig;
		onPointClick?: (detail: { category: string; value: number; row: BarChartData }) => void;
	}

	let { config, onPointClick }: Props = $props();

	let data: BarChartData[] = $state([]);
	let selectedGroups: Set<string> = $state(new Set());
	let isLoading: boolean = $state(true);
	let error: string | null = $state(null);
	let chartWidth = $state(0);
	let chartHeight = $state(0);
	let bodyWrapperEl: HTMLDivElement | undefined = $state();

	let dimField = $derived(config.dimension.field);
	let isArea = $derived((config.chartType ?? 'line') === 'area');
	let clickToFilter = $derived(config.clickToFilter !== false);
	let lineColor = $derived(config.colors?.[0] ?? PALETTE[0]);

	let curveFactory = $derived(
		(config.curve ?? 'monotone') === 'monotone' ? curveMonotoneX : curveLinear
	);

	let colorRange = $derived.by(() => {
		const groups = [...new Set(data.map((d) => String(d[dimField] ?? '')))];
		const sel = selectedGroups;
		if (sel.size === 0) return groups.map(() => lineColor);
		return groups.map((g) => (sel.has(g) ? lineColor : DIMMED));
	});

	let tooltipConfig = $derived.by(() => {
		const base = { mode: 'band' as const };
		if (!clickToFilter) {
			if (onPointClick) {
				return {
					...base,
					onclick: (_e: MouseEvent, { data: clickedData }: { data: any }) => {
						const category = String(clickedData[dimField] ?? '');
						const value = Number(clickedData.value ?? 0);
						const row = data.find((r) => String(r[dimField]) === category);
						if (row) onPointClick({ category, value, row });
					}
				};
			}
			return base;
		}
		return {
			...base,
			onclick: (_e: MouseEvent, { data: clickedData }: { data: any }) => {
				const group = String(clickedData[dimField] ?? '');
				if (!group) return;
				const next = new Set(selectedGroups);
				if (next.has(group)) {
					next.delete(group);
				} else {
					next.add(group);
				}
				selectedGroups = next;
				if (onPointClick) {
					const row = data.find((r) => String(r[dimField]) === group);
					if (row) onPointClick({ category: group, value: Number(row.value) || 0, row });
				}
			}
		};
	});

	async function fetchData() {
		isLoading = true;
		error = null;
		selectedGroups = new Set();
		try {
			const result = await executeLineAreaChartQuery(config);
			data = result.rows;
		} catch (e: unknown) {
			error = extractErrorMessage(e);
			data = [];
		} finally {
			isLoading = false;
		}
	}

	$effect(() => {
		void config.table;
		void config.dimension.field;
		void config.metric.field;
		void config.metric.aggregate;
		void config.sortDirection;
		void config.limit;
		void config.chartType;
		fetchData();
	});

	$effect(() => {
		const el = bodyWrapperEl;
		if (!el) return;
		const measure = () => {
			const r = el.getBoundingClientRect();
			chartWidth = r.width;
			chartHeight = r.height;
		};
		measure();
		const observer = new ResizeObserver(() => measure());
		observer.observe(el);
		return () => observer.disconnect();
	});

	function removeGroup(group: string) {
		const next = new Set(selectedGroups);
		next.delete(group);
		selectedGroups = next;
	}

	function clearAll() {
		selectedGroups = new Set();
	}

	let title = $derived.by(() => {
		const agg = config.metric.aggregate ?? 'SUM';
		const metricLabel = config.metric.label ?? config.metric.field;
		const dimLabel = config.dimension.label ?? config.dimension.field;
		const typeLabel = isArea ? 'Area' : 'Line';
		return `${typeLabel}: ${agg} of ${metricLabel} by ${dimLabel}`;
	});

	let selectedCount = $derived(selectedGroups.size);
</script>

<div class="chart-shell">
	<div class="chip-row">
		{#if selectedCount > 0}
			{#each [...selectedGroups] as group (group)}
				<button class="filter-chip" onclick={() => removeGroup(group)}>
					{group}
					<span class="chip-x">&times;</span>
				</button>
			{/each}
			<button class="clear-all-btn" onclick={clearAll}>Clear all</button>
		{:else}
			<span class="chip-skeleton">Click points to filter</span>
		{/if}
	</div>
	<div class="chart-header">
		<h3 class="chart-title-text">{title}</h3>
	</div>

	<div class="chart-body" bind:this={bodyWrapperEl}>
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
		{:else if chartWidth > 0 && chartHeight > 0}
			{#key colorRange}
				{#if isArea}
					<AreaChart
						data={data}
						x={dimField}
						y="value"
						c={dimField}
						cRange={colorRange}
						width={chartWidth}
						height={chartHeight}
						padding={{ top: 16, right: 24, bottom: 48, left: 64 }}
						motion="tween"
						tooltipContext={tooltipConfig}
						points={(config.showDots ?? true)}
						props={{ area: { curve: curveFactory, strokeWidth: 2 } }}
					/>
				{:else}
					<LineChart
						data={data}
						x={dimField}
						y="value"
						c={dimField}
						cRange={colorRange}
						width={chartWidth}
						height={chartHeight}
						padding={{ top: 16, right: 24, bottom: 48, left: 64 }}
						motion="tween"
						tooltipContext={tooltipConfig}
						points={(config.showDots ?? true)}
						props={{ spline: { curve: curveFactory, strokeWidth: 2 } }}
					/>
				{/if}
			{/key}
		{/if}
	</div>
</div>

<style>
	.chart-shell {
		background: var(--color-surface);
		border: none;
		border-radius: 0;
		height: 100%;
		display: flex;
		flex-direction: column;
		min-height: 0;
	}

	.chip-row {
		display: flex;
		flex-wrap: nowrap;
		gap: var(--space-2);
		align-items: center;
		padding: 0 var(--space-6);
		height: 28px;
		flex-shrink: 0;
		overflow-x: auto;
	}

	.chip-skeleton {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		line-height: 24px;
		letter-spacing: 0.02em;
		color: var(--color-text-tertiary);
		padding: 0 var(--space-2);
		border: 1px dashed var(--color-border);
		border-radius: var(--radius-xs);
		opacity: 0.6;
	}

	.filter-chip {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		padding: 0 var(--space-2);
		height: 24px;
		line-height: 1;
		border: 1px solid var(--color-accent-light);
		border-radius: var(--radius-xs);
		background: var(--color-accent-muted);
		color: var(--color-accent-dark);
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		cursor: pointer;
		transition: background var(--duration-fast) ease, border-color var(--duration-fast) ease;
		white-space: nowrap;
	}

	.filter-chip:hover {
		background: oklch(0.88 0.03 41);
	}

	.chip-x {
		font-weight: 600;
		opacity: 0.6;
	}

	.clear-all-btn {
		padding: 0 var(--space-2);
		height: 24px;
		line-height: 1;
		border: 1px solid transparent;
		border-radius: var(--radius-xs);
		background: transparent;
		color: var(--color-text-tertiary);
		font-family: var(--font-body);
		font-size: var(--text-xs);
		cursor: pointer;
		white-space: nowrap;
		transition: color var(--duration-fast) ease;
	}

	.clear-all-btn:hover {
		color: var(--color-text);
	}

	.chart-header {
		display: flex;
		align-items: center;
		gap: var(--space-3);
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
		white-space: nowrap;
	}

	.chart-body {
		flex: 1;
		min-height: 0;
		padding: var(--space-4);
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

	.spinner {
		width: 20px;
		height: 20px;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}
</style>
