---
type: Decision
title: Workspace files are canonical — agents author content by editing the dm/ tree in realtime
description: Context
tags: [agents, workspace, file-first, architecture, dm-tree, watcher]
status: accepted
supersedes: agent-connection-mcp-embedded-in-rust-backend
timestamp: "2026-09-22T13:22:24.193Z"
---

# Workspace files are canonical — agents author content by editing the dm/ tree in realtime

## Context

User steer (2026-09-22, session interview): "I mainly want to use the LLM by creating/editing files in the workspace that will create content, settings, data, connections etc. in the app." This changes the 2026-09-15 direction ([[agent-connection-mcp-embedded-in-rust-backend]], [[agent-surfaces-rust-backend-mcp-and-rest]], both still `proposed`, never built): instead of teaching agents to call APIs, the workspace folder itself becomes the agent interface — coding agents already have native file tools.

## The choice (user-locked)

**Workspace files are canonical for app content; DuckDB keeps only real data.** Four sub-decisions from the interview:

1. **Files are the source of truth** (option A) — `d8a_monster_pages/items/relationships` (+ saved queries) move from DuckDB tables to JSON/SQL files under `dm/`. Path is identity: slug = filename, master-item ref = filename, query name = filename. README.md in the workspace documents the tree for humans AND agents (self-describing, replaces most of the planned MCP/dm-skill onboarding).
2. **Layout conventions** — `dm/pages/<slug>.json`, `dm/master-items/{measures,dimensions}/<ref>.json`, `dm/relationships.json`, `dm/saved-queries/<name>.sql` (native .sql, not JSON-escaped), `dm/drafts/` ignored by the app. App only reads convention paths → agent can scribble anywhere safely.
3. **Full watcher** — `notify` crate, debounced, validate (validatePageDoc etc., never throws) → `dm:changed` / `dm:error` Tauri events → open views hot-reload, problems panel maps errors to file paths.
4. **Realtime both ways** — editor changes write through to files (debounced ~400ms, atomic temp+rename); save button and the 60s auto-save are DELETED (file IS the save; no unsaved-changes state exists). Echo suppression: Rust just-wrote set with window ≥ write debounce. Conflicts: VS Code model — on-disk change + local dirty → "Reload / Keep mine" banner; no merge machinery.
5. **All content types in one build** (user: "no, do all" — no content-type phasing): pages, master items, relationships, saved queries, AND new `dm/connections.json` persistence (connections were never persisted before).
6. **Agent docs are part of the system**: curated agent-facing docs ship in the workspace — `README.md` (L0 entry, ≤60 lines) + `dm/docs/` with skill-style PROGRESSIVE DISCLOSURE: `INDEX.md` (L1 wayfinding: intent → file → format doc, ≤100 lines) → `formats/*.md` (L2, `Read when:` header + one annotated minimal example, ≤150 lines) → `reference/` (L3 exhaustive). App-owned, embedded via `include_str!`, regenerated when `dm/docs/.version` ≠ app version. Line budgets enforced by test.
7. **`/agent` prompts page**: new route where users copy-paste starter prompts into their coding agent; every prompt mandates README→INDEX-first reading; collaborative prompts instruct interview-style work (one question at a time). Rendered via `marked` + `.prose-chat`.
8. **Workspaces are git-version-controlled-able — secrets never in content files** (user correction 2026-09-22, wiki rule `secrets-never-live-in-workspace-content-files`): my plaintext-password call was overruled. `.env` (workspace root, gitignored, app-generated when missing) is the ONLY secret home (`LLM_API_KEY`, `DM_CONN_<NAME>_PASSWORD`); `connections.json` carries `passwordEnv` refs; `settings.json` stops persisting `llmApiKey`; app generates `.gitignore` (`.env`, `*.duckdb`, `*.duckdb.wal`) without overwriting; `dm/` serialization is deterministic with no volatile timestamps (clean diffs). Agents are told to write env REFERENCES and let the user place secrets.

## Alternatives considered

- **MCP server + REST in Rust** (prior proposal) — deferred, not dead: DuckDB's single-writer lock means OPS (run query, ingest a dropped CSV) still need the app process; a tiny loopback REST + `dm` CLI remains the future answer for those. But content authoring needs none of it.
- **DB canonical + two-way file sync** — rejected: you own a sync engine with conflict rules; the file-world deletes the problem.
- **Import-only files** — rejected: agents round-trip on stale state.
- **Reload-on-focus instead of watcher** — fallback if the notify dep is unwanted; loses the live-building experience.

## Consequences

- One-time migration: export tables → files (write-all-first, drop-only-if-all-succeeded, crash-safe), then drop the content tables.
- Editor becomes a live view over the file — enables human↔agent pair-editing of the same document.
- Atomic writes required in BOTH app directions so agents never read partial files.
- Single build, no content-type phasing (user-locked). Executable spec: `.specs/workspace-file-first/spec.md` (FR-1..13; FR-13 `data/incoming/` drop folder is the one cuttable piece).
- Master-item filename = item `id` (charts reference `{ref: id}`); `kind` picks the measures/dimensions subfolder; `createdAt/updatedAt` dropped (file mtime replaces).
- Validator stays single-sourced in TS (`validatePageDoc`); Rust does parse-level shape checks only (enough to classify watcher events).
