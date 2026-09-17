# Q: What tool/resource surface should a data-analysis app expose to LLM agents? (Survey of DB/BI/analytics MCP server tool catalogs, recommended list for data.monster)

## Findings

Reference servers (as of 2026-09)

- modelcontextprotocol/servers archived the DB reference servers to servers-archived (postgres, sqlite etc. removed from main src/; only fetch/filesystem/git/memory etc. remain) [verified: https://raw.githubusercontent.com/modelcontextprotocol/servers/main/README.md] - confidence: high
- Postgres reference server: ONE query tool, read-only only (all queries run inside a READ ONLY transaction); table schemas exposed as MCP *resources* (postgres://host/table/schema) [verified: https://raw.githubusercontent.com/modelcontextprotocol/servers-archived/main/src/postgres/README.md] - confidence: high
- SQLite reference server: 6 tools split read/write - read_query, write_query, create_table, list_tables, describe-table, append_insight; plus memo://insights resource = auto-updating business-insights memo (the only business-content pattern in reference servers) [verified: https://raw.githubusercontent.com/modelcontextprotocol/servers-archived/main/src/sqlite/README.md] - confidence: high

DuckDB-adjacent

- MotherDuck MCP (mcp-server-motherduck): tools execute_query (arbitrary DuckDB SQL), list_databases, list_tables, list_columns, switch_database_connection; read-only by default, --read-write flag enables writes; results capped 1024 rows / 50,000 chars default, configurable --max-rows/--max-chars; local variant ingests/exports via local filesystem [verified: https://github.com/motherduckdb/mcp-server-motherduck README, main] - confidence: high
- MotherDuck warns read-only mode alone is not sufficient for third-party exposure (filesystem access, DuckDB settings changes) [verified: same README, Securing for Production] - confidence: high

Warehouse/BI vendors

- Supabase MCP: execute_sql (single SQL tool, not split read/write), list_tables, list_extensions, list_migrations/apply_migration (DDL as versioned migrations), query_logs, get_advisors, generate_typescript_types, edge-fn deploy, project mgmt, search_docs; read-only via read_only=true param = queries executed as a read-only Postgres user; feature flags (features=database,docs) trim the catalog [verified: https://supabase.com/mcp + https://github.com/supabase-community/supabase-mcp README, main] - confidence: high
- Databricks SQL MCP (managed, /api/2.0/mcp/sql): run AI-generated SQL read+write on Unity Catalog, governed by UC permissions; writes ON by default, read-only via disallow_writes=true in system.ai.dbsql_policy; warehouse_id set via _meta (preset config, not LLM-chosen); truncates large result sets in tool responses; async execution - agent starts query then polls [verified: https://docs.databricks.com/aws/en/agents/mcp-tools/databricks-sql] - confidence: high
- Snowflake: community Snowflake-Labs/mcp repo is deprecated - migrate to the official Snowflake MCP Server [verified: https://github.com/Snowflake-Labs/mcp README, main] - confidence: high; old community tool list (read_query/write_query/append_note) not re-verified - reported-only
- dbt MCP (dbt-labs/dbt-mcp): SQL (execute_sql with Semantic Layer, text_to_sql); semantic-layer tools (list_metrics, list_saved_queries, query_metrics, get_dimension_values, get_dimensions, get_entities); discovery (get_all_models, get_all_sources, get_lineage, get_node_details, get_exposures = downstream dashboards, get_model_health); CLI tools (build, compile, run, show, test, list) [verified: https://github.com/dbt-labs/dbt-mcp README, main] - confidence: high

Notebook/report-app servers (business content)

- Hex MCP: Knowledge tools (search projects, create/get/continue Threads, get current user) + Project editing tools (create/get project, list/get/create/update/delete cell, run cell, run notebook, get run status, get cell output, get chart image, list data connections); pagination cursor 25 default / 100 max; cell output is a preview that reports row limit, rows returned, total row count, truncated flag - columns never dropped [verified: https://learn.hex.tech/docs/api-integrations/mcp-server] - confidence: high
- Deepnote MCP (https://deepnote.com/mcp, HTTP POST, API key or OAuth): workspace (get_me, search, list_projects, create_project); notebook/block authoring (get_notebook, create_notebook, create_block, update_block, reorder_notebook_blocks, duplicate_notebook, generate_project_url = hand user a link to what was built); runs (create_run/get_run - returns presigned snapshot download URL - /list_notebook_runs); integrations incl. get_integration with cached table structure; docs (list_docs/get_doc); tools annotated read-only vs state-changing so clients can warn [verified: https://deepnote.com/docs/deepnote-mcp] - confidence: high
- Evidence: no data-query MCP; ships a Docs MCP that teaches agents to author Evidence reports (list_components, get_component, get_component_attributes, get_component_examples, list_docs, read_doc, search_docs) - component metadata + docs as the agent surface for a code-based BI tool [verified: https://docs.evidence.dev/mcp/docs] - confidence: high

## Tensions

- One general SQL tool (Supabase/MotherDuck/Databricks) vs split read/write tools (SQLite reference): split tools let clients gate writes at the protocol layer; vendors instead rely on server-side read-only users/policies.
- Result-cap conventions differ: hard row/char caps (MotherDuck 1024r/50k), unspecified truncation (Databricks), preview-with-truncation-metadata (Hex), snapshot download URL (Deepnote). No standard.
- Business content (dashboards/saved queries) is exposed only by notebook-app servers (Hex/Deepnote) and dbt (saved queries/exposures); pure DB servers expose none - precedent for data.monster is Hex/Deepnote, not postgres.

## Open questions

- Official Snowflake MCP Server tool list (docs.snowflake.com cortex-agents-mcp page not fetched).
- Whether async query + polling (Databricks pattern) matters for embedded DuckDB (likely not - local engine is fast/synchronous).

## Recommended tool list for data.monster

Discovery (read-only, always on): list_tables (name, kind, row count, tags, source), describe_table (columns + types), list_saved_queries, list_pages, get_page (PageDoc JSON), search_workspace.

Query/analysis: read_query (SELECT/SHOW/DESCRIBE only, enforced in Rust), write_query (CTAS/DDL; hidden unless write mode). Return JSON preview with rows_returned, total_rows, truncated (Hex pattern); optional export-to-workspace-CSV for big results (Deepnote snapshot pattern).

Ingestion: ingest (path or URL, CSV/Parquet/JSON, target table), import_postgres (remote conn), list_ingestions/status - write-mode gated.

Content: save_query (create/update saved query), create_page/update_page (PageDoc spec), set_table_tags.

App-control: refresh_ui (push table/page change events so the Tauri UI reloads), notify (desktop toast on long jobs/failures), get_app_status. Gate everything: --read-only default, --read-write opt-in (MotherDuck pattern), plus readOnlyHint annotations per tool (Deepnote pattern).

## Search trail

- queries: Hex MCP server tools run_project documentation; Deepnote MCP server agent tools; Databricks MCP server tools execute-sql list-catalogs documentation; Evidence.dev MCP server tools build reports
- fetched: https://raw.githubusercontent.com/modelcontextprotocol/servers/main/README.md - archival of postgres/sqlite reference servers; https://raw.githubusercontent.com/modelcontextprotocol/servers-archived/main/src/postgres/README.md - single read-only query tool, schema resources; https://raw.githubusercontent.com/modelcontextprotocol/servers-archived/main/src/sqlite/README.md - 6 tools, append_insight, memo://insights; https://github.com/supabase-community/supabase-mcp (README) + https://supabase.com/mcp - execute_sql, read_only param, migrations, features flag; https://github.com/dbt-labs/dbt-mcp (README) - SQL/semantic/discovery/CLI tool groups; https://github.com/motherduckdb/mcp-server-motherduck (README) - execute_query, read-only default, 1024/50k caps; https://docs.databricks.com/aws/en/agents/mcp-tools/databricks-sql - read/write default, disallow_writes, _meta.warehouse_id, truncation, async polling; https://learn.hex.tech/docs/api-integrations/mcp-server - knowledge + project-editing tools, pagination 25/100, output preview metadata; https://deepnote.com/docs/deepnote-mcp - full tool list, annotations, snapshot URL; https://docs.evidence.dev/mcp/docs - docs/components MCP; https://github.com/Snowflake-Labs/mcp (README) - deprecation notice
- rejected: mcpservers.org/servers/franccesco/hex-mcp, mcp.pipedream.com/app/hex, lobehub.com hex-dashboard-mcp (third-party aggregators, not primary); hex-inc/hex-cursor-plugin (wrapper, primary docs better); docs.workato.com Databricks Data Explorer MCP (reseller view, out of scope); Snowflake-Labs old tool names (absent from current README - unverified)
