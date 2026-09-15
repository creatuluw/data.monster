# Pages

Knowledge graph: concepts, entities, and artifacts that make up this project.

- [Concepts](./concepts/) — Abstract ideas, definitions, and categories
- [Entities](./entities/) — Concrete named things, systems, tools, and records
- [Artifacts](./artifacts/) — Documents, diagrams, code files, and deliverables
- [LLM Sensitive Data Privacy Research](./artifacts/llm-sensitive-data-privacy-research.md) — Research report (15 cited sources) on how to use LLMs with sensitive data in a data-analyst app. Compiled 2026-09-12 from web research; motivated by Data Monste
- [LLM & Sensitive Data White Paper](./artifacts/llm-sensitive-data-white-paper.md) — Dutch-language white paper ("LLM's & Gevoelige Data") condensing the LLM privacy research into a single self-contained HTML file designed for mobile reading.
- [remote_chat command](./entities/remote-chat-command.md) — Tauri command that proxies remote LLM chat completions (e.g. z.ai `/chat/completions`) through the Rust backend, streaming tokens back as `local-llm:*` events.
- [LLM Sensitive-Data White Paper — Finance Edition](./artifacts/llm-sensitive-data-white-paper-finance-edition.md) — Non-technical (finance-audience) edition of the LLM sensitive-data white paper, in Dutch. Same facts as the technical original, rewritten in business framing fo
- [.wiki_ignore staleness policy](./entities/wiki-ignore-staleness-policy.md) — What is it?
- [Field Functions library](./entities/field-functions-library.md) — A user-extensible library of SQL field functions (e.g. formatting, extraction, math) that can be applied to table columns from the column drawer, backed by the
- [LabsPlaceholder component](./entities/labsplaceholder-component.md) — A one-prop Svelte 5 component that renders the standard Labs page shell with "Placeholder — coming soon." It is what every not-yet-built chart type in `/labs` s
- [ChartConfigDrawer component](./entities/chartconfigdrawer-component.md) — A reusable drawer shell for chart configuration panels in `/labs`. Any labs chart page can host its own config UI by rendering fields into the shared `children`
- [BarChart component](./entities/barchart-component.md) — What is it?
- [Central chart component design](./artifacts/central-chart-component-design.md) — What it documents
- [Central-charts spec &amp; task list](./artifacts/central-charts-spec-amp-task-list.md) — The planning document for the central reusable-chart build: report pages composed of chart/block objects on a 12-col grid, with a dual-mode (Design ⇄ Code) edit
