# Library Component Templates

This directory contains the library component templates that ship with warphead.dev. These UI components are automatically loaded into DuckDB when the application initializes.

## 📁 Files

- **`library-components.json`** - JSON definition of all library component templates that ship with the app

## 🔧 How It Works

1. **JSON Definition**: All library component templates are defined in `library-components.json`
2. **Runtime Loading (Development)**: When developing, the backend reads the JSON file from disk on each app init
3. **Embedded Fallback (Production)**: In production builds, the JSON is embedded using `include_str!()` macro
4. **Auto-loaded on Init**: When DuckDB initializes, the backend:
   - Parses the JSON file (from disk if available, otherwise from embedded version)
   - Creates the `_warphead_component_templates` table if it doesn't exist
   - Inserts/updates each component template using `ON CONFLICT DO UPDATE`
   - Reports how many components were created vs updated

## 📝 JSON Schema

Each component in the JSON file has the following structure:

```json
{
  "component_name": "Filter Dropdown",
  "component_type": "FilterDropdown",
  "description": "Interactive dropdown filter with Alpine.js",
  "tags": "interactive,filter,alpine,form",
  "frameworks": "tailwind,alpinejs",
  "html_code": "<div x-data=\"{ open: false }\">...</div>",
  "css_code": "",
  "js_code": "",
  "min_metrics": 0,
  "max_metrics": 0,
  "min_dimensions": 0,
  "max_dimensions": 1
}
```

### Required Fields
- `component_name` - Display name for the component template
- `component_type` - Unique component identifier (e.g., `FilterDropdown`, `DataBadge`)
- `html_code` - Complete HTML code for the component
- `description` - Human-readable description of what the component does
- `tags` - Comma-separated tags for categorization

### Optional Fields
- `css_code` - Custom CSS styles (empty string if using only Tailwind)
- `js_code` - Custom JavaScript code (empty string if using only Alpine.js)
- `frameworks` - Comma-separated list of frameworks (e.g., `"tailwind,alpinejs"`)
- `config_schema` - JSON schema for component configuration options
- `metrics` - Comma-separated list of metric slugs this component uses
- `dimensions` - Comma-separated list of dimension slugs this component uses
- `sample_data` - Sample data for preview/testing
- `min_metrics`, `max_metrics` - Metric requirements (0 for no metrics)
- `min_dimensions`, `max_dimensions` - Dimension requirements (0 for no dimensions)

## 🎨 Available Components

### 1. Filter Dropdown
Interactive dropdown filter with Alpine.js for filtering data.
- **Tags**: interactive, filter, alpine, form
- **Frameworks**: Tailwind CSS, Alpine.js
- **Data Requirements**: 0-1 dimensions

### 2. Data Badge
Small metric badge for displaying key data points on dashboards.
- **Tags**: basic, badge, metric, kpi
- **Frameworks**: Tailwind CSS
- **Data Requirements**: 1 metric

### 3. Toggle Switch
Interactive toggle switch component with Alpine.js.
- **Tags**: interactive, toggle, alpine, form, control
- **Frameworks**: Tailwind CSS, Alpine.js
- **Data Requirements**: None

### 4. Metric Grid
Multi-metric grid layout for displaying 3 or more metrics in a dashboard view.
- **Tags**: advanced, grid, metrics, kpi, dashboard
- **Frameworks**: Tailwind CSS
- **Data Requirements**: 3-12 metrics

### 5. Status Indicator
Status badge with color coding for displaying item states.
- **Tags**: basic, status, badge, indicator
- **Frameworks**: Tailwind CSS
- **Data Requirements**: 1 dimension

### 6. Accordion
Collapsible accordion component with Alpine.js for organizing content in sections.
- **Tags**: interactive, accordion, collapse, alpine, layout
- **Frameworks**: Tailwind CSS, Alpine.js
- **Data Requirements**: None

### 7. Tabs
Tabbed interface component with Alpine.js for organizing content.
- **Tags**: interactive, tabs, alpine, navigation, layout
- **Frameworks**: Tailwind CSS, Alpine.js
- **Data Requirements**: None

### 8. Grid Container
Responsive grid layout container for organizing multiple components.
- **Tags**: layout, grid, container, responsive
- **Frameworks**: Tailwind CSS
- **Data Requirements**: None

### 9. Button Group
Group of related action buttons with various styles.
- **Tags**: basic, buttons, actions, group
- **Frameworks**: Tailwind CSS
- **Data Requirements**: None

### 10. Text Block
Rich text content display with formatting support.
- **Tags**: basic, text, content, typography
- **Frameworks**: Tailwind CSS
- **Data Requirements**: None

## 🚀 Usage

### Viewing Library Components

1. Start the application
2. Navigate to **Analysis > Components**
3. Click "New Component" or edit an existing component
4. Click **"Browse Templates"** in the ComponentEditor
5. Select a template to load it into the editor

### Editing Library Components

1. Open `src/lib/ui/library-components.json`
2. Modify existing component or add new one
3. Restart the application
4. Changes appear in the template browser

### Adding Custom Components

Add a new entry to the `components` array:

```json
{
  "component_name": "My Custom Component",
  "component_type": "MyCustomComponent",
  "description": "Custom UI component",
  "tags": "custom,ui",
  "frameworks": "tailwind",
  "html_code": "<div class=\"...\">...</div>",
  "css_code": "",
  "js_code": "",
  "min_metrics": 0,
  "max_metrics": 0,
  "min_dimensions": 0,
  "max_dimensions": 0
}
```

## 📊 Database Integration

Components are stored in `_warphead_component_templates` table:

```sql
SELECT * FROM _warphead_component_templates;
```

Library components have:
- Auto-generated slugs from component names
- `ON CONFLICT DO UPDATE` for updates
- All fields from JSON mapped to columns

## 🎯 Supported Frameworks

### Tailwind CSS
- Utility-first CSS framework
- Pre-loaded in preview iframe
- CDN: `https://cdn.tailwindcss.com`

### Alpine.js
- Lightweight JavaScript framework for interactivity
- Pre-loaded in preview iframe
- CDN: `https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js`

### Heroicons
- SVG icon library
- Can be added via CDN if needed

## ✨ Key Features

- **Automatic Loading** - Components load on app initialization
- **Hot Reload** - Edit JSON and restart to see changes
- **Template Browser** - Browse and preview components before using
- **Iframe Preview** - Secure sandbox preview with live rendering
- **Data Binding** - Components can integrate with metrics and dimensions
- **Framework Support** - Built-in support for Tailwind CSS and Alpine.js

## 📝 Technical Details

### Backend (Rust)
- Location: `src-tauri/src/commands/database.rs`
- Function: `initialize_builtin_components()`
- Model: `src-tauri/src/models/metadata.rs` (LibraryComponent, LibraryComponentsJson)

### Frontend (Svelte)
- Editor: `src/lib/components/ComponentEditor.svelte`
- Browser: `src/lib/components/ComponentTemplateBrowser.svelte`
- Runtime: `src/lib/viz/component-template-runtime.ts`
- Types: `src/lib/types/component-templates.ts`

## 🔗 Related Documentation

- [Component Templates Feature](../../docs/COMPONENT-TEMPLATES-FEATURE.md)
- [Component Editor Guide](../../docs/COMPONENT-EDITOR-GUIDE.md)
- [Library Charts](../charts/README.md)





