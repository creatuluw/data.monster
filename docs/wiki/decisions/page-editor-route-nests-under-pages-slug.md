---
type: Decision
title: Page editor route nests under /pages/<slug> (was /page/<slug>); /data goes full-width
description: Context
tags: [, routing, pages, report-pages]
status: accepted
supersedes: decisions/q16-pages-lists-creates-slug-hosts-editor
timestamp: "2026-09-15T14:41:18.561Z"
---

# Page editor route nests under /pages/<slug> (was /page/<slug>); /data goes full-width

## Context

Q16 locked `/pages` (list + create) → `/page/<slug>` (dual-mode editor) on 2026-09-15. After living with it, the user requested the editor move under the `/pages` route family so all page-related URLs share one prefix. In the same session, `/data` was made a full-width full-bleed page (the data-analyst surface).

## Decision

- **Page editor route is now `/pages/<slug>`** (SvelteKit: `src/routes/pages/[slug]/+page.svelte`). The old `src/routes/page/[slug]/` route is removed. `/pages` remains the index + create surface; creating a page routes to `/pages/<slug>`.
- **`/data` is a full-bleed page** (`data-page full-bleed` root class) — it spans the whole window as the data-analyst surface, consistent with the [[app-content-capped-at-shared-max-width]] escape hatch. *(Superseded 2026-09-16: the full-bleed class was removed — `/data` is capped/centered like every page; see [[all-pages-capped-1920px-full-bleed-removed]]. The route nesting above still stands.)*
- This supersedes the route assignment in Q16; everything else in Q16 (list-creates → slug-route-hosts-editor flow, slug as identity) is unchanged.

## Alternatives considered

- Keep `/page/<slug>` as locked in Q16 — rejected: two prefixes (`/pages` vs `/page`) for one feature family is a needless split; nested under `/pages` groups them and leaves `/page` free.

## Consequences

- Any link, e2e script, or doc referencing `/page/<slug>` must be updated — `tests/config-ui-cdp.mjs` and `tests/smoke-cdp.mjs` now both navigate to `http://localhost:6123/pages/smoke-test`.
- The slug remains the page's identity; only the URL prefix changed.
- `goto`/`href` targets after page creation point at `/pages/<slug>`.

## Related

- [[q16-pages-lists-creates-slug-hosts-editor]] — superseded on the route path
- [[all-pages-capped-1920px-full-bleed-removed]] — later reversed the full-bleed exemption this decision gave `/data`; current rule is [[app-content-capped-at-shared-max-width]]
