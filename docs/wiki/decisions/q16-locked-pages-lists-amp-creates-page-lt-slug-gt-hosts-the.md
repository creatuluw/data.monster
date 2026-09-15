---
type: Decision
title: "Q16 locked: /pages lists &amp; creates, /page/&lt;slug&gt; hosts the dual-mode editor"
description: Context
tags: [report-pages, routing, q16, interview]
status: accepted
timestamp: "2026-09-15T09:06:28.612Z"
---

# Q16 locked: /pages lists &amp; creates, /page/&lt;slug&gt; hosts the dual-mode editor

## Context

Report-pages design interview (Q-series, 2026-09-15). Q3 locked the two-surface format: one declarative spec document edited by both Design and Code mode. Q15's decision left open **Q16 — the no-code editing surface**. The user then specified the flow directly: a user creates pages in `/pages`, a new page opens at `/page/<slug>`, and the user builds the page there with content/components ("the A-pattern" = the dual-mode editor).

## Decision

**Locked (Q16):** `/pages` is the index + creation surface only. Creating a page immediately routes to `/page/<slug>`, which hosts the dual-mode (Design ⇄ Code) editor over the single declarative spec. All editing happens at the slug route, never in the list.

- Route split: `/pages` (list, create) → `/page/<slug>` (edit, full canvas).
- The slug is the page's identity in routes.
- Follows [[two-surface-report-page-format]] (Q3): both surfaces edit the same spec; Q16 adds *where* they live.

## Alternatives considered

- **Modal / in-list editing** — rejected: page authoring with grids, charts, and annotations needs full-canvas space, not a dialog.
- **One route with editor-mode tabs** — rejected: separates concerns poorly; list and editor have different data needs (index vs full spec).

## Consequences

- Q17 opened immediately after: **where the spec document physically lives** — internal DuckDB (`d8a_monster_pages`), workspace files, or files+DB cache. Not yet locked (assistant lean: A, internal DuckDB, consistent with master items Q6 and saved queries).
- Implementation must add the `/page/[slug]` SvelteKit route (does not exist yet at time of decision).

## Related

- [[two-surface-report-page-format]] — Q3: dual-surface editing of one spec
- [[q13-locked-explicit-grid-spec-declares-rows-blocks-take-colu]] — layout model the editor will expose
- [[master-item-library-table-binding-q6]] — items the page editor consumes
