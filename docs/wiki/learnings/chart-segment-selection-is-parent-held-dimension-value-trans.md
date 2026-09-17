---
type: Learning
title: Chart segment selection is parent-held {dimension, value} transient state
description: Chart components (as used in `/pages` and the library detail page demo) manage their own click/deselect handlers once selection state exists. The wiring only ne
tags: [frontend, charts, state]
timestamp: "2026-09-17T08:04:25.250Z"
---

# Chart segment selection is parent-held {dimension, value} transient state

Chart components (as used in `/pages` and the library detail page demo) manage their own click/deselect handlers once selection state exists. The wiring only needs to hold a transient `selected` object of shape `{ dimension, value }` in the parent — click selects/deselects a segment, window-click clears it.

Non-obvious because the renderer gives no API surface for selection; the state shape is the whole contract. This bit us on the library page: the chart rendered fine but segments were unselectable because no `selected` state was held in the parent.

## Source

- `src/routes/library/[id]/+page.svelte` — holds `{dimension, value}` demo state; mirrors `/pages` behavior
