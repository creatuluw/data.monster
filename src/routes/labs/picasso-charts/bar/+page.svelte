<script lang="ts">
	import { onMount } from 'svelte';
	import { app } from '$lib/stores/app.svelte';
	import { getTableMeta, extractErrorMessage } from '$lib/db-operations';
	import type { ColumnInfo } from '$lib/db-operations';
	import { PicassoBarChartCanvas } from '$lib/charts';
	import type { BarChartConfig } from '$lib/charts/types';
	import { Settings, BarChart2 } from 'lucide-svelte';

	const CATEGORICAL_TYPES = new Set([
		'VARCHAR', 'TEXT', 'STRING', 'CHAR', 'BPCHAR', 'NAME', 'UUID', 'ENUM', 'BOOLEAN', 'BOOL'
	]);
	const NUMERIC_TYPES = new Set([
		'INTEGER', 'BIGINT', 'SMALLINT', 'TINYINT', 'INT', 'INT2', 'INT4', 'INT8',
		'DOUBLE', 'FLOAT', 'FLOAT4', 'FLOAT8', 'REAL', 'DECIMAL', 'NUMERIC',
		'HUGEINT', 'UINTEGER', 'UBIGINT', 'USMALLINT', 'UTINYINT'
	]);

	const AGGREGATES = ['SUM', 'AVG', 'COUNT', 'MIN', 'MAX'] as const;

	let selectedTable = $state('');
	let tableColumns = $state<ColumnInfo[]>([]);
	let tableColumnsLoading = $state(false);
	let tableColumnsError = $state('');

	let dimColumns = $derived(
		tableColumns.filter((c) => CATEGORICAL_TYPES.has(c.type.toUpperCase()) || !NUMERIC_TYPES.has(c.type.toUpperCase()))
	);
	let metricColumns = $derived(
		tableColumns.filter((c) => NUMERIC_TYPES.has(c.type.toUpperCase()))
	);

	let selectedDim = $state('');
	let selectedMetric = $state('');
	let selectedAggregate: 'SUM' | 'AVG' | 'COUNT' | 'MIN' | 'MAX' = $state('SUM');
	let sortDirection: 'ASC' | 'DESC' = $state('DESC');
	let orientation: 'vertical' | 'horizontal' = $state('vertical');
	let limit = $state(20);
	let clickToFilter = $state(true);
	let showValues = $state(false);
	let valueFormat: 'number' | 'currency' | 'percent' | 'compact' = $state('number');
	let chartId = $state('picasso-1');
	let clickedBar: { category: string; value: number } | null = $state(null);

	let configReady = $derived(!!selectedTable && !!selectedDim && !!selectedMetric);

	let config = $derived<BarChartConfig | null>(
		configReady
			? {
					id: chartId,
					table: selectedTable,
					dimension: { field: selectedDim, label: selectedDim },
					metric: { field: selectedMetric, label: selectedMetric, aggregate: selectedAggregate },
					orientation,
					sortDirection,
					limit,
					clickToFilter,
					showValues,
					valueFormat
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

	function handleBarClick(detail: { category: string; value: number }) {
		clickedBar = detail;
		setTimeout(() => {
			clickedBar = null;
		}, 2000);
	}

	function handleTableChange(e: Event) {
		const value = (e.target as HTMLSelectElement).value;
		selectedTable = value;
		selectedDim = '';
		selectedMetric = '';
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
	<title>Picasso Charts — Data Monster</title>
</svelte:head>

<div class="chart-detail">
	<div class="chart-panel">
		<div class="chart-panel-header">
			<div class="chart-panel-badge">
				<span class="section-number">PICASSO-001</span>
			</div>
		</div>
		<div class="chart-panel-body">
			{#if !config}
				<div class="chart-placeholder">
					<BarChart2 size={32} />
					<h3>Configure your chart</h3>
					<p>Select a table, dimension, and metric to render the bar chart with Picasso.js.</p>
				</div>
			{:else}
				<PicassoBarChartCanvas {config} onBarClick={handleBarClick} />
			{/if}
			{#if clickedBar}
				<div class="bar-toast">
					<span class="bar-toast-cat">{clickedBar.category}</span>
					<span class="bar-toast-val">{clickedBar.value.toLocaleString()}</span>
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
						<label class="field-label" for="chart-dim">Dimension (X axis)</label>
						<select
							id="chart-dim"
							class="input input-mono"
							value={selectedDim}
							onchange={(e) => selectedDim = (e.target as HTMLSelectElement).value}
						>
							<option value="">Select dimension...</option>
							{#each dimColumns as col}
								<option value={col.name}>{col.name} ({col.type})</option>
							{/each}
						</select>
						<span class="field-hint">Pick a categorical column for grouping</span>
					</div>

					<div class="field">
						<label class="field-label" for="chart-metric">Metric (Y axis)</label>
						<select
							id="chart-metric"
							class="input input-mono"
							value={selectedMetric}
							onchange={(e) => selectedMetric = (e.target as HTMLSelectElement).value}
						>
							<option value="">Select metric...</option>
							{#each metricColumns as col}
								<option value={col.name}>{col.name} ({col.type})</option>
							{/each}
						</select>
						<span class="field-hint">Pick a numeric column for aggregation</span>
					</div>
				{/if}
			</div>

			<hr class="config-divider" />

			<div class="config-section">
				<h3 class="config-section-title">Aggregation</h3>

				<div class="aggregate-options">
					{#each AGGREGATES as agg}
						<button
							class="agg-btn"
							class:agg-btn-active={selectedAggregate === agg}
							onclick={() => selectedAggregate = agg}
						>
							{agg}
						</button>
					{/each}
				</div>
			</div>

			<hr class="config-divider" />

			<div class="config-section">
				<h3 class="config-section-title">Layout</h3>

				<div class="field">
					<label class="field-label" for="chart-orient">Orientation</label>
					<div class="orient-options">
						<button
							class="orient-btn"
							class:orient-btn-active={orientation === 'vertical'}
							onclick={() => orientation = 'vertical'}
						>
							Vertical
						</button>
						<button
							class="orient-btn"
							class:orient-btn-active={orientation === 'horizontal'}
							onclick={() => orientation = 'horizontal'}
						>
							Horizontal
						</button>
					</div>
				</div>
			</div>

			<hr class="config-divider" />

			<div class="config-section">
				<h3 class="config-section-title">Options</h3>

				<div class="field">
					<label class="field-label" for="chart-sort">Sort</label>
					<select
						id="chart-sort"
						class="input input-mono"
						value={sortDirection}
						onchange={(e) => sortDirection = (e.target as HTMLSelectElement).value as 'ASC' | 'DESC'}
					>
						<option value="DESC">Descending</option>
						<option value="ASC">Ascending</option>
					</select>
				</div>

				<div class="field">
					<label class="field-label" for="chart-limit">Max categories</label>
					<input
						id="chart-limit"
						type="number"
						class="input input-mono"
						min="1"
						max="1000"
						bind:value={limit}
					/>
					<span class="field-hint">Limits rows returned (default 30)</span>
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
						<span>Click to filter</span>
						<span class="toggle-hint">Click bars to highlight categories</span>
					</div>
					<button
						class="toggle"
						class:toggle-on={clickToFilter}
						onclick={() => clickToFilter = !clickToFilter}
						role="switch"
						aria-checked={clickToFilter}
					>
						<span class="toggle-knob"></span>
					</button>
				</label>

				<label class="toggle-row">
					<div class="toggle-label">
						<span>Show values</span>
						<span class="toggle-hint">Display value labels on bars</span>
					</div>
					<button
						class="toggle"
						class:toggle-on={showValues}
						onclick={() => showValues = !showValues}
						role="switch"
						aria-checked={showValues}
					>
						<span class="toggle-knob"></span>
					</button>
				</label>

				{#if showValues}
					<div class="field">
						<label class="field-label" for="chart-format">Number format</label>
						<div class="format-options">
							{#each ['number', 'currency', 'percent', 'compact'] as fmt}
								<button
									class="agg-btn"
									class:agg-btn-active={valueFormat === fmt}
									onclick={() => valueFormat = fmt as typeof valueFormat}
								>
									{fmt}
								</button>
							{/each}
						</div>
					</div>
				{/if}
			</div>

			<hr class="config-divider" />

			{#if config}
				<div class="config-section">
					<h3 class="config-section-title">Generated SQL</h3>
					<pre class="sql-preview"><code>SELECT "{selectedDim}", {selectedAggregate}("{selectedMetric}") as value
FROM "{selectedTable}"
GROUP BY "{selectedDim}"
ORDER BY value {sortDirection}
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

	.bar-toast-cat {
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

	.aggregate-options {
		display: flex;
		gap: var(--space-2);
	}

	.agg-btn {
		padding: var(--space-2) var(--space-3);
		border: 1px solid var(--color-border-strong);
		background: var(--color-surface);
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-text-tertiary);
		border-radius: var(--radius-xs);
		cursor: pointer;
		transition: all var(--duration-fast) ease;
	}

	.agg-btn:hover {
		border-color: var(--color-accent);
		color: var(--color-accent);
	}

	.agg-btn-active {
		background: var(--color-accent);
		border-color: var(--color-accent);
		color: var(--color-text-on-accent);
	}

	.agg-btn-active:hover {
		background: var(--color-accent-dark);
		border-color: var(--color-accent-dark);
		color: var(--color-text-on-accent);
	}

	.orient-options {
		display: flex;
		gap: var(--space-2);
	}

	.format-options {
		display: flex;
		gap: var(--space-2);
		flex-wrap: wrap;
	}

	.orient-btn {
		padding: var(--space-2) var(--space-3);
		border: 1px solid var(--color-border-strong);
		background: var(--color-surface);
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-text-tertiary);
		border-radius: var(--radius-xs);
		cursor: pointer;
		transition: all var(--duration-fast) ease;
	}

	.orient-btn:hover {
		border-color: var(--color-accent);
		color: var(--color-accent);
	}

	.orient-btn-active {
		background: var(--color-accent);
		border-color: var(--color-accent);
		color: var(--color-text-on-accent);
	}

	.orient-btn-active:hover {
		background: var(--color-accent-dark);
		border-color: var(--color-accent-dark);
		color: var(--color-text-on-accent);
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
