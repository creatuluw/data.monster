# Data Monster Tauri Migration Spec

## Feature Overview

Data Monster is currently a browser-only SvelteKit app that uses DuckDB WASM for in-browser SQL analytics. All data persists in IndexedDB (Parquet buffers) and localStorage (recipes, tags, queries). This data is lost on browser data clearing, browser reinstall, or when switching browsers.

This migration converts Data Monster into a Tauri desktop application. DuckDB will run natively in the Rust backend, persisting all data to a user-chosen workspace folder on disk. The frontend communicates with the Rust backend via Tauri's `invoke()` API. All metadata (tags, groups, recipes, saved queries) moves out of localStorage and into DuckDB internal tables, making everything SQL-queryable and disk-persistent.

On first launch, the user picks a workspace folder. All subsequent launches reopen that folder automatically. The DuckDB database file and source data files live in that folder, surviving browser resets, reinstalls, and machine migrations.

## Success Criteria

- User can pick a workspace folder on first launch; data persists across app restarts and browser resets
- All current features work identically: connect files/URLs, preview with column overrides, query SQL, ingest tables, save queries, AI analyst chat, charts
- Tags and groups for tables are editable via the UI and persisted in DuckDB internal tables
- No localStorage or IndexedDB is used for app data — everything is in DuckDB on disk
- Internal metadata tables use the `d8a_monster_` prefix (not `_warphead_`)

## Design Goals

### Primary (Must)
- Tauri v2 desktop app with native DuckDB (Rust `duckdb` crate with `bundled` feature)
- User-chosen workspace folder on first launch, auto-reopened on subsequent launches
- All current features preserved with identical UX
- Tags and groups on tables, persisted in DuckDB
- `d8a_monster_` prefix for all internal metadata tables

### Secondary (Nice to Have)
- File watcher for workspace folder changes
- Export workspace as portable folder

## User Experience

On first launch, a welcoming modal asks the user to choose a workspace folder. After picking, DuckDB initializes at `<workspace>/d8a_monster.duckdb`. Source files are stored in `<workspace>/data/`. The app opens immediately. On every subsequent launch, the same workspace reopens automatically.

In the `/tables` view, each table card shows tags and group. Clicking the edit icon opens an inline editor to add/remove tags and set a group. Tag chips and group labels appear in the card footer. A filter bar above the card grid lets users filter by tag or group.

## Design Rationale

Migrating from DuckDB WASM to native DuckDB via Tauri gives us true filesystem persistence — the `.duckdb` file on disk is the single source of truth. This matches the pattern proven in the `docs/src-tauri/` codebase. Moving metadata from localStorage into DuckDB tables keeps everything queryable and co-located. The `d8a_monster_` prefix replaces the legacy `_warphead_` naming.

## Constraints/Assumptions

- Tauri v2 with `tauri-plugin-dialog` for folder picker
- Rust `duckdb` crate v1.1 with `bundled` feature (no system DuckDB dependency)
- Windows primary target (current dev environment), macOS/Linux supported by Tauri
- No SSR — app remains client-side only (`export const ssr = false`)
- The LLM analyst API proxy remains a SvelteKit server endpoint (no change)
- Source files loaded into DuckDB are also saved to `<workspace>/data/` for re-ingestion on restart
- The workspace folder path is persisted in Tauri's app-local storage (`Store` plugin or a config file)

## Functional Requirements

### FR-1: Workspace Folder Selection
On first launch, the app shows a folder picker dialog. The chosen folder becomes the workspace. The folder path is persisted in Tauri app config. On subsequent launches, the app reopens the same workspace automatically. A "Switch workspace" option in the app header allows changing folders.

**Acceptance Criteria:**
- Folder picker dialog appears on first launch
- Workspace path persisted to `<app_data_dir>/workspace.json`
- On restart, same workspace opens automatically
- "Switch workspace" menu item available

### FR-2: DuckDB Initialization with Workspace
`initialize_duckdb` command opens/creates a DuckDB database at `<workspace>/d8a_monster.duckdb`. Creates internal metadata tables on first run. Existing tables from previous sessions are automatically available.

