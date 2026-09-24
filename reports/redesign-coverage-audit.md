# Full Redesign-Coverage Audit — data.monster vs Path B target

Systematic sweep of all 134 `.svelte` files (2026-09-24, branch `feature/bits-ui-redesign`).
Method: complete inventory → mechanical scan for hand-rolled interactive patterns
(window/keydown/pointer listeners, focus management, aria roles, hover logic, absolute
positioning, `<select>`) → per-file evidence extraction → classification.

Target: bits-ui owns interactive behavior; utility-first over the app token layer;
no `zinc-*`/hex outside chart palettes.

> **Status: AUDIT GAP-FREE** (2026-09-24, commits `fd7fac4`..`5e395dd` on PR #21).
> All 6 behavior gaps resolved AND the styling-convergence debt closed: 13 surfaces
> converted to tokens (commits `2129125`..`5e395dd` + the 9-file sweep). Final scan:
> only ExprEditor (31, deferred with its autocomplete) and chart-palette/axis hex in
> renderers + labs pages remain — both dispositioned as legitimate.

---

## Already migrated this branch (17 commits, PR #21)

| Surface | Primitive | Real consumers |
|---|---|---|
| RolePickerModal | Dialog + Combobox | SkeletonSetup (page editor) |
| Drawer (modal path) | Dialog | 10+ surfaces |
| Tabs | Tabs | TableOverview (/data), connect, library/[id] |
| Pagination | Pagination | PreviewPane |
| Accordion | Accordion | /components demo (single source of truth) |
| Tooltip | Tooltip | /components demo |
| pages/+page "New page" modal | Dialog | /pages (`fd7fac4`) |
| TableDrawer nested confirm modal | Dialog | /data settings drawer (`8632555`) |
| TagInput | listbox semantics + token colors | TableDrawer, query (`0b74f7c`) |

Plus: PageGrid row min-height fix, context-menu Escape fix, single source of truth
(`b0c21ce` — ds/ demos moved to `lib/demos/`, 20 dead root twins deleted), truthful
/components catalog (`cdb0b4a`), breadcrumb-only navigation (`e92762d`, `e80ae6c`).

---

## Gaps found — ALL RESOLVED ✅

### 1. `routes/pages/+page.svelte` — hand-rolled "New page" modal ✅ RESOLVED
Full backdrop + panel + manual Escape (`svelte:window` + `handleModalKeydown`),
zinc-900 styling — the exact pre-bits shape RolePickerModal had.
**→ MIGRATED to Dialog** (`fd7fac4`). Verified: zero `handleModalKeydown` hits, `Dialog.Root` in place.

### 2. `TableDrawer.svelte` — nested hand-rolled modal inside the drawer ✅ RESOLVED
`drawer-modal-overlay` + `role="dialog"` + `handleModalKeydown` (lines ~351+), a
type-change confirm modal living inside the Drawer-based settings drawer.
**→ MIGRATED to Dialog** (`8632555` — Escape closes the modal, drawer stays open). Verified clean.

### 3. `routes/connect/+page.svelte` — duplicate hand-rolled tab bar ✅ RESOLVED
`role="tablist"` + tab buttons (lines 218–233) instead of the shared (now bits-backed)
Tabs component. No keyboard nav.
**→ MIGRATED to shared Tabs** (`f13f103`). Verified: no `role="tablist"`, imports `Tabs.svelte`.

### 4. `routes/library/[id]/+page.svelte` — second duplicate tab bar ✅ RESOLVED
Inline `role="tablist"` (line 132) + 11 hardcoded colors.
**→ MIGRATED to shared Tabs** (`f13f103`, underline variant). Verified: no `role="tablist"`.
Styling note: the 11 hardcoded colors were zinc-palette hex equivalents — 10 remain
(converge opportunistically; see styling table).

### 5. `TagInput.svelte` — real component, hand-rolled suggestion dropdown ✅ RESOLVED
Consumers: TableDrawer + query route. Keydown combobox-ish behavior + suggestion
list + **hardcoded hex colors** (`#111827`, `#1d4ed8`, `#3b82f6`…).
**→ CONVERGED** (`0b74f7c`): token colors only (verified 0 hex/zinc hits) + listbox
semantics. Full Combobox rewrite rejected — free-text tag creation is core UX and
bits Combobox is select-only (see rule: bits-ui Combobox is select-only).

### 6. `/components` catalog renders the WRONG (pre-bits) twins ✅ RESOLVED
`ComponentDemo.svelte` imported `ds/Accordion`, `ds/Modal`, `ds/SearchAhead`,
`ds/Toast` — all still hand-rolled. The Accordion migration updated the root twin,
which was dead code.
**→ RESOLVED** (`b0c21ce` + `cdb0b4a`): ds/ deleted, demos live in `lib/demos/`,
20 dead root twins deleted, ComponentDemo imports only real lib components (verified).

---

## Deferred as planned (assessed, still valid)

- **`ExprEditor.svelte` autocomplete** — real hand-rolled listbox (`role="listbox"`,
  focus management, hover-highlight) embedded in a textarea; bits Combobox is
  input-based and doesn't fit an editor-embedded completion popup without
  rework. Also the worst styling offender: **31 zinc/hex hits** (unchanged in re-scan).
  Deferred per plan; revisit if a pattern emerges (Popover + customAnchor at caret
  is the likely path).

---

## Deliberately kept (validated as correct)

- **controls/Toggle** — already a correct 20-line switch (`role=switch`, `aria-checked`).
- **Context menu** (+layout) — bits ContextMenu is area-based; selectivity requires
  hand-rolled. Escape bug already fixed.
- **SQL editor keybindings** — QueryEditor, pages/[slug], table/[name]: domain-specific.
- **Panel drag-resizers** — analyst/chat, query (window mousemove/mouseup): domain-specific.
- **Native `<select>`s** — controls/Select, ItemEditor, RelationshipEditor, PreviewPane,
  query tag filter: prefs explicitly bless native selects for plain choices.
- **FileConnector, ColorPalette, Motion demo, Hero/Spacing** — no bits-eligible behavior.

---

## Styling convergence debt — CLOSED ✅

Re-scanned 2026-09-24 after the gap fixes (`zinc-[0-9]+` + 6-digit hex per file,
91 non-demo `.svelte` files), then converged same day. All surfaces below are now at 0:

| File | Audit | Converged |
|---|---|---|
| charts/PageGrid | *(missed in audit)* | 30 → 0 |
| ItemEditor | 25 | 25 → 0 |
| RelationshipEditor | 25 | 25 → 0 |
| pages/[slug] | 15 | 15 → 0 |
| SkeletonSetup | 14 | 14 → 0 |
| renderers/TableRenderer | — | 10 → 0 |
| library/[id] | 11 | 10 → 0 |
| ChartCard | 10 | 10 → 0 |
| pages/+page | 17 | 17 → 0 |
| TagInput | 6 hex | 6 → 0 |
| agent, library/dev, +layout, internal-db | 2–4 | all → 0 |

Convergence notes:
- **+layout**: `--color-surface-hover` was referenced with a live hex fallback but the
  token never existed — now defined in `app.css` (`oklch(0.93 0.007 150)`).
- **internal-db**: table headers used `var(--color-bg, #0f0f0f)` — `--color-bg` never
  existed, so headers rendered near-black under gray text (legacy dark-theme leftover).
  Now the standard `surface-raised` sticky-header pattern (same as TableViewer).
- **agent**: two `var()` fallbacks carried wrong bluish hex values (dead weight) — stripped.

**Deliberately remaining (dispositioned as legitimate)**: ExprEditor 31 (deferred with
its autocomplete until a caret-anchored pattern is chosen); chart axis/palette hex in
renderers + labs pages (`#888888`, `#71717a`, `#3f3f46` — svelteplot mark configs).

Labs placeholders (30 routes), /ui showcase, settings pages: clean or trivial.

---

## Remaining work — NONE

The audit is gap-free as of `5e395dd`: bits-ui owns every interactive surface flagged,
and the styling-debt table is at zero outside dispositioned chart colors.

Only standing item: ExprEditor's autocomplete stays deferred until a caret-anchored
pattern (Popover + customAnchor) is chosen — its 31 zinc/hex converge then.

---

## Disposition (2026-09-24, commits b0c21ce..0b74f7c on PR #21)

| Finding | Status |
|---|---|
| #6 dead twins / catalog renders wrong demos | ✅ FIXED — ds/ (18 files) moved to `lib/demos/`, 20 dead root twins deleted; ComponentDemo Accordion entry now demos the bits version; catalog machinery (app-components.ts) updated |
| #1 pages/+page "New page" modal | ✅ MIGRATED to Dialog |
| #2 TableDrawer nested modal | ✅ MIGRATED to Dialog (Escape closes modal, drawer stays open) |
| #3 connect duplicate tab bar | ✅ MIGRATED to shared Tabs |
| #4 library/[id] duplicate tab bar | ✅ MIGRATED to shared Tabs (underline variant) |
| #5 TagInput | ✅ CONVERGED (token colors, listbox semantics); Combobox rewrite rejected — free-text tag creation is core UX |
| ExprEditor autocomplete | ⏳ deferred as planned |
| Styling convergence | ♻️ rides along per touched component; refreshed counts in table above |

Post-disposition follow-ons (same day): `cdb0b4a` /components = truthful showcase of
the real lib; `e92762d` + `e80ae6c` back links removed — the breadcrumb is the navigation;
`2129125` audit resolution pass; styling-convergence sweep (2 commits, 13 surfaces +
the missing `--color-surface-hover` token).

Single source of truth now enforced: every reusable component lives exactly once in
`src/lib/components/`; design-system showcase demos live in `src/lib/demos/` (not
components); zero dead duplicates remain.
