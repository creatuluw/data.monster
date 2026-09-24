/**
 * App-component inventory (/components): every .svelte under src/lib/components,
 * discovered at build time via import.meta.glob — new components appear automatically.
 * Raw sources loaded with '?raw' so the page can show line counts and source.
 */

export type AppComponent = {
	name: string;
	dir: string;
	path: string;
	lines: number;
	source: string;
};

export function parseComponent(path: string, source: string): AppComponent {
	const m = path.match(/\/(components|demos)\/(.*)\.svelte$/);
	const rel = m ? m[2] : path;
	const parts = rel.split('/');
	return {
		name: parts[parts.length - 1],
		dir: parts.length > 1 ? parts.slice(0, -1).join('/') : '',
		path: `src/lib/${m ? m[1] : 'components'}/${rel}.svelte`,
		lines: source.trimEnd().split('\n').length,
		source
	};
}

const modules = {
	...import.meta.glob('/src/lib/components/**/*.svelte', {
		query: '?raw',
		import: 'default',
		eager: true
	}),
	// design-system showcase demos (/ui sections + some /components catalog entries)
	...import.meta.glob('/src/lib/demos/**/*.svelte', {
		query: '?raw',
		import: 'default',
		eager: true
	})
} as Record<string, string>;

/** Names that have a live visual demo in ComponentDemo.svelte — the /components
 *  catalog surfaces only these; keep in sync with that component's if/else chain. */
export const SHOWCASEABLE = new Set([
	// ds/ self-showcasing components (same set /ui renders)
	'Accordion', 'Buttons', 'Cards', 'ColorPalette', 'Footer', 'Hero', 'Icons', 'Inputs',
	'Modal', 'Motion', 'Nav', 'Principles', 'SearchAhead', 'Spacing', 'Tags', 'Toast',
	'Toggles', 'Typography',
	// charts/controls form kit
	'Btn', 'DangerZone', 'Field', 'NumberInput', 'RemoveBtn', 'Section', 'Select',
	'TextInput', 'Toggle',
	// root kit with hand-written demos
	'Badge', 'Breadcrumb', 'Drawer', 'LabsPlaceholder', 'Pagination', 'PreviewPane', 'Tag',
	'TagInput', 'TableList', 'Tabs', 'Tooltip',
	// charts
	'BarChart'
]);

/** Active components with a live visual demo, folder-then-name sorted. */
export function loadAppComponents(): AppComponent[] {
	return loadActiveAppComponents().filter((c) => SHOWCASEABLE.has(c.name));
}

/** Active components (superseded twins removed: ds > charts/controls > charts > root),
 *  folder-then-name sorted. */
export function loadActiveAppComponents(): AppComponent[] {
	const all = Object.entries(modules).map(([p, src]) => parseComponent(p, src));
	const rank = (dir: string) => {
		const i = PREFER.indexOf(dir);
		return i === -1 ? PREFER.length : i;
	};
	const active = new Map<string, AppComponent>();
	for (const c of all) {
		const cur = active.get(c.name);
		if (!cur || rank(c.dir) < rank(cur.dir)) active.set(c.name, c);
	}
	return [...active.values()].sort((a, b) => `${a.dir}/${a.name}`.localeCompare(`${b.dir}/${b.name}`));
}

const PREFER = ['charts/controls', 'charts', 'demos', ''];
