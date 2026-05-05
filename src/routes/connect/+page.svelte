<script lang="ts">
	import FileConnector from '$lib/components/FileConnector.svelte';
	import { app } from '$lib/stores/app.svelte';
	import { goto } from '$app/navigation';

	let loading = $state(false);

	async function handleFile(file: File) {
		loading = true;
		app.globalError = '';
		try {
			await app.connectFile(file);
			goto('/preview');
		} catch (e) {
			app.globalError = e instanceof Error ? e.message : 'Failed to load file';
		} finally {
			loading = false;
		}
	}

	async function handleUrl(url: string) {
		loading = true;
		app.globalError = '';
		try {
			await app.connectUrl(url);
			goto('/preview');
		} catch (e) {
			app.globalError = e instanceof Error ? e.message : 'Failed to load URL';
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Connect — Data Monster</title>
</svelte:head>

<FileConnector onfile={handleFile} onurl={handleUrl} {loading} />
