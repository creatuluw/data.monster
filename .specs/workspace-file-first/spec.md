# Workspace File-First — Agents Author the dm/ Tree (files canonical, realtime both ways, agent docs, prompts page)

## Feature Overview

The workspace folder becomes the coding agent's interface to the app. All app content
(pages, master items, relationships, saved queries, connections) moves from internal
DuckDB tables to plain files under `dm/`; the editor writes through to files in realtime
and a file watcher hot-reloads the app when the agent writes. App-owned agent docs
(`README.md` + `dm/docs/` with skill-style progressive disclosure) onboard any agent with
zero config, and a new `/agent` page gives users copy-paste starter prompts.

Scope (user-locked 2026-09-22): **all content types in one build** — no content-type
phasing. Supersedes the MCP-first connection plan for content authoring (see wiki:
`workspace-files-are-canonical-agents-author-content-by-editi`).

## Success Criteria

- A coding agent with ONLY file tools (read/write/glob), given the workspace path, can:
  create a valid page that renders at `/pages/<slug>`, add a master item usable by
  charts, add a saved query, add a Postgres connection profile — with no app restart.
  Verified once end-to-end by actually running an agent session.
- Human edits in the app (Design canvas, inspector, Code tab, drawers) appear in the
  files within ~400ms; agent file edits appear in the open app live, with validation
  errors surfaced (toast + inline if the affected doc is open) and nothing breaking.
- No save button, no dirty-state, no 60s auto-save anywhere — the file IS the save.
- A workspace folder can be `git init`'d and committed safely: no secrets in any
  committable file, `.gitignore` guards `.env` + DB files, dm/ diffs stay clean.
- Existing workspaces migrate losslessly on first launch of the new version; a crash
  mid-migration leaves tables intact and retries cleanly.
- A fresh agent reading only `README.md` → one `INDEX.md` row → one format doc can
  author correct content (progressive disclosure holds: no doc requires reading another
  except via explicit links).

## Design Rationale

- **Files canonical** (user-locked): DuckDB's single-writer lock forbids a second
  process; files are the coding agent's native interface — no MCP, tokens, or install.
- **Path is identity**: slug/master-item-id/query-slug = filename; route ↔ file 1:1;
  kills the id-drift bug class from the pages E2E report.
- **Validator stays single-sourced in TS**: Rust does parse-level shape checks only
  (enough to classify `changed` vs `error` events); full `validatePageDoc` semantics
  stay frontend (render-time guards + Code mode already have them).
- **Progressive disclosure mirrors this repo's own tooling** (pi skills / OKF wiki):
  L0 README always read → L1 INDEX routes intent → topic → L2 format doc with one
  annotated example → L3 reference only for deep cases. Token budgets enforced by test.
- **Atomic writes everywhere**: temp file + rename so the other side never reads a
  partial JSON.
- **Conflict model = VS Code's**: on-disk change + local dirty → "Reload / Keep mine"
  banner. No merge machinery.

## Constraints/Assumptions

- Stack unchanged: Svelte 5 runes, SvelteKit, Tauri v2, DuckDB via Rust commands,
  Tailwind 4. No new npm deps. New Rust dep: `notify` (file watcher) — allowed.
- Test files in `tests/`, never in `src/` (vite dep-optimizer rule). TDD throughout.
- `settings.json` stays workspace-scoped but holds NO secrets: `llmApiKey` stripped on
  save, resolved from env on read (existing env-over-settings merge in settings.rs).
  Labels + field functions STAY in DuckDB this build (small, app-internal; follow-up).
- **Secret policy (user-set 2026-09-22; wiki rule): workspaces can be git-version controlled.**
  Secrets live ONLY in the workspace `.env` (gitignored); `dm/` files + `settings.json`
  carry env-var references, never values. App generates a workspace `.gitignore` when
  missing (`.env`, `*.duckdb`, `*.duckdb.wal`); committing `data/main/` is the user's call.
- **Git-safe serialization**: deterministic key order, no volatile timestamps in `dm/`
  files (clean diffs).
- Connections: non-secret fields + `passwordEnv` reference; missing var → actionable
  error naming the var and the `.env` path.
- Secrets/storage rule "route external API calls through Rust" unaffected.

## The workspace tree

