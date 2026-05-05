# SqlEditor Component

A code editor component specifically designed for SQL queries with syntax highlighting and line numbers.

## Features

- **Syntax Highlighting**: Keywords, strings, numbers, and comments are highlighted
- **Line Numbers**: Automatic line numbering on the left side
- **Tab Support**: Press Tab to insert 2 spaces (customizable)
- **Dark Mode**: Automatically adapts to dark/light theme
- **Monospace Font**: Uses mono font for better code readability
- **Bindable Value**: Two-way binding support with `bind:value`

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `value` | `string` | `''` | The SQL query text (bindable) |
| `placeholder` | `string` | `'SELECT * FROM ...'` | Placeholder text when empty |
| `disabled` | `boolean` | `false` | Disables the editor |
| `minHeight` | `string` | `'200px'` | Minimum height of the editor |
| `label` | `string` | `''` | Optional label above the editor |
| `helpText` | `string` | `''` | Optional help text below the editor |

## Usage

### Basic Usage

```svelte
<script lang="ts">
  import SqlEditor from '$lib/components/SqlEditor.svelte';
  
  let sqlQuery = $state('SELECT * FROM users');
</script>

<SqlEditor bind:value={sqlQuery} />
```

### With Label and Help Text

```svelte
<SqlEditor
  bind:value={sqlQuery}
  label="SQL Query"
  helpText="Write your SQL query here"
  minHeight="300px"
/>
```

### Disabled State

```svelte
<SqlEditor
  bind:value={sqlQuery}
  disabled={true}
/>
```

## Syntax Highlighting

The component highlights:
- **SQL Keywords**: SELECT, FROM, WHERE, JOIN, etc. (blue)
- **Strings**: 'text' (green)
- **Numbers**: 123 (red)
- **Comments**: -- comment (gray, italic)

## Keyboard Shortcuts

- **Tab**: Insert 2 spaces at cursor position
- Standard text editing shortcuts (copy, paste, undo, etc.)

## Styling

The component automatically adapts to your app's dark/light theme. Colors are defined using Tailwind CSS classes and can be customized by modifying the component's `<style>` section.

