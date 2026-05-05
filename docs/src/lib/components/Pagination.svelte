<script lang="ts">
	import Select from '$lib/components/Select.svelte';

	type Size = 'xs' | 'sm' | 'md' | 'lg';

	interface PageSizeOption {
		value: string;
		label: string;
	}

	interface Props {
		currentPage: number;
		totalItems: number;
		pageSize: number;
		pageSizeOptions?: PageSizeOption[];
		size?: Size;
		onPageChange: (page: number) => void;
		onPageSizeChange?: (pageSize: number) => void;
	}

	let {
		currentPage = $bindable(),
		totalItems,
		pageSize = $bindable(),
		pageSizeOptions = [
			{ value: '10', label: '10 per page' },
			{ value: '20', label: '20 per page' },
			{ value: '50', label: '50 per page' },
			{ value: '100', label: '100 per page' }
		],
		size = 'md',
		onPageChange,
		onPageSizeChange
	}: Props = $props();

	const totalPages = $derived(Math.ceil(totalItems / pageSize));
	const startIndex = $derived((currentPage - 1) * pageSize + 1);
	const endIndex = $derived(Math.min(currentPage * pageSize, totalItems));

	const sizeClasses = {
		xs: {
			container: 'gap-2 text-xs',
			button: 'xs',
			select: 'xs',
			text: 'text-xs'
		},
		sm: {
			container: 'gap-2 text-xs',
			button: 'xs',
			select: 'xs',
			text: 'text-xs'
		},
		md: {
			container: 'gap-3 text-sm',
			button: 'sm',
			select: 'sm',
			text: 'text-sm'
		},
		lg: {
			container: 'gap-4 text-base',
			button: 'md',
			select: 'md',
			text: 'text-base'
		}
	};

	const currentSize = $derived(sizeClasses[size]);

	function handlePageSizeChange(newSize: string) {
		const newPageSize = parseInt(newSize);
		pageSize = newPageSize;
		currentPage = 1;
		if (onPageSizeChange) {
			onPageSizeChange(newPageSize);
		}
		if (onPageChange) {
			onPageChange(1);
		}
	}

	function goToPage(page: number) {
		if (page < 1 || page > totalPages) return;
		currentPage = page;
		if (onPageChange) {
			onPageChange(page);
		}
	}
</script>

<div class="flex flex-col sm:flex-row items-center justify-between {currentSize.container} py-3 px-6">
	<!-- Results info and page size selector -->
	<div class="flex items-center {currentSize.container}">
		<span class="{currentSize.text} text-slate-600 dark:text-slate-400 whitespace-nowrap">
			Showing {startIndex} to {endIndex} of {totalItems}
		</span>
		{#if onPageSizeChange}
			<Select
				value={pageSize.toString()}
				options={pageSizeOptions}
				size={currentSize.select as 'xs' | 'sm' | 'md' | 'lg' | 'xl'}
				onchange={handlePageSizeChange}
			/>
		{/if}
	</div>
	
	<!-- Pagination buttons -->
	<div class="flex items-center {currentSize.container}">
		<button
			disabled={currentPage === 1}
			onclick={() => goToPage(1)}
			class="px-2 py-1 {currentSize.text} text-slate-700 dark:text-slate-300 border border-slate-200 dark:border-slate-700 rounded hover:border-slate-400 dark:hover:border-slate-500 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:border-slate-200 dark:disabled:hover:border-slate-700 transition-colors"
		>
			First
		</button>
		<button
			disabled={currentPage === 1}
			onclick={() => goToPage(currentPage - 1)}
			class="px-2 py-1 {currentSize.text} text-slate-700 dark:text-slate-300 border border-slate-200 dark:border-slate-700 rounded hover:border-slate-400 dark:hover:border-slate-500 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:border-slate-200 dark:disabled:hover:border-slate-700 transition-colors"
		>
			Previous
		</button>
		<span class="{currentSize.text} text-slate-600 dark:text-slate-400 px-2 whitespace-nowrap">
			Page {currentPage} of {totalPages}
		</span>
		<button
			disabled={currentPage === totalPages}
			onclick={() => goToPage(currentPage + 1)}
			class="px-2 py-1 {currentSize.text} text-slate-700 dark:text-slate-300 border border-slate-200 dark:border-slate-700 rounded hover:border-slate-400 dark:hover:border-slate-500 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:border-slate-200 dark:disabled:hover:border-slate-700 transition-colors"
		>
			Next
		</button>
		<button
			disabled={currentPage === totalPages}
			onclick={() => goToPage(totalPages)}
			class="px-2 py-1 {currentSize.text} text-slate-700 dark:text-slate-300 border border-slate-200 dark:border-slate-700 rounded hover:border-slate-400 dark:hover:border-slate-500 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:border-slate-200 dark:disabled:hover:border-slate-700 transition-colors"
		>
			Last
		</button>
	</div>
</div>

