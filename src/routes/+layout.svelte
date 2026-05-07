<script lang="ts">
	import '../app.css';
	import { app } from '$lib/stores/app.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { FolderOpen, Settings, MoreVertical, Link, Check } from 'lucide-svelte';

	let { children } = $props();
	let showWorkspacePicker = $state(false);
	let menuOpen = $state(false);
	let copied = $state(false);

	let isDev = $derived(
		typeof window !== 'undefined' && (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1')
	);

	function toggleMenu() {
		menuOpen = !menuOpen;
	}

	function handleCopyRoute() {
		const url = $page.url.href;
		navigator.clipboard.writeText(url).then(() => {
			copied = true;
			setTimeout(() => { copied = false; }, 2000);
		});
		menuOpen = false;
	}

	function handleClickOutside(e: MouseEvent) {
		if (menuOpen && !(e.target as HTMLElement).closest('.dev-menu')) {
			menuOpen = false;
		}
	}

	onMount(() => {
		if (!app.dbReady) {
			app.init().then(() => {
				if (!app.workspacePath) {
					showWorkspacePicker = true;
				}
			});
		}
		window.addEventListener('click', handleClickOutside);
		return () => window.removeEventListener('click', handleClickOutside);
	});

	async function handleSelectWorkspace() {
		const success = await app.selectWorkspace();
		if (success) {
			showWorkspacePicker = false;
		}
	}

	function handleSelectTable(table: string) {
		goto(`/table/${encodeURIComponent(table)}`);
	}

	function handleConnect() {
		goto('/connect');
	}

	let selectedTable = $derived.by(() => {
		if (!$page.url.pathname.startsWith('/table/')) return '';
		return decodeURIComponent($page.url.pathname.slice('/table/'.length));
	});

	let isUiPage = $derived($page.url.pathname.startsWith('/ui'));
</script>

{#if isUiPage}
	{@render children()}
{:else}
<div class="app-shell">
	<header class="app-header">
		<a href="/" class="app-brand">
			<span class="brand-mark"></span>
			Data Monster
		</a>
		<div class="header-meta">
			{#if app.dbReady}
				<a href="/tables" class="header-link">
					{app.tables.length} table{app.tables.length !== 1 ? 's' : ''}
				</a>
				<span class="tag tag-success">Persistent</span>
				<div class="header-actions">
					<a href="/settings" class="btn btn-ghost btn-sm" title="Settings">
						<Settings size={12} />
					</a>
					<button class="btn btn-ghost btn-sm" onclick={handleSelectWorkspace} title="Switch workspace">
						<FolderOpen size={12} />
					</button>
					{#if isDev}
						<div class="dev-menu">
							<button class="btn btn-ghost btn-sm" onclick={toggleMenu} title="Dev menu">
								<MoreVertical size={12} />
							</button>
							{#if menuOpen}
								<div class="dev-menu-dropdown">
									<button class="dev-menu-item" onclick={handleCopyRoute}>
										{#if copied}
											<Check size={12} />
											<span>Copied!</span>
										{:else}
											<Link size={12} />
											<span>Copy URL for agent</span>
										{/if}
									</button>
								</div>
							{/if}
						</div>
					{/if}
				</div>
			{/if}
		</div>
	</header>

	{#if showWorkspacePicker && !app.dbReady}
		<div class="workspace-picker">
			<div class="workspace-card">
				<span class="brand-mark-lg"></span>
				<h2 class="workspace-title">Welcome to Data Monster</h2>
				<p class="workspace-desc">Choose a folder to store your data. Everything is saved to disk.</p>
				<button class="btn btn-primary" onclick={handleSelectWorkspace}>
					<FolderOpen size={16} />
					Choose Workspace Folder
				</button>
				{#if app.globalError}
					<div class="workspace-error">{app.globalError}</div>
				{/if}
			</div>
		</div>
	{:else if !app.dbReady}
		<div class="app-loading">
			<svg class="spinner" viewBox="0 0 24 24" fill="none">
				<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
				<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
			</svg>
			<span style="font-size: var(--text-sm); color: var(--color-text-tertiary);">Initializing database...</span>
		</div>
	{:else}
		<div class="app-body">
			<main class="app-main">
				{#if app.globalError}
					<div class="app-error">{app.globalError}</div>
				{/if}

				{@render children()}
			</main>
		</div>
	{/if}
</div>
{/if}

<style>
	.app-shell {
		display: flex;
		flex-direction: column;
		height: 100vh;
	}

	.app-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-6);
		border-bottom: 1px solid var(--color-border);
		background: oklch(0.98 0.003 250 / 0.97);
		-webkit-backdrop-filter: blur(12px);
		backdrop-filter: blur(12px);
		flex-shrink: 0;
	}

	.app-brand {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 600;
		letter-spacing: -0.02em;
		color: var(--color-text);
		text-decoration: none;
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

	.header-meta {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.header-link {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		text-decoration: none;
		transition: color var(--duration-fast) ease;
	}

	.header-link:hover {
		color: var(--color-accent);
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: 2px;
	}

	.app-loading {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-3);
	}

	.app-body {
		flex: 1;
		overflow: hidden;
		display: flex;
	}

	.app-main {
		flex: 1;
		overflow: hidden;
		padding: 0;
	}

	.app-error {
		padding: var(--space-3) var(--space-4);
		margin-bottom: var(--space-4);
		background: oklch(0.95 0.03 22);
		border: 1px solid oklch(0.9 0.04 22);
		border-radius: var(--radius-xs);
		font-size: var(--text-sm);
		color: oklch(0.38 0.12 22);
	}

	.workspace-picker {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.workspace-card {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-4);
		padding: var(--space-12) var(--space-8);
	}

	.brand-mark-lg {
		width: 64px;
		height: 64px;
		border: 2px solid var(--color-accent);
		border-radius: var(--radius-md);
		position: relative;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		background: var(--color-accent-muted);
	}

	.brand-mark-lg::after {
		content: "";
		width: 24px;
		height: 24px;
		background: var(--color-accent);
		border-radius: var(--radius-xs);
	}

	.workspace-title {
		font-family: var(--font-display);
		font-size: var(--text-xl);
		font-weight: 700;
		letter-spacing: -0.02em;
		color: var(--color-text);
	}

	.workspace-desc {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		text-align: center;
		max-width: 30ch;
	}

	.workspace-error {
		padding: var(--space-2) var(--space-3);
		background: oklch(0.95 0.03 22);
		border: 1px solid oklch(0.9 0.04 22);
		border-radius: var(--radius-xs);
		font-size: var(--text-xs);
		color: oklch(0.38 0.12 22);
	}

	.dev-menu {
		position: relative;
	}

	.dev-menu-dropdown {
		position: absolute;
		top: calc(100% + 4px);
		right: 0;
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		box-shadow: var(--shadow-md);
		min-width: 180px;
		z-index: 100;
		padding: var(--space-1);
	}

	.dev-menu-item {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		width: 100%;
		padding: var(--space-2) var(--space-3);
		font-family: var(--font-body);
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		background: transparent;
		border: none;
		border-radius: var(--radius-xs);
		cursor: pointer;
		transition: background var(--duration-fast) ease, color var(--duration-fast) ease;
		white-space: nowrap;
	}

	.dev-menu-item:hover {
		background: var(--color-surface-sunken);
		color: var(--color-text);
	}
</style>
