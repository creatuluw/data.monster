<script lang="ts">
	import TableViewer from '$lib/components/TableViewer.svelte';
	import { app } from '$lib/stores/app.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { executeQuery, dropTable, runPagedQuery } from '$lib/db-operations';
	import { onMount } from 'svelte';

	const PAGE_SIZE = 20;

	let tableName = $derived($page.params.name ?? '');
	let tableData = $state<{ columns: string[]; rows: Record<string, unknown>[]; totalRows: number } | null>(null);
	let tablePage = $state(1);
	let tableTotalPages = $state(1);

	async function loadTable(name: string, page: number) {
		if (!name) return;
		try {
			const result = await runPagedQuery(`SELECT * FROM "${name}"`, page, PAGE_SIZE);
			tableData = { columns: result.columns, rows: result.rows, totalRows: result.totalRows };
			tablePage = page;
			tableTotalPages = result.totalPages;
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
			await app.refreshTables();
			goto(app.tables.length > 0 ? '/query' : '/connect');
		} catch (e) {
			app.globalError = e instanceof Error ? e.message : 'Failed to drop table';
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
		ondroptable={handleDropTable}
	/>
{:else}
	<div style="flex: 1; display: flex; align-items: center; justify-content: center;">
		<span style="font-size: var(--text-sm); color: var(--color-text-tertiary);">Loading table...</span>
	</div>
{/if}
