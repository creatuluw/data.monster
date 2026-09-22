---
type: Decision
title: Creation saves explicitly, editing writes through live (files-010)
description: Context
tags: [workspace-file-first, files-010, write-through, save-semantics, itemeditor]
status: accepted
timestamp: "2026-09-22T15:09:28.910Z"
---

# Creation saves explicitly, editing writes through live (files-010)

## Context

Phase B of workspace-file-first (`files-010`) set out to remove every per-document Save button — "nobody presses Save". Scoping first showed only ONE true per-document editor exists: the master-item **ItemEditor** drawer. `RelationshipEditor` and the saved-queries/connections UIs were verified **already mutation-immediate** — every add/update/delete backend command writes the `dm/` file directly, so there was nothing to convert there.

## The choice

ItemEditor splits on `draft.id`:

- **Editing an existing item → live write-through**: a `$effect` feeds `markLocal` into the shared `createWriteThrough` core; every field change debounces (400ms) into `dm/master-items/<id>.json`. The close button became **Done** (`flush()` + close).
- **Creation keeps the explicit Save button**: the stable id (`mi_<table>_<label>`) is minted at save time. Live write-through during creation would write half-drafted files with empty/unstable ids into the agent-facing `dm/` tree.

## Alternatives considered

- Make creation live too — rejected: half-drafted files pollute `dm/` and the id isn't stable until table+label settle.
- Add explicit save buttons to relationships/saved-queries/connections — rejected: they are already file-immediate; a Save button would be a regression.

## Consequences

- Drawer conflicts are **last-write-wins** (documented ceiling): the Reload/Keep-mine banner stays reserved for the pages editor, whose documents are long-lived.
- This is the convention for any future per-document editor on `dm/` files: create = explicit save (id minting), edit = write-through.
- Shipped at commit `b9bb4b9`; vitest 132/132, cargo 115/115, svelte-check clean. Phase B complete (10 of 13 tasks).
