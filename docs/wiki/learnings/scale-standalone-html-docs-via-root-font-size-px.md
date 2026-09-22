---
type: Learning
title: Scale standalone HTML docs via root font-size + px sweep — zoom breaks fixed overlays
description: Discovered 2026-09-18 scaling `docs/design-system-data-monster.html` to 80%.
tags: [css, docs, design-system, html]
timestamp: "2026-09-18T10:22:58.435Z"
---

# Scale standalone HTML docs via root font-size + px sweep — zoom breaks fixed overlays

Discovered 2026-09-18 scaling `docs/design-system-data-monster.html` to 80%.

**The gotcha**: `zoom: 0.8` on the root is the tempting one-liner, but it breaks every `position: fixed` overlay — in that doc, three modal backdrops + the toast container misrender. Zoom is not equivalent to scaling.

**The two levers that scale a doc deterministically**:

1. `html { font-size: 80% }` — every rem token (type scale, spacing scale, max-width caps) scales from root.
2. Sweep px literals ≥4 by ×0.8.

**Sweep exceptions**:

- Sub-4px values (1–3px hairlines, focus rings, indicators) stay full-size — ×0.8 makes sub-pixel mush.
- Viewport media queries (e.g. 640px) stay untouched — they measure the window, not elements.
- Update px labels in the doc prose to match (spacing labels, "Grid Base"); rem token labels stay truthful — the root shrank, the tokens didn't.

Applies to the project's other standalone HTML deliverables (`docs/tours/*` players, `docs/markdown-viewer.html`) if they ever need scaling. Same family as [[tour-html-captures-embed-google-fonts-import]].

Update 2026-09-18: the 80% pass on `docs/design-system-data-monster.html` itself was reverted the same day at user request (old original size + 72rem layout preferred). The technique stands as the way to scale a standalone doc if ever needed again.