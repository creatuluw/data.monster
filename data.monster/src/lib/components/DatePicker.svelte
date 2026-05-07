<script lang="ts">
  import { onMount } from "svelte";
  import { Calendar, ChevronLeft, ChevronRight, X } from "lucide-svelte";
  import Tooltip from "./Tooltip.svelte";

  // Portal action: moves the element to document.body so it escapes
  // ancestor CSS transforms (e.g. Drawer transitions) that break position:fixed
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

  /**
   * DatePicker component props
   *
   * Date picker with calendar dropdown. Supports min/max dates, size variants, and portal rendering.
   */
  interface Props {
    /** Selected date value in YYYY-MM-DD format */
    value?: string | null;
    /** Placeholder text */
    placeholder?: string;
    /** Label text displayed above the input */
    label?: string;
    /** Disable the date picker */
    disabled?: boolean;
    /** Minimum selectable date in YYYY-MM-DD format */
    min?: string;
    /** Maximum selectable date in YYYY-MM-DD format */
    max?: string;
    /** Calendar dropdown size variant */
    size?: "sm" | "md" | "lg";
    /** Additional CSS classes for the wrapper */
    class?: string;
    /** HTML input id attribute */
    id?: string;
    /** HTML input name attribute */
    name?: string;
    /** Make field required */
    required?: boolean;
    /** Change event handler */
    onchange?: (value: string | null) => void;
    /** Input event handler */
    oninput?: (event: Event) => void;
    /** Whether the calendar is open (bindable) */
    open?: boolean;
    /** Tooltip text shown on hover over the input field */
    tooltip?: string;
  }

  let calendarPosition = $state({ top: 0, left: 0 });

  let {
    value: dateValue = $bindable<string | null>(null),
    placeholder = "Select date",
    label,
    disabled = false,
    min,
    max,
    size = "sm",
    class: className = "",
    id,
    name,
    required = false,
    onchange,
    oninput,
    open: isOpenProp = $bindable(false),
    tooltip,
  }: Props = $props();

  let showCalendar = $state(false);

  $effect(() => {
    showCalendar = isOpenProp;
  });

  $effect(() => {
    isOpenProp = showCalendar;
  });

  $effect(() => {
    if (showCalendar && calendarElement) {
      requestAnimationFrame(() => {
        updateCalendarPosition();
      });
    }
  });

  let showYearDropdown = $state(false);
  let inputElement: HTMLInputElement;
  let calendarElement = $state<HTMLDivElement | undefined>(undefined);
  let yearDropdownElement = $state<HTMLDivElement | undefined>(undefined);
  let currentMonth = $state(new Date());

  const today = $derived.by(() => {
    const todayDate = new Date();
    todayDate.setHours(0, 0, 0, 0);
    return todayDate;
  });

  function formatDate(date: Date): string {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const day = String(date.getDate()).padStart(2, "0");
    return `${year}-${month}-${day}`;
  }

  function parseDate(dateString: string | null): Date | null {
    if (!dateString) return null;
    const date = new Date(dateString + "T00:00:00");
    return isNaN(date.getTime()) ? null : date;
  }

  const selectedDate = $derived(parseDate(dateValue));

  $effect(() => {
    if (selectedDate) {
      currentMonth = new Date(
        selectedDate.getFullYear(),
        selectedDate.getMonth(),
        1,
      );
    }
  });

  const calendarDays = $derived.by(() => {
    const year = currentMonth.getFullYear();
    const month = currentMonth.getMonth();

    const firstDay = new Date(year, month, 1);
    const startDate = new Date(firstDay);
    const dayOfWeek = firstDay.getDay();
    const daysToSubtract = dayOfWeek === 0 ? 6 : dayOfWeek - 1;
    startDate.setDate(firstDay.getDate() - daysToSubtract);

    const days: Array<{
      date: Date;
      isCurrentMonth: boolean;
      isToday: boolean;
      isSelected: boolean;
      dateString: string;
      isDisabled: boolean;
    }> = [];

    for (let i = 0; i < 42; i++) {
      const date = new Date(startDate);
      date.setDate(startDate.getDate() + i);

      const dateString = formatDate(date);
      const isCurrentMonth = date.getMonth() === month;
      const isToday = formatDate(date) === formatDate(today);
      const isSelected = !!(
        selectedDate && formatDate(date) === formatDate(selectedDate)
      );

      let isDisabled = false;
      if (min) {
        const minDate = parseDate(min);
        if (minDate && date < minDate) isDisabled = true;
      }
      if (max) {
        const maxDate = parseDate(max);
        if (maxDate && date > maxDate) isDisabled = true;
      }

      days.push({
        date,
        isCurrentMonth,
        isToday,
        isSelected,
        dateString,
        isDisabled,
      });
    }

    return days;
  });

  const monthLabel = $derived.by(() => {
    const options: Intl.DateTimeFormatOptions = { month: "long" };
    return currentMonth.toLocaleDateString("en-US", options);
  });

  const currentYear = $derived(currentMonth.getFullYear());

  const yearOptions = $derived.by(() => {
    const years: number[] = [];
    const startYear = today.getFullYear() - 50;
    const endYear = today.getFullYear() + 50;
    for (let year = startYear; year <= endYear; year++) {
      years.push(year);
    }
    return years;
  });

  const dayHeaders = ["M", "T", "W", "T", "F", "S", "S"];

  function previousMonth() {
    const newMonth = new Date(currentMonth);
    newMonth.setMonth(newMonth.getMonth() - 1);
    currentMonth = newMonth;
    showYearDropdown = false;
  }

  function nextMonth() {
    const newMonth = new Date(currentMonth);
    newMonth.setMonth(newMonth.getMonth() + 1);
    currentMonth = newMonth;
    showYearDropdown = false;
  }

  function selectYear(year: number) {
    const newMonth = new Date(currentMonth);
    newMonth.setFullYear(year);
    currentMonth = newMonth;
    showYearDropdown = false;
  }

  function toggleYearDropdown() {
    showYearDropdown = !showYearDropdown;
  }

  function selectDate(dateString: string, isDisabled: boolean) {
    if (isDisabled || disabled) return;
    dateValue = dateString;
    showCalendar = false;
    if (onchange) onchange(dateString);
    if (inputElement && oninput) {
      const event = new Event("input", { bubbles: true });
      inputElement.dispatchEvent(event);
    }
  }

  function clearDate(e: MouseEvent) {
    e.stopPropagation();
    dateValue = null;
    if (onchange) onchange(null);
    showCalendar = false;
  }

  function updateCalendarPosition() {
    if (inputElement) {
      const rect = inputElement.getBoundingClientRect();
      const calendarWidthPx = size === "sm" ? 256 : size === "md" ? 288 : 320;
      const viewportWidth = window.innerWidth;
      const gap = 4;

      let left = rect.left;
      const rightEdge = left + calendarWidthPx;
      if (rightEdge > viewportWidth - 16) {
        left = viewportWidth - calendarWidthPx - 16;
      }
      if (left < 16) {
        left = 16;
      }

      const calendarHeight = calendarElement?.offsetHeight || 280;
      const top = rect.top - calendarHeight - gap;
      calendarPosition = { top: Math.max(16, top), left };
    }
  }

  function handleInputFocus() {
    if (!disabled) {
      if (!selectedDate) {
        const todayMonth = new Date();
        todayMonth.setHours(0, 0, 0, 0);
        currentMonth = new Date(
          todayMonth.getFullYear(),
          todayMonth.getMonth(),
          1,
        );
      }
      updateCalendarPosition();
      showCalendar = true;
    }
  }

  function handleInputChange(event: Event) {
    const target = event.target as HTMLInputElement;
    const value = target.value || null;
    if (value !== dateValue) {
      dateValue = value;
      if (onchange) onchange(value);
    }
    if (oninput) oninput(event);
  }

  onMount(() => {
    function handleClickOutside(event: MouseEvent) {
      const target = event.target as Node;
      if (
        calendarElement !== undefined &&
        calendarElement &&
        !calendarElement.contains(target) &&
        inputElement &&
        !inputElement.contains(target)
      ) {
        showCalendar = false;
      }
      if (
        yearDropdownElement !== undefined &&
        yearDropdownElement &&
        !yearDropdownElement.contains(target) &&
        calendarElement !== undefined &&
        calendarElement &&
        !calendarElement.contains(target)
      ) {
        showYearDropdown = false;
      }
    }

    if (typeof window !== "undefined") {
      document.addEventListener("mousedown", handleClickOutside);
      return () => {
        document.removeEventListener("mousedown", handleClickOutside);
      };
    }
  });

  const displayValue = $derived.by(() => {
    if (!dateValue) return "";
    if (
      typeof dateValue === "string" &&
      /^\d{4}-\d{2}-\d{2}$/.test(dateValue)
    ) {
      return dateValue;
    }
    try {
      const date = new Date(dateValue);
      if (isNaN(date.getTime())) return "";
      return formatDate(date);
    } catch {
      return "";
    }
  });
