<script lang="ts">
	let { children } = $props();
</script>

<div class="labs-layout">
	{@render children()}
</div>

<style>
	.labs-layout {
		display: flex;
		flex-direction: column;
		height: 100%;
		overflow: hidden;
	}
</style>