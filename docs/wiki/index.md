---
okf_version: "0.1"
---

# Project Knowledge Wiki

<!-- wiki-nav:start -->
## Navigation map

Auto-generated detailed index of every docs/wiki/ concept — the map the LLM uses to locate information. 76 concept(s). Regenerated on init and on wiki_mark_synced. Generated 2026-09-15T11:30:08.656Z.

Each entry: [title](concept-id.md) — description. Links are clickable in /wiki; pass the concept-id (link target minus .md) to wiki_get.

### Core concepts

- [Glossary](glossary.md) — Key terms for this project.
- [Overview](overview.md) — What this project contains and its structure.

### Architecture

- [File tree](architecture/file-tree.md) — Complete project file listing with per-file descriptions.

### Pages

- [Central chart component design](pages/artifacts/central-chart-component-design.md) — What it documents
- [Central-charts spec &amp; task list](pages/artifacts/central-charts-spec-amp-task-list.md) — The planning document for the central reusable-chart build: report pages composed of chart/block objects on a 12-col grid, with a dual-mode (Design ⇄ Code) edit
- [Central charts spec & tasks](pages/artifacts/central-charts-spec-tasks.md) — The executable spec + task list for phase 1 of the central chart system: 13 FRs (FR-1..13) broken into 13 TDD tasks across five phases — Core (spec types/valida
- [LLM Sensitive Data Privacy Research](pages/artifacts/llm-sensitive-data-privacy-research.md) — Research report (15 cited sources) on how to use LLMs with sensitive data in a data-analyst app. Compiled 2026-09-12 from web research; motivated by Data Monste
- [LLM & Sensitive Data White Paper](pages/artifacts/llm-sensitive-data-white-paper.md) — Dutch-language white paper ("LLM's & Gevoelige Data") condensing the LLM privacy research into a single self-contained HTML file designed for mobile reading.
- [LLM Sensitive-Data White Paper — Finance Edition](pages/artifacts/llm-sensitive-data-white-paper-finance-edition.md) — Non-technical (finance-audience) edition of the LLM sensitive-data white paper, in Dutch. Fully rewritten 2026-09-12 around one spine: *"wie traint er mee, en w
- [BarChart component](pages/entities/barchart-component.md) — What is it?
- [Chart fundament](pages/entities/chart-fundament.md) — Shared, tested pure-TS core under every /labs chart component — buildBars aggregation and sameDatum positional selection matching; components stay thin renderers.
- [Chart page spec (spec-types + validator)](pages/entities/chart-page-spec-spec-types-validator.md) — Central-charts FR-1: the TypeScript module holding the page document spec — `PageDoc` and all block/measure/dimension/filter/annotation/tooltip/axis types (`src
- [ChartConfigDrawer component](pages/entities/chartconfigdrawer-component.md) — A reusable drawer shell for chart configuration panels in `/labs`, hosted **inside each chart component**: a chart accepts an optional `config` snippet and togg
- [Design system (app.css tokens + /ui showcase)](pages/entities/design-system-app-css-tokens-ui-showcase.md) — The app-wide styling layer: design tokens in `src/app.css`, the `src/lib/components/` + `src/lib/components/ds/` component libraries, the `/ui` showcase page, a
- [Field Functions library](pages/entities/field-functions-library.md) — A user-extensible library of SQL field functions (e.g. formatting, extraction, math) that can be applied to table columns from the column drawer, backed by the
- [Heatmap component](pages/entities/heatmap-component.md) — Reusable SveltePlot-based heatmap component (generic `<T>`, cell grid with threshold colors), ported 2026-09-14 from the kees.pippeloi.nl reference. First compo
- [LabsPlaceholder component](pages/entities/labsplaceholder-component.md) — A one-prop Svelte 5 component that renders the standard Labs page shell with "Placeholder — coming soon." It is what every not-yet-built chart type in `/labs` s
- [Pages & master-items storage (Rust)](pages/entities/pages-master-items-storage-rust.md) — The Rust-side persistence layer for the central-charts system: three internal DuckDB tables plus the Tauri commands that read/write them. Persists report `PageD
- [remote_chat command](pages/entities/remote-chat-command.md) — Tauri command that proxies remote LLM chat completions (e.g. z.ai `/chat/completions`) through the Rust backend, streaming tokens back as `local-llm:*` events. 
- [.wiki_ignore staleness policy](pages/entities/wiki-ignore-staleness-policy.md) — Project-level additive ignore config layered on the wiki-context extension's built-in ignores.
- [Page Templates](pages/TEMPLATES.md) — Reference templates for Concept, Entity, and Artifact pages. Follow these when using wiki_note_page.

### Decisions

- [Central-charts v1 scope: bar + heatmap + table blocks; master items and auto-JOIN deferred](decisions/central-charts-v1-scope-bar-heatmap-table-blocks-master-item.md) — Context
- [Consolidate chart engines to Picasso.js + LayerChart, drop echarts/observable/svelteplot](decisions/consolidate-chart-engines-to-picasso-js.md) — Context
- [Labs chart catalog mirrors theunspokenpitch.com — scaffolded placeholder-first](decisions/labs-catalog-placeholder-first.md) — Context
- [Labs reorganized to one card per chart type; heatmap built on ported SveltePlot component](decisions/labs-per-chart-type.md) — Labs goes per-chart-type; first chart (heatmap) built on SveltePlot
- [Q6 locked: workspace-level master-item library (stable ids) with explicit table binding; Q7 open on binding depth](decisions/master-item-library-table-binding-q6.md) — Context
- [Master-items amendment: semantic layer moves early into central-charts v1](decisions/master-items-amendment-semantic-layer-moves-early-into-centr.md) — Context
- [Measures/dimensions are DuckDB expressions, not column+agg sugar (Q5, settled)](decisions/measures-dimensions-are-duckdb-expressions-not-column-agg-su.md) — Context
- [Professional-finance redesign: ledger green + Spectral/Public Sans](decisions/professional-finance-redesign-ledger.md) — Context
- [Proxy remote LLM calls through Rust, not webview fetch](decisions/proxy-remote-llm-calls-through-rust-not.md) — Context
- [Q10 locked: chart selections cross-filter other charts via re-query (Qlik-style, transient)](decisions/q10-locked-chart-selections-cross-filter-other-charts-via-re.md) — Context
- [Q11 locked: tooltips are declarative fields + one template string, shared renderer per chart type](decisions/q11-locked-tooltips-are-declarative-fields-one-template-stri.md) — Context
- [Q12 locked: page-level consistent colors via shared scale, optional manual overrides in spec](decisions/q12-locked-page-level-consistent-colors-via-shared-scale-opt.md) — Context
- [Q13 locked: explicit grid — spec declares rows, blocks take column spans (12-col)](decisions/q13-locked-explicit-grid-spec-declares-rows-blocks-take-colu.md) — Q13 locked: explicit grid — spec declares rows, blocks take column spans (12-col)
- [Q15 locked: annotation vocabulary = svelteplot basic marks (Arrow, Dot, Line, Text, Rect), per-type whitelisted](decisions/q15-locked-annotation-vocabulary-svelteplot-basic-marks-arro.md) — Context
- [Q15 locked: reference annotations are a declarative list; reference_line only in v1](decisions/q15-locked-reference-annotations-are-a-declarative-list-refe.md) — Context
- [Q15 reframed: annotations should speak svelteplot's own mark vocabulary (whitelisted per chart type)](decisions/q15-reframed-annotations-should-speak-svelteplot-s-own-mark-.md) — Context
- [Q16 locked: /pages lists &amp; creates, /page/&lt;slug&gt; hosts the dual-mode editor](decisions/q16-locked-pages-lists-amp-creates-page-lt-slug-gt-hosts-the.md) — Context
- [Q7 locked: relationship graph drives chart item availability and auto-JOIN](decisions/q7-locked-relationship-graph-drives-chart-item-availability-.md) — Context
- [Q8 locked: one canonical query engine with per-type hooks](decisions/q8-locked-one-canonical-query-engine-with-per-type-hooks.md) — Context
- [Q9 locked: chart option panels are schema-driven with a custom-panel hatch](decisions/q9-locked-chart-option-panels-are-schema-driven-with-a-custo.md) — Context
- [SveltePlot is the sole chart engine — all legacy chart libraries removed](decisions/svelteplot-sole-chart-engine.md) — Context
- [Two-surface report pages: code mode edits a declarative spec, not Svelte source](decisions/two-surface-report-page-format.md) — Q3 LOCKED (A): the report page is one declarative spec document edited by both surfaces — parity by construction. C (registry escape hatch) stays a future growth path.
- [Typography: Inter for all UI, Geist Mono reserved for data detail](decisions/typography-inter-mono-for-data.md) — Context

### Rules

- [Each /labs chart owns its config panel](rules/each-labs-chart-owns-its-config-panel.md) — Guideline
- [Feature-loop hard rules: PR-only shipping, opt-in worktrees, no force removal](rules/feature-loop-hard-rules.md) — Guideline
- [Inter for UI text, Geist Mono only for data detail](rules/inter-for-ui-text-geist-mono-only-for-data-detail.md) — Guideline
- [Interview the user one question at a time with lettered multiple-choice options](rules/interview-one-question-at-a-time.md) — Guideline
- [Keep test files and vitest imports out of src/](rules/keep-test-files-and-vitest-imports-out-of-src.md) — Keep test files and vitest imports out of src/
- [All /labs charts are built on the shared reusable-chart fundament](rules/labs-charts-reusable-fundament.md) — All /labs charts are built on the shared reusable-chart fundament
- [Pin Tailwind @source scanning to src/ and app.html in app.css](rules/pin-tailwind-source-scanning.md) — Pin Tailwind @source scanning to src/ and app.html in app.css
- [Route external API calls through Rust commands, never webview fetch](rules/route-external-api-calls-through-rust.md) — Guideline
- [Spec-driven features: TDD + Karpathy skills referenced in every todo](rules/spec-driven-features-tdd-karpathy-skills-referenced-in-every.md) — Guideline

### Learnings

- [CDP CAN click svelteplot marks — Input.dispatchMouseEvent with fresh coordinates; element.click() cannot](learnings/cdp-can-click-svelteplot-marks-input-dispatchmouseevent-with.md) — Correction to [[cdp-e2e-cannot-synthesize-trusted-clicks-on-svelteplot-marks]] — CDP `Input.dispatchMouseEvent` DOES click svelteplot marks (BarX `onclick` via 
- [CDP e2e cannot synthesize trusted clicks on svelteplot marks](learnings/cdp-e2e-cannot-synthesize-trusted-clicks-on-svelteplot-marks.md) — Symptom: chart **click-through (cross-filter selection) is untestable via CDP e2e** — synthesized clicks on svelteplot marks do nothing, even on known-good labs
- [CDP repro traps: DuckDB workspace lock pins a second instance at /; headless needs a mocked Tauri surface](learnings/cdp-repro-traps-duckdb-lock.md) — Follow-up to [[drive-data-monster-s-real-ui-over-cdp]] and [[webview2-cdp-gotchas-env-var-flag-stale]] — two more repro-environment traps hit while chasing the 
- [Chart authoring needs two surfaces (code + UI) — design must converge on a serializable chart spec](learnings/chart-authoring-two-surfaces-serializable-spec.md) — Requirement (user-stated, 2026-09-15 interview)
- ["Couldn't find callback id" Tauri warning is a benign reload artifact](learnings/couldn-t-find-callback-id-tauri-warning.md) — `[TAURI] Couldn't find callback id <n>. This might happen when the app is reloaded while Rust is running an asynchronous operation.` is benign. It appears when 
- [Drive data.monster's real UI over CDP with --remote-debugging-port for e2e debugging](learnings/drive-data-monster-s-real-ui-over-cdp.md) — The changelog-e2e skill's technique transfers from the changelog.monster app to **data.monster**: launch the Tauri app with `--remote-debugging-port` and drive 
- [Evidence.dev chart architecture: one typed component per chart type over shared machinery, consistency via a standardized prop taxonomy](learnings/evidence-chart-architecture.md) — Distilled 2026-09-15 while planning the central reusable-chart design (interview in progress; user asked to study docs.evidence.dev/components/scatter_chart and
- [get_settings merges env/.env over settings.json — env is source of truth](learnings/get-settings-merges-env-env-over.md) — Discovered while wiring `.env` into the app (2026-09-11).
- [kees.pippeloi.nl reference ports cleanly — same svelteplot 0.14.2 + Tailwind 4](learnings/kees-reference-ports-cleanly.md) — `E:\kees.pippeloi.nl` (esp. `src/routes/work/high-level`) is the reference project for chart-type components being ported into Labs.
- [Labs bar-chart "hang" is an infinite vite reconnect/reload loop, not a component bug](learnings/labs-hang-vite-reload-loop.md) — Reported 2026-09-14: clicking the bar chart card in /labs hung the page (heatmap fine). Root cause found same day: vitest import reachable from src via $lib/charts tripped vite's dep-optimizer, amplified by tailwind re-emitting app.css on any file churn. Fixed in PR #3 (commit 9f18749).
- [LLM API data retention: "no training" ≠ "no storage"; local models are ZDR by construction](learnings/llm-api-data-retention-no-training-no.md) — Research verified against primary docs (2026-09-11) on how the 6–8 major LLM API endpoints handle data retention and sensitive data. Directly relevant to Data M
- [LLM provider retention, part 2: Kimi, Z.ai, Together, Qwen — Kimi policy contradiction, Z.ai DPA strength, tier framework](learnings/llm-provider-retention-part-2-kimi-z-ai.md) — Follow-up research (2026-09-11) on Kimi (Moonshot), Z.ai (Zhipu/GLM), Together AI, and Qwen (Alibaba Model Studio), verified against primary docs. Extends [[llm
- [Local LLM blank-screen delay was hidden thinking tokens — disable via "thinking": {"type": "disabled"}](learnings/local-llm-blank-screen-delay-was-hidden.md) — Symptom
- [Never tree-scan .archive/ or src-tauri/target/ — du/find stall on the huge trees](learnings/never-tree-scan-archive-or-src-tauri.md) — The repo contains very large generated/historical trees: `.archive/` (entire superseded old app + chart-engine trials) and `src-tauri/target/` (Rust build artif
- [Stale-wiki file floods — only noise if an ignore pattern actually matches the tree](learnings/stale-wiki-file-floods-are-ignored.md) — Symptom and root cause — src-tauri/target leaked through, fixed via .wiki_ignore plus extension BUILTIN_IGNORES.
- [Stash pop can silently fail when wiki-recap writes conflict — verify and restore from the stash](learnings/stash-pop-silent-conflict-recovery.md) — Discovered 2026-09-14 while committing session work (PR #3).
- [svelteplot band axis crashes on empty aliases (duplicate key)](learnings/svelteplot-band-axis-crashes-on-empty-aliases-duplicate-key.md) — Symptom: charts crashed with a duplicate-key error in svelteplot's band axis when the central-charts page mounted.
- [SveltePlot BarX vs BarY: BarX is the horizontal bar mark](learnings/svelteplot-barx-bar-y-orientation.md) — Discovered while flipping `charts/BarChart.svelte` to horizontal (2026-09-14), confirmed against https://svelteplot.dev/examples ("Simple Bars"):
- [SveltePlot internals: match datums by position, not identity; guard empty data](learnings/svelteplot-datum-identity-empty-guard.md) — Two engine-level gotchas discovered while porting [[heatmap-component]] (2026-09-14), from the explanation of the SveltePlot 0.14.2 implementation. They apply t
- [SveltePlot ordinal domains sort alphabetically by default — set explicit domain or reverse](learnings/svelteplot-ordinal-domain-sorts-alphabetically.md) — Discovered 2026-09-14 while making the /labs bar chart sort desc: page-side data sorting had **no visible effect** because svelteplot's ordinal scales **sort th
- [SveltePlot scale bypass needs scale: null — scale: false still routes values through the scale](learnings/svelteplot-scale-null-not-false.md) — Symptom
- [WebView2 CDP gotchas: env-var flag, stale browser process, dual-stack vite](learnings/webview2-cdp-gotchas-env-var-flag-stale.md) — Follow-up to [[drive-data-monster-s-real-ui-over-cdp]] — four gotchas hit while verifying the 2026-09-12 redesign:
- [z.ai 401 "code 1000 Authentication Failed" means the key itself is bad — verify with curl, not app code](learnings/z-ai-401-code-1000-authentication.md) — Symptom
<!-- wiki-nav:end -->

An [OKF](https://github.com/earendil-works/okf) bundle documenting this project.

- [Overview](./overview.md) — What this project contains and its structure
- [File tree](./architecture/file-tree.md) — Complete project file listing
- [Glossary](./glossary.md) — Key terms for this project
- [Pages](./pages/) — Concepts, entities, and artifacts of this project
