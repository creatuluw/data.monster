---
type: Learning
title: Stale-wiki file floods — only noise if an ignore pattern actually matches the tree
description: Symptom and root cause — src-tauri/target leaked through, fixed via .wiki_ignore plus extension BUILTIN_IGNORES.
tags: [wiki-tooling, okf, gotcha]
timestamp: "2026-09-12T11:05:34.751Z"
---

# Stale-wiki file floods — only noise if an ignore pattern actually matches the tree

## Symptom

The end-of-turn wiki staleness report can list thousands of "changed" files — e.g. "3485 file(s) changed" — and any scan over them stalls (see also [[never-tree-scan-archive-or-src-tauri-target-du-find-stall-on]]).

## Cause

The flood is build/dependency artifacts — but such files are only noise if an ignore rule actually MATCHES them. On 2026-09-12 the flood was `src-tauri/target/` (Rust build tree): no `.wiki_ignore` pattern covered it and the wiki-context extension had no built-in defaults, so every artifact looked like a real change.

## Fix (2026-09-12)

- Appended `src-tauri/target/` to this project's `.wiki_ignore` — immediate effect.
- The wiki-context extension (`index.ts`) now merges `BUILTIN_IGNORES` (~40 always-on patterns: Rust `target/`, Python `.venv/`/caches, JVM `.gradle/`, .NET `bin/`/`obj/`, `Pods/`/`DerivedData/`, `.dart_tool/`, JS meta-frameworks, Terraform, lock files) inside `loadIgnore()` — the single choke point both the staleness scan and file-tree generation route through, with or without a `.wiki_ignore` present. Loads at next session start.

## Rule of thumb

- A huge stale-file count is noise ONLY for trees an ignore rule actually matches. Check which files are listed: under an ignored tree → do nothing; a whole build tree leaking through (like `src-tauri/target/` did) → add it to `.wiki_ignore` (or rely on extension built-ins from next session).
- Only source files under `src/`, `src-tauri/` (except `target/`), `docs/` (non-generated), `.specs/`, `.prds/` in the stale list signal real drift worth a concept update or `wiki_mark_synced()`.
- Verified matcher edge case: pattern `src/bin/` ignores the directory but keeps file `src/bin.ts` — dir patterns don't swallow same-named files.

## Related

- Documents the config behind the gotcha in [[wiki-ignore-staleness-policy]] — the entity page for `.wiki_ignore` itself.
