---
type: Learning
title: "Ref-based master items: tableName/table mismatch broke all ref charts; expression dims need raw compile"
description: Discovered 2026-09-22 during the /pages E2E session (reports/pages-e2e-feedback.md).
tags: [central-charts, master-items, bug, tauri, ipc]
timestamp: "2026-09-22T11:01:15.746Z"
---

# Ref-based master items: tableName/table mismatch broke all ref charts; expression dims need raw compile

Discovered 2026-09-22 during the /pages E2E session (reports/pages-e2e-feedback.md).

Two plumbing bugs broke EVERY ref-based master-item chart, while raw-field charts (e.g. the existing Revenue page) worked fine:

1. **`tableName` vs `table`**: Rust `items.rs` serializes `d8a_monster_items.table_name` as `tableName`, but the frontend `MasterItem` type declares `table` — the `invoke<{ items: MasterItem[] }>` type assertion hid the mismatch. `resolveItems` (items.ts) then did `involved.add(item.table)` with `undefined` → `buildJoins` threw `no relationship path from "superstore" to "undefined"`. Fixed at the single choke point: `central-api.listMasterItems` maps `tableName → table`.

2. **Expression master dimensions don't survive `checkColumn`**: `resolveItems` resolves a ref dimension to `{ col: item.expr, label }` (e.g. col = `year(order_date)`), but `dimSelect` in `query/compile.ts` validated `col` as a bare column and `ident()` would quote the expression into a column name. Master measures already compile raw, so dimensions now do too: on column-lookup miss, `dimSelect` compiles `(<expr>)` raw (ponytail-commented trust ceiling — these expressions are user-authored and validated at creation in ExprEditor, same trust level as measures).

Debugging technique that cracked it: the chart card's own error text (`no relationship path…`) was findable via targeted DOM dumps of the card region — the card rendered a silent error message instead of a plot, with no console error.
