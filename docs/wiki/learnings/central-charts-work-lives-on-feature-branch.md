---
type: Learning
title: Central-charts work lives on feature/central-charts — master is held at a restore point
description: Discovered 2026-09-16 when the user reported the `/pages` work as "completely lost."
tags: [git, central-charts, branch-state]
timestamp: "2026-09-16T17:03:36.108Z"
---

# Central-charts work lives on feature/central-charts — master is held at a restore point

Discovered 2026-09-16 when the user reported the `/pages` work as "completely lost."

## Situation

- User was on **master**, whose HEAD is deliberately `Restore point: pre central-charts build`.
- All central-charts work lives on the **`feature/central-charts`** branch (local + pushed): 20 commits, `ae48ef2` = "Central chart system v1 — registry, page specs, dual-mode editor, semantic layer".
- The **newest uncommitted** `/pages` changes were sitting in `stash@{0}` on that branch (`src/routes/pages/+page.svelte`, `src/routes/pages/[slug]/+page.svelte`, two CDP test scripts) — created by the wiki-recap stash gotcha, see [stash-pop-silent-conflict-recovery](./stash-pop-silent-conflict-recovery.md).

## Rule of thumb

Before assuming work is lost: `git branch -a --contains`, `git log feature/central-charts`, and `git stash list`. In this repo the active line of development is a feature branch, **not** master — master is a held restore point while central-charts is under construction.

## Cleanup note

- `src/routes/pages - Copy/` is a stale identical duplicate of the old page (a delete candidate once back on the branch; SvelteKit serves it as a real route).

Related: [central-charts-component-system](../pages/entities/central-charts-component-system.md)
