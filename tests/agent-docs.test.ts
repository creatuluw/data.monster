// @ts-nocheck — node built-ins used to read embedded doc sources; runtime is vitest/node,
// but the app tsconfig intentionally has no node types (tests live outside src/).
import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';
import { describe, expect, it } from 'vitest';

/**
 * FR-11 acceptance: progressive-disclosure line budgets, INDEX integrity, and
 * parseable examples — enforced against the embedded source files so future edits
 * can't silently bloat the agent's token cost.
 */
const ROOT = join(process.cwd(), 'src-tauri', 'agent-docs');

function walk(dir: string): string[] {
    return readdirSync(dir).flatMap((f) => {
        const p = join(dir, f);
        return statSync(p).isDirectory() ? walk(p) : [p];
    });
}

const all = walk(ROOT);
const byRel = new Map(all.map((p) => [relative(ROOT, p).replace(/\\/g, '/'), p]));

describe('agent docs progressive disclosure', () => {
    it('line budgets: README ≤ 60, INDEX ≤ 100, formats/concepts/recipes ≤ 150', () => {
        const budgets: Record<string, number> = {
            'README.md': 60,
            'INDEX.md': 100,
            'concepts.md': 150,
            'recipes/common-tasks.md': 150,
            'formats/page-doc.md': 150,
            'formats/master-items.md': 150,
            'formats/saved-queries.md': 150,
            'formats/connections.md': 150,
            'formats/relationships.md': 150,
            'reference/page-doc-fields.md': 400 // L3: exhaustive is the point
        };
        for (const [rel, max] of Object.entries(budgets)) {
            const p = byRel.get(rel);
            expect(p, `${rel} exists`).toBeTruthy();
            const lines = readFileSync(p!, 'utf-8').split('\n').length;
            expect(lines, `${rel} line budget`).toBeLessThanOrEqual(max);
        }
    });

    it('every doc carries a purpose header and INDEX targets exist', () => {
        for (const rel of byRel.keys()) {
            if (rel === 'README.md') continue;
            const body = readFileSync(byRel.get(rel)!, 'utf-8');
            expect(body, `${rel} should start with a markdown title`).toMatch(/^# /);
        }
        const index = readFileSync(byRel.get('INDEX.md')!, 'utf-8');
        for (const m of index.matchAll(/`((?:formats|reference|recipes)\/[a-z-]+\.md)`/g)) {
            expect(byRel.has(m[1]), `INDEX target ${m[1]} exists`).toBe(true);
        }
    });

    it('format examples are parseable (JSON / SQL header)', () => {
        // page-doc + master-items + connections + relationships examples must be valid JSON
        for (const rel of ['formats/page-doc.md', 'formats/master-items.md', 'formats/connections.md', 'formats/relationships.md']) {
            const body = readFileSync(byRel.get(rel)!, 'utf-8');
            const blocks = [...body.matchAll(/```json\n([\s\S]*?)```/g)].map((m) => m[1]);
            expect(blocks.length, `${rel} has a json example`).toBeGreaterThan(0);
            for (const b of blocks) expect(() => JSON.parse(b), `${rel} example parses`).not.toThrow();
        }
        // saved-queries example header must parse
        const sq = readFileSync(byRel.get('formats/saved-queries.md')!, 'utf-8');
        const header = sq.match(/-- dm: (.+)$/m)?.[1];
        expect(header).toBeTruthy();
        expect(() => JSON.parse(header!)).not.toThrow();
    });

    it('secret policy: the connections.json example agents copy is secret-free', () => {
        // the .env placeholder URL in the doc is intentional (it teaches where secrets go);
        // the DOC FORMAT agents write must carry no credentials
        const body = readFileSync(byRel.get('formats/connections.md')!, 'utf-8');
        const example = body.match(/```json\n([\s\S]*?)```/)![1];
        const doc = JSON.parse(example) as { connections: Record<string, unknown>[] };
        for (const conn of doc.connections) {
            expect(JSON.stringify(conn).toLowerCase()).not.toContain('password');
            expect(JSON.stringify(conn)).not.toContain('://');
        }
        for (const rel of ['INDEX.md', 'README.md']) {
            expect(readFileSync(byRel.get(rel)!, 'utf-8').includes('password@'), `${rel} plaintext password`).toBe(false);
        }
    });
});
