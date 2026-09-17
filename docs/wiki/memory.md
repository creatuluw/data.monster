---
okf_version: "0.1"
---

<!-- wiki-memory:start -->
# Memory — the live contract

Auto-generated digest of the most recent conventions, decisions, rules and
development patterns, plus architecture and global patterns — newest first.
The actual files live in the wiki subfolders; follow the links (clickable in /wiki).
Regenerated on every wiki write and on wiki_mark_synced. Generated 2026-09-17T10:45:15.063Z.

## Recent Decisions

- [Use speed-highlight/core for code highlighting instead of Prism](decisions/speed-highlight-over-prism.md) — Context (2026-09-17)
- [Library packages carry blockKind — table/text are built-in blocks, not chart types](decisions/library-packages-carry-blockkind.md) — Context (2026-09-17)
- [library-component-builder skill is the canonical path for new library components](decisions/library-component-builder-canonical-path.md) — Context (2026-09-17)
- [Library registry drives editor + /library in one shot (supersedes display-only v1)](decisions/library-registry-drives-editor-and-library.md) — Context (2026-09-17)
- [Library registry v1 lives as a TypeScript module under src/lib/library/ with self-contained component folders](decisions/library-registry-ts-module.md) — Context (2026-09-17)
- [Library demos render the real components fed dummy query-shaped data — no demo-only clones](decisions/library-demos-reuse-real-components.md) — Context (2026-09-17)
- [Library Q4: component demos get dedicated views, split into tabs — Preview is the default tab](decisions/library-q4-dedicated-tabbed-views.md) — Context (2026-09-17)
- [Library components are self-contained extension-style packages — own definition, logic, and data](decisions/library-extension-style-components.md) — Context (2026-09-17)
- [Library Q3: /library layout is master-detail — left index + full-size live demo with schema alongside](decisions/library-q3-master-detail-layout.md) — Context (2026-09-17)
- [Library Q2: registry v1 is display-only — editor wiring deferred](decisions/library-q2-registry-display-only.md) — Context (2026-09-17)
- [/library becomes the central component library (proposed — spec interview in progress)](decisions/library-central-component-library.md) — Context (2026-09-17)
- [Typography settles: Inter everywhere (display + body), Geist Mono for data detail](decisions/typography-settles-inter-everywhere-geist-mono.md) — Context (2026-09-16)
- [Typography: Host Grotesk headings, Geist body — Inter dropped](decisions/typography-host-grotesk-headings-geist-body.md) — Typography: Host Grotesk headings, Geist body — Inter dropped (2026-09-16)
- [Typography: Geist display — Syne dropped](decisions/typography-geist-display-syne-dropped.md) — Context (2026-09-16)
- [Typography: Syne display — Space Grotesk dropped](decisions/typography-syne-display-space-grotesk-dropped.md) — Typography: Syne display — Space Grotesk dropped (2026-09-16)

## Active Rules

