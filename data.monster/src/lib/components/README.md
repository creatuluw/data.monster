# Components

Svelte components matching the **Glacier Design System v2.0** (Seed: DS2-GLACIER-7742).

## Source of Truth

All components are direct translations of the design system defined in:

```
.foundations/design-system/design-system.html
```

If the design system HTML shows a different style than a component here, **the HTML file is correct**. Update the component to match.

## Design Tokens

All styling uses CSS custom properties defined in `src/app.css` under `:root`. Never hardcode colors, spacing, typography, or motion values. Always use tokens:

- **Colors:** `var(--color-accent)`, `var(--color-surface)`, `var(--color-text)`, etc.
- **Typography:** `var(--text-xs)` through `var(--text-4xl)`, `var(--font-display)`, `var(--font-body)`, `var(--font-mono)`
- **Spacing:** `var(--space-1)` (4px) through `var(--space-24)` (96px) — 4pt scale
- **Radius:** `var(--radius-xs)` (2px) through `var(--radius-full)` (9999px)
- **Shadows:** `var(--shadow-sm)`, `var(--shadow-md)`, `var(--shadow-lg)`
- **Motion:** `var(--duration-fast)` (120ms), `var(--duration-base)` (250ms), `var(--duration-slow)` (450ms), `var(--ease-out-expo)`, `var(--ease-out-expo)`

## Components

| Component     | Design System Reference                                          | Props                                                          |
| ------------- | ---------------------------------------------------------------- | -------------------------------------------------------------- |
| `Button`      | `.btn`, `.btn-primary/secondary/ghost/danger`, `.btn-sm/lg`      | `variant`, `size`, `disabled`, `href`, `onclick`, `children`   |
| `Tag`         | `.tag`, `.tag-default/accent/success/warning/danger`             | `variant`, `children`                                          |
| `Input`       | `.field`, `.field-label`, `.field-hint`, `.input`                | `type`, `label`, `id`, `hint`, `required`, `disabled`, `value` |
| `Toggle`      | `.toggle`, `.toggle-label`                                       | `checked`, `label`, `id`                                       |
| `Card`        | `.card`, `.card-id`, `.card-title`, `.card-body`, `.card-footer` | `id`, `title`, `body`, `footer`, `children`                    |
| `Accordion`   | `.accordion`, `.accordion-trigger`, `.accordion-panel`           | `items` (array of `{ref, title, content}`)                     |
| `Modal`       | `.modal-backdrop`, `.modal`, `.modal-header/body/footer`         | `open`, `ref`, `title`, `body`, `footer`                       |
| `Searchahead` | `.searchahead`, dropdown, chips                                  | `placeholder`, `options`, `maxWidth`, `onselect`               |
| `Toast`       | `.toast`, `.toast-indicator`, `.toast-progress`                  | `variant`, `title`, `message`, `duration`                      |
| `Breadcrumb`  | `.breadcrumb`, `.breadcrumb-link`, `.breadcrumb-current`         | `items` (array of `{href, label}`), `ariaLabel`                |
| `Header`      | `.nav`, `.nav-brand`, `.nav-links`                               | `navItems`, `userEmail`, `onLogout`                            |
| `DatePicker`  | Calendar dropdown, portal rendering, size variants              | `value`, `placeholder`, `disabled`, `min`, `max`, `size`, `onchange`, `open` |
| `DateRangePicker` | Compound from/to date range picker                           | `fromDate`, `toDate`, `fromPlaceholder`, `toPlaceholder`, `onchange`, `open` |
| `Pagination`  | `.pagination`, `.pagination-btn`, `.pagination-active`           | `currentPage`, `totalItems`, `perPage`, `onchange`             |

## Building New Components

When adding a new component:

1. Check if it exists in `.foundations/design-system/design-system.html` first
2. Copy the **exact CSS** from the design system into a scoped `<style>` block
3. Use `$props()` runes with typed interfaces
4. Use `Snippet` for flexible content areas (title, body, footer, children)
5. Use `$bindable()` for two-way bound values (input values, toggles, modal open state)
6. Use only design tokens — no hardcoded values
7. Export from `index.ts`
