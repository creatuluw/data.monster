/**
 * Heatmap library package — self-contained extension entry:
 * definition + renderer + demo data + docs + source (Code tab).
 */
import type { LibraryEntry } from '$lib/library/types';
import { heatmapDefinition } from './def';
import { heatmapDemo } from './demo';
import HeatmapRenderer from '$lib/components/charts/renderers/HeatmapRenderer.svelte';
import docs from './docs.md?raw';
import defSource from './def.ts?raw';
import rendererSource from '$lib/components/charts/renderers/HeatmapRenderer.svelte?raw';

const heatmap: LibraryEntry = {
	def: heatmapDefinition,
	renderer: HeatmapRenderer,
	description: 'Color grid of one measure across two dimensions (row × column pivot), threshold-scaled.',
	docs,
	demo: heatmapDemo,
	code: {
		'src/lib/library/components/heatmap/def.ts': defSource,
		'src/lib/components/charts/renderers/HeatmapRenderer.svelte': rendererSource
	}
};

export default heatmap;
