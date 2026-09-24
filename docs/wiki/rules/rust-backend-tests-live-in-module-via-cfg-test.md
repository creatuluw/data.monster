---
type: Rule
title: "Rust backend tests live in-module via #[cfg(test)]"
description: The rule
tags: [rust, testing, tdd, backend, convention]
timestamp: "2026-09-22T13:57:08.172Z"
---

# Rust backend tests live in-module via #[cfg(test)]

## The rule

Tests for Rust backend code are written **in the same file**, inside a `#[cfg(test)] mod tests` block at the bottom — no separate test files, no test crate. This is the mirror of the frontend rule ("keep test files and vitest imports out of src/"): the frontend keeps tests out of `src/`, the Rust backend keeps them in-module.

## When it applies

Any new or modified module under `src-tauri/src/` that gets unit tests.

## Rationale

- `cargo test` compiles `#[cfg(test)]` blocks together with the module, so a test run also proves the module itself compiles into the suite — a separate-file layout doesn't give you that for free.
- House pattern already in place: `commands/database.rs` (line ~354) and `commands/dm_store.rs` (line ~174, shipped 2026-09-22 with 11 tests as task `files-001`) both use it.
- TDD fits naturally: write RED tests in the in-module block first, watch them fail, then implement.

## Gotcha

Guarantees that need fault-injection seams (e.g. crash *between* temp-write and rename in an atomic write) are **not** unit-testable under this pattern — state the guarantee in the module doc-comment instead of writing theater tests. Documented ceilings beat fake coverage.
