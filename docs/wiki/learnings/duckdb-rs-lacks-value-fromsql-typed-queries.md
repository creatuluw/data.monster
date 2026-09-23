---
type: Learning
title: "duckdb-rs lacks Value: FromSql — typed queries, no generic rows"
description: "Hit in `files-006` while building the `dm_store` export path (2026-09-22): a generic row-deserialization helper over `duckdb-rs` queries is impossible because t"
tags: [rust, duckdb, backend, gotcha]
timestamp: "2026-09-22T14:41:49.629Z"
---

# duckdb-rs lacks Value: FromSql — typed queries, no generic rows

Hit in `files-006` while building the `dm_store` export path (2026-09-22): a generic row-deserialization helper over `duckdb-rs` queries is impossible because the crate does not implement `FromSql` for its `Value` type — so "map any row generically" won't compile.

**Consequence**: write boring, explicitly-typed queries per table (query → read typed columns off the row) instead of reaching for a generic row→struct mapper. The deleted generic helper was the second attempt; per-table typed queries passed the suite first try.

**Applies to**: all Rust backend code reading DuckDB rows (`src-tauri/src/commands/*`).
