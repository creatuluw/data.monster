<script lang="ts">
	import { BarChart } from 'layerchart';
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
	let chartWidth = $state(0);
	let chartHeight = $state(0);
	let bodyWrapperEl: HTMLDivElement | undefined = $state();

	let isHorizontal = $derived((config.orientation ?? 'vertical') === 'horizontal');

	let colorRange = $derived.by(() => {
		const dimField = config.dimension.field;
		const groups = [...new Set(data.map((d) => String(d[dimField] ?? '')))];
		const sel = selectedGroups;
		if (sel.size === 0) return PALETTE;
		return groups.map((g) => (sel.has(g) ? PALETTE[groups.indexOf(g) % PALETTE.length] : DIMMED));
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

	function handleBarClick(_event: MouseEvent, detail: { data: any }) {
		if (config.clickToFilter === false) return;
		const group = String(detail.data[config.dimension.field] ?? '');
		if (!group) return;
		const next = new Set(selectedGroups);
		if (next.has(group)) {
			next.delete(group);
		} else {
			next.add(group);
		}
		selectedGroups = next;
	}

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
		return `${agg} of ${metricLabel} by ${dimLabel}`;
	});
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
			{#if isHorizontal}
				<BarChart
					data={data}
					y={config.dimension.field}
					c={config.dimension.field}
					cRange={colorRange}
					orientation="horizontal"
					series={[{
						key: 'value',
						value: 'value',
						color: colorRange[0] ?? 'var(--color-accent)'
					}]}
					width={chartWidth}
					height={chartHeight}
					padding={{ top: 16, right: 24, bottom: 48, left: 64 }}
					motion="tween"
					onBarClick={handleBarClick}
				/>
			{:else}
				<BarChart
					data={data}
					x={config.dimension.field}
					c={config.dimension.field}
					cRange={colorRange}
					orientation="vertical"
					series={[{
						key: 'value',
						value: 'value',
						color: colorRange[0] ?? 'var(--color-accent)'
					}]}
					width={chartWidth}
					height={chartHeight}
					padding={{ top: 16, right: 24, bottom: 48, left: 64 }}
					motion="tween"
					onBarClick={handleBarClick}
				/>
			{/if}
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
