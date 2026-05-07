<script lang="ts">
  import DatePicker from "./DatePicker.svelte";
  import { X } from "lucide-svelte";

  /**
   * DateRangePicker component props
   *
   * Date range picker with from/to dates. Validates that 'to' date is after 'from' date.
   */
  interface Props {
    /** Start date in YYYY-MM-DD format */
    fromDate?: string | null;
    /** End date in YYYY-MM-DD format */
    toDate?: string | null;
    /** Placeholder text for from date */
    fromPlaceholder?: string;
    /** Placeholder text for to date */
    toPlaceholder?: string;
    /** Additional CSS classes */
    class?: string;
    /** Change event handler - called when either date changes */
    onchange?: (fromDate: string | null, toDate: string | null) => void;
    /** Whether the filter is currently active */
    isActive?: boolean;
    /** Whether any date picker is open (bindable, for parent z-index management) */
    open?: boolean;
    /** Tooltip text for the from date input */
    fromTooltip?: string;
    /** Tooltip text for the to date input */
    toTooltip?: string;
  }

  let {
    fromDate = $bindable<string | null>(null),
    toDate = $bindable<string | null>(null),
    fromPlaceholder = "Van datum",
    toPlaceholder = "Tot datum",
    class: className = "",
    onchange,
    isActive = false,
    open: isOpenProp = $bindable(false),
    fromTooltip = "Selecteer startdatum",
    toTooltip = "Selecteer einddatum",
  }: Props = $props();

  let fromDatePickerOpen = $state(false);
  let toDatePickerOpen = $state(false);

  $effect(() => {
    isOpenProp = fromDatePickerOpen || toDatePickerOpen;
  });

  function handleFromDateChange(value: string | null) {
    fromDate = value;
    if (fromDate && toDate && new Date(toDate) < new Date(fromDate)) {
      toDate = null;
    }
    if (onchange) onchange(fromDate, toDate);
  }

  function handleToDateChange(value: string | null) {
    toDate = value;
    if (toDate && fromDate && new Date(fromDate) > new Date(toDate)) {
      fromDate = null;
    }
    if (onchange) onchange(fromDate, toDate);
  }

  function handleClear() {
    fromDate = null;
    toDate = null;
    if (onchange) onchange(null, null);
  }

  const hasDateRange = $derived(fromDate !== null || toDate !== null);
</script>

<div class="range-picker {className}">
  <div class="range-picker-inputs">
    <div class="range-picker-field">
      <DatePicker
        bind:value={fromDate}
        placeholder={fromPlaceholder}
        max={toDate ?? undefined}
        onchange={handleFromDateChange}
        bind:open={fromDatePickerOpen}
        tooltip={fromTooltip}
      />
    </div>

    <span class="range-picker-separator">—</span>

    <div class="range-picker-field">
      <DatePicker
        bind:value={toDate}
        placeholder={toPlaceholder}
        min={fromDate ?? undefined}
        onchange={handleToDateChange}
        bind:open={toDatePickerOpen}
        tooltip={toTooltip}
      />
    </div>
  </div>

  {#if hasDateRange}
    <button
      type="button"
      onclick={handleClear}
      class="range-picker-clear"
      aria-label="Clear date range"
      title="Clear date range"
    >
      <X size={14} />
    </button>
  {/if}
</div>

<style>
  .range-picker {
    position: relative;
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .range-picker-inputs {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    flex: 1;
  }

  .range-picker-field {
    flex: 1;
    min-width: 0;
  }

  .range-picker-separator {
    color: var(--color-text-tertiary);
    font-size: var(--text-sm);
    flex-shrink: 0;
  }

  .range-picker-clear {
    position: absolute;
    top: -6px;
    right: -6px;
    padding: var(--space-1);
    border-radius: var(--radius-full);
    background: var(--color-text);
    color: var(--color-surface);
    border: none;
    cursor: pointer;
    z-index: 10;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background var(--duration-fast) ease;
  }

  .range-picker-clear:hover {
    background: var(--color-text-secondary);
  }
</style>
