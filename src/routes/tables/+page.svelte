<script lang="ts">
	import TableOverview from '$lib/components/TableOverview.svelte';
	import { app } from '$lib/stores/app.svelte';
	import { goto } from '$app/navigation';
	import { dropTable } from '$lib/db-operations';

	function handleSelect(table: string) {
		goto(`/table/${encodeURIComponent(table)}`);
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
	}
</script>

<svelte:head>
	<title>Tables — Data Monster</title>
</svelte:head>

<TableOverview tables={app.tables} onselect={handleSelect} onrename={handleRename} ondelete={handleDelete} />
