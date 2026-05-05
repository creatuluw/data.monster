<script lang="ts">
	import { X, FileText } from 'lucide-svelte';
	
	interface Props {
		isOpen: boolean;
		record: any | null;
		columns: string[];
		recordIndex: number;
		onClose: () => void;
	}
	
	let { isOpen = $bindable(), record = $bindable(), columns, recordIndex, onClose }: Props = $props();
	
	// Format cell value for display
	function formatValue(value: any): string {
		if (value === null || value === undefined) {
			return 'NULL';
		}
		if (typeof value === 'object') {
			return JSON.stringify(value);
		}
		return String(value);
	}
</script>

<!-- Backdrop -->
{#if isOpen}
	<div 
		class="fixed inset-0 bg-black/50 z-40 transition-opacity"
		onclick={onClose}
		role="button"
		tabindex="0"
		onkeydown={(e) => e.key === 'Escape' && onClose()}
	></div>
{/if}

<!-- Drawer -->
<div 
	class="fixed right-0 top-0 bg-white dark:bg-slate-900 shadow-2xl z-50 transition-transform duration-300 ease-in-out border-l border-slate-200 dark:border-slate-800"
	class:translate-x-0={isOpen}
	class:translate-x-full={!isOpen}
	style="width: 50vw; height: 100vh;"
>
	<!-- Header -->
	<div class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-800 bg-slate-50 dark:bg-slate-800">
		<div class="flex items-center gap-2">
			<FileText class="w-5 h-5 text-purple-500 dark:text-purple-400" />
			<h2 class="text-lg font-semibold text-slate-900 dark:text-slate-100">
				Record #{recordIndex + 1}
			</h2>
		</div>
		<button
			onclick={onClose}
			class="p-2 hover:bg-slate-200 dark:hover:bg-slate-700 rounded transition-colors"
			aria-label="Close record view"
		>
			<X class="w-5 h-5 text-slate-600 dark:text-slate-400" />
		</button>
	</div>
	
	<!-- Content -->
	<div class="h-[calc(100%-65px)] overflow-auto">
		{#if record}
			<div class="divide-y divide-slate-200 dark:divide-slate-700">
				{#each columns as column}
					{@const value = record[column]}
					<div class="px-6 py-3 hover:bg-purple-50/50 dark:hover:bg-purple-950/20 transition-colors">
						<div class="grid grid-cols-3 gap-4 items-start">
							<!-- Column Name -->
							<div class="text-sm font-medium text-slate-700 dark:text-slate-300 break-words">
								{column}
							</div>
							
							<!-- Value -->
							<div class="col-span-2 text-sm text-slate-900 dark:text-slate-100 font-mono break-all">
								<span class:text-slate-400={value === null || value === undefined} class:italic={value === null || value === undefined}>
									{formatValue(value)}
								</span>
							</div>
						</div>
					</div>
				{/each}
			</div>
		{:else}
			<div class="flex items-center justify-center h-full">
				<div class="text-center">
					<FileText class="w-12 h-12 mx-auto mb-4 text-slate-300 dark:text-slate-600" />
					<p class="text-sm text-slate-600 dark:text-slate-400">No record selected</p>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	/* Add scrollbar styling */
	.overflow-auto {
		scrollbar-width: thin;
		scrollbar-color: #94a3b8 transparent;
	}
	
	.overflow-auto::-webkit-scrollbar {
		width: 8px;
		height: 8px;
	}
	
	.overflow-auto::-webkit-scrollbar-track {
		background: transparent;
	}
	
	.overflow-auto::-webkit-scrollbar-thumb {
		background-color: #94a3b8;
		border-radius: 4px;
	}
	
	.overflow-auto::-webkit-scrollbar-thumb:hover {
		background-color: #64748b;
	}
</style>

