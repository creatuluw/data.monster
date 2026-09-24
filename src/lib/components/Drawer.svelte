<script lang="ts">
	import { X } from 'lucide-svelte';
	import type { Snippet } from 'svelte';
	import { fade, fly } from 'svelte/transition';
	import { expoOut } from 'svelte/easing';
	import { Dialog } from 'bits-ui';
	import { drawerResize } from './drawer-resize';

	/**
	 * THE app drawer primitive. Right-anchored slide-in panel used by every
	 * drawer in the app (table settings, column functions, chart config, …).
	 *
	 * - `open` bindable; `onClosed` fires on every close path (X / Escape /
	 *   click-away / consumer-driven)
	 * - `overlay=true` (modal, the default): bits-ui Dialog owns the focus
	 *   trap, Escape, scroll lock, aria wiring, and focus return
	 * - `overlay=false`: non-modal focused panel — the page stays live beside
	 *   it; no focus trap (correct for non-modal), Escape still closes
	 * - `contained=true` positions inside the nearest positioned ancestor
	 *   (.app-body) instead of the viewport — needs that ancestor to span
	 *   exactly the drawer area; `data-drawer` lets parents measure geometry
	 * - `footer` optional actions row (dashed top border)
	 */
	let {
		open = $bindable(false),
		title = '',
		width = '33vw',
		overlay = true,
		contained = false,
		onClosed,
		children,
		footer
	}: {
		open?: boolean;
		title?: string;
		/** drawer width, any CSS length */
		width?: string;
		/** dim + click-away overlay; off when the page must stay live beside the drawer */
		overlay?: boolean;
		contained?: boolean;
		onClosed?: () => void;
		children: Snippet;
		footer?: Snippet;
	} = $props();

	const DURATION = 450; // --duration-slow

	let previousFocus: Element | null = null;

	// onClosed for every close path — internal (X/Escape/click-away via the
	// bind:open round-trip) and consumer-driven alike
	let wasOpen = false;
	$effect(() => {
		if (wasOpen && !open) onClosed?.();
		wasOpen = open;
	});

	// non-modal Escape (modal Escape is bits-ui's)
	$effect(() => {
		if (overlay || !open) return;
		const onKey = (e: KeyboardEvent) => {
			if (e.key === 'Escape') open = false;
		};
		window.addEventListener('keydown', onKey);
		return () => window.removeEventListener('keydown', onKey);
	});
</script>

{#if overlay}
	<Dialog.Root bind:open>
		<Dialog.Overlay forceMount>
			{#snippet child({ props, open: isOpen })}
				{#if isOpen}
					<div
						{...props}
						class="drawer-overlay"
						class:drawer-overlay-contained={contained}
						transition:fade={{ duration: DURATION }}
					></div>
				{/if}
			{/snippet}
		</Dialog.Overlay>
		<Dialog.Content
			forceMount
			aria-label={title}
			onOpenAutoFocus={() => {
				// capture before bits-ui moves focus — returned on close
				previousFocus = document.activeElement;
			}}
			onCloseAutoFocus={(e) => {
				e.preventDefault();
				if (previousFocus instanceof HTMLElement) previousFocus.focus();
				previousFocus = null;
			}}
		>
			{#snippet child({ props, open: isOpen })}
				{#if isOpen}
					<section
						{...props}
						class="drawer"
						class:drawer-open={isOpen}
						class:drawer-contained={contained}
						style={`width: ${width};`}
						use:drawerResize
						data-drawer
						transition:fly={{ x: '100%', duration: DURATION, easing: expoOut }}
					>
						<div class="drawer-header">
							<div class="drawer-heading">
								<Dialog.Title class="drawer-title">{title}</Dialog.Title>
							</div>
							<button class="drawer-close" onclick={() => (open = false)} title="Close" aria-label="Close drawer">
								<X size={16} />
							</button>
						</div>
						<div class="drawer-body">
							{@render children()}
						</div>
						{#if footer}
							<div class="drawer-footer">
								{@render footer()}
							</div>
						{/if}
					</section>
				{/if}
			{/snippet}
		</Dialog.Content>
	</Dialog.Root>
{:else}
	<!-- non-modal focused panel: page stays live, no focus trap (correct for
	     non-modal), closed panel is inert so nothing off-slide is tab-reachable -->
	<section
		class="drawer"
		class:drawer-open={open}
		class:drawer-contained={contained}
		style={`width: ${width};`}
		use:drawerResize
		data-drawer
		inert={!open}
		aria-label={title}
	>
		<div class="drawer-header">
			<div class="drawer-heading">
				<h2 class="drawer-title">{title}</h2>
			</div>
			<button class="drawer-close" onclick={() => (open = false)} title="Close" aria-label="Close drawer">
				<X size={16} />
			</button>
		</div>
		<div class="drawer-body">
			{@render children()}
		</div>
		{#if footer}
			<div class="drawer-footer">
				{@render footer()}
			</div>
		{/if}
	</section>
{/if}

<style>
	.drawer-overlay {
		position: fixed;
		inset: 0;
		background: oklch(0.14 0.01 250 / 0.6);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		z-index: 200;
	}

	/* contained mode: fill the positioned ancestor (.app-body) instead of the
	   viewport. Compound selectors so they beat the base fixed rules regardless of order */
	.drawer.drawer-contained {
		position: absolute;
		max-width: 100%;
	}

	.drawer-overlay.drawer-overlay-contained {
		position: absolute;
	}

	.drawer {
		position: fixed;
		top: 0;
		right: 0;
		bottom: 0;
		width: 33vw;
		max-width: 100vw;
		background: var(--color-surface);
		border-left: 1px solid var(--color-border);
		box-shadow: var(--shadow-lg);
		z-index: 201;
		display: flex;
		flex-direction: column;
	}

	/* non-modal path: always mounted, slides via transform (the modal path
	   only renders while open, so this closed state never applies to it) */
	.drawer-open {
		transform: translateX(0);
	}

	.drawer:not(.drawer-open) {
		transform: translateX(100%);
		visibility: hidden;
	}

	.drawer-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-3);
		padding: var(--space-3) var(--space-6);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
	}

	.drawer-heading {
		min-width: 0;
	}


	.drawer-title {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 600;
		letter-spacing: -0.01em;
		color: var(--color-text);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.drawer-close {
		width: 28px;
		height: 28px;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		cursor: pointer;
		color: var(--color-text-tertiary);
		transition: color var(--duration-fast) ease, border-color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.drawer-close:hover {
		color: var(--color-text);
		border-color: var(--color-border-strong);
		background: var(--color-surface-sunken);
	}

	.drawer-close:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	.drawer-body {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-6);
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
	}

	.drawer-footer {
		display: flex;
		align-items: center;
		justify-content: flex-start;
		gap: var(--space-3);
		padding: var(--space-4) var(--space-6);
		border-top: 1px dashed var(--color-border);
		flex-shrink: 0;
	}
</style>
