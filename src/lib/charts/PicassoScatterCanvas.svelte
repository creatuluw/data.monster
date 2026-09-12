<script lang="ts">
	import picasso from 'picasso.js';
	import type { ScatterChartConfig, BarChartData } from './types';
	import { executeScatterChartQuery } from './engine/DataModelConnector';
	import { PALETTE } from './chart-helpers';
	import { extractErrorMessage } from '$lib/db-operations';
	import { onDestroy } from 'svelte';

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
	let containerEl: HTMLDivElement | undefined = $state();
	let chart: any | null = $state(null);

	let xField = $derived(config.xField.field);
	let yField = $derived(config.yField.field);
	let groupField = $derived(config.groupBy?.field ?? '');
	let hasGroup = $derived(!!groupField);

	let title = $derived.by(() => {
		const xLabel = config.xField.label ?? config.xField.field;
		const yLabel = config.yField.label ?? config.yField.field;
		return `Scatter: ${yLabel} vs ${xLabel}`;
	});

	let selectedCount = $derived(selectedGroups.size);

	function matrixData(): any {
		const header = hasGroup ? [xField, yField, groupField] : [xField, yField, '__row_id__'];
		const rows = data.map((d, i) => {
			const xVal = typeof d[xField] === 'number' ? d[xField] : (Number(d[xField]) || 0);
			const yVal = typeof d[yField] === 'number' ? d[yField] : (Number(d[yField]) || 0);
			const gVal = hasGroup ? String(d[groupField] ?? '') : String(i);
			return [xVal, yVal, gVal];
		});
		return { type: 'matrix', data: [header, ...rows] };
	}

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

	function buildSettings(selected: Set<string>): any {
		const colors = config.colors ?? PALETTE;
		const ptSize = config.pointSize ?? 8;
		const ptOpacity = config.pointOpacity ?? 0.75;
		const showLegend = config.showLegend !== false && hasGroup;
		const hasSelection = selected.size > 0;
		const selField = hasGroup ? groupField : '__row_id__';

		const components: any[] = [
			{ type: 'axis', key: 'y-axis', scale: 'y', dock: 'left' },
			{ type: 'axis', key: 'x-axis', scale: 'x', dock: 'bottom' }
		];

		if (showLegend) {
			components.push({ type: 'legend-cat', dock: 'right', scale: 'col' });
		}

		components.push({
			key: 'points',
			type: 'point',
			data: {
				extract: {
					field: selField,
					props: {
						x: { field: xField },
						y: { field: yField },
						group: { field: selField }
					}
				}
			},
			settings: {
				x: { scale: 'x' },
				y: { scale: 'y' },
				shape: 'circle',
				size: ptSize * 0.07,
				strokeWidth: hasSelection ? 2 : 1.5,
				stroke: '#fff',
				fill: hasGroup ? { scale: 'col', ref: 'group' } : colors[0],
				opacity: hasSelection
					? (d: any) => {
							const key = String(d?.datum?.value ?? d?.datum?.label ?? '');
							return selected.has(key) ? 1 : 0.2;
						}
					: ptOpacity
			}
		});

		return {
			scales: {
				x: { data: { field: xField }, expand: 0.1 },
				y: { data: { field: yField }, expand: 0.15, invert: true },
				...(hasGroup ? { col: { data: { extract: { field: groupField } }, type: 'color', range: colors } } : {})
			},
			components
		};
	}

	function applySelection() {
		if (!chart) return;
		try {
			chart.update({ settings: buildSettings(selectedGroups) as any });
		} catch (_e) { /* */ }
		if (onSelectionChange) {
			const sel = new Set(selectedGroups);
			const filtered = data.filter((d, i) => {
				const key = hasGroup ? String(d[groupField] ?? '') : String(i);
				return sel.has(key);
			});
			onSelectionChange(filtered);
		}
	}

	let clickHandler: ((e: MouseEvent) => void) | null = null;
	let mdHandler: ((e: MouseEvent) => void) | null = null;
	let mmHandler: ((e: MouseEvent) => void) | null = null;
	let muHandler: ((e: MouseEvent) => void) | null = null;

	let lassoBox = { active: false, sx: 0, sy: 0 };
	let lassoWasUsed = false;

	function mountChart() {
		if (!containerEl) return;
		if (isLoading || error || data.length === 0) return;

		if (clickHandler) {
			containerEl.removeEventListener('click', clickHandler);
			clickHandler = null;
		}
		if (mdHandler) {
			containerEl.removeEventListener('mousedown', mdHandler, true);
			mdHandler = null;
		}
		if (mmHandler) {
			document.removeEventListener('mousemove', mmHandler);
			mmHandler = null;
		}
		if (muHandler) {
			document.removeEventListener('mouseup', muHandler);
			muHandler = null;
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
				console.error('[picasso] scatter chart creation failed:', err);
				return;
			}

			// ── click handler ──
			clickHandler = function (e: MouseEvent) {
				if (lassoWasUsed) { lassoWasUsed = false; return; }
				if (!chart) return;
				const r = el.getBoundingClientRect();
				const x = e.clientX - r.left;
				const y = e.clientY - r.top;

				let shapes: any[] = [];
				try {
					shapes = chart.shapesAt(
						{ x: x - 8, y: y - 8, width: 16, height: 16 },
						{ components: [{ key: 'points', propagation: 'stop' }], propagation: 'stop' }
					);
				} catch (err) {
					console.error('[picasso] shapesAt failed:', err);
					return;
				}

				if (shapes.length === 0) return;
				const d = shapes[0].data;
				const cat = String(d.value ?? d.label ?? '');
				if (!cat) return;

				toggleGroup(cat);
			};
			el.addEventListener('click', clickHandler);

			// ── lasso handlers ──
			mdHandler = function (e: MouseEvent) {
				// Don't start lasso on filter-bar clicks (filter-bar is outside chart-area, mousedown won't reach it, but be safe)
				e.preventDefault();
				const area = el.parentElement;
				if (!area) return;
				const r = area.getBoundingClientRect();
				lassoBox = { active: true, sx: e.clientX - r.left, sy: e.clientY - r.top };
			};
			el.addEventListener('mousedown', mdHandler, true);

			mmHandler = function (e: MouseEvent) {
				if (!lassoBox.active) return;
				e.preventDefault();
				const area = el.parentElement;
				if (!area) return;
				const r = area.getBoundingClientRect();
				const cx = e.clientX - r.left;
				const cy = e.clientY - r.top;
				const x = Math.min(lassoBox.sx, cx);
				const y = Math.min(lassoBox.sy, cy);
				const w = Math.abs(cx - lassoBox.sx);
				const h = Math.abs(cy - lassoBox.sy);
				const box = area.querySelector('.lasso') as HTMLDivElement | null;
				if (box) {
					box.style.left = x + 'px';
					box.style.top = y + 'px';
					box.style.width = w + 'px';
					box.style.height = h + 'px';
					box.style.display = (w > 2 || h > 2) ? 'block' : 'none';
				}
			};
			document.addEventListener('mousemove', mmHandler);

			muHandler = function (e: MouseEvent) {
				if (!lassoBox.active) return;
				e.preventDefault();
				const sx = lassoBox.sx;
				const sy = lassoBox.sy;
				lassoBox.active = false;
				const area = el.parentElement;
				if (area) {
					const box = area.querySelector('.lasso') as HTMLDivElement | null;
					if (box) box.style.display = 'none';
				}
				if (!chart) return;

				const r = el.getBoundingClientRect();
				const ar = area?.getBoundingClientRect();
				if (!ar) return;
				// cx/cy in chart-area coords for consistent lasso rect
				const cx = e.clientX - ar.left;
				const cy = e.clientY - ar.top;
				const w = Math.abs(cx - sx);
				const h = Math.abs(cy - sy);

				if (w < 6 && h < 6) return;

				lassoWasUsed = true;

				const ax = Math.min(sx, cx);
				const ay = Math.min(sy, cy);

				// offset from chart-area to chart-body
				const ox = r.left - ar.left;
				const oy = r.top - ar.top;

				let shapes: any[] = [];
				try {
					shapes = chart.shapesAt(
						{ x: ax - ox, y: ay - oy, width: Math.max(w, 1), height: Math.max(h, 1) },
						{ components: [{ key: 'points' }] }
					);
				} catch (err) {
					console.error('[picasso] lasso shapesAt failed:', err);
					return;
				}

				if (shapes.length === 0) {
					// fallback: grid scan with small-area shapesAt (like individual clicks)
					const seen = new Set<string>();
					const step = 14;
					const lx = ax - ox;
					const ly = ay - oy;
					const lx2 = lx + w;
					const ly2 = ly + h;
					for (let sy = ly; sy < ly2; sy += step) {
						for (let sx = lx; sx < lx2; sx += step) {
							try {
								const hits = chart.shapesAt(
									{ x: sx, y: sy, width: step, height: step },
									{ components: [{ key: 'points' }] }
								);
								for (const s of hits) {
									const cat = String(s.data.value ?? s.data.label ?? '');
									if (cat) seen.add(cat);
								}
							} catch (_) { /* skip */ }
						}
					}
					if (seen.size > 0) {
						const next = new Set(selectedGroups);
						for (const cat of seen) next.add(cat);
						selectedGroups = next;
						applySelection();
					}
					return;
				}

				const next = new Set(selectedGroups);
				for (const shape of shapes) {
					const d = shape.data;
					const cat = String(d.value ?? d.label ?? '');
					if (cat) next.add(cat);
				}
				selectedGroups = next;
				applySelection();
			};
			document.addEventListener('mouseup', muHandler);
		});
	}

	function toggleGroup(cat: string) {
		const next = new Set(selectedGroups);
		if (next.has(cat)) {
			next.delete(cat);
		} else {
			next.add(cat);
		}
		selectedGroups = next;
		applySelection();

		if (onPointClick) {
			const idx = data.findIndex((r, i) => {
				const key = hasGroup ? String(r[groupField] ?? '') : String(i);
				return key === cat;
			});
			if (idx >= 0) {
				const row = data[idx];
				onPointClick({
					x: Number(row[xField]) || 0,
					y: Number(row[yField]) || 0,
					group: hasGroup ? String(row[groupField] ?? '') : String(idx),
					row
				});
			}
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

	let lastDataRef: string = '';
	let lastLabelRef: string = '';
	$effect(() => {
		const ref = data.map((d) => `${d[xField]}:${d[yField]}:${d[groupField] ?? ''}`).join('|');
		const labelRef = `${config.pointSize}-${config.pointOpacity}-${config.showLegend}-${(config.colors ?? []).join(',')}`;
		if (ref === lastDataRef && labelRef === lastLabelRef) return;
		lastDataRef = ref;
		lastLabelRef = labelRef;
		mountChart();
	});

	onDestroy(() => {
		if (clickHandler && containerEl) {
			containerEl.removeEventListener('click', clickHandler);
			clickHandler = null;
		}
		if (mdHandler && containerEl) {
			containerEl.removeEventListener('mousedown', mdHandler, true);
			mdHandler = null;
		}
		if (mmHandler) {
			document.removeEventListener('mousemove', mmHandler);
			mmHandler = null;
		}
		if (muHandler) {
			document.removeEventListener('mouseup', muHandler);
			muHandler = null;
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
		{#if selectedCount > 0}
			<span class="selection-count">{selectedCount} selected</span>
		{/if}
	</div>

	{#if selectedCount > 0}
		<div class="filter-bar">
			{#each [...selectedGroups].slice(0, 20) as group (group)}
				<button class="filter-chip" onclick={() => removeGroup(group)}>
					{group}
					<span class="chip-x">&times;</span>
				</button>
			{/each}
			{#if selectedCount > 20}
				<span class="filter-more">+{selectedCount - 20} more</span>
			{/if}
			<button class="filter-clear" onclick={clearAll}>Clear all</button>
		</div>
	{/if}

	<div class="chart-area">
		<div class="lasso"></div>
		<div class="chart-body" class:clickable={true} bind:this={containerEl}>
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
</div>

<style>
	.chart-shell { background: var(--color-surface); border: 1px solid var(--color-border); border-radius: var(--radius-lg); height: 100%; display: flex; flex-direction: column; min-height: 0; }
	.chart-header { padding: var(--space-4) var(--space-6) 0; flex-shrink: 0; display: flex; align-items: center; gap: var(--space-3); }
	.chart-title-text { font-family: var(--font-display); font-size: var(--text-sm); font-weight: 600; color: var(--color-text-secondary); letter-spacing: -0.01em; margin: 0; }
	.selection-count { font-family: var(--font-mono); font-size: 9px; font-weight: 600; color: var(--color-accent); padding: 2px var(--space-2); border: 1px solid var(--color-accent-muted); border-radius: var(--radius-xs); background: var(--color-accent-muted); }
	.filter-bar { display: flex; flex-wrap: wrap; gap: var(--space-2); padding: var(--space-3) var(--space-4); border-bottom: 1px solid var(--color-border); align-items: center; }
	.filter-chip { display: inline-flex; align-items: center; gap: var(--space-1); padding: 2px var(--space-2); border: 1px solid var(--color-accent); background: var(--color-accent-muted); color: var(--color-accent); border-radius: var(--radius-xs); font-family: var(--font-mono); font-size: 9px; font-weight: 600; cursor: pointer; transition: all var(--duration-fast) ease; }
	.filter-chip:hover { background: var(--color-accent); color: var(--color-text-on-accent); }
	.chip-x { font-size: 11px; line-height: 1; opacity: 0.7; }
	.filter-more { font-family: var(--font-mono); font-size: 9px; color: var(--color-text-tertiary); }
	.filter-clear { padding: 2px var(--space-2); border: 1px dashed var(--color-border); background: none; color: var(--color-text-tertiary); border-radius: var(--radius-xs); font-family: var(--font-mono); font-size: 9px; cursor: pointer; transition: all var(--duration-fast) ease; }
	.filter-clear:hover { border-color: var(--color-text-tertiary); color: var(--color-text); }
	.chart-body { width: 100%; height: 100%; padding: var(--space-4); overflow: hidden; position: relative; box-sizing: border-box; }
	.chart-area { position: relative; flex: 1; min-height: 0; }
	.chart-body.clickable { cursor: crosshair; }
	.chart-body :global(svg) { overflow: hidden; }
	.chart-body :global(svg text) { font-family: 'Lekton', monospace !important; font-size: var(--text-xs) !important; }
	.lasso { position: absolute; border: 1px dashed var(--color-accent); background: oklch(0.55 0.18 250 / 0.08); border-radius: 2px; pointer-events: none; z-index: 9999; display: none; }
	.chart-state { display: flex; align-items: center; justify-content: center; gap: var(--space-3); height: 100%; font-size: var(--text-sm); color: var(--color-text-tertiary); }
	.chart-error { color: var(--color-danger); }
</style>
