---
type: Learning
title: SveltePlot 0.14.2 has no tree mark — verified in the installed package
description: "Verified 2026-09-17 by a te9-research leaf against the **installed** package (not just docs): svelteplot 0.14.2 — data.monster's sole chart engine — ships no tr"
timestamp: "2026-09-17T13:07:25.353Z"
---

# SveltePlot 0.14.2 has no tree mark — verified in the installed package

Verified 2026-09-17 by a te9-research leaf against the **installed** package (not just docs): svelteplot 0.14.2 — data.monster's sole chart engine — ships no tree/hierarchy mark of any kind.

- **Implication**: any tree visualization (value driver tree, decomposition tree, org/roll-up tree) must be built from custom SVG/marks; there is nothing to configure into existence.
- **Related gap confirmed by the same research** (reports/2026-09-17-oss-value-driver-trees): no OSS chart library computes tree roll-up math, and no mature OSS app renders value driver trees with live data (closest: react-kpi-tree, Apache-2.0, a learning project; Grafana's Interactive Tree Panel is the best live-data tree UX in mainstream BI). An interactive DuckDB-backed VDT would be the first mature OSS one.
- See [[svelteplot-scale-null-not-false]] and [[svelteplot-datum-identity-empty-guard]] for other verified svelteplot internals facts.
