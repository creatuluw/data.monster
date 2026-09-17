---
type: Learning
title: Squada One is single-weight (400) — heading font-weight 600/700 gets browser-synthesized bold
description: "Discovered 2026-09-16 while re-typing the app ([[typography-squada-one-headings-libre-baskerville]])."
tags: [typography, fonts, squada-one, design, gotcha]
timestamp: "2026-09-16T17:30:26.216Z"
---

# Squada One is single-weight (400) — heading font-weight 600/700 gets browser-synthesized bold

Discovered 2026-09-16 while re-typing the app ([[typography-squada-one-headings-libre-baskerville]]).

**Squada One ships only weight 400** on Google Fonts. Any heading styled `font-weight: 600/700` (left over from the Inter era, where weight carried the hierarchy) renders as **browser-synthesized bold** — chunky faux strokes, not real glyphs.

## What to do
- On display-font elements (`--font-display` consumers), drop explicit `font-weight: 600/700`; let 400 + Squada One's naturally heavy condensed letterforms carry the hierarchy.
- If a heavier heading tier is truly needed, switch that tier's font, not its weight.
- Contrast now comes from **font-family contrast** (condensed display vs serif body vs mono), not the weight axis — the old Inter-era instinct "bump the weight" no longer applies.

Libre Baskerville, by contrast, has real 400/700 + italic — body weights are safe.
