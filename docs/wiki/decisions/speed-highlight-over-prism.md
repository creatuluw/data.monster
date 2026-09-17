---
type: Decision
title: Use speed-highlight/core for code highlighting instead of Prism
description: Context
tags: [frontend, library, dependencies, syntax-highlighting]
status: accepted
timestamp: "2026-09-17T08:51:57.993Z"
---

# Use speed-highlight/core for code highlighting instead of Prism

## Context

The `/library/[id]` detail page's Code tab highlighted source snippets with `prismjs` + `prism-svelte` wrapped in a hand-rolled `<pre>` block chrome (background, mono font, gutter).

## Choice

Swap to **@speed-highlight/core** (`highlightHTML(src, lang, { block: true, showLineNumbers: true })`, theme `@speed-highlight/core/themes/github-light.css`). Prism is fully removed: `prismjs`, `prism-svelte`, `@types/prismjs` uninstalled.

## Alternatives considered

- **Keep Prism** — worked, but three packages for one tab and a custom `<pre>` wrapper to maintain.
- **Shiki** — heavier (TextMate grammars + WASM oniguruma); more than the tab needs.

## Rationale

User-requested; also a net simplification: speed-highlight renders its own block chrome (background, mono font, line-number gutter), so the hand-rolled wrapper is gone — the file row (path + copy button) is all that sits above each block. Async API fits Svelte 5 (`$state` + `$effect` keyed on the entry).

## Consequences

- Language is chosen by file extension in `+page.svelte`: `.ts` → `ts`, `.svelte` → `html` (no Svelte grammar — see [[speed-highlight-core-has-no-svelte-grammar]]), `.md` → `md`, unknown → `plain`.
- Renaming/dropping those mappings silently degrades snippets to `plain`.
- Code entries stay keyed by full repo path (see [[library-code-entries-are-keyed-by-full-repo-paths]]).
