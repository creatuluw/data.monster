import { describe, it, expect } from 'vitest';
import { availableItems, buildJoins, linkedTables } from '../src/lib/charts/relationships';
import type { Relationship } from '../src/lib/charts/relationships';
import type { MasterItem } from '../src/lib/charts/items';

// bookings —(client_id)— clients —(region_id)— regions
const rels: Relationship[] = [
	{ id: 'r1', fromTable: 'bookings', fromColumn: 'client_id', toTable: 'clients', toColumn: 'id' },
	{ id: 'r2', fromTable: 'clients', fromColumn: 'region_id', toTable: 'regions', toColumn: 'id' }
];

const items: MasterItem[] = [
	{ id: 'a', kind: 'measure', table: 'bookings', label: 'A', expr: 'sum(a)' },
	{ id: 'b', kind: 'measure', table: 'clients', label: 'B', expr: 'sum(b)' },
	{ id: 'c', kind: 'measure', table: 'regions', label: 'C', expr: 'sum(c)' },
	{ id: 'z', kind: 'measure', table: 'island', label: 'Z', expr: 'sum(z)' }
];

describe('availableItems — Q7-B gate', () => {
	it('offers items on the source table and reachable tables', () => {
		const ids = availableItems('bookings', items, rels).map((i) => i.id).sort();
		expect(ids).toEqual(['a', 'b', 'c']);
	});

	it('excludes items from unreachable tables', () => {
		expect(availableItems('bookings', items, rels).some((i) => i.id === 'z')).toBe(false);
	});
});

describe('buildJoins — auto-JOIN generation', () => {
	it('generates a direct join', () => {
		expect(buildJoins('bookings', ['clients'], rels)).toEqual([
			'JOIN "clients" ON "bookings"."client_id" = "clients"."id"'
		]);
	});

	it('generates a two-hop join in path order', () => {
		expect(buildJoins('bookings', ['regions'], rels)).toEqual([
			'JOIN "clients" ON "bookings"."client_id" = "clients"."id"',
			'JOIN "regions" ON "clients"."region_id" = "regions"."id"'
		]);
	});

	it('returns no join for the source table alone', () => {
		expect(buildJoins('bookings', ['bookings'], rels)).toEqual([]);
	});

	it('is safe against cycles (shortest path wins, no loop)', () => {
		const cyclic: Relationship[] = [
			...rels,
			{ id: 'r3', fromTable: 'regions', fromColumn: 'id', toTable: 'bookings', toColumn: 'region_id' }
		];
		expect(buildJoins('bookings', ['regions'], cyclic)).toEqual([
			'JOIN "regions" ON "bookings"."region_id" = "regions"."id"'
		]);
	});

	it('walks relationships in both directions', () => {
		// source is on the "to" side of the edge — still joinable
		expect(buildJoins('regions', ['clients'], rels)).toEqual([
			'JOIN "clients" ON "regions"."id" = "clients"."region_id"'
		]);
	});

	it('throws for an unreachable table', () => {
		expect(() => buildJoins('bookings', ['island'], rels)).toThrow(/no relationship path/i);
	});
});

describe('linkedTables', () => {
	it('lists tables reachable via relationships, excluding the source', () => {
		const rels = [
			{ id: 'r1', fromTable: 'bookings', fromColumn: 'client_id', toTable: 'clients', toColumn: 'id' },
			{ id: 'r2', fromTable: 'clients', fromColumn: 'region_id', toTable: 'regions', toColumn: 'id' }
		];
		expect(linkedTables('bookings', rels).sort()).toEqual(['clients', 'regions']);
		expect(linkedTables('regions', rels).sort()).toEqual(['bookings', 'clients']);
	});

	it('returns empty when no relationships touch the table', () => {
		expect(linkedTables('solo', [])).toEqual([]);
	});
});
