<script lang="ts">
	import type { PreviewData, ColumnOverride } from '$lib/db-operations';

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
	<div class="preview-header">
		<h2 class="preview-title">{data.sourceName ?? 'Preview'}</h2>
		<span class="tag tag-accent">{data.totalRows.toLocaleString()} rows</span>
	</div>

	<div class="preview-table-wrap">
		<table class="data-table">
			<thead>
				<tr class="toggle-row">
					{#each data.columns as col}
						{@const on = isEnabled(col)}
						<th class="toggle-cell">
							<button
								onclick={() => toggleEnabled(col)}
								class="toggle-btn {on ? 'is-on' : 'is-off'}"
								title={on ? 'Click to exclude column' : 'Click to include column'}
							>
								{on ? 'on' : 'off'}
							</button>
						</th>
					{/each}
				</tr>
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
							<input
								type="text"
								value={getNewName(col)}
								onchange={(e) => setNewName(col, (e.target as HTMLInputElement).value)}
								disabled={!on}
								class="name-input {!on ? 'is-disabled' : ''}"
							/>
						</th>
					{/each}
				</tr>
			</thead>
			<tbody>
				{#each data.rows as row}
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

	<div class="preview-footer">
		<button onclick={onnext} class="btn btn-primary">
			Write table query &rarr;
		</button>
	</div>
</div>

<style>
	.preview {
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
	}

	.preview-header {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.preview-title {
		font-family: var(--font-display);
		font-size: var(--text-lg);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
	}

	.preview-table-wrap {
		overflow-x: auto;
		border: 1px solid var(--color-border);
		background: var(--color-surface);
	}

	.toggle-row,
	.type-row,
	.name-row {
		background: var(--color-surface-raised);
	}

	.toggle-row {
		border-bottom: 1px solid var(--color-border);
	}

	.type-row {
		border-bottom: 1px solid var(--color-border);
	}

	.name-row {
		border-bottom: 1px solid var(--color-border-strong);
	}

	.toggle-cell,
	.type-cell,
	.name-cell {
		white-space: nowrap;
		padding: var(--space-2) var(--space-4);
	}

	.toggle-btn {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.06em;
		background: none;
		border: none;
		cursor: pointer;
		transition: color var(--duration-fast) ease;
	}

	.toggle-btn.is-on {
		color: var(--color-accent);
	}

	.toggle-btn.is-off {
		color: var(--color-text-tertiary);
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

	.name-input {
		border: none;
		background: transparent;
		font-family: var(--font-body);
		font-weight: 600;
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		outline: none;
		min-width: 60px;
		width: 100%;
	}

	.name-input.is-disabled {
		opacity: 0.3;
		text-decoration: line-through;
	}

	.is-disabled {
		opacity: 0.3;
	}

	.preview-footer {
		display: flex;
		justify-content: flex-end;
	}
</style>
