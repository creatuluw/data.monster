<script lang="ts">
  let {
    size = "md",
    centered = false,
    label,
  }: {
    size?: "sm" | "md" | "lg" | "xl";
    centered?: boolean;
    label?: string;
  } = $props();
</script>

<div
  class="spinner-wrap"
  class:spinner-centered={centered}
  class:spinner-tile={!!label}
>
  <div
    class="spinner spinner-{size}"
    role="status"
    aria-label={label || "Loading"}
  >
    <span class="sr-only">{label || "Loading..."}</span>
  </div>
  {#if label}
    <p class="spinner-label">{label}</p>
  {/if}
</div>

<style>
  .spinner-wrap {
    display: inline-flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-3);
  }

  .spinner-centered {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: var(--space-8);
  }

  .spinner {
    border: 2px solid var(--color-border);
    border-top-color: var(--color-accent);
    border-radius: 50%;
    animation: spin 1.2s linear infinite;
  }

  .spinner-sm {
    width: 14px;
    height: 14px;
    border-width: 2px;
  }

  .spinner-md {
    width: 20px;
    height: 20px;
    border-width: 2px;
  }

  .spinner-lg {
    width: 32px;
    height: 32px;
    border-width: 3px;
  }

  .spinner-xl {
    width: 48px;
    height: 48px;
    border-width: 4px;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .spinner-label {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
  }

  .spinner-tile {
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    padding: var(--space-4) var(--space-6);
    background: var(--color-surface);
    box-shadow: var(--shadow-sm);
  }

  .spinner-centered.spinner-tile {
    padding: var(--space-8) var(--space-6);
  }

  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }
</style>
