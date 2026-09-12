<script lang="ts">
	import { ScatterChart } from 'layerchart';
	import type { ScatterChartConfig, BarChartData } from './types';
	import { executeScatterChartQuery } from './engine/DataModelConnector';
	import { PALETTE, DIMMED } from './chart-helpers';
	import { extractErrorMessage } from '$lib/db-operations';

	interface Props {
		config: ScatterChartConfig;
		onPointClick?: (detail: { x: number; y: number; group: string; row: BarChartData }) => void;
		onSelectionChange?: (selected: BarChartData[]) => void;
	}

	let { config, onPointClick, onSelectionChange }: Props = $props();

	let data: BarChartData[] = $state([]);
	let selectedGroups: Set<string> = $state(new Set());
	let isLoading: boolean = $state(true);
	let error: string | null = $state(null);
	let chartWidth = $state(0);
	let chartHeight = $state(0);
	let bodyWrapperEl: HTMLDivElement | undefined = $state();
	let chartContext: any = $state(null);

	let xField = $derived(config.xField.field);
	let yField = $derived(config.yField.field);
	let groupField = $derived(config.groupBy?.field);
	let hasGroup = $derived(!!groupField);
	let clickToFilter = $derived(config.clickToFilter !== false);
	let pointR = $derived(config.pointSize ?? 14);
	let pointOpacity = $derived(config.pointOpacity ?? 0.75);
	let showLegend = $derived(config.showLegend ?? true);

	let selectedCount = $derived(selectedGroups.size);

	let colorRange = $derived.by(() => {
		const baseColors = config.colors ?? PALETTE;
		const sel = selectedGroups;
		if (!hasGroup) return baseColors;
		if (sel.size === 0) return baseColors;
		const groups = [...new Set(data.map((d) => String(d[groupField!] ?? '')))];
		return groups.map((g, i) => (sel.has(g) ? (baseColors[i % baseColors.length] ?? PALETTE[i % PALETTE.length]) : DIMMED));
	});

	let tooltipConfig = $derived.by(() => {
		if (!clickToFilter) {
			if (onPointClick) {
				return {
					onclick: (_e: MouseEvent, { data: clickedData }: { data: any }) => {
						const x = Number(clickedData[xField] ?? 0);
						const y = Number(clickedData[yField] ?? 0);
						const group = hasGroup ? String(clickedData[groupField!] ?? '') : '';
						if (onPointClick) {
							onPointClick({ x, y, group, row: clickedData });
						}
					}
				};
			}
			return {};
		}
		return {
			onclick: (_e: MouseEvent, { data: clickedData }: { data: any }) => {
				const catKey = hasGroup
					? String(clickedData[groupField!] ?? '')
					: `${clickedData[xField]}_${clickedData[yField]}`;
				if (!catKey) return;
				const next = new Set(selectedGroups);
				if (next.has(catKey)) {
					next.delete(catKey);
				} else {
					next.add(catKey);
				}
				selectedGroups = next;
				notifySelection();
				if (onPointClick) {
					onPointClick({
						x: Number(clickedData[xField]) || 0,
						y: Number(clickedData[yField]) || 0,
						group: hasGroup ? String(clickedData[groupField!] ?? '') : '',
						row: clickedData
					});
				}
			}
		};
	});

	async function fetchData() {
		isLoading = true;
		error = null;
		selectedGroups = new Set();
		try {
			const result = await executeScatterChartQuery(config);
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
		void config.xField.field;
		void config.yField.field;
		void config.groupBy?.field;
		void config.limit;
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
		notifySelection();
	}

	function clearAll() {
		selectedGroups = new Set();
		notifySelection();
	}

	function notifySelection() {
		if (!onSelectionChange) return;
		const sel = new Set(selectedGroups);
		const filtered = data.filter((d) => {
			const key = hasGroup ? String(d[groupField!] ?? '') : `${d[xField]}_${d[yField]}`;
			return sel.has(key);
		});
		onSelectionChange(filtered);
	}

	let title = $derived.by(() => {
		const xLabel = config.xField.label ?? config.xField.field;
		const yLabel = config.yField.label ?? config.yField.field;
		return `Scatter: ${yLabel} vs ${xLabel}`;
	});

	let lassoBox = $state({ active: false, sx: 0, sy: 0 });
	let lassoStyle = $state({ left: '0px', top: '0px', width: '0px', height: '0px', display: 'none' });
	let lassoWasUsed = $state(false);

	function getCatKey(d: any, idx?: number): string {
		if (hasGroup) return String(d[groupField!] ?? '');
		return `${d[xField]}_${d[yField]}_${idx ?? 0}`;
	}

	const PADDING = { left: 64, top: 16, right: 24, bottom: 48 };

	function handleMouseDown(e: MouseEvent) {
		if (!bodyWrapperEl) return;
		e.preventDefault();
		const r = bodyWrapperEl.getBoundingClientRect();
		lassoBox = { active: true, sx: e.clientX - r.left, sy: e.clientY - r.top };
	}

	function handleMouseMove(e: MouseEvent) {
		if (!lassoBox.active || !bodyWrapperEl) return;
		e.preventDefault();
		const r = bodyWrapperEl.getBoundingClientRect();
		const cx = e.clientX - r.left;
		const cy = e.clientY - r.top;
		const x = Math.min(lassoBox.sx, cx);
		const y = Math.min(lassoBox.sy, cy);
		const w = Math.abs(cx - lassoBox.sx);
		const h = Math.abs(cy - lassoBox.sy);
		lassoStyle = {
			left: x + 'px',
			top: y + 'px',
			width: w + 'px',
			height: h + 'px',
			display: (w > 2 || h > 2) ? 'block' : 'none'
		};
	}

	function handleMouseUp(e: MouseEvent) {
		if (!lassoBox.active || !bodyWrapperEl) return;
		lassoBox.active = false;
		lassoStyle = { left: '0px', top: '0px', width: '0px', height: '0px', display: 'none' };

		const r = bodyWrapperEl.getBoundingClientRect();
		const cx = e.clientX - r.left;
		const cy = e.clientY - r.top;
		const w = Math.abs(cx - lassoBox.sx);
		const h = Math.abs(cy - lassoBox.sy);

		if (w < 6 && h < 6) return;

		lassoWasUsed = true;

		const xScale = chartContext?.xScale;
		const yScale = chartContext?.yScale;
		if (!xScale || !yScale) return;

		const plotSX = Math.min(lassoBox.sx, cx) - PADDING.left;
		const plotSY = Math.min(lassoBox.sy, cy) - PADDING.top;
		const plotEX = plotSX + Math.max(w, 1);
		const plotEY = plotSY + Math.max(h, 1);

		let xMin: number, xMax: number, yMin: number, yMax: number;
		try {
			xMin = xScale.invert(plotSX);
			xMax = xScale.invert(plotEX);
			yMin = yScale.invert(plotSY);
			yMax = yScale.invert(plotEY);
		} catch (_) {
			return;
		}

		if (xMin > xMax) { const t = xMin; xMin = xMax; xMax = t; }
		if (yMin > yMax) { const t = yMin; yMin = yMax; yMax = t; }

		const next = new Set(selectedGroups);
		for (let i = 0; i < data.length; i++) {
			const d = data[i];
			const dx = Number(d[xField]) ?? 0;
			const dy = Number(d[yField]) ?? 0;
			if (dx >= xMin && dx <= xMax && dy >= yMin && dy <= yMax) {
				next.add(getCatKey(d, i));
			}
		}

		if (next.size > selectedGroups.size) {
			selectedGroups = next;
			notifySelection();
		}
	}

	let mdHandler: ((e: MouseEvent) => void) | null = null;
	let mmHandler: ((e: MouseEvent) => void) | null = null;
	let muHandler: ((e: MouseEvent) => void) | null = null;

	$effect(() => {
		const el = bodyWrapperEl;
		if (!el) return;

		if (mdHandler) el.removeEventListener('mousedown', mdHandler, true);
		if (mmHandler) document.removeEventListener('mousemove', mmHandler);
		if (muHandler) document.removeEventListener('mouseup', muHandler);

		mdHandler = handleMouseDown;
		mmHandler = handleMouseMove;
		muHandler = handleMouseUp;

		el.addEventListener('mousedown', mdHandler, true);
		document.addEventListener('mousemove', mmHandler);
		document.addEventListener('mouseup', muHandler);

		return () => {
			if (mdHandler && el) el.removeEventListener('mousedown', mdHandler, true);
			if (mmHandler) document.removeEventListener('mousemove', mmHandler);
			if (muHandler) document.removeEventListener('mouseup', muHandler);
			mdHandler = null; mmHandler = null; muHandler = null;
		};
	});
</script>

<div class="chart-shell">
	<div class="chip-row">
		{#if selectedCount > 0}
			{#each [...selectedGroups].slice(0, 20) as group (group)}
				<button class="filter-chip" onclick={() => removeGroup(group)}>
					{group}
					<span class="chip-x">&times;</span>
				</button>
			{/each}
			{#if selectedCount > 20}
				<span class="filter-more">+{selectedCount - 20} more</span>
			{/if}
			<button class="clear-all-btn" onclick={clearAll}>Clear all</button>
		{:else}
			<span class="chip-skeleton">Click points or drag to lasso</span>
		{/if}
	</div>
	<div class="chart-header">
		<h3 class="chart-title-text">{title}</h3>
		{#if selectedCount > 0}
			<span class="selection-count">{selectedCount} selected</span>
		{/if}
	</div>

	<div class="chart-body" bind:this={bodyWrapperEl}>
		<div class="lasso" style="left:{lassoStyle.left};top:{lassoStyle.top};width:{lassoStyle.width};height:{lassoStyle.height};display:{lassoStyle.display}"></div>
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
				<ScatterChart
					data={data}
					x={xField}
					y={yField}
					c={hasGroup ? groupField : undefined}
					cRange={colorRange}
					width={chartWidth}
					height={chartHeight}
					padding={PADDING}
					motion="tween"
					tooltipContext={tooltipConfig}
					legend={hasGroup ? showLegend : false}
					points={{ r: pointR * 0.6, opacity: pointOpacity }}
					bind:context={chartContext}
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

	.filter-more {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
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

	.selection-count {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		color: var(--color-accent);
		padding: 2px var(--space-2);
		border: 1px solid var(--color-accent-muted);
		border-radius: var(--radius-xs);
		background: var(--color-accent-muted);
	}

	.chart-body {
		flex: 1;
		min-height: 0;
		position: relative;
	}

	.lasso {
		position: absolute;
		border: 1px dashed var(--color-accent);
		background: oklch(0.55 0.18 250 / 0.08);
		border-radius: 2px;
		pointer-events: none;
		z-index: 9999;
		display: none;
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
