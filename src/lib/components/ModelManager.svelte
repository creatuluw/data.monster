<script lang="ts">
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import {
		type ModelEntry,
		type DownloadedModel,
		type ModelLoadStatus,
		type DownloadProgress,
		detectSystemRam,
		listAvailableModels,
		downloadModel,
		cancelDownload,
		listDownloadedModels,
		deleteModel,
		loadModel,
		unloadModel,
		isModelLoaded,
		extractErrorMessage
	} from '$lib/db-operations';
	import { Hash, HardDrive, Cpu, Download, Trash2, Play, Square, CheckCircle, AlertTriangle } from 'lucide-svelte';

	let { onselect }: { onselect?: (modelId: string) => void } = $props();

	let systemRam = $state(0);
	let catalog = $state<ModelEntry[]>([]);
	let downloaded = $state<DownloadedModel[]>([]);
	let loadStatus = $state<ModelLoadStatus>({ loaded: false, model_id: null });
	let downloadingModelId = $state<string | null>(null);
	let downloadProgress = $state<DownloadProgress | null>(null);
	let loading = $state(true);
	let error = $state('');

	function bytesToGB(bytes: number): string {
		return (bytes / 1_000_000_000).toFixed(1);
	}

	function exceedsRam(ramRequired: number): boolean {
		return systemRam > 0 && ramRequired > systemRam * 0.7;
	}

	function isDownloading(modelId: string): boolean {
		return downloadingModelId === modelId;
	}

	function isDownloaded(modelId: string): boolean {
		return downloaded.some((d) => d.model_id === modelId);
	}

	async function refresh() {
		try {
			const [ram, models, dled, status] = await Promise.all([
				detectSystemRam(),
				listAvailableModels(),
				listDownloadedModels(),
				isModelLoaded()
			]);
			systemRam = ram;
			catalog = models;
			downloaded = dled;
			loadStatus = status;
		} catch (e) {
			error = extractErrorMessage(e, 'Failed to load model data');
		}
		loading = false;
	}

	async function handleDownload(modelId: string) {
		error = '';
		downloadingModelId = modelId;
		try {
			await downloadModel(modelId);
			await refresh();
		} catch (e) {
			error = extractErrorMessage(e, 'Download failed');
		}
		downloadingModelId = null;
		downloadProgress = null;
	}

	async function handleCancel() {
		if (downloadingModelId) {
			await cancelDownload();
			downloadingModelId = null;
			downloadProgress = null;
		}
	}

	async function handleDelete(modelId: string) {
		error = '';
		try {
			await deleteModel(modelId);
			await refresh();
		} catch (e) {
			error = extractErrorMessage(e, 'Delete failed');
		}
	}

	async function handleLoad(modelId: string) {
		error = '';
		try {
			await loadModel(modelId);
			await refresh();
			onselect?.(modelId);
		} catch (e) {
			error = extractErrorMessage(e, 'Load failed');
		}
	}

	async function handleUnload() {
		error = '';
		try {
			await unloadModel();
			await refresh();
		} catch (e) {
			error = extractErrorMessage(e, 'Unload failed');
		}
	}

	onMount(async () => {
		const unlisten = await listen<DownloadProgress>('local-llm:download-progress', (event) => {
			downloadProgress = event.payload;
		});

		await refresh();

		return () => { unlisten(); };
	});

	let fastModels = $derived(catalog.filter((m) => m.tier === 'fast'));
	let balancedModels = $derived(catalog.filter((m) => m.tier === 'balanced'));
</script>

