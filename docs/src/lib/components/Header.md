# Header Component

## Overview
The `Header` component is a fixed navigation bar that appears at the top of all pages in the application. It provides consistent navigation and action buttons across the entire app.

## Features
- **Fixed positioning** - Stays at top of viewport while scrolling
- **Semi-transparent backdrop** - Uses backdrop blur for modern glass effect
- **Responsive layout** - Adapts to different screen sizes
- **Message notifications** - Displays count badge for system messages

## Props

### `onMenuClick: () => void`
Callback function triggered when the menu button is clicked. Typically opens the sidebar.

### `onUploadClick: () => void`
Callback function triggered when the upload button is clicked. Opens the file upload modal.

### `onMessagesClick: () => void`
Callback function triggered when the messages button is clicked. Opens the system messages drawer.

### `messageCount?: number` (optional)
Number of unread system messages. Displays a badge when greater than 0. Defaults to 0.

## Navigation Buttons

### Left Side
1. **Menu** - Opens sidebar for navigation
2. **Home** - Links to home page (/)
3. **Upload** - Quick access to file upload
4. **Bookmarks** - Links to bookmarks page (/bookmarks)

### Right Side
1. **Messages** - Opens system messages drawer with notification badge

## Usage

```svelte
<script lang="ts">
  import Header from '$lib/components/Header.svelte';
  
  let messageCount = $state(0);
  
  function handleMenuClick() {
    // Open sidebar
  }
  
  function handleUploadClick() {
    // Open upload modal
  }
  
  function handleMessagesClick() {
    // Open messages drawer
  }
</script>

<Header 
  onMenuClick={handleMenuClick}
  onUploadClick={handleUploadClick}
  onMessagesClick={handleMessagesClick}
  messageCount={messageCount}
/>
```

## Styling

### Background
- Base: `bg-[#0A0E27]/80` - Dark blue with 80% opacity
- Backdrop blur for glass effect
- Bottom border: `border-white/5`

### Layout
- Fixed height: ~60px (3rem padding + button height)
- Full width with padding: `px-4 py-3`
- Flexbox layout: `justify-between` for left/right alignment

### Buttons
- Ghost variant with shadow
- Icon size: `w-4.5 h-4.5`
- Padding: `px-3.5 py-2`

### Notification Badge
- Circular badge: `h-5 w-5`
- Blue background: `bg-blue-600`
- Position: Absolute top-right of messages button
- Shows "9+" for counts > 9

## Integration with Layout

The Header component is integrated in the root layout (`+layout.svelte`):

```svelte
<Header 
  onMenuClick={() => isSidebarOpen = true}
  onUploadClick={handleUploadClick}
  onMessagesClick={() => isMessagesDrawerOpen = true}
  messageCount={systemMessages.length}
/>

<main class="flex-1 pt-[60px] h-screen">
  {@render children()}
</main>
```

The main content area has `pt-[60px]` padding to prevent overlap with the fixed header.

## Page Layout Adjustments

Pages that use full-height layouts should use `h-full` instead of `h-screen` since they're now inside the main container:

```svelte
<!-- Before -->
<div class="flex h-screen">...</div>

<!-- After -->
<div class="flex h-full">...</div>
```

## Accessibility

- All buttons have descriptive `title` attributes
- Semantic `<header>` element
- Interactive elements are keyboard accessible
- Visual notification badge for message count

## Dependencies

- `lucide-svelte` - Icons (Menu, Home, FilePlus2, MessageCircleWarning, Star)
- `Button.svelte` - Button component

## Z-Index

The header has `z-40` to ensure it appears above most page content but below modals/drawers (typically z-50).

