<script lang="ts">
	import { X } from 'lucide-svelte';
	import type { Snippet } from 'svelte';

	let {
		open = $bindable(false),
		title = 'Chart configuration',
		width = '30vw',
		overlay = true,
		onClosed,
		children,
	}: {
		open?: boolean;
		title?: string;
		/** drawer width, any CSS length (labs default 30vw; page config view 50vw) */
		width?: string;
		/** dim + click-away overlay; off in the focused config view (left chart stays live) */
		overlay?: boolean;
		/** config fields — plain inputs, styled via .field/.input below */
		children: Snippet;
		/** fired on close (overlay click / X) — for non-bound usage */
		onClosed?: () => void;
	} = $props();

	function close() {
		open = false;
		onClosed?.();
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="drawer-overlay" class:drawer-overlay-visible={open && overlay} onclick={close} onkeydown={() => {}}>
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<section class="drawer" class:drawer-open={open} style={`width: ${width};`} onclick={(e) => e.stopPropagation()} onkeydown={() => {}} data-drawer>
		<div class="drawer-header">
			<h2 class="drawer-title">{title}</h2>
			<button class="drawer-close" onclick={close} title="Close">
				<X size={16} />
			</button>
		</div>
		<div class="drawer-body">
			{@render children()}
		</div>
	</section>
</div>

<style>
	.drawer-overlay {
		position: fixed;
		inset: 0;
		background: oklch(0 0 0 / 0.3);
		z-index: 200;
		opacity: 0;
		transition: opacity var(--duration-base) ease;
		pointer-events: none;
	}

	.drawer-overlay-visible {
		opacity: 1;
		pointer-events: auto;
	}

	.drawer {
		position: fixed;
		top: 0;
		right: 0;
		bottom: 0;
		width: 30vw;
		max-width: 100vw;
		background: var(--color-surface);
		border-left: 1px solid var(--color-border);
		z-index: 201;
		display: flex;
		flex-direction: column;
		transform: translateX(100%);
		transition: transform var(--duration-base) var(--ease-out-expo);
		box-shadow: var(--shadow-lg);
	}

	.drawer-open {
		transform: translateX(0);
	}

	.drawer-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
	}

	.drawer-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 700;
		color: var(--color-text);
		letter-spacing: -0.01em;
	}

	.drawer-close {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-1);
		border: none;
		background: none;
		color: var(--color-text-tertiary);
		cursor: pointer;
		border-radius: var(--radius-xs);
		transition:
			color var(--duration-fast) ease,
			background var(--duration-fast) ease;
	}

	.drawer-close:hover {
		color: var(--color-text);
		background: var(--color-surface-sunken);
	}

	.drawer-body {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-4);
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	/* shared field styles for the config inputs rendered into the body */
	.drawer :global(.field) {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.drawer :global(.field-label) {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.drawer :global(.field-hint) {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
	}

	.drawer :global(.input) {
		padding: var(--space-2) var(--space-3);
		border: 1px solid var(--color-border-strong);
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		color: var(--color-text);
		background: var(--color-surface);
		border-radius: var(--radius-xs);
		transition:
			border-color var(--duration-fast) ease,
			box-shadow var(--duration-fast) ease;
	}

	.drawer :global(.input:focus) {
		outline: none;
		border-color: var(--color-accent);
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}
</style>
