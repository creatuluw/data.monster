<script lang="ts">
	let {
		onfile,
		onurl,
		loading = false
	}: {
		onfile: (file: File) => void;
		onurl: (url: string) => void;
		loading?: boolean;
	} = $props();

	let urlInput = $state('');
	let isDragOver = $state(false);

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		isDragOver = false;
		if (e.dataTransfer?.files.length) {
			onfile(e.dataTransfer.files[0]);
		}
	}

	function handleFileInput(e: Event) {
		const input = e.target as HTMLInputElement;
		if (input.files?.length) {
			onfile(input.files[0]);
		}
	}

	function handleUrlSubmit() {
		const url = urlInput.trim();
		if (url) onurl(url);
	}
</script>

<div class="connector">
	<label
		class="dropzone"
		class:drag-over={isDragOver}
		role="button"
		tabindex="0"
		ondragover={(e) => { e.preventDefault(); isDragOver = true; }}
		ondragleave={() => { isDragOver = false; }}
		ondrop={handleDrop}
		onkeydown={(e) => e.key === 'Enter' && (e.target as HTMLElement).querySelector('input')?.click()}
	>
		<svg width="32" height="32" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5" style="color: var(--color-text-tertiary); margin-bottom: var(--space-3);">
			<path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5m-13.5-9L12 3m0 0 4.5 4.5M12 3v13.5" />
		</svg>
		<span class="dropzone-label">{loading ? 'Loading...' : 'Drop a file here'}</span>
		<span class="dropzone-hint">or click to browse</span>
		<span class="dropzone-formats">CSV · Parquet · JSON · JSONL</span>
		<input type="file" accept=".csv,.parquet,.json,.jsonl,.ndjson" class="hidden" onchange={handleFileInput} style="display: none;" />
	</label>

	<div class="divider-row">
		<hr class="divider" style="flex: 1;" />
		<span class="divider-label">or paste a URL</span>
		<hr class="divider" style="flex: 1;" />
	</div>

	<div class="url-row">
		<div class="url-input-wrap">
			<svg width="16" height="16" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5" style="color: var(--color-text-tertiary); margin-right: var(--space-2); flex-shrink: 0;">
				<path stroke-linecap="round" stroke-linejoin="round" d="M13.19 8.688a4.5 4.5 0 0 1 1.242 7.244l-4.5 4.5a4.5 4.5 0 0 1-6.364-6.364l1.757-1.757m13.35-.622 1.757-1.757a4.5 4.5 0 0 0-6.364-6.364l-4.5 4.5a4.5 4.5 0 0 0 1.242 7.244" />
			</svg>
			<input
				type="url"
				bind:value={urlInput}
				placeholder="https://example.com/data.csv"
				class="url-input"
				onkeydown={(e) => e.key === 'Enter' && handleUrlSubmit()}
			/>
		</div>
		<button
			onclick={handleUrlSubmit}
			disabled={!urlInput.trim() || loading}
			class="btn btn-primary"
		>
			Go
		</button>
	</div>
</div>

<style>
	.connector {
		max-width: 32rem;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		gap: var(--space-8);
		padding: var(--space-12) 0;
	}

	.dropzone {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: var(--space-16) var(--space-6);
		border: 2px dashed var(--color-border);
		background: var(--color-surface-raised);
		cursor: pointer;
		transition: border-color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.dropzone:hover {
		border-color: var(--color-border-strong);
		background: var(--color-surface-sunken);
	}

	.dropzone.drag-over {
		border-color: var(--color-accent);
		background: var(--color-accent-muted);
	}

	.dropzone-label {
		font-family: var(--font-display);
		font-size: var(--text-base);
		font-weight: 600;
		color: var(--color-text);
	}

	.dropzone-hint {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin-top: var(--space-1);
	}

	.dropzone-formats {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		margin-top: var(--space-3);
		letter-spacing: 0.04em;
	}

	.divider-row {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.divider-label {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		white-space: nowrap;
	}

	.url-row {
		display: flex;
		gap: var(--space-2);
	}

	.url-input-wrap {
		flex: 1;
		display: flex;
		align-items: center;
		border: 1px solid var(--color-border-strong);
		padding: var(--space-2) var(--space-3);
		background: var(--color-surface);
		border-radius: var(--radius-xs);
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.url-input-wrap:focus-within {
		border-color: var(--color-accent);
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}

	.url-input {
		flex: 1;
		border: none;
		background: transparent;
		font-family: var(--font-body);
		font-size: var(--text-sm);
		color: var(--color-text);
		outline: none;
	}

	.url-input::placeholder {
		color: var(--color-text-tertiary);
	}
</style>
