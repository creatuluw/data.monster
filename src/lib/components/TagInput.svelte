<script lang="ts">
  import { X } from "lucide-svelte";

  type TagItem = string;

  let {
    tags = $bindable([]),
    suggestions = [],
    placeholder = "Tag toevoegen…",
    onchange,
  }: {
    tags?: TagItem[];
    suggestions?: string[];
    placeholder?: string;
    onchange?: (tags: TagItem[]) => void;
  } = $props();

  let inputValue = $state("");
  let showSuggestions = $state(false);

  let filteredSuggestions = $derived(
    suggestions.filter(
      (s) => s.toLowerCase().includes(inputValue.toLowerCase()) && !tags.includes(s),
    ),
  );

  function addTag(tag: string) {
    const trimmed = tag.trim();
    if (trimmed && !tags.includes(trimmed)) {
      const updated = [...tags, trimmed];
      tags = updated;
      onchange?.(updated);
    }
    inputValue = "";
    showSuggestions = false;
  }

  function removeTag(index: number) {
    const updated = tags.filter((_: string, i: number) => i !== index);
    tags = updated;
    onchange?.(updated);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Enter" && inputValue.trim()) {
      e.preventDefault();
      addTag(inputValue);
    } else if (e.key === "Backspace" && !inputValue && tags.length > 0) {
      tags = tags.slice(0, -1);
    }
  }

  function handleBlur() {
    setTimeout(() => {
      showSuggestions = false;
    }, 150);
  }
</script>

<div class="tag-input">
  <div class="tag-input-field">
    {#each tags as tag, i}
      <span class="tag-chip">
        {tag}
        <button type="button" class="tag-remove" onclick={() => removeTag(i)} aria-label="Verwijder tag {tag}">
          <X size={10} />
        </button>
      </span>
    {/each}
    <input
      type="text"
      class="tag-text-input"
      {placeholder}
      bind:value={inputValue}
      onkeydown={handleKeydown}
      onfocus={() => (showSuggestions = true)}
      onblur={handleBlur}
    />
  </div>
  {#if showSuggestions && filteredSuggestions.length > 0 && inputValue.length > 0}
    <div class="tag-suggestions">
      {#each filteredSuggestions as suggestion}
        <button type="button" class="tag-suggestion" onclick={() => addTag(suggestion)}>
          {suggestion}
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .tag-input {
    position: relative;
  }

  .tag-input-field {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-1);
    padding: var(--space-1) var(--space-2);
    border: 1px solid var(--color-border-strong, #d1d5db);
    border-radius: var(--radius-xs, 4px);
    background: var(--color-surface, #fff);
    min-height: 36px;
    align-items: center;
  }

  .tag-input-field:focus-within {
    border-color: var(--color-accent, #3b82f6);
    box-shadow: 0 0 0 2px var(--color-accent-muted, rgba(59, 130, 246, 0.2));
  }

  .tag-chip {
    display: inline-flex;
    align-items: center;
    gap: 2px;
    padding: 2px var(--space-2);
    background: var(--color-accent-muted, rgba(59, 130, 246, 0.1));
    color: var(--color-accent-dark, #1d4ed8);
    border: 1px solid var(--color-accent-muted, rgba(59, 130, 246, 0.3));
    border-radius: var(--radius-xs, 4px);
    font-size: var(--text-xs, 12px);
    font-weight: 600;
    font-family: var(--font-body, system-ui);
    line-height: 1.4;
  }

  .tag-remove {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 14px;
    height: 14px;
    border: none;
    background: transparent;
    color: var(--color-accent-dark, #1d4ed8);
    cursor: pointer;
    border-radius: 50%;
    padding: 0;
    opacity: 0.6;
    transition: opacity var(--duration-fast, 150ms) ease;
  }

  .tag-remove:hover {
    opacity: 1;
  }

  .tag-text-input {
    flex: 1;
    min-width: 80px;
    border: none;
    outline: none;
    background: transparent;
    font-family: var(--font-body, system-ui);
    font-size: var(--text-sm, 14px);
    color: var(--color-text, #111827);
    padding: 0;
    line-height: 1.5;
  }

  .tag-text-input::placeholder {
    color: var(--color-text-placeholder, #9ca3af);
    font-size: var(--text-xs, 12px);
    font-weight: 300;
  }

  .tag-suggestions {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background: var(--color-surface, #fff);
    border: 1px solid var(--color-border, #e2e8f0);
    border-radius: 0 0 var(--radius-xs, 4px) var(--radius-xs, 4px);
    box-shadow: var(--shadow-sm);
    max-height: 150px;
    overflow-y: auto;
    z-index: 10;
  }

  .tag-suggestion {
    display: block;
    width: 100%;
    text-align: left;
    padding: var(--space-1, 4px) var(--space-2, 8px);
    border: none;
    background: transparent;
    color: var(--color-text, #111827);
    font-family: var(--font-body, system-ui);
    font-size: var(--text-xs, 12px);
    cursor: pointer;
    transition: background var(--duration-fast, 150ms) ease;
  }

  .tag-suggestion:hover {
    background: var(--color-surface-sunken, #f1f5f9);
  }
</style>