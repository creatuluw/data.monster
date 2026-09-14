---
type: Decision
title: "Professional-finance redesign: ledger green + Spectral/Public Sans"
description: Context
tags: [design, redesign, tokens, impeccable]
status: accepted
timestamp: "2026-09-12T16:55:07.801Z"
---

# Professional-finance redesign: ledger green + Spectral/Public Sans

## Context
The app's visual identity was warm editorial (copper/terracotta accent, sage/sand ramps, Source Serif 4 + Manrope). User wanted a "really professional business" feel: accurate, finance, trust, insight, growth.

## Choice
Full token-level redesign in `src/app.css`, driven by the impeccable skill (design context persisted in `.impeccable.md`):

- **Theme**: light, annual-report clarity
- **Accent**: ledger green `oklch(0.44 0.1 158)` — single brand accent; neutrals tinted toward hue 160; gold ramp (`--color-copper-*`) kept as rare micro-accent for growth highlights
- **Type**: Spectral (display) + Public Sans (body, institutional grotesk) + Geist Mono/Lekton unchanged; `font-variant-numeric: tabular-nums` on body — numbers align everywhere
- Token NAMES unchanged (`sage-*`, `copper-*`, `sand-*` re-valued) so all consumers restyled automatically; ~40 files hue-mapped via exact oklch value substitution
- Chart category palettes lead with brand green; blue kept as a data hue

## Alternatives
Dark terminal theme, navy accent, keeping copper — rejected in the user interview (user confirmed light/ledger-green/serif-display).

## Consequences
`.impeccable.md` is now the design contract for all future UI work. Old copper hardcodes are gone; the /ui design-system showcase reflects the new tokens.
