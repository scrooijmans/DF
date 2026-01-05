# CSS Configuration Guide

This document provides a comprehensive overview of all CSS-related dependencies, configurations, and setup for the MudRock application. This guide will help external users replicate our CSS configuration.

## Overview

Our application uses a modern CSS stack with:
- **Tailwind CSS** for utility-first styling
- **shadcn-svelte** for component library and design system
- **PostCSS** for CSS processing
- **tw-animate-css** for animations

## Dependencies

### Core CSS Dependencies

```json
{
  "devDependencies": {
    "tailwindcss": "^3.x.x",
    "postcss": "^8.x.x",
    "autoprefixer": "^10.x.x",
    "@tailwindcss/postcss": "^4.x.x",
    "tw-animate-css": "^1.x.x",
    "prettier-plugin-tailwindcss": "^0.6.x"
  }
}
```

### Installation Commands

```bash
# Install core Tailwind CSS dependencies
npm install -D tailwindcss postcss autoprefixer --legacy-peer-deps

# Install the new PostCSS plugin for Tailwind CSS v4
npm install -D @tailwindcss/postcss --legacy-peer-deps

# Install animation library
npm install -D tw-animate-css --legacy-peer-deps

# Install shadcn-svelte (this will also install additional dependencies)
npx shadcn-svelte@latest init
```

## Configuration Files

### 1. `tailwind.config.js`

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [],
}
```

**Key Points:**
- Content paths include all Svelte, TypeScript, and HTML files
- Custom font family configuration with Inter as primary font
- Extensible theme structure for future customizations

### 2. `postcss.config.js`

```javascript
export default {
  plugins: {
    '@tailwindcss/postcss': {},
    autoprefixer: {},
  },
}
```

**Key Points:**
- Uses the new `@tailwindcss/postcss` plugin (required for Tailwind CSS v4)
- Includes autoprefixer for cross-browser compatibility
- ES module syntax for modern JavaScript

### 3. `components.json` (shadcn-svelte configuration)

```json
{
  "$schema": "https://shadcn-svelte.com/schema.json",
  "tailwind": {
    "css": "src\\app.css",
    "baseColor": "slate"
  },
  "aliases": {
    "components": "$lib/components",
    "utils": "$lib/utils",
    "ui": "$lib/components/ui",
    "hooks": "$lib/hooks",
    "lib": "$lib"
  },
  "typescript": true,
  "registry": "https://shadcn-svelte.com/registry"
}
```

**Key Points:**
- Points to our main CSS file (`src/app.css`)
- Uses "slate" as the base color theme
- Defines import aliases for components and utilities
- Enables TypeScript support

## CSS File Structure

### `src/app.css`

Our main CSS file contains:

```css
@import "tailwindcss";

@custom-variant dark (&:is(.dark *));

:root {
  --radius: 0.625rem;
  --background: oklch(1 0 0);
  --foreground: oklch(0.129 0.042 264.695);
  /* ... additional CSS custom properties for theming */
}

.dark {
  --background: oklch(0.129 0.042 264.695);
  --foreground: oklch(0.984 0.003 247.858);
  /* ... dark mode color overrides */
}

@theme inline {
  /* Theme configuration for Tailwind CSS v4 */
}

@layer base {
  * {
    @apply border-border outline-ring/50;
  }
  body {
    @apply bg-background text-foreground;
  }
}
```

**Key Features:**
- Tailwind CSS imports
- CSS custom properties for theming
- Dark mode support
- Base layer styles for consistent styling

### `src/app.html`

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <link rel="icon" href="%sveltekit.assets%/favicon.png" />
    <meta name="viewport" content="width=device-width" />
    <link rel="stylesheet" href="/src/app.css" />
    %sveltekit.head%
  </head>
  <body data-sveltekit-preload-data="hover">
    <div style="display: contents">%sveltekit.body%</div>
  </body>
</html>
```

**Key Points:**
- Links to our main CSS file
- SvelteKit-specific placeholders for dynamic content

## Font Configuration

### Primary Font Stack

```css
font-family: 'Inter', system-ui, sans-serif;
```

**Font Sources:**
- **Inter**: Google Fonts (loaded via CSS import)
- **system-ui**: Native system font fallback
- **sans-serif**: Generic sans-serif fallback

### Font Loading

Fonts are loaded via CSS import in `src/app.css`:

```css
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@100;200;300;400;500;600;700;800;900&display=swap');
```

## Component Styling Approach

### 1. Utility-First with Tailwind CSS

We use Tailwind utility classes for styling:

```svelte
<div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-indigo-500 to-purple-600 p-4">
  <div class="bg-white rounded-xl shadow-2xl p-8 w-full max-w-md">
    <!-- Component content -->
  </div>
</div>
```

