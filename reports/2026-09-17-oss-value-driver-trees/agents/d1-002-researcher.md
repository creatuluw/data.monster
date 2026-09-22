# Q: Which open-source apps/tools are dedicated to (or centered on) value driver trees / KPI trees / driver trees with data binding?

## Findings

The dedicated OSS niche is **very thin** — no established, high-star project exists. Five repos pass the floor (actually render/compute trees); everything else is BI-suite decomposition visuals or consulting templates.

1. **react-kpi-tree** (YoungjaeKim) — https://github.com/YoungjaeKim/react-kpi-tree — Apache-2.0 · React + react-flow, Express, MongoDB. KPI-management dashboard rendering editable KPI trees; **real data binding**: "external connections" adapters (OpenSearch, HTTP JSON) poll external sources and push values into tree nodes (`pollingPeriodSeconds`). Self-hostable. Maturity: 3★, pushed 2025-07; author calls it "teatime coding" — a learning project. Closest match to the question's intent. Evidence [source: repo README fetched via GitHub API] — confidence: verified.

2. **kpi-tree-generator** (peno022, runs kpi-tree.com) — https://github.com/peno022/kpi-tree-generator — MIT · Ruby on Rails 7 + React + react-d3-tree + PostgreSQL; Docker Compose self-host. Full web app dedicated solely to authoring KPI tree diagrams: click-to-build tree, per-node value editing, export as image. Data entry is manual — **no live data binding**. Maturity: 4★, pushed 2024-01. Evidence [source: repo README fetched via GitHub API] — confidence: verified.

3. **kpi_tree** Power BI custom visual (tqtensor) — https://github.com/tqtensor/kpi_tree — no license · TypeScript. Power BI custom visual that **binds table data into per-node data grids** (collapsible nodes, KPI color coding, deltas). Dedicated KPI-tree renderer, but lives inside Power BI, not standalone/self-hosted. Maturity: 1★, pushed 2019, dormant. Evidence [source: repo README fetched via GitHub API] — confidence: verified.

4. **kpi_tree_lib** (yshimasaki) — https://github.com/yshimasaki/kpi_tree_lib — no license · Python (pip-installable). Library generating KPI decomposition-tree images (KPI cards with value, YoY/MoM, color) from indented text + a values dict. Programmatic values, no live binding. 1★, pushed 2024-08. Evidence [source: repo README fetched via GitHub API] — confidence: verified.

5. **BIZOS company OS** (ungden) — https://github.com/ungden/bizos-company-os — no license file · Next.js + Supabase. 76★, pushed 2026-04. A KPI Tree screen (Task → personal KPI → team → company → Financials cascade) inside a 19-screen business OS. Adjacent: tree is one screen among many; depth of its data binding not verified. Evidence [source: repo README via GitHub API] — confidence: verified (existence) / reported (binding depth).

**Below floor / excluded**: sjgant80-hub/operator (VDT as a structured consulting form, no data binding — verified); takechanman1228/claude-ecom + florianbonnet14/ThePowerOfAnalytics_ClaudeSkills (KPI decomposition as markdown output / Claude skills, no renderer — verified); yukta-pai/AI-assisted-decision-support-prototype (0★ Streamlit prototype, driver attribution, no license — reported); kpitree.co and aaronbrooker.com/vdttool (commercial SaaS / Excel-PPT generator — not OSS); Power BI's native decomposition tree (free visual, closed source); Grafana/Superset/Chartbrew (KPI dashboards, no driver-tree construct).

## Tensions

- The demand (Reddit r/consulting threads, consultancy Excel tools, SAP's commercial VDT) clearly exists, yet the OSS supply is ~5 hobby-scale repos — the space is served by Excel/PowerPoint, Power BI's decomposition tree, and SaaS (kpitree.co), not OSS.
- The one repo with true live data binding (react-kpi-tree) is the least mature (3★, self-described learning project) and binds via polling adapters, not a query/semantic layer.
- "Driver tree" as a search term is swamped by Linux kernel drivers and ML decision trees — term collision makes discovery hard.

## Open questions

- Does anything exist outside GitHub's search index (GitLab, SourceForge, Chinese platforms like Gitee) with real maturity? Not searched.
- Does BIZOS's KPI Tree screen compute roll-ups from live Supabase data, or is it a static chart? Unverified.
- GrowthBook (OSS, 60k+★) was probed for a "metric trees" feature — a docs page guess 404'd and search was inconclusive; possibly shipped under another name. Unresolved.

## Search trail

- GitHub repo search API (sorted by stars): `value driver tree`, `KPI tree`, `driver tree`, `value tree financial model`, `value-driver-tree`, `kpitree`, `decomposition tree dashboard`, `kpi decomposition`, `driver tree dashboard finance` (last one rate-limited), `metric tree experimentation`.
- TinyFish search: "value driver tree open source tool", "KPI tree open source self-hosted dashboard", "kpitree.co github open source", "GrowthBook metric trees feature", "\"metric trees\" growthbook 4.0 hierarchical metrics".
- Deep-fetched via GitHub README API: peno022/kpi-tree-generator, YoungjaeKim/react-kpi-tree, yshimasaki/kpi_tree_lib, tqtensor/kpi_tree, ungden/bizos-company-os, sjgant80-hub/operator, takechanman1228/claude-ecom, yukta-pai/AI-assisted-decision-support-prototype.
- TinyFish fetch: kpitree.co (empty render), aaronbrooker.com/vdttool (empty render), growthbook.io blog 4.0. curl: docs.growthbook.io/app/metric-trees (404).
- No websearch/websearch_deep tool available in this agent; degraded to TinyFish Search/Fetch + GitHub REST API (noted per task instruction).
