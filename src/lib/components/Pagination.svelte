<script lang="ts">
	let {
		currentPage = 1,
		totalItems = 0,
		perPage = 20,
		onchange,
	}: {
		currentPage?: number;
		totalItems?: number;
		perPage?: number;
		onchange?: (page: number) => void;
	} = $props();

	let totalPages = $derived(Math.max(1, Math.ceil(totalItems / perPage)));
	let startItem = $derived((currentPage - 1) * perPage + 1);
	let endItem = $derived(Math.min(currentPage * perPage, totalItems));

	let pages = $derived.by(() => {
		if (totalPages <= 7) {
			return Array.from({ length: totalPages }, (_, i) => i + 1);
		}

		const p: (number | "ellipsis")[] = [];
		p.push(1);

		let start = Math.max(2, currentPage - 1);
		let end = Math.min(totalPages - 1, currentPage + 1);

		if (currentPage <= 3) {
			start = 2;
			end = Math.min(4, totalPages - 1);
		} else if (currentPage >= totalPages - 2) {
			start = Math.max(totalPages - 3, 2);
			end = totalPages - 1;
		}

		if (start > 2) p.push("ellipsis");
		for (let i = start; i <= end; i++) p.push(i);
		if (end < totalPages - 1) p.push("ellipsis");

		p.push(totalPages);
		return p;
	});

	function goTo(p: number) {
		if (p < 1 || p > totalPages || p === currentPage) return;
		onchange?.(p);
	}
</script>

{#if totalItems > 0 && totalPages > 1}
	<nav class="pagination" aria-label="Pagination">
		<div class="pagination-buttons">
			<button
				class="pagination-btn pagination-prev"
				disabled={currentPage <= 1}
				onclick={() => goTo(currentPage - 1)}
				aria-label="Previous page"
			>
				&lsaquo;
			</button>

			{#each pages as p}
				{#if p === "ellipsis"}
					<span class="pagination-ellipsis">&hellip;</span>
				{:else}
					<button
						class="pagination-btn {p === currentPage ? 'pagination-active' : ''}"
						onclick={() => goTo(p)}
						aria-label="Page {p}"
						aria-current={p === currentPage ? "page" : undefined}
					>
						{p}
					</button>
				{/if}
			{/each}

			<button
				class="pagination-btn pagination-next"
				disabled={currentPage >= totalPages}
				onclick={() => goTo(currentPage + 1)}
				aria-label="Next page"
			>
				&rsaquo;
			</button>
		</div>

		<span class="pagination-info">
			{startItem}–{endItem} of {totalItems}
		</span>
	</nav>
{/if}

<style>
	.pagination {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-3);
	}

	.pagination-buttons {
		display: flex;
		align-items: center;
		gap: var(--space-1);
	}

	.pagination-btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 32px;
		height: 32px;
		padding: 0 var(--space-2);
		font-family: var(--font-body);
		font-size: var(--text-xs);
		font-weight: 600;
		color: var(--color-text);
		background: transparent;
		border: 1px solid var(--color-border);
		border-radius: var(--radius-xs);
		cursor: pointer;
		transition:
			background var(--duration-fast) ease,
			color var(--duration-fast) ease,
			border-color var(--duration-fast) ease;
	}

	.pagination-btn:hover:not(:disabled):not(.pagination-active) {
		border-color: var(--color-accent);
		color: var(--color-accent);
	}

	.pagination-btn:focus-visible {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	.pagination-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.pagination-active {
		background: var(--color-accent);
		color: var(--color-text-on-accent);
		border-color: var(--color-accent);
	}

	.pagination-prev,
	.pagination-next {
		font-size: var(--text-sm);
		padding: 0 var(--space-1);
	}

	.pagination-ellipsis {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 32px;
		height: 32px;
		font-size: var(--text-sm);
		color: var(--color-text-tertiary);
		user-select: none;
	}

	.pagination-info {
		font-family: var(--font-body);
		font-size: var(--text-xs);
		color: var(--color-text-secondary);
	}
</style>
