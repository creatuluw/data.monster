# Data Monster

A desktop data analysis application. Connect data, query, explore.

Built with **Tauri v2**, **SvelteKit**, **DuckDB**, and **TypeScript**. Runs as a native desktop app with an embedded DuckDB engine — no server required.

## Features

- **File ingestion** — Import CSV, Parquet, and JSON files from disk or URL
- **PostgreSQL connect** — Browse and ingest tables from remote PostgreSQL databases
- **SQL query runner** — Write and run SQL queries with paginated results, column editing, and preview limiting
- **Table management** — Browse, rename, label, group, and drop tables with a visual explorer
- **Saved queries** — Save, organize, and reload frequently used SQL queries
- **AI analyst** — Chat with an LLM about your data (configurable endpoint)
- **Pages** — Build lightweight data pages from query results
- **Internal DB browser** — Inspect and edit internal metadata tables (`d8a_monster_*`) from Settings
- **Persistent workspace** — All data stored on disk in a DuckDB file; survives restarts

## App Flow

```
Homepage (/)
├── Connect (/connect)
│   ├── Upload CSV / Parquet / JSON from disk
│   ├── Download from URL
│   └── Connect to PostgreSQL
│       └── Select tables → generate ingest SQL
│
├── Preview (/preview)
│   ├── Column detection & type overrides
│   ├── Row count & sample preview
│   └── Continue → generates SQL, opens Query
│
├── Query (/query)
│   ├── SQL editor with syntax highlighting
│   ├── Run queries (SELECT, WITH, SHOW, DESCRIBE, EXPLAIN)
│   ├── CREATE TABLE AS SELECT support
│   ├── Ingest modal (table name, group, tags)
│   ├── Save / load saved queries
│   └── Results table with pagination
│
├── Data (/data)
│   ├── Table grid with search & filter
│   ├── Tag / group filtering
│   ├── Table drawer (rename, source, refresh, delete)
│   └── Click → Table detail view
│
├── Table detail (/table/[name])
│   ├── Paginated data browser
│   ├── Column types & metadata
│   └── Source info & refresh from source
│
├── Analyst (/analyst)
│   └── LLM chat interface for data questions
│
├── Pages (/pages)
│   └── Build lightweight data pages
│
└── Settings (/settings)
    ├── LLM configuration (API URL, key, model)
    └── Internal DB (/settings/internal-db)
        └── Browse / edit / delete d8a_monster_* metadata tables
```

## Architecture

```
src/                          SvelteKit frontend
├── routes/                   Page routes (SvelteKit file-based routing)
├── lib/
│   ├── db-operations.ts      Tauri invoke wrappers & types
│   ├── stores/               Reactive state (app.svelte.ts, analyst.svelte.ts)
│   ├── components/           Shared UI components (TagInput, TableOverview, etc.)
│   └── charts/               Charting utilities
└── app.css                   Global styles & design tokens

src-tauri/                    Rust backend (Tauri v2)
├── src/
│   ├── main.rs               App entry, command registration
│   ├── state.rs              Global state (DuckDB connection, workspace path)
│   └── commands/
│       ├── database.rs       DB initialization, schema setup
│       ├── files.rs          File download, preview, CSV/Parquet/JSON loading
│       ├── queries.rs        SQL execution (SELECT & DML)
│       ├── tables.rs         List, drop, rename, create tables
│       ├── labels.rs         Table tags & groups
│       ├── saved_queries.rs  Save/load/update/delete queries
│       ├── internal_db.rs    Internal metadata table CRUD
│       ├── postgres.rs       PostgreSQL connection & ingestion
│       ├── settings.rs       App settings (LLM config)
│       ├── folders.rs        Workspace folder management
│       └── workspace.rs      Workspace selection & path
└── Cargo.toml                Rust dependencies (duckdb, reqwest, parking_lot)

build.rs                      Windows stack size (32MB) for DuckDB
```

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [Rust](https://rustup.rs/) (stable, latest)
- [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/) (Windows: Visual Studio Build Tools with C++ workload)

### Install

```bash
git clone https://github.com/creatuluw/data.monster.git
cd data.monster
npm install
```

### Development

```bash
# Run the app in dev mode (builds Rust + starts SvelteKit dev server)
npm run dev
```

Or manually:

```bash
# Frontend only (no Tauri backend)
npm run dev:frontend

# Full Tauri dev
npx tauri dev
```

### Build for Production

```bash
npx tauri build
```

The installer will be in `src-tauri/target/release/bundle/`.

### Clean Rebuild

```bash
cargo clean --manifest-path src-tauri/Cargo.toml
npm run dev
```

Required when changing `build.rs` linker flags (e.g., stack size).

## Workspace Structure

When you select a workspace folder, Data Monster creates:

```
your-workspace/
├── d8a_monster.duckdb           Main database (all tables, metadata)
├── d8a_monster.duckdb.wal       Write-ahead log (auto-managed)
└── data/
    └── main/                    Ingested data files (CSV, Parquet, JSON copies)
```

## Tech Stack

| Layer      | Technology                          |
|------------|-------------------------------------|
| Desktop    | Tauri v2                            |
| Frontend   | SvelteKit + Svelte 5 + TypeScript   |
| Styling    | Tailwind CSS 4                      |
| Database   | DuckDB 1.1 (embedded, bundled)      |
| Backend    | Rust                                |
| Icons      | Lucide                              |
| Charts     | Chart.js                            |
