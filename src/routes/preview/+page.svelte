<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	onMount(() => {
		goto('/connect', { replaceState: true });
	});
</script>

<svelte:head>
	<title>Preview — Data Monster</title>
</svelte:head>

<div class="preview-redirect">
	<span>Redirecting to connect…</span>
</div>

<style>
	.preview-redirect {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-text-tertiary);
		font-size: var(--text-sm);
	}
</style>
