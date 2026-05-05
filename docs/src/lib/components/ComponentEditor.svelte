<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { Save, X, Loader, Component as ComponentIcon, AlertCircle, Sparkles, Database, BookOpen } from 'lucide-svelte';
	import Button from './Button.svelte';
	import SplitPane from './SplitPane.svelte';
	import CodeMirror from 'svelte-codemirror-editor';
	import { html as htmlLang } from '@codemirror/lang-html';
	import { oneDark } from '@codemirror/theme-one-dark';
	import { invoke } from '@tauri-apps/api/core';
	import ComponentTemplateBrowser from './ComponentTemplateBrowser.svelte';
	import Component from './Component.svelte';
	import SearchableDropdown from './SearchableDropdown.svelte';
	import type { ComponentTemplate } from '$lib/types/component-templates';
	import { AVAILABLE_FRAMEWORKS } from '$lib/types/component-templates';

	interface Props {
		initialHtml?: string;
		initialCss?: string;
		initialJs?: string;
		initialFrameworks?: string;
		initialMetrics?: string[];
		initialDimensions?: string[];
		onSave?: (html: string, css: string, js: string, frameworks: string, metrics: string[], dimensions: string[]) => Promise<void>;
		onClose?: () => void;
		availableMetrics?: Array<{ slug: string; metric_name: string }>;
		availableDimensions?: Array<{ slug: string; dimension_name: string; field_name: string; source_table: string }>;
		editingTemplate?: ComponentTemplate | null;
	}

	let {
		initialHtml = '<div class="bg-white dark:bg-slate-800 rounded-lg shadow-lg p-6">\n  <h3 class="text-lg font-semibold">My Component</h3>\n</div>',
		initialCss = '',
		initialJs = '',
		initialFrameworks = 'tailwind,alpinejs,heroicons', // All frameworks by default
		initialMetrics = [],
		initialDimensions = [],
		onSave,
		onClose,
		availableMetrics = [],
		availableDimensions = [],
		editingTemplate = null
	}: Props = $props();

	// Code state - unified code with HTML/CSS/JS
	let code = $state(buildInitialCode(initialHtml, initialCss, initialJs));
	let frameworks = $state(initialFrameworks);
	
	// Data binding state
	let selectedMetrics = $state<string[]>(initialMetrics);
	let selectedDimensions = $state<string[]>(initialDimensions);
	
	// Preview state
	let isRendering = $state(false);
	let isSaving = $state(false);
	let renderError = $state('');
	let isDark = $state(false);
	let lastRenderTime = $state(0);
	let autoPreviewTimeout: ReturnType<typeof setTimeout> | null = null;
	
	// Data loading state
	let isLoadingData = $state(false);
	let hasDataLoaded = $state(false);
	let loadedData = $state<any[]>([]);
	let loadedDataColumns = $state<{ dimension: string; metric: string }>({ dimension: '', metric: '' });
	let showDataDrawer = $state(false);
	let dataViewTab = $state<'json' | 'table'>('table');
	
	// Editor tabs
	let editorTab = $state<'create' | 'usage'>('create');
	let showTemplateBrowser = $state(false);
	let usageCode = $state('');
	
	// State for parsed props from Create tab
	let createProps = $state<{
		component_name: string;
		[key: string]: any;
	} | null>(null);
	
	// State for parsed usage props
	let usageProps = $state<{
		component_name: string;
		[key: string]: any;
	} | null>(null);
	
	// Build initial code from separate HTML/CSS/JS
	function buildInitialCode(html: string, css: string, js: string): string {
		let result = '';
		if (html) result += html;
		if (css) result += `\n\n<style>\n${css}\n</style>`;
		if (js) result += `\n\n<script>\n${js}\n<\/script>`;
		return result.trim();
	}
	
	// Compile Svelte template syntax to plain HTML
	function compileSvelteTemplate(html: string, props: any): string {
		let compiled = html;
		
		// Handle {#each} blocks
		const eachRegex = /\{#each\s+(\w+)\s+as\s+(\w+)(?:\s*,\s*(\w+))?\}([\s\S]*?)\{\/each\}/g;
		compiled = compiled.replace(eachRegex, (match, arrayName, itemName, indexName, content) => {
			const array = props?.[arrayName] || [];
			if (!Array.isArray(array)) {
				console.warn(`Array "${arrayName}" not found in props or is not an array`, props);
				return '';
			}
			
			return array.map((item, index) => {
				let itemHtml = content;
				// Replace item references like {tab.label}
				itemHtml = itemHtml.replace(new RegExp(`\\{${itemName}\\.(\\w+)\\}`, 'g'), (m, prop) => {
					return item[prop] || '';
				});
				// Replace {item}
				itemHtml = itemHtml.replace(new RegExp(`\\{${itemName}\\}`, 'g'), String(item));
				// Replace index
				if (indexName) {
					itemHtml = itemHtml.replace(new RegExp(`\\{${indexName}\\}`, 'g'), String(index));
				}
				// Handle conditional attributes like {tab.active ? 'selected' : ''}
				itemHtml = itemHtml.replace(new RegExp(`\\{${itemName}\\.(\\w+)\\s*\\?\\s*['"]([^'"]+)['"]\\s*:\\s*['"](.*?)['"]\\}`, 'g'), (m, prop, trueVal, falseVal) => {
					return item[prop] ? trueVal : falseVal;
				});
				// Handle conditional classes like {tab.active ? 'class1' : 'class2'}
				itemHtml = itemHtml.replace(new RegExp(`class="([^"]*?)\\{${itemName}\\.(\\w+)\\s*\\?\\s*'([^']+)'\\s*:\\s*'([^']*?)'\\}([^"]*?)"`, 'g'), (m, before, prop, trueVal, falseVal, after) => {
					const classValue = item[prop] ? trueVal : falseVal;
					return `class="${before}${classValue}${after}"`;
				});
				return itemHtml;
			}).join('');
		});
		
		// Handle {#if} blocks
		const ifRegex = /\{#if\s+([^}]+)\}([\s\S]*?)(?:\{:else\}([\s\S]*?))?\{\/if\}/g;
		compiled = compiled.replace(ifRegex, (match, condition, trueContent, falseContent) => {
			// Evaluate simple conditions
			let shouldShow = false;
			
			// Handle item.property checks
			const itemPropMatch = condition.match(/(\w+)\.(\w+)/);
			if (itemPropMatch && props) {
				const [_, obj, prop] = itemPropMatch;
				shouldShow = !!props[obj]?.[prop];
			}
			
			return shouldShow ? trueContent : (falseContent || '');
		});
		
		// Replace simple {variable} placeholders
		if (props) {
			Object.keys(props).forEach(key => {
				if (key !== 'component_name' && key !== 'data') {
					const value = props[key];
					const placeholder = new RegExp(`\\{${key}\\}`, 'g');
					if (typeof value === 'string' || typeof value === 'number') {
						compiled = compiled.replace(placeholder, String(value));
					}
				}
			});
		}
		
		return compiled;
	}
	
	// Derived preview iframe HTML - updates when code, frameworks, or data changes
	const previewIframeHtml = $derived.by(() => {
		const { html, css, js } = parseCode(code);
		
		// Get props to use (from usage tab or create tab)
		const props = editorTab === 'usage' ? usageProps : createProps;
		
		// Process Svelte syntax in HTML
		let processedHtml = compileSvelteTemplate(html, props);
		
		// Parse frameworks (comma-separated)
		const frameworkIds = frameworks.split(',').map(f => f.trim()).filter(f => f);
		
		// Build CDN script/link tags
		const frameworkTags = frameworkIds
			.map(id => {
				const framework = AVAILABLE_FRAMEWORKS.find(f => f.id === id);
				if (!framework) return '';
				
				if (id === 'tailwind') {
					return `<script src="${framework.cdn_url}"><\/script>`;
				} else if (id === 'alpinejs') {
					return `<script defer src="${framework.cdn_url}"><\/script>`;
				} else {
					return `<script src="${framework.cdn_url}"><\/script>`;
				}
			})
			.join('\n  ');

		// Build JavaScript that initializes all props as global variables for Alpine.js
		let processedJs = '';
		
		// First, extract all export let declarations and create window-level variables
		const exportLetMatches = [...js.matchAll(/export\s+let\s+(\w+)(?:\s*=\s*([^;]+))?;/g)];
		const propAssignments: string[] = [];
		
		exportLetMatches.forEach(match => {
			const propName = match[1];
			const defaultValue = match[2] || '""';
			
			if (props && props[propName] !== undefined) {
				const value = props[propName];
				const valueStr = typeof value === 'string' ? `'${value.replace(/'/g, "\\'")}'` : JSON.stringify(value);
				propAssignments.push(`window.${propName} = ${valueStr};`);
			} else {
				propAssignments.push(`window.${propName} = ${defaultValue};`);
			}
		});
		
		// Add prop assignments to the beginning of the script
		if (propAssignments.length > 0) {
			processedJs = propAssignments.join('\n') + '\n\n';
		}
		
		// Add the rest of the JS (without export let declarations)
		let cleanedJs = js.replace(/export\s+let\s+\w+(?:\s*=\s*[^;]+)?;/g, '');
		processedJs += cleanedJs;

		// Build iframe document
		const iframeDoc = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  ${frameworkTags}
  <style>
    * {
      margin: 0;
      padding: 0;
      box-sizing: border-box;
    }
    body {
      font-family: system-ui, -apple-system, sans-serif;
      padding: 16px;
    }
    ${css}
  </style>
</head>
<body>
  ${processedHtml}
  <script>
    // Inject data and props for component access
    window.componentData = ${JSON.stringify(hasDataLoaded ? loadedData : [])};
    window.componentProps = ${JSON.stringify(props || {})};
    
    // Custom component JavaScript
    ${processedJs}
  <\/script>
</body>
</html>`;

		// Return iframe tag with srcdoc
		return `<iframe 
			srcdoc="${iframeDoc.replace(/"/g, '&quot;')}" 
			class="w-full h-full border-0"
			style="min-height: 400px;"
			sandbox="allow-scripts"
		></iframe>`;
	});

	// Watch for theme changes
	onMount(() => {
		isDark = document.documentElement.classList.contains('dark');

		const observer = new MutationObserver(() => {
			isDark = document.documentElement.classList.contains('dark');
		});

		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ['class']
		});

		// Initial render
		renderPreview();

		return () => {
			observer.disconnect();
			if (autoPreviewTimeout) clearTimeout(autoPreviewTimeout);
		};
	});

	// Auto-preview with debounce when code changes
	$effect(() => {
		// Watch code changes
		code;
		
		if (autoPreviewTimeout) clearTimeout(autoPreviewTimeout);
		autoPreviewTimeout = setTimeout(() => {
			renderPreview();
		}, 800); // Debounce 800ms for hot reload feel
	});

	async function renderPreview() {
		isRendering = true;
		renderError = '';
		const startTime = Date.now();

		try {
			// Parse code to extract props for direct rendering
			const extractedProps = extractExportedProps(code);
			
			// Build props object with defaults
			const props: any = {
				component_name: editingTemplate?.component_type || 'MyComponent'
			};
			
			// Add extracted props with their default values
			for (const prop of extractedProps) {
				try {
					// Clean up the value by removing comments and trimming
					let cleanValue = prop.value
						.replace(/\/\/.*/g, '') // Remove single-line comments
						.replace(/\/\*[\s\S]*?\*\//g, '') // Remove multi-line comments
						.trim();
					
					// Evaluate JavaScript expressions (arrays, objects, etc.)
					if (cleanValue.startsWith('[') || cleanValue.startsWith('{')) {
						// Use Function constructor to safely evaluate JS expressions
						props[prop.name] = new Function(`return ${cleanValue}`)();
					} else if (cleanValue.startsWith('"') || cleanValue.startsWith("'")) {
						// String literals
						props[prop.name] = cleanValue.slice(1, -1);
					} else if (cleanValue === 'true' || cleanValue === 'false') {
						// Booleans
						props[prop.name] = cleanValue === 'true';
					} else if (!isNaN(Number(cleanValue))) {
						// Numbers
						props[prop.name] = Number(cleanValue);
					} else {
						// Otherwise keep as string
						props[prop.name] = cleanValue;
					}
				} catch (e) {
					console.error(`Error parsing prop ${prop.name}:`, e);
					// If parsing fails, use raw value
					props[prop.name] = prop.value.replace(/['"]/g, '');
				}
			}
			
			// Add data bindings if available
			if (hasDataLoaded && loadedData.length > 0) {
				props.data = loadedData;
				props.x_dimension = loadedDataColumns.dimension;
				props.y_metric = loadedDataColumns.metric;
				
				// If we have a single metric value, add it
				if (loadedData.length === 1 && loadedDataColumns.metric) {
					props.metric_value = loadedData[0][loadedDataColumns.metric];
				}
			}
			
			// Store props for rendering
			createProps = props;
			
			await tick();
			lastRenderTime = Date.now() - startTime;
			isRendering = false;
		} catch (error) {
			console.error('Render error:', error);
			renderError = error instanceof Error ? error.message : String(error);
			isRendering = false;
		}
	}
	
	// Parse unified code into HTML, CSS, and JS
	function parseCode(code: string): { html: string; css: string; js: string } {
		let html = code;
		let css = '';
		let js = '';
		
		// Extract <style> tags
		const styleRegex = /<style[^>]*>([\s\S]*?)<\/style>/gi;
		const styleMatches = code.matchAll(styleRegex);
		for (const match of styleMatches) {
			css += match[1] + '\n';
			html = html.replace(match[0], ''); // Remove from HTML
		}
		
		// Extract <script> tags
		const scriptRegex = /<script[^>]*>([\s\S]*?)<\/script>/gi;
		const scriptMatches = code.matchAll(scriptRegex);
		for (const match of scriptMatches) {
			js += match[1] + '\n';
			html = html.replace(match[0], ''); // Remove from HTML
		}
		
		return {
			html: html.trim(),
			css: css.trim(),
			js: js.trim()
		};
	}

	async function handleSave() {
		if (!onSave) return;

		try {
			isSaving = true;
			const { html, css, js } = parseCode(code);
			await onSave(html, css, js, frameworks, selectedMetrics, selectedDimensions);
		} finally {
			isSaving = false;
		}
	}

	function loadTemplate(template: ComponentTemplate) {
		code = buildInitialCode(template.html_code, template.css_code || '', template.js_code || '');
		frameworks = 'tailwind,alpinejs,heroicons'; // Always use all frameworks
		showTemplateBrowser = false;
		renderPreview();
	}

	function toggleMetric(slug: string) {
		if (selectedMetrics.includes(slug)) {
			selectedMetrics = selectedMetrics.filter(m => m !== slug);
		} else {
			selectedMetrics = [...selectedMetrics, slug];
		}
		loadMetricData();
	}

	function toggleDimension(slug: string) {
		if (selectedDimensions.includes(slug)) {
			selectedDimensions = selectedDimensions.filter(d => d !== slug);
		} else {
			selectedDimensions = [...selectedDimensions, slug];
		}
		loadMetricData();
	}

	async function loadMetricData() {
		const metric = availableMetrics.find(m => selectedMetrics.includes(m.slug));
		const dimension = availableDimensions.find(d => selectedDimensions.includes(d.slug));

		// Check if component requires dimensions
		const minDimensions = editingTemplate?.min_dimensions ?? 0;
		const requiresDimension = minDimensions > 0;

		if (!metric) {
			hasDataLoaded = false;
			loadedData = [];
			return;
		}

		try {
			isLoadingData = true;
			
			// If no dimension is needed (KPI component), execute metric with empty dimensions array
			if (!requiresDimension || !dimension) {
				try {
					// Use execute_metric_with_dimensions with empty dimensions array for aggregate metrics
					const result = await invoke<string>('execute_metric_with_dimensions', { 
						metricName: metric.metric_name,
						dimensions: [],
						filters: null
					});
					const response = JSON.parse(result);
					const newData = response.results || [];
					
					if (newData && newData.length > 0) {
						loadedData = newData;
						loadedDataColumns = {
							dimension: '',
							metric: metric.metric_name
						};
						hasDataLoaded = true;
					} else {
						hasDataLoaded = false;
						loadedData = [];
					}
				} catch (error) {
					console.error('Error loading metric:', error);
					hasDataLoaded = false;
					loadedData = [];
				}
				
				isLoadingData = false;
				return;
			}
			
			// Execute metric with dimensions
			const dimensionFullName = dimension.field_name.includes('.') 
				? dimension.field_name 
				: `${dimension.source_table}.${dimension.field_name}`;

			const result = await invoke<string>('execute_metric_with_dimensions', {
				metricName: metric.metric_name,
				dimensions: [dimensionFullName],
				filters: null
			});

			const response = JSON.parse(result);
			const newData = response.results || [];

			if (newData.length > 0) {
				const columns = Object.keys(newData[0]);
				
				const metricColumn = columns.find(col => 
					col.toLowerCase() === metric.metric_name.toLowerCase()
				) || columns[columns.length - 1];
				
				const dimParts = dimension.field_name.split('.');
				const dimFieldName = dimParts[dimParts.length - 1];
				const dimensionColumn = columns.find(col => 
					col.toLowerCase() === dimFieldName.toLowerCase()
				) || columns[0];

				loadedData = newData;
				loadedDataColumns = {
					dimension: dimensionColumn,
					metric: metricColumn
				};
				hasDataLoaded = true;
			} else {
				hasDataLoaded = false;
				loadedData = [];
			}
		} catch (error) {
			console.error('Error loading metric data:', error);
			hasDataLoaded = false;
			loadedData = [];
			renderError = `Error loading data: ${error}`;
		} finally {
			isLoadingData = false;
		}
	}
	
	// Generate usage code example
	function generateUsageCode(): string {
		const componentName = editingTemplate?.component_type || 'MyComponent';
		
		// Extract export let props from the component code
		const props = extractExportedProps(code);
		
		// Build usage example showing how to use Component wrapper
		let usage = `<script lang="ts">\n`;
		usage += `  import { Component } from '$lib/viz/dynamic-components';\n`;
		
		// Add data-related imports if metrics/dimensions are selected
		if (selectedMetrics.length > 0 || selectedDimensions.length > 0) {
			usage += `\n  // Data bindings from your data model:\n`;
			if (selectedMetrics.length > 0) {
				const metric = availableMetrics.find(m => selectedMetrics.includes(m.slug));
				if (metric) {
					usage += `  // Metric: ${metric.metric_name}\n`;
				}
			}
			if (selectedDimensions.length > 0) {
				const dimension = availableDimensions.find(d => selectedDimensions.includes(d.slug));
				if (dimension) {
					usage += `  // Dimension: ${dimension.field_name} (${dimension.dimension_name})\n`;
				}
			}
		}
		
		usage += `</scr` + `ipt>\n\n`;
		usage += `<Component\n`;
		usage += `  component_name="${componentName}"\n`;
		
		// Add data prop if data is loaded
		if (hasDataLoaded && loadedData.length > 0) {
			usage += `  data={data} <!-- Your data array from database -->\n`;
		}
		
		// Add each exported prop with its default value
		if (props.length > 0) {
			for (const prop of props) {
				// Show how to bind data fields to props
				if (hasDataLoaded && loadedData.length > 0) {
					// If prop name suggests it's data-related, show data binding example
					if (prop.name.toLowerCase().includes('value') && selectedMetrics.length > 0) {
						const metric = availableMetrics.find(m => selectedMetrics.includes(m.slug));
						usage += `  ${prop.name}={data[0]?.${metric?.metric_name || 'metric'}} <!-- From selected metric -->\n`;
					} else if (prop.name.toLowerCase().includes('label') && selectedDimensions.length > 0) {
						const dimension = availableDimensions.find(d => selectedDimensions.includes(d.slug));
						const fieldName = dimension?.field_name.split('.').pop() || 'field';
						usage += `  ${prop.name}={data[0]?.${fieldName}} <!-- From selected dimension -->\n`;
					} else {
						// Format prop value with single quotes for strings
						const formattedValue = prop.value.replace(/^["']|["']$/g, ''); // Remove quotes
						usage += `  ${prop.name}='${formattedValue}'\n`;
					}
				} else {
					// Format prop value with single quotes for strings
					const formattedValue = prop.value.replace(/^["']|["']$/g, ''); // Remove quotes
					usage += `  ${prop.name}='${formattedValue}'\n`;
				}
			}
		}
		
		usage += `/>\n`;
		
		// Add helpful note about data binding
		if (selectedMetrics.length > 0 || selectedDimensions.length > 0) {
			usage += `\n<!-- Note: Select metrics/dimensions below to see data binding examples -->\n`;
			usage += `<!-- You can access any field from your data model using table.field notation -->\n`;
		}
		
		return usage;
	}
	
	// Extract exported props from component code
	function extractExportedProps(code: string): Array<{ name: string; value: string; type?: string }> {
		const props: Array<{ name: string; value: string; type?: string }> = [];
		
		try {
			// Extract script section
			const scriptMatch = code.match(/<script[^>]*>([\s\S]*?)<\/script>/i);
			if (!scriptMatch) return props;
			
			const scriptContent = scriptMatch[1];
			
			// Find all export let declarations - handle multi-line values
			const lines = scriptContent.split('\n');
			let i = 0;
			
			while (i < lines.length) {
				const line = lines[i].trim();
				
				// Check if line starts with export let
				if (line.startsWith('export let')) {
					// Extract the variable name
					const nameMatch = line.match(/export\s+let\s+(\w+)/);
					if (nameMatch) {
						const name = nameMatch[1];
						
						// Check if it has a default value
						const equalIndex = line.indexOf('=');
						if (equalIndex !== -1) {
							// Has default value - collect until we find the semicolon
							let valueString = line.substring(equalIndex + 1);
							let depth = 0;
							let foundEnd = valueString.includes(';');
							
							// Count brackets to handle multi-line arrays/objects
							for (const char of valueString) {
								if (char === '[' || char === '{') depth++;
								if (char === ']' || char === '}') depth--;
							}
							
							// If we haven't found the end or depth > 0, continue to next lines
							while (!foundEnd || depth > 0) {
								i++;
								if (i >= lines.length) break;
								const nextLine = lines[i];
								valueString += '\n' + nextLine;
								
								for (const char of nextLine) {
									if (char === '[' || char === '{') depth++;
									if (char === ']' || char === '}') depth--;
								}
								
								if (nextLine.includes(';')) foundEnd = true;
							}
							
							// Clean up the value
							let value = valueString.replace(/;.*$/, '').trim();
							
							props.push({ name, value });
						} else if (line.includes(';')) {
							// No default value
							props.push({ name, value: '""' });
						}
					}
				}
				i++;
			}
		} catch (e) {
			console.error('Error extracting props:', e);
		}
		
		return props;
	}
	
	// Parse usage code to extract Component props
	function parseUsageCode(usageCode: string): {
		component_name: string;
		[key: string]: any;
	} | null {
		try {
			// Extract Component tag
			const componentMatch = usageCode.match(/<Component[\s\S]*?(?:\/?>|<\/Component>)/);
			if (!componentMatch) return null;
			
			const componentCode = componentMatch[0];
			
			// Extract component_name
			const componentNameMatch = componentCode.match(/component_name=["']([^"']+)["']/);
			if (!componentNameMatch) return null;
			
			const props: { component_name: string; [key: string]: any } = {
				component_name: componentNameMatch[1]
			};
			
			// Extract other props - handle both string values and expressions
			// Match patterns like: prop="value", prop='value', prop={value}, prop={expression}
			const propPattern = /(\w+)=(?:["']([^"']+)["']|\{([^}]+)\})/g;
			let match;
			while ((match = propPattern.exec(componentCode)) !== null) {
				const [_, propName, stringValue, exprValue] = match;
				if (propName !== 'component_name' && propName !== 'data') {
					// Use string value if present, otherwise evaluate expression
					if (stringValue !== undefined) {
						props[propName] = stringValue;
					} else if (exprValue !== undefined) {
						try {
							// Try to evaluate as JSON for simple values
							props[propName] = JSON.parse(exprValue);
						} catch {
							// Keep as string if not valid JSON
							props[propName] = exprValue;
						}
					}
				}
			}
			
			return props;
		} catch (e) {
			console.error('Error parsing usage code:', e);
			return null;
		}
	}
	
	// Watch for changes in usage code and parse it
	$effect(() => {
		if (editorTab === 'usage' && usageCode) {
			usageProps = parseUsageCode(usageCode);
			// Clear any render errors when switching to usage tab
			renderError = '';
		}
	});
	
	// Update usage code when selections change or tab switches
	$effect(() => {
		usageCode = generateUsageCode();
	});
</script>

<div class="flex flex-col h-full bg-white dark:bg-slate-900">
	<!-- Header -->
	<div class="flex items-center justify-between px-6 py-4 border-b border-slate-200 dark:border-slate-700">
		<div class="flex items-center gap-3">
			<h2 class="text-xl font-semibold text-slate-900 dark:text-slate-100">Component Builder</h2>
			{#if lastRenderTime > 0 && !renderError}
				<span class="text-xs text-slate-500 dark:text-slate-400 flex items-center gap-1">
					<Sparkles class="w-3 h-3" />
					Rendered in {lastRenderTime}ms
				</span>
			{/if}
		</div>
		<div class="flex items-center gap-2">
			<Button variant="ghost" size="sm" onclick={() => showTemplateBrowser = true}>
				<BookOpen class="w-4 h-4 mr-2" />
				Browse Templates
			</Button>
			{#if onSave}
				<Button variant="primary" size="sm" onclick={handleSave} disabled={isSaving}>
					{#if isSaving}
						<Loader class="w-4 h-4 mr-2 animate-spin" />
						Saving...
					{:else}
						<Save class="w-4 h-4 mr-2" />
						Save
					{/if}
				</Button>
			{/if}
			{#if onClose}
				<Button variant="ghost" size="sm" onclick={onClose}>
					<X class="w-4 h-4" />
				</Button>
			{/if}
		</div>
	</div>

	<!-- Main Content: Code Editor + Preview + Data Bindings -->
	<div class="flex flex-1 overflow-hidden">
		<SplitPane initialSize={50} minSize={30} maxSize={70}>
			{#snippet leftPanel()}
				<!-- Left: Code Editor with Tabs -->
				<div class="flex flex-col h-full">
					<!-- Editor Tabs -->
					<div class="flex border-b border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800">
						<button
							type="button"
							onclick={() => editorTab = 'create'}
							class="px-4 py-2.5 text-sm font-medium border-b-2 transition-colors {editorTab === 'create'
								? 'border-indigo-500 text-indigo-600 dark:text-indigo-400 bg-white dark:bg-slate-900'
								: 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'}"
						>
							Create
						</button>
						<button
							type="button"
							onclick={() => { editorTab = 'usage'; usageCode = generateUsageCode(); }}
							class="px-4 py-2.5 text-sm font-medium border-b-2 transition-colors {editorTab === 'usage'
								? 'border-indigo-500 text-indigo-600 dark:text-indigo-400 bg-white dark:bg-slate-900'
								: 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'}"
						>
							Usage
						</button>
					</div>

					<!-- CodeMirror Editor -->
					<div class="flex-1 overflow-auto codemirror-wrapper">
						{#if editorTab === 'create'}
							<CodeMirror
								bind:value={code}
								lang={htmlLang()}
								theme={isDark ? oneDark : undefined}
								styles={{
									'&': {
										width: '100%',
										height: '100%',
										fontSize: '14px'
									}
								}}
							/>
						{:else}
							<!-- Usage Tab - Editable -->
							<CodeMirror
								bind:value={usageCode}
								lang={htmlLang()}
								theme={isDark ? oneDark : undefined}
								styles={{
									'&': {
										width: '100%',
										height: '100%',
										fontSize: '14px'
									}
								}}
							/>
						{/if}
					</div>
				</div>
			{/snippet}

			{#snippet rightPanel()}
				<!-- Right: Preview + Data Bindings (Nested Vertical Split) -->
				<div class="flex flex-col h-full">
					<SplitPane initialSize={65} minSize={40} maxSize={85} orientation="vertical">
						{#snippet topPanel()}
							<!-- Preview Section -->
							<div class="flex flex-col h-full">
								<!-- Preview Toolbar -->
								<div class="px-4 py-2 bg-slate-50 dark:bg-slate-800 border-b border-slate-200 dark:border-slate-700 flex items-center justify-between">
									<span class="text-sm font-medium text-slate-700 dark:text-slate-300">Preview</span>
									<div class="flex items-center gap-3">
										<span class="text-xs text-slate-500">
											Data: {hasDataLoaded ? 'loaded' : 'not loaded'} | Rows: {loadedData.length}
										</span>
									</div>
								</div>

								<!-- Preview Area -->
								<div class="flex-1 overflow-auto bg-slate-50 dark:bg-slate-950 relative">
									{#if isLoadingData}
										<!-- Loading Data State -->
										<div class="absolute inset-0 flex items-center justify-center bg-slate-50 dark:bg-slate-950">
											<div class="text-center">
												<Loader class="w-12 h-12 text-indigo-500 animate-spin mx-auto mb-4" />
												<p class="text-sm text-slate-600 dark:text-slate-400">
													Loading data from database...
												</p>
											</div>
										</div>
									{:else if renderError}
										<!-- Error State -->
										<div class="absolute inset-0 flex items-center justify-center bg-slate-50 dark:bg-slate-950">
											<div class="max-w-md text-center">
												<div class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-red-100 dark:bg-red-900/20 mb-4">
													<AlertCircle class="w-8 h-8 text-red-600 dark:text-red-400" />
												</div>
												<h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">
													Unable to render component
												</h3>
												<div class="bg-red-50 dark:bg-red-900/10 border border-red-200 dark:border-red-800 rounded-lg p-3 text-left">
													<p class="text-xs font-mono text-red-700 dark:text-red-400 break-words whitespace-pre-wrap">
														{renderError}
													</p>
												</div>
											</div>
										</div>
									{:else if isRendering}
										<!-- Rendering State -->
										<div class="absolute inset-0 flex items-center justify-center bg-slate-50 dark:bg-slate-950">
											<div class="text-center">
												<Loader class="w-12 h-12 text-indigo-500 animate-spin mx-auto mb-4" />
												<p class="text-sm text-slate-600 dark:text-slate-400">
													Rendering your component...
												</p>
											</div>
										</div>
									{:else if (editorTab === 'usage' && usageProps) || (editorTab === 'create' && createProps)}
										<!-- Preview - Render directly in iframe (not from DB) -->
										<div class="h-full min-h-[400px] bg-white dark:bg-slate-900 rounded-lg border border-slate-200 dark:border-slate-800 overflow-hidden">
											{@html previewIframeHtml}
										</div>
									{:else}
										<!-- Initial Loading/Parsing State -->
										<div class="flex items-center justify-center h-full">
											<div class="text-center">
												<ComponentIcon class="w-12 h-12 text-slate-400 mx-auto mb-4" />
												<p class="text-sm text-slate-600 dark:text-slate-400">
													Preparing component preview...
												</p>
											</div>
										</div>
									{/if}
								</div>
							</div>
						{/snippet}

						{#snippet bottomPanel()}
							<!-- Data Bindings Panel -->
							<div class="border-t border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800 p-4 overflow-y-auto">
								<div class="flex items-center justify-between mb-3">
									<h3 class="text-sm font-semibold text-slate-900 dark:text-slate-100">Data Bindings</h3>
									{#if hasDataLoaded && loadedData.length > 0}
										<button
											type="button"
											onclick={() => showDataDrawer = true}
											class="text-xs px-2 py-1 rounded bg-indigo-50 dark:bg-indigo-900/20 text-indigo-600 dark:text-indigo-400 hover:bg-indigo-100 dark:hover:bg-indigo-900/30 transition-colors font-medium flex items-center gap-1"
										>
											<Database class="w-3 h-3" />
											View Data ({loadedData.length} rows)
										</button>
									{/if}
								</div>
								
								<!-- Metrics -->
								<div class="mb-4">
									<span class="text-xs font-medium text-slate-600 dark:text-slate-400 mb-2 block">
										Metrics {selectedMetrics.length > 0 ? `(${selectedMetrics.length} selected)` : ''}
									</span>
									{#if availableMetrics.length === 0}
										<p class="text-xs text-slate-500 italic">No metrics available</p>
									{:else}
										<SearchableDropdown
											options={availableMetrics.map(m => ({
												value: m.slug,
												label: m.metric_name,
												description: `Formula: ${m.formula || 'N/A'}`,
												category: 'Metric'
											}))}
											selected={selectedMetrics}
											placeholder="Select metrics..."
											onToggle={(slug) => toggleMetric(slug)}
											multiSelect={true}
										/>
										
										<!-- Selected Metrics with Preview Buttons -->
										{#if selectedMetrics.length > 0}
											<div class="flex flex-wrap gap-2 mt-2">
												{#each availableMetrics.filter(m => selectedMetrics.includes(m.slug)) as metric}
													<div class="flex items-center gap-1">
														<span class="px-2 py-1 text-xs rounded border bg-indigo-100 dark:bg-indigo-900/30 border-indigo-500 text-indigo-700 dark:text-indigo-400 font-medium">
															{metric.metric_name}
														</span>
														<button
															type="button"
															onclick={() => {
																// Load just this metric data
																const tempSelectedMetrics = selectedMetrics;
																const tempSelectedDimensions = selectedDimensions;
																selectedMetrics = [metric.slug];
																selectedDimensions = [];
																loadMetricData().then(() => {
																	if (hasDataLoaded) {
																		showDataDrawer = true;
																	}
																	// Restore original selections after preview
																	setTimeout(() => {
																		selectedMetrics = tempSelectedMetrics;
																		selectedDimensions = tempSelectedDimensions;
																		loadMetricData();
																	}, 100);
																});
															}}
															class="p-1 rounded bg-indigo-50 dark:bg-indigo-900/20 hover:bg-indigo-100 dark:hover:bg-indigo-900/30 text-indigo-600 dark:text-indigo-400 transition-colors"
															title="Preview data for this metric"
														>
															<Database class="w-3 h-3" />
														</button>
													</div>
												{/each}
											</div>
										{/if}
									{/if}
								</div>

								<!-- Dimensions -->
								<div>
									<span class="text-xs font-medium text-slate-600 dark:text-slate-400 mb-2 block">
										Dimensions {selectedDimensions.length > 0 ? `(${selectedDimensions.length} selected)` : ''}
									</span>
									{#if availableDimensions.length === 0}
										<p class="text-xs text-slate-500 italic">No dimensions available</p>
									{:else}
										<SearchableDropdown
											options={availableDimensions.map(d => ({
												value: d.slug,
												label: d.dimension_name,
												description: `${d.source_table}.${d.field_name}`,
												category: 'Dimension'
											}))}
											selected={selectedDimensions}
											placeholder="Select dimensions..."
											onToggle={(slug) => toggleDimension(slug)}
											multiSelect={true}
										/>
										
										<!-- Selected Dimensions with Preview Buttons -->
										{#if selectedDimensions.length > 0}
											<div class="flex flex-wrap gap-2 mt-2">
												{#each availableDimensions.filter(d => selectedDimensions.includes(d.slug)) as dimension}
													<div class="flex items-center gap-1">
														<span class="px-2 py-1 text-xs rounded border bg-purple-100 dark:bg-purple-900/30 border-purple-500 text-purple-700 dark:text-purple-400 font-medium">
															{dimension.dimension_name}
														</span>
														<button
															type="button"
															onclick={() => {
																// Load data with this dimension + selected metric
																if (selectedMetrics.length > 0) {
																	const tempSelectedMetrics = selectedMetrics;
																	const tempSelectedDimensions = selectedDimensions;
																	selectedDimensions = [dimension.slug];
																	loadMetricData().then(() => {
																		if (hasDataLoaded) {
																			showDataDrawer = true;
																		}
																		// Restore original selections after preview
																		setTimeout(() => {
																			selectedMetrics = tempSelectedMetrics;
																			selectedDimensions = tempSelectedDimensions;
																			loadMetricData();
																		}, 100);
																	});
																}
															}}
															class="p-1 rounded bg-purple-50 dark:bg-purple-900/20 hover:bg-purple-100 dark:hover:bg-purple-900/30 text-purple-600 dark:text-purple-400 transition-colors"
															title="Preview data for this dimension"
															disabled={selectedMetrics.length === 0}
														>
															<Database class="w-3 h-3" />
														</button>
													</div>
												{/each}
											</div>
										{/if}
									{/if}
								</div>
							</div>
						{/snippet}
					</SplitPane>
				</div>
			{/snippet}
		</SplitPane>
	</div>
</div>

<!-- Data Viewer Drawer -->
{#if showDataDrawer}
	<div 
		class="fixed inset-0 bg-black/50 flex items-end justify-end z-[70]" 
		onclick={() => showDataDrawer = false}
		role="button"
		tabindex="-1"
	>
		<div 
			class="bg-white dark:bg-slate-900 h-full w-[600px] shadow-2xl flex flex-col"
			onclick={(e) => e.stopPropagation()}
			role="dialog"
			aria-label="Data viewer"
			tabindex="0"
		>
			<!-- Drawer Header -->
			<div class="flex items-center justify-between px-6 py-4 border-b border-slate-200 dark:border-slate-700">
				<div>
					<h2 class="text-lg font-semibold text-slate-900 dark:text-slate-100">Data Viewer</h2>
					<p class="text-sm text-slate-600 dark:text-slate-400">
						{loadedData.length} rows × {Object.keys(loadedData[0] || {}).length} columns
					</p>
				</div>
				<button
					type="button"
					onclick={() => showDataDrawer = false}
					class="p-2 text-slate-400 hover:text-slate-600 dark:hover:text-slate-300 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
				>
					<X class="w-5 h-5" />
				</button>
			</div>

			<!-- Tabs -->
			<div class="flex border-b border-slate-200 dark:border-slate-700 px-6">
				<button
					type="button"
					onclick={() => dataViewTab = 'table'}
					class="px-4 py-3 text-sm font-medium border-b-2 transition-colors {dataViewTab === 'table'
						? 'border-indigo-500 text-indigo-600 dark:text-indigo-400'
						: 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'}"
				>
					Table
				</button>
				<button
					type="button"
					onclick={() => dataViewTab = 'json'}
					class="px-4 py-3 text-sm font-medium border-b-2 transition-colors {dataViewTab === 'json'
						? 'border-indigo-500 text-indigo-600 dark:text-indigo-400'
						: 'border-transparent text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100'}"
				>
					JSON
				</button>
			</div>

			<!-- Content -->
			<div class="flex-1 overflow-auto p-6">
				{#if dataViewTab === 'table'}
					<div class="overflow-x-auto">
						<table class="w-full text-sm">
							<thead>
								<tr class="border-b border-slate-200 dark:border-slate-700">
									{#each Object.keys(loadedData[0] || {}) as column}
										<th class="text-left py-3 px-4 font-semibold text-slate-900 dark:text-slate-100 bg-slate-50 dark:bg-slate-800">
											{column}
										</th>
									{/each}
								</tr>
							</thead>
							<tbody>
								{#each loadedData as row}
									<tr class="border-b border-slate-100 dark:border-slate-800 hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors">
										{#each Object.values(row) as value}
											<td class="py-3 px-4 text-slate-700 dark:text-slate-300">
												{typeof value === 'number' ? value.toLocaleString() : value}
											</td>
										{/each}
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				{:else}
					<div class="relative">
						<button
							type="button"
							onclick={() => navigator.clipboard.writeText(JSON.stringify(loadedData, null, 2))}
							class="absolute top-2 right-2 px-3 py-1 text-xs font-medium text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100 bg-slate-100 dark:bg-slate-800 rounded hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors"
						>
							Copy JSON
						</button>
						<pre class="bg-slate-900 dark:bg-black text-slate-100 p-4 rounded-lg overflow-x-auto text-xs leading-relaxed"><code>{JSON.stringify(loadedData, null, 2)}</code></pre>
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<!-- Template Browser Modal -->
{#if showTemplateBrowser}
	<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-[70] p-4">
		<div class="bg-white dark:bg-slate-900 rounded-lg shadow-2xl w-full max-w-6xl h-[80vh] overflow-hidden">
			<ComponentTemplateBrowser 
				onSelect={loadTemplate}
				onClose={() => showTemplateBrowser = false}
			/>
		</div>
	</div>
{/if}

<style>
	:global(.codemirror-wrapper .cm-editor) {
		height: 100% !important;
	}
	
	:global(.codemirror-wrapper .cm-scroller) {
		overflow: auto;
	}
</style>

