/**
 * Bar chart library package — self-contained extension entry:
 * definition + renderer + demo data + docs + source (Code tab).
 */
import type { LibraryEntry } from '$lib/library/types';
import { barChartDefinition } from './def';
import { barChartDemo } from './demo';
import BarChartRenderer from '$lib/components/charts/renderers/BarChartRenderer.svelte';
import docs from './docs.md?raw';
import defSource from './def.ts?raw';
import rendererSource from '$lib/components/charts/renderers/BarChartRenderer.svelte?raw';

const barChart: LibraryEntry = {
	def: barChartDefinition,
	renderer: BarChartRenderer,
	description: 'Horizontal or vertical bars — one dimension vs. a measure, with Top-N bucketing and reference rules.',
	docs,
	demo: barChartDemo,
	code: {
		'src/lib/library/components/bar-chart/def.ts': defSource,
		'src/lib/components/charts/renderers/BarChartRenderer.svelte': rendererSource
	}
};

export default barChart;
