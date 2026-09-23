---
type: Entity
title: Agent prompts page
description: "The SvelteKit route `/agent` (`src/routes/agent/+page.svelte`): six curated copy-paste prompts that teach a coding agent to operate data.monster through the wor"
tags: [agents, prompts, route, frontend, dm-tree, onboarding]
timestamp: "2026-09-22T16:45:59.945Z"
---

# Agent prompts page

The SvelteKit route `/agent` (`src/routes/agent/+page.svelte`): six curated copy-paste prompts that teach a coding agent to operate data.monster through the workspace `dm/` tree, grouped under four starter goals. Reached from two entry points: the homepage **✦ LLM skills** button (always visible, even pre-data — that's when onboarding matters) and the nav BookOpen button.

Prompts live as markdown files with frontmatter (`title`, `description`, `tags`, `goal`) under `src/lib/agent-prompts/`, loaded with `import.meta.glob` (raw) and parsed by the pure, tested `parsePrompt()` in `src/lib/agent-prompts.ts`. Every prompt mandates README→INDEX-first reading of the [agent-docs-system](./agent-docs-system.md) docs and restates the secret rules. Rendering follows the house rule [render-markdown-via-marked-prose-chat rule](../../rules/render-markdown-via-marked-prose-chat.md) (marked + `.prose-chat`).

## Why it matters

Connecting a coding agent previously required knowing the file conventions; this page makes it a copy-paste, and the goal grouping turns six prompts into a guided onboarding path: a new user picks what they want to do first (data → content → insights → actions).

## Details

- **Location**: `src/routes/agent/+page.svelte` · `src/lib/agent-prompts.ts` (loader/parser) · `src/lib/agent-prompts/*.md` (6 prompt files)
- **Interface**: `type Prompt = { file, title, description, tags, goal, body }` · `parsePrompt(file, raw)` — tolerant frontmatter parser (missing frontmatter → filename as title, empty goal)
- **Goals** (`GOALS` in the page component): `data` = Create data (Get data into the workspace and shape it), `content` = Create content (Build report pages and dashboards), `insights` = Get insights (Reusable analysis building blocks), `actions` = Take actions (Understand and keep the workspace healthy)
- **Prompts → goals**: `onboarding` (Connect data & build my first page) + `ingest-csv` → data · `dashboard-interview` → content · `add-measure` + `explain` (Explain this workspace) → insights · `cleanup` (Audit & clean up my workspace) → actions
- **Layout**: one section per goal (label + one-line blurb + md:grid-cols-2 card grid); cards keep title, description, tag chips, rendered body, per-card copy-to-clipboard with 1.5s "Copied" state
- **Tests** (`tests/agent-prompts.test.ts`): every prompt carries a valid `goal`; every goal has ≥1 prompt — the grouping can't silently break

## Relationships

- [agent-docs-system](./agent-docs-system.md) — the embedded doc layer every prompt tells the agent to read first
- [agent-docs-module-agent-docs-rs](./agent-docs-module-agent-docs-rs.md) — the Rust module serving those docs
- [workspace-files-are-canonical decision](../../decisions/workspace-files-are-canonical-agents-author.md) — why prompts target files, not app UI
- [incoming-drop-folder](./incoming-drop-folder.md) — the ingest prompt's zero-code alternative (just place the file)

## Lifecycle

- First added: files-012, 2026-09-22 (commit `1e7709b`) — phase C of [workspace-file-first-spec-tasks](../artifacts/workspace-file-first-spec-tasks.md)
- 2026-09-22 (commit `0e1afed`): prompts grouped under four starter goals — `goal:` frontmatter field added, parser carries it, page renders goal sections; homepage gained the always-visible ✦ LLM skills button linking here
