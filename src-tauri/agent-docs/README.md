# data.monster workspace

This folder IS the app's state: a portable DuckDB, your source data, and every piece
of app content as plain files under `dm/`. You (a coding agent) add or change content
by editing those files with your normal file tools — no app restart needed; the open
app hot-reloads within ~300ms.

## The tree

- `d8a_monster.duckdb` — source tables + query results (the app owns this; never edit)
- `settings.json` — app settings, non-secret only
- `.env` — SECRETS ONLY (gitignored). API keys and connection URLs live here.
- `data/main/` — ingested source files
- `dm/pages/<slug>.json` — report pages; the filename is the page slug (route /pages/<slug>)
- `dm/master-items/measures/<id>.json`, `dm/master-items/dimensions/<id>.json` — reusable expressions
- `dm/relationships.json` — the table relationship graph (one doc)
- `dm/saved-queries/<slug>.sql` — saved SQL (optional `-- dm: {json}` meta header)
- `dm/connections.json` — saved connection NAMES + env-var references (never secrets)
- `dm/docs/` — format docs; START AT `dm/docs/INDEX.md`
- `dm/drafts/` — your scratch space; the app ignores everything here

## The rules

1. Read `dm/docs/INDEX.md` first: it routes your task to the one format doc you need.
2. One file per item; the FILENAME is the identity (slug / id / name). Do not rename
   casually — routes and chart references point at filenames.
3. Never put secrets in any file above: reference a `.env` var instead.
4. Writes should be atomic (temp file + rename) if you script them; the app always is.
5. Invalid JSON is safe: the app reports it as a problem and skips the file — fix it.

## Version

App-owned docs live in `dm/docs/`; the app refreshes them when the app version changes.
Your notes belong in `dm/drafts/`.

Next: open `dm/docs/INDEX.md` and find your task.
