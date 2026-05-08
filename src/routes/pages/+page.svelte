<script lang="ts">
	import { onMount } from 'svelte';
	import { app } from '$lib/stores/app.svelte';
	import { runPagedQuery, getTableMeta, type PagedQueryResult, type ColumnInfo, extractErrorMessage } from '$lib/db-operations';
	import BarChart from '$lib/components/BarChart.svelte';
	import { FileText } from 'lucide-svelte';
	import { goto } from '$app/navigation';

	const TABLE_PAGE_SIZE = 50;

	const CATEGORICAL_TYPES = new Set([
		'VARCHAR', 'TEXT', 'STRING', 'CHAR', 'BPCHAR', 'NAME', 'UUID', 'ENUM', 'BOOLEAN', 'BOOL'
	]);
	const NUMERIC_TYPES = new Set([
		'INTEGER', 'BIGINT', 'SMALLINT', 'TINYINT', 'INT', 'INT2', 'INT4', 'INT8',
		'DOUBLE', 'FLOAT', 'FLOAT4', 'FLOAT8', 'REAL', 'DECIMAL', 'NUMERIC',
		'HUGEINT', 'UINTEGER', 'UBIGINT', 'USMALLINT', 'UTINYINT'
	]);

	interface ChartConfig {
		categoryCol: string;
		valueCol: string;
		title: string;
		selected: Set<string>;
	}

	let chartData1 = $state<Record<string, unknown>[]>([]);
	let chartData2 = $state<Record<string, unknown>[]>([]);
	let loading = $state(true);
	let error = $state('');
	let noChartData = $state(false);

	let chart1 = $state<ChartConfig | null>(null);
	let chart2 = $state<ChartConfig | null>(null);

	let tableData = $state<PagedQueryResult | null>(null);
	let tablePage = $state(1);
	let tableTotalPages = $state(1);

	let tableName = $derived(app.tables[0] ?? '');

	async function loadChartData() {
		if (app.tables.length === 0) {
			loading = false;
			return;
		}

		const table = app.tables[0];

		try {
			const meta = await getTableMeta(table);
			const catCols = meta.columns.filter((c) => CATEGORICAL_TYPES.has(c.type.toUpperCase()) || !NUMERIC_TYPES.has(c.type.toUpperCase()));
			const numCols = meta.columns.filter((c) => NUMERIC_TYPES.has(c.type.toUpperCase()));

			if (catCols.length === 0 || numCols.length === 0) {
				noChartData = true;
				loading = false;
				await loadTablePage(1);
				return;
			}

			const valCol = numCols[0].name;
			const cat1 = catCols[0].name;
			const cat2 = catCols.length > 1 ? catCols[1].name : cat1;

			chart1 = { categoryCol: cat1, valueCol: valCol, title: `${valCol} by ${cat1}`, selected: new Set() };
			chart2 = catCols.length > 1
				? { categoryCol: cat2, valueCol: valCol, title: `${valCol} by ${cat2}`, selected: new Set() }
				: null;

			const queries = [
				runPagedQuery(
					`SELECT "${cat1}", ROUND(SUM("${valCol}")::numeric, 0)::double as total_${valCol} FROM "${table}" ${buildWhere([{ col: cat1, selected: chart1.selected }, { col: cat2, selected: chart2?.selected }], cat1)} GROUP BY "${cat1}" ORDER BY total_${valCol} DESC`,
					1,
					10000
				)
			];
			if (chart2) {
				queries.push(
					runPagedQuery(
						`SELECT "${cat2}", ROUND(SUM("${valCol}")::numeric, 0)::double as total_${valCol} FROM "${table}" ${buildWhere([{ col: cat1, selected: chart1.selected }, { col: cat2, selected: chart2.selected }], cat2)} GROUP BY "${cat2}" ORDER BY total_${valCol} DESC`,
						1,
						10000
					)
				);
			}

			const results = await Promise.all(queries);
			chartData1 = results[0].rows;
			if (results[1]) chartData2 = results[1].rows;
		} catch (e) {
			error = extractErrorMessage(e, 'Failed to load chart data');
		}

		loading = false;
		await loadTablePage(1);
	}

	interface FilterDef {
		col: string;
		selected: Set<string> | undefined;
	}

	function buildWhere(filters: FilterDef[], exclude?: string): string {
		const conditions: string[] = [];
		for (const f of filters) {
			if (f.selected && f.selected.size > 0 && f.col !== exclude) {
				const vals = [...f.selected].map((s) => `'${s.replace(/'/g, "''")}'`).join(',');
				conditions.push(`"${f.col}" IN (${vals})`);
			}
		}
		return conditions.length > 0 ? 'WHERE ' + conditions.join(' AND ') : '';
	}

	async function refreshFromFilter() {
		if (!tableName) return;
		await loadChartData();
	}

	async function loadTablePage(page: number) {
		if (!tableName) return;
		const filters: FilterDef[] = [];
		if (chart1) filters.push({ col: chart1.categoryCol, selected: chart1.selected });
		if (chart2) filters.push({ col: chart2.categoryCol, selected: chart2.selected });
		const where = buildWhere(filters);
		const countResult = await runPagedQuery(`SELECT COUNT(*) as cnt FROM "${tableName}" ${where}`, 1, 1);
		const totalRows = Number(countResult.rows[0]?.cnt ?? 0);
		const offset = (page - 1) * TABLE_PAGE_SIZE;
		const result = await runPagedQuery(`SELECT * FROM "${tableName}" ${where} LIMIT ${TABLE_PAGE_SIZE} OFFSET ${offset}`, 1, TABLE_PAGE_SIZE);
		tableData = { ...result, totalRows };
		tablePage = page;
		tableTotalPages = Math.max(1, Math.ceil(totalRows / TABLE_PAGE_SIZE));
	}

	let chart1Loading = $state(false);
	let chart2Loading = $state(false);

	async function onChart1Select(_label: string) {
		chart1Loading = true;
		await refreshFromFilter();
		chart1Loading = false;
	}

	async function onChart2Select(_label: string) {
		chart2Loading = true;
		await refreshFromFilter();
		chart2Loading = false;
	}

	function formatCell(value: unknown): string {
		if (value === null || value === undefined) return '\u2014';
		if (typeof value === 'object') return JSON.stringify(value);
		return String(value);
	}

	onMount(loadChartData);
