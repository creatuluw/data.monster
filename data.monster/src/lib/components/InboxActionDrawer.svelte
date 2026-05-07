<script lang="ts">
	import { untrack } from "svelte";
	import { Drawer, Input, DatePicker, Tag, TagInput, ButtonGroup, Searchahead, Button } from "$lib/components";
	import { formatDate } from "$lib/utils/formatDate";

	type InboxActionRow = {
		id: string;
		title: string;
		description: string | null;
		actionStatus: string;
		assignedTo: string | null;
		createdBy: string;
		createdAt: string;
		updatedAt: string;
		dueDate: string | null;
		actionType: string;
		sourceType: string | null;
		sourceId: string | null;
		ruleId: string | null;
		snoozedUntil: string | null;
		refId: string | null;
		archived: boolean;
		tags: string[];
		data: Record<string, any>;
	};

	let {
		open = $bindable(false),
		action = $bindable(null as InboxActionRow | null),
	}: {
		open?: boolean;
		action?: InboxActionRow | null;
	} = $props();

	let localTitle = $state("");
	let localDescription = $state("");
	let displayStatus = $state("");
	let localAssignedToId = $state<string | null>(null);
	let localDueDate = $state<string | null>(null);
	let localTags = $state<string[]>([]);
	let saving = $state(false);
	let error = $state<string | null>(null);
	let teacherOptions: { text: string; value: string; meta?: string }[] = $state([]);
	let teacherLoading = $state(false);
	let assigneeSelected: any[] = $state([]);
	let lastActionId = $state<string | null>(null);
	let actionCounter = $state(0);

	let isDirty = $derived.by(() => {
		if (!action) return false;
		return (
			localTitle !== action.title ||
			localDescription !== (action.description ?? "") ||
			displayStatus !== action.actionStatus ||
			localAssignedToId !== action.assignedTo ||
			localDueDate !== (action.dueDate ? new Date(action.dueDate).toISOString().slice(0, 10) : null) ||
			JSON.stringify(localTags) !== JSON.stringify(action.tags ?? [])
		);
	});

	const STATUS_OPTIONS: { value: string; label: string }[] = [
		{ value: "backlog", label: "Backlog" },
		{ value: "todo", label: "Todo" },
		{ value: "in_progress", label: "In uitvoering" },
		{ value: "in_review", label: "In review" },
		{ value: "done", label: "Klaar" },
		{ value: "dismissed", label: "Afgewezen" },
	];

	let statusItems = $derived(
		STATUS_OPTIONS.map((o) => ({ value: o.value, label: o.label })),
	);

	let isOverdue = $derived(
		localDueDate !== null && new Date(localDueDate) < new Date(),
	);

	let dataEntries = $derived(
		action?.data ? Object.entries(action.data) : [],
	);

	$effect(() => {
		if (action && action.id !== lastActionId) {
			untrack(() => {
				localTitle = action!.title;
				localDescription = action!.description ?? "";
				displayStatus = action!.actionStatus;
				localAssignedToId = action!.assignedTo;
				localDueDate = action!.dueDate
					? new Date(action!.dueDate).toISOString().slice(0, 10)
					: null;
				localTags = [...(action!.tags ?? [])];
				error = null;
				if (action!.assignedTo) {
					assigneeSelected = [{ text: action!.assignedTo, value: action!.assignedTo }];
				} else {
					assigneeSelected = [];
				}
				lastActionId = action!.id;
				actionCounter += 1;
			});
		}
	});

	let teacherAbort: AbortController | null = null;
	function fetchTeachers(search?: string) {
		teacherAbort?.abort();
		teacherAbort = new AbortController();
		teacherLoading = true;
		const params = new URLSearchParams();
		if (search && search.trim().length >= 2) {
			params.set("search", search.trim());
		}
		params.set("limit", "25");
		fetch(`/api/teachers?${params}`, { signal: teacherAbort.signal })
			.then((r) => r.json())
			.then((data) => {
				teacherOptions = (data as any[]).map((t) => ({
					text: t.name || t.email,
					value: t.id,
					meta: t.email,
				}));
			})
			.catch((e) => {
				if (e.name !== "AbortError") teacherOptions = [];
			})
			.finally(() => {
				teacherLoading = false;
			});
	}

	function handleAssigneeSearch(query: string) {
		fetchTeachers(query);
	}

	async function handleSubmit() {
		if (!action || !isDirty || saving) return;
		saving = true;
		error = null;

		const patch: Record<string, any> = { updated_at: action.updatedAt };

		if (localTitle !== action.title) patch.title = localTitle;
		if (localDescription !== (action.description ?? "")) patch.description = localDescription || null;
		if (localAssignedToId !== action.assignedTo) patch.assigned_to = localAssignedToId;
		if (localDueDate !== (action.dueDate ? new Date(action.dueDate).toISOString().slice(0, 10) : null)) {
			patch.due_date = localDueDate;
		}
		if (JSON.stringify(localTags) !== JSON.stringify(action.tags ?? [])) {
			patch.tags = localTags;
		}

		const statusChanged = displayStatus !== action.actionStatus;

		try {
			if (Object.keys(patch).length > 1) {
				const res = await fetch(`/api/inbox-actions/${action.id}`, {
					method: "PATCH",
					headers: { "Content-Type": "application/json" },
					body: JSON.stringify(patch),
				});
				if (!res.ok) {
					const body = await res.json();
					error = body.error ?? "Opslaan mislukt";
					saving = false;
					return;
				}
				action = await res.json();
			}

			if (statusChanged) {
				if (!action) { saving = false; return; }
				const res = await fetch(`/api/inbox-actions/${action.id}/status`, {
					method: "PATCH",
					headers: { "Content-Type": "application/json" },
					body: JSON.stringify({
						action_status: displayStatus,
						updated_at: action.updatedAt,
					}),
				});
				if (!res.ok) {
					const body = await res.json();
					error = body.error ?? "Status wijzigen mislukt";
					saving = false;
					return;
				}
				action = await res.json();
			}
		} catch (e) {
			error = "Netwerkfout";
		} finally {
			saving = false;
		}
	}

	function handleAssigneeSelect(item: { text: string; value?: string }) {
		const id = item.value ?? item.text;
		localAssignedToId = id;
		assigneeSelected = [item];
	}

	function handleAssigneeRemove() {
		localAssignedToId = null;
		assigneeSelected = [];
	}

	function handleDueDateChange(val: string | null) {
		localDueDate = val;
	}
