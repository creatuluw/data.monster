<script lang="ts">
	import '@carbon/charts-svelte/styles.css';
	import { onMount } from 'svelte';
	import type { Component } from 'svelte';

	let ChartComponent: Component | null = $state(null);
	let chart: any = $state(null);
	let selectedGroups: Set<string> = $state(new Set());
	let salesData: any[] = $state([]);

	const PALETTE = [
		'oklch(0.69 0.16 41)',
		'oklch(0.58 0.18 250)',
		'oklch(0.68 0.14 160)',
		'oklch(0.62 0.17 145)',
		'oklch(0.78 0.09 41)',
		'oklch(0.58 0.2 280)',
		'oklch(0.72 0.13 190)',
		'oklch(0.66 0.16 130)',
		'oklch(0.58 0.17 22)',
		'oklch(0.76 0.13 80)'
	];

	const DIMMED = 'oklch(0.9 0.005 250 / 0.45)';

	onMount(async () => {
		const charts = await import('@carbon/charts-svelte');
		ChartComponent = charts.BarChartSimple;
		const res = await fetch('/data.json');
		salesData = await res.json();
	});

	const fields = ['product', 'category', 'region', 'segment', 'channel', 'salesperson', 'status', 'paymentMethod'];
	let groupBy: string = $state('product');
	let metric: string = $state('netAmount');

	let allAggregated = $derived.by(() => {
		const map = new Map<string, number>();
		for (const row of salesData) {
			const key = row[groupBy];
			map.set(key, (map.get(key) ?? 0) + row[metric]);
		}
		return [...map.entries()]
			.map(([group, value]) => ({ group, value: Math.round(value) }))
			.sort((a, b) => a.value - b.value);
	});

	let filteredData = $derived(
		selectedGroups.size === 0
			? salesData
			: salesData.filter((row) => selectedGroups.has(row[groupBy]))
	);

	let filteredAggregated = $derived.by(() => {
		const map = new Map<string, number>();
		for (const row of filteredData) {
			const key = row[groupBy];
			map.set(key, (map.get(key) ?? 0) + row[metric]);
		}
		return [...map.entries()]
			.map(([group, value]) => ({ group, value: Math.round(value) }))
			.sort((a, b) => b.value - a.value);
	});

	let chartData = $derived(allAggregated);

	const colorScale = $derived(
		Object.fromEntries(
			allAggregated.map((d, i) => {
				if (selectedGroups.size > 0 && !selectedGroups.has(d.group)) return [d.group, DIMMED];
				return [d.group, PALETTE[i % PALETTE.length]];
			})
		)
	);

	const options = $derived({
		theme: 'white',
		title: `${metric} by ${groupBy}`,
		width: '100%',
		height: chartData.length <= 20 ? Math.max(300, chartData.length * 30) + 'px' : '600px',
		color: { scale: colorScale },
		axes: {
			left: { mapsTo: 'group', scaleType: 'labels' } as const,
			bottom: { mapsTo: 'value' }
		}
	});

	function handleBarClick(e: CustomEvent) {
		const clickedGroup = e.detail?.datum?.group ?? e.detail?.group ?? null;
		if (!clickedGroup) return;
		const next = new Set(selectedGroups);
		if (next.has(clickedGroup)) {
			next.delete(clickedGroup);
		} else {
			next.add(clickedGroup);
		}
		selectedGroups = next;
	}

	function clearFilter() {
		selectedGroups = new Set();
	}

	function statusTag(status: string): { cls: string; label: string } {
		switch (status) {
			case 'Completed': return { cls: 'tag-success', label: status };
			case 'Pending': return { cls: 'tag-warning', label: status };
			case 'Refunded': return { cls: 'tag-danger', label: status };
			default: return { cls: 'tag-default', label: status };
		}
	}

	$effect(() => {
		if (chart) {
			chart.services.events.addEventListener('bar-click', handleBarClick);
			return () => {
				chart.services.events.removeEventListener('bar-click', handleBarClick);
			};
		}
	});


</script>

<svelte:head>
	<title>Data Monster</title>
	<meta name="description" content="Sales data analytics dashboard built with the LRS Glacier Design System" />
