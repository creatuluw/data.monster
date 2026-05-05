<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Database, Upload, FileText, Code } from 'lucide-svelte';
</script>

<PageLayout title="Data loads">
	<div class="max-w-4xl">
		<div class="mb-8">
			<h2 class="text-2xl font-semibold mb-4">Custom SQL Transformations & Data Models</h2>
			<p class="text-gray-600 dark:text-gray-400 mb-6">
				Write and execute custom SQL to transform, load, and model your data. Create views, 
				materialized tables, and complex data transformations.
			</p>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
			<div class="p-6 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
				<div class="flex items-center mb-3">
					<Database class="w-5 h-5 text-purple-500 mr-2" />
					<h3 class="text-lg font-semibold">Data Ingestion</h3>
				</div>
				<p class="text-sm text-gray-600 dark:text-gray-400">
					Load data from external sources using SQL queries and transformations.
				</p>
			</div>

			<div class="p-6 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
				<div class="flex items-center mb-3">
					<Code class="w-5 h-5 text-purple-500 mr-2" />
					<h3 class="text-lg font-semibold">SQL Models</h3>
				</div>
				<p class="text-sm text-gray-600 dark:text-gray-400">
					Create reusable SQL models and views for your data transformations.
				</p>
			</div>

			<div class="p-6 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
				<div class="flex items-center mb-3">
					<Upload class="w-5 h-5 text-purple-500 mr-2" />
					<h3 class="text-lg font-semibold">ETL Workflows</h3>
				</div>
				<p class="text-sm text-gray-600 dark:text-gray-400">
					Build extract, transform, and load workflows with custom SQL logic.
				</p>
			</div>

			<div class="p-6 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700">
				<div class="flex items-center mb-3">
					<FileText class="w-5 h-5 text-purple-500 mr-2" />
					<h3 class="text-lg font-semibold">Documentation</h3>
				</div>
				<p class="text-sm text-gray-600 dark:text-gray-400">
					Document your data models and transformations for team collaboration.
				</p>
			</div>
		</div>

		<div class="p-6 bg-purple-50 dark:bg-purple-900/20 rounded-lg border border-purple-200 dark:border-purple-800">
			<h3 class="text-lg font-semibold mb-2">Coming Soon</h3>
			<p class="text-sm text-gray-600 dark:text-gray-400">
				This feature is currently under development. You'll be able to write custom SQL queries,
				create data transformation pipelines, and build sophisticated data models directly in the application.
			</p>
		</div>
	</div>
</PageLayout>
