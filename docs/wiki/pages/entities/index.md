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
- [Library page (/library)](./library-page-library.md) - A new top-level route intended to become the **central component library**: every component used in the app's UI shown in one place, where component devs regist
- [Library registry system (src/lib/library + /library routes)](./library-registry-system.md) - The shipped implementation of the library registry: a one-function registration point (`registerLibraryComponent`) that feeds both the `/library` views and the
- [library-component-builder skill (.pi/skills)](./library-component-builder-skill-pi-skills.md) - A pi project skill (agentskills.io-spec-conformant) that owns the full path from a user's component idea to a registered, tested library component: interview →
- [PageGrid component](./pagegrid-component.md) - The canvas renderer + editing surface of the central-charts page editor: lays out a `PageDoc` as rows of 12-col CSS grids — each row an optional-height shell of
- [LLM prompt button (/library detail)](./llm-prompt-button-library-detail.md) - LLM prompt button (/library detail)
- [SkeletonSetup component](./skeletonsetup-component.md) - The in-chart setup card rendered inside a `ChartCard` when `needsSetup(chart)` is true: inline grouped dropdowns (⭐ master items | source-table fields | linked-
- [dev-cdp.cmd (repo-root double-click CDP restart)](./dev-cdp-cmd-repo-root-double-click-cdp-restart.md) - `dev-cdp.cmd` is a double-clickable Windows command script at the repo root that restarts the dev app in a CDP-drivable state — the packaged version of the manu
- [App tab system (virtual tabs + bottom tab bar)](./app-tab-system-virtual-tabs-bottom-tab-bar.md) - The app's browser-like tab system: right-click an internal link → "Open in new tab"; the bottom bar lists the open tabs. Tabs are **virtual** — plain routes tra
- [Shared controls kit (charts/controls)](./shared-controls-kit-charts-controls.md) - The shared form-controls kit for every drawer, inspector, and modal surface in the app: nine small Svelte 5 components plus one CSS file, all built on the app's
- [RolePickerModal component](./rolepickermodal-component.md) - The pick/create modal opened from the SkeletonSetup card buttons (new-page-modal pattern): a searchable list over ⭐ master items, source-table fields, and linke
- [ExprEditor component](./expreditor-component.md) - Smart DuckDB expression editor for master items (Qlik-Sense-style): autocomplete over bound-table fields, master items and a curated DuckDB function catalog, SQL syntax highlighting, per-kind starter templates, and live validation + result preview against the bound table.
- [Create-in-/data round-trip](./create-in-data-round-trip.md) - Deep-link flow from a /pages chart pick surfaces to the full master-item editor in /data and back: chart → /data?tab=<kind>s&add=1&table=…&return=<slug>&block=<id> → ItemEditor preset form → save → /pages/<slug>?configure=<block>&attach=<itemId> → item attached + focused drawer reopened.
- [central-api (frontend invoke client)](./central-api-frontend-invoke-client.md) - What is it?
- [Workspace command module (Rust)](./workspace-command-module-rust.md) - What is it?
- [Workspaces page (/workspaces)](./workspaces-page-workspaces.md) - What is it?
