Description: A window overlaid on either the primary window or another dialog window, rendering the content underneath inert.

Keywords: shadcn-svelte,svelte components,svelte ui library,svelte design system,tailwindcss components,headless ui svelte,accessible svelte components,copy paste components,svelte component library,sveltekit components,svelte 5 components,svelte runes,bits-ui,radix svelte,shadcn svelte,free ui components,open source components,modern ui svelte,typescript svelte components,responsive components,dark mode components,form components svelte,dashboard components,chart components svelte,svelte chart components

Title: Dialog - shadcn-svelte

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
*   Button Group
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
*   Empty
*   Field
*   Formsnap
*   Hover Card
*   Input Group
*   Input OTP
*   Input
*   Item
*   Kbd
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
*   Spinner
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

InstallationUsage

Special sponsor

We're looking for one partner to be featured here.

Support the project and reach thousands of developers.

Reach out

Dialog
======

Previous Next

A window overlaid on either the primary window or another dialog window, rendering the content underneath inert.

Docs API Reference Component Source

Preview Code

Edit Profile

```
<script lang="ts">
import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
import * as Dialog from "$lib/components/ui/dialog/index.js";
import { Input } from "$lib/components/ui/input/index.js";
import { Label } from "$lib/components/ui/label/index.js";
</script>
<Dialog.Root>
<Dialog.Trigger class={buttonVariants({ variant: "outline" })}
>Edit Profile</Dialog.Trigger
>
<Dialog.Content class="sm:max-w-[425px]">
<Dialog.Header>
<Dialog.Title>Edit profile</Dialog.Title>
<Dialog.Description>
Make changes to your profile here. Click save when you're done.
</Dialog.Description>
</Dialog.Header>
<div class="grid gap-4 py-4">
<div class="grid grid-cols-4 items-center gap-4">
<Label for="name" class="text-right">Name</Label>
<Input id="name" value="Pedro Duarte" class="col-span-3" />
</div>
<div class="grid grid-cols-4 items-center gap-4">
<Label for="username" class="text-right">Username</Label>
<Input id="username" value="@peduarte" class="col-span-3" />
</div>
</div>
<Dialog.Footer>
<Button type="submit">Save changes</Button>
</Dialog.Footer>
</Dialog.Content>
</Dialog.Root>
```

Copy

Installation
------------

CLI Manual

pnpmnpmbunyarn

```
pnpm dlx shadcn-svelte@latest add dialog
```

```
npx shadcn-svelte@latest add dialog
```

```
bun x shadcn-svelte@latest add dialog
```

```
npx shadcn-svelte@latest add dialog
```

Copy

Install `bits-ui`:

pnpmnpmbunyarn

```
pnpm i bits-ui -D
```

```
npm i bits-ui -D
```

```
bun install bits-ui -D
```

```
yarn install bits-ui -D
```

Copy

Copy and paste the component source files linked at the top of this page into your project.

Usage
-----

```
<script lang="ts">
import * as Dialog from "$lib/components/ui/dialog/index.js";
</script>
<Dialog.Root>
<Dialog.Trigger>Open</Dialog.Trigger>
<Dialog.Content>
<Dialog.Header>
<Dialog.Title>Are you sure absolutely sure?</Dialog.Title>
<Dialog.Description>
This action cannot be undone. This will permanently delete your account
and remove your data from our servers.
</Dialog.Description>
</Dialog.Header>
</Dialog.Content>
</Dialog.Root>
```

Copy

Date Picker Drawer