<script lang="ts">
	import { Plus, ChevronRight, SquarePen, Trash2 } from "lucide-svelte";
	import Drawer from "./Drawer.svelte";
	import Modal from "./Modal.svelte";
	import Input from "./Input.svelte";
	import Button from "./Button.svelte";
	import { toastSuccess, toastError } from "$lib/stores/toasts.svelte";

	interface Step {
		id: string;
		number: string;
		definition: string;
		criteria: string;
		sortOrder: number;
	}

	interface Topic {
		id: string;
		name: string;
		description?: string;
		steps: Step[];
	}

	interface Props {
		subjectId: string;
		subjectName?: string;
		topics: Topic[];
		canCreate?: boolean;
		scrollTarget?: string | null;
	}

	let { subjectId, subjectName = "", topics = [], canCreate = true, scrollTarget = null }: Props = $props();

	let expandedTopics = $state<Set<string>>(new Set());

	$effect(() => {
		if (!scrollTarget || topics.length === 0) return;
		if (scrollTarget.startsWith("topic-")) {
			const topicId = scrollTarget.replace("topic-", "");
			if (!expandedTopics.has(topicId)) {
				expandedTopics = new Set([...expandedTopics, topicId]);
			}
			requestAnimationFrame(() => {
				const el = document.getElementById(scrollTarget);
				if (el) {
					el.scrollIntoView({ behavior: "smooth", block: "center" });
					el.style.outline = "2px solid var(--color-accent)";
					el.style.outlineOffset = "2px";
					setTimeout(() => { el.style.outline = ""; el.style.outlineOffset = ""; }, 2000);
				}
			});
		} else if (scrollTarget.startsWith("step-")) {
			const stepId = scrollTarget.replace("step-", "");
			const parentTopic = topics.find((t) => t.steps?.some((s) => s.id === stepId));
			if (parentTopic && !expandedTopics.has(parentTopic.id)) {
				expandedTopics = new Set([...expandedTopics, parentTopic.id]);
			}
			requestAnimationFrame(() => {
				setTimeout(() => {
					const el = document.getElementById(scrollTarget);
					if (el) {
						el.scrollIntoView({ behavior: "smooth", block: "center" });
						(el as HTMLElement).style.outline = "2px solid var(--color-accent)";
						(el as HTMLElement).style.outlineOffset = "2px";
						setTimeout(() => { (el as HTMLElement).style.outline = ""; (el as HTMLElement).style.outlineOffset = ""; }, 2000);
					}
				}, 100);
			});
		}
	});

	let drawerOpen = $state(false);
	let drawerMode = $state<"create" | "edit">("create");
	let targetType = $state<"topic" | "step">("topic");
	let targetId = $state<string | null>(null);
	let targetTopicId = $state<string | null>(null);
	let saving = $state(false);

	let formName = $state("");
	let formDesc = $state("");
	let formNumber = $state("");
	let formDefinition = $state("");
	let formCriteria = $state("");

	let snapshot = $state({ name: "", desc: "", number: "", definition: "", criteria: "" });

	function takeSnapshot() {
		snapshot = { name: formName, desc: formDesc, number: formNumber, definition: formDefinition, criteria: formCriteria };
	}

	const isDirty = $derived(
		formName !== snapshot.name ||
		formDesc !== snapshot.desc ||
		formNumber !== snapshot.number ||
		formDefinition !== snapshot.definition ||
		formCriteria !== snapshot.criteria
	);

	let deleteModalOpen = $state(false);
	let deleteTargetType = $state<"topic" | "step">("topic");
	let deleteTargetId = $state<string | null>(null);
	let deleteTargetLabel = $state("");
	let deleteTargetParentId = $state<string | null>(null);
	let deleting = $state(false);

	function openDeleteTopic(topic: Topic) {
		deleteTargetType = "topic";
		deleteTargetId = topic.id;
		deleteTargetLabel = topic.name;
		deleteTargetParentId = null;
		deleteModalOpen = true;
	}

	function openDeleteStep(step: Step, topicId: string) {
		deleteTargetType = "step";
		deleteTargetId = step.id;
		deleteTargetLabel = `${step.number} — ${step.definition}`;
		deleteTargetParentId = topicId;
		deleteModalOpen = true;
	}

	async function handleDelete() {
		if (!deleteTargetId) return;
		deleting = true;
		try {
			const endpoint = deleteTargetType === "topic"
				? `/api/topics/${deleteTargetId}`
				: `/api/steps/${deleteTargetId}`;
			const res = await fetch(endpoint, { method: "DELETE" });
			if (res.ok) {
				if (deleteTargetType === "topic") {
					topics = topics.filter((t) => t.id !== deleteTargetId);
					expandedTopics = new Set([...expandedTopics].filter((id) => id !== deleteTargetId));
					toastSuccess("Onderwerp verwijderd");
				} else {
					topics = topics.map((t) =>
						t.id === deleteTargetParentId
							? { ...t, steps: t.steps.filter((s) => s.id !== deleteTargetId) }
							: t,
					);
					toastSuccess("Onderdeel verwijderd");
				}
				deleteModalOpen = false;
			} else {
				const err = await res.json();
				toastError("Fout", err.error ?? "Kon niet verwijderen.");
			}
		} catch {
			toastError("Fout", "Kan geen verbinding maken.");
		} finally {
			deleting = false;
		}
	}

	function toggleTopic(topicId: string) {
		const next = new Set(expandedTopics);
		if (next.has(topicId)) {
			next.delete(topicId);
		} else {
			next.add(topicId);
		}
		expandedTopics = next;
	}

	function openCreateTopic() {
		drawerMode = "create";
		targetType = "topic";
		targetId = null;
		targetTopicId = null;
		formName = "";
		formDesc = "";
		formNumber = "";
		formDefinition = "";
		formCriteria = "";
		takeSnapshot();
		drawerOpen = true;
	}

	function openCreateStep(topicId: string) {
		drawerMode = "create";
		targetType = "step";
		targetId = null;
		targetTopicId = topicId;
		formName = "";
		formDesc = "";
		formNumber = "";
		formDefinition = "";
		formCriteria = "";
		takeSnapshot();
		drawerOpen = true;
	}

	function openEditTopic(topic: Topic) {
		drawerMode = "edit";
		targetType = "topic";
		targetId = topic.id;
		targetTopicId = null;
		formName = topic.name;
		formDesc = topic.description ?? "";
		formNumber = "";
		formDefinition = "";
		formCriteria = "";
		takeSnapshot();
		drawerOpen = true;
	}

	function openEditStep(step: Step, topicId: string) {
		drawerMode = "edit";
		targetType = "step";
		targetId = step.id;
		targetTopicId = topicId;
		formName = "";
		formDesc = "";
		formNumber = step.number;
		formDefinition = step.definition;
		formCriteria = step.criteria;
		takeSnapshot();
		drawerOpen = true;
	}

	async function handleSave() {
		if (drawerMode === "create") {
			await handleCreate();
		} else {
			await handleUpdate();
		}
	}

	async function handleCreate() {
		if (targetType === "topic") {
			if (!formName.trim()) {
				toastError("Fout", "Naam is vereist.");
				return;
			}
			saving = true;
			try {
				const res = await fetch(`/api/subjects/${subjectId}/topics`, {
					method: "POST",
					headers: { "Content-Type": "application/json" },
					body: JSON.stringify({ name: formName.trim(), description: formDesc.trim() || undefined }),
				});
				if (res.ok) {
					const created = await res.json();
					topics = [...topics, { ...created, steps: [] }];
					drawerOpen = false;
					toastSuccess("Onderwerp aangemaakt");
				} else {
					const err = await res.json();
					toastError("Fout", err.error ?? "Kon onderwerp niet aanmaken.");
				}
			} catch {
				toastError("Fout", "Kan geen verbinding maken.");
			} finally {
				saving = false;
			}
		} else if (targetType === "step" && targetTopicId) {
			if (!formNumber.trim() || !formDefinition.trim() || !formCriteria.trim()) {
				toastError("Fout", "Alle velden zijn vereist.");
				return;
			}
			saving = true;
			try {
				const res = await fetch(`/api/topics/${targetTopicId}/steps`, {
					method: "POST",
					headers: { "Content-Type": "application/json" },
					body: JSON.stringify({
						number: formNumber.trim(),
						definition: formDefinition.trim(),
						criteria: formCriteria.trim(),
					}),
				});
				if (res.ok) {
					const created = await res.json();
					const nextExpanded = new Set(expandedTopics);
					nextExpanded.add(targetTopicId!);
					expandedTopics = nextExpanded;
					topics = topics.map((t) =>
						t.id === targetTopicId ? { ...t, steps: [...t.steps, created] } : t,
					);
					drawerOpen = false;
					toastSuccess("Onderdeel aangemaakt");
				} else {
					const err = await res.json();
					toastError("Fout", err.error ?? "Kon onderdeel niet aanmaken.");
				}
			} catch {
				toastError("Fout", "Kan geen verbinding maken.");
			} finally {
				saving = false;
			}
		}
	}

	async function handleUpdate() {
		if (targetType === "topic" && targetId) {
			if (!formName.trim()) {
				toastError("Fout", "Naam is vereist.");
				return;
			}
			saving = true;
			try {
				const res = await fetch(`/api/topics/${targetId}`, {
					method: "PATCH",
					headers: { "Content-Type": "application/json" },
					body: JSON.stringify({ name: formName.trim(), description: formDesc.trim() }),
				});
				if (res.ok) {
					const updated = await res.json();
					topics = topics.map((t) =>
						t.id === targetId ? { ...t, name: updated.name, description: updated.description } : t,
					);
					drawerOpen = false;
					toastSuccess("Onderwerp bijgewerkt");
				} else {
					const err = await res.json();
					toastError("Fout", err.error ?? "Kon onderwerp niet bijwerken.");
				}
			} catch {
				toastError("Fout", "Kan geen verbinding maken.");
			} finally {
				saving = false;
			}
		} else if (targetType === "step" && targetId) {
			if (!formNumber.trim() || !formDefinition.trim() || !formCriteria.trim()) {
				toastError("Fout", "Alle velden zijn vereist.");
				return;
			}
			saving = true;
			try {
				const res = await fetch(`/api/steps/${targetId}`, {
					method: "PATCH",
					headers: { "Content-Type": "application/json" },
					body: JSON.stringify({
						number: formNumber.trim(),
						definition: formDefinition.trim(),
						criteria: formCriteria.trim(),
					}),
				});
				if (res.ok) {
					const updated = await res.json();
					topics = topics.map((t) =>
						t.id === targetTopicId
							? { ...t, steps: t.steps.map((s) => s.id === targetId ? { ...s, ...updated } : s) }
							: t,
					);
					drawerOpen = false;
					toastSuccess("Onderdeel bijgewerkt");
				} else {
					const err = await res.json();
					toastError("Fout", err.error ?? "Kon onderdeel niet bijwerken.");
				}
			} catch {
				toastError("Fout", "Kan geen verbinding maken.");
			} finally {
				saving = false;
			}
		}
	}

	const drawerTitle = $derived.by(() => {
		if (drawerMode === "create") {
			return targetType === "topic" ? "Nieuw onderwerp" : "Nieuw onderdeel";
		}
		return targetType === "topic" ? "Onderwerp bewerken" : "Onderdeel bewerken";
	});

	const contextLabel = $derived.by(() => {
		if (targetType === "step" && targetTopicId) {
			const topic = topics.find((t) => t.id === targetTopicId);
			return topic ? `${subjectName}${subjectName ? " / " : ""}${topic.name}` : subjectName;
		}
		return subjectName;
	});

	const saveLabel = $derived(saving
		? (drawerMode === "create" ? "Aanmaken..." : "Opslaan...")
    	: (drawerMode === "create" ? "Aanmaken" : "Opslaan")
	);
