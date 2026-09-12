<script lang="ts">
	import { onMount } from 'svelte';
	import { app } from '$lib/stores/app.svelte';
	import { getTableMeta, extractErrorMessage } from '$lib/db-operations';
	import type { ColumnInfo } from '$lib/db-operations';
	import { PicassoScatterCanvas } from '$lib/charts';
	import type { ScatterChartConfig, BarChartData } from '$lib/charts/types';
	import { Settings, ScatterChart } from 'lucide-svelte';

	const CATEGORICAL_TYPES = new Set([
		'VARCHAR', 'TEXT', 'STRING', 'CHAR', 'BPCHAR', 'NAME', 'UUID', 'ENUM', 'BOOLEAN', 'BOOL'
	]);
	const NUMERIC_TYPES = new Set([
		'INTEGER', 'BIGINT', 'SMALLINT', 'TINYINT', 'INT', 'INT2', 'INT4', 'INT8',
		'DOUBLE', 'FLOAT', 'FLOAT4', 'FLOAT8', 'REAL', 'DECIMAL', 'NUMERIC',
		'HUGEINT', 'UINTEGER', 'UBIGINT', 'USMALLINT', 'UTINYINT'
	]);

	let selectedTable = $state('');
	let tableColumns = $state<ColumnInfo[]>([]);
	let tableColumnsLoading = $state(false);
	let tableColumnsError = $state('');

	let numericColumns = $derived(
		tableColumns.filter((c) => NUMERIC_TYPES.has(c.type.toUpperCase()))
	);
	let categoricalColumns = $derived(
		tableColumns.filter((c) => CATEGORICAL_TYPES.has(c.type.toUpperCase()) || !NUMERIC_TYPES.has(c.type.toUpperCase()))
	);

	let selectedX = $state('');
	let selectedY = $state('');
	let selectedGroup = $state('');
	let limit = $state(500);
	let pointSize = $state(8);
	let pointOpacity = $state(0.75);
	let showLegend = $state(true);
	let chartId = $state('picasso-scatter-1');
	let clickedPoint: { x: number; y: number; group: string } | null = $state(null);
	let selectedRows: BarChartData[] = $state([]);

	let configReady = $derived(!!selectedTable && !!selectedX && !!selectedY);

	let config = $derived<ScatterChartConfig | null>(
		configReady
			? {
					id: chartId,
					table: selectedTable,
					xField: { field: selectedX, label: selectedX },
					yField: { field: selectedY, label: selectedY },
					groupBy: selectedGroup ? { field: selectedGroup, label: selectedGroup } : undefined,
					limit,
					pointSize,
					pointOpacity,
					showLegend
				}
			: null
	);

	async function loadTableColumns(tableName: string) {
		tableColumnsLoading = true;
		tableColumnsError = '';
		try {
			const meta = await getTableMeta(tableName);
			tableColumns = meta.columns;
		} catch (e) {
			tableColumnsError = extractErrorMessage(e, 'Failed to load columns');
			tableColumns = [];
		}
		tableColumnsLoading = false;
	}

	function handlePointClick(detail: { x: number; y: number; group: string }) {
		clickedPoint = detail;
		setTimeout(() => {
			clickedPoint = null;
		}, 2000);
	}

	function handleSelectionChange(selected: BarChartData[]) {
		selectedRows = selected;
	}

	function handleTableChange(e: Event) {
		const value = (e.target as HTMLSelectElement).value;
		selectedTable = value;
		selectedX = '';
		selectedY = '';
		selectedGroup = '';
		if (value) {
			loadTableColumns(value);
		} else {
			tableColumns = [];
		}
	}

	onMount(() => {
		if (app.tables.length > 0) {
			selectedTable = app.tables[0];
			loadTableColumns(app.tables[0]);
		}
	});
</script>

<svelte:head>
	<title>Picasso Charts — Scatter — Data Monster</title>
</svelte:head>

