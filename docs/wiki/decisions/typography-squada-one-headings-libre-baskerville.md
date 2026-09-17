---
type: Decision
title: "Typography: Squada One headings, Libre Baskerville body, Geist Mono data — Inter dropped"
description: Context (superseded by [[typography-calluna-headings-inter-body]])
tags: [design, typography, tokens, fonts, squada-one, libre-baskerville, geist-mono]
status: superseded
supersedes: "["decisions/typography-inter-mono-for-data"]"
timestamp: "2026-09-16T17:29:46.542Z"
---

# Typography: Squada One headings, Libre Baskerville body, Geist Mono data — Inter dropped

## Context

User asked (2026-09-16) to re-type the whole app: **Squada One** for headings, **Libre Baskerville** for body copy, **Geist Mono** for low-level data (tables, legends, labels, badges). This deliberately reverses [[typography-inter-mono-for-data]] (Inter everywhere + Geist Mono for data).

## Choice

All changes through the existing CSS tokens in `src/app.css` — token names unchanged, every consumer restyles automatically (same trick as the ledger redesign and the Inter swap before it):

- `--font-display: 'Squada One', 'Libre Baskerville', serif` — every heading/title.
- `--font-body: 'Libre Baskerville', Georgia, serif` — all body copy.
- `.tag` badges → `--font-mono` — badges/labels now Geist Mono.
- Tables, data cells, chart legends, and data-detail surfaces already ran on `--font-mono` — unchanged, now consistent with the badge change.
- `src/app.html`: Google Fonts link loads Squada One + Libre Baskerville (400/700 + italic) + Geist Mono; **Inter dropped**.

## Alternatives

- Keep Inter (previous decision) — rejected by user request.
- Keep `.tag` badges on the body font — rejected; badges are "detailed info" per the user's spec, so they belong to the mono tier.

## Consequences

- The app is now a **three-font system**: display (Squada One) / body (Libre Baskerville) / data (Geist Mono). The mono-for-data convention survives unchanged and now also covers badges/labels.
- **Squada One ships only weight 400** — any heading styled `font-weight: 600/700` gets browser-synthesized bold. If it looks off, strip those weights on display-font elements.
- Tour HTML captures in `docs/tours/` embed the old Google-Fonts `@import` (Inter) — **recapture tours once typography settles**.
- Type hierarchy now comes from font-family contrast (condensed display vs serif body), not Inter's weight axis.
