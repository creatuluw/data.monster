<script lang="ts">
	import { PieChart } from 'layerchart';
	import type { PieDonutChartConfig, BarChartData } from './types';
	import { executePieDonutChartQuery } from './engine/DataModelConnector';
	import { PALETTE } from './chart-helpers';
	import { extractErrorMessage } from '$lib/db-operations';

	interface Props {
		config: PieDonutChartConfig;
		onSliceClick?: (detail: { category: string; value: number; row: BarChartData }) => void;
	}

	let { config, onSliceClick }: Props = $props();

	let data: BarChartData[] = $state([]);
	let selectedGroups: Set<string> = $state(new Set());
	let isLoading: boolean = $state(true);
	let error: string | null = $state(null);
	let chartWidth = $state(0);
	let chartHeight = $state(0);
	let bodyWrapperEl: HTMLDivElement | undefined = $state();

	let dimField = $derived(config.dimension.field);
	let isDonut = $derived((config.chartType ?? 'pie') === 'donut');
	let clickToFilter = $derived(config.clickToFilter !== false);
	let showLegend = $derived(config.showLegend ?? true);
	let showValues = $derived(config.showValues ?? false);

	let colorRange = $derived.by(() => {
		const baseColors = config.colors ?? PALETTE;
		const groups = [...new Set(data.map((d) => String(d[dimField] ?? '')))];
		const sel = selectedGroups;
		if (sel.size === 0) return baseColors;
		const valueMap = new Map(groups.map((g) => {
			const val = data.reduce((sum, d) => String(d[dimField]) === g ? sum + Number(d.value ?? 0) : sum, 0);
			return [g, val] as const;
		}));
		const maxVal = Math.max(...valueMap.values(), 1);
		return groups.map((g, i) => {
			if (sel.has(g)) return baseColors[i % baseColors.length] ?? PALETTE[i % PALETTE.length];
			const ratio = (valueMap.get(g) ?? 0) / maxVal;
			const lightness = 0.94 - ratio * 0.18;
			return `oklch(${lightness.toFixed(3)} 0.005 250 / 0.7)`;
		});
	});

	function handleArcClick(_e: MouseEvent, { data: clickedData }: { data: any; series: any }) {
		if (!clickToFilter) return;
		const group = String(clickedData[dimField] ?? '');
		if (!group) return;
		const next = new Set(selectedGroups);
		if (next.has(group)) {
			next.delete(group);
		} else {
			next.add(group);
		}
		selectedGroups = next;
		if (onSliceClick) {
			const row = data.find((r) => String(r[dimField]) === group);
			if (row) onSliceClick({ category: group, value: Number(row.value) || 0, row });
		}
	}

	async function fetchData() {
		isLoading = true;
		error = null;
		selectedGroups = new Set();
		try {
			const result = await executePieDonutChartQuery(config);
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
		const typeLabel = isDonut ? 'Donut' : 'Pie';
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
			<span class="chip-skeleton">Click slices to filter</span>
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
				<PieChart
					data={data}
					key={dimField}
					value="value"
					c={dimField}
					cRange={colorRange}
					innerRadius={isDonut ? 0.6 : undefined}
					cornerRadius={isDonut ? 4 : 1}
					padAngle={isDonut ? 0.01 : 0}
					width={chartWidth}
					height={chartHeight}
					padding={{ top: 8, right: 8, bottom: 8, left: 8 }}
					motion="tween"
					onArcClick={handleArcClick}
					legend={showLegend}
					labels={showValues}
					props={{ arc: { stroke: 'oklch(1 0 0 / 0.35)', strokeWidth: 1 } }}
				/>
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
