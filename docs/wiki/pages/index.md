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
- [Central charts spec & tasks](./artifacts/central-charts-spec-tasks.md) — The executable spec + task list for phase 1 of the central chart system: 13 FRs (FR-1..13) broken into 13 TDD tasks across five phases — Core (spec types/valida
- [Chart page spec (spec-types + validator)](./entities/chart-page-spec-spec-types-validator.md) — Central-charts FR-1: the TypeScript module holding the page document spec — `PageDoc` and all block/measure/dimension/filter/annotation/tooltip/axis types (`src
- [Pages & master-items storage (Rust)](./entities/pages-master-items-storage-rust.md) — The Rust-side persistence layer for the central-charts system: three internal DuckDB tables plus the Tauri commands that read/write them. Persists report `PageD
- [PageGrid component](./entities/pagegrid-component.md) — The canvas renderer for the central-charts page editor: lays out a
- [LLM agent connection research report](./artifacts/llm-agent-connection-research-report.md) — Fractal-research report on how to connect any LLM / coding agent / harness to data.monster and let it operate the app — add data & content, run analysis. Produc
- [central-api (frontend invoke client)](./entities/central-api-frontend-invoke-client.md) — What is it?
- [Library page (/library)](./entities/library-page-library.md) — A new top-level route intended to become the **central component library**: every component used in the app's UI shown in one place, where component devs regist
- [Library registry system (src/lib/library + /library routes)](./entities/library-registry-system-src-lib-library-library-routes.md) — The shipped implementation of the library registry: a one-function registration point (`registerLibraryComponent`) that feeds both the `/library` views and the
- [library-component-builder skill (.pi/skills)](./entities/library-component-builder-skill-pi-skills.md) — A pi project skill (agentskills.io-spec-conformant) that owns the full path from a user's component idea to a registered, tested library component: interview →
- [LLM prompt button (/library detail)](./entities/llm-prompt-button-library-detail.md) — A `Bot`-icon button in the top-right of the `/library/[id]` page head (tooltip on hover explains what it is). Click copies a ready-to-paste bootstrap prompt tha
