---
type: Decision
title: Component detail pages are Visual-only — Source sections and highlight machinery removed
description: Context
tags: [components, design-system, frontend, ui]
status: accepted
timestamp: "2026-09-23T07:25:46.902Z"
---

# Component detail pages are Visual-only — Source sections and highlight machinery removed

## Context

`/components/<slug>` detail pages shipped with two numbered sections: **01 Visual** (live ds/ self-showcase or an honest note) and **02 Source** (speed-highlighted code + Copy). Only the 18 `ds/` components had live visuals; the other 58 showed source only.

## Decision (user-directed, 2026-09-23, commit `d138bf1`)

All component pages are **Visual-only** — the Source sections and the highlight machinery are removed. Every page shows its component visually as designed, like the Accordion page did:

- New `ComponentDemo.svelte` registry renders a live demo per showcaseable component (33 of 76): all 18 `ds/`, the 9 `charts/controls/` form kit, 6 root-kit components (Tag, Tabs, Tooltip, Pagination, TagInput, TableList), and a real `charts/BarChart` fed 6 dummy rows.
- Genuinely app-wired components (TableDrawer, ItemEditor, QueryEditor, renderers, …) keep a one-line honest note instead of a fake visual — no source fallback.

## Alternatives considered

- Keep Visual + Source stacked — rejected: user wants design-first pages; source is one editor-tab away.
- Build demo harnesses for all 76 up front — deferred: app-wired components each need a data harness; build on demand.

## Consequences

- `app-components.ts` still bundles raw sources for line counts; the `?raw` glob could shrink to a line-count-only pass later.
- 43 components remain note-only until someone writes their harnesses.
- Demos needing snippet children must be authored as inline markup in ComponentDemo (snippets are template constructs) — see the learning in this wiki.
