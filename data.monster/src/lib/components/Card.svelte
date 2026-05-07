<script lang="ts">
	import Tag from "./Tag.svelte";

	let {
		variant = "default",
		id: cardId,
		title,
		body,
		footer,
		children,
		name,
		group,
		role,
		status,
		email,
		progress,
	}: {
		variant?: "default" | "bare" | "student";
		id?: string;
		title?: import("svelte").Snippet;
		body?: import("svelte").Snippet;
		footer?: import("svelte").Snippet;
		children?: import("svelte").Snippet;
		name?: string;
		group?: string;
		role?: string;
		status?: string;
		email?: string;
		progress?: string;
	} = $props();

	let initials = $derived(
		name
			?.split(" ")
			.map((w) => w[0])
			.join("")
			.slice(0, 2)
			.toUpperCase() ?? "?"
	);

	let statusVariant = $derived(
		status === "active"
			? "success" as const
			: status === "at risk"
				? "warning" as const
				: status === "inactive"
					? "danger" as const
					: "default" as const
	);
</script>

{#if variant === "student"}
	<article class="card card-student">
		<div class="student-top">
			<span class="student-avatar">{initials}</span>
			<div class="student-meta">
				<span class="student-name">{name}</span>
				<span class="student-group">{group}</span>
			</div>
		</div>
		<div class="student-details">
			{#if role}<span class="student-detail"><span class="student-label">Rol</span>{role}</span>{/if}
			{#if email}<span class="student-detail"><span class="student-label">E-mail</span>{email}</span>{/if}
			{#if progress}<span class="student-detail"><span class="student-label">Voortgang</span>{progress}</span>{/if}
		</div>
		<div class="student-footer">
			{#if status}<Tag variant={statusVariant}>{status}</Tag>{/if}
			{#if children}{@render children()}{/if}
		</div>
	</article>
{:else}
	<article class="card" class:card-bare={variant === "bare"}>
		{#if cardId}
			<span class="card-id">{cardId}</span>
		{/if}
		{#if title}
			<h4 class="card-title">
				{@render title()}
			</h4>
		{/if}
		{#if body}
			<p class="card-body">
				{@render body()}
			</p>
		{/if}
		{#if children}
			{@render children()}
		{/if}
		{#if footer}
			<div class="card-footer">
				{@render footer()}
			</div>
		{/if}
	</article>
{/if}

<style>
	.card {
		background: var(--color-surface);
		padding: var(--space-8);
		position: relative;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
	}

	.card-bare {
		padding: var(--space-6);
	}

	.card-id {
		font-family: var(--font-mono);
		font-size: 9px;
		letter-spacing: 0.1em;
		color: var(--color-text-tertiary);
		margin-bottom: var(--space-4);
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.card-id::before {
		content: "";
		width: 4px;
		height: 4px;
		background: var(--color-accent);
	}

	.card-title {
		font-family: var(--font-display);
		font-size: var(--text-lg);
		font-weight: 700;
		letter-spacing: -0.01em;
		color: var(--color-text);
		margin-bottom: var(--space-3);
	}

	.card-body {
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		line-height: var(--leading-relaxed);
		color: var(--color-text-secondary);
		max-width: 45ch;
	}

	.card-footer {
		margin-top: var(--space-6);
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding-top: var(--space-4);
		border-top: 1px dashed var(--color-border);
	}

	/* ── Student variant ── */
	.card-student {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-sm);
		padding: var(--space-6);
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}

	.student-top {
		display: flex;
		align-items: center;
		gap: var(--space-4);
	}

	.student-avatar {
		width: 40px;
		height: 40px;
		border-radius: var(--radius-full);
		background: var(--color-surface-sunken);
		border: 1px solid var(--color-border);
		display: flex;
		align-items: center;
		justify-content: center;
		font-family: var(--font-display);
		font-size: var(--text-sm);
		font-weight: 700;
		color: var(--color-text-secondary);
		flex-shrink: 0;
	}

	.student-meta {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}

	.student-name {
		font-family: var(--font-display);
		font-size: var(--text-base);
		font-weight: 700;
		color: var(--color-text);
	}

	.student-group {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		letter-spacing: 0.06em;
		color: var(--color-text-tertiary);
		text-transform: uppercase;
	}

	.student-details {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}

	.student-detail {
		font-family: var(--font-mono);
		font-size: var(--text-sm);
		color: var(--color-text-secondary);
	}

	.student-label {
		font-family: var(--font-mono);
		font-size: var(--text-xs);
		letter-spacing: 0.06em;
		text-transform: uppercase;
		color: var(--color-text-tertiary);
		margin-right: var(--space-3);
	}

	.student-footer {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding-top: var(--space-3);
		border-top: 1px dashed var(--color-border);
	}
</style>
