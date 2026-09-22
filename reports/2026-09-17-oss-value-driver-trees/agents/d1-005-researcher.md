# Q: Which open-source visualization libraries/components could render a value driver tree bound to live data — and which are genuinely used for KPI/driver trees?

## Findings

- **d3-hierarchy** (https://github.com/d3/d3-hierarchy, ISC, 1,273★, pushed 2025-04) — Layout-only library (tidy tree, dendrogram, treemap, partition). Not a component: you bind data and render nodes/links yourself in React/Svelte. The de-facto engine under most hand-rolled driver trees. Evidence: d3js.org/d3-hierarchy docs + d3indepth.com hierarchy tutorial. Confidence: **verified** (repo API + docs).
- **Observable Plot** (https://observablehq.github.io/plot/marks/tree, ISC) — `Plot.tree` / `Plot.cluster` marks: Reingold–Tilford node-link diagrams fed by **tidy tabular data** (a `path` column), declarative node styling/labels. Genuinely data-bound. Confidence: **verified** (official mark docs + tree-transform docs).
- **svelteplot 0.14.2** (the engine this repo uses) — **no Tree/Cluster mark exists**: zero `Tree` occurrences in `dist/index.d.ts`, no tree components in `dist/`. Plot's tree mark has not been ported. Confidence: **verified** (inspected installed package).
- **Apache ECharts** (https://echarts.apache.org, Apache-2.0, 67,344★, pushed 2026-09) — `tree` series: nested-JSON data with per-node `value`, orient/collapse/initial expand, canvas perf, tooltips — the most batteries-included chart-lib tree. Used in BI contexts (Superset/Preset "Tree Chart" for KPI dashboards). A Power-BI-style *decomposition tree* was requested and closed unimplemented (issue #18043, 2022) — you compute node values yourself. Confidence: **verified** (repo API, issue API, preset.io report).
- **React Flow / xyflow** (https://github.com/xyflow/xyflow, MIT, 38,402★, pushed 2026-09) — Node-based editor; custom nodes = KPI cards with live values, edges = links. No built-in tree layout — needs dagre/elkjs add-ons. Demonstrated for parent/child tree viz (craxinno walkthrough video); commercial analog xViz "Performance Flow – KPI Tree" proves the pattern in BI. Confidence: **verified** (repo API) / usage **reported**.
- **react-kpi-tree** (https://github.com/YoungjaeKim/react-kpi-tree, Apache-2.0, **3★**, pushed 2025-07) — A literal KPI-tree dashboard component ("widely used KPI management dashboard" is the author's claim). Exactly on-topic but immature. Confidence: **verified** (repo API); real-world adoption **unverifiable**.
- **AntV G6** (https://github.com/antvis/G6, MIT, 12,298★, pushed 2026-07) — General graph/DAG/tree-layout library with rich node metrics via custom nodes/labels; strong in Chinese fintech flow-analysis contexts. Heavier than a chart lib for a pure hierarchy. Confidence: **verified** (repo API); KPI usage **reported**.
- **react-d3-tree** (https://github.com/bkrem/react-d3-tree, MIT, 1,197★, pushed 2025-02) — React component wrapping d3-hierarchy: data binding, collapsible nodes, custom node rendering. Moderate maintenance. Confidence: **verified** (repo API).
- **Kedro-viz** (https://github.com/kedro-org/kedro-viz, Apache-2.0, 760★, pushed 2026-09) — Pipeline DAG viz with dataset-stats click-through — proves "diagram nodes bound to data" is shippable OSS, but domain-locked to Kedro pipelines; not a reusable KPI-tree component. Confidence: **verified** (repo API).
- **Rete.js** (https://github.com/retejs/rete, MIT, 12,261★, active) and **LiteGraph.js** (MIT, 8,141★, pushed 2024-08) — Visual-programming/dataflow *editor* frameworks. Render nodes-with-values fine but target builders (ComfyUI-style), not read-mostly KPI presentation. Confidence: **verified** (repo API).
- **pyvis** (https://github.com/WestHealth/pyvis, BSD-3-Clause, 1,206★, pushed 2024-04) — Python/vis.js network wrapper; can show metric labels on NetworkX trees. Notebook-grade, stale-ish, weak live-binding story. Confidence: **verified** (repo API).
- **Svelvet** (https://www.svelvet.io, Svelte node-graph component lib, MIT) — Svelte-native React Flow alternative surfaced by search; smaller ecosystem. Confidence: **reported** (productsear.ch).

**Genuinely used for KPI/driver trees**: ECharts tree (BI dashboards), d3-hierarchy + custom renderer (most common hand-rolled path), React Flow (KPI-tree dashboards/editors), Observable Plot tree (analytical/notebook). Not: Rete.js/LiteGraph (editor frameworks), pyvis (notebooks), Kedro-viz (domain-locked).

## Tensions

- **No OSS lib computes the driver math.** Every candidate renders *pre-computed* node values; roll-up logic (sum/product decomposition) is always yours. The closest commercial artifact (Power BI decomposition tree / xViz Performance Flow) has no OSS clone — ECharts' equivalent feature request was closed unimplemented.
- **Layout quality vs node richness trade**: d3-hierarchy/Plot give perfect tidy layouts with poor rich-node support; React Flow gives arbitrary KPI-card nodes but pushes layout onto dagre/elkjs.
- **svelteplot gap**: this repo's sole chart engine has no tree mark — a driver tree here means porting Plot's tree mark, embedding d3-hierarchy, or stepping outside svelteplot (contradicts the wiki's "SveltePlot is the sole chart engine" decision).

## Open questions

- Is react-kpi-tree (3★) worth forking as a reference implementation, or just a pattern demo?
- G6 v5's built-in tree layouts — production-grade quality vs d3-hierarchy's tidy algorithm? Not depth-checked.
- Any OSS decomposition tree with drill-on-click + auto-aggregation (Power BI parity) below search visibility? None found.

## Search trail

- tinyfish: "d3-hierarchy driver tree KPI visualization"; "react flow KPI tree dashboard"; "apache echarts tree series kpi decomposition dashboard"; "observable plot tree mark Plot.tree hierarchy"; "open source value driver tree component github" (websearch/websearch_deep tools unavailable in this session — degraded to TinyFish Search + productsear.ch API + GitHub REST API verification; noted per task instructions).
- productsear.ch: "value driver tree visualization"; "tree diagram library data binding".
- GitHub API (verified licenses/stars/activity): react-kpi-tree, kedro-viz, xyflow, d3-hierarchy, G6, bkrem/react-d3-tree, retejs/rete, litegraph.js, pyvis, apache/echarts (+issue 18043).
- Local inspection: node_modules/svelteplot@0.14.2 (no tree/cluster marks).
