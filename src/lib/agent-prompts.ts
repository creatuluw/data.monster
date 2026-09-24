/**
 * /agent prompts (workspace-file-first FR-12): curated copy-paste prompts that teach a
 * coding agent to operate the app via workspace files. Loaded with import.meta.glob
 * (raw), frontmatter parsed here (pure, tested). House rule: rendered via marked +
 * .prose-chat — never a new pipeline.
 */

export type Prompt = {
	file: string;
	title: string;
	description: string;
	tags: string[];
	goal: string;
	body: string;
};

/** Parse `---\ntitle: ...\ndescription: ...\ntags: a, b\n---\n<body>`. */
export function parsePrompt(file: string, raw: string): Prompt {
	const m = raw.match(/^---\n([\s\S]*?)\n---\n?([\s\S]*)$/);
	if (!m) return { file, title: file, description: '', tags: [], goal: '', body: raw };
	const meta: Record<string, string> = {};
	for (const line of m[1].split('\n')) {
		const [k, ...rest] = line.split(':');
		if (rest.length) meta[k.trim()] = rest.join(':').trim();
	}
	return {
		file,
		title: meta.title || file,
		description: meta.description || '',
		tags: (meta.tags || '')
			.split(',')
			.map((t) => t.trim())
			.filter(Boolean),
		goal: meta.goal || '',
		body: m[2].trim()
	};
}

const modules = import.meta.glob('./agent-prompts/*.md', {
	query: '?raw',
	import: 'default',
	eager: true
}) as Record<string, string>;

/** All prompts, title-sorted. */
export function loadPrompts(): Prompt[] {
	return Object.entries(modules)
		.map(([path, raw]) =>
			parsePrompt(path.split('/').pop()!.replace(/\.md$/, ''), raw)
		)
		.sort((a, b) => a.title.localeCompare(b.title));
}

export const WORKSPACE_PLACEHOLDER = '<PASTE WORKSPACE FOLDER PATH>';

/** Swap the placeholder for the live workspace path; keeps it when none is open. */
export function injectWorkspace(body: string, workspacePath: string | null): string {
	return workspacePath ? body.split(WORKSPACE_PLACEHOLDER).join(workspacePath) : body;
}
