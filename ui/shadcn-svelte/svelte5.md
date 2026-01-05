Description: How to migrate from Svelte 4 and Tailwind 3 to Svelte 5.

Keywords: shadcn-svelte,svelte components,svelte ui library,svelte design system,tailwindcss components,headless ui svelte,accessible svelte components,copy paste components,svelte component library,sveltekit components,svelte 5 components,svelte runes,bits-ui,radix svelte,shadcn svelte,free ui components,open source components,modern ui svelte,typescript svelte components,responsive components,dark mode components,form components svelte,dashboard components,chart components svelte,svelte chart components

Title: Svelte 5 - shadcn-svelte

Get Started

- Introduction
- Installation
- components.json
- Theming
- Dark Mode
- CLI
- JavaScript
- Figma
- Changelog
- Legacy Docs

Migration

- Svelte 5
- Tailwind v4

Components

- Accordion
- Alert Dialog
- Alert
- Aspect Ratio
- Avatar
- Badge
- Breadcrumb
- Button Group
- Button
- Calendar
- Card
- Carousel
- Chart
- Checkbox
- Collapsible
- Combobox
- Command
- Context Menu
- Data Table
- Date Picker
- Dialog
- Drawer
- Dropdown Menu
- Empty
- Field
- Formsnap
- Hover Card
- Input Group
- Input OTP
- Input
- Item
- Kbd
- Label
- Menubar
- Navigation Menu
- Pagination
- Popover
- Progress
- Radio Group
- Range Calendar
- Resizable
- Scroll Area
- Select
- Separator
- Sheet
- Sidebar
- Skeleton
- Slider
- Sonner
- Spinner
- Switch
- Table
- Tabs
- Textarea
- Toggle Group
- Toggle
- Tooltip
- Typography

Installation

- SvelteKit
- Vite
- Astro
- Manual Installation

Dark Mode

- Svelte
- Astro

Registry

- Registry
- Getting Started
- FAQ
- Examples
- registry.json
- registry-item.json

On This Page

Svelte 4 to Svelte 5PrerequisitesUpdate ConfigsUpdate components.jsonUpdate tailwind.config.jsUpdate utils.tsUpgrade ComponentsAlias Dependencies (optional)Update DependenciesStart Migrating ComponentsRemove Unused Dependenciescmdk-svsvelte-headless-tablesvelte-radixlucide-svelteNext Steps

Special sponsor

We're looking for one partner to be featured here.

Support the project and reach thousands of developers.

Reach out

# Svelte 5

Previous Next

How to migrate from Svelte 4 and Tailwind 3 to Svelte 5.

**Note**: With Svelte 5 comes significant changes to this project, along with the headless UI library used bits-ui. This guide is specifically focused on migrating the shadcn-svelte portions and does not cover the migration of `bits-ui`. See Bits UI's migration guide for more information.

## Svelte 4 to Svelte 5

This first guide will take your project from Svelte 4 with Tailwind 3 to Svelte 5 and Tailwind 3.

Once you've completed this guide and you're comfortable everything is working, you can move on to the next guide to migrate to Tailwind 4.

## Prerequisites

1.  Ensure you have read up on the changes from Svelte 4 to Svelte 5. Svelte provides a comprehensive guide for this on their website.
2.  Commit any pending changes to your repository.
3.  Determine which of your components have custom behavior/styles so that you can reimplement those after updating.
4.  Use `sv-migrate` to help you migrate your project to Svelte 5.

## Update Configs

The `components.json`, `utils`, and the global CSS file have changed for Svelte 5.

### Update `components.json`

Add the `registry` to the root object, and add `hooks`, `ui`, and `lib` keys under `aliases`.

```
{
"$schema": "https://shadcn-svelte.com/schema.json",
"style": "default",
"tailwind": {
"css": "src/app.css",
"baseColor": "slate"
},
"aliases": {
"components": "$lib/components",
"utils": "$lib/utils",
+   "ui": "$lib/components/ui",
+   "hooks": "$lib/hooks",
+   "lib": "$lib"
},
"typescript": true,
+ "registry": "https://shadcn-svelte.com/registry"
}
```

Copy

### Update `tailwind.config.js`

Add `tailwindcss-animate`.

pnpmnpmbunyarn

```
pnpm i tailwindcss-animate
```

```
npm i tailwindcss-animate
```

```
bun install tailwindcss-animate
```

```
yarn install tailwindcss-animate
```

Copy

Add the `tailwindcss-animate` plugin, sidebar colors, and animations config.

tailwind.config.js

```
import type { Config } from "tailwindcss";
import tailwindcssAnimate from "tailwindcss-animate";
const config: Config = {
darkMode: ["class"],
content: ["./src/**/*.{html,js,svelte,ts}"],
safelist: ["dark"],
theme: {
container: {
// unchanged ...
},
extend: {
colors: {
// unchanged ...
sidebar: {
DEFAULT: "hsl(var(--sidebar-background))",
foreground: "hsl(var(--sidebar-foreground))",
primary: "hsl(var(--sidebar-primary))",
"primary-foreground": "hsl(var(--sidebar-primary-foreground))",
accent: "hsl(var(--sidebar-accent))",
"accent-foreground": "hsl(var(--sidebar-accent-foreground))",
border: "hsl(var(--sidebar-border))",
ring: "hsl(var(--sidebar-ring))",
},
},
borderRadius: {
// unchanged ...
},
fontFamily: {
// unchanged ...
},
keyframes: {
"accordion-down": {
from: { height: "0" },
to: { height: "var(--bits-accordion-content-height)" },
},
"accordion-up": {
from: { height: "var(--bits-accordion-content-height)" },
to: { height: "0" },
},
"caret-blink": {
"0%,70%,100%": { opacity: "1" },
"20%,50%": { opacity: "0" },
},
},
animation: {
"accordion-down": "accordion-down 0.2s ease-out",
"accordion-up": "accordion-up 0.2s ease-out",
"caret-blink": "caret-blink 1.25s ease-out infinite",
},
},
},
plugins: [tailwindcssAnimate],
};
export default config;
```