- [Drawers reuse the shared drawerResize action](rules/drawers-reuse-the-shared-drawerresize-action.md) — Guideline (2026-09-17)
- [Render markdown via marked + .prose-chat, never a new pipeline](rules/render-markdown-via-marked-prose-chat.md) — When rendering any markdown anywhere in the app (docs tabs, chat, notes), parse with `marked` (already a dependency) and wrap the output in … (2026-09-17)
- [Two-font rule: Inter for all UI (display + body), Geist Mono for data detail](rules/inter-for-ui-text-geist-mono-only-for-data-detail.md) — Inter everywhere for UI text; Geist Mono reserved for data detail (tables, chart ticks, tags, IDs). (2026-09-16)
- [All pages are capped at 1920px and centered by the shared layout — no per-page opt-out](rules/app-content-capped-at-shared-max-width.md) — One wrapper, .app-column, owns header + breadcrumb + content, is capped at 1920px, centered, and always exactly viewport-high with full-heig… (2026-09-16)
- ["Demo" means an app-tour-demo UI tour, not eval suites](rules/demo-means-app-tour-demo-not-eval-suites.md) — Guideline (2026-09-16)
- [Pointer cursor comes from one global rule in app.css](rules/pointer-cursor-from-global-rule-app-css.md) — Guideline (2026-09-15)
- [Card spacing comes from the grid gap, never per-card margins](rules/card-spacing-from-grid-gap-not-margins.md) — In any grid of chart/component cards (page editor canvas, labs), inter-card (2026-09-15)
- [Spec-driven features: TDD + Karpathy skills referenced in every todo](rules/spec-driven-features-tdd-karpathy-in-todos.md) — Guideline (2026-09-15)
- [Interview the user one question at a time with lettered multiple-choice options](rules/interview-one-question-at-a-time.md) — Guideline (2026-09-15)
- [Each /labs chart owns its config panel](rules/each-labs-chart-owns-its-config-panel.md) — Guideline (2026-09-15)
- [Pin Tailwind @source scanning to src/ and app.html in app.css](rules/pin-tailwind-source-scanning.md) — Pin Tailwind @source scanning to src/ and app.html in app.css (2026-09-14)
- [Keep test files and vitest imports out of src/](rules/keep-test-files-and-vitest-imports-out-of-src.md) — Keep test files and vitest imports out of src/ (2026-09-14)
- [All /labs charts are built on the shared reusable-chart fundament](rules/labs-charts-reusable-fundament.md) — All /labs charts are built on the shared reusable-chart fundament (2026-09-14)
- [Feature-loop hard rules: PR-only shipping, opt-in worktrees, no force removal](rules/feature-loop-hard-rules.md) — Guideline (2026-09-14)
- [Route external API calls through Rust commands, never webview fetch](rules/route-external-api-calls-through-rust.md) — Guideline (2026-09-11)

## Preferences & Conventions

- [Never start npm run dev / tauri dev — the user owns the dev app](preferences/never-start-npm-run-dev-tauri-dev.md) — The LLM must never launch the dev app itself — no `npm run dev`, `npx tauri dev`, or background dev-server starts. The user starts and owns … (2026-09-15)

## Recent Learnings — development patterns

