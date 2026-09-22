---
type: Artifact
title: Workspace-file-first spec & tasks
description: "The executable spec + task list for the workspace-file-first build: 13 FRs (FR-1..13) across four phases — A Files-are-canonical (`dm_store` module, pages/maste"
tags: [workspace, file-first, spec, tasks, agents, dm-tree, planning]
timestamp: "2026-09-22T13:34:25.812Z"
---

# Workspace-file-first spec & tasks

The executable spec + task list for the workspace-file-first build: 13 FRs (FR-1..13) across four phases — A Files-are-canonical (`dm_store` module, pages/master-items/relationships/saved-queries/connections remapped to `dm/` files, one-time migration), B Realtime both ways (notify watcher → `dm:changed`/`dm:error`, live-reload, editor write-through, save button + 60s auto-save deleted), C Docs + prompts (agent docs layer with progressive disclosure, `/agent` prompts page), D Extras (FR-13 `data/incoming/` drop folder — the one cuttable piece).

## What it documents

- The locked scope of [decisions/workspace-files-are-canonical-agents-author-content-by-editi](../../decisions/workspace-files-are-canonical-agents-author-content-by-editi.md) — user-locked: all content types in one build, no phasing
- New persistence surface: `dm/connections.json` (Postgres connections were never persisted before — this is new persistence, not a migration)
- Design details not in the decision: saved-query metadata in an optional leading `-- dm: {json}` comment header (SQL body stays native); master-item filename = item `id`, `kind` picks the measures/dimensions subfolder; validator single-sourced in TS, Rust does parse-level shape checks only (enough to classify watcher events)
- Per-task methodology follows [rules/spec-driven-features-tdd-karpathy-in-todos](../../rules/spec-driven-features-tdd-karpathy-in-todos.md) — TDD red-first, starting with task `files-001` (`dm_store` path mapping + atomic writes)

- Progress (2026-09-22): Phase A shipped (`files-001`–`006`: dm_store + pages/master-items/saved-queries/connections file-backed + one-time migration); Phase B 3 of 4 — `files-007` shipped the notify watcher ([dm_watch command module](../entities/dm-watch-command-module.md)), `files-008` shipped frontend live-reload ([dm-events frontend module](../entities/dm-events-frontend-module.md)); `files-009` shipped the write-through editor ([write-through core](../entities/write-through-core-src-lib-write-through-ts.md) - Save button, saved toast, and 60s auto-save deleted; Reload/Keep-mine conflict banner added); `files-010` shipped the ItemEditor write-through (edit = live write-through, create = explicit Save; RelationshipEditor + saved-queries/connections verified already mutation-immediate — see [decisions/creation-saves-explicitly-editing-writes-through-live-files-](../../decisions/creation-saves-explicitly-editing-writes-through-live-files-.md)). **All 13 tasks done (2026-09-22)** — Phase C: `files-011` agent docs tree ([agent docs system](../entities/agent-docs-system.md)) + `files-012` /agent prompts page ([agent prompts page](../entities/agent-prompts-page.md)); Phase D: `files-013` `data/incoming/` drop folder ([incoming drop folder](../entities/incoming-drop-folder.md)). Branch `feature/workspace-file-first`: 15 commits `b8d23b4`…`93ab24c`, cargo 121/121, vitest 140/140, svelte-check clean.
## Details

- **Format**: Spec-driven markdown + machine-readable tasks (`.specs/workspace-file-first/tasks.json` with acceptanceCriteria per task)
- **Location**: `.specs/workspace-file-first/spec.md`
- **Docs layer contract (FR-11)**: repo-authored `dm/docs/` embedded via `include_str!`, regenerated when `dm/docs/.version` ≠ app version; L0 workspace README ≤60 lines → L1 `INDEX.md` ≤100 → L2 `formats/*.md` ≤150 with `Read when:` header + one annotated example → L3 `reference/` exhaustive; line budgets enforced by test

## Relationships

- [Workspace files are canonical (decision)](../../decisions/workspace-files-are-canonical-agents-author-content-by-editi.md) — the decision this spec encodes; supersedes the MCP-embedded proposals
- [Pages & master-items storage (Rust)](../entities/pages-master-items-storage-rust.md) — the DuckDB content tables this build migrates out of and drops
- [Workspace command module (Rust)](../entities/workspace-command-module-rust.md) — sibling backend module; `dm_store` joins it for path mapping + atomic writes

## Source

- `.specs/workspace-file-first/spec.md` — the 13 FRs
- `.specs/workspace-file-first/tasks.json` — the TDD task list
