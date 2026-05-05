<script lang="ts">
	import Toast from './Toast.svelte';
	import { toasts } from '$lib/stores/toast';
</script>

<div class="toast-container">
	{#each $toasts as toast (toast.id)}
		<Toast 
			message={toast.message} 
			type={toast.type}
			duration={toast.duration}
			onClose={() => toasts.remove(toast.id)}
		/>
	{/each}
</div>

<style>
	.toast-container {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		pointer-events: none;
		z-index: 9999;
	}
	
	.toast-container :global(*) {
		pointer-events: auto;
	}
</style>

