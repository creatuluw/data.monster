# Root synthesis — Can D2 render interactive diagrams (selectable elements, behavior/links)? If not, what are the alternatives?

## Direct answer

**Partially, and only in the weakest sense.** D2 has exactly two native interactivity features — `tooltip` (hover text via SVG `<title>`) and `link` (click a shape → navigate to an external URL). It has **no** element selection, **no** click callbacks, and **no** JS event binding on rendered elements. Its own "Interactive" tour section lists only Tooltips and Links — the same two features shipped in Dec 2022 (v0.1.4), despite that release promising "much more in store for interactivity" that has not landed in the docs since.

**But there is a real DIY path**: D2's SVG output is a scriptable DOM. A leaf agent installed `@terrastruct/d2` (the official WASM npm package) and empirically verified that every shape renders as `<g class="{base64url(shapeKey)}"><g class="shape">…`, so a host page *can* attach its own click handlers and selection behavior by decoding those classes (`atob`) and binding to `querySelectorAll('g.shape')`. Caveat: the class scheme is de-facto stable (enforced by security tests) but **not a documented public contract** — no id attributes, no plugin system, no render hooks.

So: if "interactive" means hover + hyperlink → D2 works today. If it means select an element and run behavior → D2 requires post-processing its SVG yourself, or you switch tools.

## Branch findings

### Branch 1 — D2 native interactivity (verified)

- `link` on shapes → clickable external URL (fragments with `#` must be quoted). [d2lang.com/tour/interactive]
- `tooltip` → hover text, no markdown; degrades to numbered appendix on PNG export. [d2lang.com/tour/interactive]
- No click callbacks / event API anywhere in docs or repo (verified by absence). Open issues confirm limits: #897 no touch support for tooltips (since Feb 2023), #1205 no custom labels for links/tooltips.
- Animations are passive loops (GIF board cycling via `--animate-interval`, marching-ants dash), not user-triggered interaction.
- play.d2lang.com being "interactive" refers to source editing, not the rendered diagram.

### Branch 2 — D2 extensibility for DIY interactivity (empirically verified)

- Output: SVG (default), PNG, GIF, PDF, PPTX — only SVG is scriptable.
- Shape identity IS recoverable: outer `<g class="base64url(full-dotted-key)">` wraps inner `<g class="shape">`; connections encode `(src -> dst)[0]`. Verified by running `@terrastruct/d2` v0.1.33 (WASM, D2 v0.7.0-HEAD).
- JS API: `new D2()` → `compile(src)` → `render(diagram)` → SVG string, isomorphic browser/Node. Go lib exists (`oss.terrastruct.com/d2`).
- Escape hatch: the compiled IR (JSON) exposes `shapes[].classes` — a JS caller can inject custom classes between compile and render.
- Common claim "SVG elements carry the shape name as an id" is FALSE (deliberate: XSS/id-collision safety).

### Branch 3 — JS libraries with true interactivity (the real answer for "select + add behavior")

All verified from primary docs — every one supports selection + click/behavior binding:

- **React Flow** — `elementsSelectable` default true, `onNodeClick`, `onSelectionChange`; MIT; **React-only** (Svelte Flow is the sibling for Svelte apps).
- **Cytoscape.js** — click-select, box select, modifier multi-select, delegated `cy.on('tap','node',…)`; MIT; vanilla/framework-agnostic.
- **AntV X6** — editing engine: `node:click`/`port:click`/`edge:click` events, built-in lasso selection, minimap; MIT; vanilla core + framework shape packages.
- **AntV G6** — analysis-oriented, `node:click` event system; MIT.
- **JointJS** — open core (MPL) + commercial JointJS+; advanced selection tooling largely lands in the paid tier.
- **Rete.js** — `selectableNodes` extension, `nodepicked` event; MIT; framework-agnostic with Svelte renderer plugin.
- **GoJS** — best-in-class built-in interaction (select, drag, undo); **commercial** — evaluation watermark.
- **D3.js** — `selection.on("click",…)` works, but selection state, node model, and links are all hand-built.

### Branch 4 — Text-to-diagram DSLs with interactivity (D2's true peer group)

- **Mermaid is the only DSL with real JS click callbacks**: `click nodeId callback` invokes a function on the hosting page — requires `securityLevel: 'loose'` (default `strict` disables it), and `mermaid.render()` users must call `bindFunctions()` after DOM insertion. Links + tooltips also supported. Dies in static exports, GitHub, and Kroki.
- **Graphviz** — `URL`/`href` attribute on nodes/edges/clusters + native `tooltip`; pure SVG anchors, zero scripting.
- **PlantUML** — `[[url{tooltip} label]]` syntax across ~8 diagram types; links inert in PNG output.
- **Kroki** (server-side rendering) kills scripted interactivity by construction: `securityLevel` cannot be overridden per request; only native SVG `<a>` hyperlinks survive, and only when the SVG is inlined (browsers ignore links inside `<img>`).
- Net: among DSLs, D2's tooltip+link ≈ Graphviz/PlantUML tier; Mermaid alone goes further (callbacks); nothing in the DSL world gives you selection state.

