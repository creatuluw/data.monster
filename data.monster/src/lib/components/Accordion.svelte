<script lang="ts">
	import { ChevronDown } from "lucide-svelte";

	let {
		items,
	}: {
		items: {
			ref?: string;
			title: string;
			content: import("svelte").Snippet;
		}[];
	} = $props();

	let expandedIndex = $state<number | null>(null);

	function toggle(index: number) {
		expandedIndex = expandedIndex === index ? null : index;
	}
</script>

<div class="accordion-group">
	{#each items as item, i}
		<div class="accordion">
			<button
				class="accordion-trigger"
				aria-expanded={expandedIndex === i}
				aria-controls="acc-panel-{i}"
				id="acc-trigger-{i}"
				onclick={() => toggle(i)}
			>
				<span class="accordion-trigger-left">
					{#if item.ref}
						<span class="accordion-ref">{item.ref}</span>
					{/if}
					{item.title}
				</span>
				<span class="accordion-icon" aria-hidden="true"><ChevronDown size={16} /></span>
			</button>
			<div
				class="accordion-panel"
				id="acc-panel-{i}"
				role="region"
				aria-labelledby="acc-trigger-{i}"
				aria-hidden={expandedIndex !== i}
			>
				<div class="accordion-content">
					{@render item.content()}
				</div>
			</div>
		</div>
	{/each}
</div>

<style>
	.accordion-group {
		display: flex;
		flex-direction: column;
	}

	.accordion {
		border: 1px solid var(--color-border);
		background: var(--color-surface);
	}

	.accordion + .accordion {
		border-top: none;
	}

	.accordion-trigger {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-4) var(--space-6);
		background: var(--color-surface);
		border: none;
		cursor: pointer;
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
		text-align: left;
		transition:
			background var(--duration-fast) ease,
			color var(--duration-fast) ease;
	}

	.accordion-trigger:hover {
		background: var(--color-surface-raised);
	}

	.accordion-trigger:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: -2px;
		z-index: 1;
	}

	.accordion-trigger-left {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.accordion-ref {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.08em;
		color: var(--color-accent);
		min-width: 48px;
	}

	.accordion-icon {
		width: 16px;
		height: 16px;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: transform var(--duration-base) var(--ease-out-expo);
		color: var(--color-text-tertiary);
		flex-shrink: 0;
	}

	.accordion-trigger[aria-expanded="true"] .accordion-icon {
		transform: rotate(180deg);
	}

	.accordion-trigger[aria-expanded="true"] {
		background: var(--color-surface-raised);
		border-bottom: 1px solid var(--color-border);
	}

	.accordion-panel {
		overflow: hidden;
		max-height: 0;
		transition: max-height var(--duration-slow) var(--ease-out-expo);
	}

	.accordion-panel[aria-hidden="false"] {
		max-height: 500px;
	}

	.accordion-content {
		padding: var(--space-6);
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		max-width: 60ch;
	}
</style>
