<script lang="ts">
	import { tabs } from '$lib/stores/tabs';
	import { X } from 'lucide-svelte';

	function handleTabClick(id: string) {
		console.log('🔵 Tab clicked:', id);
		tabs.setActiveTab(id);
	}

	function handleCloseTab(e: MouseEvent, id: string) {
		e.stopPropagation();
		console.log('❌ Closing tab:', id);
		tabs.closeTab(id);
	}
</script>

<!-- Tab Bar styled like Breadcrumb -->
<div class="bg-[#F9F9F9] border-t border-b border-slate-200 dark:border-slate-700 dark:bg-slate-800">
	<div class="px-4 sm:px-6 lg:px-8 py-2 min-h-[36px]">
		<div class="flex items-center gap-2 overflow-x-auto">
			{#each $tabs as tab (tab.id)}
				{#if tab.id !== 'home'}
				<div
					class="group relative flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold transition-all whitespace-nowrap {tab.isActive
						? 'bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400'
						: 'bg-slate-100 text-slate-600 dark:bg-slate-700 dark:text-slate-400 hover:bg-slate-200 dark:hover:bg-slate-600'}"
				>
					<!-- svelte-ignore a11y_click_events_have_key_events -->
					<!-- svelte-ignore a11y_no_static_element_interactions -->
					<span
						onclick={() => handleTabClick(tab.id)}
						class="truncate max-w-[150px] cursor-pointer"
					>
						{tab.title}
					</span>
					{#if $tabs.length > 1}
						<button
							type="button"
							onclick={(e) => handleCloseTab(e, tab.id)}
							class="flex items-center justify-center w-3 h-3 rounded-full hover:bg-orange-200 dark:hover:bg-orange-800 transition-colors"
							aria-label="Close tab"
						>
							<X class="w-2.5 h-2.5" />
						</button>
					{/if}
				</div>
				{/if}
			{/each}
		</div>
	</div>
</div>
