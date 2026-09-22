---
type: Decision
title: "Workspace content tree: dm/ with path-is-identity, native formats, agent README (proposed)"
description: Context
tags: [agents, workspace, file-tree, storage, path-is-identity, proposed]
status: proposed
timestamp: "2026-09-22T13:19:50.603Z"
---

# Workspace content tree: dm/ with path-is-identity, native formats, agent README (proposed)

## Context

Follow-up to [[decisions/agent-authors-app-content-by-editing-workspace-files-the-wor]]: the user answered Q1 with "A" — files canonical, DuckDB keeps only real data — and added "we need a smart file tree for this to work." A dumb folder dump won't do; the layout is the design (2026-09-22 planning session).

## The choice (proposed)

The workspace gains a `dm/` content tree the app reads by convention, and agents author directly:

```
my-workspace/
├── d8a_monster.duckdb          # source tables + query results ONLY
├── settings.json               # existing — LLM config, connections
├── README.md                   # auto-generated: documents this tree for humans AND agents
├── data/main/                  # existing — ingested source files
└── dm/
    ├── pages/revenue.json      # filename = slug → route /pages/revenue
    ├── master-items/measures/total_revenue.json   # filename = stable ref id
    ├── master-items/dimensions/region.json
    ├── relationships.json      # whole graph as one doc
    ├── saved-queries/top_customers.sql  # native .sql, not JSON-escaped
    └── drafts/                 # agent scratch space — app ignores everything here
```

Three principles that make it "smart":

- **Path is identity.** Slug = filename, master-item ref = filename, query name = filename. Kills the whole id-drift bug class from the E2E report (`tableName`/`table` mismatches — see [[pages/learnings/ref-based-master-items-tablename-mismatch-broke]]); rename = file rename. `/pages/revenue` and `dm/pages/revenue.json` are the same thing — an agent that can `ls` already knows the app's content map.
- **Native formats.** SQL lives in `.sql` files — agents write brilliant SQL and terrible JSON-escaped SQL. Other files stay JSON matching existing `PageDoc`/master-item shapes, so `validatePageDoc` validates them unchanged ([[pages/entities/chart-page-spec-spec-types-validator]]).
- **Self-describing.** The generated `README.md` is the agent's onboarding — agents read READMEs unprompted, so conventions transfer with zero install/config. This replaces most of what the MCP/`dm` skill ([[decisions/agent-surfaces-rust-backend-mcp-and-rest]]) was going to do.

Cheap by construction: the app reads only its convention paths; unknown files (agent notes, scratch, datasets) are ignored, so agents can drop anything without breaking the app.

## Alternatives considered (Q2: what does "smart file tree" mean?)

- **A — layout conventions** (path-is-identity, README-for-agents, native formats): what is proposed above.
- **B — in-app file-tree UI**: a VS Code-style workspace explorer so the user watches files appear live as the agent works, and clicks one to open it in the app.
- **C — both**: conventions now, explorer as the visible half of the same feature.
- **D — something else**: the user's answer settles this. Status locks when Q2 is answered.

## Consequences

- One-time migration: Rust exports the `d8a_monster_*` tables ([[pages/entities/pages-master-items-storage-rust]]) to files, then drops them — write-all-first, drop-only-if-all-succeeded, so a crash leaves tables intact.
- Labels and field-functions follow the same files pattern later.
- Live reload still needs the file watcher + Tauri event + echo suppression from the parent decision (the app's own writes, including the silent 60s pages auto-save, must not re-trigger the watcher).
