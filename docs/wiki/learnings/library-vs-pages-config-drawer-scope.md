---
type: Learning
title: The /library vs /pages config-drawer difference is scope, not components
description: Symptom
tags: [library, pages, blockinspector, demo-drawer, heatmap, mergeoptions]
timestamp: "2026-09-17T08:34:47.889Z"
---

# The /library vs /pages config-drawer difference is scope, not components

## Symptom

The `/library/bar` demo drawer looked like it showed only a few "dummy" options, while the drawer on `/pages/smoke-test` showed many. Read as two different component sets.

## What's actually going on

They are the **same components** — one registry, same renderers; `/pages` renders exactly what `/library` previews. The difference is drawer *scope*:

- `/pages` → `BlockInspector` edits the **whole block spec**: title, source table, dimensions, measures, roles, options, annotations. Full data binding exists only here.
- `/library` → the demo drawer edits **demo options** only, fed by demo data.

## Fix shipped

The library demo drawer now labels this honestly: a read-only **"Demo data"** section (dimensions × measures · row count) plus **"Demo options"** with the same schema-driven fields, live-updating the chart. So the drawer no longer reads as dummy — it's explicit about what's demo-scope vs full binding.

## Re-verified on heatmap (2026-09-17): the options merge is ALSO identical

User report: "the heatmap I can add in /pages is not the same as `/library/heatmap`." End-to-end trace confirmed both surfaces render the **same** `HeatmapRenderer.svelte` from the registered `heatmap` package *and* build chart options with the exact same `mergeOptions` (schema defaults + def defaults + block options — identical to the library demo merge). So a visual difference between the two surfaces can **never** come from the component or the options merge — the only remaining variable is the **data/props layer**: the query + pivot output fed into the renderer.

## Rule of thumb

When `/pages` and `/library` previews look divergent, check in order: (1) drawer scope, (2) the fed data/props (query/pivot output) — never assume different components or different option merging; both are shared machinery. See [[library-registry-system]] and decision `library-packages-carry-blockkind` (blockKind field).
