<script lang="ts">
  import { ChevronRight } from "lucide-svelte";

  type BreadcrumbItem = {
    href: string;
    label: string;
  };

  let {
    items,
    ariaLabel = "Breadcrumb",
  }: {
    items: BreadcrumbItem[];
    ariaLabel?: string;
  } = $props();
</script>

{#if items.length > 0}
  <nav aria-label={ariaLabel} class="breadcrumb">
    <ol class="breadcrumb-list">
      {#each items as item, i}
        <li class="breadcrumb-item">
          {#if i < items.length - 1}
            <a href={item.href} class="breadcrumb-link">{item.label}</a>
            <span class="breadcrumb-sep" aria-hidden="true"
              ><ChevronRight size={10} /></span
            >
          {:else}
            <span class="breadcrumb-current" aria-current="page"
              >{item.label}</span
            >
          {/if}
        </li>
      {/each}
    </ol>
  </nav>
{/if}

<style>
  .breadcrumb {
    display: flex;
    align-items: center;
  }

  .breadcrumb-list {
    display: flex;
    align-items: center;
    gap: 0;
    list-style: none;
    margin: 0;
    padding: 0;
    flex-wrap: wrap;
  }

  .breadcrumb-item {
    display: inline-flex;
    align-items: center;
    gap: 0;
  }

  .breadcrumb-link {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    font-weight: 300;
    color: var(--color-text-secondary);
    text-decoration: none;
    padding: 2px var(--space-1);
    border-radius: var(--radius-xs);
    transition: color var(--duration-fast) ease;
  }

  .breadcrumb-link:hover {
    color: var(--color-accent);
  }

  .breadcrumb-link:focus-visible {
    outline: 2px solid var(--color-accent);
    outline-offset: 1px;
  }

  .breadcrumb-sep {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 12px;
    height: 12px;
    color: var(--color-text-tertiary);
    flex-shrink: 0;
  }

  .breadcrumb-current {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    font-weight: 400;
    color: var(--color-text);
    padding: 2px var(--space-1);
  }

  @media (max-width: 768px) {
    .breadcrumb-list {
      flex-wrap: nowrap;
      overflow-x: auto;
      -webkit-overflow-scrolling: touch;
      scrollbar-width: none;
    }

    .breadcrumb-list::-webkit-scrollbar {
      display: none;
    }
  }
</style>
