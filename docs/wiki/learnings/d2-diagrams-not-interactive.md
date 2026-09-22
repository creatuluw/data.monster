---
type: Learning
title: D2 diagrams are not interactive — tooltip and external link only; base64url shape classes are the DIY hook
description: Question
tags: [d2, diagrams, interactivity, research, te9-research]
timestamp: "2026-09-17T13:55:02.347Z"
---

# D2 diagrams are not interactive — tooltip and external link only; base64url shape classes are the DIY hook

## Question

Can D2 (d2lang.com) render an interactive diagram — select elements, attach behavior/links — and if not, what does?

## Answer (web-verified 2026-09-17, te9-research run)

**D2: no native interactivity.** Exactly two features exist — `tooltip` (hover text) and `link` (click → external URL). No selection, no click callbacks, no event API. Verified against d2lang.com's own "Interactive" tour; unchanged since Dec 2022.

**DIY path exists.** D2's SVG output is scriptable. Verified live against the official `@terrastruct/d2` WASM package: every shape renders as `<g class="{base64url(shapeName)}">`. A host page can decode those base64url classes and attach its own click/selection handlers. Caveat: that class scheme is **undocumented** — it can break on upgrades.

## Alternatives with native interactivity

- **Text DSL with JS callbacks**: Mermaid — `click nodeId callback`, requires `securityLevel: 'loose'`, browser-only.
- **Interactive diagram components** (all MIT, selection + click events out of the box): Cytoscape.js, React Flow, **Svelte Flow**, AntV X6, Rete.js.
- **Whiteboard-style embedding**: Excalidraw (MIT); tldraw (production needs a license key).
- **Commercial gold standard**: GoJs, yFiles.

**data.monster fit**: the app is SvelteKit — Svelte Flow or Cytoscape.js are the natural choices if interactive diagrams land in-app.

## Source

- `reports/2026-09-17-d2-interactive-diagrams/report.html` — full report (audit trail: `agents/`, `research.log`, `metrics.json` alongside).
