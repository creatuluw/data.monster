<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { FolderOpen, Plus, Check, LoaderCircle } from 'lucide-svelte';
	import { listWorkspaces, isTauriAvailable, extractErrorMessage, type WorkspaceEntry } from '$lib/db-operations';
	import { app } from '$lib/stores/app.svelte';

	let workspaces = $state<WorkspaceEntry[]>([]);
	let loading = $state(true);
	let switchingPath = $state<string | null>(null);
	let error = $state('');

	const folderName = (p: string): string => p.replaceAll('\\', '/').split('/').filter(Boolean).pop() ?? p;

	onMount(async () => {
		if (!isTauriAvailable()) {
			loading = false;
			return;
		}
		try {
			workspaces = await listWorkspaces();
		} catch (e) {
			error = extractErrorMessage(e, 'Failed to load workspaces');
		}
		loading = false;
	});

	async function switchTo(path: string) {
		if (path === app.workspacePath || switchingPath) return;
		switchingPath = path;
		error = '';
		const ok = await app.selectWorkspaceByPath(path);
		switchingPath = null;
		if (ok) {
			goto('/');
		} else {
			error = app.globalError;
		}
	}

	async function addWorkspace() {
		if (switchingPath) return;
		switchingPath = '__add__';
		error = '';
		const ok = await app.selectWorkspace();
		switchingPath = null;
		if (ok) {
			goto('/');
		} else {
			error = app.globalError;
			try {
				workspaces = await listWorkspaces();
			} catch { /* keep old list */ }
		}
	}
</script>

<svelte:head>
	<title>Workspaces — Data Monster</title>
</svelte:head>

<div class="workspaces-page">
	<div class="section-header">
		<h1 class="section-title page-title">Workspaces</h1>
		<button class="btn btn-primary btn-lg" onclick={addWorkspace} disabled={switchingPath !== null}>
			{#if switchingPath === '__add__'}
				<LoaderCircle size={16} class="spin" />
			{:else}
				<Plus size={16} />
			{/if}
			Add
		</button>
	</div>

	<p class="section-subtitle">
		Every folder you have opened as a workspace, most recently used first. Each workspace carries its
		own data, definitions and settings.
	</p>

	{#if error}
		<div class="ws-error">{error}</div>
	{/if}

	{#if loading}
		<div class="ws-empty">Loading…</div>
	{:else if workspaces.length === 0}
		<div class="ws-empty">No workspaces yet — add a folder to get started.</div>
	{:else}
		<div class="ws-grid">
			{#each workspaces as ws (ws.path)}
				<button
					class="ws-card"
					class:current={ws.path === app.workspacePath}
					onclick={() => switchTo(ws.path)}
					disabled={switchingPath !== null}
					title={ws.path === app.workspacePath ? 'Current workspace' : 'Open this workspace'}
				>
					<div class="lab-icon">
						<FolderOpen size={18} />
					</div>
					<div class="ws-info">
						<span class="ws-name">{folderName(ws.path)}</span>
						<span class="ws-path">{ws.path}</span>
					</div>
					{#if switchingPath === ws.path}
						<LoaderCircle size={14} class="spin ws-state" />
					{:else if ws.path === app.workspacePath}
						<Check size={14} class="ws-state ws-current-check" />
					{/if}
				</button>
			{/each}
		</div>
	{/if}
</div>

<style>
	.workspaces-page {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-6);
	}

	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-4);
	}

	.section-subtitle {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: var(--space-3) 0 0 0;
		max-width: 64ch;
		line-height: var(--leading-relaxed);
	}

	.ws-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: var(--space-3);
		margin-top: var(--space-6);
	}

	.ws-card {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-4);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		text-align: left;
		color: var(--color-text);
		background: var(--color-surface);
		cursor: pointer;
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.ws-card:hover:not(:disabled) {
		border-color: var(--color-border-strong);
		box-shadow: var(--shadow-md);
	}

	.ws-card:disabled {
		opacity: 0.6;
	}

	.ws-card.current {
		border-color: var(--color-accent);
	}

	.lab-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		border-radius: var(--radius-sm);
		background: var(--color-accent-muted);
		color: var(--color-accent);
		flex-shrink: 0;
	}

	.ws-info {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
		flex: 1;
		min-width: 0;
	}

	.ws-name {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 700;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.ws-path {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.ws-state {
		flex-shrink: 0;
		color: var(--color-text-tertiary);
	}

	.ws-current-check {
		color: var(--color-accent);
	}

	.ws-error {
		margin-top: var(--space-4);
		padding: var(--space-3) var(--space-4);
		border: 1px solid var(--color-danger-muted, var(--color-border));
		border-radius: var(--radius-sm);
		color: var(--color-danger, var(--color-text));
		font-size: var(--text-sm);
	}

	.ws-empty {
		margin-top: var(--space-6);
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
	}

	.spin {
		animation: ws-spin 1s linear infinite;
	}

	@keyframes ws-spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
