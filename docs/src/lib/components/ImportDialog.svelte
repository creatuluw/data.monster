<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { Download, Loader2, Database, AlertCircle, CheckCircle2, Zap, HardDrive, Cloud } from 'lucide-svelte';

	interface Props {
		connectionId: string;
		schemaName: string;
		tableName: string;
		onSuccess?: () => void;
		onCancel?: () => void;
	}

	let { connectionId, schemaName, tableName, onSuccess, onCancel }: Props = $props();

	// Form state
	let localTableName = $state(tableName.replace(/[^a-zA-Z0-9_]/g, '_'));
	let accessMode = $state<'live' | 'cached' | 'imported'>('cached');
	let importing = $state(false);
	let error = $state('');
	let success = $state(false);
	let importResult = $state('');

	// Access mode descriptions
	const accessModes = [
		{
			value: 'live',
			label: 'Live Mode',
			icon: Zap,
			description: 'Query PostgreSQL directly without importing',
			details: 'Always fresh data, zero local storage, best for small tables',
			color: 'yellow'
		},
		{
			value: 'cached',
			label: 'Cached Mode',
			icon: HardDrive,
			description: 'Import data, keep connection for easy refresh',
			details: 'Fast queries, periodically refreshable, best for medium tables',
			color: 'blue'
		},
		{
			value: 'imported',
			label: 'Imported Mode',
			icon: Database,
			description: 'One-time import, disconnect after',
			details: 'Fastest queries, manual refresh, best for large static tables',
			color: 'green'
		}
	] as const;

	async function performImport() {
		if (!localTableName.trim()) {
			error = 'Table name is required';
			return;
		}

		importing = true;
		error = '';
		success = false;

		try {
			// Tauri automatically converts camelCase to snake_case
			const result = await invoke<string>('import_postgres_table', {
				connectionId,
				schemaName,
				tableName,
				localTableName: localTableName.trim(),
				accessMode
			});

			importResult = result;
			success = true;

			// Call onSuccess after a brief delay to show success message
			setTimeout(() => {
				if (onSuccess) onSuccess();
			}, 1500);
		} catch (err) {
			error = String(err);
		} finally {
			importing = false;
		}
	}

	function getColorClasses(color: string, selected: boolean) {
		const baseClasses = selected ? 'ring-2' : 'hover:border-slate-400 dark:hover:border-slate-500';
		
		switch (color) {
			case 'yellow':
				return selected
					? `${baseClasses} ring-yellow-500 border-yellow-500 bg-yellow-50 dark:bg-yellow-900/20`
					: `${baseClasses} border-slate-300 dark:border-slate-600`;
			case 'blue':
				return selected
					? `${baseClasses} ring-blue-500 border-blue-500 bg-blue-50 dark:bg-blue-900/20`
					: `${baseClasses} border-slate-300 dark:border-slate-600`;
			case 'green':
				return selected
					? `${baseClasses} ring-green-500 border-green-500 bg-green-50 dark:bg-green-900/20`
					: `${baseClasses} border-slate-300 dark:border-slate-600`;
			default:
				return baseClasses;
		}
	}
</script>

<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
	<div class="bg-white dark:bg-slate-800 rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
		<!-- Header -->
		<div class="p-6 border-b border-slate-200 dark:border-slate-700">
			<div class="flex items-center gap-3 mb-2">
				<div class="p-2 bg-blue-100 dark:bg-blue-900/30 rounded-lg">
					<Download class="w-5 h-5 text-blue-600 dark:text-blue-400" />
				</div>
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">
					Import Table
				</h2>
			</div>
			<p class="text-sm text-slate-600 dark:text-slate-400">
				{schemaName}.{tableName}
			</p>
		</div>

		<!-- Content -->
		<div class="p-6 space-y-6">
			<!-- Local Table Name -->
			<div>
				<label for="local-table-name" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
					Local Table Name
				</label>
				<input
					id="local-table-name"
					type="text"
					bind:value={localTableName}
					placeholder="my_table"
					class="form-input w-full px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
					disabled={importing || success}
					required
				/>
				<p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
					Name for the table in your local DuckDB database
				</p>
			</div>

			<!-- Access Mode Selection -->
			<div>
				<div class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-3">
					Access Mode
				</div>
				<div class="space-y-3" role="group" aria-label="Access mode selection">
					{#each accessModes as mode}
						<button
							type="button"
							onclick={() => accessMode = mode.value}
							disabled={importing || success}
							class="w-full text-left p-4 border rounded-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed {getColorClasses(mode.color, accessMode === mode.value)}"
						>
							<div class="flex items-start gap-3">
								<div class="p-2 bg-{mode.color}-100 dark:bg-{mode.color}-900/30 rounded-lg flex-shrink-0">
									{#if mode.value === 'live'}
										<Zap class="w-5 h-5 text-{mode.color}-600 dark:text-{mode.color}-400" />
									{:else if mode.value === 'cached'}
										<HardDrive class="w-5 h-5 text-{mode.color}-600 dark:text-{mode.color}-400" />
									{:else}
										<Database class="w-5 h-5 text-{mode.color}-600 dark:text-{mode.color}-400" />
									{/if}
								</div>
								<div class="flex-1 min-w-0">
									<div class="flex items-center gap-2 mb-1">
										<h4 class="font-semibold text-slate-900 dark:text-slate-100">
											{mode.label}
										</h4>
										{#if accessMode === mode.value}
											<CheckCircle2 class="w-4 h-4 text-{mode.color}-600 dark:text-{mode.color}-400" />
										{/if}
									</div>
									<p class="text-sm text-slate-700 dark:text-slate-300 mb-1">
										{mode.description}
									</p>
									<p class="text-xs text-slate-500 dark:text-slate-400">
										{mode.details}
									</p>
								</div>
							</div>
						</button>
					{/each}
				</div>
			</div>

			<!-- Success Message -->
			{#if success}
				<div class="flex items-start gap-3 p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg">
					<CheckCircle2 class="w-5 h-5 text-green-600 dark:text-green-400 flex-shrink-0 mt-0.5" />
					<div>
						<p class="font-medium text-green-900 dark:text-green-300 mb-1">Import Successful!</p>
						<p class="text-sm text-green-800 dark:text-green-400">{importResult}</p>
					</div>
				</div>
			{/if}

			<!-- Error Message -->
			{#if error}
				<div class="flex items-start gap-3 p-4 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
					<AlertCircle class="w-5 h-5 text-red-600 dark:text-red-400 flex-shrink-0 mt-0.5" />
					<p class="text-sm text-red-800 dark:text-red-300">{error}</p>
				</div>
			{/if}
		</div>

		<!-- Footer -->
		<div class="p-6 bg-slate-50 dark:bg-slate-900/50 border-t border-slate-200 dark:border-slate-700 flex gap-3 justify-end">
			{#if onCancel}
				<button
					onclick={onCancel}
					disabled={importing}
					class="px-4 py-2 border border-slate-300 dark:border-slate-600 rounded-lg text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{success ? 'Close' : 'Cancel'}
				</button>
			{/if}

			{#if !success}
				<button
					onclick={performImport}
					disabled={importing}
					class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
				>
					{#if importing}
						<Loader2 class="w-4 h-4 animate-spin" />
						Importing...
					{:else}
						<Download class="w-4 h-4" />
						Import Table
					{/if}
				</button>
			{/if}
		</div>
	</div>
</div>

