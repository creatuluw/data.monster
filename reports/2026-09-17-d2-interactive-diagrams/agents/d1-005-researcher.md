# d1-005 — Interactive text-to-diagram DSLs (click callbacks, links, tooltips, hover)

## Findings

### Mermaid (richest: JS callbacks + links + tooltips)

- `click nodeId callback` / `click nodeId call callback()` binds a **JavaScript function defined on the hosting page**, invoked with the nodeId — runs fully in the browser, requires `securityLevel='loose'` (disabled under `'strict'`). [source: https://mermaid.js.org/syntax/flowchart.html#interaction] verified
- Link form: `click B "https://..." "tooltip"` (optionally with target `_self/_blank/_parent/_top`, also `click D href "url"`); plain hyperlinks, no JS needed beyond mermaid itself. Tooltip text is the quoted third arg, styled via `.mermaidTooltip`; tooltips + URLs available since v0.5.2. [source: https://mermaid.js.org/syntax/flowchart.html] verified
- `securityLevel` values: `strict` (default — click disabled), `loose`/`antiscript` (click enabled), `sandbox` (sandboxed iframe "may hinder interactive functionality"); introduced v8.2, set via `mermaid.initialize({securityLevel:'loose'})`. [source: https://mermaid.js.org/config/usage.html] verified
- API users must call the `bindFunctions(element)` returned by `mermaid.render()` **after** inserting the SVG into the DOM, or events don't attach. [source: https://mermaid.js.org/config/usage.html] verified
- Limitations: callbacks only survive where mermaid JS itself runs (live web page); every static export (PNG/SVG file) and most embedded renderers (GitHub, wikis) keep `strict` and silently drop clicks — common failure mode. [source: https://github.com/mermaid-js/mermaid/issues/6809] reported

### Graphviz/DOT (links + native tooltips only, no callbacks)

- `URL` attribute (synonym `href`) on nodes, edges, clusters, and graph; emitted as hyperlinks in `svg`, `cmap`/`cmapx`, `i*map`, `ps2` output. In SVG the active area is the node's visible image; edge variants `headURL`, `tailURL`, `labelURL`, `edgeURL`. [source: https://graphviz.org/docs/attrs/URL/] verified
- `tooltip` attribute (mouse hover text) on node/edge/cluster/graph — **cmap and svg formats only**; defaults to the object's label if unset. [source: https://graphviz.org/docs/attrs/tooltip/] verified
- Where it runs: browser-native SVG anchors + `<title>` hover — **zero scripting**; anything dynamic must be layered by the embedding page (e.g. post-hoc JS on SVG elements). Overlapping active areas have unspecified dominance. [source: https://graphviz.org/docs/attrs/URL/] verified

### PlantUML (links + tooltips, no callbacks)

- `[[http://url]]`, `[[url label]]`, `[[url{tooltip} label]]`, `[[{tooltip} label]]`; triple brackets `[[[ ]]]` for class fields/methods; works across sequence, class, activity, state, nwdiag, JSON/YAML, notes, partitions; explicit `url of|for XXX is [[yyy]]` directive. SkinParams: `hyperlinkColor`, `hyperlinkUnderline`, `topurl` (global URL prefix). [source: https://plantuml.com/link] verified
- Where it runs: plain hyperlinks/anchors — interactive in **SVG output**; PNG export renders links inert (some wikis report non-clickable links due to raster output / renderer). [source: https://community.atlassian.com/forums/Confluence-questions/Include-links-in-PlantUML/qaq-p/1260494] reported

### Kroki (server-side rendering — interactivity mostly does NOT survive)

- Kroki returns static SVG/PNG; Mermaid/Excalidraw/diagrams.net are rendered in a **companion container with headless Chromium over CDP**, Graphviz via `dot` binary, PlantUML via Java binary. [source: https://docs.kroki.io/kroki/setup/configuration/] verified
- Mermaid's `securityLevel`, `secure`, `maxTextSize`, `maxEdges`, `startOnLoad` **cannot be overridden per request** — clients cannot opt into `loose`, so `click … callback` cannot work (no page JS context in a served image regardless). [source: https://docs.kroki.io/kroki/setup/configuration/] verified
- Declarative hyperlinks *can* survive for engines that emit SVG `<a>` elements natively (Graphviz `URL=`, PlantUML `[[ ]]`) — provided the SVG is inlined / embedded via object-iframe, not `<img>` (browsers ignore links in img-embedded SVG). Mermaid links under Kroki likely dropped with strict. Caveat: browser-img behavior is general knowledge, not verified against a specific doc. [source: https://graphviz.org/docs/attrs/URL/] verified (attribute) / browser caveat reported

### Comparison

| Capability | Mermaid | Graphviz | PlantUML | via Kroki |
|---|---|---|---|---|
| JS click callback | ✅ browser-side, `loose` only | ❌ | ❌ | ❌ |
| Hyperlink on element | ✅ (target control) | ✅ `URL`/`href` | ✅ `[[ ]]` | Graphviz/PlantUML SVG only; Mermaid uncertain |
| Tooltip/hover | ✅ custom `.mermaidTooltip` | ✅ native `tooltip` attr | ✅ `{tooltip}` syntax | Only native SVG `<title>`-based hover survives |

## Search trail

1. tinyfish "mermaid click callback interaction flowchart securityLevel loose" → flowchart.html, issue #6809
2. tinyfish "graphviz node url href attribute svg tooltip" → graphviz.org attrs URL/tooltip, forum thread
3. tinyfish "plantuml link element url interactive" → plantuml.com/link, Atlassian thread
4. tinyfish "kroki mermaid interactive click callback svg securityLevel" + "kroki … puppeteer" → docs.kroki.io configuration
5. tinyfish fetch: mermaid.js.org/syntax/flowchart.html, /config/usage.html, /config/configuration.html; graphviz.org/docs/attrs/URL/, /attrs/tooltip/; plantuml.com/link; docs.kroki.io/kroki/setup/configuration/

## Tensions

- Kroki docs forbid per-request `securityLevel` override but never state the companion's effective default — inferred strict, not documented on fetched pages.
- PlantUML links "work" per official docs, yet multiple integrators report non-clickable links — output format (PNG vs SVG) is the hidden variable.

## Open questions

- Effective `securityLevel` baked into the kroki-mermaid companion container (needs source read of yuzutech/kroki-mermaid-server).
- Whether Mermaid's plain URL-click (not callback) emits an SVG anchor under `strict`, which would make links survive headless rendering.
- Hover *styling* beyond native tooltips (CSS-driven) in Graphviz/PlantUML — none found; only Mermaid offers a styled tooltip.
