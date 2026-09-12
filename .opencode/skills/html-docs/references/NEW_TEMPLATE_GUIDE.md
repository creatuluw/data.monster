# Creating New Templates for Unrecognized Use Cases

When a user prompt describes a document use case that doesn't match any of the 20 existing templates, follow this process to create and validate a new one.

## Step 1: Detect a New Use Case

A use case is "new" when:
- The user's need doesn't fit any of the 9 categories or 20 templates in CATEGORIES.md
- The user explicitly asks for something that combines categories in a novel way
- The user wants a format not represented by any existing template

## Step 2: Design the Template

Create a minimal self-contained `.html` file following these conventions:

### Template Conventions
- **Single file**: Everything in one `.html` — CSS in `<style>`, JS at end of `<body>`
- **No external deps**: No CDN links, no npm packages
- **Responsive**: Works on mobile and desktop
- **Dark mode**: Support via `prefers-color-scheme: dark` or a toggle
- **Export**: If it's an editor, include a copy/export button
- **Semantic HTML**: Use `<header>`, `<main>`, `<section>`, `<nav>`, `<article>`, `<aside>`

### Design Principles from the Originals
- Each page has a clear hero/banner with title and context
- Content is chunked visually (cards, columns, timelines)
- Interactive elements have clear affordances
- Color palette is minimal (2-3 accent colors + neutral scale)
- Typography uses system font stack
- Animations are subtle and purposeful

## Step 3: Present to User for Approval

Before generating the full output:
1. Describe the proposed template structure
2. Show a brief outline of sections/components
3. Ask: "This use case doesn't match an existing template. Here's what I'm proposing — shall I proceed?"

## Step 4: Save and Register

After approval:
1. Save the template to `assets/templates/` with a descriptive filename
2. Register it in this document with category, description, and triggers
3. Note: The template becomes available for future use
