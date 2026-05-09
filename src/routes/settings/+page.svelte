<script lang="ts">
	import { onMount } from 'svelte';
	import { getSettings, saveSettings, extractErrorMessage } from '$lib/db-operations';
	import { analyst } from '$lib/stores/analyst.svelte';
	import { Settings, Save, CheckCircle, AlertCircle, Database, Cloud, Cpu } from 'lucide-svelte';
	import ModelManager from '$lib/components/ModelManager.svelte';

	let apiUrl = $state('');
	let apiKey = $state('');
	let apiModel = $state('');
	let inferenceMode = $state<'remote' | 'local'>('remote');
	let localModelId = $state('');
	let saving = $state(false);
	let saved = $state(false);
	let error = $state('');
	let loading = $state(true);

	onMount(async () => {
		try {
			const settings = await getSettings();
			apiUrl = (settings.llmApiUrl as string) || 'https://api.z.ai/api/coding/paas/v4/chat/completions';
			apiKey = (settings.llmApiKey as string) || '';
			apiModel = (settings.llmModel as string) || 'glm-5';
			inferenceMode = (settings.inferenceMode as string) === 'local' ? 'local' : 'remote';
			localModelId = (settings.localModelId as string) || '';
		} catch {
			apiUrl = 'https://api.z.ai/api/coding/paas/v4/chat/completions';
			apiModel = 'glm-5';
		}
		loading = false;
	});

	async function handleSave() {
		saving = true;
		saved = false;
		error = '';
		try {
			await saveSettings({
				llmApiUrl: apiUrl,
				llmApiKey: apiKey,
				llmModel: apiModel,
				inferenceMode,
				localModelId: localModelId || null
			});
			analyst.apiUrl = apiUrl;
			analyst.apiKey = apiKey;
			analyst.apiModel = apiModel;
			analyst.inferenceMode = inferenceMode;
			analyst.localModelId = localModelId || null;
			saved = true;
			setTimeout(() => { saved = false; }, 3000);
		} catch (e) {
			error = extractErrorMessage(e, 'Failed to save settings');
		}
		saving = false;
	}
</script>

<svelte:head>
	<title>Settings — Data Monster</title>
</svelte:head>

