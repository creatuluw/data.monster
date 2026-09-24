<script lang="ts">
	import { Pagination } from 'bits-ui';

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
</script>

{#if totalItems > 0 && totalPages > 1}
	<div class="pagination">
		<Pagination.Root
			count={totalItems}
			{perPage}
			page={currentPage}
			onPageChange={(p) => onchange?.(p)}
			aria-label="Pagination"
		>
			{#snippet children({ pages, range })}
				<div class="pagination-buttons">
					<Pagination.PrevButton class="pagination-btn pagination-prev" aria-label="Previous page">
						&lsaquo;
					</Pagination.PrevButton>

					{#each pages as pg (pg.key)}
						{#if pg.type === 'ellipsis'}
							<span class="pagination-ellipsis">&hellip;</span>
						{:else}
							<Pagination.Page
								page={pg}
								class="pagination-btn {pg.value === currentPage ? 'pagination-active' : ''}"
								aria-label="Page {pg.value}"
							>
								{pg.value}
							</Pagination.Page>
						{/if}
					{/each}

					<Pagination.NextButton class="pagination-btn pagination-next" aria-label="Next page">
						&rsaquo;
					</Pagination.NextButton>
				</div>

				<span class="pagination-info">
					{range.start}–{range.end} of {totalItems}
				</span>
			{/snippet}
		</Pagination.Root>
	</div>
{/if}

<style>
	.pagination {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-3);
	}

	/* bits-ui renders the buttons — scoped selectors reach them via :global() */
	.pagination :global(.pagination-buttons) {
		display: flex;
		align-items: center;
		gap: var(--space-1);
	}

	.pagination :global(.pagination-btn) {
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

	.pagination :global(.pagination-btn:hover:not(:disabled):not(.pagination-active)) {
		border-color: var(--color-accent);
		color: var(--color-accent);
	}

	.pagination :global(.pagination-btn:focus-visible) {
		outline: 2px solid var(--color-accent);
		outline-offset: 1px;
	}

	.pagination :global(.pagination-btn:disabled) {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.pagination :global(.pagination-active) {
		background: var(--color-accent);
		color: var(--color-text-on-accent);
		border-color: var(--color-accent);
	}

	.pagination :global(.pagination-prev),
	.pagination :global(.pagination-next) {
		font-size: var(--text-sm);
		padding: 0 var(--space-1);
	}

	.pagination :global(.pagination-ellipsis) {
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