**Acceptance Criteria:**
- DuckDB file created at `<workspace>/d8a_monster.duckdb`
- Internal tables created if they don't exist
- Existing data loads automatically on restart
- WAL corruption handled with retry/recovery logic

### FR-3: File Loading (CSV, Parquet, JSON, JSONL)
`load_csv_file`, `load_parquet_file`, `load_json_file` commands ingest files into DuckDB. Source files are copied to `<workspace>/data/main/` for persistence. Table metadata registered in `d8a_monster_table_metadata`.

**Acceptance Criteria:**
- Files loaded via open dialog
- Source file copied to workspace data folder
- Table registered in metadata with type 'source'
- Row count returned after loading
- Tables survive app restart

### FR-4: SQL Query Execution
`execute_query` command runs arbitrary SQL. SELECT queries return JSON with columns and data. DML queries (INSERT, UPDATE, DELETE, CREATE, DROP, ALTER) execute directly.

**Acceptance Criteria:**
- SELECT queries return `{columns: string[], data: any[]}`
- DML queries execute and confirm
- Query cancellation supported via `cancel_query`
- Large result sets handled without memory issues

### FR-5: Table Management
`list_tables`, `drop_table`, `create_table_from_query`, `save_query_as_table` commands. Tables registered in `d8a_monster_table_metadata`. Drop removes from both DuckDB and metadata.

**Acceptance Criteria:**
- List returns all user tables from `information_schema`
- Drop removes table and metadata entry
- Create from query registers as type 'model'
- Metadata tracks creation SQL

### FR-6: Table Tags and Groups
`d8a_monster_table_labels` internal table stores tags (comma-separated) and group per table. `save_table_labels`, `get_table_labels`, `get_all_tags`, `get_all_groups` commands. UI shows tags and groups on table cards with inline editing and filter bar.

**Acceptance Criteria:**
- Tags and group persisted in DuckDB `d8a_monster_table_labels` table
- `save_table_labels` upserts tags and group for a table
- `get_table_labels` returns tags array and group for a table
- `get_all_tags` returns distinct tags across all tables
- `get_all_groups` returns distinct groups across all tables
- TableOverview shows tags and group on each card
- Inline edit UI for adding/removing tags and setting group
- Filter bar to filter tables by tag or group
- Data persists across app restarts

### FR-7: Saved Queries
`d8a_monster_saved_queries` internal table. `save_query`, `update_saved_query`, `delete_saved_query`, `list_saved_queries` commands. Same UI as current (save modal with name/description/tags, filtered list).

**Acceptance Criteria:**
- Saved queries persisted in DuckDB (not localStorage)
- CRUD operations via Tauri commands
- Tags filterable in UI
- Data persists across app restarts

### FR-8: AI Analyst Chat
The `/api/analyst` server endpoint remains unchanged (SvelteKit server route). The analyst store's `getTableSchemas()` method calls Tauri commands instead of DuckDB WASM directly.

**Acceptance Criteria:**
- Analyst chat works identically to current version
- SQL extraction and auto-execution via Tauri `execute_query`
- SSE streaming from LLM API unchanged

## Edge Cases

- **WAL corruption**: DuckDB WAL file can corrupt if app is force-killed. Recovery: detect WAL error, delete WAL, retry open (from `docs/src-tauri/database.rs` pattern).
- **Locked database**: During hot-reload, the DuckDB file may be locked. Retry with backoff up to 10 attempts.
- **Workspace folder deleted**: If the workspace folder is deleted while app is running, show error and prompt to choose new folder.
- **Workspace folder moved**: If the workspace folder is moved/renamed on disk, detect on next launch and prompt for new location.
- **Empty workspace**: First launch with empty folder creates the `.duckdb` file and `data/` subdirectory.
- **Orphaned metadata**: Metadata entries for dropped tables should be cleaned up on init (from `docs/src-tauri` `cleanup_orphaned_metadata` pattern).
- **Unsupported file types**: Only CSV, Parquet, JSON, JSONL supported. Reject others in file dialog.