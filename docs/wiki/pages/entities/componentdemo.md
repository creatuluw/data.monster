---
type: Entity
title: ComponentDemo
description: What is it?
tags: [components, design-system, frontend, demo]
timestamp: "2026-09-23T07:26:00.391Z"
---

# ComponentDemo

## What is it?

The demo registry behind the [components-page-components](./components-page-components.md) detail pages: one Svelte component (`name` prop) that resolves and renders a **live, standalone demo** for every component that can run without app data or Tauri. Shipped 2026-09-23 (commit `d138bf1`) when the Source sections were removed and all component pages went Visual-only (see the decision in this wiki).

## Why it matters

It is the single place a new demo is added — drop a component into the import block and add one `{name}` branch. The detail page stays generic; no per-page wiring. It keeps the honest-note fallback for app-wired components, so a page never renders a fake visual.

## Key rules / properties

- **Self-showcasing `ds/`**: all 18 `ds/` components render via the `DS_SHOWCASE` map — same behavior as `/ui`, no props.
- **Hand-written dummy-data demos**: `charts/controls/` kit (Btn, Field+TextInput/Select/NumberInput in a real form layout, Section, Toggle, RemoveBtn, DangerZone with confirm flow), root kit (Tag ×3 variants, Tabs with snippet labels, Tooltip, Pagination, TagInput, TableList with dummy tables), and a real `charts/BarChart` rendering 6 bars from sample rows.
- **Snippet demos are inline markup** — snippets are template constructs and cannot be passed through a name-keyed registry dynamically; Tabs' snippet labels are authored directly in the branch markup.
- **Accessor props, not strings** — BarChart takes accessor functions over rows, so the demo passes `(row) => …` accessors, not column names.
- **Honest note otherwise** — genuinely app-wired components (TableDrawer, ItemEditor, QueryEditor, renderers, …) show a one-line note instead of a fake visual. Coverage: 38 of 76 components with live visuals. 2026-09-23: five root-only demoable components added — Badge, Breadcrumb, LabsPlaceholder, PreviewPane, SearchAhead (commit `9799535`).

## Relationships

- [components-page-components](./components-page-components.md) — the detail route that renders `<ComponentDemo name={…} />` in its Visual section
- [design-system-app-css-tokens-ui-showcase](./design-system-app-css-tokens-ui-showcase.md) — the `ds/` half reuses the same self-showcase components as `/ui`

## Source

- `src/lib/components/ComponentDemo.svelte` — the registry (DS_SHOWCASE map + per-name branches, ~240 lines)
- `src/routes/components/[...slug]/+page.svelte` — sole consumer
