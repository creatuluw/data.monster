import { describe, expect, it } from 'vitest';
import { parseComponent, loadAppComponents } from '../src/lib/app-components';

describe('parseComponent', () => {
	it('splits top-level components', () => {
		const c = parseComponent('/src/lib/components/Drawer.svelte', 'a\nb\n');
		expect(c.name).toBe('Drawer');
		expect(c.dir).toBe('');
		expect(c.path).toBe('src/lib/components/Drawer.svelte');
		expect(c.lines).toBe(2);
	});

	it('keeps subfolder as dir', () => {
		const c = parseComponent('/src/lib/components/charts/ChartCard.svelte', 'x');
		expect(c.name).toBe('ChartCard');
		expect(c.dir).toBe('charts');
		expect(c.path).toBe('src/lib/components/charts/ChartCard.svelte');
	});
});

describe('loadAppComponents', () => {
	it('returns one showcaseable entry per component, demos excluded', () => {
		const all = loadAppComponents();
		expect(all.length).toBeGreaterThan(25);
		expect(all.length).toBeLessThan(50);
		const names = all.map((c) => c.name);
		// one entry per component
		expect(new Set(names).size, 'duplicate names').toBe(names.length);
		// known residents of the showcase catalog
		for (const known of ['Btn', 'Badge', 'PageGrid', 'TableDrawer', 'ExprEditor', 'Tabs']) {
			expect(names.includes(known), known).toBe(true);
		}
		// deleted twins / demo files / meta files are not surfaced
		for (const gone of ['Modal', 'SearchAhead', 'Toast', 'QueryEditor', 'Accordion', 'Tooltip', 'ComponentDemo']) {
			expect(names.includes(gone), gone).toBe(false);
		}
		for (const c of all) {
			expect(c.lines, c.path).toBeGreaterThan(0);
			expect(c.source, c.path).toContain('<');
		}
	});
});
