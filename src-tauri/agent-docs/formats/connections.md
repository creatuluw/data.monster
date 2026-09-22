# Connections format

> Read when: saving or editing Postgres connections.

`dm/connections.json` holds NAMES and env-var REFERENCES only — never secrets, never
URLs (the URL contains the password). The actual connection string lives in the
workspace `.env` (gitignored).

Example — `dm/connections.json`:

```json
{
  "connections": [
    { "name": "Production", "urlEnv": "DM_CONN_PRODUCTION_URL" }
  ]
}
```

And in `.env`:

```text
DM_CONN_PRODUCTION_URL=postgresql://user:password@db.example.com:5432/appdb
```

Rules:

- `urlEnv` convention: `DM_CONN_<SLUG>_URL` (uppercase, dashes → underscores).
- If the referenced var is missing, the app tells the user exactly which var to add to
  which file. Never work around this by embedding credentials in the doc.
- To create a connection: add the entry to `connections.json` AND tell the user to put
  the real URL into `.env` themselves. Do not ask for the password in chat.
- `.env` is gitignored; keep it that way — the app generates the `.gitignore` for you.
