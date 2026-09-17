<script lang="ts">
	import { X } from 'lucide-svelte';

	/** Drawer control kit — compact row-remove button (× on list rows). */
	let { onclick, title = 'Remove', disabled = false }: { onclick: () => void; title?: string; disabled?: boolean } = $props();
</script>

<button type="button" class="remove" {title} {disabled} onclick={(e) => { e.stopPropagation(); onclick(); }}>
	<X size={13} />
</button>

<style>
	.remove {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 22px;
		height: 22px;
		flex-shrink: 0;
		border: none;
		border-radius: var(--radius-xs);
		background: none;
		color: var(--color-text-tertiary);
		cursor: pointer;
		transition:
			color 120ms ease,
			background 120ms ease;
	}

	.remove:disabled {
		opacity: 0.35;
		cursor: not-allowed;
	}

	.remove:hover:not(:disabled) {
		color: var(--color-danger);
		background: oklch(0.55 0.16 25 / 0.08);
	}

	.remove:focus-visible {
		outline: none;
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}
</style>
