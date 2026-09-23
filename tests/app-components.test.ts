import { describe, expect, it } from 'vitest';
import { parseComponent, loadAppComponents } from '../src/lib/app-components';

describe('parseComponent', () => {
	it('splits top-level components', () => {
		const c = parseComponent('/src/lib/components/Modal.svelte', 'a\nb\n');
		expect(c.name).toBe('Modal');
		expect(c.dir).toBe('');
		expect(c.path).toBe('src/lib/components/Modal.svelte');
		expect(c.lines).toBe(2);
	});

	it('keeps subfolder as dir', () => {
		const c = parseComponent('/src/lib/components/ds/Badge.svelte', 'x');
		expect(c.name).toBe('Badge');
		expect(c.dir).toBe('ds');
		expect(c.path).toBe('src/lib/components/ds/Badge.svelte');
	});
});

describe('loadAppComponents', () => {
	it('discovers the real component tree, deduped to one active component per name', () => {
		const all = loadAppComponents();
		expect(all.length).toBeGreaterThan(50);
		const names = all.map((c) => c.name);
		// one active variant per name (root twins of ds/ components are hidden)
		expect(new Set(names).size, 'duplicate names').toBe(names.length);
		// known residents of src/lib/components
		for (const known of ['Modal', 'TableDrawer', 'Toast', 'ExprEditor']) {
			expect(names.includes(known), known).toBe(true);
		}
		for (const c of all) {
			expect(c.lines, c.path).toBeGreaterThan(0);
			expect(c.source, c.path).toContain('<');
		}
	});
});
