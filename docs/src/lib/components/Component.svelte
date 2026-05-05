<script lang="ts">
	import { onMount } from 'svelte';
	import { Loader, AlertCircle } from 'lucide-svelte';
	import { loadComponentTemplate } from '$lib/viz/component-template-runtime';

	interface Props {
		component_name: string; // Component name to load (e.g., "Text", "FilterDropdown", "KPICard")
		data?: any[];
		x_dimension?: string;
		y_dimension?: string;
		y_metric?: string;
		x_metric?: string;
		metric_value?: any;
		dimension_value?: any;
		[key: string]: any;
	}

	let {
		component_name,
		data = [],
		x_dimension,
		y_dimension,
		y_metric,
		x_metric,
		metric_value,
		dimension_value,
		...restProps
	}: Props = $props();

	let isLoading = $state(true);
	let error = $state<string | null>(null);
	let templateData = $state<any>(null);
	let componentProps = $state<Record<string, any>>({});

	onMount(async () => {
		await loadComponent();
	});

	async function loadComponent() {
		isLoading = true;
		error = null;

		try {
			const template = await loadComponentTemplate(component_name);
			
			if (!template) {
				error = `Component "${component_name}" not found`;
				isLoading = false;
				return;
			}

			templateData = template;
			
			// Extract exported props from the template's html_code
			const extractedProps = extractExportedProps(template.html_code);
			
			// Build props object with values from restProps or defaults
			const props: Record<string, any> = {};
			
			for (const prop of extractedProps) {
				// Use passed prop if available, otherwise use default value
				if (restProps[prop.name] !== undefined) {
					props[prop.name] = restProps[prop.name];
				} else if (prop.name === 'data' && data) {
					props[prop.name] = data;
				} else {
					// Parse default value
					try {
						if (prop.value.startsWith('[') || prop.value.startsWith('{')) {
							props[prop.name] = JSON.parse(prop.value);
						} else if (prop.value.startsWith('"') || prop.value.startsWith("'")) {
							props[prop.name] = prop.value.slice(1, -1);
						} else {
							props[prop.name] = prop.value;
						}
					} catch {
						props[prop.name] = prop.value;
					}
				}
			}
			
			componentProps = props;
			isLoading = false;
		} catch (err) {
			console.error('Failed to load component:', err);
			error = err instanceof Error ? err.message : String(err);
			isLoading = false;
		}
	}
	
	// Extract exported props from component code
	function extractExportedProps(code: string): Array<{ name: string; value: string }> {
		const props: Array<{ name: string; value: string }> = [];
		
		try {
			// Extract script section
			const scriptMatch = code.match(/<script[^>]*>([\s\S]*?)<\/script>/i);
			if (!scriptMatch) return props;
			
			const scriptContent = scriptMatch[1];
			
			// Match export let declarations
			const exportRegex = /export\s+let\s+(\w+)(?:\s*=\s*(.+?))?;/g;
			
			let match;
			while ((match = exportRegex.exec(scriptContent)) !== null) {
				const [_, name, defaultValue] = match;
				props.push({
					name,
					value: defaultValue?.trim() || '""'
				});
			}
		} catch (e) {
			console.error('Error extracting props:', e);
		}
		
		return props;
	}
	
	// Get the clean HTML (without <script> tags)
	function getCleanHtml(code: string, props: Record<string, any>): string {
		// Remove <script> tags
		let html = code.replace(/<script[^>]*>[\s\S]*?<\/script>/gi, '');
		
		// Replace {variable} with actual values
		Object.keys(props).forEach(key => {
			const value = props[key];
			const placeholder = `{${key}}`;
			if (typeof value === 'string' || typeof value === 'number') {
				html = html.replace(new RegExp(placeholder.replace(/[{}]/g, '\\$&'), 'g'), String(value));
			}
		});
		
		return html.trim();
	}

	// Reload component when props change
	$effect(() => {
		// Watch for changes in key props
		component_name;
		data;
		x_dimension;
		y_metric;
		
		if (!isLoading) {
			loadComponent();
		}
	});
</script>

{#if isLoading}
	<div class="flex items-center justify-center h-full min-h-[200px]">
		<div class="text-center">
			<Loader class="w-8 h-8 text-indigo-500 animate-spin mx-auto mb-2" />
			<p class="text-sm text-slate-600 dark:text-slate-400">Loading component...</p>
		</div>
	</div>
{:else if error}
	<div class="flex items-center justify-center h-full min-h-[200px]">
		<div class="text-center max-w-md px-4">
			<AlertCircle class="w-12 h-12 text-red-400 mx-auto mb-3" />
			<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
				Component Error
			</h3>
			<p class="text-sm text-slate-600 dark:text-slate-400">
				{error}
			</p>
		</div>
	</div>
{:else if templateData}
	<div class="w-full h-full">
		{@html getCleanHtml(templateData.html_code, componentProps)}
	</div>
{/if}

