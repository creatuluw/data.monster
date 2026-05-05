<script lang="ts">
	import { X, Inbox } from 'lucide-svelte';
	
	interface SystemMessage {
		id: string;
		message: string;
		timestamp: Date;
		type: 'info' | 'success' | 'error' | 'warning';
	}
	
	let { 
		messages = $bindable<SystemMessage[]>([]),
		isOpen = $bindable(false)
	} = $props();
	
	function closeDrawer() {
		isOpen = false;
	}
	
	function clearMessages() {
		messages = [];
	}
	
	function getMessageIcon(type: SystemMessage['type']) {
		switch (type) {
			case 'success':
				return '✓';
			case 'error':
				return '✗';
			case 'warning':
				return '⚠';
			default:
				return 'ℹ';
		}
	}
	
	function getMessageColor(type: SystemMessage['type']) {
		switch (type) {
			case 'success':
				return 'text-green-700 bg-green-50 dark:bg-green-900/20 dark:text-green-400';
			case 'error':
				return 'text-red-700 bg-red-50 dark:bg-red-900/20 dark:text-red-400';
			case 'warning':
				return 'text-orange-700 bg-orange-50 dark:bg-orange-900/20 dark:text-orange-400';
			default:
				return 'text-blue-700 bg-blue-50 dark:bg-blue-900/20 dark:text-blue-400';
		}
	}
	
	function formatTime(date: Date) {
		return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
	}
</script>

{#if isOpen}
	<div class="fixed inset-0 z-[60]">
		<!-- Backdrop -->
		<div 
			class="absolute inset-0 bg-black/20 backdrop-blur-sm transition-opacity"
			onclick={closeDrawer}
			onkeydown={(e) => { if (e.key === 'Escape') closeDrawer(); }}
			role="button"
			tabindex="-1"
		></div>
		
		<!-- Drawer -->
		<div class="absolute inset-0 pl-10 sm:pl-16 pointer-events-none">
			<div class="ml-auto block size-full max-w-md transform transition-transform duration-500 ease-in-out pointer-events-auto">
				<div class="relative flex h-full flex-col overflow-y-auto bg-white shadow-xl dark:bg-gray-800 dark:after:absolute dark:after:inset-y-0 dark:after:left-0 dark:after:w-px dark:after:bg-white/10">
					<!-- Header -->
					<div class="px-4 py-6 sm:px-6 border-b border-gray-200 dark:border-gray-700">
						<div class="flex items-start justify-between">
							<h2 class="text-base font-semibold text-gray-900 dark:text-white">System Messages</h2>
							<div class="ml-3 flex h-7 items-center gap-2">
								{#if messages.length > 0}
									<button
										type="button"
										onclick={clearMessages}
										class="cursor-pointer rounded-md px-2 py-1 text-xs font-medium text-gray-600 hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-700 transition-colors"
									>
										Clear All
									</button>
								{/if}
								<button
									type="button"
									onclick={closeDrawer}
									class="cursor-pointer relative rounded-md text-gray-400 hover:text-gray-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 dark:hover:text-white dark:focus-visible:outline-blue-500"
								>
									<span class="sr-only">Close panel</span>
									<X class="size-6" />
								</button>
							</div>
						</div>
						<p class="mt-1 text-sm text-gray-500 dark:text-gray-400">
							{messages.length} {messages.length === 1 ? 'message' : 'messages'}
						</p>
					</div>
					
					<!-- Content -->
					<div class="relative flex-1 px-4 py-4 sm:px-6 overflow-y-auto">
					{#if messages.length === 0}
						<div class="flex flex-col items-center justify-center h-full text-center py-12">
							<Inbox class="w-16 h-16 text-gray-300 dark:text-gray-600 mb-4" strokeWidth={1.5} />
							<p class="text-gray-500 dark:text-gray-400 font-medium">No system messages</p>
							<p class="text-sm text-gray-400 dark:text-gray-500 mt-1">Messages will appear here when actions occur</p>
						</div>
						{:else}
							<div class="space-y-3">
								{#each messages as msg (msg.id)}
									<div class="rounded-lg p-3 {getMessageColor(msg.type)} border border-current/20">
										<div class="flex items-start gap-2">
											<span class="text-lg font-semibold">{getMessageIcon(msg.type)}</span>
											<div class="flex-1 min-w-0">
												<p class="text-sm font-medium break-words">{msg.message}</p>
												<p class="text-xs opacity-70 mt-1">{formatTime(msg.timestamp)}</p>
											</div>
										</div>
									</div>
								{/each}
							</div>
						{/if}
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}

