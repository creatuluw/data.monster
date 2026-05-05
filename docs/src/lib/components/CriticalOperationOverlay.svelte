<script lang="ts">
	interface Props {
		isVisible: boolean;
		operation: string;
		showWarning?: boolean;
		showProgress?: boolean;
		progressPercent?: number;
		progressMessage?: string;
		downloadedMb?: number;
		totalMb?: number;
	}
	
	let { 
		isVisible = false, 
		operation = 'Processing', 
		showWarning = true,
		showProgress = false,
		progressPercent = 0,
		progressMessage = '',
		downloadedMb = 0,
		totalMb = 0
	}: Props = $props();
</script>

{#if isVisible}
	<div class="fixed inset-0 z-[9999] flex items-center justify-center bg-black/50 backdrop-blur-sm">
		<div class="bg-white dark:bg-slate-800 rounded-lg shadow-2xl p-8 max-w-md mx-4 border-2 border-blue-500 dark:border-blue-400">
			<!-- Spinner -->
			<div class="flex justify-center mb-6">
				<div class="relative">
					<div class="w-16 h-16 border-4 border-blue-200 dark:border-blue-900 rounded-full"></div>
					<div class="w-16 h-16 border-4 border-blue-500 dark:border-blue-400 border-t-transparent rounded-full animate-spin absolute top-0 left-0"></div>
				</div>
			</div>
			
			<!-- Operation Text -->
			<div class="text-center mb-6">
				<h3 class="text-xl font-semibold text-slate-800 dark:text-white mb-2">
					{operation}
				</h3>
				<p class="text-sm text-slate-600 dark:text-slate-400">
					Please wait...
				</p>
			</div>
			
			<!-- Progress Bar (shown when showProgress is true) -->
			{#if showProgress}
				<div class="mb-6">
					<div class="flex justify-between items-center mb-2">
						<span class="text-sm font-medium text-slate-700 dark:text-slate-300">
							{progressMessage || 'Downloading...'}
						</span>
						<span class="text-sm font-semibold text-blue-600 dark:text-blue-400">
							{progressPercent.toFixed(0)}%
						</span>
					</div>
					<div class="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-3 overflow-hidden">
						<div 
							class="bg-gradient-to-r from-blue-500 to-blue-600 h-full rounded-full transition-all duration-300 ease-out"
							style="width: {progressPercent}%"
						></div>
					</div>
					{#if totalMb > 0}
						<div class="flex justify-between items-center mt-2 text-xs text-slate-500 dark:text-slate-400">
							<span>{downloadedMb.toFixed(1)} MB</span>
							<span>{totalMb.toFixed(1)} MB</span>
						</div>
					{/if}
				</div>
			{/if}
			
			{#if showWarning}
				<!-- Warning Box -->
				<div class="bg-amber-50 dark:bg-amber-900/20 border-2 border-amber-400 dark:border-amber-600 rounded-lg p-4">
					<div class="flex items-start gap-3">
						<svg class="w-6 h-6 text-amber-600 dark:text-amber-400 flex-shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
						</svg>
						<div class="flex-1">
							<h4 class="font-semibold text-amber-800 dark:text-amber-300 mb-1 text-sm">
								Do Not Close the Application
							</h4>
							<p class="text-xs text-amber-700 dark:text-amber-400">
								{#if showProgress}
									This is a one-time download for demo purposes. The app is loading 1.2M+ sample records to showcase analytics capabilities.
								{:else}
									Closing the app during this operation may corrupt your database. Please wait until it completes.
								{/if}
							</p>
						</div>
					</div>
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}
	
	.animate-spin {
		animation: spin 1s linear infinite;
	}
</style>

