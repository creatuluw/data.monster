<script lang="ts">
	import QueryEditor from '$lib/components/QueryEditor.svelte';
	import { app } from '$lib/stores/app.svelte';
	import { goto } from '$app/navigation';
	import { runQuery, saveQueryAsTable } from '$lib/db-helpers';
	import { saveRecipe } from '$lib/recipes';
	import { onMount } from 'svelte';
	import type { PagedQueryResult } from '$lib/db-helpers';

	let queryResult = $state<PagedQueryResult | null>(null);
	let queryLoading = $state(false);
	let queryError = $state('');
	let queryTime = $state(0);
	let initialSql = $state('');

	async function handleRunQuery(sql: string, page: number = 1, pageSize: number = 10) {
		queryLoading = true;
		queryError = '';
		const t0 = performance.now();
		try {
			const result = await runQuery(sql, page, pageSize);
			queryTime = performance.now() - t0;
			queryResult = result;
			await app.refreshTables();
		} catch (e) {
			queryError = e instanceof Error ? e.message : 'Query failed';
			queryTime = performance.now() - t0;
		} finally {
			queryLoading = false;
		}
	}

	async function handleSaveAsTable(sql: string, tableName: string) {
		if (!tableName.trim()) return;
		app.globalError = '';
		try {
			await saveQueryAsTable(sql, tableName);
			saveRecipe({
				tableName,
				createdAt: new Date().toISOString(),
				source: { type: app.previewSourceType, name: app.previewSourceName },
				columnOverrides: app.columnOverrides,
				query: sql
			});
			await app.refreshTables();
			goto('/tables');
		} catch (e) {
			app.globalError = e instanceof Error ? e.message : 'Failed to save table';
		}
	}

	onMount(() => {
		if (app.pendingSql) {
			initialSql = app.pendingSql;
			const shouldAutoRun = app.pendingAutoRun;
			app.pendingSql = '';
			app.pendingAutoRun = false;

			if (shouldAutoRun && initialSql) {
				requestAnimationFrame(() => handleRunQuery(initialSql, 1, 10));
			}
		}
	});
</script>

<svelte:head>
	<title>Query — Data Monster</title>
</svelte:head>

<QueryEditor
	onrun={handleRunQuery}
	onsaveastable={handleSaveAsTable}
	result={queryResult}
	loading={queryLoading}
	error={queryError}
	queryTime={queryTime}
	initialSql={initialSql}
	suggestedTableName={app.ingestTableName}
/>
