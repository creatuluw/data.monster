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
	const m = path.match(/components\/(.*)\.svelte$/);
	const rel = m ? m[1] : path;
	const parts = rel.split('/');
	return {
		name: parts[parts.length - 1],
		dir: parts.length > 1 ? parts.slice(0, -1).join('/') : '',
		path: `src/lib/components/${rel}.svelte`,
		lines: source.trimEnd().split('\n').length,
		source
	};
}

const modules = import.meta.glob('/src/lib/components/**/*.svelte', {
	query: '?raw',
	import: 'default',
	eager: true
}) as Record<string, string>;

/** All app components, folder-then-name sorted. */
export function loadAppComponents(): AppComponent[] {
	return Object.entries(modules)
		.map(([p, src]) => parseComponent(p, src))
		.sort((a, b) => `${a.dir}/${a.name}`.localeCompare(`${b.dir}/${b.name}`));
}
