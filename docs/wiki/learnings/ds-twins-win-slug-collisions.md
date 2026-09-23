---
type: Learning
title: 18 root components duplicate the ds/ showcase set — slug resolution prefers the ds/ twin
description: Discovered 2026-09-23 fixing `/components/Accordion` showing no visual (commit `f19dd80`).
tags: [components, design-system, gotcha]
timestamp: "2026-09-23T07:15:08.857Z"
---

# 18 root components duplicate the ds/ showcase set — slug resolution prefers the ds/ twin

Discovered 2026-09-23 fixing `/components/Accordion` showing no visual (commit `f19dd80`).

**Fact**: all 18 `src/lib/components/ds/*.svelte` components (Accordion, Buttons, Modal, Toast…) have a same-named twin at the root of `src/lib/components/` — 36 root files, 18 exact name collisions. The root twins are leftovers from before the `ds/` folder existed and are NOT self-showcasing (no live demo, props-driven).

**Gotcha**: an exact slug match (`Accordion` → root `Accordion.svelte`) picks the demo-less variant. Fix lives in `src/routes/components/[...slug]/+page.svelte`: on collision the `ds/` twin wins, both for the detail-page fallback and the index-card links. Bare-name slugs with only a `ds/` match also resolve to the twin. `DS_SHOWCASE` is a hand-maintained map in that file — a new `ds/` component must be added there or it renders source-only.

**Cleanup candidate** (not done): the 18 root duplicates could be deleted; that's a separate pass — components like `TableDrawer` (root-only, no twin) must keep working.
