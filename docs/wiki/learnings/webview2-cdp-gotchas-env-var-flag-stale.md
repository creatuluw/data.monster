---
type: Learning
title: "WebView2 CDP gotchas: env-var flag, stale browser process, dual-stack vite"
description: "Follow-up to [[drive-data-monster-s-real-ui-over-cdp]] — four gotchas hit while verifying the 2026-09-12 redesign:"
tags: [cdp, tauri, webview2, windows, e2e, verification]
timestamp: "2026-09-12T16:56:41.898Z"
---

# WebView2 CDP gotchas: env-var flag, stale browser process, dual-stack vite

Follow-up to [[drive-data-monster-s-real-ui-over-cdp]] — four gotchas hit while verifying the 2026-09-12 redesign:

1. **Env var, not process arg**: on Windows/WebView2 the remote-debugging flag must be passed as `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS="--remote-debugging-port=<port>"` (env var form), not as a CLI arg to the exe.
2. **Port not binding?** A leftover WebView2 browser process from a flag-less first launch holds the user-data-dir. Find and kill that specific instance, then relaunch with the env var.
3. **IPv4/IPv6 mismatch**: the webview may resolve `localhost` to a different stack than vite binds. Start vite with `--host` (dual-stack) so the app can reach the dev URL. (Also detach it properly — plain backgrounding flaked; `nohup`-style detach worked.)
4. **Node ≥21 has native WebSocket** — quick CDP probes (HTTP /json list + `Runtime.evaluate`) need zero deps.

Bonus technique: when no vision tool is available, verify UI restyling objectively via **computed styles** over CDP (`getComputedStyle` on headings/buttons/body) instead of screenshots — confirmed Spectral/Public Sans, ledger-green brand mark, and tabular-nums this way.
