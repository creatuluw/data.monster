---
okf_version: "0.1"
---

<!-- wiki-memory:start -->
# Memory — the live contract

Auto-generated digest of the most recent conventions, decisions, rules and
development patterns, plus architecture and global patterns — newest first.
The actual files live in the wiki subfolders; follow the links (clickable in /wiki).
Regenerated on every wiki write and on wiki_mark_synced. Generated 2026-09-22T11:46:23.064Z.

## Recent Decisions

- [Timeout and retry defend against hung IPC](decisions/timeout-and-retry-defend-against-hung-ipc.md) — Context (2026-09-22)
- [All drawers adopt the /data (TableDrawer) design pattern — DrawerTabs removed](decisions/all-drawers-adopt-the-data-tabledrawer-design.md) — Context (2026-09-18)
- [Drawer chrome restyle reverted — control kit stands, lms/kees motifs rejected](decisions/drawer-chrome-restyle-reverted-control-kit-stands.md) — Context (2026-09-17)
- [Tab bar shows only explicitly opened tabs — navigation never creates tabs](decisions/tab-bar-shows-only-explicitly-opened-tabs.md) — Context (2026-09-17)
- [App gets virtual multi-tab navigation: bottom bar is the tab bar](decisions/app-gets-virtual-multi-tab-navigation-bottom-bar.md) — Context (2026-09-17)
- [Skeleton pick/create moved from inline dropdowns to card buttons opening a modal (searchahead + New)](decisions/skeleton-pick-create-moved-from-inline-dropdowns.md) — Skeleton role-assignment: card buttons open a pick/create modal (searchahead + New) (2026-09-17)
- [Skeleton card is the inline role-assignment surface — pick/create in-chart, drawer optional](decisions/skeleton-card-is-the-inline-role-assignment.md) — Context (2026-09-17)
- [Chart blocks start empty — data renders only when role requirements are met (needsSetup gate)](decisions/chart-blocks-start-empty-needssetup-gate.md) — Context (2026-09-17)
- [Linked-table raw fields are transient with auto-JOIN — master-item creation stays optional](decisions/linked-table-raw-fields-transient-autojoin.md) — Context (2026-09-17)
- [Use speed-highlight/core for code highlighting instead of Prism](decisions/speed-highlight-over-prism.md) — Context (2026-09-17)
- [Library packages carry blockKind — table/text are built-in blocks, not chart types](decisions/library-packages-carry-blockkind.md) — Context (2026-09-17)
- [library-component-builder skill is the canonical path for new library components](decisions/library-component-builder-canonical-path.md) — Context (2026-09-17)
- [Library registry drives editor + /library in one shot (supersedes display-only v1)](decisions/library-registry-drives-editor-and-library.md) — Context (2026-09-17)
- [Library registry v1 lives as a TypeScript module under src/lib/library/ with self-contained component folders](decisions/library-registry-ts-module.md) — Context (2026-09-17)
- [Library demos render the real components fed dummy query-shaped data — no demo-only clones](decisions/library-demos-reuse-real-components.md) — Context (2026-09-17)

## Active Rules

- [Config drawers are one scrolling column — settings sections, Danger zone last](rules/config-drawers-one-scrolling-column.md) — Guideline (2026-09-18)
- [Drawer form controls come from the shared controls kit — never hand-roll input chrome](rules/drawer-form-controls-come-from-the-shared-controls.md) — Guideline (2026-09-17)
- [Pick display labels resolve through roleLabels() — never hand-roll chip labels](rules/pick-display-labels-resolve-through-rolelabels.md) — Guideline (2026-09-17)
- [Pick values flow through one codec — src/lib/charts/pickers.ts](rules/pick-values-flow-through-one-codec-src-lib-charts.md) — Guideline (2026-09-17)
- [Adding a component never auto-opens the config drawer — skeleton is the start state; drawer-open seeds picker rows](rules/adding-a-component-never-auto-opens-the-config.md) — When a component is added to a page-editor row (`addComponent` in the page editor), the **config drawer must NOT auto-open**. The newly adde… (2026-09-17)
- [Leave wiki-recap noise uncommitted — branch fresh and commit selectively, never stash](rules/leave-wiki-recap-noise-uncommitted.md) — Guideline (2026-09-17)
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

- [Agent may run the CDP restart chain (kill webviews + env flag + npm run dev) itself](preferences/agent-may-run-the-cdp-restart-chain-kill-webviews.md) — Agent may run the CDP restart chain itself (2026-09-17)
- [CDP-verify the dev app via WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS (exact restart procedure)](preferences/cdp-verify-the-dev-app-via-webview2-additional.md) — How to get the dev app CDP-drivable (exact procedure) (2026-09-17)
- [Never start npm run dev / tauri dev — the user owns the dev app](preferences/never-start-npm-run-dev-tauri-dev.md) — The LLM must never launch the dev app itself — no `npm run dev`, `npx tauri dev`, or background dev-server starts. The user starts and owns … (2026-09-15)

## Recent Learnings — development patterns

