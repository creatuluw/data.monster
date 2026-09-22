---
type: Learning
title: Mock-Tauri browser repro harness is gone — verify visually via self-contained routes
description: "Discovered 2026-09-18 while trying to visually verify the drawer restyle: the CDP port wasn't open, so I reached for the mock-Tauri browser repro technique docu"
tags: [testing, repro, cdp, mock-tauri]
timestamp: "2026-09-18T10:39:36.983Z"
---

# Mock-Tauri browser repro harness is gone — verify visually via self-contained routes

Discovered 2026-09-18 while trying to visually verify the drawer restyle: the CDP port wasn't open, so I reached for the mock-Tauri browser repro technique documented in [[query-editor-blowup-was-app-column-min-height-auto]] - **the mocktauri harness has been removed from the repo**. That learning's repro recipe no longer works.

Fallback that does work: open a **self-contained route** (one that needs no Tauri data, e.g. `/labs/bar-chart`) in the agent browser and verify visually there. Don't burn time hunting for the harness; it's gone.
