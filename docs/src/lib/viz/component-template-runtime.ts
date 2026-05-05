/**
 * Component Template Runtime
 * 
 * Dynamically loads HTML/CSS/JS component templates from DuckDB and
 * renders them in an iframe sandbox with data binding support.
 * 
 * Flow:
 * 1. Load templates from DB (component_templates table)
 * 2. Parse template code and extract variable placeholders
 * 3. Replace template variables with actual props/data
 * 4. Render in iframe with framework CDN links
 */

import { invoke } from '@tauri-apps/api/core';
import type { ComponentTemplate, ComponentTemplateProps } from '$lib/types/component-templates';
import { AVAILABLE_FRAMEWORKS } from '$lib/types/component-templates';

/**
 * Load all component templates from the database
 */
export async function loadComponentTemplates(): Promise<ComponentTemplate[]> {
	try {
		const result = await invoke<string>('list_component_templates');
		return JSON.parse(result);
	} catch (error) {
		console.error('Failed to load component templates:', error);
		return [];
	}
}

/**
 * Load a single component template by slug or component_type
 */
export async function loadComponentTemplate(
	identifier: string
): Promise<ComponentTemplate | null> {
	try {
		// Try loading by slug first
		const result = await invoke<string>('get_component_template', { slug: identifier });
		return JSON.parse(result);
	} catch (error) {
		// If that fails, try searching by component_type
		try {
			const allTemplates = await loadComponentTemplates();
			return allTemplates.find(t => t.component_type === identifier) || null;
		} catch (e) {
			console.error('Failed to load component template:', e);
			return null;
		}
	}
}

/**
 * Compile component template with data and props
 * Replaces template variables with actual values
 */
export function compileComponentTemplate(
	template: ComponentTemplate,
	props: ComponentTemplateProps,
	data?: any[]
): string {
	console.log('🔧 Compiling template:', template.component_type, 'with props:', props);
	
	let html = template.html_code;
	let css = template.css_code || '';
	let js = template.js_code || '';

	// Convert Svelte-style script to plain JavaScript
	const { convertedHtml, convertedJs } = convertSvelteToPlainJS(html, js, props, data);
	html = convertedHtml;
	js = convertedJs;
	
	console.log('📝 After Svelte conversion - HTML:', convertedHtml);
	console.log('📝 After Svelte conversion - JS:', convertedJs);

	// Replace template variables in HTML
	html = replaceTemplateVariables(html, props, data);
	css = replaceTemplateVariables(css, props, data);
	js = replaceTemplateVariables(js, props, data);

	// Build complete HTML document with iframe structure
	const result = buildIframeDocument(html, css, js, template.frameworks || '', props, data);
	console.log('✅ Final compiled HTML length:', result.length);
	return result;
}

/**
 * Convert Svelte-style component to plain JavaScript
 * Extracts `export let` props from script and converts to plain variables
 */
function convertSvelteToPlainJS(
	html: string,
	js: string,
	props: ComponentTemplateProps,
	data?: any[]
): { convertedHtml: string; convertedJs: string } {
	let convertedHtml = html;
	let convertedJs = js;

	// Check if HTML contains a Svelte-style <script> tag
	const scriptMatch = convertedHtml.match(/<script[^>]*>([\s\S]*?)<\/script>/i);
	
	if (scriptMatch) {
		const scriptContent = scriptMatch[1];
		
		// Extract export let declarations and convert to const with values from props
		const exportLetPattern = /export\s+let\s+(\w+)\s*=\s*([^;]+);/g;
		let convertedScript = scriptContent;
		let match;
		
		const propDeclarations: string[] = [];
		
		while ((match = exportLetPattern.exec(scriptContent)) !== null) {
			const [fullMatch, propName, defaultValue] = match;
			
			// Get value from props or use default
			let value = props[propName];
			if (value === undefined) {
				if (propName === 'data' && data) {
					value = data;
				} else {
					// Try to evaluate default value
					try {
						const cleanDefault = defaultValue.trim();
						if (cleanDefault.startsWith('[') || cleanDefault.startsWith('{')) {
							value = JSON.parse(cleanDefault);
						} else if (cleanDefault.startsWith("'") || cleanDefault.startsWith('"')) {
							value = cleanDefault.slice(1, -1);
						} else {
							value = cleanDefault;
						}
					} catch {
						value = defaultValue.trim();
					}
				}
			}
			
			// Create const declaration with actual value
			const valueStr = typeof value === 'string' ? `'${value}'` : JSON.stringify(value);
			propDeclarations.push(`const ${propName} = ${valueStr};`);
			
			// Remove export let line
			convertedScript = convertedScript.replace(fullMatch, '');
		}
		
		// Also handle export let without default values
		const exportLetNoDefaultPattern = /export\s+let\s+(\w+);/g;
		while ((match = exportLetNoDefaultPattern.exec(scriptContent)) !== null) {
			const [fullMatch, propName] = match;
			
			let value = props[propName];
			if (value === undefined) {
				if (propName === 'data' && data) {
					value = data;
				} else {
					value = '';
				}
			}
			
			const valueStr = typeof value === 'string' ? `'${value}'` : JSON.stringify(value);
			propDeclarations.push(`const ${propName} = ${valueStr};`);
			
			convertedScript = convertedScript.replace(fullMatch, '');
		}
		
		// Prepend prop declarations to script
		convertedScript = propDeclarations.join('\n') + '\n' + convertedScript;
		
		// Remove the script tag from HTML and add converted script to js
		convertedHtml = convertedHtml.replace(scriptMatch[0], '');
		convertedJs = convertedScript + '\n' + convertedJs;
		
		// Replace Svelte template expressions {variable} with actual values or <span> elements
		// We need to handle these in the HTML after extracting the script
		const templateExprPattern = /\{(\w+)\}/g;
		convertedHtml = convertedHtml.replace(templateExprPattern, (match, varName) => {
			// If we have a value for this prop, use it directly
			if (props[varName] !== undefined) {
				return String(props[varName]);
			}
			// Otherwise, create a span that will be populated by JavaScript
			return `<span id="svelte-var-${varName}"></span>`;
		});
		
		// Add script to populate dynamic spans
		const dynamicVars = Array.from(new Set(
			Array.from(scriptContent.matchAll(/export\s+let\s+(\w+)/g))
				.map(m => m[1])
		));
		
		if (dynamicVars.length > 0) {
			const populateScript = dynamicVars
				.filter(varName => props[varName] === undefined)
				.map(varName => `
					const ${varName}Elem = document.getElementById('svelte-var-${varName}');
					if (${varName}Elem && typeof ${varName} !== 'undefined') {
						${varName}Elem.textContent = ${varName};
					}
				`)
				.join('\n');
			
			convertedJs += '\n' + populateScript;
		}
	}

	return { convertedHtml, convertedJs };
}

