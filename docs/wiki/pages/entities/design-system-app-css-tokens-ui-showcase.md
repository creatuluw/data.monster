---
type: Entity
title: Design system (app.css tokens + /ui showcase)
description: "The app-wide styling layer: design tokens in `src/app.css`, the `src/lib/components/` + `src/lib/components/ds/` component libraries, the `/ui` showcase page, a"
tags: [design-system, tokens, frontend, typography]
timestamp: "2026-09-12T16:56:53.776Z"
---

# Design system (app.css tokens + /ui showcase)

The app-wide styling layer: design tokens in `src/app.css`, the `src/lib/components/` + `src/lib/components/ds/` component libraries, the `/ui` showcase page, and the `.impeccable.md` design contract. All UI work must go through it.

## Details

- **Location**: `src/app.css` (tokens), `src/lib/components/` + `src/lib/components/ds/` (components), `src/routes/ui/+page.svelte` (showcase), `.impeccable.md` (contract, persisted by the impeccable skill)
- **Key tokens** (light theme, annual-report clarity):
  - Brand accent: ledger green `oklch(0.44 0.1 158)` — single accent; neutrals tinted toward hue ~160
  - `--color-copper-*` re-valued to **gold** — rare micro-accent only (growth highlights); name kept, value changed
  - `sage-*` ramp re-harmonized to brand green; status hues: danger ≈ hue 25, warning ≈ 85, success = brand
- **Typography**: Inter for all UI text (display + body, tabular figures so numbers align), Geist Mono reserved for data detail (SQL, chart tick/data labels). Spectral, Public Sans and Lekton were removed from the `src/app.html` font stack (2026-09-14)
- **Chart palettes**: category palettes lead with brand green; blue is allowed as a categorical data hue, never as brand
- **Anti-pattern**: 3px border-left blockquote stripe is a banned AI-tell — use full hairline border + sunken background instead

## Key rules / properties

- Token **names are stable** (`sage-*`, `copper-*`, `sand-*`) — restyling re-values tokens instead of renaming, so every consumer updates automatically
- Hardcoded oklch hues in routes/charts are swept by exact-value mapping when tokens change (~40 files in the 2026-09-12 pass); `ColorPalette.svelte` duplicates token literals for copy-to-clipboard and must be updated too

## Relationships

- [Professional-finance redesign decision](../../decisions/professional-finance-redesign-ledger.md) — the decision that defined this system
- [Typography: Inter for UI, Geist Mono for data decision](../../decisions/typography-inter-mono-for-data.md) — the font pairing now baked into the token layer

## Lifecycle

- 2026-09-12: copper/editorial theme replaced by ledger-green/Spectral/Public Sans professional-finance theme via token re-valuation (rollback point: `git reset --hard f728c70`)
- 2026-09-14: typography re-paired to Inter (UI) + Geist Mono (data); Picasso canvases switched Lekton → Geist Mono; Labs page reorganized to one card per chart type
