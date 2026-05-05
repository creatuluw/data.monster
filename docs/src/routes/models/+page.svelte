<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import Card from '$lib/components/Card.svelte';
	import { Database, Calculator, Tag, Code, LineChart, Network, Settings, FunctionSquare, Sparkles } from 'lucide-svelte';
</script>

<PageLayout>
	<!-- Navigation Cards -->
	<section class="mb-12">
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6 items-stretch">
			<Card
				title="Data loads"
				description="Manage data ingestion and settings"
				href="/models/data-loads"
				icon={Code}
				badge="Advanced"
				badgeColor="purple"
			/>
			<Card
				title="Data model"
				description="See how the data model is structured and related"
				href="/models/datamodel"
				icon={Sparkles}
				badge="Preview"
				badgeColor="green"
			/>
			<Card
				title="SQL Studio"
				description="Write and execute SQL queries"
				href="/sql-studio"
				icon={Code}
				badgeColor="purple"
			/>
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
				badgeColor="blue"
			/>
			<Card
				title="Dimensions"
				description="Configure dimension attributes for data analysis"
				href="/models/dimensions"
				icon={Tag}
				badgeColor="purple"
			/>
			<Card
				title="Functions"
				description="Create and manage user-defined functions (UDFs)"
				href="/models/functions"
				icon={FunctionSquare}
				badge="Advanced"
				badgeColor="purple"
			/>
			<Card
				title="Manage tables"
				description="View and delete tables from your data model"
				href="/models/manage"
				icon={Settings}
				badge="Core"
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
</PageLayout>

