# Redesign Analysis — Impeccable + Bits UI for data.monster

Analysis of the current app against the target stack in `E:\stack-preferences-smb.md` §9
("Bits UI owns behavior, Impeccable owns looks — your components are thin styled wrappers
around Bits UI parts").

---

## 1. Stack delta — target vs. today

| Target (prefs §9) | data.monster today | Verdict |
|---|---|---|
| Svelte 5 (bits-ui requirement) | Svelte 5.55, runes everywhere | ✅ ready |
| Bits UI headless primitives | **not installed** — every interactive primitive hand-rolled | ❌ the gap |
| Impeccable — design tokens in CSS | Tailwind 4 `@theme` tokens in `app.css` (629 lines, ledger-green ramps, well-structured) | ⚠️ tokens exist; workflow not adopted |
| Utility-first styling | **Hybrid**: Tailwind imported but ~95% of styling is scoped `<style>` blocks; utilities only sprinkled (175 class hits / 38 files) | ⚠️ decision point (§3) |
| lucide-svelte icons | `lucide-svelte` installed | ✅ already conformant |
| shadcn-svelte / Flowbite / Skeleton banned | none installed | ✅ |

**Key finding**: the app already has the hard part Impeccable asks for — a disciplined
token layer in `app.css`. What it lacks is Bits UI underneath the interactive components.

---

## 2. Candidate components to redesign (ranked by value)

Behavior debt = hand-rolled keyboard / focus / ARIA logic that Bits UI ships for free.

| # | Component | Today | Bits UI replacement | Notes |
|---|---|---|---|---|
| 1 | **Drawer primitive** (`Drawer.svelte`, 249 ln, 10+ consumers) | Manual Escape key, hand-rolled focus trap + return, backdrop, resize action | `Dialog.Root/Portal/Overlay/Content` (side variant) | Just got its chrome redesign (2026-09-23) — keep chrome + `drawerResize`, swap only the behavior layer. All 10 consumers inherit a11y for free. |
| 2 | **RolePickerModal** (`charts/RolePickerModal.svelte`) | The app's *real* modal (Modal.svelte is a /ui demo with 0 consumers) — hand-rolled pick/create with inline searchahead | `Dialog` + `Combobox` | Highest-value single migration: exercises both layers at once. |
| 3 | **SearchAhead / inline comboboxes** (`SearchAhead.svelte` 366 ln + hand-rolled search in RolePickerModal, ExprEditor autocomplete) | Duplicated combobox logic per surface; wiki learning: "SearchAhead is a /ui showcase demo, not prop-driven" | `Combobox` | Bits UI finally lets us build the one real prop-driven combobox the app never had. |
| 4 | **Tabs** (`Tabs.svelte`, 298 ln, 2 aria attrs) | Full hand-rolled tablist | `Tabs.Root/List/Trigger/Content` | Mechanical. |
| 5 | **Tooltip** (`Tooltip.svelte`, 105 ln, **0 aria attributes**) | Position + show/hide by hand | `Tooltip` | Worst a11y offender in the codebase. |
| 6 | **Tab context menu** (`routes/+layout.svelte` right-click on virtual tabs) | Inline hand-rolled | `ContextMenu` | Small, isolated. |
| 7 | **Toast** (`Toast.svelte`, 288 ln) | Custom queue + timers | `Toast` (bits-ui v2) | Optional — current one works; migrate opportunistically. |
| 8 | **Accordion** (209 ln) | Hand-rolled expand/collapse | `Accordion` | Mechanical. |
| 9 | **Pagination** (172 ln) | Hand-rolled | `Pagination` | Mechanical. |
| 10 | **Toggle** (`Toggle.svelte` + `controls/Toggle.svelte`) | Hand-rolled switch | `Switch` | Mechanical; keep controls-kit API. |
| 11 | **ExprEditor autocomplete dropdown** | Absolute-positioned suggestion list | `Combobox`/`Popover` positioning | Careful: editor-integrated, not a plain combobox. |
| 12 | **7 native `<select>` uses + `controls/Select.svelte`** (28 ln) | Native select | `Select` **only where search is needed** | Prefs explicitly bless native elements for plain inputs — don't gold-plate. |

Not candidates: chart components (SveltePlot), controls-kit form fields (Field, Section,
DangerZone — pure layout), Button/inputs (native elements + CSS per prefs).

---

## 3. Decision point: what "adopting Impeccable" means here

The prefs model styles Bits UI parts via `class` props + data-attribute rules in
`app.css`. Today's app styles via scoped component CSS + those same tokens.

- **Path A — behavior migration only (recommended, lazy)**: install `bits-ui`; adopt the
  Impeccable *workflow* (`/impeccable craft` for redesigns, `/impeccable extract`,
  `/impeccable document`); style the new Bits UI wrappers with the existing token layer
  (scoped CSS or data-attribute rules in `app.css`). bits-ui is CSS-agnostic — nothing
  forces a utility rewrite. Tailwind 4 stays as the utility engine it already is.
- **Path B — full §9 conformance**: additionally converge all 38 components' scoped CSS
  to utilities. Big-bang across every surface; only worth doing per-component as each
  gets redesigned anyway, never as a standalone sweep.

---

## 4. Constraints the redesign must respect (from the wiki)

- Drawer chrome decisions of 2026-09-23: title-only header (no kickers), content renders
  directly in the primitive's padded body (no inner wrapper divs), one scrolling column,
  Danger zone last, `drawerResize` action stays.
- "Adding a component never auto-opens the config drawer."
- Form controls keep coming from the shared controls kit — Bits UI goes *under* the kit,
  not around it.
- `/components` catalog + `/ui` showcase demos must keep rendering the **real**
  components (registry prefers ds/ twins) — after each migration the demo entry updates
  with it.
- All pages capped at 1920px; grid-gap owns card spacing.

---

## 5. Proposed next steps

1. **Install**: `npm i bits-ui`; feed `bits-ui.com/llms.txt` to the agent (per prefs §9.2).
2. **Pilot — RolePickerModal on Dialog + Combobox** (candidate #2+#3): proves the
   wrapper pattern, the token styling approach, and the /components demo update in one
   small surface.
3. **Drawer internals → `Dialog.Root`** (candidate #1): keep 2026-09-23 chrome +
   `drawerResize`; 10+ consumers gain focus trap, Escape, and ARIA for free.
4. **Mechanical sweep**: Tabs, Tooltip, Accordion, Pagination, Switch, Context Menu —
   small diffs, one component per PR-style pass.
5. **Record the system**: `/impeccable document` after the pilot; update the
   `/components` catalog entries as each wrapper lands.

Rough effort: pilot ~1 session; drawer ~1 session; mechanical sweep ~1 session for 4–5
components. Toast and ExprEditor deferred until touched for other reasons.