</script>

<svelte:head>
	<title>Pages — Data Monster</title>
</svelte:head>

<div class="pages">
	{#if app.tables.length === 0}
		<div class="pages-empty">
			<FileText size={32} />
			<h2 class="pages-title">Pages</h2>
			<p class="pages-desc">Connect data to see charts and insights.</p>
		</div>
	{:else if loading}
		<div class="pages-loading">
			<span class="pages-loading-text">Loading…</span>
		</div>
	{:else if error}
		<div class="pages-error">
			<span>{error}</span>
		</div>
	{:else if noChartData}
		<div class="pages-content">
			<div class="pages-nochart">
				<p>No chartable columns found. The table needs at least one text column and one numeric column.</p>
			</div>
			{#if tableData}
				<div class="pages-table-section">
					<div class="pages-table-header">
						<h3 class="pages-table-title">{tableName}</h3>
						<span class="pages-table-count">{tableData.totalRows.toLocaleString()} rows</span>
					</div>
					<div class="pages-table-wrap">
						<table class="data-table">
							<thead>
								<tr>
									{#each tableData.columns as col}
										<th>{col}</th>
									{/each}
								</tr>
							</thead>
							<tbody>
								{#each tableData.rows as row}
									<tr>
										{#each tableData.columns as col}
											<td>{formatCell(row[col])}</td>
										{/each}
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
					{#if tableTotalPages > 1}
						<div class="pagination">
							<button onclick={() => loadTablePage(tablePage - 1)} disabled={tablePage <= 1} class="btn btn-ghost btn-sm">&larr; prev</button>
							<span class="page-info">Page {tablePage} of {tableTotalPages}</span>
							<button onclick={() => loadTablePage(tablePage + 1)} disabled={tablePage >= tableTotalPages} class="btn btn-ghost btn-sm">next &rarr;</button>
						</div>
					{/if}
				</div>
			{/if}
		</div>
	{:else}
		<div class="pages-content">
			{#if (chart1 && chart1.selected.size > 0) || (chart2 && chart2.selected.size > 0)}
				<div class="filter-bar">
					<span class="filter-label">Active filters:</span>
					{#if chart1}
						{#each [...chart1.selected] as c}
							<button class="filter-chip" onclick={() => { const s = new Set(chart1!.selected); s.delete(c); chart1!.selected = s; refreshFromFilter(); }}>{c} &times;</button>
						{/each}
					{/if}
					{#if chart2}
						{#each [...chart2.selected] as p}
							<button class="filter-chip" onclick={() => { const s = new Set(chart2!.selected); s.delete(p); chart2!.selected = s; refreshFromFilter(); }}>{p} &times;</button>
						{/each}
					{/if}
					<button class="filter-clear" onclick={() => { if (chart1) chart1.selected = new Set(); if (chart2) chart2.selected = new Set(); refreshFromFilter(); }}>Clear all</button>
				</div>
			{:else}
				<div class="filter-bar filter-bar-empty">
					<span class="filter-label">No filters — click a bar to filter</span>
				</div>
			{/if}

			<div class="pages-charts" class:pages-charts-single={!chart2}>
				{#if chart1}
					<BarChart data={chartData1} labelKey={chart1.categoryCol} valueKey={`total_${chart1.valueCol}`} title={chart1.title} tagLabel="{chartData1.length} {chart1.categoryCol}" bind:selected={chart1.selected} onselect={onChart1Select} onaction={() => goto(`/pages/chart/${encodeURIComponent(chart1!.categoryCol)}`)} />
				{/if}
				{#if chart2}
					<BarChart data={chartData2} labelKey={chart2.categoryCol} valueKey={`total_${chart2.valueCol}`} title={chart2.title} tagLabel="{chartData2.length} {chart2.categoryCol}" bind:selected={chart2.selected} onselect={onChart2Select} onaction={() => goto(`/pages/chart/${encodeURIComponent(chart2!.categoryCol)}`)} />
				{/if}
			</div>

			{#if tableData}
				<div class="pages-table-section">
					<div class="pages-table-header">
						<h3 class="pages-table-title">{tableName}</h3>
						<span class="pages-table-count">{tableData.totalRows.toLocaleString()} rows</span>
					</div>
					<div class="pages-table-wrap">
						<table class="data-table">
							<thead>
								<tr>
									{#each tableData.columns as col}
										<th>{col}</th>
									{/each}
								</tr>
							</thead>
							<tbody>
								{#each tableData.rows as row}
									<tr>
										{#each tableData.columns as col}
											<td>{formatCell(row[col])}</td>
										{/each}
									</tr>
								{/each}
							</tbody>
						</table>
					</div>

					{#if tableTotalPages > 1}
						<div class="pagination">
							<button
								onclick={() => loadTablePage(tablePage - 1)}
								disabled={tablePage <= 1}
								class="btn btn-ghost btn-sm"
							>
								&larr; prev
							</button>
							<span class="page-info">Page {tablePage} of {tableTotalPages}</span>
							<button
								onclick={() => loadTablePage(tablePage + 1)}
								disabled={tablePage >= tableTotalPages}
								class="btn btn-ghost btn-sm"
							>
								next &rarr;
							</button>
						</div>
					{/if}
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.pages {
		flex: 1;
		display: flex;
	}

	.pages-empty {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: var(--space-3);
		color: var(--color-text-tertiary);
	}

	.pages-title {
		font-family: var(--font-display);
		font-size: var(--text-lg);
		font-weight: 600;
		color: var(--color-text);
	}

	.pages-desc {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}

	.pages-loading {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.pages-loading-text {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}

	.pages-error {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-6);
		font-size: var(--text-sm);
		color: var(--color-danger);
		border: 1px dashed var(--color-danger);
		background: oklch(0.96 0.02 22);
		border-radius: var(--radius-sm);
	}

	.pages-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
		padding: var(--space-6);
		overflow: auto;
	}

	.pages-nochart {
		padding: var(--space-4) var(--space-6);
		background: var(--color-surface-sunken);
		border: 1px dashed var(--color-border);
		border-radius: var(--radius-sm);
		text-align: center;
	}

	.pages-nochart p {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: 0;
	}

	.pages-charts-single {
		grid-template-columns: 1fr;
	}

	.filter-bar {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		flex-wrap: wrap;
		padding: var(--space-2) var(--space-3);
		background: var(--color-surface-sunken);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
	}

	.filter-label {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.04em;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
	}

	.filter-chip {
		display: inline-flex;
		align-items: center;
		gap: 2px;
		padding: 2px var(--space-2);
		border: 1px solid #3b82f6;
		background: oklch(0.95 0.03 250);
		color: #3b82f6;
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		border-radius: var(--radius-xs);
		cursor: pointer;
		white-space: nowrap;
		max-width: 20ch;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.filter-chip:hover {
		background: oklch(0.92 0.04 250);
	}

	.filter-clear {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		background: none;
		border: 1px dashed var(--color-border);
		padding: 2px var(--space-2);
		border-radius: var(--radius-xs);
		cursor: pointer;
		margin-left: auto;
	}

	.filter-clear:hover {
		color: var(--color-text);
		border-color: var(--color-text-tertiary);
	}

	.filter-bar-empty {
		opacity: 0.5;
	}

	.pages-charts {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: var(--space-6);
	}

	.pages-table-section {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.pages-table-header {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.pages-table-title {
		font-family: var(--font-display);
		font-size: var(--text-base);
		font-weight: 700;
		color: var(--color-text);
	}

	.pages-table-count {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.04em;
		color: var(--color-text-tertiary);
		padding: 2px var(--space-2);
		background: var(--color-surface-raised);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
	}

	.pages-table-wrap {
		overflow-x: auto;
		border: 1px solid var(--color-border);
		background: var(--color-surface);
	}

	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-3);
	}

	.page-info {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.04em;
	}
</style>
