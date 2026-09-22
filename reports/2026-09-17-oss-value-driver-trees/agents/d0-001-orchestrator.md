# Q: Which open source apps/tools implement value driver trees (KPI/driver decomposition trees) with live data and/or analysis?

## Direct answer

**No mature open-source app does value driver trees with live data.** Across five research branches (dedicated apps, BI platforms, FP&A engines, viz libraries, same-concept-different-name tools) the finding is consistent: the niche is empty at the top and fragmented at the bottom. What exists:

1. **Hobby-scale dedicated apps** — the closest match is **react-kpi-tree** (Apache-2.0, 3★): a React/React-Flow KPI-tree dashboard with genuine live data binding via polling adapters (OpenSearch, HTTP JSON → node values). Also kpi-tree-generator (MIT web app for authoring KPI trees, manual values only), a dormant 2019 Power BI custom visual (tqtensor/kpi_tree), and a Python image generator (kpi_tree_lib).
2. **Weak tree visuals inside mainstream BI** — Apache Superset's built-in Tree Chart renders hierarchies but is "not super useful at visualizing quantitative data" (Preset's own words); Grafana's **Interactive Tree Panel** (equansdatahub/bright-tree-panel, Apache-2.0, actively maintained) is the best live-data tree UX — table in, tree out, node clicks set dashboard variables — but it's a navigator, not a decomposition engine. Metabase/Redash/Lightdash/Evidence: nothing.
3. **Compute engines without trees** — FinanceToolkit (5.4k★, MIT) computes Extended DuPont from statements; revenue-model-builder (7★) implements the most literal driver tree (revenue = base × penetration × share × price) with sensitivity/Monte Carlo; Modeleon (Apache-2.0) tracks dependency-typed formula trees and exports live Excel; DoWhy/pgmpy/Pyro compute causal/probabilistic node values but have no tree UI.
4. **Dormant strategy-tool UIs** — bambooBSC (232★, dead since 2021) and hillfog rendered KPI cascades/strategy maps; ThingsBoard CE computes a live OEE = A×P×Q decomposition but only that fixed 3-driver tree.

**The market gap is verified from three directions**: Power BI's decomposition tree has no OSS clone (ECharts feature request #18043 closed unimplemented); everything that computes is a library without UI; everything with a UI is dormant, fixed-purpose, or immature. Nobody in OSS joins a data-bound computed tree with an editable/interactive UI.

## Ranked shortlist (what to actually look at)

| # | Tool | License | Live data? | Computes? | Maturity |
|---|------|---------|-----------|-----------|----------|
| 1 | react-kpi-tree | Apache-2.0 | ✅ polling adapters | ❌ renders given values | 3★, learning project |
| 2 | Grafana Interactive Tree Panel (BrightGrafana) | Apache-2.0 | ✅ any table datasource | ❌ aggregation in query | active (2026-05), catalog-vetted |
| 3 | FinanceToolkit | MIT | ✅ input statements | ✅ Extended DuPont | 5.4k★, mature |
| 4 | revenue-model-builder | unverified | programmatic | ✅ full driver math + sensitivity | 7★ |
| 5 | Modeleon | Apache-2.0 | via Excel round-trip | ✅ dependency-tracked formulas | 47★, early |
| 6 | kpi-tree-generator | MIT | ❌ manual values | ❌ | 4★, self-hostable Rails app |
| 7 | BIZOS company OS | none | reported | reported | 76★, KPI-tree screen among 19 |
| 8 | bambooBSC | Apache-2.0 | ❌ manual measure data | ✅ BSC scores | dead 2021 |
| 9 | ThingsBoard CE OEE | Apache-2.0 | ✅ telemetry | ✅ fixed A×P×Q only | very active |
| 10 | ECharts `tree` / d3-hierarchy / React Flow+elkjs / Observable Plot.tree | per-lib | DIY binding | ❌ | 1.2k–67k★, all healthy |

## Branch findings (integrated)

