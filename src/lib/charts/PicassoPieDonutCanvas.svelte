<script lang="ts">
	import picasso from 'picasso.js';
	import type { PieDonutChartConfig, BarChartData } from './types';
	import { executePieDonutChartQuery } from './engine/DataModelConnector';
	import { PALETTE } from './chart-helpers';
	import { extractErrorMessage } from '$lib/db-operations';
	import { onDestroy } from 'svelte';

	function formatValue(value: number, fmt: string): string {
		switch (fmt) {
			case 'currency': return '$' + value.toLocaleString();
			case 'percent': return value.toFixed(1) + '%';
			case 'compact': {
				if (value >= 1_000_000) return (value / 1_000_000).toFixed(1) + 'M';
				if (value >= 1_000) return (value / 1_000).toFixed(1) + 'K';
				return value.toLocaleString();
			}
			default: return value.toLocaleString();
		}
	}

	interface Props {
		config: PieDonutChartConfig;
		onSliceClick?: (detail: { category: string; value: number; row: BarChartData }) => void;
	}

	let { config, onSliceClick }: Props = $props();

	let data: BarChartData[] = $state([]);
	let isLoading: boolean = $state(true);
	let error: string | null = $state(null);
	let containerEl: HTMLDivElement | undefined = $state();
	let chart: any | null = $state(null);

	let dimField = $derived(config.dimension.field);
	let isDonut = $derived(config.chartType === 'donut');

	let title = $derived.by(() => {
		const agg = config.metric.aggregate ?? 'SUM';
		const metricLabel = config.metric.label ?? config.metric.field;
		const dimLabel = config.dimension.label ?? config.dimension.field;
		const typeLabel = isDonut ? 'Donut' : 'Pie';
		return `${typeLabel}: ${agg} of ${metricLabel} by ${dimLabel}`;
	});

	let total = $derived(data.reduce((sum, d) => sum + (Number(d.value) || 0), 0));

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
			const result = await executePieDonutChartQuery(config);
			data = result.rows;
		} catch (e: unknown) {
			error = extractErrorMessage(e);
			data = [];
		} finally {
			isLoading = false;
		}
	}

	function buildSettings(): any {
		const colors = config.colors ?? PALETTE;
		const showLegend = config.showLegend !== false;

		const components: any[] = [];

		if (showLegend) {
			components.push({
				type: 'legend-cat',
				scale: 'c',
				dock: 'right'
			});
		}

		const sliceSettings: any = {
			arc: { ref: 'num' },
			fill: { scale: 'c' },
			outerRadius: 0.9,
			strokeWidth: 1,
			stroke: 'rgba(255, 255, 255, 0.5)',
			cornerRadius: isDonut ? 4 : 1
		};

		if (isDonut) {
			sliceSettings.innerRadius = 0.5;
		}

		components.push({
			key: 'p',
			type: 'pie',
			data: {
				extract: {
					field: dimField,
					props: {
						num: { field: 'value' }
					}
				}
			},
			settings: {
				padAngle: isDonut ? 0.01 : 0,
				slice: sliceSettings
			}
		});

		return {
			scales: {
				c: {
					data: { extract: { field: dimField } },
					type: 'color',
					range: colors
				}
			},
			components
		};
	}

	let clickHandler: ((e: MouseEvent) => void) | null = null;

	function renderValueLabels() {
		if (!containerEl || !chart || !config.showValues) return;
		const existing = containerEl.querySelectorAll('.value-label');
		existing.forEach((el) => el.remove());

		const svg = containerEl.querySelector('svg');
		if (!svg) return;

		let shapes: any[] = [];
		try {
			shapes = chart.shapesAt(
				{ x: 0, y: 0, width: 99999, height: 99999 },
				{ components: [{ key: 'p', propagation: 'stop' }], propagation: 'stop' }
			);
		} catch (_) { return; }

		for (let i = 0; i < data.length; i++) {
			const row = data[i];
			const cat = String(row[dimField] ?? '');
			const val = Number(row.value) || 0;
			const pct = total > 0 ? ((val / total) * 100).toFixed(1) + '%' : '0%';
			const text = config.valueFormat === 'percent' ? pct : `${formatValue(val, config.valueFormat ?? 'number')} (${pct})`;

			const shape = shapes[i];
			if (!shape) continue;

			const bounds = shape.bounds;
			if (!bounds) continue;

			const textEl = document.createElementNS('http://www.w3.org/2000/svg', 'text');
			textEl.setAttribute('class', 'value-label');
			textEl.setAttribute('fill', '#333');
			textEl.setAttribute('font-family', 'Lekton, monospace');
			textEl.setAttribute('font-size', '10');
			textEl.setAttribute('text-anchor', 'middle');
			textEl.setAttribute('dominant-baseline', 'central');
			textEl.textContent = text;

			const cx = bounds.x + bounds.width / 2;
			const cy = bounds.y + bounds.height / 2;

			if (isDonut) {
				textEl.setAttribute('x', String(cx));
				textEl.setAttribute('y', String(cy));
			} else {
				const svgRect = svg.getBoundingClientRect();
				const svgCx = svgRect.width / 2;
				const svgCy = svgRect.height / 2;
				const dx = cx - svgCx;
				const dy = cy - svgCy;
				const dist = Math.sqrt(dx * dx + dy * dy);
				const pushFactor = 0.65;
				textEl.setAttribute('x', String(svgCx + dx * pushFactor));
				textEl.setAttribute('y', String(svgCy + dy * pushFactor));
			}

			svg.appendChild(textEl);
		}
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
					settings: buildSettings()
				});
			} catch (err) {
				console.error('[picasso] pie/donut chart creation failed:', err);
				return;
			}

			renderValueLabels();

			if (onSliceClick) {
				clickHandler = function (e: MouseEvent) {
					if (!chart) return;
					const r = el.getBoundingClientRect();
					const x = e.clientX - r.left;
					const y = e.clientY - r.top;

					let shapes: any[] = [];
					try {
						shapes = chart.shapesAt(
							{ x, y, width: 1, height: 1 },
							{ components: [{ key: 'p', propagation: 'stop' }], propagation: 'stop' }
						);
					} catch (err) {
						return;
					}

					if (shapes.length === 0) return;
					const d = shapes[0].data;
					const cat = String(d.value ?? d.label ?? '');
					if (!cat) return;

					const row = data.find((r) => String(r[dimField]) === cat);
					if (row) {
						onSliceClick({ category: cat, value: Number(row.value) || 0, row });
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
		void config.chartType;
		fetchData();
	});

	let lastDataRef: string = '';
	let lastSettingsRef: string = '';
	$effect(() => {
		const ref = data.map((d) => `${d[dimField]}=${d.value}`).join('|');
		const settingsRef = `${config.chartType}-${config.showValues}-${config.valueFormat}-${config.showLegend}-${(config.colors ?? []).join(',')}`;
		if (ref === lastDataRef && settingsRef === lastSettingsRef) return;
		lastDataRef = ref;
		lastSettingsRef = settingsRef;
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
</script>

<div class="chart-shell">
	<div class="chart-header">
		<h3 class="chart-title-text">{title}</h3>
	</div>

	{#if isDonut && total > 0}
		<div class="donut-center-hint">
			<span class="donut-total">{formatValue(total, config.valueFormat ?? 'number')}</span>
			<span class="donut-total-label">total</span>
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
		position: relative;
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

	.donut-center-hint {
		position: absolute;
		top: 50%;
		left: calc(50% - 60px);
		transform: translate(-50%, -50%);
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 2px;
		pointer-events: none;
		z-index: 5;
	}

	.donut-total {
		font-family: var(--font-mono);
		font-size: var(--text-lg);
		font-weight: 700;
		color: var(--color-text);
		letter-spacing: -0.02em;
	}

	.donut-total-label {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 400;
		color: var(--color-text-tertiary);
		letter-spacing: 0.06em;
		text-transform: uppercase;
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

	.chart-body :global(svg text) {
		font-family: 'Lekton', monospace !important;
		font-size: var(--text-xs) !important;
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
