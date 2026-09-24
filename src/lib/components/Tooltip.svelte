<script lang="ts">
	import { Tooltip } from 'bits-ui';

	let {
		text,
		position = 'top',
		block = false,
		children
	}: {
		text?: string;
		position?: 'top' | 'bottom' | 'left' | 'right';
		block?: boolean;
		children?: import('svelte').Snippet;
	} = $props();
</script>

{#if !text}
	<!-- nothing to show: render the wrapped content bare, as before -->
	{#if children}{@render children()}{/if}
{:else}
	<Tooltip.Provider>
		<Tooltip.Root delayDuration={400}>
			<Tooltip.Trigger>
				{#snippet child({ props })}
					{#if block}
						<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
						<div {...props} class="tooltip-wrap tooltip-wrap-block" tabindex="0">
							{#if children}{@render children()}{/if}
						</div>
					{:else}
						<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
						<span {...props} class="tooltip-wrap" tabindex="0">
							{#if children}{@render children()}{/if}
						</span>
					{/if}
				{/snippet}
			</Tooltip.Trigger>
			<Tooltip.Portal>
				<Tooltip.Content
					class="tooltip"
					side={position}
					sideOffset={6}
				>
					{text}
				</Tooltip.Content>
			</Tooltip.Portal>
		</Tooltip.Root>
	</Tooltip.Provider>
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

	/* bubble renders in a portal — chrome hangs off a :global rule */
	:global(.tooltip) {
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
	}
</style>
