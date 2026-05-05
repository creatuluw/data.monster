<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Button from '$lib/components/Button.svelte';
	import { FunctionSquare, Play, Save, Trash2, Plus, Edit, X, Check, Copy, AlertCircle, Info } from 'lucide-svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	
	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}
	
	// UDF Types
	interface UserDefinedFunction {
		function_name: string;
		parameters: string;
		return_type: string;
		function_body: string;
		description: string | null;
		created_at: string;
		updated_at: string;
	}
	
	// State
	let udfs = $state<UserDefinedFunction[]>([]);
	let isLoading = $state(false);
	let isTauriAvailable = $state(false);
	
	// Form state for creating/editing UDFs
	let showCreateModal = $state(false);
	let editingUdf = $state<string | null>(null);
	let functionName = $state('');
	let parameters = $state('');
	let returnType = $state('VARCHAR');
	let functionBody = $state('');
	let description = $state('');
	let isSaving = $state(false);
	
	// Test state
	let testingUdf = $state<string | null>(null);
	let testQuery = $state('');
	let testResults = $state<any[]>([]);
	let testColumns = $state<string[]>([]);
	let isTestingFunction = $state(false);
	let testError = $state('');
	
	// Delete confirmation
	let deletingUdf = $state<string | null>(null);
	
	// Info dialog
	let showInfoDialog = $state(false);
	let infoUdf = $state<UserDefinedFunction | null>(null);
	
	function openInfoDialog(udf: UserDefinedFunction) {
		infoUdf = udf;
		showInfoDialog = true;
	}
	
	function closeInfoDialog() {
		showInfoDialog = false;
		infoUdf = null;
	}
	
	async function loadUdfs() {
		try {
			isLoading = true;
			const result = await invoke<string>('list_udfs');
			udfs = JSON.parse(result);
		} catch (error) {
			console.error('Error loading UDFs:', error);
			addSystemMessage(`Error loading functions: ${error}`, 'error');
		} finally {
			isLoading = false;
		}
	}
	
	function openCreateModal() {
		showCreateModal = true;
		editingUdf = null;
		functionName = '';
		parameters = '';
		returnType = 'VARCHAR';
		functionBody = '';
		description = '';
	}
	
	function openEditModal(udf: UserDefinedFunction) {
		showCreateModal = true;
		editingUdf = udf.function_name;
		functionName = udf.function_name;
		parameters = udf.parameters;
		returnType = udf.return_type;
		functionBody = udf.function_body;
		description = udf.description || '';
	}
	
	function closeModal() {
		showCreateModal = false;
		editingUdf = null;
		functionName = '';
		parameters = '';
		returnType = 'VARCHAR';
		functionBody = '';
		description = '';
	}
	
	async function saveFunction() {
		if (!functionName.trim() || !functionBody.trim()) {
			addSystemMessage('Function name and body are required', 'warning');
			return;
		}
		
		try {
			isSaving = true;
			
			const params = {
				functionName: functionName.trim(),
				parameters: parameters.trim(),
				returnType: returnType.trim(),
				functionBody: functionBody.trim(),
				description: description.trim()
			};
			
			console.log('Saving UDF:', params);
			
			const result = await invoke<string>('create_udf', params);
			
			console.log('Save result:', result);
			addSystemMessage(result, 'success');
			closeModal();
			await loadUdfs();
		} catch (error) {
			console.error('Error saving function:', error);
			addSystemMessage(`Error saving function: ${error}`, 'error');
		} finally {
			isSaving = false;
		}
	}
	
	async function deleteFunction(functionName: string) {
		try {
			const result = await invoke<string>('delete_udf', { functionName });
			addSystemMessage(result, 'success');
			deletingUdf = null;
			await loadUdfs();
		} catch (error) {
			addSystemMessage(`Error deleting function: ${error}`, 'error');
		}
	}
	
	function openTestPanel(udf: UserDefinedFunction) {
		testingUdf = udf.function_name;
		testQuery = generateTestQuery(udf);
		testResults = [];
		testColumns = [];
		testError = '';
	}
	
	function generateTestQuery(udf: UserDefinedFunction): string {
		// Generate a sample test query based on the function signature
		const paramList = udf.parameters.split(',').map(p => p.trim());
		const sampleArgs = paramList.map((param, idx) => {
			if (param.toLowerCase().includes('varchar') || param.toLowerCase().includes('text')) {
				return "'sample_value'";
			} else if (param.toLowerCase().includes('int') || param.toLowerCase().includes('numeric')) {
				return '123';
			} else if (param.toLowerCase().includes('bool')) {
				return 'TRUE';
			} else {
				return `'arg${idx + 1}'`;
			}
		}).join(', ');
		
		return `SELECT ${udf.function_name}(${sampleArgs}) AS result;`;
	}
	
	async function testFunction() {
		if (!testQuery.trim()) {
			addSystemMessage('Please enter a test query', 'warning');
			return;
		}
		
		try {
			isTestingFunction = true;
			testError = '';
			testResults = [];
			testColumns = [];
			
			const result = await invoke<string>('test_udf', {
				functionName: testingUdf,
				testQuery: testQuery.trim()
			});
			
			const parsed = JSON.parse(result);
			
			if (parsed.length > 0) {
				testColumns = Object.keys(parsed[0]);
				testResults = parsed;
				addSystemMessage(`Test returned ${parsed.length} row(s)`, 'success');
			} else {
				testColumns = [];
				testResults = [];
				addSystemMessage('Test returned no results', 'info');
			}
		} catch (error) {
			testError = `${error}`;
			addSystemMessage(`Test error: ${error}`, 'error');
		} finally {
			isTestingFunction = false;
		}
	}
	
	function closeTestPanel() {
		testingUdf = null;
		testQuery = '';
		testResults = [];
		testColumns = [];
		testError = '';
	}
	
	function copyToClipboard(text: string) {
		navigator.clipboard.writeText(text);
		addSystemMessage('Copied to clipboard', 'success');
	}
	
	onMount(async () => {
		try {
			await invoke('initialize_duckdb');
			isTauriAvailable = true;
			addSystemMessage('Connected to database', 'success');
			await loadUdfs();
		} catch (error) {
			if (typeof window !== 'undefined' && !('__TAURI__' in window)) {
				isTauriAvailable = false;
				addSystemMessage('Tauri API not available. Please run: npm run tauri:dev', 'error');
			} else {
				isTauriAvailable = true;
				addSystemMessage('Database connection issue. Some features may not work.', 'warning');
				console.error('DuckDB initialization error:', error);
			}
		}
	});
