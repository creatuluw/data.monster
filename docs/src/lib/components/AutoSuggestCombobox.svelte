<script lang="ts">
	import { ChevronDown, X } from 'lucide-svelte';

	interface Props {
		options: string[];
		value?: string;
		placeholder?: string;
		onValueChange?: (value: string) => void;
		label?: string;
		disabled?: boolean;
	}

	let {
		options = [],
		value = $bindable(''),
		placeholder = 'Select...',
		label,
		disabled = false,
		onValueChange
	}: Props = $props();

	let isOpen = $state(false);
	let searchQuery = $state('');
	let highlightedIndex = $state(0);
	let comboboxRef: HTMLDivElement;
	let inputRef = $state<HTMLInputElement>();
	
	// Generate unique id for accessibility
	const inputId = `combobox-${Math.random().toString(36).slice(2, 9)}`;

	// Filtered options based on search query
	const filteredOptions = $derived(() => {
		if (!searchQuery.trim()) {
			return options;
		}
		const query = searchQuery.toLowerCase();
		return options.filter(option => 
			option.toLowerCase().includes(query)
		);
	});

	function selectOption(option: string) {
		value = option;
		searchQuery = '';
		isOpen = false;
		onValueChange?.(option);
	}

	function clearSelection() {
		value = '';
		searchQuery = '';
		onValueChange?.('');
		inputRef?.focus();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (disabled) return;

		const filtered = filteredOptions();

		switch (e.key) {
			case 'ArrowDown':
				e.preventDefault();
				if (!isOpen) {
					isOpen = true;
				} else {
					highlightedIndex = Math.min(highlightedIndex + 1, filtered.length - 1);
				}
				break;
			case 'ArrowUp':
				e.preventDefault();
				highlightedIndex = Math.max(highlightedIndex - 1, 0);
				break;
			case 'Enter':
				e.preventDefault();
				if (isOpen && filtered.length > 0) {
					selectOption(filtered[highlightedIndex]);
				}
				break;
			case 'Escape':
				e.preventDefault();
				isOpen = false;
				searchQuery = '';
				break;
		}
	}

	function handleClickOutside(event: MouseEvent) {
		if (comboboxRef && !comboboxRef.contains(event.target as Node)) {
			isOpen = false;
			searchQuery = '';
		}
	}

	$effect(() => {
		if (typeof window !== 'undefined') {
			document.addEventListener('click', handleClickOutside);
			return () => {
				document.removeEventListener('click', handleClickOutside);
			};
		}
	});

	// Reset highlighted index when filtered options change
	$effect(() => {
		if (filteredOptions().length > 0) {
			highlightedIndex = 0;
		}
	});
</script>

<div bind:this={comboboxRef} class="relative">
	{#if label}
		<label for={inputId} class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1.5">
			{label}
		</label>
	{/if}

	<div class="relative">
		<!-- Display selected value or input for searching -->
		<div class="relative">
			{#if value && !isOpen}
				<button
					type="button"
					class="w-full px-3 py-2 pr-20 bg-white dark:bg-slate-800 border border-slate-300 dark:border-slate-600 rounded-lg text-left transition-colors {disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer hover:border-blue-500 dark:hover:border-blue-400'}"
					onclick={(e) => {
						if (!disabled) {
							isOpen = true;
						}
					}}
					disabled={disabled}
				>
					<span class="text-sm text-slate-900 dark:text-slate-100">{value}</span>
				</button>
				
				<!-- Icons positioned outside the button with pointer-events management -->
				<div class="absolute inset-y-0 right-0 flex items-center pr-3 gap-1 pointer-events-none">
					<div
						role="button"
						tabindex="-1"
						onclick={(e) => {
							e.preventDefault();
							e.stopPropagation();
							if (!disabled) {
								clearSelection();
							}
						}}
						onkeydown={(e) => {
							if (e.key === 'Enter' || e.key === ' ') {
								e.preventDefault();
								e.stopPropagation();
								if (!disabled) {
									clearSelection();
								}
							}
						}}
						class="p-0.5 rounded hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors cursor-pointer pointer-events-auto"
						title="Clear selection"
					>
						<X class="w-3.5 h-3.5 text-slate-500 dark:text-slate-400" />
					</div>
					<ChevronDown class="w-4 h-4 text-slate-500 dark:text-slate-400" />
				</div>
			{:else}
				<input
					id={inputId}
					bind:this={inputRef}
					type="text"
					bind:value={searchQuery}
					onfocus={() => !disabled && (isOpen = true)}
					onkeydown={handleKeydown}
					placeholder={placeholder}
					disabled={disabled}
					class="w-full px-3 py-2 pr-9 bg-white dark:bg-slate-800 border border-slate-300 dark:border-slate-600 rounded-lg text-sm text-slate-900 dark:text-slate-100 placeholder-slate-400 dark:placeholder-slate-500 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:opacity-50 disabled:cursor-not-allowed"
				/>
				<div class="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">
					<ChevronDown class="w-4 h-4 text-slate-500 dark:text-slate-400" />
				</div>
			{/if}
		</div>

		<!-- Dropdown -->
		{#if isOpen}
			<div class="absolute z-50 w-full mt-1 bg-white dark:bg-slate-800 border border-slate-300 dark:border-slate-600 rounded-lg shadow-lg max-h-60 overflow-auto">
				{#if filteredOptions().length === 0}
					<div class="px-3 py-2 text-sm text-slate-500 dark:text-slate-400 text-center">
						No options found
					</div>
				{:else}
					{#each filteredOptions() as option, index}
						<button
							type="button"
							onclick={() => selectOption(option)}
							class="w-full px-3 py-2 text-left text-sm transition-colors {index === highlightedIndex ? 'bg-blue-50 dark:bg-blue-900/30 text-blue-900 dark:text-blue-100' : 'text-slate-900 dark:text-slate-100 hover:bg-slate-100 dark:hover:bg-slate-700'}"
						>
							{option}
						</button>
					{/each}
				{/if}
			</div>
		{/if}
	</div>
</div>

<style>
	/* Custom scrollbar for dropdown */
	div::-webkit-scrollbar {
		width: 8px;
	}

	div::-webkit-scrollbar-track {
		background: transparent;
	}

	div::-webkit-scrollbar-thumb {
		background: #cbd5e1;
		border-radius: 4px;
	}

	:global(.dark) div::-webkit-scrollbar-thumb {
		background: #475569;
	}

	div::-webkit-scrollbar-thumb:hover {
		background: #94a3b8;
	}

	:global(.dark) div::-webkit-scrollbar-thumb:hover {
		background: #64748b;
	}
</style>

