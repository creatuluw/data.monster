---
type: Rule
title: Leave wiki-recap noise uncommitted — branch fresh and commit selectively, never stash
description: Guideline
tags: [git, workflow, wiki-recap, pr, feature-loop]
timestamp: "2026-09-17T11:52:26.740Z"
---

# Leave wiki-recap noise uncommitted — branch fresh and commit selectively, never stash

## Guideline

When opening a PR (per [[rules/feature-loop-hard-rules]]), if `git status` shows
dirty `docs/wiki/` files you didn't touch, **leave them uncommitted in the
working tree**. Do NOT stash them, do NOT commit them:

1. Branch from fresh master (`git pull --ff-only`, then branch).
2. Stage and commit **only the files your task actually changed**.
3. Push, open the PR, and let the recap noise keep sitting in the tree.

## Why

The background **wiki-recap agent** writes to `docs/wiki/` across sessions, so
dirty wiki files in your tree are (a) not yours and (b) constantly changing.
Stashing them leads to silent stash-pop conflicts because the recap agent has
already written newer versions of the same files — see
[[learnings/stash-pop-silent-conflict-recovery]] (PR #3 lost a session that
way). Selective commit sidesteps the whole problem: the stash never exists, so
it can't fail. The recap files get committed by whichever session owns them.

Also prevents shipping another session's half-written recap into your PR
("recap noise", PR #6).

## Applies to

Every PR push in this repo when the working tree has dirty `docs/wiki/` files
you don't recognize. Sanity-check the dirty list first — if a dirty file IS
yours, commit it with your change.
