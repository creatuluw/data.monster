/**
 * Template Variable System & Compiler
 * 
 * Converts raw SveltePlot code with template variables into reusable components.
 * 
 * Template Variables:
 * - {data} - Data array
 * - {x_dimension} - X-axis field
 * - {y_metric} - Y-axis field
 * - {series} - Series/grouping field
 * - {color} - Fill color
 * - {title} - Chart title
 * - {x_label} - X-axis label
 * - {y_label} - Y-axis label
 * - {sort} - Sort option
 * - {format} - Value format
 */

export interface TemplateVariable {
	name: string;
	type: 'string' | 'number' | 'boolean' | 'data' | 'function';
	default?: any;
	required?: boolean;
}

export interface ChartTemplateDefinition {
	id: string;
	name: string;
	description: string;
	category: 'basic' | 'comparison' | 'trend' | 'distribution' | 'relationship' | 'composition';
	tags: string[];
	rawCode: string; // Raw SveltePlot with template variables
	variables: TemplateVariable[];
	preview?: string; // Preview image or data
}

/**
 * Extract template variables from raw SveltePlot code
 */
export function extractTemplateVariables(code: string): string[] {
	const regex = /\{([a-zA-Z_][a-zA-Z0-9_]*)\}/g;
	const matches = [...code.matchAll(regex)];
	const variables = new Set<string>();
	
	matches.forEach(match => {
		variables.add(match[1]);
	});
	
	return Array.from(variables);
}

/**
 * Compile template with provided values
 */
export function compileTemplate(
	rawCode: string,
	values: Record<string, any>
): string {
	let compiled = rawCode;
	
	// Replace each variable with its value
	Object.entries(values).forEach(([key, value]) => {
		const regex = new RegExp(`\\{${key}\\}`, 'g');
		
		// Handle different value types
		let replacement: string;
		if (typeof value === 'string') {
			// If it's a field name, use it directly (no quotes)
			// If it's a literal string (like color), use quotes
			replacement = value.startsWith('#') || value.startsWith('var(') ? `"${value}"` : value;
		} else if (typeof value === 'boolean' || typeof value === 'number') {
			replacement = String(value);
		} else if (value === null || value === undefined) {
			replacement = 'undefined';
		} else {
			replacement = JSON.stringify(value);
		}
		
		compiled = compiled.replace(regex, replacement);
	});
	
	return compiled;
}

/**
 * Generate component props interface from template variables
 */
export function generatePropsInterface(variables: TemplateVariable[]): string {
	const props = variables.map(v => {
		const optional = v.required === false ? '?' : '';
		const type = v.type === 'data' ? 'any[]' : v.type;
		return `  ${v.name}${optional}: ${type};`;
	});
	
	return `interface ChartProps {\n${props.join('\n')}\n}`;
}

/**
 * Validate template code
 */
export function validateTemplate(code: string): { valid: boolean; errors: string[] } {
	const errors: string[] = [];
	
	// Check for required imports
	if (!code.includes('import')) {
		errors.push('Missing import statement');
	}
	
	// Check for Plot component
	if (!code.includes('<Plot')) {
		errors.push('Missing <Plot> component');
	}
	
	// Check for at least one mark (BarY, LineY, etc.)
	const marks = ['BarY', 'BarX', 'LineY', 'AreaY', 'Dot', 'Text', 'RuleY', 'RuleX'];
	const hasMarks = marks.some(mark => code.includes(`<${mark}`));
	if (!hasMarks) {
		errors.push('Missing chart mark (BarY, LineY, etc.)');
	}
	
	// Check for unmatched template variables
	const variables = extractTemplateVariables(code);
	const dataVariable = variables.find(v => v === 'data');
	if (!dataVariable) {
		errors.push('Missing {data} variable');
	}
	
	return {
		valid: errors.length === 0,
		errors
	};
}

/**
 * Create a reusable component from template
 */
export function createComponent(template: ChartTemplateDefinition): string {
	return `<script lang="ts">
  ${template.rawCode.match(/<script[^>]*>([\s\S]*?)<\/script>/)?.[1] || ''}
  
  interface Props {
${template.variables.map(v => `    ${v.name}${v.required === false ? '?' : ''}: ${v.type === 'data' ? 'any[]' : v.type};`).join('\n')}
  }
  
  let { ${template.variables.map(v => v.name).join(', ')} }: Props = $props();
</script>

${template.rawCode.replace(/<script[^>]*>[\s\S]*?<\/script>/, '').trim()}
`;
}

/**
 * Parse raw SveltePlot code to extract component structure
 */
export function parseRawCode(code: string): {
	imports: string;
	script: string;
	template: string;
} {
	const importMatch = code.match(/import\s+.*?;/gs);
	const scriptMatch = code.match(/<script[^>]*>([\s\S]*?)<\/script>/);
	const templateContent = code.replace(/<script[^>]*>[\s\S]*?<\/script>/, '').trim();
	
	return {
		imports: importMatch?.join('\n') || '',
		script: scriptMatch?.[1]?.trim() || '',
		template: templateContent
	};
}

/**
 * Smart type inference for template variables
 */
export function inferVariableType(variableName: string): TemplateVariable['type'] {
	if (variableName === 'data') return 'data';
	if (variableName.includes('color') || variableName.includes('fill')) return 'string';
	if (variableName.includes('width') || variableName.includes('height') || variableName.includes('size')) return 'number';
	if (variableName.includes('show') || variableName.includes('enable') || variableName.includes('sort')) return 'boolean';
	if (variableName.includes('label') || variableName.includes('title') || variableName.includes('format')) return 'string';
	if (variableName.includes('dimension') || variableName.includes('metric') || variableName.includes('field')) return 'string';
	
	return 'string'; // Default
}

/**
 * Generate smart defaults for variables
 */
export function generateSmartDefaults(variables: string[]): Record<string, any> {
	const defaults: Record<string, any> = {};
	
	variables.forEach(varName => {
		switch (varName) {
			case 'data':
				defaults[varName] = [];
				break;
			case 'color':
			case 'fill':
			case 'fillColor':
				defaults[varName] = '#CA3500';
				break;
			case 'sort':
				defaults[varName] = true;
				break;
			case 'x_dimension':
			case 'y_metric':
			case 'series':
				defaults[varName] = null;
				break;
			default:
				defaults[varName] = null;
		}
	});
	
	return defaults;
}

