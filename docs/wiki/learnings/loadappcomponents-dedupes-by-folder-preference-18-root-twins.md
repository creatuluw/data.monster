---
type: Learning
title: loadAppComponents dedupes by folder preference — 18 root twins are dead code
description: "Discovered 2026-09-23 while removing the "Also at:" dup note from /components detail pages (commit `e8e329a`)."
tags: [frontend, components, dedup, dead-code]
timestamp: "2026-09-23T07:50:46.203Z"
---

# loadAppComponents dedupes by folder preference — 18 root twins are dead code

Discovered 2026-09-23 while removing the "Also at:" dup note from /components detail pages (commit `e8e329a`).

- `loadAppComponents()` (`src/lib/app-components.ts`) now dedupes by name with a folder preference order: **`ds` > `charts/controls` > `charts` > root**. The catalog surfaces 58 active components instead of 76 file entries; dead variants can't be navigated to, and legacy bare-name URLs (`/components/Accordion`) still resolve to the active variant. The catalog test now asserts name-uniqueness.
- Before deduping, importers were verified: all 18 root twins (`Accordion`, `Modal`, `Toast`, …) plus root `Toggle` (collides with `charts/controls/Toggle`) are imported by **nothing** — the `ds/` and `charts/controls/` variants are what the app actually uses. Those root files are confirmed dead code, safe to delete anytime; the catalog won't change.

Extends [[ds-twins-win-slug-collisions]] (slug resolution already preferred the `ds/` twin); this is the loader-side dedup. Entity: [[components-catalog-components-app-components-ts]].
