# Dropdown Menu Components

A set of Tailwind CSS-styled dropdown menu components for Svelte 5, inspired by Bits UI.

## Components

### DropdownMenu
The main container component that manages the dropdown state and positioning.

**Props:**
- `trigger?: Snippet` - Custom trigger button content
- `children?: Snippet` - Menu items content
- `align?: 'left' | 'right'` - Alignment of the dropdown (default: 'left')
- `sideOffset?: number` - Spacing between trigger and content in pixels (default: 8)

### DropdownMenuItem
Individual menu item button.

**Props:**
- `onclick?: () => void` - Click handler
- `disabled?: boolean` - Disabled state (default: false)
- `icon?: Snippet` - Icon to display before the label
- `children?: Snippet` - Main content of the menu item
- `shortcut?: string` - Keyboard shortcut to display (e.g., "⌘+R" or "Ctrl+S")
- `variant?: 'default' | 'danger'` - Visual style variant (default: 'default')

### DropdownMenuSeparator
Visual separator line between menu items.

## Usage Example

```svelte
<script lang="ts">
  import DropdownMenu from '$lib/components/DropdownMenu.svelte';
  import DropdownMenuItem from '$lib/components/DropdownMenuItem.svelte';
  import DropdownMenuSeparator from '$lib/components/DropdownMenuSeparator.svelte';
  import { RefreshCw, Layout, Plus, ChevronDown } from 'lucide-svelte';

  let isLoading = $state(false);

  function handleRefresh() {
    console.log('Refresh clicked');
  }

  function handleLayout() {
    console.log('Layout clicked');
  }

  function handleCreate() {
    console.log('Create clicked');
  }
</script>

<DropdownMenu align="left">
  {#snippet trigger()}
    <span class="flex items-center gap-2">
      Actions
      <ChevronDown class="w-4 h-4" />
    </span>
  {/snippet}
  
  {#snippet children()}
    <DropdownMenuItem 
      onclick={handleRefresh} 
      disabled={isLoading}
      shortcut="⌘+R"
    >
      {#snippet icon()}
        <RefreshCw class="w-4 h-4" />
      {/snippet}
      Refresh
    </DropdownMenuItem>
    
    <DropdownMenuItem 
      onclick={handleLayout} 
      shortcut="⌘+L"
    >
      {#snippet icon()}
        <Layout class="w-4 h-4" />
      {/snippet}
      Apply Layout
    </DropdownMenuItem>
    
    <DropdownMenuSeparator />
    
    <DropdownMenuItem 
      onclick={handleCreate}
    >
      {#snippet icon()}
        <Plus class="w-4 h-4" />
      {/snippet}
      Create New
    </DropdownMenuItem>
    
    <DropdownMenuSeparator />
    
    <DropdownMenuItem 
      onclick={() => console.log('Delete')}
      variant="danger"
    >
      Delete
    </DropdownMenuItem>
  {/snippet}
</DropdownMenu>
```

## Features

- ✅ **Svelte 5 Runes** - Uses `$state` and modern Svelte 5 syntax
- ✅ **Tailwind CSS** - Fully styled with Tailwind utility classes
- ✅ **Dark Mode** - Automatic dark mode support
- ✅ **Keyboard Support** - ESC key to close
- ✅ **Click Outside** - Closes when clicking outside the menu
- ✅ **Accessibility** - ARIA attributes for screen readers
- ✅ **Keyboard Shortcuts** - Display keyboard shortcuts in menu items
- ✅ **Icons** - Support for icon snippets
- ✅ **Variants** - Default and danger styles
- ✅ **Alignment** - Left or right alignment options

## Styling

The components use Tailwind CSS with dark mode support. The color scheme follows:

- Background: `bg-white dark:bg-slate-800`
- Border: `border-slate-200 dark:border-slate-700`
- Text: `text-slate-700 dark:text-slate-300`
- Hover: `hover:bg-slate-50 dark:hover:bg-slate-700`
- Danger: `text-red-600 dark:text-red-400`

## Browser Compatibility

Works in all modern browsers that support Svelte 5.

