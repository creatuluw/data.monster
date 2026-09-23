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
	it('returns only showcaseable components, one active variant per name', () => {
		const all = loadAppComponents();
		expect(all.length).toBeGreaterThan(30);
		expect(all.length).toBeLessThan(50);
		const names = all.map((c) => c.name);
		// one active variant per name (root twins of ds/ components are hidden)
		expect(new Set(names).size, 'duplicate names').toBe(names.length);
		// known residents of the showcase catalog
		for (const known of ['Accordion', 'Btn', 'Badge', 'BarChart', 'TableList']) {
			expect(names.includes(known), known).toBe(true);
		}
		// app-wired components without a standalone demo are not surfaced
		for (const gone of ['TableDrawer', 'ExprEditor', 'QueryEditor', 'PageGrid']) {
			expect(names.includes(gone), gone).toBe(false);
		}
		for (const c of all) {
			expect(c.lines, c.path).toBeGreaterThan(0);
			expect(c.source, c.path).toContain('<');
		}
	});
});