### Branch 5 — Embeddable diagram editors/canvas apps

- **tldraw** — React SDK (no iframe); `editor.on('event')` gives shape-level pointer targets, `store.listen()` for all mutations — richest programmatic attach. Source-available: **production requires a license key** (v1's MIT is gone).
- **Excalidraw** — React component, **MIT**; `onChange(elements, appState, files)` + `onLinkOpen`; own JSON scene format; programmatic element API still beta.
- **draw.io embed mode** — iframe + postMessage protocol gives lifecycle events (init/save/exit) but **no per-shape selection/click events**; only `openLink` forwarding. Apache-2.0. Not suitable for click-to-behave dashboards.
- **maxGraph** — the maintained Apache-2.0 TypeScript successor to mxGraph; library-embed with full programmatic selection model.
- **yFiles for HTML** — fully programmatic graph/selection; closed-source commercial SDK.

## Tensions (unresolved disagreements surfaced)

1. **D2 marketing vs reality**: "Hover over shapes to learn more, or click through to another page" sounds rich; it resolves to exactly tooltip + external link.
2. **Recoverable shape names, unstable contract**: the base64-class scheme is empirically solid but undocumented — a D2 upgrade could break DIY interactivity silently.
3. **"Open source" claims diverge**: tldraw is not OSS for production (license key); GoJS and JointJS gate their best interaction features behind commercial tiers; draw.io embed is open but event-blind.
4. **Mermaid callbacks are fragile**: they exist, but die under `strict`, static export, and Kroki — the most-advertised interactivity feature with the most deployment traps.

## Decision matrix

| You want… | Pick |
|---|---|
| Hover text + hyperlinks only, best-in-class DSL | **D2 as-is** (`tooltip`/`link`) |
| D2 layout + custom selection/behavior in your app | **@terrastruct/d2 WASM + post-process SVG** (decode base64 classes, bind handlers) — DIY, undocumented contract |
| Text DSL with JS click callbacks | **Mermaid** (`securityLevel: 'loose'`, browser only) |
| Interactive diagram *component* in a web app | **Cytoscape.js** (vanilla) / **React Flow** (React) / **Svelte Flow** (Svelte) / **AntV X6** (editing-focused) |
| Node-editor UX (select, wire, move) | **Rete.js** (MIT) or **GoJS** (commercial, best polish) |
| Whiteboard-style embeddable editor | **Excalidraw** (MIT) — tldraw if you'll buy the license |

Context note for this repo (data.monster = SvelteKit + Tauri): Svelte Flow, Cytoscape.js, X6, or Rete.js-with-Svelte-renderer are the natural fits if the goal is interactive diagrams embedded in the app; the D2-WASM + SVG post-processing route is viable for authoring-in-D2 with behavior layered on top.

## Open questions (from leaves, worth follow-up only if pursued)

- Does the kroki-mermaid companion default to `strict`? (needs source read)
- JointJS Community: does the free core include the selection toolchain?
- Exact current GoJS / tldraw pricing tiers (pages unreachable this run).
- D2 issue #2561 ("always-show tooltips", closed Jul 2025) — did an option ship, in which renderer?

## Sources (primary, per branch)

- D2: d2lang.com/tour/interactive/, d2lang.com/releases/0.1.4/, github.com/d2lang/d2 (+issues #897, #1205), npmjs.com/package/@terrastruct/d2, d2svg.go (render source)
- Libraries: reactflow.dev/api-reference/react-flow, js.cytoscape.org, github.com/antvis/X6, g6.antv.antgroup.com/en/api/event, jointjs.com, retejs.org/docs/guides/selectable/, d3js.org/d3-selection/events, gojs.net/latest/learn
- DSLs: mermaid.js.org/syntax/flowchart.html#interaction, mermaid.js.org/config/usage.html, graphviz.org/docs/attrs/URL/ + /tooltip/, plantuml.com/link, docs.kroki.io/kroki/setup/configuration/
- Editors: tldraw.dev/sdk-features/events + /community/license, excalidraw docs (mintlify), drawio.com/docs/reference/embed-mode/, github.com/maxGraph/maxGraph, yworks.com/products/yfiles-for-html
