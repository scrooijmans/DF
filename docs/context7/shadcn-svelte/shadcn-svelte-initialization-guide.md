# shadcn-svelte Initialization Guide

This document explains how to initialize shadcn-svelte for a new project, based on the official shadcn-svelte documentation.

## Overview

shadcn-svelte is a collection of reusable components built with Svelte and Tailwind CSS. It provides accessible, customizable UI components that you copy into your project (not installed as a dependency).

## Prerequisites

### Required

1. **Node.js v20 or higher**

   ```bash
   node --version  # Should be v20+
   ```

2. **SvelteKit Project** (or Vite + Svelte, or Astro + Svelte)

   ```bash
   npm create svelte@latest my-app
   cd my-app
   npm install
   ```

3. **TypeScript** (recommended)
   - SvelteKit projects include TypeScript by default
   - For Vite projects, ensure TypeScript is configured

### Optional but Recommended

- **Tailwind CSS** (can be added during setup)
- **Path aliases** configured in `tsconfig.json` or `svelte.config.js`

## Step-by-Step Initialization

### For SvelteKit Projects

#### Step 1: Create SvelteKit Project (if not already created)

```bash
npm create svelte@latest my-app
cd my-app
npm install
```

#### Step 2: Add Tailwind CSS

shadcn-svelte requires Tailwind CSS. Add it using the SvelteKit CLI:

```bash
npx sv add tailwindcss
```

**Important**: Answer "Yes" to all prompts during the Tailwind installation.

This will:

- Install Tailwind CSS and its dependencies
- Create `tailwind.config.js` (or update existing)
- Create/update your global CSS file
- Configure PostCSS

#### Step 3: Configure Path Aliases (if not already configured)

Ensure path aliases are set up in `svelte.config.js`:

```javascript
// svelte.config.js
import adapter from "@sveltejs/adapter-auto";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter(),
    alias: {
      $lib: "./src/lib",
      "$lib/*": "./src/lib/*",
    },
  },
};

export default config;
```

And in `tsconfig.json`:

```json
{
  "extends": "./.svelte-kit/tsconfig.json",
  "compilerOptions": {
    "allowJs": true,
    "checkJs": true,
    "esModuleInterop": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "skipLibCheck": true,
    "sourceMap": true,
    "strict": true,
    "baseUrl": ".",
    "paths": {
      "$lib": ["./src/lib"],
      "$lib/*": ["./src/lib/*"]
    }
  }
}
```

#### Step 4: Run shadcn-svelte Init

Run the initialization command:

```bash
# Using npm
npx shadcn-svelte@latest init

# Using pnpm
pnpm dlx shadcn-svelte@latest init

# Using bun
bun x shadcn-svelte@latest init

# Using yarn
npx shadcn-svelte@latest init
```

#### Step 5: Answer Configuration Questions

The CLI will ask you several questions to configure `components.json`:

```
? Which base color would you like to use? › Slate
  Slate
  Gray
  Zinc
  Neutral
  Stone
  Red
  Rose
  Orange
  Green
  Blue
  Yellow
  Violet

? Where is your global CSS file? › src/app.css
  (Enter the path to your global CSS file)

? Configure the import alias for lib: › $lib
  (Enter the alias for lib directory)

? Configure the import alias for components: › $lib/components
  (Enter the alias for components directory)

? Configure the import alias for utils: › $lib/utils
  (Enter the alias for utils directory)

? Configure the import alias for hooks: › $lib/hooks
  (Enter the alias for hooks directory)

? Configure the import alias for ui: › $lib/components/ui
  (Enter the alias for UI components directory)
```

**Recommended Answers for SvelteKit**:

- **Base color**: `Slate` (or your preference)
- **Global CSS file**: `src/app.css` (or `src/routes/+layout.css` if using SvelteKit layout)
- **lib alias**: `$lib`
- **components alias**: `$lib/components`
- **utils alias**: `$lib/utils`
- **hooks alias**: `$lib/hooks`
- **ui alias**: `$lib/components/ui`

#### Step 6: Verify Installation

After initialization, you should have:

