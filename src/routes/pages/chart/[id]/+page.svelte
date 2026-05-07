<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { app } from '$lib/stores/app.svelte';
	import { runPagedQuery } from '$lib/db-operations';
	import BarChart from '$lib/components/BarChart.svelte';
	import { ArrowLeft, XCircle } from 'lucide-svelte';

	let chartId = $derived($page.params.id);

	let chartConfig = $derived(
		chartId === 'country'
			? { labelKey: 'country', valueKey: 'total_sales', title: 'Sales by Country', tagLabel: 'countries', groupBy: 'country' }
			: { labelKey: 'product_name', valueKey: 'total_sales', title: 'Sales by Product', tagLabel: 'products', groupBy: 'product_name' }
	);

	let data = $state<Record<string, unknown>[]>([]);
	let selected = $state(new Set<string>());
	let loading = $state(true);

	async function loadChart() {
		if (app.tables.length === 0) {
			loading = false;
			return;
		}

		const table = app.tables[0];

		try {
			const result = await runPagedQuery(
				`SELECT ${chartConfig.groupBy}, ROUND(SUM(sales)::numeric, 0)::double as total_sales FROM "${table}" GROUP BY ${chartConfig.groupBy} ORDER BY total_sales DESC`,
				1,
				10000
			);
			data = result.rows;
		} catch (e) {
			console.error(e);
		}

		loading = false;
	}

	onMount(loadChart);
</script>

<svelte:head>
	<title>{chartConfig.title} — Data Monster</title>
</svelte:head>

<div class="chart-detail">
	<div class="chart-panel">
		<div class="chart-panel-header">
			<button class="back-btn" onclick={() => goto('/pages')} aria-label="Back to pages">
				<ArrowLeft size={18} />
			</button>
			<span class="back-label">Pages</span>
		</div>
		<div class="chart-panel-body">
			{#if loading}
				<span class="chart-loading">Loading…</span>
			{:else}
				<BarChart data={data} labelKey={chartConfig.labelKey} valueKey={chartConfig.valueKey} title={chartConfig.title} tagLabel="{data.length} {chartConfig.tagLabel}" bind:selected fill drawerOpen onaction={() => goto('/pages')} />
			{/if}
		</div>
	</div>

	<aside class="drawer">
		<div class="drawer-header">
			<h2 class="drawer-title">{chartConfig.title}</h2>
			<button class="close-btn" onclick={() => goto('/pages')} aria-label="Close">
				<XCircle size={18} />
			</button>
		</div>
		<div class="drawer-body">
			<p class="drawer-empty">Chart configuration coming soon.</p>
		</div>
	</aside>
</div>

<style>
	.chart-detail {
		display: flex;
		height: calc(100vh - var(--app-header-height, 57px));
		margin: calc(-1 * var(--space-6));
	}

	.chart-panel {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		border-right: 1px solid var(--color-border);
	}

	.chart-panel-header {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
		height: 41px;
		box-sizing: border-box;
	}

	.back-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: none;
		border: none;
		cursor: pointer;
		color: var(--color-text-tertiary);
		border-radius: var(--radius-xs);
		transition: color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.back-btn:hover {
		color: var(--color-text);
		background: var(--color-surface-sunken);
	}

	.back-btn:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	.back-label {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.04em;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
	}

	.chart-panel-body {
		flex: 1;
		min-height: 0;
		overflow: auto;
	}

	.chart-loading {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}

	.drawer {
		width: 50%;
		flex-shrink: 0;
		display: flex;
		flex-direction: column;
	}

	.drawer-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
		height: 41px;
		box-sizing: border-box;
	}

	.drawer-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
		margin: 0;
	}

	.close-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		background: none;
		border: none;
		cursor: pointer;
		color: var(--color-text-tertiary);
		border-radius: var(--radius-xs);
		transition: color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.close-btn:hover {
		color: var(--color-text);
		background: var(--color-surface-sunken);
	}

	.close-btn:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	.drawer-body {
		flex: 1;
		padding: var(--space-4);
		overflow-y: auto;
	}

	.drawer-empty {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}
</style>
