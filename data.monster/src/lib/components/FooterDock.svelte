<script lang="ts">
	import { page } from "$app/state";
	import { Bolt, BookOpen } from "lucide-svelte";
	import Tooltip from "./Tooltip.svelte";

	interface NavItem {
		href: string;
		label: string;
	}

	let {
		adminItems = [],
		docsItems = [],
	}: {
		adminItems?: NavItem[];
		docsItems?: NavItem[];
	} = $props();

	let adminOpen = $state(false);
	let docsOpen = $state(false);

	function isActive(href: string): boolean {
		return page.url.pathname === href || page.url.pathname.startsWith(href + "/");
	}

	function toggleAdmin(e: MouseEvent) {
		e.stopPropagation();
		adminOpen = !adminOpen;
		docsOpen = false;
	}

	function closeAdmin() {
		adminOpen = false;
	}

	function toggleDocs(e: MouseEvent) {
		e.stopPropagation();
		docsOpen = !docsOpen;
		adminOpen = false;
	}

	function closeDocs() {
		docsOpen = false;
	}

	function closeAll() {
		adminOpen = false;
		docsOpen = false;
	}
</script>

<svelte:window onclick={closeAll} />

{#if adminItems.length > 0 || docsItems.length > 0}
	<div class="footer-dock">
		{#if adminItems.length > 0}
			<div class="dock-item">
				<Tooltip text="Beheer" position="left">
					<button
						class="dock-trigger"
						class:is-active={adminItems.some((i) => isActive(i.href))}
						onclick={toggleAdmin}
						aria-label="Beheer"
						aria-expanded={adminOpen}
					>
						<Bolt size={16} />
					</button>
				</Tooltip>
				{#if adminOpen}
					<div class="dock-dropdown">
						{#each adminItems as item}
							<a
								href={item.href}
								class="dock-dropdown-item"
								class:is-active={isActive(item.href)}
								onclick={closeAdmin}
								data-sveltekit-preload-data="hover"
							>
								{item.label}
							</a>
						{/each}
					</div>
				{/if}
			</div>
		{/if}

		{#if docsItems.length > 0}
			<div class="dock-item">
				<Tooltip text="Docs" position="left">
					<button
						class="dock-trigger"
						class:is-active={docsItems.some((i) => isActive(i.href))}
						onclick={toggleDocs}
						aria-label="Docs"
						aria-expanded={docsOpen}
					>
						<BookOpen size={16} />
					</button>
				</Tooltip>
				{#if docsOpen}
					<div class="dock-dropdown">
						{#each docsItems as item}
							<a
								href={item.href}
								class="dock-dropdown-item"
								class:is-active={isActive(item.href)}
								onclick={closeDocs}
								data-sveltekit-preload-data="hover"
							>
								{item.label}
							</a>
						{/each}
					</div>
				{/if}
			</div>
		{/if}
	</div>
{/if}

<style>
	.footer-dock {
		position: fixed;
		bottom: var(--gutter);
		right: var(--gutter);
		z-index: 100;
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.dock-item {
		position: relative;
	}

	.dock-trigger {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		cursor: pointer;
		color: var(--color-text-tertiary);
		box-shadow: var(--shadow-sm);
		transition:
			color var(--duration-fast) ease,
			background var(--duration-fast) ease,
			border-color var(--duration-fast) ease,
			box-shadow var(--duration-fast) ease;
	}

	.dock-trigger:hover {
		color: var(--color-text);
		background: var(--color-surface-sunken);
		border-color: var(--color-border);
		box-shadow: var(--shadow-md);
	}

	.dock-trigger.is-active {
		color: var(--color-accent);
		background: var(--color-accent-muted);
		border-color: oklch(0.82 0.03 41);
	}

	.dock-trigger:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	.dock-dropdown {
		position: absolute;
		bottom: 0;
		right: calc(100% + 4px);
		min-width: 160px;
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		box-shadow: var(--shadow-md);
		z-index: 200;
		animation: slideLeft var(--duration-fast) var(--ease-out-expo) both;
	}

	@keyframes slideLeft {
		from {
			opacity: 0;
			transform: translateX(4px);
		}
		to {
			opacity: 1;
			transform: translateX(0);
		}
	}

	.dock-dropdown-item {
		display: block;
		padding: var(--space-2) var(--space-3);
		font-family: var(--font-body);
		font-size: var(--text-sm);
		font-weight: 500;
		color: var(--color-text-secondary);
		text-decoration: none;
		transition:
			color var(--duration-fast) ease,
			background var(--duration-fast) ease;
	}

	.dock-dropdown-item:first-child {
		border-radius: var(--radius-sm) var(--radius-sm) 0 0;
	}

	.dock-dropdown-item:last-child {
		border-radius: 0 0 var(--radius-sm) var(--radius-sm);
	}

	.dock-dropdown-item:hover {
		color: var(--color-text);
		background: var(--color-surface-sunken);
	}

	.dock-dropdown-item.is-active {
		color: var(--color-accent);
		background: var(--color-accent-muted);
	}
</style>
