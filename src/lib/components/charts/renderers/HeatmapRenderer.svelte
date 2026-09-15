<script lang="ts">
	import { Plot, Cell, HTMLTooltip, Text, ColorLegend } from 'svelteplot';
	import type { ColorScheme } from 'svelteplot/types/colorScheme.js';
	import type { RawValue } from 'svelteplot/types/data.js';
	import type { Snippet } from 'svelte';
	import ChartCard from '../ChartCard.svelte';
	import { toGrid } from '$lib/charts/query/compile';
	import { resolveTooltip } from '$lib/charts/tooltip';
	import type { ChartRendererProps } from '$lib/charts/renderer-types';

	let {
		rows,
		status: statusProp,
		error: errorProp = '',
		dimensionAliases,
		measureAliases,
		options,
		title = 'Heatmap',
		subtitle = '',
		tooltip,
		selected,
		onSelect,
		fmts,
		heightVh = 0.3,
		config
	}: ChartRendererProps & { config?: Snippet } = $props();

	const xAlias = dimensionAliases[0];
	const yAlias = dimensionAliases[1];
	const vAlias = measureAliases[0];

	const grid = $derived(
		toGrid(
			rows,
			(r) => String(r[xAlias]),
			(r) => String(r[yAlias]),
			(r) => Number(r[vAlias])
		)
	);

	// full cell matrix — holes in the data render as no-data cells
	type CellRow = { xi: number; yi: number; x: string; y: string; v: number | null };
	const cells = $derived.by(() => {
		const out: CellRow[] = [];
		for (let yi = 0; yi < grid.yValues.length; yi++) {
			for (let xi = 0; xi < grid.xValues.length; xi++) {
				const x = grid.xValues[xi];
				const y = grid.yValues[yi];
				const v = grid.get(x, y) ?? null;
				out.push({ xi, yi, x, y, v });
			}
		}
		return out;
	});

	const filledCells = $derived(cells.filter((c) => c.v !== null) as (CellRow & { v: number })[]);
	const emptyCells = $derived(cells.filter((c) => c.v === null));

	const maxValue = $derived(Math.max(1, ...filledCells.map((c) => c.v)));
	// threshold domain from the data max; options.threshold sets the number of stops
	const stops = $derived(Math.max(2, Number(options.threshold ?? 5)));
	const threshold = $derived(
		Array.from({ length: stops }, (_, i) => Math.round((maxValue * i) / (stops - 1) * 100) / 100)
	);

	const template = $derived(tooltip?.template ?? `{${xAlias}} · {${yAlias}}: {${vAlias}}`);
	function tipText(d: CellRow): string {
		return resolveTooltip(template, { [xAlias]: d.x, [yAlias]: d.y, [vAlias]: d.v }, fmts);
	}

	let clientWidth = $state(500);
	let innerHeight = $state(800);
	let chartWrap: HTMLDivElement | undefined = $state();

	// flip axes on narrow containers for readability (ported from Heatmap.svelte)
	const ax = $derived(clientWidth < 600 ? 'y' : 'x');
	const ay = $derived(clientWidth < 600 ? 'x' : 'y');
	const plotHeight = $derived(innerHeight * heightVh);

	function isSelCell(c: CellRow): boolean {
		return selected !== null && selected.dimension === xAlias && selected.value === c.x;
	}

	function handleCellClick(_: unknown, d: Record<string | symbol, unknown>) {
		const c = d as unknown as CellRow;
		if (isSelCell(c)) onSelect(null);
		else onSelect({ dimension: xAlias, value: c.x });
	}

	function handleCardClick(e: MouseEvent) {
		if (!(e.target as Element).closest('rect')) onSelect(null);
	}

	function handleWindowClick(e: MouseEvent) {
		const t = e.target as Element;
		if (selected && !chartWrap?.contains(t) && !t.closest?.('[data-drawer]')) onSelect(null);
	}

	const selectionLabel = $derived.by(() => {
		if (!selected || selected.dimension !== xAlias) return null;
		const cell = filledCells.find((c) => c.x === selected.value);
		return cell ? `${cell.x} · ${cell.y} · ${cell.v}` : selected.value;
	});
	const status = $derived(statusProp ?? (rows.length === 0 ? 'empty' : 'ok'));
</script>

<svelte:window onclick={handleWindowClick} bind:innerHeight />

<ChartCard {title} {subtitle} selectionLabel={selectionLabel} {config} {status} error={errorProp}>
	<div bind:this={chartWrap} bind:clientWidth onclick={handleCardClick}>
		<Plot
			padding={0}
			color={{
				scheme: (options.scheme ?? 'orrd') as ColorScheme,
				type: 'threshold',
				domain: threshold,
				legend: false
			}}
			height={plotHeight}
			{...{
				[ax]: {
					ticks: grid.xValues.map((_, i) => i),
					tickFormat: (i: number) => grid.xValues[i] ?? ''
				},
				[ay]: {
					ticks: grid.yValues.map((_, i) => i),
					tickFormat: (i: number) => grid.yValues[i] ?? '',
					axis: clientWidth < 600 ? 'both' : 'left'
				}
			}}
		>
			<Cell
				data={filledCells as unknown as Record<string | symbol, RawValue>[]}
				{...{ [ax]: (d: any) => d.xi, [ay]: (d: any) => d.yi }}
				fill={(d: any) => (d as CellRow & { v: number }).v}
				stroke={{ scale: null, value: (d: any) => (isSelCell(d as CellRow) ? '#000' : 'none') }}
				strokeWidth={2}
				inset={0.5}
				onclick={handleCellClick}
			/>
			{#if emptyCells.length}
				<Cell
					data={emptyCells as unknown as Record<string | symbol, RawValue>[]}
					{...{ [ax]: (d: any) => (d as CellRow).xi, [ay]: (d: any) => (d as CellRow).yi }}
					fill="#eee"
					stroke={{ scale: null, value: (d: any) => (isSelCell(d as CellRow) ? '#000' : 'none') }}
					strokeWidth={2}
					inset={0.5}
					onclick={handleCellClick}
				/>
			{/if}
			<Text
				data={filledCells as unknown as Record<string | symbol, RawValue>[]}
				{...{ [ax]: (d: any) => (d as CellRow).xi, [ay]: (d: any) => (d as CellRow).yi }}
				text={(d: any) => String((d as CellRow & { v: number }).v)}
				fontSize={13}
				fontWeight={600}
				fill={{
					scale: null,
					value: (d: any) =>
						(d as CellRow & { v: number }).v >= (threshold[2] ?? maxValue * 0.5) ? '#fff' : '#3f3f46'
				}}
			/>
			{#snippet overlay()}
				<HTMLTooltip data={cells as unknown as Record<string | symbol, RawValue>[]} {...{ [ax]: (d: any) => (d as CellRow).xi, [ay]: (d: any) => (d as CellRow).yi }}>
					{#snippet children({ datum })}
						{#if datum}
							<div
								class="bg-zinc-900 text-white text-xs rounded-md px-2.5 py-1.5 shadow-lg whitespace-nowrap -translate-x-1/2 -translate-y-[calc(100%+8px)]"
							>
								{tipText(datum as unknown as CellRow)}
							</div>
						{/if}
					{/snippet}
				</HTMLTooltip>
			{/snippet}
			{#snippet footer()}
				<div class="flex justify-end">
					<ColorLegend class={null} />
				</div>
			{/snippet}
		</Plot>
	</div>
</ChartCard>
