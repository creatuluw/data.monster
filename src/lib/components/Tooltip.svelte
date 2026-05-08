<script lang="ts">
	let {
		text,
		position = "top",
		block = false,
		children,
	}: {
		text?: string;
		position?: "top" | "bottom" | "left" | "right";
		block?: boolean;
		children?: import("svelte").Snippet;
	} = $props();

	let visible = $state(false);
	let delayTimer: ReturnType<typeof setTimeout> | null = null;

	function show() {
		if (!text) return;
		delayTimer = setTimeout(() => (visible = true), 400);
	}

	function hide() {
		if (delayTimer) {
			clearTimeout(delayTimer);
			delayTimer = null;
		}
		visible = false;
	}
</script>

{#if block}
	<div class="tooltip-wrap tooltip-wrap-block" onmouseenter={show} onmouseleave={hide} role="presentation">
		{#if children}{@render children()}{/if}
		{#if visible && text}
			<span class="tooltip tooltip-{position}" role="tooltip">{text}</span>
		{/if}
	</div>
{:else}
	<span class="tooltip-wrap" onmouseenter={show} onmouseleave={hide} role="presentation">
		{#if children}{@render children()}{/if}
		{#if visible && text}
			<span class="tooltip tooltip-{position}" role="tooltip">{text}</span>
		{/if}
	</span>
{/if}

<style>
	.tooltip-wrap {
		position: relative;
		display: inline-flex;
	}

	.tooltip-wrap-block {
		display: flex;
		flex-direction: column;
		width: 100%;
	}

	.tooltip {
		position: absolute;
		z-index: 200;
		padding: var(--space-1) var(--space-2);
		background: var(--color-text);
		color: var(--color-text-on-accent);
		font-family: var(--font-body);
		font-size: 9px;
		font-weight: 500;
		line-height: 1.5;
		letter-spacing: 0.01em;
		white-space: nowrap;
		border-radius: var(--radius-xs);
		box-shadow: var(--shadow-md);
		animation: tooltipIn 120ms var(--ease-out-expo) both;
		pointer-events: none;
	}

	.tooltip-top {
		bottom: calc(100% + 6px);
		left: 50%;
		transform: translateX(-50%);
	}

	.tooltip-bottom {
		top: calc(100% + 6px);
		left: 50%;
		transform: translateX(-50%);
	}

	.tooltip-left {
		right: calc(100% + 6px);
		top: 50%;
		transform: translateY(-50%);
	}

	.tooltip-right {
		left: calc(100% + 6px);
		top: 50%;
		transform: translateY(-50%);
	}

	@keyframes tooltipIn {
		from { opacity: 0; }
		to { opacity: 1; }
	}
</style>
