<script lang="ts">
	import { X } from "lucide-svelte";

	let {
		open = $bindable(false),
		mode = "slide",
		flip = false,
		overlay = true,
		title,
		body,
		footer,
		ref,
		style,
	}: {
		open?: boolean;
		mode?: "slide" | "push" | "reveal" | "none";
		flip?: boolean;
		overlay?: boolean;
		title?: import("svelte").Snippet;
		body?: import("svelte").Snippet;
		footer?: import("svelte").Snippet;
		ref?: string;
		style?: string;
	} = $props();

	let drawerEl = $state<HTMLDivElement | undefined>(undefined);

	function closeOnBackdrop(e: MouseEvent) {
		if (e.target === e.currentTarget) open = false;
	}

	let animClass = $derived(
		mode === "push"
			? "drawer-push"
			: mode === "reveal"
				? "drawer-reveal"
				: mode === "none"
					? "drawer-none"
					: "drawer-slide"
	);

	let hasOverlay = $derived(mode === "slide" && overlay);

	let pushes = $derived(mode === "push" || mode === "reveal");

	$effect(() => {
		if (open && pushes && drawerEl) {
			const w = drawerEl.getBoundingClientRect().width;
			const trans = `margin var(--duration-slow) var(--ease-out-expo)`;
			document.body.style.transition = trans;
			if (flip) {
				document.body.style.marginRight = `${w}px`;
			} else {
				document.body.style.marginLeft = `${w}px`;
			}
		} else {
			document.body.style.marginLeft = "";
			document.body.style.marginRight = "";
		}
		return () => {
			document.body.style.marginLeft = "";
			document.body.style.marginRight = "";
		};
	});

	$effect(() => {
		if (open) {
			document.documentElement.style.overflow = "hidden";
			document.body.style.overflow = "hidden";
		} else {
			document.documentElement.style.overflow = "";
			document.body.style.overflow = "";
		}
		return () => {
			document.documentElement.style.overflow = "";
			document.body.style.overflow = "";
		};
	});
</script>

{#if open}
	<div
		class="drawer-backdrop"
		class:drawer-overlay={hasOverlay}
		class:drawer-no-overlay={!hasOverlay}
		onclick={closeOnBackdrop}
		{style}
		onkeydown={(e) => {
			if (e.key === "Escape") open = false;
		}}
	>
		<div
			class="drawer-bar {animClass}"
			class:drawer-flip={flip}
			bind:this={drawerEl}
			role="dialog"
			aria-modal={hasOverlay ? "true" : undefined}
		>
			<div class="drawer-header">
				<div class="drawer-header-left">
					{#if ref}
						<span class="drawer-ref">{ref}</span>
					{/if}
					{#if title}
						<h2 class="drawer-title">
							{@render title()}
						</h2>
					{/if}
				</div>
				<button class="drawer-close" aria-label="Close drawer" onclick={() => (open = false)}>
					<X size={16} />
				</button>
			</div>
			{#if body}
				<div class="drawer-body">
					{@render body()}
				</div>
			{/if}
			{#if footer}
				<div class="drawer-footer">
					{@render footer()}
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	.drawer-backdrop {
		position: fixed;
		inset: 0;
		z-index: 1000;
		display: flex;
		justify-content: flex-start;
		visibility: hidden;
	}

	.drawer-overlay {
		background: oklch(0.14 0.01 250 / 0.6);
		backdrop-filter: blur(4px);
		-webkit-backdrop-filter: blur(4px);
		opacity: 0;
		transition:
			opacity var(--duration-slow) var(--ease-out-expo),
			visibility var(--duration-slow);
	}

	.drawer-no-overlay {
		transition: visibility var(--duration-slow);
	}

	.drawer-backdrop:has(.drawer-bar) {
		visibility: visible;
	}

	.drawer-overlay:has(.drawer-bar) {
		opacity: 1;
	}

	.drawer-flip {
		margin-left: auto;
	}

	.drawer-bar {
		display: flex;
		flex-direction: column;
		width: var(--drawer-width, 25vw);
		max-width: 90vw;
		height: 100%;
		background: var(--color-surface);
		border-right: 1px solid var(--color-border);
		box-shadow: var(--shadow-lg);
		overflow-y: auto;
	}

	.drawer-flip {
		border-right: none;
		border-left: 1px solid var(--color-border);
	}

	.drawer-slide {
		animation: slideIn var(--duration-slow) var(--ease-out-expo) both;
	}

	.drawer-slide.drawer-flip {
		animation-name: slideInFlip;
	}

	.drawer-push {
		animation: pushIn var(--duration-slow) var(--ease-out-expo) both;
	}

	.drawer-push.drawer-flip {
		animation-name: pushInFlip;
	}

	.drawer-reveal {
		animation: revealIn var(--duration-slow) var(--ease-out-expo) both;
	}

	.drawer-reveal.drawer-flip {
		animation-name: revealInFlip;
	}

	.drawer-none {
		animation: fadeIn var(--duration-fast) ease both;
	}

	@keyframes slideIn {
		from { transform: translateX(-100%); }
		to { transform: translateX(0); }
	}

	@keyframes slideInFlip {
		from { transform: translateX(100%); }
		to { transform: translateX(0); }
	}

	@keyframes pushIn {
		from { transform: translateX(-100%); }
		to { transform: translateX(0); }
	}

	@keyframes pushInFlip {
		from { transform: translateX(100%); }
		to { transform: translateX(0); }
	}

	@keyframes revealIn {
		from { clip-path: inset(0 100% 0 0); }
		to { clip-path: inset(0 0 0 0); }
	}

	@keyframes revealInFlip {
		from { clip-path: inset(0 0 0 100%); }
		to { clip-path: inset(0 0 0 0); }
	}

	@keyframes fadeIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}

	.drawer-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--gutter);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
	}

	.drawer-header-left {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.drawer-ref {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		color: var(--color-accent);
	}

	.drawer-title {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 600;
		letter-spacing: -0.01em;
		color: var(--color-text);
	}

	.drawer-close {
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
		transition:
			color var(--duration-fast) ease,
			border-color var(--duration-fast) ease,
			background var(--duration-fast) ease;
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
		padding: var(--space-6);
		flex: 1;
		overflow-y: auto;
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

	@media (max-width: 640px) {
		.drawer-bar {
			width: 95vw;
		}
	}
</style>
