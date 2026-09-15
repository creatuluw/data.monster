---
type: Learning
title: Stash pop can silently fail when wiki-recap writes conflict — verify and restore from the stash
description: "Discovered 2026-09-14 while committing session work (PR #3)."
tags: [git, workflow, wiki-recap, recovery]
timestamp: "2026-09-14T08:38:22.409Z"
---

# Stash pop can silently fail when wiki-recap writes conflict — verify and restore from the stash

Discovered 2026-09-14 while committing session work (PR #3).

## Symptom
`git stash pop` appeared to succeed, but `git status` showed only 1 tracked file modified — the entire stashed session (Inter/Geist Mono fonts, URL-copy icon, component polish) was missing from the working tree.

## Root cause
The pop conflicted: the background **wiki-recap agent** had written newer recap content into `docs/wiki/` files that also existed in the stash. On conflict the pop aborts and **the stash is silently kept** — untracked/other paths from the stash are not restored either, so the product work (`src/`) stays hidden inside `stash@{0}`.

## Recovery recipe
1. Verify: `git status` (file count far below expected) + `git stash list` (stash still there).
2. Restore product code from the stash without clobbering newer working-tree files:
   `git checkout stash@{0} -- src/` — restores only `src/`, keeping the newer wiki-recap files in the working tree.
3. Commit, then before `git stash drop` confirm the stash is fully superseded: diff the stash against the branch; only stale wiki snapshots should remain.

## Prevention
After ANY stash pop in this repo, sanity-check that the expected number of modified files is actually present before editing further — concurrent wiki-recap writes make silent conflicts the norm, not the exception.