- **Dedicated apps (d1-002)**: five repos pass the floor, all hobby-scale. Demand is real (consulting Excel tools, SAP's commercial VDT, SaaS like kpitree.co) but OSS supply is ~5 repos — the space runs on Excel/PowerPoint and closed BI. Term collision ("driver tree" = Linux kernel/ML) hampers discovery.
- **BI platforms (d1-003)**: no mainstream OSS BI ships a true decomposition tree. Superset Tree Chart = plain hierarchy; Grafana Interactive Tree Panel = closest live-data UX but aggregation hardcoded in queries; ECharts `tree` is the DIY base. Superset itself downplays its tree chart for quantitative use.
- **FP&A engines (d1-004)**: FinanceToolkit is the only mature DuPont engine; revenue-model-builder is the only literal VDT engine (near-zero adoption); Modeleon has the right structure without FP&A semantics; openfpa delegates model structure to an AI agent. Interactive what-if VDT remains commercial-EPM-only (SAP/Jedox/Anaplan).
- **Viz libraries (d1-005)**: ECharts tree, d3-hierarchy, React Flow (+dagre/elkjs), Observable Plot.tree are genuinely used; Rete.js/LiteGraph are editor frameworks; pyvis notebook-grade. **No OSS library computes the roll-up math — all render pre-computed values.** Repo-relevant: svelteplot 0.14.2 (data.monster's sole engine) has no tree/cluster mark — verified by local package inspection.
- **Same concept, other names (d1-006)**: the concept exists twice — as strategy-execution suites (BSC/hoshin/OKR: bambooBSC, hillfog — all dead) and as compute engines (DoWhy 8.3k★, Pyro 9k★, pgmpy 3.3k★ — all healthy, no tree UI). CausalNex matched best but hit EOL 2026-06. ThingsBoard's OEE is the only live data-bound computed tree in OSS, and it's hardwired to one formula.

## Tensions

- **Empty-at-the-top vs healthy-compute**: d1-002/003/006 all independently conclude the interactive data-bound VDT doesn't exist in OSS, while the math/estimation half is well served — branches disagree only on which library is the best base, not on the gap itself.
- **"Tree panel" ≠ "decomposition tree"**: Grafana/Superset render a hierarchy you must pre-shape in SQL; Power BI decomposes a measure along dimensions chosen at view time with server-side aggregation. Marketing copy blurs this; the leaves do not.
- **Adoption vs fit**: the only tool with the full pattern (react-kpi-tree) is a 3★ learning project; the mature tools (FinanceToolkit, DoWhy) each cover only one half (compute, no tree).
- **data.monster engine tension**: a driver tree in this repo requires porting Plot's tree mark, embedding d3-hierarchy, or custom nodes — svelteplot can't do it today, conflicting with the "SveltePlot is the sole chart engine" decision.

## Open questions

- Chinese platforms (Gitee) likely host more BSC/DuPont apps (bambooBSC clones suggest a Chinese-language ecosystem) — unsearched.
- BIZOS KPI-tree screen: computes roll-ups from live Supabase data or static chart? Unverified.
- GrowthBook (60k★) may have shipped "metric trees" under another name — inconclusive.
- Superset's third-party plugin directory page wouldn't render; an unpublished decomposition-tree plugin can't be ruled out.
- Licenses of revenue-model-builder, finance-bp-dashboard, and tqtensor/kpi_tree are unverified — check before reuse.

## Implication for data.monster (repo context)

The verified gap is an opportunity: an interactive, data-bound value driver tree over DuckDB (spec-defined tree, node values computed from measures/dimensions, click-to-drill) would be the first mature OSS implementation. The library landscape says: compute the tree server/query-side (nothing does it for you), render with d3-hierarchy or custom React-Flow-style nodes, and treat react-kpi-tree + Grafana's tree panel as pattern references, not dependencies.

## Method note

Grounded strategy, depth 1, fan-out 5. All five leaves degraded from the built-in websearch tool to TinyFish Search/Fetch + GitHub REST API + productsear.ch (noted per leaf); GitHub API evidence (licenses, stars, activity) is primary-source-verified; doc-page claims carry their URLs. Confidence labels per finding live in the agent files.
