/**
 * Registers library component packages with the central chart registry.
 * Import this module from any route that renders registry charts.
 * The definitions themselves live in self-contained packages under
 * src/lib/library/components/<type>/ — adding a chart to the library makes it
 * available in /pages (picker + config panel) and /library by construction.
 */
import { registerLibraryComponent } from '$lib/library/registry';
import barChart from '$lib/library/components/bar-chart';
import heatmap from '$lib/library/components/heatmap';
import table from '$lib/library/components/table';
import text from '$lib/library/components/text';

export function setupChartRegistry(): void {
	registerLibraryComponent(barChart);
	registerLibraryComponent(heatmap);
	registerLibraryComponent(table);
	registerLibraryComponent(text);
}

/** Renderer components keyed by chart type (kept out of the pure registry).
 *  Table/text are built-in block kinds rendered by PageGrid directly. */
export const chartRenderers = {
	[barChart.def.type]: barChart.renderer,
	[heatmap.def.type]: heatmap.renderer
};
