---
type: Entity
title: .wiki_ignore staleness policy
description: Project-level additive ignore config layered on the wiki-context extension's built-in ignores.
tags: [wiki, tooling, config]
timestamp: "2026-09-12T11:05:34.751Z"
---

# .wiki_ignore staleness policy

## What is it?

`.wiki_ignore` at the repo root is the project-level staleness-detection config for the OKF wiki: paths listed here never trigger the "wiki may be stale" warning, because changes to them don't change what the wiki documents. As of 2026-09-12 it is **additive** — the wiki-context extension now merges ~40 always-on built-in ignore patterns (BUILTIN_IGNORES) inside `loadIgnore()`, so common build trees are ignored even without a `.wiki_ignore` entry.

## Why it matters

Without ignores, every build artifact and archived experiment (thousands of files under `src-tauri/target/`, `.archive/`) falsely marks the wiki stale and drowns out real product-code changes.

## Details

- **Location**: `E:/data.monster/.wiki_ignore`
- **Format**: gitignore-style patterns, `#` comments
- **Layering**: extension `BUILTIN_IGNORES` (Rust `target/`, Python `.venv/`/caches, JVM `.gradle/`, .NET `bin/`/`obj/`, Apple `Pods/`/`DerivedData/`, Flutter `.dart_tool/`, JS meta-frameworks, Terraform, lock files) are always-on and merged in `loadIgnore()` — the single choke point BOTH the staleness scan and file-tree generation route through, with or without a `.wiki_ignore` present. This file only adds project-specific entries on top.

**Ignored (do NOT affect staleness):**

- `docs/wiki/` — self-ignore; wiki edits aren't product changes
- Standard caches: `node_modules/`, `dist/`, `build/`, `.next/`, `.cache/`, `__pycache__/`, `.DS_Store`
- `src-tauri/target/` — Rust build tree (added 2026-09-12 after it caused a 3485-file staleness flood; no earlier pattern matched it)
- `.archive/` — superseded old app + chart trials; historical, huge, never affects wiki staleness
- `.svelte-kit/`, `.cargo/`, `.work/`, `.pi/`, `.opencode/` — build caches and agent/tooling state that churn constantly
- `data.monster/` — nested design-docs site generator, generated output
- `.env`, `data/`, `global_superstore.csv` — data and secrets, never scanned

**Tracked (changes SHOULD trigger staleness):**

- `src/`, `src-tauri/` (except `target/`), `.specs/`, `.prds/`, `docs/` (non-wiki) — the product code and specs the wiki documents

## Lifecycle

- First added: 2026-09-11 — generic defaults only (caches, self-ignore)
- Significant changes: 2026-09-11 — project-specific entries added after stale detection fired on ~3500 build/archive files
- Significant changes: 2026-09-12 — `src-tauri/target/` added after a 3485-file flood (it matched no pattern); same day the wiki-context extension gained always-on BUILTIN_IGNORES merged in `loadIgnore()`, making this file additive-only

## Relationships

- the wiki overview ([../../overview.md](../../overview.md)) — the wiki documents this project's structure; `.wiki_ignore` keeps its staleness signal clean

## Source

- `.wiki_ignore` — the config itself
- `C:\Users\PTW\.pi\agent\extensions\wiki-context\index.ts` — BUILTIN_IGNORES merged inside `loadIgnore()`
