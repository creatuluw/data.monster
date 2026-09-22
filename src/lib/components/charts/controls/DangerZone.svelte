<script lang="ts">
	import { Trash2 } from 'lucide-svelte';

	/**
	 * Drawer control kit — danger-zone panel for DrawerTabs' Danger tab.
	 * One consistent destructive confirmation surface across all drawers.
	 */
	let {
		heading,
		description,
		confirmLabel,
		onconfirm
	}: {
		heading: string;
		description: string;
		confirmLabel: string;
		onconfirm: () => void;
	} = $props();
</script>

<div class="danger">
	<div class="danger-copy">
		<p class="danger-heading">{heading}</p>
		<p class="danger-desc">{description}</p>
	</div>
	<button type="button" class="danger-btn" onclick={onconfirm}>
		<Trash2 size={13} />
		{confirmLabel}
	</button>
</div>

<style>
	.danger {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		padding: var(--space-4);
		border: 1px solid oklch(0.85 0.05 25);
		border-radius: var(--radius-md);
		background: oklch(0.97 0.015 25);
	}

	.danger-copy {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.danger-heading {
		font-size: var(--text-sm);
		font-weight: 600;
		color: oklch(0.45 0.15 25);
	}

	.danger-desc {
		font-size: var(--text-xs);
		line-height: var(--leading-normal);
		color: oklch(0.55 0.08 25 / 0.85);
	}

	.danger-btn {
		align-self: flex-start;
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		padding: var(--space-1) var(--space-3);
		height: 28px;
		border: none;
		border-radius: var(--radius-sm);
		background: var(--color-danger);
		color: white;
		font-size: var(--text-xs);
		font-weight: 500;
		cursor: pointer;
		transition: background 120ms ease;
	}

	.danger-btn:hover {
		background: oklch(0.48 0.16 25);
	}

	.danger-btn:focus-visible {
		outline: none;
		box-shadow: 0 0 0 2px oklch(0.55 0.16 25 / 0.25);
	}
</style>
