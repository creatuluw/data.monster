---
type: Learning
title: duckdb plain-bundled lacks static JSON extension — dynamic auto-load heap-corrupts on Windows
description: Symptom
tags: [duckdb, windows, heap-corruption, extensions, cargo]
timestamp: "2026-09-22T12:44:59.260Z"
---

# duckdb plain-bundled lacks static JSON extension — dynamic auto-load heap-corrupts on Windows

## Symptom

`cargo test` died mid-run with `STATUS_HEAP_CORRUPTION (0xc0000374)` on any test touching `read_json_auto` — even a bare `execute_batch("CREATE TABLE t AS SELECT * FROM read_json_auto(...)")` on an in-memory connection. Isolated to the JSON extension path; CSV/parquet tests were fine.

## Root cause

The dependency is `duckdb = { version = "1.1", features = ["bundled"] }` — **plain `bundled` does NOT statically link the JSON extension**. On first `read_json_auto` call DuckDB auto-installed `json.duckdb_extension` (a dynamic DLL, found in `~/.duckdb/extensions/v1.5.2/windows_amd64/`) and loaded it at runtime. A dynamically-loaded extension against statically-linked DuckDB on Windows frees memory across CRT heap boundaries → heap corruption, process death. This also silently affected the app's JSON ingest (`load_json_file`, `get_file_columns`, `preview_file` with .json), not just tests.

## Fix

Enable the crate's `json` feature — it statically compiles the JSON extension into the binary (same treatment as parquet):

```toml
duckdb = { version = "1.1", features = ["bundled", "json"] }
```

Also deleted the downloaded DLL (`~/.duckdb/extensions/v1.5.2/windows_amd64/json.duckdb_extension*`) so nothing can silently fall back to dynamic loading.

## Generalization

Any `read_*` table function from a non-core extension (json, postgres_scanner, httpfs…) will auto-download-and-dynamically-load on first use. Prefer the crate's static feature for extensions the app needs at its core. `postgres.rs` deliberately does `INSTALL postgres; LOAD postgres` (dynamic by design) — if Postgres ingest ever heap-corrupts the same way, this is why.

Verified: DuckDB-rs 1.10502.0 = DuckDB v1.5.2 bundled. The version scheme `1.MAJOR_MINOR_PATCH.x` started at DuckDB v1.5.0.
