---
type: Rule
title: Pointer cursor comes from one global rule in app.css
description: Guideline
tags: [css, app.css, interaction, global-rule]
timestamp: "2026-09-15T14:37:36.489Z"
---

# Pointer cursor comes from one global rule in app.css

## Guideline

All interactive affordances (buttons, links, selects, tabs, summary, `label[for]`, `[role='button']`, `[onclick]`) get `cursor: pointer` from ONE global rule in `src/app.css`:

```css
button:not(:disabled), a, select:not(:disabled), summary,
label[for], [role='button'], [role='tab'], [onclick] { cursor: pointer; }
```

Never add `cursor: pointer` in a component. If an interactive element still shows the arrow, extend the global selector list instead.

Disabled controls are deliberately excluded (they keep `not-allowed`/default).

## When it applies

Every time a new interactive element is added anywhere in the app — no per-component cursor styling is needed or wanted.

## Rationale

Windows/Chrome does not give buttons a pointer cursor by default, so every button in the app looked "dead" until this rule landed (added 2026-09-15, verified via CDP full-scan across /pages, /data, /analyst, /settings, /labs/bar-chart — 0 exceptions). One global rule is the root-cause fix; per-component cursor styles would drift and multiply.
