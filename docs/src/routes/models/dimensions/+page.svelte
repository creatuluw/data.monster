<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Button from '$lib/components/Button.svelte';
	import { 
		Tag as TagIcon, 
		Plus, 
		Edit, 
		Trash2, 
		X, 
		Check, 
		AlertCircle,
		Copy
	} from 'lucide-svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { parseQueryResult } from '$lib/db-utils';

	// System messages via global event
	function addSystemMessage(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
		window.dispatchEvent(new CustomEvent('add-system-message', {
			detail: { message, type }
		}));
	}

	// Dimension Types
	interface Dimension {
		slug: string;
		dimension_name: string;
		field_name: string;
		source_table: string;
		description: string | null;
		tags: string | null;
		created_at: string;
		updated_at: string;
	}

	// State
	let dimensions = $state<Dimension[]>([]);
	let isLoading = $state(false);
	let isTauriAvailable = $state(false);

	// Form state for creating/editing dimensions
	let showCreateModal = $state(false);
	let editingDimension = $state<string | null>(null);
	let dimensionSlug = $state('');
	let dimensionName = $state('');
	let fieldName = $state('');
	let sourceTable = $state('');
	let description = $state('');
	let tags = $state('');
	let isSaving = $state(false);

	// Delete confirmation
	let deletingDimension = $state<string | null>(null);

	// Filter state
	let searchTerm = $state('');
	let selectedTag = $state('');

	// Field selector state
	interface FieldOption {
		table: string;
		column: string;
		type: string;
		fullName: string; // table.column
	}
	let availableFields = $state<FieldOption[]>([]);
	let selectedField = $state('');
	let isLoadingFields = $state(false);

	async function loadDimensions() {
		try {
			isLoading = true;
			const result = await invoke<string>('list_dimensions');
			dimensions = parseQueryResult(result);
		} catch (error) {
			console.error('Error loading dimensions:', error);
			addSystemMessage(`Error loading dimensions: ${error}`, 'error');
		} finally {
			isLoading = false;
		}
	}

	async function loadAvailableFields() {
		try {
			isLoadingFields = true;
			
			// Get all tables from the data model
			const datamodelsList = await invoke<string[]>('get_saved_tables');
			
			// Get columns for each model table
			const fields: FieldOption[] = [];
			
			for (const tableName of datamodelsList) {
				try {
					const columnsResult = await invoke<string>('execute_query', {
						query: `SELECT column_name, data_type FROM information_schema.columns WHERE table_name = '${tableName}' ORDER BY ordinal_position`
					});
					const columns: Array<{ column_name: string; data_type: string }> = parseQueryResult(columnsResult);
					
					for (const col of columns) {
						fields.push({
							table: tableName,
							column: col.column_name,
							type: col.data_type,
							fullName: `${tableName}.${col.column_name}`
						});
					}
				} catch (error) {
					console.error(`Error loading columns for ${tableName}:`, error);
				}
			}
			
			availableFields = fields;
		} catch (error) {
			console.error('Error loading fields:', error);
			addSystemMessage(`Error loading fields: ${error}`, 'error');
		} finally {
			isLoadingFields = false;
		}
	}

	function onFieldSelect() {
		if (!selectedField) return;
		
		// Extract table and column name from the selected field (table.column)
		const [tableName, columnName] = selectedField.split('.');
		
		// Set field name and source table
		fieldName = selectedField;
		sourceTable = tableName;
		
		// Auto-set dimension name if empty
		if (!dimensionName && columnName) {
			// Convert snake_case or camelCase to Title Case
			dimensionName = columnName
				.replace(/_/g, ' ')
				.replace(/([A-Z])/g, ' $1')
				.split(' ')
				.map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
				.join(' ')
				.trim();
		}
	}

	function openCreateModal() {
		showCreateModal = true;
		editingDimension = null;
		dimensionSlug = '';
		dimensionName = '';
		fieldName = '';
		sourceTable = '';
		description = '';
		tags = '';
		selectedField = '';
		loadAvailableFields();
	}

	function openEditModal(dimension: Dimension) {
		showCreateModal = true;
		editingDimension = dimension.dimension_name;
		dimensionSlug = dimension.slug;
		dimensionName = dimension.dimension_name;
		fieldName = dimension.field_name;
		sourceTable = dimension.source_table;
		description = dimension.description || '';
		tags = dimension.tags || '';
		selectedField = dimension.field_name;
		loadAvailableFields();
	}

	function closeModal() {
		showCreateModal = false;
		editingDimension = null;
		dimensionSlug = '';
		dimensionName = '';
		fieldName = '';
		sourceTable = '';
		description = '';
		tags = '';
		selectedField = '';
	}

	async function saveDimension() {
		console.log('=== SAVE DIMENSION DEBUG START ===');
		console.log('dimensionName:', dimensionName);
		console.log('fieldName:', fieldName);
		console.log('sourceTable:', sourceTable);
		console.log('description:', description);
		console.log('tags:', tags);
		console.log('dimensionSlug:', dimensionSlug);
		console.log('editingDimension:', editingDimension);
		
		if (!dimensionName.trim() || !fieldName.trim() || !sourceTable.trim()) {
			console.log('❌ Validation failed - missing required fields');
			console.log('dimensionName.trim():', dimensionName.trim());
			console.log('fieldName.trim():', fieldName.trim());
			console.log('sourceTable.trim():', sourceTable.trim());
			addSystemMessage('Dimension name, field, and source table are required', 'warning');
			return;
		}
		
		console.log('✅ Validation passed');

		try {
			isSaving = true;
			console.log('Calling invoke with params:', {
				dimensionName: dimensionName.trim(),
				fieldName: fieldName.trim(),
				sourceTable: sourceTable.trim(),
				description: description.trim(),
				tags: tags.trim(),
				existingSlug: dimensionSlug || null
			});
			
			const result = await invoke<string>('create_dimension', {
				dimensionName: dimensionName.trim(),
				fieldName: fieldName.trim(),
				sourceTable: sourceTable.trim(),
				description: description.trim(),
				tags: tags.trim(),
				existingSlug: dimensionSlug || null  // Pass slug when editing
			});

			console.log('✅ Success! Result:', result);
			addSystemMessage(result, 'success');
			closeModal();
			await loadDimensions();
		} catch (error) {
			console.error('❌ Error saving dimension:', error);
			console.error('Error type:', typeof error);
			console.error('Error details:', JSON.stringify(error, null, 2));
			addSystemMessage(`Error saving dimension: ${error}`, 'error');
		} finally {
			isSaving = false;
			console.log('=== SAVE DIMENSION DEBUG END ===');
		}
	}

	async function deleteDimension(dimensionName: string) {
		try {
			const result = await invoke<string>('delete_dimension', { dimensionName });
			addSystemMessage(result, 'success');
			deletingDimension = null;
			await loadDimensions();
		} catch (error) {
			addSystemMessage(`Error deleting dimension: ${error}`, 'error');
		}
	}

	function copyToClipboard(text: string) {
		navigator.clipboard.writeText(text);
		addSystemMessage('Copied to clipboard', 'success');
	}

	// Get unique tags
	const allTags = $derived(
		Array.from(new Set(
			dimensions
				.filter(d => d.tags)
				.flatMap(d => d.tags!.split(',').map(t => t.trim()).filter(t => t))
		)).sort()
	);

	// Filtered dimensions
	const filteredDimensions = $derived(
		dimensions.filter(d => {
			const matchesSearch = !searchTerm || 
				d.dimension_name.toLowerCase().includes(searchTerm.toLowerCase()) ||
				d.field_name.toLowerCase().includes(searchTerm.toLowerCase()) ||
				(d.description && d.description.toLowerCase().includes(searchTerm.toLowerCase()));
			
			const matchesTag = !selectedTag || 
				(d.tags && d.tags.split(',').map(t => t.trim()).includes(selectedTag));
			
			return matchesSearch && matchesTag;
		})
	);

	onMount(async () => {
		try {
			await invoke('initialize_duckdb');
			isTauriAvailable = true;
			addSystemMessage('Connected to database', 'success');
			await loadDimensions();
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
	<div class="flex items-start justify-between mb-6 gap-4">
		<div class="flex items-center gap-3">
			<Button variant="ghost" onclick={() => window.location.href = '/models/metrics'}>
				View Metrics
			</Button>
			<Button variant="primary" onclick={openCreateModal} disabled={!isTauriAvailable}>
				<Plus class="w-4 h-4 mr-2" />
				New Dimension
			</Button>
		</div>
	</div>

	{#if isLoading}
		<div class="flex items-center justify-center h-64">
			<div class="text-slate-500 dark:text-slate-400">Loading dimensions...</div>
		</div>
	{:else if dimensions.length === 0}
		<div class="flex flex-col items-center justify-center h-64 text-center">
			<TagIcon class="w-16 h-16 text-slate-300 dark:text-slate-600 mb-4" />
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
				No dimensions yet
			</h3>
			<p class="text-slate-600 dark:text-slate-400 mb-6 max-w-md">
				Create your first dimension by selecting a field from your data model and giving it a meaningful name and tags for analysis.
			</p>
			<Button variant="primary" onclick={openCreateModal} disabled={!isTauriAvailable}>
				<Plus class="w-4 h-4 mr-2" />
				Create Dimension
			</Button>
		</div>
	{:else}
		<!-- Search and Filter -->
		<div class="mb-6 flex items-center gap-4 flex-wrap">
			<input
				type="text"
				bind:value={searchTerm}
				placeholder="Search dimensions..."
				class="flex-1 min-w-[200px] px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
			/>
			{#if allTags.length > 0}
				<select
					bind:value={selectedTag}
					class="px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
				>
					<option value="">All Tags</option>
					{#each allTags as tag}
						<option value={tag}>{tag}</option>
					{/each}
				</select>
			{/if}
			<span class="text-sm text-slate-500 dark:text-slate-400">
				{filteredDimensions.length} dimension{filteredDimensions.length === 1 ? '' : 's'}
			</span>
		</div>

		<!-- Dimensions Grid -->
		<div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4">
			{#each filteredDimensions as dimension}
				<div class="bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-800 rounded-lg p-5 hover:shadow-md transition-shadow">
					<!-- Header -->
					<div class="flex items-start justify-between mb-3">
						<div class="flex-1">
							<div class="flex items-center gap-2 mb-1">
								<TagIcon class="w-5 h-5 text-purple-500" />
								<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100">
									{dimension.dimension_name}
								</h3>
							</div>
							{#if dimension.description}
								<p class="text-sm text-slate-600 dark:text-slate-400 mb-2">
									{dimension.description}
								</p>
							{/if}
						</div>
						<div class="flex items-center gap-2">
							<button
								type="button"
								onclick={() => openEditModal(dimension)}
								class="p-2 text-slate-400 hover:text-blue-500 hover:bg-slate-100 dark:hover:bg-slate-800 rounded transition-colors"
								title="Edit dimension"
							>
								<Edit class="w-4 h-4" />
							</button>
							<button
								type="button"
								onclick={() => deletingDimension = dimension.dimension_name}
								class="p-2 text-slate-400 hover:text-red-500 hover:bg-slate-100 dark:hover:bg-slate-800 rounded transition-colors"
								title="Delete dimension"
							>
								<Trash2 class="w-4 h-4" />
							</button>
						</div>
					</div>

					<!-- Field Name -->
					<div class="bg-slate-50 dark:bg-slate-950 rounded p-3 mb-3">
						<div class="flex items-center justify-between mb-2">
							<span class="text-xs font-semibold text-slate-500 dark:text-slate-400 uppercase">Field</span>
							<button
								type="button"
								onclick={() => copyToClipboard(dimension.field_name)}
								class="text-slate-400 hover:text-slate-600 dark:hover:text-slate-300"
								title="Copy field name"
							>
								<Copy class="w-3.5 h-3.5" />
							</button>
						</div>
						<pre class="text-sm font-mono text-slate-700 dark:text-slate-300">{dimension.field_name}</pre>
					</div>

					<!-- Source Table -->
					<div class="mb-3">
						<span class="text-xs font-semibold text-slate-500 dark:text-slate-400 uppercase">Source Table:</span>
						<span class="ml-2 text-sm font-mono text-slate-700 dark:text-slate-300">{dimension.source_table}</span>
					</div>

					<!-- Tags -->
					{#if dimension.tags}
						<div class="flex items-center gap-2 flex-wrap mb-3">
							{#each dimension.tags.split(',').map(t => t.trim()).filter(t => t) as tag}
								<span class="inline-flex items-center gap-1 text-xs px-2 py-1 rounded-full bg-purple-50 dark:bg-purple-900/20 text-purple-700 dark:text-purple-400 border border-purple-200 dark:border-purple-800">
									<TagIcon class="w-3 h-3" />
									{tag}
								</span>
							{/each}
						</div>
					{/if}

					<!-- Metadata -->
					<div class="text-xs text-slate-500 dark:text-slate-400">
						Updated: {new Date(dimension.updated_at).toLocaleDateString()}
					</div>

					<!-- Delete Confirmation -->
					{#if deletingDimension === dimension.dimension_name}
						<div class="mt-3 pt-3 border-t border-slate-200 dark:border-slate-800">
							<div class="flex items-center gap-2 text-sm">
								<AlertCircle class="w-4 h-4 text-red-500" />
								<span class="text-slate-700 dark:text-slate-300 flex-1">Delete this dimension?</span>
								<button
									type="button"
									onclick={() => deleteDimension(dimension.dimension_name)}
									class="px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600 transition-colors"
								>
									<Check class="w-4 h-4" />
								</button>
								<button
									type="button"
									onclick={() => deletingDimension = null}
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
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
			<div class="p-6 border-b border-slate-200 dark:border-slate-800">
				<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">
					{editingDimension ? 'Edit Dimension' : 'Create New Dimension'}
				</h2>
			</div>

			<div class="p-6 space-y-4">
				<!-- Field Selector -->
				<div>
					<label for="field-selector" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Select Field *
					</label>
					<select
						id="field-selector"
						bind:value={selectedField}
						onchange={onFieldSelect}
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
						disabled={isLoadingFields || editingDimension !== null}
					>
						<option value="">
							{isLoadingFields ? 'Loading fields...' : availableFields.length === 0 ? 'No model tables' : 'Select a field from your data model'}
						</option>
						{#if availableFields.length > 0}
							{@const tableGroups = availableFields.reduce((acc, field) => {
								if (!acc[field.table]) acc[field.table] = [];
								acc[field.table].push(field);
								return acc;
							}, {} as Record<string, FieldOption[]>)}
							{#each Object.entries(tableGroups) as [tableName, fields]}
								<optgroup label={tableName}>
									{#each fields as field}
										<option value={field.fullName}>
											{field.column} ({field.type})
										</option>
									{/each}
								</optgroup>
							{/each}
						{/if}
					</select>
					<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
						Choose a field from your data model to use as a dimension
					</p>
				</div>

				<!-- Dimension Name -->
				<div>
					<label for="dimension-name" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Title *
					</label>
				<input
					id="dimension-name"
					type="text"
					bind:value={dimensionName}
					placeholder="Product Category"
					class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
				/>
					<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
						A friendly name for this dimension (auto-suggested from field name)
					</p>
				</div>

				<!-- Slug (read-only, only shown when editing) -->
				{#if editingDimension && dimensionSlug}
					<div>
						<label for="dimension-slug" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
							Slug (Unique Identifier)
						</label>
						<input
							id="dimension-slug"
							type="text"
							value={dimensionSlug}
							class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-slate-50 dark:bg-slate-800 text-slate-500 dark:text-slate-400 font-mono text-sm cursor-not-allowed"
							readonly
							disabled
						/>
						<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
							The slug is auto-generated and cannot be changed
						</p>
					</div>
				{/if}

				<!-- Field Name (read-only) -->
				<div>
					<label for="field-name" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Field Name
					</label>
					<input
						id="field-name"
						type="text"
						value={fieldName}
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-slate-50 dark:bg-slate-800 text-slate-900 dark:text-slate-100 font-mono text-sm"
						readonly
					/>
				</div>

				<!-- Source Table (read-only) -->
				<div>
					<label for="source-table" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Source Table
					</label>
					<input
						id="source-table"
						type="text"
						value={sourceTable}
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-slate-50 dark:bg-slate-800 text-slate-900 dark:text-slate-100 font-mono text-sm"
						readonly
					/>
				</div>

				<!-- Description -->
				<div>
					<label for="dimension-description" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Description
					</label>
					<textarea
						id="dimension-description"
						bind:value={description}
						placeholder="What does this dimension represent?"
						rows="2"
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
					></textarea>
				</div>

				<!-- Tags -->
				<div>
					<label for="dimension-tags" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
						Tags
					</label>
					<input
						id="dimension-tags"
						type="text"
						bind:value={tags}
						placeholder="product, categorical, geography"
						class="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
					/>
					<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">
						Comma-separated tags for organizing and filtering dimensions
					</p>
					</div>
			</div>

			<div class="p-6 border-t border-slate-200 dark:border-slate-800 flex items-center justify-end gap-3">
				<Button variant="ghost" onclick={closeModal}>
					Cancel
				</Button>
				<Button variant="primary" onclick={saveDimension} disabled={isSaving}>
					{#if isSaving}
						Saving...
					{:else}
						{editingDimension ? 'Update Dimension' : 'Create Dimension'}
					{/if}
				</Button>
			</div>
		</div>
	</div>
{/if}
