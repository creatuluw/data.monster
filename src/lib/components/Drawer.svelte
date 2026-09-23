<script lang="ts">
	import { X } from 'lucide-svelte';
	import type { Snippet } from 'svelte';
	import { drawerResize } from './drawer-resize';

	/**
	 * THE app drawer primitive. Right-anchored slide-in panel used by every
	 * drawer in the app (table settings, column functions, chart config, …).
	 *
	 * - `open` bindable; `onClosed` fires after close (overlay click / X / Escape)
	 * - `overlay=false` keeps the page live beside the drawer (focused config view)
	 * - `contained=true` positions inside the nearest positioned ancestor
	 *   (.app-body) instead of the viewport — needs that ancestor to span
	 *   exactly the drawer area; `data-drawer` lets parents measure geometry
	 * - `kicker` is the small mono reference line above the title
	 * - `footer` optional actions row (dashed top border)
	 * - a11y: Escape closes, focus is trapped while open and returned on close
	 */
	let {
		open = $bindable(false),
		title = '',
		kicker = '',
		width = '33vw',
		overlay = true,
		contained = false,
		onClosed,
		children,
		footer
	}: {
		open?: boolean;
		title?: string;
		kicker?: string;
		/** drawer width, any CSS length */
		width?: string;
		/** dim + click-away overlay; off when the page must stay live beside the drawer */
		overlay?: boolean;
		contained?: boolean;
		onClosed?: () => void;
		children: Snippet;
		footer?: Snippet;
	} = $props();

	let previousFocus: Element | null = null;

	function close() {
		if (!open) return;
		open = false;
		onClosed?.();
		if (previousFocus instanceof HTMLElement) previousFocus.focus();
		previousFocus = null;
	}

	function handleBackdropKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			close();
			return;
		}
		if (e.key !== 'Tab') return;
		const backdrop = e.currentTarget as HTMLElement | null;
		if (!backdrop) return;
		const focusable = backdrop.querySelectorAll('button, input, [tabindex]:not([tabindex="-1"])');
		if (focusable.length === 0) return;
		const first = focusable[0] as HTMLElement;
		const last = focusable[focusable.length - 1] as HTMLElement;
		if (e.shiftKey && document.activeElement === first) {
			e.preventDefault();
			last.focus();
		} else if (!e.shiftKey && document.activeElement === last) {
			e.preventDefault();
			first.focus();
		}
	}

	$effect(() => {
		if (open && overlay) {
			previousFocus = document.activeElement;
			document.body.style.overflow = 'hidden';
			return () => {
				document.body.style.overflow = '';
			};
		}
	});
</script>

<!-- dim backdrop only in overlay mode — the drawer itself must NEVER sit inside
     the overlay: opacity:0 on the overlay hides its whole subtree -->
{#if overlay}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="drawer-overlay"
		class:drawer-overlay-visible={open}
		class:drawer-overlay-contained={contained}
		onclick={close}
		onkeydown={handleBackdropKeydown}
		role="dialog"
		aria-modal="true"
		aria-label={title}
	></div>
{/if}
<!-- svelte-ignore a11y_no_static_element_interactions -->
<section
	class="drawer"
	class:drawer-open={open}
	class:drawer-contained={contained}
	style={`width: ${width};`}
	use:drawerResize
	onclick={(e) => e.stopPropagation()}
	onkeydown={handleBackdropKeydown}
	data-drawer
>
	<div class="drawer-header">
		<div class="drawer-heading">
			{#if kicker}<span class="drawer-kicker">{kicker}</span>{/if}
			<h2 class="drawer-title">{title}</h2>
		</div>
		<button class="drawer-close" onclick={close} title="Close" aria-label="Close drawer">
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

<style>
	.drawer-overlay {
		position: fixed;
		inset: 0;
		background: oklch(0.14 0.01 250 / 0.6);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		z-index: 200;
		opacity: 0;
		transition: opacity var(--duration-slow) var(--ease-out-expo);
		visibility: hidden;
		pointer-events: none;
	}

	.drawer-overlay-visible {
		opacity: 1;
		visibility: visible;
		pointer-events: auto;
	}

	/* contained mode: fill the positioned ancestor (.app-body) instead of the
	   viewport — its overflow:hidden also clips the closed off-slide state.
	   Compound selectors so they beat the base fixed rules regardless of order */
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
		transform: translateX(100%);
		transition: transform var(--duration-slow) var(--ease-out-expo);
	}

	.drawer-open {
		transform: translateX(0);
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

	.drawer-kicker {
		display: block;
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--color-accent);
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
