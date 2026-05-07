<script lang="ts">
  import { tick } from "svelte";
  import { MoreVertical, MoreHorizontal } from "lucide-svelte";

  function portal(node: HTMLElement) {
    const target = document.body;
    target.appendChild(node);
    return {
      destroy() {
        if (node.parentNode === target) {
          target.removeChild(node);
        }
      },
    };
  }

  interface MenuItem {
    href?: string;
    label: string;
    icon?: import("svelte").Snippet;
    onclick?: (e: MouseEvent) => void;
  }

  let {
    items = [],
    ariaLabel = "Acties",
    variant = "vertical",
  }: {
    items?: MenuItem[];
    ariaLabel?: string;
    variant?: "vertical" | "horizontal";
  } = $props();

  let open = $state(false);
  let triggerEl: HTMLElement | undefined = $state();

  function toggle(e: MouseEvent) {
    e.stopPropagation();
    open = !open;
  }

  function close() {
    open = false;
  }

  function handleItemClick(item: MenuItem, e: MouseEvent) {
    if (item.onclick) item.onclick(e);
    close();
  }

  async function positionDropdown(dropdownEl: HTMLElement) {
    await tick();
    if (!triggerEl) return;
    const rect = triggerEl.getBoundingClientRect();
    dropdownEl.style.position = "fixed";
    dropdownEl.style.top = `${rect.bottom + 4}px`;
    dropdownEl.style.right = `${window.innerWidth - rect.right}px`;
    dropdownEl.style.zIndex = "9999";
  }
</script>

<svelte:window onclick={close} />

<div class="ellipsis-menu">
  <button
    class="ellipsis-trigger"
    class:is-open={open}
    onclick={toggle}
    aria-label={ariaLabel}
    aria-expanded={open}
    bind:this={triggerEl}
  >
    {#if variant === "horizontal"}
      <MoreHorizontal size={16} />
    {:else}
      <MoreVertical size={16} />
    {/if}
  </button>
  {#if open}
    <div class="ellipsis-dropdown" use:portal use:positionDropdown>
      {#each items as item}
        {#if item.href}
          <a
            href={item.href}
            class="ellipsis-item"
            onclick={(e) => handleItemClick(item, e)}
          >
            {#if item.icon}{@render item.icon()}{/if}
            {item.label}
          </a>
        {:else}
          <button
            class="ellipsis-item"
            onclick={(e) => handleItemClick(item, e)}
          >
            {#if item.icon}{@render item.icon()}{/if}
            {item.label}
          </button>
        {/if}
      {/each}
    </div>
  {/if}
</div>

<style>
  .ellipsis-menu {
    position: relative;
  }

  .ellipsis-trigger {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: var(--radius-xs);
    cursor: pointer;
    color: var(--color-text-tertiary);
    transition:
      color var(--duration-fast) ease,
      background var(--duration-fast) ease,
      border-color var(--duration-fast) ease;
  }

  .ellipsis-trigger:hover {
    color: var(--color-text);
    background: var(--color-surface-sunken);
    border-color: var(--color-border);
  }

  .ellipsis-trigger.is-open {
    color: var(--color-text);
    background: var(--color-surface-sunken);
    border-color: var(--color-border);
  }

  .ellipsis-trigger:focus-visible {
    outline: 2px solid var(--color-accent);
    outline-offset: 1px;
  }

  .ellipsis-dropdown {
    min-width: 180px;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    box-shadow: var(--shadow-md);
    animation: slideDown var(--duration-fast) var(--ease-out-expo) both;
  }

  @keyframes slideDown {
    from {
      opacity: 0;
      transform: translateY(-4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .ellipsis-item {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    width: 100%;
    padding: var(--space-2) var(--space-3);
    font-family: var(--font-body);
    font-size: var(--text-sm);
    font-weight: 500;
    color: var(--color-text-secondary);
    text-decoration: none;
    background: transparent;
    border: none;
    cursor: pointer;
    transition:
      color var(--duration-fast) ease,
      background var(--duration-fast) ease;
  }

  .ellipsis-item:first-child {
    border-radius: var(--radius-sm) var(--radius-sm) 0 0;
  }

  .ellipsis-item:last-child {
    border-radius: 0 0 var(--radius-sm) var(--radius-sm);
  }

  .ellipsis-item:only-child {
    border-radius: var(--radius-sm);
  }

  .ellipsis-item:hover {
    color: var(--color-text);
    background: var(--color-surface-sunken);
  }
</style>
