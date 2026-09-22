# INDEX — find your task

Read this table, open ONLY the format doc your task needs (each is self-contained).

| I want to... | Create / edit | Read first |
|---|---|---|
| Add or edit a report page | `dm/pages/<slug>.json` | `formats/page-doc.md` |
| Add a reusable measure or dimension | `dm/master-items/{measures,dimensions}/<id>.json` | `formats/master-items.md` |
| Save a SQL query | `dm/saved-queries/<slug>.sql` | `formats/saved-queries.md` |
| Save a Postgres connection | `dm/connections.json` + `.env` var | `formats/connections.md` |
| Link tables so charts can auto-JOIN | `dm/relationships.json` | `formats/relationships.md` |
| Understand the app's model | — | `concepts.md` |
| Walk through a common task | — | `recipes/common-tasks.md` |
| Check every page-doc field | — | `reference/page-doc-fields.md` |

Common intents:

- "Build a dashboard for table X" → read `concepts.md`, then `formats/page-doc.md`,
  then create `dm/pages/<slug>.json`. The open app shows it at /pages/<slug> instantly.
- "Add a profit measure usable in every chart" → `formats/master-items.md`.
- "Connect my Postgres" → `formats/connections.md` (secret goes to `.env`, never to you).

Validation: the app validates on load and hot-reloads. Broken JSON = problem banner +
the file is skipped; nothing crashes, nothing is deleted. Fix the file, it comes back.
