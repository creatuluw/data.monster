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

## RESOLVED — 2026-09-17: work merged via PR #4, master is current

The same panic recurred ("2 days of work gone" — /pages, /data). Again nothing was lost: **`feature/central-charts` had been merged into `origin/master` as PR #4** (23 commits, ~20k lines, HEAD `f1fc3b9`), but local master was still sitting at the `Restore point: pre central-charts build` commit and was never pulled. Fixed with: stash → `git pull` (fast-forward) → `stash pop`.

- **The rule of thumb above is now obsolete**: master is the current line of development again. But the first step stands and proved out twice: before assuming work is lost, check **`origin/master`** (`git log origin/master`, `git status -sb` behind/ahead count) in addition to `git branch -a --contains` and `git stash list`. In this repo "gone" has meant "on a branch or upstream, un-pulled" both times it was reported.
- Stash-pop conflicts in auto-generated wiki files (changelog/memory churn): the stash side was a strictly older subset, so taking upstream for both files was safe — verify the stashed side holds no unique entries before discarding, per [stash-pop-silent-conflict-recovery](./stash-pop-silent-conflict-recovery.md).
- Habit to avoid a third occurrence: `git pull` on master after every PR merge — something checked master out at the restore point after the merge and it went unnoticed for ~2 days.
