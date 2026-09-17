---
type: Learning
title: CSS text-transform changes innerText, not textContent — probe labels case-insensitively
description: "Symptom: a CDP DOM probe checking for the label `"Rows"` failed on the Page"
tags: [cdp, e2e, testing, gotcha]
timestamp: "2026-09-15T13:14:38.563Z"
---

# CSS text-transform changes innerText, not textContent — probe labels case-insensitively

Symptom: a CDP DOM probe checking for the label `"Rows"` failed on the Page
tab even though the tab rendered fine. The element's CSS
`text-transform: uppercase` renders it as `"ROWS"` — and **`innerText` reflects
the transformed (rendered) text**, while `textContent` keeps the source casing.

## Rule of thumb
When probing visible labels in this app (tab labels, section headers use
`uppercase`/`capitalize` utilities), match **case-insensitively** or read
`textContent` instead of `innerText`. Don't conclude "element missing" from a
casing mismatch.

Related probe gotchas: [[visibility-probes-walk-ancestor-opacity-chain]],
[[apparent-ui-bug-stale-hmr-webview]].
