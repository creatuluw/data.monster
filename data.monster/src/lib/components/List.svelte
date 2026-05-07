<script lang="ts">
	type ListItem = {
		label: string;
		value: string;
	};

	let {
		items,
	}: {
		items: ListItem[];
	} = $props();
</script>

<dl class="list">
	{#each items as item}
		<div class="list-row">
			<dt class="list-label">{item.label}</dt>
			<dd class="list-value">{item.value}</dd>
		</div>
	{/each}
</dl>

<style>
	.list {
		display: flex;
		flex-direction: column;
		margin: 0;
	}

	.list-row {
		display: grid;
		grid-template-columns: 10ch 1fr;
		gap: var(--space-4);
		align-items: baseline;
		padding: var(--space-3) 0;
		border-bottom: 1px solid var(--color-grid);
	}

	.list-row:last-child {
		border-bottom: none;
	}

	.list-label {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
		margin: 0;
	}

	.list-value {
		font-size: var(--text-sm);
		color: var(--color-text);
		line-height: var(--leading-relaxed);
		margin: 0;
	}
</style>
