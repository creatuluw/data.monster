<script lang="ts">
	import type { Snippet } from "svelte";
	import Badge from "./Badge.svelte";
	import { Menu, X } from "lucide-svelte";

	interface TabItem {
		key: string;
		label: string;
		content?: Snippet;
		href?: string;
		badge?: number;
		badgeVariant?: "default" | "accent" | "danger";
	}

	let {
		items = [],
		activeKey = $bindable(""),
		variant = "default",
		onchange,
	}: {
		items: TabItem[];
		activeKey?: string;
		variant?: "default" | "underline" | "pill";
		onchange?: (key: string) => void;
	} = $props();

	if (items.length > 0 && !activeKey) {
		activeKey = items[0].key;
	}

	let mobileMenuOpen = $state(false);

	function handleClick(key: string) {
		activeKey = key;
		mobileMenuOpen = false;
		if (onchange) onchange(key);
	}
</script>

<div class="tabs tabs--{variant}">
	<div class="tab-bar" role="tablist">
		<button class="tab-hamburger" onclick={() => mobileMenuOpen = !mobileMenuOpen} aria-label="Menu">
			{#if mobileMenuOpen}
				<X size={18} />
			{:else}
				<Menu size={18} />
			{/if}
		</button>
		{#each items as item}
			<button
				class="tab-btn"
				class:tab-btn--add={item.key === "nieuw" || item.key === "add"}
				class:active={activeKey === item.key}
				role="tab"
				aria-selected={activeKey === item.key}
				onclick={() => handleClick(item.key)}
			>
				{item.label}
				{#if item.badge}
					<span class="tab-badge">
						<Badge count={item.badge} variant={item.badgeVariant ?? "default"} />
					</span>
				{/if}
			</button>
		{/each}
	</div>

	{#if mobileMenuOpen}
		<div class="tab-mobile-menu">
			{#each items as item}
				<button
					class="tab-mobile-item"
					class:active={activeKey === item.key}
					onclick={() => handleClick(item.key)}
				>
					{item.label}
					{#if item.badge}
						<Badge count={item.badge} variant={item.badgeVariant ?? "default"} />
					{/if}
				</button>
			{/each}
		</div>
	{/if}

	{#if activeKey}
		{#each items as item}
			{#if activeKey === item.key && item.content}
				<div class="tab-panel" role="tabpanel">
					{@render item.content()}
				</div>
			{/if}
		{/each}
	{/if}
</div>

<style>
	.tabs {
		display: flex;
		flex-direction: column;
	}

	.tab-bar {
		display: flex;
		align-items: center;
	}

	.tab-hamburger {
		display: none;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		padding: 0;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		background: var(--color-surface);
		color: var(--color-text-secondary);
		cursor: pointer;
	}

	.tab-hamburger:hover {
		color: var(--color-text);
		border-color: var(--color-accent);
	}

	.tab-btn {
		font-family: var(--font-body);
		font-size: var(--text-sm);
		font-weight: 500;
		cursor: pointer;
		transition: all var(--duration-fast) ease;
		background: none;
		border: none;
		color: var(--color-text-secondary);
		white-space: nowrap;
	}

	.tab-btn:hover {
		color: var(--color-text);
	}

	.tab-btn.active {
		color: var(--color-text);
	}

	.tab-panel {
		margin-top: var(--space-4);
	}

	/* Default variant: contained tabs with a background track */
	.tabs--default .tab-bar {
		gap: var(--space-1);
		padding: var(--space-1);
		background: var(--color-surface-raised);
		border-radius: var(--radius-lg);
	}

	.tabs--default .tab-btn {
		padding: var(--space-2) var(--space-4);
		border-radius: var(--radius-sm);
	}

	.tabs--default .tab-btn:hover {
		background: var(--color-surface-sunken);
	}

	.tabs--default .tab-btn.active {
		background: var(--color-surface);
		box-shadow: var(--shadow-sm);
	}

	.tab-btn--add {
		font-size: var(--text-base);
		font-weight: 500;
		line-height: 1;
	}

	.tab-badge {
		position: relative;
		top: -1px;
		left: 4px;
	}

	.tabs--default .tab-panel {
		padding: var(--space-4);
		background: var(--color-surface-sunken);
		border-radius: var(--radius-lg);
		border: 1px solid var(--color-border);
	}

	/* Underline variant: minimal tabs with a bottom border */
	.tabs--underline .tab-bar {
		gap: var(--space-6);
		border-bottom: 1px solid var(--color-border);
	}

	.tabs--underline .tab-btn {
		padding: var(--space-2) 0;
		border-bottom: 2px solid transparent;
		margin-bottom: -1px;
		border-radius: 0;
	}

	.tabs--underline .tab-btn.active {
		border-bottom-color: var(--color-accent);
		color: var(--color-text);
	}

	/* Pill variant: rounded floating tabs */
	.tabs--pill .tab-bar {
		gap: var(--space-2);
	}

	.tabs--pill .tab-btn {
		padding: var(--space-2) var(--space-4);
		border-radius: var(--radius-full);
		background: var(--color-surface-raised);
		border: none;
		font-size: var(--text-xs);
		min-height: 32px;
	}

	.tabs--pill .tab-btn:hover {
		background: var(--color-surface-sunken);
	}

	.tabs--pill .tab-btn.active {
		background: var(--color-accent);
		color: var(--color-text-on-accent);
	}

	/* Mobile hamburger menu */
	.tab-mobile-menu {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
		padding: var(--space-2);
		background: var(--color-surface-raised);
		border-radius: var(--radius-md);
		border: 1px solid var(--color-border);
		margin-top: var(--space-2);
	}

	.tab-mobile-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		border: none;
		border-radius: var(--radius-sm);
		background: transparent;
		color: var(--color-text-secondary);
		font-family: var(--font-body);
		font-size: var(--text-sm);
		font-weight: 500;
		cursor: pointer;
		text-align: left;
	}

	.tab-mobile-item:hover {
		background: var(--color-surface-sunken);
		color: var(--color-text);
	}

	.tab-mobile-item.active {
		background: var(--color-surface);
		color: var(--color-text);
		box-shadow: var(--shadow-sm);
	}

	@media (max-width: 768px) {
		.tab-bar {
			flex-wrap: wrap;
			gap: var(--space-1);
		}
	}

	@media (max-width: 640px) {
		.tab-hamburger {
			display: flex;
		}

		.tab-btn {
			display: none;
		}

		.tab-bar {
			flex-wrap: nowrap;
		}

		.tab-mobile-menu {
			display: flex;
		}
	}
</style>
