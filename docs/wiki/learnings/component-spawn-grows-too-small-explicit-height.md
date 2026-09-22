---
type: Learning
title: Component spawn grows too-small explicit-height rows to 320px minimum
description: "Discovered 2026-09-17 while verifying the page-editor skeleton-clip bug (fixed in PR #9, 1 file +8)."
tags: [page-editor, pagegrid, skeleton, cdp]
timestamp: "2026-09-17T15:18:52.909Z"
---

# Component spawn grows too-small explicit-height rows to 320px minimum

Discovered 2026-09-17 while verifying the page-editor skeleton-clip bug (fixed in PR #9, 1 file +8).

## Symptom

Adding a component to a page-editor row with an explicit small height (the 180px default) rendered the [[needsSetup-gate]] skeleton **clipped** — its buttons unreachable, `clipPx > 0` on the row.

## Behavior (as shipped)

- A component spawning into an **explicit-height row below 320px** grows that row to a **320px minimum at spawn time**.
- Auto-height rows and taller rows are untouched.
- The bump happens **only at spawn** — a later drag-resize below 320px still works. That invariant matters: enforcing the minimum continuously would break intentional small rows after spawn.

## Verification

CDP live-run: 180px row + added bar chart → row at 320px, skeleton fully visible, `clipPx: 0`, buttons reachable (109/109 vitest, svelte-check clean).

## Relationships

- [[../pages/entities/pagegrid-component]] — the canvas renderer whose row sizing owns the bump
- [[adding-a-component-never-auto-opens-the-config]] — sibling spawn-time rule (skeleton is the start state)
- [[normalizepagedoc-field-whitelist]] — row-height persistence context