{#if loading}
	<div class="mm-loading">Loading model catalog...</div>
{:else}
	{#if error}
		<div class="mm-error">
			<AlertTriangle size={14} />
			<span>{error}</span>
			<button class="mm-error-dismiss" onclick={() => { error = ''; }}>Dismiss</button>
		</div>
	{/if}

	{#if loadStatus.loaded}
		<div class="mm-loaded">
			<div class="mm-loaded-indicator">
				<CheckCircle size={16} />
				<span>Model loaded: <strong>{loadStatus.model_id}</strong></span>
			</div>
			<button class="btn btn-secondary btn-sm" onclick={handleUnload}>
				<Square size={12} />
				Unload
			</button>
		</div>
	{/if}

	{#if systemRam > 0}
		<div class="mm-ram-info">
			<Cpu size={12} />
			<span>System RAM: {bytesToGB(systemRam)} GB | Free for model: ~{bytesToGB(systemRam * 0.7)} GB</span>
		</div>
	{/if}

	{#if downloadProgress}
		<div class="mm-progress">
			<div class="mm-progress-header">
				<span>Downloading {downloadProgress.model_id}</span>
				<button class="btn btn-secondary btn-sm" onclick={handleCancel}>Cancel</button>
			</div>
			<div class="mm-progress-bar">
				<div class="mm-progress-fill" style="width: {Math.min(downloadProgress.percent, 100)}%"></div>
			</div>
			<div class="mm-progress-stats">
				{bytesToGB(downloadProgress.bytes_downloaded)} / {bytesToGB(downloadProgress.bytes_total)} GB
				({downloadProgress.percent.toFixed(0)}%)
			</div>
		</div>
	{/if}

	<div class="mm-tier">
		<div class="mm-tier-header">
			<span class="mm-tier-badge mm-tier-fast">Fast</span>
			<span class="mm-tier-desc">For 8 GB RAM or more</span>
		</div>
		<div class="mm-grid">
			{#each fastModels as model}
				{@const downloaded = isDownloaded(model.id)}
				{@const downloading = isDownloading(model.id)}
				{@const isLoaded = loadStatus.loaded && loadStatus.model_id === model.id}
				{@const tooBig = exceedsRam(model.ram_required_bytes)}
				<div class="mm-card" class:is-loaded={isLoaded}>
					<div class="mm-card-header">
						<span class="mm-card-name">{model.name}</span>
						<span class="mm-card-params">{model.params}</span>
					</div>
					<div class="mm-card-desc">{model.description}</div>
					<div class="mm-card-stats">
						<span class="mm-stat"><HardDrive size={10} /> {bytesToGB(model.size_bytes)} GB</span>
						<span class="mm-stat"><Cpu size={10} /> {bytesToGB(model.ram_required_bytes)} GB RAM</span>
					</div>
					{#if tooBig}
						<div class="mm-card-warn">
							<AlertTriangle size={10} />
							May not fit in available RAM
						</div>
					{/if}
					<div class="mm-card-actions">
						{#if isLoaded}
							<span class="mm-badge mm-badge-active">
								<CheckCircle size={10} /> Active
							</span>
						{:else if downloading}
							<span class="mm-badge mm-badge-downloading">Downloading...</span>
						{:else if downloaded}
							<button class="btn btn-primary btn-sm" onclick={() => handleLoad(model.id)}>
								<Play size={10} /> Load
							</button>
							<button class="btn btn-ghost btn-sm" onclick={() => handleDelete(model.id)}>
								<Trash2 size={10} />
							</button>
						{:else}
							<button class="btn btn-primary btn-sm" onclick={() => handleDownload(model.id)} disabled={downloadingModelId !== null}>
								<Download size={10} /> Download
							</button>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	</div>

	<div class="mm-tier">
		<div class="mm-tier-header">
			<span class="mm-tier-badge mm-tier-balanced">Balanced</span>
			<span class="mm-tier-desc">For 16 GB RAM or more</span>
		</div>
		<div class="mm-grid">
			{#each balancedModels as model}
				{@const downloaded = isDownloaded(model.id)}
				{@const downloading = isDownloading(model.id)}
				{@const isLoaded = loadStatus.loaded && loadStatus.model_id === model.id}
				{@const tooBig = exceedsRam(model.ram_required_bytes)}
				<div class="mm-card" class:is-loaded={isLoaded}>
					<div class="mm-card-header">
						<span class="mm-card-name">{model.name}</span>
						<span class="mm-card-params">{model.params}</span>
					</div>
					<div class="mm-card-desc">{model.description}</div>
					<div class="mm-card-stats">
						<span class="mm-stat"><HardDrive size={10} /> {bytesToGB(model.size_bytes)} GB</span>
						<span class="mm-stat"><Cpu size={10} /> {bytesToGB(model.ram_required_bytes)} GB RAM</span>
					</div>
					{#if tooBig}
						<div class="mm-card-warn">
							<AlertTriangle size={10} />
							May not fit in available RAM
						</div>
					{/if}
					<div class="mm-card-actions">
						{#if isLoaded}
							<span class="mm-badge mm-badge-active">
								<CheckCircle size={10} /> Active
							</span>
						{:else if downloading}
							<span class="mm-badge mm-badge-downloading">Downloading...</span>
						{:else if downloaded}
							<button class="btn btn-primary btn-sm" onclick={() => handleLoad(model.id)}>
								<Play size={10} /> Load
							</button>
							<button class="btn btn-ghost btn-sm" onclick={() => handleDelete(model.id)}>
								<Trash2 size={10} />
							</button>
						{:else}
							<button class="btn btn-primary btn-sm" onclick={() => handleDownload(model.id)} disabled={downloadingModelId !== null}>
								<Download size={10} /> Download
							</button>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	</div>
{/if}

<style>
	.mm-loading {
		padding: var(--space-4);
		text-align: center;
		color: var(--color-text-tertiary);
		font-size: var(--text-sm);
	}

	.mm-error {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		background: color-mix(in srgb, var(--color-danger) 12%, var(--color-surface));
		border: 1px solid color-mix(in srgb, var(--color-danger) 25%, var(--color-border));
		border-radius: var(--radius-sm);
		margin-bottom: var(--space-3);
		font-size: var(--text-xs);
		color: var(--color-danger);
	}

	.mm-error-dismiss {
		margin-left: auto;
		background: none;
		border: none;
		color: var(--color-text-tertiary);
		font-size: var(--text-xs);
		cursor: pointer;
	}

	.mm-loaded {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-2) var(--space-3);
		background: color-mix(in srgb, var(--color-success) 12%, var(--color-surface));
		border: 1px solid color-mix(in srgb, var(--color-success) 25%, var(--color-border));
		border-radius: var(--radius-sm);
		margin-bottom: var(--space-3);
	}

	.mm-loaded-indicator {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-xs);
		color: var(--color-success);
		flex: 1;
	}

	.mm-ram-info {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		margin-bottom: var(--space-4);
	}

	.mm-progress {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		padding: var(--space-3);
		margin-bottom: var(--space-4);
	}

	.mm-progress-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		font-size: var(--text-xs);
		color: var(--color-text);
		margin-bottom: var(--space-2);
	}

	.mm-progress-bar {
		width: 100%;
		height: 4px;
		background: var(--color-surface-sunken);
		border-radius: 2px;
		overflow: hidden;
		margin-bottom: var(--space-1);
	}

	.mm-progress-fill {
		height: 100%;
		background: var(--color-accent);
		border-radius: 2px;
		transition: width 0.3s;
	}

	.mm-progress-stats {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}

	.mm-tier {
		margin-bottom: var(--space-4);
	}

	.mm-tier-header {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		margin-bottom: var(--space-3);
	}

	.mm-tier-badge {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		padding: 2px var(--space-2);
		border-radius: var(--radius-xs);
	}

	.mm-tier-fast {
		background: color-mix(in srgb, var(--color-success) 15%, var(--color-surface));
		color: var(--color-success);
	}

	.mm-tier-balanced {
		background: color-mix(in srgb, var(--color-accent) 15%, var(--color-surface));
		color: var(--color-accent);
	}

	.mm-tier-desc {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}

	.mm-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: var(--space-2);
	}

	.mm-card {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		padding: var(--space-3);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		background: var(--color-surface);
	}

	.mm-card.is-loaded {
		border-color: var(--color-success);
	}

	.mm-card-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-2);
	}

	.mm-card-name {
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
	}

	.mm-card-params {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}

	.mm-card-desc {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		line-height: 1.4;
	}

	.mm-card-stats {
		display: flex;
		gap: var(--space-3);
	}

	.mm-stat {
		display: flex;
		align-items: center;
		gap: 3px;
		font-family: var(--font-mono);
		font-size: 10px;
		color: var(--color-text-tertiary);
	}

	.mm-card-warn {
		display: flex;
		align-items: center;
		gap: var(--space-1);
		font-size: 10px;
		color: var(--color-warning);
	}

	.mm-card-actions {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding-top: var(--space-1);
	}

	.mm-badge {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		font-size: var(--text-xs);
		font-weight: 600;
		padding: 2px var(--space-2);
		border-radius: var(--radius-xs);
	}

	.mm-badge-active {
		background: color-mix(in srgb, var(--color-success) 15%, var(--color-surface));
		color: var(--color-success);
	}

	.mm-badge-downloading {
		background: color-mix(in srgb, var(--color-accent) 15%, var(--color-surface));
		color: var(--color-accent);
	}
</style>
