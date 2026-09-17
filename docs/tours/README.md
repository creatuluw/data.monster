# data.monster — feature tours

Interactive step-by-step demos of the real app UI, built with the
[`app-tour-demo`](E:/skills.te9.dev/app-tour-demo/SKILL.md) skill: each tour
captures the actual running app as DOM snapshots (via CDP against the real
Tauri app) and replays them with an animated cursor, subtitles and result
checkmarks. Pixel-accurate by construction — nothing is mocked or redrawn.

Open any `*-tour.html` directly from disk (double-click; works from `file://`).

## The eight feature tours

| Feature | Tour | What it shows |
|---------|------|---------------|
| Connect | [connect-tour.html](connect-tour/connect-tour.html) | The three ways data enters: file picker, live URL download into Preview, PostgreSQL hookup |
| Preview | [preview-tour.html](preview-tour/preview-tour.html) | Column/type detection before commit, type overrides, and the ingest that lands a table |
| Query | [query-tour.html](query-tour/query-tour.html) | SQL workbench: run an aggregation, save it, materialize a result as a table, 51k-row pagination |
| Data & tables | [data-tables-tour.html](data-tables-tour/data-tables-tour.html) | Table list, tags/groups, table detail browsing |
| Labs | [labs-tour.html](labs-tour/labs-tour.html) | The 32-type chart catalog (heatmap + bar built, rest honest placeholders), chart + config panel |
| Pages | [pages-tour.html](pages-tour/pages-tour.html) | Building composed data pages from bar/heatmap/table blocks |
| Settings | [settings-tour.html](settings-tour/settings-tour.html) | LLM provider config and the internal `d8a_monster_*` metadata browser |
| Analyst | [analyst-tour.html](analyst-tour/analyst-tour.html) | Chat-about-your-data setup: table picker, provider picker, local llama.cpp option (honest empty chat — no fake replies) |

## Guarantees

- Every tour passed the skill's `verify.mjs` gate: end card reached, all step
  dots, clicks within 2.5px, zero console errors.
- No API-key material in any artifact (settings captured with a
  `sk-DEMO-not-a-real-key` dummy; verified by sweep).
- Honest beats staged: placeholder charts, empty chat and real row counts are
  shown as they are, narrated — never faked.

## Regenerating

See [RUNBOOK.md](RUNBOOK.md): start the app with CDP, seed the demo workspace
(`E:/demo-tour-workspace`, table `superstore` + URL server on :8123), then per
tour re-run `capture.mjs` → `build.mjs` → `verify.mjs`.