```text
my-workspace/
├── d8a_monster.duckdb          # source tables + query results ONLY
├── settings.json               # non-secret config only (API key stripped on save)
├── .env                        # SECRETS ONLY — LLM_API_KEY, DM_CONN_<NAME>_PASSWORD (gitignored)
├── .gitignore                  # generated when missing: .env, *.duckdb, *.duckdb.wal
├── README.md                   # L0 agent entry point (generated, ≤60 lines)
├── data/
│   ├── main/                   # existing — ingested source files
│   └── incoming/               # drop zone → auto-ingest (FR-13, cuttable)
└── dm/                         # app content — the part agents author
    ├── pages/<slug>.json       # PageDoc; filename = slug → /pages/<slug>
    ├── master-items/
    │   ├── measures/<id>.json
    │   └── dimensions/<id>.json    # filename = item id; kind picks folder
    ├── relationships.json      # whole graph, one doc
    ├── saved-queries/<slug>.sql    # SQL body + optional `-- dm: {json}` meta header
    ├── connections.json        # Postgres profiles — NO secrets, passwordEnv refs (NEW)
    └── docs/                   # app-owned agent docs (regenerated on version change)
        ├── .version            # app version marker — drives regeneration
        ├── INDEX.md            # L1 wayfinding: intent → file → format doc (≤100 lines)
        ├── concepts.md         # workspace, tables, pages, master items (short)
        ├── formats/
        │   ├── page-doc.md         # + ONE annotated minimal example
        │   ├── master-items.md
        │   ├── saved-queries.md
        │   ├── connections.md
        │   └── relationships.md
        ├── reference/
        │   └── page-doc-fields.md  # L3 exhaustive field tables
        └── recipes/                # task walkthroughs (build-dashboard, add-measure, …)
```

`dm/drafts/` (agent scratch) and any unknown files are ignored by the app — the agent
can scribble anywhere safely. Docs files carry a `> Read when: …` header line so agents
can skip irrelevant ones (token discipline).

## Functional Requirements

### Phase A — Files are canonical

**FR-1 dm/ conventions + atomic write (Rust `dm_store` module).** Path mapping
kind↔path for all content types; `atomic_write(path, bytes)` (temp + rename);
parse-level shape checks per kind (`valid JSON`, has required keys — enough to classify
events; NOT full validation). Acceptance: cargo tests — round-trip mapping for every
kind, atomic write leaves no temp file on success, content intact on simulated crash
(temp left behind, original untouched).

**FR-2 Pages file store.** `list_pages`/`get_page`/`save_page`/`delete_page` remapped to
`dm/pages/*.json`; list skips unparseable files but reports them (`{slug, error}`); save
normalizes through the existing shape and writes atomically. Acceptance: cargo tests —
CRUD over tempdir workspace; invalid file appears in list errors, never breaks listing;
filename=slug enforced (save to `revenue.json` → slug `revenue`).

**FR-3 Master items + relationships store.** Items → `dm/master-items/{measures,
dimensions}/<id>.json` (filename = `id`; `kind` → folder; `createdAt/updatedAt` dropped —
file mtime replaces them); relationships graph → one `relationships.json`. CRUD commands
remapped. Acceptance: cargo tests — CRUD both kinds; id↔filename round-trip; ref
integrity: a chart referencing `{"ref": id}` resolves after store swap (frontend test in
`tests/`).

**FR-4 Saved queries file store.** `dm/saved-queries/<slug>.sql`; optional leading
`-- dm: {"name": …, "description": …, "tags": […]}` comment header (app-written,
agent-editable, absent = defaults); body is the SQL. CRUD remapped. Acceptance: cargo
tests — header parse/serialize round-trip (incl. SQL containing `--` comment lines and
quotes), headerless file → name=slug, tags=[].

**FR-5 Connections store (new persistence, secret-free).** `dm/connections.json`:
`[{name, host, port, database, user, passwordEnv}]` — non-secret metadata only; the
password lives in the workspace `.env` under the referenced var name. Rust: extend the
existing `.env` parser in `settings.rs` to resolve ARBITRARY vars (workspace `.env` →
process env), resolve `passwordEnv` at connect time; missing var → error naming the var
and the `.env` path. `/connect` gains a saved-connections list (save current form minus
secret, load, delete). Acceptance: cargo tests — CRUD + unknown fields preserved;
password resolution from workspace `.env`; missing-var error; a scan asserting
connections.json never contains secret-shaped values. Frontend test — save/load
round-trip via mocked invoke.

**FR-6 Migration + workspace git bootstrap.** On DuckDB init: if `d8a_monster_pages` /
`_items` / `_relationships` / `_saved_queries` exist → export all rows to files (write ALL files
first) → verify read-back → drop tables. Idempotent (tables absent = no-op). If files
and rows both exist (crash between write and drop): files win, rows dropped. Missing
rows for an existing file = no file overwrite. Bootstrap on workspace open (when missing):
write `.gitignore` (`.env`, `*.duckdb`, `*.duckdb.wal`) — never overwrite an existing one;
`llmApiKey` scrubbed from `settings.json` on next save. Acceptance: cargo tests —
full export, crash-point retries (files partial, tables full), idempotent second run,
files-win case, gitignore generated only when missing.

