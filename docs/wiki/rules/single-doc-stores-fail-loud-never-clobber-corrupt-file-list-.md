---
type: Rule
title: "Single-doc stores fail loud, never clobber — corrupt file: list errors, save refuses"
description: "**The rule**: any store backed by a single agent-editable doc file (e.g. `dm/relationships.json`, later the connections store) must fail loud instead of clobber"
tags: [workspace-file-first, data-safety, rust, backend]
timestamp: "2026-09-22T14:17:14.745Z"
---

# Single-doc stores fail loud, never clobber — corrupt file: list errors, save refuses

**The rule**: any store backed by a single agent-editable doc file (e.g. `dm/relationships.json`, later the connections store) must fail loud instead of clobbering: if the file is corrupt or unparseable, `list`/`get` surfaces the error to the caller, and `save` **refuses to write** rather than overwriting the hand-edited doc with an empty or default one.

**When it applies**: read-modify-write stores over single docs in the `dm/` tree, where an agent or user may be mid-edit while the app reads.

**Rationale**: shipped in `files-003` (`src-tauri/src/commands/relationships.rs`, 2026-09-22). Without the guard, a save triggered during a half-written agent edit silently destroys the relationship graph — the exact "the clobber" failure mode the workspace-file-first design was built to avoid. Per-item JSON stores (master items) don't need it: a broken item lands in `errors[]` and the rest of the list survives.

**Always materialize, even empty** (added `files-006`, 2026-09-22): a single-doc store must write its file **even when the collection is empty** — an empty relationships graph still produces `dm/relationships.json`. The migration test caught the edge: an empty table produced no file, and `verify` then failed on the missing doc. `store_in` is exposed so migration and other writers share the exact same write path.
