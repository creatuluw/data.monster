<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Card from '$lib/components/Card.svelte';
	import { Presentation, Eye, FileText, Lightbulb } from 'lucide-svelte';
</script>

<PageLayout>
	<!-- Section 1: Perform Analysis -->
	<section class="mb-12">
		<h2 class="font-aspekta text-2xl font-[650] text-slate-900 dark:text-slate-100 mb-6">Perform Analysis</h2>
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 items-stretch">
			<Card
				title="Analysis Views"
				description="Browse and interact with existing analysis views and dashboards"
				href="/analysis/dashboards"
				icon={Eye}
			/>
			<Card
				title="View Slidedecks"
				description="View and present your existing slide decks and presentations"
				href="/analysis/slides"
				icon={Presentation}
			/>
			<Card
				title="Reports"
				description="Access and review generated reports and findings"
				href="/analysis/reports"
				icon={FileText}
			/>
			<Card
				title="Identify Action"
				description="Discover insights and recommended actions from your data"
				href="/analysis/insights"
				icon={Lightbulb}
			/>
		</div>
	</section>
</PageLayout>

