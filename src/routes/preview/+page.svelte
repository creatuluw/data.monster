<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import { app } from '$lib/stores/app.svelte';
	import {
		previewFile,
		executeQuery,
		type PreviewData,
		type ColumnOverride
	} from '$lib/db-operations';
	import PreviewPane from '$lib/components/PreviewPane.svelte';
	import { LoaderCircle } from 'lucide-svelte';

	let loading = $state(true);
	let error = $state('');
	let paneData: PreviewData | null = $state(null);
	let columnOverrides: ColumnOverride[] = $state([]);
	let tableName = $state('');
	let filePath = $state('');
	let fileSize = $state<number | null>(null);

	let fileType = $derived.by(() => {
		const ext = filePath.split('.').pop()?.toLowerCase() ?? '';
		if (ext === 'parquet') return 'Parquet';
		if (ext === 'json' || ext === 'jsonl' || ext === 'ndjson') return 'JSON';
		return 'CSV';
	});

	let fieldCount = $derived.by(() => paneData?.columns.length ?? 0);
	let rowCount = $derived.by(() => paneData?.totalRows ?? 0);

	function formatNumber(n: number): string {
		if (n >= 1_000_000) return (n / 1_000_000).toFixed(1).replace(/\.0$/, '') + 'M';
		if (n >= 1_000) return (n / 1_000).toFixed(1).replace(/\.0$/, '') + 'K';
		return n.toLocaleString();
	}

	function formatSize(bytes: number): string {
		if (bytes >= 1_048_576) return (bytes / 1_048_576).toFixed(1).replace(/\.0$/, '') + ' MB';
		if (bytes >= 1_024) return (bytes / 1_024).toFixed(1).replace(/\.0$/, '') + ' KB';
		return bytes + ' B';
	}

	onMount(async () => {
		if (!app.pendingFile) {
			goto('/connect', { replaceState: true });
			return;
		}

		filePath = app.pendingFile.path;
		tableName = app.pendingFile.tableName;

		try {
			const result = await previewFile(filePath, 100);
			const colInfos = result.columnTypes;

			paneData = {
				columns: result.columns,
				rows: result.rows,
				detectedTypes: colInfos.map(c => ({ name: c.name, type: c.type })),
				totalRows: result.totalRows,
				sourceName: tableName
			};

			columnOverrides = colInfos.map(c => ({
				originalName: c.name,
				newName: c.name,
				detectedType: c.type,
				overrideType: null,
				enabled: true
			}));

			try {
				const safePath = filePath.replace(/\\/g, '/');
				const sizeRes = await executeQuery(`SELECT file_size('${safePath}') AS sz`) as any;
				if (sizeRes?.data?.[0]?.[0] != null) fileSize = Number(sizeRes.data[0][0]);
			} catch {}
		} catch (e: unknown) {
			const msg = e instanceof Error ? e.message : typeof e === 'string' ? e : 'Failed to load file';
			error = msg || 'Failed to load file';
		} finally {
			loading = false;
		}
	});

	function handleContinue() {
		if (!paneData) return;

		const ext = filePath.split('.').pop()?.toLowerCase() ?? 'csv';
		const readFn = ext === 'parquet'
			? 'read_parquet'
			: ext === 'json' || ext === 'jsonl' || ext === 'ndjson'
				? 'read_json_auto'
				: 'read_csv_auto';

		const safePath = filePath.replace(/\\/g, '/');
		const enabled = columnOverrides.filter(o => o.enabled);

		const selectList = enabled.map(o => {
			const col = `"${o.originalName}"`;
			const alias = o.newName !== o.originalName ? ` AS "${o.newName}"` : '';
			const cast = o.overrideType && o.overrideType !== o.detectedType
				? `CAST(${col} AS ${o.overrideType})`
				: col;
			return `  ${cast}${alias}`;
		}).join(',\n');

		const sql = `-- ${enabled.length} columns, ${paneData.totalRows.toLocaleString()} rows\nCREATE TABLE "${tableName}" AS\nSELECT\n${selectList}\nFROM ${readFn}('${safePath}');`;

		app.pendingSql = sql;
		app.pendingAutoRun = true;
		app.pendingPreviewData = {
			tableName,
			columns: enabled.map(o => o.newName !== o.originalName ? o.newName : o.originalName)
		};
		goto('/query');
	}

	function handleBack() {
		app.pendingFile = null;
		goto('/connect');
	}
</script>

<svelte:head>
	<title>Preview — Data Monster</title>
</svelte:head>

<div class="preview-page">
	<div class="preview-shell">
		<div class="preview-header">
			<h2 class="preview-title">Preview</h2>
		</div>

		{#if paneData}
			<div class="preview-meta-bar">
				<span class="meta-item"><span class="meta-label">Name</span>{tableName}</span>
				<span class="meta-sep">|</span>
				<span class="meta-item"><span class="meta-label">Type</span>{fileType}</span>
				<span class="meta-sep">|</span>
				<span class="meta-item"><span class="meta-label">Fields</span>{fieldCount}</span>
				<span class="meta-sep">|</span>
				<span class="meta-item"><span class="meta-label">Rows</span>{formatNumber(rowCount)}</span>
				<span class="meta-sep">|</span>
				<span class="meta-item"><span class="meta-label">Size</span>{fileSize != null ? formatSize(fileSize) : '\u2014'}</span>
			</div>
		{/if}

		<hr class="preview-divider" />

		<div class="preview-content">
		{#if loading}
			<div class="preview-loading">
				<LoaderCircle size={32} class="spinner-icon" />
				<span>Loading file...</span>
			</div>
		{:else if error}
			<div class="preview-error">
				<span class="preview-error-text">{error}</span>
				<button onclick={handleBack} class="btn btn-secondary btn-sm">Go back</button>
			</div>
		{:else if paneData}
			<div class="preview-pane-wrap">
				<PreviewPane
					data={paneData}
					bind:columnOverrides
					onnext={handleContinue}
				/>
			</div>
		{/if}
	</div>
	</div>
</div>

<style>
	:global(.app-body) {
		min-height: 0;
	}

	:global(.app-main) {
		overflow: hidden !important;
		padding: 0 !important;
		display: flex !important;
		flex-direction: column !important;
	}

	.preview-page {
		flex: 1;
		display: flex;
		flex-direction: column;
		height: 100%;
		overflow: hidden;
	}

	.preview-shell {
		flex: 1;
		display: flex;
		flex-direction: column;
		height: 100%;
		overflow: hidden;
		padding: var(--space-6);
		gap: var(--space-4);
	}

	.preview-header {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		flex-shrink: 0;
	}

	.preview-title {
		font-family: var(--font-display);
		font-size: var(--text-lg);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
		margin: 0;
	}

	.preview-meta-bar {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-1) var(--space-4);
		background: var(--color-surface-raised);
		border-radius: var(--radius-lg);
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
	}

	.meta-item {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
	}

	.meta-label {
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.meta-sep {
		color: var(--color-border);
	}

	.preview-divider {
		border: none;
		height: 0;
		border-top: 1px dashed var(--color-border);
	}

	.preview-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.preview-loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: var(--space-4);
		flex: 1;
		color: var(--color-text-tertiary);
	}

	.preview-error {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: var(--space-3);
		flex: 1;
		padding: var(--space-8);
		text-align: center;
	}

	.preview-error-text {
		font-size: var(--text-sm);
		color: var(--color-danger);
	}

	.preview-pane-wrap {
		flex: 1;
		overflow: auto;
	}

	.spinner-icon {
		animation: spin 1s linear infinite;
	}
</style>
