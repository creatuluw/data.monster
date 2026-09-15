---
type: Rule
title: Keep test files and vitest imports out of src/
description: Keep test files and vitest imports out of src/
tags: [testing, vite, conventions, frontend]
timestamp: "2026-09-14T09:43:37.652Z"
---

# Keep test files and vitest imports out of src/

# Keep test files and vitest imports out of src/

## Guideline

Test files (anything importing `vitest`/`@testing-library` etc.) live in the top-level `tests/` directory — **never inside `src/`**, and never co-located next to the module under test.

## When it applies

Every time a test is added or moved in this project. Currently `tests/fundament.test.ts` (chart fundament, 9 tests) is the pattern to follow.

## Rationale

Vite's dev-server dep-optimizer scans everything reachable from `src/`. A `vitest` import inside the source tree makes the optimizer choke, which surfaced as the `/labs` bar-chart "hang" (2026-09-14): the bar-chart page was the only page importing from `$lib/charts/`, where `fundament.test.ts` sat, and clicking its card froze the app in an infinite vite reconnect loop. Moving the test to `tests/` (PR #3, commit `9f18749`) removed the trigger. Full story: [[labs-hang-vite-reload-loop]].

## Source

- `tests/fundament.test.ts` — current location
- PR creatuluw/data.monster#3
