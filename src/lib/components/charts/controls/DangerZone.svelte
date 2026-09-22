<script lang="ts">
	import { Trash2 } from 'lucide-svelte';
	import Btn from './Btn.svelte';

	/**
	 * Drawer control kit — danger zone section in the /data drawer pattern:
	 * dashed divider above, "Danger zone" micro-header, an outline danger
	 * button that flips into an inline confirm panel (Cancel / Delete permanently).
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

	let confirming = $state(false);
</script>

<div class="danger">
	<h3 class="danger-title">Danger zone</h3>
	{#if confirming}
		<div class="danger-confirm">
			<p class="danger-text">{heading}? <span class="danger-desc">{description}</span></p>
			<div class="danger-actions">
				<Btn variant="secondary" size="sm" onclick={() => (confirming = false)}>Cancel</Btn>
				<Btn variant="danger" size="sm" onclick={onconfirm}>Delete permanently</Btn>
			</div>
		</div>
	{:else}
		<button type="button" class="danger-btn" onclick={() => (confirming = true)}>
			<Trash2 size={13} />
			{confirmLabel}
		</button>
	{/if}
</div>

<style>
	.danger {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		border-top: 1px dashed var(--color-border);
		padding-top: var(--space-4);
	}

	.danger-title {
		font-size: 9px;
		font-weight: 700;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.danger-btn {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		align-self: flex-start;
		padding: var(--space-2) var(--space-3);
		border: 1px solid oklch(0.86 0.035 25);
		background: oklch(0.97 0.008 25);
		color: oklch(0.38 0.12 25);
		border-radius: var(--radius-xs);
		font-family: var(--font-body);
		font-size: var(--text-xs);
		font-weight: 600;
		cursor: pointer;
		transition: all var(--duration-fast, 120ms) ease;
	}

	.danger-btn:hover {
		background: var(--color-danger);
		border-color: var(--color-danger);
		color: white;
	}

	.danger-confirm {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		padding: var(--space-3);
		border: 1px solid oklch(0.86 0.035 25);
		background: oklch(0.97 0.008 25);
		border-radius: var(--radius-sm);
	}

	.danger-text {
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
	}

	.danger-desc {
		font-weight: 400;
		color: var(--color-text-secondary);
	}

	.danger-actions {
		display: flex;
		gap: var(--space-2);
		justify-content: flex-end;
	}
</style>
