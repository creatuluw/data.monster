# d1-004 — Interactive node-diagram libraries: selection, click behavior, links, programmatic editing

## Findings

**React Flow (@xyflow/react)** — Built-in selection: `elementsSelectable` prop defaults `true` ("nodes and edges can be selected by clicking"; per-element `selectable` override) [source: https://reactflow.dev/api-reference/react-flow] [verified]. Click binding: first-class `onNodeClick`, `onNodeDoubleClick`, plus `onSelectionChange` and selection-box drag events [source: same] [verified]. Links on nodes = custom React node components (any JSX/behavior); editing = fully controlled nodes/edges via props. License: MIT, optional paid React Flow Pro subscription [source: https://reactflow.dev/] [verified]. Framework: **React only** (Svelte Flow is a separate sibling) [reported].

**Cytoscape.js** — Interaction out of the box: "Click a node to select it", multi-select via modifier keys, box selection, grab/drag nodes; "All gesture actions can be controlled by the programmer" [source: https://js.cytoscape.org/] [verified]. Event binding incl. delegation: `cy.on('tap', 'node', evt => ...)`; `selectionType: 'additive' | 'single'` [source: same] [verified]. Full programmatic graph JSON API. Framework: vanilla JS, framework-agnostic. License: MIT [reported — npm/GitHub].

**JointJS** — Open-source Community version ("open-source code, essential features") vs commercial JointJS+ ("Commercial license and source code", "40+ UI features", "priced per developer") [source: https://www.jointjs.com/] [verified]. Framework: vanilla JS/TS core + official React/Angular/Vue/Svelte wrappers; "New: JointJS for React" [source: same] [verified]. Element events/click handlers and programmatic model (dia.Graph JSON) exist in core [source: https://www.jointjs.com/ docs — not directly fetched] [reported]. Advanced selection/interaction tooling largely marketed as JointJS+ features [reported].

**AntV X6** — Graph **editing** engine: "comprehensive event system that allows listening to any events", "Built-in 10+ graph editing extensions, such as lasso selection, alignment lines, minimap" [source: https://github.com/antvis/X6 README] [verified]. Event names: `cell:click`, `node:click`, `node:port:click`, `edge:click`, `blank:click`, dblclick/contextmenu/mousedown variants [source: https://x6.antv.antgroup.com/en/tutorial/basic/events] [verified]. Nodes customizable via SVG/HTML/React/Vue/Angular [verified, README]. License MIT [verified, README]. Framework: vanilla core + framework shape packages.

**AntV G6** — Graph **visualization/analysis** engine: "powerful event mechanism ... node clicks, edge hovers, canvas drags"; `[object]:[event]` names like `node:click` [source: https://g6.antv.antgroup.com/en/api/event] [verified]. React-based custom nodes supported [source: https://g6.antv.antgroup.com/en/manual/element/overview] [verified]. MIT [reported]. Less editor-oriented than X6 (no built-in editing canvas chrome).

**D3.js (diagram layer)** — Selections are DOM-bound: `selection.on("click", ...)` — "selections allow listening for and dispatching of events" [source: https://d3js.org/d3-selection/events] [verified]. No built-in element-selection state machine, node model, or links — all behavior/edges are hand-built on DOM events. Framework: vanilla, embeds anywhere. License ISC/BSD-style permissive [reported].

**GoJS** — Interaction built in: "Click a node to select it, or click and drag to move it", selection box, `Delete` removes selected nodes, undo manager [source: https://gojs.net/latest/learn/index.html] [verified]. Declarative templates (Node/GraphObject) with click/contextmenu handlers and Model-based programmatic editing [reported — API docs not fetched]. **Commercial**: "Commercial use is permitted under the Evaluation License Agreement or our standard License Agreement. After you have purchased a license..." — evaluation version renders a watermark [source: https://gojs.net (deployment & license keys), via search snippet] [reported]; pricing at nwoods.com "Pricing and Ordering" [reported]. Framework: vanilla JS, works with any framework.

**Rete.js** — Node-editor framework: `AreaExtensions.selectableNodes(area, selector, { accumulating })` enables node selection, Ctrl multi-select, collective move; `selector.pick/isPicked/translate` [source: https://retejs.org/docs/guides/selectable/] [verified]. `nodepicked` event fires on node click [source: https://retejs.org/docs/faq/ search snippet] [reported]. Programmatic editing via plugins (add/remove nodes, connections). License: MIT [verified — guide footer "Released under the MIT License, © 2018-2026 Vitaliy Stoliarov"]. Framework: framework-agnostic core with React/Vue/Angular/Svelte render plugins [reported].

## Search trail

- tinyfish search: "interactive diagram javascript library select node click event"; "react flow vs jointjs vs gojs comparison"; "antv x6 interactive diagram node click"; "antv x6 graph events documentation node:click"; "gojs license pricing commercial evaluation watermark"; "retejs area plugin selectable nodes click events"
- tinyfish fetch (browser-rendered): reactflow.dev/api-reference/react-flow; js.cytoscape.org; jointjs.com; x6.antv.antgroup.com/en/tutorial/basic/events; g6.antv.antgroup.com/en/api/event + /manual/element/overview; d3js.org/d3-selection/events; gojs.net/latest/learn/index.html; retejs.org + /docs/guides/selectable/; raw.githubusercontent.com/antvis/X6/master/README.md; reactflow.dev/
- 404s: x6 api/graph/event (zh+en), gojs.net/company/pricing.html, nwoods.com/pricing, G6 v5 raw README

## Tensions

- JointJS commercial split: marketing page centers JointJS+ ("40+ UI features"); open-source Community is described as "essential features" — reviewers differ on how much interaction (selection tools etc.) is usable without paying [neoteric.eu: "React Flow sells source code you own yourself... JointJS for React sells..."]
- GoJS "free" claims vs watermark: forum/SAS reports show unlicensed use renders an evaluation watermark — free only for evaluation (and, per a 2015 GitHub answer, OSS/educational use with watermark) [reported].
- React Flow is React-bound; vanilla/Svelte projects need Svelte Flow or a wrapper — a real constraint for framework fit.

## Open questions

- Does JointJS Community (MPL) include a selection/click toolchain without JointJS+? (Docs site clients.jointjs.com not fetched.)
- Exact current GoJS pricing tiers (nwoods.com pricing page unreachable this run).
- G6 license (MIT) not verified from primary source this run; only X6 README verified.
