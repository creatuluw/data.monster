---
type: Decision
title: "Page editor block config: focused two-panel mode via cog icon (no selection ring)"
description: Context
tags: [central-charts, page-editor, ux]
status: accepted
timestamp: "2026-09-15T11:54:58.537Z"
---

# Page editor block config: focused two-panel mode via cog icon (no selection ring)

## Context

On `/page/<slug>` (the dual-mode editor, see [[q16-locked-pages-lists-amp-creates-page-lt-slug-gt-hosts-the]]), clicking a block selected it with a ring and the full canvas stayed visible while config UI appeared alongside. This conflicted with chart interaction (selection/cross-filter click-through) and gave config a cramped surface.

## Choice

**Focused config mode.** Clicking a chart is plain chart interaction again — no border/ring. Every block carries the **⚡ Bolt config icon** top-right — same icon, size, and hover as /labs (initially a ⚙ cog; swapped in PR #4 for cross-surface consistency); clicking it enters a dedicated config UI:

- The clicked chart alone renders on the **left**
- A **50vw** drawer (overlay mode) on the **right** hosts the full [[chartconfigdrawer-component]]-shelled BlockInspector (title/subtitle, roles + master-item pickers, schema-driven options, tooltip template, annotations, heightVh, span)
- All other blocks are hidden; **Close (X)** restores the full canvas
- Edits apply **live** to the left-side chart

## Alternatives considered

- Keep inline selection + side drawer over the full canvas — rejected: ring fights chart click-through, config surface is small
- Reuse the /labs bolt-drawer (30vw, hosted inside the chart) — kept for labs; the page editor needs the full BlockInspector, not per-chart config snippets

## Consequences

- [[chartconfigdrawer-component]] grew `width`/`overlay` props — labs keeps its 30vw bolt drawer unchanged
- BlockInspector gained an `onremove` callback so deleting a block closes the drawer
- Fixed along the way: the inspector Title field was writing block-level `title` that charts never render — now writes `chart.title`/`chart.subtitle` (+ new Subtitle field)

## Source

- `src/routes/page/[slug]/` (PageGrid focused mode), verified 8/8 via CDP in the running app, PR #4
