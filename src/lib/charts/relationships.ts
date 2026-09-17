/**
 * Relationship graph + auto-JOIN generation (FR-15 / Q7-B).
 * Relationships connect two tables with a column pair; traversal is
 * undirected (a chart may sit on either side), shortest path via BFS.
 */
import type { MasterItem } from './items';

export type Relationship = {
	id: string;
	fromTable: string;
	fromColumn: string;
	toTable: string;
	toColumn: string;
};

type Edge = { via: Relationship; other: (t: string) => { table: string; leftTable: string; leftColumn: string; rightTable: string; rightColumn: string } };

function adjacency(rels: Relationship[]): Map<string, { rel: Relationship; other: string }[]> {
	const map = new Map<string, { rel: Relationship; other: string }[]>();
	const add = (a: string, entry: { rel: Relationship; other: string }) =>
		map.set(a, [...(map.get(a) ?? []), entry]);
	for (const rel of rels) {
		if (rel.fromTable === rel.toTable) continue; // self-joins are explicit, never walked
		add(rel.fromTable, { rel, other: rel.toTable });
		add(rel.toTable, { rel, other: rel.fromTable });
	}
	return map;
}

/** Tables reachable from sourceTable via relationships (excludes the source itself). */
export function linkedTables(sourceTable: string, rels: Relationship[]): string[] {
	const adj = adjacency(rels);
	const seen = new Set<string>([sourceTable]);
	const queue = [sourceTable];
	while (queue.length) {
		const table = queue.shift()!;
		for (const { other } of adj.get(table) ?? []) {
			if (!seen.has(other)) {
				seen.add(other);
				queue.push(other);
			}
		}
	}
	seen.delete(sourceTable);
	return [...seen];
}

/** Items usable from a chart on sourceTable: same table or any table connected via relationships. */
export function availableItems(sourceTable: string, items: MasterItem[], rels: Relationship[]): MasterItem[] {
	const adj = adjacency(rels);
	const reachable = new Set([sourceTable]);
	const queue = [sourceTable];
	while (queue.length) {
		const table = queue.shift()!;
		for (const { other } of adj.get(table) ?? []) {
			if (!reachable.has(other)) {
				reachable.add(other);
				queue.push(other);
			}
		}
	}
	return items.filter((item) => reachable.has(item.table));
}

/** JOIN clauses for every involved table, in BFS path order from the source. */
export function buildJoins(sourceTable: string, involvedTables: string[], rels: Relationship[]): string[] {
	const adj = adjacency(rels);
	// BFS from source, remembering the edge that discovered each table
	const discovered = new Map<string, { from: string; rel: Relationship }>([[sourceTable, { from: '', rel: null as unknown as Relationship }]]);
	const queue = [sourceTable];
	while (queue.length) {
		const table = queue.shift()!;
		for (const { rel, other } of adj.get(table) ?? []) {
			if (!discovered.has(other)) {
				discovered.set(other, { from: table, rel });
				queue.push(other);
			}
		}
	}

	const clauses: string[] = [];
	for (const target of involvedTables) {
		if (target === sourceTable) continue;
		if (!discovered.has(target)) throw new Error(`no relationship path from "${sourceTable}" to "${target}"`);
		// walk back from target to source, collecting edges in reverse
		const path: { from: string; rel: Relationship }[] = [];
		let cursor: string | undefined = target;
		while (cursor && cursor !== sourceTable) {
			const step = discovered.get(cursor);
			if (!step) break;
			path.unshift(step);
			cursor = step.from;
		}
		for (const { from, rel } of path) {
			const forward = rel.fromTable === from; // edge direction relative to walk
			const clause = forward
				? `JOIN "${rel.toTable}" ON "${rel.fromTable}"."${rel.fromColumn}" = "${rel.toTable}"."${rel.toColumn}"`
				: `JOIN "${rel.fromTable}" ON "${rel.toTable}"."${rel.toColumn}" = "${rel.fromTable}"."${rel.fromColumn}"`;
			if (!clauses.includes(clause)) clauses.push(clause);
		}
	}
	return clauses;
}
