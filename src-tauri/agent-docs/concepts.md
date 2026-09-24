# Concepts

> Read when: you want the 2-minute model of the app before authoring content.

- **Workspace** — this folder. Portable: copy it, git it, move it; the app opens it.
- **Source tables** — data ingested into `d8a_monster.duckdb` (CSV/Parquet/JSON/Postgres).
  Tables live in the DB, their files in `data/main/`. Owned by the app.
- **Page** — a report: one JSON document of rows → columns → blocks (chart/table/text).
  Rendered at /pages/<slug>. Declarative: the file is the page; the no-code editor and
  you edit the same document.
- **Master item** — a named, table-bound DuckDB expression (measure = aggregated number,
  dimension = grouping column) reused across charts by reference `{"ref": "<id>"}`.
- **Relationship** — a declared join between two tables (from table/column to
  table/column); the query compiler auto-JOINs through it so charts can use fields
  from related tables.
- **Saved query** — a named SQL file, runnable from the /query UI.
- **Connection** — a saved Postgres location. The secret URL lives in `.env`; the doc
  only references it by env-var name.

The app validates everything on load (never crashes on bad files) and hot-reloads all
open views within ~300ms of a file change — including changes you make.
