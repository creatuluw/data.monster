<script lang="ts">
	import { X } from "lucide-svelte";

	let {
		open = $bindable(false),
		ref,
		title,
		body,
		footer,
	}: {
		open?: boolean;
		ref?: string;
		title?: import("svelte").Snippet;
		body?: import("svelte").Snippet;
		footer?: import("svelte").Snippet;
	} = $props();

	function closeOnBackdrop(e: MouseEvent) {
		if (e.target === e.currentTarget) open = false;
	}
</script>

{#if open}
	<div
		class="modal-backdrop is-open"
		role="dialog"
		aria-modal="true"
		onclick={closeOnBackdrop}
		onkeydown={(e) => {
			if (e.key === "Escape") open = false;
		}}
	>
		<div class="modal">
			<div class="modal-header">
				<div class="modal-header-left">
					{#if ref}
						<span class="modal-header-ref">{ref}</span>
					{/if}
					{#if title}
						<h2 class="modal-title">
							{@render title()}
						</h2>
					{/if}
				</div>
				<button class="modal-close" aria-label="Close modal" onclick={() => (open = false)}>
					<X size={16} />
				</button>
			</div>
			{#if body}
				<div class="modal-body">
					{@render body()}
				</div>
			{/if}
			{#if footer}
				<div class="modal-footer">
					{@render footer()}
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	.modal-backdrop {
		position: fixed;
		inset: 0;
		background: oklch(0.14 0.01 250 / 0.6);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		z-index: 1000;
		display: flex;
		align-items: center;
		justify-content: center;
		opacity: 0;
		visibility: hidden;
		transition:
			opacity var(--duration-slow) var(--ease-out-expo),
			visibility var(--duration-slow);
	}

	.modal-backdrop.is-open {
		opacity: 1;
		visibility: visible;
	}

	.modal {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		box-shadow: var(--shadow-lg);
		width: 90%;
		max-width: 680px;
		transform: translateY(12px) scale(0.98);
		transition: transform var(--duration-slow) var(--ease-out-expo);
	}

	.modal-backdrop.is-open .modal {
		transform: translateY(0) scale(1);
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-6);
		border-bottom: 1px solid var(--color-border);
	}

	.modal-header-left {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.modal-header-ref {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		color: var(--color-accent);
	}

	.modal-title {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 600;
		letter-spacing: -0.01em;
		color: var(--color-text);
	}

	.modal-close {
		width: 28px;
		height: 28px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		cursor: pointer;
		color: var(--color-text-tertiary);
		font-size: 14px;
		line-height: 1;
		transition:
			color var(--duration-fast) ease,
			border-color var(--duration-fast) ease,
			background var(--duration-fast) ease;
	}

	.modal-close:hover {
		color: var(--color-text);
		border-color: var(--color-border-strong);
		background: var(--color-surface-sunken);
	}

	.modal-close:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	.modal-body {
		padding: var(--space-6);
	}

	.modal-footer {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: var(--space-3);
		padding: var(--space-4) var(--space-6);
		border-top: 1px dashed var(--color-border);
	}
</style>
