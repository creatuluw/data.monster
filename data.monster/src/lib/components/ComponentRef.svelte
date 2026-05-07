<script lang="ts">
  import { Copy, Check } from "lucide-svelte";

  let {
    name,
    importLine,
    usage,
    uid,
    onpopoverenter,
    onpopoverleave,
  }: {
    name: string;
    importLine: string;
    usage: string;
    uid: string;
    onpopoverenter?: () => void;
    onpopoverleave?: () => void;
  } = $props();

  let copiedId = $state<string | null>(null);

  function copy(text: string, id: string) {
    navigator.clipboard.writeText(text);
    copiedId = id;
    setTimeout(() => (copiedId = null), 1200);
  }
</script>

<div class="pop" onclick={(e) => e.stopPropagation()} onmouseenter={onpopoverenter} onmouseleave={onpopoverleave}>
  <div class="pop-head">
    <span class="pop-name">{name}</span>
  </div>
  <div class="pop-row">
    <code class="pop-code">{importLine}</code>
    <button class="pop-copy" onclick={() => copy(importLine, uid + "-i")}>
      {#if copiedId === uid + "-i"}<Check size={11} />{:else}<Copy size={11} />{/if}
    </button>
  </div>
  <div class="pop-row">
    <code class="pop-code">{usage}</code>
    <button class="pop-copy" onclick={() => copy(usage, uid + "-u")}>
      {#if copiedId === uid + "-u"}<Check size={11} />{:else}<Copy size={11} />{/if}
    </button>
  </div>
</div>

<style>
  .pop {
    position: absolute;
    bottom: calc(100% + 6px);
    left: 50%;
    transform: translateX(-50%);
    width: max-content;
    min-width: 240px;
    max-width: 380px;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    box-shadow: var(--shadow-lg);
    z-index: 100;
    padding: var(--space-3) var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    animation: popIn 120ms var(--ease-out-expo) both;
    pointer-events: auto;
  }

  @keyframes popIn {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(4px);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0);
    }
  }

  .pop-head {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .pop-name {
    font-family: var(--font-display);
    font-size: var(--text-sm);
    font-weight: 700;
    color: var(--color-text);
  }

  .pop-row {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    background: var(--color-surface-sunken);
    border: 1px solid var(--color-border);
    padding: 3px var(--space-2);
    border-radius: var(--radius-xs);
  }

  .pop-code {
    flex: 1;
    font-family: var(--font-mono);
    font-size: 9px;
    line-height: 1.4;
    color: var(--color-text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .pop-copy {
    flex-shrink: 0;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-xs);
    cursor: pointer;
    color: var(--color-text-tertiary);
    transition:
      color var(--duration-fast) ease,
      border-color var(--duration-fast) ease;
  }

  .pop-copy:hover {
    color: var(--color-accent);
    border-color: var(--color-accent);
  }

  @media (max-width: 768px) {
    .pop {
      left: 0;
      transform: none;
      min-width: 200px;
    }
  }
</style>