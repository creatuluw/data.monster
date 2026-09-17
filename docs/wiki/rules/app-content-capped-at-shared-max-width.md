---
type: Rule
title: All pages are capped at 1920px and centered by the shared layout — no per-page opt-out
description: One wrapper, .app-column, owns header + breadcrumb + content, is capped at 1920px, centered, and always exactly viewport-high with full-height side borders.
tags: [frontend, layout, design-system, css]
timestamp: "2026-09-16T17:19:55.074Z"
---

# All pages are capped at 1920px and centered by the shared layout — no per-page opt-out

## Guideline

The whole app chrome — header, breadcrumb bar, content, any future footer — lives inside **one wrapper, `.app-column`** in `src/routes/+layout.svelte` (2026-09-16, user request: the container should "also contain the header, breadcrumb, content… and always be as high as the viewport"). The column carries the single cap: `width: 100%; max-width: var(--max-width); margin-left/right: auto`, with `--max-width: 120rem` (1920px) from `src/app.css`. The earlier per-bar cap on `.app-header` (app.css) and the per-child cap on `.app-main > :global(*)` were removed — children of `.app-main` now only get `width: 100%` so they fill the column (the `width: 100%` is still load-bearing: auto margins in a flex column disable stretch — see [[auto-margins-app-main-disable-flex-stretch]]).

**Viewport-high by construction**: the outer `.app-shell` is `display: flex; flex-direction: column; height: 100vh`, and `.app-column` is `flex: 1` inside it — the column always stretches to exactly the viewport height, while its inner `.app-main` (`flex: 1; overflow-y: auto`) scrolls internally.

**Side borders run full height**: `@media (min-width: 1920px)` sets `border-left/right: 1px solid #d4d9d6` on `.app-column`. Because the column is viewport-high, the borders delimit the whole column (header through content) against the empty window space — including alongside the internally-scrolling area. `border-box` sizing keeps the column at exactly 1920px including the border.

**There is no escape hatch.** Never set a per-page max-width (or `max-width: none`) on a page root. Only `/ui` bypasses the shell (it renders before the `app-shell` wrapper via `isUiPage`).

## When it applies

- New route/page: just render into `.app-main` — the cap and centering come from the column; no width CSS needed.
- Adjusting the app-wide cap: change only the `--max-width` token (keep the `/ui` showcase containers — Nav/Hero/Footer — aligned to it).
- New persistent chrome (footer, status bar, …): put it inside `.app-column` — it inherits the cap, centering, and side borders automatically. This is the point of the wrapper: the pre-column design needed every bar added to both the cap and the border rule separately (the breadcrumb was full-width until 2026-09-16).

## Rationale

One wrapper, zero opt-outs = zero drift. A column that owns all chrome means new bars can't forget the cap, and stretching the column inside a 100vh shell is what makes the borders read as full-height rails instead of stopping at the scroll container.

## Related decisions

- [All pages capped at 1920px centered; full-bleed exemption removed](../decisions/all-pages-capped-1920px-full-bleed-removed.md) — the reversal that produced this rule; mechanism since evolved to `.app-column` (dated update at the end of that decision)

## Source

- `src/routes/+layout.svelte` — `.app-shell` (100vh) → `.app-column` (single 1920px cap + ≥1920px full-height side borders) → header, breadcrumb, `.app-body` → `.app-main` (internal scroll; children `width: 100%`)
- `src/app.css` — `--max-width: 120rem`
- Related: [[design-system-app-css-tokens-ui-showcase]], [[auto-margins-app-main-disable-flex-stretch]]
