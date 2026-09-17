# Rules

- [Route external API calls through Rust commands, never webview fetch](./route-external-api-calls-through-rust-commands-never-webview.md) - Guideline
- [Keep test files and vitest imports out of src/](./keep-test-files-and-vitest-imports-out-of-src.md) - Keep test files and vitest imports out of src/
- [Pin Tailwind @source scanning to src/ and app.html in app.css](./pin-tailwind-source-scanning.md) - Pin Tailwind @source scanning to src/ and app.html in app.css
- [Each /labs chart owns its config panel](./each-labs-chart-owns-its-config-panel.md) - Guideline
- [Interview the user one question at a time with lettered multiple-choice options](./interview-one-question-at-a-time.md) - Guideline
- [Spec-driven features: TDD + Karpathy skills referenced in every todo](./spec-driven-features-tdd-karpathy-in-todos.md) - Guideline
- [Card spacing comes from the grid gap, never per-card margins](./card-spacing-from-grid-gap-not-margins.md) - In any grid of chart/component cards (page editor canvas, labs), inter-card
- [All pages are capped at 1920px and centered by the shared layout — no per-page opt-out](./app-content-capped-at-shared-max-width.md) - Guideline
- [Pointer cursor comes from one global rule in app.css](./pointer-cursor-from-global-rule-app-css.md) - Guideline
- [Render markdown via marked + .prose-chat, never a new pipeline](./render-markdown-via-marked-prose-chat.md) - When rendering any markdown anywhere in the app (docs tabs, chat, notes), parse with `marked` (already a dependency) and wrap the output in the `.prose-chat` cl
