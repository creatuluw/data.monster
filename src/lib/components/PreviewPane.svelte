<script lang="ts">
	import type { PreviewData, ColumnOverride } from '$lib/db-operations';
	import Pagination from './Pagination.svelte';

	let {
		data,
		columnOverrides = $bindable([]),
		onnext
	}: {
		data: PreviewData;
		columnOverrides: ColumnOverride[];
		onnext: () => void;
	} = $props();

	const TYPE_OPTIONS = ['VARCHAR', 'INTEGER', 'BIGINT', 'DOUBLE', 'BOOLEAN', 'DATE', 'TIMESTAMP'];
	const PAGE_SIZE = 10;

	let currentPage = $state(1);

	let totalPages = $derived(Math.ceil(Math.min(data.rows.length, 100) / PAGE_SIZE));
	let pageRows = $derived(
		data.rows.slice(0, 100).slice((currentPage - 1) * PAGE_SIZE, currentPage * PAGE_SIZE)
	);

	function formatCell(value: unknown): string {
		if (value === null || value === undefined) return '\u2014';
		if (typeof value === 'object') return JSON.stringify(value);
		return String(value);
	}

	function isEnabled(colName: string): boolean {
		return columnOverrides.find((c) => c.originalName === colName)?.enabled ?? true;
	}

	function toggleEnabled(colName: string) {
		const existing = columnOverrides.find((c) => c.originalName === colName);
		if (existing) {
			existing.enabled = !existing.enabled;
		}
	}

	function getOverrideType(colName: string): string {
		const o = columnOverrides.find((c) => c.originalName === colName);
		if (!o) return '';
		return o.overrideType ?? o.detectedType;
	}

	function setOverrideType(colName: string, type: string) {
		const detected = data.detectedTypes.find((c) => c.name === colName);
		const existing = columnOverrides.find((c) => c.originalName === colName);
		if (existing) {
			existing.overrideType = type === detected?.type ? null : type;
		}
	}

	function getNewName(colName: string): string {
		const o = columnOverrides.find((c) => c.originalName === colName);
		return o?.newName ?? colName;
	}

	function setNewName(colName: string, newName: string) {
		const existing = columnOverrides.find((c) => c.originalName === colName);
		if (existing) {
			existing.newName = newName;
		}
	}

	function isTypeChanged(colName: string): boolean {
		const o = columnOverrides.find((c) => c.originalName === colName);
		const detected = data.detectedTypes.find((c) => c.name === colName);
		return !!(o?.overrideType && o.overrideType !== detected?.type);
	}
</script>

<div class="preview">
	<div class="preview-table-wrap">
		<table class="data-table">
			<thead>
				<tr class="type-row">
					{#each data.columns as col}
						{@const on = isEnabled(col)}
						<th class="type-cell">
							<select
								value={getOverrideType(col)}
								onchange={(e) => setOverrideType(col, (e.target as HTMLSelectElement).value)}
								disabled={!on}
								class="type-select {isTypeChanged(col) ? 'is-changed' : ''} {!on ? 'is-disabled' : ''}"
							>
								{#each TYPE_OPTIONS as opt}
									<option value={opt} selected={getOverrideType(col) === opt}>{opt}</option>
								{/each}
							</select>
						</th>
					{/each}
				</tr>
				<tr class="name-row">
					{#each data.columns as col}
						{@const on = isEnabled(col)}
						<th class="name-cell">
							<div class="name-cell-inner">
								<input
									type="text"
									value={getNewName(col)}
									onchange={(e) => setNewName(col, (e.target as HTMLInputElement).value)}
									disabled={!on}
									class="name-input {!on ? 'is-disabled' : ''}"
								/>
								<button
									onclick={() => toggleEnabled(col)}
									class="toggle-label {on ? 'is-on' : 'is-off'}"
									title={on ? 'Click to exclude column' : 'Click to include column'}
								>
									{on ? 'on' : 'off'}
								</button>
							</div>
						</th>
					{/each}
				</tr>
			</thead>
			<tbody>
				{#each pageRows as row}
					<tr>
						{#each data.columns as col}
							{@const on = isEnabled(col)}
							<td class={!on ? 'is-disabled' : ''}>{formatCell(row[col])}</td>
						{/each}
					</tr>
				{/each}
			</tbody>
		</table>
	</div>

	<div class="preview-bottom">
		<Pagination
			{currentPage}
			totalItems={Math.min(data.rows.length, 100)}
			perPage={PAGE_SIZE}
			onchange={(p) => currentPage = p}
		/>
		<button onclick={onnext} class="btn btn-primary preview-next-btn">
			Next &rarr;
		</button>
	</div>
</div>

<style>
	.preview {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.preview-table-wrap {
		overflow-x: auto;
		border: 1px solid var(--color-border);
		background: var(--color-surface);
	}

	.type-row,
	.name-row {
		background: var(--color-surface-raised);
	}

	.type-row {
		border-bottom: 1px solid var(--color-border);
	}

	.name-row {
		border-bottom: 1px solid var(--color-border-strong);
	}

	.type-cell,
	.name-cell {
		white-space: nowrap;
		padding: var(--space-2) var(--space-4);
		border-right: 1px solid color-mix(in oklch, var(--color-border) 40%, transparent);
	}

	.type-cell:last-child,
	.name-cell:last-child {
		border-right: none;
	}

	.toggle-label {
		font-family: var(--font-mono);
		font-size: 8px;
		letter-spacing: 0.06em;
		background: none;
		border: none;
		cursor: pointer;
		flex-shrink: 0;
		width: 22px;
		text-align: right;
		transition: color var(--duration-fast) ease;
	}

	.toggle-label.is-on {
		color: var(--color-text-tertiary);
	}

	.toggle-label.is-off {
		color: var(--color-border);
		text-decoration: line-through;
	}

	.type-select {
		padding: 1px var(--space-1);
		border: 1px solid var(--color-border);
		background: var(--color-surface);
		border-radius: var(--radius-xs);
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-secondary);
		transition: border-color var(--duration-fast) ease;
	}

	.type-select.is-changed {
		border-color: var(--color-accent);
		color: var(--color-accent-dark);
	}

	.type-select.is-disabled {
		opacity: 0.3;
	}

	.name-cell-inner {
		display: flex;
		align-items: baseline;
		gap: var(--space-1);
	}

	.name-input {
		border: none;
		background: transparent;
		font-family: var(--font-body);
		font-weight: 600;
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		outline: none;
		min-width: 60px;
		flex: 1;
	}

	.name-input.is-disabled {
		opacity: 0.3;
		text-decoration: line-through;
	}

	.is-disabled {
		opacity: 0.3;
	}

	.data-table td {
		border-right: 1px solid color-mix(in oklch, var(--color-border) 40%, transparent);
		padding: var(--space-2) var(--space-4);
		white-space: nowrap;
	}

	.data-table td:last-child {
		border-right: none;
	}

	.preview-bottom {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: var(--space-4);
	}

	.preview-next-btn {
		height: 32px;
		flex-shrink: 0;
	}
</style>
