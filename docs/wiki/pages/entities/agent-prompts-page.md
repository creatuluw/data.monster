---
type: Entity
title: Agent prompts page
description: "The SvelteKit route `/agent` (`src/routes/agent/+page.svelte`): six curated copy-paste prompts that teach a coding agent to operate data.monster through the wor"
tags: [agents, prompts, route, frontend, dm-tree]
timestamp: "2026-09-22T15:30:22.588Z"
---

# Agent prompts page

The SvelteKit route `/agent` (`src/routes/agent/+page.svelte`): six curated copy-paste prompts that teach a coding agent to operate data.monster through the workspace `dm/` tree — onboarding, dashboard-interview, add-measure, ingest-csv, cleanup, explain. Reached from the nav via the BookOpen button.

Prompts live as markdown files with frontmatter (`title`, `description`, `tags`) under `src/lib/agent-prompts/`, loaded with `import.meta.glob` (raw) and parsed by the pure, tested `parsePrompt()` in `src/lib/agent-prompts.ts`. Every prompt mandates README→INDEX-first reading of the [agent-docs-system](./agent-docs-system.md) docs and restates the secret rules. Rendering follows the house rule [rules/render-markdown-via-marked-prose-chat](./rules-render-markdown-via-marked-prose-chat.md) (marked + `.prose-chat`).

## Details

- **Location**: `src/routes/agent/+page.svelte` (69 lines) · `src/lib/agent-prompts.ts` (loader/parser) · `src/lib/agent-prompts/*.md` (6 prompt files)
- **Interface**: `type Prompt = { file, title, description, tags, body }` · `parsePrompt(file, raw)` — tolerant frontmatter parser (missing frontmatter → filename as title)
- **Prompts**: `onboarding`, `dashboard-interview`, `add-measure`, `ingest-csv`, `cleanup`, `explain`

## Relationships

- [agent-docs-system](./agent-docs-system.md) — the embedded doc layer every prompt tells the agent to read first
- [decisions/workspace-files-are-canonical-agents-author-content-by-editi](./decisions-workspace-files-are-canonical-agents-author-conten.md) — why prompts target files, not app UI
- [incoming-drop-folder](./incoming-drop-folder.md) — the ingest prompt's zero-code alternative (just place the file)

## Lifecycle

- First added: files-012, 2026-09-22 (commit `1e7709b`) — phase C of [pages/artifacts/workspace-file-first-spec-tasks](./workspace-file-first-spec-tasks.md)
