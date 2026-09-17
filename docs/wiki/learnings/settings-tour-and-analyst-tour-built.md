---
type: Learning
title: settings-tour and analyst-tour built — honest-beats-staged applied to the chat
description: Built 2026-09-16 — settings-tour and analyst-tour complete the 8-tour set in docs/tours/ (connect, preview, query, data-tables, pages, labs, settings, analyst).
tags: [tours, analyst, settings, app-tour-demo]
timestamp: "2026-09-16T14:40:07.364Z"
---

# settings-tour and analyst-tour built — honest-beats-staged applied to the chat

Built 2026-09-16 — settings-tour and analyst-tour complete the 8-tour set in docs/tours/ (connect, preview, query, data-tables, pages, labs, settings, analyst).

- **settings-tour** (3 frames): LLM config form on /settings (mode toggle Remote API/Local AI, endpoint/key/model fields — key is type=password so no key material serializes into the tour at all), Internal DB browser listing the 4 d8a_monster_* tables (saved_queries, field_functions, table_labels, table_metadata — waited for the real list, not the spinner), and the field_functions data grid.
- **analyst-tour** (6 frames): table picker on /analyst, selected card, honest EMPTY chat welcome (no live LLM behind the dummy key — narrated, not staged; question typed but NOT sent), then the provider picker states on /settings: Remote API active + Local AI llama.cpp ModelManager catalog (Fast/Balanced tiers). The provider/local-model picker lives in Settings, not in the chat UI — the analyst tour ends there by design.

Gotchas hit while building are in [[settings-swap-for-tours-must-cover-env-too]]. Both builds verify GESLAAGD (0.0px click deviation).
