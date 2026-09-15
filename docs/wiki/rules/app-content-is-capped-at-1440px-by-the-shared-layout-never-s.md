---
type: Rule
title: App content is capped at 1440px by the shared layout — never set per-page max-width
description: Guideline
tags: [frontend, layout, design-system, css]
timestamp: "2026-09-15T13:18:42.420Z"
---

# App content is capped at 1440px by the shared layout — never set per-page max-width

## Guideline

Every page's content container is capped at **1440px and auto-centered** by a single lever in the shared layout — `.app-main > :global(*)` in `src/routes/+layout.svelte` sets `max-width: var(--max-width); margin-inline: auto;` on every direct child of the main scroll container. The token `--max-width` in `src/app.css` is **90rem (1440px)**.

**Never set a per-page max-width on a page root.** The old `/pages` 1024px self-cap was exactly this anti-pattern and was removed (2026-09-15, PR #4).

## When it applies

- Creating any new route/page: just render into `.app-main` — the cap applies automatically.
- Adjusting the app-wide cap: change only the `--max-width` token (and keep the showcase containers — Nav/Hero/Footer in `/ui` — aligned to it).
- Writing layout-level selectors that target children owned by page components: Svelte scopes plain selectors away from them — the rule uses `> :global(*)` for exactly this reason.

## Rationale

One lever = zero drift. Per-page caps diverge (1024 vs 1440) and every future cap change becomes a sweep across routes. Verified on `/`, `/analyst`, `/pages`, `/page/smoke-test`, `/labs/bar-chart` (2026-09-15).

## Source

- `src/routes/+layout.svelte` — the `.app-main > :global(*)` rule
- `src/app.css` — `--max-width: 90rem`
- Related: [[design-system-app-css-tokens-ui-showcase]] — token layer this lever consumes
