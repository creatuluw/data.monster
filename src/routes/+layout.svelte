<script lang="ts">
	import '../app.css';
	import { app } from '$lib/stores/app.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { listen, type UnlistenFn } from '@tauri-apps/api/event';
	import { FolderOpen, Settings, MoreVertical, Link, Check } from 'lucide-svelte';
	import Breadcrumb from '$lib/components/Breadcrumb.svelte';

	let { children } = $props();
	let showWorkspacePicker = $state(false);
	let menuOpen = $state(false);
	let copied = $state(false);
	let initStatus = $state('Initializing database...');

	let isDev = $derived(
		typeof window !== 'undefined' && (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1')
	);

	function toggleMenu() {
		menuOpen = !menuOpen;
	}

	function handleCopyUrl() {
		navigator.clipboard.writeText($page.url.href).then(() => {
			copied = true;
			setTimeout(() => { copied = false; }, 2000);
		});
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
		let destroyed = false;
		let unlistenFn: (() => void) | null = null;

		listen<string>('db-init-progress', (event) => {
			initStatus = event.payload;
		}).then((unlisten) => {
			if (destroyed) { unlisten(); return; }
			unlistenFn = unlisten;
		});

		if (!app.dbReady) {
			app.init().then(() => {
				if (!app.workspacePath) {
					showWorkspacePicker = true;
				}
			});
		}

		const handleBeforeUnload = () => { app.shutdown(); };
		window.addEventListener('beforeunload', handleBeforeUnload);
		window.addEventListener('click', handleClickOutside);
		return () => {
			destroyed = true;
			window.removeEventListener('beforeunload', handleBeforeUnload);
			window.removeEventListener('click', handleClickOutside);
			if (unlistenFn) unlistenFn();
		};
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

	const routeLabels: Record<string, string> = {
		data: 'Data',
		connect: 'Connect',
		query: 'Query',
		table: 'Data',
		settings: 'Settings',
		'internal-db': 'Internal DB',
		labs: 'Labs',
		'chart-lib': 'Chart Lib',
		charts: 'Charts',
		analyst: 'Analyst',
		chat: 'Chat',
		pages: 'Pages',
		chart: 'Chart',
		preview: 'Preview',
	};

	let breadcrumbs = $derived.by(() => {
		const path = $page.url.pathname;
		if (path === '/' || isUiPage) return [];
		const segments = path.split('/').filter(Boolean);
		const hrefOverrides: Record<string, string> = { table: '/data' };
		return segments.map((seg, i) => ({
			href: hrefOverrides[seg] ?? ('/' + segments.slice(0, i + 1).join('/')),
			label: decodeURIComponent(routeLabels[seg] || seg),
		}));
	});
</script>

{#if isUiPage}
	{@render children()}
{:else}
<div class="app-shell">
	<header class="app-header">
		<div class="app-brand-group">
			<a href="/" class="app-brand">
				<img src="/monster-on-white.svg" alt="Data Monster logo" class="brand-logo" />
				Data Monster
			</a>
			{#if app.workspacePath}
				<span class="brand-divider" aria-hidden="true">&middot;</span>
				<span class="workspace-label" title={app.workspacePath}>{app.workspacePath}</span>
			{/if}
		</div>
		<div class="header-meta">
			{#if app.dbReady}
				<a href="/data" class="header-link">
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

	{#if isDev}
		<button class="dev-copy-url" onclick={handleCopyUrl} title="Copy current URL for LLM">
			{#if copied}
				<Check size={14} />
			{:else}
				<Link size={14} />
			{/if}
		</button>
	{/if}

	{#if breadcrumbs.length > 0}
		<div class="breadcrumb-bar">
			<Breadcrumb items={breadcrumbs} />
		</div>
	{/if}

	{#if showWorkspacePicker && !app.dbReady}
		<div class="workspace-picker">
			<div class="workspace-card">
				<img src="/monster-on-white.svg" alt="Data Monster logo" class="brand-logo brand-logo-lg" />
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
			<span style="font-size: var(--text-sm); color: var(--color-text-tertiary);">{initStatus}</span>
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
		background: oklch(0.98 0.003 160 / 0.97);
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

	.brand-logo {
		width: 30px;
		height: 32px;
		display: block;
		margin-right: var(--space-3);
	}

	.brand-logo-lg {
		width: 72px;
		height: 76px;
		margin: 0 auto var(--space-4);
	}

	.app-brand-group {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.brand-divider {
		color: var(--color-text-tertiary);
		font-size: var(--text-md);
	}

	.workspace-label {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		background: var(--color-surface-sunken);
		padding: 2px var(--space-2);
		border-radius: var(--radius-xs);
		max-width: 420px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
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

	.breadcrumb-bar {
		display: flex;
		align-items: center;
		padding: var(--space-1) var(--space-6);
		border-bottom: 1px solid var(--color-border);
		background: var(--color-surface-sunken);
		flex-shrink: 0;
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
		overflow-y: auto;
		padding: 0;
	}

	/* app-wide content cap: every page's root container maxes out and centers
	   at 1440px (token --max-width) */
	.app-main > :global(*) {
		width: 100%;
		max-width: var(--max-width);
		margin-left: auto;
		margin-right: auto;
	}

	/* full-bleed pages (analyst, data, page editor) span the whole window */
	.app-main > :global(.full-bleed) {
		max-width: none;
	}

	.app-error {
		padding: var(--space-3) var(--space-4);
		margin-bottom: var(--space-4);
		background: oklch(0.95 0.03 25);
		border: 1px solid oklch(0.9 0.04 25);
		border-radius: var(--radius-xs);
		font-size: var(--text-sm);
		color: oklch(0.38 0.12 25);
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
		background: oklch(0.95 0.03 25);
		border: 1px solid oklch(0.9 0.04 25);
		border-radius: var(--radius-xs);
		font-size: var(--text-xs);
		color: oklch(0.38 0.12 25);
	}

	.dev-copy-url {
		position: fixed;
		right: var(--space-3);
		bottom: var(--space-3);
		z-index: 90;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-full, 999px);
		background: var(--color-surface);
		color: var(--color-text-tertiary);
		cursor: pointer;
		box-shadow: var(--shadow-sm);
	}

	.dev-copy-url:hover {
		color: var(--color-accent);
		border-color: var(--color-accent);
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
