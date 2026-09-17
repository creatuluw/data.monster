---
type: Learning
title: Tour DOM snapshots scale with the live DOM — bound secondary frames, keep one big frame when size is the story
description: "Discovered 2026-09-16 finishing the query-tour (see [[app-tour-set-docs-tours]])."
tags: [true, app-tour-demo, tours, cdp, performance]
timestamp: "2026-09-16T14:51:40.102Z"
---

# Tour DOM snapshots scale with the live DOM — bound secondary frames, keep one big frame when size is the story

Discovered 2026-09-16 finishing the query-tour (see [[app-tour-set-docs-tours]]).

## Symptom

query-tour.html was 20.6 MB. Two frames (pagination + ingest modal) each carried a ~9.4 MB snapshot because the captured query returned 51k rows and the app pages non-LIMIT queries at 10,000 rows/page.

## Key facts

- A tour frame's size = the live DOM at capture time. A 10k-row results page is ~9.4 MB per frame.
- The pager UI only appears for big results — the big DOM is *real app behavior*, not a capture accident.
- Modal/secondary frames don't need the big result: the same modal renders identically over a small aggregation result.

## The lazy-honest fix

- Keep **one** big frame when the size IS the story (the 10k-row pagination frame shows the pager — that's the feature).
- Re-capture every other frame (modals, secondary views) over a small bounded query.
- Result: 20.6 MB → 10.2 MB with narration intact.

## Rule of thumb

Before capture, audit which frames genuinely need the large dataset; everything else runs on a bounded query. Also remember cleanup handles (the bounded query was saved — delete it in the draaiboek's cleanup step).