</script>

<PageLayout>
	<div class="flex items-center justify-between mb-6">

		<Button
			variant="primary"
			onclick={openCreateModal}
			disabled={!isTauriAvailable}
		>
			<Plus class="w-4 h-4 mr-2" />
			New Function
		</Button>
	</div>
	
	{#if isLoading}
		<div class="flex items-center justify-center h-64">
			<div class="text-slate-500 dark:text-slate-400">Loading functions...</div>
		</div>
	{:else if udfs.length === 0}
		<div class="flex flex-col items-center justify-center h-64 text-center">
			<FunctionSquare class="w-16 h-16 text-slate-300 dark:text-slate-600 mb-4" />
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
				No functions yet
			</h3>
			<p class="text-slate-600 dark:text-slate-400 mb-6 max-w-md">
				Create your first user-defined function to encapsulate reusable SQL logic and simplify your queries.
			</p>
			<Button variant="primary" onclick={openCreateModal} disabled={!isTauriAvailable}>
				<Plus class="w-4 h-4 mr-2" />
				Create Function
			</Button>
		</div>
	{:else}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
				{#each udfs as udf}
					<div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg p-5 hover:shadow-md transition-shadow">
						<!-- Header -->
						<div class="flex items-center justify-between mb-3">
							<div class="flex items-center gap-2">
								<FunctionSquare class="w-5 h-5 text-indigo-500" />
								<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 font-mono">
									{udf.function_name}
								</h3>
							</div>
							<div class="flex items-center gap-1">
								<button
									type="button"
									onclick={() => openInfoDialog(udf)}
									class="p-2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 rounded transition-colors"
									title="View details"
								>
									<Info class="w-4 h-4" />
								</button>
								<button
									type="button"
									onclick={() => openTestPanel(udf)}
									class="p-2 text-slate-400 hover:text-indigo-500 hover:bg-slate-100 dark:hover:bg-slate-800 rounded transition-colors"
									title="Test function"
								>
									<Play class="w-4 h-4" />
								</button>
								<button
									type="button"
									onclick={() => openEditModal(udf)}
									class="p-2 text-slate-400 hover:text-blue-500 hover:bg-slate-100 dark:hover:bg-slate-800 rounded transition-colors"
									title="Edit function"
								>
									<Edit class="w-4 h-4" />
								</button>
								<button
									type="button"
									onclick={() => deletingUdf = udf.function_name}
									class="p-2 text-slate-400 hover:text-red-500 hover:bg-slate-100 dark:hover:bg-slate-800 rounded transition-colors"
									title="Delete function"
								>
									<Trash2 class="w-4 h-4" />
								</button>
							</div>
						</div>
						
						<!-- Description (Full Width) -->
						{#if udf.description}
							<p class="text-sm text-slate-600 dark:text-slate-400 mb-3">
								{udf.description}
							</p>
						{/if}
						
						<!-- Example Usage -->
						<div class="bg-slate-50 dark:bg-slate-950 rounded px-3 py-2 font-mono text-xs text-slate-600 dark:text-slate-400">
							{udf.function_name}({udf.parameters || '[field_name]'})
						</div>
						
						<!-- Delete Confirmation -->
						{#if deletingUdf === udf.function_name}
							<div class="mt-3 pt-3 border-t border-slate-200 dark:border-slate-800">
								<div class="flex items-center gap-2 text-sm">
									<AlertCircle class="w-4 h-4 text-red-500" />
									<span class="text-slate-700 dark:text-slate-300 flex-1">Delete this function?</span>
									<button
										type="button"
										onclick={() => deleteFunction(udf.function_name)}
										class="px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600 transition-colors"
									>
										<Check class="w-4 h-4" />
									</button>
									<button
										type="button"
										onclick={() => deletingUdf = null}
										class="px-3 py-1 bg-slate-200 dark:bg-slate-700 text-slate-700 dark:text-slate-300 rounded hover:bg-slate-300 dark:hover:bg-slate-600 transition-colors"
									>
										<X class="w-4 h-4" />
									</button>
								</div>
							</div>
						{/if}
					</div>
			{/each}
		</div>
	{/if}
</PageLayout>

<!-- Create/Edit Modal -->
{#if showCreateModal}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-[60] p-4" style="padding-top: 100px;">
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-xl max-w-3xl w-full max-h-[90vh] overflow-y-auto">
			<div class="p-6 border-b border-slate-200 dark:border-slate-800">
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">
					{editingUdf ? 'Edit Function' : 'Create New Function'}
				</h2>
			</div>
			
			<div class="p-6 space-y-4">
				<!-- Function Name -->
				<div>
					<label for="function-name" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Function Name *
					</label>
					<input
						id="function-name"
						type="text"
						bind:value={functionName}
						placeholder="my_function"
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500 font-mono"
						disabled={editingUdf !== null}
					/>
					<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
						Must be unique. Use letters, numbers, and underscores only.
					</p>
				</div>
				
				<!-- Parameters -->
				<div>
					<label for="function-params" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Parameters
					</label>
					<input
						id="function-params"
						type="text"
						bind:value={parameters}
						placeholder="param1 VARCHAR, param2 INTEGER"
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500 font-mono"
					/>
					<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
						Comma-separated list of parameters with types. Leave empty for no parameters.
					</p>
				</div>
				
				<!-- Return Type -->
				<div>
					<label for="function-return-type" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Return Type
					</label>
					<select
						id="function-return-type"
						bind:value={returnType}
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
					>
						<option value="VARCHAR">VARCHAR</option>
						<option value="INTEGER">INTEGER</option>
						<option value="BIGINT">BIGINT</option>
						<option value="DOUBLE">DOUBLE</option>
						<option value="BOOLEAN">BOOLEAN</option>
						<option value="DATE">DATE</option>
						<option value="TIMESTAMP">TIMESTAMP</option>
					</select>
				</div>
				
				<!-- Function Body -->
				<div>
					<label for="function-body" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Function Body (SQL Expression) *
					</label>
					<textarea
						id="function-body"
						bind:value={functionBody}
						placeholder="UPPER(param1) || ' - ' || param2"
						rows="8"
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500 font-mono text-sm"
					></textarea>
					<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
						The SQL expression that defines your function. Can reference parameters by name.
					</p>
				</div>
				
				<!-- Description -->
				<div>
					<label for="function-description" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Description
					</label>
					<textarea
						id="function-description"
						bind:value={description}
						placeholder="What does this function do?"
						rows="2"
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
					></textarea>
				</div>
			</div>
			
			<div class="p-6 border-t border-slate-200 dark:border-slate-800 flex items-center justify-end gap-3">
				<Button variant="ghost" onclick={closeModal}>
					Cancel
				</Button>
				<Button variant="primary" onclick={saveFunction} disabled={isSaving}>
					{#if isSaving}
						<svg class="animate-spin h-4 w-4 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
							<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
							<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
						</svg>
						Saving...
					{:else}
						<Save class="w-4 h-4 mr-2" />
						{editingUdf ? 'Update Function' : 'Create Function'}
					{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}

<!-- Test Panel -->
{#if testingUdf}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-[60] p-4" style="padding-top: 100px;">
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] flex flex-col">
			<div class="p-6 border-b border-slate-200 dark:border-slate-800 flex items-center justify-between">
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100 flex items-center gap-2">
					<Play class="w-5 h-5 text-indigo-500" />
					Test Function: {testingUdf}
				</h2>
				<button
					type="button"
					onclick={closeTestPanel}
					class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-300"
				>
					<X class="w-6 h-6" />
				</button>
			</div>
			
			<div class="p-6 space-y-4 flex-1 overflow-auto">
				<!-- Test Query Input -->
				<div>
					<label for="test-query-input" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Test Query
					</label>
					<textarea
						id="test-query-input"
						bind:value={testQuery}
						rows="4"
						placeholder="SELECT my_function('arg1', 123) AS result;"
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500 font-mono text-sm"
					></textarea>
				</div>
				
				<div class="flex gap-2">
					<Button variant="primary" onclick={testFunction} disabled={isTestingFunction}>
						{#if isTestingFunction}
							<svg class="animate-spin h-4 w-4 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
								<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
								<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
							</svg>
							Running...
						{:else}
							<Play class="w-4 h-4 mr-2" />
							Run Test
						{/if}
					</Button>
				</div>
				
				<!-- Test Results -->
				{#if testError}
					<div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
						<p class="text-sm text-red-700 dark:text-red-400 font-mono">{testError}</p>
					</div>
				{:else if testResults.length > 0}
					<div class="border border-slate-200 dark:border-slate-800 rounded-lg overflow-hidden">
						<div class="overflow-x-auto">
							<table class="w-full text-sm">
								<thead class="bg-slate-50 dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700">
									<tr>
										{#each testColumns as column}
											<th class="px-4 py-2 text-left font-medium text-slate-900 dark:text-slate-100 whitespace-nowrap">
												{column}
											</th>
										{/each}
									</tr>
								</thead>
								<tbody>
									{#each testResults as row}
										<tr class="border-b border-slate-100 dark:border-slate-800 hover:bg-slate-50 dark:hover:bg-slate-800/50">
											{#each testColumns as column}
												<td class="px-4 py-2 text-slate-700 dark:text-slate-300 whitespace-nowrap font-mono text-xs">
													{row[column] ?? 'NULL'}
												</td>
											{/each}
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					</div>
				{:else if !isTestingFunction}
					<div class="flex items-center justify-center h-32 text-slate-500 dark:text-slate-400 text-sm">
						Run a test query to see results
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<!-- Info Dialog -->
{#if showInfoDialog && infoUdf}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-[60] p-4" style="padding-top: 100px;">
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-xl max-w-3xl w-full max-h-[90vh] overflow-y-auto">
			<div class="p-6 border-b border-slate-200 dark:border-slate-800 flex items-center justify-between">
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100 flex items-center gap-2">
					<FunctionSquare class="w-6 h-6 text-indigo-500" />
					{infoUdf.function_name}
				</h2>
				<button
					type="button"
					onclick={closeInfoDialog}
					class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-300"
				>
					<X class="w-6 h-6" />
				</button>
			</div>
			
			<div class="p-6 space-y-4">
				<!-- Description -->
				{#if infoUdf.description}
					<div>
						<h3 class="text-sm font-semibold text-slate-500 dark:text-slate-400 uppercase mb-2">Description</h3>
						<p class="text-slate-700 dark:text-slate-300">
							{infoUdf.description}
						</p>
					</div>
				{/if}
				
				<!-- Signature -->
				<div>
					<h3 class="text-sm font-semibold text-slate-500 dark:text-slate-400 uppercase mb-2">Signature</h3>
					<div class="bg-slate-50 dark:bg-slate-950 rounded p-4 font-mono text-sm">
						<div class="text-slate-600 dark:text-slate-400 mb-2">
							<span class="text-purple-500">CREATE MACRO</span> {infoUdf.function_name}({infoUdf.parameters})
						</div>
						<div class="text-slate-500 dark:text-slate-500">
							→ Returns: <span class="text-blue-500">{infoUdf.return_type}</span>
						</div>
					</div>
				</div>
				
				<!-- Function Body -->
				<div>
					<div class="flex items-center justify-between mb-2">
						<h3 class="text-sm font-semibold text-slate-500 dark:text-slate-400 uppercase">Function Body</h3>
						<button
							type="button"
							onclick={() => copyToClipboard(infoUdf.function_body)}
							class="flex items-center gap-1 text-xs text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-300 transition-colors"
							title="Copy to clipboard"
						>
							<Copy class="w-3.5 h-3.5" />
							Copy
						</button>
					</div>
					<div class="bg-slate-50 dark:bg-slate-950 rounded p-4">
						<pre class="text-sm text-slate-700 dark:text-slate-300 overflow-x-auto whitespace-pre-wrap break-words">{infoUdf?.function_body}</pre>
					</div>
				</div>
				
				<!-- Metadata -->
				<div>
					<h3 class="text-sm font-semibold text-slate-500 dark:text-slate-400 uppercase mb-2">Metadata</h3>
					<div class="bg-slate-50 dark:bg-slate-950 rounded p-4 space-y-2 text-sm">
						<div class="flex justify-between">
							<span class="text-slate-500 dark:text-slate-400">Created:</span>
							<span class="text-slate-700 dark:text-slate-300 font-medium">
								{new Date(infoUdf.created_at).toLocaleString()}
							</span>
						</div>
						<div class="flex justify-between">
							<span class="text-slate-500 dark:text-slate-400">Updated:</span>
							<span class="text-slate-700 dark:text-slate-300 font-medium">
								{new Date(infoUdf.updated_at).toLocaleString()}
							</span>
						</div>
					</div>
				</div>
			</div>
			
			<div class="p-6 border-t border-slate-200 dark:border-slate-800 flex items-center justify-end">
				<Button variant="ghost" onclick={closeInfoDialog}>
					Close
				</Button>
			</div>
		</div>
	</div>
{/if}