</script>

<div class="wtree">
	{#if topics.length === 0}
		{#if canCreate}
		<ul class="wtree-root">
			<li class="wtree-last wtree-add-item">
				<button class="wtree-add-root" onclick={openCreateTopic} title="Onderwerp toevoegen">
					<Plus size={14} />
					<span>Onderwerp toevoegen</span>
				</button>
			</li>
		</ul>
		{/if}
	{:else}
		<ul class="wtree-root">
			{#each topics as topic, i (topic.id)}
				{@const isExpanded = expandedTopics.has(topic.id)}
				{@const hasChildren = topic.steps && topic.steps.length > 0}
				<li class:wtree-last={i === topics.length - 1 && !canCreate}>
					<button
						id="topic-{topic.id}"
						class="wtree-node wtree-node-topic"
						class:wtree-node-expanded={isExpanded}
						onclick={() => toggleTopic(topic.id)}
					>
						<span class="node-chevron" class:node-chevron-open={isExpanded}>
							<ChevronRight size={14} />
						</span>
						<span class="node-label">{topic.name}</span>
						{#if hasChildren}
							<span class="node-count">{topic.steps.length}</span>
						{/if}
						{#if canCreate}
							<button class="node-action" onclick={(e) => { e.stopPropagation(); openEditTopic(topic); }} title="Bewerken">
								<SquarePen size={12} />
							</button>
							<button class="node-action delete-action" onclick={(e) => { e.stopPropagation(); openDeleteTopic(topic); }} title="Verwijderen">
								<Trash2 size={12} />
							</button>
							<button class="node-action" onclick={(e) => { e.stopPropagation(); openCreateStep(topic.id); }} title="Onderdeel toevoegen">
								<Plus size={12} />
							</button>
						{/if}
					</button>
					{#if isExpanded}
						<ul>
							{#each topic.steps as step, j (topic.id + '-' + step.id)}
								<li class:wtree-last={false}>
									<span id="step-{step.id}" class="wtree-node wtree-node-step">
										<span class="step-number">{step.number}</span>
										<span class="step-definition">{step.definition}</span>
										{#if canCreate}
											<button class="node-action" onclick={() => openEditStep(step, topic.id)} title="Bewerken">
												<SquarePen size={11} />
											</button>
											<button class="node-action delete-action" onclick={() => openDeleteStep(step, topic.id)} title="Verwijderen">
												<Trash2 size={11} />
											</button>
										{/if}
									</span>
								</li>
							{/each}
							{#if canCreate}
								<li class="wtree-last wtree-add-item">
									<button class="wtree-add-root" onclick={() => openCreateStep(topic.id)} title="Onderdeel toevoegen">
										<Plus size={12} />
										<span>Onderdeel toevoegen</span>
									</button>
								</li>
							{/if}
						</ul>
					{/if}
				</li>
			{/each}
			{#if canCreate}
				<li class="wtree-last wtree-add-item">
					<button class="wtree-add-root" onclick={openCreateTopic} title="Onderwerp toevoegen">
						<Plus size={14} />
						<span>Onderwerp toevoegen</span>
					</button>
				</li>
			{/if}
		</ul>
	{/if}
</div>

<Drawer bind:open={drawerOpen} mode="push" flip overlay>
	{#snippet title()}{drawerTitle}{/snippet}
	{#snippet body()}
		{#if contextLabel}
			<div class="drawer-context">
				<span class="drawer-context-label">{drawerMode === "create" ? "Toevoegen aan" : "Onderdeel van"}</span>
				<span class="drawer-context-path">{contextLabel}</span>
			</div>
		{/if}
		<div class="drawer-form">
			{#if targetType === "topic"}
				<Input
					id="form-name"
					label="Naam"
					placeholder="Bijv. Hoofdrekenen"
					required
					bind:value={formName}
				/>
				<div class="field">
					<label class="field-label" for="form-desc">Omschrijving</label>
					<textarea
						id="form-desc"
						class="textarea"
						placeholder="Optionele beschrijving..."
						bind:value={formDesc}
						rows="3"
					></textarea>
				</div>
			{:else}
				<Input
					id="form-number"
					label="Volgnummer"
					placeholder="Bijv. 1.1"
					required
					bind:value={formNumber}
				/>
				<Input
					id="form-definition"
					label="Definitie"
					placeholder="Beschrijf de stap..."
					required
					bind:value={formDefinition}
				/>
				<div class="field">
					<label class="field-label" for="form-criteria">Criteria</label>
					<textarea
						id="form-criteria"
						class="textarea"
						placeholder="Wat moet de leerling kunnen?"
						bind:value={formCriteria}
						rows="3"
					></textarea>
				</div>
			{/if}
		</div>
	{/snippet}
	{#snippet footer()}
		<Button variant="primary" disabled={saving || (drawerMode === "edit" && !isDirty)} onclick={handleSave}>
			{saveLabel}
		</Button>
		<Button variant="secondary" onclick={() => (drawerOpen = false)}>Annuleren</Button>
	{/snippet}
</Drawer>

<Modal bind:open={deleteModalOpen}>
	{#snippet title()}Verwijderen{/snippet}
	{#snippet body()}
		<p class="confirm-text">
			Weet je zeker dat je <strong>{deleteTargetLabel}</strong> wilt verwijderen?
			{#if deleteTargetType === "topic"}
				Dit verwijdert ook alle onderdelen binnen dit onderwerp.
			{/if}
			Dit kan niet ongedaan worden gemaakt.
		</p>
	{/snippet}
	{#snippet footer()}
		<Button variant="danger" disabled={deleting} onclick={handleDelete}>
			{deleting ? "Verwijderen..." : "Verwijderen"}
		</Button>
		<Button variant="secondary" onclick={() => (deleteModalOpen = false)}>Annuleren</Button>
	{/snippet}
</Modal>

<style>
	.wtree {
		padding: var(--space-4) 0;
	}

	:global(.wtree-root),
	.wtree ul {
		list-style: none;
		margin: 0;
		padding: 0;
		margin-left: 28px;
	}

	.wtree-root {
		margin-left: 0;
	}

	.wtree li {
		list-style-type: none;
		margin: var(--space-2) 0 var(--space-2) var(--space-3);
		position: relative;
	}

	.wtree li::before {
		content: "";
		position: absolute;
		top: -10px;
		left: -28px;
		border-left: 1px solid var(--color-border-strong);
		border-bottom: 1px solid var(--color-border-strong);
		width: 28px;
		height: 15px;
		border-bottom-left-radius: var(--radius-sm);
	}

	.wtree li::after {
		position: absolute;
		content: "";
		top: 5px;
		left: -28px;
		border-left: 1px solid var(--color-border-strong);
		width: 28px;
		height: calc(100% - 5px);
	}

	.wtree li.wtree-last::after {
		display: none;
	}

	.wtree-node {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		border: 1px solid var(--color-border);
		padding: var(--space-2) var(--space-3);
		border-radius: var(--radius-sm);
		transition:
			border-color var(--duration-fast) ease,
			background var(--duration-fast) ease;
	}

	.wtree-node::before {
		content: none;
	}

	.wtree-node-topic {
		background: oklch(0.97 0.008 250);
		cursor: pointer;
		width: 100%;
		text-align: left;
		font-family: inherit;
		font-size: inherit;
	}

	.wtree-node-expanded {
		border-color: var(--color-border-strong);
		background: oklch(0.955 0.01 250);
	}

	.wtree-node-step {
		background: oklch(0.965 0.005 250);
	}

	.wtree-node:hover {
		border-color: var(--color-border-strong);
	}

	.node-label {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
		flex: 1;
		min-width: 0;
	}

	.node-chevron {
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-text-tertiary);
		flex-shrink: 0;
		transition: transform var(--duration-fast) var(--ease-pop);
	}

	.node-chevron-open {
		transform: rotate(90deg);
	}

	.node-count {
		font-family: var(--font-mono);
		font-size: 9px;
		color: var(--color-text-tertiary);
		background: var(--color-surface-sunken);
		padding: 1px 6px;
		border-radius: var(--radius-full);
		border: 1px solid var(--color-border);
		flex-shrink: 0;
	}

	.node-action {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 22px;
		height: 22px;
		background: none;
		border: 1px solid transparent;
		border-radius: var(--radius-xs);
		cursor: pointer;
		flex-shrink: 0;
		opacity: 0;
		transition:
			opacity var(--duration-fast) ease,
			background var(--duration-fast) ease,
			border-color var(--duration-fast) ease;
	}

	.node-action.edit-action {
		color: var(--color-text-tertiary);
	}

	.node-action.add-action {
		color: oklch(0.72 0.17 45);
	}

	.node-action.delete-action {
		color: oklch(0.65 0.2 25);
	}

	.node-action.delete-action:hover {
		background: oklch(0.95 0.03 25);
		border-color: oklch(0.75 0.15 25);
	}

	.wtree-node:hover .node-action,
	.node-action:focus-visible {
		opacity: 1;
	}

	.node-action:hover {
		background: var(--color-surface-sunken);
		border-color: var(--color-border);
	}

	.node-action:focus-visible {
		opacity: 1;
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	.step-number {
		font-family: var(--font-mono);
		font-size: 10px;
		color: var(--color-accent);
		background: var(--color-accent-muted);
		padding: 1px 4px;
		border-radius: var(--radius-xs);
		border: 1px solid var(--color-accent-muted);
		flex-shrink: 0;
	}

	.step-definition {
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		flex: 1;
		min-width: 0;
	}

	.wtree-add-root {
		display: inline-flex;
		align-items: center;
		gap: var(--space-2);
		background: none;
		border: 1px dashed var(--color-border);
		border-radius: var(--radius-sm);
		padding: var(--space-2) var(--space-3);
		cursor: pointer;
		font-family: var(--font-body);
		font-size: var(--text-xs);
		color: oklch(0.72 0.17 45);
		transition:
			border-color var(--duration-fast) ease,
			background var(--duration-fast) ease;
	}

	.wtree-add-root:hover {
		border-color: oklch(0.72 0.17 45);
		background: oklch(0.97 0.03 45);
	}

	.wtree-add-item::before,
	.wtree-add-item::after {
		border-left-style: dashed !important;
		border-bottom-style: dashed !important;
	}

	.drawer-context {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
		padding: var(--space-3) var(--space-4);
		background: var(--color-surface-sunken);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		margin-bottom: var(--space-4);
	}

	.drawer-context-label {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.08em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
	}

	.drawer-context-path {
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 600;
		color: var(--color-text);
	}

	.drawer-form {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.textarea {
		padding: var(--space-2) var(--space-3);
		border: 1px solid var(--color-border-strong);
		color: var(--color-text);
		background: var(--color-surface);
		border-radius: var(--radius-xs);
		resize: vertical;
		outline: none;
		transition:
			border-color var(--duration-fast) ease,
			box-shadow var(--duration-fast) ease;
	}

	.textarea:focus {
		border-color: var(--color-accent);
		box-shadow: 0 0 0 2px var(--color-accent-muted);
	}

	.confirm-text {
		font-size: var(--text-sm);
		color: var(--color-text-secondary);
		line-height: var(--leading-relaxed);
	}
</style>
