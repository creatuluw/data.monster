---
type: Artifact
title: Design component reference (docs/design/components/)
description: What is it?
tags: [design-system, documentation, html, components]
timestamp: "2026-09-22T13:18:37.993Z"
---

# Design component reference (docs/design/components/)

## What is it?

A set of 39 standalone, self-contained HTML reference documents in `docs/design/components/` — one per UI component (Accordion, Badge, Button, ButtonGroup, Card, ColorPicker, DatePicker, Drawer, Modal, Searchahead, Select, Table, Tabs, Tag, Toast, Tooltip, etc.). Each file documents that component's variants, states, and behavior with live rendered examples, in its own inline-CSS theme (Source Serif 4 display / Manrope body / Geist Mono), loadable directly in a browser with no build step.

## What it documents

- [design-system-app-css-tokens-ui-showcase](../entities/design-system-app-css-tokens-ui-showcase.md) — the app's actual styling layer and component library these references describe
- [design-system-reference-doc-docs-design](./design-system-reference-doc-docs-design.md) — the sibling single-file design-system doc (`docs/design-system-data-monster.html`); this set is the per-component expansion of that idea
- [central-chart-component-design](./central-chart-component-design.md) — sits alongside it in `docs/design/`

## Details

- **Location**: `docs/design/components/*.html` (39 files)
- **Format**: static HTML, self-contained (inline CSS + Google Fonts link), one component per file
- Distinct from the `/ui` showcase route (live in-app components) — these are offline reference docs

## Lifecycle

- First added: committed 2026-09-22 (dd44774, "design-system docs") from a design-docs session
- Documented in wiki: 2026-09-22 — first wiki coverage; previously invisible to the knowledge graph

## Source

- `docs/design/components/` — the artifact itself
