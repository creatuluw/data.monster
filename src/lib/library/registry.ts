/**
 * Library registry — one registration point for extension-style components.
 * registerLibraryComponent feeds BOTH this map (for /library views) and the
 * central chart registry (for /pages runtime), so a component added here is
 * usable app-wide by construction.
 */
import { registerChartType } from '$lib/charts/registry';
import type { LibraryEntry } from './types';

const entries = new Map<string, LibraryEntry>();

export function registerLibraryComponent(entry: LibraryEntry): void {
	entries.set(entry.def.type, entry);
	// only chart-kind entries feed the chart registry — table/text are built-in
	// block kinds with their own spec shape, handled directly by PageGrid
	if ((entry.blockKind ?? 'chart') === 'chart') registerChartType(entry.def);
}

export function getLibraryComponent(type: string): LibraryEntry | undefined {
	return entries.get(type);
}

export function getLibraryComponents(): LibraryEntry[] {
	return [...entries.values()];
}
