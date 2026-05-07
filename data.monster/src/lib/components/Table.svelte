<script lang="ts">
  import { ChevronDown } from "lucide-svelte";
  import Modal from "./Modal.svelte";
  import Drawer from "./Drawer.svelte";
  import Spinner from "./Spinner.svelte";

  type Column = {
    key: string;
    label: string;
    render?: import("svelte").Snippet<[any]>;
  };

  type RowAction = "none" | "accordion" | "modal" | "drawer";

  let {
    columns,
    rows,
    hover = false,
    rowAction = "none",
    detail,
    drawerMode = "slide",
    drawerFlip = false,
    drawerOverlay = true,
    loading = false,
    emptyMessage = "No data",
    ref,
    onrowclick,
  }: {
    columns: Column[];
    rows: Record<string, any>[];
    hover?: boolean;
    rowAction?: RowAction;
    detail?: import("svelte").Snippet<[any]>;
    drawerMode?: "slide" | "push" | "reveal" | "none";
    drawerFlip?: boolean;
    drawerOverlay?: boolean;
    loading?: boolean;
    emptyMessage?: string;
    ref?: string;
    onrowclick?: (row: Record<string, any>, index: number) => void;
  } = $props();

  let expandedIndex = $state<number | null>(null);
  let modalOpen = $state(false);
  let drawerOpen = $state(false);
  let activeRow = $state<Record<string, any> | null>(null);

  function handleRowClick(index: number) {
    const row = rows[index];
    onrowclick?.(row, index);
    if (rowAction === "none") return;
    if (rowAction === "accordion") {
      expandedIndex = expandedIndex === index ? null : index;
    } else if (rowAction === "modal") {
      activeRow = row;
      modalOpen = true;
    } else if (rowAction === "drawer") {
      activeRow = row;
      drawerOpen = true;
    }
  }

  let hasDetail = $derived(detail !== undefined);
</script>

