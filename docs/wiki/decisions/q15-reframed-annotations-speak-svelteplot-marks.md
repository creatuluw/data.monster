---
type: Decision
title: "Q15 reframed: annotations should speak svelteplot's own mark vocabulary (whitelisted per chart type)"
description: Context
tags: [charts, spec-design, annotations, q15, svelteplot]
status: proposed
timestamp: "2026-09-15T09:01:40.281Z"
---

# Q15 reframed: annotations should speak svelteplot's own mark vocabulary (whitelisted per chart type)

## Context

Reframe of [[q15-reference-annotations-declarative-list]] (Q15: annotations are a declarative list, engine-evaluated `at`, per-type draw hook — that mechanism survives). In the follow-up interview the user pointed at https://svelteplot.dev/features/marks: since [[svelteplot-sole-chart-engine]] (svelteplot renders everything), we don't need to invent Evidence.dev's `reference_line/area/point` vocabulary — svelteplot already names ~20 marks (area, arrow, bar, box, brush, cell, dot, frame, grid, line, link, pointer, raster, rect, regression, rule, text, tick, trail, vector, waffle, …). Q15 was re-asked as "how far does the annotation spec go in svelteplot's vocabulary?"

## Options on the table

- **A — Whitelisted annotation marks** (assistant's lean): spec gets `annotations: [...]` where each entry is a svelteplot mark + values, e.g. `{ "mark": "ruleY", "at": 120 | { "expr": "avg(hours) * 1.1" }, "label": "Target", "color": null }`. `at` accepts a literal or a **measure expression** (engine evaluates against the chart's data). Each chart type's registry entry declares **which annotation marks it hosts** (cartesian: rule/dot/text; heatmap: none). No-code surface: an "Add annotation" menu listing the type's supported marks + fields.
- **B — Fixed trio, svelteplot underneath**: only line/area/point in the spec, mapped internally to ruleY/ruleX, rect/area, dot. Smaller vocabulary but re-invents types svelteplot already names; "add a tick/text label" later means new spec types again. (Closest to the original Q15 lock.)
- **C — A + arbitrary layered marks**: any svelteplot mark with data refs, extra channels, own dataset (second series layered in). Max power — but that's a second chart-engine DSL inside the spec; the `plot_options` escape hatch (Q1-C) already serves power users.

## Current lean

**A** — svelteplot's own vocabulary, bounded by a per-type whitelist, expressions allowed for values. B re-invents what svelteplot names; C is what the escape hatch is for.

## Status

Proposed — awaiting the user's pick. If **A** is locked, this **supersedes** the v1 vocabulary of the original Q15 lock (its `type: "line"` entry becomes `mark: "ruleY"`, etc.); if **B**, the original lock stands. The `annotations` list + engine-evaluated `at` + per-type registry hook mechanism is common to all options.

## Related

- Builds on [[svelteplot-sole-chart-engine]] — one engine, so its mark names are the vocabulary.
- Extends [[measures-dimensions-are-duckdb-expressions]] — `at` accepts measure expressions.
- Same serialization-first posture as [[two-surface-report-page-format]] — annotations must stay spec-serializable for code/UI parity.
