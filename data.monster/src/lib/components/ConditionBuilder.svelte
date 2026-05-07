<script lang="ts">
  import { Plus, Trash2, Group, Ungroup } from "lucide-svelte";
  import type { Operator, SimpleCondition, ConditionGroup } from "$lib/components/condition-types";

  let {
    conditions = $bindable(),
    maxDepth = 3,
    depth = 0,
    onRemove,
    onchange,
  }: {
    conditions: ConditionGroup;
    maxDepth?: number;
    depth?: number;
    onRemove?: () => void;
    onchange?: (group: ConditionGroup) => void;
  } = $props();

  const operatorOptions: { value: Operator; label: string }[] = [
    { value: "eq", label: "=" },
    { value: "neq", label: "≠" },
    { value: "gt", label: ">" },
    { value: "gte", label: "≥" },
    { value: "lt", label: "<" },
    { value: "lte", label: "≤" },
    { value: "contains", label: "bevat" },
    { value: "not_contains", label: "bevat niet" },
    { value: "is_null", label: "is leeg" },
    { value: "is_not_null", label: "is niet leeg" },
  ];

  function isGroup(item: SimpleCondition | ConditionGroup): item is ConditionGroup {
    return "logic" in item;
  }

  function addCondition(): void {
    conditions = {
      ...conditions,
      conditions: [...conditions.conditions, { field: "", operator: "eq", value: "" }],
    };
    notifyParent();
  }

  function addGroup(): void {
    if (depth >= maxDepth - 1) return;
    conditions = {
      ...conditions,
      conditions: [
        ...conditions.conditions,
        { logic: "AND", conditions: [{ field: "", operator: "eq", value: "" }] },
      ],
    };
    notifyParent();
  }

  function removeItem(index: number): void {
    if (conditions.conditions.length <= 1 && depth === 0) return;
    conditions = {
      ...conditions,
      conditions: conditions.conditions.filter((c: SimpleCondition | ConditionGroup, i: number) => i !== index),
    };
    notifyParent();
  }

  function updateCondition(index: number, key: string, val: string): void {
    const updated = [...conditions.conditions];
    const item = { ...(updated[index] as SimpleCondition) };
    if (key === "operator") {
      item.operator = val as Operator;
      if (val === "is_null" || val === "is_not_null") item.value = null;
    } else if (key === "value") {
      item.value = val;
    } else if (key === "field") {
      item.field = val;
    }
    updated[index] = item;
    conditions = { ...conditions, conditions: updated };
    notifyParent();
  }

  function handleNestedChange(index: number, group: ConditionGroup): void {
    const updated = [...conditions.conditions];
    updated[index] = group;
    conditions = { ...conditions, conditions: updated };
    notifyParent();
  }

  function handleUngroup(index: number): void {
    const nested = conditions.conditions[index] as ConditionGroup;
    const without = conditions.conditions.filter((_: SimpleCondition | ConditionGroup, i: number) => i !== index);
    conditions = {
      ...conditions,
      conditions: [...without, ...nested.conditions],
    };
    notifyParent();
  }

  function toggleLogic(): void {
    conditions = { ...conditions, logic: conditions.logic === "AND" ? "OR" : "AND" };
    notifyParent();
  }

  function canGroup(): boolean {
    return depth < maxDepth - 1 && conditions.conditions.length >= 2;
  }

  function notifyParent(): void {
    if (onchange) onchange(conditions);
  }
</script>

