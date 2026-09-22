---
type: Decision
title: Chart blocks start empty — data renders only when role requirements are met (needsSetup gate)
description: Context
tags: [central-charts, page-editor, library-registry, ux]
status: accepted
timestamp: "2026-09-17T13:14:59.135Z"
---

# Chart blocks start empty — data renders only when role requirements are met (needsSetup gate)

## Context

Previously, dropping a new chart component onto a page-editor row auto-filled dimensions/measures with fallback picks (e.g. `count(*)`) and immediately queried — producing guessed charts or malformed SQL, with no guided path to a correct config.

User direction (2026-09-17): fresh chart blocks should show **no data by default** — a skeleton silhouette with buttons to add dimensions/measures — and only render data once the chart type's requirements are met.

## Decision

1. **Fresh blocks start empty**: `+ Component` adds a chart with empty dimensions/measures (source table preselected). No fallback auto-fill, no `count(*)`.
2. **`needsSetup(chart)`** — a pure function in the component registry (`registry.ts`) returns true while any role (dimension/measure) is below its declared min. It is the single source of truth for "not ready yet".
3. **The gate covers both query and render**: page-runtime marks the block `unconfigured` and **never issues a query** while `needsSetup` is true; `PageGrid.svelte` renders a setup `ChartCard` (`'setup'` status) hosting [[skeletonsetup-component]] (in-chart pickers) when the data context is loaded, else a plain dashed silhouette with **Add dimension / Add measure** buttons that open the inspector drawer. *(Amended 2026-09-17, PRs #10 + #11: pickers moved in-chart; the pulsing silhouette now lives only inside SkeletonSetup — PageGrid's old wrapper silhouette rendered a duplicate and was removed.)*
4. The moment every role minimum is met, the query runs and real data replaces the skeleton.

## Alternatives considered

- Keep auto-fill defaults — rejected: guessed charts hide misconfiguration and can emit malformed SQL.
- Gate render only, still query — rejected: wasted queries and possible SQL errors for unconfigured blocks.

## Consequences

- Do NOT re-introduce fallback/auto-fill defaults for new blocks — the empty template + skeleton is deliberate.
- Any new block status must be added to `ChartCard`'s status union (unknown statuses fall through to children).
- Registry component definitions own their role minimums; `needsSetup` reads them, so new component packages get the skeleton behavior for free.

Related: [[library-registry-drives-editor-and-library]], [[library-packages-carry-blockkind]].
