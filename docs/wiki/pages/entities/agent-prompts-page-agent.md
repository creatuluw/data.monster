---
type: Entity
title: Agent prompts page (/agent)
description: Agent prompts page (/agent)
tags: [svelte, frontend, workspace-file-first, agents, prompts, route]
timestamp: "2026-09-22T15:29:40.286Z"
---

# Agent prompts page (/agent)

# Agent prompts page (/agent)

New top-level route `src/routes/agent/+page.svelte` — a grid of six copy-paste starter prompts (workspace-file-first FR-12, 2026-09-22) that hand a coding agent (Claude Code, Cursor, Codex, …) the workspace path so it can author app content — pages, master items, saved queries, connections — by writing `dm/` files. The user-visible front door to [workspace-files-are-canonical-agents-author-content-by-editi](./workspace-files-are-canonical-agents-author-content-by-editi.md).

## Why it matters

Connecting a coding agent previously required knowing the file conventions; this page makes it a copy-paste. Every prompt teaches the agent to read the workspace `README.md` and `dm/docs/` first — i.e. the prompts route into [agent-docs-module-agent-docs-rs](./agent-docs-module-agent-docs-rs.md) rather than duplicating it.

## Details

- **Location**: `src/routes/agent/+page.svelte`; loader `src/lib/agent-prompts.ts`; prompt bodies in `src/lib/agent-prompts/*.md` (`onboarding`, `ingest-csv`, `explain`, `cleanup`, `dashboard-interview`, `add-measure`)
- **Loader**: `import.meta.glob('./agent-prompts/*.md', { query: '?raw', eager: true })`; `parsePrompt` is a pure frontmatter parser (title / description / comma-separated tags), unit-tested; `loadPrompts()` returns prompts title-sorted
- **Card UI**: title, description, tag chips, rendered body (scrollable, `max-h-64`), per-card copy-to-clipboard button with a 1.5s "Copied" state; clipboard failure degrades to selectable text
- **Rendering**: `marked` + `.prose-chat` — per [rules/render-markdown-via-marked-prose-chat](../../rules/render-markdown-via-marked-prose-chat.md), never a new markdown pipeline
- No new npm dependencies

## Relationships

- [agent-docs-module-agent-docs-rs](./agent-docs-module-agent-docs-rs.md) — the docs the prompts point agents at
- [workspace-file-first-spec-tasks](./workspace-file-first-spec-tasks.md) — FR-12 of the executable spec

## Lifecycle

- First added: 2026-09-22, FR-12 of the workspace-file-first build
