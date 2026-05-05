<script lang="ts">
	import PreviewPane from '$lib/components/PreviewPane.svelte';
	import { app } from '$lib/stores/app.svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	onMount(() => {
		if (!app.previewData) {
			goto('/connect', { replaceState: true });
		}
	});

	function handleNext() {
		const sql = app.buildPreviewSql();
		app.pendingSql = sql;
		app.pendingAutoRun = true;
		goto('/query');
	}
</script>

<svelte:head>
	<title>Preview — Data Monster</title>
</svelte:head>

{#if app.previewData}
	<PreviewPane
		data={app.previewData}
		bind:columnOverrides={app.columnOverrides}
		onnext={handleNext}
	/>
{/if}
