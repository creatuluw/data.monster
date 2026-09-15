---
type: System Overview
title: Overview
description: What this project contains and its structure.
timestamp: "2026-09-11T21:35:13.611Z"
---

# Overview

**Data Monster** is a desktop data-analysis application — "connect data, query, explore." It is built with **Tauri v2** (native desktop shell), a **SvelteKit + Svelte 5 + TypeScript + Tailwind CSS 4** frontend, and an embedded **DuckDB 1.1** engine living in a **Rust** backend. No server is required: the Rust process owns the DuckDB connection, and the webview frontend talks to it through Tauri `invoke` commands.

The core workflow flows through routes: **Connect** (`/connect`) ingests CSV/Parquet/JSON from disk or URL and browses/ingests remote PostgreSQL tables; **Preview** (`/preview`) detects columns and types; **Query** (`/query`) runs SQL (SELECT/CTAS/SHOW/DESCRIBE) with an editor, pagination, and an ingest modal; **Data** (`/data`) and **Table detail** (`/table/[name]`) manage and browse tables with tags/groups; **Analyst** (`/analyst`) chats with an LLM about the data (including local llama.cpp models); **Pages** (`/pages`) builds lightweight data pages from query results; **Labs** (`/labs`) is a 32-type chart catalog — one card per chart type, scaffolded placeholder-first after theunspokenpitch.com and built one at a time on SveltePlot over a shared chart fundament (heatmap and horizontal bar chart done, 30 still placeholders); and **Settings** (`/settings`) configures the LLM and exposes an internal-DB browser for the `d8a_monster_*` metadata tables. Everything persists in a user-selected workspace folder as `d8a_monster.duckdb` plus copied source files under `data/main/`.

Organization is split between product code and process artifacts. Product code lives in `src/` (frontend: routes, components, reusable chart canvases, Svelte 5 rune stores) and `src-tauri/` (Rust backend: per-domain command modules — files, queries, tables, labels, saved_queries, internal_db, postgres, local_llm, workspace, settings — plus state and utils). Process artifacts document how features are built: `.specs/` holds spec-driven feature specs with task logs (chart-lib, field-function-library, local-llm, tauri-migration), `.prds/` holds product requirement docs with interviews (reporting-dashboard-pages, table-relationships), `docs/` holds research notes, picasso.js chart examples, re-usable chart specs, and this wiki, and `prompts.md`/`opencode.json`/`.pi/` configure the AI-agent tooling used in development.

History and experimentation are deliberately quarantined. `.archive/` keeps superseded versions — including the original browser-only app (`data-monster-old`, DuckDB-WASM with a Node server) and chart-engine trials (echarts, svelteplot, observable) — while the nested `data.monster/` project generates the design-system documentation site, and `build/` is static-export output. `global_superstore.csv` at the root is the sample retail dataset used for demos and testing.
