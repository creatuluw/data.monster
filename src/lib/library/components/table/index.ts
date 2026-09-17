/**
 * Table library package — built-in block kind (blockKind: 'table'; PageGrid
 * renders it via TableRenderer, no chart-registry registration).
 */
import type { LibraryEntry } from '$lib/library/types';
import { tableDefinition } from './def';
import { tableDemo } from './demo';
import TableRenderer from '$lib/components/charts/renderers/TableRenderer.svelte';
import docs from './docs.md?raw';
import defSource from './def.ts?raw';
import rendererSource from '$lib/components/charts/renderers/TableRenderer.svelte?raw';

const table: LibraryEntry = {
	def: tableDefinition,
	blockKind: 'table',
	renderer: TableRenderer,
	description: 'Compact sticky-header table of raw rows — detail data block with a row limit.',
	docs,
	demo: tableDemo,
	code: {
		'src/lib/library/components/table/def.ts': defSource,
		'src/lib/components/charts/renderers/TableRenderer.svelte': rendererSource
	}
};

export default table;
