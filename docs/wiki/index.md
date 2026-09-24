---
okf_version: "0.1"
---

# Project Knowledge Wiki

<!-- wiki-nav:start -->
## Navigation map

Auto-generated detailed index of every docs/wiki/ concept — the map the LLM uses to locate information. 237 concept(s). Regenerated on init and on wiki_mark_synced. Generated 2026-09-23T05:34:31.499Z.

Each entry: [title](concept-id.md) — description. Links are clickable in /wiki; pass the concept-id (link target minus .md) to wiki_get.

### Core concepts

- [Glossary](glossary.md) — Key terms for this project.
- [Overview](overview.md) — What this project contains and its structure.

### Architecture

- [File tree](architecture/file-tree.md) — Complete project file listing with per-file descriptions.

### Pages

- [aisure.uk pricing research report](pages/artifacts/aisure-uk-pricing-research-report.md) — Fractal-research (te9-research skill, `recursive_research`, depth 1, 3 leaves) answering a standalone question — not app-internal research: *why is https://aisu
- [App tour set (docs/tours/)](pages/artifacts/app-tour-set-docs-tours.md) — The interactive demo-tour deliverable for all 8 app features: one standalone HTML player per feature (connect, preview, query, data-tables, pages, labs, settings, analyst), built from real-UI CDP captures — not staged mocks.
- [Central chart component design](pages/artifacts/central-chart-component-design.md) — What it documents
- [Central-charts spec &amp; task list](pages/artifacts/central-charts-spec-amp-task-list.md) — The planning document for the central reusable-chart build: report pages composed of chart/block objects on a 12-col grid, with a dual-mode (Design ⇄ Code) edit
- [Central charts spec & tasks](pages/artifacts/central-charts-spec-tasks.md) — The executable spec + task list for phase 1 of the central chart system: 13 FRs (FR-1..13) broken into 13 TDD tasks across five phases — Core (spec types/valida
- [Design component reference (docs/design/components/)](pages/artifacts/design-component-reference-docs-design-components.md) — What is it?
- [Design-component reference set (docs/design/components/)](pages/artifacts/design-component-reference-set.md) — 40 standalone per-component design-reference HTML pages (Button, Modal, Table, Searchahead, …) — static explorations in their own token set, not app components.
- [Design-system reference doc (docs/design-system-data-monster.html)](pages/artifacts/design-system-reference-doc-docs-design.md) — The standalone design-system documentation deliverable: a single self-contained HTML file rendering the app's current tokens, typography, color ramps, and compo
- [Feature skill catalog (docs/features/)](pages/artifacts/feature-skill-catalog-docs-features.md) — What it is
- [LLM agent connection research report](pages/artifacts/llm-agent-connection-research-report.md) — Fractal-research report on how to connect any LLM / coding agent / harness to data.monster and let it operate the app — add data & content, run analysis. Produc
- [LLM Sensitive Data Privacy Research](pages/artifacts/llm-sensitive-data-privacy-research.md) — Research report (15 cited sources) on how to use LLMs with sensitive data in a data-analyst app. Compiled 2026-09-12 from web research; motivated by Data Monste
- [LLM & Sensitive Data White Paper](pages/artifacts/llm-sensitive-data-white-paper.md) — Dutch-language white paper ("LLM's & Gevoelige Data") condensing the LLM privacy research into a single self-contained HTML file designed for mobile reading.
- [LLM Sensitive-Data White Paper — Finance Edition](pages/artifacts/llm-sensitive-data-white-paper-finance-edition.md) — Non-technical (finance-audience) edition of the LLM sensitive-data white paper, in Dutch. Fully rewritten 2026-09-12 around one spine: *"wie traint er mee, en w
- [OSS value driver trees research report](pages/artifacts/oss-value-driver-trees-research-report.md) — What it documents
- [Pages E2E feedback report](pages/artifacts/pages-e2e-feedback-report.md) — E2E test report for the /pages report-page flow built on the semantic (master-item) layer: `reports/pages-e2e-feedback.md`, produced 2026-09-22 by driving the r
- [Workspace-file-first spec & tasks](pages/artifacts/workspace-file-first-spec-tasks.md) — The executable spec + task list for the workspace-file-first build: 13 FRs (FR-1..13) across four phases — A Files-are-canonical (`dm_store` module, pages/maste
- [Agent docs module (agent_docs.rs)](pages/entities/agent-docs-module-agent-docs-rs.md) — Agent docs module (agent_docs.rs)
- [Agent docs system](pages/entities/agent-docs-system.md) — The repo-authored documentation layer that teaches coding agents to operate the app by editing workspace files. Markdown sources live at `src-tauri/agent-docs/`
- [Agent prompts page](pages/entities/agent-prompts-page.md) — The SvelteKit route `/agent` (`src/routes/agent/+page.svelte`): six curated copy-paste prompts that teach a coding agent to operate data.monster through the wor
- [App tab system (virtual tabs + bottom tab bar)](pages/entities/app-tab-system-virtual-tabs-bottom-tab-bar.md) — The app's browser-like tab system: right-click an internal link → "Open in new tab"; the bottom bar lists the open tabs. Tabs are **virtual** — plain routes tra
- [BarChart component](pages/entities/barchart-component.md) — What is it?
- [central-api (frontend invoke client)](pages/entities/central-api-frontend-invoke-client.md) — What is it?
- [Central-charts component system](pages/entities/central-charts-component-system.md) — The shipped v1 implementation of the central-charts system: the reusable component set under `src/lib/components/charts/` that renders report pages composed of 
- [Chart fundament](pages/entities/chart-fundament.md) — Shared, tested pure-TS core under every /labs chart component — buildBars aggregation and sameDatum positional selection matching; components stay thin renderers.
- [Chart page spec (spec-types + validator)](pages/entities/chart-page-spec-spec-types-validator.md) — Central-charts FR-1: the PageDoc data contract (spec-types.ts + validate.ts) - columned rows (PageColumn span/height), blocks, measures/dimensions, rowColumns/normalizePageDoc
- [ChartConfigDrawer component](pages/entities/chartconfigdrawer-component.md) — A reusable drawer shell for chart configuration panels, hosted **inside each chart component** in `/labs`: a chart accepts an optional `config` snippet and togg
- [Connections file store (connections.rs)](pages/entities/connections-file-store-connections-rs.md) — The Rust content-command module that backs **saved PostgreSQL connections** — task `files-005` of the workspace-file-first build (commit `fb2f6fb`, 2026-09-22).
- [Create-in-/data round-trip](pages/entities/create-in-data-round-trip.md) — Deep-link flow from a /pages chart's pick surfaces to the full master-item editor in /data and back: chart → /data?tab=<kind>s&add=1&table=…&return=<slug>&block=<id> → ItemEditor preset form → save → /pages/<slug>?configure=<block>&attach=<itemId> → item attached to the chart + focused drawer reopened.
- [data/incoming drop folder (incoming.rs)](pages/entities/data-incoming-drop-folder-incoming-rs.md) — data/incoming drop folder (incoming.rs)
- [database command module](pages/entities/database-command-module.md) — The Rust command module owning DuckDB **lifecycle** in the Tauri backend: initialize, graceful shutdown, and full reset. It is the code that turns a user-select
- [Design system (app.css tokens + /ui showcase)](pages/entities/design-system-app-css-tokens-ui-showcase.md) — The app-wide styling layer: design tokens in `src/app.css`, the `src/lib/components/` + `src/lib/components/ds/` component libraries, the `/ui` showcase page, a
- [dev-cdp.cmd (repo-root double-click CDP restart)](pages/entities/dev-cdp-cmd-repo-root-double-click-cdp-restart.md) — `dev-cdp.cmd` is a double-clickable Windows command script at the repo root that restarts the dev app in a CDP-drivable state — the packaged version of the manu
- [dm-events frontend module](pages/entities/dm-events-frontend-module.md) — `src/lib/dm-events.ts` — the frontend half of the dm/ live-reload loop (workspace-file-first task `files-008`, commit `d7f0fc7`): one Tauri event bus for `dm:ch
- [dm_store command module](pages/entities/dm-store-command-module.md) — `src-tauri/src/commands/dm_store.rs` — the Rust foundation module for the workspace-file-first architecture: dm/ path conventions, name validation, atomic write
- [dm_watch command module](pages/entities/dm-watch-command-module.md) — `src-tauri/src/commands/dm_watch.rs` — the Rust file watcher for the workspace `dm/` tree (task `files-007`, commit `72bb92e`, 2026-09-22, branch `feature/works
- [E2E CDP driver](pages/entities/e2e-cdp-driver.md) — `e2e/cdp.mjs` — the committed CLI driver for e2e-verifying data.monster's real UI over the Chrome DevTools Protocol (port 9223), node >= 21 native WebSocket, no dependencies.
- [ExprEditor component](pages/entities/expreditor-component.md) — Smart DuckDB expression editor for master items (Qlik-Sense-style): autocomplete over bound-table fields, master items and a curated DuckDB function catalog, SQL syntax highlighting, per-kind starter templates, and live validation + result preview against the bound table.
- [Field Functions library](pages/entities/field-functions-library.md) — A user-extensible library of SQL field functions (e.g. formatting, extraction, math) that can be applied to table columns from the column drawer, backed by the
- [Heatmap component](pages/entities/heatmap-component.md) — Reusable SveltePlot-based heatmap component (generic `<T>`, cell grid with threshold colors), ported 2026-09-14 from the kees.pippeloi.nl reference. First compo
- [Incoming drop folder](pages/entities/incoming-drop-folder.md) — The `data/incoming/` drop folder in the workspace (workspace-file-first FR-13, the "one cuttable piece" that shipped): drop a CSV, Parquet, or JSON file in and 
- [LabsPlaceholder component](pages/entities/labsplaceholder-component.md) — The shared Svelte 5 placeholder shell that renders a section page with "Placeholder — coming soon." Every not-yet-built chart type in `/labs` shows it, and other not-yet-built sections (e.g. `/library`) reuse it via the `section` prop.
- [library-component-builder skill (.pi/skills)](pages/entities/library-component-builder-skill-pi-skills.md) — A pi project skill (agentskills.io-spec-conformant) that owns the full path from a user's component idea to a registered, tested library component: interview → 
- [Library page (/library)](pages/entities/library-page-library.md) — A new top-level route intended to become the **central component library**: every component used in the app's UI shown in one place, where component devs regist
- [Library registry system (src/lib/library + /library routes)](pages/entities/library-registry-system.md) — The shipped implementation of the library registry: a one-function registration point (`registerLibraryComponent`) that feeds both the `/library` views and the 
- [LLM prompt button (/library detail)](pages/entities/llm-prompt-button-library-detail.md) — LLM prompt button (/library detail)
- [Master items & relationships file store (items.rs + relationships.rs)](pages/entities/master-items-relationships-file-store-items-rs.md) — The Rust content-command modules that back master items and the relationship graph as **files in the `dm/` workspace tree** — task `files-003` of the workspace-
- [PageGrid component](pages/entities/pagegrid-component.md) — The canvas renderer + editing surface of the central-charts page editor: lays out a `PageDoc` as rows of 12-col CSS grids — each row an optional-height shell of
- [Pages & master-items storage (Rust)](pages/entities/pages-master-items-storage-rust.md) — The Rust-side persistence layer for the central-charts system: three internal DuckDB tables plus the Tauri commands that read/write them. Persists report `PageD
- [remote_chat command](pages/entities/remote-chat-command.md) — Tauri command that proxies remote LLM chat completions (e.g. z.ai `/chat/completions`) through the Rust backend, streaming tokens back as `local-llm:*` events. 
- [RolePickerModal component](pages/entities/rolepickermodal-component.md) — The pick/create modal opened from the SkeletonSetup card buttons (new-page-modal pattern): a searchable list over ⭐ master items, source-table fields, and linke
- [Saved queries file store (saved_queries.rs)](pages/entities/saved-queries-file-store-saved-queries-rs.md) — The Rust content-command module that backs saved queries as **plain `.sql` files in the `dm/saved-queries/` workspace tree** — task `files-004` of the workspace
- [Shared controls kit (charts/controls)](pages/entities/shared-controls-kit-charts-controls.md) — The shared form-controls kit for every drawer, inspector, and modal surface in the app: nine small Svelte 5 components plus one CSS file, all built on the app's
- [SkeletonSetup component](pages/entities/skeletonsetup-component.md) — The in-chart setup card rendered inside a `ChartCard` when `needsSetup(chart)` is true: a card button per unmet role opens the RolePickerModal (searchable picks over ⭐ master items | source-table fields | linked-table fields, + New) — the common configuration path never opens the config drawer.
- [.wiki_ignore staleness policy](pages/entities/wiki-ignore-staleness-policy.md) — Project-level additive ignore config layered on the wiki-context extension's built-in ignores.
- [Workspace command module (Rust)](pages/entities/workspace-command-module-rust.md) — The Rust command module owning workspace identity — picking, persisting (workspace.json + open history), and resolving the active workspace folder.
- [Workspaces page (/workspaces)](pages/entities/workspaces-page-workspaces.md) — Dedicated workspace-switcher page at /workspaces — header folder button lands here; every workspace ever opened shows as a chip, and a chip click runs the full portable-workspace reload.
- [Write-through core (src/lib/write-through.ts)](pages/entities/write-through-core-src-lib-write-through-ts.md) — A pure, Svelte-free write-through state machine — "the file IS the save" (workspace-file-first FR-9, task `files-009`, commit `05d6ea1`, 2026-09-22). All deboun
- [Page Templates](pages/TEMPLATES.md) — Reference templates for Concept, Entity, and Artifact pages. Follow these when using wiki_note_page.

### Decisions

- [Agent authors app content by editing workspace files — the workspace folder is the interface (proposed)](decisions/agent-authors-app-content-by-editing-workspace.md) — Context
- [Agent connection: MCP server embedded in the Rust backend](decisions/agent-connection-mcp-embedded-in-rust-backend.md) — Context
- [Agent surfaces: one Rust backend serves MCP and loopback REST; ship a dm skill+CLI alongside](decisions/agent-surfaces-rust-backend-mcp-and-rest.md) — Context
- [All drawers adopt the /data (TableDrawer) design pattern — DrawerTabs removed](decisions/all-drawers-adopt-the-data-tabledrawer-design.md) — Context
- [All pages capped at 1920px and centered; full-bleed exemption removed](decisions/all-pages-capped-1920px-full-bleed-removed.md) — Context
- [App gets virtual multi-tab navigation: bottom bar is the tab bar](decisions/app-gets-virtual-multi-tab-navigation-bottom-bar.md) — Context
- [Central-charts v1 scope: bar + heatmap + table blocks; master items and auto-JOIN deferred](decisions/central-charts-v1-scope-bar-heatmap-table.md) — Context
- [Chart blocks start empty — data renders only when role requirements are met (needsSetup gate)](decisions/chart-blocks-start-empty-needssetup-gate.md) — Context
- [Consolidate chart engines to Picasso.js + LayerChart, drop echarts/observable/svelteplot](decisions/consolidate-chart-engines-to-picasso-js.md) — Context
- [Creation saves explicitly, editing writes through live (files-010)](decisions/creation-saves-explicitly-editing-writes-through.md) — Context
- [dm/ migration: write all, verify, then drop — files win](decisions/dm-migration-write-all-verify-then-drop-files-win.md) — Context
- [Drawer chrome restyle reverted — control kit stands, lms/kees motifs rejected](decisions/drawer-chrome-restyle-reverted-control-kit-stands.md) — Context
- [Labs chart catalog mirrors theunspokenpitch.com — scaffolded placeholder-first](decisions/labs-catalog-placeholder-first.md) — Context
- [Labs reorganized to one card per chart type; heatmap built on ported SveltePlot component](decisions/labs-per-chart-type.md) — Labs goes per-chart-type; first chart (heatmap) built on SveltePlot
- [/library becomes the central component library (proposed — spec interview in progress)](decisions/library-central-component-library.md) — Context
- [library-component-builder skill is the canonical path for new library components](decisions/library-component-builder-canonical-path.md) — Context
- [Library demos render the real components fed dummy query-shaped data — no demo-only clones](decisions/library-demos-reuse-real-components.md) — Context
- [Library components are self-contained extension-style packages — own definition, logic, and data](decisions/library-extension-style-components.md) — Context
- [Library packages carry blockKind — table/text are built-in blocks, not chart types](decisions/library-packages-carry-blockkind.md) — Context
- [Library Q2: registry v1 is display-only — editor wiring deferred](decisions/library-q2-registry-display-only.md) — Context
- [Library Q3: /library layout is master-detail — left index + full-size live demo with schema alongside](decisions/library-q3-master-detail-layout.md) — Context
- [Library Q4: component demos get dedicated views, split into tabs — Preview is the default tab](decisions/library-q4-dedicated-tabbed-views.md) — Context
- [Library registry drives editor + /library in one shot (supersedes display-only v1)](decisions/library-registry-drives-editor-and-library.md) — Context
- [Library registry v1 lives as a TypeScript module under src/lib/library/ with self-contained component folders](decisions/library-registry-ts-module.md) — Context
- [Linked-table raw fields are transient with auto-JOIN — master-item creation stays optional](decisions/linked-table-raw-fields-transient-autojoin.md) — Context
- [Live-reload mechanics: notify watcher → validate → dm:changed/dm:error events](decisions/live-reload-mechanics-notify-watcher-validate-dm.md) — Shipped as files-007 (2026-09-22): notify watcher + 300ms debounce + writer-fed echo suppression emits dm:changed/dm:error; frontend listeners shipped as files-008 (2026-09-22): dm-events bus + six views live-reload; clobber banner shipped as files-009 (write-through core).
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
- [Skeleton card is the inline role-assignment surface — pick/create in-chart, drawer optional](decisions/skeleton-card-is-the-inline-role-assignment.md) — Context
- [Skeleton pick/create moved from inline dropdowns to card buttons opening a modal (searchahead + New)](decisions/skeleton-pick-create-moved-from-inline-dropdowns.md) — Skeleton role-assignment: card buttons open a pick/create modal (searchahead + New)
- [Use speed-highlight/core for code highlighting instead of Prism](decisions/speed-highlight-over-prism.md) — Context
- [SveltePlot is the sole chart engine — all legacy chart libraries removed](decisions/svelteplot-sole-chart-engine.md) — Context
- [Tab bar shows only explicitly opened tabs — navigation never creates tabs](decisions/tab-bar-shows-only-explicitly-opened-tabs.md) — Context
- [Timeout and retry defend against hung IPC](decisions/timeout-and-retry-defend-against-hung-ipc.md) — Context
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
- [Workspace content tree: dm/ with path-is-identity, native formats, agent README (proposed)](decisions/workspace-content-tree-dm-with-path-is-identity.md) — Context
- [Workspace files are canonical — agents author content by editing the dm/ tree in realtime](decisions/workspace-files-are-canonical-agents-author.md) — Context
- [Workspaces are fully portable — switching reloads data, content, and settings](decisions/workspaces-are-fully-portable-switch-reloads.md) — Context

### Rules

- [Adding a component never auto-opens the config drawer — skeleton is the start state; drawer-open seeds picker rows](rules/adding-a-component-never-auto-opens-the-config.md) — When a component is added to a page-editor row (`addComponent` in the page editor), the **config drawer must NOT auto-open**. The newly added block stays on the
- [All pages are capped at 1920px and centered by the shared layout — no per-page opt-out](rules/app-content-capped-at-shared-max-width.md) — One wrapper, .app-column, owns header + breadcrumb + content, is capped at 1920px, centered, and always exactly viewport-high with full-height side borders.
- [Card spacing comes from the grid gap, never per-card margins](rules/card-spacing-from-grid-gap-not-margins.md) — In any grid of chart/component cards (page editor canvas, labs), inter-card
- [Config drawers are one scrolling column — settings sections, Danger zone last](rules/config-drawers-one-scrolling-column.md) — Guideline
- ["Demo" means an app-tour-demo UI tour, not eval suites](rules/demo-means-app-tour-demo-not-eval-suites.md) — Guideline
- [dm/ live-reload wires through dm-events — never a raw listen()](rules/dm-live-reload-wires-through-dm-events.md) — **The rule**: any view or component that renders `dm/`-managed content subscribes to live changes through `src/lib/dm-events.ts` — `onDmChanged(kind, () => void
- [Drawer form controls come from the shared controls kit — never hand-roll input chrome](rules/drawer-form-controls-come-from-the-shared-controls.md) — Guideline
- [Drawers reuse the shared drawerResize action](rules/drawers-reuse-the-shared-drawerresize-action.md) — Guideline
- [Each /labs chart owns its config panel](rules/each-labs-chart-owns-its-config-panel.md) — Guideline
- [Feature-loop hard rules: PR-only shipping, opt-in worktrees, no force removal](rules/feature-loop-hard-rules.md) — Guideline
- [Two-font rule: Inter for all UI (display + body), Geist Mono for data detail](rules/inter-for-ui-text-geist-mono-only-for-data-detail.md) — Inter everywhere for UI text; Geist Mono reserved for data detail (tables, chart ticks, tags, IDs).
- [Interview the user one question at a time with lettered multiple-choice options](rules/interview-one-question-at-a-time.md) — Guideline
- [Keep test files and vitest imports out of src/](rules/keep-test-files-and-vitest-imports-out-of-src.md) — Keep test files and vitest imports out of src/
- [All /labs charts are built on the shared reusable-chart fundament](rules/labs-charts-reusable-fundament.md) — All /labs charts are built on the shared reusable-chart fundament
- [Leave wiki-recap noise uncommitted — branch fresh and commit selectively, never stash](rules/leave-wiki-recap-noise-uncommitted.md) — Guideline
- [Pick display labels resolve through roleLabels() — never hand-roll chip labels](rules/pick-display-labels-resolve-through-rolelabels.md) — Guideline
- [Pick values flow through one codec — src/lib/charts/pickers.ts](rules/pick-values-flow-through-one-codec-src-lib-charts.md) — Guideline
- [Pin Tailwind @source scanning to src/ and app.html in app.css](rules/pin-tailwind-source-scanning.md) — Pin Tailwind @source scanning to src/ and app.html in app.css
- [Pointer cursor comes from one global rule in app.css](rules/pointer-cursor-from-global-rule-app-css.md) — Guideline
- [Render markdown via marked + .prose-chat, never a new pipeline](rules/render-markdown-via-marked-prose-chat.md) — When rendering any markdown anywhere in the app (docs tabs, chat, notes), parse with `marked` (already a dependency) and wrap the output in the `.prose-chat` cl
- [Resize requests use the app's existing size classes — never ad-hoc multipliers](rules/resize-requests-use-existing-size-classes.md) — Resize requests map onto the app's existing size classes — never invent ad-hoc pixel multipliers.
- [Restore points are git tags restore-point/<feature>-start on pushed master HEAD](rules/restore-points-are-git-tags-restore-point.md) — The rule
- [Route external API calls through Rust commands, never webview fetch](rules/route-external-api-calls-through-rust.md) — Guideline
- [Rust backend tests live in-module via #[cfg(test)]](rules/rust-backend-tests-live-in-module-via-cfg-test.md) — The rule
- [Secrets never live in workspace content files — env references only, gitignored .env at all times](rules/secrets-never-live-in-workspace-content-files.md) — The rule (2026-09-22, user-set during the workspace-file-first design): workspaces can be git/version controlled, so every file the app writes into a workspace 
- [Single-doc stores fail loud, never clobber — corrupt file: list errors, save refuses](rules/single-doc-stores-fail-loud-never-clobber.md) — **The rule**: any store backed by a single agent-editable doc file (e.g. `dm/relationships.json`, later the connections store) must fail loud instead of clobber
- [Spec-driven features: TDD + Karpathy skills referenced in every todo](rules/spec-driven-features-tdd-karpathy-in-todos.md) — Guideline

### Learnings

- [A dm/ write that bypasses dm_store::atomic_write reload-loops — echo suppression is fed by the writer](learnings/a-dm-write-that-bypasses-dm-store-atomic-write.md) — Shipped in `files-007` (2026-09-22): the `dm/` watcher skips any path recorded by `dm_watch::mark_self_write` — and the only caller is `dm_store::atomic_write`,
- [Apparent UI bug after dev-server restarts = stale HMR webview — Ctrl+R before debugging](learnings/apparent-ui-bug-stale-hmr-webview.md) — Discovered 2026-09-15 while verifying the page-editor config drawer (cog → 50vw focused panel).
- [Auto margins in the flex-column .app-main disable flex stretch — full-bleed pages shrink without width: 100%](learnings/auto-margins-app-main-disable-flex-stretch.md) — Symptom
- [Bash heredoc writes mangle non-ASCII — patch with python explicit escapes, and verify bytes before assuming corruption](learnings/bash-heredoc-writes-mangle-non-ascii-patch-with.md) — Hit twice while rewiring the drawers (PR #18, 2026-09-17).
- [Calluna is not on Google Fonts — css2 returns 200 but silently drops it](learnings/calluna-not-on-google-fonts-css2-drops-silently.md) — Discovered 2026-09-16 while recording the Calluna/Inter retype ([[typography-calluna-headings-inter-body]]).
- [CDP CAN click svelteplot marks — Input.dispatchMouseEvent with fresh coordinates; element.click() cannot](learnings/cdp-can-click-svelteplot-marks-dispatchmouseevent.md) — Correction to [[cdp-cannot-synthesize-clicks-on-svelteplot-marks]] — CDP `Input.dispatchMouseEvent` DOES click svelteplot marks (BarX `onclick` via 
- [CDP e2e cannot synthesize trusted clicks on svelteplot marks](learnings/cdp-cannot-synthesize-clicks-on-svelteplot-marks.md) — Symptom: chart **click-through (cross-filter selection) is untestable via CDP e2e** — synthesized clicks on svelteplot marks do nothing, even on known-good labs
- [CDP context-menu e2e: real right-click dispatch, and check the binding before blaming synthetic events](learnings/cdp-context-menu-e2e-real-right-click-dispatch-and.md) — Discovered 2026-09-17 shipping PR #16 (virtual tab system, CDP e2e steps 1–7). Extends the synthetic-event family: [[cdp-can-click-svelteplot-marks-dispatchmous
- [CDP e2e failures right after a source save are often HMR races — re-run before debugging](learnings/cdp-e2e-failures-after-source-save-hmr-race.md) — Discovered 2026-09-22 while debugging the /data tab URL-sync bug (verified over CDP, port 9223).
- [CDP form probes must be container-scoped — shared placeholders between list rows and create forms cause silent wrong-input traps](learnings/cdp-form-probes-must-be-container-scoped-shared.md) — Discovered 2026-09-17 while CDP-testing master-item creation (page editor measure form).
- [CDP gate assertions need settle time after doc mutations, and svg counts must be chart-scoped](learnings/cdp-gate-assertions-need-settle-time-after-doc.md) — Two CDP-e2e traps hit while testing the needsSetup gate (2026-09-17, PR #10):
- [CDP probe `$` is querySelector — indexing it silently kills clicks](learnings/cdp-probe-is-queryselector-indexing-it-silently.md) — Discovered 2026-09-17 shipping PR #12 (skeleton pick/create modal, CDP e2e steps 1–7).
- [CDP repro traps: DuckDB workspace lock pins a second instance at /; headless needs a mocked Tauri surface](learnings/cdp-repro-traps-duckdb-lock.md) — Follow-up to [[drive-data-monster-s-real-ui-over-cdp]] and [[webview2-cdp-gotchas-env-var-flag-stale]] — two more repro-environment traps hit while chasing the 
- [Central-charts work lives on feature/central-charts — master is held at a restore point](learnings/central-charts-work-lives-on-feature-branch.md) — Discovered 2026-09-16 when the user reported the `/pages` work as "completely lost."
- [Chart authoring needs two surfaces (code + UI) — design must converge on a serializable chart spec](learnings/chart-authoring-two-surfaces-serializable-spec.md) — Requirement (user-stated, 2026-09-15 interview)
- [Chart segment selection is parent-held {dimension, value} transient state](learnings/chart-segment-selection-parent-held-state.md) — Chart components (as used in `/pages` and the library detail page demo) manage their own click/deselect handlers once selection state exists. The wiring only ne
- [compile.ts dimension guard: raw flag is the only validation bypass — never blanket-catch checkColumn failures](learnings/compile-ts-dimension-guard-raw-flag-only-bypass.md) — Symptom
- [Component spawn grows too-small explicit-height rows to 320px minimum](learnings/component-spawn-grows-too-small-explicit-height.md) — Discovered 2026-09-17 while verifying the page-editor skeleton-clip bug (fixed in PR #9, 1 file +8).
- ["Couldn't find callback id" Tauri warning is a benign reload artifact](learnings/couldn-t-find-callback-id-tauri-warning.md) — `[TAURI] Couldn't find callback id <n>. This might happen when the app is reloaded while Rust is running an asynchronous operation.` is benign. It appears when 
- [CSS text-transform changes innerText, not textContent — probe labels case-insensitively](learnings/css-text-transform-changes-innertext-probes.md) — Symptom: a CDP DOM probe checking for the label `"Rows"` failed on the Page
- [D2 diagrams are not interactive — tooltip and external link only; base64url shape classes are the DIY hook](learnings/d2-diagrams-not-interactive.md) — Question
- [/data tab keys ≠ labels — "Metadata" writes ?tab=definitions](learnings/data-tab-keys-labels-metadata-writes-definitions.md) — Discovered 2026-09-22 while CDP-verifying the /data tab URL sync (port 9223): a probe matching tabs by `textContent.includes('Definitions')` never matched — the
- [dm:changed with no subscriber silently no-ops — incoming ingest's kind:"table" had zero listeners](learnings/dm-changed-with-no-subscriber-silently-no-ops.md) — Found by the workspace-file-first e2e pass (2026-09-22): the `data/incoming/` ingest correctly emitted `dm:changed {kind:"table"}` via the [[dm-watch-command-mo
- [dm live-reload events don't replay — e2e must navigate first, then write the file](learnings/dm-live-reload-events-don-t-replay-e2e.md) — Hit while e2e-testing the dm:error pipeline (workspace-file-first pass, 2026-09-22): dropping a broken-JSON file into `dm/pages/` **before** navigating to /page
- [Drive data.monster's real UI over CDP with --remote-debugging-port for e2e debugging](learnings/drive-data-monster-s-real-ui-over-cdp.md) — The changelog-e2e skill's technique transfers from the changelog.monster app to **data.monster**: launch the Tauri app with `--remote-debugging-port` and drive 
- [DuckDB app hangs = poisoned connection on Windows (duckdb-rs #209); in-process recovery fix](learnings/duckdb-app-hangs-poisoned-connection-windows.md) — Diagnosed 2026-09-22 while investigating the data.monster app hangs (reports/pages-e2e-feedback.md).
- [duckdb plain-bundled lacks static JSON extension — dynamic auto-load heap-corrupts on Windows](learnings/duckdb-bundled-lacks-static-json-extension.md) — Symptom
- [duckdb-rs lacks Value: FromSql — typed queries, no generic rows](learnings/duckdb-rs-lacks-value-fromsql-typed-queries.md) — Hit in `files-006` while building the `dm_store` export path (2026-09-22): a generic row-deserialization helper over `duckdb-rs` queries is impossible because t
- [Empty pages/queries lists after a Rust rebuild = dm/ not yet migrated — not data loss](learnings/empty-pages-queries-lists-after-a-rust-rebuild.md) — **Gotcha (mid-migration window, branch `feature/workspace-file-first`)**: once the Rust backend is rebuilt with the file-backed commands, pages/saved-queries li
- [Evidence.dev chart architecture: one typed component per chart type over shared machinery, consistency via a standardized prop taxonomy](learnings/evidence-chart-architecture.md) — Distilled 2026-09-15 while planning the central reusable-chart design (interview in progress; user asked to study docs.evidence.dev/components/scatter_chart and
- [ExprEditor suggestions are computed locally](learnings/expreditor-suggestions-are-computed-locally.md) — While hunting the suspected "per-keystroke autocomplete invoke flood" (bug #4 of the /pages E2E report, 2026-09-22): **no such flood exists — don't chase it aga
- [Extending docs/features/ requires add-evals-to-skill's name-dir match and case pattern](learnings/extending-docs-features-requires-add-evals.md) — Constraints of add-evals-to-skill (hit while building [[feature-skill-catalog-docs-features]])
- [get_settings merges env/.env over settings.json — env is source of truth](learnings/get-settings-merges-env-env-over.md) — Discovered while wiring `.env` into the app (2026-09-11).
- [Hard-reload storms deadlock DuckDB in-process — writes fail with "resource deadlock would occur" until full restart](learnings/hard-reload-storms-deadlock-duckdb-in-process.md) — Discovered 2026-09-17 while CDP-testing the master-items create flow in the page editor.
- [initialize_duckdb no-ops while initialized — workspace switch must shutdown first](learnings/initialize-duckdb-no-ops-while-initialized.md) — Fact
- [kees.pippeloi.nl reference ports cleanly — same svelteplot 0.14.2 + Tailwind 4](learnings/kees-reference-ports-cleanly.md) — `E:\kees.pippeloi.nl` (esp. `src/routes/work/high-level`) is the reference project for chart-type components being ported into Labs.
- [Labs bar-chart "hang" is an infinite vite reconnect/reload loop, not a component bug](learnings/labs-hang-vite-reload-loop.md) — Reported 2026-09-14: clicking the bar chart card in /labs hung the page (heatmap fine). Root cause found same day: vitest import reachable from src via $lib/charts tripped vite's dep-optimizer, amplified by tailwind re-emitting app.css on any file churn. Fixed in PR #3 (commit 9f18749).
- [Library code entries are keyed by full repo paths](learnings/library-code-entries-are-keyed-by-full-repo-paths.md) — What
- [The /library vs /pages config-drawer difference is scope, not components](learnings/library-vs-pages-config-drawer-scope.md) — Symptom
- [LLM API data retention: "no training" ≠ "no storage"; local models are ZDR by construction](learnings/llm-api-data-retention-no-training-no.md) — Research verified against primary docs (2026-09-11) on how the 6–8 major LLM API endpoints handle data retention and sensitive data. Directly relevant to Data M
- [LLM provider retention, part 2: Kimi, Z.ai, Together, Qwen — Kimi policy contradiction, Z.ai DPA strength, tier framework](learnings/llm-provider-retention-part-2-kimi-z-ai.md) — Follow-up research (2026-09-11) on Kimi (Moonshot), Z.ai (Zhipu/GLM), Together AI, and Qwen (Alibaba Model Studio), verified against primary docs. Extends [[llm
- [Local checkout is the running dev app — branch switches live-revert it until all PRs merge](learnings/local-checkout-is-the-running-dev-app.md) — Discovered 2026-09-22 while handling the post-merge state of PR #19 (bug fixes) and its stranded follow-up commit `dd44774` (opened as PR #20, session artifacts
- [Local LLM blank-screen delay was hidden thinking tokens — disable via "thinking": {"type": "disabled"}](learnings/local-llm-blank-screen-delay-was-hidden.md) — Symptom
- [Minimized/occluded WebView2 window throttles the page — bringToFront before CDP UI automation](learnings/minimized-occluded-webview2-throttles-page.md) — Discovered 2026-09-22 while CDP-testing bug fixes on the dev app (reports/pages-e2e-feedback.md).
- [Mock-Tauri browser repro harness is gone — verify visually via self-contained routes](learnings/mock-tauri-browser-repro-harness-is-gone-verify.md) — Discovered 2026-09-18 while trying to visually verify the drawer restyle: the CDP port wasn't open, so I reached for the mock-Tauri browser repro technique docu
- [MSYS path conversion mangles /f-style Windows flags — use MSYS_NO_PATHCONV=1 or PowerShell](learnings/msys-path-conversion-mangles-f-style-flags.md) — Discovered 2026-09-22 while running the blessed CDP restart chain from the MSYS/Git-Bash shell (verifying the /data tab URL-sync fix).
- [Never tree-scan .archive/ or src-tauri/target/ — du/find stall on the huge trees](learnings/never-tree-scan-archive-or-src-tauri.md) — The repo contains very large generated/historical trees: `.archive/` (entire superseded old app + chart-engine trials) and `src-tauri/target/` (Rust build artif
- [normalizePageDoc is a field whitelist — new PageDoc fields must be passed through or they're stripped on load](learnings/normalizepagedoc-field-whitelist.md) — Discovered 2026-09-17 fixing the `/pages` row-height persistence bug: user resized a row, revisited the page, height was gone — yet the save path stored it corr
- [PageDoc has block.title AND chart.title — charts render only chart.title; inspector must write there](learnings/pagedoc-block-title-and-chart-title-rendering.md) — In the central-charts [[chart-page-spec-spec-types-validator]] `PageDoc`, a block carries a **block-level `title`** *and* (for chart blocks) **`chart.title` / `
- [Pages editor auto-saves silently every 60s — no UI signal is deliberate](learnings/pages-editor-auto-saves-silently-every-60s-no-ui.md) — SUPERSEDED 2026-09-22 by the write-through core (files-009). Historical: user-requested behavior on `src/routes/pages/[slug]/+page.svelte` (2026-09-18, /pages/revenue): auto-save runs every 60s via `handleSave(true)`, which **skips t
- [Query editor blowup was .app-column min-height:auto — mock-Tauri browser repro technique](learnings/query-editor-blowup-was-app-column-min-height-auto.md) — Symptom: on /query, clicking a Data-source table made the SQL editor pane "huge" (1689px in a 786px window) while the initial page looked fine.
- [Ref-based master items: tableName/table mismatch broke all ref charts; expression dims need raw compile](learnings/ref-based-master-items-tablename-mismatch-broke.md) — Discovered 2026-09-22 during the /pages E2E session (reports/pages-e2e-feedback.md).
- [Scale standalone HTML docs via root font-size + px sweep — zoom breaks fixed overlays](learnings/scale-standalone-html-docs-via-root-font-size-px.md) — Discovered 2026-09-18 scaling `docs/design-system-data-monster.html` to 80%.
- [SearchAhead.svelte is a /ui showcase demo, not prop-driven — build inline searchaheads](learnings/searchahead-svelte-is-a-ui-showcase-demo-not-prop.md) — Discovered 2026-09-17 building the skeleton pick/create modal.
- [Secret-policy test targets the connections.json example — the .env placeholder password is intentional](learnings/secret-policy-test-targets-the-connections-json.md) — Hit while finishing files-011's secret-policy test (2026-09-22): the test flagged `password@` inside the `.env` sample URL (`DM_CONN_PRODUCTION_URL=postgresql:/
- [Settings-swap for tours must cover .env too, and the app webview must never navigate off-origin](learnings/settings-swap-for-tours-must-cover-env-too.md) — Discovered 2026-09-16 building the settings-tour + analyst-tour (docs/tours/RUNBOOK.md CRITICAL section).
- [settings-tour and analyst-tour built — honest-beats-staged applied to the chat](learnings/settings-tour-and-analyst-tour-built.md) — Built 2026-09-16 — settings-tour and analyst-tour complete the 8-tour set in docs/tours/ (connect, preview, query, data-tables, pages, labs, settings, analyst).
- [Shallow URL state in SvelteKit: replaceState from $app/navigation, never goto or window.history](learnings/shallow-url-state-sveltekit-replacestate.md) — Discovered 2026-09-22 making /data tab selection URL-addressable (`TableOverview.svelte`, +6 lines).
- [speed-highlight/core has no Svelte grammar](learnings/speed-highlight-core-has-no-svelte-grammar.md) — Gotchas discovered wiring `@speed-highlight/core` into the library Code tab
- [Squada One is single-weight (400) — heading font-weight 600/700 gets browser-synthesized bold](learnings/squada-one-is-single-weight-400.md) — Discovered 2026-09-16 while re-typing the app ([[typography-squada-one-headings-libre-baskerville]]).
- [Stale component CSS after an edit can be fixed with touch — no dev-server restart needed](learnings/stale-component-css-after-an-edit-can-be-fixed.md) — Extends [[stale-vite-module-graph-can-survive-reloads-only-arestart]].
- [Stale vite module graph can survive reloads — only a full app restart clears it](learnings/stale-vite-module-graph-can-survive-reloads-only-a.md) — Discovered 2026-09-17 while wiring the skeleton's "Add dimension" button in the page editor. Extends [[apparent-ui-bug-stale-hmr-webview]]: that learning's fix 
- [Stale-wiki file floods — only noise if an ignore pattern actually matches the tree](learnings/stale-wiki-file-floods-are-ignored.md) — Symptom and root cause — src-tauri/target leaked through, fixed via .wiki_ignore plus extension BUILTIN_IGNORES.
- [Stash pop can silently fail when wiki-recap writes conflict — verify and restore from the stash](learnings/stash-pop-silent-conflict-recovery.md) — Discovered 2026-09-14 while committing session work (PR #3).
- [SvelteKit page.url is stale after replaceState — never guard write-effects by reading it back](learnings/sveltekit-page-url-stale-after-replacestate.md) — Discovered 2026-09-22 while making /data tab selection URL-addressable (`TableOverview.svelte`, verified over CDP against the live dev app).
- [svelteplot band axis crashes on empty aliases (duplicate key)](learnings/svelteplot-band-axis-empty-aliases-crash.md) — Symptom: charts crashed with a duplicate-key error in svelteplot's band axis when the central-charts page mounted.
- [SveltePlot BarX vs BarY: BarX is the horizontal bar mark](learnings/svelteplot-barx-bar-y-orientation.md) — Discovered while flipping `charts/BarChart.svelte` to horizontal (2026-09-14), confirmed against https://svelteplot.dev/examples ("Simple Bars"):
- [SveltePlot internals: match datums by position, not identity; guard empty data](learnings/svelteplot-datum-identity-empty-guard.md) — Two engine-level gotchas discovered while porting [[heatmap-component]] (2026-09-14), from the explanation of the SveltePlot 0.14.2 implementation. They apply t
- [SveltePlot 0.14.2 has no tree mark — verified in the installed package](learnings/svelteplot-has-no-tree-mark.md) — Verified 2026-09-17 by a te9-research leaf against the **installed** package (not just docs): svelteplot 0.14.2 — data.monster's sole chart engine — ships no tr
- [SveltePlot ordinal domains sort alphabetically by default — set explicit domain or reverse](learnings/svelteplot-ordinal-domain-sorts-alphabetically.md) — Discovered 2026-09-14 while making the /labs bar chart sort desc: page-side data sorting had **no visible effect** because svelteplot's ordinal scales **sort th
- [SveltePlot scale bypass needs scale: null — scale: false still routes values through the scale](learnings/svelteplot-scale-null-not-false.md) — Symptom
- [Tour DOM snapshots scale with the live DOM — bound secondary frames, keep one big frame when size is the story](learnings/tour-dom-snapshots-scale-with-live-dom.md) — Discovered 2026-09-16 finishing the query-tour (see [[app-tour-set-docs-tours]]).
- [Tour HTML captures embed the Google-Fonts @import — font changes require recapturing tours](learnings/tour-html-captures-embed-google-fonts-import.md) — Discovered 2026-09-16 while re-typing the app ([[typography-squada-one-headings-libre-baskerville]]).
- [Visibility probes must walk the ancestor opacity/display/visibility chain — an opacity:0 parent hides everything](learnings/visibility-probes-walk-ancestor-opacity-chain.md) — CDP "visibility" checks lied twice on the page-editor config drawer (2026-09-15):
- [WebView2 CDP gotchas: env-var flag, stale browser process, dual-stack vite](learnings/webview2-cdp-gotchas-env-var-flag-stale.md) — Follow-up to [[drive-data-monster-s-real-ui-over-cdp]] — four gotchas hit while verifying the 2026-09-12 redesign:
- [First-run welcome gate renders instead of the router — its actions must act directly, never navigate](learnings/welcome-gate-renders-instead-of-router.md) — In `src/routes/+layout.svelte`, the first-run welcome gate (shown when no workspace is open) renders **instead of** the routed content — there is no router-rend
- [wiki_note_page wikilinks resolve ./-relative to the page's own folder — cross-folder links need explicit paths](learnings/wiki-note-page-wikilinks-resolve-relative.md) — Discovered 2026-09-16 while writing the [[feature-skill-catalog-docs-features]] artifact page.
- [z.ai 401 "code 1000 Authentication Failed" means the key itself is bad — verify with curl, not app code](learnings/z-ai-401-code-1000-authentication.md) — Symptom
- [z.ai GLM Coding Plan keys use the Anthropic endpoint — a valid key still 401s against /paas/v4](learnings/z-ai-glm-coding-plan-keys-use-the-anthropic.md) — Refinement of [[z-ai-401-code-1000-authentication]] — a 401 from z.ai does not always mean the key is bad. Discovered 2026-09-17 while checking whether little-c

### Preferences

- [Agent may run the CDP restart chain (kill webviews + env flag + npm run dev) itself](preferences/agent-may-run-the-cdp-restart-chain-kill-webviews.md) — Agent may run the CDP restart chain itself
- [CDP-verify the dev app via WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS (exact restart procedure)](preferences/cdp-verify-the-dev-app-via-webview2-additional.md) — How to get the dev app CDP-drivable (exact procedure)
- [Never start npm run dev / tauri dev — the user owns the dev app](preferences/never-start-npm-run-dev-tauri-dev.md) — The LLM must never launch the dev app itself — no `npm run dev`, `npx tauri dev`, or background dev-server starts. The user starts and owns the dev app.
<!-- wiki-nav:end -->

An [OKF](https://github.com/earendil-works/okf) bundle documenting this project.

- [Overview](./overview.md) — What this project contains and its structure
- [File tree](./architecture/file-tree.md) — Complete project file listing
- [Glossary](./glossary.md) — Key terms for this project
- [Pages](./pages/) — Concepts, entities, and artifacts of this project