</script>

{#if action}
	<Drawer bind:open flip={true} style="--drawer-width: 40vw">
		{#snippet title()}
			{action!.title}
		{/snippet}

		{#snippet body()}
			<div class="detail-form">
				{#if error}
					<Tag variant="danger">{error}</Tag>
				{/if}

				<Input
					id="action-title"
					label="Titel"
					bind:value={localTitle}
					disabled={saving}
				/>

				<div class="field-group">
					<label class="field-label" for="action-desc">Beschrijving</label>
					<textarea
						id="action-desc"
						class="field-textarea"
						bind:value={localDescription}
						rows="3"
						disabled={saving}
						placeholder="Voeg een beschrijving toe…"
					></textarea>
				</div>

				<div class="field-group status-group" onclick={(e) => {
					const btn = (e.target as HTMLElement).closest("[data-value]");
					if (btn) {
						const val = (btn as HTMLElement).dataset.value;
						if (val) displayStatus = val;
					}
				}}>
					<label class="field-label">Status</label>
					<ButtonGroup
						items={statusItems}
						bind:value={displayStatus}
						fluid={true}
					/>
				</div>

				<div class="field-group">
					<label class="field-label">Toegewezen aan</label>
					<Searchahead
						placeholder="Zoek docent…"
						options={teacherOptions}
						loading={teacherLoading}
						onquery={handleAssigneeSearch}
						onselect={handleAssigneeSelect}
						onremove={handleAssigneeRemove}
						initialSelected={assigneeSelected}
						resetKey={actionCounter}
						noFilter={true}
						maxWidth="100%"
					/>
				</div>

				<DatePicker
					label="Uiterste datum"
					bind:value={localDueDate}
					onchange={handleDueDateChange}
					disabled={saving}
					size="sm"
				/>

				{#if isOverdue}
					<Tag variant="danger">Vervallen</Tag>
				{/if}

				<div class="field-group">
					<label class="field-label">Tags</label>
					<TagInput bind:tags={localTags} />
				</div>

				{#if dataEntries.length > 0}
					<div class="field-group">
						<label class="field-label">Brongegevens</label>
						<div class="data-table">
							{#each dataEntries as [key, val]}
								<div class="data-row">
									<span class="data-key">{key}</span>
									<span class="data-val">{typeof val === "object" ? JSON.stringify(val, null, 2) : String(val)}</span>
								</div>
							{/each}
						</div>
					</div>
				{/if}

				<div class="field-meta">
				<Tag variant="default">Aangemaakt: {formatDate(action!.createdAt)}</Tag>
				<Tag variant="default">Bijgewerkt: {formatDate(action!.updatedAt)}</Tag>
					{#if action!.sourceType}
						<Tag variant="accent">Bron: {action!.sourceType}{#if action!.sourceId} ({action!.sourceId}){/if}</Tag>
					{/if}
				</div>
			</div>
		{/snippet}

		{#snippet footer()}
			<Button
				variant="primary"
				disabled={!isDirty || saving}
				onclick={handleSubmit}
			>
				{saving ? "Opslaan…" : "Opslaan"}
			</Button>
		{/snippet}
	</Drawer>
{/if}

<style>
	.detail-form {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.field-group {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.field-label {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.field-textarea {
		width: 100%;
		padding: var(--space-2) var(--space-3);
		border: 1px solid var(--color-border-strong);
		border-radius: var(--radius-xs);
		background: var(--color-surface);
		color: var(--color-text);
		font-family: var(--font-body);
		font-size: var(--text-sm);
		resize: vertical;
		min-height: 60px;
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.field-textarea:focus {
		outline: none;
		border-color: var(--color-accent);
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}

	.field-textarea::placeholder {
		color: var(--color-text-placeholder);
		font-size: var(--text-xs);
		font-weight: 300;
	}

	.data-table {
		background: var(--color-surface-sunken);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		overflow: hidden;
	}

	.data-row {
		display: grid;
		grid-template-columns: 1fr 2fr;
		gap: var(--space-2);
		padding: var(--space-1) var(--space-2);
		border-bottom: 1px solid var(--color-border);
		font-size: var(--text-xs);
	}

	.data-row:last-child {
		border-bottom: none;
	}

	.data-key {
		font-family: var(--font-mono);
		color: var(--color-text-tertiary);
		word-break: break-all;
	}

	.data-val {
		color: var(--color-text);
		white-space: pre-wrap;
		word-break: break-word;
	}

	.field-meta {
		margin-top: var(--space-2);
		padding-top: var(--space-3);
		border-top: 1px dashed var(--color-border);
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-2);
	}
</style>