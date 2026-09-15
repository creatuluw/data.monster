---
type: Decision
title: Page editor route nests under /pages/<slug> (was /page/<slug>); /data goes full-width
description: Context
tags: [, routing, pages, report-pages]
status: accepted
supersedes: decisions/q16-locked-pages-lists-amp-creates-page-lt-slug-gt-hosts-the
timestamp: "2026-09-15T14:41:18.561Z"
---

# Page editor route nests under /pages/<slug> (was /page/<slug>); /data goes full-width

## Context

Q16 locked `/pages` (list + create) → `/page/<slug>` (dual-mode editor) on 2026-09-15. After living with it, the user requested the editor move under the `/pages` route family so all page-related URLs share one prefix. In the same session, `/data` was made a full-width full-bleed page (the data-analyst surface).

## Decision

- **Page editor route is now `/pages/<slug>`** (SvelteKit: `src/routes/pages/[slug]/+page.svelte`). The old `src/routes/page/[slug]/` route is removed. `/pages` remains the index + create surface; creating a page routes to `/pages/<slug>`.
- **`/data` is a full-bleed page** (`data-page full-bleed` root class) — it now spans the whole window as the data-analyst surface, consistent with the [[app-content-is-capped-at-1440px-by-the-shared-layout-never-s]] escape hatch.
- This supersedes the route assignment in Q16; everything else in Q16 (list-creates → slug-route-hosts-editor flow, slug as identity) is unchanged.

## Alternatives considered

- Keep `/page/<slug>` as locked in Q16 — rejected: two prefixes (`/pages` vs `/page`) for one feature family is a needless split; nested under `/pages` groups them and leaves `/page` free.

## Consequences

- Any link, e2e script, or doc referencing `/page/<slug>` must be updated (e.g. `tests/config-ui-cdp.mjs` already moved).
- The slug remains the page's identity; only the URL prefix changed.
- `goto`/`href` targets after page creation point at `/pages/<slug>`.

## Related

- [[q16-locked-pages-lists-amp-creates-page-lt-slug-gt-hosts-the]] — superseded on the route path
- [[app-content-is-capped-at-1440px-by-the-shared-layout-never-s]] — full-bleed opt-out /data now uses