1. **`components.json`** file in the project root:

   ```json
   {
     "$schema": "https://shadcn-svelte.com/schema.json",
     "tailwind": {
       "css": "src/app.css",
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

2. **Dependencies installed**:
   - `tailwind-variants`
   - `clsx`
   - `tailwind-merge`
   - `tw-animate-css`

3. **`src/lib/utils.ts`** (or `src/lib/utils.js`) with the `cn` utility function:

   ```typescript
   import { type ClassValue, clsx } from "clsx";
   import { twMerge } from "tailwind-merge";

   export function cn(...inputs: ClassValue[]) {
     return twMerge(clsx(inputs));
   }
   ```

4. **Global CSS file updated** with:
   - Tailwind CSS imports
   - CSS variables for theming
   - Base styles

#### Step 7: Add Your First Component

Test the setup by adding a component:

```bash
# Add a button component
npx shadcn-svelte@latest add button

# Add multiple components
npx shadcn-svelte@latest add button card input
```

This will:

- Create component files in `src/lib/components/ui/`
- Add necessary dependencies
- Make components ready to use

## For Vite + Svelte Projects

### Step 1: Create Vite Project

```bash
npm create vite@latest my-app -- --template svelte-ts
cd my-app
npm install
```

### Step 2: Install Tailwind CSS

```bash
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

### Step 3: Configure Tailwind CSS

Update `tailwind.config.js`:

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,svelte}"],
  theme: {
    extend: {},
  },
  plugins: [],
};
```

Create/update `src/app.css`:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

### Step 4: Configure TypeScript Paths

Update `tsconfig.json`:

```json
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "$lib": ["./src/lib"],
      "$lib/*": ["./src/lib/*"]
    }
  }
}
```

### Step 5: Configure Vite Alias

Update `vite.config.ts`:

```typescript
import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import path from "path";

