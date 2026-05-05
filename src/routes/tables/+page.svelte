<script lang="ts">
	import TableOverview from '$lib/components/TableOverview.svelte';
	import { app } from '$lib/stores/app.svelte';
	import { goto } from '$app/navigation';

	function handleSelect(table: string) {
		goto(`/table/${encodeURIComponent(table)}`);
	}
</script>

<svelte:head>
	<title>Tables — Data Monster</title>
</svelte:head>

<TableOverview tables={app.tables} onselect={handleSelect} />
