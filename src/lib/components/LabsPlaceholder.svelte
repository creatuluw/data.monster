<script lang="ts">
	let { title, section = 'Labs' }: { title: string; section?: string } = $props();
</script>

<svelte:head>
	<title>{[title, section, 'Data Monster'].filter(Boolean).join(' — ')}</title>
</svelte:head>

<div class="page">
	<div class="section-header">
		<h1 class="section-title">{title}</h1>
	</div>

	<p class="section-subtitle">Placeholder — coming soon.</p>
</div>

<style>
	.page {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-6);
	}

	.section-header {
		display: flex;
		align-items: baseline;
		gap: var(--space-4);
	}

	.section-title {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: 700;
		letter-spacing: -0.02em;
		margin: 0;
	}

	.section-subtitle {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: var(--space-1) 0 0 0;
		max-width: 64ch;
		line-height: var(--leading-relaxed);
	}
</style>
