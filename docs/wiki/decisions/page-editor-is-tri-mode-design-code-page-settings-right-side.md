---
type: Decision
title: Page editor is tri-mode (Design / Code / Page settings); right sidebar removed — canvas shows only visualizations & data
description: Context
tags: [central-charts, page-editor, ux]
status: accepted
timestamp: "2026-09-15T13:10:13.877Z"
---

# Page editor is tri-mode (Design / Code / Page settings); right sidebar removed — canvas shows only visualizations & data

## Context

On `/page/<slug>` the Design mode kept a persistent right sidebar (an inspector aside with a "click the ⚡ to configure" hint plus row management), shrinking the canvas and mixing page-level chrome with chart content. User direction (2026-09-15): **pages should only show visualizations and data-related content** — non-visual configuration doesn't belong on the canvas.

This amends [[q16-locked-pages-lists-amp-creates-page-lt-slug-gt-hosts-the]] ("dual-mode editor"): the editor is now **tri-mode**.

## Choice

**Three tabs: Design / Code / Page; no right sidebar.**

- **Design** — full-width canvas: only [PageGrid](../pages/entities/pagegrid-component.md) blocks (charts, tables, text) and the add-block buttons. The inspector `aside` is deleted.
- **Code** — unchanged (spec JSON editing).
- **Page** (new, FileCog icon) — the home for *non-visual* page settings: page title (editable), slug (read-only), and row list with remove buttons.

Block config is unchanged and stays where [[page-editor-block-config-focused-two-panel-mode-via-cog-icon]] put it: ⚡ Bolt on a block → focused two-panel config via [[chartconfigdrawer-component]]. Block-level config is per-block (visual, contextual); *page-level* config is global (non-visual) → hence the tab split.

## Alternatives considered

- Keep the sidebar for row management — rejected: user explicitly wants the canvas data-only; row CRUD is settings, not visualization.
- Put page settings in a drawer/gear on the canvas — rejected: another overlay over the charts; a peer tab is discoverable and consistent.

## Consequences

- `mode` state is `'design' | 'code' | 'page'`; Page tab edits `doc.title`/rows directly.
- Future page-level spec fields (filters, theme, shared color scale from [[q12-locked-page-level-consistent-colors-via-shared-scale-opt]]) have an obvious home: the Page tab.
- Design canvas is now full-width with no side rail at rest — block-config entry remains the ⚡ Bolt.

## Source

- `src/routes/page/[slug]/+page.svelte` — tri-mode tabs, Page settings panel, sidebar removal
