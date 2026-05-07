<script lang="ts">
	import { onMount } from 'svelte';
	import { app } from '$lib/stores/app.svelte';
	import { runPagedQuery, type PagedQueryResult, type ColumnInfo } from '$lib/db-operations';
	import BarChart from '$lib/components/BarChart.svelte';
	import { FileText } from 'lucide-svelte';
	import { goto } from '$app/navigation';

	const TABLE_PAGE_SIZE = 50;

	let salesByCountry = $state<Record<string, unknown>[]>([]);
	let salesByProduct = $state<Record<string, unknown>[]>([]);
	let loading = $state(true);
	let error = $state('');

	let tableData = $state<PagedQueryResult | null>(null);
	let tablePage = $state(1);
	let tableTotalPages = $state(1);

	let tableName = $derived(app.tables[0] ?? '');

	let selectedCountries = $state(new Set<string>());
	let selectedProducts = $state(new Set<string>());

	async function loadChartData() {
		if (app.tables.length === 0) {
			loading = false;
			return;
		}

		const table = app.tables[0];

		try {
			const [countryResult, productResult] = await Promise.all([
				runPagedQuery(
					`SELECT country, ROUND(SUM(sales)::numeric, 0)::double as total_sales FROM "${table}" ${buildWhere(['country'])} GROUP BY country ORDER BY total_sales DESC`,
					1,
					10000
				),
				runPagedQuery(
					`SELECT product_name, ROUND(SUM(sales)::numeric, 0)::double as total_sales FROM "${table}" ${buildWhere(['product_name'])} GROUP BY product_name ORDER BY total_sales DESC`,
					1,
					10000
				)
			]);
			salesByCountry = countryResult.rows;
			salesByProduct = productResult.rows;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load chart data';
		}

		loading = false;
		await loadTablePage(1);
	}

	function buildWhere(exclude: string[] = []): string {
		const conditions: string[] = [];
		if (selectedCountries.size > 0 && !exclude.includes('country')) {
			const vals = [...selectedCountries].map((s) => `'${s.replace(/'/g, "''")}'`).join(',');
			conditions.push(`country IN (${vals})`);
		}
		if (selectedProducts.size > 0 && !exclude.includes('product_name')) {
			const vals = [...selectedProducts].map((s) => `'${s.replace(/'/g, "''")}'`).join(',');
			conditions.push(`product_name IN (${vals})`);
		}
		return conditions.length > 0 ? 'WHERE ' + conditions.join(' AND ') : '';
	}

	async function refreshFromFilter() {
		if (!tableName) return;
		await loadChartData();
		await loadTablePage(1);
	}

	async function loadTablePage(page: number) {
		if (!tableName) return;
		const where = buildWhere([]);
		const countResult = await runPagedQuery(`SELECT COUNT(*) as cnt FROM "${tableName}" ${where}`, 1, 1);
		const totalRows = Number(countResult.rows[0]?.cnt ?? 0);
		const offset = (page - 1) * TABLE_PAGE_SIZE;
		const result = await runPagedQuery(`SELECT * FROM "${tableName}" ${where} LIMIT ${TABLE_PAGE_SIZE} OFFSET ${offset}`, 1, TABLE_PAGE_SIZE);
		tableData = { ...result, totalRows };
		tablePage = page;
		tableTotalPages = Math.max(1, Math.ceil(totalRows / TABLE_PAGE_SIZE));
	}

	let countryLoading = $state(false);
	let productLoading = $state(false);

	async function onCountrySelect(_label: string) {
		countryLoading = true;
		await refreshFromFilter();
		countryLoading = false;
	}

	async function onProductSelect(_label: string) {
		productLoading = true;
		await refreshFromFilter();
		productLoading = false;
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
	{:else}
		<div class="pages-content">
			{#if selectedCountries.size > 0 || selectedProducts.size > 0}
				<div class="filter-bar">
					<span class="filter-label">Active filters:</span>
					{#each [...selectedCountries] as c}
						<button class="filter-chip" onclick={() => { const s = new Set(selectedCountries); s.delete(c); selectedCountries = s; refreshFromFilter(); }}>{c} &times;</button>
					{/each}
					{#each [...selectedProducts] as p}
						<button class="filter-chip" onclick={() => { const s = new Set(selectedProducts); s.delete(p); selectedProducts = s; refreshFromFilter(); }}>{p} &times;</button>
					{/each}
					<button class="filter-clear" onclick={() => { selectedCountries = new Set(); selectedProducts = new Set(); refreshFromFilter(); }}>Clear all</button>
				</div>
			{:else}
				<div class="filter-bar filter-bar-empty">
					<span class="filter-label">No filters — click a bar to filter</span>
				</div>
			{/if}

			<div class="pages-charts">
				<BarChart data={salesByCountry} labelKey="country" valueKey="total_sales" title="Sales by Country" tagLabel="{salesByCountry.length} countries" bind:selected={selectedCountries} onselect={onCountrySelect} onaction={() => goto('/pages/chart/country')} />
				<BarChart data={salesByProduct} labelKey="product_name" valueKey="total_sales" title="Sales by Product" tagLabel="{salesByProduct.length} products" bind:selected={selectedProducts} onselect={onProductSelect} onaction={() => goto('/pages/chart/product')} />
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