<div class="settings-page">
	<div class="settings-header">
		<Settings size={20} />
		<h1 class="settings-title">Settings</h1>
		<div style="flex:1"></div>
		<a href="/settings/internal-db" class="btn btn-secondary btn-sm" title="Internal Database">
			<Database size={14} />
			Internal DB
		</a>
	</div>

	{#if loading}
		<div class="settings-loading">Loading...</div>
	{:else}
		<div class="settings-section">
			<h2 class="section-title">AI Analyst</h2>
			<p class="section-desc">Choose how to power the AI Analyst — remote API or local on-device inference.</p>

			<div class="mode-toggle-row">
				<button
					class="mode-btn"
					class:active={inferenceMode === 'remote'}
					onclick={() => { inferenceMode = 'remote'; }}
				>
					<Cloud size={16} />
					<span class="mode-btn-text">
						<span class="mode-btn-label">Remote API</span>
						<span class="mode-btn-desc">Use a cloud LLM provider</span>
					</span>
				</button>
				<button
					class="mode-btn"
					class:active={inferenceMode === 'local'}
					onclick={() => { inferenceMode = 'local'; }}
				>
					<Cpu size={16} />
					<span class="mode-btn-text">
						<span class="mode-btn-label">Local AI</span>
						<span class="mode-btn-desc">Run models on your device</span>
					</span>
				</button>
			</div>

			{#if inferenceMode === 'remote'}
				<div class="settings-section" style="margin-top: var(--space-4); border: 1px solid var(--color-border);">
					<h2 class="section-title">LLM Configuration</h2>
					<p class="section-desc">Configure the AI analyst endpoint. Settings are stored locally on disk.</p>

					<div class="field">
						<label for="api-url" class="field-label">API URL</label>
						<input
							id="api-url"
							type="text"
							class="input input-mono"
							bind:value={apiUrl}
							placeholder="https://api.z.ai/api/coding/paas/v4/chat/completions"
						/>
						<span class="field-hint">The chat completions endpoint URL</span>
					</div>

					<div class="field">
						<label for="api-key" class="field-label">API Key</label>
						<input
							id="api-key"
							type="password"
							class="input input-mono"
							bind:value={apiKey}
							placeholder="sk-..."
						/>
						<span class="field-hint">Your API key (stored locally, never sent to us)</span>
					</div>

					<div class="field">
						<label for="api-model" class="field-label">Model</label>
						<input
							id="api-model"
							type="text"
							class="input"
							bind:value={apiModel}
							placeholder="glm-5"
						/>
						<span class="field-hint">The model identifier to use</span>
					</div>
				</div>
			{:else}
				<div class="settings-section" style="margin-top: var(--space-4); border: 1px solid var(--color-border);">
					<h2 class="section-title">Local Model</h2>
					<p class="section-desc">Download and manage local AI models for offline use.</p>
					<div class="field-hint" style="margin-top: var(--space-2); margin-bottom: var(--space-4);">
						Local models require ~2-5 GB of disk space and run on your CPU. Chat responses may be slower than remote APIs.
					</div>
					<ModelManager onselect={(id) => { localModelId = id; }} />
				</div>
			{/if}

			<div class="save-row">
				<button class="btn btn-primary" onclick={handleSave} disabled={saving}>
					{#if saving}
						<svg class="spinner spinner--sm" viewBox="0 0 24 24" fill="none">
							<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2" opacity="0.25" />
							<path d="M12 2a10 10 0 0 1 10 10" stroke="currentColor" stroke-width="2" stroke-linecap="round" />
						</svg>
						Saving
					{:else}
						<Save size={14} />
						Save settings
					{/if}
				</button>
				{#if saved}
					<span class="save-status save-status-ok"><CheckCircle size={14} /> Saved</span>
				{/if}
				{#if error}
					<span class="save-status save-status-err"><AlertCircle size={14} /> {error}</span>
				{/if}
			</div>
		</div>
	{/if}
</div>

<style>
	.settings-page {
		max-width: 48rem;
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
		padding: var(--space-6);
	}

	.settings-header {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		color: var(--color-text);
	}

	.settings-title {
		font-family: var(--font-display);
		font-size: var(--text-lg);
		font-weight: 700;
		letter-spacing: -0.02em;
		color: var(--color-text);
		margin: 0;
	}

	.settings-loading {
		padding: var(--space-8);
		text-align: center;
		color: var(--color-text-tertiary);
		font-size: var(--text-sm);
	}

	.settings-section {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		padding: var(--space-6);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		background: var(--color-surface);
	}

	.section-title {
		font-family: var(--font-display);
		font-size: var(--text-base);
		font-weight: 600;
		color: var(--color-text);
		margin: 0;
	}

	.section-desc {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: 0;
		margin-top: calc(var(--space-2) * -1);
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.field-label {
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.field-hint {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		opacity: 0.7;
	}

	.input-mono {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
	}

	.save-row {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding-top: var(--space-2);
	}

	.save-status {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		font-size: var(--text-xs);
		font-weight: 600;
	}

	.save-status-ok {
		color: var(--color-success);
	}

	.save-status-err {
		color: var(--color-danger);
	}

	.spinner--sm {
		width: 14px;
		height: 14px;
	}

	.mode-toggle-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: var(--space-3);
	}

	.mode-btn {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-4);
		border: 2px solid var(--color-border);
		border-radius: var(--radius-md);
		background: var(--color-surface);
		color: var(--color-text-tertiary);
		cursor: pointer;
		transition: border-color 0.15s, background 0.15s, color 0.15s;
		text-align: center;
	}

	.mode-btn:hover {
		border-color: var(--color-text-tertiary);
	}

	.mode-btn.active {
		border-color: var(--color-accent);
		background: color-mix(in srgb, var(--color-accent) 8%, var(--color-surface));
		color: var(--color-text);
	}

	.mode-btn-text {
		display: flex;
		flex-direction: column;
		gap: 1px;
	}

	.mode-btn-label {
		font-size: var(--text-sm);
		font-weight: 600;
	}

	.mode-btn-desc {
		font-size: var(--text-xs);
		opacity: 0.6;
	}
</style>
