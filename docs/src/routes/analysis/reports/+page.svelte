<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Button from '$lib/components/Button.svelte';
	import { FileText, Plus } from 'lucide-svelte';
</script>

<PageLayout>
	<div class="flex items-center justify-between mb-6">
		<Button variant="primary" onclick={() => {}}>
			<Plus class="w-4 h-4 mr-2" />
			New Report
		</Button>
	</div>

	<!-- Placeholder for reports -->
	<div class="bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-lg p-12 text-center">
		<FileText class="w-16 h-16 mx-auto mb-4 text-slate-400 dark:text-slate-600" />
		<h2 class="font-aspekta text-xl font-[650] text-slate-900 dark:text-slate-100 mb-2">No Reports Yet</h2>
		<p class="text-slate-600 dark:text-slate-400 mb-6">Create your first report to get started</p>
		<a 
			href="/analysis/reports/create"
			class="inline-flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
		>
			<Plus class="w-4 h-4" />
			Create Report
		</a>
	</div>
</PageLayout>

