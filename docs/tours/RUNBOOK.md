# Tour runbook — data.monster app tours (app-tour-demo, Tauri adaptation)

Proven pipeline (connect-tour is the reference). Follow it exactly.

## Environment (already running — do NOT restart the app)

- App: `tauri dev` running, webview driven over **CDP http://localhost:9222**
  (page at http://localhost:6123). Never `browser.close()` — capture.mjs ends
  with `process.exit(0)`.
- Demo workspace: `E:/demo-tour-workspace` (NOT the user's E:\workspace).
  DuckDB table `superstore` (51,290 rows) already seeded.
- URL server: http://localhost:8123/regions.csv (small CSV, served from the
  demo workspace). The connect flow downloads it live.
- All commands run with **cwd = E:/data.monster**.
- If CDP is down: `curl http://localhost:9222/json/version`. If truly dead,
  relaunch: `cd E:/data.monster && WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS="--remote-debugging-port=9222" nohup npm run dev > /tmp/dm-dev.log 2>&1 &`
  and wait for the port.

## Per tour (one folder, four files)

Folder: `docs/tours/<feature>-tour/` containing `capture.mjs`, `steps.json`,
`tour-assets.json`, `<feature>-tour.html`.

1. **Read the route source** for real anchors: `src/routes/<route>/+page.svelte`
   (+ any component it renders for the interesting part, e.g. PreviewPane).
   Anchors are `[name, '() => finder']`; mark text buttons via
   `[...document.querySelectorAll('button')].find(b => b.textContent.trim() === '…')`.
2. **capture.mjs** — copy `docs/tours/connect-tour/capture.mjs`, replace ONLY
   the DRAAIBOEK block (frames, seed, cleanup). Keep the machinery as-is.
   - `url: '/route'` navigates; `url: null` stays (previous voor navigated).
   - Wait for REAL content: `page.waitForSelector(<data-bearing selector>,
     { timeout: 60000 })`, never just fixed ms. The preview frame bug was
     exactly this: spinner captured instead of data.
   - A click-step's result must be the next captured frame. Tag the anchor a
     later click needs INTO the frame it lives on.
   - seed/cleanup via `invoke('command', { args })` — the real app API. Drop
     tables a tour created (check `src-tauri/src/commands/tables.rs` for the
     drop/rename commands). The `regions` table (from the preview tour) may
     persist.
3. **Run** `node docs/tours/<feature>-tour/capture.mjs` — every frame must log
   `anchors n/n`. If less, inspect the frame HTML in tour-assets.json and fix
   the wait/anchor. Do not proceed with partial anchors.
4. **steps.json** — schema + rubric: `cap` short badge, `tekst` 20–240 chars
   (what you do + why), `klaar` on every click, NO em-dashes, `tour` arrays
   point things out, `act: "type"` types `text` live. English narration.
   Frames must exist; type-`text` must match what capture actually typed.
5. **Build + verify** (exit must be GESLAAGD):
   ```bash
   node "E:/skills.te9.dev/app-tour-demo/scripts/build.mjs" \
     --assets docs/tours/<f>-tour/tour-assets.json --steps docs/tours/<f>-tour/steps.json \
     --out docs/tours/<f>-tour/<f>-tour.html --titel "<Feature> - app tour" \
     --badge "<Feature>" --eindtekst "<one-sentence flow summary>"
   node "E:/skills.te9.dev/app-tour-demo/scripts/verify.mjs" --demo docs/tours/<f>-tour/<f>-tour.html
   ```
   If verify reports click deviation: the anchor wasn't tagged in that frame.
   If console errors: inspect, usually a broken asset reference.

## CRITICAL safety rules

- **Never capture the user's real API key.** The running app reads
  `%APPDATA%/com.data-monster.app/settings.json` — for the settings and
  analyst tours FIRST back it up and swap in a dummy:
  ```bash
  cp "$APPDATA/com.data-monster.app/settings.json" /tmp/settings.bak
  node -e "require('fs').writeFileSync(process.env.APPDATA + '/com.data-monster.app/settings.json', JSON.stringify({llm: {provider: 'zai', apiKey: 'sk-DEMO-not-a-real-key', model: 'glm-4.6'}}, null, 2))"
  # after capture: cp /tmp/settings.bak "$APPDATA/com.data-monster.app/settings.json"
  ```
  The dummy key must be visibly fake. Restore immediately after capture.
- Never click a real delete/confirm on the user's data. The demo workspace is
  disposable, but the app-data settings are not.
- Screenshots of a native file picker are impossible — narrate those buttons
  (tour-only), never fake a click.

## Honest-beats-staged

If a feature has no live backend in this environment (e.g. analyst has no
configured LLM), capture the real empty/config state and narrate it — never
fake chat content or data.