<div class="chart-detail">
	<div class="chart-panel">
		<div class="chart-panel-header">
			<div class="chart-panel-badge">
				<span class="section-number">PICASSO-004</span>
			</div>
		</div>
		<div class="chart-panel-body">
			{#if !config}
				<div class="chart-placeholder">
					<ScatterChart size={32} />
					<h3>Configure your chart</h3>
					<p>Select a table and two numeric columns to render a scatter plot with Picasso.js.</p>
				</div>
			{:else}
				<PicassoScatterCanvas {config} onPointClick={handlePointClick} onSelectionChange={handleSelectionChange} />
			{/if}
			{#if clickedPoint}
				<div class="bar-toast">
					<span class="bar-toast-key">{clickedPoint.group || 'point'}</span>
					<span class="bar-toast-val">x: {clickedPoint.x.toLocaleString()}</span>
					<span class="bar-toast-val">y: {clickedPoint.y.toLocaleString()}</span>
				</div>
			{/if}
			{#if !clickedPoint && selectedRows.length > 0}
				<div class="bar-toast">
					<span class="bar-toast-key">{selectedRows.length} selected</span>
					<span class="bar-toast-val">drag to lasso</span>
				</div>
			{/if}
		</div>
	</div>

	<aside class="drawer">
		<div class="drawer-header">
			<div class="drawer-header-left">
				<Settings size={14} />
				<h2 class="drawer-title">Chart config</h2>
			</div>
		</div>
		<div class="drawer-body">
			<div class="config-section">
				<h3 class="config-section-title">Data</h3>

				<div class="field">
					<label class="field-label" for="chart-table">Table</label>
					<select
						id="chart-table"
						class="input input-mono"
						value={selectedTable}
						onchange={handleTableChange}
					>
						<option value="">Select a table...</option>
						{#each app.tables as t}
							<option value={t}>{t}</option>
						{/each}
					</select>
				</div>

				{#if tableColumnsLoading}
					<div class="config-hint">Loading columns...</div>
				{:else if tableColumnsError}
					<div class="config-error">{tableColumnsError}</div>
				{:else if selectedTable && tableColumns.length > 0}
					<div class="field">
						<label class="field-label" for="chart-x">X axis</label>
						<select
							id="chart-x"
							class="input input-mono"
							value={selectedX}
							onchange={(e) => selectedX = (e.target as HTMLSelectElement).value}
						>
							<option value="">Select X field...</option>
							{#each numericColumns as col}
								<option value={col.name}>{col.name} ({col.type})</option>
							{/each}
						</select>
						<span class="field-hint">Pick a numeric column for the horizontal axis</span>
					</div>

					<div class="field">
						<label class="field-label" for="chart-y">Y axis</label>
						<select
							id="chart-y"
							class="input input-mono"
							value={selectedY}
							onchange={(e) => selectedY = (e.target as HTMLSelectElement).value}
						>
							<option value="">Select Y field...</option>
							{#each numericColumns as col}
								<option value={col.name}>{col.name} ({col.type})</option>
							{/each}
						</select>
						<span class="field-hint">Pick a numeric column for the vertical axis</span>
					</div>

					<div class="field">
						<label class="field-label" for="chart-group">Color by (optional)</label>
						<select
							id="chart-group"
							class="input input-mono"
							value={selectedGroup}
							onchange={(e) => selectedGroup = (e.target as HTMLSelectElement).value}
						>
							<option value="">None</option>
							{#each categoricalColumns as col}
								<option value={col.name}>{col.name} ({col.type})</option>
							{/each}
						</select>
						<span class="field-hint">Group points by a categorical column for color coding</span>
					</div>
				{/if}
			</div>

			<hr class="config-divider" />

			<div class="config-section">
				<h3 class="config-section-title">Options</h3>

				<div class="field">
					<label class="field-label" for="chart-limit">Max points</label>
					<input
						id="chart-limit"
						type="number"
						class="input input-mono"
						min="1"
						max="10000"
						bind:value={limit}
					/>
					<span class="field-hint">Limits rows returned (default 500)</span>
				</div>

				<div class="field">
					<label class="field-label" for="chart-size">Point size</label>
					<input
						id="chart-size"
						type="range"
						min="2"
						max="24"
						step="1"
						bind:value={pointSize}
					/>
					<div class="range-row">
						<span class="field-hint">2</span>
						<span class="range-current">{pointSize}px</span>
						<span class="field-hint">24</span>
					</div>
				</div>

				<div class="field">
					<label class="field-label" for="chart-opacity">Point opacity</label>
					<input
						id="chart-opacity"
						type="range"
						min="0.1"
						max="1"
						step="0.05"
						bind:value={pointOpacity}
					/>
					<div class="range-row">
						<span class="field-hint">0.1</span>
						<span class="range-current">{pointOpacity.toFixed(2)}</span>
						<span class="field-hint">1.0</span>
					</div>
				</div>

				<div class="field">
					<label class="field-label" for="chart-fqid">Chart ID</label>
					<input
						id="chart-fqid"
						type="text"
						class="input input-mono"
						bind:value={chartId}
					/>
				</div>

				<label class="toggle-row">
					<div class="toggle-label">
						<span>Show legend</span>
						<span class="toggle-hint">Color group legend (requires color by)</span>
					</div>
					<button
						class="toggle"
						class:toggle-on={showLegend}
						onclick={() => showLegend = !showLegend}
						role="switch"
						aria-checked={showLegend}
					>
						<span class="toggle-knob"></span>
					</button>
				</label>
			</div>

			<hr class="config-divider" />

			{#if config}
				<div class="config-section">
					<h3 class="config-section-title">Generated SQL</h3>
					<pre class="sql-preview"><code>SELECT "{selectedX}", "{selectedY}"{selectedGroup ? `, "${selectedGroup}"` : ''}
FROM "{selectedTable}"
LIMIT {limit}</code></pre>
				</div>
			{/if}
		</div>
	</aside>
</div>

<style>
	.chart-detail {
		display: flex;
		height: 100%;
		overflow: hidden;
	}

	.chart-panel {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		border-right: 1px solid var(--color-border);
		height: 100%;
	}

	.chart-panel-header {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: var(--space-2);
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
		height: 41px;
		box-sizing: border-box;
	}

	.chart-panel-badge {
		margin-left: auto;
	}

	.section-number {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		color: var(--color-accent);
		padding: 2px var(--space-2);
		border: 1px solid var(--color-accent-muted);
		border-radius: var(--radius-xs);
		background: var(--color-accent-muted);
		white-space: nowrap;
	}

	.chart-panel-body {
		flex: 1;
		min-height: 0;
		overflow: hidden;
		position: relative;
	}

	.bar-toast {
		position: absolute;
		bottom: var(--space-6);
		left: 50%;
		transform: translateX(-50%);
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-2) var(--space-4);
		background: var(--color-text);
		color: var(--color-surface);
		border-radius: var(--radius-md);
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		box-shadow: 0 4px 12px oklch(0.15 0.005 250 / 0.3);
		z-index: 10;
		animation: toast-in 0.2s ease-out;
		pointer-events: none;
	}

	.bar-toast-key {
		font-weight: 600;
	}

	.bar-toast-val {
		opacity: 0.7;
	}

	@keyframes toast-in {
		from {
			opacity: 0;
			transform: translateX(-50%) translateY(8px);
		}
		to {
			opacity: 1;
			transform: translateX(-50%) translateY(0);
		}
	}

	.chart-placeholder {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: var(--space-4);
		padding: var(--space-24) var(--space-6);
		text-align: center;
		color: var(--color-text-tertiary);
	}

	.chart-placeholder h3 {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 600;
		color: var(--color-text);
		margin: 0;
	}

	.chart-placeholder p {
		font-size: var(--text-sm);
		margin: 0;
		max-width: 32ch;
	}

	.drawer {
		width: 360px;
		flex-shrink: 0;
		display: flex;
		flex-direction: column;
		min-width: 0;
	}

	.drawer-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--color-border);
		flex-shrink: 0;
		height: 41px;
		box-sizing: border-box;
	}

	.drawer-header-left {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		color: var(--color-text-tertiary);
	}

	.drawer-title {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
		margin: 0;
	}

	.drawer-body {
		flex: 1;
		padding: var(--space-4);
		overflow-y: auto;
		display: flex;
		flex-direction: column;
	}

	.config-section {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}

	.config-section-title {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.08em;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
		margin: 0;
	}

	.config-divider {
		border: none;
		height: 0;
		border-top: 1px dashed var(--color-border);
		margin: var(--space-6) 0;
	}

	.config-hint {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
	}

	.config-error {
		font-family: var(--font-mono);
		font-size: 9px;
		padding: var(--space-2) var(--space-3);
		color: var(--color-danger);
		background: oklch(0.95 0.03 22);
		border: 1px solid oklch(0.9 0.04 22);
		border-radius: var(--radius-xs);
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.field-label {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
	}

	.field-hint {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		letter-spacing: 0.02em;
	}

	.input {
		padding: var(--space-2) var(--space-3);
		border: 1px solid var(--color-border-strong);
		font-family: var(--font-body);
		font-size: var(--text-sm);
		color: var(--color-text);
		background: var(--color-surface);
		border-radius: var(--radius-xs);
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.input::placeholder {
		color: var(--color-text-tertiary);
	}

	.input:focus {
		outline: none;
		border-color: var(--color-accent);
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}

	.input-mono {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
	}

	select.input option {
		font-family: var(--font-mono);
	}

	input[type='range'] {
		width: 100%;
		height: 4px;
		appearance: none;
		background: var(--color-border);
		border-radius: var(--radius-full);
		outline: none;
		cursor: pointer;
	}

	input[type='range']::-webkit-slider-thumb {
		appearance: none;
		width: 14px;
		height: 14px;
		border-radius: 50%;
		background: var(--color-accent);
		border: 2px solid #fff;
		box-shadow: 0 1px 3px oklch(0.22 0.005 250 / 0.2);
		cursor: pointer;
	}

	.range-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.range-current {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 600;
		color: var(--color-text);
	}

	.toggle-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-4);
		cursor: pointer;
	}

	.toggle-label {
		display: flex;
		flex-direction: column;
		gap: 2px;
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
	}

	.toggle-hint {
		font-family: var(--font-mono);
		font-size: 9px;
		font-weight: 400;
		color: var(--color-text-tertiary);
		letter-spacing: 0.02em;
	}

	.toggle {
		width: 36px;
		height: 20px;
		border-radius: var(--radius-full);
		border: 1px solid var(--color-border-strong);
		background: var(--color-surface-sunken);
		cursor: pointer;
		position: relative;
		transition: background var(--duration-fast) ease, border-color var(--duration-fast) ease;
		flex-shrink: 0;
		padding: 0;
	}

	.toggle-on {
		background: var(--color-accent);
		border-color: var(--color-accent);
	}

	.toggle-knob {
		position: absolute;
		top: 2px;
		left: 2px;
		width: 14px;
		height: 14px;
		border-radius: 50%;
		background: #fff;
		transition: transform var(--duration-fast) var(--ease-out-expo);
		box-shadow: 0 1px 2px oklch(0.22 0.005 250 / 0.15);
	}

	.toggle-on .toggle-knob {
		transform: translateX(16px);
	}

	.sql-preview {
		margin: 0;
		padding: var(--space-3);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		background: var(--color-surface-sunken);
		overflow-x: auto;
		white-space: pre;
	}

	.sql-preview code {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		line-height: 1.6;
	}
</style>
