---
type: Decision
title: "Typography: Calluna headings, Inter body, Geist Mono data — Squada One/Libre Baskerville dropped"
description: Context
tags: [design, typography, tokens, fonts, calluna, inter, geist-mono]
status: accepted
supersedes: typography-squada-one-headings-libre-baskerville
timestamp: "2026-09-16T18:47:26.684Z"
---

# Typography: Calluna headings, Inter body, Geist Mono data — Squada One/Libre Baskerville dropped

## Context

One day after the Squada One / Libre Baskerville re-type ([[typography-squada-one-headings-libre-baskerville]]), the user changed direction: **Calluna for headings, Inter for body, Geist Mono for data detail** (unchanged). Squada One's single-weight limitation (see [[squada-one-is-single-weight-400]]) made synthesized bolds on headings; Calluna ships real 700 + italics.

## Choice

Same token-swap pattern as every prior re-type — only `src/app.css` tokens and the Google-Fonts link in `src/app.html` change; token names unchanged, every consumer restyles automatically:

- `--font-display: 'Calluna', 'Inter', serif` — headings/titles. Loads Calluna 400/700 + italics; heading bolds are genuine weights now.
- `--font-body: 'Inter', sans-serif` — Inter is back in the fonts link after being dropped 2026-09-16.
- `--font-mono` (Geist Mono) unchanged — still tables, chart axis ticks/legends, `.tag` badges, data detail.

## Alternatives

- Keep Squada One/Libre Baskerville — rejected by user request; Squada One 400-only forced synthesized bolds.
- Keep Libre Baskerville as body — dropped in favor of Inter.

## Consequences

- Three-tier system survives (display serif / neutral body / mono data); only the display and body families changed. Mono-for-data convention untouched.
- Tour HTML captures in `docs/tours/` embed the Google-Fonts `@import`, so **all tours were recaptured** for this font change (see [[tour-html-captures-embed-google-fonts-import]]).
- Never hardcode font-family — always the CSS vars, so the next re-type is again a two-file diff.
