<script lang="ts">
	import { X } from "lucide-svelte";

	let {
		variant = "default",
		title,
		message,
		duration = 4500,
	}: {
		variant?: "default" | "success" | "warning" | "danger";
		title: string;
		message?: string;
		duration?: number;
	} = $props();

	let visible = $state(false);
	let dismissing = $state(false);
	let progressWidth = $state(100);

	function dismiss() {
		dismissing = true;
		setTimeout(() => {
			visible = false;
			dismissing = false;
		}, 450);
	}

	$effect(() => {
		visible = true;
		dismissing = false;
		progressWidth = 100;
		const start = Date.now();
		let rafId: number;
		const tick = () => {
			const elapsed = Date.now() - start;
			progressWidth = Math.max(0, 100 - (elapsed / duration) * 100);
			if (progressWidth > 0) rafId = requestAnimationFrame(tick);
		};
		rafId = requestAnimationFrame(tick);
		const timer = setTimeout(dismiss, duration);
		return () => {
			cancelAnimationFrame(rafId);
			clearTimeout(timer);
		};
	});
</script>

{#if visible}
	<div class="toast toast-{variant}" class:is-visible={visible && !dismissing} class:is-dismissing={dismissing}>
		<div class="toast-indicator"></div>
		<div class="toast-body">
			<span class="toast-title">{title}</span>
			{#if message}
				<span class="toast-message">{message}</span>
			{/if}
		</div>
		<button class="toast-close" onclick={dismiss} aria-label="Dismiss notification"><X size={14} /></button>
		<div class="toast-progress" style="width: {progressWidth}%"></div>
	</div>
{/if}

<style>
	.toast {
		display: flex;
		align-items: flex-start;
		gap: var(--space-3);
		padding: var(--space-3) var(--space-4);
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		box-shadow: var(--shadow-md);
		min-width: 300px;
		max-width: 420px;
		pointer-events: auto;
		transform: translateX(110%);
		opacity: 0;
		transition:
			transform var(--duration-slow) var(--ease-out-expo),
			opacity var(--duration-slow) var(--ease-out-expo);
		position: relative;
		overflow: hidden;
	}

	.toast.is-visible {
		transform: translateX(0);
		opacity: 1;
	}

	.toast.is-dismissing {
		transform: translateX(110%);
		opacity: 0;
	}

	.toast-indicator {
		width: 3px;
		align-self: stretch;
		flex-shrink: 0;
	}

	.toast-default .toast-indicator {
		background: var(--color-accent);
	}

	.toast-success .toast-indicator {
		background: var(--color-success);
	}

	.toast-warning .toast-indicator {
		background: var(--color-warning);
	}

	.toast-danger .toast-indicator {
		background: var(--color-danger);
	}

	.toast-body {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.toast-title {
		font-family: var(--font-body);
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-text);
	}

	.toast-message {
		font-size: var(--text-sm);
		line-height: var(--leading-normal);
		color: var(--color-text-secondary);
	}

	.toast-close {
		width: 20px;
		height: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: transparent;
		border: none;
		cursor: pointer;
		color: var(--color-text-tertiary);
		font-size: 12px;
		line-height: 1;
		flex-shrink: 0;
		transition: color var(--duration-fast) ease;
	}

	.toast-close:hover {
		color: var(--color-text);
	}

	.toast-progress {
		position: absolute;
		bottom: 0;
		left: 0;
		height: 2px;
		background: var(--color-accent);
		transform-origin: left;
		transition: transform linear;
	}

	.toast-success .toast-progress {
		background: var(--color-success);
	}

	.toast-warning .toast-progress {
		background: var(--color-warning);
	}

	.toast-danger .toast-progress {
		background: var(--color-danger);
	}
</style>
