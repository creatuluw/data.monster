---
type: Decision
title: All pages capped at 1920px and centered; full-bleed exemption removed
description: Context
tags: [frontend, layout, design-system, css]
status: accepted
timestamp: "2026-09-16T17:12:41.259Z"
---

# All pages capped at 1920px and centered; full-bleed exemption removed

## Context

PR #4 (2026-09-15) introduced a 1440px content cap via one shared layout lever plus a `.full-bleed` escape hatch, used by `/analyst`, `/data`, `/pages`, `/pages/[slug]` — data-dense surfaces that stretched to the full window. The user then requested: "on ALL pages the content container is centered and has a max width of 1920."

## Decision

- `--max-width` in `src/app.css` changes **90rem (1440px) → 120rem (1920px)**.
- The `.full-bleed` override (`.app-main > :global(.full-bleed) { max-width: none; }`) is **deleted** from `src/routes/+layout.svelte`; the inert `full-bleed` classes are dropped from the four pages that used them.
- One mechanism, zero exemptions: `.app-main > :global(*)` → `width: 100%; max-width: var(--max-width); margin-inline: auto` governs every page.

## Alternatives considered

- Keep the full-bleed hatch for data-dense pages — rejected: the user explicitly asked for ALL pages; two mechanisms for one concern is drift.
- Per-page caps — rejected long ago (1024 vs 1440 vs none divergence); stays rejected.

## Consequences

- Supersedes **only the full-bleed/full-width clauses** of [Page editor route nests under /pages/&lt;slug&gt;; /data goes full-width](page-editor-route-nests-under-pages-slug.md) — its route-nesting decision stands unchanged.
- On screens wider than 1920px, `/analyst`, `/data`, and the page editor now center instead of stretching; smaller screens are unchanged.
- Rule updated in place (renamed): [All pages are capped at 1920px and centered by the shared layout — no per-page opt-out](../rules/app-content-capped-at-shared-max-width.md).
- `/ui` showcase containers (Nav/Hero/Footer) should stay aligned to the token.

## Update 2026-09-16 — mechanism evolved to a single `.app-column`

Later the same day the user asked that the container "also contain the header, breadcrumb, content and all other like footers etc." and "always be as high as the viewport." The cap moved off `.app-main > :global(*)` and the separate `.app-header` rule onto one wrapper: `.app-column` in `src/routes/+layout.svelte` (capped, centered, `flex: 1` inside a `height: 100vh` `.app-shell`) now contains header + breadcrumb + content and gets full-viewport-height side borders above 1920px, with `.app-main` scrolling internally. Decision unchanged in substance — one lever, 1920px, zero exemptions; see the rule for the current mechanism.
