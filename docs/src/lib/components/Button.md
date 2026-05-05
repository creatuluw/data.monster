# Button Component

A flexible, reusable button component for Svelte 5 with support for both button and anchor elements, multiple variants, sizes, and full dark mode support.

## Features

- **Multiple Variants**: Primary, Secondary, Ghost, and Danger styles
- **Flexible Sizing**: Small, Medium, and Large sizes
- **Smart Element Rendering**: Automatically renders as `<a>` when `href` is provided, otherwise renders as `<button>`
- **Dark Mode Support**: Built-in dark mode styling that adapts to your theme
- **TypeScript Support**: Full type safety with proper prop typing
- **Accessibility**: Focus states and proper disabled states
- **Customizable**: Extend with custom classes via the `class` prop

## Installation

The Button component is located at `src/lib/components/Button.svelte`. Import it in your Svelte files:

```svelte
import Button from '$lib/components/Button.svelte';
```

## Basic Usage

### Default Button (Primary)

```svelte
<Button>Click Me</Button>
```

### Button with Click Handler

```svelte
<Button onclick={() => console.log('Clicked!')}>
  Click Me
</Button>
```

### Link Button

```svelte
<Button href="/about">Go to About</Button>
```

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `variant` | `'primary' \| 'secondary' \| 'ghost' \| 'danger' \| 'accent'` | `'primary'` | Visual style variant of the button |
| `size` | `'sm' \| 'md' \| 'lg'` | `'md'` | Size of the button |
| `fullWidth` | `boolean` | `false` | Whether button should take full width of container |
| `href` | `string` | `undefined` | If provided, renders as anchor tag instead of button |
| `class` | `string` | `''` | Additional custom CSS classes |
| `disabled` | `boolean` | `false` | Disables the button (button element only) |
| `...restProps` | `HTMLButtonAttributes \| HTMLAnchorAttributes` | - | All other native button/anchor attributes |

## Variants

### Primary (Default)
Green background with white text. Ideal for primary actions.

```svelte
<Button variant="primary">Primary Action</Button>
```

**Styling:**
- Background: Green-600 (hover: Green-700, active: Green-800)
- Text: White
- Focus ring: Green-500
- Dark mode: Same green colors

### Secondary
Light green background with dark green text. Good for secondary actions.

```svelte
<Button variant="secondary">Secondary Action</Button>
```

**Styling:**
- Light mode: Green-100 background with green-800 text
- Dark mode: Green-800 background with green-100 text
- Focus ring: Green-500

### Ghost
Bordered button with transparent background. Great for tertiary actions.

```svelte
<Button variant="ghost">Ghost Button</Button>
```

**Styling:**
- Border: Slate-200 (hover: Slate-300)
- Dark mode border: Slate-700 (hover: Slate-600)
- No background, text-only with border
- Focus ring: Slate-500

### Danger
Red background for destructive actions.

```svelte
<Button variant="danger">Delete</Button>
```

**Styling:**
- Background: Red-600 (hover: Red-700, active: Red-800)
- Text: White
- Focus ring: Red-500

### Accent
Purple background for special emphasis actions.

```svelte
<Button variant="accent">Special Action</Button>
```

**Styling:**
- Background: Purple-600 (hover: Purple-700, active: Purple-800)
- Text: White
- Focus ring: Purple-500
- Dark mode: Same purple colors

## Sizes

### Small (`sm`)
Compact size for tight spaces.

```svelte
<Button size="sm">Small Button</Button>
```

**Spacing:** `px-3 py-1` with `text-sm`

### Medium (`md`) - Default
Standard size for most use cases.

```svelte
<Button size="md">Medium Button</Button>
```

**Spacing:** `px-4 py-2` with `text-base`

### Large (`lg`)
Larger size for prominent actions.

```svelte
<Button size="lg">Large Button</Button>
```

**Spacing:** `px-6 py-3` with `text-lg`

## Examples

### Button Group

