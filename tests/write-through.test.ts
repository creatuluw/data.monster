import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { createWriteThrough } from '../src/lib/write-through';

beforeEach(() => vi.useFakeTimers());
afterEach(() => vi.useRealTimers());

function setup() {
    const writes: string[] = [];
    const wt = createWriteThrough({
        read: () => 'current-text',
        write: async (spec) => {
            writes.push(spec);
        }
    });
    return { wt, writes };
}

describe('write-through', () => {
    it('coalesces N local edits into ONE write after the debounce window', async () => {
        const { wt, writes } = setup();
        wt.markLocal('v1');
        wt.markLocal('v2');
        await vi.advanceTimersByTimeAsync(200);
        wt.markLocal('v3');
        expect(writes).toHaveLength(0); // nothing written before the window settles

        await vi.advanceTimersByTimeAsync(500); // past 400ms from the LAST mark
        expect(writes).toEqual(['current-text']); // exactly one write
        expect(wt.pending).toBe(false);
    });

    it('baseline load does not trigger writes', () => {
        const { wt, writes } = setup();
        wt.ackLoad('loaded-content');
        wt.markLocal('loaded-content'); // identical → not dirty
        expect(writes).toHaveLength(0);
        expect(wt.pending).toBe(false);
    });

    it('conflict matrix: clean + external change → reload (no banner)', () => {
        const { wt } = setup();
        wt.ackLoad('loaded');
        expect(wt.decideExternal()).toBe('reload');
        expect(wt.conflict).toBe(false);
    });

    it('conflict matrix: dirty + external change → banner', () => {
        const { wt } = setup();
        wt.ackLoad('loaded');
        wt.markLocal('edited, not yet written');
        expect(wt.decideExternal()).toBe('conflict');
        expect(wt.conflict).toBe(true);
    });

    it('keep-mine = flush: writes the local text and clears the conflict', async () => {
        const { wt, writes } = setup();
        wt.ackLoad('loaded');
        wt.markLocal('mine');
        expect(wt.decideExternal()).toBe('conflict');

        await wt.flush();
        expect(writes).toEqual(['current-text']);
        expect(wt.conflict).toBe(false);
        expect(wt.pending).toBe(false);
    });

    it('reload ack clears pending state and rebaselines', async () => {
        const { wt, writes } = setup();
        wt.ackLoad('loaded');
        wt.markLocal('mine');
        wt.ackLoad('theirs-from-disk');
        expect(wt.pending).toBe(false);
        expect(wt.conflict).toBe(false);
        await vi.advanceTimersByTimeAsync(1000);
        expect(writes).toHaveLength(0); // the discarded timer never writes
    });

    it('a failed write keeps pending so the change is not lost', async () => {
        const writes: string[] = [];
        const wt = createWriteThrough({
            read: () => 'text',
            write: async () => {
                throw new Error('disk gone');
            }
        });
        wt.markLocal('text');
        await vi.advanceTimersByTimeAsync(500);
        expect(writes).toHaveLength(0);
        expect(wt.pending).toBe(true); // retryable — next markLocal/flush tries again
    });
});
