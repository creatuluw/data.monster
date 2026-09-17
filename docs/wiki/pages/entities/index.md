# Entities

_Concrete named things will be listed here._
- [remote_chat command](./remote-chat-command.md) - Tauri command that proxies remote LLM chat completions (e.g. z.ai `/chat/completions`) through the Rust backend, streaming tokens back as `local-llm:*` events.
- [.wiki_ignore staleness policy](./wiki-ignore-staleness-policy.md) - What is it?
- [Field Functions library](./field-functions-library.md) - A user-extensible library of SQL field functions (e.g. formatting, extraction, math) that can be applied to table columns from the column drawer, backed by the
- [LabsPlaceholder component](./labsplaceholder-component.md) - A one-prop Svelte 5 component that renders the standard Labs page shell with "Placeholder — coming soon." It is what every not-yet-built chart type in `/labs` s
- [BarChart component](./barchart-component.md) - What is it?
- [Chart page spec (spec-types + validator)](./chart-page-spec-spec-types-validator.md) - Central-charts FR-1: the TypeScript module holding the page document spec — `PageDoc` and all block/measure/dimension/filter/annotation/tooltip/axis types (`src
- [Pages & master-items storage (Rust)](./pages-master-items-storage-rust.md) - The Rust-side persistence layer for the central-charts system: three internal DuckDB tables plus the Tauri commands that read/write them. Persists report `PageD
- [ChartConfigDrawer component](./chartconfigdrawer-component.md) - A reusable drawer shell for chart configuration panels, hosted **inside each chart component** in `/labs`: a chart accepts an optional `config` snippet and togg
- [PageGrid component](./pagegrid-component.md) - The canvas renderer for the central-charts page editor: lays out a
- [central-api (frontend invoke client)](./central-api-frontend-invoke-client.md) - What is it?
- [Library page (/library)](./library-page-library.md) - A new top-level route intended to become the **central component library**: every component used in the app's UI shown in one place, where component devs regist
- [Library registry system (src/lib/library + /library routes)](./library-registry-system.md) - The shipped implementation of the library registry: a one-function registration point (`registerLibraryComponent`) that feeds both the `/library` views and the
- [library-component-builder skill (.pi/skills)](./library-component-builder-skill-pi-skills.md) - A pi project skill (agentskills.io-spec-conformant) that owns the full path from a user's component idea to a registered, tested library component: interview →
- [LLM prompt button (/library detail)](./llm-prompt-button-library-detail.md) - A `Bot`-icon button in the top-right of the `/library/[id]` page head (tooltip on hover explains what it is). Click copies a ready-to-paste bootstrap prompt tha
