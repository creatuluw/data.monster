---
type: Artifact
title: Design-system reference doc (docs/design-system-data-monster.html)
description: "The standalone design-system documentation deliverable: a single self-contained HTML file rendering the app's current tokens, typography, color ramps, and compo"
tags: [design-system, docs, tokens, frontend]
timestamp: "2026-09-18T10:18:58.868Z"
---

# Design-system reference doc (docs/design-system-data-monster.html)

The standalone design-system documentation deliverable: a single self-contained HTML file rendering the app's current tokens, typography, color ramps, and component patterns. Open it in a browser for the team-facing answer to "what does our design system currently look like."

## What it documents

- The [app design system](../entities/design-system-app-css-tokens-ui-showcase.md) — the same tokens as `src/app.css`, restated as a readable spec/reference
- Contents after the 2026-09-17 re-theme: Inter (display + body) / Geist Mono (data detail) two-font system; ledger-green accent `oklch(0.44 0.1 158)` with hue-160–165 neutrals and brand-hue status colors (success 158, warning 85, danger 25); sage / gold(copper) / sand ramp strips as `:root` tokens; bordered-card pattern (hairline border, `radius-md`, `space-6` padding, hover = border-strong + shadow, grid gap — not per-card margins); Geist Mono `.tag` badges; a Data Table section (mono cells, uppercase mono headers, green row hover); `--max-width` 72rem; `DM-*` branding, v3.0
- 2026-09-18 scale pass **reverted the same day** at user request — the doc renders at the old file’s original size: 100% root font-size, original px literals (9px/10px labels, 80px swatches, 520px modal, 9999px radius-full), and the old layout tokens restored (`--max-width: 72rem`; gutter was already identical). The 80% root + ×0.8 px-sweep technique itself stands, documented in [the scaling learning](../../learnings/scale-standalone-html-docs-via-root-font-size-px.md).

## Details

- **Location**: `docs/design-system-data-monster.html`
- **Format**: single self-contained HTML — inline CSS, Google Fonts link (Inter 400/600/700)
- **Generated from**: the 2026-09-17 conversion pass — 55 exact-value token replacements plus structural fixes (fonts link, card pattern, input/tag fonts, ramps, data-table section), all sourced from `src/app.css`; replaced the dead orange "SYNAPSE" theme (Source Serif 4 + Manrope, 72rem cap). Spacing/radius/shadow/motion tokens were already identical to the app and untouched. The prose-chat/blockquote banned-pattern section was deliberately skipped.

## Maintenance gotcha

Hand-maintained, **not generated** from `app.css` — it drifts silently when the app re-types or re-tokens (it sat on a dead theme for weeks). After any token/typography change, re-value this doc too — same family as [the tours' embedded-fonts gotcha](../../learnings/tour-html-captures-embed-google-fonts-import.md).

## Per-component set (docs/design/components/)

A sibling deliverable committed 2026-09-22 (`dd44774`): **39 standalone per-component HTML docs** in `docs/design/components/` — Button, Input, Select, Modal, Drawer, Tabs, Table, Tag, Tooltip, Toast, Pagination, Searchahead, etc., plus app-shaped composites (ConditionBuilder, NodeTree, InboxActionDrawer, StudentForm, RecordingFieldsDrawer, GenerateResultDrawer). Each file is fully self-contained (inline CSS, own `:root` tokens, Google Fonts link) and shows the component's variants in bordered demo blocks.

**Theme caveat — still on the dead SYNAPSE palette**: these files render the old orange accent (`oklch(0.69 0.16 41)`, hue 41) with Source Serif 4 + Manrope — NOT the current ledger-green/Inter system. Same drift family as the maintenance gotcha above; re-theme before citing them as the current component reference.

## Source

- `docs/design-system-data-monster.html` — the single-file deliverable
- `docs/design/components/*.html` — the 39-file per-component set
- `src/app.css` — source of truth for every token value
