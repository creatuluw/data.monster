<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Card from '$lib/components/Card.svelte';
	import { Database, Calculator, Tag, Code, GitBranch, LineChart } from 'lucide-svelte';
</script>

<PageLayout>
	<!-- Quick Actions -->
			<section class="mb-12">
				<h2 class="font-aspekta text-xl font-[650] text-slate-900 dark:text-slate-100 mb-6">Quick Actions</h2>
				<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 items-stretch">
					<Card
						title="Tables"
						description="Add and manage data tables from your database"
						href="/models/tables"
						icon={Database}
						badge="Core"
						badgeColor="green"
					/>
					<Card
						title="Metrics"
						description="Define calculated measures and business metrics"
						href="/models/metrics"
						icon={Calculator}
						badge="Coming Soon"
						badgeColor="blue"
					/>
					<Card
						title="Dimensions"
						description="Configure dimension attributes for data analysis"
						href="/models/dimensions"
						icon={Tag}
						badge="Coming Soon"
						badgeColor="purple"
					/>
					<Card
						title="SQL Models"
						description="Write custom SQL transformations and data models"
						href="/models/sql"
						icon={Code}
						badge="Advanced"
						badgeColor="purple"
					/>
					<Card
						title="Relationships"
						description="Define relationships between tables and entities"
						href="/models/datamodel"
						icon={GitBranch}
						badge="Coming Soon"
						badgeColor="orange"
					/>
					<Card
						title="Analytics"
						description="View usage analytics and model performance"
						href="/models/analytics"
						icon={LineChart}
						badge="Coming Soon"
						badgeColor="orange"
					/>
				</div>
			</section>

			<!-- Placeholder Content -->
			<section class="bg-slate-50 dark:bg-slate-800/30 rounded-lg border border-slate-200 dark:border-slate-700 p-8 text-center">
				<div class="max-w-2xl mx-auto">
					<LineChart class="w-16 h-16 mx-auto mb-4 text-slate-400 dark:text-slate-500" />
					<h3 class="text-xl font-aspekta font-[650] text-slate-900 dark:text-slate-100 mb-3">
						Model Analytics Coming Soon
					</h3>
					<p class="text-slate-600 dark:text-slate-400 mb-6">
						This feature is currently under development. Soon you'll be able to view usage analytics and performance metrics for your data models.
					</p>
					<div class="inline-block px-4 py-2 bg-orange-50 dark:bg-orange-900/30 border border-orange-200 dark:border-orange-700 rounded-lg">
						<span class="text-sm font-medium text-orange-800 dark:text-orange-300">Coming Soon</span>
					</div>
			</div>
		</section>
</PageLayout>
