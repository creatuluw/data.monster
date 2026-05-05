<script lang="ts" generics="T">
	import { createEventDispatcher } from 'svelte';
	import { Check, ChevronDown, X } from 'lucide-svelte';

	interface Props<T> {
		options?: T[];
		value?: T | T[] | null;
		displayField?: keyof T | ((item: T) => string);
		placeholder?: string;
		label?: string;
		disabled?: boolean;
		required?: boolean;
		multiple?: boolean;
		className?: string;
		helpText?: string;
	}

	const {
		options = [],
		value = null,
		displayField = ((item: T) => String(item)) as keyof T | ((item: T) => string),
		placeholder = 'Select...',
		label = '',
		disabled = false,
		required = false,
		multiple = false,
		className = '',
		helpText = ''
	}: Props<T> = $props();

	const dispatch = createEventDispatcher<{
		select: T | T[];
		change: T | T[] | null;
	}>();

	let isOpen = $state(false);
	let dropdownRef: HTMLDivElement;
	
	// Generate unique id for accessibility
	const selectId = `dropdown-select-${Math.random().toString(36).slice(2, 9)}`;

	function getDisplayValue(item: T): string {
		return typeof displayField === 'function'
			? displayField(item)
			: String(item[displayField]);
	}

	function isSelected(option: T): boolean {
		if (multiple && Array.isArray(value)) {
			return value.includes(option);
		}
		return value === option;
	}

	function selectOption(option: T) {
		if (multiple) {
			const currentValue = Array.isArray(value) ? value : [];
			let newValue: T[];
			
			if (currentValue.includes(option)) {
				// Remove if already selected
				newValue = currentValue.filter(v => v !== option);
			} else {
				// Add if not selected
				newValue = [...currentValue, option];
			}
			
			dispatch('select', newValue);
			dispatch('change', newValue);
		} else {
			dispatch('select', option);
			dispatch('change', option);
			isOpen = false;
		}
	}

	function clearSelection() {
		dispatch('change', null);
		isOpen = false;
	}

	function getSelectedDisplay(): string {
		if (!value) return placeholder;
		
		if (multiple && Array.isArray(value)) {
			if (value.length === 0) return placeholder;
			if (value.length === 1) return getDisplayValue(value[0]);
			return `${value.length} selected`;
		}
		
		return getDisplayValue(value as T);
	}

	function handleClickOutside(event: MouseEvent) {
		if (dropdownRef && !dropdownRef.contains(event.target as Node)) {
			isOpen = false;
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
</script>

<div bind:this={dropdownRef} class="relative {className}">
	{#if label}
		<label for={selectId} class="block text-sm/6 font-medium text-gray-900 dark:text-white mb-2">
			{label}
			{#if required}
				<span class="text-red-500">*</span>
			{/if}
		</label>
	{/if}

	<div class="grid grid-cols-1">
		<button
			id={selectId}
			type="button"
			onclick={() => !disabled && (isOpen = !isOpen)}
			disabled={disabled}
			class="col-start-1 row-start-1 w-full appearance-none rounded-md bg-white py-1.5 pr-8 pl-3 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-indigo-600 sm:text-sm/6 dark:bg-white/5 dark:text-white dark:outline-white/10 dark:focus-visible:outline-indigo-500 text-left transition-colors {disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}"
		>
			<span class={value ? '' : 'text-gray-500 dark:text-gray-400'}>
				{getSelectedDisplay()}
			</span>
		</button>
		
		<div class="col-start-1 row-start-1 mr-2 self-center justify-self-end flex items-center gap-1 pointer-events-none">
			{#if value && (multiple ? (Array.isArray(value) && value.length > 0) : true)}
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
					class="pointer-events-auto p-0.5 rounded hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors cursor-pointer"
					title="Clear selection"
				>
					<X class="size-4 text-gray-500 dark:text-gray-400" />
				</div>
			{/if}
			<ChevronDown class="size-5 text-gray-500 sm:size-4 dark:text-gray-400 transition-transform {isOpen ? 'rotate-180' : ''}" />
		</div>
	</div>

	{#if helpText}
		<p class="text-xs text-slate-500 dark:text-slate-400 mt-1">{helpText}</p>
	{/if}

	{#if isOpen}
		<div class="absolute z-50 mt-1 w-full max-h-60 overflow-auto rounded-md bg-white py-1 text-base shadow-lg outline outline-1 -outline-offset-1 outline-black/5 sm:text-sm dark:bg-gray-800 dark:shadow-none dark:outline-white/10">
			{#if options.length === 0}
				<div class="px-3 py-2 text-gray-500 dark:text-gray-400 text-sm">
					No options available
				</div>
			{:else}
				{#each options as option}
					<button
						type="button"
						onclick={() => selectOption(option)}
						class="group relative w-full text-left px-3 py-2 text-gray-900 select-none hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700 {isSelected(option) ? 'bg-indigo-50 dark:bg-indigo-900/30' : ''} transition-colors"
					>
						<span class="block {multiple ? 'pr-8' : 'pr-6'}">
							{getDisplayValue(option)}
						</span>
						{#if isSelected(option)}
							<span class="absolute right-3 top-1/2 -translate-y-1/2">
								<Check class="w-4 h-4 text-indigo-600 dark:text-indigo-400" />
							</span>
						{/if}
					</button>
				{/each}
			{/if}
		</div>
	{/if}
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

