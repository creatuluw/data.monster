---
okf_version: "0.1"
---

<!-- wiki-memory:start -->
# Memory — the live contract

Auto-generated digest of the most recent conventions, decisions, rules and
development patterns, plus architecture and global patterns — newest first.
The actual files live in the wiki subfolders; follow the links (clickable in /wiki).
Regenerated on every wiki write and on wiki_mark_synced. Generated 2026-09-22T13:45:56.794Z.

## Recent Decisions

- [Workspace files are canonical — agents author content by editing the dm/ tree in realtime](decisions/workspace-files-are-canonical-agents-author-content-by-editi.md) — Context (2026-09-22)
- [Live-reload mechanics: notify watcher → validate → dm:changed/dm:error events (proposed)](decisions/live-reload-mechanics-notify-watcher-validate-dm-changed-dm-.md) — Context (2026-09-22)
- [Workspace content tree: dm/ with path-is-identity, native formats, agent README (proposed)](decisions/workspace-content-tree-dm-with-path-is-identity-native-forma.md) — Context (2026-09-22)
- [Agent authors app content by editing workspace files — the workspace folder is the interface (proposed)](decisions/agent-authors-app-content-by-editing-workspace-files-the-wor.md) — Context (2026-09-22)
- [Workspaces are fully portable — switching reloads data, content, and settings](decisions/workspaces-are-fully-portable-switch-reloads.md) — Context (2026-09-22)
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

## Active Rules

