<script lang="ts">
  import { ArrowBigDownDash } from "lucide-svelte";

  let {
    label,
    id,
    hint,
    placeholder,
    disabled = false,
    value = $bindable(""),
    onchange,
    options = [],
    class: className = "",
  }: {
    label?: string;
    id?: string;
    hint?: string;
    placeholder?: string;
    disabled?: boolean;
    value?: string;
    onchange?: (value: string) => void;
    options?: { value: string; label: string }[];
    class?: string;
  } = $props();

  let open = $state(false);
  let container: HTMLDivElement | undefined = $state();

  let displayLabel = $derived(
    options.find((o) => o.value === value)?.label ?? value,
  );

  let hasValue = $derived(value !== "" && value !== undefined);

  function toggle() {
    if (disabled) return;
    open = !open;
  }

  function select(val: string) {
    value = val;
    open = false;
    if (onchange) onchange(val);
  }

  function handleClickOutside(e: MouseEvent) {
    if (container && !container.contains(e.target as Node)) {
      open = false;
    }
  }
</script>

<svelte:window onclick={handleClickOutside} />

<div class="field {className}" bind:this={container}>
  {#if label}
    <label class="field-label" for={id}>{label}</label>
  {/if}
  <div class="select-wrap">
    <button
      {id}
      {disabled}
      type="button"
      class="select-trigger"
      class:select-open={open}
      class:select-disabled={disabled}
      onclick={toggle}
      aria-haspopup="listbox"
      aria-expanded={open}
    >
      <span class="select-value" class:select-placeholder={!hasValue}>
        {hasValue ? displayLabel : (placeholder || "\u2014")}
      </span>
      <ArrowBigDownDash size={14} class="select-icon" />
    </button>
    {#if open}
      <ul class="select-dropdown" role="listbox">
        {#each options as opt}
          <li>
            <button
              type="button"
              role="option"
              aria-selected={opt.value === value}
              class="select-option"
              class:select-option-active={opt.value === value}
              onclick={() => select(opt.value)}
            >
              {opt.label}
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
  {#if hint}
    <span class="field-hint">{hint}</span>
  {/if}
</div>

<style>
  .field {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .field-hint {
    font-family: var(--font-mono);
    font-size: 9px;
    color: var(--color-text-tertiary);
    letter-spacing: 0.02em;
  }

  .select-wrap {
    position: relative;
  }

  .select-trigger {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-2) var(--space-3);
    border: 1px solid var(--color-border-strong);
    color: var(--color-text);
    background: var(--color-surface);
    border-radius: var(--radius-xs);
    font-family: var(--font-body);
    font-size: var(--text-sm);
    line-height: 1.5;
    min-height: 2.3rem;
    cursor: pointer;
    box-sizing: border-box;
    transition:
      border-color var(--duration-fast) ease,
      box-shadow var(--duration-fast) ease;
  }

  .select-trigger:focus-visible {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: 0 0 0 2px var(--color-accent-muted);
  }

  .select-trigger.select-open {
    border-color: var(--color-accent);
    box-shadow: 0 0 0 2px var(--color-accent-muted);
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
  }

  .select-trigger.select-disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .select-value {
    flex: 1;
    text-align: left;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-family: var(--font-mono);
    font-size: var(--text-xs);
  }

  .select-placeholder {
    color: var(--color-text-placeholder);
    font-size: var(--text-xs);
    font-weight: 300;
  }

  :global(.select-icon) {
    flex-shrink: 0;
    margin-left: var(--space-2);
    color: var(--color-text-tertiary);
    transition: color var(--duration-fast) ease;
  }

  .select-trigger.select-open :global(.select-icon) {
    color: var(--color-accent);
  }

  .select-dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    list-style: none;
    margin: 0;
    padding: 0;
    background: var(--color-surface);
    border: 1px solid var(--color-accent);
    border-top: none;
    border-bottom-left-radius: var(--radius-xs);
    border-bottom-right-radius: var(--radius-xs);
    z-index: 900;
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

  .select-option {
    display: block;
    width: 100%;
    padding: var(--space-2) var(--space-3);
    color: var(--color-text);
    background: transparent;
    border: none;
    cursor: pointer;
    text-align: left;
    font-family: var(--font-body);
    font-size: var(--text-xs);
    transition:
      background var(--duration-fast) ease,
      color var(--duration-fast) ease;
  }

  .select-option:hover {
    background: var(--color-accent-dark);
    color: white;
  }

  .select-option-active {
    color: var(--color-accent);
  }
</style>
