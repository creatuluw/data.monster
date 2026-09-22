# Leaf: D2 rendered-output extensibility (post-render interactivity)

## Findings

- **Output formats**: D2 CLI exports **SVG (default), PNG, GIF, PDF, PPTX**; PNG/GIF/PDF/PPTX are non-interactive so an appendix of tooltips/links can be added (`--appendix`); `--watch` serves the SVG in a live-reloading browser. Only SVG is a DOM you can script. [source: https://github.com/d2lang/d2 README "Export file types"] [verified]

- **SVG DOM structure — shape names ARE recoverable, as base64 classes not ids**: every shape renders as outer `<g class="{base64url(shapeID)}">` wrapping inner `<g class="shape">`; connections render `<g class="{base64url('(src -> dst)[0]')}">`. The ID is the full dotted DSL key path. Empirically verified by running `@terrastruct/d2` v0.1.33 (WASM, D2 `v0.7.0-HEAD`): `server` → `class="c2VydmVy"`, `db` → `ZGI=`, nested `a.b.c` → `YS5iLmM=`; **no `id=` attributes on groups**; root has `data-d2-version`. Decode from JS: `atob(cls.replaceAll('-','+').replaceAll('_','/'))`. So click/selection wiring = `querySelectorAll('g.shape')` → decode parent's first class. [source: https://github.com/d2lang/d2/blob/master/d2renderers/d2svg/d2svg.go lines ~1078, ~1705, ~1745 + local npm run] [verified]

- **JS/TS API exists**: `@terrastruct/d2` npm package — WASM build + web worker, ESM/CJS, isomorphic browser/Node. API: `new D2()` → `await d2.compile(src, opts)` → `{ diagram, renderOptions }` → `await d2.render(diagram, opts)` → SVG string. [source: https://www.npmjs.com/package/@terrastruct/d2 / https://unpkg.com/@terrastruct/d2/README.md] [verified — ran it]

- **Go library**: `oss.terrastruct.com/d2` usable as a library from Go programs (examples in `docs/examples/lib`); the WASM/JS build derives from this. [source: README "D2 as a library" + https://pkg.go.dev/oss.terrastruct.com/d2] [verified]

- **Playground runs WASM in-browser**: FAQ states D2 runs on https://play.d2lang.com via WebAssembly (repo contains `d2js/d2wasm`). [source: https://d2lang.com FAQ + repo tree] [reported]

- **Built-in interactivity, no plugin system**: native `<title>` tooltips (`tooltip:`) and safe-scheme `<link>` anchors are baked into the SVG (unsafe URL schemes rejected in renderer). No general JS plugin/render-hook API. Extension points: layout engines (dagre/ELK), theme catalog, sketch renderer, custom TTF fonts (`fontRegular/Italic/Bold` byte arrays). IR-level hook: `d2target.Shape.Classes []string` / `Connection.Classes` are part of the JSON diagram IR — since `render(diagram)` takes the compiled IR object, a JS caller can inject custom classes (or mutate positions) between compile and render; no DSL keyword sets `classes`. [source: d2svg.go + d2target/d2target.go + npm README] [verified field exists & renders; mutation path inferred, not tested]

## Search trail

1. tinyfish: "d2 diagram svg element id shape name interactivity" → GitHub README (formats), mankier (appendix)
2. tinyfish: "terrastruct d2 npm javascript api embed library" → @terrastruct/d2, pkg.go.dev
3. tinyfish: "d2 playground wasm how it works browser" → GitHub discussion #234, jsDelivr, FAQ
4. Fetched: unpkg README, raw d2svg.go (repo moved terrastruct/d2 → d2lang/d2), README sections
5. **Empirical**: `npm i @terrastruct/d2` + rendered 2 diagrams; decoded classes; checked ids/`<title>`

## Tensions

- Common claim "SVG elements carry the shape name as id" is **false today** — it's base64-encoded in a class (deliberate: XSS/id-collision safety, per `class_escaping_test.go`, `id_reference_security_test.go`).
- The class scheme is de-facto stable (driven by security tests) but **not a documented public contract** — could shift between versions.
- npm wrapper is v0.1.x around `v0.7.0-HEAD` wasm — young API; Go lib is the mature surface.

## Open questions

- Does play.d2lang.com source demonstrate selection/highlight patterns worth copying? (repo `d2js/js` not inspected)
- Tooltips/links in SVG **appendix icons** use `svg.SVGID(shape.ID)` (sanitized readable ids) — exact template format not captured.
- IDE integration (VSCode extension) emits no richer event API than the SVG itself — unexplored.
