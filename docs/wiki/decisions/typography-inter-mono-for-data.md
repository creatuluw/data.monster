---
type: Decision
title: "Typography: Inter for all UI, Geist Mono reserved for data detail"
description: Context
tags: [design, typography, tokens, inter, fonts]
status: superseded
timestamp: "2026-09-14T07:46:53.998Z"
---

# Typography: Inter for all UI, Geist Mono reserved for data detail

## Context
The 2026-09-12 [[professional-finance-redesign-ledger]] chose Spectral (display) + Public Sans (body) + Geist Mono/Lekton for data. User then asked to change **all fonts to Inter**, with low-level data detail (table cells, chart labels) allowed to stay mono. Ledger-green theme/tokens unchanged.

## Choice
- **Inter** for everything UI: `--font-display` and `--font-body` both → `'Inter', sans-serif` in `src/app.css` (token names unchanged, so all consumers restyle automatically — same trick as the ledger redesign).
- **One mono**: Geist Mono only. Lekton dropped; chart labels in `src/lib/charts/Picasso*Canvas.svelte` switched Lekton → Geist Mono. `src/app.html` font link loads only Inter + Geist Mono.
- **Data detail stays mono**: `Heatmap.svelte` (svelteplot) tick/in-cell labels forced to `--font-mono`; `.data-table th/td` already mono — unchanged.
- Typography showcase (`/ui` + both `Typography.svelte` copies) updated; fixed stale "Manrope" body-font card.

## Alternatives
Keep the Spectral/Public Sans pairing — rejected by user request. Keep Lekton as a second mono — dropped in favor of one mono everywhere.

## Consequences
Type hierarchy now relies on Inter weight/size, not serif/grotesk contrast. The mono-for-data convention (see the rule below) governs new components: any new data-dense surface (table cell, axis tick, in-cell label, code) uses `--font-mono`; everything else Inter.
