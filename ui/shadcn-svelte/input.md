Description: Displays a form input field or a component that looks like an input field.

Keywords: shadcn-svelte,svelte components,svelte ui library,svelte design system,tailwindcss components,headless ui svelte,accessible svelte components,copy paste components,svelte component library,sveltekit components,svelte 5 components,svelte runes,bits-ui,radix svelte,shadcn svelte,free ui components,open source components,modern ui svelte,typescript svelte components,responsive components,dark mode components,form components svelte,dashboard components,chart components svelte,svelte chart components

Title: Input - shadcn-svelte

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

InstallationUsageExamplesDefaultDisabledWith LabelWith TextWith ButtonInvalidFileForm

Special sponsor

We're looking for one partner to be featured here.

Support the project and reach thousands of developers.

Reach out

Input
=====

Previous Next

Displays a form input field or a component that looks like an input field.

Component Source

Preview Code

```
<script lang="ts">
import { Input } from "$lib/components/ui/input/index.js";
</script>
<Input type="email" placeholder="email" class="max-w-xs" />
```

Copy

Installation
------------

CLI Manual

pnpmnpmbunyarn

```
pnpm dlx shadcn-svelte@latest add input
```

```
npx shadcn-svelte@latest add input
```

```
bun x shadcn-svelte@latest add input
```

```
npx shadcn-svelte@latest add input
```

Copy

Copy and paste the component source files linked at the top of this page into your project.

Usage
-----

```
<script lang="ts">
import { Input } from "$lib/components/ui/input/index.js";
</script>
<Input />
```

Copy

Examples
--------

### Default

Preview Code

```
<script lang="ts">
import { Input } from "$lib/components/ui/input/index.js";
</script>
<Input type="email" placeholder="email" class="max-w-xs" />
```

Copy

### Disabled

Preview Code

```
<script lang="ts">
import { Input } from "$lib/components/ui/input/index.js";
</script>
<Input disabled type="email" placeholder="email" class="max-w-sm" />
```

Copy

### With Label

Preview Code

Email 

```
<script lang="ts">
import { Input } from "$lib/components/ui/input/index.js";
import { Label } from "$lib/components/ui/label/index.js";
const id = $props.id();
</script>
<div class="flex w-full max-w-sm flex-col gap-1.5">
<Label for="email-{id}">Email</Label>
<Input type="email" id="email-{id}" placeholder="email" />
</div>
```

Copy

### With Text

Preview Code

Email Enter your email address.

```
<script lang="ts">
import { Input } from "$lib/components/ui/input/index.js";
import { Label } from "$lib/components/ui/label/index.js";
</script>
<div class="flex w-full max-w-sm flex-col gap-1.5">
<Label for="email-2">Email</Label>
<Input type="email" id="email-2" placeholder="Email" />
<p class="text-muted-foreground text-sm">Enter your email address.</p>
</div>
```

Copy

### With Button

Preview Code

Subscribe

```
<script lang="ts">
import { Button } from "$lib/components/ui/button/index.js";
import { Input } from "$lib/components/ui/input/index.js";
</script>
<form class="flex w-full max-w-sm items-center space-x-2">
<Input type="email" placeholder="email" />
<Button type="submit">Subscribe</Button>
</form>
```

Copy

### Invalid

Preview Code

```
<script lang="ts">
import { Input } from "$lib/components/ui/input/index.js";
</script>
<Input
aria-invalid
type="email"
placeholder="email"
value="shadcn@example"
class="max-w-sm"
/>
```

Copy

### File

Preview Code

Picture 

```
<script lang="ts">
import { Input } from "$lib/components/ui/input/index.js";
import { Label } from "$lib/components/ui/label/index.js";
</script>
<div class="grid w-full max-w-sm items-center gap-1.5">
<Label for="picture">Picture</Label>
<Input id="picture" type="file" />
</div>
```

Copy

### Form

Preview Code

Username 

This is your public display name.

Submit

```
<script lang="ts" module>
import { z } from "zod/v4";
const formSchema = z.object({
username: z.string().min(2).max(50)
});
</script>
<script lang="ts">
import { defaults, superForm } from "sveltekit-superforms";
import { zod4 } from "sveltekit-superforms/adapters";
import { toast } from "svelte-sonner";
import * as Form from "$lib/components/ui/form/index.js";
import { Input } from "$lib/components/ui/input/index.js";
const form = superForm(defaults(zod4(formSchema)), {
validators: zod4(formSchema),
SPA: true,
onUpdate: ({ form: f }) => {
if (f.valid) {
toast.success(`You submitted ${JSON.stringify(f.data, null, 2)}`);
} else {
toast.error("Please fix the errors in the form.");
}
}
});
const { form: formData, enhance } = form;
</script>
<form method="POST" class="w-2/3 space-y-6" use:enhance>
<Form.Field {form} name="username">
<Form.Control>
{#snippet children({ props })}
<Form.Label>Username</Form.Label>
<Input {...props} bind:value={$formData.username} />
{/snippet}
</Form.Control>
<Form.Description>This is your public display name.</Form.Description>
<Form.FieldErrors />
</Form.Field>
<Form.Button>Submit</Form.Button>
</form>
```

Copy

Input OTP Label