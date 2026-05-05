/**
 * Component Template Types
 * 
 * Type definitions for HTML/CSS/JS component templates that can be
 * created in the component editor and stored in DuckDB.
 */

export interface ComponentTemplate {
	slug: string;
	component_name: string;
	component_type: string;
	html_code: string;
	css_code: string | null;
	js_code: string | null;
	config_schema: string | null;
	description: string | null;
	tags: string | null;
	metrics: string | null;
	dimensions: string | null;
	sample_data: string | null;
	frameworks: string | null; // Comma-separated: "alpinejs,tailwind,heroicons"
	min_metrics: number;
	max_metrics: number;
	min_dimensions: number;
	max_dimensions: number;
	created_at: string;
	updated_at: string;
}

export interface ComponentTemplateProps {
	[key: string]: any;
	data?: any[];
	x_dimension?: string;
	y_dimension?: string;
	y_metric?: string;
	x_metric?: string;
	metric_value?: any;
	dimension_value?: any;
}

export interface Framework {
	id: string;
	name: string;
	cdn_url: string;
	description: string;
}

export const AVAILABLE_FRAMEWORKS: Framework[] = [
	{
		id: 'tailwind',
		name: 'Tailwind CSS',
		cdn_url: 'https://cdn.tailwindcss.com',
		description: 'Utility-first CSS framework'
	},
	{
		id: 'alpinejs',
		name: 'Alpine.js',
		cdn_url: 'https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js',
		description: 'Lightweight JavaScript framework'
	},
	{
		id: 'heroicons',
		name: 'Heroicons',
		cdn_url: 'https://cdn.jsdelivr.net/npm/heroicons@2.0.18/outline/index.js',
		description: 'SVG icon library'
	}
];

