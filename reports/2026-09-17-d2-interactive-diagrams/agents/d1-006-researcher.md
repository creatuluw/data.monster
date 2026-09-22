# d1-006 — Embeddable diagram editors with interactive/selectable elements

## Findings

### tldraw
- Embed mechanism: React component `<Tldraw>` mounted in any host app (SDK, not iframe); custom shapes/tools/bindings supported. [source: https://github.com/tldraw/tldraw] verified
- Selection/click + behavior: `Editor` extends EventEmitter — `editor.on('event', ...)` gives `pointer_down` with `info.target === 'shape'` and `info.shape.id`; `change`/`store.listen()` (filters: source user/remote, scope document/session/presence); `onUiEvent` prop for UI actions. Behavior attaches to shape ids — fully programmatic. [source: https://tldraw.dev/sdk-features/events] verified
- License: source-available **tldraw license** — dev-only by default; production needs trial/commercial/hobby license **key** (watermark "made with tldraw" on hobby). Not open source; v1 was MIT. [source: https://tldraw.dev/community/license] verified
- Format: own TLStore/records (JSON documents), not a declarative text format. [source: https://github.com/tldraw/tldraw] reported

### Excalidraw
- Embed mechanism: React component from `@excalidraw/excalidraw` npm, embeddable directly; children components customize UI. [source: https://excalidraw-excalidraw.mintlify.app] verified
- Selection/click + behavior: `onChange` callback fired with `elements`, `appState`, `files` — primary way to track scene changes (incl. selection lives in appState); simplified programmatic element API (`convertToExcalidrawElements`) is beta. [source: https://excalidraw-excalidraw.mintlify.app ; https://docs.excalidraw.com] verified (search-surface)
- Links: elements carry a `link` prop and an `onLinkOpen` callback exists. [source: https://svelte-excalidraw.tips.dev] reported
- License: MIT. [source: https://github.com/excalidraw/excalidraw] reported
- Format: own `.excalidraw` JSON scene format (elements+appState), not declarative text. [source: https://docs.excalidraw.com] verified

### draw.io / diagrams.net (embed mode)
- Embed mechanism: iframe to `https://embed.diagrams.net` with `embed=1`; host↔iframe via `postMessage` (HTTPS only). `ready`/`init` handshake, host sends diagram as XML, editor returns XML on Apply/Cancel/exit; `proto=json` enables JSON protocol; autosave, export, merge/patch (diff sync) actions; `configure=1` config. [source: https://www.drawio.com/docs/reference/embed-mode/] verified
- Selection/click: **no per-shape selection/click event documented** in embed mode — events are lifecycle (init/load/save/autosave/exit/template/export) plus host-forwarded `openLink` (link clicks reported to host) and `shortcut` (reclaimed keychords via `passThroughKeys`). GitHub issue confirms postMessage is the only data channel when embedding embed.diagrams.net. [source: https://www.drawio.com/docs/reference/embed-mode/ ; https://github.com/jgraph/drawio/issues/5575] verified
- License: Apache-2.0 (drawio client). [source: https://github.com/jgraph/drawio] reported
- Format: mxGraph XML (own editor format); Mermaid import exists as an action, not native declarative source. [source: https://www.drawio.com/docs/reference/embed-mode/] verified

### maxGraph (mxGraph successor)
- mxGraph archived 2020; maxGraph is the actively maintained TypeScript successor (same XML format preserved). [source: https://github.com/maxGraph/maxGraph ; https://maxgraph.github.io/maxGraph/] verified
- Embed: library embedded directly in host app (no iframe); full programmatic API — selection model with change events (inherited mxGraph design). [source: https://maxgraph.github.io/maxGraph/] reported
- License: Apache-2.0. [source: https://github.com/maxGraph/maxGraph] verified
- Format: mxGraph XML. [source: https://github.com/maxGraph/maxGraph] verified

### yFiles for HTML (yWorks)
- Embed: commercial JS/HTML SDK library embedded in host app; white-label, royalty-free, term-based licensing; Single Developer/Project/Site licenses; evaluation license file. [source: https://www.yfiles.com/pricing.html ; https://www.yworks.com/products/yfiles-for-html/license-types] verified (search-surface)
- Interaction: full programmatic graph model — selection listeners, click item listeners, custom node/edge visualizations are core SDK features. [source: https://www.yfiles.com/the-yfiles-sdk/web/yfiles-for-html] reported
- License: closed-source commercial (demo code usable by licensees). Format: own graph model + GraphML import. [source: https://github.com/yWorks/yfiles-for-html-demos] reported

## Search trail
1. TF search: "tldraw embed custom behavior selection events API" → tldraw.dev events docs, GitHub.
2. TF search: "excalidraw embed programmatic API react component" → docs.excalidraw.com, mintlify.
3. TF search: "draw.io embed mode postMessage events diagrams.net" → drawio.com embed-mode ref, jgraph issue #5575.
4. TF search: "maxGraph mxGraph successor TypeScript GitHub jgraph" → maxGraph repo/site.
5. TF search: "yFiles for HTML embed license commercial yWorks" → yfiles.com/yworks.com.
6. TF search: "tldraw license watermarker business source license MIT" → tldraw.dev license.
7. TF fetch (verified): drawio embed-mode full doc; tldraw events doc; tldraw license doc; excalidraw API index; maxGraph README; excalidraw props page (partial).
8. TF search: "excalidraw props onChange elements appState onChangePointerDown onLinkOpen" → onChange confirmed.

## Tensions
- draw.io "interactive elements": viewer vs editor — embed mode gives an *editor* with no selection events, while `openLink` forwarding suggests links-on-cells work but selection telemetry does not. Hosts needing click-to-select dashboards must parse XML, not subscribe to events.
- tldraw "open source": marketed as SDK-friendly but license docs explicitly state it "would not be Open Source by any definition" (production = paid key). v1 MIT is dead end.

## Open questions
- Does the legacy www2.drawio.com/doc/faq/embed-mode doc document an undocumented `select` event? (old URL unfetchable via TinyFish — invalid_url).
- Excalidraw: full current props list (onChangePointerDown/onLinkOpen signatures) not read from primary docs page directly (props URL 404s via fetcher) — read package README to confirm.
- maxGraph selection-event API names post-refactor (Graph vs mxGraph) unverified against docs.
