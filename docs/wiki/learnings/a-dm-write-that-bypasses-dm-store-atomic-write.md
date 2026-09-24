---
type: Learning
title: "A dm/ write that bypasses dm_store::atomic_write reload-loops — echo suppression is fed by the writer"
description: "Shipped in `files-007` (2026-09-22): the `dm/` watcher skips any path recorded by `dm_watch::mark_self_write` — and the only caller is `dm_store::atomic_write`,"
tags: [workspace-file-first, dm-watch, echo-suppression, gotcha, rust]
timestamp: "2026-09-22T14:53:30.985Z"
---

# A dm/ write that bypasses dm_store::atomic_write reload-loops — echo suppression is fed by the writer

Shipped in `files-007` (2026-09-22): the `dm/` watcher skips any path recorded by `dm_watch::mark_self_write` — and the only caller is `dm_store::atomic_write`, which marks the destination right after a successful rename. Registry window 600ms ≥ the pages editor's 400ms write debounce, so self-writes can never echo back.

**Consequence**: any new code path that writes files under `dm/` MUST go through `dm_store::atomic_write`. A bare `std::fs::write` bypasses the registry, and the watcher classifies the app's own write as an external change — a spurious `dm:changed` reload loop or `dm:error`.

Debounce semantics worth knowing when touching the flusher: within one 300ms window, a removal beats a modify for the same path (`pending.entry().and_modify(e.1 = e.1 || removed)`); paths still inside the window are carried to the next tick, not dropped.
