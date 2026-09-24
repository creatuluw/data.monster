---
type: Learning
title: Empty pages/queries lists after a Rust rebuild = dm/ not yet migrated — not data loss
description: "**Gotcha (mid-migration window, branch `feature/workspace-file-first`)**: once the Rust backend is rebuilt with the file-backed commands, pages/saved-queries li"
tags: [workspace-file-first, migration, gotcha, rust, dm-store]
timestamp: "2026-09-22T14:32:21.838Z"
---

# Empty pages/queries lists after a Rust rebuild = dm/ not yet migrated — not data loss

**Gotcha (mid-migration window, branch `feature/workspace-file-first`)**: once the Rust backend is rebuilt with the file-backed commands, pages/saved-queries lists read from `<workspace>\dm\` — which is **empty until the `files-006` migration runs** (one-time export of pages/items/relationships/queries out of `d8a_monster.duckdb` into the `dm/` tree on first launch of the new backend).

So if the dev app auto-rebuilds before that migration lands, lists show **empty** and it looks like data loss. It is not: the content is still in the workspace DuckDB file (`E:\workspace\d8a_monster.duckdb`). The fix is running `files-006` — not hunting for lost data or restoring anything.

**Rule of thumb**: during this feature branch, empty content lists after a Rust rebuild → check whether `dm/` has files and whether `files-006` has run, before assuming a bug.