<div class="condition-group" class:condition-group--root={depth === 0} class:condition-group--nested={depth > 0}>
  <div class="group-header">
    {#if depth === 0}
      <span class="connector-label connector-label--root">Waar</span>
      <div class="logic-toggle-inline" style="margin-left: auto;">
        <button type="button" class="logic-btn" class:logic-btn--active={conditions.logic === "AND"} onclick={toggleLogic} aria-label="AND logica">EN</button>
        <button type="button" class="logic-btn" class:logic-btn--active={conditions.logic === "OR"} onclick={toggleLogic} aria-label="OR logica">OF</button>
      </div>
    {:else}
      <div class="logic-toggle-inline">
        <button type="button" class="logic-btn" class:logic-btn--active={conditions.logic === "AND"} onclick={toggleLogic} aria-label="AND logica">EN</button>
        <button type="button" class="logic-btn" class:logic-btn--active={conditions.logic === "OR"} onclick={toggleLogic} aria-label="OR logica">OF</button>
      </div>
      {#if onRemove}
        <button type="button" class="group-action-btn" onclick={onRemove} title="Groep opheffen" aria-label="Groep opheffen">
          <Ungroup size={14} />
        </button>
      {/if}
    {/if}
  </div>

  <div class="group-items">
    {#each conditions.conditions as item, i}
      <div class="condition-item">
        {#if i > 0}
          <div class="connector-badge" class:connector-badge--and={conditions.logic === "AND"} class:connector-badge--or={conditions.logic === "OR"}>
            {conditions.logic === "AND" ? "EN" : "OF"}
          </div>
        {/if}

        {#if isGroup(item)}
          <div class="nested-group-wrapper">
            <svelte:self
              conditions={item}
              {maxDepth}
              depth={depth + 1}
              onRemove={() => handleUngroup(i)}
              onchange={(group: ConditionGroup) => handleNestedChange(i, group)}
            />
          </div>
        {:else}
          <div class="condition-row">
            <input
              type="text"
              class="input cond-field"
              placeholder="veldnaam"
              value={item.field}
              oninput={(e) => updateCondition(i, "field", (e.target as HTMLInputElement).value)}
            />
            <div class="cond-operator-wrap">
              <select
                class="input cond-operator"
                value={item.operator}
                onchange={(e) => updateCondition(i, "operator", (e.target as HTMLSelectElement).value)}
              >
                {#each operatorOptions as op}
                  <option value={op.value}>{op.label}</option>
                {/each}
              </select>
            </div>
            {#if item.operator !== "is_null" && item.operator !== "is_not_null"}
              <input
                type="text"
                class="input cond-value"
                placeholder="waarde"
                value={item.value ?? ""}
                oninput={(e) => updateCondition(i, "value", (e.target as HTMLInputElement).value)}
              />
            {/if}
            <button
              type="button"
              class="cond-remove"
              disabled={conditions.conditions.length <= 1 && depth === 0}
              onclick={() => removeItem(i)}
              aria-label="Verwijder conditie"
            >
              <Trash2 size={14} />
            </button>
          </div>
        {/if}
      </div>
    {/each}
  </div>

  <div class="group-actions">
    <button type="button" class="btn-add" onclick={addCondition}>
      <Plus size={14} />
      Conditie
    </button>
    {#if canGroup()}
      <button type="button" class="btn-add btn-add--group" onclick={addGroup}>
        <Group size={14} />
        Groep
      </button>
    {/if}
  </div>
</div>

<style>
  .condition-group {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .condition-group--root {
    gap: var(--space-3);
  }

  .condition-group--nested {
    border-left: 3px solid var(--color-accent-muted, rgba(59, 130, 246, 0.3));
    padding-left: var(--space-3);
    margin-top: var(--space-2);
    margin-bottom: var(--space-1);
    background: var(--color-surface-sunken, #f8f9fa);
    border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
    padding-top: var(--space-2);
    padding-bottom: var(--space-2);
    padding-right: var(--space-3);
  }

  .group-header {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex-wrap: wrap;
  }

  .connector-label {
    font-family: var(--font-mono, monospace);
    font-size: var(--text-xs, 11px);
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--color-accent-dark, #1d4ed8);
  }

  .connector-label--root {
    color: var(--color-text-secondary, #6b7280);
  }

  .logic-toggle-inline {
    display: inline-flex;
    gap: 2px;
    background: var(--color-surface-sunken, #f1f5f9);
    border-radius: var(--radius-xs, 4px);
    padding: 2px;
  }

  .logic-btn {
    padding: var(--space-1, 4px) var(--space-2, 8px);
    border: 1px solid var(--color-border, #e2e8f0);
    border-radius: var(--radius-xs, 4px);
    background: transparent;
    font-family: var(--font-mono, monospace);
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.08em;
    cursor: pointer;
    color: var(--color-text-secondary, #6b7280);
    transition: all 0.15s ease;
    white-space: nowrap;
  }

  .logic-btn:hover {
    background: var(--color-surface, #fff);
    color: var(--color-text, #111827);
  }

  .logic-btn--active {
    background: var(--color-accent, #3b82f6);
    color: white;
    border-color: var(--color-accent, #3b82f6);
  }

  .logic-btn--active:hover {
    background: var(--color-accent-dark, #1d4ed8);
    color: white;
  }

  .group-action-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: 1px solid var(--color-border, #e2e8f0);
    border-radius: var(--radius-xs, 4px);
    background: transparent;
    color: var(--color-text-tertiary, #9ca3af);
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .group-action-btn:hover {
    background: var(--color-danger-bg, rgba(220, 38, 38, 0.1));
    color: var(--color-danger, #dc2626);
    border-color: var(--color-danger, #dc2626);
  }

  .group-items {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .condition-item {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }

  .connector-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-family: var(--font-mono, monospace);
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.08em;
    padding: 2px 8px;
    border-radius: var(--radius-xs, 4px);
    width: fit-content;
    margin-top: var(--space-1);
  }

  .connector-badge--and {
    background: var(--color-accent-muted, rgba(59, 130, 246, 0.1));
    color: var(--color-accent-dark, #1d4ed8);
    border: 1px solid var(--color-accent-muted, rgba(59, 130, 246, 0.3));
  }

  .connector-badge--or {
    background: rgba(245, 158, 11, 0.1);
    color: #b45309;
    border: 1px solid rgba(245, 158, 11, 0.3);
  }

  .condition-row {
    display: flex;
    gap: var(--space-2, 8px);
    align-items: center;
  }

  .input {
    width: 100%;
    padding: var(--space-2, 8px) var(--space-3, 12px);
    border: 1px solid var(--color-border-strong, #d1d5db);
    color: var(--color-text, #111827);
    background: var(--color-surface, #fff);
    border-radius: var(--radius-xs, 4px);
    font-family: var(--font-body, system-ui);
    font-size: var(--text-sm, 14px);
    line-height: 1.5;
    box-sizing: border-box;
    transition:
      border-color 0.15s ease,
      box-shadow 0.15s ease;
  }

  .input::placeholder {
    color: var(--color-text-placeholder, #9ca3af);
    font-size: var(--text-xs, 12px);
    font-weight: 300;
  }

  .input:focus {
    outline: none;
    border-color: var(--color-accent, #3b82f6);
    box-shadow: 0 0 0 2px var(--color-accent-muted, rgba(59, 130, 246, 0.2));
  }

  .cond-field {
    flex: 2;
    min-width: 0;
  }

  .cond-operator-wrap {
    flex: 2;
    min-width: 0;
    position: relative;
  }

  .cond-operator {
    width: 100%;
    padding: var(--space-2, 8px) var(--space-2, 8px);
    border: 1px solid var(--color-border-strong, #d1d5db);
    border-radius: var(--radius-xs, 4px);
    background: var(--color-surface, #fff);
    color: var(--color-text, #111827);
    font-size: var(--text-sm, 14px);
    appearance: none;
    -webkit-appearance: none;
    cursor: pointer;
  }

  .cond-value {
    flex: 2;
    min-width: 0;
  }

  .cond-remove {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 1px solid var(--color-border, #e2e8f0);
    border-radius: var(--radius-xs, 4px);
    background: transparent;
    color: var(--color-text-tertiary, #9ca3af);
    cursor: pointer;
    flex-shrink: 0;
    transition: all 0.15s ease;
  }

  .cond-remove:hover:not(:disabled) {
    background: var(--color-danger-bg, rgba(220, 38, 38, 0.1));
    color: var(--color-danger, #dc2626);
    border-color: var(--color-danger, #dc2626);
  }

  .cond-remove:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .nested-group-wrapper {
    margin-top: var(--space-1);
  }

  .group-actions {
    display: flex;
    gap: var(--space-2, 8px);
    flex-wrap: wrap;
    padding-top: var(--space-2, 8px);
  }

  .btn-add {
    display: inline-flex;
    align-items: center;
    gap: var(--space-1, 4px);
    padding: var(--space-1, 4px) var(--space-2, 8px);
    border: 1px dashed var(--color-border, #e2e8f0);
    border-radius: var(--radius-xs, 4px);
    background: transparent;
    color: var(--color-text-secondary, #6b7280);
    font-family: var(--font-body, system-ui);
    font-size: var(--text-xs, 12px);
    font-weight: 600;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .btn-add:hover {
    color: var(--color-accent, #3b82f6);
    border-color: var(--color-accent, #3b82f6);
    background: var(--color-accent-muted, rgba(59, 130, 246, 0.05));
  }

  .btn-add--group:hover {
    color: #b45309;
    border-color: #f59e0b;
    background: rgba(245, 158, 11, 0.05);
  }

  @media (max-width: 640px) {
    .condition-row {
      flex-wrap: wrap;
    }

    .cond-field,
    .cond-value {
      flex: 1 1 100%;
    }

    .cond-operator-wrap {
      flex: 1 1 100%;
    }

    .logic-toggle-inline {
      width: 100%;
    }

    .logic-btn {
      flex: 1;
      text-align: center;
    }
  }
</style>