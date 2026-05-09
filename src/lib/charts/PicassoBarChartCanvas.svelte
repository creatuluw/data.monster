<script lang="ts">
	import picasso from 'picasso.js';
	import type { BarChartConfig, BarChartData } from './types';
	import { executeBarChartQuery } from './engine/DataModelConnector';
	import { PALETTE } from './chart-helpers';
	import { extractErrorMessage } from '$lib/db-operations';
	import { onDestroy } from 'svelte';

	interface Props {
		config: BarChartConfig;
		onBarClick?: (detail: { category: string; value: number; row: BarChartData }) => void;
	}

	let { config, onBarClick }: Props = $props();

	let data: BarChartData[] = $state([]);
	let selectedGroups: Set<string> = $state(new Set());
	let isLoading: boolean = $state(true);
	let error: string | null = $state(null);
	let containerEl: HTMLDivElement | undefined = $state();
	let chart: any | null = $state(null);

	let dimField = $derived(config.dimension.field);

	let title = $derived.by(() => {
		const agg = config.metric.aggregate ?? 'SUM';
		const metricLabel = config.metric.label ?? config.metric.field;
		const dimLabel = config.dimension.label ?? config.dimension.field;
		return `${agg} of ${metricLabel} by ${dimLabel}`;
	});

	function matrixData(): any {
		const header = [dimField, 'value'];
		const rows = data.map((d) => [String(d[dimField] ?? ''), typeof d.value === 'number' ? d.value : (Number(d.value) || 0)]);
		return {
			type: 'matrix',
			data: [header, ...rows]
		};
	}

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

	function buildSettings(selected: Set<string>) {
		const hasSelection = selected.size > 0;

		return {
			scales: {
				cat: { data: { extract: { field: dimField } }, type: 'band', padding: 0.15 },
				val: { data: { extract: { field: 'value' } }, type: 'linear', min: 0, expand: 0.05, invert: true }
			},
			components: [
				{
					type: 'axis',
					key: 'yaxis',
					scale: 'val',
					layout: { dock: 'left' }
				},
				{
					type: 'axis',
					key: 'xaxis',
					scale: 'cat',
					layout: { dock: 'bottom' }
				},
				{
					type: 'box',
					key: 'bars',
					data: {
						extract: {
							field: dimField,
							props: {
								start: 0,
								end: { field: 'value' }
							}
						}
					},
					settings: {
						major: { scale: 'cat' },
						minor: { scale: 'val' },
						orientation: 'vertical',
						box: {
							fill: (d: any) => {
								const cat = d?.datum?.value?.[dimField] ?? d?.data?.value?.[dimField] ?? '';
								const cats = data.map((r) => String(r[dimField] ?? ''));
								const idx = cats.indexOf(cat);
								return PALETTE[idx % PALETTE.length];
							},
							opacity: hasSelection
								? (d: any) => {
										const cat = d?.datum?.value ?? d?.data?.value ?? '';
										return selected.has(String(cat)) ? 1 : 0.35;
									}
								: undefined,
							stroke: 'transparent',
							strokeWidth: 0,
							width: 0.85,
							minWidthPx: 2,
							minHeightPx: 1
						}
					}
				}
			]
		};
	}

	let clickHandler: ((e: MouseEvent) => void) | null = null;

	function applySelection() {
		if (!chart) return;
		try {
			chart.update({ settings: buildSettings(selectedGroups) as any });
		} catch (_e) { /* */ }
	}

	function mountChart() {
		if (!containerEl) return;
		if (isLoading || error || data.length === 0) return;

		if (clickHandler) {
			containerEl.removeEventListener('click', clickHandler);
			clickHandler = null;
		}

		if (chart) {
			try { chart.destroy(); } catch (_e) { /* */ }
			chart = null;
		}

		const el = containerEl;

		requestAnimationFrame(() => {
			if (!containerEl) return;
			const rect = containerEl.getBoundingClientRect();
			if (rect.width === 0 || rect.height === 0) return;

			try {
				chart = picasso.chart({
					element: el,
					data: [matrixData()],
					settings: buildSettings(selectedGroups) as any
				});
			} catch (err) {
				console.error('[picasso] chart creation failed:', err);
				return;
			}

			if (config.clickToFilter !== false) {
				clickHandler = function (e: MouseEvent) {
					if (!chart) return;
					const r = el.getBoundingClientRect();
					const x = e.clientX - r.left;
					const y = e.clientY - r.top;

					let shapes: any[] = [];
					try {
						shapes = chart.shapesAt(
							{ x, y, width: 1, height: 1 },
							{ components: [{ key: 'bars', propagation: 'stop' }], propagation: 'stop' }
						);
					} catch (err) {
						console.error('[picasso] shapesAt failed:', err);
						return;
					}

					if (shapes.length === 0) return;
					const d = shapes[0].data;
					const cat = String(d.value ?? d.label ?? '');
					if (!cat) return;

					const next = new Set(selectedGroups);
					if (next.has(cat)) {
						next.delete(cat);
					} else {
						next.add(cat);
					}
					selectedGroups = next;

					applySelection();

					if (onBarClick) {
						const row = data.find((r) => String(r[dimField]) === cat);
						if (row) {
							onBarClick({ category: cat, value: Number(row.value) || 0, row });
						}
					}
				};

				el.addEventListener('click', clickHandler);
			}
		});
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

	let lastDataRef: string = '';
	$effect(() => {
		const ref = data.map((d) => `${d[dimField]}=${d.value}`).join('|');
		if (ref === lastDataRef) return;
		lastDataRef = ref;
		mountChart();
	});

	onDestroy(() => {
		if (clickHandler && containerEl) {
			containerEl.removeEventListener('click', clickHandler);
			clickHandler = null;
		}
		if (chart) {
			try { chart.destroy(); } catch (_e) { /* */ }
			chart = null;
		}
	});

	function removeGroup(group: string) {
		const next = new Set(selectedGroups);
		next.delete(group);
		selectedGroups = next;
		applySelection();
	}

	function clearAll() {
		selectedGroups = new Set();
		applySelection();
	}
</script>

<div class="chart-shell">
	<div class="chart-header">
		<h3 class="chart-title-text">{title}</h3>
	</div>

	<div class="chart-body" class:clickable={config.clickToFilter !== false} bind:this={containerEl}>
		{#if config.clickToFilter !== false && selectedGroups.size > 0}
			<div class="filter-bar">
				{#each [...selectedGroups] as group (group)}
					<button class="filter-chip" onclick={() => removeGroup(group)}>
						{group}
						<span class="chip-x">&times;</span>
					</button>
				{/each}
				<button class="filter-clear" onclick={clearAll}>Clear all</button>
			</div>
		{/if}
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

	.filter-bar {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-2);
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
		align-items: center;
	}

	.filter-chip {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		padding: 2px var(--space-2);
		border: 1px solid var(--color-accent);
		background: var(--color-accent-muted);
		color: var(--color-accent);
		border-radius: var(--radius-xs);
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		cursor: pointer;
		transition: all var(--duration-fast) ease;
	}

	.filter-chip:hover {
		background: var(--color-accent);
		color: var(--color-text-on-accent);
	}

	.chip-x {
		font-size: 11px;
		line-height: 1;
		opacity: 0.7;
	}

	.filter-clear {
		padding: 2px var(--space-2);
		border: 1px dashed var(--color-border);
		background: none;
		color: var(--color-text-tertiary);
		border-radius: var(--radius-xs);
		font-family: var(--font-mono);
		font-size: 9px;
		cursor: pointer;
		transition: all var(--duration-fast) ease;
	}

	.filter-clear:hover {
		border-color: var(--color-text-tertiary);
		color: var(--color-text);
	}

	.chart-body {
		flex: 1;
		min-height: 0;
		padding: var(--space-4);
		overflow: hidden;
		position: relative;
	}

	.chart-body :global(svg) {
		overflow: hidden;
	}

	.chart-body.clickable {
		cursor: pointer;
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
