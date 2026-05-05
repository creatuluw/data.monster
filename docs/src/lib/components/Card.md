# Card Component Usage

A beautifully redesigned card component with modern aesthetics, smooth animations, and excellent space utilization. Features color-coordinated icons, hover effects, and dark mode support.

## Import

```svelte
<script lang="ts">
	import Card from '$lib/components/Card.svelte';
	import { Database, Calculator, Tag } from 'lucide-svelte';
</script>
```

## Basic Usage

### Simple Card (no icon, no badge)
```svelte
<Card
	title="My Feature"
	description="A brief description of what this card represents"
	href="/path/to/feature"
/>
```

### Card with Icon
```svelte
<Card
	title="Database"
	description="Manage your database tables and schemas"
	href="/database"
	icon={Database}
/>
```

### Card with Badge
```svelte
<Card
	title="Analytics"
	description="View analytics and insights"
	href="/analytics"
	badge="New"
	badgeColor="green"
/>
```

### Full Featured Card
```svelte
<Card
	title="Advanced Features"
	description="Access powerful advanced functionality"
	href="/advanced"
	icon={Calculator}
	badge="Premium"
	badgeColor="purple"
/>
```

## Props

| Prop | Type | Required | Default | Description |
|------|------|----------|---------|-------------|
| `title` | `string` | ✅ | - | The main title of the card |
| `description` | `string` | ✅ | - | Description text below the title |
| `href` | `string` | ✅ | - | Link destination |
| `icon` | `ComponentType<Icon>` | ❌ | - | Lucide icon component |
| `badge` | `string` | ❌ | - | Badge text (e.g., "New", "Beta") |
| `badgeColor` | `'green' \| 'blue' \| 'purple' \| 'orange'` | ❌ | `'green'` | Badge color theme (affects both badge and icon styling) |

## Badge Colors

The `badgeColor` prop controls the color theme for both the badge and the icon container:

- **green**: Success/positive states (e.g., "Active", "Open-Source") - Green accents throughout
- **blue**: Informational (e.g., "Beta", "Info") - Blue accents throughout
- **purple**: Premium/advanced features (e.g., "Pro", "Advanced") - Purple accents throughout
- **orange**: Warnings/upcoming (e.g., "Coming Soon", "Limited") - Orange accents throughout

## Design Features

### Visual Enhancements
- **Larger icons** (12x12 container with 5x5 icon) for better prominence
- **Color-coordinated icon backgrounds** that match the badge color
- **Rounded-xl borders** for a more modern, softer appearance
- **Improved typography hierarchy** with larger title (text-xl) and better spacing
- **Enhanced padding** (p-6 instead of p-5) for better breathing room

### Hover Effects
- **Lift animation**: Card translates up slightly (-translate-y-0.5)
- **Enhanced shadow**: Larger, colored shadow on hover
- **Gradient overlay**: Subtle gradient appears on hover
- **Icon scale**: Icon container scales to 110% on hover
- **Action indicator**: "Explore" text with arrow that moves on hover
- **Smooth transitions**: All animations use 300ms duration with ease-out

### Interactive Elements
- **"Explore" call-to-action** appears at the bottom with animated arrow
- **Arrow translates horizontally** on hover for clear interaction feedback
- **Color-coded action text** matches the badge color theme

### Dark Mode
- Full dark mode support with appropriate contrast
- Gradient overlays adapt to dark backgrounds
- All colors have dark mode variants

## Grid Layouts

### 2 Column Grid
```svelte
<div class="grid sm:grid-cols-2 gap-5">
	<Card title="..." description="..." href="..." />
	<Card title="..." description="..." href="..." />
</div>
```

### 3 Column Grid (recommended)
```svelte
<div class="grid sm:grid-cols-2 md:grid-cols-1 lg:grid-cols-3 gap-5">
	<Card title="..." description="..." href="..." />
	<Card title="..." description="..." href="..." />
	<Card title="..." description="..." href="..." />
</div>
```

## Available Lucide Icons

Common icons you might want to use:
- `Database`, `Table`, `Columns`
- `Calculator`, `TrendingUp`, `BarChart`, `LineChart`
- `Tag`, `Tags`, `Bookmark`
- `Code`, `FileCode`, `Terminal`
- `GitBranch`, `GitMerge`
- `Settings`, `Sliders`
- `Users`, `User`, `UserPlus`
- `Package`, `Boxes`
- `Zap`, `Sparkles`
- `Lock`, `Shield`

See full list: https://lucide.dev/icons/

