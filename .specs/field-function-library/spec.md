# Field Function Library — Query-Side Field Enrichment

## Feature Overview

Replace the automatic VIEW-based date enrichment pattern with a query-side function library. When a user clicks a table in the `/query` page sidebar, a drawer opens on the right showing the table's columns grouped by type. Each column type exposes a curated set of functions from a type-aware function library. The user selects a column and one or more functions to apply; the chosen functions are injected into the SQL query's SELECT clause as computed expressions alongside the original column.

This shifts enrichment from a permanent data-model transformation (VIEWs) to an explicit, user-controlled query-building action. Functions are applied only when and where the user chooses, keeping the underlying table unmodified.

## Success Criteria

1. Clicking a table in the query sidebar opens a right-side drawer showing all columns with their detected DuckDB types
2. Each column displays available functions based on its type category (date, numeric, text, etc.)
3. Selecting a function on a column adds a computed expression to the current query's SELECT clause
4. The generated SQL is valid DuckDB SQL and executes without errors
5. Multiple functions can be applied to the same column
6. Functions can be removed individually from the query
7. The function library is defined in a single source-of-truth file that maps DuckDB type patterns to available functions
8. All previous VIEW/`__base`/automatic enrichment code is removed cleanly

## Design Goals

**Primary (must):**
- Function library persisted in DuckDB with full CRUD management in settings
- Pre-populated with date functions (week end, month end, quarter end)
- Type-aware function filtering in the query page drawer
- Function selection injects computed columns into SELECT clause

**Secondary (nice to have):**
- Numeric functions (running total, percent of total)
- Text functions (uppercase, lowercase, length)
- Function preview showing generated SQL snippet before applying
- Import/export of function definitions

## User Experience

1. User navigates to `/query` and sees the table list in the left sidebar
2. User clicks a table name — the right-side drawer slides open showing the table's columns
3. Each column row shows: column name, type badge, and a "＋" button
4. Clicking "＋" reveals available functions for that column's type category
5. User clicks a function (e.g., "Week end" on a DATE column) — it gets added as active
6. The main query editor updates to include the computed expression: `date_trunc('week', "order_date") + INTERVAL 6 DAYS AS "order_date_week_end"`
7. User can click the active function tag to remove it
8. User closes the drawer and runs the query — computed columns appear in results

## Design Rationale

The previous VIEW approach modified the underlying data model, which meant every consumer (chart-lib, table viewer, query page) saw the enriched fields regardless of context. This was heavy-handed: it renamed tables, created `__base` copies, and required cleanup logic in drop/rename/refresh commands.

The query-side approach is lighter: the table stays untouched, functions are applied explicitly in the query builder, and only the `/query` page is affected. The function library is a simple mapping from type patterns to SQL expression templates, making it easy to extend without touching backend code.

Trade-off: chart-lib and table viewer no longer automatically show enriched fields. Users who want enrichments in those contexts must first create a table via `CREATE TABLE ... AS SELECT ...` with the functions applied.

## Constraints/Assumptions

- DuckDB SQL dialect for all function templates
- Function definitions stored in `d8a_monster_field_functions` internal table
- Function library loaded from DB on query page mount
- Settings management requires new Rust commands for CRUD on `d8a_monster_field_functions`
- Drawer shares visual patterns with existing `TableDrawer.svelte`
- Column types come from `getTableMeta()` (already available in query page)
- The query editor is a plain `<textarea>` — function expressions are injected as text
- Template placeholders `{column}` and `{alias}` are replaced at SQL generation time

## Functional Requirements

### FR-1: Function Library Data Store
The function library is a first-class system persisted in the DuckDB database. A new internal table `d8a_monster_field_functions` stores function definitions. A settings page section allows users to add, edit, and delete functions. Each function has: an id, display label, description, a list of DuckDB type patterns it applies to, and a SQL expression template using `{column}` and `{alias}` placeholders.

