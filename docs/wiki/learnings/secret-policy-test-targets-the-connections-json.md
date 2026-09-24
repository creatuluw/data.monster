---
type: Learning
title: Secret-policy test targets the connections.json example — the .env placeholder password is intentional
description: "Hit while finishing files-011's secret-policy test (2026-09-22): the test flagged `password@` inside the `.env` sample URL (`DM_CONN_PRODUCTION_URL=postgresql:/"
tags: [secrets, testing, agent-docs, dm-tree]
timestamp: "2026-09-22T15:30:43.295Z"
---

# Secret-policy test targets the connections.json example — the .env placeholder password is intentional

Hit while finishing files-011's secret-policy test (2026-09-22): the test flagged `password@` inside the `.env` sample URL (`DM_CONN_PRODUCTION_URL=postgresql://user:password@db.example.com:...` in `src-tauri/agent-docs/formats/connections.md`). That placeholder is **intentional** — it shows the user/agent where a full connection URL goes, and `.env` is gitignored by design.

The real invariant the test must assert: the **`connections.json` example agents copy** is secret-free (`{name, urlEnv}` env-var references only). Don't "fix" the `.env` placeholder, and don't loosen the policy — point the assertion at the connections example.
