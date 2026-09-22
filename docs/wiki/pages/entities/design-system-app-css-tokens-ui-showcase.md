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
- **Typography**: two-font system — Inter (`--font-display` + `--font-body`, all UI, weights 400/600/700 from Google Fonts), Geist Mono (`--font-mono`, data detail + `.tag` badges) (settled 2026-09-17)
- **Chart palettes**: category palettes lead with brand green; blue is allowed as a categorical data hue, never as brand
- **Layout**: `--max-width` = 120rem (1920px); one `.app-column` wrapper in `src/routes/+layout.svelte` (header + breadcrumb + content) owns the cap, centering, and full-viewport-height side borders — no full-bleed opt-out (2026-09-16)
- **Anti-pattern**: 3px border-left blockquote stripe is a banned AI-tell — use full hairline border + sunken background instead

## Key rules / properties

- Token **names are stable** (`sage-*`, `copper-*`, `sand-*`) — restyling re-values tokens instead of renaming, so every consumer updates automatically
- Hardcoded oklch hues in routes/charts are swept by exact-value mapping when tokens change (~40 files in the 2026-09-12 pass); `ColorPalette.svelte` duplicates token literals for copy-to-clipboard and must be updated too

## Relationships

- [Professional-finance redesign decision](../../decisions/professional-finance-redesign-ledger.md) — the decision that defined this system
- [Typography: Poppins / Inter / Geist Mono decision](../../decisions/typography-poppins-headings-figtree-dropped.md) — the font pairing now baked into the token layer (display face churned Squada One → Calluna → Figtree → Poppins on 2026-09-16)
- [Design-system reference doc](../artifacts/design-system-reference-doc-docs-design.md) — the `docs/` HTML deliverable restating these tokens (re-themed 2026-09-17)

## Lifecycle

- 2026-09-12: copper/editorial theme replaced by ledger-green/Spectral/Public Sans professional-finance theme via token re-valuation (rollback point: `git reset --hard f728c70`)
- 2026-09-14: typography re-paired to Inter (UI) + Geist Mono (data); Picasso canvases switched Lekton → Geist Mono; Labs page reorganized to one card per chart type
- 2026-09-16: typography re-paired again — Squada One (display) / Libre Baskerville (body) / Geist Mono (data + badges); Inter dropped
- 2026-09-15: app-wide 1440px content cap via shared layout lever; `--max-width` 72rem → 90rem (PR #4)
- 2026-09-16: cap raised to 120rem (1920px) and the full-bleed class removed — every page, no exemptions; same day the cap moved onto the single `.app-column` wrapper (viewport-high, full-height side borders) that now contains header + breadcrumb + content
- 2026-09-16 (later): typography re-paired again — Calluna (display) / Inter (body) / Geist Mono (data + badges); tours recaptured
- 2026-09-16 (latest): display face swapped twice more same day — Calluna → Figtree (variable 300–900) → **Poppins** (400/600/700); tours recaptured after each swap
- 2026-09-17: typography settled — Inter everywhere (display + body), Geist Mono for data detail (see [settles decision](../../decisions/typography-settles-inter-everywhere-geist-mono.md)); same day the [design-system reference doc](../artifacts/design-system-reference-doc-docs-design.md) was re-themed from the dead SYNAPSE theme to these tokens
