<script lang="ts">
	import TableViewer from '$lib/components/TableViewer.svelte';
	import { app } from '$lib/stores/app.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { getTableData, dropTable } from '$lib/db-helpers';
	import { getRecipe, deleteRecipe } from '$lib/recipes';
	import { onMount } from 'svelte';
	import type { QueryResult } from '$lib/db-helpers';

	const PAGE_SIZE = 10;

	let tableName = $derived($page.params.name ?? '');
	let tableData = $state<(QueryResult & { totalRows: number }) | null>(null);
	let tablePage = $state(1);
	let tableTotalPages = $state(1);

	async function loadTable(name: string, page: number) {
		if (!name) return;
		try {
			tableData = await getTableData(name, page, PAGE_SIZE);
			tablePage = page;
			tableTotalPages = Math.max(1, Math.ceil(tableData.totalRows / PAGE_SIZE));
		} catch (e) {
			app.globalError = e instanceof Error ? e.message : 'Failed to load table';
		}
	}

	onMount(() => {
		loadTable(tableName, 1);
	});

	function handlePageChange(page: number) {
		loadTable(tableName, page);
	}

	function handleQueryTable() {
		app.pendingSql = `SELECT * FROM "${tableName}"\nLIMIT 100`;
		app.pendingAutoRun = false;
		goto('/query');
	}

	async function handleDropTable() {
		if (!tableName) return;
		app.globalError = '';
		try {
			await dropTable(tableName);
			deleteRecipe(tableName);
			await app.refreshTables();
			goto(app.tables.length > 0 ? '/query' : '/connect');
		} catch (e) {
			app.globalError = e instanceof Error ? e.message : 'Failed to drop table';
		}
	}

	function handleEditSource() {
		const recipe = getRecipe(tableName);
		if (!recipe) return;
		if (recipe.query) {
			app.pendingSql = recipe.query;
			app.pendingAutoRun = false;
			goto('/query');
		} else {
			goto('/connect');
		}
	}
</script>

<svelte:head>
	<title>{tableName} — Data Monster</title>
</svelte:head>

{#if tableData}
	<TableViewer
		result={tableData}
		tableName={tableName}
		page={tablePage}
		totalPages={tableTotalPages}
		onpagechange={handlePageChange}
		onquerytable={handleQueryTable}
		oneditsource={handleEditSource}
		ondroptable={handleDropTable}
		hasRecipe={!!getRecipe(tableName)}
	/>
{:else}
	<div class="flex flex-1 items-center justify-center">
		<span class="text-sm text-sand-400">Loading table...</span>
	</div>
{/if}