</script>

<div class="datepicker {className}">
  {#if label}
    <label class="field-label" for={id}>{label}</label>
  {/if}
  <!-- Input Field -->
  <Tooltip text={tooltip} position="top" block={true}>
    <div class="input-wrapper">
      <span class="input-icon">
        <Calendar size={16} />
      </span>
      {#if dateValue && !disabled}
        <button
          type="button"
          onclick={clearDate}
          class="input-clear"
          aria-label="Clear date"
        >
          <X size={16} />
        </button>
      {/if}
      <input
        bind:this={inputElement}
        type="date"
        readonly
        {id}
        {name}
        value={displayValue || ""}
        onchange={handleInputChange}
        onfocus={handleInputFocus}
        {min}
        {max}
        {placeholder}
        {disabled}
        {required}
        class="input"
        class:has-value={!!dateValue && !disabled}
      />
    </div>
  </Tooltip>

  <!-- Calendar Dropdown (Portaled to body to escape ancestor transforms) -->
  {#if showCalendar && !disabled}
    <div
      bind:this={calendarElement}
      use:portal
      data-size={size}
      style="position: fixed; top: {calendarPosition.top}px; left: {calendarPosition.left}px; z-index: 9999;"
      class="calendar"
    >
      <!-- Month Navigation -->
      <div class="calendar-nav">
        <button
          type="button"
          onclick={previousMonth}
          class="calendar-nav-btn"
          aria-label="Previous month"
        >
          <ChevronLeft size={20} />
        </button>
        <div class="calendar-month-year">
          <span class="calendar-month-label">{monthLabel}</span>
          <button
            type="button"
            onclick={toggleYearDropdown}
            class="calendar-year-btn"
          >
            {currentYear}
          </button>
          {#if showYearDropdown}
            <div bind:this={yearDropdownElement} class="calendar-year-dropdown">
              {#each yearOptions as year (year)}
                <button
                  type="button"
                  onclick={() => selectYear(year)}
                  class="calendar-year-option"
                  class:active={year === currentYear}
                >
                  {year}
                </button>
              {/each}
            </div>
          {/if}
        </div>
        <button
          type="button"
          onclick={nextMonth}
          class="calendar-nav-btn"
          aria-label="Next month"
        >
          <ChevronRight size={20} />
        </button>
      </div>

      <!-- Day Headers -->
      <div class="calendar-day-headers">
        {#each dayHeaders as day}
          <div class="calendar-day-header">{day}</div>
        {/each}
      </div>

      <!-- Calendar Grid -->
      <div class="calendar-grid">
        {#each calendarDays as day, index (day.dateString)}
          <button
            type="button"
            onclick={() => selectDate(day.dateString, day.isDisabled)}
            disabled={day.isDisabled}
            class="calendar-day"
            class:outside={!day.isCurrentMonth}
            class:today={day.isToday}
            class:selected={day.isSelected}
            class:disabled={day.isDisabled}
          >
            <time datetime={day.dateString} class="day-number">
              {day.date.getDate()}
            </time>
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .datepicker {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .datepicker-hint {
    font-family: var(--font-mono);
    font-size: 9px;
    color: var(--color-text-tertiary);
  }

  /* ── Input ── */
  .input-wrapper {
    position: relative;
  }

  .input-icon {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    display: flex;
    align-items: center;
    padding-left: var(--space-3);
    pointer-events: none;
    color: var(--color-text-tertiary);
  }

  .input-clear {
    position: absolute;
    top: 0;
    bottom: 0;
    right: 0;
    display: flex;
    align-items: center;
    padding-right: var(--space-3);
    color: var(--color-text-tertiary);
    background: none;
    border: none;
    cursor: pointer;
    z-index: 1;
    transition: color var(--duration-fast) ease;
  }

  .input-clear:hover {
    color: var(--color-text-secondary);
  }

  .input {
    width: 100%;
    padding: var(--space-2) var(--space-3);
    padding-left: calc(var(--space-3) + 20px + var(--space-1));
    border: 1px solid var(--color-border-strong);
    color: var(--color-text);
    background: var(--color-surface);
    border-radius: var(--radius-xs);
    cursor: pointer;
    box-sizing: border-box;
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    transition:
      border-color var(--duration-fast) ease,
      box-shadow var(--duration-fast) ease;
  }

  .input.has-value {
    padding-right: calc(var(--space-8) + var(--space-2));
  }

  .input:focus {
    outline: none;
    border-color: var(--color-accent);
    box-shadow: 0 0 0 2px var(--color-accent-muted);
  }

  .input:disabled {
    background: var(--color-surface-sunken);
    cursor: not-allowed;
  }

  .input::placeholder {
    color: var(--color-text-tertiary);
    font-size: var(--text-xs);
    font-weight: 300;
    opacity: 0.6;
  }

  /* ── Calendar dropdown ── */
  .calendar {
    background: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    border: 1px solid var(--color-border);
  }

  .calendar[data-size="sm"] {
    width: 256px;
  }

  .calendar[data-size="md"] {
    width: 288px;
  }

  .calendar[data-size="lg"] {
    width: 320px;
  }

  /* ── Calendar nav ── */
  .calendar-nav {
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid var(--color-border);
  }

  .calendar[data-size="sm"] .calendar-nav {
    padding: var(--space-2) var(--space-3);
  }

  .calendar[data-size="md"] .calendar-nav {
    padding: var(--space-3) var(--space-4);
  }

  .calendar[data-size="lg"] .calendar-nav {
    padding: var(--space-4) calc(var(--space-4) + var(--space-1));
  }

  .calendar-nav-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: var(--space-1);
    margin: calc(-1 * var(--space-1));
    color: var(--color-text-tertiary);
    background: none;
    border: none;
    cursor: pointer;
    transition: color var(--duration-fast) ease;
  }

  .calendar-nav-btn:hover {
    color: var(--color-text-secondary);
  }

  /* ── Month / Year label ── */
  .calendar-month-year {
    display: flex;
    align-items: center;
    gap: var(--space-1);
    position: relative;
    flex: 1;
    justify-content: center;
  }

  .calendar-month-label {
    font-family: var(--font-mono);
    font-weight: 600;
    color: var(--color-text);
  }

  .calendar[data-size="sm"] .calendar-month-label {
    font-size: var(--text-xs);
  }

  .calendar[data-size="md"] .calendar-month-label {
    font-size: var(--text-sm);
  }

  .calendar[data-size="lg"] .calendar-month-label {
    font-size: var(--text-base);
  }

  .calendar-year-btn {
    font-family: var(--font-mono);
    font-weight: 600;
    color: var(--color-text);
    background: none;
    border: none;
    cursor: pointer;
    padding: 0 var(--space-1);
    margin: 0 calc(-1 * var(--space-1));
    border-radius: var(--radius-xs);
    transition: color var(--duration-fast) ease;
  }

  .calendar-year-btn:hover {
    color: var(--color-text-secondary);
  }

  .calendar-year-btn:focus {
    outline: none;
    box-shadow: 0 0 0 2px var(--color-accent-muted);
  }

  .calendar[data-size="sm"] .calendar-year-btn {
    font-size: var(--text-xs);
  }

  .calendar[data-size="md"] .calendar-year-btn {
    font-size: var(--text-sm);
  }

  .calendar[data-size="lg"] .calendar-year-btn {
    font-size: var(--text-base);
  }

  /* ── Year dropdown ── */
  .calendar-year-dropdown {
    position: absolute;
    top: 100%;
    left: 50%;
    transform: translateX(-50%);
    margin-top: var(--space-1);
    background: var(--color-surface);
    border: 1px solid var(--color-border-strong);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-lg);
    max-height: 240px;
    overflow: auto;
    min-width: 80px;
    z-index: 50;
  }

  .calendar-year-option {
    display: block;
    width: 100%;
    padding: var(--space-1) var(--space-3);
    text-align: left;
    background: none;
    border: none;
    font-size: var(--text-sm);
    color: var(--color-text);
    cursor: pointer;
    transition: background var(--duration-fast) ease;
  }

  .calendar-year-option:hover {
    background: var(--color-surface-raised);
  }

  .calendar-year-option:focus {
    background: var(--color-surface-raised);
    outline: none;
  }

  .calendar-year-option.active {
    background: var(--color-surface-raised);
    font-weight: 600;
  }

  /* ── Day headers ── */
  .calendar-day-headers {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    border-bottom: 1px solid var(--color-border);
  }

  .calendar[data-size="sm"] .calendar-day-headers {
    padding: var(--space-1) var(--space-3);
  }

  .calendar[data-size="md"] .calendar-day-headers {
    padding: var(--space-2) var(--space-4);
  }

  .calendar[data-size="lg"] .calendar-day-headers {
    padding: var(--space-2) calc(var(--space-4) + var(--space-1));
  }

  .calendar-day-header {
    text-align: center;
    font-family: var(--font-mono);
    color: var(--color-text-tertiary);
  }

  .calendar[data-size="sm"] .calendar-day-header {
    font-size: 9px;
  }

  .calendar[data-size="md"] .calendar-day-header,
  .calendar[data-size="lg"] .calendar-day-header {
    font-size: var(--text-xs);
  }

  /* ── Calendar grid ── */
  .calendar-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 1px;
    background: var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
    box-shadow: var(--shadow-sm);
    border: 1px solid var(--color-border);
    isolation: isolate;
  }

  /* ── Day cells ── */
  .calendar-day {
    background: var(--color-surface);
    border: none;
    cursor: pointer;
    transition: background var(--duration-fast) ease;
    text-align: center;
    font-family: var(--font-mono);
    color: var(--color-text);
  }

  .calendar[data-size="sm"] .calendar-day {
    padding: var(--space-1) 0;
  }

  .calendar[data-size="md"] .calendar-day {
    padding: var(--space-1);
  }

  .calendar[data-size="lg"] .calendar-day {
    padding: var(--space-2) 0;
  }

  .calendar-day:focus {
    z-index: 10;
    outline: none;
    box-shadow: inset 0 0 0 2px var(--color-accent);
  }

  .calendar-day:hover {
    background: var(--color-surface-raised);
  }

  .calendar-day.outside {
    background: var(--color-surface-sunken);
    color: var(--color-text-tertiary);
  }

  .calendar-day.outside:hover {
    background: var(--color-surface-raised);
  }

  .calendar-day.disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  .calendar-day.disabled:hover {
    background: var(--color-surface);
  }

  .calendar-day.disabled.outside:hover {
    background: var(--color-surface-sunken);
  }

  /* ── Day number ── */
  .day-number {
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 0 auto;
    border-radius: var(--radius-full);
  }

  .calendar[data-size="sm"] .day-number {
    width: 24px;
    height: 24px;
    font-size: var(--text-xs);
  }

  .calendar[data-size="md"] .day-number {
    width: 28px;
    height: 28px;
    font-size: var(--text-sm);
  }

  .calendar[data-size="lg"] .day-number {
    width: 32px;
    height: 32px;
    font-size: var(--text-base);
  }

  .calendar-day.today .day-number {
    font-weight: 600;
    background: var(--color-surface-raised);
    box-shadow: inset 0 0 0 1px var(--color-border-strong);
  }

  .calendar-day.selected .day-number {
    font-weight: 600;
    background: var(--color-text);
    color: var(--color-surface);
  }

  .calendar-day.selected.today .day-number {
    background: var(--color-text-secondary);
    color: var(--color-surface);
  }
</style>