The library ships pre-populated with date functions:
- **Week end**: `date_trunc('week', {column}) + INTERVAL 6 DAYS` (last day of the week)
- **Month end**: `(date_trunc('month', {column}) + INTERVAL 1 MONTH) - INTERVAL 1 DAY` (last day of the month)
- **Quarter end**: `(date_trunc('quarter', {column}) + INTERVAL 3 MONTHS) - INTERVAL 1 DAY` (last day of the quarter)

Each function's `appliesTo` field specifies which DuckDB type patterns it matches (e.g., `["DATE", "TIMESTAMP%"]`), enabling the UI to show only relevant functions per column type.

Acceptance: `d8a_monster_field_functions` table in schema with pre-populated date functions. `src/lib/field-functions/library.ts` exports helpers to load, filter by type, and generate SQL from function definitions.

### FR-2: Function Library Settings Page
A new section in `/settings` (or a dedicated `/settings/field-functions` page) lets users manage the function library. The UI shows a list of all defined functions with their label, description, and applicable types. An "Add function" button opens a form with fields: label, description, SQL template, and type pattern selector. Each function row has edit and delete actions.

Acceptance: Users can view, add, edit, and delete functions from settings. Changes are persisted to `d8a_monster_field_functions`. Pre-populated functions are visible on first load.

### FR-3: Column Drawer in Query Page
Clicking a table in the query page sidebar opens a right-side drawer displaying the table's columns. Each column shows its name and type badge. Clicking a column name (or a "＋" button) reveals the available functions from the library that match the column's type. The drawer uses the existing drawer animation pattern from `TableDrawer.svelte`.

Acceptance: Drawer opens on table click, shows columns from `getTableMeta()`, closes on X button or overlay click. Functions are filtered by column type.

### FR-4: Function Selection Per Column
Within the drawer, each column's expanded view lists the functions available for its type. Clicking a function toggles it on/off for that column. Active functions show as tags below the column name with a remove (×) button. Multiple functions can be active on the same column.

Acceptance: Toggling a function adds/removes it from the active set for that column. Visual state updates immediately.

### FR-5: SQL Injection into Query Editor
When functions are selected/deselected, the main query editor's SELECT clause is regenerated. For columns with active functions, the original column reference is followed by the function SQL expressions. If a table-click template query is active, the entire SELECT list is regenerated. If the user has hand-written SQL, computed columns are appended to the SELECT clause.

Example: selecting "Week end" and "Month end" on `order_date` produces:
```sql
SELECT
  "order_date",
  date_trunc('week', "order_date") + INTERVAL 6 DAYS AS "order_date_week_end",
  (date_trunc('month', "order_date") + INTERVAL 1 MONTH) - INTERVAL 1 DAY AS "order_date_month_end"
FROM "orders"
LIMIT 100;
```

Acceptance: Generated SQL is valid DuckDB SQL. Active function expressions appear in the editor immediately.

### FR-6: Remove Previous VIEW Implementation
Cleanly remove all code related to the automatic date enrichment VIEW pattern: `d8a_monster_date_fields` table, `date_fields.rs` module, VIEW creation/destruction logic in `tables.rs` and `database.rs`, `refresh_table_view` command, `__base` table filtering, `reset_all_data` command, frontend `DateFieldConfig` types and API bindings, date field toggles in `TableDrawer.svelte` and preview page, and the danger zone reset button in settings.

Acceptance: No references to `d8a_monster_date_fields`, `__base`, `refresh_table_view`, `reset_all_data`, `DateFieldConfig`, `isDateColumn`, or `buildTableSelect` remain. Tables are plain DuckDB tables.

## Edge Cases

- Table has no columns → drawer shows empty state
- Column type matches no functions → "No functions available" message
- All functions deselected → column reverts to plain reference in SELECT
- Table/column names with special characters → properly double-quoted in generated SQL
- User manually edits query after function injection → no conflict; drawer reflects last known state
- Function deleted from settings while active on a column → gracefully handle (function removed from active set)
- Very wide tables (100+ columns) → drawer scrolls, performance acceptable
- Function template contains invalid SQL → validation on save in settings
