---
type: Learning
title: "Auto margins in the flex-column .app-main disable flex stretch — full-bleed pages shrink without width: 100%"
description: Symptom
tags: [flexbox, css, layout, full-bleed]
timestamp: "2026-09-15T14:41:18.561Z"
---

# Auto margins in the flex-column .app-main disable flex stretch — full-bleed pages shrink without width: 100%

## Symptom

Making `/data` full-width: the root `.data-page` had `max-width: none` (via the `full-bleed` class) yet rendered only **633px wide** on a 1375px window — shrink-to-fit, as if the cap were still applied.

## Root cause

`.app-main` is a **flex column** (`flex: 1` inside a flex row). The shared layout centers every child with `margin-left/right: auto`. Per the flexbox spec, **auto margins on a flex item absorb the free space and disable `align-items: stretch`** — the item reverts to shrink-to-fit (max-content) width. `max-width: none` removes the cap but nothing forces the child wide, so it collapses to content width.

## Fix

Give children of `.app-main` an explicit `width: 100%` alongside the auto margins (in `src/routes/+layout.svelte`):

```css
.app-main > :global(*) {
	width: 100%;
	max-width: var(--max-width);
	margin-left: auto;
	margin-right: auto;
}
```

`width: 100%` resolves against the flex container's width, so centering still works when capped and full-bleed children actually span the window.

## Rule of thumb

In a **flex column** scroll container, auto-margin centering silently disables stretch. Any "cap-and-center children" lever needs `width: 100%` (or `align-self: stretch`), or full-bleed children shrink to content width.

## Related

- [[app-content-capped-at-shared-max-width]] — the layout lever this bug lived in
