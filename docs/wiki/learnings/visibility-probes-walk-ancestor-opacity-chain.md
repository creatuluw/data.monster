---
type: Learning
title: "Visibility probes must walk the ancestor opacity/display/visibility chain — an opacity:0 parent hides everything"
description: "CDP "visibility" checks lied twice on the page-editor config drawer (2026-09-15):"
tags: [cdp, testing, css, central-charts]
timestamp: "2026-09-15T12:12:18.321Z"
---

# Visibility probes must walk the ancestor opacity/display/visibility chain — an opacity:0 parent hides everything

CDP "visibility" checks lied twice on the page-editor config drawer (2026-09-15):

- **Symptom**: user reported the config drawer never appearing; every probe passed (element exists, `drawer-open` class, correct width, own computed style visible).
- **Root cause**: the drawer was nested inside `.drawer-overlay`, which stayed `opacity: 0` in `overlay=false` mode — **opacity on an ancestor hides the whole subtree visually** while every check of the child's own styles stays green.
- **Rule**: an element is only human-visible if the **entire ancestor chain** passes `opacity !== 0 && display !== none && visibility !== hidden`. Walk up from the element; never trust the element's own computed style.
- Related: [[apparent-ui-bug-stale-hmr-webview]] (the other cause of "works in probes, broken for user" — stale HMR webview; Ctrl+R first, then re-probe).
