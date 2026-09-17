---
type: Learning
title: The /library vs /pages config-drawer difference is scope, not components
description: Symptom
tags: [library, pages, blockinspector, demo-drawer]
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

## Rule of thumb

When `/pages` and `/library` previews look divergent, first check drawer scope, then assume different components (they aren't). See [[library-registry-system-src-lib-library-library-routes]] and decision `library-packages-carry-blockkind` (blockKind field).