### Phase B — Realtime both ways

**FR-7 File watcher (Rust).** `notify` watcher on `dm/` (recursive); debounce ~300ms;
echo suppression (just-wrote path map, window ≥500ms > write debounce); classify event
→ validate shape (FR-1 checks) → emit Tauri event `dm:changed {kind, name}` or
`dm:error {path, reason}`. `drafts/` and docs/ ignored. Acceptance: cargo integration
tests over tempdir — external write emits correct event after debounce; app write
through `dm_store::atomic_write` emits nothing; unparseable write → `dm:error`.

**FR-8 Frontend live-reload.** Listener (app store layer) for `dm:changed`/`dm:error`;
changed → invalidate/reload the matching store + any open view of that doc; error →
toast + inline banner if the affected doc is open. Store logic pure and unit-tested.
Acceptance: `tests/` — event → store invalidation matrix (page open/closed, items,
queries); manual CDP verify at task end.

**FR-9 Pages editor write-through.** Delete: save button, dirty tracking, 60s
auto-save interval (and the "saved" toast). Every doc change (Design + Code modes)
→ debounced 400ms `atomic_write`. Conflict banner: file changed on disk since last
read AND local edits pending → "Reload / Keep mine" (keep mine = write anyway).
Acceptance: `tests/` — debounce coalesces N changes into 1 write; conflict decision
matrix; manual CDP verify (edit file externally → editor updates; two-way race →
banner).

**FR-10 Write-through everywhere else.** Shared `writeThrough` helper (debounce +
conflict check, extracted from FR-9) reused by: /data Measures | Dimensions drawers,
relationship editor, saved-queries UI, connections list. Same deletion of explicit
save affordances. Acceptance: `tests/` — helper unit tests; each surface wired;
manual CDP verify.

### Phase C — Docs the agent reads + prompts the user copies

**FR-11 Agent docs layer (progressive disclosure).** Files live in repo at
`src-tauri/agent-docs/`, embedded via `include_str!`, written into the workspace when
missing or when `dm/docs/.version` ≠ app version (app-owned; header notes this).
Structure: `README.md` (L0, ≤60 lines: workspace map + tree + write rules + secret
policy + INDEX pointer) → `dm/docs/INDEX.md` (L1, ≤100 lines: intent → file → format-doc table) →
`dm/docs/formats/*.md` + `concepts.md` + `recipes/*.md` (L2, ≤150 lines each, `Read
when:` header, ONE annotated minimal-valid example, explicit links to L3) →
`dm/docs/reference/page-doc-fields.md` (L3, exhaustive). Acceptance: `tests/` — line
budgets enforced (README/INDEX/formats), every INDEX row's target file exists, every
format doc's example parses as valid JSON/SQL (parse-level), docs written on version
change; `cargo test` for embedding.

**FR-12 /agent prompts page.** New route `/agent` (nav item). Curated prompt cards:
markdown files in repo (`src/lib/agent-prompts/*.md` with small frontmatter: title,
description, tags), rendered with `marked` + `.prose-chat` (house rule — no new
pipeline), copy-to-clipboard per card. Every prompt: names the workspace folder, says
"read `README.md` first, then only the INDEX row you need", collaborative ones instruct
interview-style work (one question at a time). Prompts touching connections or keys
instruct the agent to write env-var REFERENCES into `dm/` files and tell the USER to put
the secret in `.env` — agents never handle raw secrets. Starters: onboarding (connect data +
first page), build a dashboard from table X (interview), add a measure, ingest a CSV,
clean up the workspace, explain this workspace. Acceptance: `tests/` — frontmatter
parse; component test — renders cards, copy writes clipboard; manual visual check.

### Phase D — Extras

**FR-13 `data/incoming/` drop folder (cuttable).** Watcher detects new file →
auto-ingest via existing /connect detection with smart defaults (skip preview) →
`dm:changed {kind: "table"}` → table lists refresh. Ingest failures → `dm:error` with
reason; file stays (never deleted). Acceptance: cargo integration test — drop CSV →
ingest event + table exists; bad CSV → error event, file untouched.

## Out of scope / follow-ups

- Labels + field functions move to files (stay in DuckDB this build).
- Ops surface (loopback REST + `dm` CLI for run-query/ingest) — only if a real gap
  appears after living with file-first.
- In-app workspace file explorer view (the "watch files appear" UI) — separate feature.
- MCP/REST — superseded for authoring; revisit only for ops.
