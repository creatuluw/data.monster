/**
 * Virtual tab system for the single-webview app: the bottom bar lists tabs,
 * right-clicking an internal link offers "Open in new tab". Tabs are just
 * routes — the active tab tracks the current URL, switching tabs navigates.
 */
import { goto } from '$app/navigation';

export type Tab = { id: number; path: string; label: string };

let nextId = 1;
const state = $state<{ list: Tab[]; activeId: number | null }>({ list: [], activeId: null });

export const tabs = {
	get list() {
		return state.list;
	},
	get activeId() {
		return state.activeId;
	}
};

/** call on every navigation: while a tab is active it tracks the current URL;
 *  the base session (before any open-in-new-tab) is NOT a tab. */
export function ensureActive(path: string, label: string) {
	const t = state.list.find((x) => x.id === state.activeId);
	if (!t) return;
	t.path = path;
	t.label = label;
}

export function pathLabel(path: string): string {
	const seg = path.split('/').filter(Boolean).pop();
	return seg ? decodeURIComponent(seg) : 'Home';
}

export function openInNewTab(path: string, label?: string) {
	const t = { id: nextId++, path, label: label ?? pathLabel(path) };
	state.list.push(t);
	state.activeId = t.id;
	goto(path);
}

export function activate(id: number) {
	const t = state.list.find((x) => x.id === id);
	if (!t || t.id === state.activeId) return;
	state.activeId = id;
	goto(t.path);
}

export function closeTab(id: number) {
	const i = state.list.findIndex((x) => x.id === id);
	if (i === -1) return;
	state.list.splice(i, 1);
	if (state.list.length === 0) {
		// last tab closed: fall back to the base session on the current page
		state.activeId = null;
		return;
	}
	if (state.activeId === id) {
		const next = state.list[Math.min(i, state.list.length - 1)];
		state.activeId = next.id;
		goto(next.path);
	}
}