<div class="table-wrap">
  {#if ref}
    <span class="table-ref">{ref}</span>
  {/if}
  {#if loading}
    <Spinner centered />
  {:else if rows.length === 0}
    <div class="table-empty">{emptyMessage}</div>
  {:else}
    <table
      class="data-table"
      class:data-table-hover={hover}
      class:data-table-interactive={rowAction !== "none" || !!onrowclick}
    >
      <thead>
        <tr>
          {#each columns as col}
            <th>{col.label}</th>
          {/each}
          {#if rowAction === "accordion"}
            <th class="th-icon"></th>
          {/if}
        </tr>
      </thead>
      <tbody>
        {#each rows as row, i}
          <tr
            class:tr-expanded={rowAction === "accordion" && expandedIndex === i}
            onclick={() => handleRowClick(i)}
            role={rowAction !== "none" || onrowclick ? "button" : undefined}
            tabindex={rowAction !== "none" || onrowclick ? 0 : undefined}
            onkeydown={(e) => {
              if (e.key === "Enter" || e.key === " ") {
                e.preventDefault();
                handleRowClick(i);
              }
            }}
          >
            {#each columns as col}
              <td>
                {#if col.render}
                  {@render col.render(row)}
                {:else}
                  {row[col.key] ?? ""}
                {/if}
              </td>
            {/each}
            {#if rowAction === "accordion"}
              <td class="td-icon">
                <span
                  class="accordion-icon"
                  aria-hidden="true"
                  class:icon-expanded={expandedIndex === i}
                >
                  <ChevronDown size={14} />
                </span>
              </td>
            {/if}
          </tr>
          {#if rowAction === "accordion" && expandedIndex === i && hasDetail}
            <tr class="tr-detail">
              <td
                colspan={columns.length + (rowAction === "accordion" ? 1 : 0)}
              >
                <div class="detail-content">
                  {@render detail!(row)}
                </div>
              </td>
            </tr>
          {/if}
        {/each}
      </tbody>
    </table>
  {/if}
</div>

{#if rowAction === "modal" && activeRow && hasDetail}
  <Modal bind:open={modalOpen} {ref}>
    {#snippet title()}
      {activeRow![columns[0]?.key] ?? "Detail"}
    {/snippet}
    {#snippet body()}
      {@render detail!(activeRow!)}
    {/snippet}
    {#snippet footer()}
      <button class="modal-footer-close" onclick={() => (modalOpen = false)}
        >Close</button
      >
    {/snippet}
  </Modal>
{/if}

{#if rowAction === "drawer" && activeRow && hasDetail}
  <Drawer
    bind:open={drawerOpen}
    mode={drawerMode}
    flip={drawerFlip}
    overlay={drawerOverlay}
  >
    {#snippet title()}
      {activeRow![columns[0]?.key] ?? "Detail"}
    {/snippet}
    {#snippet body()}
      {@render detail!(activeRow)}
    {/snippet}
  </Drawer>
{/if}

<style>
  .table-wrap {
    width: 100%;
    overflow-x: auto;
    position: relative;
  }

  .table-ref {
    font-family: var(--font-mono);
    font-size: 9px;
    letter-spacing: 0.1em;
    color: var(--color-text-tertiary);
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin-bottom: var(--space-3);
  }

  .table-ref::before {
    content: "";
    width: 4px;
    height: 4px;
    background: var(--color-accent);
  }

  .data-table {
    width: 100%;
    border-collapse: collapse;
  }

  .data-table th {
    font-family: var(--font-mono);
    font-size: 9px;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--color-text-tertiary);
    padding: var(--space-2) var(--space-3);
    border-bottom: 1px solid var(--color-border);
    text-align: left;
    white-space: nowrap;
  }

  .th-icon {
    width: 32px;
  }

  .data-table td {
    padding: var(--space-2) var(--space-3);
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    font-weight: 400;
    color: var(--color-text);
    border-bottom: 1px solid var(--color-grid);
    vertical-align: middle;
  }

  .td-icon {
    width: 32px;
    text-align: center;
  }

  .data-table-hover tbody tr:hover td {
    background: var(--color-surface-raised);
  }

  .data-table-interactive tbody tr {
    cursor: pointer;
    transition: background var(--duration-fast) ease;
  }

  .data-table-interactive tbody tr:focus-visible {
    outline: 2px solid var(--color-accent);
    outline-offset: -2px;
    z-index: 1;
  }

  .tr-expanded td {
    background: var(--color-surface-raised) !important;
    border-bottom: none;
  }

  .accordion-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    transition: transform var(--duration-base) var(--ease-out-expo);
    color: var(--color-text-tertiary);
  }

  .icon-expanded {
    transform: rotate(180deg);
  }

  .tr-detail td {
    background: transparent;
    border-bottom: 1px solid var(--color-border);
    padding: 0;
  }

  .detail-content {
    padding: var(--space-4) var(--space-6);
    font-size: var(--text-sm);
    line-height: var(--leading-relaxed);
    color: var(--color-text-secondary);
    max-width: 60ch;
    animation: expandIn var(--duration-base) var(--ease-out-expo) both;
  }

  @keyframes expandIn {
    from {
      opacity: 0;
      max-height: 0;
    }
    to {
      opacity: 1;
      max-height: 500px;
    }
  }

  .table-empty {
    text-align: center;
    padding: var(--space-8);
    font-size: var(--text-sm);
    color: var(--color-text-tertiary);
  }

  .modal-footer-close {
    font-family: var(--font-display);
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    background: transparent;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-xs);
    padding: var(--space-2) var(--space-4);
    cursor: pointer;
    transition:
      color var(--duration-fast) ease,
      border-color var(--duration-fast) ease;
  }

  .modal-footer-close:hover {
    color: var(--color-text);
    border-color: var(--color-border-strong);
  }
</style>
