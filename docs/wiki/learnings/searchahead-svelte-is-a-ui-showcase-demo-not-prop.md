---
type: Learning
title: SearchAhead.svelte is a /ui showcase demo, not prop-driven — build inline searchaheads
description: Discovered 2026-09-17 building the skeleton pick/create modal.
tags: [ui-components, searchahead, showcase, yagni]
timestamp: "2026-09-17T16:05:15.482Z"
---

# SearchAhead.svelte is a /ui showcase demo, not prop-driven — build inline searchaheads

Discovered 2026-09-17 building the skeleton pick/create modal.

**Fact:** `SearchAhead.svelte` (the `/ui` showcase component) is a hardcoded demo with no props — its options and behavior are baked in for the showcase page. It is **not** a reusable, prop-driven component.

**Consequence:** Don't reach for it when you need a searchahead elsewhere; you'll waste a read and then have to build anyway. Build the (small) inline version where you need it, and only extract a real prop-driven `SearchAhead` when a second consumer exists.

Used-in: [[skeleton-pick-create-moved-from-inline-dropdowns]] — the pick modal's searchahead is inline, reusing the new-page modal overlay pattern instead.
