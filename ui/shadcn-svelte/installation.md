Description: How to setup shadcn-svelte in a SvelteKit project.

Keywords: shadcn-svelte,svelte components,svelte ui library,svelte design system,tailwindcss components,headless ui svelte,accessible svelte components,copy paste components,svelte component library,sveltekit components,svelte 5 components,svelte runes,bits-ui,radix svelte,shadcn svelte,free ui components,open source components,modern ui svelte,typescript svelte components,responsive components,dark mode components,form components svelte,dashboard components,chart components svelte,svelte chart components

Title: SvelteKit - shadcn-svelte

Get Started

*   Introduction
*   Installation
*   components.json
*   Theming
*   Dark Mode
*   CLI
*   JavaScript
*   Figma
*   Changelog
*   Legacy Docs

Migration

*   Svelte 5
*   Tailwind v4

Components

*   Accordion
*   Alert Dialog
*   Alert
*   Aspect Ratio
*   Avatar
*   Badge
*   Breadcrumb
*   Button
*   Calendar
*   Card
*   Carousel
*   Chart
*   Checkbox
*   Collapsible
*   Combobox
*   Command
*   Context Menu
*   Data Table
*   Date Picker
*   Dialog
*   Drawer
*   Dropdown Menu
*   Formsnap
*   Hover Card
*   Input OTP
*   Input
*   Label
*   Menubar
*   Navigation Menu
*   Pagination
*   Popover
*   Progress
*   Radio Group
*   Range Calendar
*   Resizable
*   Scroll Area
*   Select
*   Separator
*   Sheet
*   Sidebar
*   Skeleton
*   Slider
*   Sonner
*   Switch
*   Table
*   Tabs
*   Textarea
*   Toggle Group
*   Toggle
*   Tooltip
*   Typography

Installation

*   SvelteKit
*   Vite
*   Astro
*   Manual Installation

Dark Mode

*   Svelte
*   Astro

Registry

*   Registry
*   Getting Started
*   FAQ
*   Examples
*   registry.json
*   registry-item.json

On This Page

Create projectAdd TailwindCSSSetup path aliasesRun the CLIConfigure components.jsonThat's it

Special sponsor

We're looking for one partner to be featured here.

Support the project and reach thousands of developers.

Reach out

SvelteKit
=========

Previous Next

How to setup shadcn-svelte in a SvelteKit project.

### Create project

Use the SvelteKit CLI to create a new project.

pnpmnpmbunyarn

```
pnpm dlx sv create my-app
```

```
npx sv create my-app
```

```
bun x sv create my-app
```

```
npx sv create my-app
```

Copy

### Add TailwindCSS

Use the Svelte CLI to add Tailwind CSS to your project.

pnpmnpmbunyarn

```
pnpm dlx sv add tailwindcss
```

```
npx sv add tailwindcss
```

```
bun x sv add tailwindcss
```

```
npx sv add tailwindcss
```

Copy

### Setup path aliases

If you are not using the default alias `$lib`, you'll need to update your `svelte.config.js` file to include those aliases.

svelte.config.js

```
const config = {
// ... other config
kit: {
// ... other config
alias: {
"@/*": "./path/to/lib/*",
},
},
};
```

Copy

### Run the CLI

pnpmnpmbunyarn

```
pnpm dlx shadcn-svelte@latest init
```

```
npx shadcn-svelte@latest init
```

```
bun x shadcn-svelte@latest init
```

```
npx shadcn-svelte@latest init
```

Copy

### Configure components.json

You will be asked a few questions to configure `components.json`:

```
Which base color would you like to use? › Slate
Where is your global CSS file? (this file will be overwritten) › src/app.css
Configure the import alias for lib: › $lib
Configure the import alias for components: › $lib/components
Configure the import alias for utils: › $lib/utils
Configure the import alias for hooks: › $lib/hooks
Configure the import alias for ui: › $lib/components/ui
```

Copy

### That's it

You can now start adding components to your project.

pnpmnpmbunyarn

```
pnpm dlx shadcn-svelte@latest add button
```

```
npx shadcn-svelte@latest add button
```

```
bun x shadcn-svelte@latest add button
```

```
npx shadcn-svelte@latest add button
```

Copy

The command above will add the `Button` component to your project. You can then import it like this:

```
<script lang="ts">
import { Button } from "$lib/components/ui/button/index.js";
</script>
<Button>Click me</Button>
```

Copy

Typography Vite