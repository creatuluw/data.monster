---
type: Learning
title: speed-highlight/core has no Svelte grammar
description: Gotchas discovered wiring `@speed-highlight/core` into the library Code tab
tags: [frontend, syntax-highlighting, library, svelte]
timestamp: "2026-09-17T08:51:57.994Z"
---

# speed-highlight/core has no Svelte grammar

## Gotchas discovered wiring `@speed-highlight/core` into the library Code tab

- **No Svelte grammar.** `.svelte` files must map to `html` — the template markup tokenizes fine but script/style blocks get generic markup coloring. Never promise Svelte-accurate highlighting from this lib.
- **`highlightHTML(..., { block: true })` emits a CLASSLESS root `<div>`.** (Corrects an earlier claim here that it "ships its own chrome.") The theme CSS only matches `[class*=shj-lang-]` — a class that only `highlightElement` adds at runtime, never `highlightHTML`. Without it you get unstyled wrapping divs: no mono, no pre-wrap, no gutter. Fix: put `shj-lang-<lang>` on your own wrapper div — that is the library's intended theme activation, and the wrapper then supplies background, border, mono font and line-number flex layout. A small scoped override fits it to the app.
- **Async API.** `highlightHTML` returns a Promise — in Svelte 5, resolve into `$state` from an `$effect` keyed on the entry, not at module init.
- **Language codes are its own set** (`ts`, `md`, `plain`, …). Unknown extensions must be forced to `plain` or highlighting silently fails.

## Source

- `src/routes/library/[id]/+page.svelte` — language map, async highlight in effect, `shj-lang-*` wrapper
- Decision: [[speed-highlight-over-prism]]