export default defineConfig({
  plugins: [svelte()],
  resolve: {
    alias: {
      $lib: path.resolve("./src/lib"),
    },
  },
});
```

### Step 6: Run Init

```bash
npx shadcn-svelte@latest init
```

Follow the same configuration questions as SvelteKit.

## For Astro + Svelte Projects

### Step 1: Create Astro Project

```bash
npm create astro@latest my-app
cd my-app
```

### Step 2: Add Svelte Support

```bash
npx astro add svelte
```

**Important**: Answer "Yes" to all prompts.

### Step 3: Add Tailwind CSS

```bash
npx astro add tailwind
```

**Important**: Answer "Yes" to all prompts.

### Step 4: Configure Path Aliases

Update `tsconfig.json`:

```json
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "$lib": ["./src/lib"],
      "$lib/*": ["./src/lib/*"]
    }
  }
}
```

### Step 5: Run Init

```bash
npx shadcn-svelte@latest init
```

## What Gets Created

### File Structure

After initialization, your project structure should look like:

```
my-app/
├── components.json          # Configuration file
├── src/
│   ├── app.css             # Global CSS (updated with theme variables)
│   └── lib/
│       ├── components/
│       │   └── ui/         # UI components will be added here
│       └── utils.ts        # cn() utility function
└── package.json            # Dependencies added
```

### components.json Structure

```json
{
  "$schema": "https://shadcn-svelte.com/schema.json",
  "tailwind": {
    "css": "src/app.css",
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

### Global CSS File

The global CSS file (e.g., `src/app.css`) will be updated with:

1. **Tailwind CSS imports**:

   ```css
   @import "tailwindcss";
   @import "tw-animate-css";
   ```

2. **CSS variables for theming** (light and dark mode):

   ```css
   :root {
     --radius: 0.625rem;
     --background: oklch(1 0 0);
     --foreground: oklch(0.145 0 0);
     /* ... more variables */
   }

   .dark {
     --background: oklch(0.145 0 0);
     --foreground: oklch(0.985 0 0);
     /* ... dark mode variables */
   }
   ```

3. **Base styles**:

   ```css
   @layer base {
     * {
       @apply border-border outline-ring/50;
     }

     body {
       @apply bg-background text-foreground;
     }
   }
   ```

## Dependencies Installed

The `init` command automatically installs:

- **`tailwind-variants`** - For component variants
- **`clsx`** - For conditional class names
- **`tailwind-merge`** - For merging Tailwind classes
- **`tw-animate-css`** - For animations (replaces `tailwindcss-animate`)

## Usage After Initialization

### Adding Components

```bash
# Add a single component
npx shadcn-svelte@latest add button

# Add multiple components
npx shadcn-svelte@latest add button card input select

# Add all components (not recommended)
npx shadcn-svelte@latest add --all
```

### Using Components

```svelte
<script>
  import { Button } from "$lib/components/ui/button";
</script>

<Button>Click me</Button>
```

### Using the `cn` Utility

```svelte
<script>
  import { cn } from "$lib/utils";
</script>

<div class={cn("base-class", condition && "conditional-class")}>
  Content
</div>
```

## Troubleshooting

### Error: "Cannot apply unknown utility class 'border-border'"

**Problem**: This error occurs when Tailwind CSS v4 is being used but the CSS variables aren't properly configured or the syntax is incorrect.

**Solution**:

1. **Ensure `@theme inline` has color mappings**: Your `@theme inline` block must map CSS variables to Tailwind colors:

   ```css
   @theme inline {
     --color-border: var(--border);
     --color-input: var(--input);
     --color-ring: var(--ring);
     /* ... all other color mappings */
   }
   ```

2. **Fix the `@apply` directive**: The `border-border` utility should work if `--color-border` is defined in `@theme inline`. If you're getting an error (common in Tailwind CSS v4.1.x), use CSS variables directly:

   ```css
   @layer base {
     * {
       border-color: var(--border);
       outline-color: color-mix(in oklch, var(--ring) 50%, transparent);
     }
   }
   ```

   **Note**: This is a workaround for a known issue. The `@apply border-border` syntax should work once Tailwind properly recognizes the theme mapping.

3. **Verify Tailwind v4 Vite plugin**: Ensure you're using the Vite plugin (not PostCSS):

   ```bash
   npm install -D @tailwindcss/vite
   ```

4. **Update `vite.config.js`**:

   ```javascript
   import tailwindcss from "@tailwindcss/vite";

   export default defineConfig({
     plugins: [svelte(), tailwindcss()],
   });
   ```

5. **Restart dev server**: After making changes, restart your development server.

### Error: "Module not found: Can't resolve '$lib'"

**Problem**: Path aliases aren't configured correctly.

**Solution**:

1. Check `svelte.config.js` (for SvelteKit) or `vite.config.ts` (for Vite)
2. Ensure `tsconfig.json` has the correct paths
3. Restart your dev server

### Components Not Styling Correctly

**Problem**: Tailwind CSS isn't detecting component files.

**Solution**:

1. Check `tailwind.config.js` content paths:

   ```javascript
   content: ["./src/**/*.{html,js,svelte,ts}"];
   ```

2. Ensure your global CSS file is imported in your root layout:
   ```svelte
   <!-- src/routes/+layout.svelte -->
   <script>
     import '../app.css';
   </script>
   ```

## Next Steps

After initialization:

1. **Add components** you need:

   ```bash
   npx shadcn-svelte@latest add button card input
   ```

2. **Customize theme** by editing CSS variables in your global CSS file

3. **Configure dark mode** (see shadcn-svelte dark mode documentation)

4. **Start building** your UI with the components

## Summary

Initializing shadcn-svelte involves:

1. ✅ **Prerequisites**: Node.js v20+, SvelteKit/Vite/Astro project, Tailwind CSS
2. ✅ **Run init**: `npx shadcn-svelte@latest init`
3. ✅ **Configure**: Answer questions about base color, CSS file, and path aliases
4. ✅ **Verify**: Check that `components.json`, `utils.ts`, and CSS are created/updated
5. ✅ **Add components**: Use `npx shadcn-svelte@latest add <component-name>`

The initialization process is straightforward and mostly automated. The CLI handles dependency installation, file creation, and configuration setup.
