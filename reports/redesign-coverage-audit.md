# Full Redesign-Coverage Audit — data.monster vs Path B target

Systematic sweep of all 134 `.svelte` files (2026-09-24, branch `feature/bits-ui-redesign`).
Method: complete inventory → mechanical scan for hand-rolled interactive patterns
(window/keydown/pointer listeners, focus management, aria roles, hover logic, absolute
positioning, `<select>`) → per-file evidence extraction → classification.

Target: bits-ui owns interactive behavior; utility-first over the app token layer;
no `zinc-*`/hex outside chart palettes.

---

## Already migrated this branch (8 commits, PR #21)

| Surface | Primitive | Real consumers |
|---|---|---|
| RolePickerModal | Dialog + Combobox | SkeletonSetup (page editor) |
| Drawer (modal path) | Dialog | 10+ surfaces |
| Tabs | Tabs | TableOverview (/data) |
| Pagination | Pagination | PreviewPane |
| Accordion (root) | Accordion | ⚠️ see gap #6 — dead twin |
| Tooltip | Tooltip | /components demo |

Plus: PageGrid row min-height fix, context-menu Escape fix.

---

## Gaps found — real behavior still hand-rolled

### 1. `routes/pages/+page.svelte` — hand-rolled "New page" modal
Full backdrop + panel + manual Escape (`svelte:window` + `handleModalKeydown`),
zinc-900 styling — the exact pre-bits shape RolePickerModal had.
**→ MIGRATE to Dialog** (small; one consumer).

### 2. `TableDrawer.svelte` — nested hand-rolled modal inside the drawer
`drawer-modal-overlay` + `role="dialog"` + `handleModalKeydown` (lines ~351+), a
type-change confirm modal living inside the Drawer-based settings drawer.
**→ MIGRATE to Dialog** (nested Dialog in Drawer is a supported bits pattern).

### 3. `routes/connect/+page.svelte` — duplicate hand-rolled tab bar
`role="tablist"` + tab buttons (lines 218–233) instead of the shared (now bits-backed)
Tabs component. No keyboard nav.
**→ MIGRATE to shared Tabs.**

### 4. `routes/library/[id]/+page.svelte` — second duplicate tab bar
Inline `role="tablist"` (line 132) + 11 hardcoded colors.
**→ MIGRATE to shared Tabs + CONVERGE styling.**

### 5. `TagInput.svelte` — real component, hand-rolled suggestion dropdown
Consumers: TableDrawer + query route. Keydown combobox-ish behavior + suggestion
list + **hardcoded hex colors** (`#111827`, `#1d4ed8`, `#3b82f6`…).
**→ MIGRATE to Combobox (multiple) or at minimum CONVERGE colors.**

### 6. `/components` catalog renders the WRONG (pre-bits) twins
`ComponentDemo.svelte` imports `ds/Accordion`, `ds/Modal`, `ds/SearchAhead`,
`ds/Toast` — all still hand-rolled. My Accordion migration updated the root twin,
which is dead code. Tabs/Tooltip/Pagination imports are correct (no ds twins).
**→ Repoint ComponentDemo to the migrated root components (or migrate ds twins),
then delete dead duplicates** (root Modal/SearchAhead/Toast/Toggle/Toggles have
zero consumers).

---

## Deferred as planned (assessed, still valid)

- **`ExprEditor.svelte` autocomplete** — real hand-rolled listbox (`role="listbox"`,
  focus management, hover-highlight) embedded in a textarea; bits Combobox is
  input-based and doesn't fit an editor-embedded completion popup without
  rework. Also the worst styling offender: **31 zinc/hex hits**. Deferred per plan;
  revisit if a pattern emerges (Popover + customAnchor at caret is the likely path).

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

## Styling convergence debt (no behavior change needed)

`zinc-*`/hex counts per file (target: 0 outside chart palettes):

| File | Hits | | File | Hits |
|---|---|---|---|---|
| charts/ExprEditor | 31 | | pages/[slug] | 15 |
| ItemEditor | 25 | | SkeletonSetup | 14 |
| RelationshipEditor | 25 | | library/[id] | 11 |
| pages/+page | 17 | | ChartCard | 10 |
| TagInput | 6 hex | | agent, library/dev, internal-db | 2–4 |

Renderers carry 2 hex each — chart palette colors, likely legitimate.

Scoped `<style>` blocks remain in ~all showcase demos and BlockInspector/ChartCard —
converge opportunistically per component as each is touched (Path B policy).

Labs placeholders (30 routes), /ui showcase, settings pages: clean or trivial.

---

## Recommended order

1. Repoint ComponentDemo imports + delete dead root twins (#6 — prevents future confusion)
2. pages/+page modal → Dialog (#1)
3. TableDrawer nested modal → Dialog (#2)
4. connect + library/[id] → shared Tabs (#3, #4)
5. TagInput → Combobox multiple + token colors (#5)
6. ExprEditor autocomplete — stays deferred until a caret-anchored pattern is chosen
7. Styling convergence rides along with each touch (per Path B)


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
| Styling convergence | ♻️ rides along per touched component; remaining counts in table above |

Single source of truth now enforced: every reusable component lives exactly once in
`src/lib/components/`; design-system showcase demos live in `src/lib/demos/` (not
components); zero dead duplicates remain.
