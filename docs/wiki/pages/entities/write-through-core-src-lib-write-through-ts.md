---
type: Entity
title: Write-through core (src/lib/write-through.ts)
description: A pure, Svelte-free write-through state machine — "the file IS the save" (workspace-file-first FR-9, task `files-009`, commit `05d6ea1`, 2026-09-22). All deboun
tags: [write-through, autosave, workspace-file-first, pages-editor, itemeditor, files-009, files-010]
timestamp: "2026-09-22T15:10:55.895Z"
---

# Write-through core (src/lib/write-through.ts)

A pure, Svelte-free write-through state machine — "the file IS the save" (workspace-file-first FR-9, task `files-009`, commit `05d6ea1`, 2026-09-22). All debouncing, coalescing, conflict classification, and rebaselining live here; editors are thin binders. Reused by the pages editor (FR-9) and the ItemEditor (FR-10, commit `b9bb4b9`).

## Details

- **Location**: `src/lib/write-through.ts` — unit-tested in `tests/write-through.test.ts` (7 tests, written RED-first)
- **Factory**: `createWriteThrough({ read, write, debounceMs = 400 })` — `read` returns current editor text (Design JSON, raw Code-mode text, or item-draft JSON), `write` persists atomically to the document's `dm/` file
- **Interface** (`WriteThrough`):
  - `markLocal(text)` — a local edit; no-op when text is identical to `lastWritten` (baseline touch ≠ edit), else schedules the debounced flush
  - `flush()` — write pending edits now ("Keep mine"); failed writes keep `pending` set and log, never lose the edit
  - `decideExternal()` — an external `dm:changed` arrived: returns `'reload'` (clean → silently re-fetch) or `'conflict'` (unwritten local edits → amber Reload / Keep-mine banner, VS Code model)
  - `ackLoad(text)` — rebaseline after a load/reload; `pending`/`conflict`/`lastWritten` readonly state

## Editor wiring

**Pages editor** (`/pages/[slug]`, files-009):

- Two `$effect`s feed `markLocal`: Design/Page mutations (deep doc JSON) and Code-mode raw textarea text — **invalid JSON is written verbatim**, so the file mirrors the editor and the watcher flags it `dm:error`
- The /data round-trip flushes pending edits before navigating away
- Deleted with the old flow: Save button, `saving`/`saved` state, saved toast, the 60s auto-save interval, every explicit `handleSave` call (see the superseded [60s auto-save learning](../../learnings/pages-editor-auto-saves-silently-every-60s-no-ui.md))

**ItemEditor** (master-item drawer, files-010) — splits on `draft.id` per the [creation-saves-explicitly decision](../../decisions/creation-saves-explicitly-editing-writes-through-live-files-.md):

- **Editing an existing item → live write-through**: an `$effect` feeds every draft field change into `markLocal`; each change debounces 400ms into `dm/master-items/<id>.json`. The close button became **Done** (`flush()` + close)
- **Creation keeps the explicit Save**: the stable id (`mi_<table>_<label>`) is minted at save time — live write-through during creation would write half-drafted files with empty/unstable ids into the agent-facing `dm/` tree
- **Ceiling**: drawer conflicts are last-write-wins — the Reload/Keep-mine banner stays reserved for the pages editor, whose documents are long-lived

**RelationshipEditor + saved-queries/connections UIs**: verified already mutation-immediate (every add/update/delete command writes files) — zero wiring needed.

## Relationships

- [dm-events-frontend-module](./dm-events-frontend-module.md) — supplies the external-change events; `decideExternal` classifies them against local pending state
- [dm-watch-command-module](./dm-watch-command-module.md) — echo suppression in the watcher makes write-throughs not reload-loop (writer-fed)
- [Workspace files are canonical](../../decisions/workspace-files-are-canonical-agents-author-content-by-editi.md) — the decision this implements
- [Creation saves explicitly, editing writes through live](../../decisions/creation-saves-explicitly-editing-writes-through-live-files-.md) — the files-010 save-semantics convention
- [Workspace-file-first spec & tasks](../artifacts/workspace-file-first-spec-tasks.md) — FR-9 + FR-10

## Lifecycle

- First added: 2026-09-22 — `files-009` (commit `05d6ea1`, branch `feature/workspace-file-first`), pages editor
- Extended: 2026-09-22 — `files-010` (commit `b9bb4b9`): ItemEditor live write-through for existing items, Done button; RelationshipEditor/saved-queries/connections confirmed already file-immediate. Phase B complete (10 of 13)
