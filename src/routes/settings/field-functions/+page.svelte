<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import {
		listFieldFunctions,
		createFieldFunction,
		updateFieldFunction,
		deleteFieldFunction,
		extractErrorMessage,
		type FieldFunction
	} from '$lib/db-operations';
	import { ArrowLeft, Plus, Trash2, Pencil, X, Check, XCircle } from 'lucide-svelte';

	let functions = $state<FieldFunction[]>([]);
	let loading = $state(true);
	let error = $state('');

	let showForm = $state(false);
	let editingId = $state<string | null>(null);
	let formId = $state('');
	let formLabel = $state('');
	let formDescription = $state('');
	let formSqlTemplate = $state('');
	let formAppliesTo = $state('');
	let formOutputType = $state('');
	let formError = $state('');
	let formSaving = $state(false);

	let deletingId = $state<string | null>(null);

	async function load() {
		loading = true;
		error = '';
		try {
			functions = await listFieldFunctions();
		} catch (e) {
			error = extractErrorMessage(e, 'Failed to load field functions');
		}
		loading = false;
	}

	function openCreateForm() {
		editingId = null;
		formId = '';
		formLabel = '';
		formDescription = '';
		formSqlTemplate = '';
		formAppliesTo = '';
		formOutputType = '';
		formError = '';
		showForm = true;
	}

	function openEditForm(fn: FieldFunction) {
		editingId = fn.id;
		formId = fn.id;
		formLabel = fn.label;
		formDescription = fn.description || '';
		formSqlTemplate = fn.sql_template;
		formAppliesTo = fn.applies_to;
		formOutputType = fn.output_type || '';
		formError = '';
		showForm = true;
	}

	function closeForm() {
		showForm = false;
		editingId = null;
	}

	async function handleSave() {
		formError = '';
		if (!formId.trim()) { formError = 'ID is required'; return; }
		if (!formLabel.trim()) { formError = 'Label is required'; return; }
		if (!formSqlTemplate.trim()) { formError = 'SQL template is required'; return; }
		if (!formAppliesTo.trim()) { formError = 'Type patterns are required'; return; }

		formSaving = true;
		try {
			if (editingId) {
				await updateFieldFunction(
					editingId,
					formLabel.trim(),
					formDescription.trim() || null,
					formSqlTemplate.trim(),
					formAppliesTo.trim(),
					formOutputType.trim()
				);
				functions = functions.map(f =>
					f.id === editingId
						? { ...f, label: formLabel.trim(), description: formDescription.trim() || null, sql_template: formSqlTemplate.trim(), applies_to: formAppliesTo.trim(), output_type: formOutputType.trim() }
						: f
				);
			} else {
				await createFieldFunction(
					formId.trim(),
					formLabel.trim(),
					formDescription.trim() || null,
					formSqlTemplate.trim(),
					formAppliesTo.trim(),
					formOutputType.trim()
				);
				await load();
			}
			closeForm();
		} catch (e) {
			formError = extractErrorMessage(e, 'Failed to save');
		}
		formSaving = false;
	}

	async function handleDelete(id: string) {
		try {
			await deleteFieldFunction(id);
			functions = functions.filter(f => f.id !== id);
		} catch {
		}
		deletingId = null;
	}

	onMount(() => {
		load();
	});
</script>

<svelte:head>
	<title>Field Functions — Data Monster</title>
</svelte:head>

