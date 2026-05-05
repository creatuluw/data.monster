# Combobox Component

## Purpose
A searchable dropdown component with keyboard navigation, filtering, and flexible display options. Perfect for selecting from large lists of options.

## Features
- Type-ahead search
- Keyboard navigation (arrows, enter, escape)
- Custom display and search fields
- Generic type support
- Auto-generated IDs
- Disabled state
- Required field support
- No results message

## Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `options` | `T[]` | `[]` | Array of options to select from |
| `value` | `T \| null` | `null` | Currently selected value |
| `displayField` | `keyof T \| (item: T) => string` | String conversion | How to display options |
| `searchField` | `keyof T \| (item: T) => string` | String conversion | Which field to search |
| `placeholder` | `string` | `'Select an option...'` | Input placeholder |
| `label` | `string` | `''` | Label text |
| `disabled` | `boolean` | `false` | Disabled state |
| `required` | `boolean` | `false` | Required field |
| `className` | `string` | `''` | Additional classes |

## Events

| Event | Payload | Description |
|-------|---------|-------------|
| `select` | `T` | Fired when option is selected |
| `change` | `T \| null` | Fired when selection changes (including clear) |

## Usage Examples

### Basic String Array
```svelte
<script>
  import { Combobox } from '$lib/components';
  
  const fruits = ['Apple', 'Banana', 'Cherry', 'Date', 'Elderberry'];
  let selected = $state(null);
</script>

<Combobox 
  options={fruits}
  bind:value={selected}
  placeholder="Select a fruit..."
  label="Favorite Fruit"
/>
```

### Object Array with Custom Display
```svelte
<script>
  interface User {
    id: string;
    name: string;
    email: string;
  }
  
  const users: User[] = [
    { id: '1', name: 'John Doe', email: 'john@example.com' },
    { id: '2', name: 'Jane Smith', email: 'jane@example.com' },
    { id: '3', name: 'Bob Johnson', email: 'bob@example.com' }
  ];
  
  let selectedUser = $state<User | null>(null);
</script>

<Combobox 
  options={users}
  bind:value={selectedUser}
  displayField={(user) => `${user.name} (${user.email})`}
  searchField={(user) => `${user.name} ${user.email}`}
  label="Select User"
  placeholder="Search users..."
  required
/>

{#if selectedUser}
  <p>Selected: {selectedUser.name}</p>
{/if}
```

### With Event Handlers
```svelte
<script>
  import { showSuccess } from '$lib/stores/toast';
  
  let selected = $state(null);
  
  function handleSelect(event) {
    const item = event.detail;
    showSuccess(`Selected: ${item}`);
  }
  
  function handleChange(event) {
    const item = event.detail;
    console.log('Selection changed:', item);
  }
</script>

<Combobox 
  options={items}
  bind:value={selected}
  on:select={handleSelect}
  on:change={handleChange}
/>
```

### LVS Users Example
```svelte
<script>
  import { Combobox } from '$lib/components';
  import type { LvsUser } from '$lib/services/directus';
  
  let students: LvsUser[] = $state([]);
  let selectedStudent = $state<LvsUser | null>(null);
  
  function handleStudentSelect(event: CustomEvent<LvsUser>) {
    selectedStudent = event.detail;
    console.log('Selected student:', selectedStudent);
  }
</script>

<Combobox
  options={students}
  bind:value={selectedStudent}
  on:select={handleStudentSelect}
  displayField={(student) => `${student.first_name} ${student.last_name}${student.leerling ? ` (${student.leerling})` : ''}`}
  searchField={(student) => `${student.first_name} ${student.last_name} ${student.leerling || ''} ${student.eid || ''}`}
  placeholder="Zoek en selecteer een student..."
  label="Student"
  required
/>
```

### Disabled State
```svelte
<Combobox 
  options={countries}
  value={selectedCountry}
  disabled
  label="Country"
  placeholder="Selection disabled"
/>
```

### With Custom Styling
```svelte
<Combobox 
  options={options}
  bind:value={selected}
  className="max-w-md"
  label="Choose Option"
/>
```

### Form Integration
```svelte
<script>
  interface Category {
    id: string;
    name: string;
  }
  
  let categories: Category[] = [...];
  let formData = $state({
    title: '',
    categoryId: null
  });
</script>

<form onsubmit|preventDefault={handleSubmit}>
  <Input 
    label="Title" 
    bind:value={formData.title}
    required 
  />
  
  <Combobox 
    options={categories}
    bind:value={formData.categoryId}
    displayField="name"
    searchField="name"
    label="Category"
    required
  />
  
  <Button type="submit">Submit</Button>
</form>
```

## Keyboard Navigation

| Key | Action |
|-----|--------|
| `↓` | Move to next option |
| `↑` | Move to previous option |
| `Enter` | Select highlighted option |
| `Escape` | Close dropdown |
| `Space` | Open dropdown (when closed) |
| Type | Filter options |

## Features in Detail

### Search Functionality
- Real-time filtering as you type
- Case-insensitive search
- Searches across searchField
- "No results" message when empty

### Dropdown Behavior
- Opens on focus
- Opens on click
- Opens when typing
- Closes on blur (with delay for selection)
- Closes on escape key
- Closes after selection

### Visual Feedback
- Highlighted option on hover/keyboard navigation
- Selected option indicator
- Dropdown arrow rotates when open
- Focus states
- Indigo accent colors

## Accessibility
- Proper ARIA labels
- Keyboard navigation
- Focus management
- Screen reader support
- Required field indication

## Styling
- Indigo focus colors
- Smooth transitions
- Max height with scroll
- Rounded corners
- Shadow on dropdown
- Dark mode support

## Best Practices
1. Provide clear labels
2. Use appropriate displayField for readability
3. Include searchable fields in searchField
4. Handle null/empty states
5. Provide placeholder text
6. Use required when necessary
7. Keep option lists reasonable in size

