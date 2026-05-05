<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Download } from 'lucide-svelte';
</script>

<PageLayout>
	<!-- Placeholder Content -->
	<section class="bg-slate-50 dark:bg-slate-800/30 rounded-lg border border-slate-200 dark:border-slate-700 p-8 text-center">
		<div class="max-w-2xl mx-auto">
			<Download class="w-16 h-16 mx-auto mb-4 text-slate-400 dark:text-slate-500" />
			<h3 class="text-xl font-aspekta font-[650] text-slate-900 dark:text-slate-100 mb-3">
				Data Export Coming Soon
			</h3>
			<p class="text-slate-600 dark:text-slate-400 mb-6">
				This feature is currently under development. Soon you'll be able to export your data to CSV, JSON, Parquet, and other formats.
			</p>
			<div class="inline-block px-4 py-2 bg-orange-50 dark:bg-orange-900/30 border border-orange-200 dark:border-orange-700 rounded-lg">
				<span class="text-sm font-medium text-orange-800 dark:text-orange-300">Coming Soon</span>
			</div>
		</div>
	</section>
</PageLayout>
