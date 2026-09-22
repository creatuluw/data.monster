---
type: Entity
title: Master items & relationships file store (items.rs + relationships.rs)
description: The Rust content-command modules that back master items and the relationship graph as **files in the `dm/` workspace tree** — task `files-003` of the workspace-
tags: [rust, backend, workspace-file-first, master-items, relationships, dm-store]
timestamp: "2026-09-22T14:17:07.308Z"
---

# Master items & relationships file store (items.rs + relationships.rs)

The Rust content-command modules that back master items and the relationship graph as **files in the `dm/` workspace tree** — task `files-003` of the workspace-file-first build (commit `55fbea3`, 2026-09-22). Replaces the DuckDB-backed storage in [pages-master-items-storage-rust](./pages-master-items-storage-rust.md) for these two content types; `/pages` content was already file-backed by `files-002`.

## Why it matters

Charts reference master items by stable id (`{"ref": id}`) and the relationship graph drives item availability — both must survive agent hand-edits and git versioning. Files make the workspace folder the interface: agents author content by editing the tree directly, with zero timestamps so diffs stay clean.

## Details

- **Location**: `src-tauri/src/commands/items.rs`, `src-tauri/src/commands/relationships.rs` (built on [dm-store-command-module](./dm-store-command-module.md))
- **Master items**: `dm/master-items/{measures,dimensions}/<id>.json` — filename = the stable id; `get` searches both folders; `delete` is idempotent; invalid agent-written files land in `errors[]` without breaking the list
- **Relationships**: one `relationships.json` doc, read-modify-write, upsert-by-id keeps the generated `rel-<from>-<to>` slug
- **Clobber guard**: if the doc is corrupt (agent mid-edit), `list` errors loudly and `save` **refuses** rather than overwriting the hand-edited graph with an empty one — see [single-doc-stores-fail-loud-never-clobber](../../rules/single-doc-stores-fail-loud-never-clobber-corrupt-file-list-.md)
- **Tests**: 13 new in-module tests (full suite 92/92)

## Lifecycle

- First added: 2026-09-22, task `files-003` (commit `55fbea3` on `feature/workspace-file-first`)
- Caught during the task: `dm_store::item_dir` initially created singular `measure/`/`dimension/` folders — corrected to the spec's plural `measures/`/`dimensions/`, because the README tree agents will see comes from the spec, and **the spec wins over the implementation** when they disagree

## Relationships

- [dm-store-command-module](./dm-store-command-module.md) — path conventions, atomic writes, shape checks underneath
- [workspace-file-first-spec-tasks](../artifacts/workspace-file-first-spec-tasks.md) — `files-003` of the executable spec
- [workspace-files-are-canonical-agents-author-content-by-editi](../../decisions/workspace-files-are-canonical-agents-author-content-by-editi.md) — the architecture this implements
- [pages-master-items-storage-rust](./pages-master-items-storage-rust.md) — the DuckDB storage being replaced
