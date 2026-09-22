# Q: Which open-source tools implement the SAME concept (metric decomposed into a computed node tree) under different names — influence diagrams, BSC/strategy maps, OEE/RA trees, KPI pyramids/cascades, driver trees in MES/treasury, Bayesian/decision tools that compute node values from data?

## Findings

**Balanced scorecard / strategy map software**

1. **bambooBSC** — https://github.com/billchen198318/bamboobsc — Apache-2.0 · Java/Tapestry web BI for BSC: Vision → Perspectives → Objectives → KPIs, maintains measure-data, renders KPI reports, personal/dept BSC scores, Strategy Map, plus SQL-client charts. Passes floor partially: strategy map renders computed KPI scores from stored measure-data, but measure-data is manually maintained (no auto data binding). Maturity: 232★, dormant since 2021-06, author moved on. Verdict: closest OSS BSC realization of the concept; dead. Evidence [source: https://github.com/billchen198318/bamboobsc (README via GitHub API)] — confidence: verified.

2. **hillfog** (bambooBSC successor) — https://github.com/billchen198318/hillfog — no license file · KPI/OKR/PDCA/BSC web platform. 57★, dormant since 2022-05. Verdict: same concept under the OKR-cascade name; dead, legally unusable (no license). Evidence [source: GitHub API] — confidence: verified (existence) / reported (feature depth).

3. **BSPG** — https://sourceforge.net/projects/bspg/ — GPL-era PHP "complete opensource Balanced Scorecard environment for measurement-based management" (2013-era). Verdict: historical only; presumed dead, not verified runnable. Evidence [source: https://sourceforge.net/ search hit] — confidence: reported.

**Influence diagrams that compute**

4. **DecisionProgramming.jl** — https://github.com/gamma-opt/DecisionProgramming.jl — MIT · Julia package modeling multi-stage decision problems as influence diagrams (generalization of Bayesian networks); builds and **solves** the diagram (path probabilities, expected values) from user data via MILP. Passes floor on compute; no interactive rendering. Maturity: 53★, active (2025-05), academic (gamma-opt research group, JOSS-published). Verdict: the rigorous OSS influence-diagram engine. Evidence [source: GitHub API + repo docs page found in search] — confidence: verified.

5. **SamIam** (UCLA) — https://reason.cs.ucla.edu/samiam/ — open-source Java tool for Bayesian networks incl. influence-diagram evaluation. Academic, dated, small community. Verdict: computes influence diagrams from data; unmaintained UI-era Java. Evidence [source: prior knowledge + https://hig.diva-portal.org/smash/record.jsf?pid=diva2:873572 comparison study listing Netica/Hugin/PrecisionTree as the commercial set] — confidence: reported (site not fetched).

**Bayesian / causal libraries that compute node values from data**

6. **pgmpy** — https://github.com/pgmpy/pgmpy — MIT · Python library: structure learning + CPD estimation (MLE/Bayesian) from data, inference engine computing node marginals. 3330★, pushed 2026-09 (very active). Verdict: full compute engine for probabilistic node trees; no metric-decomposition UI. Evidence [source: GitHub API] — confidence: verified.

7. **DoWhy** (py-why) — https://github.com/py-why/dowhy — MIT · causal-graph estimation ("what drives Y by how much") with backdoor/IV estimators from data. 8317★, pushed 2026-09. Verdict: computes causal driver effects, the analysis half of a driver tree; Microsoft-originated, active. Evidence [source: GitHub API] — confidence: verified.

8. **Pyro** — https://github.com/pyro-ppl/pyro — Apache-2.0 · probabilistic programming (SVI): computes posterior node values from data. 9055★, active. Verdict: general PPL engine — a driver tree only if you hand-build the model; farthest from the BI concept. Evidence [source: GitHub API] — confidence: verified.

9. **CausalNex** (McKinsey) — https://github.com/mckinsey/causalnex — MIT-per-README (API: NOASSERTION) · structure learning + Do-calculus: learns causal graph from data, computes dists/effects per node. 2478★ but **end-of-life, discontinued 2026-06-24**. Verdict: matched the concept best of the ML libraries; now unmaintained. Evidence [source: repo banner via search result + GitHub API] — confidence: verified.

**Manufacturing OEE trees**

10. **ThingsBoard CE OEE calculated fields** — https://thingsboard.io/iot-hub/calculated-fields/overall-equipment-effectiveness-oee/ (+ OEE Score Card widget) — Apache-2.0 (CE) · computes OEE = Availability × Performance × Quality from live telemetry and renders score-with-breakdown. This IS a computed 3-driver decomposition of one metric from data — but a fixed, hardwired one; no general tree authoring. Maturity: very active. Verdict: OEE tree-by-another-name, single-purpose. Evidence [source: thingsboard.io docs] — confidence: verified (docs) / reported (widget rendering details).

11. **OpenMES** (AlTheHammer/openmes) — Java MES, "most established OSS MES" per surveys, intermittent since early 2010s — computes OEE but no driver-tree construct found. Verdict: below floor for the concept. Evidence [source: https://teeptrak.com + https://carbon.ms survey hits] — confidence: reported.

**KPI pyramid / hoshin cascades**: all hits commercial (Cascade, KPI Fire, Tability, BusinessMap X-matrix). GitHub searches for "strategy map kpi" and "kpi pyramid" returned only 0–1★ hobby repos. **Treasury driver trees**: nothing found in OSS. Evidence [source: TinyFish searches] — confidence: verified absence (within searched indexes).

## Tensions

- The concept exists twice over — as strategy-execution suites (BSC/OKR/hoshin, all commercial or dead) and as compute engines (pgmpy/DoWhy/DecisionProgramming.jl, all healthy) — but **nobody joins the two**: no active OSS renders a data-bound decomposition tree AND lets you edit it.
- Everything that passes the compute floor is a library (no UI); everything with a UI is dormant (bambooBSC/hillfog) or fixed-purpose (ThingsBoard OEE).
- Naming fragmentation is the discovery killer: BSC/strategy map/hoshin pyramid/driver tree/OEE/influence diagram never cross-reference each other, so each niche believes the space is empty.

## Open questions

- Chinese platforms (Gitee) may host active BSC/driver-tree apps — the bambooBSC clones suggest a Chinese-language ecosystem; not searched.
- Is there an active fork community around hillfog despite no license? Unverified.
- MES/SCADA world (Ignition scripting, openSCADA) may embed tree-style KPI computation not indexed by web search.

## Search trail

- TinyFish search: "open source balanced scorecard software", "strategy map open source tool KPI cascade", "OEE driver tree open source MES", "influence diagram open source tool compute", "hoshin kanri open source software X-matrix KPI cascade", "CausalNex causal tree compute node values from data open source", "ThingsBoard OEE calculation open source dashboard".
- GitHub API: repo search "bambooBSC", "DecisionProgramming", "influence diagram", "hillfog", "OEE driver tree", "strategy map kpi", "kpi pyramid"; direct repo fetches for mckinsey/causalnex, pgmpy/pgmpy, py-why/dowhy, pyro-ppl/pyro; README fetch for billchen198318/bamboobsc.
- No websearch/websearch_deep tool available in this agent; degraded to TinyFish Search/Fetch + GitHub REST API (same degradation as sibling d1-002).
