---
okf_version: "0.1"
---

<!-- wiki-memory:start -->
# Memory — the live contract

Auto-generated digest of the most recent conventions, decisions, rules and
development patterns, plus architecture and global patterns — newest first.
The actual files live in the wiki subfolders; follow the links (clickable in /wiki).
Regenerated on every wiki write and on wiki_mark_synced. Generated 2026-09-15T12:12:18.370Z.

## Recent Decisions

- [Page editor block config: focused two-panel mode via cog icon (no selection ring)](decisions/page-editor-block-config-focused-two-panel-mode-via-cog-icon.md) — Context (2026-09-15)
- [Master-items amendment: semantic layer moves early into central-charts v1](decisions/master-items-amendment-semantic-layer-moves-early-into-centr.md) — Context (2026-09-15)
- [Central-charts v1 scope: bar + heatmap + table blocks; master items and auto-JOIN deferred](decisions/central-charts-v1-scope-bar-heatmap-table-blocks-master-item.md) — Context (2026-09-15)
- [Q16 locked: /pages lists &amp; creates, /page/&lt;slug&gt; hosts the dual-mode editor](decisions/q16-locked-pages-lists-amp-creates-page-lt-slug-gt-hosts-the.md) — Context (2026-09-15)
- [Q15 locked: annotation vocabulary = svelteplot basic marks (Arrow, Dot, Line, Text, Rect), per-type whitelisted](decisions/q15-locked-annotation-vocabulary-svelteplot-basic-marks-arro.md) — Context (2026-09-15)
- [Q15 reframed: annotations should speak svelteplot's own mark vocabulary (whitelisted per chart type)](decisions/q15-reframed-annotations-should-speak-svelteplot-s-own-mark-.md) — Context (2026-09-15)
- [Q15 locked: reference annotations are a declarative list; reference_line only in v1](decisions/q15-locked-reference-annotations-are-a-declarative-list-refe.md) — Context (2026-09-15)
- [Q13 locked: explicit grid — spec declares rows, blocks take column spans (12-col)](decisions/q13-locked-explicit-grid-spec-declares-rows-blocks-take-colu.md) — Q13 locked: explicit grid — spec declares rows, blocks take column spans (12-col) (2026-09-15)
- [Q12 locked: page-level consistent colors via shared scale, optional manual overrides in spec](decisions/q12-locked-page-level-consistent-colors-via-shared-scale-opt.md) — Context (2026-09-15)
- [Q11 locked: tooltips are declarative fields + one template string, shared renderer per chart type](decisions/q11-locked-tooltips-are-declarative-fields-one-template-stri.md) — Context (2026-09-15)
- [Q10 locked: chart selections cross-filter other charts via re-query (Qlik-style, transient)](decisions/q10-locked-chart-selections-cross-filter-other-charts-via-re.md) — Context (2026-09-15)
- [Q9 locked: chart option panels are schema-driven with a custom-panel hatch](decisions/q9-locked-chart-option-panels-are-schema-driven-with-a-custo.md) — Context (2026-09-15)
- [Q8 locked: one canonical query engine with per-type hooks](decisions/q8-locked-one-canonical-query-engine-with-per-type-hooks.md) — Context (2026-09-15)
- [Q7 locked: relationship graph drives chart item availability and auto-JOIN](decisions/q7-locked-relationship-graph-drives-chart-item-availability-.md) — Context (2026-09-15)
- [Q6 locked: workspace-level master-item library (stable ids) with explicit table binding; Q7 open on binding depth](decisions/master-item-library-table-binding-q6.md) — Context (2026-09-15)

## Active Rules

- [Spec-driven features: TDD + Karpathy skills referenced in every todo](rules/spec-driven-features-tdd-karpathy-skills-referenced-in-every.md) — Guideline (2026-09-15)
- [Interview the user one question at a time with lettered multiple-choice options](rules/interview-one-question-at-a-time.md) — Guideline (2026-09-15)
- [Each /labs chart owns its config panel](rules/each-labs-chart-owns-its-config-panel.md) — Guideline (2026-09-15)
- [Pin Tailwind @source scanning to src/ and app.html in app.css](rules/pin-tailwind-source-scanning.md) — Pin Tailwind @source scanning to src/ and app.html in app.css (2026-09-14)
- [Keep test files and vitest imports out of src/](rules/keep-test-files-and-vitest-imports-out-of-src.md) — Keep test files and vitest imports out of src/ (2026-09-14)
- [All /labs charts are built on the shared reusable-chart fundament](rules/labs-charts-reusable-fundament.md) — All /labs charts are built on the shared reusable-chart fundament (2026-09-14)
- [Feature-loop hard rules: PR-only shipping, opt-in worktrees, no force removal](rules/feature-loop-hard-rules.md) — Guideline (2026-09-14)
- [Inter for UI text, Geist Mono only for data detail](rules/inter-for-ui-text-geist-mono-only-for-data-detail.md) — Guideline (2026-09-14)
- [Route external API calls through Rust commands, never webview fetch](rules/route-external-api-calls-through-rust.md) — Guideline (2026-09-11)

## Recent Learnings — development patterns

- [Visibility probes must walk the ancestor opacity/display/visibility chain — an opacity:0 parent hides everything](learnings/visibility-probes-must-walk-the-ancestor-opacity-display-vis.md) — CDP "visibility" checks lied twice on the page-editor config drawer (2026-09-15): (2026-09-15)
- [Apparent UI bug after dev-server restarts = stale HMR webview — Ctrl+R before debugging](learnings/apparent-ui-bug-after-dev-server-restarts-stale-hmr-webview-.md) — Discovered 2026-09-15 while verifying the page-editor config drawer (cog → 50vw focused panel). (2026-09-15)
- [PageDoc has block.title AND chart.title — charts render only chart.title; inspector must write there](learnings/pagedoc-has-block-title-and-chart-title-charts-render-only-c.md) — In the central-charts [[chart-page-spec-spec-types-validator]] `PageDoc`, a block carries a **block-level `title`** *and* (for chart blocks)… (2026-09-15)
- [CDP CAN click svelteplot marks — Input.dispatchMouseEvent with fresh coordinates; element.click() cannot](learnings/cdp-can-click-svelteplot-marks-input-dispatchmouseevent-with.md) — Correction to [[cdp-e2e-cannot-synthesize-trusted-clicks-on-svelteplot-marks]] — CDP `Input.dispatchMouseEvent` DOES click svelteplot marks … (2026-09-15)
- [CDP e2e cannot synthesize trusted clicks on svelteplot marks](learnings/cdp-e2e-cannot-synthesize-trusted-clicks-on-svelteplot-marks.md) — Symptom: chart **click-through (cross-filter selection) is untestable via CDP e2e** — synthesized clicks on svelteplot marks do nothing, eve… (2026-09-15)
- [svelteplot band axis crashes on empty aliases (duplicate key)](learnings/svelteplot-band-axis-crashes-on-empty-aliases-duplicate-key.md) — Symptom: charts crashed with a duplicate-key error in svelteplot's band axis when the central-charts page mounted. (2026-09-15)
- [Chart authoring needs two surfaces (code + UI) — design must converge on a serializable chart spec](learnings/chart-authoring-two-surfaces-serializable-spec.md) — Requirement (user-stated, 2026-09-15 interview) (2026-09-15)
- [Evidence.dev chart architecture: one typed component per chart type over shared machinery, consistency via a standardized prop taxonomy](learnings/evidence-chart-architecture.md) — Distilled 2026-09-15 while planning the central reusable-chart design (interview in progress; user asked to study docs.evidence.dev/componen… (2026-09-15)
- [SveltePlot scale bypass needs scale: null — scale: false still routes values through the scale](learnings/svelteplot-scale-null-not-false.md) — Symptom (2026-09-15)
- [SveltePlot ordinal domains sort alphabetically by default — set explicit domain or reverse](learnings/svelteplot-ordinal-domain-sorts-alphabetically.md) — Discovered 2026-09-14 while making the /labs bar chart sort desc: page-side data sorting had **no visible effect** because svelteplot's ordi… (2026-09-14)
- [SveltePlot BarX vs BarY: BarX is the horizontal bar mark](learnings/svelteplot-barx-bar-y-orientation.md) — Discovered while flipping `charts/BarChart.svelte` to horizontal (2026-09-14), confirmed against https://svelteplot.dev/examples ("Simple Ba… (2026-09-14)
- [Labs bar-chart "hang" is an infinite vite reconnect/reload loop, not a component bug](learnings/labs-hang-vite-reload-loop.md) — Reported 2026-09-14: clicking the bar chart card in /labs hung the page (heatmap fine). Root cause found same day: vitest import reachable f… (2026-09-14)
- [CDP repro traps: DuckDB workspace lock pins a second instance at /; headless needs a mocked Tauri surface](learnings/cdp-repro-traps-duckdb-lock.md) — Follow-up to [[drive-data-monster-s-real-ui-over-cdp]] and [[webview2-cdp-gotchas-env-var-flag-stale]] — two more repro-environment traps hi… (2026-09-14)
- [Stash pop can silently fail when wiki-recap writes conflict — verify and restore from the stash](learnings/stash-pop-silent-conflict-recovery.md) — Discovered 2026-09-14 while committing session work (PR #3). (2026-09-14)
- [SveltePlot internals: match datums by position, not identity; guard empty data](learnings/svelteplot-datum-identity-empty-guard.md) — Two engine-level gotchas discovered while porting [[heatmap-component]] (2026-09-14), from the explanation of the SveltePlot 0.14.2 implemen… (2026-09-14)
- [kees.pippeloi.nl reference ports cleanly — same svelteplot 0.14.2 + Tailwind 4](learnings/kees-reference-ports-cleanly.md) — `E:\kees.pippeloi.nl` (esp. `src/routes/work/high-level`) is the reference project for chart-type components being ported into Labs. (2026-09-14)
- [WebView2 CDP gotchas: env-var flag, stale browser process, dual-stack vite](learnings/webview2-cdp-gotchas-env-var-flag-stale.md) — Follow-up to [[drive-data-monster-s-real-ui-over-cdp]] — four gotchas hit while verifying the 2026-09-12 redesign: (2026-09-12)
- [Stale-wiki file floods — only noise if an ignore pattern actually matches the tree](learnings/stale-wiki-file-floods-are-ignored.md) — Symptom and root cause — src-tauri/target leaked through, fixed via .wiki_ignore plus extension BUILTIN_IGNORES. (2026-09-12)
- [Never tree-scan .archive/ or src-tauri/target/ — du/find stall on the huge trees](learnings/never-tree-scan-archive-or-src-tauri.md) — The repo contains very large generated/historical trees: `.archive/` (entire superseded old app + chart-engine trials) and `src-tauri/target… (2026-09-12)
- [Local LLM blank-screen delay was hidden thinking tokens — disable via "thinking": {"type": "disabled"}](learnings/local-llm-blank-screen-delay-was-hidden.md) — Symptom (2026-09-11)

## Architecture

- [File tree](architecture/file-tree.md) — Complete project file listing with per-file descriptions. (2026-09-11)

<!-- wiki-memory:end -->
