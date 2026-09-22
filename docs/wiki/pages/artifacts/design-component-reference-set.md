---
type: Artifact
title: Design-component reference set (docs/design/components/)
description: 40 standalone per-component design-reference HTML pages (Button, Modal, Table, Searchahead, …) — static explorations in their own token set, not app components.
tags: [design-system, components, html, reference, static-docs]
timestamp: "2026-09-22T13:14:51.753Z"
---

# Design-component reference set (docs/design/components/)

## What is it?

40 standalone, self-contained HTML pages under `docs/design/components/` — one per UI component (Button, ButtonGroup, Input, Select, Modal, Drawer, Tabs, Table, Pagination, Toast, Tooltip, Tag, TagInput, Searchahead, DatePicker, ColorPicker, ConditionBuilder, NodeTree, EllipsisMenu, Spinner, Card, List, Breadcrumb, Accordion, Badge, Label, Toggle, Header, FooterDock, ErrorPage, plus domain-drawer/form mockups like InboxActionDrawer, GenerateResultDrawer, TestResultDrawer, RecordingFieldsDrawer, StudentForm, GroupForm). `ComponentRef.html` is the per-component reference layout in the same format.

## What it documents

- [Design system (app.css tokens + /ui showcase)](../entities/design-system-app-css-tokens-ui-showcase.md) — the live token/component layer these static references sit apart from
- [Central-charts design doc](central-chart-component-design.md) — the sibling design document in the same `docs/design/` folder

## Details

- **Format**: single-file HTML per component — inline CSS token block, demo sections with labeled variants; opens directly in a browser, no build
- **Location**: `docs/design/components/*.html` (committed 2026-09-22 in `dd44774` as session artifacts)
- **Visual language**: each file carries its own token set — Source Serif 4 (display) / Manrope (body) / Geist Mono, warm oklch accent around hue 41 — which is **not** the live app theme (Inter + Geist Mono, ledger green). Treat these as design references/explorations, not as the app's component specs.

## Source

- Committed as part of the 2026-09-22 session-artifacts commit (`dd44774`: "commit remaining session artifacts, design-system docs, research reports + wiki sync")
