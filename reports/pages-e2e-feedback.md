# E2E Test Report — /pages page with master measures & dimensions

**Session**: 2026-09-22 · **Tester**: agent over CDP (real + synthetic input) · **App**: dev instance, Tauri + SvelteKit + DuckDB

## What was tested

End-to-end creation of a new report page built on the semantic (master-item) layer:

1. **Dataset**: ingested `global_superstore.csv` (51,290 rows) as `superstore` via `/query` CTAS.
2. **Master measures** (via `/data?tab=measures` → Add measure): `Total Sales` = `SUM(sales)` (usd), `Total Profit` = `SUM(profit)` (usd), `Profit Ratio` = `SUM(profit) / NULLIF(SUM(sales), 0)` (pct).
3. **Master dimension**: `Order Year` = `year(order_date)`.
4. **Page**: "Sales Overview" created under `/pages` → `/pages/sales-overview`.
5. **Blocks**: Bar chart wired to ⭐ Order Year (dimension) + ⭐ Total Sales (measure); Table block on `superstore` (50 rows).
6. **Persistence**: reload-verified — blocks, roles and data survive; chart draws 5 bars (0→2.6M axis), table renders 50 live rows.

## Bugs found & fixed during the session

| # | Severity | Bug | Fix |
|---|----------|-----|-----|
| 1 | **P1** | Master-item refs in chart roles crashed every chart query: `no relationship path from "superstore" to "undefined"`. Rust serializes items with `tableName`; the frontend `MasterItem` expects `table` — the type assertion hid the mismatch, so `resolveItems` pushed `undefined` into the join planner. | `central-api.ts` — map `tableName → table` in `listMasterItems` (single choke point). |
| 2 | **P1** | Expression-based master dimensions failed compilation: `unknown column "year(order_date)"` — `checkColumn` treats a dimension's `col` as a bare column name and `ident()` would quote the expression. Master measures compile raw already; dimensions from the validated ItemEditor are the same trust level. | `query/compile.ts` `dimSelect` — on column-lookup miss, compile the dimension as a parenthesized expression (`ponytail:` note documents the trust ceiling). |

Both fixes: `svelte-check` 0 errors.

## Root cause of the app hangs (#3) — diagnosed & fixed

The wedge was a **poisoned DuckDB connection** (Windows + duckdb-rs, matching [duckdb-rs issue #209](https://github.com/duckdb/duckdb-rs/issues/209)): after certain failed/interrupted statements, every later operation on the connection fails with "resource deadlock would occur" until process restart. Trigger: reload storms — webview unload fires `shutdownDuckdb` while statements are in flight.

**Fix** (`queries.rs`): `execute_query` detects the poisoning error and recovers in-process — drops + reopens the connection (workspace path from state, schema re-init) and retries the query once. A wedge is now a ~1s transparent blip instead of an app restart.

## Bugs found — resolution status (same session)

| # | Severity | Finding | Status |
|---|----------|---------|--------|
| 3 | **P1** | **App hangs (backend wedge).** Query validation stuck at "checking…" forever, Save silently no-ops, no timeout/cancel anywhere. Root cause of the stall: DuckDB in-process deadlock under reload storms (known learning); UI had no defenses. | **Fixed (frontend defenses):** ExprEditor validation invoke races a 15s timeout → "validation timed out" surfaces instead of infinite "checking…"; `writeInvoke` in central-api races 20s timeouts so hung saves surface; save failures render via the existing saveError path. Backend deadlock itself still needs investigation (see priorities). |
| 4 | **P2** | ~~Per-keystroke autocomplete query flood.~~ **Investigated — not a bug:** ExprEditor suggestions are computed locally (no invokes while typing); validation is debounced 500ms + serialized + stale-guarded; the /query editor is a plain bind textarea. The wedge was the known DuckDB deadlock amplified by parallel automation sessions. | Closed — no fix needed |
| 5 | **P2** | **First invoke after page load intermittently fails** ("IPC custom protocol failed … postMessage") and the triggering action is silently lost. | **Fixed:** `writeInvoke` in central-api retries once after 300ms (landing on the postMessage path) and times out after 20s. |
| 6 | **P2** | "Create page" doesn't navigate to the new editor. Root cause: savePage's first-invoke failure aborted the handler before `goto` (#5). | **Fixed** via #5 retry; CDP-verified: creating a page now lands on `/pages/<slug>`. |
| 7 | **P2** | Save button "Saved" state unreliable. Investigation: the title flip is transient (2s) and truthful — the observed failures were the hung invoke (#3) and poll timing. | **Verified truthful** + hardened by #3/#5. |
| 8 | **P3** | ExprEditor autocomplete popup can overlap the Save button and swallow the click (inserted a suggestion instead of saving). | **Fixed:** global pointerdown handler closes suggestions when clicking outside the editor shell; CDP-verified the Save click now lands. |
| 9 | **P3** | CTAS/DDL runs give no success confirmation. | **Fixed:** results empty-state shows "✓ Table created successfully." / "✓ Query executed successfully" for mutation results. (Non-CTAS DDL remains blocked by the existing "Only SELECT queries" guard — by design.) |

## What worked well

- The **semantic-layer concept holds end to end**: master items → role picker → chart roles → resolved SQL, once the two plumbing bugs were fixed.
- **Code mode** (direct PageDoc JSON editing with apply-on-switch) is an excellent power surface — it unblocked the test when the UI path was flaky, and `validatePageDoc` caught structural mistakes.
- The **needsSetup gate** behaves as designed: blocks start empty, render only when roles are satisfied.
- **Spawn persistence** (add row/component writes through instantly) works when IPC is healthy.
- Table block renders 50 live rows with zero configuration beyond table pick.

## Suggested priorities

1. Investigate the backend DuckDB deadlock properly (hangs under reload/query storms — known learning) so wedges stop happening at all; the frontend now defends (timeouts + surfaced errors).
2. Consider surfacing rows-affected counts for DML runs (#9 extension).
3. Keep an eye on the IPC first-invoke failure (#5) — the retry masks it, but the underlying custom-protocol flake may deserve a Tauri issue.
4. Popover overlap (#8) and run feedback (#9) as polish.
