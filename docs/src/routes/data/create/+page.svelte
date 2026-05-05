<script lang="ts">
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { Plus, Table, FileSpreadsheet, List } from 'lucide-svelte';
</script>

<PageLayout>

	<!-- Coming Soon Message -->
	<div class="max-w-3xl mx-auto">
		<div class="p-8 bg-orange-50 dark:bg-orange-900/20 border border-orange-200 dark:border-orange-800 rounded-lg mb-8">
			<div class="flex items-start gap-4">
				<div class="p-3 bg-orange-100 dark:bg-orange-900/30 rounded-lg">
					<Plus class="w-8 h-8 text-orange-600 dark:text-orange-400" />
				</div>
				<div>
					<h2 class="text-xl font-semibold text-orange-900 dark:text-orange-300 mb-2">Feature Coming Soon</h2>
					<p class="text-orange-800 dark:text-orange-400 mb-4">
						Manual dataset creation features are currently in development. Soon you'll be able to create custom datasets from scratch.
					</p>
				</div>
			</div>
		</div>

		<!-- Planned Features -->
		<div class="space-y-6">
			<h3 class="text-xl font-semibold text-slate-900 dark:text-slate-100">Planned Features</h3>

			<!-- Spreadsheet Editor -->
			<div class="p-6 bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-lg">
				<div class="flex items-start gap-4">
					<FileSpreadsheet class="w-6 h-6 text-slate-600 dark:text-slate-400 flex-shrink-0 mt-1" />
					<div>
						<h4 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">Spreadsheet-Style Editor</h4>
						<p class="text-slate-600 dark:text-slate-400 mb-3">
							Create and edit data with a familiar spreadsheet interface.
						</p>
						<ul class="text-sm text-slate-600 dark:text-slate-400 space-y-1 ml-4">
							<li>• Define custom columns with data types</li>
							<li>• Add, edit, and delete rows</li>
							<li>• Import/export to CSV</li>
							<li>• Formula support for calculations</li>
							<li>• Data validation rules</li>
						</ul>
					</div>
				</div>
			</div>

			<!-- Table Builder -->
			<div class="p-6 bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-lg">
				<div class="flex items-start gap-4">
					<Table class="w-6 h-6 text-slate-600 dark:text-slate-400 flex-shrink-0 mt-1" />
					<div>
						<h4 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">Schema Designer</h4>
						<p class="text-slate-600 dark:text-slate-400 mb-3">
							Design your data structure with a visual schema builder.
						</p>
						<ul class="text-sm text-slate-600 dark:text-slate-400 space-y-1 ml-4">
							<li>• Visual column editor</li>
							<li>• Support for various data types (text, number, date, boolean)</li>
							<li>• Set default values and constraints</li>
							<li>• Define relationships between tables</li>
							<li>• Generate SQL DDL statements</li>
						</ul>
					</div>
				</div>
			</div>

			<!-- Form Builder -->
			<div class="p-6 bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-lg">
				<div class="flex items-start gap-4">
					<List class="w-6 h-6 text-slate-600 dark:text-slate-400 flex-shrink-0 mt-1" />
					<div>
						<h4 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">Form-Based Entry</h4>
						<p class="text-slate-600 dark:text-slate-400 mb-3">
							Enter data row-by-row using custom forms.
						</p>
						<ul class="text-sm text-slate-600 dark:text-slate-400 space-y-1 ml-4">
							<li>• Create custom input forms</li>
							<li>• Field validation and error handling</li>
							<li>• Dropdown selects and autocomplete</li>
							<li>• Date and time pickers</li>
							<li>• Batch entry mode</li>
						</ul>
					</div>
				</div>
			</div>

			<!-- Quick Templates -->
			<div class="p-6 bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-lg">
				<div class="flex items-start gap-4">
					<Plus class="w-6 h-6 text-slate-600 dark:text-slate-400 flex-shrink-0 mt-1" />
					<div>
						<h4 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">Dataset Templates</h4>
						<p class="text-slate-600 dark:text-slate-400 mb-3">
							Start with pre-built templates for common use cases.
						</p>
						<ul class="text-sm text-slate-600 dark:text-slate-400 space-y-1 ml-4">
							<li>• Sales transactions</li>
							<li>• Customer records</li>
							<li>• Inventory tracking</li>
							<li>• Event logs</li>
							<li>• Custom templates</li>
						</ul>
					</div>
				</div>
			</div>
		</div>

		<!-- Workaround Section -->
		<div class="mt-8 p-6 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg">
			<h3 class="text-lg font-semibold text-blue-900 dark:text-blue-300 mb-2">💡 Current Workaround</h3>
			<p class="text-sm text-blue-800 dark:text-blue-400 mb-3">
				While this feature is being developed, you can create datasets using:
			</p>
			<ul class="text-sm text-blue-800 dark:text-blue-400 space-y-1 ml-4">
				<li>• Create a CSV file in Excel or Google Sheets and upload it</li>
				<li>• Use SQL Studio to create tables with INSERT statements</li>
				<li>• Use the Data Model feature to create tables from SQL queries</li>
			</ul>
		</div>

		<!-- Feedback Section -->
		<div class="mt-6 p-6 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg">
			<h3 class="text-lg font-semibold text-green-900 dark:text-green-300 mb-2">Share Your Ideas</h3>
			<p class="text-sm text-green-800 dark:text-green-400">
				What kind of manual data entry features would be most useful for you? Your feedback helps us prioritize development.
			</p>
		</div>
	</div>
</PageLayout>