</svelte:head>

<nav class="nav" aria-label="Main navigation">
	<div class="nav-inner">
		<a href="/" class="nav-brand">
			<span class="brand-mark"></span>
			Data Monster
		</a>
		<ul class="nav-links">
			<li><a href="#dashboard">Dashboard</a></li>
			<li><a href="#data">Data</a></li>
		</ul>
	</div>
</nav>

{#if salesData.length === 0}
	<section class="section">
		<p class="loading-text">Loading data...</p>
	</section>
{:else}
	<section class="section" id="dashboard" aria-labelledby="dashboard-title">
		<div class="section-header entrance">
			<span class="section-number">01</span>
			<h2 class="section-title" id="dashboard-title">Dashboard</h2>
		</div>

		<div class="controls entrance entrance-d1">
			<div class="field">
				<label class="field-label" for="group-by">Group by</label>
				<select class="input" id="group-by" bind:value={groupBy}>
					{#each fields as f}
						<option value={f}>{f}</option>
					{/each}
				</select>
			</div>
			<div class="field">
				<label class="field-label" for="metric">Metric</label>
				<select class="input" id="metric" bind:value={metric}>
					<option value="netAmount">Net Amount</option>
					<option value="quantity">Quantity</option>
					<option value="unitPrice">Unit Price</option>
					<option value="discountPct">Discount %</option>
					<option value="rating">Rating</option>
				</select>
			</div>
			<span class="row-count">
				{filteredData.length.toLocaleString()} of {salesData.length.toLocaleString()} rows
				{#if selectedGroups.size > 0}
					&middot; {selectedGroups.size} group{selectedGroups.size > 1 ? 's' : ''} selected
				{/if}
			</span>
		</div>

		<div class="filter-bar entrance">
			{#each [...selectedGroups] as group}
				<button class="filter-chip" onclick={() => {
					const next = new Set(selectedGroups);
					next.delete(group);
					selectedGroups = next;
				}}>
					{group} &times;
				</button>
			{/each}
			{#if selectedGroups.size > 0}
				<button class="btn btn-ghost btn-sm" onclick={clearFilter}>
					Clear all
				</button>
			{/if}
		</div>

		<div class="chart-wrap entrance entrance-d2">
			{#if ChartComponent}
				<ChartComponent bind:chart data={chartData} {options} />
			{:else}
				<p>Loading chart...</p>
			{/if}
		</div>

		{#if allAggregated.length > 0}
			<div class="summary-cards entrance entrance-d3">
				<article class="summary-card">
					<span class="summary-label">Top Group</span>
					<span class="summary-value">{filteredAggregated[0]?.group ?? allAggregated[allAggregated.length - 1]?.group ?? '—'}</span>
				</article>
				<article class="summary-card">
					<span class="summary-label">Top Value</span>
					<span class="summary-value">${(filteredAggregated[0]?.value ?? allAggregated[allAggregated.length - 1]?.value ?? 0).toLocaleString()}</span>
				</article>
				<article class="summary-card">
					<span class="summary-label">Groups</span>
					<span class="summary-value">{selectedGroups.size > 0 ? filteredAggregated.length : allAggregated.length}</span>
				</article>
				<article class="summary-card">
					<span class="summary-label">Total</span>
					<span class="summary-value">${(selectedGroups.size > 0 ? filteredAggregated : allAggregated).reduce((s, d) => s + d.value, 0).toLocaleString()}</span>
				</article>
			</div>
		{/if}
	</section>

	<hr class="divider-dashed" />

	<section class="section" id="data" aria-labelledby="data-title">
		<div class="section-header entrance">
			<span class="section-number">02</span>
			<h2 class="section-title" id="data-title">Data Table</h2>
		</div>

		<div class="table-wrap entrance entrance-d1">
			<table>
				<thead>
					<tr>
						<th>Product</th>
						<th>Salesperson</th>
						<th>Net Amount</th>
						<th>Quantity</th>
						<th>Status</th>
					</tr>
				</thead>
				<tbody>
					{#each filteredData as row (row.id)}
						{@const st = statusTag(row.status)}
						<tr>
							<td>{row.product}</td>
							<td>{row.salesperson}</td>
							<td>${row.netAmount.toLocaleString()}</td>
							<td>{row.quantity}</td>
							<td><span class="tag {st.cls}">{st.label}</span></td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</section>
{/if}

<footer class="footer">
	<span class="footer-brand">Data Monster</span>
	<div class="footer-meta">
		<span>DS2-GLACIER-7742</span>
		<span>OKLCH</span>
	</div>
</footer>

<style>
	.nav {
		position: sticky;
		top: 0;
		z-index: 100;
		background: oklch(0.98 0.003 250 / 0.92);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
		border-bottom: 1px solid var(--color-border);
	}

	.nav-inner {
		max-width: var(--max-width);
		margin: 0 auto;
		padding: var(--space-3) var(--gutter);
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.nav-brand {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 600;
		letter-spacing: -0.02em;
		color: var(--color-text);
		text-decoration: none;
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.brand-mark {
		width: 20px;
		height: 20px;
		border: 2px solid var(--color-accent);
		border-radius: var(--radius-xs);
		position: relative;
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	.brand-mark::after {
		content: '';
		width: 6px;
		height: 6px;
		background: var(--color-accent);
		border-radius: 1px;
	}

	.nav-links {
		display: flex;
		gap: var(--space-1);
		list-style: none;
	}

	.nav-links a {
		font-family: var(--font-body);
		font-size: var(--text-sm);
		font-weight: 500;
		color: var(--color-text-secondary);
		text-decoration: none;
		padding: var(--space-2) var(--space-3);
		border-radius: var(--radius-xs);
		transition:
			color var(--duration-fast) ease,
			background var(--duration-fast) ease;
	}

	.nav-links a:hover {
		color: var(--color-accent);
		background: var(--color-accent-muted);
	}

	.nav-links a:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	.divider {
		border: none;
		height: 1px;
		background: var(--color-border);
		max-width: var(--max-width);
		margin: 0 auto;
	}

	.divider-dashed {
		border: none;
		height: 0;
		border-top: 1px dashed var(--color-border);
		max-width: var(--max-width);
		margin: 0 auto;
	}

	.section {
		max-width: var(--max-width);
		margin: 0 auto;
		padding: var(--space-16) var(--gutter);
		position: relative;
	}

	.section-header {
		display: flex;
		align-items: baseline;
		gap: var(--space-4);
		margin-bottom: var(--space-12);
	}

	.section-number {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		letter-spacing: 0.1em;
		color: var(--color-accent);
		padding: var(--space-1) var(--space-2);
		border: 1px solid var(--color-accent-muted);
		border-radius: var(--radius-xs);
		background: var(--color-accent-muted);
		white-space: nowrap;
	}

	.section-title {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: 700;
		letter-spacing: -0.02em;
		line-height: var(--leading-snug);
		color: var(--color-text);
	}

	.loading-text {
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}

	.controls {
		display: flex;
		align-items: flex-end;
		gap: var(--space-6);
		margin-bottom: var(--space-8);
		flex-wrap: wrap;
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.field-label {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
	}

	.input {
		padding: var(--space-2) var(--space-3);
		border: 1px solid var(--color-border-strong);
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		color: var(--color-text);
		background: var(--color-surface);
		border-radius: var(--radius-xs);
		transition:
			border-color var(--duration-fast) ease,
			box-shadow var(--duration-fast) ease;
	}

	.input:focus {
		outline: none;
		border-color: var(--color-accent);
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}

	.row-count {
		font-family: var(--font-mono);
		font-size: 10px;
		letter-spacing: 0.04em;
		color: var(--color-text-tertiary);
		padding-bottom: var(--space-3);
	}

	.filter-bar {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		margin-bottom: var(--space-6);
		flex-wrap: wrap;
		min-height: calc(var(--text-xs) + var(--space-4));
	}

	.filter-chip {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		padding: 2px var(--space-2);
		background: var(--color-accent-muted);
		border: 1px solid oklch(0.82 0.03 41);
		border-radius: var(--radius-xs);
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-accent-dark);
		cursor: pointer;
		transition: background var(--duration-fast) ease;
	}

	.filter-chip:hover {
		background: oklch(0.88 0.03 41);
	}

	.btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-4);
		font-family: var(--font-body);
		font-size: var(--text-xs);
		font-weight: 600;
		line-height: 1.4;
		cursor: pointer;
		border: 1px solid transparent;
		border-radius: var(--radius-xs);
		transition:
			transform var(--duration-fast) var(--ease-out-expo),
			background var(--duration-fast) ease,
			color var(--duration-fast) ease,
			border-color var(--duration-fast) ease;
	}

	.btn:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 2px;
	}

	.btn:active {
		transform: scale(0.97);
	}

	.btn-ghost {
		background: transparent;
		color: var(--color-text-secondary);
		border-color: transparent;
	}

	.btn-ghost:hover {
		background: var(--color-surface-sunken);
		color: var(--color-text);
	}

	.btn-sm {
		padding: var(--space-1) var(--space-3);
		font-size: 9px;
	}

	.chart-wrap {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-lg);
		padding: var(--space-4);
		box-shadow: var(--shadow-sm);
	}

	.summary-cards {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
		gap: 1px;
		background: var(--color-border);
		margin-top: var(--space-8);
	}

	.summary-card {
		background: var(--color-surface);
		padding: var(--space-6);
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		transition: background var(--duration-fast) ease;
	}

	.summary-card:hover {
		background: var(--color-surface-raised);
	}

	.summary-label {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.summary-value {
		font-family: var(--font-display);
		font-size: var(--text-lg);
		font-weight: 600;
		letter-spacing: -0.02em;
		color: var(--color-text);
	}

	.table-wrap {
		overflow-x: auto;
	}

	table {
		width: 100%;
		border-collapse: collapse;
		background: var(--color-surface);
		font-size: var(--text-sm);
		border: 1px solid var(--color-border);
	}

	th,
	td {
		text-align: left;
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
	}

	th {
		font-family: var(--font-body);
		font-weight: 600;
		font-size: var(--text-xs);
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-text-secondary);
		border-bottom: 2px solid var(--color-border);
		background: var(--color-surface-raised);
	}

	tbody tr {
		transition: background var(--duration-fast) ease;
	}

	tbody tr:hover {
		background: var(--color-surface-raised);
	}

	.tag {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		padding: 2px var(--space-2);
		border: 1px solid;
		font-family: var(--font-body);
		font-size: 9px;
		font-weight: 600;
		line-height: 1.5;
		border-radius: var(--radius-xs);
	}

	.tag-default {
		background: var(--color-surface-sunken);
		color: var(--color-text-secondary);
		border-color: var(--color-border);
	}

	.tag-success {
		background: oklch(0.95 0.03 160);
		color: oklch(0.4 0.1 160);
		border-color: oklch(0.88 0.04 160);
	}

	.tag-warning {
		background: oklch(0.96 0.03 80);
		color: oklch(0.42 0.1 80);
		border-color: oklch(0.9 0.03 80);
	}

	.tag-danger {
		background: oklch(0.95 0.03 22);
		color: oklch(0.38 0.12 22);
		border-color: oklch(0.9 0.04 22);
	}

	.footer {
		max-width: var(--max-width);
		margin: 0 auto;
		padding: var(--space-12) var(--gutter) var(--space-8);
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex-wrap: wrap;
		gap: var(--space-4);
		border-top: 1px solid var(--color-border);
	}

	.footer-brand {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 600;
		letter-spacing: -0.02em;
		color: var(--color-text-tertiary);
	}

	.footer-meta {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.08em;
		color: var(--color-text-tertiary);
		display: flex;
		gap: var(--space-6);
	}

	@media (max-width: 640px) {
		.nav-links {
			display: none;
		}

		.controls {
			flex-direction: column;
			align-items: stretch;
		}

		.summary-cards {
			grid-template-columns: repeat(2, 1fr);
		}

		.footer {
			flex-direction: column;
			align-items: flex-start;
		}

		.footer-meta {
			flex-direction: column;
			gap: var(--space-2);
		}
	}
</style>
