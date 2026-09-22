# D2 (d2lang.com, Terrastruct) — native interactivity in rendered output

## Findings

- **(a) Hyperlinks: YES (native).** The `link` keyword on shapes makes them clickable; click navigates to an external URL. Official "Interactive" tour section = exactly two features: Tooltips and Links. Caveat: URL fragments containing `#` are parsed as comments unless quoted. [source: https://d2lang.com/tour/interactive/] — **verified**
- **(b) Tooltips: YES (native).** `tooltip` keyword adds hover text; implemented with HTML `<title>` tags — "basic support for text formatting. Markdown won't be rendered." On static export (PNG), tooltips degrade to numbered icons + appendix. [source: https://d2lang.com/tour/interactive/] — **verified**
- **Interactivity claim is exactly tooltip+link.** Release 0.1.4 (Dec 2022) introduced "interactive diagrams. Namely, `tooltip` and `link` … hover to see more or click to go to an external link" (PR #548), and promised "much more in store for interactivity, stay tuned!" — but the tour's Interactive section still lists only Tooltips and Links today. [source: https://d2lang.com/releases/0.1.4/] — **verified**
- **(c) Click callbacks / JS event binding: NO native support.** No `on_click`, no event-handler or callback keyword exists in the docs; nothing in the docs index, tour, or GitHub issue search shows an API to bind JS events to rendered SVG elements. Custom interactivity means post-processing the SVG yourself or embedding via `@terrastruct/d2` (WASM) and writing your own JS against the DOM. [sources: https://d2lang.com/tour/interactive/ ; https://github.com/d2lang/d2] — **verified (absence, from official docs + repo)**
- **(d) Selection: none found.** No native selectable-shape / multi-select state documented anywhere on d2lang.com. [source: https://d2lang.com/] — **verified (absence)**
- **Animations: passive only.** `--animate-interval` renders boards into a looping GIF ("Text to (animated) diagrams", Apr 2023); `style.animated` gives connections a marching-ants dash animation. Looping visual animation, not user-triggered interaction. [sources: https://d2lang.com/blog/ ; issue #1666 https://github.com/terrastruct/d2/issues/1666] — **reported**
- **Known limitations from GitHub:** #897 (open since Feb 2023) tooltips get "no response on touching tooltipped elements" — no touch support; #1205 (open since Apr 2023) "links and tooltips should allow custom labels". [sources: https://github.com/terrastruct/d2/issues/897 (fetched via GitHub API); https://github.com/terrastruct/d2/issues?q=is%3Aissue+interactive] — **verified**
- The interactive playground (play.d2lang.com) is interactive *editing* of D2 source, not interactive rendered diagrams. [source: https://play.d2lang.com/] — **verified**

## Search trail

- TinyFish searches: `d2lang interactive diagram click`; `terrastruct d2 github issue click event javascript`; `d2lang diagram onclick callback javascript event handler` (mostly generic JS noise); `d2lang animate-interval gif boards`.
- Fetched: d2lang.com/tour/interactive/ (canonical), d2lang.com/releases/0.1.4/, github.com/terrastruct/d2/issues?q=is%3Aissue+interactive (browser-rendered list), issue #897 body via api.github.com.
- Rejected: d2lang.com/tour/oracle/ (404 — no such page), d2lang.com/tour/links/ (404 — links live under /tour/interactive/), anonymous GitHub search API (HTTP 422, requires auth), openseadragon/easeljs/stackoverflow hits (wrong product).

## Tensions

- D2 homepage advertises "Hover over shapes to learn more, or click through to another page" — sounds rich, but resolves to only tooltip + external link.
- 0.1.4's "much more in store for interactivity" (Dec 2022) vs. the Interactive tour section still showing exactly two features ~4 years later.

## Open questions

- Issue #2561 "tooltips should have an option to always show" was closed/completed (Jul 2025) — did an always-on tooltip option ship, and in which renderer?
- Does `link` work on connections/edges and containers, or shapes only? (Tour demo implies shapes; per-element-type coverage not explicitly verified.)
- Does the `@terrastruct/d2` WASM wrapper expose any render/element events for embedders? (Not documented; unchecked.)
