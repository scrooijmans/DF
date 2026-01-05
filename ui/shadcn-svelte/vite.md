Title: Vite - shadcn/ui

Description: Install and configure shadcn/ui for Vite.

Keywords: Next.js,React,Tailwind CSS,Components,shadcn

Sections

- Get Started
- Components
- Registry
- MCP Server
- Changelog

Get Started

- Installation
- components.json
- Theming
- Dark Mode
- CLI
- Monorepo
- Open in v0
- JavaScript
- Blocks
- Figma
- Legacy Docs

Components

- Accordion
- Alert
- Alert Dialog
- Aspect Ratio
- Avatar
- Badge
- Breadcrumb
- Button
- Button Group
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
- React Hook Form
- Hover Card
- Input
- Input Group
- Input OTP
- Item
- Kbd
- Label
- Menubar
- Navigation Menu
- Pagination
- Popover
- Progress
- Radio Group
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
- Toast
- Toggle
- Toggle Group
- Tooltip
- Typography

Registry

- Introduction
- Getting Started
- Namespaces
- Authentication
- Examples
- MCP Server
- Index
- Open in v0
- registry.json
- registry-item.json

# Vite

Copy Page

PreviousNext

Install and configure shadcn/ui for Vite.

### Create project

Start by creating a new React project using `vite`. Select the **React + TypeScript** template:

pnpmnpmyarnbun

```
pnpm create vite@latest
```

Copy

### Add Tailwind CSS

pnpmnpmyarnbun

```
pnpm add tailwindcss @tailwindcss/vite
```

Copy

Replace everything in `src/index.css` with the following:

src/index.css

Copy`@import "tailwindcss";`

### Edit tsconfig.json file

The current version of Vite splits TypeScript configuration into three files, two of which need to be edited. Add the `baseUrl` and `paths` properties to the `compilerOptions` section of the `tsconfig.json` and `tsconfig.app.json` files:

tsconfig.json

Copy```
{
"files": [],
"references": [
{
"path": "./tsconfig.app.json"
},
{
"path": "./tsconfig.node.json"
}
],
"compilerOptions": {
"baseUrl": ".",
"paths": {
"@/_": ["./src/_"]
}
}
}

````

### Edit tsconfig.app.json file

Add the following code to the `tsconfig.app.json` file to resolve paths, for your IDE:

tsconfig.app.json

Copy```
{
"compilerOptions": {
// ...
"baseUrl": ".",
"paths": {
"@/*": [
"./src/*"
]
}
// ...
}
}
````

### Update vite.config.ts

Add the following code to the vite.config.ts so your app can resolve paths without error:

pnpmnpmyarnbun

```
pnpm add -D @types/node
```

Copy

vite.config.ts

Copy```
import path from "path"
import tailwindcss from "@tailwindcss/vite"
import react from "@vitejs/plugin-react"
import { defineConfig } from "vite"
// https://vite.dev/config/
export default defineConfig({
plugins: [react(), tailwindcss()],
resolve: {
alias: {
"@": path.resolve(\_\_dirname, "./src"),
},
},
})

```

### Run the CLI

Run the `shadcn` init command to setup your project:

pnpmnpmyarnbun

```

pnpm dlx shadcn@latest init

```

Copy

You will be asked a few questions to configure `components.json`.

Copy`Which color would you like to use as base color? › Neutral`

### Add Components

You can now start adding components to your project.

pnpmnpmyarnbun

```

pnpm dlx shadcn@latest add button

````

Copy

The command above will add the `Button` component to your project. You can then import it like this:

src/App.tsx

Copy```
import { Button } from "@/components/ui/button"
function App() {
return (
<div className="flex min-h-svh flex-col items-center justify-center">
<Button>Click me</Button>
</div>
)
}
export default App
````

Next.jsLaravel

On This Page

Create projectAdd Tailwind CSSEdit tsconfig.json fileEdit tsconfig.app.json fileUpdate vite.config.tsRun the CLIAdd Components

Deploy your shadcn/ui app on Vercel

Trusted by OpenAI, Sonos, Adobe, and more.

Vercel provides tools and infrastructure to deploy apps and features at scale.

Deploy NowDeploy to Vercel
