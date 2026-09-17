---
type: Decision
title: Library registry v1 lives as a TypeScript module under src/lib/library/ with self-contained component folders
description: Context
tags: [library, registry, components, spec-interview, frontend]
status: accepted
timestamp: "2026-09-17T07:10:29.142Z"
---

# Library registry v1 lives as a TypeScript module under src/lib/library/ with self-contained component folders

## Context

Exchange in the `/library` spec interview (parent: [[library-central-component-library]]; prior locks: Q2 display-only registry [[library-q2-registry-display-only]], Q3 master-detail layout [[library-q3-master-detail-layout]], Q4 dedicated tabbed views [[library-q4-dedicated-tabbed-views]], self-contained extension-style components [[library-extension-style-components]], demos render real components with dummy query-shaped data [[library-demos-reuse-real-components]]). The last open question before writing the spec + todo was where the registry lives. The user answered **"a"** (2026-09-17), and the interview moved on to the next question (whether the bar-chart entry reuses the central-charts BarChart — open).

## The choice

**A — the registry is a TypeScript registry module under `src/lib/library/`**: types + a `register()` API + the entries, with each component living as a self-contained folder under `src/lib/library/components/<name>/` (definition, logic, and data in one place, matching [[library-extension-style-components]]).

## Rationale

- A TS module keeps the registry in code — typed, greppable, no DB/metadata indirection for v1 (consistent with Q2: display-only, no editor wiring yet).
- Self-contained component folders match the extension-style contract: one entry = demo + production.

## Alternatives considered

- **B — user-specified structure** — declined by answering A; no custom structure proposed.

## Consequences

- `src/lib/library/` becomes the canonical home for the library registry and component entries; the existing route stub `src/routes/library/+page.svelte` consumes it.
- First entry (barchart) wiring is the next open question: reuse [[barchart-component]] from central-charts with dummy query-shaped demo data, vs a fresh demo-only chart.
- Then: write the spec + todo and build.
