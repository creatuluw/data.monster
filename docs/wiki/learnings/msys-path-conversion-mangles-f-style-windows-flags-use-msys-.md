---
type: Learning
title: MSYS path conversion mangles /f-style Windows flags — use MSYS_NO_PATHCONV=1 or PowerShell
description: Discovered 2026-09-22 while running the blessed CDP restart chain from the MSYS/Git-Bash shell (verifying the /data tab URL-sync fix).
tags: [windows, msys, bash, cdp-restart-chain, gotcha]
timestamp: "2026-09-22T08:25:12.889Z"
---

# MSYS path conversion mangles /f-style Windows flags — use MSYS_NO_PATHCONV=1 or PowerShell

Discovered 2026-09-22 while running the blessed CDP restart chain from the MSYS/Git-Bash shell (verifying the /data tab URL-sync fix).

**Symptom**: `taskkill /f /pid <n>` and `start` quoting silently misbehaved — MSYS path conversion rewrote leading-slash arguments into Windows paths (`/f` → `F:/`), changing the command's meaning without erroring.

**Fix**: prefix the command with `MSYS_NO_PATHCONV=1`, or run Windows-native commands (`taskkill`, `start`) through PowerShell instead of bash.

**Related practice from the same session**: when restarting the app, kill the app + vite by **specific PID**, never `taskkill /IM node.exe /f` — the blanket image-name kill takes down unrelated node processes.
