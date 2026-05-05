<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { Database, Table2, ChevronRight, ChevronDown, Loader2, AlertCircle, Search } from 'lucide-svelte';

	interface PostgresTable {
		table_name: string;
		schema_name: string;
		row_count: number | null;
	}

	interface Props {
		connectionId: string;
		onSelectTable?: (schema: string, table: string) => void;
	}

	let { connectionId, onSelectTable }: Props = $props();

	let schemas = $state<string[]>([]);
	let expandedSchemas = $state<Set<string>>(new Set());
	let schemaTables = $state<Map<string, PostgresTable[]>>(new Map());
	let loading = $state(true);
	let loadingTables = $state<Set<string>>(new Set());
	let error = $state('');
	let searchQuery = $state('');

	// Filtered schemas based on search
	let filteredSchemas = $derived(
		searchQuery.trim()
			? schemas.filter(s => s.toLowerCase().includes(searchQuery.toLowerCase()))
			: schemas
	);

	onMount(() => {
		loadSchemas();
	});

	async function loadSchemas() {
		loading = true;
		error = '';

		try {
			// Tauri automatically converts camelCase to snake_case
			const result = await invoke<string>('browse_postgres_schemas', { connectionId });
			schemas = JSON.parse(result);
		} catch (err) {
			error = String(err);
			schemas = [];
		} finally {
			loading = false;
		}
	}

	async function toggleSchema(schema: string) {
		if (expandedSchemas.has(schema)) {
			expandedSchemas.delete(schema);
			expandedSchemas = new Set(expandedSchemas);
		} else {
			expandedSchemas.add(schema);
			expandedSchemas = new Set(expandedSchemas);

			// Load tables if not already loaded
			if (!schemaTables.has(schema)) {
				await loadTables(schema);
			}
		}
	}

	async function loadTables(schema: string) {
		loadingTables.add(schema);
		loadingTables = new Set(loadingTables);
		error = '';

		try {
			// Tauri automatically converts camelCase to snake_case
			const result = await invoke<string>('browse_postgres_tables', {
				connectionId,
				schemaName: schema
			});
			const tables: PostgresTable[] = JSON.parse(result);
			schemaTables.set(schema, tables);
			schemaTables = new Map(schemaTables);
		} catch (err) {
			error = String(err);
		} finally {
			loadingTables.delete(schema);
			loadingTables = new Set(loadingTables);
		}
	}

	function getFilteredTables(schema: string): PostgresTable[] {
		const tables = schemaTables.get(schema) || [];
		if (!searchQuery.trim()) return tables;

		return tables.filter(t =>
			t.table_name.toLowerCase().includes(searchQuery.toLowerCase())
		);
	}
</script>

<div class="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg">
	<!-- Header -->
	<div class="p-4 border-b border-slate-200 dark:border-slate-700">
		<div class="flex items-center gap-3 mb-4">
			<div class="p-2 bg-blue-100 dark:bg-blue-900/30 rounded-lg">
				<Database class="w-5 h-5 text-blue-600 dark:text-blue-400" />
			</div>
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100">
				Database Browser
			</h3>
		</div>

		<!-- Search -->
		<div class="relative">
			<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400" />
			<input
				type="text"
				bind:value={searchQuery}
				placeholder="Search schemas and tables..."
				class="w-full pl-10 pr-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent"
			/>
		</div>
	</div>

	<!-- Error Message -->
	{#if error}
		<div class="p-4 border-b border-slate-200 dark:border-slate-700">
			<div class="flex items-start gap-3 p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
				<AlertCircle class="w-5 h-5 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5" />
				<p class="text-sm text-red-800 dark:text-red-300">{error}</p>
			</div>
		</div>
	{/if}

	<!-- Loading State -->
	{#if loading}
		<div class="flex items-center justify-center py-12">
			<Loader2 class="w-8 h-8 animate-spin text-blue-600" />
		</div>
	{:else if filteredSchemas.length === 0}
		<!-- Empty State -->
		<div class="text-center py-12 px-4">
			<div class="p-4 bg-slate-100 dark:bg-slate-700/50 rounded-full w-16 h-16 mx-auto mb-4 flex items-center justify-center">
				<Database class="w-8 h-8 text-slate-400" />
			</div>
			<h4 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
				{searchQuery ? 'No Matching Schemas' : 'No Schemas Found'}
			</h4>
			<p class="text-slate-600 dark:text-slate-400">
				{searchQuery ? 'Try a different search term.' : 'This database has no user schemas.'}
			</p>
		</div>
	{:else}
		<!-- Schemas List -->
		<div class="max-h-96 overflow-y-auto">
			{#each filteredSchemas as schema (schema)}
				<div class="border-b border-slate-200 dark:border-slate-700 last:border-b-0">
					<!-- Schema Header -->
					<button
						onclick={() => toggleSchema(schema)}
						class="w-full flex items-center gap-3 p-4 hover:bg-slate-50 dark:hover:bg-slate-700/50 transition-colors text-left"
					>
						{#if expandedSchemas.has(schema)}
							<ChevronDown class="w-4 h-4 text-slate-500 flex-shrink-0" />
						{:else}
							<ChevronRight class="w-4 h-4 text-slate-500 flex-shrink-0" />
						{/if}
						<Database class="w-4 h-4 text-blue-600 dark:text-blue-400 flex-shrink-0" />
						<span class="font-medium text-slate-900 dark:text-slate-100">{schema}</span>
						{#if schemaTables.has(schema)}
							<span class="ml-auto text-sm text-slate-500 dark:text-slate-400">
								{schemaTables.get(schema)?.length || 0} tables
							</span>
						{/if}
					</button>

					<!-- Tables List -->
					{#if expandedSchemas.has(schema)}
						<div class="bg-slate-50 dark:bg-slate-900/50">
							{#if loadingTables.has(schema)}
								<div class="flex items-center justify-center py-8">
									<Loader2 class="w-6 h-6 animate-spin text-blue-600" />
								</div>
							{:else}
								{@const tables = getFilteredTables(schema)}
								{#if tables.length === 0}
									<div class="px-4 py-6 text-center text-sm text-slate-500 dark:text-slate-400">
										{searchQuery ? 'No matching tables in this schema' : 'No tables in this schema'}
									</div>
								{:else}
									{#each tables as table (table.table_name)}
										<button
											onclick={() => onSelectTable?.(schema, table.table_name)}
											class="w-full flex items-center gap-3 px-4 py-3 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors text-left border-t border-slate-200 dark:border-slate-700"
										>
											<div class="w-8"></div> <!-- Spacer for alignment -->
											<Table2 class="w-4 h-4 text-slate-600 dark:text-slate-400 flex-shrink-0" />
											<span class="text-sm text-slate-900 dark:text-slate-100">{table.table_name}</span>
											{#if table.row_count !== null}
												<span class="ml-auto text-xs text-slate-500 dark:text-slate-400">
													{table.row_count.toLocaleString()} rows
												</span>
											{/if}
										</button>
									{/each}
								{/if}
							{/if}
						</div>
					{/if}
				</div>
			{/each}
		</div>
	{/if}

	<!-- Footer Info -->
	<div class="p-4 bg-slate-50 dark:bg-slate-900/50 border-t border-slate-200 dark:border-slate-700 text-sm text-slate-600 dark:text-slate-400">
		{#if loading}
			Loading schemas...
		{:else}
			{filteredSchemas.length} schema{filteredSchemas.length !== 1 ? 's' : ''} found
		{/if}
	</div>
</div>

