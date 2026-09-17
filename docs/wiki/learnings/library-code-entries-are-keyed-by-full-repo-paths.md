---
type: Learning
title: Library code entries are keyed by full repo paths
description: What
tags: [library, registry, frontend, conventions]
timestamp: "2026-09-17T08:41:47.662Z"
---

# Library code entries are keyed by full repo paths

## What

The code-source map exported by each library component package is keyed by the **full repo path** of the file, not a bare filename — e.g. `src/lib/library/components/bar-chart/def.ts`, `src/lib/components/charts/renderers/BarChartRenderer.svelte`. All registered component packages follow this.

## Why it exists

The `/library/[id]` detail page's **Code tab** renders each entry as a speed-highlight/core block (see [[speed-highlight-over-prism]]) with the path as its header, plus a copy button per block — so a reader always knows exactly where extension code lives in the repo.

## Gotcha

New component extensions must key their code entries by full repo path or their Code tab silently loses the location info. Extension packages live under `src/lib/library/components/<type>/`; renderers under `src/lib/components/charts/renderers/`.

## Relationships

- Shape defined by [[library-registry-system]] (one folder per component: `def.ts`, `demo.ts`, `docs.md`, `index.ts`)

## Source

- `src/lib/library/components/<type>/index.ts` — code entries keyed by full repo path