- [speed-highlight/core has no Svelte grammar](learnings/speed-highlight-core-has-no-svelte-grammar.md) — Gotchas discovered wiring `@speed-highlight/core` into the library Code tab (2026-09-17)
- [Library code entries are keyed by full repo paths](learnings/library-code-entries-are-keyed-by-full-repo-paths.md) — What (2026-09-17)
- [The /library vs /pages config-drawer difference is scope, not components](learnings/library-vs-pages-config-drawer-scope.md) — Symptom (2026-09-17)
- [Chart segment selection is parent-held {dimension, value} transient state](learnings/chart-segment-selection-parent-held-state.md) — Chart components (as used in `/pages` and the library detail page demo) manage their own click/deselect handlers once selection state exists… (2026-09-17)
- [Calluna is not on Google Fonts — css2 returns 200 but silently drops it](learnings/calluna-not-on-google-fonts-css2-drops-silently.md) — Discovered 2026-09-16 while recording the Calluna/Inter retype ([[typography-calluna-headings-inter-body]]). (2026-09-16)
- [Squada One is single-weight (400) — heading font-weight 600/700 gets browser-synthesized bold](learnings/squada-one-is-single-weight-400.md) — Discovered 2026-09-16 while re-typing the app ([[typography-squada-one-headings-libre-baskerville]]). (2026-09-16)
- [Tour HTML captures embed the Google-Fonts @import — font changes require recapturing tours](learnings/tour-html-captures-embed-google-fonts-import.md) — Discovered 2026-09-16 while re-typing the app ([[typography-squada-one-headings-libre-baskerville]]). (2026-09-16)
- [Central-charts work lives on feature/central-charts — master is held at a restore point](learnings/central-charts-work-lives-on-feature-branch.md) — Discovered 2026-09-16 when the user reported the `/pages` work as "completely lost." (2026-09-16)
- [Tour DOM snapshots scale with the live DOM — bound secondary frames, keep one big frame when size is the story](learnings/tour-dom-snapshots-scale-with-live-dom.md) — Discovered 2026-09-16 finishing the query-tour (see [[app-tour-set-docs-tours]]). (2026-09-16)
- [settings-tour and analyst-tour built — honest-beats-staged applied to the chat](learnings/settings-tour-and-analyst-tour-built.md) — Built 2026-09-16 — settings-tour and analyst-tour complete the 8-tour set in docs/tours/ (connect, preview, query, data-tables, pages, labs,… (2026-09-16)
- [Settings-swap for tours must cover .env too, and the app webview must never navigate off-origin](learnings/settings-swap-for-tours-must-cover-env-too.md) — Discovered 2026-09-16 building the settings-tour + analyst-tour (docs/tours/RUNBOOK.md CRITICAL section). (2026-09-16)
- [wiki_note_page wikilinks resolve ./-relative to the page's own folder — cross-folder links need explicit paths](learnings/wiki-note-page-wikilinks-resolve-relative.md) — Discovered 2026-09-16 while writing the [[feature-skill-catalog-docs-features]] artifact page. (2026-09-16)
- [Extending docs/features/ requires add-evals-to-skill's name-dir match and case pattern](learnings/extending-docs-features-requires-add-evals.md) — Constraints of add-evals-to-skill (hit while building [[feature-skill-catalog-docs-features]]) (2026-09-16)
- [Auto margins in the flex-column .app-main disable flex stretch — full-bleed pages shrink without width: 100%](learnings/auto-margins-app-main-disable-flex-stretch.md) — Symptom (2026-09-15)
- [CSS text-transform changes innerText, not textContent — probe labels case-insensitively](learnings/css-text-transform-changes-innertext-probes.md) — Symptom: a CDP DOM probe checking for the label `"Rows"` failed on the Page (2026-09-15)
- [Visibility probes must walk the ancestor opacity/display/visibility chain — an opacity:0 parent hides everything](learnings/visibility-probes-walk-ancestor-opacity-chain.md) — CDP "visibility" checks lied twice on the page-editor config drawer (2026-09-15): (2026-09-15)
- [Apparent UI bug after dev-server restarts = stale HMR webview — Ctrl+R before debugging](learnings/apparent-ui-bug-stale-hmr-webview.md) — Discovered 2026-09-15 while verifying the page-editor config drawer (cog → 50vw focused panel). (2026-09-15)
- [PageDoc has block.title AND chart.title — charts render only chart.title; inspector must write there](learnings/pagedoc-block-title-and-chart-title-rendering.md) — In the central-charts [[chart-page-spec-spec-types-validator]] `PageDoc`, a block carries a **block-level `title`** *and* (for chart blocks)… (2026-09-15)
- [CDP CAN click svelteplot marks — Input.dispatchMouseEvent with fresh coordinates; element.click() cannot](learnings/cdp-can-click-svelteplot-marks-dispatchmouseevent.md) — Correction to [[cdp-cannot-synthesize-clicks-on-svelteplot-marks]] — CDP `Input.dispatchMouseEvent` DOES click svelteplot marks (BarX `oncli… (2026-09-15)
- [CDP e2e cannot synthesize trusted clicks on svelteplot marks](learnings/cdp-cannot-synthesize-clicks-on-svelteplot-marks.md) — Symptom: chart **click-through (cross-filter selection) is untestable via CDP e2e** — synthesized clicks on svelteplot marks do nothing, eve… (2026-09-15)

## Architecture

- [File tree](architecture/file-tree.md) — Complete project file listing with per-file descriptions. (2026-09-11)

<!-- wiki-memory:end -->
