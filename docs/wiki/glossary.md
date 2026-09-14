---
type: Glossary
title: Glossary
description: Key terms for this project.
timestamp: "2026-09-11T21:35:13.611Z"
---

# Glossary

| Term | Definition |
|------|------------|
| Tauri | Desktop app framework (v2) wrapping the SvelteKit webview in a native Rust process; the Rust side owns DuckDB and exposes commands via `invoke`. |
| DuckDB | Embedded analytical SQL database (1.1, bundled in the Rust build) storing all ingested data and metadata in a single file. |
| Workspace | User-selected folder where Data Monster creates `d8a_monster.duckdb`, its WAL, and `data/main/` copies of ingested files; survives restarts. |
| `d8a_monster.duckdb` | The main DuckDB database file holding all user tables and internal metadata tables. |
| `d8a_monster_*` tables | Internal metadata tables (labels, groups, saved queries, settings, field functions) editable via Settings → Internal DB. |
| Tauri command | Rust function in `src-tauri/src/commands/` registered in `main.rs` and callable from the frontend through `invoke('command_name', …)`. |
| db-operations | `src/lib/db-operations.ts` — typed frontend wrappers for every Tauri command plus shared result/error types. |
| Ingest | Turning a source (file/URL/Postgres table/CTAS query) into a persistent DuckDB table, optionally with a group and tags. |
| Connect | Route `/connect` for uploading files, downloading from URLs, and connecting to PostgreSQL. |
| Preview | Route `/preview` showing column detection, type overrides, and row samples before generating ingest SQL. |
| CTAS | `CREATE TABLE AS SELECT` — supported way to materialize query results as a new table, with the ingest modal. |
| Labels | Tags and groups attached to tables for organizing/filtering in the Data explorer (persisted in `d8a_monster_*`). |
| Saved queries | SQL queries stored with a name in internal metadata, reloadable from the Query page. |
| Analyst | `/analyst` chat interface that asks a configured LLM questions about the workspace data. |
| Local LLM | llama.cpp integration (`llama-cpp-2` in Rust) letting the analyst run against a local model instead of a remote API. |
| Pages | `/pages` — lightweight data pages (KPIs, charts) built from query results. |
| Chart library | Reusable, configurable chart components (`src/lib/charts/`) — bar/line/area/pie/donut/scatter canvases with click-to-filter. |
| LayerChart | Svelte-native charting library (primary engine for the chart library). |
| Picasso.js / SveltePlot / Unovis / ECharts / Observable Plot | Alternative chart engines evaluated and prototyped in Labs and `.archive/`. |
| Labs | `/labs` experimental workspace where chart engines and components are tested before promotion. |
| Field functions | Saved column transformations (function library) speced in `.specs/field-function-library`, managed under Settings. |
| Internal DB browser | Settings → Internal DB UI for inspecting/editing/deleting `d8a_monster_*` metadata rows. |
| WAL | Write-ahead log (`d8a_monster.duckdb.wal`) auto-managed by DuckDB. |
| Runes | Svelte 5 reactivity primitives (`$state`, `$derived`, `$effect`, `$props`) used in all stores and components. |
| `.specs` | Spec-driven feature folders: `spec.md` plus `tasks.json`/`tasks-log.json` task tracking per feature. |
| `.prds` | Product requirement documents with interview transcripts for upcoming initiatives. |
| `.archive` | Quarantined superseded code: the old DuckDB-WASM web app, worker-scaffolding experiments, and old chart trials. |
| OKF | Open Knowledge Format — the frontmatter convention used by `docs/wiki/`. |
| OpenCode / pi | AI coding-agent tools configured here (`opencode.json`, `.opencode/`, `.pi/`) used during development. |
| Global Superstore | Sample retail dataset (`global_superstore.csv`) used for demos and testing. |
