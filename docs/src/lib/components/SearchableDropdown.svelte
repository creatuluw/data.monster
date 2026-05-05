<script lang="ts">
	import { Search, X } from 'lucide-svelte';
	
	interface Option {
		value: string;
		label: string;
		description?: string;
		category?: string;
	}
	
	interface Props {
		options: Option[];
		selected: string[];
		placeholder?: string;
		onToggle: (value: string) => void;
		multiSelect?: boolean;
		categoryColors?: Record<string, { bg: string; text: string; border: string }>;
	}
	
	let {
		options = [],
		selected = [],
		placeholder = 'Search...',
		onToggle,
		multiSelect = true,
		categoryColors = {
			metric: { bg: 'bg-indigo-50 dark:bg-indigo-900/20', text: 'text-indigo-700 dark:text-indigo-400', border: 'border-indigo-200 dark:border-indigo-800' },
			dimension: { bg: 'bg-purple-50 dark:bg-purple-900/20', text: 'text-purple-700 dark:text-purple-400', border: 'border-purple-200 dark:border-purple-800' }
		}
	}: Props = $props();
	
	let searchTerm = $state('');
	let isOpen = $state(false);
	let dropdownRef: HTMLDivElement;
	
	// Filter options based on search term
	const filteredOptions = $derived(
		searchTerm.trim() === ''
			? options
			: options.filter(opt =>
				opt.label.toLowerCase().includes(searchTerm.toLowerCase()) ||
				(opt.description && opt.description.toLowerCase().includes(searchTerm.toLowerCase())) ||
				(opt.category && opt.category.toLowerCase().includes(searchTerm.toLowerCase()))
			)
	);
	
	// Group options by category
	const groupedOptions = $derived(
		filteredOptions.reduce((acc, opt) => {
			const category = opt.category || 'Other';
			if (!acc[category]) acc[category] = [];
			acc[category].push(opt);
			return acc;
		}, {} as Record<string, Option[]>)
	);
	
	function handleClickOutside(event: MouseEvent) {
		if (dropdownRef && !dropdownRef.contains(event.target as Node)) {
			isOpen = false;
		}
	}
	
	function handleToggle(value: string) {
		onToggle(value);
		if (!multiSelect) {
			isOpen = false;
		}
	}
	
	$effect(() => {
		if (isOpen) {
			document.addEventListener('mousedown', handleClickOutside);
		} else {
			document.removeEventListener('mousedown', handleClickOutside);
		}
		
		return () => {
			document.removeEventListener('mousedown', handleClickOutside);
		};
	});
</script>

<div bind:this={dropdownRef} class="relative w-full">
	<!-- Trigger Button -->
	<button
		type="button"
		onclick={() => isOpen = !isOpen}
		class="w-full px-3 py-2 text-left border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 hover:bg-slate-50 dark:hover:bg-slate-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-colors flex items-center justify-between"
	>
		<span class="text-sm text-slate-600 dark:text-slate-400">
			{#if selected.length > 0}
				{selected.length} selected
			{:else}
				{placeholder}
			{/if}
		</span>
		<Search class="w-4 h-4 text-slate-400" />
	</button>
	
	<!-- Dropdown Panel -->
	{#if isOpen}
		<div class="absolute z-50 mt-2 w-full bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg shadow-xl max-h-[300px] overflow-hidden flex flex-col">
			<!-- Search Input -->
			<div class="p-2 border-b border-slate-200 dark:border-slate-700">
				<div class="relative">
					<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400" />
					<input
						type="text"
						bind:value={searchTerm}
						placeholder="Search fields..."
						class="w-full pl-9 pr-8 py-2 text-sm border border-slate-300 dark:border-slate-600 rounded-md bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-indigo-500"
						autofocus
					/>
					{#if searchTerm}
						<button
							type="button"
							onclick={() => searchTerm = ''}
							class="absolute right-2 top-1/2 -translate-y-1/2 p-1 hover:bg-slate-100 dark:hover:bg-slate-700 rounded"
						>
							<X class="w-3 h-3 text-slate-400" />
						</button>
					{/if}
				</div>
			</div>
			
			<!-- Options List -->
			<div class="overflow-y-auto flex-1">
				{#if filteredOptions.length === 0}
					<div class="p-4 text-center text-sm text-slate-500 dark:text-slate-400">
						No results found
					</div>
				{:else}
					{#each Object.entries(groupedOptions) as [category, categoryOptions]}
						<div class="border-b border-slate-100 dark:border-slate-800 last:border-b-0">
							<!-- Category Header -->
							<div class="px-3 py-2 bg-slate-50 dark:bg-slate-900">
								<span class="text-xs font-semibold text-slate-600 dark:text-slate-400 uppercase tracking-wider">
									{category}
								</span>
							</div>
							
							<!-- Category Options -->
							{#each categoryOptions as option}
								{@const isSelected = selected.includes(option.value)}
								{@const colors = categoryColors[category.toLowerCase()] || { bg: 'bg-slate-50', text: 'text-slate-700', border: 'border-slate-200' }}
								<button
									type="button"
									onclick={() => handleToggle(option.value)}
									class="w-full px-3 py-2 text-left hover:bg-slate-50 dark:hover:bg-slate-700 transition-colors flex items-start gap-2 {isSelected ? colors.bg : ''}"
								>
									<div class="flex-shrink-0 mt-0.5">
										<div class="w-4 h-4 rounded border {isSelected ? `${colors.border} ${colors.bg}` : 'border-slate-300 dark:border-slate-600'}">
											{#if isSelected}
												<svg class="w-full h-full {colors.text}" fill="currentColor" viewBox="0 0 16 16">
													<path d="M13.854 3.646a.5.5 0 0 1 0 .708l-7 7a.5.5 0 0 1-.708 0l-3.5-3.5a.5.5 0 1 1 .708-.708L6.5 10.293l6.646-6.647a.5.5 0 0 1 .708 0z"/>
												</svg>
											{/if}
										</div>
									</div>
									<div class="flex-1 min-w-0">
										<div class="text-sm font-medium text-slate-900 dark:text-slate-100">
											{option.label}
										</div>
										{#if option.description}
											<div class="text-xs text-slate-500 dark:text-slate-400 truncate">
												{option.description}
											</div>
										{/if}
									</div>
								</button>
							{/each}
						</div>
					{/each}
				{/if}
			</div>
		</div>
	{/if}
</div>

