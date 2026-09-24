/**
 * dm/ live-reload plumbing (workspace-file-first FR-8).
 *
 * Backend watcher (src-tauri dm_watch.rs) emits:
 *   dm:changed {kind, name, removed} — a content file changed/was removed
 *   dm:error   {path, reason}        — a content file failed parse-level validation
 *
 * Views subscribe per kind (`onDmChanged`) and re-fetch; the routing core below is
 * pure and unit-tested in tests/dm-events.test.ts.
 */
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

export type DmChanged = { kind: string; name: string | null; removed: boolean };
export type DmError = { path: string; reason: string };

export type ChangedHandler = (name: string | null, removed: boolean) => void | Promise<void>;
export type ErrorHandler = (path: string, reason: string) => void;

const changedHandlers = new Map<string, Set<ChangedHandler>>();
const errorHandlers = new Set<ErrorHandler>();

/** Subscribe to changes of one content kind. Returns an off() (use as $effect cleanup). */
export function onDmChanged(kind: string, cb: ChangedHandler): () => void {
	let set = changedHandlers.get(kind);
	if (!set) {
		set = new Set();
		changedHandlers.set(kind, set);
	}
	set.add(cb);
	return () => set!.delete(cb);
}

/** Subscribe to validation errors for any dm/ file. Returns an off(). */
export function onDmError(cb: ErrorHandler): () => void {
	errorHandlers.add(cb);
	return () => errorHandlers.delete(cb);
}

/** Dispatch a parsed change to the matching kind's handlers. */
export function dispatchChanged(e: DmChanged): void {
	for (const cb of changedHandlers.get(e.kind) ?? []) void cb(e.name, e.removed);
}

/** Dispatch a validation error to all error handlers. */
export function dispatchError(e: DmError): void {
	for (const cb of errorHandlers) cb(e.path, e.reason);
}

/**
 * Pure routing core: event name + raw payload → handlers. Malformed payloads are
 * ignored (the watcher never sends them, but the bus is defensive).
 */
export function routeDmEvent(
	handlers: { changed: (e: DmChanged) => void; error: (e: DmError) => void },
	event: string,
	payload: unknown
): void {
	if (event === 'dm:changed') {
		const p = (payload ?? {}) as Partial<DmChanged>;
		if (typeof p.kind !== 'string') return;
		handlers.changed({
			kind: p.kind,
			name: typeof p.name === 'string' ? p.name : null,
			removed: p.removed === true
		});
		return;
	}
	if (event === 'dm:error') {
		const p = (payload ?? {}) as Partial<DmError>;
		if (typeof p.path !== 'string') return;
		handlers.error({ path: p.path, reason: typeof p.reason === 'string' ? p.reason : '' });
	}
}

let unlisten: UnlistenFn[] | null = null;

/**
 * Wire the Tauri listeners once (call from the root layout). Errors surface through
 * the app's global error banner plus the console.
 */
export async function initDmEvents(onError?: ErrorHandler): Promise<void> {
	if (unlisten) return;
	const l1 = await listen<DmChanged>('dm:changed', (ev) => dispatchChanged(ev.payload));
	const l2 = await listen<DmError>('dm:error', (ev) => {
		console.warn(`[dm] ${ev.payload.path}: ${ev.payload.reason}`);
		dispatchError(ev.payload);
		onError?.(ev.payload.path, ev.payload.reason);
	});
	unlisten = [l1, l2];
}