Copy

### Update `utils.ts`

**Note**: You may not want to do this step until after you've updated your components, as some components may rely on the now removed `flyAndScale` function.

`utils.ts` now only exports the `cn` function and a few utility types.

src/lib/utils.ts

```
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
export function cn(...inputs: ClassValue[]) {
return twMerge(clsx(inputs));
}
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any }
? Omit<T, "children">
: T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & {
ref?: U | null;
};
```

Copy

## Upgrade Components

### Alias Dependencies (optional)

If you plan to slowly migrate components, it's recommended to alias the old versions of the major dependencies, like `bits-ui`, in your `package.json` file so that you can use both versions of the library in your project while you migrate.

package.json

```
{
"devDependencies": {
-	"bits-ui": "^0.22.0",
+   "bits-ui-old": "npm:bits-ui@0.22.0",
}
}
```

Copy

You'll then want to replace all the imports used in your project to `bits-ui-old`.

src/lib/components/ui/dialog-content.svelte

```
<script lang="ts">
-  import { Dialog as DialogPrimitive } from "bits-ui";
+  import { Dialog as DialogPrimitive } from "bits-ui-old";
</script>
```

Copy

You can do the same for any of the other dependencies that you're using in your project.

### Update Dependencies

The following dependencies have been updated to support Svelte 5:

- `bits-ui` - `^1.0.0`
- `svelte-sonner` - `^1.0.0`
- `@lucide/svelte` - `^0.482.0`
- `paneforge` - `^1.0.0-next.5`
- `vaul-svelte` - `^1.0.0-next.7`
- `mode-watcher` - `^1.0.0`
- `cmdk-sv` - deprecated in favor of Bits UI's `Command` component
- `svelte-headless-table` - deprecated in favor of `@tanstack/table-core`
- `svelte-radix` - icons deprecated in favor of `@lucide/svelte`
- `lucide-svelte` - replaced with `@lucide/svelte`

You can update your dependencies by running the following command:

pnpmnpmbunyarn

```
pnpm i bits-ui@latest svelte-sonner@latest @lucide/svelte@latest paneforge@next vaul-svelte@next mode-watcher@latest -D
```

```
npm i bits-ui@latest svelte-sonner@latest @lucide/svelte@latest paneforge@next vaul-svelte@next mode-watcher@latest -D
```

```
bun install bits-ui@latest svelte-sonner@latest @lucide/svelte@latest paneforge@next vaul-svelte@next mode-watcher@latest -D
```

```
yarn install bits-ui@latest svelte-sonner@latest @lucide/svelte@latest paneforge@next vaul-svelte@next mode-watcher@latest -D
```

Copy

### Start Migrating Components

Now you're ready to begin updating your components to their new versions. The CLI doesn't actually _update_ your components, it simply replaces them with the new versions, so be sure to commit your changes before running the CLI.

```
git add .
git commit -m 'before migration'
```

Copy

Now you can run the `add` command to start migrating your components.

pnpmnpmbunyarn

```
pnpm dlx shadcn-svelte@latest add dialog --overwrite
```

```
npx shadcn-svelte@latest add dialog --overwrite
```

```
bun x shadcn-svelte@latest add dialog --overwrite
```

```
npx shadcn-svelte@latest add dialog --overwrite
```

Copy

Review the diff to see what was updated and make any necessary adjustments. Rinse and repeat for each component you want to migrate.

## Remove Unused Dependencies

Once you've updated all your components, you can remove the old dependencies from your `package.json` file.

### cmdk-sv

`cmdk-sv` has been replaced with Bits UI's `Command` component.

pnpmnpmbunyarn

```
pnpm remove cmdk-sv
```

```
npm uninstall cmdk-sv
```

```
bun remove cmdk-sv
```

```
yarn remove cmdk-sv
```

Copy

### svelte-headless-table

`svelte-headless-table` has been replaced with `@tanstack/table-core`.

pnpmnpmbunyarn

```
pnpm remove svelte-headless-table
```

```
npm uninstall svelte-headless-table
```

```
bun remove svelte-headless-table
```

```
yarn remove svelte-headless-table
```

Copy

### svelte-radix

`svelte-radix` has been replaced with `@lucide/svelte`.

pnpmnpmbunyarn

```
pnpm remove svelte-radix
```

```
npm uninstall svelte-radix
```

```
bun remove svelte-radix
```

```
yarn remove svelte-radix
```

Copy

### lucide-svelte

`lucide-svelte` has been replaced with `@lucide/svelte`.

pnpmnpmbunyarn

```
pnpm remove lucide-svelte
```

```
npm uninstall lucide-svelte
```

```
bun remove lucide-svelte
```

```
yarn remove lucide-svelte
```

Copy

## Next Steps

Once you've completed this guide and you're comfortable everything is working as expected, you can move on to the Tailwind 4 Guide.

Migration Tailwind v4
