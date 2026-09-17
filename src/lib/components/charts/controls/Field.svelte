<script lang="ts">
	/**
	 * Drawer control kit — shared field wrapper. Label + optional hint above the
	 * control; `inline` renders label and control on one row instead.
	 */
	import type { Snippet } from 'svelte';

	let {
		label,
		hint,
		inline = false,
		children
	}: {
		label: string;
		hint?: string;
		inline?: boolean;
		children: Snippet;
	} = $props();
</script>

<label class="field" class:field-inline={inline}>
	<span class="field-label">{label}</span>
	{#if hint}<span class="field-hint">{hint}</span>{/if}
	{@render children()}
</label>

<style>
	.field {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
		min-width: 0;
	}

	.field-inline {
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-3);
	}

	.field-label {
		font-size: var(--text-xs);
		font-weight: 500;
		color: var(--color-text-secondary);
		letter-spacing: 0.01em;
	}

	.field-inline .field-label {
		flex-shrink: 0;
	}

	.field-hint {
		font-size: 10px;
		color: var(--color-text-tertiary);
		margin-top: -2px;
	}
</style>