- [Secrets never live in workspace content files — env references only, gitignored .env at all times](rules/secrets-never-live-in-workspace-content-files-env-references.md) — The rule (2026-09-22, user-set during the workspace-file-first design): workspaces can be git/version controlled, so every file the app writ… (2026-09-22)
- [Restore points are git tags restore-point/<feature>-start on pushed master HEAD](rules/restore-points-are-git-tags-restore-point-feature-start-on-p.md) — The rule (2026-09-22)
- [Resize requests use the app's existing size classes — never ad-hoc multipliers](rules/resize-requests-use-existing-size-classes.md) — Resize requests map onto the app's existing size classes — never invent ad-hoc pixel multipliers. (2026-09-22)
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

- [First-run welcome gate renders instead of the router — its actions must act directly, never navigate](learnings/welcome-gate-renders-instead-of-router.md) — In `src/routes/+layout.svelte`, the first-run welcome gate (shown when no workspace is open) renders **instead of** the routed content — the… (2026-09-22)
- [compile.ts dimension guard: raw flag is the only validation bypass — never blanket-catch checkColumn failures](learnings/compile-ts-dimension-guard-raw-flag-only-bypass.md) — Symptom (2026-09-22)
- [duckdb plain-bundled lacks static JSON extension — dynamic auto-load heap-corrupts on Windows](learnings/duckdb-bundled-lacks-static-json-extension.md) — Symptom (2026-09-22)
- [initialize_duckdb no-ops while initialized — workspace switch must shutdown first](learnings/initialize-duckdb-no-ops-while-initialized.md) — Fact (2026-09-22)
- [Local checkout is the running dev app — branch switches live-revert it until all PRs merge](learnings/local-checkout-is-the-running-dev-app.md) — Discovered 2026-09-22 while handling the post-merge state of PR #19 (bug fixes) and its stranded follow-up commit `dd44774` (opened as PR #2… (2026-09-22)
- [DuckDB app hangs = poisoned connection on Windows (duckdb-rs #209); in-process recovery fix](learnings/duckdb-app-hangs-poisoned-connection-windows.md) — Diagnosed 2026-09-22 while investigating the data.monster app hangs (reports/pages-e2e-feedback.md). (2026-09-22)
- [ExprEditor suggestions are computed locally](learnings/expreditor-suggestions-are-computed-locally.md) — While hunting the suspected "per-keystroke autocomplete invoke flood" (bug #4 of the /pages E2E report, 2026-09-22): **no such flood exists … (2026-09-22)
- [Minimized/occluded WebView2 window throttles the page — bringToFront before CDP UI automation](learnings/minimized-occluded-webview2-throttles-page.md) — Discovered 2026-09-22 while CDP-testing bug fixes on the dev app (reports/pages-e2e-feedback.md). (2026-09-22)
- [Ref-based master items: tableName/table mismatch broke all ref charts; expression dims need raw compile](learnings/ref-based-master-items-tablename-mismatch-broke.md) — Discovered 2026-09-22 during the /pages E2E session (reports/pages-e2e-feedback.md). (2026-09-22)
- [MSYS path conversion mangles /f-style Windows flags — use MSYS_NO_PATHCONV=1 or PowerShell](learnings/msys-path-conversion-mangles-f-style-flags.md) — Discovered 2026-09-22 while running the blessed CDP restart chain from the MSYS/Git-Bash shell (verifying the /data tab URL-sync fix). (2026-09-22)
- [/data tab keys ≠ labels — "Metadata" writes ?tab=definitions](learnings/data-tab-keys-labels-metadata-writes-definitions.md) — Discovered 2026-09-22 while CDP-verifying the /data tab URL sync (port 9223): a probe matching tabs by `textContent.includes('Definitions')`… (2026-09-22)
- [CDP e2e failures right after a source save are often HMR races — re-run before debugging](learnings/cdp-e2e-failures-after-source-save-hmr-race.md) — Discovered 2026-09-22 while debugging the /data tab URL-sync bug (verified over CDP, port 9223). (2026-09-22)
- [SvelteKit page.url is stale after replaceState — never guard write-effects by reading it back](learnings/sveltekit-page-url-stale-after-replacestate.md) — Discovered 2026-09-22 while making /data tab selection URL-addressable (`TableOverview.svelte`, verified over CDP against the live dev app). (2026-09-22)
- [Shallow URL state in SvelteKit: replaceState from $app/navigation, never goto or window.history](learnings/shallow-url-state-sveltekit-replacestate.md) — Discovered 2026-09-22 making /data tab selection URL-addressable (`TableOverview.svelte`, +6 lines). (2026-09-22)
- [Pages editor auto-saves silently every 60s — no UI signal is deliberate](learnings/pages-editor-auto-saves-silently-every-60s-no-ui.md) — User-requested behavior on `src/routes/pages/[slug]/+page.svelte` (2026-09-18, /pages/revenue): auto-save runs every 60s via `handleSave(tru… (2026-09-21)
- [Stale component CSS after an edit can be fixed with touch — no dev-server restart needed](learnings/stale-component-css-after-an-edit-can-be-fixed.md) — Extends [[stale-vite-module-graph-can-survive-reloads-only-arestart]]. (2026-09-18)
- [Mock-Tauri browser repro harness is gone — verify visually via self-contained routes](learnings/mock-tauri-browser-repro-harness-is-gone-verify.md) — Discovered 2026-09-18 while trying to visually verify the drawer restyle: the CDP port wasn't open, so I reached for the mock-Tauri browser … (2026-09-18)
- [Scale standalone HTML docs via root font-size + px sweep — zoom breaks fixed overlays](learnings/scale-standalone-html-docs-via-root-font-size-px.md) — Discovered 2026-09-18 scaling `docs/design-system-data-monster.html` to 80%. (2026-09-18)
- [z.ai GLM Coding Plan keys use the Anthropic endpoint — a valid key still 401s against /paas/v4](learnings/z-ai-glm-coding-plan-keys-use-the-anthropic.md) — Refinement of [[z-ai-401-code-1000-authentication]] — a 401 from z.ai does not always mean the key is bad. Discovered 2026-09-17 while check… (2026-09-17)
- [Bash heredoc writes mangle non-ASCII — patch with python explicit escapes, and verify bytes before assuming corruption](learnings/bash-heredoc-writes-mangle-non-ascii-patch-with.md) — Hit twice while rewiring the drawers (PR #18, 2026-09-17). (2026-09-17)

## Architecture

- [File tree](architecture/file-tree.md) — Complete project file listing with per-file descriptions. (2026-09-11)

<!-- wiki-memory:end -->
