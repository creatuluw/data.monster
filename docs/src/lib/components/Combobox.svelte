<script lang="ts" generics="T">
  import { createEventDispatcher } from 'svelte';
  
  interface Props<T> {
    options?: T[];
    value?: T | null;
    displayField?: keyof T | ((item: T) => string);
    searchField?: keyof T | ((item: T) => string);
    placeholder?: string;
    label?: string;
    disabled?: boolean;
    required?: boolean;
    className?: string;
    helpText?: string;
  }
  
  const {
    options = [],
    value = null,
    displayField = ((item: T) => String(item)) as keyof T | ((item: T) => string),
    searchField = ((item: T) => String(item)) as keyof T | ((item: T) => string),
    placeholder = 'Select an option...',
    label = '',
    disabled = false,
    required = false,
    className = '',
    helpText = ''
  }: Props<T> = $props();
  
  const dispatch = createEventDispatcher<{
    select: T;
    change: T | null;
  }>();
  
  let isOpen = $state(false);
  let searchQuery = $state('');
  let filteredOptions = $state<T[]>([]);
  let highlightedIndex = $state(-1);
  let inputElement: HTMLInputElement;
  
  // Filter options based on search query
  $effect(() => {
    if (!searchQuery.trim()) {
      filteredOptions = options;
    } else {
      const query = searchQuery.toLowerCase();
      filteredOptions = options.filter(option => {
        const searchValue = typeof searchField === 'function' 
          ? searchField(option).toLowerCase()
          : String(option[searchField]).toLowerCase();
        return searchValue.includes(query);
      });
    }
    highlightedIndex = -1;
  });
  
  function getDisplayValue(item: T): string {
    return typeof displayField === 'function' 
      ? displayField(item)
      : String(item[displayField]);
  }
  
  function getSearchValue(item: T): string {
    return typeof searchField === 'function' 
      ? searchField(item)
      : String(item[searchField]);
  }
  
  function selectOption(option: T) {
    searchQuery = getDisplayValue(option);
    isOpen = false;
    dispatch('select', option);
    dispatch('change', option);
  }
  
  function clearSelection() {
    searchQuery = '';
    isOpen = false;
    dispatch('change', null);
  }
  
  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
    searchQuery = target.value;
    isOpen = true;
    highlightedIndex = -1;
  }
  
  function handleKeydown(event: KeyboardEvent) {
    if (!isOpen) {
      if (event.key === 'Enter' || event.key === ' ' || event.key === 'ArrowDown') {
        event.preventDefault();
        isOpen = true;
      }
      return;
    }
    
    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        highlightedIndex = Math.min(highlightedIndex + 1, filteredOptions.length - 1);
        break;
      case 'ArrowUp':
        event.preventDefault();
        highlightedIndex = Math.max(highlightedIndex - 1, -1);
        break;
      case 'Enter':
        event.preventDefault();
        if (highlightedIndex >= 0 && highlightedIndex < filteredOptions.length) {
          selectOption(filteredOptions[highlightedIndex]);
        }
        break;
      case 'Escape':
        event.preventDefault();
        isOpen = false;
        highlightedIndex = -1;
        break;
    }
  }
  
  function handleBlur() {
    // Delay closing to allow for option selection
    setTimeout(() => {
      isOpen = false;
      highlightedIndex = -1;
    }, 150);
  }
  
  function handleFocus() {
    if (!disabled) {
      isOpen = true;
    }
  }
</script>

<div class="relative {className}">
  {#if label}
    <label for="combobox-input" class="block text-sm/6 font-medium text-gray-900 dark:text-white mb-2">
      {label}
      {#if required}
        <span class="text-red-500">*</span>
      {/if}
    </label>
  {/if}
  
  <div class="relative">
    <input
      id="combobox-input"
      bind:this={inputElement}
      type="text"
      {placeholder}
      {disabled}
      {required}
      value={searchQuery}
      oninput={handleInput}
      onkeydown={handleKeydown}
      onblur={handleBlur}
      onfocus={handleFocus}
      class="block w-full rounded-md bg-white py-1.5 pr-12 pl-3 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-orange-600 sm:text-sm/6 dark:bg-white/5 dark:text-white dark:outline-white/10 dark:placeholder:text-gray-500 dark:focus:outline-orange-500 {disabled ? 'opacity-50 cursor-not-allowed' : ''}"
    />
    
    <button
      type="button"
      onclick={() => isOpen ? clearSelection() : (isOpen = true)}
      {disabled}
      aria-label={isOpen ? 'Close dropdown' : 'Open dropdown'}
      class="absolute inset-y-0 right-0 flex items-center rounded-r-md px-2 {disabled ? 'cursor-not-allowed' : 'cursor-pointer'}"
    >
      <svg 
        viewBox="0 0 20 20" 
        fill="currentColor" 
        data-slot="icon" 
        aria-hidden="true" 
        class="size-5 text-gray-400 {isOpen ? 'rotate-180' : ''} transition-transform duration-200"
      >
        <path d="M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z" clip-rule="evenodd" fill-rule="evenodd" />
      </svg>
    </button>
  </div>
  
  {#if helpText}
    <p class="text-xs text-slate-500 dark:text-slate-400 mt-1">{helpText}</p>
  {/if}
  
  {#if isOpen && filteredOptions.length > 0}
    <div class="absolute z-10 mt-1 w-full max-h-60 overflow-auto rounded-md bg-white py-1 text-base shadow-lg outline outline-black/5 transition-discrete [--anchor-gap:--spacing(1)] data-leave:transition data-leave:duration-100 data-leave:ease-in data-closed:data-leave:opacity-0 sm:text-sm dark:bg-gray-800 dark:shadow-none dark:-outline-offset-1 dark:outline-white/10">
      {#each filteredOptions as option, index}
        <button
          type="button"
          onclick={() => selectOption(option)}
          class="group relative block w-full text-left truncate px-3 py-2 text-gray-900 select-none hover:bg-gray-100 {index === highlightedIndex ? 'bg-orange-600 text-white hover:bg-orange-500' : ''} {value === option ? 'bg-orange-600 text-white hover:bg-orange-500' : ''} dark:text-gray-300 dark:hover:bg-gray-700 {index === highlightedIndex ? 'dark:bg-orange-500 dark:hover:bg-orange-400' : ''} {value === option ? 'dark:bg-orange-500 dark:hover:bg-orange-400' : ''}"
        >
          <span class="block pr-6">{getDisplayValue(option)}</span>
          {#if value === option || index === highlightedIndex}
            <svg class="absolute right-2 top-1/2 -translate-y-1/2 w-3 h-3 opacity-0 group-hover:opacity-100 transition-opacity" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          {/if}
        </button>
      {/each}
    </div>
  {/if}
  
  {#if isOpen && filteredOptions.length === 0 && searchQuery.trim()}
    <div class="absolute z-10 mt-1 w-full rounded-md bg-white py-1 text-base shadow-lg outline outline-black/5 sm:text-sm dark:bg-gray-800 dark:shadow-none dark:-outline-offset-1 dark:outline-white/10">
      <div class="px-3 py-2 text-gray-500 text-sm">
        No options found for "{searchQuery}"
      </div>
    </div>
  {/if}
</div>

<style>
  .transition-discrete {
    transition: opacity 0.1s ease-in-out;
  }
</style>