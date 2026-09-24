---
type: Artifact
title: Redesign coverage audit (reports/redesign-coverage-audit.md)
description: Redesign coverage audit (reports/redesign-coverage-audit.md)
tags: [audit, bits-ui, redesign, report]
timestamp: "2026-09-24T07:03:17.963Z"
---

# Redesign coverage audit (reports/redesign-coverage-audit.md)

Systematic sweep of all 134 `.svelte` files (2026-09-24, branch `feature/bits-ui-redesign`) checking the Path B bits-ui migration for coverage gaps: which interactive surfaces are still hand-rolled, which hand-rolling is deliberate, and where styling-debt (zinc/hex) remains.

**Status (2026-09-24, later same day): all 6 behavior gaps RESOLVED** — commits `fd7fac4`..`e80ae6c` on PR #21, each re-verified against the working tree. The report now carries resolution markers inline, a refreshed styling-debt table, and a reduced "Remaining work" section (styling convergence only). What remains: PageGrid (30 zinc/hex, missed by the original table), ItemEditor/RelationshipEditor (25 each), ExprEditor (31, deferred with its autocomplete until a caret-anchored pattern is chosen).

## What it documents

- [bits-ui-impeccable-redesign-analysis-report](./bits-ui-impeccable-redesign-analysis-report.md) — extends that analysis from "which components to migrate" to "did the migration actually cover everything".
- The Path B redesign in progress (PR #21, branch `feature/bits-ui-redesign`).

## Headline findings

- **6 real gaps found — all now fixed**: (1) hand-rolled "New page" modal in `pages/+page.svelte` → Dialog (`fd7fac4`); (2) nested hand-rolled confirm modal inside `TableDrawer` → Dialog (`8632555`); (3) duplicate hand-rolled tab bar in `connect/+page.svelte` → shared Tabs (`f13f103`); (4) same in `library/[id]` → shared Tabs (`f13f103`); (5) `TagInput` hand-rolled suggestion list + hardcoded hex → converged, token colors + listbox semantics (`0b74f7c`); (6) the sneakiest — `/components` catalog rendered the **pre-bits ds/ twins** → ds/ deleted, demos in `lib/demos/`, dead root twins removed (`b0c21ce`, `cdb0b4a`).
- **Deliberately kept, validated**: controls/Toggle, context menu, SQL keybindings, panel drag-resizers, native `<select>`s — all correct per project preferences, not skips.
- **Styling debt re-quantified after the fixes**: PageGrid 30 (missed in the original table), ItemEditor 25, RelationshipEditor 25, pages/+page 17→8, TagInput 6→0; ExprEditor 31 stays deferred with its autocomplete.
- **No behavioral migration work remains** — bits-ui owns every interactive surface the audit flagged; only styling convergence is left, riding along per touched component.

## Details

- **Location**: `reports/redesign-coverage-audit.md`
- **Format**: Markdown report; method = full inventory → mechanical scan (window/keydown/pointer listeners, aria roles, absolute positioning, `<select>`) → per-file evidence extraction → classification
- Generated from: static analysis of `src/` on branch `feature/bits-ui-redesign` (2026-09-24); updated same day after the fix commits (`fd7fac4`..`e80ae6c`) with re-scan verification (grep per finding + fresh zinc/hex count over the 91 non-demo `.svelte` files)

## Source

- `reports/redesign-coverage-audit.md` — the report itself
