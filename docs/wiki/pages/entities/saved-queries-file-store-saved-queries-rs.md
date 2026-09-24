---
type: Entity
title: Saved queries file store (saved_queries.rs)
description: The Rust content-command module that backs saved queries as **plain `.sql` files in the `dm/saved-queries/` workspace tree** — task `files-004` of the workspace
tags: [rust, backend, workspace-file-first, saved-queries, dm-store]
timestamp: "2026-09-22T14:22:33.104Z"
---

# Saved queries file store (saved_queries.rs)

The Rust content-command module that backs saved queries as **plain `.sql` files in the `dm/saved-queries/` workspace tree** — task `files-004` of the workspace-file-first build (commit `fd5a5fc`, 2026-09-22). Fourth of six Phase A content-type migrations, after pages (`files-002`) and master items + relationships (`files-003`); connections store (`files-005`) and migration + git bootstrap (`files-006`) remain.

## Why it matters

Saved queries become agent-native artifacts: a coding agent with only file tools reads/writes SQL directly — comments, quotes, newlines intact — while the app's invoke contract stays unchanged (same command names/params; the frontend still splits tags on `','`).

## Details

- **Location**: `src-tauri/src/commands/saved_queries.rs` (built on [dm-store-command-module](./dm-store-command-module.md))
- **File format**: `dm/saved-queries/<slug>.sql` — slug = `generate_slug(name)`; same-name overwrite semantics preserved; filename = identity
- **Meta header**: optional single-line `-- dm: {json}` first line carrying `name`/`description`/`tags`. App-written files always carry it; agent-written *headerless* files default name = filename stem with the whole file as SQL body
- **Tags duality**: JSON array in the file (agent-native), comma-separated string at the invoke boundary (frontend splits on `,`)
- **Fail loud**: a malformed `-- dm:` header is a reported error (appears in list `errors[]`; update refuses) — never silently treated as SQL; applies the [single-doc-stores-fail-loud-never-clobber](../../rules/single-doc-stores-fail-loud-never-clobber.md) philosophy to a per-file store
- **Update semantics**: partial-merge keeps unspecified fields; rename keeps slug/filename stable; zero timestamps in files (git-clean diffs) — mtime is surfaced as ISO `updated`

## Lifecycle

- First added: 2026-09-22, task `files-004` (commit `fd5a5fc` on the workspace-file-first branch; 7 new tests, full suite 95/95)

## Relationships

- [dm-store-command-module](./dm-store-command-module.md) — path conventions, atomic writes, shape checks underneath
- [workspace-file-first-spec-tasks](../artifacts/workspace-file-first-spec-tasks.md) — `files-004` of the executable spec
- [workspace-files-are-canonical-agents-author](../../decisions/workspace-files-are-canonical-agents-author.md) — the architecture this implements
- [master-items-relationships-file-store-items-rs](./master-items-relationships-file-store-items-rs.md) — sibling Phase A store (`files-003`)
