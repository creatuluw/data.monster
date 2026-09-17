import { describe, it, expect } from 'vitest';
import { blocksAffectedBySelection, crossFilterFor } from '../src/lib/charts/page-runtime.svelte';
import type { Block, PageDoc } from '../src/lib/charts/spec-types';

const schemas = {
	bookings: ['month', 'hours', 'region'],
	clients: ['id', 'client_name']
};

const doc: PageDoc = {
	slug: 'x',
	title: 'X',
	rows: [
		{
			blocks: [
				{
					type: 'chart',
					span: 6,
					chart: { type: 'bar', source: { table: 'bookings' }, dimensions: [{ col: 'region' }], measures: [{ expr: 'sum(hours)', label: 'Hours' }] }
				},
				{
					type: 'chart',
					span: 6,
					chart: { type: 'heatmap', source: { table: 'bookings' }, dimensions: [{ col: 'month' }, { col: 'region' }], measures: [{ expr: 'sum(hours)' }] }
				},
				{ type: 'table', span: 12, table: 'bookings' },
				{ type: 'table', span: 12, table: 'clients' },
				{ type: 'text', span: 12, text: 'note' }
			]
		}
	]
};

const blocks = (doc.rows![0].blocks ?? []).map((b, i) => ({ ...b, __id: `r0-b${i}` }) as Block & { __id: string });

describe('blocksAffectedBySelection', () => {
	it('affects blocks carrying the column, excluding the source', () => {
		const affected = blocksAffectedBySelection(blocks, { blockId: 'r0-b0', dimension: 'region', value: 'AMS' }, schemas);
		expect(affected.map((b) => (b as Block & { __id: string }).__id)).toEqual(['r0-b1', 'r0-b2']);
	});

	it('skips tables without the column', () => {
		const affected = blocksAffectedBySelection(blocks, { blockId: 'r0-b0', dimension: 'client_name', value: 'ACME' }, schemas);
		expect(affected.map((b) => (b as Block & { __id: string }).__id)).toEqual(['r0-b3']);
	});

	it('returns nothing without a selection', () => {
		expect(blocksAffectedBySelection(blocks, null, schemas)).toEqual([]);
	});
});

describe('crossFilterFor', () => {
	it('injects the WHERE pair when the table has the column', () => {
		expect(crossFilterFor(blocks[2], { blockId: 'r0-b0', dimension: 'region', value: 'AMS' }, schemas)).toEqual({ col: 'region', value: 'AMS' });
	});

	it('returns null when the table lacks the column', () => {
		expect(crossFilterFor(blocks[3], { blockId: 'r0-b0', dimension: 'region', value: 'AMS' }, schemas)).toBeNull();
	});
});
