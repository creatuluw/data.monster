import { describe, expect, it } from 'vitest';
import { parsePrompt, loadPrompts } from '../src/lib/agent-prompts';

describe('parsePrompt', () => {
	it('parses frontmatter and body', () => {
		const p = parsePrompt(
			'test',
			'---\ntitle: My prompt\ndescription: Does a thing\ntags: onboarding, interview\n---\n\nBody line one.\n\nBody line two.'
		);
		expect(p.title).toBe('My prompt');
		expect(p.description).toBe('Does a thing');
		expect(p.tags).toEqual(['onboarding', 'interview']);
		expect(p.body).toBe('Body line one.\n\nBody line two.');
	});

	it('tolerates values containing colons', () => {
		const p = parsePrompt('t', '---\ntitle: A: B: C\n---\nbody');
		expect(p.title).toBe('A: B: C');
	});

	it('falls back gracefully without frontmatter', () => {
		const p = parsePrompt('fallback.md', 'just text');
		expect(p.title).toBe('fallback.md');
		expect(p.body).toBe('just text');
		expect(p.tags).toEqual([]);
	});
});

describe('loadPrompts', () => {
	it('loads the six starter prompts, each meeting the content contract', () => {
		const prompts = loadPrompts();
		expect(prompts.length).toBeGreaterThanOrEqual(6);
		for (const p of prompts) {
			expect(p.title, p.file).toBeTruthy();
			expect(p.description, p.file).toBeTruthy();
			// every prompt points the agent at the workspace docs
			expect(p.body, p.file).toMatch(/README\.md/);
			// every prompt names the workspace folder placeholder
			expect(p.body, p.file).toContain('WORKSPACE FOLDER PATH');
		}
		const byFile = new Map(prompts.map((p) => [p.file, p]));
		// collaborative prompts instruct interview-style questioning (one at a time)
		const collab = byFile.get('dashboard-interview')!;
		expect(collab.body).toMatch(/ONE AT A TIME/);
		// the connection prompt must respect the secret policy
		const conn = byFile.get('ingest-csv');
		expect(conn).toBeTruthy();
	});
});
