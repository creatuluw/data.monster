<script lang="ts">
	import { Plot, BarX, BarY, RuleY, HTMLTooltip } from 'svelteplot';
	import type { RawValue } from 'svelteplot/types/data.js';
	import type { Snippet } from 'svelte';
	import ChartCard from '../ChartCard.svelte';
	import { sameDatum } from '$lib/charts/fundament';
	import { resolveTooltip } from '$lib/charts/tooltip';
	import type { ChartRendererProps } from '$lib/charts/renderer-types';

	let {
		rows,
		status: statusProp,
		error: errorProp = '',
		dimensionAliases,
		measureAliases,
		options,
		annotations,
		title = 'Bar chart',
		subtitle = '',
		tooltip,
		selected,
		onSelect,
		fmts,
		heightVh = 0.3,
		heightPx = 0,
		config
	}: ChartRendererProps & { config?: Snippet } = $props();

	const horizontal = $derived(options.orientation !== 'vertical');
	const dim = dimensionAliases[0];
	const meas = measureAliases[0];

	const category = (d: Record<string, unknown>) => String(d[dim]);
	const value = (d: Record<string, unknown>) => Number(d[meas]);

	const template = $derived(tooltip?.template ?? `{${dim}}: {${meas}}`);

	function tipText(d: Record<string, unknown>): string {
		return resolveTooltip(template, d, fmts);
	}

	let chartWrap: HTMLDivElement | undefined = $state();
	let innerHeight = $state(800);
	const plotHeight = $derived(heightPx > 0 ? heightPx : innerHeight * heightVh);

	const markData = $derived(rows as unknown as Record<string | symbol, RawValue>[]);
	const isSel = $derived(sameDatum<Record<string, unknown>>([category]));
	const isSelectedRow = $derived(selected !== null && selected.dimension === dim);

	function fillOf(d: Record<string, unknown>): string {
		return isSelectedRow && selected && isSel(d, { [dim]: selected.value })
			? 'oklch(0.44 0.1 158)'
			: '#888888';
	}

	function handleBarClick(_: unknown, d: Record<string | symbol, unknown>) {
		const row = d as Record<string, unknown>;
		if (isSelectedRow && selected && selected.value === category(row)) onSelect(null);
		else onSelect({ dimension: dim, value: category(row) });
	}

	function handleCardClick(e: MouseEvent) {
		if (!(e.target as Element).closest('rect,path,button')) onSelect(null);
	}

	function handleWindowClick(e: MouseEvent) {
		const t = e.target as Element;
		if (selected && !chartWrap?.contains(t) && !t.closest?.('[data-drawer]')) onSelect(null);
	}

	const yDomain = $derived(horizontal ? markData.map((d) => category(d as Record<string, unknown>)) : undefined);
	const status = $derived(statusProp ?? (rows.length === 0 ? 'empty' : 'ok'));
</script>

<svelte:window onclick={handleWindowClick} bind:innerHeight />

<ChartCard {title} {subtitle} {config} selectionLabel={selected && isSelectedRow ? `${selected.value} · ${value(rows.find((r) => selected && category(r) === selected.value) ?? rows[0])}` : null} {status} error={errorProp}>
	<div bind:this={chartWrap} onclick={handleCardClick}>
		<Plot
			padding={0}
			x={horizontal ? { grid: true } : { padding: 0.3, domain: yDomain }}
			y={horizontal ? { padding: 0.3, domain: yDomain, reverse: true } : { grid: true }}
			color={{ legend: false }}
			height={plotHeight}
		>
			{#if horizontal}
				<BarX
					data={markData}
					x={(d: any) => value(d)}
					y={(d: any) => category(d)}
					fill={{ scale: null, value: (d: any) => fillOf(d) }}
					onclick={handleBarClick}
				/>
			{:else}
				<BarY
					data={markData}
					x={(d: any) => category(d)}
					y={(d: any) => value(d)}
					fill={{ scale: null, value: (d: any) => fillOf(d) }}
					onclick={handleBarClick}
				/>
			{/if}
			{#each annotations as a (a.mark + a.value)}
				{#if a.mark === 'ruleY'}
					<!-- ponytail: annotation labels are v2; the rule itself is the v1 contract -->
					<RuleY data={[{ v: a.value }]} y={(d: any) => d.v} strokeDasharray="4 3" />
				{/if}
			{/each}
			{#snippet overlay()}
				<HTMLTooltip data={markData} x={(d: any) => (horizontal ? value(d) : category(d))} y={(d: any) => (horizontal ? category(d) : value(d))}>
					{#snippet children({ datum })}
						{#if datum}
							<div
								class="bg-zinc-900 text-white text-xs rounded-md px-2.5 py-1.5 shadow-lg whitespace-nowrap -translate-x-1/2 -translate-y-[calc(100%+8px)]"
							>
								{tipText(datum as Record<string, unknown>)}
							</div>
						{/if}
					{/snippet}
				</HTMLTooltip>
			{/snippet}
		</Plot>
	</div>
</ChartCard>
