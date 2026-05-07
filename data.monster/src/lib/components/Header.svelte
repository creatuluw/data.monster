	<script lang="ts">
	import { page } from "$app/state";
	import { LogOut, MoveRight } from "lucide-svelte";
	import Button from "./Button.svelte";
	import Tooltip from "./Tooltip.svelte";
	import Drawer from "./Drawer.svelte";

	interface NavItem {
		href: string;
		label: string;
	}

	let {
		navItems = [],
		drawerItems = [],
		userEmail = "",
		onLogout,
	}: {
		navItems?: NavItem[];
		drawerItems?: NavItem[];
		userEmail?: string;
		onLogout?: () => void;
	} = $props();

	let drawerOpen = $state(false);

	function isActive(href: string): boolean {
		return page.url.pathname === href || page.url.pathname.startsWith(href + "/");
	}

	const wallImages = ["/wall1.png", "/wall2.png", "/wall3.png", "/wall4.png", "/wall5.png", "/wall6.png", "/wall7.png", "/wall8.png", "/wall9.png"];
	const mascotImage = $derived(wallImages[Math.floor(Math.random() * wallImages.length)]);
	</script>

<nav class="nav">
	<div class="nav-inner">
		<div class="brand-wrapper">
			<a href="/dashboard" class="nav-brand" data-sveltekit-preload-data="hover">
				<span class="brand-mark"></span>
				LRS
			</a>
			{#if page.url.pathname === '/dashboard'}
				<img src={mascotImage} alt="Pippe en Loi gluren" class="brand-mascot" />
			{/if}
		</div>

		<ul class="nav-links">
			{#each navItems as item}
				<li>
					<a href={item.href} class:active={isActive(item.href)} data-sveltekit-preload-data="hover">{item.label}</a>
				</li>
			{/each}
			{#if drawerItems.length > 0 && !drawerOpen}
				<li>
					<a href="#meer" class="nav-more" onclick={(e) => { e.preventDefault(); drawerOpen = true; }}>Meer <MoveRight size={14} style="margin-left: 2px; position: relative; top: 3px;" /></a>
				</li>
			{/if}
		</ul>

		<div class="nav-user">
			<a href="/account" class="nav-user-email">{userEmail}</a>
			<Tooltip text="Uitloggen" position="bottom">
				<Button variant="icon" onclick={onLogout}><LogOut size={16} /></Button>
			</Tooltip>
		</div>
	</div>
</nav>

{#if drawerItems.length > 0}
	<Drawer bind:open={drawerOpen} mode="push" overlay style="--drawer-width: 20vw">
		{#snippet title()}
			Menu
		{/snippet}
		{#snippet body()}
			<ul class="drawer-nav">
				{#each drawerItems as item}
					<li>
						<a href={item.href} class:active={isActive(item.href)} data-sveltekit-preload-data="hover">{item.label}</a>
					</li>
				{/each}
			</ul>
		{/snippet}
	</Drawer>
{/if}

<style>
	.nav {
		position: sticky;
		top: 0;
		z-index: 100;
		background: #ffffff;
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
	}

	.nav-inner {
		padding: var(--space-3) var(--gutter);
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-4);
	}

	.nav-brand {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 600;
		letter-spacing: -0.02em;
		color: var(--color-text);
		text-decoration: none;
		display: flex;
		align-items: center;
		gap: var(--space-2);
		flex-shrink: 0;
		position: relative;
	}

	.brand-mark {
		width: 20px;
		height: 20px;
		border: 2px solid var(--color-accent);
		border-radius: var(--radius-xs);
		position: relative;
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	.brand-mark::after {
		content: "";
		width: 6px;
		height: 6px;
		background: var(--color-accent);
		border-radius: 1px;
	}

	.brand-wrapper {
		position: relative;
		display: flex;
		align-items: center;
	}

	.brand-mascot {
		width: 150px;
		height: auto;
		object-fit: contain;
		border-radius: var(--radius-xs);
		position: absolute;
		left: calc(100% + var(--space-6) + var(--space-4));
		top: -7px;
		opacity: 0.8;
	}

	.nav-links {
		display: flex;
		gap: var(--space-1);
		list-style: none;
	}

	.nav-links a {
		font-family: var(--font-body);
		font-size: var(--text-sm);
		font-weight: 500;
		color: var(--color-text-secondary);
		text-decoration: none;
		padding: var(--space-2) var(--space-3);
		border-radius: var(--radius-xs);
		transition:
			color var(--duration-fast) ease,
			background var(--duration-fast) ease;
	}

	.nav-links a:hover {
		color: var(--color-accent);
		background: var(--color-accent-muted);
	}

	.nav-links a:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	.nav-links a.active {
		color: var(--color-accent);
		background: var(--color-accent-muted);
	}

	.drawer-nav {
		list-style: none;
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.drawer-nav a {
		font-family: var(--font-body);
		font-size: var(--text-sm);
		font-weight: 500;
		color: var(--color-text-secondary);
		text-decoration: none;
		padding: var(--space-2) var(--space-3);
		border-radius: var(--radius-xs);
		display: block;
		transition:
			color var(--duration-fast) ease,
			background var(--duration-fast) ease;
	}

	.drawer-nav a:hover {
		color: var(--color-accent);
		background: var(--color-accent-muted);
	}

	.drawer-nav a.active {
		color: var(--color-accent);
		background: var(--color-accent-muted);
	}

	.nav-user {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		flex-shrink: 0;
	}

	.nav-user-email {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		text-decoration: none;
		border-radius: var(--radius-xs);
		padding: var(--space-1) var(--space-2);
		transition:
			color var(--duration-fast) ease,
			background var(--duration-fast) ease;
	}

	.nav-user-email:hover {
		color: var(--color-accent);
		background: var(--color-accent-muted);
	}

	.nav-user-email:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	@media (max-width: 768px) {
		.nav-links {
			display: none;
		}

		.nav-user-email {
			display: none;
		}

		.nav-inner {
			padding: var(--space-2) var(--space-4);
		}

		.brand-mascot {
			display: none;
		}
	}
</style>
