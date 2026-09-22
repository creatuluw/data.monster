---
type: Entity
title: Write-through core (src/lib/write-through.ts)
description: A pure, Svelte-free write-through state machine — "the file IS the save" (workspace-file-first FR-9, task `files-009`, commit `05d6ea1`, 2026-09-22). All deboun
tags: [write-through, autosave, workspace-file-first, pages-editor, files-009]
timestamp: "2026-09-22T15:04:04.691Z"
---

# Write-through core (src/lib/write-through.ts)

A pure, Svelte-free write-through state machine — "the file IS the save" (workspace-file-first FR-9, task `files-009`, commit `05d6ea1`, 2026-09-22). All debouncing, coalescing, conflict classification, and rebaselining live here; the pages editor is a thin binder, and `files-010` will reuse it for the /data drawers + saved-queries UI.

## Details

- **Location**: `src/lib/write-through.ts` — unit-tested in `tests/write-through.test.ts` (7 tests, written RED-first)
- **Factory**: `createWriteThrough({ read, write, debounceMs = 400 })` — `read` returns current editor text (Design JSON or raw Code-mode text), `write` persists atomically via `save_page`
- **Interface** (`WriteThrough`):
  - `markLocal(text)` — a local edit; no-op when text is identical to `lastWritten` (baseline touch ≠ edit), else schedules the debounced flush
  - `flush()` — write pending edits now ("Keep mine"); failed writes keep `pending` set and log, never lose the edit
  - `decideExternal()` — an external `dm:changed` arrived: returns `'reload'` (clean → silently re-fetch) or `'conflict'` (unwritten local edits → amber Reload / Keep-mine banner, VS Code model)
  - `ackLoad(text)` — rebaseline after a load/reload; `pending`/`conflict`/`lastWritten` readonly state

## Editor wiring (pages editor)

- Two `$effect`s feed `markLocal`: Design/Page mutations (deep doc JSON) and Code-mode raw textarea text — **invalid JSON is written verbatim**, so the file mirrors the editor and the watcher flags it `dm:error`
- The /data round-trip flushes pending edits before navigating away
- Deleted with the old flow: Save button, `saving`/`saved` state, saved toast, the 60s auto-save interval, every explicit `handleSave` call (see the superseded [../../learnings/pages-editor-auto-saves-silently-every-60s-no-ui.md|60s auto-save learning](./learnings-pages-editor-auto-saves-silently-every-60s-no-ui-m.md))

## Relationships

- [dm-events-frontend-module](./dm-events-frontend-module.md) — supplies the external-change events; `decideExternal` classifies them against local pending state
- [dm-watch-command-module](./dm-watch-command-module.md) — echo suppression in the watcher makes write-throughs not reload-loop (writer-fed)
- [Workspace files are canonical](../../decisions/workspace-files-are-canonical-agents-author-content-by-editi.md) — the decision this implements
- [Workspace-file-first spec & tasks](../artifacts/workspace-file-first-spec-tasks.md) — FR-9; `files-010` extends this core to all drawers

## Lifecycle

- First added: 2026-09-22 — `files-009` (commit `05d6ea1`, branch `feature/workspace-file-first`)
- Next: `files-010` — same core wired into /data drawers + saved-queries UI