```svelte
<div class="flex gap-3">
  <Button variant="primary">Save</Button>
  <Button variant="secondary">Cancel</Button>
  <Button variant="ghost">Reset</Button>
</div>
```

### Full Width Button

```svelte
<Button fullWidth variant="primary">
  Sign Up Now
</Button>
```

### Disabled Button

```svelte
<Button disabled>
  Disabled Button
</Button>
```

**Note:** Disabled state only works with button elements (not links). Disabled buttons show reduced opacity and are not clickable.

### Button as Link

```svelte
<Button href="/subscribe" variant="primary" size="sm">
  Subscribe
</Button>
```

### Button with Icon

```svelte
<script>
  import { ArrowRight } from 'lucide-svelte';
</script>

<Button>
  <span>Continue</span>
  <ArrowRight class="w-4 h-4 ml-2" />
</Button>
```

### Loading Button

```svelte
<script>
  let isLoading = $state(false);
  
  async function handleSubmit() {
    isLoading = true;
    // ... async operation
    isLoading = false;
  }
</script>

<Button onclick={handleSubmit} disabled={isLoading}>
  {#if isLoading}
    <svg class="animate-spin h-5 w-5 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
    </svg>
    Processing...
  {:else}
    Submit
  {/if}
</Button>
```

### Custom Styling

```svelte
<Button class="shadow-lg hover:shadow-xl">
  Custom Styled Button
</Button>
```

## Design Reference

This component is based on the button patterns from the DevSpace HTML template, featuring:

- Consistent padding and spacing
- Smooth hover transitions (150ms ease-in-out)
- Proper focus ring states for accessibility
- Dark mode support with appropriate contrast
- Disabled state handling with visual feedback

## Styling Classes

The component uses Tailwind CSS classes. Key classes include:

**Base:**
- `inline-flex items-center justify-center`
- `font-medium`
- `transition duration-150 ease-in-out`
- `rounded-md`

**Variants:** See individual variant sections above

**States:**
- **Hover:** Color transitions on background/border
- **Focus:** 2px focus ring with offset
- **Disabled:** Reduced opacity (60%), gray background, no pointer events

## Accessibility

- Uses semantic `<button>` or `<a>` elements
- Includes focus ring for keyboard navigation
- Disabled state properly prevents interaction
- All native button/link attributes are supported

## TypeScript Support

The component includes full TypeScript types:

```typescript
type ButtonProps = {
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger' | 'accent';
  size?: 'sm' | 'md' | 'lg';
  fullWidth?: boolean;
  href?: never;
  children: Snippet;
} & HTMLButtonAttributes;

type LinkProps = {
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger' | 'accent';
  size?: 'sm' | 'md' | 'lg';
  fullWidth?: boolean;
  href: string;
  children: Snippet;
} & HTMLAnchorAttributes;
```

## Best Practices

1. **Primary Actions**: Use `variant="primary"` (green) for the main action on a page
2. **Secondary Actions**: Use `variant="secondary"` (light green) for alternative actions
3. **Tertiary Actions**: Use `variant="ghost"` for less prominent actions
4. **Destructive Actions**: Use `variant="danger"` (red) for delete/remove actions
5. **Special Emphasis**: Use `variant="accent"` (purple) for unique or promotional CTAs
6. **Links**: Always use `href` prop when navigation is the primary purpose
7. **Loading States**: Disable buttons during async operations
8. **Full Width**: Use `fullWidth` in modals or mobile layouts

## Related Components

- **Card**: Use buttons inside cards for actions
- **SystemMessagesDrawer**: Contains buttons for dismissing messages

## Browser Support

This component works in all modern browsers that support:
- CSS Grid and Flexbox
- CSS Custom Properties (for dark mode)
- ES6+ JavaScript (for Svelte 5)

## Notes

- When using `href`, the component automatically renders as an anchor element
- Dark mode classes are automatically applied based on the `dark:` class on the HTML element
- The component spreads remaining props, so you can add any valid HTML attributes
- Custom classes via the `class` prop are appended to the default classes

