<script lang="ts">
	import type { PreviewData, ColumnOverride } from '$lib/db-helpers';

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
		if (value === null || value === undefined) return '—';
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

<div class="flex flex-col gap-6">
	<!-- Header -->
	<div class="flex items-center gap-3">
		<h2 class="font-display text-lg font-bold text-sand-800">{data.sourceName ?? 'Preview'}</h2>
		<span class="rounded-full bg-sage-100 px-2 py-0.5 text-[11px] font-medium text-sage-700">
			{data.totalRows.toLocaleString()} rows
		</span>
	</div>

	<!-- Preview table with inline type selectors and enable toggles -->
	<div class="overflow-x-auto rounded-lg border border-sand-200 bg-white">
		<table class="w-full text-left text-sm">
			<thead>
				<!-- Enable toggle row -->
				<tr class="border-b border-sand-100 bg-sand-50">
					{#each data.columns as col}
						{@const on = isEnabled(col)}
						<th class="whitespace-nowrap px-4 pt-3 pb-1">
							<button
								onclick={() => toggleEnabled(col)}
								class="flex items-center gap-1 text-[11px] transition-colors {on ? 'text-sage-600' : 'text-sand-300 line-through'}"
								title={on ? 'Click to exclude column' : 'Click to include column'}
							>
								<svg class="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
									{#if on}
										<path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
									{:else}
										<path stroke-linecap="round" stroke-linejoin="round" d="m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z" />
									{/if}
								</svg>
								{on ? 'on' : 'off'}
							</button>
						</th>
					{/each}
				</tr>
				<!-- Type selector row -->
				<tr class="border-b border-sand-100 bg-sand-50">
					{#each data.columns as col}
						{@const on = isEnabled(col)}
						<th class="whitespace-nowrap px-4 pb-1">
							<select
								value={getOverrideType(col)}
								onchange={(e) => setOverrideType(col, (e.target as HTMLSelectElement).value)}
								disabled={!on}
								class="rounded border border-sand-200 bg-white px-1.5 py-0.5 text-[11px] text-sand-600 shadow-sm focus:border-sage-400 focus:outline-none focus:ring-1 focus:ring-sage-200 {isTypeChanged(col) ? 'border-copper-300 text-copper-700' : ''} {!on ? 'opacity-30' : ''}"
							>
								{#each TYPE_OPTIONS as opt}
									<option value={opt} selected={getOverrideType(col) === opt}>{opt}</option>
								{/each}
							</select>
						</th>
					{/each}
				</tr>
				<!-- Column name row (editable) -->
				<tr class="border-b border-sand-200 bg-sand-50">
					{#each data.columns as col}
						{@const on = isEnabled(col)}
						<th class="whitespace-nowrap px-4 pb-3">
							<input
								type="text"
								value={getNewName(col)}
								onchange={(e) => setNewName(col, (e.target as HTMLInputElement).value)}
								disabled={!on}
								class="w-full min-w-[60px] bg-transparent font-semibold text-sand-600 outline-none focus:text-sand-800 {!on ? 'opacity-30 line-through' : ''}"
							/>
						</th>
					{/each}
				</tr>
			</thead>
			<tbody>
				{#each data.rows as row}
					<tr class="border-b border-sand-100 transition-colors hover:bg-sage-50/40">
						{#each data.columns as col}
							{@const on = isEnabled(col)}
							<td class="max-w-[300px] truncate px-4 py-2.5 text-sand-700 {!on ? 'opacity-30' : ''}">{formatCell(row[col])}</td>
						{/each}
					</tr>
				{/each}
			</tbody>
		</table>
	</div>

	<!-- Next step -->
	<div class="flex items-center justify-end">
		<button
			onclick={onnext}
			class="rounded-lg bg-copper-400 px-4 py-2 text-sm font-medium text-white shadow-sm transition-all hover:bg-copper-500"
		>
			Write table query →
		</button>
	</div>
</div>
