<script lang="ts">
	import TableOverview from '$lib/components/TableOverview.svelte';
	import TableDrawer from '$lib/components/TableDrawer.svelte';
	import { app } from '$lib/stores/app.svelte';
	import { goto } from '$app/navigation';
	import { dropTable } from '$lib/db-operations';

	let drawerTable = $state<string | null>(null);

	function handleSelect(table: string) {
		goto(`/table/${encodeURIComponent(table)}`);
	}

	function handleOpenDrawer(table: string) {
		drawerTable = table;
	}

	function handleCloseDrawer() {
		drawerTable = null;
	}

	async function handleRename(_oldName: string, _newName: string) {
		await app.refreshTables();
	}

	async function handleDelete(tableName: string) {
		app.globalError = '';
		try {
			await dropTable(tableName);
			await app.refreshTables();
		} catch (e) {
			app.globalError = e instanceof Error ? e.message : 'Failed to delete table';
		}
		drawerTable = null;
	}
</script>

<svelte:head>
	<title>Data — Data Monster</title>
</svelte:head>

<div class="data-page">
	<TableOverview
		tables={app.tables}
		onselect={handleSelect}
		onopendrawer={handleOpenDrawer}
		ondelete={handleDelete}
	/>
</div>

{#if drawerTable}
	<TableDrawer
		tableName={drawerTable}
		onclose={handleCloseDrawer}
		onrename={handleRename}
		ondelete={handleDelete}
	/>
{/if}

<style>
	:global(.app-body) {
		min-height: 0;
	}

	:global(.app-main) {
		overflow: hidden !important;
		padding: 0 !important;
		display: flex !important;
		flex-direction: column !important;
	}

	.data-page {
		flex: 1;
		overflow: hidden;
	}
</style>