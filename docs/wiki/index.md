---
okf_version: "0.1"
---

# Project Knowledge Wiki

<!-- wiki-nav:start -->
## Navigation map

Auto-generated detailed index of every docs/wiki/ concept — the map the LLM uses to locate information. 119 concept(s). Regenerated on init and on wiki_mark_synced. Generated 2026-09-17T05:44:37.365Z.

Each entry: [title](concept-id.md) — description. Links are clickable in /wiki; pass the concept-id (link target minus .md) to wiki_get.

### Core concepts

- [Glossary](glossary.md) — Key terms for this project.
- [Overview](overview.md) — What this project contains and its structure.

### Architecture

- [File tree](architecture/file-tree.md) — Complete project file listing with per-file descriptions.

### Pages

- [App tour set (docs/tours/)](pages/artifacts/app-tour-set-docs-tours.md) — The interactive demo-tour deliverable for all 8 app features: one standalone HTML player per feature (connect, preview, query, data-tables, pages, labs, settings, analyst), built from real-UI CDP captures — not staged mocks.
- [Central chart component design](pages/artifacts/central-chart-component-design.md) — What it documents
- [Central-charts spec &amp; task list](pages/artifacts/central-charts-spec-amp-task-list.md) — The planning document for the central reusable-chart build: report pages composed of chart/block objects on a 12-col grid, with a dual-mode (Design ⇄ Code) edit
- [Central charts spec & tasks](pages/artifacts/central-charts-spec-tasks.md) — The executable spec + task list for phase 1 of the central chart system: 13 FRs (FR-1..13) broken into 13 TDD tasks across five phases — Core (spec types/valida
- [Feature skill catalog (docs/features/)](pages/artifacts/feature-skill-catalog-docs-features.md) — What it is
- [LLM agent connection research report](pages/artifacts/llm-agent-connection-research-report.md) — Fractal-research report on how to connect any LLM / coding agent / harness to data.monster and let it operate the app — add data & content, run analysis. Produc
- [LLM Sensitive Data Privacy Research](pages/artifacts/llm-sensitive-data-privacy-research.md) — Research report (15 cited sources) on how to use LLMs with sensitive data in a data-analyst app. Compiled 2026-09-12 from web research; motivated by Data Monste
- [LLM & Sensitive Data White Paper](pages/artifacts/llm-sensitive-data-white-paper.md) — Dutch-language white paper ("LLM's & Gevoelige Data") condensing the LLM privacy research into a single self-contained HTML file designed for mobile reading.
- [LLM Sensitive-Data White Paper — Finance Edition](pages/artifacts/llm-sensitive-data-white-paper-finance-edition.md) — Non-technical (finance-audience) edition of the LLM sensitive-data white paper, in Dutch. Fully rewritten 2026-09-12 around one spine: *"wie traint er mee, en w
- [BarChart component](pages/entities/barchart-component.md) — What is it?
- [central-api (frontend invoke client)](pages/entities/central-api-frontend-invoke-client.md) — What is it?
- [Central-charts component system](pages/entities/central-charts-component-system.md) — The shipped v1 implementation of the central-charts system: the reusable component set under `src/lib/components/charts/` that renders report pages composed of 
- [Chart fundament](pages/entities/chart-fundament.md) — Shared, tested pure-TS core under every /labs chart component — buildBars aggregation and sameDatum positional selection matching; components stay thin renderers.
- [Chart page spec (spec-types + validator)](pages/entities/chart-page-spec-spec-types-validator.md) — Central-charts FR-1: the TypeScript module holding the page document spec — `PageDoc` and all block/measure/dimension/filter/annotation/tooltip/axis types (`src
- [ChartConfigDrawer component](pages/entities/chartconfigdrawer-component.md) — A reusable drawer shell for chart configuration panels, hosted **inside each chart component** in `/labs`: a chart accepts an optional `config` snippet and togg
- [database command module](pages/entities/database-command-module.md) — The Rust command module owning DuckDB **lifecycle** in the Tauri backend: initialize, graceful shutdown, and full reset. It is the code that turns a user-select
- [Design system (app.css tokens + /ui showcase)](pages/entities/design-system-app-css-tokens-ui-showcase.md) — The app-wide styling layer: design tokens in `src/app.css`, the `src/lib/components/` + `src/lib/components/ds/` component libraries, the `/ui` showcase page, a
- [Field Functions library](pages/entities/field-functions-library.md) — A user-extensible library of SQL field functions (e.g. formatting, extraction, math) that can be applied to table columns from the column drawer, backed by the
- [Heatmap component](pages/entities/heatmap-component.md) — Reusable SveltePlot-based heatmap component (generic `<T>`, cell grid with threshold colors), ported 2026-09-14 from the kees.pippeloi.nl reference. First compo
- [LabsPlaceholder component](pages/entities/labsplaceholder-component.md) — A one-prop Svelte 5 component that renders the standard Labs page shell with "Placeholder — coming soon." It is what every not-yet-built chart type in `/labs` s
- [PageGrid component](pages/entities/pagegrid-component.md) — The canvas renderer for the central-charts page editor: lays out a
- [Pages & master-items storage (Rust)](pages/entities/pages-master-items-storage-rust.md) — The Rust-side persistence layer for the central-charts system: three internal DuckDB tables plus the Tauri commands that read/write them. Persists report `PageD
- [remote_chat command](pages/entities/remote-chat-command.md) — Tauri command that proxies remote LLM chat completions (e.g. z.ai `/chat/completions`) through the Rust backend, streaming tokens back as `local-llm:*` events. 
- [.wiki_ignore staleness policy](pages/entities/wiki-ignore-staleness-policy.md) — Project-level additive ignore config layered on the wiki-context extension's built-in ignores.
- [Page Templates](pages/TEMPLATES.md) — Reference templates for Concept, Entity, and Artifact pages. Follow these when using wiki_note_page.

### Decisions

- [Agent connection: MCP server embedded in the Rust backend](decisions/agent-connection-mcp-embedded-in-rust-backend.md) — Context
- [Agent surfaces: one Rust backend serves MCP and loopback REST; ship a dm skill+CLI alongside](decisions/agent-surfaces-rust-backend-mcp-and-rest.md) — Context
- [All pages capped at 1920px and centered; full-bleed exemption removed](decisions/all-pages-capped-1920px-full-bleed-removed.md) — Context
- [Central-charts v1 scope: bar + heatmap + table blocks; master items and auto-JOIN deferred](decisions/central-charts-v1-scope-bar-heatmap-table.md) — Context
- [Consolidate chart engines to Picasso.js + LayerChart, drop echarts/observable/svelteplot](decisions/consolidate-chart-engines-to-picasso-js.md) — Context
- [Labs chart catalog mirrors theunspokenpitch.com — scaffolded placeholder-first](decisions/labs-catalog-placeholder-first.md) — Context
- [Labs reorganized to one card per chart type; heatmap built on ported SveltePlot component](decisions/labs-per-chart-type.md) — Labs goes per-chart-type; first chart (heatmap) built on SveltePlot
- [Q6 locked: workspace-level master-item library (stable ids) with explicit table binding; Q7 open on binding depth](decisions/master-item-library-table-binding-q6.md) — Context
- [Master-items amendment: semantic layer moves early into central-charts v1](decisions/master-items-amendment-semantic-layer-moves-early.md) — Context
- [Measures/dimensions are DuckDB expressions, not column+agg sugar (Q5, settled)](decisions/measures-dimensions-are-duckdb-expressions.md) — Context
- [Page editor block config: focused two-panel mode via cog icon (no selection ring)](decisions/page-editor-block-config-focused-two-panel.md) — Context
- [Page editor route nests under /pages/<slug> (was /page/<slug>); /data goes full-width](decisions/page-editor-route-nests-under-pages-slug.md) — Context
- [Page editor is tri-mode (Design / Code / Page settings); right sidebar removed — canvas shows only visualizations & data](decisions/page-editor-tri-mode-design-code-settings.md) — Context
- [Professional-finance redesign: ledger green + Spectral/Public Sans](decisions/professional-finance-redesign-ledger.md) — Context
- [Proxy remote LLM calls through Rust, not webview fetch](decisions/proxy-remote-llm-calls-through-rust-not.md) — Context
- [Q10 locked: chart selections cross-filter other charts via re-query (Qlik-style, transient)](decisions/q10-chart-selections-cross-filter-via-requery.md) — Context
- [Q11 locked: tooltips are declarative fields + one template string, shared renderer per chart type](decisions/q11-tooltips-declarative-fields-one-template.md) — Context
- [Q12 locked: page-level consistent colors via shared scale, optional manual overrides in spec](decisions/q12-page-level-consistent-colors-shared-scale.md) — Context
- [Q13 locked: explicit grid — spec declares rows, blocks take column spans (12-col)](decisions/q13-explicit-grid-rows-blocks-take-col-spans.md) — Q13 locked: explicit grid — spec declares rows, blocks take column spans (12-col)
- [Q15 locked: annotation vocabulary = svelteplot basic marks (Arrow, Dot, Line, Text, Rect), per-type whitelisted](decisions/q15-annotation-vocabulary-svelteplot-basic-marks.md) — Context
- [Q15 locked: reference annotations are a declarative list; reference_line only in v1](decisions/q15-reference-annotations-declarative-list.md) — Context
- [Q15 reframed: annotations should speak svelteplot's own mark vocabulary (whitelisted per chart type)](decisions/q15-reframed-annotations-speak-svelteplot-marks.md) — Context
- [Q16 locked: /pages lists &amp; creates, /page/&lt;slug&gt; hosts the dual-mode editor](decisions/q16-pages-lists-creates-slug-hosts-editor.md) — Context
- [Q7 locked: relationship graph drives chart item availability and auto-JOIN](decisions/q7-relationship-graph-drives-item-availability.md) — Context
- [Q8 locked: one canonical query engine with per-type hooks](decisions/q8-one-canonical-query-engine-per-type-hooks.md) — Context
- [Q9 locked: chart option panels are schema-driven with a custom-panel hatch](decisions/q9-chart-option-panels-schema-driven.md) — Context
- [SveltePlot is the sole chart engine — all legacy chart libraries removed](decisions/svelteplot-sole-chart-engine.md) — Context
- [Two-surface report pages: code mode edits a declarative spec, not Svelte source](decisions/two-surface-report-page-format.md) — Q3 LOCKED (A): the report page is one declarative spec document edited by both surfaces — parity by construction. C (registry escape hatch) stays a future growth path.
- [Typography: Bricolage Grotesque display — Poppins dropped](decisions/typography-bricolage-grotesque-display.md) — Context
- [Typography: Calluna headings, Inter body, Geist Mono data — Squada One/Libre Baskerville dropped](decisions/typography-calluna-headings-inter-body.md) — Context
- [Typography: Figtree bold display, Inter body, Geist Mono data](decisions/typography-figtree-bold-display-inter-body.md) — Context
- [Typography: Figtree headings — Calluna dropped](decisions/typography-figtree-headings-calluna-dropped.md) — Context
- [Typography: Geist display — Syne dropped](decisions/typography-geist-display-syne-dropped.md) — Context
- [Typography: Host Grotesk headings, Geist body — Inter dropped](decisions/typography-host-grotesk-headings-geist-body.md) — Typography: Host Grotesk headings, Geist body — Inter dropped
- [Typography: Inter for all UI, Geist Mono reserved for data detail](decisions/typography-inter-mono-for-data.md) — Context
- [Typography: Poppins headings — Figtree dropped](decisions/typography-poppins-headings-figtree-dropped.md) — Context
- [Typography settles: Inter everywhere (display + body), Geist Mono for data detail](decisions/typography-settles-inter-everywhere-geist-mono.md) — Context
- [Typography: Space Grotesk display, Bricolage dropped](decisions/typography-space-grotesk-display-bricolage-dropped.md) — Context
- [Typography: Squada One headings, Libre Baskerville body, Geist Mono data — Inter dropped](decisions/typography-squada-one-headings-libre-baskerville.md) — Context (superseded by [[typography-calluna-headings-inter-body]])
- [Typography: Syne display — Space Grotesk dropped](decisions/typography-syne-display-space-grotesk-dropped.md) — Typography: Syne display — Space Grotesk dropped

### Rules

- [All pages are capped at 1920px and centered by the shared layout — no per-page opt-out](rules/app-content-capped-at-shared-max-width.md) — One wrapper, .app-column, owns header + breadcrumb + content, is capped at 1920px, centered, and always exactly viewport-high with full-height side borders.
- [Card spacing comes from the grid gap, never per-card margins](rules/card-spacing-from-grid-gap-not-margins.md) — In any grid of chart/component cards (page editor canvas, labs), inter-card
- ["Demo" means an app-tour-demo UI tour, not eval suites](rules/demo-means-app-tour-demo-not-eval-suites.md) — Guideline
- [Each /labs chart owns its config panel](rules/each-labs-chart-owns-its-config-panel.md) — Guideline
- [Feature-loop hard rules: PR-only shipping, opt-in worktrees, no force removal](rules/feature-loop-hard-rules.md) — Guideline
- [Two-font rule: Inter for all UI (display + body), Geist Mono for data detail](rules/inter-for-ui-text-geist-mono-only-for-data-detail.md) — Inter everywhere for UI text; Geist Mono reserved for data detail (tables, chart ticks, tags, IDs).
- [Interview the user one question at a time with lettered multiple-choice options](rules/interview-one-question-at-a-time.md) — Guideline
- [Keep test files and vitest imports out of src/](rules/keep-test-files-and-vitest-imports-out-of-src.md) — Keep test files and vitest imports out of src/
- [All /labs charts are built on the shared reusable-chart fundament](rules/labs-charts-reusable-fundament.md) — All /labs charts are built on the shared reusable-chart fundament
- [Pin Tailwind @source scanning to src/ and app.html in app.css](rules/pin-tailwind-source-scanning.md) — Pin Tailwind @source scanning to src/ and app.html in app.css
- [Pointer cursor comes from one global rule in app.css](rules/pointer-cursor-from-global-rule-app-css.md) — Guideline
- [Route external API calls through Rust commands, never webview fetch](rules/route-external-api-calls-through-rust.md) — Guideline
- [Spec-driven features: TDD + Karpathy skills referenced in every todo](rules/spec-driven-features-tdd-karpathy-in-todos.md) — Guideline

### Learnings

- [Apparent UI bug after dev-server restarts = stale HMR webview — Ctrl+R before debugging](learnings/apparent-ui-bug-stale-hmr-webview.md) — Discovered 2026-09-15 while verifying the page-editor config drawer (cog → 50vw focused panel).
- [Auto margins in the flex-column .app-main disable flex stretch — full-bleed pages shrink without width: 100%](learnings/auto-margins-app-main-disable-flex-stretch.md) — Symptom
- [Calluna is not on Google Fonts — css2 returns 200 but silently drops it](learnings/calluna-not-on-google-fonts-css2-drops-silently.md) — Discovered 2026-09-16 while recording the Calluna/Inter retype ([[typography-calluna-headings-inter-body]]).
- [CDP CAN click svelteplot marks — Input.dispatchMouseEvent with fresh coordinates; element.click() cannot](learnings/cdp-can-click-svelteplot-marks-dispatchmouseevent.md) — Correction to [[cdp-cannot-synthesize-clicks-on-svelteplot-marks]] — CDP `Input.dispatchMouseEvent` DOES click svelteplot marks (BarX `onclick` via 
- [CDP e2e cannot synthesize trusted clicks on svelteplot marks](learnings/cdp-cannot-synthesize-clicks-on-svelteplot-marks.md) — Symptom: chart **click-through (cross-filter selection) is untestable via CDP e2e** — synthesized clicks on svelteplot marks do nothing, even on known-good labs
- [CDP repro traps: DuckDB workspace lock pins a second instance at /; headless needs a mocked Tauri surface](learnings/cdp-repro-traps-duckdb-lock.md) — Follow-up to [[drive-data-monster-s-real-ui-over-cdp]] and [[webview2-cdp-gotchas-env-var-flag-stale]] — two more repro-environment traps hit while chasing the 
- [Central-charts work lives on feature/central-charts — master is held at a restore point](learnings/central-charts-work-lives-on-feature-branch.md) — Discovered 2026-09-16 when the user reported the `/pages` work as "completely lost."
- [Chart authoring needs two surfaces (code + UI) — design must converge on a serializable chart spec](learnings/chart-authoring-two-surfaces-serializable-spec.md) — Requirement (user-stated, 2026-09-15 interview)
- ["Couldn't find callback id" Tauri warning is a benign reload artifact](learnings/couldn-t-find-callback-id-tauri-warning.md) — `[TAURI] Couldn't find callback id <n>. This might happen when the app is reloaded while Rust is running an asynchronous operation.` is benign. It appears when 
- [CSS text-transform changes innerText, not textContent — probe labels case-insensitively](learnings/css-text-transform-changes-innertext-probes.md) — Symptom: a CDP DOM probe checking for the label `"Rows"` failed on the Page
- [Drive data.monster's real UI over CDP with --remote-debugging-port for e2e debugging](learnings/drive-data-monster-s-real-ui-over-cdp.md) — The changelog-e2e skill's technique transfers from the changelog.monster app to **data.monster**: launch the Tauri app with `--remote-debugging-port` and drive 
- [Evidence.dev chart architecture: one typed component per chart type over shared machinery, consistency via a standardized prop taxonomy](learnings/evidence-chart-architecture.md) — Distilled 2026-09-15 while planning the central reusable-chart design (interview in progress; user asked to study docs.evidence.dev/components/scatter_chart and
- [Extending docs/features/ requires add-evals-to-skill's name-dir match and case pattern](learnings/extending-docs-features-requires-add-evals.md) — Constraints of add-evals-to-skill (hit while building [[feature-skill-catalog-docs-features]])
- [get_settings merges env/.env over settings.json — env is source of truth](learnings/get-settings-merges-env-env-over.md) — Discovered while wiring `.env` into the app (2026-09-11).
- [kees.pippeloi.nl reference ports cleanly — same svelteplot 0.14.2 + Tailwind 4](learnings/kees-reference-ports-cleanly.md) — `E:\kees.pippeloi.nl` (esp. `src/routes/work/high-level`) is the reference project for chart-type components being ported into Labs.
- [Labs bar-chart "hang" is an infinite vite reconnect/reload loop, not a component bug](learnings/labs-hang-vite-reload-loop.md) — Reported 2026-09-14: clicking the bar chart card in /labs hung the page (heatmap fine). Root cause found same day: vitest import reachable from src via $lib/charts tripped vite's dep-optimizer, amplified by tailwind re-emitting app.css on any file churn. Fixed in PR #3 (commit 9f18749).
- [LLM API data retention: "no training" ≠ "no storage"; local models are ZDR by construction](learnings/llm-api-data-retention-no-training-no.md) — Research verified against primary docs (2026-09-11) on how the 6–8 major LLM API endpoints handle data retention and sensitive data. Directly relevant to Data M
- [LLM provider retention, part 2: Kimi, Z.ai, Together, Qwen — Kimi policy contradiction, Z.ai DPA strength, tier framework](learnings/llm-provider-retention-part-2-kimi-z-ai.md) — Follow-up research (2026-09-11) on Kimi (Moonshot), Z.ai (Zhipu/GLM), Together AI, and Qwen (Alibaba Model Studio), verified against primary docs. Extends [[llm
- [Local LLM blank-screen delay was hidden thinking tokens — disable via "thinking": {"type": "disabled"}](learnings/local-llm-blank-screen-delay-was-hidden.md) — Symptom
- [Never tree-scan .archive/ or src-tauri/target/ — du/find stall on the huge trees](learnings/never-tree-scan-archive-or-src-tauri.md) — The repo contains very large generated/historical trees: `.archive/` (entire superseded old app + chart-engine trials) and `src-tauri/target/` (Rust build artif
- [PageDoc has block.title AND chart.title — charts render only chart.title; inspector must write there](learnings/pagedoc-block-title-and-chart-title-rendering.md) — In the central-charts [[chart-page-spec-spec-types-validator]] `PageDoc`, a block carries a **block-level `title`** *and* (for chart blocks) **`chart.title` / `
- [Settings-swap for tours must cover .env too, and the app webview must never navigate off-origin](learnings/settings-swap-for-tours-must-cover-env-too.md) — Discovered 2026-09-16 building the settings-tour + analyst-tour (docs/tours/RUNBOOK.md CRITICAL section).
- [settings-tour and analyst-tour built — honest-beats-staged applied to the chat](learnings/settings-tour-and-analyst-tour-built.md) — Built 2026-09-16 — settings-tour and analyst-tour complete the 8-tour set in docs/tours/ (connect, preview, query, data-tables, pages, labs, settings, analyst).
- [Squada One is single-weight (400) — heading font-weight 600/700 gets browser-synthesized bold](learnings/squada-one-is-single-weight-400.md) — Discovered 2026-09-16 while re-typing the app ([[typography-squada-one-headings-libre-baskerville]]).
- [Stale-wiki file floods — only noise if an ignore pattern actually matches the tree](learnings/stale-wiki-file-floods-are-ignored.md) — Symptom and root cause — src-tauri/target leaked through, fixed via .wiki_ignore plus extension BUILTIN_IGNORES.
- [Stash pop can silently fail when wiki-recap writes conflict — verify and restore from the stash](learnings/stash-pop-silent-conflict-recovery.md) — Discovered 2026-09-14 while committing session work (PR #3).
- [svelteplot band axis crashes on empty aliases (duplicate key)](learnings/svelteplot-band-axis-empty-aliases-crash.md) — Symptom: charts crashed with a duplicate-key error in svelteplot's band axis when the central-charts page mounted.
- [SveltePlot BarX vs BarY: BarX is the horizontal bar mark](learnings/svelteplot-barx-bar-y-orientation.md) — Discovered while flipping `charts/BarChart.svelte` to horizontal (2026-09-14), confirmed against https://svelteplot.dev/examples ("Simple Bars"):
- [SveltePlot internals: match datums by position, not identity; guard empty data](learnings/svelteplot-datum-identity-empty-guard.md) — Two engine-level gotchas discovered while porting [[heatmap-component]] (2026-09-14), from the explanation of the SveltePlot 0.14.2 implementation. They apply t
- [SveltePlot ordinal domains sort alphabetically by default — set explicit domain or reverse](learnings/svelteplot-ordinal-domain-sorts-alphabetically.md) — Discovered 2026-09-14 while making the /labs bar chart sort desc: page-side data sorting had **no visible effect** because svelteplot's ordinal scales **sort th
- [SveltePlot scale bypass needs scale: null — scale: false still routes values through the scale](learnings/svelteplot-scale-null-not-false.md) — Symptom
- [Tour DOM snapshots scale with the live DOM — bound secondary frames, keep one big frame when size is the story](learnings/tour-dom-snapshots-scale-with-live-dom.md) — Discovered 2026-09-16 finishing the query-tour (see [[app-tour-set-docs-tours]]).
- [Tour HTML captures embed the Google-Fonts @import — font changes require recapturing tours](learnings/tour-html-captures-embed-google-fonts-import.md) — Discovered 2026-09-16 while re-typing the app ([[typography-squada-one-headings-libre-baskerville]]).
- [Visibility probes must walk the ancestor opacity/display/visibility chain — an opacity:0 parent hides everything](learnings/visibility-probes-walk-ancestor-opacity-chain.md) — CDP "visibility" checks lied twice on the page-editor config drawer (2026-09-15):
- [WebView2 CDP gotchas: env-var flag, stale browser process, dual-stack vite](learnings/webview2-cdp-gotchas-env-var-flag-stale.md) — Follow-up to [[drive-data-monster-s-real-ui-over-cdp]] — four gotchas hit while verifying the 2026-09-12 redesign:
- [wiki_note_page wikilinks resolve ./-relative to the page's own folder — cross-folder links need explicit paths](learnings/wiki-note-page-wikilinks-resolve-relative.md) — Discovered 2026-09-16 while writing the [[feature-skill-catalog-docs-features]] artifact page.
- [z.ai 401 "code 1000 Authentication Failed" means the key itself is bad — verify with curl, not app code](learnings/z-ai-401-code-1000-authentication.md) — Symptom

### Preferences

- [Never start npm run dev / tauri dev — the user owns the dev app](preferences/never-start-npm-run-dev-tauri-dev.md) — The LLM must never launch the dev app itself — no `npm run dev`, `npx tauri dev`, or background dev-server starts. The user starts and owns the dev app.
<!-- wiki-nav:end -->

An [OKF](https://github.com/earendil-works/okf) bundle documenting this project.

- [Overview](./overview.md) — What this project contains and its structure
- [File tree](./architecture/file-tree.md) — Complete project file listing
- [Glossary](./glossary.md) — Key terms for this project
- [Pages](./pages/) — Concepts, entities, and artifacts of this project
