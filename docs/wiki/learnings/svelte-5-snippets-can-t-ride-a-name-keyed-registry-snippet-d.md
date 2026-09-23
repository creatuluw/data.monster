---
type: Learning
title: Svelte 5 snippets can't ride a name-keyed registry — snippet demos are inline markup; BarChart takes accessors
description: "Discovered 2026-09-23 building `ComponentDemo.svelte` (the /components visual-demo registry):"
tags: [svelte-5, snippets, components, charts]
timestamp: "2026-09-23T07:26:06.997Z"
---

# Svelte 5 snippets can't ride a name-keyed registry — snippet demos are inline markup; BarChart takes accessors

Discovered 2026-09-23 building `ComponentDemo.svelte` (the /components visual-demo registry):

- **Svelte 5 snippets are template constructs** — you cannot stash a snippet block in a name-keyed registry/map and hand it to a component dynamically. Any demo that must pass snippet children (e.g. Tabs with snippet labels, Field with a control child) has to be authored as literal inline markup in its own branch. That is why the registry is a `DS_SHOWCASE` map plus an if/else chain over `name`, not one data-driven config.
- **`charts/BarChart` takes accessor functions, not column-name strings** — a demo (or any caller) passes `(row) => row.x` accessors over its rows; passing field-name strings renders nothing.
