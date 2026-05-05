<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { Database, Table, RefreshCw, Trash2 } from 'lucide-svelte';
	import Button from '$lib/components/Button.svelte';
	import { getDbContext } from '$lib/db-context';

	// Get database context from layout
	const dbContext = getDbContext();

	interface AttachedTable {
		table_id: string;
		connection_id: string;
		schema_name: string;
		table_name: string;
		full_name: string;
		access_mode: string;
		local_table_name: string;
		cached_at: string | null;
		cache_row_count: number | null;
	}

	interface Props {
		onTableDeleted?: () => void;
	}

	let { onTableDeleted }: Props = $props();

	let attachedTables = $state<AttachedTable[]>([]);
	let isLoading = $state(false);

	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}

	async function loadAttachedTables() {
		if (!dbContext.isInitialized) {
			console.log('Skipping loadAttachedTables - DB not initialized yet');
			return;
		}
		
		try {
			isLoading = true;
			const result = await invoke<string>('get_attached_tables');
			attachedTables = JSON.parse(result);
		} catch (error) {
			console.error('Failed to load attached tables:', error);
			addSystemMessage(`Error loading attached tables: ${error}`, 'error');
		} finally {
			isLoading = false;
		}
	}

	async function deleteTable(tableName: string) {
		if (!confirm(`Are you sure you want to drop table "${tableName}"?`)) {
			return;
		}

		try {
			const result = await invoke<string>('debug_delete_table', { tableName });
			addSystemMessage(result, 'success');
			await loadAttachedTables();
			
			if (onTableDeleted) {
				onTableDeleted();
			}
		} catch (error) {
			addSystemMessage(`Error deleting table: ${error}`, 'error');
		}
	}

	// Load when DB is initialized
	$effect(() => {
		if (dbContext.isInitialized) {
			loadAttachedTables();
		}
	});
</script>

<div class="bg-white dark:bg-slate-900 rounded-lg border border-slate-200 dark:border-slate-700 overflow-hidden">
	<!-- Header -->
	<div class="p-6 border-b border-slate-200 dark:border-slate-700">
		<div class="flex items-center justify-between">
			<div>
				<h2 class="font-aspekta text-xl font-[650] text-slate-900 dark:text-slate-100 flex items-center gap-2">
					<Database class="w-5 h-5 text-blue-500" />
					Database Connections
				</h2>
				<p class="text-sm text-slate-600 dark:text-slate-400 mt-1">
					Tables imported from connected databases
				</p>
			</div>
			
			<Button
				variant="ghost"
				size="sm"
				onclick={loadAttachedTables}
				disabled={isLoading}
				title="Refresh list"
			>
				{#if isLoading}
					<svg class="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
						<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
					</svg>
				{:else}
					<RefreshCw class="w-4 h-4" />
				{/if}
			</Button>
		</div>
	</div>
	
	<!-- Table -->
	<div class="overflow-x-auto">
		<table class="w-full">
			<thead class="bg-slate-50 dark:bg-slate-800/50">
				<tr>
					<th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider">
						Local Table
					</th>
					<th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider">
						Source
					</th>
					<th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider">
						Access Mode
					</th>
					<th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider">
						Rows
					</th>
					<th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider">
						Cached At
					</th>
					<th class="px-6 py-3 text-right text-xs font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider">
						Actions
					</th>
				</tr>
			</thead>
			<tbody class="bg-white dark:bg-slate-900 divide-y divide-slate-200 dark:divide-slate-700">
				{#if isLoading}
					<tr>
						<td colspan="6" class="px-6 py-12 text-center text-slate-500 dark:text-slate-400">
							<svg class="animate-spin h-8 w-8 mx-auto mb-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
							</svg>
							Loading attached tables...
						</td>
					</tr>
				{:else if attachedTables.length === 0}
					<tr>
						<td colspan="6" class="px-6 py-12 text-center text-slate-500 dark:text-slate-400">
							<Database class="w-12 h-12 mx-auto mb-2 opacity-50" />
							<p class="font-medium">No database tables attached</p>
							<p class="text-sm mt-1">Import tables from the Connect page</p>
						</td>
					</tr>
				{:else}
					{#each attachedTables as table}
						<tr class="hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors">
							<td class="px-6 py-4 whitespace-nowrap">
								<div class="flex items-center gap-2">
									<Table class="w-4 h-4 text-blue-500" />
									<a
										href="/table-view?filename={encodeURIComponent(table.local_table_name + '.db')}&folder=database"
										class="text-sm font-medium text-blue-600 dark:text-blue-400 hover:underline"
									>
										{table.local_table_name}
									</a>
								</div>
							</td>
							<td class="px-6 py-4">
								<div class="text-sm">
									<div class="font-medium text-slate-900 dark:text-slate-100">{table.full_name}</div>
									<div class="text-slate-500 dark:text-slate-400 text-xs">{table.schema_name}.{table.table_name}</div>
								</div>
							</td>
							<td class="px-6 py-4 whitespace-nowrap">
								<span class="text-xs font-semibold px-2 py-1 rounded {
									table.access_mode === 'live' ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400' :
									table.access_mode === 'cached' ? 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400' :
									'bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-400'
								}">
									{table.access_mode.toUpperCase()}
								</span>
							</td>
							<td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 dark:text-slate-400">
								{table.cache_row_count?.toLocaleString() ?? 'N/A'}
							</td>
							<td class="px-6 py-4 whitespace-nowrap text-sm text-slate-600 dark:text-slate-400">
								{table.cached_at ?? 'Never'}
							</td>
							<td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
								<Button
									variant="ghost"
									size="sm"
									onclick={() => deleteTable(table.local_table_name)}
									title="Drop table"
								>
									<Trash2 class="w-4 h-4 text-red-600 dark:text-red-400" />
								</Button>
							</td>
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
	</div>
</div>

