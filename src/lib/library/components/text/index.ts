/**
 * Text library package — built-in block kind (blockKind: 'text'; PageGrid
 * renders it inline, so the entry carries no renderer component).
 */
import type { LibraryEntry } from '$lib/library/types';
import { textDefinition } from './def';
import { textDemo } from './demo';
import docs from './docs.md?raw';
import defSource from './def.ts?raw';

const text: LibraryEntry = {
	def: textDefinition,
	blockKind: 'text',
	// PageGrid renders text blocks inline; no renderer component to demo
	renderer: {} as never,
	description: 'Plain paragraph card — narrative context between charts.',
	docs,
	demo: textDemo,
	code: { 'src/lib/library/components/text/def.ts': defSource }
};

export default text;