<div class="ff-page">
	<div class="ff-header">
		<button class="btn btn-ghost btn-sm" onclick={() => goto('/settings')}>
			<ArrowLeft size={14} />
			Settings
		</button>
		<h1 class="ff-title">Field Functions</h1>
		<div style="flex:1"></div>
		<button class="btn btn-primary btn-sm" onclick={openCreateForm}>
			<Plus size={14} />
			Add function
		</button>
	</div>

	<p class="ff-desc">Define query-side functions that can be applied to table columns. Functions are filtered by column type when shown in the query builder.</p>

	{#if loading}
		<div class="ff-loading">Loading...</div>
	{:else if error}
		<div class="ff-error">{error}</div>
	{:else if functions.length === 0}
		<div class="ff-empty">
			<span>No functions defined</span>
			<button class="btn btn-secondary btn-sm" onclick={openCreateForm}>
				<Plus size={14} />
				Add your first function
			</button>
		</div>
	{:else}
		<div class="ff-table-wrap">
			<table class="ff-table">
				<thead>
					<tr>
						<th>ID</th>
						<th>Label</th>
						<th>Description</th>
						<th>Applies to</th>
						<th>Output</th>
						<th style="width:80px">Actions</th>
					</tr>
				</thead>
				<tbody>
					{#each functions as fn}
						<tr>
							<td class="ff-monos">{fn.id}</td>
							<td class="ff-label-col">{fn.label}</td>
							<td class="ff-desc-col">{fn.description || '\u2014'}</td>
							<td>
								<div class="ff-type-tags">
									{#each fn.applies_to.split(',').map(t => t.trim()).filter(t => t) as type}
										<span class="tag tag-default">{type}</span>
									{/each}
							</div>
						</td>
							<td class="ff-monos">{fn.output_type || '\u2014'}</td>
							<td>
								<div class="ff-actions">
									<button class="btn btn-ghost btn-sm" onclick={() => openEditForm(fn)} title="Edit">
										<Pencil size={14} />
									</button>
									{#if deletingId === fn.id}
										<div class="ff-delete-confirm">
											<button class="btn btn-sm btn-danger" onclick={() => handleDelete(fn.id)}>
												<Check size={12} />
											</button>
											<button class="btn btn-sm btn-secondary" onclick={() => deletingId = null}>
												<XCircle size={12} />
											</button>
										</div>
									{:else}
										<button class="btn btn-ghost btn-sm" onclick={() => deletingId = fn.id} title="Delete">
											<Trash2 size={14} style="color: var(--color-danger);" />
										</button>
									{/if}
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>

<!-- Add/Edit Modal -->
{#if showForm}
	<div class="modal-overlay" onclick={closeForm}>
		<div class="modal" onclick={(e) => e.stopPropagation()}>
			<div class="modal-header">
				<h2 class="modal-title">{editingId ? 'Edit function' : 'Add function'}</h2>
				<button onclick={closeForm} class="modal-close"><XCircle size={20} /></button>
			</div>

			<div class="modal-body">
				<div class="field">
					<label for="ff-id" class="field-label">ID *</label>
					<input
						id="ff-id"
						type="text"
						class="input input-mono"
						bind:value={formId}
						placeholder="week_end"
						disabled={editingId !== null}
					/>
					<span class="field-hint">Unique identifier, e.g. "week_end"</span>
				</div>

				<div class="field">
					<label for="ff-label" class="field-label">Label *</label>
					<input
						id="ff-label"
						type="text"
						class="input"
						bind:value={formLabel}
						placeholder="Week end"
					/>
				</div>

				<div class="field">
					<label for="ff-desc" class="field-label">Description</label>
					<input
						id="ff-desc"
						type="text"
						class="input"
						bind:value={formDescription}
						placeholder="Last day of the week"
					/>
				</div>

				<div class="field">
					<label for="ff-template" class="field-label">SQL template *</label>
					<textarea
						id="ff-template"
						class="input input-mono"
						bind:value={formSqlTemplate}
						placeholder="date_trunc('week', col) + INTERVAL 6 DAYS"
						rows={3}
						style="resize: none;"
					></textarea>
					<span class="field-hint">Use (column) and (alias) placeholders</span>
				</div>

				<div class="field">
					<label for="ff-applies" class="field-label">Applies to *</label>
					<input
						id="ff-applies"
						type="text"
						class="input"
						bind:value={formAppliesTo}
						placeholder="DATE,TIMESTAMP"
					/>
					<span class="field-hint">Comma-separated type patterns. Use TIMESTAMP% for wildcard matching.</span>
				</div>

				<div class="field">
					<label for="ff-output-type" class="field-label">Output type</label>
					<input
						id="ff-output-type"
						type="text"
						class="input"
						bind:value={formOutputType}
						placeholder="DATE"
					/>
					<span class="field-hint">DuckDB type to cast the result to, e.g. DATE, VARCHAR, INTEGER</span>
				</div>
			</div>

			{#if formError}
				<div class="modal-error">
					<span>{formError}</span>
					<button class="modal-error-close" onclick={() => formError = ''}>&times;</button>
				</div>
			{/if}

			<div class="modal-footer">
				<button class="btn btn-secondary" onclick={closeForm} disabled={formSaving}>Cancel</button>
				<button class="btn btn-primary" onclick={handleSave} disabled={formSaving}>
					{#if formSaving}
						Saving...
					{:else}
						{editingId ? 'Update' : 'Create'}
					{/if}
				</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.ff-page {
		max-width: 56rem;
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
		padding: var(--space-6);
	}

	.ff-header {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}

	.ff-title {
		font-family: var(--font-display);
		font-size: var(--text-lg);
		font-weight: 700;
		letter-spacing: -0.02em;
		color: var(--color-text);
		margin: 0;
	}

	.ff-desc {
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		margin: 0;
		max-width: 60ch;
	}

	.ff-loading {
		padding: var(--space-8);
		text-align: center;
		color: var(--color-text-tertiary);
	}

	.ff-error {
		padding: var(--space-4);
		background: oklch(0.95 0.03 25);
		border: 1px solid oklch(0.9 0.04 25);
		border-radius: var(--radius-xs);
		font-size: var(--text-sm);
		color: oklch(0.38 0.12 25);
	}

	.ff-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-4);
		padding: var(--space-12) 0;
		text-align: center;
		color: var(--color-text-tertiary);
	}

	.ff-table-wrap {
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		overflow: hidden;
		background: var(--color-surface);
	}

	.ff-table {
		width: 100%;
		border-collapse: collapse;
		font-size: var(--text-xs);
	}

	.ff-table th {
		text-align: left;
		padding: var(--space-2) var(--space-3);
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
		background: var(--color-surface-raised);
		border-bottom: 1px solid var(--color-border);
	}

	.ff-table td {
		padding: var(--space-2) var(--space-3);
		border-bottom: 1px solid var(--color-border);
		color: var(--color-text-secondary);
		vertical-align: middle;
	}

	.ff-table tr:last-child td {
		border-bottom: none;
	}

	.ff-table tr:hover td {
		background: var(--color-surface-sunken);
	}

	.ff-monos {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		color: var(--color-text);
	}

	.ff-label-col {
		font-weight: 600;
		color: var(--color-text);
	}

	.ff-desc-col {
		max-width: 240px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.ff-type-tags {
		display: flex;
		flex-wrap: wrap;
		gap: 2px;
	}

	.ff-actions {
		display: flex;
		align-items: center;
		gap: 2px;
	}

	.ff-delete-confirm {
		display: flex;
		gap: 2px;
	}

	.input-mono {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.field-label {
		font-size: 9px;
		font-weight: 600;
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.field-hint {
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
		opacity: 0.7;
	}

	.modal-overlay {
		position: fixed;
		inset: 0;
		background: oklch(0.14 0.01 250 / 0.5);
		display: flex;
		align-items: flex-start;
		justify-content: center;
		padding: 4rem 1rem;
		z-index: 60;
	}

	.modal {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-lg);
		box-shadow: var(--shadow-lg);
		width: 100%;
		max-width: 32rem;
		max-height: 80vh;
		overflow-y: auto;
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-4) var(--space-6);
		border-bottom: 1px solid var(--color-border);
		position: sticky;
		top: 0;
		background: var(--color-surface);
		z-index: 1;
	}

	.modal-title {
		font-family: var(--font-display);
		font-size: var(--text-md);
		font-weight: 600;
		color: var(--color-text);
	}

	.modal-close {
		display: flex;
		align-items: center;
		border: none;
		background: none;
		cursor: pointer;
		color: var(--color-text-tertiary);
		transition: color var(--duration-fast) ease;
	}

	.modal-close:hover {
		color: var(--color-text);
	}

	.modal-body {
		padding: var(--space-6);
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.modal-error {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin: 0 var(--space-6) var(--space-3);
		padding: var(--space-2) var(--space-3);
		color: var(--color-error);
		background: rgba(220, 38, 38, 0.08);
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
	}

	.modal-error-close {
		background: none;
		border: none;
		color: var(--color-error);
		cursor: pointer;
		padding: 0 0 0 var(--space-2);
		font-size: var(--text-sm);
		opacity: 0.7;
	}

	.modal-footer {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		gap: var(--space-3);
		padding: var(--space-4) var(--space-6);
		border-top: 1px solid var(--color-border);
	}

	:global(.btn-danger) {
		background: var(--color-danger);
		color: white;
		border-color: var(--color-danger);
	}
</style>
