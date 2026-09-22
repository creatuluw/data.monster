---
type: Learning
title: "Shallow URL state in SvelteKit: replaceState from $app/navigation, never goto or window.history"
description: Discovered 2026-09-22 making /data tab selection URL-addressable (`TableOverview.svelte`, +6 lines).
tags: [sveltekit, url-state, shallow-routing, svelte5]
timestamp: "2026-09-22T07:33:09.581Z"
---

# Shallow URL state in SvelteKit: replaceState from $app/navigation, never goto or window.history

Discovered 2026-09-22 making /data tab selection URL-addressable (`TableOverview.svelte`, +6 lines).

**The technique** — sync ephemeral view state (active tab) to the URL:

- Use `replaceState(url, {})` from `$app/navigation` (SvelteKit's shallow-routing helper), **not** `window.history.replaceState` and **not** `goto()`. The SvelteKit version updates the reactive `page.url` (from `$app/state`) without re-running load functions, without navigation, and without a history entry. Raw `window.history.replaceState` would leave `page.url` desynced from the address bar, breaking any effect that reads it.
- Guard the `$effect` against no-op writes (`if ((page.url.searchParams.get('tab') ?? 'tables') === activeTab) return;`) or it fires on every dependency touch and loops.
- Strip the param when the value equals the default (`replaceState('/data', {})`) — keeps URLs clean.
- Coexists with consume-once preset params: the `add=1&table=…&return=…` deep-link preset ([[create-in-data-round-trip]]) is parsed and stripped at init, before the tab effect runs, so the two never fight over the URL.

Reusable anywhere a filter/tab/panel state should survive reload or be shareable (e.g. /labs, /query filters).
