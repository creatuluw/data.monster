---
type: Decision
title: Agent authors app content by editing workspace files — the workspace folder is the interface (proposed)
description: Context
tags: [agents, workspace, storage, files-as-interface, proposed]
status: proposed
supersedes: "[]"
timestamp: "2026-09-22T13:18:37.993Z"
---

# Agent authors app content by editing workspace files — the workspace folder is the interface (proposed)

## Context

User steer (2026-09-22 planning session): "I mainly want to use the LLM by creating/editing files in the workspace that will create content, settings, data, connections etc in the frontend/desktop app as a user could do." The agent's native interface is already the filesystem — instead of teaching it an API, **the workspace folder itself becomes the interface**.

This redirects the content-authoring slice of [[agent-surfaces-rust-backend-mcp-and-rest]] (MCP + loopback REST + dm skill/CLI, 2026-09-15, still unshipped/proposed).

Why files beat an API for content authoring:

- **DuckDB's single-writer lock**: no second process can touch the DB while the app runs. Files sidestep that entirely — the agent uses its native read/write/glob tools. No API, no MCP, no token, nothing to install.
- **The workspace is already half file-based** ([[workspace-portable-bundle-switch-fully-reloads-the-db-and-se]]): settings.json is a file, source data is files under `data/main/`. What's left in DuckDB (`d8a_monster_*` content tables — pages, master items, relationships, saved queries) is tiny JSON docs nobody ever JOINs — in DuckDB by historical accident, not need ([[pages-master-items-storage-rust]]).
- **PageDoc is proven agent-authorable**: the Code tab edits exactly the JSON an agent would write, and `validatePageDoc` never throws (returns error paths) — a ready-made load-time validation gate ([[chart-page-spec-spec-types-validator]]).

## The choice (proposed — Q1 of the source-of-truth interview pending)

**Option A (recommended): files are canonical.** Move the `d8a_monster_*` content tables to JSON files in the workspace; DuckDB keeps only real data (source tables, query results). UI edits → file writes; agent edits → live reload (file watcher + Tauri event). No sync layer exists because there's nothing to sync. New machinery is one watcher + one reload event + **echo suppression** — the app's own writes (including the silent 60s pages auto-save) must not re-trigger the watcher.

## Alternatives considered

- **B — DB canonical, two-way file mirror**: works, but you own a sync engine with conflict rules — the exact complexity the file-world deletes. Rejected.
- **C — files import-only**: simplest, but agents round-trip on stale state — bad for exactly this use case. Rejected.
- **D — hybrid**: open; e.g. pages-as-files but master items stay in DB because stable ids are referenced everywhere. Q1 answer will settle this.

## Consequences

- If A lands, the [[pages-master-items-storage-rust]] persistence layer moves from DuckDB tables to files; bad JSON shows in a problems panel at load — nothing bricks.
- Ops that genuinely need the process (run a query, ingest a dropped CSV, create a Postgres connection) stay a much smaller, later surface — maybe a `dm` CLI on a loopback endpoint, maybe nothing at all if a watched `data/incoming/` folder covers it.
- [[agent-surfaces-rust-backend-mcp-and-rest]] stays proposed but **narrowed to ops** — not superseded yet.
- Status locks when the user answers Q1 (source of truth).