/**
 * Replace template variables in code
 * Supports: {{data}}, {x_dimension}, {y_metric}, {metric_value}, etc.
 */
function replaceTemplateVariables(
	code: string,
	props: ComponentTemplateProps,
	data?: any[]
): string {
	let result = code;

	// Replace {{data}} with actual data array
	if (data) {
		result = result.replace(/\{\{data\}\}/g, JSON.stringify(data));
	}

	// Replace simple variables like {x_dimension}, {y_metric}, etc.
	Object.keys(props).forEach(key => {
		const value = props[key];
		const placeholder = `{${key}}`;
		
		if (typeof value === 'string' || typeof value === 'number') {
			result = result.replace(new RegExp(placeholder.replace(/[{}]/g, '\\$&'), 'g'), String(value));
		}
	});

	return result;
}

/**
 * Build complete iframe HTML document
 */
function buildIframeDocument(
	html: string,
	css: string,
	js: string,
	frameworks: string,
	props: ComponentTemplateProps,
	data?: any[]
): string {
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

	// Build complete document
	return `<!DOCTYPE html>
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
  ${html}
  <script>
    // Inject data and props into window for component access
    window.componentData = ${JSON.stringify(data || [])};
    window.componentProps = ${JSON.stringify(props)};
    
    // Custom component JavaScript
    ${js}
  <\/script>
</body>
</html>`;
}

/**
 * Parse component props to extract metadata
 * Used in the editor to understand component structure
 */
export function parseComponentProps(html: string): {
	hasDataBinding: boolean;
	requiredProps: string[];
	optionalProps: string[];
} {
	const dataBindingPattern = /\{\{data\}\}/;
	const propPattern = /\{(\w+)\}/g;
	
	const hasDataBinding = dataBindingPattern.test(html);
	const props = new Set<string>();
	
	let match;
	while ((match = propPattern.exec(html)) !== null) {
		props.add(match[1]);
	}
	
	// Determine required vs optional based on common patterns
	const requiredProps: string[] = [];
	const optionalProps: string[] = [];
	
	props.forEach(prop => {
		// Common required props
		if (['x_dimension', 'y_metric', 'metric_value'].includes(prop)) {
			requiredProps.push(prop);
		} else {
			optionalProps.push(prop);
		}
	});
	
	return { hasDataBinding, requiredProps, optionalProps };
}

/**
 * Generate starter HTML template based on type
 */
export function generateStarterTemplate(type: 'basic' | 'alpine' | 'data-driven' = 'basic'): {
	html: string;
	css: string;
	js: string;
	frameworks: string;
} {
	if (type === 'alpine') {
		return {
			html: `<div x-data="{ open: false }" class="relative">
  <button @click="open = !open" class="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors">
    Toggle Content
  </button>
  <div x-show="open" class="mt-4 p-4 bg-slate-100 dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-700">
    <p class="text-slate-700 dark:text-slate-300">Content revealed!</p>
  </div>
</div>`,
			css: '',
			js: '',
			frameworks: 'tailwind,alpinejs,heroicons'
		};
	}
	
	if (type === 'data-driven') {
		return {
			html: `<div class="bg-gradient-to-br from-indigo-500 to-purple-600 rounded-lg p-6 text-white shadow-lg">
  <div class="text-sm opacity-90 mb-1">Metric Name</div>
  <div class="text-4xl font-bold">{metric_value}</div>
  <div class="text-xs opacity-75 mt-2">Updated just now</div>
</div>`,
			css: '',
			js: '',
			frameworks: 'tailwind,alpinejs,heroicons'
		};
	}
	
	// Basic template
	return {
		html: `<div class="bg-white dark:bg-slate-800 rounded-lg shadow-lg p-6 border border-slate-200 dark:border-slate-700">
  <h3 class="text-lg font-semibold text-slate-900 dark:text-slate-100 mb-2">My Component</h3>
  <p class="text-slate-600 dark:text-slate-400">Component content goes here. Edit the HTML to customize.</p>
</div>`,
		css: '',
		js: '',
		frameworks: 'tailwind,alpinejs,heroicons'
	};
}

