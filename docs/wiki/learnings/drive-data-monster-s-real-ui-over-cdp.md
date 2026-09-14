---
type: Learning
title: Drive data.monster's real UI over CDP with --remote-debugging-port for e2e debugging
description: "The changelog-e2e skill's technique transfers from the changelog.monster app to **data.monster**: launch the Tauri app with `--remote-debugging-port` and drive "
tags: [e2e, cdp, tauri, debugging, analyst]
timestamp: "2026-09-11T22:10:42.500Z"
---

# Drive data.monster's real UI over CDP with --remote-debugging-port for e2e debugging

The changelog-e2e skill's technique transfers from the changelog.monster app to **data.monster**: launch the Tauri app with `--remote-debugging-port` and drive the real webview UI over CDP (Chrome DevTools Protocol). This works because both are Tauri v2 apps whose frontend is a webview.

## When to use

- Debugging "page isn't working" reports (e.g. the Analyst `/analyst` page) where you need to click the real UI, watch console/network, and reproduce the user's flow — not just read code.
- Any e2e verification of a registered flow: connect → preview → query → data.

## How

1. Run the app with the remote debugging flag (the changelog-e2e skill at `C:\Users\PTW\.pi\agent\skills\changelog-e2e\SKILL.md` documents the exact incantation for a Tauri v2 app).
2. Attach over CDP and drive the actual rendered UI.

## Why it matters

Static code reading missed the bug class the user is reporting; driving the real UI reproduces it directly. This gives data.monster the same e2e debugging capability that changelog.monster already has, with zero new tooling.
