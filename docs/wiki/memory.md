---
okf_version: "0.1"
---

<!-- wiki-memory:start -->
# Memory — the live contract

Auto-generated digest of the most recent conventions, decisions, rules and
development patterns, plus architecture and global patterns — newest first.
The actual files live in the wiki subfolders; follow the links (clickable in /wiki).
Regenerated on every wiki write and on wiki_mark_synced. Generated 2026-09-12T11:06:03.745Z.

## Recent Decisions

- [Consolidate chart engines to Picasso.js + LayerChart, drop echarts/observable/svelteplot](decisions/consolidate-chart-engines-to-picasso-js-layerchart-drop-echa.md) — Context (2026-09-12)
- [Proxy remote LLM calls through Rust, not webview fetch](decisions/proxy-remote-llm-calls-through-rust-not-webview-fetch.md) — Context (2026-09-11)

## Active Rules

- [Route external API calls through Rust commands, never webview fetch](rules/route-external-api-calls-through-rust-commands-never-webview.md) — Guideline (2026-09-11)

## Recent Learnings — development patterns

- [Stale-wiki file floods — only noise if an ignore pattern actually matches the tree](learnings/stale-wiki-file-floods-are-ignored-build-artifacts-wiki-igno.md) — Symptom and root cause — src-tauri/target leaked through, fixed via .wiki_ignore plus extension BUILTIN_IGNORES. (2026-09-12)
- [Never tree-scan .archive/ or src-tauri/target/ — du/find stall on the huge trees](learnings/never-tree-scan-archive-or-src-tauri-target-du-find-stall-on.md) — The repo contains very large generated/historical trees: `.archive/` (entire superseded old app + chart-engine trials) and `src-tauri/target… (2026-09-12)
- [Local LLM blank-screen delay was hidden thinking tokens — disable via "thinking": {"type": "disabled"}](learnings/local-llm-blank-screen-delay-was-hidden-thinking-tokens-disa.md) — Symptom (2026-09-11)
- [Drive data.monster's real UI over CDP with --remote-debugging-port for e2e debugging](learnings/drive-data-monster-s-real-ui-over-cdp-with-remote-debugging-.md) — The changelog-e2e skill's technique transfers from the changelog.monster app to **data.monster**: launch the Tauri app with `--remote-debugg… (2026-09-11)
- [get_settings merges env/.env over settings.json — env is source of truth](learnings/get-settings-merges-env-env-over-settings-json-env-is-source.md) — Discovered while wiring `.env` into the app (2026-09-11). (2026-09-11)
- [z.ai 401 "code 1000 Authentication Failed" means the key itself is bad — verify with curl, not app code](learnings/z-ai-401-code-1000-authentication-failed-means-the-key-itsel.md) — Symptom (2026-09-11)
- [LLM provider retention, part 2: Kimi, Z.ai, Together, Qwen — Kimi policy contradiction, Z.ai DPA strength, tier framework](learnings/llm-provider-retention-part-2-kimi-z-ai-together-qwen-kimi-p.md) — Follow-up research (2026-09-11) on Kimi (Moonshot), Z.ai (Zhipu/GLM), Together AI, and Qwen (Alibaba Model Studio), verified against primary… (2026-09-11)
- [LLM API data retention: "no training" ≠ "no storage"; local models are ZDR by construction](learnings/llm-api-data-retention-no-training-no-storage-local-models-a.md) — Research verified against primary docs (2026-09-11) on how the 6–8 major LLM API endpoints handle data retention and sensitive data. Directl… (2026-09-11)
- ["Couldn't find callback id" Tauri warning is a benign reload artifact](learnings/couldn-t-find-callback-id-tauri-warning-is-a-benign-reload-a.md) — `[TAURI] Couldn't find callback id <n>. This might happen when the app is reloaded while Rust is running an asynchronous operation.` is beni… (2026-09-11)

## Architecture

- [File tree](architecture/file-tree.md) — Complete project file listing with per-file descriptions. (2026-09-11)

<!-- wiki-memory:end -->
