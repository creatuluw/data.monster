/**
 * Write-through core (workspace-file-first FR-9): the file IS the save.
 *
 * Pure state machine, no Svelte — unit-tested in tests/write-through.test.ts.
 * The pages editor binds it: `read` returns the current text (Design JSON or raw
 * Code-mode text), `write` persists atomically via save_page.
 *
 * Conflicts follow the VS Code model: an external change while we have unwritten
 * local edits → 'conflict' (banner: Reload / Keep mine). Clean → silently reload.
 */

export type WriteThrough = {
	/** A local edit happened. Schedules the debounced write (unless identical to lastWritten). */
	markLocal(text: string): void;
	/** Write pending edits now (Keep mine). Resolves after the write attempt. */
	flush(): Promise<void>;
	/** An external change arrived: 'reload' (clean) or 'conflict' (dirty local edits). */
	decideExternal(): 'reload' | 'conflict';
	/** Rebaseline after a load/reload: disk content is now our baseline. */
	ackLoad(text: string): void;
	/** Retry a failed write (or flush early). */
	readonly pending: boolean;
	readonly conflict: boolean;
	readonly lastWritten: string;
};

export function createWriteThrough(opts: {
	read: () => string;
	write: (spec: string) => Promise<void>;
	debounceMs?: number;
}): WriteThrough {
	const debounceMs = opts.debounceMs ?? 400;

	let timer: ReturnType<typeof setTimeout> | null = null;
	let pending = false;
	let conflict = false;
	let lastWritten = '';

	function schedule() {
		if (timer) clearTimeout(timer);
		timer = setTimeout(() => {
			timer = null;
			void wt.flush();
		}, debounceMs);
	}

	const wt: WriteThrough = {
		markLocal(text: string) {
			if (text === lastWritten && !pending) return; // baseline touch, not an edit
			pending = true;
			schedule();
		},
		async flush() {
			if (timer) {
				clearTimeout(timer);
				timer = null;
			}
			if (!pending) return;
			const spec = opts.read();
			try {
				await opts.write(spec);
				lastWritten = spec;
				pending = false;
				conflict = false;
			} catch (err) {
				console.error('[write-through] write failed — change stays pending', err);
			}
		},
		decideExternal() {
			if (pending) {
				conflict = true;
				return 'conflict';
			}
			return 'reload';
		},
		ackLoad(text: string) {
			if (timer) {
				clearTimeout(timer);
				timer = null;
			}
			pending = false;
			conflict = false;
			lastWritten = text;
		},
		get pending() {
			return pending;
		},
		get conflict() {
			return conflict;
		},
		get lastWritten() {
			return lastWritten;
		}
	};
	return wt;
}
