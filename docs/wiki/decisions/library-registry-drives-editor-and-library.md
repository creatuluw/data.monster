---
type: Decision
title: Library registry drives editor + /library in one shot (supersedes display-only v1)
description: Context
tags: [library, central-charts, registry, extensions]
status: accepted
supersedes: "["library-q2-registry-display-only"]"
timestamp: "2026-09-17T07:17:26.416Z"
---

# Library registry drives editor + /library in one shot (supersedes display-only v1)

## Context

The /library route (interviewed 2026-09-17) becomes the central component library: every chart component usable in /pages is shown here, and component devs add new components as self-contained extension packages. Earlier same-day decisions said registry v1 would be display-only (editor wiring deferred) with a master-detail layout.

## Choice

Full scope in one go (user chose "C" then "one go"): the library registry feeds the SAME central chart registry the /pages runtime consumes — registering a component makes it appear in /library AND in the page editor's add-block picker AND gets a schema-driven config panel, by construction. Layout: card grid on /library + detail route /library/[id] with tabs Preview (default) | Schema | Code | Docs. Demos render the REAL renderer (src/lib/components/charts/renderers/*) fed bundled dummy rows — never a clone. Preview bypasses DuckDB entirely.

## Alternatives

- Display-only registry, wire the editor later — rejected by user (wanted full wiring now).
- Master-detail layout — superseded by cards + detail route (user answer to the layout question).
- Demo-only standalone chart — rejected: a clone drifts from the real component.

## Consequences

- src/lib/library/ = extension surface: types.ts (LibraryEntry), registry.ts (registerLibraryComponent → also calls registerChartType), components/<type>/ packages (def.ts + demo.ts + docs.md + index.ts with ?raw source for the Code tab).
- src/lib/charts/registry-setup.svelte.ts now only imports packages and registers them — chart definitions moved into packages (bar-chart, heatmap).
- Editor add-block buttons iterate getLibraryComponents(); new blocks get role-aware defaults (fills each role to its min).
- Adding a chart anywhere in the app = drop a folder under src/lib/library/components/ + register it in registry-setup.
- Supersedes [[library-q2-registry-display-only]] and refines [[library-q3-master-detail-layout]].