### 2. shadcn-svelte Components

For complex components, we use shadcn-svelte:

```svelte
<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
</script>

<Button>Click me</Button>
```

### 3. Custom CSS (Minimal)

We avoid custom CSS in favor of Tailwind utilities. When needed, we use CSS custom properties:

```css
.custom-component {
  background-color: var(--background);
  color: var(--foreground);
}
```

## Theme System

### Color Palette

Our theme uses CSS custom properties with OKLCH color space:

- **Background**: Light/dark mode variants
- **Foreground**: Text colors
- **Primary**: Brand colors
- **Secondary**: Supporting colors
- **Muted**: Subtle colors
- **Accent**: Highlight colors
- **Destructive**: Error/warning colors

### Dark Mode Support

Dark mode is implemented using CSS custom properties and the `.dark` class:

```css
:root {
  --background: oklch(1 0 0); /* Light mode */
}

.dark {
  --background: oklch(0.129 0.042 264.695); /* Dark mode */
}
```

## Animation System

### tw-animate-css

We use `tw-animate-css` for animations:

```svelte
<div class="animate-spin">
  <!-- Spinning element -->
</div>
```

### Custom Animations

Custom animations can be added to the Tailwind config:

```javascript
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      animation: {
        'fade-in': 'fadeIn 0.5s ease-in-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
      },
    },
  },
}
```

## Development Workflow

### 1. Adding New Components

```bash
# Add shadcn-svelte components
npx shadcn-svelte@latest add button
npx shadcn-svelte@latest add input
npx shadcn-svelte@latest add card
```

### 2. Customizing Themes

Edit CSS custom properties in `src/app.css`:

```css
:root {
  --primary: oklch(0.5 0.2 200); /* Custom primary color */
}
```

### 3. Adding Custom Utilities

Extend Tailwind config:

```javascript
// tailwind.config.js
module.exports = {
  theme: {
    extend: {
      spacing: {
        '18': '4.5rem',
      },
    },
  },
}
```

## Troubleshooting

### Common Issues

1. **PostCSS Plugin Error**
   ```
   Error: It looks like you're trying to use `tailwindcss` directly as a PostCSS plugin
   ```
   **Solution**: Install `@tailwindcss/postcss` and update `postcss.config.js`

2. **Missing tw-animate-css**
   ```
   Error: Can't resolve 'tw-animate-css'
   ```
   **Solution**: Install `tw-animate-css` package

3. **Font Not Loading**
   **Solution**: Ensure Google Fonts import is in `src/app.css`

### Dependency Conflicts

If you encounter peer dependency conflicts:

```bash
npm install --legacy-peer-deps
```

## Best Practices

### 1. Utility-First Approach
- Use Tailwind utility classes instead of custom CSS
- Leverage responsive prefixes (`sm:`, `md:`, `lg:`, `xl:`)
- Use state variants (`hover:`, `focus:`, `active:`)

### 2. Component Organization
- Keep components in `src/lib/components/`
- Use shadcn-svelte for complex UI components
- Create custom components for application-specific needs

### 3. Theme Consistency
- Use CSS custom properties for colors
- Maintain consistent spacing using Tailwind's scale
- Follow the established color palette

### 4. Performance
- Use `@apply` sparingly
- Prefer utility classes over custom CSS
- Optimize font loading with `display=swap`

## File Structure Summary

```
project-root/
├── src/
│   ├── app.css              # Main CSS file with imports and theme
│   ├── app.html             # HTML template with CSS link
│   └── lib/
│       └── components/
│           ├── ui/          # shadcn-svelte components
│           └── auth/        # Custom auth components
├── docs/
│   └── css/
│       └── CSS_CONFIGURATION_GUIDE.md  # This file
├── tailwind.config.js       # Tailwind configuration
├── postcss.config.js        # PostCSS configuration
├── components.json          # shadcn-svelte configuration
└── package.json             # Dependencies
```

## Replication Steps

To replicate this CSS configuration in a new project:

1. **Install Dependencies**
   ```bash
   npm install -D tailwindcss postcss autoprefixer @tailwindcss/postcss tw-animate-css --legacy-peer-deps
   ```

2. **Initialize Tailwind CSS**
   ```bash
   npx tailwindcss init -p
   ```

3. **Set up shadcn-svelte**
   ```bash
   npx shadcn-svelte@latest init
   ```

4. **Copy Configuration Files**
   - Copy `tailwind.config.js`
   - Copy `postcss.config.js`
   - Copy `src/app.css` content
   - Update `src/app.html` to include CSS link

5. **Install Additional Components**
   ```bash
   npx shadcn-svelte@latest add button input card
   ```

This configuration provides a solid foundation for modern, maintainable CSS in SvelteKit applications with Tailwind CSS and shadcn-svelte.
