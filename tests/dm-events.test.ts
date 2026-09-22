import { describe, expect, it, vi } from 'vitest';
import {
	dispatchChanged,
	dispatchError,
	onDmChanged,
	onDmError,
	routeDmEvent,
	type DmChanged,
	type DmError
} from '../src/lib/dm-events';

describe('routeDmEvent', () => {
	it('routes a valid dm:changed payload', () => {
		const changed = vi.fn();
		const error = vi.fn();
		routeDmEvent({ changed, error }, 'dm:changed', { kind: 'page', name: 'revenue', removed: false });
		expect(changed).toHaveBeenCalledWith({ kind: 'page', name: 'revenue', removed: false });
		expect(error).not.toHaveBeenCalled();
	});

	it('normalizes missing name/removed on dm:changed', () => {
		const changed = vi.fn();
		routeDmEvent({ changed, error: vi.fn() }, 'dm:changed', { kind: 'relationships' });
		expect(changed).toHaveBeenCalledWith({ kind: 'relationships', name: null, removed: false });
	});

	it('routes dm:error payloads', () => {
		const changed = vi.fn();
		const error = vi.fn();
		routeDmEvent({ changed, error }, 'dm:error', { path: 'pages/broken.json', reason: 'invalid JSON' });
		expect(error).toHaveBeenCalledWith({ path: 'pages/broken.json', reason: 'invalid JSON' });
		expect(changed).not.toHaveBeenCalled();
	});

	it('ignores malformed payloads and unknown events', () => {
		const changed = vi.fn();
		const error = vi.fn();
		const h = { changed, error };
		routeDmEvent(h, 'dm:changed', null);
		routeDmEvent(h, 'dm:changed', { nope: true });
		routeDmEvent(h, 'dm:error', null);
		routeDmEvent(h, 'dm:error', { nope: true });
		routeDmEvent(h, 'some:other-event', { kind: 'page' });
		expect(changed).not.toHaveBeenCalled();
		expect(error).not.toHaveBeenCalled();
	});
});

describe('registry dispatch', () => {
	it('only notifies the matching kind and cleans up on off()', async () => {
		const page = vi.fn();
		const item = vi.fn();
		const offPage = onDmChanged('page', page);
		onDmChanged('measure', item);

		dispatchChanged({ kind: 'page', name: 'revenue', removed: false } satisfies DmChanged);
		await vi.waitFor(() => expect(page).toHaveBeenCalledWith('revenue', false));
		expect(item).not.toHaveBeenCalled();

		offPage();
		dispatchChanged({ kind: 'page', name: 'x', removed: true } satisfies DmChanged);
		await vi.waitFor(() => expect(page).toHaveBeenCalledTimes(1)); // not called again
	});

	it('dispatches errors to all subscribers and supports off()', async () => {
		const a = vi.fn();
		const b = vi.fn();
		const offA = onDmError(a);
		onDmError(b);
		dispatchError({ path: 'pages/bad.json', reason: 'invalid JSON' } satisfies DmError);
		await vi.waitFor(() => {
			expect(a).toHaveBeenCalledWith('pages/bad.json', 'invalid JSON');
			expect(b).toHaveBeenCalledWith('pages/bad.json', 'invalid JSON');
		});
		offA();
		dispatchError({ path: 'x', reason: 'y' });
		await vi.waitFor(() => expect(a).toHaveBeenCalledTimes(1));
	});
});