- [DuckDB app hangs = poisoned connection on Windows (duckdb-rs #209); in-process recovery fix](learnings/duckdb-app-hangs-poisoned-connection-on-windows-duckdb-rs-20.md) — Diagnosed 2026-09-22 while investigating the data.monster app hangs (reports/pages-e2e-feedback.md). (2026-09-22)
- [ExprEditor suggestions are computed locally](learnings/expreditor-suggestions-are-computed-locally.md) — While hunting the suspected "per-keystroke autocomplete invoke flood" (bug #4 of the /pages E2E report, 2026-09-22): **no such flood exists … (2026-09-22)
- [Minimized/occluded WebView2 window throttles the page — bringToFront before CDP UI automation](learnings/minimized-occluded-webview2-window-throttles-the-page-bringt.md) — Discovered 2026-09-22 while CDP-testing bug fixes on the dev app (reports/pages-e2e-feedback.md). (2026-09-22)
- [Ref-based master items: tableName/table mismatch broke all ref charts; expression dims need raw compile](learnings/ref-based-master-items-tablename-table-mismatch-broke-all-re.md) — Discovered 2026-09-22 during the /pages E2E session (reports/pages-e2e-feedback.md). (2026-09-22)
- [MSYS path conversion mangles /f-style Windows flags — use MSYS_NO_PATHCONV=1 or PowerShell](learnings/msys-path-conversion-mangles-f-style-windows-flags-use-msys-.md) — Discovered 2026-09-22 while running the blessed CDP restart chain from the MSYS/Git-Bash shell (verifying the /data tab URL-sync fix). (2026-09-22)
- [/data tab keys ≠ labels — "Metadata" writes ?tab=definitions](learnings/data-tab-keys-labels-metadata-writes-tab-definitions.md) — Discovered 2026-09-22 while CDP-verifying the /data tab URL sync (port 9223): a probe matching tabs by `textContent.includes('Definitions')`… (2026-09-22)
- [CDP e2e failures right after a source save are often HMR races — re-run before debugging](learnings/cdp-e2e-failures-right-after-a-source-save-are-often-hmr-rac.md) — Discovered 2026-09-22 while debugging the /data tab URL-sync bug (verified over CDP, port 9223). (2026-09-22)
- [SvelteKit page.url is stale after replaceState — never guard write-effects by reading it back](learnings/sveltekit-page-url-is-stale-after-replacestate-never-guard-w.md) — Discovered 2026-09-22 while making /data tab selection URL-addressable (`TableOverview.svelte`, verified over CDP against the live dev app). (2026-09-22)
- [Shallow URL state in SvelteKit: replaceState from $app/navigation, never goto or window.history](learnings/shallow-url-state-sveltekit-replacestate.md) — Discovered 2026-09-22 making /data tab selection URL-addressable (`TableOverview.svelte`, +6 lines). (2026-09-22)
- [Pages editor auto-saves silently every 60s — no UI signal is deliberate](learnings/pages-editor-auto-saves-silently-every-60s-no-ui.md) — User-requested behavior on `src/routes/pages/[slug]/+page.svelte` (2026-09-18, /pages/revenue): auto-save runs every 60s via `handleSave(tru… (2026-09-21)
- [Stale component CSS after an edit can be fixed with touch — no dev-server restart needed](learnings/stale-component-css-after-an-edit-can-be-fixed.md) — Extends [[stale-vite-module-graph-can-survive-reloads-only-arestart]]. (2026-09-18)
- [Mock-Tauri browser repro harness is gone — verify visually via self-contained routes](learnings/mock-tauri-browser-repro-harness-is-gone-verify.md) — Discovered 2026-09-18 while trying to visually verify the drawer restyle: the CDP port wasn't open, so I reached for the mock-Tauri browser … (2026-09-18)
- [Scale standalone HTML docs via root font-size + px sweep — zoom breaks fixed overlays](learnings/scale-standalone-html-docs-via-root-font-size-px.md) — Discovered 2026-09-18 scaling `docs/design-system-data-monster.html` to 80%. (2026-09-18)
- [z.ai GLM Coding Plan keys use the Anthropic endpoint — a valid key still 401s against /paas/v4](learnings/z-ai-glm-coding-plan-keys-use-the-anthropic.md) — Refinement of [[z-ai-401-code-1000-authentication]] — a 401 from z.ai does not always mean the key is bad. Discovered 2026-09-17 while check… (2026-09-17)
- [Bash heredoc writes mangle non-ASCII — patch with python explicit escapes, and verify bytes before assuming corruption](learnings/bash-heredoc-writes-mangle-non-ascii-patch-with.md) — Hit twice while rewiring the drawers (PR #18, 2026-09-17). (2026-09-17)
- [CDP context-menu e2e: real right-click dispatch, and check the binding before blaming synthetic events](learnings/cdp-context-menu-e2e-real-right-click-dispatch-and.md) — Discovered 2026-09-17 shipping PR #16 (virtual tab system, CDP e2e steps 1–7). Extends the synthetic-event family: [[cdp-can-click-svelteplo… (2026-09-17)
- [CDP probe `$$` is querySelector — indexing it silently kills clicks](learnings/cdp-probe-is-queryselector-indexing-it-silently.md) — Discovered 2026-09-17 shipping PR #12 (skeleton pick/create modal, CDP e2e steps 1–7). (2026-09-17)
- [SearchAhead.svelte is a /ui showcase demo, not prop-driven — build inline searchaheads](learnings/searchahead-svelte-is-a-ui-showcase-demo-not-prop.md) — Discovered 2026-09-17 building the skeleton pick/create modal. (2026-09-17)
- [CDP gate assertions need settle time after doc mutations, and svg counts must be chart-scoped](learnings/cdp-gate-assertions-need-settle-time-after-doc.md) — Two CDP-e2e traps hit while testing the needsSetup gate (2026-09-17, PR #10): (2026-09-17)
- [Component spawn grows too-small explicit-height rows to 320px minimum](learnings/component-spawn-grows-too-small-explicit-height.md) — Discovered 2026-09-17 while verifying the page-editor skeleton-clip bug (fixed in PR #9, 1 file +8). (2026-09-17)

## Architecture

- [File tree](architecture/file-tree.md) — Complete project file listing with per-file descriptions. (2026-09-11)

<!-- wiki-memory:end -->
