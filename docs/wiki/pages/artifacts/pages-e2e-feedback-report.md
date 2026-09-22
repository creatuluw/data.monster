---
type: Artifact
title: Pages E2E feedback report
description: "E2E test report for the /pages report-page flow built on the semantic (master-item) layer: `reports/pages-e2e-feedback.md`, produced 2026-09-22 by driving the r"
tags: [central-charts, e2e, cdp, report, bug-findings, resolved]
timestamp: "2026-09-22T11:36:40.975Z"
---

# Pages E2E feedback report

E2E test report for the /pages report-page flow built on the semantic (master-item) layer: `reports/pages-e2e-feedback.md`, produced 2026-09-22 by driving the real dev app over CDP. **Same-day follow-up session resolved all 7 open findings** — 5 fixed in code, 2 closed as non-bugs (report updated in place).

## What it documents

- The full end-to-end flow works once two P1 plumbing bugs were fixed: ingest via /query → master measures/dimensions via /data → page + blocks via /pages → persisted and reload-verified (bar chart "Sales by Order Year", 5 bars; table, 50 rows).
- **Fixed during the first session** (see [tableName/table + raw-compile learning](../../learnings/ref-based-master-items-tablename-table-mismatch-broke-all-re.md)): (1) Rust `tableName` vs frontend `table` — mapped in [central-api (frontend invoke client)](../entities/central-api-frontend-invoke-client.md) `listMasterItems`; (2) expression master dimensions rejected by the query compiler — now compile raw in `query/compile.ts`.
- **The 7 open findings — all resolved 2026-09-22** (CDP-verified, `npm run check` clean):
  - #3 + #5 (P1/P2): hung invokes now time out and surface — ExprEditor validation 15s, writes 20s via `writeInvoke` with one 300ms retry. See [Timeout and retry defend against hung IPC](../../decisions/timeout-and-retry-defend-against-hung-ipc.md).
  - #7 (P2): closed as already-truthful — the Save button was outcome-driven; the 2s "Saved" flip is transient by design, polls just missed it.
  - #4 (P2): closed as non-bug — the per-keystroke autocomplete flood doesn't exist (see [ExprEditor suggestions are computed locally](../../learnings/expreditor-suggestions-are-computed-locally.md)); the real hang is the DuckDB in-process deadlock.
  - #6 (P2): "Create page" now lands in the editor — the flow always tried; the #5 IPC race was aborting it before `goto`. Error surfacing added.
  - #8 (P3): ExprEditor suggestions close on outside `pointerdown` — a Save click can no longer be swallowed and turned into a suggestion insert.
  - #9 (P3): CTAS runs show "✓ Table created successfully." instead of a misleading empty result.
  - **Top follow-up**: the backend DuckDB deadlock itself is still open — the frontend now defends (timeouts + retry), but the deadlock needs its own investigation.

## Details

- **Format**: Markdown report with tested-flow walkthrough, bug tables (fixed + open), what-worked-well, and priority ordering; now updated with per-bug resolution status.
- **Location**: `reports/pages-e2e-feedback.md`
- **Generated from**: a CDP-driven session against the real dev app (ingest → master items → page build → reload verification), with `src/lib/central-api.ts` + `src/lib/charts/query/compile.ts` patched mid-session; plus the follow-up fix session (7 CDP probe scripts `reports/step-*.mjs`).
- Companion session files: `reports/cdp-driver.mjs`, `reports/e2e-log.txt`.

## Source

- `reports/pages-e2e-feedback.md` — the report itself
- `src/lib/central-api.ts`, `src/lib/components/charts/ExprEditor.svelte`, `src/lib/db-operations.ts`, `src/routes/query/+page.svelte` — the fix sites
