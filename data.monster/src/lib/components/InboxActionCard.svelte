<script lang="ts">
	import { Clock, AlertCircle, PanelRightOpen, ChevronDown } from "lucide-svelte";
	import { Tag } from "$lib/components";
	import { formatDate } from "$lib/utils/formatDate";

	interface InboxActionRow {
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
		snoozedUntil: string | null;
		refId: string | null;
		archived: boolean;
		tags: string[];
		data: Record<string, any>;
	}

	let {
		data,
		onclick,
		ondrawer,
		compact = false,
	}: {
		data: InboxActionRow;
		onclick?: (id: string) => void;
		ondrawer?: (id: string) => void;
		compact?: boolean;
	} = $props();

	let expanded = $state(false);

	const statusConfig: Record<string, { label: string; variant: "default" | "accent" | "success" | "warning" | "danger" }> = {
		backlog: { label: "Backlog", variant: "accent" },
		todo: { label: "Todo", variant: "default" },
		in_progress: { label: "In uitvoering", variant: "warning" },
		in_review: { label: "In review", variant: "default" },
		done: { label: "Klaar", variant: "success" },
		dismissed: { label: "Afgewezen", variant: "danger" },
	};

	let config = $derived(statusConfig[data.actionStatus] ?? { label: data.actionStatus, variant: "default" as const });

	let isOverdue = $derived(
		data.dueDate !== null && new Date(data.dueDate) < new Date(),
	);

	let dueDateFormatted = $derived.by(() => {
		if (!data.dueDate) return null;
		return formatDate(data.dueDate);
	});
</script>

<div
	class="action-card"
	class:compact
	class:overdue={isOverdue}
	class:expanded={compact && expanded}
	tabindex="0"
	role="button"
	aria-label="{data.title} — {config.label}"
	aria-expanded={compact ? expanded : undefined}
	onclick={() => {
		if (compact) {
			expanded = !expanded;
		} else {
			onclick?.(data.id);
		}
	}}
	onkeydown={(e) => { if (e.key === "Enter" || e.key === " ") { e.preventDefault(); if (compact) { expanded = !expanded; } else { onclick?.(data.id); } } }}
>
	<div class="action-card-header">
		<span class="action-title">{data.title}</span>
		{#if compact && !expanded}
			<button class="accordion-toggle" aria-label="Uitklappen">
				<ChevronDown size={14} />
			</button>
		{:else}
			<button
				class="drawer-open-btn"
				aria-label="Details openen"
				onclick={(e) => { e.stopPropagation(); ondrawer?.(data.id); }}
			>
				<PanelRightOpen size={14} />
			</button>
		{/if}
	</div>

	{#if !compact || expanded}
		<div class="action-card-body">
			<div class="action-card-meta">
				{#if dueDateFormatted}
					<span class="meta-item" class:overdue-text={isOverdue}>
						{#if isOverdue}
							<AlertCircle size={12} />
						{:else}
							<Clock size={12} />
						{/if}
						{dueDateFormatted}
					</span>
				{/if}
			</div>

			{#if data.description}
				<p class="action-desc">{data.description}</p>
			{/if}
		</div>

		<div class="action-card-footer">
			<div class="action-tags">
				{#if data.tags && data.tags.length > 0}
					{#each data.tags.slice(0, 3) as tag}
						<span class="action-tag-chip">{tag}</span>
					{/each}
				{/if}
			</div>
			<Tag variant={config.variant}>{config.label}</Tag>
		</div>
	{/if}
</div>

<style>
	.action-card {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		padding: var(--space-3) var(--space-4);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-lg);
		background: var(--color-background);
		cursor: pointer;
		font-family: var(--font-mono);
		transition: border-color var(--duration-fast) ease, box-shadow var(--duration-fast) ease;
	}

	.action-card:not(.compact) {
		height: 100%;
		min-height: max(100px, 10vw);
	}

	.action-card.compact {
		padding: var(--space-2) var(--space-3);
	}

	.action-card.compact .action-card-body {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		padding-top: var(--space-2);
		border-top: 1px solid var(--color-border);
		margin-top: var(--space-1);
	}

	.action-card:hover {
		border-color: var(--color-accent);
		box-shadow: var(--shadow-sm);
	}

	.action-card:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 2px;
	}

	.action-card.overdue {
		border-left: 3px solid oklch(0.55 0.2 22);
	}

	.action-card-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: var(--space-2);
	}

	.action-title {
		font-family: var(--font-mono);
		font-weight: 400;
		font-size: var(--text-sm);
		color: var(--color-accent-dark);
		line-height: 1.4;
	}

	.action-card-meta {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		flex-wrap: wrap;
	}

	.meta-item {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		font-size: var(--text-xs);
		color: var(--color-text-tertiary);
	}

	.meta-item.overdue-text {
		color: oklch(0.45 0.15 22);
		font-weight: 600;
	}

	.action-card-footer {
		margin-top: auto;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: var(--space-2);
		border-top: 1px dotted var(--color-border);
		padding-top: var(--space-2);
	}

	.action-tags {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-1);
	}

	.action-desc {
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
		line-height: 1.5;
		margin: 0;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.action-tag-chip {
		display: inline-flex;
		align-items: center;
		padding: 2px var(--space-2);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		background: var(--color-surface);
		color: var(--color-accent);
		font-family: var(--font-body);
		font-size: 9px;
		font-weight: 600;
		line-height: 1.5;
	}

	.drawer-open-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		width: 24px;
		height: 24px;
		padding: 0;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		background: var(--color-surface);
		color: var(--color-text-tertiary);
		cursor: pointer;
		transition: color var(--duration-fast) ease, border-color var(--duration-fast) ease, background var(--duration-fast) ease;
	}

	.drawer-open-btn:hover {
		color: var(--color-accent);
		border-color: var(--color-accent);
		background: var(--color-accent-muted);
	}

	.accordion-toggle {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		width: 20px;
		height: 20px;
		padding: 0;
		border: none;
		border-radius: var(--radius-xs);
		background: transparent;
		color: var(--color-text-tertiary);
		cursor: pointer;
		transition: transform var(--duration-fast) ease, color var(--duration-fast) ease;
	}

	.action-card.expanded .accordion-toggle {
		transform: rotate(180deg);
	}

	.accordion-toggle:hover {
		color: var(--color-accent);
	}
</style>